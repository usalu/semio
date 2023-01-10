# from pydantic import Field

# from gateway import ExtendingService
# from utils import Server

# class ExtensionServer (Server):
#     gatway_address: str = "localhost:51000"
#     extension: ExtendingService = Field(default_factory=ExtendingService)

# TODO This file can be generated
from grpc import insecure_channel

from utils import SemioServer, SemioServiceDescription, SemioProxy, SemioServerDescription
from .adapter.v1.adapter_pb2 import DESCRIPTOR as ADAPTER_DESCRIPTOR
from .adapter.v1.adapter_pb2_grpc import add_AdapterServiceServicer_to_server, AdapterServiceServicer, AdapterServiceStub
from .converter.v1.converter_pb2 import DESCRIPTOR as CONVERTER_DESCRIPTOR
from .converter.v1.converter_pb2_grpc import add_ConverterServiceServicer_to_server, ConverterServiceServicer, ConverterServiceStub
from .transformer.v1.transformer_pb2 import DESCRIPTOR as TRANSFORMER_DESCRIPTOR
from .transformer.v1.transformer_pb2_grpc import add_TransformerServiceServicer_to_server, TransformerServiceServicer, TransformerServiceStub
from .translator.v1.translator_pb2 import DESCRIPTOR as TRANSLATOR_DESCRIPTOR
from .translator.v1.translator_pb2_grpc import add_TranslatorServiceServicer_to_server, TranslatorServiceServicer, TranslatorServiceStub

DEFAULT_EXTENSION_PORT = 59001

class ExtensionServer(SemioServer):
    def __init__(self,port = DEFAULT_EXTENSION_PORT, name = "Python Semio Gateway Server", **kw):
        super().__init__(port=port,name=name, **kw)

    def getServicesDescriptions(self):
        return [SemioServiceDescription(servicer=GatewayServiceServicer,add_Service_to_server=add_GatewayServiceServicer_to_server,descriptor=DESCRIPTOR)]

class ExtensionProxy(SemioProxy):
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
    

