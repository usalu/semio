from abc import ABC, abstractmethod
from pydantic import BaseModel
from typing import Iterable

class SemioServerDescription(BaseModel):
    stub:type

class SemioProxy(BaseModel):
    address: str

    # # TODO Update to abstract class method
    # @abstractmethod
    # def getServerDescription(self)-> Iterable[SemioServerDescription]:
    #     pass

    class Config:
        extra = 'allow'