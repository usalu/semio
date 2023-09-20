from semio.model.v1 import model_pb2 as _model_pb2
from semio.extension.adapter.v1 import adapter_pb2 as _adapter_pb2
from semio.extension.converter.v1 import converter_pb2 as _converter_pb2
from semio.extension.transformer.v1 import transformer_pb2 as _transformer_pb2
from semio.extension.translator.v1 import translator_pb2 as _translator_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class Extending(_message.Message):
    __slots__ = ["adaptings", "address", "convertings", "name", "transformings", "translatings"]
    ADAPTINGS_FIELD_NUMBER: _ClassVar[int]
    ADDRESS_FIELD_NUMBER: _ClassVar[int]
    CONVERTINGS_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    TRANSFORMINGS_FIELD_NUMBER: _ClassVar[int]
    TRANSLATINGS_FIELD_NUMBER: _ClassVar[int]
    adaptings: _containers.RepeatedCompositeFieldContainer[_adapter_pb2.Adapting]
    address: str
    convertings: _containers.RepeatedCompositeFieldContainer[_converter_pb2.Converting]
    name: str
    transformings: _containers.RepeatedCompositeFieldContainer[_transformer_pb2.Transforming]
    translatings: _containers.RepeatedCompositeFieldContainer[_translator_pb2.Translating]
    def __init__(self, address: _Optional[str] = ..., name: _Optional[str] = ..., adaptings: _Optional[_Iterable[_Union[_adapter_pb2.Adapting, _Mapping]]] = ..., convertings: _Optional[_Iterable[_Union[_converter_pb2.Converting, _Mapping]]] = ..., transformings: _Optional[_Iterable[_Union[_transformer_pb2.Transforming, _Mapping]]] = ..., translatings: _Optional[_Iterable[_Union[_translator_pb2.Translating, _Mapping]]] = ...) -> None: ...
