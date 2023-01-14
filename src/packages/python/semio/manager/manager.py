from pydantic import Field

from grpc import insecure_channel

from constants import DEFAULT_MANAGER_PORT, DEFAULT_GATEWAY_PORT
from utils import SemioServer, SemioServiceDescription, SemioProxy, SemioService
from .v1.manager_pb2 import DESCRIPTOR
from .v1.manager_pb2_grpc import add_ManagerServiceServicer_to_server, ManagerServiceServicer, ManagerServiceStub
from extension import Extending

class ManagerServer(SemioServer,SemioService):
    gatewayAddress: str = "localhost:"+str(DEFAULT_GATEWAY_PORT)
    extensions: dict[str,Extending]= Field(default_factory=dict, description="Extensions with address as key and extension description as value.")
    
    def __init__(self,port = DEFAULT_MANAGER_PORT, name = "Python Semio Manager Server", **kw):
        super().__init__(port=port,name=name, **kw)

    def getServicesDescriptions(self):
        return [SemioServiceDescription(service=self,servicer=ManagerServiceServicer,add_Service_to_server=add_ManagerServiceServicer_to_server,descriptor=DESCRIPTOR)]

    def getGatewayProxy(self):#->GatewayProxy:
        """Get the gateway proxy. The proxy needs to be created at runtime to avoid cyclic imports between proxies and servers."""
        if not hasattr(self,'gatewayProxy'):
            from gateway import GatewayProxy
            self.gatewayProxy = GatewayProxy(self.gatewayAddress)
        return self.gatewayProxy

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

    def RequestAttraction(self, request, context = None):
        return self._stub.RequestAttraction(request,context)

    def RegisterExtension(self, request, context = None):
        return self._stub.RegisterExtension(request,context)

    def GetRegisteredExtensions(self, request, context = None):
        return self._stub.GetRegisteredExtensions(request,context)


