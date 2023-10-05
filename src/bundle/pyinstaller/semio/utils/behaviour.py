from multipledispatch import dispatch

from semio.geometry import Point,Quaternion
from semio.model import Pose,Sobject

from mathutils import Vector as BlenderVector, Quaternion as BlenderQuaternion,Euler as BlenderEuler ,  Matrix as BlenderMatrix
from numpy import shape,array,matmul,dot, allclose

@dispatch(Point,Point)
def add(p1:Point,p2:Point)->Point:
    return Point(x=p1.x+p2.x,y=p1.y+p2.y,z=p1.z+p2.z)

@dispatch(Point,Point)
def subtract(p1:Point,p2:Point)->Point:
    return Point(x=p1.x-p2.x,y=p1.y-p2.y,z=p1.z-p2.z)

@dispatch(Pose,Point)
def adjustPointOfView(pose:Pose,pointOfView:Point)->Pose:
    return Pose(point_of_view=pointOfView,view=pose.view)

@dispatch(Sobject,Point)
def adjustPointOfView(sobject:Sobject,pointOfView:Point)->Sobject:
    # TODO: Make a function that takes message, a property name and a value and returns a copy of the message with replacing that property and copying all other values automagically. 
    return Sobject(id=sobject.id,pose=adjustPointOfView(sobject.pose),concepts=sobject.concepts)

class BlenderMathConverter:

    # Semio to Blender
    @dispatch(Quaternion)
    @staticmethod
    def convertToBlender(view:Quaternion):
        return BlenderQuaternion((view.w,view.x,view.y,view.z))

    @dispatch(Point)
    @staticmethod
    def convertToBlender(point:Point):
        return BlenderVector((point.x,point.y,point.z))

    @dispatch(Pose)
    @staticmethod
    def convertToBlender(pose:Pose):
        origin = BlenderMathConverter.convert(pose.point_of_view)
        quaternion = BlenderMathConverter.convert(pose.view)
        matrix = quaternion.to_matrix().to_4x4()
        matrix[0][3]=origin.x
        matrix[1][3]=origin.y
        matrix[2][3]=origin.z
        return matrix

    # Blender to Semio

    @dispatch(BlenderVector)
    @staticmethod
    def convert(vector:Point):
        return Point(x=vector.x,y=vector.y,z=vector.z)

def applyTransforms(vector:BlenderVector,transform:BlenderMatrix):
        """Apply transformation to a point of view."""
        return BlenderVector(matmul(array(transform),array(vector.to_4d())))

def getLocalPointOfView(pose:Pose ,worldPointOfView:Point, considerPointOfView = True, considerView = True)->Point:
    """
    Get another point of view from a world perspective in a local perspective.
    Parameters:
    worldPointOfViewLike: Point of view from world view.
    """
    transformedPointOfView = BlenderMathConverter.convertToBlender(worldPointOfView)
    if considerPointOfView:
        transformedPointOfView = applyTransforms(transformedPointOfView,BlenderMatrix.Translation(-BlenderMathConverter.convert(pose.point_of_view)))
    if considerView:
        transformedPointOfView = applyTransforms(transformedPointOfView,BlenderMathConverter.convertToBlender(pose.view).to_matrix().to_4x4())
    return BlenderMathConverter.convert(transformedPointOfView)

def getWorldPointOfView(pose:Pose, localPointOfView:Point, considerPointOfView = True, considerView = True)->Point:
    """
    Get another point of view from a local perspective in a world perspective.
    Parameters:
    localPointOfView: Point of view from the local view.
    """
    transformedPointOfView = BlenderMathConverter.convertToBlender(localPointOfView)
    if considerView:
        transformedPointOfView = applyTransforms(transformedPointOfView,BlenderMathConverter.convertToBlender(pose.view).inverted().to_matrix().to_4x4())
    if considerPointOfView:
        transformedPointOfView = applyTransforms(transformedPointOfView,BlenderMatrix.Translation(BlenderMathConverter.convertToBlender(pose.point_of_view)))
    return BlenderMathConverter.convert(transformedPointOfView)