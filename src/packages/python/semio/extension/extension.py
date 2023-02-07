from __future__ import annotations
import logging

from pydantic import Field

from grpc import insecure_channel

from typing import TYPE_CHECKING, Iterable

from .adapter.v1.adapter_pb2 import DESCRIPTOR as ADAPTER_DESCRIPTOR, ConnectionPointRequest
from .adapter.v1.adapter_pb2_grpc import add_AdapterServiceServicer_to_server, AdapterServiceServicer, AdapterServiceStub
from .converter.v1.converter_pb2 import DESCRIPTOR as CONVERTER_DESCRIPTOR, RepresentationConversionRequest
from .converter.v1.converter_pb2_grpc import add_ConverterServiceServicer_to_server, ConverterServiceServicer, ConverterServiceStub
from .transformer.v1.transformer_pb2 import DESCRIPTOR as TRANSFORMER_DESCRIPTOR, RewriteLayoutRequest
from .transformer.v1.transformer_pb2_grpc import add_TransformerServiceServicer_to_server, TransformerServiceServicer, TransformerServiceStub
from .translator.v1.translator_pb2 import DESCRIPTOR as TRANSLATOR_DESCRIPTOR, TranslateRepresentationRequest
from .translator.v1.translator_pb2_grpc import add_TranslatorServiceServicer_to_server, TranslatorServiceServicer, TranslatorServiceStub

from geometry import Point
from model import REPRESENTATIONPROTOCOL_NONE,REPRESENTATIONPROTOCOL_SIMPLE,REPRESENTATIONPROTOCOL_FULL,Pose,Platform,Representation,Plan,Link,Sobject,Layout,Decision,Prototype
from utils import SemioServer, SemioServiceDescription, SemioProxy
from constants import DEFAULT_MANAGER_PORT

from extension.adapter import AdapterService
from extension.converter import ConverterService
from extension.transformer import TransformerService
from extension.translator import TranslatorService

if TYPE_CHECKING:
    from manager import ManagerProxy

# This import style is necissary to not trigger cyclic imports.
import manager

class ExtensionServer(SemioServer):
    managerProxyAddress: str = "localhost:" + str(DEFAULT_MANAGER_PORT)
    # These should be abstract classes but pydantic doesn't let you define this without using Union[ALL, SUB, CLASSES]
    # https://stackoverflow.com/questions/58301364/pydantic-and-subclasses-of-abstract-class
    adapter: AdapterService = Field(default_factory=AdapterService)
    converter: ConverterService = Field(default_factory=ConverterService)
    transformer: TransformerService = Field(default_factory=TransformerService)
    translator: TranslatorService = Field(default_factory=TranslatorService)

    def _getManagerProxy(self):#->ManagerProxy:
        if not hasattr(self,'managerProxy'):
            from manager import ManagerProxy
            self.managerProxy = ManagerProxy(self.managerProxyAddress)
        return self.managerProxy

    def _getServicesDescriptions(self):
        servicesDescriptions = [
            SemioServiceDescription(service=self.adapter,servicer=AdapterServiceServicer,add_Service_to_server=add_AdapterServiceServicer_to_server,descriptor=ADAPTER_DESCRIPTOR),
            SemioServiceDescription(service=self.converter,servicer=ConverterServiceServicer,add_Service_to_server=add_ConverterServiceServicer_to_server,descriptor=CONVERTER_DESCRIPTOR),
            SemioServiceDescription(service=self.transformer,servicer=TransformerServiceServicer,add_Service_to_server=add_TransformerServiceServicer_to_server,descriptor=TRANSFORMER_DESCRIPTOR),
            SemioServiceDescription(service=self.translator,servicer=TranslatorServiceServicer,add_Service_to_server=add_TranslatorServiceServicer_to_server,descriptor=TRANSLATOR_DESCRIPTOR)
        ]
        return servicesDescriptions
    
    def initialize(self):
        from .v1.extension_pb2 import Extending
        address = 'localhost:' +str(self.port)
        success, oldAddress = self._getManagerProxy().RegisterExtension(
                extending=Extending(
                    name = self.name,
                    address = address,
                    adaptings = self.adapter._getDescriptions(),
                    convertings = self.converter._getDescriptions(), 
                    transformings = self.transformer._getDescriptions(),
                    translatings = self.translator._getDescriptions()))
        if success:
            logging.info(f'Extension {self.name} ({address}) was successfully registered at manager {self.managerProxyAddress}')
        else:
            logging.info(f'The extension {self.name} ({address}) couldn\'t be registered at manager {self.managerProxyAddress}.'+
             f'Probably there is already an extension registered either at {address} or with name {self.name}. Make sure to set replace existing in the extension registration request to true if you want to override the other extension.')


class ExtensionProxy(SemioProxy):
    def __init__(self,address, **kw):
        super().__init__(address=address,**kw)
        self._adapterStub = AdapterServiceStub(insecure_channel(self.address))
        self._converterStub = ConverterServiceStub(insecure_channel(self.address))
        self._transformerStub = TransformerServiceStub(insecure_channel(self.address))
        self._translatorStub = TranslatorServiceStub(insecure_channel(self.address))

    def RequestPrototype(self, plan: Plan)->Prototype:
        return self._adapterStub.RequestPrototype(plan)

    def RequestConnectionPoint(self, plan:Plan, link:Link, representation: None | Point | Representation  = None)-> Point:
        representationType = link.representationProtocol
        if representationType == REPRESENTATIONPROTOCOL_SIMPLE:
            connectionPointRequest = ConnectionPointRequest(plan=plan,link=link,simple_representation=representation)
        elif representationType == REPRESENTATIONPROTOCOL_FULL:
            connectionPointRequest = ConnectionPointRequest(plan=plan,link=link,full_representation=representation)
        else:
            connectionPointRequest = ConnectionPointRequest(plan=plan,link=link)
        return self._adapterStub.RequestConnectionPoint(connectionPointRequest)

    def ConvertRepresentation(self, representation:Representation, target_platform:Platform)->Representation:
        return self._converterStub.ConvertRepresentation(
            RepresentationConversionRequest(representation=representation,target_platform=target_platform))
    
    def RewriteLayout(self, decisions: Iterable[Decision], initial_layout:Layout | None = None)->Layout:
        return self._transformerStub.RewriteLayout(
            RewriteLayoutRequest(decisions=decisions,initial_layout=initial_layout))

    def TranslateRepresentation(self,representation:Representation, target_pose:Pose ,source_pose:Pose | None = None):
        return self._translatorStub.TranslateRepresentation(
            TranslateRepresentationRequest(representation=representation,target_pose=target_pose,source_pose=source_pose))

