from extension.service  import ExtensionService
from .v1.converter_pb2 import Converting
from .v1.converter_pb2_grpc import ConverterServiceServicer

class ConverterService(ExtensionService,ConverterServiceServicer):

    def getDescriptions(self) -> list[Converting]:
        return []