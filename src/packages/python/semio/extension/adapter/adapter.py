from extension.service import ExtensionService
from .v1.adapter_pb2 import Adapting
from .v1.adapter_pb2_grpc import AdapterServiceServicer

class AdapterService(ExtensionService,AdapterServiceServicer):

    def getDescriptions(self) -> list[Adapting]:
        return []