from typing import TYPE_CHECKING,Iterable,Tuple
from abc import ABC, abstractmethod

from pydantic import Field

from grpc import insecure_channel

from .v1.assembler_pb2 import DESCRIPTOR
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
        connectElementResponse = self.managerProxy.ConnectElement(sobjects=sobjects,connection=connection)
        return (connectElementResponse.connected_element_pose,connectElementResponse.connection_point)

    def _requestElement(
        self, sobject: Sobject = Sobject(),
        target_representation_platforms:Iterable[Platform] = [],
        target_representation_concepts:Iterable[str] = [],
        target_representation_lods:Iterable[int] = [],
        targets_required:bool = False)-> Element:
        return self.managerProxy.RequestElement(
            sobject=sobject,
            target_representation_platforms=target_representation_platforms,
            target_representation_concepts=target_representation_concepts,
            target_representation_lods=target_representation_lods,
            targets_required=targets_required)

class AssemblerProxy(SemioProxy):
    def __init__(self,address ='localhost:'+str(DEFAULT_ASSEMBLER_PORT), **kw):
        super().__init__(address=address,**kw)
        self._stub = AssemblerServiceStub(insecure_channel(self.address))

    def LayoutToAssemblies(self, layout: Layout):
        return self._stub.LayoutToAssemblies(request=layout).assemblies

    def AssemblyToElements(self, assembly: Assembly) -> list[Element]:
        return self._stub.AssemblyToElements(request= assembly).elements