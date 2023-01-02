import model_pb2 as _model_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class AttractionResponse(_message.Message):
    __slots__ = ["attracted_target_pose", "attraction_point"]
    ATTRACTED_TARGET_POSE_FIELD_NUMBER: _ClassVar[int]
    ATTRACTION_POINT_FIELD_NUMBER: _ClassVar[int]
    attracted_target_pose: _model_pb2.Pose
    attraction_point: _model_pb2.Point
    def __init__(self, attracted_target_pose: _Optional[_Union[_model_pb2.Pose, _Mapping]] = ..., attraction_point: _Optional[_Union[_model_pb2.Point, _Mapping]] = ...) -> None: ...

class RepresentationRequest(_message.Message):
    __slots__ = ["lod", "name", "sobject", "type"]
    LOD_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    SOBJECT_FIELD_NUMBER: _ClassVar[int]
    TYPE_FIELD_NUMBER: _ClassVar[int]
    lod: int
    name: str
    sobject: _model_pb2.Sobject
    type: str
    def __init__(self, sobject: _Optional[_Union[_model_pb2.Sobject, _Mapping]] = ..., type: _Optional[str] = ..., name: _Optional[str] = ..., lod: _Optional[int] = ...) -> None: ...

class RepresentationsRequest(_message.Message):
    __slots__ = ["lods", "names", "sobject", "types"]
    LODS_FIELD_NUMBER: _ClassVar[int]
    NAMES_FIELD_NUMBER: _ClassVar[int]
    SOBJECT_FIELD_NUMBER: _ClassVar[int]
    TYPES_FIELD_NUMBER: _ClassVar[int]
    lods: _containers.RepeatedScalarFieldContainer[int]
    names: _containers.RepeatedScalarFieldContainer[str]
    sobject: _model_pb2.Sobject
    types: _containers.RepeatedScalarFieldContainer[str]
    def __init__(self, sobject: _Optional[_Union[_model_pb2.Sobject, _Mapping]] = ..., types: _Optional[_Iterable[str]] = ..., names: _Optional[_Iterable[str]] = ..., lods: _Optional[_Iterable[int]] = ...) -> None: ...
