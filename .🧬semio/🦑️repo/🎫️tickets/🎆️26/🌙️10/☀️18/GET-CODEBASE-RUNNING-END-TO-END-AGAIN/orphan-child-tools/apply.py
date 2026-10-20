#!/usr/bin/env python3
"""Anchored apply of the staged orphan-child set. usage: apply.py [--root DIR] [--dry-run]. Every anchor must occur exactly once; new files must be absent or identical; refuses while a dependency sync or `show projects` runs (live root only)."""
import json,os,shutil,subprocess,sys
HERE=os.path.dirname(os.path.abspath(__file__)); ST=HERE+"/staged"
args=sys.argv[1:]; dry="--dry-run" in args
root=args[args.index("--root")+1] if "--root" in args else "/Users/ueli/Documents/semio"
live=root=="/Users/ueli/Documents/semio"
BK=HERE+"/../🗑️generated/orphan-child-backup"
S=root+"/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage"
ST_DIR=root+"/🧰️framework/🔨️modules/🏃️process/💾️state"
CMD=S+"/🧹️cleanup/🎛️command/🟦️.ts"
if live:
    ps=subprocess.run(["ps","-axo","command"],capture_output=True,text=True).stdout
    if any(("dependencies sync" in l or "show projects" in l) and "apply.py" not in l for l in ps.split("\n")): sys.exit("refused: dependency sync or show projects running")
plan=[]
def anchored(path,pairs):
    text=open(path).read()
    if all(new in text for _,new in pairs): print("already applied:",path[-50:]); return
    for old,new in pairs:
        if text.count(old)!=1: sys.exit(f"anchor count {text.count(old)}: {old[:90]!r} in {path[-50:]}")
    plan.append((path,text,[(o,n) for o,n in pairs]))
def newfile(path,source):
    data=open(ST+"/"+source).read()
    if os.path.exists(path):
        if open(path).read()!=data: sys.exit("new file exists with other content: "+path[-60:])
        print("already present:",path[-50:]); return
    plan.append((path,None,data))
def schema_edit():
    p=S+"/🧬️schema/🔣️.json"; raw=open(p).read(); d=json.loads(raw)
    if json.dumps(d,indent=2,ensure_ascii=False)+"\n"!=raw: sys.exit("schema not canonical")
    defs=d["definitions"]
    if "DescendantCorpus" in defs: print("already applied: schema"); return
    defs["ImageProtocol"]["const"]["persistentNamespace"]="persistent"
    ev={"$ref":"#/definitions/Event"}
    defs["DescendantCase"]={"type":"object","additionalProperties":False,"required":["name","journal","directory","expected","recovery"],"properties":{"name":{"type":"string","minLength":1,"maxLength":160},"journal":{"type":"array","minItems":1,"maxItems":16,"items":ev},"directory":{"type":"string","minLength":1,"maxLength":256},"expected":{"enum":["funded","persistent-state","unfunded"]},"recovery":{"oneOf":[{"type":"null"},{"type":"object","additionalProperties":False,"required":["binding","intent","directoryPresent","expected"],"properties":{"binding":{"type":"boolean"},"intent":{"type":"boolean"},"directoryPresent":{"type":"boolean"},"expected":{"enum":["birth","recover-intent","resume","refuse"]}}}]}}}
    defs["DescendantCorpus"]={"type":"object","additionalProperties":False,"required":["version","parentRoot","cases"],"properties":{"version":{"const":1},"parentRoot":{"type":"string","minLength":1,"maxLength":200},"cases":{"type":"array","minItems":1,"maxItems":32,"items":{"$ref":"#/definitions/DescendantCase"}}}}
    plan.append((p,raw,("json",json.dumps(d,indent=2,ensure_ascii=False)+"\n")))
