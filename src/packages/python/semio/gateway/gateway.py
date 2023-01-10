from utils import SemioServer, SemioService
from .v1.gateway_pb2 import DESCRIPTOR
from .v1.gateway_pb2_grpc import add_GatewayServiceServicer_to_server, GatewayServiceServicer

class GatewayServer(SemioServer):
    servicesDescriptions = [SemioService(servicer=GatewayServiceServicer,add_Service_to_server=add_GatewayServiceServicer_to_server,descriptor=DESCRIPTOR)]