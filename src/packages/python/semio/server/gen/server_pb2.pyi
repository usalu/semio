import model_pb2 as _model_pb2
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
