from google.protobuf.internal import well_known_types as _well_known_types
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Optional as _Optional

DESCRIPTOR: _descriptor.FileDescriptor

class Duration(_message.Message, _well_known_types.Duration):
    __slots__ = ["nanos", "seconds"]
    NANOS_FIELD_NUMBER: _ClassVar[int]
    SECONDS_FIELD_NUMBER: _ClassVar[int]
    nanos: int
    seconds: int
    def __init__(self, seconds: _Optional[int] = ..., nanos: _Optional[int] = ...) -> None: ...
