import logging

from tempfile import TemporaryFile
from base64 import b64decode

from semio.geometry import Point
from semio.model import ENCODING_TEXT_UFT8,PLATFORM_GRASSHOPPER,REPRESENTATIONPROTOCOL_NONE,REPRESENTATIONPROTOCOL_SIMPLE,REPRESENTATIONPROTOCOL_FULL,Representation,Plan,Link,Prototype
from semio.extension import ExtensionServer
from semio.extension.adapter import AdapterService, Adapting
from semio.constants import PLATFORMS, GRASSHOPPER
from semio.utils import hashObject

from grasshopper import callGrasshopper, getOutputParam

def parametersToDict(parameters):
    parametersDictionary = {}
    for parameter in parameters:
        valueType = parameter.value.WhichOneof("value")
        parametersDictionary[parameter.name]=getattr(parameter.value,valueType)
    return parametersDictionary

# def computeResponseToRepresentations(response):
#     representationsTree = getOutputParam(response,'REPRESENTATIONS')
#     representations = []
#     for path,branch in representationsTree.items():
#         for item in branch:
#             representations.append(
#                 Representation(
#                     body=str.encode(item['data'],'utf-8'),
#                     encoding=ENCODING_TEXT_UFT8))
#     return representations

def computeResponseToRepresentations(response):
    representationsTree = getOutputParam(response,'REPRESENTATIONS')
    representations = []
    for path,branch in representationsTree.items():
        for item in branch:
            representation = Representation.FromString(b64decode(item['data']))
            representations.append(representation)
    return representations

class GrasshopperAdapter(AdapterService):
    """An adapter for the REST Endpoint for Grasshopper of the Compute Rhino server."""
    computeUrl:str = "http://localhost:6500/"
    computeAuthToken:str = ""

    def _getDescriptions(self):
        return [Adapting(platform=PLATFORM_GRASSHOPPER)]
    
    def requestConnectionPoint(self, connected_plan: Plan, connecting_link: Link) -> Point:
        parameters = {}
        if connected_plan.parameters:
            parameters.update(parametersToDict(connected_plan.parameters))
        match connecting_link.representationProtocol:
            case 
        if connecting_link.bias_parameters:

        response = callGrasshopper(connected_plan.uri, parameters, self.computeUrl, self.computeUrl)
        representations = computeResponseToRepresentations(response)


    def requestPrototype(self, plan: Plan) -> Prototype:
        parameters = {}
        if plan.parameters:
            parameters.update(parametersToDict(plan.parameters))
        response = callGrasshopper(plan.uri, parameters, self.computeUrl, self.computeUrl)
        representations = computeResponseToRepresentations(response)
        return Prototype(representations=representations,plan_hash=hashObject(plan))

        # representations = [Representation(body=b'Zzzzh',platform=PLATFORM_GRASSHOPPER)]
        # return Prototype(representations=representations)

if __name__=="__main__":
    logging.basicConfig()
    grasshopperServer = ExtensionServer(port=GRASSHOPPER['DEFAULT_PORT'],name='semio.gh')
    grasshopperServer.adapter=GrasshopperAdapter()
    grasshopperServer.serve()