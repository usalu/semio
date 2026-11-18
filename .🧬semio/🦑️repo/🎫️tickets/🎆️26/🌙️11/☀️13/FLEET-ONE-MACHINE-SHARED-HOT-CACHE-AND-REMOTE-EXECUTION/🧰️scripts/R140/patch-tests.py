import os,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from lib import *
OBS=LIB+"/🏗️build/🕸️graph/📥️inference/🔎️owner-discovery/🧪️tests/🔁️observation/🟦️.ts"
def patch_observation(t):
    return sub1(t,"buildRelativeImportProjectV1:()=>undefined,","registryOwnerIndexesV1:()=>({byPackage:new Map(),byRoot:new Map(),relativeProject:()=>undefined}),")
RG=LIB+"/🏗️build/🕸️graph/📥️inference/🧪️tests/🔌️providers/🎯️requested-group/🟦️.ts"
ENV=LIB+"/🏗️build/📥️inputs/🌿️environment/🧪️tests/🟦️.ts"
def patch_requested(t):
    return sub1(t,'.rejects.toThrow("Unknown selected named source input");','.rejects.toMatchObject({key:"build.graph.unknown-source-input",values:{input:"scriptProvider:absent-port:scheduling"}});')
def patch_environment(t):
    return sub1(t,'toThrow("Unknown selected named source input: "+row.unknown)','toThrow("build.graph.unknown-source-input")')
