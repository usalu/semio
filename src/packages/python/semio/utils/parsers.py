from abc import ABC, abstractstaticmethod
from multipledispatch import dispatch
from typing import Union, Tuple
from numbers import Number
from collections.abc import Iterable
from numpy import shape
from math import radians

from semio.geometry import Point,Vector,Quaternion,EulerAngles,NauticAngles,Transform
from semio.model import Pose

# ViewLike = Union[
#     # x part of Euler angles
#     Number,
#     # x and y part of Euler angles
#     Tuple[Number,Number],
#     # (XYZ) Euler angles
#     Tuple[Number,Number,Number],
#     # Quaternion
#     Tuple[Number,Number,Number,Number],
#     # Rotation matrix
#     Tuple[
#         Tuple[Number,Number,Number],
#         Tuple[Number,Number,Number],
#         Tuple[Number,Number,Number],
#     ],
#     Point,Vector,Quaternion,EulerAngles,NauticAngles,Transform]

# PointOfViewLike = Union[
#     # x
#     Number,
#     # x,y
#     Tuple[Number,Number],
#     # x,y,z
#     Tuple[Number,Number,Number],
#     # 4d Vector
#     Tuple[Number,Number,Number,Number],
#     # Transformation matrix with translation part x = [0,3], y = [1,3], z = [2,3]
#     Tuple[
#         Tuple[Number,Number,Number,Number],
#         Tuple[Number,Number,Number,Number],
#         Tuple[Number,Number,Number,Number],
#         Tuple[Number,Number,Number,Number]
#     ],
#     Point,Vector,Transform]

# PoseLike = Union[
#     Tuple[PointOfViewLike,ViewLike],
#     PointOfViewLike,ViewLike]

ViewLike = (
    # x part of Euler angles
    Number,
    # x and y part of Euler angles
    (Number,Number),
    # (XYZ) Euler angles
    (Number,Number,Number),
    # Quaternion
    (Number,Number,Number,Number),
    # Rotation matrix
    (
        (Number,Number,Number),
        (Number,Number,Number),
        (Number,Number,Number),
    ),
    Point,Vector,Quaternion,EulerAngles,NauticAngles,Transform)

PointOfViewLike = (
    # x
    Number,
    # x,y
    (Number,Number),
    # x,y,z
    (Number,Number,Number),
    # 4d Vector
    (Number,Number,Number,Number),
    # Transformation matrix with translation part x = [0,3], y = [1,3], z = [2,3]
    (
        (Number,Number,Number,Number),
        (Number,Number,Number,Number),
        (Number,Number,Number,Number),
        (Number,Number,Number,Number)
    ),
    Point,Vector,Transform)

PoseLike = (
    (PointOfViewLike,ViewLike),
    PointOfViewLike,ViewLike)

def objectToNumber(object):
    return float(str(object))

class Parser(ABC):
    @abstractstaticmethod
    def parse(self):
        pass

