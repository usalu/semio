from __future__ import annotations
import logging

from pydantic import Field

from logging import debug

from grpc import insecure_channel

from typing import TYPE_CHECKING, Iterable

from socket import gethostbyname,gethostname

from semio.extension.adapter.v1.adapter_pb2 import DESCRIPTOR as ADAPTER_DESCRIPTOR, ConnectionPointRequest
from semio.extension.adapter.v1.adapter_pb2_grpc import add_AdapterServiceServicer_to_server, AdapterServiceServicer, AdapterServiceStub
from semio.extension.converter.v1.converter_pb2 import DESCRIPTOR as CONVERTER_DESCRIPTOR, RepresentationConversionRequest
from semio.extension.converter.v1.converter_pb2_grpc import add_ConverterServiceServicer_to_server, ConverterServiceServicer, ConverterServiceStub
from semio.extension.transformer.v1.transformer_pb2 import DESCRIPTOR as TRANSFORMER_DESCRIPTOR, RewriteLayoutRequest
from semio.extension.transformer.v1.transformer_pb2_grpc import add_TransformerServiceServicer_to_server, TransformerServiceServicer, TransformerServiceStub
from semio.extension.translator.v1.translator_pb2 import DESCRIPTOR as TRANSLATOR_DESCRIPTOR, TranslateRepresentationRequest
from semio.extension.translator.v1.translator_pb2_grpc import add_TranslatorServiceServicer_to_server, TranslatorServiceServicer, TranslatorServiceStub

from semio.geometry import Point
from semio.model import REPRESENTATIONPROTOCOL_NONE,REPRESENTATIONPROTOCOL_SIMPLE,REPRESENTATIONPROTOCOL_FULL,Pose,Platform,Representation,Plan,Link,Sobject,Layout,Decision,Prototype
from semio.utils import SemioServer, SemioServiceDescription, SemioProxy, getAddressFromBaseAndPort
from semio.constants import DEFAULT_MANAGER_PORT

from semio.extension.adapter import AdapterService
from semio.extension.converter import ConverterService
from semio.extension.transformer import TransformerService
from semio.extension.translator import TranslatorService

if TYPE_CHECKING:
    from semio.manager import ManagerProxy

# This import style is necissary to not trigger cyclic imports.
# import manager

class ExtensionServer(SemioServer):
    managerBaseAddress: str = "manager"
    managerPort: int = DEFAULT_MANAGER_PORT
    # These should be abstract classes but pydantic doesn't let you define this without using Union[ALL, SUB, CLASSES]
    # https://stackoverflow.com/questions/58301364/pydantic-and-subclasses-of-abstract-class
    adapter: AdapterService = Field(default_factory=AdapterService)
    converter: ConverterService = Field(default_factory=ConverterService)
    transformer: TransformerService = Field(default_factory=TransformerService)
    translator: TranslatorService = Field(default_factory=TranslatorService)

    def initialize(self,local):
        if local:
            self.managerBaseAddress = 'localhost'
            debug(f'Extension server [{self.name}] initialized in local mode. \n The manager service is supposed to be available under localhost.')
            address = getAddressFromBaseAndPort('localhost',(self.port))
        else:
            address = gethostbyname(gethostname())  + ':' + str(self.port)
        from semio.extension.v1.extension_pb2 import Extending
        
        success, oldAddress = self._getManagerProxy().RegisterExtension(
                extending=Extending(
                    address = address,
                    name = self.name,
                    adaptings = self.adapter._getDescriptions(),
                    convertings = self.converter._getDescriptions(), 
                    transformings = self.transformer._getDescriptions(),
                    translatings = self.translator._getDescriptions()))
        if success:
            logging.info(f'Extension {self.name} ({address}) was successfully registered at manager {getAddressFromBaseAndPort(self.managerBaseAddress,self.managerPort)}')
        else:
            logging.info(f'The extension {self.name} ({address}) couldn\'t be registered at manager {getAddressFromBaseAndPort(self.managerBaseAddress,self.managerPort)}.'+
             f'Probably there is already an extension registered either at {address} or with name {self.name}. Make sure to set replace existing in the extension registration request to true if you want to override the other extension.')
    
    def _getManagerProxy(self):#->ManagerProxy:
        if not hasattr(self,'managerProxy'):
            from semio.manager import ManagerProxy
            self.managerProxy = ManagerProxy(self.managerBaseAddress,self.managerPort)
        return self.managerProxy

    def _getServicesDescriptions(self):
        servicesDescriptions = [
            SemioServiceDescription(service=self.adapter,servicer=AdapterServiceServicer,add_Service_to_server=add_AdapterServiceServicer_to_server,descriptor=ADAPTER_DESCRIPTOR),
            SemioServiceDescription(service=self.converter,servicer=ConverterServiceServicer,add_Service_to_server=add_ConverterServiceServicer_to_server,descriptor=CONVERTER_DESCRIPTOR),
            SemioServiceDescription(service=self.transformer,servicer=TransformerServiceServicer,add_Service_to_server=add_TransformerServiceServicer_to_server,descriptor=TRANSFORMER_DESCRIPTOR),
            SemioServiceDescription(service=self.translator,servicer=TranslatorServiceServicer,add_Service_to_server=add_TranslatorServiceServicer_to_server,descriptor=TRANSLATOR_DESCRIPTOR)
        ]
        return servicesDescriptions

class ExtensionProxy(SemioProxy):
    def __init__(self,baseAddress,port, **kw):
        super().__init__(baseAddress=baseAddress,port=port,**kw)
        address = getAddressFromBaseAndPort(baseAddress,port)
        self._adapterStub = AdapterServiceStub(insecure_channel(address))
        self._converterStub = ConverterServiceStub(insecure_channel(address))
        self._transformerStub = TransformerServiceStub(insecure_channel(address))
        self._translatorStub = TranslatorServiceStub(insecure_channel(address))

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

    #https://stackoverflow.com/questions/2909106/whats-a-correct-and-good-way-to-implement-hash
    def __key(self):
        return (self.baseAddress, self.port)

    def __hash__(self):
        return hash(self.__key())

    def __eq__(self, other):
        if isinstance(other, ExtensionProxy):
            return self.__key() == other.__key()
        return NotImplemented