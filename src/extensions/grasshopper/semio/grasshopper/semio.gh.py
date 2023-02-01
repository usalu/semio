import logging

from tempfile import TemporaryFile

from semio.model import Point,PLATFORM_GRASSHOPPER,Representation,Plan,Link,Prototype
from semio.extension import ExtensionServer
from semio.extension.adapter import AdapterService, Adapting
from semio.constants import PLATFORMS, GRASSHOPPER

from grasshopper import callGrasshopper, filterOutputParams

def parametersToDict(parameters):
    parametersDictionary = {}
    for parameter in parameters:
        valueType = parameter.value.WhichOneof("value")
        parametersDictionary[parameter.name]=getattr(parameter.value,valueType)
    return parametersDictionary


class GrasshopperAdapter(AdapterService):
    """An adapter for the REST Endpoint for Grasshopper of the Compute Rhino server."""
    computeUrl:str = "http://localhost:6500/"
    computeAuthToken:str = ""

    def _getDescriptions(self):
        return [Adapting(platform=PLATFORM_GRASSHOPPER)]
    
    def requestConnectionPoint(self, connected_plan: Plan, connecting_link: Link) -> Point:
        return Point(x=-5)

    def requestPrototype(self, plan: Plan) -> Prototype:
        parameters = {}
        if plan.parameters:
            parameters.update(parametersToDict(plan.parameters))
        representationsDict = filterOutputParams(callGrasshopper(plan.url, parameters, self.computeUrl, self.computeUrl),'REPRESENTATION')
        representations = []
        # for name, tree in representationsDict.items():
            
        representations = [Representation(body=b'Zzzzh',platform=PLATFORM_GRASSHOPPER)]
        return Prototype(representations=representations)

if __name__=="__main__":
    logging.basicConfig()
    grasshopperServer = ExtensionServer(port=GRASSHOPPER['DEFAULT_PORT'],name='semio.gh')
    grasshopperServer.adapter=GrasshopperAdapter()
    grasshopperServer.serve()