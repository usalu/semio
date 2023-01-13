# TODO This file can be generated
from grpc import insecure_channel

from utils import SemioServer, SemioServiceDescription, SemioProxy
from .v1.manager_pb2 import DESCRIPTOR
from .v1.manager_pb2_grpc import add_ManagerServiceServicer_to_server, ManagerServiceServicer, ManagerServiceStub

DEFAULT_Manager_PORT = 50000

class ManagerServer(SemioServer):
    def __init__(self,port = DEFAULT_Manager_PORT, name = "Python Semio Manager Server", **kw):
        super().__init__(port=port,name=name, **kw)

    def getServicesDescriptions(self):
        return [SemioServiceDescription(servicer=ManagerServiceServicer,add_Service_to_server=add_ManagerServiceServicer_to_server,descriptor=DESCRIPTOR)]

class ManagerProxy(SemioProxy):
    def __init__(self,address ='localhost:'+str(DEFAULT_Manager_PORT), **kw):
        super().__init__(address=address,**kw)
        self._stub = ManagerServiceStub(insecure_channel(self.address))

    def RequestElement(self, request, context = None):
        return self._stub.RequestElement(request,context)

    def RequestAttraction(self, request, context = None):
        return self._stub.RequestAttraction(request,context)


