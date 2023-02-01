from __future__ import annotations
from abc import ABC,abstractmethod
from typing import TYPE_CHECKING,Iterable,Tuple
from pydantic import Field

import logging

from grpc import insecure_channel

from .v1.manager_pb2 import DESCRIPTOR, PrototypeRequest, ConnectElementRequest, ConnectElementResponse,RegisterExtensionRequest,RegisterExtensionResponse
from .v1.manager_pb2_grpc import add_ManagerServiceServicer_to_server, ManagerServiceServicer, ManagerServiceStub
from model import Point,Pose,Platform,Plan,Sobject,Assembly,Layout,Connection,Prototype,Element,Design
from utils import SemioServer, SemioServiceDescription, SemioProxy, SemioService
from constants import DEFAULT_MANAGER_PORT, DEFAULT_ASSEMBLER_PORT

# Avoid import cycle
if TYPE_CHECKING:
    from semio.extension import Extending

class ManagerServer(SemioServer,SemioService,ABC):
    assemblerAddress: str = "localhost:"+str(DEFAULT_ASSEMBLER_PORT)
    # dict[str,Extending]
    extensions: dict[str,object]= Field(default_factory=dict, description="Extensions with address as key and extension description as value.")
    
    def __init__(self,port = DEFAULT_MANAGER_PORT, name = "Python Semio Manager Server", **kw):
        super().__init__(port=port,name=name, **kw)

    def _getServicesDescriptions(self):
        return [SemioServiceDescription(service=self,servicer=ManagerServiceServicer,add_Service_to_server=add_ManagerServiceServicer_to_server,descriptor=DESCRIPTOR)]

    def _getAssemblerProxy(self):#->AssemblerProxy:
        """Get the assembler proxy. The proxy needs to be created at runtime to avoid cyclic imports between proxies and servers."""
        if not hasattr(self,'assemblerProxy'):
            from assembler import AssemblerProxy
            self.assemblerProxy = AssemblerProxy(self.assemblerAddress)
        return self.assemblerProxy

    def _getExtensionProxy(self,extensionAddress: str):#->ExtensionProxy
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

    @abstractmethod
    def requestPrototype(self, 
        plan:Plan,
        target_platform:Platform | None = None)->Prototype:
        pass

    def RequestPrototype(self, request, context):
        return self.requestPrototype(
            request.plan,
            request.target_platform)
    
    @abstractmethod
    def connectElement(self, 
        connected_sobject: Sobject,
        connecting_sobject:Sobject,
        connection: Connection)->Tuple[Pose,Point]:
        pass

    def ConnectElement(self, request, context):
        connected_element_pose,connection_point = self.connectElement(
            request.connected_sobject,
            request.connecting_sobject,
            request.connection)
        return ConnectElementResponse(connected_element_pose=connected_element_pose,connection_point=connection_point)

    def registerExtension(self, 
        extending,
        replace_existing: bool = True)->Tuple[bool,str]:
        oldAddress= ""
        for extensionAddress, extension in self.extensions.items():
            if extension.name == extending.name:
                if replace_existing:
                    oldAddress = extensionAddress
                else:
                    raise ValueError(f'There is already an extension with the name {extension.name}. If you wish to replace it set replace existing to true.')
        self.extensions[extending.address]=extending
        logging.info(f"Extension: {extending.name} was registered at {extending.address}")
        return (True,oldAddress)

    def RegisterExtension(self,request, context):
        success,oldAddress = self.registerExtension(request.extending,request.replace_existing)
        return RegisterExtensionResponse(success=success,old_address=oldAddress)

    def getRegisteredExtensions(self)->Iterable[Extending]:
        return self.extensions.values()
    
    def GetRegisteredExtensions(self, request, context):
        return self.getRegisteredExtensions


class ManagerProxy(SemioProxy):
    def __init__(self,address ='localhost:'+str(DEFAULT_MANAGER_PORT), **kw):
        super().__init__(address=address,**kw)
        self._stub = ManagerServiceStub(insecure_channel(self.address))

    def RequestPrototype(self,
        plan: Plan,
        target_platform:Platform | None = None,)-> Prototype:
        return self._stub.RequestPrototype(PrototypeRequest(
            plan=plan,
            target_platform=target_platform))

    def ConnectElement(self,
        connected_sobject: Sobject,
        connecting_sobject: Sobject,
        connection:Connection)->Tuple[Pose,Point]:
        connectElementResponse = self._stub.ConnectElement(request=ConnectElementRequest(
            connected_sobject=connected_sobject,
            connecting_sobject=connecting_sobject,
            connection=connection))
        return (connectElementResponse.connected_element_pose,connectElementResponse.connection_point)

    def RegisterExtension(self,extending:Extending, replace_existing=True):
        registerExtensionResponse = self._stub.RegisterExtension(
            request=RegisterExtensionRequest(
                extending=extending,replace_existing=replace_existing))
        return (registerExtensionResponse.success,registerExtensionResponse.old_address)

    def GetRegisteredExtensions(self)->Iterable[Extending]:
        registeredExtensionsResponse = self._stub.GetRegisteredExtensions()
        return registeredExtensionsResponse.extensions


