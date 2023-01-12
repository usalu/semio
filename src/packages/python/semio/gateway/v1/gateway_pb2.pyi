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

class ConvertingService(_message.Message):
    __slots__ = ["source_type_url", "target_type_url"]
    SOURCE_TYPE_URL_FIELD_NUMBER: _ClassVar[int]
    TARGET_TYPE_URL_FIELD_NUMBER: _ClassVar[int]
    source_type_url: str
    target_type_url: str
    def __init__(self, source_type_url: _Optional[str] = ..., target_type_url: _Optional[str] = ...) -> None: ...

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

class GatewayServices(_message.Message):
    __slots__ = ["extendingServices", "managingService"]
    EXTENDINGSERVICES_FIELD_NUMBER: _ClassVar[int]
    MANAGINGSERVICE_FIELD_NUMBER: _ClassVar[int]
    extendingServices: _containers.RepeatedCompositeFieldContainer[ExtendingService]
    managingService: ManagingService
    def __init__(self, managingService: _Optional[_Union[ManagingService, _Mapping]] = ..., extendingServices: _Optional[_Iterable[_Union[ExtendingService, _Mapping]]] = ...) -> None: ...

class GetRegisteredServicesRequest(_message.Message):
    __slots__ = []
    def __init__(self) -> None: ...

class LayoutDesignRequest(_message.Message):
    __slots__ = ["layout", "target_type"]
    LAYOUT_FIELD_NUMBER: _ClassVar[int]
    TARGET_TYPE_FIELD_NUMBER: _ClassVar[int]
    layout: _model_pb2.Layout
    target_type: str
    def __init__(self, layout: _Optional[_Union[_model_pb2.Layout, _Mapping]] = ..., target_type: _Optional[str] = ...) -> None: ...

class ManagingService(_message.Message):
    __slots__ = ["address", "name"]
    ADDRESS_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    address: str
    name: str
    def __init__(self, name: _Optional[str] = ..., address: _Optional[str] = ...) -> None: ...

class ServiceRegistrationRequest(_message.Message):
    __slots__ = ["extendingService", "managingService", "replace_existing"]
    EXTENDINGSERVICE_FIELD_NUMBER: _ClassVar[int]
    MANAGINGSERVICE_FIELD_NUMBER: _ClassVar[int]
    REPLACE_EXISTING_FIELD_NUMBER: _ClassVar[int]
    extendingService: ExtendingService
    managingService: ManagingService
    replace_existing: bool
    def __init__(self, replace_existing: bool = ..., managingService: _Optional[_Union[ManagingService, _Mapping]] = ..., extendingService: _Optional[_Union[ExtendingService, _Mapping]] = ...) -> None: ...

class ServiceRegistrationResponse(_message.Message):
    __slots__ = ["old_address", "success"]
    OLD_ADDRESS_FIELD_NUMBER: _ClassVar[int]
    SUCCESS_FIELD_NUMBER: _ClassVar[int]
    old_address: str
    success: bool
    def __init__(self, success: bool = ..., old_address: _Optional[str] = ...) -> None: ...

class TransformingService(_message.Message):
    __slots__ = []
    def __init__(self) -> None: ...

class TranslatingService(_message.Message):
    __slots__ = ["platform_name"]
    PLATFORM_NAME_FIELD_NUMBER: _ClassVar[int]
    platform_name: str
    def __init__(self, platform_name: _Optional[str] = ...) -> None: ...
