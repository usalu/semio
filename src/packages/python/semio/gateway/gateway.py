# TODO This file can be generated
from abc import ABC, abstractmethod

from grpc import insecure_channel

from utils import SemioServer, SemioServiceDescription, SemioProxy, SemioServerDescription, SemioService
from .v1.gateway_pb2 import DESCRIPTOR
from .v1.gateway_pb2_grpc import add_GatewayServiceServicer_to_server, GatewayServiceServicer, GatewayServiceStub

DEFAULT_GATEWAY_PORT = 50000

class GatewayServer(SemioServer, ABC):
    gatewayService: SemioService
    def __init__(self,port = DEFAULT_GATEWAY_PORT, name = "Python Semio Gateway Server", **kw):
        super().__init__(port=port,name=name, **kw)

    def getServicesDescriptions(self):
        return [SemioServiceDescription(service=self.getGetGatewayService(),servicer=GatewayServiceServicer,add_Service_to_server=add_GatewayServiceServicer_to_server,descriptor=DESCRIPTOR)]

    @abstractmethod
    def getGetGatewayService(self) -> SemioService:
        pass


class GatewayProxy(SemioProxy):
    def __init__(self,address ='localhost:'+str(DEFAULT_GATEWAY_PORT), **kw):
        super().__init__(address=address,**kw)
        self._stub = GatewayServiceStub(insecure_channel(self.address))

    def LayoutDesign(self, request, context = None):
        return self._stub.LayoutDesign(request,context)
    
