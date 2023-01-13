from model.v1 import model_pb2 as _model_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class AdaptingService(_message.Message):
    __slots__ = ["platform_name"]
    PLATFORM_NAME_FIELD_NUMBER: _ClassVar[int]
    platform_name: str
    def __init__(self, platform_name: _Optional[str] = ...) -> None: ...

class AttractionRequest(_message.Message):
    __slots__ = ["attraction", "target_type_url"]
    ATTRACTION_FIELD_NUMBER: _ClassVar[int]
    TARGET_TYPE_URL_FIELD_NUMBER: _ClassVar[int]
    attraction: _model_pb2.Attraction
    target_type_url: str
    def __init__(self, attraction: _Optional[_Union[_model_pb2.Attraction, _Mapping]] = ..., target_type_url: _Optional[str] = ...) -> None: ...

class AttractionResponse(_message.Message):
    __slots__ = ["attracted_pose", "attraction_point"]
    ATTRACTED_POSE_FIELD_NUMBER: _ClassVar[int]
    ATTRACTION_POINT_FIELD_NUMBER: _ClassVar[int]
    attracted_pose: _model_pb2.Pose
    attraction_point: _model_pb2.Point
    def __init__(self, attracted_pose: _Optional[_Union[_model_pb2.Pose, _Mapping]] = ..., attraction_point: _Optional[_Union[_model_pb2.Point, _Mapping]] = ...) -> None: ...

class ConvertingService(_message.Message):
    __slots__ = ["source_type_url", "target_type_url"]
    SOURCE_TYPE_URL_FIELD_NUMBER: _ClassVar[int]
    TARGET_TYPE_URL_FIELD_NUMBER: _ClassVar[int]
    source_type_url: str
    target_type_url: str
    def __init__(self, source_type_url: _Optional[str] = ..., target_type_url: _Optional[str] = ...) -> None: ...

class ElementRequest(_message.Message):
    __slots__ = ["sobject", "target_type_url"]
    SOBJECT_FIELD_NUMBER: _ClassVar[int]
    TARGET_TYPE_URL_FIELD_NUMBER: _ClassVar[int]
    sobject: _model_pb2.Sobject
    target_type_url: str
    def __init__(self, sobject: _Optional[_Union[_model_pb2.Sobject, _Mapping]] = ..., target_type_url: _Optional[str] = ...) -> None: ...

class ExtendingService(_message.Message):
    __slots__ = ["adaptingServices", "address", "convertingServices", "name", "transformingServices", "translatingServices"]
    ADAPTINGSERVICES_FIELD_NUMBER: _ClassVar[int]
    ADDRESS_FIELD_NUMBER: _ClassVar[int]
    CONVERTINGSERVICES_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    TRANSFORMINGSERVICES_FIELD_NUMBER: _ClassVar[int]
    TRANSLATINGSERVICES_FIELD_NUMBER: _ClassVar[int]
    adaptingServices: _containers.RepeatedCompositeFieldContainer[AdaptingService]
    address: str
    convertingServices: _containers.RepeatedCompositeFieldContainer[ConvertingService]
    name: str
    transformingServices: _containers.RepeatedCompositeFieldContainer[TransformingService]
    translatingServices: _containers.RepeatedCompositeFieldContainer[TranslatingService]
    def __init__(self, name: _Optional[str] = ..., address: _Optional[str] = ..., adaptingServices: _Optional[_Iterable[_Union[AdaptingService, _Mapping]]] = ..., convertingServices: _Optional[_Iterable[_Union[ConvertingService, _Mapping]]] = ..., transformingServices: _Optional[_Iterable[_Union[TransformingService, _Mapping]]] = ..., translatingServices: _Optional[_Iterable[_Union[TranslatingService, _Mapping]]] = ...) -> None: ...

class ExtendingServices(_message.Message):
    __slots__ = ["extendingServices"]
    EXTENDINGSERVICES_FIELD_NUMBER: _ClassVar[int]
    extendingServices: _containers.RepeatedCompositeFieldContainer[ExtendingService]
    def __init__(self, extendingServices: _Optional[_Iterable[_Union[ExtendingService, _Mapping]]] = ...) -> None: ...

class ExtensionRegistrationRequest(_message.Message):
    __slots__ = ["extendingService", "replace_existing"]
    EXTENDINGSERVICE_FIELD_NUMBER: _ClassVar[int]
    REPLACE_EXISTING_FIELD_NUMBER: _ClassVar[int]
    extendingService: ExtendingService
    replace_existing: bool
    def __init__(self, replace_existing: bool = ..., extendingService: _Optional[_Union[ExtendingService, _Mapping]] = ...) -> None: ...

class ExtensionRegistrationResponse(_message.Message):
    __slots__ = ["old_address", "success"]
    OLD_ADDRESS_FIELD_NUMBER: _ClassVar[int]
    SUCCESS_FIELD_NUMBER: _ClassVar[int]
    old_address: str
    success: bool
    def __init__(self, success: bool = ..., old_address: _Optional[str] = ...) -> None: ...

class GetRegisteredExtensionsRequest(_message.Message):
    __slots__ = []
    def __init__(self) -> None: ...

class TransformingService(_message.Message):
    __slots__ = []
    def __init__(self) -> None: ...

class TranslatingService(_message.Message):
    __slots__ = ["platform_name"]
    PLATFORM_NAME_FIELD_NUMBER: _ClassVar[int]
    platform_name: str
    def __init__(self, platform_name: _Optional[str] = ...) -> None: ...
