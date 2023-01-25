from dataclasses import dataclass,field,InitVar,asdict,is_dataclass
from abc import ABC, abstractmethod
from typing import overload, Sequence, Union, Tuple, Type, Optional, Dict
from numbers import Number
from collections.abc import Iterable
from sys import modules
from inspect import getmembers,isclass
from uuid import uuid4, UUID

from multipledispatch import dispatch
#from pydantic import BaseModel, Field,validator
from dacite import from_dict
from mathutils import Matrix,Vector,Quaternion,Euler
from numpy import shape,array,matmul,dot, allclose
from networkx import Graph,connected_components,chain_decomposition
from ifcopenshell import entity_instance

from .parsers import PointOfViewParser,ViewParser,TransformationParser,ViewLike,PointOfViewLike,TransformationLike

ParameterDict = Dict[str,str]

@dataclass
class Pose():
    """
    A pose describing the point of view and the view of a sobject.
    In CAD terminology this is a plane.
    """
    pointOfView: Vector = field(default=[0,0,0])
    view: Quaternion = field(default=[1,0,0,0])
    #_matrix: Matrix = field(init=False,repr=False,compare=False,hash=False,)
    pointOfViewLike:InitVar[PointOfViewLike] = field(default=[0,0,0])
    viewLike:InitVar[ViewLike] = field(default=[1,0,0,0])
      
    @staticmethod
    def FromMatrix(matrix):
        """
        Construct a pose directly form a matrix.
        """
        newPose = Pose()
        newPose._matrix=matrix
        return newPose

    @property
    def pointOfView(self) -> Vector:
        return self._matrix.to_translation()

    @pointOfView.setter
    def pointOfView(self,pointOfViewLike):
        """The point of view of a sobject. This is the origin of the plane."""
        #A bit hacky to bypass the original initialization but necissary to keep the dataclass for serialization
        #https://stackoverflow.com/questions/51079503/dataclasses-and-property-decorator#51080197
        if type(pointOfViewLike) is property:
            pointOfViewLike = Pose.pointOfViewLike
        pointOfView = PointOfViewParser.get(pointOfViewLike)
        try:
            self._matrix= Pose._mergePointOfViewAndView(pointOfView,self.view)
        except:
            self._matrix = TransformationParser.get(pointOfView)

    @property
    def view(self) -> Quaternion:
        """Defines the view of a sobject. This is a quaternion.
        It does not include the origin transformation"""
        return self._matrix.to_quaternion()
    
    @view.setter
    def view(self,viewLike:ViewLike):
        #A bit hacky to bypass the original initialization but necissary to keep the dataclass for serialization
        #https://stackoverflow.com/questions/51079503/dataclasses-and-property-decorator#51080197
        if type(viewLike) is property:
            viewLike = Pose.viewLike
        view = ViewParser.get(viewLike)
        try:
            self._matrix = Pose._mergePointOfViewAndView(self.pointOfView, view)
        except:
            self._matrix = TransformationParser.get(view)

    def getLocalPointOfView(self, worldPointOfViewLike:PointOfViewLike,considerPointOfView = True, considerView = True):
        """
        Get another point of view from a world perspective in a local perspective.
        Parameters:
        worldPointOfViewLike: Point of view from world view.
        """
        worldPointOfView = PointOfViewParser.get(worldPointOfViewLike)
        transformedPointOfView = worldPointOfView
        if considerPointOfView:
            transformedPointOfView = Pose.applyTransforms(transformedPointOfView,-self.pointOfView)
        if considerView:
            transformedPointOfView = Pose.applyTransforms(transformedPointOfView,self.view)
        return transformedPointOfView

    def getWorldPointOfView(self, localPointOfViewLike:PointOfViewLike,considerPointOfView = True, considerView = True):
        """
        Get another point of view from a local perspective in a world perspective.
        Parameters:
        localPointOfViewLike: Point of view from the local view.
        """
        localPointOfView = PointOfViewParser.get(localPointOfViewLike)
        transformedPointOfView = localPointOfView
        if considerView:
            transformedPointOfView = Pose.applyTransforms(transformedPointOfView,self.view.inverted())
        if considerPointOfView:
            transformedPointOfView = Pose.applyTransforms(transformedPointOfView,self.pointOfView)
        return transformedPointOfView

    @staticmethod
    def applyTransforms(pointOfViewLike:PointOfViewLike,transformLike:TransformationLike):
        """Apply transformation to a point of view."""
        pointOfView = PointOfViewParser.get(pointOfViewLike)
        return Vector(matmul(array(TransformationParser.get(transformLike)),array(pointOfView.to_4d()))).to_3d()


    #TODO Implement a generic function that gets a local and referential coordinate in the other pose. 
    # def getRelativePointOfView(self, pose,pointOfViewLike:PointOfViewLike,considerPointOfView = True, considerView = True):
    #     """
    #     Get another point of view from a local perspective in a world perspective.
    #     Parameters:
    #     localPointOfViewLike: Point of view from the local view.
    #     """
    #     pointOfView = PointOfViewParser.get(pointOfViewLike)
    #     transformedPointOfView = pointOfView
    #     if considerPointOfView:
    #         transformedPointOfView = applyTransforms(transformedPointOfView,self.pointOfView)
    #     if considerView:
    #         transformedPointOfView = applyTransforms(transformedPointOfView,self.view.inverted())
    #     return applyTransforms(localPointOfView,self._matrix)

    #TODO Implement a generic function that gets a local and referential representation in the other pose. 
    # def getRelativeRepresentation(self, pose,representationLike:RepresentationLike,considerPointOfView = True, considerView = True):
    #     """
    #     Get another representation from a local perspective in a world perspective.
    #     """
    #     representation = RepresentationParser.get(representationLike)
    #     transformedRepresentation = representation
    #     if considerPointOfView:
    #         transformedRepresentation = applyTransforms(transformedRepresentation,self.pointOfView)
    #     if considerView:
    #         transformedRepresentation = applyTransforms(transformedRepresentation,self.view.inverted())
    #     return applyTransforms(localRepresentation,self._matrix)


    #TODO Currently only supports points but should support arbibtray shape. Need for a geometry kernel like pythonOCC.
    def getLocalRepresentation(self, worldRepresentationLike,considerPointOfView = True, considerView = True):
        return self.getWorldPointOfView(worldRepresentationLike)

    def close(self, other, relativeTolerance = 0.1 ,absoluteTolerance = 0.01):
        try:
            return allclose(array(self.pointOfView),array(other.pointOfView),relativeTolerance,absoluteTolerance
            ) and allclose(array(self.view),array(other.view),relativeTolerance,absoluteTolerance)
        except AttributeError:
            return False

    @staticmethod
    def _mergePointOfViewAndView(pointOfView,view):
        """Merge a point of view and view to a matrix."""
        prepare = lambda x : array(TransformationParser.get(x))
        return Matrix(matmul(prepare(pointOfView),prepare(view)))

