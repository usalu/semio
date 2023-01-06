from model.v1 import model_pb2 as _model_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class AttractionPointRequest(_message.Message):
    __slots__ = ["attracted_attractionStrategy", "attractor_url"]
    ATTRACTED_ATTRACTIONSTRATEGY_FIELD_NUMBER: _ClassVar[int]
    ATTRACTOR_URL_FIELD_NUMBER: _ClassVar[int]
    attracted_attractionStrategy: _model_pb2.AttractionStragegy
    attractor_url: str
    def __init__(self, attractor_url: _Optional[str] = ..., attracted_attractionStrategy: _Optional[_Union[_model_pb2.AttractionStragegy, _Mapping]] = ...) -> None: ...

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
