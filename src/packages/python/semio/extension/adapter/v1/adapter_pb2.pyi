from model.v1 import model_pb2 as _model_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class Adapting(_message.Message):
    __slots__ = ["platform_url"]
    PLATFORM_URL_FIELD_NUMBER: _ClassVar[int]
    platform_url: str
    def __init__(self, platform_url: _Optional[str] = ...) -> None: ...

class AttractionPointRequest(_message.Message):
    __slots__ = ["bias", "full_representation", "parameters", "simple_representation", "url"]
    class BiasEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    class ParametersEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    BIAS_FIELD_NUMBER: _ClassVar[int]
    FULL_REPRESENTATION_FIELD_NUMBER: _ClassVar[int]
    PARAMETERS_FIELD_NUMBER: _ClassVar[int]
    SIMPLE_REPRESENTATION_FIELD_NUMBER: _ClassVar[int]
    URL_FIELD_NUMBER: _ClassVar[int]
    bias: _containers.ScalarMap[str, str]
    full_representation: _model_pb2.Representation
    parameters: _containers.ScalarMap[str, str]
    simple_representation: _model_pb2.Point
    url: str
    def __init__(self, url: _Optional[str] = ..., parameters: _Optional[_Mapping[str, str]] = ..., bias: _Optional[_Mapping[str, str]] = ..., simple_representation: _Optional[_Union[_model_pb2.Point, _Mapping]] = ..., full_representation: _Optional[_Union[_model_pb2.Representation, _Mapping]] = ...) -> None: ...

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

class RepresentationsResponse(_message.Message):
    __slots__ = ["representations"]
    REPRESENTATIONS_FIELD_NUMBER: _ClassVar[int]
    representations: _containers.RepeatedCompositeFieldContainer[_model_pb2.Representation]
    def __init__(self, representations: _Optional[_Iterable[_Union[_model_pb2.Representation, _Mapping]]] = ...) -> None: ...
