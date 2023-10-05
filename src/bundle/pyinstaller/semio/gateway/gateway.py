from typing import TYPE_CHECKING, Iterable,Tuple
from abc import ABC, abstractmethod

from pydantic import Field

from logging import debug

from grpc import insecure_channel

from semio.gateway.v1.gateway_pb2 import DESCRIPTOR,LayoutDesignRequest
from semio.gateway.v1.gateway_pb2_grpc import add_GatewayServiceServicer_to_server, GatewayServiceServicer, GatewayServiceStub
from semio.model import Platform,Plan,Sobject,Assembly,Connection,Layout,Prototype,Element,Design,PLATFORM_SEMIO
from semio.utils import SemioServer, SemioServiceDescription, SemioProxy, SemioService, getAddressFromBaseAndPort
from semio.constants import DEFAULT_GATEWAY_PORT, DEFAULT_ASSEMBLER_PORT, DEFAULT_MANAGER_PORT

# Avoid import cycle
if TYPE_CHECKING:
    from semio.assembler import AssemblerProxy

class GatewayServer(SemioServer, SemioService, ABC):
    # Base address of the assembler service without the port number.
    # Assumes that assembler has a dns entry that points to the ip address of the assembler service
    assemblerBaseAddress: str = "assembler"
    # Port number of the assembler service.
    assemblerPort: int = DEFAULT_ASSEMBLER_PORT
    # Assumes that manager has a dns entry that points to the ip address of the assembler service
    managerBaseAddress: str = "manager"
    managerPort: int = DEFAULT_MANAGER_PORT

    def __init__(self,port = DEFAULT_GATEWAY_PORT, name = "Python Semio Gateway Server", **kw):
        super().__init__(port=port,name=name, **kw)

    def initialize(self,local=False):
        if local:
            self.assemblerBaseAddress = 'localhost'
            self.managerBaseAddress = 'localhost'
            debug(f'Gateway server [{self.name}] initialized in local mode. \n Assembler and manager service are supposed to be available under localhost.')

    def _getServicesDescriptions(self):
        return [SemioServiceDescription(service=self,servicer=GatewayServiceServicer,add_Service_to_server=add_GatewayServiceServicer_to_server,descriptor=DESCRIPTOR)]

    def _getAssemblerProxy(self):#->AssemblerProxy:
        if not hasattr(self,'assemblerProxy'):
            from semio.assembler import AssemblerProxy
            self.assemblerProxy = AssemblerProxy(self.assemblerBaseAddress,self.assemblerPort)
        return self.assemblerProxy

    def _getManagerProxy(self):#->ManagerProxy:
        if not hasattr(self,'managerProxy'):
            from semio.manager import ManagerProxy
            self.managerProxy = ManagerProxy(self.managerBaseAddress,self.managerPort)
        return self.managerProxy

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
        connections: Iterable[Connection] | None = None
        )->Tuple[Iterable[Prototype],Iterable[Element]]:
        return self._getAssemblerProxy().AssemblyToElements(assembly,sobjects,connections)

    def RequestPrototype(self, 
        plan:Plan,
        target_platform:Platform = PLATFORM_SEMIO)->Prototype:
        return self._getManagerProxy().RequestPrototype(plan,target_platform)
    
class GatewayProxy(SemioProxy):
    def __init__(self, baseAddress ='gateway', port = DEFAULT_GATEWAY_PORT, **kw):
        super().__init__(baseAddress=baseAddress,port=port,**kw)
        address = getAddressFromBaseAndPort(baseAddress,port)
        self._stub = GatewayServiceStub(insecure_channel(address))

    def LayoutDesign(self, layout:Layout, target_platform:Platform):
        return self._stub.LayoutDesign(LayoutDesignRequest(layout=layout,target_platform=target_platform))
    