def state_corpus():
    p=ST_DIR+"/🧪️tests/🔣️.json"; raw=open(p).read(); d=json.loads(raw)
    if json.dumps(d,indent=2,ensure_ascii=False)+"\n"!=raw: sys.exit("state corpus not canonical")
    name="half-born-intent-and-backing"
    if any(r["name"]==name for r in d["recovery"]): print("already applied: state corpus"); return
    d["recovery"].append({"name":name,"charge":True,"phase":"reserved","binding":False,"intent":True,"directory":True,"owner":"actual","expected":"resume"})
    plan.append((p,raw,("json",json.dumps(d,indent=2,ensure_ascii=False)+"\n")))
schema_edit(); state_corpus()
newfile(S+"/📜️journal/\U0001F9F7️descendant/🟦️.ts","descendant.ts")
newfile(S+"/📜️journal/\U0001F9F7️descendant/🧪️tests/🟦️.ts","descendant.test.ts")
newfile(S+"/📜️journal/\U0001F9F7️descendant/🧪️tests/🔣️.json","descendant.fixture.json")
newfile(ST_DIR+"/🧪️tests/\U0001F9F7️half-born/🟦️.ts","state-recovery.test.ts")
IMP='import{createJournalReceiptV1}from"./📜️journal/🔖️revision/🟦️.ts";'
MID='const mountedChildSchema=structuredClone(schema),'
FUNC='''/** 🧷️ Admits an unfunded mounted descendant only as the persistent state child of the reserved namespace; the persistent state owner reconciles its content. */
export async function isNativeStoragePersistentDescendantV1(parent:NativeStorageBackingV1,directory:string,leaseDirectory:string,control:ResourceLeaseControl):Promise<boolean>{if(dirname(directory)!==nativeStoragePersistentNamespaceV1(parent.root))return false;const check=()=>{checkResourceLeaseControl(control);requireNativeStorageMappingReadV1({...parent,leaseDirectory,control});};check();const observed=await readCleanupJournal(parent.root,parent.maximumPhysicalBytes,check,()=>control.remainingMilliseconds()??Number.MAX_SAFE_INTEGER);check();return classifyNativeStorageDescendantV1(observed.state,parent.root,directory)==="persistent-state";}
'''
anchored(S+"/🟦️.ts",[(IMP,IMP+'\nimport{classifyNativeStorageDescendantV1,nativeStoragePersistentNamespaceV1}from"./📜️journal/🧷️descendant/🟦️.ts";'),(MID,FUNC+MID)])
anchored(CMD,[
 ('import{readNativeStorageMountedChildV1}from"../../🟦️.ts";','import{readNativeStorageMountedChildV1,isNativeStoragePersistentDescendantV1}from"../../🟦️.ts";'),
 ('observations=new Map<string,Awaited<ReturnType<typeof readNativeStorageMountedChildV1>>>(),','observations=new Map<string,Awaited<ReturnType<typeof readNativeStorageMountedChildV1>>>(),tolerated=new Set<string>(),'),
 ('throw Error("Exact funded persistent child mount required");const observed=await readNativeStorageMountedChildV1(','throw Error("Exact funded persistent child mount required");if(await isNativeStoragePersistentDescendantV1(parent(row.root),dirname(row.root),receiver.options.leaseDirectory,control)){tolerated.add(row.root);return retain(index+1);}const observed=await readNativeStorageMountedChildV1('),
 ('for(const row of before){const after=await readNativeStorageMountedChildV1(','for(const row of before){if(tolerated.has(row.root))continue;const after=await readNativeStorageMountedChildV1('),
])
for item in plan:
    path=item[0]
    print(("would write " if dry else "write ")+path[-70:])
if dry: print("dry-run ok:",len(plan),"targets"); sys.exit(0)
os.makedirs(BK,exist_ok=True)
for path,old,new in plan:
    if old is not None: shutil.copyfile(path,BK+"/"+str(abs(hash(path)))+".orig"); 
    os.makedirs(os.path.dirname(path),exist_ok=True)
    if old is None: open(path,"w").write(new)
    elif isinstance(new,tuple): open(path,"w").write(new[1])
    else:
        t=old
        for o,n in new: t=t.replace(o,n)
        open(path,"w").write(t)
print("applied",len(plan),"targets")
