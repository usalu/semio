from model.v1 import model_pb2 as _model_pb2
from google.api import annotations_pb2 as _annotations_pb2
from google.api import client_pb2 as _client_pb2
from google.api import resource_pb2 as _resource_pb2
from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class LayoutDesignRequest(_message.Message):
    __slots__ = ["layout", "target_type"]
    LAYOUT_FIELD_NUMBER: _ClassVar[int]
    TARGET_TYPE_FIELD_NUMBER: _ClassVar[int]
    layout: _model_pb2.Layout
    target_type: str
    def __init__(self, layout: _Optional[_Union[_model_pb2.Layout, _Mapping]] = ..., target_type: _Optional[str] = ...) -> None: ...
