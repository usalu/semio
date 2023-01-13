from abc import ABC, abstractmethod
from utils import SemioService

class ExtensionService(SemioService,ABC):
    #TODO Make class decorator
    @abstractmethod
    def getDescription(self):
        pass
