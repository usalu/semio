// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.27.1
// 	protoc        (unknown)
// source: extension/v1/extension.proto

package v1

import (
	v1 "github.com/usalu/semio/src/packages/go/extension/adapter/v1"
	v11 "github.com/usalu/semio/src/packages/go/extension/converter/v1"
	v12 "github.com/usalu/semio/src/packages/go/extension/transformer/v1"
	v13 "github.com/usalu/semio/src/packages/go/extension/translator/v1"
	_ "github.com/usalu/semio/src/packages/go/model/v1"
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

type Extending struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Adaptings     []*v1.Adapting      `protobuf:"bytes,3,rep,name=adaptings,proto3" json:"adaptings,omitempty"`
	Convertings   []*v11.Converting   `protobuf:"bytes,4,rep,name=convertings,proto3" json:"convertings,omitempty"`
	Transformings []*v12.Transforming `protobuf:"bytes,5,rep,name=transformings,proto3" json:"transformings,omitempty"`
	Translatings  []*v13.Translating  `protobuf:"bytes,6,rep,name=translatings,proto3" json:"translatings,omitempty"`
}

func (x *Extending) Reset() {
	*x = Extending{}
	if protoimpl.UnsafeEnabled {
		mi := &file_extension_v1_extension_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Extending) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Extending) ProtoMessage() {}

