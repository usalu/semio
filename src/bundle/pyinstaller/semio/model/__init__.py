from semio.model.v1.model_pb2 import *

from semio.utils.hashing import hashMonkeyPatch, equalityMonkeyPath

RepresentationProtocol.__hash__= hashMonkeyPatch
RepresentationProtocol.__eq__= equalityMonkeyPath
Pose.__hash__= hashMonkeyPatch
Pose.__eq__= equalityMonkeyPath
Platform.__hash__= hashMonkeyPatch
Platform.__eq__= equalityMonkeyPath
Plan.__hash__= hashMonkeyPatch
Plan.__eq__= equalityMonkeyPath
Sobject.__hash__= hashMonkeyPatch
Sobject.__eq__= equalityMonkeyPath
Connection.__hash__= hashMonkeyPatch
Connection.__eq__= equalityMonkeyPath
Layout.__hash__= hashMonkeyPatch
Layout.__eq__= equalityMonkeyPath
Prototype.__hash__= hashMonkeyPatch
Prototype.__eq__= equalityMonkeyPath
Element.__hash__= hashMonkeyPatch
Element.__eq__= equalityMonkeyPath
Design.__hash__= hashMonkeyPatch
Design.__eq__= equalityMonkeyPath
Representation.__hash__= hashMonkeyPatch
Representation.__eq__= equalityMonkeyPath
Platform.__hash__= hashMonkeyPatch
Platform.__eq__= equalityMonkeyPath
Link.__hash__= hashMonkeyPatch
Link.__eq__= equalityMonkeyPath