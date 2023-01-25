from model.v1 import model_pb2 as _model_pb2
from extension.v1 import extension_pb2 as _extension_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class ConnectionRequest(_message.Message):
    __slots__ = ["connection"]
    CONNECTION_FIELD_NUMBER: _ClassVar[int]
    connection: _model_pb2.Connection
    def __init__(self, connection: _Optional[_Union[_model_pb2.Connection, _Mapping]] = ...) -> None: ...

class ConnectionResponse(_message.Message):
    __slots__ = ["connected_pose", "connection_point"]
    CONNECTED_POSE_FIELD_NUMBER: _ClassVar[int]
    CONNECTION_POINT_FIELD_NUMBER: _ClassVar[int]
    connected_pose: _model_pb2.Pose
    connection_point: _model_pb2.Point
    def __init__(self, connected_pose: _Optional[_Union[_model_pb2.Pose, _Mapping]] = ..., connection_point: _Optional[_Union[_model_pb2.Point, _Mapping]] = ...) -> None: ...

class ElementRequest(_message.Message):
    __slots__ = ["sobject", "target_representation_lod", "target_representation_name", "target_representation_platform", "targets_required"]
    SOBJECT_FIELD_NUMBER: _ClassVar[int]
    TARGETS_REQUIRED_FIELD_NUMBER: _ClassVar[int]
    TARGET_REPRESENTATION_LOD_FIELD_NUMBER: _ClassVar[int]
    TARGET_REPRESENTATION_NAME_FIELD_NUMBER: _ClassVar[int]
    TARGET_REPRESENTATION_PLATFORM_FIELD_NUMBER: _ClassVar[int]
    sobject: _model_pb2.Sobject
    target_representation_lod: int
    target_representation_name: str
    target_representation_platform: _model_pb2.Platform
    targets_required: bool
    def __init__(self, sobject: _Optional[_Union[_model_pb2.Sobject, _Mapping]] = ..., target_representation_platform: _Optional[_Union[_model_pb2.Platform, str]] = ..., target_representation_name: _Optional[str] = ..., target_representation_lod: _Optional[int] = ..., targets_required: bool = ...) -> None: ...

class ExtensionRegistrationRequest(_message.Message):
    __slots__ = ["address", "extending", "name", "replace_existing"]
    ADDRESS_FIELD_NUMBER: _ClassVar[int]
    EXTENDING_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    REPLACE_EXISTING_FIELD_NUMBER: _ClassVar[int]
    address: str
    extending: _extension_pb2.Extending
    name: str
    replace_existing: bool
    def __init__(self, address: _Optional[str] = ..., name: _Optional[str] = ..., extending: _Optional[_Union[_extension_pb2.Extending, _Mapping]] = ..., replace_existing: bool = ...) -> None: ...

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
