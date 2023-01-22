from tempfile import TemporaryFile

from semio.model import Point,Representation,Any
from semio.extension import ExtensionServer
from semio.extension.adapter import AdapterService, AttractionPointRequest, RepresentationRequest, RepresentationsRequest, Adapting
from semio.constants import PLATFORMS

from grasshopper import parseModelFromOutput, callGrasshopper, encodeModel

class GrasshopperAdapter(AdapterService):
    """An adapter for the REST Endpoint for Grasshopper of the Compute Rhino server."""
    computeUrl:str = "http://localhost:6500/"
    computeAuthToken:str = ""

    def getDescriptions(self):
        return [Adapting(platform_name="mcneel/rhino/grasshopper")]
    
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
        parameters = {}
        if request.sobject.parameters:
            parameters.update(request.sobject.parameters)
        representationName = 'REPRESENTATION'
        # if request.type != 'native':
        #     representationName+='.'+ request.type
        #model = parseModelFromOutput(callGrasshopper(request.sobject.url,parameters, self.computeUrl, self.computeUrl),representationName)
        #return Representation(byteArray=encodeModel(model),type=PLATFORMS['rhino']['URL'],name=request.name,lod=request.lod)
        return Representation()

    def RequestRepresentations(self, request : RepresentationsRequest, context):
        pass

if __name__=="__main__":
    grasshopperServer = ExtensionServer(port=59002,name='semio.gh', adapter=GrasshopperAdapter())
    grasshopperServer.serve()