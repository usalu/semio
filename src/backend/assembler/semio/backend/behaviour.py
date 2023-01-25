# # Behaviour that can be dynamically added at runtime (aka monkey patching) to classes
 
# from semio.model import Point as _Point, Pose, Connection as _Connection
# from mathutils import Vector

# class Point(_Point):

#     def __add__(self, o):
#         return Point(x=self.x+o.x,y=self.y+o.y,z=self.z+o.z)

#     def applyTransforms(self,transformLike:TransformationLike):
#         """Apply transformation to a point."""
#         return Vector(matmul(array(TransformationParser.get(transformLike)),array(self.to_4d()))).to_3d()
  
# # Pose behaviour
# def getLocalPointOfView(self:Pose, worldPointOfView:Point,considerPointOfView = True, considerView = True):
#     """
#     Get another point of view from a world perspective in a local perspective.
#     Parameters:
#     worldPointOfViewLike: Point of view from world view.
#     """
#     transformedPointOfView = worldPointOfView
#     if considerPointOfView:
#         transformedPointOfView = applyTransforms(transformedPointOfView,-self.point_of_view)
#     if considerView:
#         transformedPointOfView = applyTransforms(transformedPointOfView,self.view)
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




# #TODO Implement a generic function that gets a local and referential coordinate in the other pose. 
# # def getRelativePointOfView(self, pose,pointOfViewLike:PointOfViewLike,considerPointOfView = True, considerView = True):
# #     """
# #     Get another point of view from a local perspective in a world perspective.
# #     Parameters:
# #     localPointOfViewLike: Point of view from the local view.
# #     """
# #     pointOfView = PointOfViewParser.get(pointOfViewLike)
# #     transformedPointOfView = pointOfView
# #     if considerPointOfView:
# #         transformedPointOfView = applyTransforms(transformedPointOfView,self.pointOfView)
# #     if considerView:
# #         transformedPointOfView = applyTransforms(transformedPointOfView,self.view.inverted())
# #     return applyTransforms(localPointOfView,self._matrix)

# #TODO Implement a generic function that gets a local and referential representation in the other pose. 
# # def getRelativeRepresentation(self, pose,representationLike:RepresentationLike,considerPointOfView = True, considerView = True):
# #     """
# #     Get another representation from a local perspective in a world perspective.
# #     """
# #     representation = RepresentationParser.get(representationLike)
# #     transformedRepresentation = representation
# #     if considerPointOfView:
# #         transformedRepresentation = applyTransforms(transformedRepresentation,self.pointOfView)
# #     if considerView:
# #         transformedRepresentation = applyTransforms(transformedRepresentation,self.view.inverted())
# #     return applyTransforms(localRepresentation,self._matrix)


# #TODO Currently only supports points but should support arbibtray shape. Need for a geometry kernel like pythonOCC.
# def getLocalRepresentation(self, worldRepresentationLike,considerPointOfView = True, considerView = True):
#     return self.getWorldPointOfView(worldRepresentationLike)


# # Connection behaviour

# def connect(self: Connection):
#         """Returns the pose of the connected sobject after getting connected to the connecting.
#         connectedMeetingInRegardsToAttractorMeetingPoint (bool) : If true connected will choose meeting point in regards to meeting point from connected.
#         Otherwise the connected will choose the meeting point in regards to the point of view from the connecting. """

#         connectedConnectionParameters = self.connected.strategy.

#         connectingPointFromAttractor = self.connecting.getConnectionPoint(connectedConnectionParameters)
#         connectingPoint =  self.connecting.pose.getWorldPointOfView(connectingPointFromAttractor)

#         connectingConnectionParameters = self.connectingConnectionProtocol.getConnectionParameters()
#         connectedPointFromAttracted = self.connected.getConnectionPoint(connectingConnectionParameters,self.biasAttracted)
#         #This is the point that will be connected from the connected but only relative from the connected point of view.
#         #The point of view of the connected is irrelevant after the connection points have been received.
#         relativeAttractedPoint = self.connected.pose.getWorldPointOfView(connectedPointFromAttracted,considerPointOfView=False)

#         connectedTargetPointOfViewFromWorld = connectingPoint-relativeAttractedPoint

#         return ConnectionResult(Pose(connectedTargetPointOfViewFromWorld,self.connected.pose.view),
#             connectingPoint,connectingPointFromAttractor,relativeAttractedPoint,connectedPointFromAttracted)