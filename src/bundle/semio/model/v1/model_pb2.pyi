from semio.geometry.v1 import geometry_pb2 as _geometry_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DEPENDENCY_DEPENDENT: Dependency
DEPENDENCY_INDEPENDENT: Dependency
DEPENDENCY_POINTDEPENDENT: Dependency
DEPENDENCY_VIEWDEPENDENT: Dependency
DESCRIPTOR: _descriptor.FileDescriptor
ENCODING_TEXT_ASCII: Encoding
ENCODING_TEXT_BASE64: Encoding
ENCODING_TEXT_UFT16: Encoding
ENCODING_TEXT_UFT32: Encoding
ENCODING_TEXT_UFT8: Encoding
FILETYPE_C: FileType
FILETYPE_CPP: FileType
FILETYPE_CSHARP: FileType
FILETYPE_GO: FileType
FILETYPE_JSON: FileType
FILETYPE_NATIVE: FileType
FILETYPE_PY: FileType
FILETYPE_RUST: FileType
FILETYPE_STEP: FileType
FILETYPE_TOML: FileType
FILETYPE_XML: FileType
FILETYPE_YAML: FileType
LAYOUTSTRATEGY_BREADTHFIRST: LayoutStrategy
LAYOUTSTRATEGY_DEPTHFIRST: LayoutStrategy
PLATFORM_ARCHICAD: Platform
PLATFORM_CADQUERY: Platform
PLATFORM_CITYENGINE: Platform
PLATFORM_DYNAMO: Platform
PLATFORM_ENERGYPLUS: Platform
PLATFORM_EXCEL: Platform
PLATFORM_FORNJOT: Platform
PLATFORM_FREECAD: Platform
PLATFORM_GRASSHOPPER: Platform
PLATFORM_HYPAR: Platform
PLATFORM_IFCOPENSHELL: Platform
PLATFORM_JSCAD: Platform
PLATFORM_OPENSCAD: Platform
PLATFORM_OPENSTUDIO: Platform
PLATFORM_REVIT: Platform
PLATFORM_RHINO: Platform
PLATFORM_SEMIO: Platform
PLATFORM_SPECKLE: Platform
PLATFORM_SVERCHOK: Platform
PLATFORM_THREE: Platform
PLATFORM_TRUCK: Platform
REPRESENTATIONPROTOCOL_FULL: RepresentationProtocol
REPRESENTATIONPROTOCOL_NONE: RepresentationProtocol
REPRESENTATIONPROTOCOL_SIMPLE: RepresentationProtocol

class Assembly(_message.Message):
    __slots__ = ["parts", "sobject_id"]
    PARTS_FIELD_NUMBER: _ClassVar[int]
    SOBJECT_ID_FIELD_NUMBER: _ClassVar[int]
    parts: _containers.RepeatedCompositeFieldContainer[Assembly]
    sobject_id: str
    def __init__(self, sobject_id: _Optional[str] = ..., parts: _Optional[_Iterable[_Union[Assembly, _Mapping]]] = ...) -> None: ...

class Connectable(_message.Message):
    __slots__ = ["link", "sobject_id"]
    LINK_FIELD_NUMBER: _ClassVar[int]
    SOBJECT_ID_FIELD_NUMBER: _ClassVar[int]
    link: Link
    sobject_id: str
    def __init__(self, sobject_id: _Optional[str] = ..., link: _Optional[_Union[Link, _Mapping]] = ...) -> None: ...

class Connection(_message.Message):
    __slots__ = ["connected", "connecting"]
    CONNECTED_FIELD_NUMBER: _ClassVar[int]
    CONNECTING_FIELD_NUMBER: _ClassVar[int]
    connected: Connectable
    connecting: Connectable
    def __init__(self, connecting: _Optional[_Union[Connectable, _Mapping]] = ..., connected: _Optional[_Union[Connectable, _Mapping]] = ...) -> None: ...

class Decision(_message.Message):
    __slots__ = ["modification", "strategy"]
    MODIFICATION_FIELD_NUMBER: _ClassVar[int]
    STRATEGY_FIELD_NUMBER: _ClassVar[int]
    modification: LayoutModification
    strategy: LayoutModificationStrategy
    def __init__(self, modification: _Optional[_Union[LayoutModification, _Mapping]] = ..., strategy: _Optional[_Union[LayoutModificationStrategy, _Mapping]] = ...) -> None: ...

