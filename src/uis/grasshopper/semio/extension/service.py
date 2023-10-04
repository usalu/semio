from abc import ABC, abstractmethod
from semio.utils import SemioService

class ExtensionService(SemioService,ABC):
    #TODO Make class decorator
    @abstractmethod
    def _getDescriptions(self):
        pass
