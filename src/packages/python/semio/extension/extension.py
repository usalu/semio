from pydantic import Field

from grpc import insecure_channel

from typing import TYPE_CHECKING

from constants import DEFAULT_MANAGER_PORT

from utils import SemioServer, SemioServiceDescription, SemioProxy,SemioService

from .adapter import AdapterService
from .converter import ConverterService
from .transformer import TransformerService
from .translator import TranslatorService

from .adapter.v1.adapter_pb2 import DESCRIPTOR as ADAPTER_DESCRIPTOR
from .adapter.v1.adapter_pb2_grpc import add_AdapterServiceServicer_to_server, AdapterServiceServicer, AdapterServiceStub
from .converter.v1.converter_pb2 import DESCRIPTOR as CONVERTER_DESCRIPTOR
from .converter.v1.converter_pb2_grpc import add_ConverterServiceServicer_to_server, ConverterServiceServicer, ConverterServiceStub
from .transformer.v1.transformer_pb2 import DESCRIPTOR as TRANSFORMER_DESCRIPTOR
from .transformer.v1.transformer_pb2_grpc import add_TransformerServiceServicer_to_server, TransformerServiceServicer, TransformerServiceStub
from .translator.v1.translator_pb2 import DESCRIPTOR as TRANSLATOR_DESCRIPTOR
from .translator.v1.translator_pb2_grpc import add_TranslatorServiceServicer_to_server, TranslatorServiceServicer, TranslatorServiceStub

if TYPE_CHECKING:
    from manager import ManagerProxy

class ExtensionServer(SemioServer):
    managerProxyAddress: str = "localhost:" + str(DEFAULT_MANAGER_PORT)
    adapters: list[AdapterService] = Field(default_factory=list)
    converters: list[ConverterService] = Field(default_factory=list)
    transformers: list[TransformerService] = Field(default_factory=list)
    translators: list[TranslatorService] = Field(default_factory=list)

    def __init__(self,port, name = "Python Semio Extension Server", **kw):
        super().__init__(port=port,name=name, **kw)

    def getManagerProxy(self):#->ManagerProxy:
        if not hasattr(self,'managerProxy'):
            from manager import ManagerProxy
            self.managerProxy = ManagerProxy(self.managerAddress)
        return self.managerProxy

    def getServicesDescriptions(self):
        servicesDescriptions = []
        for adapter in self.adapters:
            servicesDescriptions.append(SemioServiceDescription(service=adapter,servicer=AdapterServiceServicer,add_Service_to_server=add_AdapterServiceServicer_to_server,descriptor=ADAPTER_DESCRIPTOR))
        for converter in self.converters:
            servicesDescriptions.append(SemioServiceDescription(service=converter,servicer=ConverterServiceServicer,add_Service_to_server=add_ConverterServiceServicer_to_server,descriptor=CONVERTER_DESCRIPTOR))
        for transformer in self.transformers:
            servicesDescriptions.append(SemioServiceDescription(service=transformer,servicer=TransformerServiceServicer,add_Service_to_server=add_TransformerServiceServicer_to_server,descriptor=TRANSFORMER_DESCRIPTOR))
        for translator in self.translators:
            servicesDescriptions.append(SemioServiceDescription(service=translator,servicer=TranslatorServiceServicer,add_Service_to_server=add_TranslatorServiceServicer_to_server,descriptor=TRANSLATOR_DESCRIPTOR))
        return servicesDescriptions

class ExtensionProxy(SemioProxy):
    def __init__(self,address, **kw):
        super().__init__(address=address,**kw)
        self._adapterStub = AdapterServiceStub(insecure_channel(self.address))
        self._converterStub = ConverterServiceStub(insecure_channel(self.address))
        self._transformerStub = TransformerServiceStub(insecure_channel(self.address))
        self._translatorStub = TranslatorServiceStub(insecure_channel(self.address))

    def RequestAttractionPoint(self, request, context = None):
        self._adapterStub.RequestAttractionPoint(request,context)

    def RequestRepresentation(self, request, context = None):
        self._adapterStub.RequestRepresentation(request,context)

    def RequestRepresentations(self, request, context = None):
        self._adapterStub.RequestRepresentations(request,context)

    def ConvertRepresentation(self, request, context = None):
        self._converterStub.ConvertRepresentation(request,context)
    
    def RewriteLayout(self, request, context = None):
        self._transformerStub.RewriteLayout(request,context)

    def TranslateRepresentation(self, request, context = None):
        self._translatorStub.TranslateRepresentation(request,context)