class Design(_message.Message):
    __slots__ = ["elements", "prototypes"]
    ELEMENTS_FIELD_NUMBER: _ClassVar[int]
    PROTOTYPES_FIELD_NUMBER: _ClassVar[int]
    elements: _containers.RepeatedCompositeFieldContainer[Element]
    prototypes: _containers.RepeatedCompositeFieldContainer[Prototype]
    def __init__(self, prototypes: _Optional[_Iterable[_Union[Prototype, _Mapping]]] = ..., elements: _Optional[_Iterable[_Union[Element, _Mapping]]] = ...) -> None: ...

class Element(_message.Message):
    __slots__ = ["pose", "prototype_plan_hash", "sobject_id"]
    POSE_FIELD_NUMBER: _ClassVar[int]
    PROTOTYPE_PLAN_HASH_FIELD_NUMBER: _ClassVar[int]
    SOBJECT_ID_FIELD_NUMBER: _ClassVar[int]
    pose: Pose
    prototype_plan_hash: str
    sobject_id: str
    def __init__(self, sobject_id: _Optional[str] = ..., prototype_plan_hash: _Optional[str] = ..., pose: _Optional[_Union[Pose, _Mapping]] = ...) -> None: ...

class Layout(_message.Message):
    __slots__ = ["assemblies", "connections", "sobjects", "strategy"]
    ASSEMBLIES_FIELD_NUMBER: _ClassVar[int]
    CONNECTIONS_FIELD_NUMBER: _ClassVar[int]
    SOBJECTS_FIELD_NUMBER: _ClassVar[int]
    STRATEGY_FIELD_NUMBER: _ClassVar[int]
    assemblies: _containers.RepeatedCompositeFieldContainer[Assembly]
    connections: _containers.RepeatedCompositeFieldContainer[Connection]
    sobjects: _containers.RepeatedCompositeFieldContainer[Sobject]
    strategy: LayoutStrategy
    def __init__(self, sobjects: _Optional[_Iterable[_Union[Sobject, _Mapping]]] = ..., connections: _Optional[_Iterable[_Union[Connection, _Mapping]]] = ..., strategy: _Optional[_Union[LayoutStrategy, str]] = ..., assemblies: _Optional[_Iterable[_Union[Assembly, _Mapping]]] = ...) -> None: ...

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

class Link(_message.Message):
    __slots__ = ["bias_parameters", "ports", "representationProtocol"]
    BIAS_PARAMETERS_FIELD_NUMBER: _ClassVar[int]
    PORTS_FIELD_NUMBER: _ClassVar[int]
    REPRESENTATIONPROTOCOL_FIELD_NUMBER: _ClassVar[int]
    bias_parameters: _containers.RepeatedCompositeFieldContainer[Parameter]
    ports: _containers.RepeatedScalarFieldContainer[str]
    representationProtocol: RepresentationProtocol
    def __init__(self, representationProtocol: _Optional[_Union[RepresentationProtocol, str]] = ..., ports: _Optional[_Iterable[str]] = ..., bias_parameters: _Optional[_Iterable[_Union[Parameter, _Mapping]]] = ...) -> None: ...

class Parameter(_message.Message):
    __slots__ = ["context", "name", "value"]
    CONTEXT_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    VALUE_FIELD_NUMBER: _ClassVar[int]
    context: _containers.RepeatedCompositeFieldContainer[Scope]
    name: str
    value: Value
    def __init__(self, name: _Optional[str] = ..., context: _Optional[_Iterable[_Union[Scope, _Mapping]]] = ..., value: _Optional[_Union[Value, _Mapping]] = ...) -> None: ...

class Plan(_message.Message):
    __slots__ = ["parameters", "uri"]
    PARAMETERS_FIELD_NUMBER: _ClassVar[int]
    URI_FIELD_NUMBER: _ClassVar[int]
    parameters: _containers.RepeatedCompositeFieldContainer[Parameter]
    uri: str
    def __init__(self, uri: _Optional[str] = ..., parameters: _Optional[_Iterable[_Union[Parameter, _Mapping]]] = ...) -> None: ...

