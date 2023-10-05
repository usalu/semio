from abc import ABC,abstractmethod
from collections.abc import Iterable
from numbers import Number

from semio.geometry import Point

from rhino3dm import (File3dm,
Point as RhinoPoint,Point2d,Point2f,Point3d,Point4d,PointCloud,Vector2d,Vector3d,Vector3f,
Line,LineCurve,Mesh,Plane,
Curve,Polyline,PolylineCurve,Curve,Arc,BezierCurve,Circle,Ellipse,NurbsCurve,
Surface,
BoundingBox,Cone,Cylinder,Extrusion,Sphere,Brep,
Transform)
from multipledispatch import dispatch
from numpy import shape,array,matmul,dot

class Rhino3dmConverter():
    
    @dispatch(Point3d)
    @staticmethod
    def convert(point):
        return Point(x=point.X,y=point.Y,z=point.Z)

    @dispatch(Point)
    @staticmethod
    def convert(point:Point):
        return Point3d(x=point.x,y=point.y,z=point.z)
