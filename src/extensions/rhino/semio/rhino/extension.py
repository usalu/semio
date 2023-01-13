from logging import basicConfig
from concurrent.futures import ThreadPoolExecutor
from grpc import server, insecure_channel

from semio.model import Point
from semio.extension import AdapterService
from semio.utils import Semio


# TODO Make proper request with warnings, errors, argument checking, etc
class GrasshopperAdapter(BaseModel, AdapterService):
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
    rhinoServer = ExtensionServer(5900 ,ExtendingService(
            name='semio.rhino', 
            address=f'localhost:{self.port}',
            adaptingServices=[
                AdaptingService(platform_name='mcneel/rhino'),
                AdaptingService(platform_name='mcneel/grasshopper')
            ]
        ))
    rhinoServer.serve()