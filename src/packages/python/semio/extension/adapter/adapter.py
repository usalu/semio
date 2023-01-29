from abc import ABC, abstractmethod

from .v1.adapter_pb2 import Adapting
from .v1.adapter_pb2_grpc import AdapterServiceServicer

from model import Point,Plan,Link,Prototype
from extension.service import ExtensionService

class AdapterService(ExtensionService,AdapterServiceServicer,ABC):

    def getDescriptions(self) -> list[Adapting]:
        return []

    @abstractmethod
    def requestPrototype(self, plan: Plan) -> Prototype:
        pass

    def RequestPrototype(self, request, context):
        return self.requestPrototype(request.plan)

    @abstractmethod
    def requestConnectionPoint(self, connected_plan:Plan, connecting_link:Link) -> Point:
        pass

    def RequestConnectionPoint(self, request, context):
        return self.requestConnectionPoint(request.connected_plan, request.connecting_link)