from abc import ABC, abstractmethod
from extension.service import ExtensionService
from .v1.adapter_pb2 import Adapting

class AdapterService(ExtensionService, ABC):

    @abstractmethod
    def getDescription(self) -> Adapting:
        pass