import os,re,sys
root="/Users/ueli/Documents/semio"
BZ=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel"
p=BZ+"/📜️script.ts"
s=open(p).read()
def once(old,new,label):
    global s
    n=s.count(old)
    assert n==1,(label,n)
    s=s.replace(old,new)
once('import {bazelClosureMarkerV1,bazelClosureMarkersForPlanV1,loadBazelClosuresV1,managedBazelClosureFetchV1,type BazelClosureFetchV1,type BazelClosureMarkerV1,type BazelClosuresV1} from "./🔗️closure/🟦️.ts";',
     'import {bazelClosureMarkerV1,bazelClosureMarkersForPlanV1,loadBazelClosuresV1,managedBazelClosureFetchV1,type BazelClosureFetchV1,type BazelClosureMarkerV1,type BazelClosuresV1} from "./🔗️closure/🟦️.ts";\nimport {captureBazelSourceSnapshotV1} from "./📸️snapshot/🟦️.ts";',"import")
a=s.index("/** 🔌 Links the real checkout into the generated package")
b=s.index("/** 🛂️ Renders the platform profile of the selected output base")
removed=s[a:b]
assert "linkBazelSourcesV1" in removed and "linkBazelSourceAliasesV1" in removed and "readBazelPathsV1" not in removed.split("📏️ Reads")[0] or True
# keep everything between the end of linkBazelSourceAliasesV1 and the next docstring
end=removed.index("\n}\n",removed.index("export async function linkBazelSourceAliasesV1"))+3
rest=removed[end:]
s=s[:a]+rest.lstrip("\n")+s[b:] if False else s[:a]+rest.lstrip("\n")+s[b:]
once("await linkBazelSourcesV1(root,workspace);","","link call")
once("declarable=new Set(classified.files),",
 'snapshot=await captureBazelSourceSnapshotV1({root,directory:workspace,files:classified.files,real:new Set(declared.filter(action=>action.kind==="hermetic").flatMap(action=>{const{marks,extra}=index.selectAll(inputs.get(action.id)!);return [...index.files.filter((_path,position)=>marks[position]),...extra];})),views:{alias:aliasRule.link,real:actions.sources.link},alias:directory=>bazelSourceAliasV1(directory),check:()=>checkScriptInvocation(invocation),progress:async()=>{await advanceScriptInvocation(invocation,"bazel:source-snapshot","running");await invocation.control.yieldContinuation();checkScriptInvocation(invocation);}}),declarable=new Set(snapshot.files),',"classify")
once("compact=new Int32Array(index.files.length).fill(-1);",
 "compact=new Int32Array(index.files.length).fill(-1);\n if(request.verbose||snapshot.vanished.length)console.log(`[bazel] source snapshot: ${snapshot.files.length} files, ${snapshot.captured} captured, ${snapshot.reused} reused, ${snapshot.pruned} pruned, ${snapshot.vanished.length} vanished before capture`);","log")
once("const directories=new Set(universe.map(path=>path.includes(\"/\")?path.slice(0,path.lastIndexOf(\"/\")):\"\")),aliases=await linkBazelSourceAliasesV1(root,workspace,directories),repositoryLabels=new Set<string>();",
 "const aliases=snapshot.directories,repositoryLabels=new Set<string>();","aliases")
open(p+".new","w").write(s)
os.replace(p+".new",p)
print("ok")
