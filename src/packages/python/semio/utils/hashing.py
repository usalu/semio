from hashlib import md5

def hashObject(semioObject):
    return md5(semioObject.SerializeToString()).hexdigest()