from model.v1 import model_pb2 as _model_pb2
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class TranslateRepresentationRequest(_message.Message):
    __slots__ = ["representation", "source_pose", "target_pose"]
    REPRESENTATION_FIELD_NUMBER: _ClassVar[int]
    SOURCE_POSE_FIELD_NUMBER: _ClassVar[int]
    TARGET_POSE_FIELD_NUMBER: _ClassVar[int]
    representation: _model_pb2.Representation
    source_pose: _model_pb2.Pose
    target_pose: _model_pb2.Pose
    def __init__(self, representation: _Optional[_Union[_model_pb2.Representation, _Mapping]] = ..., target_pose: _Optional[_Union[_model_pb2.Pose, _Mapping]] = ..., source_pose: _Optional[_Union[_model_pb2.Pose, _Mapping]] = ...) -> None: ...
