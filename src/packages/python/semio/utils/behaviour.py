from multipledispatch import dispatch

from semio.geometry import Point,Quaternion
from semio.model import Pose,Sobject

from mathutils import Vector as BlenderVector, Quaternion as BlenderQuaternion,Euler as BlenderEuler ,  Matrix as BlenderMatrix
from numpy import shape,array,matmul,dot, allclose

class BlenderMathConverter:

    # Semio to Blender
    @dispatch(Quaternion)
    @staticmethod
    def convert(view:Quaternion):
        return BlenderQuaternion((view.w,view.x,view.y,view.z))

    @dispatch(Point)
    @staticmethod
    def convert(point:Point):
        return BlenderVector((point.x,point.y,point.z))

    @dispatch(Pose)
    @staticmethod
    def convert(pose:Pose):
        origin = convert(pose.point_of_view)
        quaternion = convert(pose.view)
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
        return Vector(matmul(array(transform),array(vector.to_4d())))

def getLocalPointOfView(pose:Pose ,worldPointOfView:Point, considerPointOfView = True, considerView = True)->Point:
    """
    Get another point of view from a world perspective in a local perspective.
    Parameters:
    worldPointOfViewLike: Point of view from world view.
    """
    transformedPointOfView = BlenderMathConverter.convert(worldPointOfView)
    if considerPointOfView:
        transformedPointOfView = applyTransforms(transformedPointOfView,BlenderMatrix.Translation(-BlenderMathConverter.convert(pose.point_of_view)))
    if considerView:
        transformedPointOfView = applyTransforms(transformedPointOfView,BlenderMathConverter.convert(pose.view).to_matrix().to_4x4())
    return convert(transformedPointOfView)

def getWorldPointOfView(pose:Pose, localPointOfView:Point, considerPointOfView = True, considerView = True)->Point:
    """
    Get another point of view from a local perspective in a world perspective.
    Parameters:
    localPointOfView: Point of view from the local view.
    """
    transformedPointOfView = convert(localPointOfView)
    if considerView:
        transformedPointOfView = applyTransforms(transformedPointOfView,BlenderMathConverter.convert(pose.view).inverted().to_matrix().to_4x4())
    if considerPointOfView:
        transformedPointOfView = applyTransforms(transformedPointOfView,BlenderMatrix.Translation(BlenderMathConverter.convert(pose.point_of_view)))
    return BlenderMathConverter.convert(transformedPointOfView)