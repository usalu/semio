from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor
LAYOUTSTRATEGY_BREADTHFIRST: LayoutStrategy
LAYOUTSTRATEGY_DEPTHFIRST: LayoutStrategy
REPRESENTATIONPROTOCOL_FULL: RepresentationProtocol
REPRESENTATIONPROTOCOL_NONE: RepresentationProtocol
REPRESENTATIONPROTOCOL_SIMPLE: RepresentationProtocol

class Attraction(_message.Message):
    __slots__ = ["attracted", "attractor", "id"]
    ATTRACTED_FIELD_NUMBER: _ClassVar[int]
    ATTRACTOR_FIELD_NUMBER: _ClassVar[int]
    ID_FIELD_NUMBER: _ClassVar[int]
    attracted: AttractionParticipant
    attractor: AttractionParticipant
    id: str
    def __init__(self, id: _Optional[str] = ..., attractor: _Optional[_Union[AttractionParticipant, _Mapping]] = ..., attracted: _Optional[_Union[AttractionParticipant, _Mapping]] = ...) -> None: ...

class AttractionParticipant(_message.Message):
    __slots__ = ["bias", "patricipant_id", "ports", "representationProtocol"]
    class BiasEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    BIAS_FIELD_NUMBER: _ClassVar[int]
    PATRICIPANT_ID_FIELD_NUMBER: _ClassVar[int]
    PORTS_FIELD_NUMBER: _ClassVar[int]
    REPRESENTATIONPROTOCOL_FIELD_NUMBER: _ClassVar[int]
    bias: _containers.ScalarMap[str, str]
    patricipant_id: str
    ports: _containers.RepeatedScalarFieldContainer[str]
    representationProtocol: RepresentationProtocol
    def __init__(self, patricipant_id: _Optional[str] = ..., representationProtocol: _Optional[_Union[RepresentationProtocol, str]] = ..., ports: _Optional[_Iterable[str]] = ..., bias: _Optional[_Mapping[str, str]] = ...) -> None: ...

class AttractionTree(_message.Message):
    __slots__ = ["attraction_id", "children"]
    ATTRACTION_ID_FIELD_NUMBER: _ClassVar[int]
    CHILDREN_FIELD_NUMBER: _ClassVar[int]
    attraction_id: str
    children: _containers.RepeatedCompositeFieldContainer[AttractionTree]
    def __init__(self, attraction_id: _Optional[str] = ..., children: _Optional[_Iterable[_Union[AttractionTree, _Mapping]]] = ...) -> None: ...

class Decision(_message.Message):
    __slots__ = ["modification", "strategy"]
    MODIFICATION_FIELD_NUMBER: _ClassVar[int]
    STRATEGY_FIELD_NUMBER: _ClassVar[int]
    modification: LayoutModification
    strategy: LayoutModificationStrategy
    def __init__(self, modification: _Optional[_Union[LayoutModification, _Mapping]] = ..., strategy: _Optional[_Union[LayoutModificationStrategy, _Mapping]] = ...) -> None: ...

class Design(_message.Message):
    __slots__ = ["elements"]
    ELEMENTS_FIELD_NUMBER: _ClassVar[int]
    elements: _containers.RepeatedCompositeFieldContainer[Element]
    def __init__(self, elements: _Optional[_Iterable[_Union[Element, _Mapping]]] = ...) -> None: ...

class Element(_message.Message):
    __slots__ = ["pose", "representations"]
    POSE_FIELD_NUMBER: _ClassVar[int]
    REPRESENTATIONS_FIELD_NUMBER: _ClassVar[int]
    pose: Pose
    representations: _containers.RepeatedCompositeFieldContainer[Representation]
    def __init__(self, pose: _Optional[_Union[Pose, _Mapping]] = ..., representations: _Optional[_Iterable[_Union[Representation, _Mapping]]] = ...) -> None: ...

