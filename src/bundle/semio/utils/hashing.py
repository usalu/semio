from hashlib import md5

def hashObject(semioObject):
    return md5(semioObject.SerializeToString()).hexdigest()

# Necissary for caching
def hashMonkeyPatch(self):
    return hash(hashObject(self))

# Necissary for caching. Not optimal because protobuf serialization is not normalized.
def equalityMonkeyPath(self, other):
    ss =  self.SerializeToString()
    os = other.SerializeToString()
    e = ss == os
    return e