from concurrent import futures
import logging

from typing import Tuple

import grpc

from pydantic import Field

from semio.model import Design,Element,Pose,Point

from semio.gateway import (GatewayServer, GatewayServices,
LayoutDesignRequest, ServiceRegistrationRequest,ServiceRegistrationResponse)

class Gateway(GatewayServer):
    services: GatewayServices = Field(default_factory=GatewayServices)

    def getAdapterAddress(self, platformName:str) -> str:
        """Get the adapter address for a platform by its name"""
        for extendingService in self.services.extendingServices:
            for adaptingService in extendingService.adaptingServices:
                if adaptingService.platform_name == platformName:
                    return extendingService.address
        raise ValueError(f"No adapting service was found for the {platformName} platform. Register an appropriate extension which can adapt this element.")

    def getConverterAddress(self, sourceTypeUrl:str,targetTypeUrl:str) -> str:
        for extendingService in self.services.extendingServices:
            for convertingService in extendingService.convertingServices:
                if convertingService.source_type_url == sourceTypeUrl and convertingService.target_type_url == targetTypeUrl:
                    return extendingService.address
        raise ValueError(f"No converting service was found that can convert {sourceTypeUrl} into {targetTypeUrl}. Register an appropriate extension which can convert this type.")

    #TODO Implement
    def getTransformerAddress(self,sourceTypeUrl:str,targetTypeUrl:str) -> str:
        raise NotImplementedError()
        # for extendingService in self.services.extendingServices:
        #     for transformingService in extendingService.transformingServices:
        #         if CONDITION:
        #             return extendingService.address
        # raise ValueError(f"No transforming service was found that can transform. Register an appropriate extension which can convert this type.")

    def LayoutDesign(self, request:LayoutDesignRequest, context) -> Design:
        # layout = request.layout.sobjects
        elements =  [Element(pose=Pose(point_of_view=Point(x=46)))]
        design = Design(elements=elements)
        return design

    def RegisterService(self, request: ServiceRegistrationRequest, context) -> ServiceRegistrationResponse:
        oldAddress = ""
        service = request.WhichOneof('server_service')
        if service == 'managingService':
            if self.services.managingService.ByteSize()!=0 and not request.replace_existing:
                raise ValueError('There is already a manager. If you wish to replace it set replace existing to true.')
            self.services.managingService = request.managingService
        elif service == 'translatingService':
            if self.services.translatingService.ByteSize()!=0 and not request.replace_existing:
                raise ValueError('There is already a translator. If you wish to replace it set replace existing to true.')
            self.services.translatingService = request.translatingService
        elif service == 'extendingService':
            for extendingService in self.services.extendingServices:
                if extendingService.name == request.extendingService.name:
                    if request.replace_existing:
                        oldAddress = extendingService.address
                    else:
                        raise ValueError(f'There is already an extension with the name {extendingService.name}. If you wish to replace it set replace existing to true.')
            self.services.extendingServices.append(request.extendingService)
        return ServiceRegistrationResponse(success=True,old_address=oldAddress)

    def GetRegisteredServices(self, request, context) -> GatewayServices:
        return self.services

if __name__ == '__main__':
    logging.basicConfig()
    gateway = Gateway()
    gateway.serve()