from model.v1 import model_pb2 as _model_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class RewriteLayoutRequest(_message.Message):
    __slots__ = ["decisions", "initial_layout"]
    DECISIONS_FIELD_NUMBER: _ClassVar[int]
    INITIAL_LAYOUT_FIELD_NUMBER: _ClassVar[int]
    decisions: _containers.RepeatedCompositeFieldContainer[_model_pb2.Decision]
    initial_layout: _model_pb2.Layout
    def __init__(self, decisions: _Optional[_Iterable[_Union[_model_pb2.Decision, _Mapping]]] = ..., initial_layout: _Optional[_Union[_model_pb2.Layout, _Mapping]] = ...) -> None: ...
