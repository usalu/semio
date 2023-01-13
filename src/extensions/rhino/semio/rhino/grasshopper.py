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

