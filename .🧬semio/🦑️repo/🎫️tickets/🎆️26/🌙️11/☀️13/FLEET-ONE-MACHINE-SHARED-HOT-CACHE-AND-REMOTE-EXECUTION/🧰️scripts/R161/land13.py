import sys,os,json,hashlib
ROOT=os.environ.get("R161_ROOT","/Users/ueli/Documents/semio")
FW=ROOT+"/🧰️framework"
FILES={"resolve":FW+"/🔨️modules/🏃️process/🧰️tools/🔎️resolve/🟦️.ts","tools":FW+"/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🔭️watch/🏗️provision/🔌️tools/🟨️.mjs"}
def read(p): return open(p,encoding="utf8").read()
def write(p,t): open(p,"w",encoding="utf8").write(t)
def sha(t): return hashlib.sha256(t.encode()).hexdigest()
def one(text,old,new):
    assert text.count(old)==1,("anchor count",text.count(old),old[:90])
    return text.replace(old,new)
def resolve(t):
    t=one(t,'import { nativeCompilerArgumentsV1, selectNativeCompilerV1 } from "../🛠️native-compiler/🟦️.ts";','import { nativeCompilerArgumentsV1, selectNativeCompilerV1 } from "../🛠️native-compiler/🟦️.ts";\nimport { machinePathV1 } from "../../🏗️build/🏠️machine-state/🟦️.ts";')
    return one(t,'const partition = host.root + "/" + (registry.partitions[locator.partition] ?? locator.partition);','const partition = machinePathV1(host.root, registry.partitions[locator.partition] ?? locator.partition);')
def tools(t):
    t=one(t,'import {ensureWatcherV1,findInstalledWatcherV1} from "../🟨️.mjs";','import {ensureWatcherV1,findInstalledWatcherV1} from "../🟨️.mjs";\nimport {machinePathV1} from "../../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";')
    return one(t,'outputRoot:join(root,TOOL_REGISTRY_V1.partitions.tools,"repo-watch")','outputRoot:machinePathV1(root,TOOL_REGISTRY_V1.partitions.tools,"repo-watch")')
PATCH={"resolve":resolve,"tools":tools}
cmd,backup=sys.argv[1],sys.argv[2]
if cmd=="apply":
    os.makedirs(backup,exist_ok=True);manifest={};new={}
    for k,p in FILES.items(): new[k]=PATCH[k](read(p))
    for i,(k,p) in enumerate(FILES.items()):
        old=read(p);write(backup+"/%d.bak"%i,old);manifest[k]={"i":i,"before":sha(old),"after":sha(new[k])}
    json.dump(manifest,open(backup+"/manifest.json","w"))
    for k,p in FILES.items(): write(p,new[k])
    print("applied")
elif cmd=="restore":
    manifest=json.load(open(backup+"/manifest.json"));bad=[]
    for k,p in FILES.items():
        m=manifest[k];now=sha(read(p))
        if now==m["after"]: write(p,read(backup+"/%d.bak"%m["i"]))
        elif now!=m["before"]: bad.append(k)
    print("CONFLICT" if bad else "restored",bad)
    sys.exit(4 if bad else 0)
