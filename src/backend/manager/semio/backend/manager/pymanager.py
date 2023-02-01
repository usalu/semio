from __future__ import annotations
from typing import Iterable,Tuple

import logging

from os.path import splitext
from semio.model import Point,Pose,Platform,Plan,Sobject,Connection,Layout,Prototype,Element,Design, Representation,Platform
from semio.assembler import AssemblerProxy
from semio.manager import ManagerServer,PrototypeRequest,RegisterExtensionRequest, RegisterExtensionResponse
from semio.extension import ExtensionProxy
from semio.constants import PLATFORM_BYEXTENSION, GENERAL_EXTENSIONS


def getPlatformFromElementUrl(elementUrl):
    splitElementUrl = splitext(elementUrl)
    fileExtension = splitElementUrl[1]
    if fileExtension in GENERAL_EXTENSIONS:
        fileExtension=splitext(splitElementUrl[0])[1]+fileExtension
    if not fileExtension:
        raise ValueError(f'The element type with url {elementUrl} can\'t determine the type. Use another second level type extension to tell me what platform the element belongs to e.g. ELEMENT.cadquery.py for indicating that the file is a cadquery file.')
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
        adapterAddress =self.getAdapterAddress(getPlatformFromElementUrl(plan.url))
        extensionProxy = self._getExtensionProxy(adapterAddress)
        # TODO Implement target platform logic over checking of response, converters, etc
        return extensionProxy.RequestPrototype(plan)
    
    def connectElement(self, connected_sobject: Sobject, connecting_sobject: Sobject, connection: Connection) -> Tuple[Pose, Point]:
        raise NotImplementedError()
    # def requestElement(self, request: PrototypeRequest, context):
    #     extensionAddress = self.getAdapterAddress(getPlatformUrlFromElementUrl(request.sobject.url))
    #     extensionProxy = self.getExtensionProxy(extensionAddress)
    #     element = extensionProxy.RequestPrototype(request.sobject)
    #     return element

    # def connectElement(self, request, context):
    #     raise NotImplementedError('Method not implemented!')


if __name__ == '__main__':
    logging.basicConfig()
    manager = Manager()
    manager.serve()