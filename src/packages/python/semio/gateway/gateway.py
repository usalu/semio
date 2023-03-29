from typing import TYPE_CHECKING, Iterable,Tuple
from abc import ABC, abstractmethod

from pydantic import Field

from logging import debug

from grpc import insecure_channel

from argparse import ArgumentParser,Namespace

from .v1.gateway_pb2 import DESCRIPTOR,LayoutDesignRequest
from .v1.gateway_pb2_grpc import add_GatewayServiceServicer_to_server, GatewayServiceServicer, GatewayServiceStub
from model import Platform,Plan,Sobject,Assembly,Connection,Layout,Prototype,Element,Design,PLATFORM_SEMIO
from server import GrpcServer, GrpcServiceDescription
from proxy import Proxy
from service import Service
from networking import getAddressFromBaseAndPort
from constants import GATEWAY, ASSEMBLER, MANAGER, GATEWAY_PORT, ASSEMBLER_PORT, MANAGER_PORT, GATEWAY_NAME

if TYPE_CHECKING:
    from assembler import AssemblerProxy


class GatewayService(Service, ABC):
    type = 'gateway'
    dependencies = ['assembler','manager']

    @abstractmethod
    def layoutDesign(self, layout:Layout,target_platform:Platform)->Design:
        pass
    
    def LayoutDesign(self, request, context):
        return self.layoutDesign(request.layout,request.target_platform)


class GatewayServer(GrpcServer, GatewayGrpcService, ABC):
    # Base address of the assembler service without the port number.
    # Assumes that assembler has a dns entry that points to the ip address of the assembler service
    assemblerBaseAddress: str = "assembler"
    # Port number of the assembler service.
    assemblerPort: int = ASSEMBLER_PORT
    # Assumes that manager has a dns entry that points to the ip address of the assembler service
    managerBaseAddress: str = "manager"
    managerPort: int = MANAGER_PORT

    def __init__(self,port = GATEWAY_PORT, name = "Python Semio Gateway Server", **kw):
        super().__init__(port=port,name=name, **kw)

    def modifyArgumentParser(self, argumentParser: ArgumentParser):
        argumentParser.add_argument()

    def initializeAfterCli(self,args:Namespace):
        if args.local:
            self.assemblerBaseAddress = 'localhost'
            self.managerBaseAddress = 'localhost'
            debug(f'Gateway server [{self.name}] initialized in local mode. \n Assembler and manager service are supposed to be available under localhost.')

    def _getGrpcServicesDescriptions(self):
        return [GrpcServiceDescription(service=self,servicer=GatewayServiceServicer,add_Service_to_server=add_GatewayServiceServicer_to_server,descriptor=DESCRIPTOR)]

    def _getAssemblerProxy(self):#->AssemblerProxy:
        if not hasattr(self,'assemblerProxy'):
            from assembler import AssemblerProxy
            self.assemblerProxy = AssemblerProxy(self.assemblerBaseAddress,self.assemblerPort)
        return self.assemblerProxy

    def _getManagerProxy(self):#->ManagerProxy:
        if not hasattr(self,'managerProxy'):
            from manager import ManagerProxy
            self.managerProxy = ManagerProxy(self.managerBaseAddress,self.managerPort)
        return self.managerProxy
    
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
    
class GatewayProxy(Proxy):
    def __init__(self, baseAddress ='gateway', port = GATEWAY_PORT, **kw):
        super().__init__(baseAddress=baseAddress,port=port,**kw)
        address = getAddressFromBaseAndPort(baseAddress,port)
        self._stub = GatewayServiceStub(insecure_channel(address))

    def LayoutDesign(self, layout:Layout, target_platform:Platform):
        return self._stub.LayoutDesign(LayoutDesignRequest(layout=layout,target_platform=target_platform))
    
