from __future__ import annotations
from typing import TYPE_CHECKING,Iterable,Tuple
from abc import ABC, abstractmethod

from pydantic import Field

from grpc import insecure_channel

from .v1.assembler_pb2 import DESCRIPTOR,LayoutToAssembliesResponse,AssemblyToElementsResponse
from .v1.assembler_pb2_grpc import add_AssemblerServiceServicer_to_server, AssemblerServiceServicer, AssemblerServiceStub
from semio.model import Point,Pose,Platform,Sobject,Connection,Assembly,Layout, Element
from semio.utils import SemioServer, SemioServiceDescription, SemioProxy, SemioService
from semio.constants import DEFAULT_ASSEMBLER_PORT, DEFAULT_MANAGER_PORT

if TYPE_CHECKING:
    from manager import ManagerProxy

class AssemblerServer(SemioServer, SemioService, ABC):
    managerAddress: str = "localhost:"+str(DEFAULT_MANAGER_PORT)

    def __init__(self,port = DEFAULT_ASSEMBLER_PORT, name = "Python Semio Assembler Server", **kw):
        super().__init__(port=port,name=name, **kw)

    def getServicesDescriptions(self):
        return [SemioServiceDescription(
            service=self,
            servicer=AssemblerServiceServicer,
            add_Service_to_server=add_AssemblerServiceServicer_to_server,
            descriptor=DESCRIPTOR)]

    def getManagerProxy(self):#->ManagerProxy:
        if not hasattr(self,'managerProxy'):
            from manager import ManagerProxy
            self.managerProxy = ManagerProxy(self.managerAddress)
        return self.managerProxy
    
    def _connectElement(self,
        sobjects:Tuple[Sobject,Sobject],
        connection:Connection)->Tuple[Pose,Point]:
        return self.managerProxy.ConnectElement(sobjects=sobjects,connection=connection)

    def _requestElement(
        self, sobject: Sobject = Sobject(),
        target_representation_platforms:Iterable[Platform] | None = None,
        target_representation_concepts:Iterable[str] | None = None,
        target_representation_lods:Iterable[int] | None = None,
        targets_required:bool = False)-> Element:
        return self.managerProxy.RequestElement(
            sobject=sobject,
            target_representation_platforms=target_representation_platforms,
            target_representation_concepts=target_representation_concepts,
            target_representation_lods=target_representation_lods,
            targets_required=targets_required)

    @abstractmethod
    def layoutToAssemblies(layout: Layout)->Iterable[Assembly]:
        pass

    @abstractmethod
    def assemblyToElements(sobjects: Iterable[Sobject],connections: Iterable[Connection], assembly:Assembly)->Iterable[Element]:
        pass

    def LayoutToAssemblies(self,request,context):
        return LayoutToAssembliesResponse(assemblies = self.layoutToAssemblies(request))

    def AssemblyToElements(self,request,context):
        return AssemblyToElementsResponse(elements =self.assemblyToElements(request.sobject,request.connections,request.assembly))

class AssemblerProxy(SemioProxy):
    def __init__(self,address ='localhost:'+str(DEFAULT_ASSEMBLER_PORT), **kw):
        super().__init__(address=address,**kw)
        self._stub = AssemblerServiceStub(insecure_channel(self.address))

    def LayoutToAssemblies(self, layout: Layout):
        return self._stub.LayoutToAssemblies(request=layout).assemblies

    def AssemblyToElements(self, assembly: Assembly) -> list[Element]:
        return self._stub.AssemblyToElements(request= assembly).elements