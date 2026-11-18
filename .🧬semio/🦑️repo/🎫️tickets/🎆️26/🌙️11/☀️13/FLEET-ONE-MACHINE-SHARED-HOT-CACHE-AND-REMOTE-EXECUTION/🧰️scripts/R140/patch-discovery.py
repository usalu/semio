import os,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from lib import *
PATH=LIB+"/🏗️build/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs"
def patch(t):
    t=sub1(t,"import { buildRelativeImportProjectV1, buildInputSourceRoleV1,","import { registryOwnerIndexesV1, buildInputSourceRoleV1,")
    t=sub1(t,"const byPackage=new Map(registry.owners.flatMap(owner=>owner.packages.map(name=>[name,owner.name]))),byRoot=new Map(registry.owners.flatMap(owner=>[owner.root,...owner.nativeRoots??[]].map(root=>[root,owner.name]))),owners=registry.owners.flatMap(owner=>[owner.root,...owner.nativeRoots??[]].map(root=>({name:owner.name,root}))).sort((a,b)=>b.root.length-a.root.length);","const {byPackage,byRoot,relativeProject}=registryOwnerIndexesV1(registry);")
    t=sub1(t,"const name=buildRelativeImportProjectV1(owners,path);","const name=relativeProject(path);")
    t=sub1(t,'throw Error("Unknown selected named source input: "+value);','throw refuseBuildV1("invalid","graph.unknown-source-input",{owner:owner.name,input:value});')
    return t
if __name__=="__main__":
    t=read(PATH);n=patch(t);print(sha(t),"->",sha(n));write(sys.argv[1],n)
