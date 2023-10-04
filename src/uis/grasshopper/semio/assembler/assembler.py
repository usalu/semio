from __future__ import annotations
from typing import TYPE_CHECKING,Iterable,Tuple
from abc import ABC, abstractmethod

from pydantic import Field

from logging import debug

from grpc import insecure_channel

from semio.assembler.v1.assembler_pb2 import DESCRIPTOR,LayoutToAssembliesResponse,AssemblyToElementsRequest,AssemblyToElementsResponse
from semio.assembler.v1.assembler_pb2_grpc import add_AssemblerServiceServicer_to_server, AssemblerServiceServicer, AssemblerServiceStub
from semio.geometry import Point
from semio.model import Pose,Platform,Plan,Sobject,Connection,Assembly,Layout,Prototype,Element,PLATFORM_SEMIO
from semio.utils import SemioServer, SemioServiceDescription, SemioProxy, SemioService, getAddressFromBaseAndPort
from semio.constants import DEFAULT_ASSEMBLER_PORT, DEFAULT_MANAGER_PORT

if TYPE_CHECKING:
    from semio.manager import ManagerProxy

class AssemblerServer(SemioServer, SemioService, ABC):
    managerBaseAddress: str = 'manager'
    managerPort: int = DEFAULT_MANAGER_PORT

    def __init__(self,port = DEFAULT_ASSEMBLER_PORT, name = "Python Semio Assembler Server", **kw):
        super().__init__(port=port,name=name, **kw)

    def initialize(self,local=False):
        if local:
            self.managerBaseAddress = 'localhost'
            debug(f'Assembler server [{self.name}] initialized in local mode. \n The manager service is supposed to be available under localhost.')

    def _getServicesDescriptions(self):
        return [SemioServiceDescription(
            service=self,
            servicer=AssemblerServiceServicer,
            add_Service_to_server=add_AssemblerServiceServicer_to_server,
            descriptor=DESCRIPTOR)]

    def _getManagerProxy(self):#->ManagerProxy:
        if not hasattr(self,'managerProxy'):
            from semio.manager import ManagerProxy
            self.managerProxy = ManagerProxy(self.managerBaseAddress,self.managerPort)
        return self.managerProxy
    
    # Service definitions

    @abstractmethod
    def layoutToAssemblies(self, layout: Layout)->Iterable[Assembly]:
        pass

    def LayoutToAssemblies(self, request, context):
        return LayoutToAssembliesResponse(assemblies = self.layoutToAssemblies(request))

    @abstractmethod
    def assemblyToElements(self, assembly:Assembly, sobjects: Iterable[Sobject], connections: Iterable[Connection] | None = None)->Iterable[Element]:
        pass

    def AssemblyToElements(self, request, context):
        elements = self.assemblyToElements(request.assembly,request.sobjects,request.connections)
        return AssemblyToElementsResponse(elements=elements)

    # Proxy definitions

    def ConnectElement(self,
        connected_sobject: Sobject,
        connecting_sobject: Sobject,
        connection:Connection)->Tuple[Pose,Point]:
        return self._getManagerProxy().ConnectElement(connected_sobject,connecting_sobject,connection)

    def RequestPrototype(
        self, plan: Plan,
        target_platform:Platform | None = None,)-> Prototype:
        return self._getManagerProxy().RequestPrototype(plan,target_platform)

class AssemblerProxy(SemioProxy):
    def __init__(self,baseAddress ='assembler:', port = DEFAULT_ASSEMBLER_PORT, **kw):
        super().__init__(baseAddress=baseAddress,port=port,**kw)
        self._stub = AssemblerServiceStub(insecure_channel(getAddressFromBaseAndPort(self.baseAddress,self.port)))

    def LayoutToAssemblies(self, layout: Layout):
        return self._stub.LayoutToAssemblies(request=layout).assemblies

    def AssemblyToElements(self,
        assembly:Assembly,
        sobjects: Iterable[Sobject],
        connections: Iterable[Connection] | None = None
        )->Tuple[Iterable[Prototype],Iterable[Element]]:
        assemblyToElementsResponse = self._stub.AssemblyToElements(
            request=AssemblyToElementsRequest(
                assembly=assembly,
                sobjects=sobjects,
                connections=connections
            ))
        return assemblyToElementsResponse.elements