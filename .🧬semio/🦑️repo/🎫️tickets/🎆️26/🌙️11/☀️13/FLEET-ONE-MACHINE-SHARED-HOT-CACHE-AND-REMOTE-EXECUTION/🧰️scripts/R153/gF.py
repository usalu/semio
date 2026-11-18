import sys,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from stagelib import *
g=Group("F")
p=L+"/🧪️tests/🕸️graph/🔌️nx-engine/🟦️.ts"
g.sub(p,'import {expect,test} from "bun:test";','import {expect,test} from "bun:test";\nimport {privateRepositoryWorkspaceV1} from "../../../../🧪️testing/🧫️private-repository/🟦️.ts";')
g.sub(p,',externalNodes:{"npm:nx":{name:"npm:nx",type:"npm",data:{packageName:"nx",version:"23.2.0",hash:nx}},"npm:typescript":{name:"npm:typescript",type:"npm",data:{packageName:"typescript",version:"5.9.3",hash:"typescript-fixed"}}},',',externalNodes:{...Object.fromEntries(nxBuildEngineInputsV1().flatMap((value:any)=>value?.externalDependencies??[]).map((name:string)=>["npm:"+name,{name:"npm:"+name,type:"npm",data:{packageName:name,version:"1.0.0",hash:"fixed-"+name}}])),"npm:nx":{name:"npm:nx",type:"npm",data:{packageName:"nx",version:"23.2.0",hash:nx}},"npm:typescript":{name:"npm:typescript",type:"npm",data:{packageName:"typescript",version:"5.9.3",hash:"typescript-fixed"}}},')
g.sub(p,'test("an Nx target caches only when every output lies in a partition the cache hook can hold, equal to the actual output classifier",()=>{\n const root=repositoryRootV1,','test("an Nx target caches only when every output lies in a partition the cache hook can hold, equal to the actual output classifier (in the private repository of this test, never the live store of the checkout)",async()=>{\n const root=(await privateRepositoryWorkspaceV1()).root,')
g.save()
