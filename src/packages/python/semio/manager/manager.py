from typing import Iterable,Tuple
from pydantic import Field

from grpc import insecure_channel

from semio.constants import DEFAULT_MANAGER_PORT, DEFAULT_ASSEMBLER_PORT
from semio.utils import SemioServer, SemioServiceDescription, SemioProxy, SemioService
from .v1.manager_pb2 import DESCRIPTOR, ElementRequest, ConnectElementRequest, ConnectElementResponse
from .v1.manager_pb2_grpc import add_ManagerServiceServicer_to_server, ManagerServiceServicer, ManagerServiceStub
from semio.model import Point,Pose,Platform,Sobject,Connection,Element
from semio.extension import Extending

class ManagerServer(SemioServer,SemioService):
    assemblerAddress: str = "localhost:"+str(DEFAULT_ASSEMBLER_PORT)
    extensions: dict[str,Extending]= Field(default_factory=dict, description="Extensions with address as key and extension description as value.")
    
    def __init__(self,port = DEFAULT_MANAGER_PORT, name = "Python Semio Manager Server", **kw):
        super().__init__(port=port,name=name, **kw)

    def getServicesDescriptions(self):
        return [SemioServiceDescription(service=self,servicer=ManagerServiceServicer,add_Service_to_server=add_ManagerServiceServicer_to_server,descriptor=DESCRIPTOR)]

    def getAssemblerProxy(self):#->AssemblerProxy:
        """Get the assembler proxy. The proxy needs to be created at runtime to avoid cyclic imports between proxies and servers."""
        if not hasattr(self,'assemblerProxy'):
            from assembler import AssemblerProxy
            self.assemblerProxy = AssemblerProxy(self.assemblerAddress)
        return self.assemblerProxy

    def getExtensionProxy(self,extensionAddress: str):#->ExtensionProxy
        """Get the extension proxy for an address. The proxy needs to be created at runtime to avoid cyclic imports between proxies and servers."""
        if not extensionAddress in self.extensions:
            raise ValueError(f'There is no extension registered at {extensionAddress}. Make sure that the extension initializes properly.')
        if not hasattr(self,'extensionsProxies'):
            from extension import ExtensionProxy
            self.extensionsProxies = {extensionAddress:ExtensionProxy(extensionAddress)}
        if not extensionAddress in self.extensionsProxies:
            from extension import ExtensionProxy
            self.extensionsProxies[extensionAddress] = ExtensionProxy(extensionAddress)
        return self.extensionsProxies[extensionAddress]

class ManagerProxy(SemioProxy):
    def __init__(self,address ='localhost:'+str(DEFAULT_MANAGER_PORT), **kw):
        super().__init__(address=address,**kw)
        self._stub = ManagerServiceStub(insecure_channel(self.address))

    def RequestElement(
        self, sobject: Sobject = Sobject(),
        target_representation_platforms:Iterable[Platform] = [],
        target_representation_concepts:Iterable[str] = [],
        target_representation_lods:Iterable[int] = [],
        targets_required:bool = False)-> Element:
        return self._stub.RequestElement(ElementRequest(
            sobject=sobject,
            target_representation_platforms=target_representation_platforms,
            target_representation_concepts=target_representation_concepts,
            target_representation_lods=target_representation_lods,
            targets_required=targets_required))

    def ConnectElement(self,
        sobjects:Tuple[Sobject,Sobject],
        connection:Connection)->ConnectElementResponse:
        return self._stub.ConnectElement(request=ConnectElementRequest(
            connected_sobject=sobjects[0],
            connecting_sobject=sobjects[1],
            connection=connection))

    def RegisterExtension(self, request, context = None):
        return self._stub.RegisterExtension(request,context)

    def GetRegisteredExtensions(self, request, context = None):
        return self._stub.GetRegisteredExtensions(request,context)


