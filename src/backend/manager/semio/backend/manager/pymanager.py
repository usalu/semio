from __future__ import annotations
from typing import Iterable,Tuple

import logging

from os.path import splitext
from functools import lru_cache

from semio.geometry import Point
from semio.model import (RepresentationProtocol,Pose,Platform,Plan,Sobject,Connection,Layout,Prototype,Element,Design, Representation,Platform,Link,
                         REPRESENTATIONPROTOCOL_SIMPLE,REPRESENTATIONPROTOCOL_FULL)
from semio.assembler import AssemblerProxy
from semio.manager import ManagerServer,PrototypeRequest,RegisterExtensionRequest, RegisterExtensionResponse
from semio.extension import ExtensionProxy
from semio.constants import PLATFORM_BYEXTENSION, GENERAL_EXTENSIONS

from semio.utils import getLocalPointOfView, getWorldPointOfView, subtract, hashObject

# In order to be able to cache objects, a hash function is monkey patched
def hashMonkeyPatch(self):
    return hash(hashObject(self))

Pose.__hash__= hashMonkeyPatch
Plan.__hash__= hashMonkeyPatch
Link.__hash__= hashMonkeyPatch

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

# TODO Add cache invalidation when extension changes.

# @lru_cache()
def requestPrototypeCached(extensionProxy:ExtensionProxy, plan:Plan, target_platform: Platform | None = None) -> Prototype:
    return extensionProxy.RequestPrototype(plan)

# TODO Update ugly passing of representation of connecting. Reason: Sobject should be passed but for easy caching, it is no longer available
def getRepresentation(connected_sobject_pose: Pose, representationProtocol: RepresentationProtocol, connecting : Point | None = None):
    if representationProtocol == REPRESENTATIONPROTOCOL_SIMPLE:
        # Representation is the point of view from the connecting from the pose of the connected.
        assert isinstance(connecting,Point)
        return getLocalPointOfView(connected_sobject_pose,connecting)
    elif representationProtocol == REPRESENTATIONPROTOCOL_FULL:
        return connecting
    else:
        return None

# @lru_cache()
def connectElementCached(
    extensionProxyConnected:ExtensionProxy, 
    extensionProxyConnecting:ExtensionProxy, 
    connected_sobject_pose: Pose,
    connected_sobject_plan: Plan, 
    connecting_sobject_pose: Pose,
    connecting_sobject_plan: Plan, 
    connected_link: Link,
    connecting_link: Link
    ) -> Tuple[Pose, Point]:
    
   
    representationConnecting = getRepresentation(connected_sobject_pose,connected_link.representationProtocol,connecting_sobject_pose.point_of_view)
    connectionPointFromConnected = extensionProxyConnected.RequestConnectionPoint(connected_sobject_plan,connected_link,representationConnecting)
    connectionPointFromWorld = getWorldPointOfView(connected_sobject_pose,connectionPointFromConnected)
    
    representationConnected = getRepresentation(connecting_sobject_pose,connecting_link.representationProtocol,connecting_sobject_pose.point_of_view)
    connectionPointFromConnecting = extensionProxyConnecting.RequestConnectionPoint(connecting_sobject_plan,connecting_link,representationConnected)
    relativeConnectionPointFromConnectedFromWorld = getWorldPointOfView(connecting_sobject_pose,connectionPointFromConnecting,False)

    connectingTargetPointOfView = subtract(connectionPointFromWorld,relativeConnectionPointFromConnectedFromWorld)
    return (Pose(point_of_view=connectingTargetPointOfView,view=connecting_sobject_pose.view),connectionPointFromWorld)

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
        return requestPrototypeCached(extensionProxy,plan,target_platform)

    def connectElement(self, connected_sobject: Sobject, connecting_sobject: Sobject, connection: Connection) -> Tuple[Pose, Point]:
        adapterAddressConnected = self.getAdapterAddress(getPlatformFromElementUri(connected_sobject.plan.uri))
        extensionProxyConnected = self._getExtensionProxy(adapterAddressConnected)
        adapterAddressConnecting = self.getAdapterAddress(getPlatformFromElementUri(connecting_sobject.plan.uri))
        extensionProxyConnecting = self._getExtensionProxy(adapterAddressConnecting)

        return connectElementCached(extensionProxyConnected,extensionProxyConnecting,
            connected_sobject.pose,
            connected_sobject.plan,
            connecting_sobject.pose,
            connecting_sobject.plan,
            connection.connected.link,
            connection.connecting.link)

def main():
    logging.basicConfig()
    manager = Manager()
    manager.serve()

if __name__ == '__main__':
    main()