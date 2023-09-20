from typing import Iterable

from semio.extension.transformer.v1.transformer_pb2 import Transforming,RewriteLayoutRequest
from semio.extension.transformer.v1.transformer_pb2_grpc import TransformerServiceServicer

from semio.model import Layout,Decision
from semio.extension.service import ExtensionService

class TransformerService(ExtensionService,TransformerServiceServicer):

    def _getDescriptions(self) -> list[Transforming]:
        return []

    def rewriteLayout(self, decisions: Iterable[Decision], initial_layout:Layout | None = None)->Layout:
        raise NotImplementedError("This method needs to be overriden by the implementation if you want to use it.")

    def RewriteLayout(self, request,context):
        return self.rewriteLayout(request.decisions,request.initial_layout)

    