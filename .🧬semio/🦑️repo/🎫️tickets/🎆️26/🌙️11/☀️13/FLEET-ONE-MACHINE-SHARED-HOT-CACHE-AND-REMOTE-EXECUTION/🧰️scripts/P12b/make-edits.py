#!/usr/bin/env python3
"""P12b: writes edits.py from the live files (whole-file anchors for the P12 files this package replaces, small anchors for the others)."""
import os,sys
sys.path.insert(0,"../P12")
import edits as base
L="/Users/Shared/semio/"
LIB=base.LIB;G=base.G;RES=base.RES;STAGE=base.STAGE
ENV=G+"/🕰️generations/🌍️environment"
out=['#!/usr/bin/env python3','"""P12b edit set (see make-edits.py): anchored replacements on live files and new files copied from P12b/new."""',"EDITS=[]","def rep(path,old,new,count=1):EDITS.append((path,old,new,count))","LIB=%r"%LIB,"G=%r"%G,"RES=%r"%RES,"STAGE=%r"%STAGE,"ENV=%r"%ENV,"MEM=LIB+'/🧠️memory'"]
def rep(path,old,new,count=1):
    out.append("rep(%r,%r,%r,%d)"%(path,old,new,count))
def whole(path,src):
    old=open(L+path,encoding="utf8").read()
    new=open("new/"+src,encoding="utf8").read()
    assert old!=new
    rep(path,old,new)
for dest,src in [(STAGE+"/🟨️.mjs","plan-stage/module.mjs"),(STAGE+"/🟨️.d.mts","plan-stage/module.d.mts"),(STAGE+"/🧬️schema/🔣️.json","plan-stage/schema/schema.json"),(STAGE+"/🧪️tests/🟦️.ts","plan-stage/tests/main.ts"),(RES+"/🧪️tests/\U0001FA93️plan-limit/🟦️.ts","pool-plan-limit.ts"),(RES+"/🧪️tests/\U0001FA9C️plan-refusal/🟦️.ts","plan-refusal.ts"),(G+"/🧪️tests/\U0001FA9C️plan-stages/🟦️.ts","plan-stages.ts")]:
    whole(dest,src)
rep(G+"/🧭️discovery/🟦️.ts",'''read,stat});
    stages.mark("tests");''','''read,stat,operation:name=>stages.operation(name)});
    stages.mark("tests");''')
E=ENV+"/🟨️.mjs"
rep(E,'export async function deriveEnvironmentV1({projects,workspaceManifests,lock,taxonomy,read,stat}){','export async function deriveEnvironmentV1({projects,workspaceManifests,lock,taxonomy,read,stat,operation=()=>undefined}){')
rep(E,'from+=FIELD_CONCURRENCY_V1)await Promise.all(projects.slice(','from+=FIELD_CONCURRENCY_V1)await operation("fields"),await Promise.all(projects.slice(')
rep(E,'for(const path of[...workspaceManifests].sort()){const text=await read(path);','for(const path of[...workspaceManifests].sort()){await operation("manifests");const text=await read(path);')
rep(E,'const lockDigest=lock?await digest(lock):null,taxonomyDigest=taxonomy?await digest(taxonomy):null;','await operation("lock");const lockDigest=lock?await digest(lock):null;await operation("taxonomy");const taxonomyDigest=taxonomy?await digest(taxonomy):null;')
rep(ENV+"/🟨️.d.mts",'lock?:string;taxonomy?:string}):Promise<{digest:string','lock?:string;taxonomy?:string;operation?:(name:"fields"|"manifests"|"lock"|"taxonomy")=>void|Promise<void>}):Promise<{digest:string')
P=RES+"/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs"
rep(P,'if(message?.type==="stage"){entry.stage=String(message.stage);return;}','if(message?.type==="stage"){entry.stage=String(message.stage);entry.operation=String(message.operation??message.stage);return;}')
rep(P,'stage:entry.stage??"?",currentMegabytes','stage:entry.stage??"?",operation:entry.operation??"?",currentMegabytes')
S=RES+"/🏛️service/🌐️host/🟨️.mjs"
rep(S,'JSON.stringify({stage:error.stage,megabytes:error.currentMegabytes','JSON.stringify({stage:error.stage,operation:error.operation,megabytes:error.currentMegabytes')
rep(S,'"host-graph-plan-too-heavy",{stage:error.stage,megabytes:','"host-graph-plan-too-heavy",{stage:error.stage,operation:error.operation,megabytes:')
rep(RES+"/🏛️service/🧯️failure/🟨️.mjs",'return{stage:String(values.stage??"?"),megabytes:','return{stage:String(values.stage??"?"),operation:String(values.operation??"?"),megabytes:')
for f in [RES+"/🧬️schema/🔣️.json",LIB+"/📖️texts/🔣️.json"]:
    rep(f,'the plan stage {stage} grew to {megabytes} MB,','the plan stage {stage} (during {operation}) grew to {megabytes} MB,')
    rep(f,'die Plan-Stufe {stage} ist auf {megabytes} MB gewachsen,','die Plan-Stufe {stage} (während {operation}) ist auf {megabytes} MB gewachsen,')
exec(open("ret-snippet.py",encoding="utf8").read())
out.append("NEWFILES=[(ENV+'/🧪️tests/\\U0001F9ED\\uFE0Foperations/🟦️.ts','environment-operations.ts'),(%r+'/🧪️tests/\\U0001FAB6\\uFE0Fbounded-read/🟦️.ts','retention-bounded.ts')]"%RET)
open("edits.py","w",encoding="utf8").write("\n".join(out)+"\n")
print("written",len(out))
