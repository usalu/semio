from geometry.v1 import geometry_pb2 as _geometry_pb2
from model.v1 import model_pb2 as _model_pb2
from extension.v1 import extension_pb2 as _extension_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class ConnectElementRequest(_message.Message):
    __slots__ = ["connected_sobject", "connecting_sobject", "connection"]
    CONNECTED_SOBJECT_FIELD_NUMBER: _ClassVar[int]
    CONNECTING_SOBJECT_FIELD_NUMBER: _ClassVar[int]
    CONNECTION_FIELD_NUMBER: _ClassVar[int]
    connected_sobject: _model_pb2.Sobject
    connecting_sobject: _model_pb2.Sobject
    connection: _model_pb2.Connection
    def __init__(self, connected_sobject: _Optional[_Union[_model_pb2.Sobject, _Mapping]] = ..., connecting_sobject: _Optional[_Union[_model_pb2.Sobject, _Mapping]] = ..., connection: _Optional[_Union[_model_pb2.Connection, _Mapping]] = ...) -> None: ...

class ConnectElementResponse(_message.Message):
    __slots__ = ["connected_element_pose", "connection_point"]
    CONNECTED_ELEMENT_POSE_FIELD_NUMBER: _ClassVar[int]
    CONNECTION_POINT_FIELD_NUMBER: _ClassVar[int]
    connected_element_pose: _model_pb2.Pose
    connection_point: _geometry_pb2.Point
    def __init__(self, connected_element_pose: _Optional[_Union[_model_pb2.Pose, _Mapping]] = ..., connection_point: _Optional[_Union[_geometry_pb2.Point, _Mapping]] = ...) -> None: ...

class GetRegisteredExtensionsRequest(_message.Message):
    __slots__ = []
    def __init__(self) -> None: ...

class PrototypeRequest(_message.Message):
    __slots__ = ["plan", "target_platform"]
    PLAN_FIELD_NUMBER: _ClassVar[int]
    TARGET_PLATFORM_FIELD_NUMBER: _ClassVar[int]
    plan: _model_pb2.Plan
    target_platform: _model_pb2.Platform
    def __init__(self, plan: _Optional[_Union[_model_pb2.Plan, _Mapping]] = ..., target_platform: _Optional[_Union[_model_pb2.Platform, str]] = ...) -> None: ...

class RegisterExtensionRequest(_message.Message):
    __slots__ = ["extending", "replace_existing"]
    EXTENDING_FIELD_NUMBER: _ClassVar[int]
    REPLACE_EXISTING_FIELD_NUMBER: _ClassVar[int]
    extending: _extension_pb2.Extending
    replace_existing: bool
    def __init__(self, extending: _Optional[_Union[_extension_pb2.Extending, _Mapping]] = ..., replace_existing: bool = ...) -> None: ...

class RegisterExtensionResponse(_message.Message):
    __slots__ = ["old_address", "success"]
    OLD_ADDRESS_FIELD_NUMBER: _ClassVar[int]
    SUCCESS_FIELD_NUMBER: _ClassVar[int]
    old_address: str
    success: bool
    def __init__(self, success: bool = ..., old_address: _Optional[str] = ...) -> None: ...

class RegisteredExtensionsResponse(_message.Message):
    __slots__ = ["extensions"]
    class ExtensionsEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: _extension_pb2.Extending
        def __init__(self, key: _Optional[str] = ..., value: _Optional[_Union[_extension_pb2.Extending, _Mapping]] = ...) -> None: ...
    EXTENSIONS_FIELD_NUMBER: _ClassVar[int]
    extensions: _containers.MessageMap[str, _extension_pb2.Extending]
    def __init__(self, extensions: _Optional[_Mapping[str, _extension_pb2.Extending]] = ...) -> None: ...
