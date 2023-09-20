from semio.geometry.v1 import geometry_pb2 as _geometry_pb2
from semio.model.v1 import model_pb2 as _model_pb2
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
    __slots__ = ["full_representation", "link", "plan", "simple_representation"]
    FULL_REPRESENTATION_FIELD_NUMBER: _ClassVar[int]
    LINK_FIELD_NUMBER: _ClassVar[int]
    PLAN_FIELD_NUMBER: _ClassVar[int]
    SIMPLE_REPRESENTATION_FIELD_NUMBER: _ClassVar[int]
    full_representation: _model_pb2.Representation
    link: _model_pb2.Link
    plan: _model_pb2.Plan
    simple_representation: _geometry_pb2.Point
    def __init__(self, plan: _Optional[_Union[_model_pb2.Plan, _Mapping]] = ..., link: _Optional[_Union[_model_pb2.Link, _Mapping]] = ..., simple_representation: _Optional[_Union[_geometry_pb2.Point, _Mapping]] = ..., full_representation: _Optional[_Union[_model_pb2.Representation, _Mapping]] = ...) -> None: ...
