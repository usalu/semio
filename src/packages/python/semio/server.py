
from logging import basicConfig, info, debug, DEBUG, NOTSET, CRITICAL, WARN, INFO
from argparse import ArgumentParser, Namespace

from concurrent import futures
import grpc
from grpc_reflection.v1alpha import reflection
from grpc.aio import ServicerContext

from abc import ABC, abstractclassmethod,abstractmethod
from enum import Enum
from typing import Tuple, Any, Iterable

from pydantic import BaseModel, Field

from service import GrpcServiceDescription

logger = basicConfig()

class PreconfiguredEnvironment(Enum):
    NONE = 0
    LOCAL = 1
    DOCKERCOMPOSE = 2
    KUBERNETES = 3

PRECONFIGUREDENVIRONMENTS = {
    None : PreconfiguredEnvironment.NONE,
    'none': PreconfiguredEnvironment.NONE,
    'local': PreconfiguredEnvironment.LOCAL,
    'dockercompose': PreconfiguredEnvironment.DOCKERCOMPOSE,
    'kubernetes' : PreconfiguredEnvironment.KUBERNETES,
}

class LogLevel(Enum):
    NONE = 0
    CRITICAL = 1
    WARN = 2
    INFO = 3
    DEBUG = 4

LOGLEVELS = {
    None : LogLevel.NONE ,
    'none': LogLevel.NONE ,
    'critical' : LogLevel.CRITICAL,
    'warn' : LogLevel.WARN,
    'info': LogLevel.INFO,
    'debug': LogLevel.DEBUG,
}

NATIVELOGLEVELS = {
    LogLevel.NONE : NOTSET,
    LogLevel.CRITICAL : CRITICAL,
    LogLevel.WARN : WARN,
    LogLevel.INFO : INFO,
    LogLevel.DEBUG : DEBUG,
}

class Server(BaseModel,ABC):
    port: int =  80
    name: str = ""
    startOverCli: bool = False
    logLevel: LogLevel = LogLevel.NONE
    preconfiguredEnvironment: PreconfiguredEnvironment = PreconfiguredEnvironment.DOCKERCOMPOSE
    serviceTypes: list[str] = Field(default_factory=list)

    class Config:
        arbitrary_types_allowed = True
        extra = 'allow'

    def modifyArgumentParser(self, argumentParser:ArgumentParser):
        """Override this method to modify the argument parser.
        Add an argument with argumentParser.add_argument(...).
        
        Keyword arguments:
        argumentParser -- argument parser object that is used to parse the arguments later.
        """

    def initializeAfterCli(self,args:Namespace):
        """Override this method to add additional initialization logic after being called over the cli.

        Keyword arguments:
        args -- all arguments collected from the argument parser
        """
        return

    def cli(self):
        """Call this method to prepare for use over command line interface (cli)."""
        argumentParser = ArgumentParser(prog = self.name, description = 'The gateway in the semio service landscape.')
        argumentParser.add_argument('-e', '--preconfiguredEnvironment', choices = ['local','dockercompose','kubernetes'],
                            help = 'Optionally select a preconfigured environment to automagically preconfigure the server.')
        argumentParser.add_argument('-l', '--loglevel', choices = ['critical','info','warning','error','debug'],
                            help = 'Optionally set logging level.\nnone: nothing.  critical: fatal   warn: ups, what, ehhh   info: worked   debug: everything.') 
        self.modifyArgumentParser(argumentParser)
        args = argumentParser.parse_args()
        self.logLevel = LOGLEVELS[args.logLevel]
        self.preconfiguredEnvironment = PRECONFIGUREDENVIRONMENTS[args.preconfiguredEnvironment]
        self.initializeAfterCli(args)

    def initialize(self):
        if self.startOverCli:
            self.cli()
        basicConfig(level=NATIVELOGLEVELS[self.loglevel])

    @abstractmethod
    def serve(self):
        pass

    def main(self):
        self.initialize()
        self.serve()

class GrpcServer(Server):

    # TODO replace with abstractclassmethod
    @abstractmethod
    def _getGrpcServicesDescriptions(self) -> Iterable[GrpcServiceDescription]:
        """ Don't touch this if you don't know exactly what it is for!
        Override this method to add service description information for the gRPC Server.
        This is only a helper method in order to link the generated code from protobuf.
        Due to import cycles this is a method and not a proterty from the class.
        """
        pass

    def initialize(self):
        super().initialize()
        server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
        server.add_insecure_port('[::]:' + str(self.port))
        for serviceDescription in self._getGrpcServicesDescriptions():
            serviceDescription.add_Service_to_server(serviceDescription.service,server)
            serviceName = serviceDescription.servicer.__name__.replace('Servicer','')
            SERVICE_NAMES = (
                serviceDescription.descriptor.services_by_name[serviceName].full_name,
                reflection.SERVICE_NAME,
            )
            reflection.enable_server_reflection(SERVICE_NAMES, server) 
        self.server = server
        debug(f'Initialized: {self}') 

    def serve(self) -> None:
        """Call this function to start the server."""  
        self.server.start()
        info(f"{self.name} started, listening on " + str(self.port))
        self.server.wait_for_termination()


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
