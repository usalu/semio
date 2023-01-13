from semio.model import Point,Representation
from semio.extension import ExtensionServer
from semio.extension.adapter import AdapterService, AttractionPointRequest, RepresentationRequest, RepresentationsRequest, Adapting

from .grasshopper import parseSingleResults, parseSingleValue, prepareParameter, callGrasshopper

class GrasshopperAdapter(AdapterService):
    """An adapter for the REST Endpoint for Grasshopper of the Compute Rhino server."""
    computeUrl:str = "http://localhost:6500/"
    computeAuthToken:str = ""

    def getDescription(self):
        return Adapting(platform_name="mcneel/rhino/compute/grasshopper")
    
    def RequestAttractionPoint(self, request : AttractionPointRequest, context):
        # parameters = {}
        # attractedRepresentation = request.attracted_attractionStrategy.representation.body
        # if attractedRepresentation.body:
        #     parameters['ATTRACTED']= attractedRepresentation.body.ToJsonString()
        # if request.attracted_attractionStrategy.port:
        #     parameters['PORT']= request.attracted_attractionStrategy.port.ToJsonString()
        # meetingPoint = parseSingleResults(callGrasshopper(request.attractor_url,parameters, self.computeUrl, self.computeUrl))[0]['ATTRACTIONPOINT']
        # return Point(meetingPoint.X,meetingPoint.Y,meetingPoint.Z)
        return Point()

    def RequestRepresentation(self, request : RepresentationRequest, context):
        # parameters = {}
        # if request.sobject.parameters:
        #     parameters.update(request.sobject.parameters)
        # representation = parseSingleResults(callGrasshopper(request.sobject.url, self.computeUrl, self.computeUrl))[0]['REPRESENTATION.'+request.type]
        # return representation
        return Representation()

    def RequestRepresentations(self, request : RepresentationsRequest, context):
        pass

if __name__=="__main__":
    rhinoServer = ExtensionServer(59002,name='semio.rhino', adapters=[GrasshopperAdapter()])
    rhinoServer.serve()