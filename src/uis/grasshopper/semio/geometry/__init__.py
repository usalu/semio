from semio.geometry.v1.geometry_pb2 import *

from semio.utils.hashing import hashMonkeyPatch, equalityMonkeyPath

Point.__hash__= hashMonkeyPatch
Point.__eq__= equalityMonkeyPath