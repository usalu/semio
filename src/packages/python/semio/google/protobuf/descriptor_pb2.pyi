from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf.internal import python_message as _python_message
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class DescriptorProto(_message.Message):
    __slots__ = ["enum_type", "extension", "extension_range", "field", "name", "nested_type", "oneof_decl", "options", "reserved_name", "reserved_range"]
    class ExtensionRange(_message.Message):
        __slots__ = ["end", "options", "start"]
        END_FIELD_NUMBER: _ClassVar[int]
        OPTIONS_FIELD_NUMBER: _ClassVar[int]
        START_FIELD_NUMBER: _ClassVar[int]
        end: int
        options: ExtensionRangeOptions
        start: int
        def __init__(self, start: _Optional[int] = ..., end: _Optional[int] = ..., options: _Optional[_Union[ExtensionRangeOptions, _Mapping]] = ...) -> None: ...
    class ReservedRange(_message.Message):
        __slots__ = ["end", "start"]
        END_FIELD_NUMBER: _ClassVar[int]
        START_FIELD_NUMBER: _ClassVar[int]
        end: int
        start: int
        def __init__(self, start: _Optional[int] = ..., end: _Optional[int] = ...) -> None: ...
    ENUM_TYPE_FIELD_NUMBER: _ClassVar[int]
    EXTENSION_FIELD_NUMBER: _ClassVar[int]
    EXTENSION_RANGE_FIELD_NUMBER: _ClassVar[int]
    FIELD_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    NESTED_TYPE_FIELD_NUMBER: _ClassVar[int]
    ONEOF_DECL_FIELD_NUMBER: _ClassVar[int]
    OPTIONS_FIELD_NUMBER: _ClassVar[int]
    RESERVED_NAME_FIELD_NUMBER: _ClassVar[int]
    RESERVED_RANGE_FIELD_NUMBER: _ClassVar[int]
    enum_type: _containers.RepeatedCompositeFieldContainer[EnumDescriptorProto]
    extension: _containers.RepeatedCompositeFieldContainer[FieldDescriptorProto]
    extension_range: _containers.RepeatedCompositeFieldContainer[DescriptorProto.ExtensionRange]
    field: _containers.RepeatedCompositeFieldContainer[FieldDescriptorProto]
    name: str
    nested_type: _containers.RepeatedCompositeFieldContainer[DescriptorProto]
    oneof_decl: _containers.RepeatedCompositeFieldContainer[OneofDescriptorProto]
    options: MessageOptions
    reserved_name: _containers.RepeatedScalarFieldContainer[str]
    reserved_range: _containers.RepeatedCompositeFieldContainer[DescriptorProto.ReservedRange]
    def __init__(self, name: _Optional[str] = ..., field: _Optional[_Iterable[_Union[FieldDescriptorProto, _Mapping]]] = ..., extension: _Optional[_Iterable[_Union[FieldDescriptorProto, _Mapping]]] = ..., nested_type: _Optional[_Iterable[_Union[DescriptorProto, _Mapping]]] = ..., enum_type: _Optional[_Iterable[_Union[EnumDescriptorProto, _Mapping]]] = ..., extension_range: _Optional[_Iterable[_Union[DescriptorProto.ExtensionRange, _Mapping]]] = ..., oneof_decl: _Optional[_Iterable[_Union[OneofDescriptorProto, _Mapping]]] = ..., options: _Optional[_Union[MessageOptions, _Mapping]] = ..., reserved_range: _Optional[_Iterable[_Union[DescriptorProto.ReservedRange, _Mapping]]] = ..., reserved_name: _Optional[_Iterable[str]] = ...) -> None: ...

class EnumDescriptorProto(_message.Message):
    __slots__ = ["name", "options", "reserved_name", "reserved_range", "value"]
    class EnumReservedRange(_message.Message):
        __slots__ = ["end", "start"]
        END_FIELD_NUMBER: _ClassVar[int]
        START_FIELD_NUMBER: _ClassVar[int]
        end: int
        start: int
        def __init__(self, start: _Optional[int] = ..., end: _Optional[int] = ...) -> None: ...
    NAME_FIELD_NUMBER: _ClassVar[int]
    OPTIONS_FIELD_NUMBER: _ClassVar[int]
    RESERVED_NAME_FIELD_NUMBER: _ClassVar[int]
    RESERVED_RANGE_FIELD_NUMBER: _ClassVar[int]
    VALUE_FIELD_NUMBER: _ClassVar[int]
    name: str
    options: EnumOptions
    reserved_name: _containers.RepeatedScalarFieldContainer[str]
    reserved_range: _containers.RepeatedCompositeFieldContainer[EnumDescriptorProto.EnumReservedRange]
    value: _containers.RepeatedCompositeFieldContainer[EnumValueDescriptorProto]
    def __init__(self, name: _Optional[str] = ..., value: _Optional[_Iterable[_Union[EnumValueDescriptorProto, _Mapping]]] = ..., options: _Optional[_Union[EnumOptions, _Mapping]] = ..., reserved_range: _Optional[_Iterable[_Union[EnumDescriptorProto.EnumReservedRange, _Mapping]]] = ..., reserved_name: _Optional[_Iterable[str]] = ...) -> None: ...

class EnumOptions(_message.Message):
    __slots__ = ["allow_alias", "deprecated", "uninterpreted_option"]
    ALLOW_ALIAS_FIELD_NUMBER: _ClassVar[int]
    DEPRECATED_FIELD_NUMBER: _ClassVar[int]
    Extensions: _python_message._ExtensionDict
    UNINTERPRETED_OPTION_FIELD_NUMBER: _ClassVar[int]
    allow_alias: bool
    deprecated: bool
    uninterpreted_option: _containers.RepeatedCompositeFieldContainer[UninterpretedOption]
    def __init__(self, allow_alias: bool = ..., deprecated: bool = ..., uninterpreted_option: _Optional[_Iterable[_Union[UninterpretedOption, _Mapping]]] = ...) -> None: ...

class EnumValueDescriptorProto(_message.Message):
    __slots__ = ["name", "number", "options"]
    NAME_FIELD_NUMBER: _ClassVar[int]
    NUMBER_FIELD_NUMBER: _ClassVar[int]
    OPTIONS_FIELD_NUMBER: _ClassVar[int]
    name: str
    number: int
    options: EnumValueOptions
    def __init__(self, name: _Optional[str] = ..., number: _Optional[int] = ..., options: _Optional[_Union[EnumValueOptions, _Mapping]] = ...) -> None: ...

class EnumValueOptions(_message.Message):
    __slots__ = ["deprecated", "uninterpreted_option"]
    DEPRECATED_FIELD_NUMBER: _ClassVar[int]
    Extensions: _python_message._ExtensionDict
    UNINTERPRETED_OPTION_FIELD_NUMBER: _ClassVar[int]
    deprecated: bool
    uninterpreted_option: _containers.RepeatedCompositeFieldContainer[UninterpretedOption]
    def __init__(self, deprecated: bool = ..., uninterpreted_option: _Optional[_Iterable[_Union[UninterpretedOption, _Mapping]]] = ...) -> None: ...

class ExtensionRangeOptions(_message.Message):
    __slots__ = ["uninterpreted_option"]
    Extensions: _python_message._ExtensionDict
    UNINTERPRETED_OPTION_FIELD_NUMBER: _ClassVar[int]
    uninterpreted_option: _containers.RepeatedCompositeFieldContainer[UninterpretedOption]
    def __init__(self, uninterpreted_option: _Optional[_Iterable[_Union[UninterpretedOption, _Mapping]]] = ...) -> None: ...

class FieldDescriptorProto(_message.Message):
    __slots__ = ["default_value", "extendee", "json_name", "label", "name", "number", "oneof_index", "options", "proto3_optional", "type", "type_name"]
    class Label(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
        __slots__ = []
    class Type(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
        __slots__ = []
    DEFAULT_VALUE_FIELD_NUMBER: _ClassVar[int]
    EXTENDEE_FIELD_NUMBER: _ClassVar[int]
    JSON_NAME_FIELD_NUMBER: _ClassVar[int]
    LABEL_FIELD_NUMBER: _ClassVar[int]
    LABEL_OPTIONAL: FieldDescriptorProto.Label
    LABEL_REPEATED: FieldDescriptorProto.Label
    LABEL_REQUIRED: FieldDescriptorProto.Label
    NAME_FIELD_NUMBER: _ClassVar[int]
    NUMBER_FIELD_NUMBER: _ClassVar[int]
    ONEOF_INDEX_FIELD_NUMBER: _ClassVar[int]
    OPTIONS_FIELD_NUMBER: _ClassVar[int]
    PROTO3_OPTIONAL_FIELD_NUMBER: _ClassVar[int]
    TYPE_BOOL: FieldDescriptorProto.Type
    TYPE_BYTES: FieldDescriptorProto.Type
    TYPE_DOUBLE: FieldDescriptorProto.Type
    TYPE_ENUM: FieldDescriptorProto.Type
    TYPE_FIELD_NUMBER: _ClassVar[int]
    TYPE_FIXED32: FieldDescriptorProto.Type
    TYPE_FIXED64: FieldDescriptorProto.Type
    TYPE_FLOAT: FieldDescriptorProto.Type
    TYPE_GROUP: FieldDescriptorProto.Type
    TYPE_INT32: FieldDescriptorProto.Type
    TYPE_INT64: FieldDescriptorProto.Type
    TYPE_MESSAGE: FieldDescriptorProto.Type
    TYPE_NAME_FIELD_NUMBER: _ClassVar[int]
    TYPE_SFIXED32: FieldDescriptorProto.Type
    TYPE_SFIXED64: FieldDescriptorProto.Type
    TYPE_SINT32: FieldDescriptorProto.Type
    TYPE_SINT64: FieldDescriptorProto.Type
    TYPE_STRING: FieldDescriptorProto.Type
    TYPE_UINT32: FieldDescriptorProto.Type
    TYPE_UINT64: FieldDescriptorProto.Type
    default_value: str
    extendee: str
    json_name: str
    label: FieldDescriptorProto.Label
    name: str
    number: int
    oneof_index: int
    options: FieldOptions
    proto3_optional: bool
    type: FieldDescriptorProto.Type
    type_name: str
    def __init__(self, name: _Optional[str] = ..., number: _Optional[int] = ..., label: _Optional[_Union[FieldDescriptorProto.Label, str]] = ..., type: _Optional[_Union[FieldDescriptorProto.Type, str]] = ..., type_name: _Optional[str] = ..., extendee: _Optional[str] = ..., default_value: _Optional[str] = ..., oneof_index: _Optional[int] = ..., json_name: _Optional[str] = ..., options: _Optional[_Union[FieldOptions, _Mapping]] = ..., proto3_optional: bool = ...) -> None: ...

class FieldOptions(_message.Message):
    __slots__ = ["ctype", "deprecated", "jstype", "lazy", "packed", "uninterpreted_option", "unverified_lazy", "weak"]
    class CType(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
        __slots__ = []
    class JSType(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
        __slots__ = []
    CORD: FieldOptions.CType
    CTYPE_FIELD_NUMBER: _ClassVar[int]
    DEPRECATED_FIELD_NUMBER: _ClassVar[int]
    Extensions: _python_message._ExtensionDict
    JSTYPE_FIELD_NUMBER: _ClassVar[int]
    JS_NORMAL: FieldOptions.JSType
    JS_NUMBER: FieldOptions.JSType
    JS_STRING: FieldOptions.JSType
    LAZY_FIELD_NUMBER: _ClassVar[int]
    PACKED_FIELD_NUMBER: _ClassVar[int]
    STRING: FieldOptions.CType
    STRING_PIECE: FieldOptions.CType
    UNINTERPRETED_OPTION_FIELD_NUMBER: _ClassVar[int]
    UNVERIFIED_LAZY_FIELD_NUMBER: _ClassVar[int]
    WEAK_FIELD_NUMBER: _ClassVar[int]
    ctype: FieldOptions.CType
    deprecated: bool
    jstype: FieldOptions.JSType
    lazy: bool
    packed: bool
    uninterpreted_option: _containers.RepeatedCompositeFieldContainer[UninterpretedOption]
    unverified_lazy: bool
    weak: bool
    def __init__(self, ctype: _Optional[_Union[FieldOptions.CType, str]] = ..., packed: bool = ..., jstype: _Optional[_Union[FieldOptions.JSType, str]] = ..., lazy: bool = ..., unverified_lazy: bool = ..., deprecated: bool = ..., weak: bool = ..., uninterpreted_option: _Optional[_Iterable[_Union[UninterpretedOption, _Mapping]]] = ...) -> None: ...

class FileDescriptorProto(_message.Message):
    __slots__ = ["dependency", "enum_type", "extension", "message_type", "name", "options", "package", "public_dependency", "service", "source_code_info", "syntax", "weak_dependency"]
    DEPENDENCY_FIELD_NUMBER: _ClassVar[int]
    ENUM_TYPE_FIELD_NUMBER: _ClassVar[int]
    EXTENSION_FIELD_NUMBER: _ClassVar[int]
    MESSAGE_TYPE_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    OPTIONS_FIELD_NUMBER: _ClassVar[int]
    PACKAGE_FIELD_NUMBER: _ClassVar[int]
    PUBLIC_DEPENDENCY_FIELD_NUMBER: _ClassVar[int]
    SERVICE_FIELD_NUMBER: _ClassVar[int]
    SOURCE_CODE_INFO_FIELD_NUMBER: _ClassVar[int]
    SYNTAX_FIELD_NUMBER: _ClassVar[int]
    WEAK_DEPENDENCY_FIELD_NUMBER: _ClassVar[int]
    dependency: _containers.RepeatedScalarFieldContainer[str]
    enum_type: _containers.RepeatedCompositeFieldContainer[EnumDescriptorProto]
    extension: _containers.RepeatedCompositeFieldContainer[FieldDescriptorProto]
    message_type: _containers.RepeatedCompositeFieldContainer[DescriptorProto]
    name: str
    options: FileOptions
    package: str
    public_dependency: _containers.RepeatedScalarFieldContainer[int]
    service: _containers.RepeatedCompositeFieldContainer[ServiceDescriptorProto]
    source_code_info: SourceCodeInfo
    syntax: str
    weak_dependency: _containers.RepeatedScalarFieldContainer[int]
    def __init__(self, name: _Optional[str] = ..., package: _Optional[str] = ..., dependency: _Optional[_Iterable[str]] = ..., public_dependency: _Optional[_Iterable[int]] = ..., weak_dependency: _Optional[_Iterable[int]] = ..., message_type: _Optional[_Iterable[_Union[DescriptorProto, _Mapping]]] = ..., enum_type: _Optional[_Iterable[_Union[EnumDescriptorProto, _Mapping]]] = ..., service: _Optional[_Iterable[_Union[ServiceDescriptorProto, _Mapping]]] = ..., extension: _Optional[_Iterable[_Union[FieldDescriptorProto, _Mapping]]] = ..., options: _Optional[_Union[FileOptions, _Mapping]] = ..., source_code_info: _Optional[_Union[SourceCodeInfo, _Mapping]] = ..., syntax: _Optional[str] = ...) -> None: ...

class FileDescriptorSet(_message.Message):
    __slots__ = ["file"]
    FILE_FIELD_NUMBER: _ClassVar[int]
    file: _containers.RepeatedCompositeFieldContainer[FileDescriptorProto]
    def __init__(self, file: _Optional[_Iterable[_Union[FileDescriptorProto, _Mapping]]] = ...) -> None: ...

class FileOptions(_message.Message):
    __slots__ = ["cc_enable_arenas", "cc_generic_services", "csharp_namespace", "deprecated", "go_package", "java_generate_equals_and_hash", "java_generic_services", "java_multiple_files", "java_outer_classname", "java_package", "java_string_check_utf8", "objc_class_prefix", "optimize_for", "php_class_prefix", "php_generic_services", "php_metadata_namespace", "php_namespace", "py_generic_services", "ruby_package", "swift_prefix", "uninterpreted_option"]
    class OptimizeMode(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
        __slots__ = []
    CC_ENABLE_ARENAS_FIELD_NUMBER: _ClassVar[int]
    CC_GENERIC_SERVICES_FIELD_NUMBER: _ClassVar[int]
    CODE_SIZE: FileOptions.OptimizeMode
    CSHARP_NAMESPACE_FIELD_NUMBER: _ClassVar[int]
    DEPRECATED_FIELD_NUMBER: _ClassVar[int]
    Extensions: _python_message._ExtensionDict
    GO_PACKAGE_FIELD_NUMBER: _ClassVar[int]
    JAVA_GENERATE_EQUALS_AND_HASH_FIELD_NUMBER: _ClassVar[int]
    JAVA_GENERIC_SERVICES_FIELD_NUMBER: _ClassVar[int]
    JAVA_MULTIPLE_FILES_FIELD_NUMBER: _ClassVar[int]
    JAVA_OUTER_CLASSNAME_FIELD_NUMBER: _ClassVar[int]
    JAVA_PACKAGE_FIELD_NUMBER: _ClassVar[int]
    JAVA_STRING_CHECK_UTF8_FIELD_NUMBER: _ClassVar[int]
    LITE_RUNTIME: FileOptions.OptimizeMode
    OBJC_CLASS_PREFIX_FIELD_NUMBER: _ClassVar[int]
    OPTIMIZE_FOR_FIELD_NUMBER: _ClassVar[int]
    PHP_CLASS_PREFIX_FIELD_NUMBER: _ClassVar[int]
    PHP_GENERIC_SERVICES_FIELD_NUMBER: _ClassVar[int]
    PHP_METADATA_NAMESPACE_FIELD_NUMBER: _ClassVar[int]
    PHP_NAMESPACE_FIELD_NUMBER: _ClassVar[int]
    PY_GENERIC_SERVICES_FIELD_NUMBER: _ClassVar[int]
    RUBY_PACKAGE_FIELD_NUMBER: _ClassVar[int]
    SPEED: FileOptions.OptimizeMode
    SWIFT_PREFIX_FIELD_NUMBER: _ClassVar[int]
    UNINTERPRETED_OPTION_FIELD_NUMBER: _ClassVar[int]
    cc_enable_arenas: bool
    cc_generic_services: bool
    csharp_namespace: str
    deprecated: bool
    go_package: str
    java_generate_equals_and_hash: bool
    java_generic_services: bool
    java_multiple_files: bool
    java_outer_classname: str
    java_package: str
    java_string_check_utf8: bool
    objc_class_prefix: str
    optimize_for: FileOptions.OptimizeMode
    php_class_prefix: str
    php_generic_services: bool
    php_metadata_namespace: str
    php_namespace: str
    py_generic_services: bool
    ruby_package: str
    swift_prefix: str
    uninterpreted_option: _containers.RepeatedCompositeFieldContainer[UninterpretedOption]
    def __init__(self, java_package: _Optional[str] = ..., java_outer_classname: _Optional[str] = ..., java_multiple_files: bool = ..., java_generate_equals_and_hash: bool = ..., java_string_check_utf8: bool = ..., optimize_for: _Optional[_Union[FileOptions.OptimizeMode, str]] = ..., go_package: _Optional[str] = ..., cc_generic_services: bool = ..., java_generic_services: bool = ..., py_generic_services: bool = ..., php_generic_services: bool = ..., deprecated: bool = ..., cc_enable_arenas: bool = ..., objc_class_prefix: _Optional[str] = ..., csharp_namespace: _Optional[str] = ..., swift_prefix: _Optional[str] = ..., php_class_prefix: _Optional[str] = ..., php_namespace: _Optional[str] = ..., php_metadata_namespace: _Optional[str] = ..., ruby_package: _Optional[str] = ..., uninterpreted_option: _Optional[_Iterable[_Union[UninterpretedOption, _Mapping]]] = ...) -> None: ...

class GeneratedCodeInfo(_message.Message):
    __slots__ = ["annotation"]
    class Annotation(_message.Message):
        __slots__ = ["begin", "end", "path", "source_file"]
        BEGIN_FIELD_NUMBER: _ClassVar[int]
        END_FIELD_NUMBER: _ClassVar[int]
        PATH_FIELD_NUMBER: _ClassVar[int]
        SOURCE_FILE_FIELD_NUMBER: _ClassVar[int]
        begin: int
        end: int
        path: _containers.RepeatedScalarFieldContainer[int]
        source_file: str
        def __init__(self, path: _Optional[_Iterable[int]] = ..., source_file: _Optional[str] = ..., begin: _Optional[int] = ..., end: _Optional[int] = ...) -> None: ...
    ANNOTATION_FIELD_NUMBER: _ClassVar[int]
    annotation: _containers.RepeatedCompositeFieldContainer[GeneratedCodeInfo.Annotation]
    def __init__(self, annotation: _Optional[_Iterable[_Union[GeneratedCodeInfo.Annotation, _Mapping]]] = ...) -> None: ...

class MessageOptions(_message.Message):
    __slots__ = ["deprecated", "map_entry", "message_set_wire_format", "no_standard_descriptor_accessor", "uninterpreted_option"]
    DEPRECATED_FIELD_NUMBER: _ClassVar[int]
    Extensions: _python_message._ExtensionDict
    MAP_ENTRY_FIELD_NUMBER: _ClassVar[int]
    MESSAGE_SET_WIRE_FORMAT_FIELD_NUMBER: _ClassVar[int]
    NO_STANDARD_DESCRIPTOR_ACCESSOR_FIELD_NUMBER: _ClassVar[int]
    UNINTERPRETED_OPTION_FIELD_NUMBER: _ClassVar[int]
    deprecated: bool
    map_entry: bool
    message_set_wire_format: bool
    no_standard_descriptor_accessor: bool
    uninterpreted_option: _containers.RepeatedCompositeFieldContainer[UninterpretedOption]
    def __init__(self, message_set_wire_format: bool = ..., no_standard_descriptor_accessor: bool = ..., deprecated: bool = ..., map_entry: bool = ..., uninterpreted_option: _Optional[_Iterable[_Union[UninterpretedOption, _Mapping]]] = ...) -> None: ...

class MethodDescriptorProto(_message.Message):
    __slots__ = ["client_streaming", "input_type", "name", "options", "output_type", "server_streaming"]
    CLIENT_STREAMING_FIELD_NUMBER: _ClassVar[int]
    INPUT_TYPE_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    OPTIONS_FIELD_NUMBER: _ClassVar[int]
    OUTPUT_TYPE_FIELD_NUMBER: _ClassVar[int]
    SERVER_STREAMING_FIELD_NUMBER: _ClassVar[int]
    client_streaming: bool
    input_type: str
    name: str
    options: MethodOptions
    output_type: str
    server_streaming: bool
    def __init__(self, name: _Optional[str] = ..., input_type: _Optional[str] = ..., output_type: _Optional[str] = ..., options: _Optional[_Union[MethodOptions, _Mapping]] = ..., client_streaming: bool = ..., server_streaming: bool = ...) -> None: ...

class MethodOptions(_message.Message):
    __slots__ = ["deprecated", "idempotency_level", "uninterpreted_option"]
    class IdempotencyLevel(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
        __slots__ = []
    DEPRECATED_FIELD_NUMBER: _ClassVar[int]
    Extensions: _python_message._ExtensionDict
    IDEMPOTENCY_LEVEL_FIELD_NUMBER: _ClassVar[int]
    IDEMPOTENCY_UNKNOWN: MethodOptions.IdempotencyLevel
    IDEMPOTENT: MethodOptions.IdempotencyLevel
    NO_SIDE_EFFECTS: MethodOptions.IdempotencyLevel
    UNINTERPRETED_OPTION_FIELD_NUMBER: _ClassVar[int]
    deprecated: bool
    idempotency_level: MethodOptions.IdempotencyLevel
    uninterpreted_option: _containers.RepeatedCompositeFieldContainer[UninterpretedOption]
    def __init__(self, deprecated: bool = ..., idempotency_level: _Optional[_Union[MethodOptions.IdempotencyLevel, str]] = ..., uninterpreted_option: _Optional[_Iterable[_Union[UninterpretedOption, _Mapping]]] = ...) -> None: ...

class OneofDescriptorProto(_message.Message):
    __slots__ = ["name", "options"]
    NAME_FIELD_NUMBER: _ClassVar[int]
    OPTIONS_FIELD_NUMBER: _ClassVar[int]
    name: str
    options: OneofOptions
    def __init__(self, name: _Optional[str] = ..., options: _Optional[_Union[OneofOptions, _Mapping]] = ...) -> None: ...

class OneofOptions(_message.Message):
    __slots__ = ["uninterpreted_option"]
    Extensions: _python_message._ExtensionDict
    UNINTERPRETED_OPTION_FIELD_NUMBER: _ClassVar[int]
    uninterpreted_option: _containers.RepeatedCompositeFieldContainer[UninterpretedOption]
    def __init__(self, uninterpreted_option: _Optional[_Iterable[_Union[UninterpretedOption, _Mapping]]] = ...) -> None: ...

class ServiceDescriptorProto(_message.Message):
    __slots__ = ["method", "name", "options"]
    METHOD_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    OPTIONS_FIELD_NUMBER: _ClassVar[int]
    method: _containers.RepeatedCompositeFieldContainer[MethodDescriptorProto]
    name: str
    options: ServiceOptions
    def __init__(self, name: _Optional[str] = ..., method: _Optional[_Iterable[_Union[MethodDescriptorProto, _Mapping]]] = ..., options: _Optional[_Union[ServiceOptions, _Mapping]] = ...) -> None: ...

class ServiceOptions(_message.Message):
    __slots__ = ["deprecated", "uninterpreted_option"]
    DEPRECATED_FIELD_NUMBER: _ClassVar[int]
    Extensions: _python_message._ExtensionDict
    UNINTERPRETED_OPTION_FIELD_NUMBER: _ClassVar[int]
    deprecated: bool
    uninterpreted_option: _containers.RepeatedCompositeFieldContainer[UninterpretedOption]
    def __init__(self, deprecated: bool = ..., uninterpreted_option: _Optional[_Iterable[_Union[UninterpretedOption, _Mapping]]] = ...) -> None: ...

class SourceCodeInfo(_message.Message):
    __slots__ = ["location"]
    class Location(_message.Message):
        __slots__ = ["leading_comments", "leading_detached_comments", "path", "span", "trailing_comments"]
        LEADING_COMMENTS_FIELD_NUMBER: _ClassVar[int]
        LEADING_DETACHED_COMMENTS_FIELD_NUMBER: _ClassVar[int]
        PATH_FIELD_NUMBER: _ClassVar[int]
        SPAN_FIELD_NUMBER: _ClassVar[int]
        TRAILING_COMMENTS_FIELD_NUMBER: _ClassVar[int]
        leading_comments: str
        leading_detached_comments: _containers.RepeatedScalarFieldContainer[str]
        path: _containers.RepeatedScalarFieldContainer[int]
        span: _containers.RepeatedScalarFieldContainer[int]
        trailing_comments: str
        def __init__(self, path: _Optional[_Iterable[int]] = ..., span: _Optional[_Iterable[int]] = ..., leading_comments: _Optional[str] = ..., trailing_comments: _Optional[str] = ..., leading_detached_comments: _Optional[_Iterable[str]] = ...) -> None: ...
    LOCATION_FIELD_NUMBER: _ClassVar[int]
    location: _containers.RepeatedCompositeFieldContainer[SourceCodeInfo.Location]
    def __init__(self, location: _Optional[_Iterable[_Union[SourceCodeInfo.Location, _Mapping]]] = ...) -> None: ...

class UninterpretedOption(_message.Message):
    __slots__ = ["aggregate_value", "double_value", "identifier_value", "name", "negative_int_value", "positive_int_value", "string_value"]
    class NamePart(_message.Message):
        __slots__ = ["is_extension", "name_part"]
        IS_EXTENSION_FIELD_NUMBER: _ClassVar[int]
        NAME_PART_FIELD_NUMBER: _ClassVar[int]
        is_extension: bool
        name_part: str
        def __init__(self, name_part: _Optional[str] = ..., is_extension: bool = ...) -> None: ...
    AGGREGATE_VALUE_FIELD_NUMBER: _ClassVar[int]
    DOUBLE_VALUE_FIELD_NUMBER: _ClassVar[int]
    IDENTIFIER_VALUE_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    NEGATIVE_INT_VALUE_FIELD_NUMBER: _ClassVar[int]
    POSITIVE_INT_VALUE_FIELD_NUMBER: _ClassVar[int]
    STRING_VALUE_FIELD_NUMBER: _ClassVar[int]
    aggregate_value: str
    double_value: float
    identifier_value: str
    name: _containers.RepeatedCompositeFieldContainer[UninterpretedOption.NamePart]
    negative_int_value: int
    positive_int_value: int
    string_value: bytes
    def __init__(self, name: _Optional[_Iterable[_Union[UninterpretedOption.NamePart, _Mapping]]] = ..., identifier_value: _Optional[str] = ..., positive_int_value: _Optional[int] = ..., negative_int_value: _Optional[int] = ..., double_value: _Optional[float] = ..., string_value: _Optional[bytes] = ..., aggregate_value: _Optional[str] = ...) -> None: ...
