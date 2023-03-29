from abc import ABC, abstractclassmethod,abstractmethod
from typing import Tuple, Any, Iterable
from collections.abc import Callable
from pydantic import BaseModel, Field

class Service(BaseModel,ABC):
    """This class implements the business logic of the rpc."""
    type: str
    dependencies: list[str] = Field(default_factory=list)

class GrpcService(Service,ABC):
    pass

class GrpcServiceDescription(BaseModel):
    service: GrpcService
    servicer:type
    # TODO Update typing to be more specific
    add_Service_to_server: Callable[[Any,Any],Any]
    # TODO Update typing to be more specific
    descriptor: Any