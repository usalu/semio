from model.v1 import model_pb2 as _model_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class Converting(_message.Message):
    __slots__ = ["source_platform", "target_platform"]
    SOURCE_PLATFORM_FIELD_NUMBER: _ClassVar[int]
    TARGET_PLATFORM_FIELD_NUMBER: _ClassVar[int]
    source_platform: _model_pb2.Platform
    target_platform: _model_pb2.Platform
    def __init__(self, source_platform: _Optional[_Union[_model_pb2.Platform, str]] = ..., target_platform: _Optional[_Union[_model_pb2.Platform, str]] = ...) -> None: ...

class RepresentationConversionRequest(_message.Message):
    __slots__ = ["options", "representation", "target_platform"]
    class OptionsEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    OPTIONS_FIELD_NUMBER: _ClassVar[int]
    REPRESENTATION_FIELD_NUMBER: _ClassVar[int]
    TARGET_PLATFORM_FIELD_NUMBER: _ClassVar[int]
    options: _containers.ScalarMap[str, str]
    representation: _model_pb2.Representation
    target_platform: _model_pb2.Platform
    def __init__(self, representation: _Optional[_Union[_model_pb2.Representation, _Mapping]] = ..., target_platform: _Optional[_Union[_model_pb2.Platform, str]] = ..., options: _Optional[_Mapping[str, str]] = ...) -> None: ...
