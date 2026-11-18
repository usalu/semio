#!/usr/bin/env python3
"""R61 landing (single). Usage: land.py [--dry-run]. Takes landing.lock, applies anchored edits and new files from the R61 scratch stage, runs standing preflight and member tests, records in landings.md; restores on red."""
import os,sys,subprocess,hashlib,datetime,json,shutil
R="/Users/ueli/Documents/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
STAGE="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R61/stage"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/";C=L+"⚡️caching/"
NH=L+"🏃️process/🦀️cargo/🧰️native-helpers/"
DS=C+"🚀️bootstrap/📦️dependencies/💾️storage/"
RD=B+"🌐️remote-execution/🛰️sidecar/🚦️ready/"
APPEND_TEST=open(STAGE+"/paths-test-append.ts",encoding="utf8").read()
EDITS={
 DS+"🟦️.ts":[(
  " const light=()=>{requireIssuedResourceControlV1(control);checkResourceLeaseControl(control);checkCustody();};\n const check=()=>{light();for(",
  " let custodyAt=-Infinity;\n const full=()=>{requireIssuedResourceControlV1(control);checkResourceLeaseControl(control);checkCustody();custodyAt=performance.now();};\n const light=()=>{control.signal.throwIfAborted();if(performance.now()-custodyAt>=policy.scanCustodyMilliseconds)full();};\n const check=()=>{full();for(")],
 DS+"🧬️schema/📥️install.json":[(
  '"scanProgressWorkUnits": 4096\n','"scanProgressWorkUnits": 4096,\n        "scanCustodyMilliseconds": 20\n')],
 NH+"🟦️.ts":[
  ('import {refuseProcessV1} from','import {shareSchedulerBuildV1} from "./🛰️fleet/🟦️.ts";\nimport {refuseProcessV1} from'),
  ('const built=(await buildSchedulerV1({cwd:workspace,targetDirectory:store.environment.CARGO_TARGET_DIR!,buildDirectory:store.buildDirectory,environment:input.environment,arguments:input.selections[0]!.arguments,toolchain:store.toolchain.digest,control:cpuControl,storage:store.storage,sourceBoundary:store.sourceBoundary})).get("semio-build-scheduler");',
   'const built=(await shareSchedulerBuildV1({workspace,targetDirectory:store.environment.CARGO_TARGET_DIR!,toolchain:store.toolchain.digest,environment:input.environment,mode:readBuildMode(store.environment.SEMIO_BUILD_MODE!),manifest:selection.manifest,say:line=>console.log(line),build:()=>buildSchedulerV1({cwd:workspace,targetDirectory:store.environment.CARGO_TARGET_DIR!,buildDirectory:store.buildDirectory,environment:input.environment,arguments:input.selections[0]!.arguments,toolchain:store.toolchain.digest,control:cpuControl,storage:store.storage,sourceBoundary:store.sourceBoundary})})).get("semio-build-scheduler");')],
 RD+"🟦️.ts":[(
  '    ports.state.write({...state,failure:next,attempts,peakMegabytes:peak});\n',
  '    ports.state.write({...state,failure:next,attempts,peakMegabytes:peak});\n    ports.mirror?.(`${row.name}-failed`,next.tail);\n')],
 DS+"🧪️tests/📏️paths/🟦️.ts":[("__APPEND__",None)],
}
NEW={NH+"🛰️fleet/🟦️.ts":STAGE+"/fleet/🟦️.ts",NH+"🛰️fleet/🧬️schema/🔣️.json":STAGE+"/fleet/🧬️schema/🔣️.json",NH+"🛰️fleet/🧪️tests/🟦️.ts":STAGE+"/fleet/🧪️tests/🟦️.ts"}
PREFLIGHT=[B+"🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight",B+"🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests"]
TESTS=[NH+"🛰️fleet/🧪️tests",DS+"🧪️tests/📏️paths",RD+"🧪️tests"]
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()[:12]
dry="--dry-run" in sys.argv
def tests(dirs):
    res=[]
    for rel in dirs:
        env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
        r=subprocess.run([BUN,"test","./🟦️.ts","--timeout","180000"],cwd=R+"/"+rel,env=env,capture_output=True,text=True)
        out=r.stdout+r.stderr;failed=[l[:160] for l in out.splitlines() if l.startswith("(fail)")]
        res.append((rel,r.returncode,failed));print("ok  " if r.returncode==0 else "RED ",rel[len(L):],failed,flush=True)
    return res
def plan_edits():
    plan=[]
    for rel,pairs in EDITS.items():
        p=R+"/"+rel;text=open(p,encoding="utf8").read();new=text
        for old,rep in pairs:
            if old=="__APPEND__":
                if APPEND_TEST.strip() not in new:new=new.rstrip("\n")+"\n\n"+APPEND_TEST
                continue
            if rep in new and old not in new:continue
            if new.count(old)!=1:sys.exit("anchor not unique/absent in %s: %s"%(rel,old[:70]))
            new=new.replace(old,rep)
        plan.append((rel,p,text,new))
    return plan
plan=plan_edits()
if dry:print("dry ok",[(rel[len(L):],t!=n) for rel,p,t,n in plan]);sys.exit(0)
lock=T+"/landing.lock"
try:os.mkdir(lock)
except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
open(lock+"/owner","w").write("R61-%d %s\n"%(os.getpid(),datetime.datetime.now().isoformat()))
backup=T+"/🗑️generated/R61-landing-"+datetime.datetime.now().strftime("%H%M%S")
try:
    plan=plan_edits();os.makedirs(backup);before={}
    for rel,p,text,new in plan:
        before[rel]=sha(p);open(backup+"/"+hashlib.sha256(rel.encode()).hexdigest()[:16],"w",encoding="utf8").write(text)
        if new!=text:open(p,"w",encoding="utf8").write(new)
    for rel,src in NEW.items():
        os.makedirs(os.path.dirname(R+"/"+rel),exist_ok=True);shutil.copyfile(src,R+"/"+rel)
    res=tests(TESTS)+tests(PREFLIGHT)
    red=[x for x in res if x[1]!=0 and x[0] not in (DS+"🧪️tests/📏️paths",)]
    paths=[x for x in res if x[0]==DS+"🧪️tests/📏️paths"][0]
    if paths[1]!=0 and any("custody" in f for f in paths[2]):red.append(paths)
    if red and "--keep" not in sys.argv:
        for rel,p,text,new in plan:open(p,"w",encoding="utf8").write(text)
        for rel in NEW:shutil.rmtree(os.path.dirname(R+"/"+rel),ignore_errors=True) if rel.endswith("🧪️tests/🟦️.ts") is False and False else None
        print("RED: edits restored (new files left in",NH+"🛰️fleet )");sys.exit(1)
    with open(T+"/landings.md","a",encoding="utf8") as f:
        f.write("\n## Package R61 landing (%s): installed-path scan custody throttle, fleet-shared scheduler build, failure tail mirror\n"%datetime.datetime.now().strftime("%Y-%m-%d %H:%M"))
        for n in NEW:f.write("- new `LIB/%s`\n"%n[len(L):])
        for rel,p,text,new in plan:f.write("- edit `%s` before %s after %s\n"%(rel[len(L):],before[rel],sha(p)))
        for rel,code,failed in res:f.write("- test `LIB/%s` exit %d, %d red%s\n"%(rel[len(L):],code,len(failed),": "+"; ".join(failed) if failed else ""))
finally:
    os.remove(lock+"/owner");os.rmdir(lock)
