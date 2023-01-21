from abc import ABC, abstractstaticmethod
from multipledispatch import dispatch
from typing import Union, Tuple
from numbers import Number
from collections.abc import Iterable
from numpy import shape
from math import radians
from mathutils import Matrix,Vector,Quaternion,Euler


ViewLike = Union[Tuple[Number,Number,Number],Tuple[Number,Number,Number,Number],Tuple[Tuple[Number,Number,Number],Tuple[Number,Number,Number],Tuple[Number,Number,Number],
Matrix,Quaternion,Euler]]
PointOfViewLike = Union[Tuple[Number,Number,Number],Vector]
TransformationLike = Union[ViewLike,PointOfViewLike]

def objectToNumber(object):
    return float(str(object))


class Parser(ABC):
    @abstractstaticmethod
    def get(self):
        pass

class PointOfViewParser(Parser):

    @dispatch(Vector)
    def get(x):
        return x

    @dispatch(Number)
    def get(x):
        return PointOfViewParser.get([x,0,0])

    @dispatch(Number,Number)
    def get(x,y):
        return PointOfViewParser.get([x,y,0])

    @dispatch(Number,Number,Number)
    def get(x,y,z):
        return PointOfViewParser.get([x,y,z])

    @dispatch(Number,Number,Number,Number)
    def get(x,y,z,w):
        """Project a 4d vector to a 3d vector"""
        return PointOfViewParser.get([x,y,z])

    @dispatch(str)
    def get(x):
        return PointOfViewParser.get([float(x),0,0])

    @dispatch(str,str)
    def get(x,y):
        return PointOfViewParser.get([float(x),float(y),0])

    @dispatch(str,str,str)
    def get(x,y,z):
        return PointOfViewParser.get([float(x),float(y),float(z)])

    @dispatch(str,str,str,str)
    def get(x,y,z,w):
        """Project a 4d vector to a 3d vector"""
        return PointOfViewParser.get([float(x),float(y),float(z)])

    @dispatch(Iterable)
    def get(PointOfViewLike):
        """Get a 4D vector from something point like."""
        vector = None
        if isinstance(PointOfViewLike,Vector):
            vector = vector
        elif shape(PointOfViewLike)==(1,):
            vector = Vector([objectToNumber(PointOfViewLike[0]),0,0])
        elif shape(PointOfViewLike)==(2,) or shape(PointOfViewLike)==(3,) or shape(PointOfViewLike)==(4,):
            vector = Vector(PointOfViewLike)
        else:
            raise ValueError("The format of view is not supported.")
        return vector

    @dispatch(object)
    def get(x):
        return PointOfViewParser.get([objectToNumber(x),0,0])

    @dispatch(object,object)
    def get(x,y):
        return PointOfViewParser.get([objectToNumber(x),objectToNumber(y),0])

    @dispatch(object,object,object)
    def get(x,y,z):
        return PointOfViewParser.get([objectToNumber(x),objectToNumber(y),objectToNumber(z)])

    @dispatch(object,object,object,object)
    def get(x,y,z,w):
        """Project a 4d vector to a 3d vector"""
        return PointOfViewParser.get([objectToNumber(x),objectToNumber(y),objectToNumber(z)])

class ViewParser(Parser):

    def get(viewLike:ViewLike):
        """Get a quaternion from something view like.
        An object needs to either be a representation or have an orientation in it where it can be extracted.
        A list with 3 elements will be interpreted as euler angles."""
        quaternion = None
        if isinstance(viewLike,Quaternion):
            quaternion = viewLike
        elif isinstance(viewLike,Matrix) or isinstance(viewLike,Euler):
            quaternion = viewLike.to_quaternion()
        elif shape(viewLike)==(4,):
            quaternion = Quaternion(viewLike)
        elif shape(viewLike)==(3,):
            quaternion = Euler([radians(x) for x in viewLike]).to_quaternion()
        elif shape(viewLike)==(3,3) or shape(viewLike)==(4,4):
            quaternion = Matrix(viewLike).to_quaternion()
        elif shape(viewLike)==(3,4):
            rows = [row[:3] for row in viewLike]
            quaternion = Matrix(rows).to_quaternion()
        else:
            raise ValueError("The format of view is not supported.")
        return quaternion

class TransformationParser(Parser):

    def get(transformationLike:TransformationLike):
        """Get a 4x4 transformation from something transformation like.
        This can be a matrix (2x2,3x3,4x4), quaternion, euler angles or a translation vector.
        If it is a vector of 3 numbers then it will be interpreted as translation vector. Pass Euler(vector)"""
        matrix  = None
        if isinstance(transformationLike,Matrix):
            matrix = transformationLike
        elif isinstance(transformationLike,Quaternion) or isinstance(transformationLike,Euler):
            matrix = transformationLike.to_matrix()
        elif isinstance(transformationLike,Quaternion):
            matrix = transformationLike.to_matrix()
        elif shape(transformationLike)==(3,3) or shape(transformationLike)==(4,4):
            matrix = Matrix(transformationLike)
        elif shape(transformationLike)==(3,4):
            matrix = Matrix(transformationLike+[[0,0,0,1]])
        elif shape(transformationLike)==(3,):
            matrix = Matrix.Translation(transformationLike)
        elif shape(transformationLike)==(4,):
            matrix = Quaternion(transformationLike).to_matrix()
        else:
            raise ValueError("The format of transformationLike is not supported.")
        return matrix.to_4x4()