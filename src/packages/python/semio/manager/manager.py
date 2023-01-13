# TODO This file can be generated
from grpc import insecure_channel

from constants import DEFAULT_MANAGER_PORT, DEFAULT_GATEWAY_PORT
from utils import SemioServer, SemioServiceDescription, SemioProxy, SemioService
from .v1.manager_pb2 import DESCRIPTOR
from .v1.manager_pb2_grpc import add_ManagerServiceServicer_to_server, ManagerServiceServicer, ManagerServiceStub

class ManagerServer(SemioServer,SemioService):
    gatewayAddress: str = "localhost:"+str(DEFAULT_GATEWAY_PORT)
    
    def __init__(self,port = DEFAULT_MANAGER_PORT, name = "Python Semio Manager Server", **kw):
        super().__init__(port=port,name=name, **kw)

    def getServicesDescriptions(self):
        return [SemioServiceDescription(service=self,servicer=ManagerServiceServicer,add_Service_to_server=add_ManagerServiceServicer_to_server,descriptor=DESCRIPTOR)]

    def getGatewayProxy(self):
        return 

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


