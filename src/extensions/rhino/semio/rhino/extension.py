from logging import basicConfig
from concurrent.futures import ThreadPoolExecutor
from grpc import server
from semio.model import Point
from semio.extensionservices.adapter import AttractionPointRequest, RepresentationRequest, RepresentationsRequest, AdapterServicer, add_AdapterServicer_to_server

import compute_rhino3d
import compute_rhino3d.Grasshopper as gh
from rhino3dm import Point3d,CommonObject, Brep
from json import loads,dumps,JSONDecodeError

from pydantic import BaseModel

def encode(value):
    if type(value).__module__.startswith('rhino3dm'):
        return dumps(value.Encode())
    else:
        return value

def convertParameter(parameter):
    if type(parameter) is Point:
        return Point3d(parameter.x,parameter.y,parameter.z)
    else:
        return parameter

def prepareParameter(parameter):
    return encode(convertParameter(parameter))

def convertToTrees(parameters):
    trees = []
    if type(parameters) is not dict:
        return "Parameters have to be a dictionary"
    
    for kw in parameters:
        branches = parameters[kw]
        tree = gh.DataTree(kw)
        #check if branches is just item
        if type(branches) != list:
            tree.Append([0], [prepareParameter(branches)])
        else:
            #check if branches is list
            if type(branches[0]) != list:
                tree.Append([0], [prepareParameter(item) for item in branches])
            else:
                #branches must be a "real" tree with many branches
                for i,branch in enumerate(branches):
                    tree.Append([i], [prepareParameter(item) for item in branch])
        trees.append(tree)
    return trees

def parseSingleValue(valueReply):
    try:
        if 'type' in valueReply:
            valueType =  valueReply['type']
            data =  loads(valueReply['data'])
            if valueType.endswith('Point3d'):
                return Point3d(data['X'],data['Y'],data['Z'])
            else:
                return CommonObject.Decode(data)
        return valueReply
    except:
        return valueReply

def parseSingleResults(valuesReply):
    results = {}
    log = []
    for valueReply in valuesReply['values']:
        tree = valueReply['InnerTree']
        if len(tree)!=1:
            log.append("Tree did not have exactly one 1 branch" +valueReply)
            continue
        value =list(tree.values())[0]
        if len(value)!=1:
            log.append("Branch did not have exactly one 1 value")
        results[valueReply['ParamName']]=parseSingleValue(value[0])
    return (results,log)

"""Simplified call with dictionaries as input."""
def callGrasshopper(path,parameters, computeUrl = "http://localhost:6500/", computeAuthToken= ""):
    trees = convertToTrees(parameters)
    try:
        compute_rhino3d.Util.url = computeUrl
        compute_rhino3d.Util.authToken = computeAuthToken
        return gh.EvaluateDefinition(path, trees)
    except JSONDecodeError as e:
            raise ValueError("Probably the path was wrong. Make sure to give an absolute path otherwise the call will most likely fail. This is the log\n" + str(e))
    except Exception as e: return "The call to compute went wrong. " + str(e)


# TODO Make proper request with warnings, errors, argument checking, etc
class GrasshopperAdapter(AdapterServicer):
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


class GrasshopperExtensionServer(BaseModel):
    port: int = 59001

    def serve(self):
        basicConfig()
        ghServer = server(ThreadPoolExecutor(max_workers=10))
        add_AdapterServicer_to_server(GrasshopperAdapter(), ghServer)
        ghServer.add_insecure_port('[::]:' + str(self.port))
        ghServer.start()
        print("Server started, listening on " + str(self.port))
        ghServer.wait_for_termination()

if __name__=="__main__":
    ghServer = GrasshopperExtensionServer()
    ghServer.serve()