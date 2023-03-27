from logging import basicConfig, info, debug, DEBUG
from argparse import ArgumentParser

from concurrent import futures
import grpc
from grpc_reflection.v1alpha import reflection
from grpc.aio import ServicerContext

from abc import ABC, abstractclassmethod,abstractmethod
from typing import Tuple, Any, Iterable
from collections.abc import Callable

from pydantic import BaseModel, Field

class SemioService(BaseModel,ABC):
    """This class implements the business logic of the rpc."""
    
class SemioServiceDescription(BaseModel):
    service: SemioService
    servicer:type
    # TODO Update typing to be more specific
    add_Service_to_server: Callable[[Any,Any],Any]
    # TODO Update typing to be more specific
    descriptor: Any

class SemioServer(BaseModel,ABC):
    port: int =  8080
    name: str = ""
    # Set this to True, to find all other services under localhost
    local: bool = False
    
    # TODO replace with abstractclassmethod
    @abstractmethod
    def _getServicesDescriptions(self) -> Iterable[SemioServiceDescription]:
        pass

    def serve(self) -> None:
        """Call this function to start the server."""
        parser = ArgumentParser(
                    prog=self.name,
                    description='The gateway in the semio service landscape.')
        parser.add_argument('-l', '--local', action='store_true',
                            help = 'When local mode is on, all other services will be searched under localhost.') 
        parser.add_argument('-v', '--verbose', action='store_true',
                            help = 'This will set logging level to the lowest (debug) and display all information.') 
        args = parser.parse_args()
        # Set logging
        if args.verbose:
            basicConfig(level=DEBUG)
        else:
            basicConfig()
        if args.local:
            self.initialize(args.local)
            debug('Starting in local mode where other services are supposed to be available under localhost.')

        server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
        server.add_insecure_port('[::]:' + str(self.port))
        for serviceDescription in self._getServicesDescriptions():
            serviceDescription.add_Service_to_server(serviceDescription.service,server)
            serviceName = serviceDescription.servicer.__name__.replace('Servicer','')
            SERVICE_NAMES = (
                serviceDescription.descriptor.services_by_name[serviceName].full_name,
                reflection.SERVICE_NAME,
            )
            reflection.enable_server_reflection(SERVICE_NAMES, server)    
        server.start()
        info(f"{self.name} started, listening on " + str(self.port))
        server.wait_for_termination()
    
    def initialize(self,local=False):
        """Override this method to add additional initialization logic.
        
        Keyword arguments:
        local -- Wheather all other service are running on localhost. (default False)
        """
        return

    class Config:
        arbitrary_types_allowed = True
        extra = 'allow'


# class AsyncServer(BaseModel):
#     port: int =  Field(default=50000,description="Port of server.")
#     name: str = ""
#     services: list[Service] = Field(default=None, description="All services")

#     async def serve(self) -> None:
#         """Call this function to start the server."""
#         server = grpc.aio.server()
#         server.add_insecure_port('[::]:' + str(self.port))
#         for service in self.services:
#             SERVICE_NAMES = (
#                 service.descriptor.services_by_name[service.type.__name__].full_name,
#                 reflection.SERVICE_NAME,
#             )
#             reflection.enable_server_reflection(SERVICE_NAMES, server)    
#         await server.start()
#         print(f"Server {self.name} started, listening on " + str(self.port))
#         await server.wait_for_termination()

#     class Config:
#         arbitrary_types_allowed = True
