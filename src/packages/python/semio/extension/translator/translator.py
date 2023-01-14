from extension.service import ExtensionService
from .v1.translator_pb2 import Translating
from .v1.translator_pb2_grpc import TranslatorServiceServicer

class TranslatorService(ExtensionService,TranslatorServiceServicer):

    def getDescriptions(self) -> list[Translating]:
        return []