class Layout(_message.Message):
    __slots__ = ["attractionTrees", "attractions", "root_sobject_id", "sobjects", "stragegy"]
    ATTRACTIONS_FIELD_NUMBER: _ClassVar[int]
    ATTRACTIONTREES_FIELD_NUMBER: _ClassVar[int]
    ROOT_SOBJECT_ID_FIELD_NUMBER: _ClassVar[int]
    SOBJECTS_FIELD_NUMBER: _ClassVar[int]
    STRAGEGY_FIELD_NUMBER: _ClassVar[int]
    attractionTrees: _containers.RepeatedCompositeFieldContainer[AttractionTree]
    attractions: _containers.RepeatedCompositeFieldContainer[Attraction]
    root_sobject_id: str
    sobjects: _containers.RepeatedCompositeFieldContainer[Sobject]
    stragegy: LayoutStrategy
    def __init__(self, sobjects: _Optional[_Iterable[_Union[Sobject, _Mapping]]] = ..., attractions: _Optional[_Iterable[_Union[Attraction, _Mapping]]] = ..., root_sobject_id: _Optional[str] = ..., stragegy: _Optional[_Union[LayoutStrategy, str]] = ..., attractionTrees: _Optional[_Iterable[_Union[AttractionTree, _Mapping]]] = ...) -> None: ...

class LayoutModification(_message.Message):
    __slots__ = ["context", "modified_context"]
    CONTEXT_FIELD_NUMBER: _ClassVar[int]
    MODIFIED_CONTEXT_FIELD_NUMBER: _ClassVar[int]
    context: Layout
    modified_context: Layout
    def __init__(self, context: _Optional[_Union[Layout, _Mapping]] = ..., modified_context: _Optional[_Union[Layout, _Mapping]] = ...) -> None: ...

class LayoutModificationStrategy(_message.Message):
    __slots__ = ["match_count"]
    MATCH_COUNT_FIELD_NUMBER: _ClassVar[int]
    match_count: int
    def __init__(self, match_count: _Optional[int] = ...) -> None: ...

class Point(_message.Message):
    __slots__ = ["x", "y", "z"]
    X_FIELD_NUMBER: _ClassVar[int]
    Y_FIELD_NUMBER: _ClassVar[int]
    Z_FIELD_NUMBER: _ClassVar[int]
    x: float
    y: float
    z: float
    def __init__(self, x: _Optional[float] = ..., y: _Optional[float] = ..., z: _Optional[float] = ...) -> None: ...

class Pose(_message.Message):
    __slots__ = ["point_of_view", "view"]
    POINT_OF_VIEW_FIELD_NUMBER: _ClassVar[int]
    VIEW_FIELD_NUMBER: _ClassVar[int]
    point_of_view: Point
    view: Quaternion
    def __init__(self, point_of_view: _Optional[_Union[Point, _Mapping]] = ..., view: _Optional[_Union[Quaternion, _Mapping]] = ...) -> None: ...

class Quaternion(_message.Message):
    __slots__ = ["w", "x", "y", "z"]
    W_FIELD_NUMBER: _ClassVar[int]
    X_FIELD_NUMBER: _ClassVar[int]
    Y_FIELD_NUMBER: _ClassVar[int]
    Z_FIELD_NUMBER: _ClassVar[int]
    w: float
    x: float
    y: float
    z: float
    def __init__(self, w: _Optional[float] = ..., x: _Optional[float] = ..., y: _Optional[float] = ..., z: _Optional[float] = ...) -> None: ...

class Representation(_message.Message):
    __slots__ = ["byteArray", "lod", "name", "text", "type"]
    BYTEARRAY_FIELD_NUMBER: _ClassVar[int]
    LOD_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    TEXT_FIELD_NUMBER: _ClassVar[int]
    TYPE_FIELD_NUMBER: _ClassVar[int]
    byteArray: bytes
    lod: int
    name: str
    text: str
    type: str
    def __init__(self, type: _Optional[str] = ..., name: _Optional[str] = ..., lod: _Optional[int] = ..., byteArray: _Optional[bytes] = ..., text: _Optional[str] = ...) -> None: ...

class Sobject(_message.Message):
    __slots__ = ["id", "parameters", "pose", "url"]
    class ParametersEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    ID_FIELD_NUMBER: _ClassVar[int]
    PARAMETERS_FIELD_NUMBER: _ClassVar[int]
    POSE_FIELD_NUMBER: _ClassVar[int]
    URL_FIELD_NUMBER: _ClassVar[int]
    id: str
    parameters: _containers.ScalarMap[str, str]
    pose: Pose
    url: str
    def __init__(self, id: _Optional[str] = ..., url: _Optional[str] = ..., pose: _Optional[_Union[Pose, _Mapping]] = ..., parameters: _Optional[_Mapping[str, str]] = ...) -> None: ...

class RepresentationProtocol(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []

class LayoutStrategy(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []
