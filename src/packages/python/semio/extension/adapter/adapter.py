from .v1.adapter_pb2 import Adapting
from .v1.adapter_pb2_grpc import AdapterServiceServicer

from model import Point,Plan,Link,Prototype
from extension.service import ExtensionService

class AdapterService(ExtensionService,AdapterServiceServicer):

    def getDescriptions(self) -> list[Adapting]:
        return []

    def requestPrototype(self, plan: Plan) -> Prototype:
        raise NotImplementedError("This method needs to be overriden by the implementation if you want to use it.")

    def RequestPrototype(self, request, context):
        return self.requestPrototype(request.plan)

    def requestConnectionPoint(self, connected_plan:Plan, connecting_link:Link) -> Point:
        raise NotImplementedError("This method needs to be overriden by the implementation if you want to use it.")

    def RequestConnectionPoint(self, request, context):
        return self.requestConnectionPoint(request.connected_plan, request.connecting_link)