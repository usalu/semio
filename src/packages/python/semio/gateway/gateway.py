from typing import TYPE_CHECKING
from abc import ABC, abstractmethod

from pydantic import Field

from grpc import insecure_channel

from constants import DEFAULT_GATEWAY_PORT, DEFAULT_MANAGER_PORT
from .v1.gateway_pb2 import DESCRIPTOR
from .v1.gateway_pb2_grpc import add_GatewayServiceServicer_to_server, GatewayServiceServicer, GatewayServiceStub
from utils import SemioServer, SemioServiceDescription, SemioProxy, SemioService

if TYPE_CHECKING:
    from manager import ManagerProxy

class GatewayServer(SemioServer, SemioService, ABC):
    managerAddress: str = "localhost:"+str(DEFAULT_MANAGER_PORT)

    def __init__(self,port = DEFAULT_GATEWAY_PORT, name = "Python Semio Gateway Server", **kw):
        super().__init__(port=port,name=name, **kw)

    def getServicesDescriptions(self):
        return [SemioServiceDescription(service=self,servicer=GatewayServiceServicer,add_Service_to_server=add_GatewayServiceServicer_to_server,descriptor=DESCRIPTOR)]

    def getManagerProxy(self):#->ManagerProxy:
        if not hasattr(self,'managerProxy'):
            from manager import ManagerProxy
            self.managerProxy = ManagerProxy(self.managerAddress)
        return self.managerProxy

class GatewayProxy(SemioProxy):
    def __init__(self,address ='localhost:'+str(DEFAULT_GATEWAY_PORT), **kw):
        super().__init__(address=address,**kw)
        self._stub = GatewayServiceStub(insecure_channel(self.address))

    def LayoutDesign(self, request, context = None):
        return self._stub.LayoutDesign(request,context)
    
