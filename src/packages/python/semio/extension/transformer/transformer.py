from abc import ABC, abstractmethod
from extension.service import ExtensionService
from .v1.transformer_pb2 import Transforming

class TransformerService(ExtensionService, ABC):

    @abstractmethod
    def getDescription(self) -> Transforming:
        pass