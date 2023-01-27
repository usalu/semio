from model.v1 import model_pb2 as _model_pb2
from google.api import annotations_pb2 as _annotations_pb2
from protoc_gen_openapiv2.options import annotations_pb2 as _annotations_pb2_1
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class LayoutDesignRequest(_message.Message):
    __slots__ = ["layout", "target_platform"]
    LAYOUT_FIELD_NUMBER: _ClassVar[int]
    TARGET_PLATFORM_FIELD_NUMBER: _ClassVar[int]
    layout: _model_pb2.Layout
    target_platform: _model_pb2.Platform
    def __init__(self, layout: _Optional[_Union[_model_pb2.Layout, _Mapping]] = ..., target_platform: _Optional[_Union[_model_pb2.Platform, str]] = ...) -> None: ...
