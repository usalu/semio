from __future__ import annotations
from typing import Iterable,Tuple

import logging

from os.path import splitext

from semio.geometry import Point
from semio.model import Pose,Platform,Plan,Sobject,Connection,Layout,Prototype,Element,Design, Representation,Platform
from semio.assembler import AssemblerProxy
from semio.manager import ManagerServer,PrototypeRequest,RegisterExtensionRequest, RegisterExtensionResponse
from semio.extension import ExtensionProxy
from semio.constants import PLATFORM_BYEXTENSION, GENERAL_EXTENSIONS


def getPlatformFromElementUri(elementUri):
    splitElementUri = splitext(elementUri)
    fileExtension = splitElementUri[1]
    if fileExtension in GENERAL_EXTENSIONS:
        fileExtension=splitext(splitElementUri[0])[1]+fileExtension
    if not fileExtension:
        raise ValueError(f'The element type with uri {elementUri} can\'t determine the type. Rename the file with the extension defined by semio for this platform.')
    if not fileExtension in PLATFORM_BYEXTENSION:
        raise ValueError(f'The element type with ending .{fileExtension} is not supported by me (yet).')
    platform = PLATFORM_BYEXTENSION[fileExtension]
    return platform

class Manager(ManagerServer):

    def getAdapterAddress(self, platform:Platform) -> str:
        """Get the adapter address for a platform by its name"""
        for extensionAddress, extendingService in self.extensions.items():
            for adaptingService in extendingService.adaptings:
                if adaptingService.platform == platform:
                    return extensionAddress
        raise ValueError(f"No adapting service was found for the {platform} platform. Register an appropriate extension which can adapt this element.")

    def getConverterAddress(self, source_platform:Platform,target_platform:Platform) -> str:
        for extensionAddress, extendingService in self.extensions.items():
            for convertingService in extendingService.convertings:
                if convertingService.source_platform == source_platform and convertingService.target_type_url == target_platform:
                    return extensionAddress
        raise ValueError(f"No converting service was found that can convert {source_platform} into {target_platform}. Register an appropriate extension which can convert this type.")

    #TODO Implement
    def getTransformerAddress(self,sourceTypeUrl:str,targetTypeUrl:str) -> str:
        raise NotImplementedError()
        # for extendingService in self.services.extendingServices:
        #     for transformingService in extendingService.transformingServices:
        #         if CONDITION:
        #             return extendingService.address
        # raise ValueError(f"No transforming service was found that can transform. Register an appropriate extension which can convert this type.")

    # Services

    def requestPrototype(self, plan: Plan, target_platform: Platform | None = None) -> Prototype:
        adapterAddress =self.getAdapterAddress(getPlatformFromElementUri(plan.uri))
        extensionProxy = self._getExtensionProxy(adapterAddress)
        # TODO Implement target platform logic over checking of response, converters, etc
        return extensionProxy.RequestPrototype(plan)
    
    def connectElement(self, connected_sobject: Sobject, connecting_sobject: Sobject, connection: Connection) -> Tuple[Pose, Point]:
        adapterAddressConnected = self.getAdapterAddress(getPlatformFromElementUri(connected_sobject.plan.uri))
        extensionProxyConnected = self._getExtensionProxy(adapterAddressConnected)
        adapterAddressConnecting = self.getAdapterAddress(getPlatformFromElementUri(connecting_sobject.plan.uri))
        extensionProxyConnecting = self._getExtensionProxy(adapterAddressConnecting)
        
        # TODO Migrate all functions to behaviour.py and import appropriate functions here.
        # connectingPointOfViewFromConnected = getLocalPointOfView(connected_sobject.pose,self.connecting.pose.pointOfView)

        connectedPointFromConnected = extensionProxyConnected.ConnectElement(connected_sobject.plan,connection.connecting.link)
    


        connectedPointFromWorld =  self.connected.pose.getWorldPointOfView(connectedPointFromConnected)
        
        connectedPointOfViewFromConnecting = self.connecting.pose.getLocalPointOfView(self.connected.pose.pointOfView,considerPointOfView=False)
        connectingPointFromConnecting = self.connecting.meetingPoint(connectedPointOfViewFromConnecting,self.biasConnecting)

        #This is the point that will be connecting from the connecting but only relative from the connecting.
        #This is because the point of view of the connecting is irrelevant after the meeting points have been exchanged.
        relativeConnectingPointFromWorld = self.connecting.pose.getWorldPointOfView(connectingPointFromConnecting,considerPointOfView=False)

        connectingTargetPointOfViewFromWorld = connectedPointFromWorld-relativeConnectingPointFromWorld


        return (Pose(point_of_view=connectingTargetPointOfViewFromWorld,view=connecting_sobject.pose),connectedPointFromWorld)



if __name__ == '__main__':
    logging.basicConfig()
    manager = Manager()
    manager.serve()