func (x *Extending) ProtoReflect() protoreflect.Message {
	mi := &file_extension_v1_extension_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Extending.ProtoReflect.Descriptor instead.
func (*Extending) Descriptor() ([]byte, []int) {
	return file_extension_v1_extension_proto_rawDescGZIP(), []int{0}
}

func (x *Extending) GetAdaptings() []*v1.Adapting {
	if x != nil {
		return x.Adaptings
	}
	return nil
}

func (x *Extending) GetConvertings() []*v11.Converting {
	if x != nil {
		return x.Convertings
	}
	return nil
}

func (x *Extending) GetTransformings() []*v12.Transforming {
	if x != nil {
		return x.Transformings
	}
	return nil
}

func (x *Extending) GetTranslatings() []*v13.Translating {
	if x != nil {
		return x.Translatings
	}
	return nil
}

var File_extension_v1_extension_proto protoreflect.FileDescriptor

var file_extension_v1_extension_proto_rawDesc = []byte{
	0x0a, 0x1c, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x2f, 0x76, 0x31, 0x2f, 0x65,
	0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x12,
	0x73, 0x65, 0x6d, 0x69, 0x6f, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x2e,
	0x76, 0x31, 0x1a, 0x14, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x2f, 0x76, 0x31, 0x2f, 0x6d, 0x6f, 0x64,
	0x65, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x22, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73,
	0x69, 0x6f, 0x6e, 0x2f, 0x61, 0x64, 0x61, 0x70, 0x74, 0x65, 0x72, 0x2f, 0x76, 0x31, 0x2f, 0x61,
	0x64, 0x61, 0x70, 0x74, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x26, 0x65, 0x78,
	0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x2f, 0x63, 0x6f, 0x6e, 0x76, 0x65, 0x72, 0x74, 0x65,
	0x72, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6e, 0x76, 0x65, 0x72, 0x74, 0x65, 0x72, 0x2e, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x2a, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x2f,
	0x74, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x6f, 0x72, 0x6d, 0x65, 0x72, 0x2f, 0x76, 0x31, 0x2f, 0x74,
	0x72, 0x61, 0x6e, 0x73, 0x66, 0x6f, 0x72, 0x6d, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x1a, 0x28, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x2f, 0x74, 0x72, 0x61, 0x6e,
	0x73, 0x6c, 0x61, 0x74, 0x6f, 0x72, 0x2f, 0x76, 0x31, 0x2f, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x6c,
	0x61, 0x74, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xbf, 0x02, 0x0a, 0x09, 0x45,
	0x78, 0x74, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x12, 0x42, 0x0a, 0x09, 0x61, 0x64, 0x61, 0x70,
	0x74, 0x69, 0x6e, 0x67, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x24, 0x2e, 0x73, 0x65,
	0x6d, 0x69, 0x6f, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x2e, 0x61, 0x64,
	0x61, 0x70, 0x74, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x2e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x69, 0x6e,
	0x67, 0x52, 0x09, 0x61, 0x64, 0x61, 0x70, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x12, 0x4a, 0x0a, 0x0b,
	0x63, 0x6f, 0x6e, 0x76, 0x65, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28,
	0x0b, 0x32, 0x28, 0x2e, 0x73, 0x65, 0x6d, 0x69, 0x6f, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73,
	0x69, 0x6f, 0x6e, 0x2e, 0x63, 0x6f, 0x6e, 0x76, 0x65, 0x72, 0x74, 0x65, 0x72, 0x2e, 0x76, 0x31,
	0x2e, 0x43, 0x6f, 0x6e, 0x76, 0x65, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x52, 0x0b, 0x63, 0x6f, 0x6e,
	0x76, 0x65, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x12, 0x52, 0x0a, 0x0d, 0x74, 0x72, 0x61, 0x6e,
	0x73, 0x66, 0x6f, 0x72, 0x6d, 0x69, 0x6e, 0x67, 0x73, 0x18, 0x05, 0x20, 0x03, 0x28, 0x0b, 0x32,
	0x2c, 0x2e, 0x73, 0x65, 0x6d, 0x69, 0x6f, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f,
	0x6e, 0x2e, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x6f, 0x72, 0x6d, 0x65, 0x72, 0x2e, 0x76, 0x31,
	0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x6f, 0x72, 0x6d, 0x69, 0x6e, 0x67, 0x52, 0x0d, 0x74,
	0x72, 0x61, 0x6e, 0x73, 0x66, 0x6f, 0x72, 0x6d, 0x69, 0x6e, 0x67, 0x73, 0x12, 0x4e, 0x0a, 0x0c,
	0x74, 0x72, 0x61, 0x6e, 0x73, 0x6c, 0x61, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x18, 0x06, 0x20, 0x03,
	0x28, 0x0b, 0x32, 0x2a, 0x2e, 0x73, 0x65, 0x6d, 0x69, 0x6f, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e,
	0x73, 0x69, 0x6f, 0x6e, 0x2e, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x6c, 0x61, 0x74, 0x6f, 0x72, 0x2e,
	0x76, 0x31, 0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x6c, 0x61, 0x74, 0x69, 0x6e, 0x67, 0x52, 0x0c,
	0x74, 0x72, 0x61, 0x6e, 0x73, 0x6c, 0x61, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x42, 0xc7, 0x01, 0x0a,
	0x16, 0x63, 0x6f, 0x6d, 0x2e, 0x73, 0x65, 0x6d, 0x69, 0x6f, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e,
	0x73, 0x69, 0x6f, 0x6e, 0x2e, 0x76, 0x31, 0x42, 0x0e, 0x45, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69,
	0x6f, 0x6e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x33, 0x67, 0x69, 0x74, 0x68, 0x75,
	0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x75, 0x73, 0x61, 0x6c, 0x75, 0x2f, 0x73, 0x65, 0x6d, 0x69,
	0x6f, 0x2f, 0x73, 0x72, 0x63, 0x2f, 0x70, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x73, 0x2f, 0x67,
	0x6f, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x2f, 0x76, 0x31, 0xa2, 0x02,
	0x03, 0x53, 0x45, 0x58, 0xaa, 0x02, 0x12, 0x53, 0x65, 0x6d, 0x69, 0x6f, 0x2e, 0x45, 0x78, 0x74,
	0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x12, 0x53, 0x65, 0x6d, 0x69,
	0x6f, 0x5c, 0x45, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x5c, 0x56, 0x31, 0xe2, 0x02,
	0x1e, 0x53, 0x65, 0x6d, 0x69, 0x6f, 0x5c, 0x45, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e,
	0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea,
	0x02, 0x14, 0x53, 0x65, 0x6d, 0x69, 0x6f, 0x3a, 0x3a, 0x45, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69,
	0x6f, 0x6e, 0x3a, 0x3a, 0x56, 0x31, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_extension_v1_extension_proto_rawDescOnce sync.Once
	file_extension_v1_extension_proto_rawDescData = file_extension_v1_extension_proto_rawDesc
)

func file_extension_v1_extension_proto_rawDescGZIP() []byte {
	file_extension_v1_extension_proto_rawDescOnce.Do(func() {
		file_extension_v1_extension_proto_rawDescData = protoimpl.X.CompressGZIP(file_extension_v1_extension_proto_rawDescData)
	})
	return file_extension_v1_extension_proto_rawDescData
}

var file_extension_v1_extension_proto_msgTypes = make([]protoimpl.MessageInfo, 1)
var file_extension_v1_extension_proto_goTypes = []interface{}{
	(*Extending)(nil),        // 0: semio.extension.v1.Extending
	(*v1.Adapting)(nil),      // 1: semio.extension.adapter.v1.Adapting
	(*v11.Converting)(nil),   // 2: semio.extension.converter.v1.Converting
	(*v12.Transforming)(nil), // 3: semio.extension.transformer.v1.Transforming
	(*v13.Translating)(nil),  // 4: semio.extension.translator.v1.Translating
}
var file_extension_v1_extension_proto_depIdxs = []int32{
	1, // 0: semio.extension.v1.Extending.adaptings:type_name -> semio.extension.adapter.v1.Adapting
	2, // 1: semio.extension.v1.Extending.convertings:type_name -> semio.extension.converter.v1.Converting
	3, // 2: semio.extension.v1.Extending.transformings:type_name -> semio.extension.transformer.v1.Transforming
	4, // 3: semio.extension.v1.Extending.translatings:type_name -> semio.extension.translator.v1.Translating
	4, // [4:4] is the sub-list for method output_type
	4, // [4:4] is the sub-list for method input_type
	4, // [4:4] is the sub-list for extension type_name
	4, // [4:4] is the sub-list for extension extendee
	0, // [0:4] is the sub-list for field type_name
}

func init() { file_extension_v1_extension_proto_init() }
func file_extension_v1_extension_proto_init() {
	if File_extension_v1_extension_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_extension_v1_extension_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Extending); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_extension_v1_extension_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   1,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_extension_v1_extension_proto_goTypes,
		DependencyIndexes: file_extension_v1_extension_proto_depIdxs,
		MessageInfos:      file_extension_v1_extension_proto_msgTypes,
	}.Build()
	File_extension_v1_extension_proto = out.File
	file_extension_v1_extension_proto_rawDesc = nil
	file_extension_v1_extension_proto_goTypes = nil
	file_extension_v1_extension_proto_depIdxs = nil
}