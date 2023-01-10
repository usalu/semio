# TODO This file can be generated
from grpc import insecure_channel

from utils import SemioServer, SemioServiceDescription, SemioProxy, SemioServerDescription
from .v1.gateway_pb2 import DESCRIPTOR
from .v1.gateway_pb2_grpc import add_GatewayServiceServicer_to_server, GatewayServiceServicer, GatewayServiceStub

DEFAULT_GATEWAY_PORT = 50000

class GatewayServer(SemioServer):
    def __init__(self,port = DEFAULT_GATEWAY_PORT, name = "Python Semio Gateway Server", **kw):
        super().__init__(port=port,name=name, **kw)

    def getServicesDescriptions(self):
        return [SemioServiceDescription(servicer=GatewayServiceServicer,add_Service_to_server=add_GatewayServiceServicer_to_server,descriptor=DESCRIPTOR)]

class GatewayProxy(SemioProxy):
    def __init__(self,address ='localhost:'+str(DEFAULT_GATEWAY_PORT), **kw):
        super().__init__(address=address,**kw)
        self._stub = GatewayServiceStub(insecure_channel(self.address))

    # def getServerDescription(self):
    #     return [SemioServerDescription(stub=GatewayServiceStub)]

    def LayoutDesign(self, request, context = None):
        return self._stub.LayoutDesign(request,context)

    def RegisterService(self, request, context = None):
        return self._stub.RegisterService(request,context)

    def GetRegisteredServices(self, request, context = None):
        return self._stub.GetRegisteredServices(request,context)
    

