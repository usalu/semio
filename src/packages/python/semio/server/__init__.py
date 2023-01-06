# TODO File should be automatically generated.
import sys
from os import path
from pathlib import Path,PurePath
sys.path.append(str(PurePath(Path(path.dirname(__file__)).parent,('model/gen'))))

from .gen.server_pb2 import *
from .gen.server_pb2_grpc import ServerServicer, add_ServerServicer_to_server, ServerStub