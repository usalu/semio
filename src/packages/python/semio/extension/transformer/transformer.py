from extension.service import ExtensionService
from .v1.transformer_pb2 import Transforming
from .v1.transformer_pb2_grpc import TransformerServiceServicer

class TransformerService(ExtensionService,TransformerServiceServicer):

    def getDescriptions(self) -> list[Transforming]:
        return []