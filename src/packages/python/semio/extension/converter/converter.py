from abc import ABC, abstractmethod

from .v1.converter_pb2 import Converting,RepresentationConversionRequest
from .v1.converter_pb2_grpc import ConverterServiceServicer

from semio.model import Platform,Representation
from semio.extension.service  import ExtensionService

class ConverterService(ExtensionService,ConverterServiceServicer,ABC):

    def getDescriptions(self) -> list[Converting]:
        return []

    @abstractmethod
    def convertRepresentation(self, representation:Representation, target_platform:Platform)->Representation:
        pass

    def ConvertRepresentation(self, request, context):
        return self.convertRepresentation(request.representation, request.target_platform)