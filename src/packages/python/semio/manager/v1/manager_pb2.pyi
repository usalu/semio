from model.v1 import model_pb2 as _model_pb2
from extension.v1 import extension_pb2 as _extension_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

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
    connection_point: _model_pb2.Point
    def __init__(self, connected_element_pose: _Optional[_Union[_model_pb2.Pose, _Mapping]] = ..., connection_point: _Optional[_Union[_model_pb2.Point, _Mapping]] = ...) -> None: ...

class ElementRequest(_message.Message):
    __slots__ = ["sobject", "target_representation_concepts", "target_representation_lods", "target_representation_platforms", "targets_required"]
    SOBJECT_FIELD_NUMBER: _ClassVar[int]
    TARGETS_REQUIRED_FIELD_NUMBER: _ClassVar[int]
    TARGET_REPRESENTATION_CONCEPTS_FIELD_NUMBER: _ClassVar[int]
    TARGET_REPRESENTATION_LODS_FIELD_NUMBER: _ClassVar[int]
    TARGET_REPRESENTATION_PLATFORMS_FIELD_NUMBER: _ClassVar[int]
    sobject: _model_pb2.Sobject
    target_representation_concepts: _containers.RepeatedScalarFieldContainer[str]
    target_representation_lods: _containers.RepeatedScalarFieldContainer[int]
    target_representation_platforms: _containers.RepeatedScalarFieldContainer[_model_pb2.Platform]
    targets_required: bool
    def __init__(self, sobject: _Optional[_Union[_model_pb2.Sobject, _Mapping]] = ..., target_representation_platforms: _Optional[_Iterable[_Union[_model_pb2.Platform, str]]] = ..., target_representation_concepts: _Optional[_Iterable[str]] = ..., target_representation_lods: _Optional[_Iterable[int]] = ..., targets_required: bool = ...) -> None: ...

class GetRegisteredExtensionsRequest(_message.Message):
    __slots__ = []
    def __init__(self) -> None: ...

class RegisterExtensionRequest(_message.Message):
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
