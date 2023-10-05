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

# ViewLike = (
#     # x part of Euler angles
#     Number,
#     # x and y part of Euler angles
#     (Number,Number),
#     # (XYZ) Euler angles
#     (Number,Number,Number),
#     # Quaternion
#     (Number,Number,Number,Number),
#     # Rotation matrix
#     (
#         (Number,Number,Number),
#         (Number,Number,Number),
#         (Number,Number,Number),
#     ),
#     Point,Vector,Quaternion,EulerAngles,NauticAngles,Transform)

# PointOfViewLike = (
#     # x
#     Number,
#     # x,y
#     (Number,Number),
#     # x,y,z
#     (Number,Number,Number),
#     # 4d Vector
#     (Number,Number,Number,Number),
#     # Transformation matrix with translation part x = [0,3], y = [1,3], z = [2,3]
#     (
#         (Number,Number,Number,Number),
#         (Number,Number,Number,Number),
#         (Number,Number,Number,Number),
#         (Number,Number,Number,Number)
#     ),
#     Point,Vector,Transform)

# PoseLike = (
#     (PointOfViewLike,ViewLike),
#     PointOfViewLike,ViewLike)

def objectToNumber(object):
    return float(str(object))

class PointOfViewParser():
    """Parse a point of view from anything that could look like a point."""

    @dispatch()
    def parsePointOfView():
        return Point()

    @dispatch(Point)
    def parsePointOfView(p: Point):
        return p

    @dispatch((Vector,Quaternion,EulerAngles))
    def parsePointOfView(x):
        return Point(x=x.x,y=x.y,z=x.z)
    
    @dispatch((Number,str))
    def parsePointOfView(x):
        return Point(x=float(x))

    @dispatch(dict)
    def parsePointOfView(d):
        try: return Point(
            x = float(d['x']),
            y = float(d['y']),
            z = float(d['z']))
        except: pass
        try: return Point(
            x = float(d['X']),
            y = float(d['Y']),
            z = float(d['Z']))
        except: pass
        try: return Point(
            x = float(d['u']),
            y = float(d['v']),
            z = float(d['w']))
        except: pass
        try: return Point(
            x = float(d['U']),
            y = float(d['V']),
            z = float(d['W']))
        except: pass
        try: return Point(
            x = float(d['a']),
            y = float(d['b']),
            z = float(d['c']))
        except: pass
        try: return Point(
            x = float(d['A']),
            y = float(d['B']),
            z = float(d['C']))
        except: pass
        try: x = float(d['x'])
        except:
            try: x = float(d['X'])
            except:
                try: x = float(d['u'])
                except:
                    try: x = float(d['U'])
                    except:
                        try: x = float(d['b'])
                        except:
                            try: x = float(d['B'])
                            except: x = None
        try: y = float(d['y'])
        except:
            try: y = float(d['Y'])
            except:
                try: x = float(d['v'])
                except:
                    try: x = float(d['V'])
                    except:
                        try: y = float(d['c'])
                        except:
                            try: y = float(d['C'])
                            except: y = None
        try: z = float(d['z'])
        except:
            try: z = float(d['Z'])
            except:
                try: x = float(d['w'])
                except:
                    try: x = float(d['W'])
                    except:
                        try: z = float(d['d'])
                        except:
                            try: z = float(d['D'])
                            except: z = None
        if x or y or z:
            return Point(x=x,y=y,z=z)
        raise ValueError(f"The dictionary can't be turned into a point (of view).")
    
    # Including Vector,Quaternion,EulerAngles
    @dispatch(object)
    def parsePointOfView(x):
        try: return Point(
            x = float(x.x),
            y = float(x.y),
            z = float(x.z))
        except: pass
        try: return Point(
            x = float(x.X),
            y = float(x.Y),
            z = float(x.Z))
        except: pass
        try: return Point(
            x = float(x.u),
            y = float(x.v),
            z = float(x.w))
        except: pass
        try: return Point(
            x = float(x.U),
            y = float(x.V),
            z = float(x.W))
        except: pass
        try: return Point(
            x = float(x.a),
            y = float(x.b),
            z = float(x.c))
        except: pass
        try: return Point(
            x = float(x.A),
            y = float(x.B),
            z = float(x.C))
        except: pass
        try: x = float(x.x)
        except:
            try: x = float(x.X)
            except:
                try: x = float(x.u)
                except:
                    try: x = float(x.U)
                    except:
                        try: x = float(x.b)
                        except:
                            try: x = float(x.B)
                            except: x = None
        try: y = float(x.y)
        except:
            try: y = float(x.Y)
            except:
                try: x = float(x.v)
                except:
                    try: x = float(x.V)
                    except:
                        try: y = float(x.c)
                        except:
                            try: y = float(x.C)
                            except: y = None
        try: z = float(x.z)
        except:
            try: z = float(x.Z)
            except:
                try: x = float(x.w)
                except:
                    try: x = float(x.W)
                    except:
                        try: z = float(x.d)
                        except:
                            try: z = float(x.D)
                            except: z = None
        if x or y or z:
            return Point(x=x,y=y,z=z)
        raise ValueError(f"The type of {str(type(x))} can't be turned into a point (of view).")

    @dispatch((Number,str),(Number,str))
    def parsePointOfView(x,y):
        return Point(x=float(x),y=float(y))

    @dispatch(object,object)
    def parsePointOfView(x,y):
        return Point(x = objectToNumber(x), y = objectToNumber(y))

    @dispatch((Number,str),(Number,str),(Number,str))
    def parsePointOfView(x,y,z):
        return Point(x=float(x),y=float(y),z=float(z))

    @dispatch(object,object,object)
    def parsePointOfView(x,y,z):
        return Point(x = objectToNumber(x), y = objectToNumber(y), z = objectToNumber(z))

    @dispatch((Number,str),(Number,str),(Number,str),(Number,str))
    def parsePointOfView(x,y,z,w):
        """Project a 4d vector to a 3d vector"""
        return Point(x=float(x),y=float(y),z=float(z))

    @dispatch(object,object,object,object)
    def parsePointOfView(x,y,z,w):
        return Point(x = objectToNumber(x), y = objectToNumber(y), z = objectToNumber(z))

    @dispatch(Iterable)
    def parsePointOfView(pointOfViewLikeList):
        """Get a 4D vector from something point like."""
        if shape(pointOfViewLikeList)==(1,):
            return Point(x = float(pointOfViewLikeList[0]))
        elif shape(pointOfViewLikeList)==(2,):
            return Point(
                x=float(pointOfViewLikeList[0]),
                y=float(pointOfViewLikeList[1]))
        elif shape(pointOfViewLikeList)==(3,):
             return Point(
                x=float(pointOfViewLikeList[0]),
                y=float(pointOfViewLikeList[1]),
                z=float(pointOfViewLikeList[2]))
        elif shape(pointOfViewLikeList)==(4,):
            return Point(
                x=float(pointOfViewLikeList[0]),
                y=float(pointOfViewLikeList[1]),
                z=float(pointOfViewLikeList[2]))
        raise ValueError("The format of the list is not interpretable as a point.")


