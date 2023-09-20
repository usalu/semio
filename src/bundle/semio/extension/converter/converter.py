from semio.extension.converter.v1.converter_pb2 import Converting,RepresentationConversionRequest
from semio.extension.converter.v1.converter_pb2_grpc import ConverterServiceServicer

from semio.model import Platform,Representation
from semio.extension.service  import ExtensionService

class ConverterService(ExtensionService,ConverterServiceServicer):

    def _getDescriptions(self) -> list[Converting]:
        return []

    def convertRepresentation(self, representation:Representation, target_platform:Platform)->Representation:
        raise NotImplementedError("This method needs to be overriden by the implementation if you want to use it.")

    def ConvertRepresentation(self, request, context):
        return self.convertRepresentation(request.representation, request.target_platform)