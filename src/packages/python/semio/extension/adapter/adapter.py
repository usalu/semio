from .v1.adapter_pb2 import Adapting
from .v1.adapter_pb2_grpc import AdapterServiceServicer

from geometry import Point
from model import Plan,Link,Representation,Prototype
from extension.service import ExtensionService

class AdapterService(ExtensionService,AdapterServiceServicer):

    def _getDescriptions(self) -> list[Adapting]:
        return []

    def requestPrototype(self, plan: Plan) -> Prototype:
        raise NotImplementedError("This method needs to be overriden by the implementation if you want to use it.")

    def RequestPrototype(self, request, context):
        return self.requestPrototype(request)

    def requestConnectionPoint(self, plan:Plan, link:Link, representation: None | Point | Representation = None) -> Point:
        raise NotImplementedError("This method needs to be overriden by the implementation if you want to use it.")

    def RequestConnectionPoint(self, request, context):
        representationType = request.WhichOneof("representation")
        match representationType:
            case "simple_representation":
                representation = request.simple_representation
            case "full_representation":
                representation = request.full_representation
            case _:
                representation = None
        return self.requestConnectionPoint(request.plan, request.link, representation)