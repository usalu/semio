from abc import ABC, abstractmethod
from pydantic import BaseModel
from typing import Iterable

# class ServerDescription(BaseModel):
#     stub:type

class Proxy(BaseModel,ABC):
    baseAddress: str
    port: int
    
    class Config:
        extra = 'allow'