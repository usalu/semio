from tempfile import TemporaryDirectory
from os.path import join

from pydantic import BaseModel

from json import loads,dumps,JSONDecodeError

from rhino3dm import (File3dm,CommonObject,ObjectAttributes,
Point,Point2d,Point2f,Point3d,Point4d,PointCloud,Vector2d,Vector3d,Vector3f,
Line,LineCurve,Mesh,Plane,
Curve,Polyline,PolylineCurve,Curve,Arc,BezierCurve,Circle,Ellipse,NurbsCurve,
Surface,
BoundingBox,Cone,Cylinder,Extrusion,Sphere,Brep,
Transform)

import compute_rhino3d
import compute_rhino3d.Grasshopper as gh
from compute_rhino3d.Util import DecodeToCommonObject, DecodeToPoint3d, DecodeToLine, DecodeToBoundingBox, DecodeToVector3d


def encode(value):
    if type(value).__module__.startswith('rhino3dm'):
        return dumps(value.Encode())
    else:
        return value

def prepareParameter(parameter):
    return encode(parameter)

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

def parseSingleItemTree(tree):
    assert len(tree)==1
    values =list(tree.values())[0]
    assert len(values)==1
    result=decode(values[0])
    return result

decodeDict= {
    "Rhino.Geometry.Point3d": DecodeToPoint3d,
    "Rhino.Geometry.Vector3d": DecodeToVector3d,
    # 'Rhino.Geometry.Box': DecodeToCommonObject,
    'Rhino.Geometry.PolylineCurve': DecodeToCommonObject
}
def decode(item):
    data = loads(item['data'])
    type = item['type']
    if not type in decodeDict:
        raise ValueError(f'The item type {type} is currently not supported.')
    return decodeDict[type](data)

def addObjectToModel(item,model):
    if type(item) in [Point, Point2d, Point2f, Point2d, Point3d, Point4d]:
        model.Objects.AddPoint(item)
    elif type(item) in [PointCloud]:
        model.Objects.AddPointCloud(item)
    elif type(item) in [Line]:
        model.Objects.AddLine(item)
    elif type(item) in [Polyline]:
        model.Objects.AddPolyline(item)
    elif type(item) in [Arc]:
        model.Objects.AddArc(item)
    elif type(item) in [Circle]:
        model.Objects.AddCircle(item)
    elif type(item) in [Ellipse]:
        model.Objects.AddEllipse(item)
    elif type(item) in [Sphere]:
        model.Objects.AddSphere(item)
    elif type(item) in [Curve, PolylineCurve]:
        model.Objects.AddCurve(item)
    elif type(item) in [Surface]:
        model.Objects.AddSurface(item)
    elif type(item) in [Extrusion]:
        model.Objects.AddExtrusion(item)
    elif type(item) in [Mesh]:
        model.Objects.AddMesh(item)
    elif type(item) in [Brep]:
        model.Objects.AddBrep(item)
    elif type(item) in [Surface]:
        model.Objects.AddSurface(item)
    else:
        model.Objects.Add(item) 


def parseModelFromOutput(grasshopperReply, outputName):
    model = File3dm()
    valuesDictionary = { valueReply['ParamName']:valueReply['InnerTree'] for valueReply in grasshopperReply['values']}
    if not outputName in valuesDictionary:
        raise ValueError(f'Output name {outputName} was not in grasshopper reply.')
    # rhino3dm doesn't let you write attributes to all objects...
    representationLayerIndex = model.Layers.AddLayer(outputName,(0,0,0,255))
    representationObjectAttributes = ObjectAttributes()
    for branch in valuesDictionary[outputName].values():
        for item in branch:
            try:
                decodedObject = decode(item)
                addObjectToModel(decodedObject,model)
            except Exception as e:
                print(e)
    return model

def encodeModel(model):
    temp = TemporaryDirectory()
    tempFile = join(temp.name,'model.3dm')
    model.Write(tempFile)
    byteModel = None
    with open(tempFile, 'rb') as fd:
        byteModel = fd.read()
    temp.cleanup()
    return byteModel

# def filterOutputParams(grasshopperReply, outputBaseName):
#     valuesDictionary = { valueReply['ParamName']:valueReply['InnerTree'] for valueReply in grasshopperReply['values']}
#     reducedOutput = {}
#     for key, value in valuesDictionary.items():
#         if key.startswith(outputBaseName):
#             reducedOutput[key]=value
#     return reducedOutput

def getOutputParam(grasshopperReply, outputName):
    valuesDictionary = { valueReply['ParamName']:valueReply['InnerTree'] for valueReply in grasshopperReply['values']}
    if outputName in valuesDictionary:
        return valuesDictionary[outputName]
    raise ValueError(f"Outputname {outputName} was not in the results.")

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

