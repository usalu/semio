from pydantic import Field

from grpc import insecure_channel

from constants import DEFAULT_MANAGER_PORT, DEFAULT_ASSEMBLER_PORT
from utils import SemioServer, SemioServiceDescription, SemioProxy, SemioService
from .v1.manager_pb2 import DESCRIPTOR
from .v1.manager_pb2_grpc import add_ManagerServiceServicer_to_server, ManagerServiceServicer, ManagerServiceStub
from extension import Extending

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

    def RequestElement(self, request, context = None):
        return self._stub.RequestElement(request,context)

    def RequestConnection(self, request, context = None):
        return self._stub.RequestConnection(request,context)

    def RegisterExtension(self, request, context = None):
        return self._stub.RegisterExtension(request,context)

    def GetRegisteredExtensions(self, request, context = None):
        return self._stub.GetRegisteredExtensions(request,context)


