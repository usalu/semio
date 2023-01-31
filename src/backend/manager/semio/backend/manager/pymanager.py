from __future__ import annotations
from typing import Iterable,Tuple

import logging

from os.path import splitext
from semio.model import Point,Pose,Platform,Sobject,Connection,Layout,Element,Design, Representation,Platform
from semio.assembler import AssemblerProxy
from semio.manager import ManagerServer,PrototypeRequest,RegisterExtensionRequest, RegisterExtensionResponse
from semio.extension import ExtensionProxy
from semio.constants import PLATFORMURL_BYEXTENSION, GENERAL_EXTENSIONS


def getPlatformUrlFromElementUrl(elementUrl):
    splitElementUrl = splitext(elementUrl)
    fileExtension = splitElementUrl[1]
    if fileExtension in GENERAL_EXTENSIONS:
        fileExtension=splitext(splitElementUrl[0])[1]+fileExtension
    if not fileExtension:
        raise ValueError(f'The element type with url {elementUrl} can\'t determine the type. Use another second level type extension to tell me what platform the element belongs to e.g. ELEMENT.cadquery.py for indicating that the file is a cadquery file.')
    if not fileExtension in PLATFORMURL_BYEXTENSION:
        raise ValueError(f'The element type with ending .{fileExtension} is not supported by me (yet).')
    platformUrl = PLATFORMURL_BYEXTENSION[fileExtension]
    return platformUrl

class Manager(ManagerServer):

    def getAdapterAddress(self, platformName:str) -> str:
        """Get the adapter address for a platform by its name"""
        for extensionAddress, extendingService in self.extensions.items():
            for adaptingService in extendingService.adaptings:
                if adaptingService.platform_name == platformName:
                    return extensionAddress
        raise ValueError(f"No adapting service was found for the {platformName} platform. Register an appropriate extension which can adapt this element.")

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

    def requestPrototype(self, sobject: Sobject, target_representation_platforms: Iterable[Platform] | None = None, target_representation_concepts: Iterable[str] | None = None, target_representation_lods: Iterable[int] | None = None, targets_required: bool = False) -> Element:
        raise NotImplementedError()
    
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