# TODO File should be automatically generated.
import sys
from os import path
from os.path import join
sys.path.append(join(path.dirname(__file__),'v1'))

from .v1.adapter_pb2 import *
from .v1.adapter_pb2_grpc import *
#from .v1.adapter_pb2_grpc import AdapterServiceServicer, add_AdapterServiceServicer_to_server, AdapterServiceStub

from utils import Server

class AdapterService(AdapterServiceServicer):
    

