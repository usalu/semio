from typing import TYPE_CHECKING, Iterable,Tuple
from abc import ABC, abstractmethod

from pydantic import Field

from grpc import insecure_channel

from .v1.gateway_pb2 import DESCRIPTOR,LayoutDesignRequest
from .v1.gateway_pb2_grpc import add_GatewayServiceServicer_to_server, GatewayServiceServicer, GatewayServiceStub
from model import Platform,Sobject,Assembly,Connection,Layout,Prototype,Element,Design,PLATFORM_SEMIO
from utils import SemioServer, SemioServiceDescription, SemioProxy, SemioService
from constants import DEFAULT_GATEWAY_PORT, DEFAULT_ASSEMBLER_PORT

if TYPE_CHECKING:
    from assembler import AssemblerProxy

class GatewayServer(SemioServer, SemioService, ABC):
    assemblerAddress: str = "localhost:"+str(DEFAULT_ASSEMBLER_PORT)

    def __init__(self,port = DEFAULT_GATEWAY_PORT, name = "Python Semio Gateway Server", **kw):
        super().__init__(port=port,name=name, **kw)

    def _getServicesDescriptions(self):
        return [SemioServiceDescription(service=self,servicer=GatewayServiceServicer,add_Service_to_server=add_GatewayServiceServicer_to_server,descriptor=DESCRIPTOR)]

    def _getAssemblerProxy(self):#->AssemblerProxy:
        if not hasattr(self,'assemblerProxy'):
            from assembler import AssemblerProxy
            self.assemblerProxy = AssemblerProxy(self.assemblerAddress)
        return self.assemblerProxy

    @abstractmethod
    def layoutDesign(self, layout:Layout,target_platform:Platform)->Design:
        pass
    
    def LayoutDesign(self, request, context):
        return self.layoutDesign(request.layout,request.target_platform)
    
    # Proxy definitions

    def LayoutToAssemblies(self,layout: Layout)->Iterable[Assembly]:
        return self._getAssemblerProxy().LayoutToAssemblies(layout)

    def AssemblyToElements(self,
        assembly:Assembly,
        sobjects: Iterable[Sobject],
        connections: Iterable[Connection] | None = None,
        target_platform:Platform = PLATFORM_SEMIO
        )->Tuple[Iterable[Prototype],Iterable[Element]]:
        return self._getAssemblerProxy().AssemblyToElements(assembly,sobjects,connections,target_platform)
    
class GatewayProxy(SemioProxy):
    def __init__(self,address ='localhost:'+str(DEFAULT_GATEWAY_PORT), **kw):
        super().__init__(address=address,**kw)
        self._stub = GatewayServiceStub(insecure_channel(self.address))

    def LayoutDesign(self, layout:Layout, target_platform:Platform):
        return self._stub.LayoutDesign(LayoutDesignRequest(layout=layout,target_platform=target_platform))
    
