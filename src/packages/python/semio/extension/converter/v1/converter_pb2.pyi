from model.v1 import model_pb2 as _model_pb2
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
    __slots__ = ["representation", "target_platform"]
    REPRESENTATION_FIELD_NUMBER: _ClassVar[int]
    TARGET_PLATFORM_FIELD_NUMBER: _ClassVar[int]
    representation: _model_pb2.Representation
    target_platform: _model_pb2.Platform
    def __init__(self, representation: _Optional[_Union[_model_pb2.Representation, _Mapping]] = ..., target_platform: _Optional[_Union[_model_pb2.Platform, str]] = ...) -> None: ...
