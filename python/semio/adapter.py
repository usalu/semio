from typing import Union, Any
from abc import ABC, abstractmethod
from rdflib import Graph

class Adapter(ABC):
  """An adapter for a platform."""
  
  @abstractmethod
  async def compute(plan:Graph):
      pass