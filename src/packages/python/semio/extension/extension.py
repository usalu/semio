# This file should be automatically generated
import logging

from pydantic import Field

from grpc import insecure_channel

from typing import TYPE_CHECKING

from constants import DEFAULT_MANAGER_PORT

from utils import SemioServer, SemioServiceDescription, SemioProxy

from model import Sobject

from .adapter import AdapterService
from .converter import ConverterService
from .transformer import TransformerService
from .translator import TranslatorService

from .adapter.v1.adapter_pb2 import DESCRIPTOR as ADAPTER_DESCRIPTOR, RepresentationRequest, RepresentationsRequest
from .adapter.v1.adapter_pb2_grpc import add_AdapterServiceServicer_to_server, AdapterServiceServicer, AdapterServiceStub
from .converter.v1.converter_pb2 import DESCRIPTOR as CONVERTER_DESCRIPTOR
from .converter.v1.converter_pb2_grpc import add_ConverterServiceServicer_to_server, ConverterServiceServicer, ConverterServiceStub
from .transformer.v1.transformer_pb2 import DESCRIPTOR as TRANSFORMER_DESCRIPTOR
from .transformer.v1.transformer_pb2_grpc import add_TransformerServiceServicer_to_server, TransformerServiceServicer, TransformerServiceStub
from .translator.v1.translator_pb2 import DESCRIPTOR as TRANSLATOR_DESCRIPTOR
from .translator.v1.translator_pb2_grpc import add_TranslatorServiceServicer_to_server, TranslatorServiceServicer, TranslatorServiceStub

from .v1.extension_pb2 import Extending

if TYPE_CHECKING:
    from manager import ManagerProxy

# This import style is necissary to not trigger cyclic imports.
import manager

class ExtensionServer(SemioServer):
    managerProxyAddress: str = "localhost:" + str(DEFAULT_MANAGER_PORT)
    adapter: AdapterService = Field(default_factory=AdapterService)
    converter: ConverterService = Field(default_factory=ConverterService)
    transformer: TransformerService = Field(default_factory=TransformerService)
    translator: TranslatorService = Field(default_factory=TranslatorService)

    def getManagerProxy(self):#->ManagerProxy:
        if not hasattr(self,'managerProxy'):
            from manager import ManagerProxy
            self.managerProxy = ManagerProxy(self.managerProxyAddress)
        return self.managerProxy

    def getServicesDescriptions(self):
        servicesDescriptions = [
            SemioServiceDescription(service=self.adapter,servicer=AdapterServiceServicer,add_Service_to_server=add_AdapterServiceServicer_to_server,descriptor=ADAPTER_DESCRIPTOR),
            SemioServiceDescription(service=self.converter,servicer=ConverterServiceServicer,add_Service_to_server=add_ConverterServiceServicer_to_server,descriptor=CONVERTER_DESCRIPTOR),
            SemioServiceDescription(service=self.transformer,servicer=TransformerServiceServicer,add_Service_to_server=add_TransformerServiceServicer_to_server,descriptor=TRANSFORMER_DESCRIPTOR),
            SemioServiceDescription(service=self.translator,servicer=TranslatorServiceServicer,add_Service_to_server=add_TranslatorServiceServicer_to_server,descriptor=TRANSLATOR_DESCRIPTOR)
        ]
        return servicesDescriptions
    
    def initialize(self):
        address = 'localhost:' +str(self.port)
        response = self.getManagerProxy().RegisterExtension(manager.ExtensionRegistrationRequest(
            address=address,
            extending=Extending(
                adaptings=self.adapter.getDescriptions(), convertings=self.converter.getDescriptions(), 
                transformings=self.transformer.getDescriptions(), translatings=self.translator.getDescriptions())))
        if response.success:
            logging.debug(f'Extension {self.name} ({address}) was successfully registered at manager {self.managerProxyAddress}')
        else:
            logging.debug(f'The extension {self.name} ({address}) couldn\'t be registered at manager {self.managerProxyAddress}.'+
             f'Probably there is already an extension registered either at {address} or with name {self.name}. Make sure to set replace existing in the extension registration request to true if you want to override the other extension.')

class ExtensionProxy(SemioProxy):
    def __init__(self,address, **kw):
        super().__init__(address=address,**kw)
        self._adapterStub = AdapterServiceStub(insecure_channel(self.address))
        self._converterStub = ConverterServiceStub(insecure_channel(self.address))
        self._transformerStub = TransformerServiceStub(insecure_channel(self.address))
        self._translatorStub = TranslatorServiceStub(insecure_channel(self.address))

    def RequestAttractionPoint(self, request, context = None):
        return self._adapterStub.RequestAttractionPoint(request,context)

    def RequestRepresentation(self, sobject:Sobject,  type: str = 'native', name: str = 'normal', lod: int = 0):
        return self._adapterStub.RequestRepresentation(request=RepresentationRequest(sobject=sobject,type=type,name=name,lod=lod))

    def RequestRepresentations(self, request, context = None):
        return self._adapterStub.RequestRepresentations(request,context)

    def ConvertRepresentation(self, request, context = None):
        return self._converterStub.ConvertRepresentation(request,context)
    
    def RewriteLayout(self, request, context = None):
        return self._transformerStub.RewriteLayout(request,context)

    def TranslateRepresentation(self, request, context = None):
        return self._translatorStub.TranslateRepresentation(request,context)

