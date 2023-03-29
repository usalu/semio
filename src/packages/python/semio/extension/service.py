from abc import ABC, abstractmethod
from service import Service

class ExtensionService(Service,ABC):
    #TODO Make class decorator
    @abstractmethod
    def _getDescriptions(self):
        pass
