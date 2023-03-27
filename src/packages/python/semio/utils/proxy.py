from abc import ABC, abstractmethod
from pydantic import BaseModel
from typing import Iterable

# class SemioServerDescription(BaseModel):
#     stub:type

class SemioProxy(BaseModel,ABC):
    baseAddress: str
    port: int
    
    class Config:
        extra = 'allow'