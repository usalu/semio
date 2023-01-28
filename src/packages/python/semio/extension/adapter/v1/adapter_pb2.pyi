from model.v1 import model_pb2 as _model_pb2
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class Adapting(_message.Message):
    __slots__ = ["platform"]
    PLATFORM_FIELD_NUMBER: _ClassVar[int]
    platform: _model_pb2.Platform
    def __init__(self, platform: _Optional[_Union[_model_pb2.Platform, str]] = ...) -> None: ...

class ConnectionPointRequest(_message.Message):
    __slots__ = ["connected_plan", "connecting_link"]
    CONNECTED_PLAN_FIELD_NUMBER: _ClassVar[int]
    CONNECTING_LINK_FIELD_NUMBER: _ClassVar[int]
    connected_plan: _model_pb2.Plan
    connecting_link: _model_pb2.Link
    def __init__(self, connected_plan: _Optional[_Union[_model_pb2.Plan, _Mapping]] = ..., connecting_link: _Optional[_Union[_model_pb2.Link, _Mapping]] = ...) -> None: ...
