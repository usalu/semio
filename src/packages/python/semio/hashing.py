import hashlib

def md5(semioObject):
    return hashlib.md5(semioObject.SerializeToString()).hexdigest().upper()

def pyhash(semioObject):
    hash(semioObject.SerializeToString())