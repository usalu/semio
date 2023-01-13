from semio.model import Point
from semio.extension import ExtensionServer
from semio.extension.adapter import AdapterService, AttractionPointRequest, RepresentationRequest, RepresentationsRequest

from .grasshopper import parseSingleResults, parseSingleValue, prepareParameter, callGrasshopper

class GrasshopperAdapter(AdapterService):
    """An adapter for the REST Endpoint for Grasshopper3D of the Compute Rhino server."""
    computeUrl:str = "http://localhost:6500/"
    computeAuthToken:str = ""
    
    def RequestAttractionPoint(self, request : AttractionPointRequest, context):
        parameters = {}
        attractedRepresentation = request.attracted_attractionStrategy.representation.body
        if attractedRepresentation.body:
            parameters['ATTRACTED']= attractedRepresentation.body.ToJsonString()
        if request.attracted_attractionStrategy.port:
            parameters['PORT']= request.attracted_attractionStrategy.port.ToJsonString()
        meetingPoint = parseSingleResults(callGrasshopper(request.attractor_url,parameters, self.computeUrl, self.computeUrl))[0]['ATTRACTIONPOINT']
        return Point(meetingPoint.X,meetingPoint.Y,meetingPoint.Z)

    def RequestRepresentation(self, request : RepresentationRequest, context):
        parameters = {}
        if request.sobject.parameters:
            parameters.update(request.sobject.parameters)
        representation = parseSingleResults(callGrasshopper(request.sobject.url, self.computeUrl, self.computeUrl))[0]['REPRESENTATION.'+request.type]
        return representation

    def RequestRepresentations(self, request : RepresentationsRequest, context):
        pass

       

if __name__=="__main__":
    rhinoServer = ExtensionServer(5900,name='semio.rhino', adapters=[GrasshopperAdapter()])=
    rhinoServer.serve()