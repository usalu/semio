from tempfile import TemporaryFile

from semio.model import Point,PLATFORM_GRASSHOPPER,Representation,Plan,Link,Prototype
from semio.extension import ExtensionServer
from semio.extension.adapter import AdapterService, Adapting
from semio.constants import PLATFORMS, GRASSHOPPER

from grasshopper import parseModelFromOutput, callGrasshopper, encodeModel

class GrasshopperAdapter(AdapterService):
    """An adapter for the REST Endpoint for Grasshopper of the Compute Rhino server."""
    computeUrl:str = "http://localhost:6500/"
    computeAuthToken:str = ""

    def getDescriptions(self):
        return [Adapting(platform=PLATFORM_GRASSHOPPER)]
    
    def requestConnectionPoint(self, connected_plan: Plan, connecting_link: Link) -> Point:
        return Point(x=-5)

    def requestPrototype(self, plan: Plan) -> Prototype:
        parameters = {}
        if plan.parameters:
            parameters.update({ parameter.name:parameter.number for parameter in plan.parameters})
        representationName = 'REPRESENTATION'
        # if request.type != 'native':
        #     representationName+='.'+ request.type
        #model = parseModelFromOutput(callGrasshopper(request.sobject.url,parameters, self.computeUrl, self.computeUrl),representationName)
        #return Representation(byteArray=encodeModel(model),type=PLATFORMS['rhino']['URL'],name=request.name,lod=request.lod)
        return Prototype(representations=[Representation(body=b'Zzz')])

if __name__=="__main__":
    grasshopperServer = ExtensionServer(port=GRASSHOPPER['DEFAULT_PORT'],name='semio.gh', adapter=GrasshopperAdapter())
    grasshopperServer.serve()