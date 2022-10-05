// This file is generated by rust-protobuf 4.0.0-alpha.0. Do not edit
// .proto file is parsed by protoc --rust_out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `rustproto.proto`

/// Extension fields
pub mod exts {

    pub const generate_accessors_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, bool> = crate::ext::ExtFieldOptional::new(17004, crate::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const generate_getter_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, bool> = crate::ext::ExtFieldOptional::new(17005, crate::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const tokio_bytes_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, bool> = crate::ext::ExtFieldOptional::new(17011, crate::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const tokio_bytes_for_string_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, bool> = crate::ext::ExtFieldOptional::new(17012, crate::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const lite_runtime_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, bool> = crate::ext::ExtFieldOptional::new(17035, crate::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const generate_accessors: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, bool> = crate::ext::ExtFieldOptional::new(17004, crate::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const generate_getter: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, bool> = crate::ext::ExtFieldOptional::new(17005, crate::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const tokio_bytes: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, bool> = crate::ext::ExtFieldOptional::new(17011, crate::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const tokio_bytes_for_string: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, bool> = crate::ext::ExtFieldOptional::new(17012, crate::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const generate_accessors_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, bool> = crate::ext::ExtFieldOptional::new(17004, crate::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const generate_getter_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, bool> = crate::ext::ExtFieldOptional::new(17005, crate::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const tokio_bytes_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, bool> = crate::ext::ExtFieldOptional::new(17011, crate::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const tokio_bytes_for_string_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, bool> = crate::ext::ExtFieldOptional::new(17012, crate::descriptor::field_descriptor_proto::Type::TYPE_BOOL);
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0frustproto.proto\x12\trustproto\x1a\x20google/protobuf/descriptor.p\
    roto:T\n\x16generate_accessors_all\x18\xec\x84\x01\x20\x01(\x08\x12\x1c.\
    google.protobuf.FileOptionsR\x14generateAccessorsAll:N\n\x13generate_get\
    ter_all\x18\xed\x84\x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOptions\
    R\x11generateGetterAll:F\n\x0ftokio_bytes_all\x18\xf3\x84\x01\x20\x01(\
    \x08\x12\x1c.google.protobuf.FileOptionsR\rtokioBytesAll:Z\n\x1atokio_by\
    tes_for_string_all\x18\xf4\x84\x01\x20\x01(\x08\x12\x1c.google.protobuf.\
    FileOptionsR\x16tokioBytesForStringAll:H\n\x10lite_runtime_all\x18\x8b\
    \x85\x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x0eliteRuntim\
    eAll:P\n\x12generate_accessors\x18\xec\x84\x01\x20\x01(\x08\x12\x1f.goog\
    le.protobuf.MessageOptionsR\x11generateAccessors:J\n\x0fgenerate_getter\
    \x18\xed\x84\x01\x20\x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\
    \x0egenerateGetter:B\n\x0btokio_bytes\x18\xf3\x84\x01\x20\x01(\x08\x12\
    \x1f.google.protobuf.MessageOptionsR\ntokioBytes:V\n\x16tokio_bytes_for_\
    string\x18\xf4\x84\x01\x20\x01(\x08\x12\x1f.google.protobuf.MessageOptio\
    nsR\x13tokioBytesForString:Y\n\x18generate_accessors_field\x18\xec\x84\
    \x01\x20\x01(\x08\x12\x1d.google.protobuf.FieldOptionsR\x16generateAcces\
    sorsField:S\n\x15generate_getter_field\x18\xed\x84\x01\x20\x01(\x08\x12\
    \x1d.google.protobuf.FieldOptionsR\x13generateGetterField:K\n\x11tokio_b\
    ytes_field\x18\xf3\x84\x01\x20\x01(\x08\x12\x1d.google.protobuf.FieldOpt\
    ionsR\x0ftokioBytesField:_\n\x1ctokio_bytes_for_string_field\x18\xf4\x84\
    \x01\x20\x01(\x08\x12\x1d.google.protobuf.FieldOptionsR\x18tokioBytesFor\
    StringFieldJ\x9d\x0f\n\x06\x12\x04\0\0.\x01\n\x08\n\x01\x0c\x12\x03\0\0\
    \x12\n\t\n\x02\x03\0\x12\x03\x02\0*\n\xe5\x01\n\x01\x02\x12\x03\n\0\x122\
    ^\x20see\x20https://github.com/gogo/protobuf/blob/master/gogoproto/gogo.\
    proto\n\x20for\x20the\x20original\x20idea\n2{\x20Generated\x20files\x20c\
    an\x20be\x20customized\x20using\x20this\x20proto\n\x20or\x20using\x20`Cu\
    stomize`\x20struct\x20when\x20codegen\x20is\x20invoked\x20programmatical\
    ly.\n\n\t\n\x01\x07\x12\x04\x0c\0\x18\x01\nP\n\x02\x07\0\x12\x03\x0e\x04\
    1\x1aE\x20When\x20false,\x20`get_`,\x20`set_`,\x20`mut_`\x20etc.\x20acce\
    ssors\x20are\x20not\x20generated\n\n\n\n\x03\x07\0\x02\x12\x03\x0c\x07\"\
    \n\n\n\x03\x07\0\x04\x12\x03\x0e\x04\x0c\n\n\n\x03\x07\0\x05\x12\x03\x0e\
    \r\x11\n\n\n\x03\x07\0\x01\x12\x03\x0e\x12(\n\n\n\x03\x07\0\x03\x12\x03\
    \x0e+0\nL\n\x02\x07\x01\x12\x03\x10\x04.\x1aA\x20When\x20false,\x20`get_\
    `\x20is\x20not\x20generated\x20even\x20if\x20`syntax\x20=\x20\"proto2\"`\
    \n\n\n\n\x03\x07\x01\x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\x01\x04\x12\x03\
    \x10\x04\x0c\n\n\n\x03\x07\x01\x05\x12\x03\x10\r\x11\n\n\n\x03\x07\x01\
    \x01\x12\x03\x10\x12%\n\n\n\x03\x07\x01\x03\x12\x03\x10(-\n2\n\x02\x07\
    \x02\x12\x03\x12\x04*\x1a'\x20Use\x20`bytes::Bytes`\x20for\x20`bytes`\
    \x20fields\n\n\n\n\x03\x07\x02\x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\x02\
    \x04\x12\x03\x12\x04\x0c\n\n\n\x03\x07\x02\x05\x12\x03\x12\r\x11\n\n\n\
    \x03\x07\x02\x01\x12\x03\x12\x12!\n\n\n\x03\x07\x02\x03\x12\x03\x12$)\n3\
    \n\x02\x07\x03\x12\x03\x14\x045\x1a(\x20Use\x20`bytes::Bytes`\x20for\x20\
    `string`\x20fields\n\n\n\n\x03\x07\x03\x02\x12\x03\x0c\x07\"\n\n\n\x03\
    \x07\x03\x04\x12\x03\x14\x04\x0c\n\n\n\x03\x07\x03\x05\x12\x03\x14\r\x11\
    \n\n\n\x03\x07\x03\x01\x12\x03\x14\x12,\n\n\n\x03\x07\x03\x03\x12\x03\
    \x14/4\nN\n\x02\x07\x04\x12\x03\x17\x04+\x1aC\x20When\x20true,\x20will\
    \x20only\x20generate\x20codes\x20that\x20works\x20with\x20lite\x20runtim\
    e.\n\n\n\n\x03\x07\x04\x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\x04\x04\x12\
    \x03\x17\x04\x0c\n\n\n\x03\x07\x04\x05\x12\x03\x17\r\x11\n\n\n\x03\x07\
    \x04\x01\x12\x03\x17\x12\"\n\n\n\x03\x07\x04\x03\x12\x03\x17%*\n\t\n\x01\
    \x07\x12\x04\x1a\0#\x01\nP\n\x02\x07\x05\x12\x03\x1c\x04-\x1aE\x20When\
    \x20false,\x20`get_`,\x20`set_`,\x20`mut_`\x20etc.\x20accessors\x20are\
    \x20not\x20generated\n\n\n\n\x03\x07\x05\x02\x12\x03\x1a\x07%\n\n\n\x03\
    \x07\x05\x04\x12\x03\x1c\x04\x0c\n\n\n\x03\x07\x05\x05\x12\x03\x1c\r\x11\
    \n\n\n\x03\x07\x05\x01\x12\x03\x1c\x12$\n\n\n\x03\x07\x05\x03\x12\x03\
    \x1c',\nL\n\x02\x07\x06\x12\x03\x1e\x04*\x1aA\x20When\x20false,\x20`get_\
    `\x20is\x20not\x20generated\x20even\x20if\x20`syntax\x20=\x20\"proto2\"`\
    \n\n\n\n\x03\x07\x06\x02\x12\x03\x1a\x07%\n\n\n\x03\x07\x06\x04\x12\x03\
    \x1e\x04\x0c\n\n\n\x03\x07\x06\x05\x12\x03\x1e\r\x11\n\n\n\x03\x07\x06\
    \x01\x12\x03\x1e\x12!\n\n\n\x03\x07\x06\x03\x12\x03\x1e$)\n2\n\x02\x07\
    \x07\x12\x03\x20\x04&\x1a'\x20Use\x20`bytes::Bytes`\x20for\x20`bytes`\
    \x20fields\n\n\n\n\x03\x07\x07\x02\x12\x03\x1a\x07%\n\n\n\x03\x07\x07\
    \x04\x12\x03\x20\x04\x0c\n\n\n\x03\x07\x07\x05\x12\x03\x20\r\x11\n\n\n\
    \x03\x07\x07\x01\x12\x03\x20\x12\x1d\n\n\n\x03\x07\x07\x03\x12\x03\x20\
    \x20%\n3\n\x02\x07\x08\x12\x03\"\x041\x1a(\x20Use\x20`bytes::Bytes`\x20f\
    or\x20`string`\x20fields\n\n\n\n\x03\x07\x08\x02\x12\x03\x1a\x07%\n\n\n\
    \x03\x07\x08\x04\x12\x03\"\x04\x0c\n\n\n\x03\x07\x08\x05\x12\x03\"\r\x11\
    \n\n\n\x03\x07\x08\x01\x12\x03\"\x12(\n\n\n\x03\x07\x08\x03\x12\x03\"+0\
    \n\t\n\x01\x07\x12\x04%\0.\x01\nP\n\x02\x07\t\x12\x03'\x043\x1aE\x20When\
    \x20false,\x20`get_`,\x20`set_`,\x20`mut_`\x20etc.\x20accessors\x20are\
    \x20not\x20generated\n\n\n\n\x03\x07\t\x02\x12\x03%\x07#\n\n\n\x03\x07\t\
    \x04\x12\x03'\x04\x0c\n\n\n\x03\x07\t\x05\x12\x03'\r\x11\n\n\n\x03\x07\t\
    \x01\x12\x03'\x12*\n\n\n\x03\x07\t\x03\x12\x03'-2\nL\n\x02\x07\n\x12\x03\
    )\x040\x1aA\x20When\x20false,\x20`get_`\x20is\x20not\x20generated\x20eve\
    n\x20if\x20`syntax\x20=\x20\"proto2\"`\n\n\n\n\x03\x07\n\x02\x12\x03%\
    \x07#\n\n\n\x03\x07\n\x04\x12\x03)\x04\x0c\n\n\n\x03\x07\n\x05\x12\x03)\
    \r\x11\n\n\n\x03\x07\n\x01\x12\x03)\x12'\n\n\n\x03\x07\n\x03\x12\x03)*/\
    \n2\n\x02\x07\x0b\x12\x03+\x04,\x1a'\x20Use\x20`bytes::Bytes`\x20for\x20\
    `bytes`\x20fields\n\n\n\n\x03\x07\x0b\x02\x12\x03%\x07#\n\n\n\x03\x07\
    \x0b\x04\x12\x03+\x04\x0c\n\n\n\x03\x07\x0b\x05\x12\x03+\r\x11\n\n\n\x03\
    \x07\x0b\x01\x12\x03+\x12#\n\n\n\x03\x07\x0b\x03\x12\x03+&+\n3\n\x02\x07\
    \x0c\x12\x03-\x047\x1a(\x20Use\x20`bytes::Bytes`\x20for\x20`string`\x20f\
    ields\n\n\n\n\x03\x07\x0c\x02\x12\x03%\x07#\n\n\n\x03\x07\x0c\x04\x12\
    \x03-\x04\x0c\n\n\n\x03\x07\x0c\x05\x12\x03-\r\x11\n\n\n\x03\x07\x0c\x01\
    \x12\x03-\x12.\n\n\n\x03\x07\x0c\x03\x12\x03-16\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static crate::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: crate::rt::Lazy<crate::descriptor::FileDescriptorProto> = crate::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        crate::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static crate::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: crate::rt::Lazy<crate::reflect::GeneratedFileDescriptor> = crate::rt::Lazy::new();
    static file_descriptor: crate::rt::Lazy<crate::reflect::FileDescriptor> = crate::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(crate::descriptor::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(0);
            let mut enums = ::std::vec::Vec::with_capacity(0);
            crate::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        crate::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
