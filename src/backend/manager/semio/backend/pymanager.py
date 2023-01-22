import logging

from os.path import splitext

from semio.model import Point,Sobject,Attraction,Layout,Element,Design, Representation
from semio.assembler import AssemblerProxy,LayoutDesignRequest
from semio.manager import ManagerServer,AttractionRequest,AttractionResponse,ElementRequest,ExtensionRegistrationRequest, ExtensionRegistrationResponse
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

    def getConverterAddress(self, sourceTypeUrl:str,targetTypeUrl:str) -> str:
        for extensionAddress, extendingService in self.extensions.items():
            for convertingService in extendingService.convertingServices:
                if convertingService.source_type_url == sourceTypeUrl and convertingService.target_type_url == targetTypeUrl:
                    return extensionAddress
        raise ValueError(f"No converting service was found that can convert {sourceTypeUrl} into {targetTypeUrl}. Register an appropriate extension which can convert this type.")

    #TODO Implement
    def getTransformerAddress(self,sourceTypeUrl:str,targetTypeUrl:str) -> str:
        raise NotImplementedError()
        # for extendingService in self.services.extendingServices:
        #     for transformingService in extendingService.transformingServices:
        #         if CONDITION:
        #             return extendingService.address
        # raise ValueError(f"No transforming service was found that can transform. Register an appropriate extension which can convert this type.")

    def RequestElement(self, request: ElementRequest, context):
        extensionAddress = self.getAdapterAddress(getPlatformUrlFromElementUrl(request.sobject.url))
        extensionProxy = self.getExtensionProxy(extensionAddress)
        representation = extensionProxy.RequestRepresentation(request.sobject)
        return Element(representations=[representation])

    def RequestAttraction(self, request, context):
        raise NotImplementedError('Method not implemented!')

    def RegisterExtension(self, request: ExtensionRegistrationRequest, context):
        oldAddress= ""
        for extensionAddress, extension in self.extensions.items():
            if extension.name == request.extending.name:
                if request.replace_existing:
                    oldAddress = extensionAddress
                else:
                    raise ValueError(f'There is already an extension with the name {extension.name}. If you wish to replace it set replace existing to true.')
        self.extensions[request.address]=request.extending
        return ExtensionRegistrationResponse(success=True,old_address=oldAddress)

    def GetRegisteredExtensions(self, request, context):
        raise NotImplementedError('Method not implemented!')

if __name__ == '__main__':
    logging.basicConfig()
    manager = Manager()
    manager.serve()