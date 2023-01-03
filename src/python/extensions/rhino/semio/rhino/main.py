from semio.model import Point

print(dir())

# from logging import basicConfig
# from concurrent.futures import ThreadPoolExecutor
# from grpc import server
# from semio.model import Point
# from semio.extensionservices.adapter import AttractionPointRequest, RepresentationRequest, RepresentationsRequest, AdapterServicer, add_AdapterServicer_to_server

# from dataclasses import dataclass,field, InitVar

# import compute_rhino3d
# import compute_rhino3d.Grasshopper as gh
# from rhino3dm import Point3d,CommonObject, Brep
# from json import loads,dumps,JSONDecodeError

# def encode(value):
#     if type(value).__module__.startswith('rhino3dm'):
#         return dumps(value.Encode())
#     else:
#         return value

# def convertParameter(parameter):
#     if type(parameter) is Point:
#         return Point3d(parameter.x,parameter.y,parameter.z)
#     else:
#         return parameter

# def prepareParameter(parameter):
#     return encode(convertParameter(parameter))

# def convertToTrees(parameters):
#     trees = []
#     if type(parameters) is not dict:
#         return "Parameters have to be a dictionary"
    
#     for kw in parameters:
#         branches = parameters[kw]
#         tree = gh.DataTree(kw)
#         #check if branches is just item
#         if type(branches) != list:
#             tree.Append([0], [prepareParameter(branches)])
#         else:
#             #check if branches is list
#             if type(branches[0]) != list:
#                 tree.Append([0], [prepareParameter(item) for item in branches])
#             else:
#                 #branches must be a "real" tree with many branches
#                 for i,branch in enumerate(branches):
#                     tree.Append([i], [prepareParameter(item) for item in branch])
#         trees.append(tree)
#     return trees

# def parseSingleValue(valueReply):
#     try:
#         if 'type' in valueReply:
#             valueType =  valueReply['type']
#             data =  loads(valueReply['data'])
#             if valueType.endswith('Point3d'):
#                 return Point3d(data['X'],data['Y'],data['Z'])
#             else:
#                 return CommonObject.Decode(data)
#         return valueReply
#     except:
#         return valueReply

# def parseSingleResults(valuesReply):
#     results = {}
#     log = []
#     for valueReply in valuesReply['values']:
#         tree = valueReply['InnerTree']
#         if len(tree)!=1:
#             log.append("Tree did not have exactly one 1 branch" +valueReply)
#             continue
#         value =list(tree.values())[0]
#         if len(value)!=1:
#             log.append("Branch did not have exactly one 1 value")
#         results[valueReply['ParamName']]=parseSingleValue(value[0])
#     return (results,log)

# """Simplified call with dictionaries as input."""
# def callGrasshopper(path,parameters):
#     trees = convertToTrees(parameters)
#     try:
#         return gh.EvaluateDefinition(path, trees)
#     except JSONDecodeError as e:
#             raise ValueError("Probably the path was wrong. Make sure to give an absolute path otherwise the call will most likely fail. This is the log\n" + str(e))
#     except Exception as e: return "The call to compute went wrong. " + str(e)


# # TODO Make proper request with warnings, errors, argument checking, etc
# @dataclass
# class GrasshopperAdapter(AdapterServicer):
#     """An adapter for the REST Endpoint for Grasshopper3D of the Compute Rhino server."""

#     computeUrl: str
#     computeAuthToken:str
#     _computeUrl:InitVar[str] = field(init=False,repr=False,default ="http://localhost:6500/")
#     _computeAuthToken:InitVar[str] = field(init=False,repr=False,default ="")
    
#     @property
#     def computeUrl(self):
#         return self._computeUrl
    
#     @computeUrl.setter
#     def computeUrl(self,computeUrl):
#         #A bit hacky to bypass the original initialization
#         #https://stackoverflow.com/questions/51079503/dataclasses-and-property-decorator#51080197
#         if type(computeUrl) is property:
#             computeUrl = GrasshopperAdapter._computeUrl
#         compute_rhino3d.Util.url = computeUrl
#         self._computeUrl = computeUrl

#     @property
#     def computeAuthToken(self):
#         return self._computeAuthToken
    
#     @computeAuthToken.setter
#     def computeAuthToken(self,computeAuthToken):
#         if type(computeAuthToken) is property:
#             computeAuthToken = GrasshopperAdapter._computeAuthToken
#         compute_rhino3d.Util.authToken = computeAuthToken
#         self._computeAuthToken = computeAuthToken

#     def RequestAttractionPoint(self, request : AttractionPointRequest, context):
#         parameters = {}
#         attractedRepresentation = request.attracted_attractionStrategy.representation.body
#         if attractedRepresentation.body:
#             parameters['ATTRACTED']= attractedRepresentation.body.ToJsonString()
#         if request.attracted_attractionStrategy.port:
#             parameters['PORT']= request.attracted_attractionStrategy.port.ToJsonString()
#         meetingPoint = parseSingleResults(callGrasshopper(request.attractor_url,parameters))[0]['ATTRACTIONPOINT']
#         return Point(meetingPoint.X,meetingPoint.Y,meetingPoint.Z)

#     def RequestRepresentation(self, request : RepresentationRequest, context):
#         parameters = {}
#         if request.sobject.parameters:
#             parameters.update(request.sobject.parameters)
#         representation = parseSingleResults(callGrasshopper(request.sobject.url,))[0]['REPRESENTATION.'+request.type]
#         return representation

#     def RequestRepresentations(self, request : RepresentationsRequest, context):
#         pass


# def serve():
#     port = '50051'
#     ghServer = server(ThreadPoolExecutor(max_workers=10))
#     add_AdapterServicer_to_server(GrasshopperAdapter(), ghServer)
#     ghServer.add_insecure_port('[::]:' + port)
#     ghServer.start()
#     print("Server started, listening on " + port)
#     ghServer.wait_for_termination()

# if __name__ == '__main__':
#     basicConfig()
#     serve()