class ViewParser():
    """Parse a view from anything that could look like a view."""

    @dispatch()
    def parseView():
        return Quaternion()

    @dispatch(Quaternion)
    def parseView(q: Quaternion):
        return q

    @dispatch((Number,str))
    def parseView(w: (Number,str)):
        return Quaternion(w = float(w))

    @dispatch((Point,Vector,Quaternion,EulerAngles))
    def parseView(o):
        if o.x == 0 and o.y == 0 and o.z == 0:
            return Quaternion(w=1)
        return Quaternion(x=o.x,y=o.y,z=o.z)

    @dispatch(dict)
    def parseView(d):
        try: return Quaternion(
            w = float(d['w']),
            x = float(d['x']),
            y = float(d['y']),
            z = float(d['z']))
        except: pass
        try: return Quaternion(
            w = float(d['W']),
            x = float(d['X']),
            y = float(d['Y']),
            z = float(d['Z']))
        except: pass
        try: return Quaternion(
            w = float(d['a']),
            x = float(d['b']),
            y = float(d['c']),
            z = float(d['d']))
        except: pass
        try: return Quaternion(
            w = float(d['A']),
            x = float(d['B']),
            y = float(d['C']),
            z = float(d['D']))
        except: pass
        try: w = float(d['w'])
        except:
            try: w = float(d['W'])
            except:
                try: w = float(d['a'])
                except:
                    try: w = float(d['A'])
                    except: w = None
        try: x = float(d['x'])
        except:
            try: x = float(d['X'])
            except:
                try: x = float(d['b'])
                except:
                    try: x = float(d['B'])
                    except: x = None
        try: y = float(d['y'])
        except:
            try: y = float(d['Y'])
            except:
                try: y = float(d['c'])
                except:
                    try: y = float(d['C'])
                    except: y = None
        try: z = float(d['z'])
        except:
            try: z = float(d['Z'])
            except:
                try: z = float(d['d'])
                except:
                    try: z = float(d['D'])
                    except: z = None
        if w or x or y or z:
            return Quaternion(w=w,x=x,y=y,z=z)
        raise ValueError(f"The dictionary can't be turned into a view.")

    @dispatch(object)
    def parseView(x):
        try: return Quaternion(
            w = float(x.w),
            x = float(x.x),
            y = float(x.y),
            z = float(x.z))
        except: pass
        try: return Quaternion(
            w = float(x.W),
            x = float(x.X),
            y = float(x.Y),
            z = float(x.Z))
        except: pass
        try: return Quaternion(
            w = float(x.a),
            x = float(x.b),
            y = float(x.c),
            z = float(x.d))
        except: pass
        try: return Quaternion(
            w = float(x.A),
            x = float(x.B),
            y = float(x.C),
            z = float(x.D))
        except: pass
        try: w = float(x.w)
        except:
            try: w = float(x.W)
            except:
                try: w = float(x.a)
                except:
                    try: w = float(x.A)
                    except: w = None
        try: x = float(x.x)
        except:
            try: x = float(x.X)
            except:
                try: x = float(x.b)
                except:
                    try: x = float(x.B)
                    except: x = None
        try: y = float(x.y)
        except:
            try: y = float(x.Y)
            except:
                try: y = float(x.c)
                except:
                    try: y = float(x.C)
                    except: y = None
        try: z = float(x.z)
        except:
            try: z = float(x.Z)
            except:
                try: z = float(x.d)
                except:
                    try: z = float(x.D)
                    except: z = None
        if w or x or y or z:
            return Quaternion(w=w,x=x,y=y,z=z)
        raise ValueError(f"The type of {str(type(x))} can't be turned into a view.")


    @dispatch((Number,str),(Number,str))
    def parseView(x: (Number,str),y : (Number,str)):
        return Quaternion(x = float(x), y = float(y))

    @dispatch((Number,str),(Number,str),(Number,str))
    def parseView(x: (Number,str),y : (Number,str), z: (Number,str)):
        return Quaternion(x = float(x), y = float(y), z = float(z))

    @dispatch((Number,str),(Number,str),(Number,str),(Number,str))
    def parseView(w: (Number,str), x: (Number,str),y : (Number,str), z: (Number,str)):
        return Quaternion(w = float(w), x = float(x), y = float(y), z = float(z))

    @dispatch(Iterable)
    def parseView(viewLikeList):
        match shape(viewLikeList):
            case (1,):
                return Quaternion(w=float(viewLikeList[0]))
            case (2,):
                return Quaternion(
                    x=float(viewLikeList[0]),
                    y=float(viewLikeList[1]))
            case (3,):
                return Quaternion(
                    x=float(viewLikeList[0]),
                    y=float(viewLikeList[1]),
                    z=float(viewLikeList[2]),)
            case (4,):
                return Quaternion(
                    w=float(viewLikeList[0]),
                    x=float(viewLikeList[1]),
                    y=float(viewLikeList[2]),
                    z=float(viewLikeList[3]))
            case (3,3) | (4,4) | (3,4):
                # TODO Parse matrix and convert to quaternion.
                raise NotImplementedError()
        raise ValueError("The format of view is not supported.")

class PoseParser():
    """Parse a pose from anything that could look like a point of view and a view."""

    @dispatch()
    def parsePose():
        return Pose()

    @dispatch(Pose)
    def parsePose(p: Pose):
        return p

    @dispatch(object)
    def parsePose(pointOfViewLike):
        pointOfView = PointOfViewParser.parsePointOfView(pointOfViewLike)
        return Pose(point_of_view=pointOfView)

    @dispatch(object,object)
    def parsePose(pointOfViewLike, viewLike):
        pointOfView = PointOfViewParser.parsePointOfView(pointOfViewLike)
        view = ViewParser.parseView(viewLike)
        return Pose(point_of_view=pointOfView,view=view)