@dataclass
class ElementProxy(ABC):
    """A proxy for a module that will be computed somewhere else."""
    url: str

    @abstractmethod
    def requestEntity(self, parameters):
        """Send a request to the proxy for an ifc entity."""
        pass
    
    @abstractmethod
    def requestConnectionPoint(self,parameters = {}, connectionParameters = {})->Vector:
        """Send a request for a meeting point."""
        pass

    def __init_subclass__(cls,**kwargs):
        """Register the subclasses, so they can be loaded from string if needed."""
        super().__init_subclass__(**kwargs)
        classes[cls.__name__]=cls


@dataclass
class Sobject():
    """A Sobject holds all semantics for an instance of an element object.
    It is a SemiObject because it only has half of the information. The logic is in the element.
    It is almost a subject because it has its on point of view and view, and can interact with other sobjects."""

    elementProxy:ElementProxy
    pose:Pose = field(default_factory=Pose)
    parameters:ParameterDict = field(default_factory=dict)
    id:UUID = field(default_factory=uuid4)
    
    def getConnectionPoint(self, connectionParameters = {}):
        return self.elementProxy.requestConnectionPoint(self.parameters, connectionParameters)
       
    def getEntity(self):
        return self.elementProxy.requestEntity(self.parameters)

@dataclass
class ConnectionProtocol(ABC):
    bias:ParameterDict# = field(default_factory=dict)
    
    def getConnectionParameters(self):
        return toJson(self)
  
@dataclass
class RepresentationBasedConnectionProtocol(ConnectionProtocol,ABC):
    connected: object = field(init=False)
    connectingPose: InitVar[Pose]
    connectedSobject: InitVar[Sobject]

    def __post_init__(self,connectingPose,connectedSobject):
        self.connected = self.getRepresentationFromAttractor(connectingPose,self.getRepresentation(connectedSobject))

    @abstractmethod
    def getRepresentation(self,sobject:Sobject):
        pass
    
    def getRepresentationFromAttractor(self,connectingPose:Pose,representation):
        return connectingPose.getLocalRepresentation(representation)

@dataclass
class SimpleConnectionProtocol(RepresentationBasedConnectionProtocol):

    def getRepresentation(self,sobject:Sobject):
        return sobject.pose.pointOfView
    

@dataclass
class PortConnectionProtocol(ConnectionProtocol):
    type: str
    parameters : ParameterDict

