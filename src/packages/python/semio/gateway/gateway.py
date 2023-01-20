from typing import TYPE_CHECKING
from abc import ABC, abstractmethod

from pydantic import Field

from grpc import insecure_channel

from constants import DEFAULT_GATEWAY_PORT, DEFAULT_ASSEMBLER_PORT
from .v1.gateway_pb2 import DESCRIPTOR
from .v1.gateway_pb2_grpc import add_GatewayServiceServicer_to_server, GatewayServiceServicer, GatewayServiceStub
from utils import SemioServer, SemioServiceDescription, SemioProxy, SemioService

if TYPE_CHECKING:
    from assembler import AssemblerProxy

class GatewayServer(SemioServer, SemioService, ABC):
    assemblerAddress: str = "localhost:"+str(DEFAULT_ASSEMBLER_PORT)

    def __init__(self,port = DEFAULT_GATEWAY_PORT, name = "Python Semio Gateway Server", **kw):
        super().__init__(port=port,name=name, **kw)

    def getServicesDescriptions(self):
        return [SemioServiceDescription(service=self,servicer=GatewayServiceServicer,add_Service_to_server=add_GatewayServiceServicer_to_server,descriptor=DESCRIPTOR)]

    def getAssemblerProxy(self):#->AssemblerProxy:
        if not hasattr(self,'assemblerProxy'):
            from assembler import AssemblerProxy
            self.assemblerProxy = AssemblerProxy(self.assemblerAddress)
        return self.assemblerProxy

class GatewayProxy(SemioProxy):
    def __init__(self,address ='localhost:'+str(DEFAULT_GATEWAY_PORT), **kw):
        super().__init__(address=address,**kw)
        self._stub = GatewayServiceStub(insecure_channel(self.address))

    def LayoutDesign(self, request, context = None):
        return self._stub.LayoutDesign(request,context)
    
