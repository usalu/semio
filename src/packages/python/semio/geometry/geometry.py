from .v1.geometry_pb2 import *
from hashing import md5,pyhash

EulerAngles.__hash__ = pyhash
NauticAngles.__hash__ = pyhash
Point.__hash__ = pyhash
Quaternion.__hash__ = pyhash
Transform.__hash__ = pyhash
Vector.__hash__ = pyhash

EulerAngles.hash = md5
NauticAngles.hash = md5
Point.hash = md5
Quaternion.hash = md5
Transform.hash = md5
Vector.hash = md5

def add(p1:Point,p2:Point)->Point:
    return Point(x=p1.x+p2.x,y=p1.y+p2.y,z=p1.z+p2.z)

Point.__add__ = add

def subtract(p1:Point,p2:Point)->Point:
    return Point(x=p1.x-p2.x,y=p1.y-p2.y,z=p1.z-p2.z)

Point.__sub__ = subtract