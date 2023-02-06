from multipledispatch import dispatch

from semio.geometry import Point
from semio.model import Pose,Sobject

from mathutils import Matrix,Vector,Quaternion,Euler
from numpy import shape,array,matmul,dot, allclose


@dispatch(Point,Point)
def add(p1:Point,p2:Point):
    Point(x=p1.x+p2.x,y=p1.y+p2.y,z=p1.z+p2.z)

@dispatch(Point,Point)
def subtract(p1:Point,p2:Point):
    Point(x=p1.x-p2.x,y=p1.y-p2.y,z=p1.z-p2.z)

@dispatch(Pose,Point)
def adjustPointOfView(pose:Pose,pointOfView:Point)->Pose:
    return Pose(point_of_view=pointOfView,view=pose.view)

@dispatch(Sobject,Point)
def adjustPointOfView(sobject:Sobject,pointOfView:Point)->Sobject:
    # TODO: Make a function that takes message, a property name and a value and returns a copy of the message with replacing that property and copying all other values automagically. 
    return Sobject(id=sobject.id,pose=adjustPointOfView(sobject.pose),concepts=sobject.concepts)



# @dispatch(Pose)
# def toTransform(pose:Pose):
#     matrix  = Matrix()
#     return matrix

# @dispatch(list)
# def toPoint(l:list):
#     return Point(x=l[0],y=l[1],z=l[2])

# def applyTransforms(pointOfView:Point,transform:Matrix):
#         """Apply transformation to a point of view."""
#         return toPoint(matmul(array(toTransform(Matrix)),array(pointOfView)+[0]))


# def getLocalPointOfView( worldPointOfView:Point, considerPointOfView = True, considerView = True):
#     """
#     Get another point of view from a world perspective in a local perspective.
#     Parameters:
#     worldPointOfViewLike: Point of view from world view.
#     """
#     transformedPointOfView = worldPointOfView
#     if considerPointOfView:
#         transformedPointOfView = Pose.applyTransforms(transformedPointOfView,-self.pointOfView)
#     if considerView:
#         transformedPointOfView = Pose.applyTransforms(transformedPointOfView,self.view)
#     return transformedPointOfView

# def getWorldPointOfView(self, localPointOfViewLike:PointOfViewLike,considerPointOfView = True, considerView = True):
#     """
#     Get another point of view from a local perspective in a world perspective.
#     Parameters:
#     localPointOfViewLike: Point of view from the local view.
#     """
#     localPointOfView = PointOfViewParser.get(localPointOfViewLike)
#     transformedPointOfView = localPointOfView
#     if considerView:
#         transformedPointOfView = Pose.applyTransforms(transformedPointOfView,self.view.inverted())
#     if considerPointOfView:
#         transformedPointOfView = Pose.applyTransforms(transformedPointOfView,self.pointOfView)
#     return transformedPointOfView