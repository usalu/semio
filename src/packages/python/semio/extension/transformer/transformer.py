from typing import Iterable

from abc import ABC, abstractmethod

from .v1.transformer_pb2 import Transforming,RewriteLayoutRequest
from .v1.transformer_pb2_grpc import TransformerServiceServicer

from model import Layout,Decision
from extension.service import ExtensionService

class TransformerService(ExtensionService,TransformerServiceServicer):

    def getDescriptions(self) -> list[Transforming]:
        return []

    @abstractmethod
    def rewriteLayout(self, decisions: Iterable[Decision], initial_layout:Layout | None = None)->Layout:
        pass

    def RewriteLayout(self, request,context):
        return self.rewriteLayout(request.decisions,request.initial_layout)

    