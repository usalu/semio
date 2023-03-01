import logging

from tempfile import TemporaryFile
from base64 import b64decode

from semio.geometry import Point
from semio.model import ENCODING_TEXT_UFT8,PLATFORM_GRASSHOPPER,REPRESENTATIONPROTOCOL_NONE,REPRESENTATIONPROTOCOL_SIMPLE,REPRESENTATIONPROTOCOL_FULL,Representation,Plan,Link,Prototype
from semio.extension import ExtensionServer
from semio.extension.adapter import AdapterService, Adapting
from semio.constants import PLATFORMS, GRASSHOPPER
from semio.utils import hashObject

from .rhino import Rhino3dmConverter
from .grasshopper import callGrasshopper, getOutputParam, parseSingleItemTree

def parametersToDict(parameters):
    parametersDictionary = {}
    for parameter in parameters:
        valueType = parameter.value.WhichOneof("value")
        parametersDictionary[parameter.name]=getattr(parameter.value,valueType)
    return parametersDictionary

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
    
    def requestConnectionPoint(self, plan: Plan, link: Link, representation: object = None) -> Point:
        parameters = {}
        if plan.parameters:
            parameters.update(parametersToDict(plan.parameters))

        protocol = link.representationProtocol
        if protocol == REPRESENTATIONPROTOCOL_SIMPLE:
            parameters['CONNECTION:CONNECTING']=Rhino3dmConverter.convert(representation)
        elif protocol == REPRESENTATIONPROTOCOL_FULL:
            # TODO Implement
            #representationConnected = 
            raise NotImplementedError()

        if link.bias_parameters:
            parameters.update({'CONNECTION:'+name:key for name,key in parametersToDict(plan.parameters).items()})
        if len(link.ports)>0:
            parameters['CONNECTION:PORTS'] = list(link.ports)
        response = callGrasshopper(plan.uri, parameters, self.computeUrl, self.computeUrl)
        connectionPoint = parseSingleItemTree(getOutputParam(response,'CONNECTION:POINT'))
        return Rhino3dmConverter.convert(connectionPoint)

    def requestPrototype(self, plan: Plan) -> Prototype:
        parameters = {}
        if plan.parameters:
            parameters.update(parametersToDict(plan.parameters))
        response = callGrasshopper(plan.uri, parameters, self.computeUrl, self.computeUrl)
        representations = computeResponseToRepresentations(response)
        return Prototype(representations=representations,plan_hash=hashObject(plan))

        # representations = [Representation(body=b'Zzzzh',platform=PLATFORM_GRASSHOPPER)]
        # return Prototype(representations=representations)

def main():
    logging.basicConfig()
    grasshopperServer = ExtensionServer(port=GRASSHOPPER['DEFAULT_PORT'],name='semio.gh')
    grasshopperServer.adapter=GrasshopperAdapter()
    grasshopperServer.serve()

if __name__=="__main__":
    main()