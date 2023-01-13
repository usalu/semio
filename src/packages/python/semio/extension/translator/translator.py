from abc import ABC, abstractmethod
from extension.service import ExtensionService
from .v1.translator_pb2 import Translating

class TranslatorService(ExtensionService, ABC):

    @abstractmethod
    def getDescription(self) -> Translating:
        pass