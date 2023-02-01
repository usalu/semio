from tempfile import TemporaryFile

from semio.model import Point,Representation,Any, ENCODING_TEXT_UFT32, FILETYPE_JSON, PLATFORM_SEMIO
from semio.extension import ExtensionServer
from semio.extension.adapter import AdapterService, ConnectionPointRequest, RepresentationRequest, RepresentationsRequest, Adapting
from semio.constants import PLATFORMS

class DummyAdapter(AdapterService):
    """An adapter for the REST Endpoint for Dummy of the Compute Rhino server."""
    computeUrl:str = "http://localhost:6500/"
    computeAuthToken:str = ""

    def _getDescriptions(self):
        return [
            Adapting(platform_url="dummy/verycomplexplatform"),
            Adapting(platform_url='dummy/anothercomplexplatform')
            ]
    
    def RequestConnectionPoint(self, request : ConnectionPointRequest, context):
        return Point(x=20,y=-6,z=78)

    def RequestRepresentation(self, request : RepresentationRequest, context):
        return Representation(body='{"k":\"I am beatiful!\"}'.encode('uft-32'),encoding=ENCODING_TEXT_UFT32,file_type=FILETYPE_JSON)

    def RequestRepresentations(self, request : RepresentationsRequest, context):
        pass

if __name__=="__main__":
    grasshopperServer = ExtensionServer(port=65000,name='semio.gh', adapter=DummyAdapter())
    grasshopperServer.serve()