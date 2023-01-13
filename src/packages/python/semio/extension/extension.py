
# TODO This file can be generated
from pydantic import Field

from utils import SemioServer, SemioServiceDescription, SemioProxy, SemioServerDescription, SemioService

from manager import ExtendingService

from grpc import insecure_channel



from .adapter import DESCRIPTOR as ADAPTER_DESCRIPTOR, add_AdapterServiceServicer_to_server, AdapterServiceServicer, AdapterServiceStub
from .converter import DESCRIPTOR as CONVERTER_DESCRIPTOR, add_ConverterServiceServicer_to_server, ConverterServiceServicer, ConverterServiceStub
from .transformer import DESCRIPTOR as TRANSFORMER_DESCRIPTOR, add_TransformerServiceServicer_to_server, TransformerServiceServicer, TransformerServiceStub
from .translator import DESCRIPTOR as TRANSLATOR_DESCRIPTOR, add_TranslatorServiceServicer_to_server, TranslatorServiceServicer, TranslatorServiceStub



DEFAULT_EXTENSION_PORT = 59001


class AdapterService(ExtensionService):


class ExtensionService(SemioService):
    def registerService(self):


class ExtensionServer(SemioServer):
    extension: ExtendingService = Field(default_factory=ExtendingService)
    adapters: list[AdapterService] = Field(default_factory=list)

    def __init__(self,port = DEFAULT_EXTENSION_PORT, name = "Python Semio Extension Server", **kw):
        super().__init__(port=port,name=name, **kw)

    def getServicesDescriptions(self):
        servicesDescriptions = []
        for adapter in self.services
        return [

            # SemioServiceDescription(servicer=AdapterServiceServicer,add_Service_to_server=add_AdapterServiceServicer_to_server,descriptor=ADAPTER_DESCRIPTOR),
            # SemioServiceDescription(servicer=ConverterServiceServicer,add_Service_to_server=add_ConverterServiceServicer_to_server,descriptor=CONVERTER_DESCRIPTOR),
            # SemioServiceDescription(servicer=TransformerServiceServicer,add_Service_to_server=add_TransformerServiceServicer_to_server,descriptor=TRANSFORMER_DESCRIPTOR),
            # SemioServiceDescription(servicer=TranslatorServiceServicer,add_Service_to_server=add_TranslatorServiceServicer_to_server,descriptor=TRANSLATOR_DESCRIPTOR)
        ]

class ExtensionProxy(SemioProxy):
    def __init__(self,address ='localhost:'+str(DEFAULT_EXTENSION_PORT), **kw):
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