class PointOfViewParser(Parser):
    """Parse a point of view from anything like could look like a point."""

    @dispatch(Point)
    def parse(p: Point):
        return p

    @dispatch((Vector,Quaternion,EulerAngles))
    def parse(x):
        return Point(x=x.x,y=x.y,z=x.z)
    
    @dispatch((Number,str))
    def parse(x):
        return Point(x=float(x))

    @dispatch((Number,str),(Number,str))
    def parse(x,y):
        return Point(x=float(x),y=float(y))

    @dispatch((Number,str),(Number,str),(Number,str))
    def parse(x,y,z):
        return Point(x=float(x),y=float(y),z=float(z))

    @dispatch((Number,str),(Number,str),(Number,str),(Number,str))
    def parse(x,y,z,w):
        """Project a 4d vector to a 3d vector"""
        return Point(x=float(x),y=float(y),z=float(z))

    @dispatch(Iterable)
    def parse(pointOfViewLikeList):
        """Get a 4D vector from something point like."""
        if shape(pointOfViewLikeList)==(1,):
            return PointOfViewParser.parse(pointOfViewLikeList[0])
        elif shape(pointOfViewLikeList)==(2,):
            return Point(
                x=PointOfViewParser.parse(pointOfViewLikeList[0]),
                y=PointOfViewParser.parse(pointOfViewLikeList[1]))
        elif shape(pointOfViewLikeList)==(3,):
             return Point(
                x=PointOfViewParser.parse(pointOfViewLikeList[0]),
                y=PointOfViewParser.parse(pointOfViewLikeList[1]),
                z=PointOfViewParser.parse(pointOfViewLikeList[2]))
        elif shape(pointOfViewLikeList)==(4,):
            return Point(
                x=PointOfViewParser.parse(pointOfViewLikeList[0]),
                y=PointOfViewParser.parse(pointOfViewLikeList[1]),
                z=PointOfViewParser.parse(pointOfViewLikeList[2]))
        raise ValueError("The format of the list is not interpretable as a point.")

    # Including Vector,Quaternion,EulerAngles
    @dispatch(object)
    def parse(x):
        try: return Point(
            x = PointOfViewParser.parse(x.x),
            y = PointOfViewParser.parse(x.y),
            z = PointOfViewParser.parse(x.y))
        except: pass
        try: return Point(
            x = PointOfViewParser.parse(x.X),
            y = PointOfViewParser.parse(x.Y),
            z = PointOfViewParser.parse(x.Z))
        except: pass
        try: return Point(
            x = PointOfViewParser.parse(x.u),
            y = PointOfViewParser.parse(x.v),
            z = PointOfViewParser.parse(x.w))
        except: pass
        try: return Point(
            x = PointOfViewParser.parse(x.U),
            y = PointOfViewParser.parse(x.V),
            z = PointOfViewParser.parse(x.W))
        except: pass
        try: return Point(
            x = PointOfViewParser.parse(x.a),
            y = PointOfViewParser.parse(x.b),
            z = PointOfViewParser.parse(x.c))
        except: pass
        try: return Point(
            x = PointOfViewParser.parse(x.A),
            y = PointOfViewParser.parse(x.B),
            z = PointOfViewParser.parse(x.C))
        except: pass
        raise ValueError(f"The type of {str(type(x))} can't be turned into a point (of view).")

    @dispatch(object,object)
    def parse(x,y):
        return Point(x = objectToNumber(x), y = objectToNumber(y))

    @dispatch(object,object,object)
    def parse(x,y,z):
        return Point(x = objectToNumber(x), y = objectToNumber(y), z = objectToNumber(z))

    @dispatch(object,object,object,object)
    def parse(x,y,z,w):
        return Point(x = objectToNumber(x), y = objectToNumber(y), z = objectToNumber(z))


class ViewParser(Parser):
    """Parse a view from anything like could look like a view."""

    @dispatch(Quaternion)
    def parse(q: Quaternion):
        return q

    @dispatch((Point,Vector,Quaternion,EulerAngles))
    def parse(o):
        if o.x == 0 and o.y == 0 and o.z == 0:
            return Quaternion(w=1)
        return Quaternion(x=o.x,y=o.y,z=o.z)

    @dispatch(Iterable)
    def parse(viewLikeList):
        """Get a quaternion from something view like.
        An object needs to either be a representation or have an orientation in it where it can be extracted.
        A list with 3 elements will be interpreted as euler angles."""
        match shape(viewLikeList):
            case (1,):
                return Quaternion(w=ViewParser.parse(viewLikeList[0]))
            case (2,):
                return Quaternion(
                    x=ViewParser.parse(viewLikeList[0]),
                    y=ViewParser.parse(viewLikeList[1]))
            case (3,):
                return Quaternion(
                    x=ViewParser.parse(viewLikeList[0]),
                    y=ViewParser.parse(viewLikeList[1]),
                    z=ViewParser.parse(viewLikeList[2]),)
            case (4,):
                return Quaternion(
                    w=ViewParser.parse(viewLikeList[0]),
                    x=ViewParser.parse(viewLikeList[1]),
                    y=ViewParser.parse(viewLikeList[2]),
                    z=ViewParser.parse(viewLikeList[3]))
            case (3,3) | (4,4) | (3,4):
                # TODO Parse matrix and convert to quaternion.
                raise NotImplementedError()
        raise ValueError("The format of view is not supported.")

class PoseParser(Parser):
    """Parse a pose from anything like could look like a pose."""

    @dispatch(PointOfViewLike)
    def parse(pointOfViewLike):
        pointOfView = PointOfViewParser.parse(pointOfViewLike)
        return Pose(point_of_view=pointOfView)

    @dispatch(ViewLike)
    def parse(viewLike):
        view = ViewParser.parse(viewLike)
        return Pose(view=view)

    @dispatch((PointOfViewLike,ViewLike))
    def parse(pointOfViewLike, viewLike):
        pointOfView = PointOfViewParser.parse(pointOfViewLike)
        view = ViewParser.parse(viewLike)
        return Pose(point_of_view=pointOfView,view=view)