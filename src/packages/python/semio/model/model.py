from multipledispatch import dispatch
from mathutils import Vector as BlenderVector, Quaternion as BlenderQuaternion,Euler as BlenderEuler ,  Matrix as BlenderMatrix
from numpy import shape,array,matmul,dot, allclose

from geometry import Point,Quaternion
from hashing import md5, pyhash

from .v1.model_pb2 import *

Assembly.__hash__ = pyhash
Connectable.__hash__ = pyhash
Connection.__hash__ = pyhash
Decision.__hash__ = pyhash
Design.__hash__ = pyhash
Element.__hash__ = pyhash
Layout.__hash__ = pyhash
LayoutModification.__hash__ = pyhash
LayoutModificationStrategy.__hash__ = pyhash
Link.__hash__ = pyhash
Parameter.__hash__ = pyhash
Plan.__hash__ = pyhash
Pose.__hash__ = pyhash
Prototype.__hash__ = pyhash
Representation.__hash__ = pyhash
Scope.__hash__ = pyhash
Sobject.__hash__ = pyhash
Value.__hash__ = pyhash
Encoding.__hash__ = pyhash
FileType.__hash__ = pyhash
Platform.__hash__ = pyhash
RepresentationProtocol.__hash__ = pyhash
Dependency.__hash__ = pyhash
LayoutStrategy.__hash__ = pyhash

Assembly.hash = md5
Connectable.hash = md5
Connection.hash = md5
Decision.hash = md5
Design.hash = md5
Element.hash = md5
Layout.hash = md5
LayoutModification.hash = md5
LayoutModificationStrategy.hash = md5
Link.hash = md5
Parameter.hash = md5
Plan.hash = md5
Pose.hash = md5
Prototype.hash = md5
Representation.hash = md5
Scope.hash = md5
Sobject.hash = md5
Value.hash = md5
Encoding.hash = md5
FileType.hash = md5
Platform.hash = md5
RepresentationProtocol.hash = md5
Dependency.hash = md5
LayoutStrategy.hash = md5


def setPointOfView(pose:Pose,pointOfView:Point)->Pose:
    return Pose(point_of_view=pointOfView,view=pose.view)

Pose.setPointOfView = setPointOfView

def setPointOfView(sobject:Sobject,pointOfView:Point)->Sobject:
    # TODO: Make a function that takes message, a property name and a value and returns a copy of the message with replacing that property and copying all other values automagically. 
    return Sobject(id=sobject.id,pose=sobject.pose.setPointOfView(pointOfView),concepts=sobject.concepts)

Sobject.setPointOfView = setPointOfView

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
    transformedPointOfView = BlenderMathConverter.convert(worldPointOfView)
    if considerPointOfView:
        transformedPointOfView = applyTransforms(transformedPointOfView,BlenderMatrix.Translation(-BlenderMathConverter.convert(pose.point_of_view)))
    if considerView:
        transformedPointOfView = applyTransforms(transformedPointOfView,BlenderMathConverter.convert(pose.view).to_matrix().to_4x4())
    return BlenderMathConverter.convert(transformedPointOfView)

Pose.getLocalPointOfView = getLocalPointOfView

def getWorldPointOfView(pose:Pose, localPointOfView:Point, considerPointOfView = True, considerView = True)->Point:
    """
    Get another point of view from a local perspective in a world perspective.
    Parameters:
    localPointOfView: Point of view from the local view.
    """
    transformedPointOfView = BlenderMathConverter.convert(localPointOfView)
    if considerView:
        transformedPointOfView = applyTransforms(transformedPointOfView,BlenderMathConverter.convert(pose.view).inverted().to_matrix().to_4x4())
    if considerPointOfView:
        transformedPointOfView = applyTransforms(transformedPointOfView,BlenderMatrix.Translation(BlenderMathConverter.convert(pose.point_of_view)))
    return BlenderMathConverter.convert(transformedPointOfView)

Pose.getWorldPointOfView = getWorldPointOfView