@dataclass
class Connection():
    """An connection defines a relationship between an connecting sobject (connecting) and an connected sobject (connected)."""
    connecting: Sobject
    connected: Sobject
    connectingConnectionProtocol:ConnectionProtocol = field(default_factory=SimpleConnectionProtocol)
    connectedConnectionProtocol:ConnectionProtocol = field(default_factory=SimpleConnectionProtocol)
    
    def connect(self):
        """Returns the pose of the connected sobject after getting connected to the connecting.
        connectedMeetingInRegardsToAttractorMeetingPoint (bool) : If true connected will choose meeting point in regards to meeting point from connected.
        Otherwise the connected will choose the meeting point in regards to the point of view from the connecting. """

        connectedConnectionParameters = self.connectedConnectionProtocol.getConnectionParameters()
        connectingPointFromAttractor = self.connecting.getConnectionPoint(connectedConnectionParameters)
        connectingPoint =  self.connecting.pose.getWorldPointOfView(connectingPointFromAttractor)

        connectingConnectionParameters = self.connectingConnectionProtocol.getConnectionParameters()
        connectedPointFromAttracted = self.connected.getConnectionPoint(connectingConnectionParameters,self.biasAttracted)
        #This is the point that will be connected from the connected but only relative from the connected point of view.
        #The point of view of the connected is irrelevant after the connection points have been received.
        relativeAttractedPoint = self.connected.pose.getWorldPointOfView(connectedPointFromAttracted,considerPointOfView=False)

        connectedTargetPointOfViewFromWorld = connectingPoint-relativeAttractedPoint

        return ConnectionResult(Pose(connectedTargetPointOfViewFromWorld,self.connected.pose.view),
            connectingPoint,connectingPointFromAttractor,relativeAttractedPoint,connectedPointFromAttracted)
    
@dataclass
class ConnectionResult:
    connectedTargetPose:Pose
    connectingPoint:Vector
    connectingPointFromAttractor:Vector
    connectedPointFromAttracted:Vector
    relativeAttractedPoint:Vector
    connectedPointFromAttracted:Vector

@dataclass
class LayoutGraph():
    """A layout graph is an imprecise set of involved sobjects and atrratctions."""
    sobjects : list[Sobject]
    connections : list[Connection]

    def toNXGraph(self):
        G = Graph()
        nodes = [(sobject.id,{'instance':sobject}) for sobject in self.sobjects ]
        edges = [(connection.connecting.id,connection.connected.id,{'instance':connection}) for connection in self.connections ]
        G.add_nodes_from(nodes)
        G.add_edges_from(edges)
        return G
    
    def toTreeChoreography(self,rootSobjects:list[Sobject]=None):
        """Get a Choreography through Breadth First Search (BFS)."""
        graph = self.toNXGraph()
        subGraphs = [graph.subgraph(c).copy() for c in connected_components(graph)]
        for subGraph in subGraphs:
            rootNode = None
            for rootSobject in rootSobjects:
                if rootSobject.id in subGraph:
                    rootNode = rootSobject.id
                    continue
            for chain in chain_decomposition(subGraph,rootNode):
                connection = chain
                
            
@dataclass
class Choreography():
    individualSobjects: list[Sobject]
    connectionChains: list[list[Connection]]
    """A choreography is a precisely instructed order of connections between sobjects."""
    def getPoses(self,updatePointOfViews=True)->Dict[Sobject,Pose]:
        #All individual sobjects stay where they are
        poses = {individualSobject:individualSobject.pose for individualSobject in self.individualSobjects}
        #Can be parallelized
        for connectionChain in self.connectionChains:
            displacement = Vector()
            for connection in connectionChain:
                connectionResult = connection.connect()
                poses[connection.connected] = Pose(connectionResult.connectedTargetPose.pointOfView+displacement,
                    connectionResult.connectedTargetPose.view)
                if updatePointOfViews:
                    displacement += connection.connected.pose.pointOfView-connectionResult.connectedTargetPose.pointOfView
        return poses
    


class Chor(ABC):
    """A chor is a place where the choreograph lays out the choreography.
    This can be any scene like enviroment like a Three.js scene or a file like IFC or Rhino3dm.
    Currently this means all CRUD operations have to be supported.
    Another apporoach would be that one common standard is selected but the drawback of this approach is that all clients need to take care the conversion."""
    def __init__(self) -> None:
        self.initalizeNew()
    
    @abstractmethod
    def initalizeNew(self):
        pass
    @abstractmethod
    def exportChor(self,path):
        pass
    @abstractmethod
    def importChor(self,path):
        pass
    @abstractmethod
    def addShape(self,shape):
        pass
    @abstractmethod
    def updateShape(self,shapeId,shape):
        pass
    @abstractmethod
    def removeShape(self,shapeId):
        pass
    @abstractmethod
    def getShape(self,shapeId):
        pass

class Choreograph:
    """A choreograph is responsible for choreographing the choreography of sobjects inside a chor."""
    def __init__(self, chor,choreography):
        self.chor = chor
        self.choreography = choreography
    

#https://stackoverflow.com/questions/22119850/get-all-class-names-in-a-python-package
classes = {name:cls for name, cls in getmembers(modules[__name__], isclass)}