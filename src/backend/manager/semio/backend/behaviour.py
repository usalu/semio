# # Behaviour that can be dynamically added at runtime (aka monkey patching) to classes
 
# from semio.model import Point as _Point, Pose, Attraction as _Attraction
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


# # Attraction behaviour

# def attract(self: Attraction):
#         """Returns the pose of the attracted sobject after getting attracted to the attractor.
#         attractedMeetingInRegardsToAttractorMeetingPoint (bool) : If true attracted will choose meeting point in regards to meeting point from attracted.
#         Otherwise the attracted will choose the meeting point in regards to the point of view from the attractor. """

#         attractedAttractionParameters = self.attracted.strategy.

#         attractorPointFromAttractor = self.attractor.getAttractionPoint(attractedAttractionParameters)
#         attractorPoint =  self.attractor.pose.getWorldPointOfView(attractorPointFromAttractor)

#         attractorAttractionParameters = self.attractorAttractionProtocol.getAttractionParameters()
#         attractedPointFromAttracted = self.attracted.getAttractionPoint(attractorAttractionParameters,self.biasAttracted)
#         #This is the point that will be attracted from the attracted but only relative from the attracted point of view.
#         #The point of view of the attracted is irrelevant after the attraction points have been received.
#         relativeAttractedPoint = self.attracted.pose.getWorldPointOfView(attractedPointFromAttracted,considerPointOfView=False)

#         attractedTargetPointOfViewFromWorld = attractorPoint-relativeAttractedPoint

#         return AttractionResult(Pose(attractedTargetPointOfViewFromWorld,self.attracted.pose.view),
#             attractorPoint,attractorPointFromAttractor,relativeAttractedPoint,attractedPointFromAttracted)