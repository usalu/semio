from abc import ABC, abstractmethod
from extension.service  import ExtensionService
from .v1.converter_pb2 import Converting

class ConverterService(ExtensionService, ABC):

    @abstractmethod
    def getDescription(self) -> Converting:
        pass