class Pose(_message.Message):
    __slots__ = ["point_of_view", "view"]
    POINT_OF_VIEW_FIELD_NUMBER: _ClassVar[int]
    VIEW_FIELD_NUMBER: _ClassVar[int]
    point_of_view: _geometry_pb2.Point
    view: _geometry_pb2.Quaternion
    def __init__(self, point_of_view: _Optional[_Union[_geometry_pb2.Point, _Mapping]] = ..., view: _Optional[_Union[_geometry_pb2.Quaternion, _Mapping]] = ...) -> None: ...

class Prototype(_message.Message):
    __slots__ = ["description", "plan_hash", "representations"]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    PLAN_HASH_FIELD_NUMBER: _ClassVar[int]
    REPRESENTATIONS_FIELD_NUMBER: _ClassVar[int]
    description: str
    plan_hash: str
    representations: _containers.RepeatedCompositeFieldContainer[Representation]
    def __init__(self, plan_hash: _Optional[str] = ..., representations: _Optional[_Iterable[_Union[Representation, _Mapping]]] = ..., description: _Optional[str] = ...) -> None: ...

class Representation(_message.Message):
    __slots__ = ["body", "concepts", "description", "encoding", "file_type", "lod", "platform"]
    BODY_FIELD_NUMBER: _ClassVar[int]
    CONCEPTS_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    ENCODING_FIELD_NUMBER: _ClassVar[int]
    FILE_TYPE_FIELD_NUMBER: _ClassVar[int]
    LOD_FIELD_NUMBER: _ClassVar[int]
    PLATFORM_FIELD_NUMBER: _ClassVar[int]
    body: bytes
    concepts: _containers.RepeatedScalarFieldContainer[str]
    description: str
    encoding: Encoding
    file_type: FileType
    lod: int
    platform: Platform
    def __init__(self, body: _Optional[bytes] = ..., encoding: _Optional[_Union[Encoding, str]] = ..., file_type: _Optional[_Union[FileType, str]] = ..., platform: _Optional[_Union[Platform, str]] = ..., description: _Optional[str] = ..., concepts: _Optional[_Iterable[str]] = ..., lod: _Optional[int] = ...) -> None: ...

class Scope(_message.Message):
    __slots__ = ["concept", "order"]
    CONCEPT_FIELD_NUMBER: _ClassVar[int]
    ORDER_FIELD_NUMBER: _ClassVar[int]
    concept: str
    order: int
    def __init__(self, concept: _Optional[str] = ..., order: _Optional[int] = ...) -> None: ...

class Sobject(_message.Message):
    __slots__ = ["concepts", "id", "plan", "pose"]
    CONCEPTS_FIELD_NUMBER: _ClassVar[int]
    ID_FIELD_NUMBER: _ClassVar[int]
    PLAN_FIELD_NUMBER: _ClassVar[int]
    POSE_FIELD_NUMBER: _ClassVar[int]
    concepts: _containers.RepeatedScalarFieldContainer[str]
    id: str
    plan: Plan
    pose: Pose
    def __init__(self, id: _Optional[str] = ..., pose: _Optional[_Union[Pose, _Mapping]] = ..., plan: _Optional[_Union[Plan, _Mapping]] = ..., concepts: _Optional[_Iterable[str]] = ...) -> None: ...

class Value(_message.Message):
    __slots__ = ["integer_number", "natural_number", "number", "point", "text"]
    INTEGER_NUMBER_FIELD_NUMBER: _ClassVar[int]
    NATURAL_NUMBER_FIELD_NUMBER: _ClassVar[int]
    NUMBER_FIELD_NUMBER: _ClassVar[int]
    POINT_FIELD_NUMBER: _ClassVar[int]
    TEXT_FIELD_NUMBER: _ClassVar[int]
    integer_number: int
    natural_number: int
    number: float
    point: _geometry_pb2.Point
    text: str
    def __init__(self, text: _Optional[str] = ..., number: _Optional[float] = ..., integer_number: _Optional[int] = ..., natural_number: _Optional[int] = ..., point: _Optional[_Union[_geometry_pb2.Point, _Mapping]] = ...) -> None: ...

class Encoding(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []

class FileType(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []

class Platform(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []

class RepresentationProtocol(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []

class Dependency(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []

class LayoutStrategy(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []