#!/usr/bin/env python3
"""R22 landing: no workspace bare specifier on any command or native-repo-script closure; the fleet-status entry loads its heavy readers lazily.
usage: land.py --dry-run | --land      (--land needs N/LAND-OK and takes T/landing.lock)
Edits are literal and idempotent; imports.json lists (file, old, new, count) for the bare-to-relative rewrites."""
import os,sys,json,subprocess,hashlib,datetime,re
R="/Users/Shared/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
N=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
RD=B+"🚢️fleet-status/📥️readers/🟦️.ts"
FX=B+"🚪️door/🧪️tests/🚦️startup/🔣️.json"
def E(f,o,n,c=1,delete=False):return dict(file=f,old=o,new=n,count=c,delete=delete)
EDITS=json.load(open(T+"/🧰️scripts/R22/imports.json"))
for e in EDITS:e.setdefault("delete",False)
EDITS+=[
 E(RD,'import {cacheRunnerPortsV1,machineHostFreeBytesV1,machineUsesV1,reportCacheLedgerV1} from "../../🧹️cache-gc/🏃️runner/🟦️.ts";\n',"",1,True),
 E(RD,'import {refuseBuildV1} from "../../📖️texts/🟦️.ts";\n',"",1,True),
 E(RD,'import {fleetFactOfSnapshotV1,SCHEDULER_POLICY_V1} from "../../🗓️scheduler/🟦️.ts";\n',"",1,True),
 E(RD,'import {createFileSchedulerStoreV1,schedulerDirectoryV1} from "../../🗓️scheduler/💾️store/🟦️.ts";\n',"",1,True),
 E(RD,'import {readRemoteExecutionFleetFactsV1} from "../../🌐️remote-execution/🚢️status/🟦️.ts";\n',"",1,True),
 E(RD,' const ledger=cacheLedgerV1(),ports=',' const {cacheRunnerPortsV1,machineHostFreeBytesV1,machineUsesV1,reportCacheLedgerV1}=await import("../../🧹️cache-gc/🏃️runner/🟦️.ts");\n const ledger=cacheLedgerV1(),ports='),
 E(RD,'throw refuseBuildV1(','throw (await import("../../📖️texts/🟦️.ts")).refuseBuildV1(',2),
 E(RD,' const store=createFileSchedulerStoreV1(',' const [{fleetFactOfSnapshotV1,SCHEDULER_POLICY_V1},{createFileSchedulerStoreV1,schedulerDirectoryV1}]=await Promise.all([import("../../🗓️scheduler/🟦️.ts"),import("../../🗓️scheduler/💾️store/🟦️.ts")]);\n const store=createFileSchedulerStoreV1('),
 E(RD,' const fleet=await readRemoteExecutionFleetFactsV1(',' const {readRemoteExecutionFleetFactsV1}=await import("../../🌐️remote-execution/🚢️status/🟦️.ts");\n const fleet=await readRemoteExecutionFleetFactsV1('),
 E(FX,'        "/🏗️build/🏛️sealed/🧭️mode/"\n      ]\n    }\n  ],','        "/🏗️build/🏛️sealed/🧭️mode/"\n      ]\n    },\n    {\n      "id": "remote-execution",\n      "match": [\n        "/🏗️build/🌐️remote-execution/"\n      ]\n    },\n    {\n      "id": "scheduler",\n      "match": [\n        "/🏗️build/🗓️scheduler/"\n      ]\n    },\n    {\n      "id": "cache-gc",\n      "match": [\n        "/🏗️build/🧹️cache-gc/"\n      ]\n    }\n  ],'),
 E(FX,'        "store-receipt",\n        "tools",\n        "revision"\n      ],\n      "maximumModules": 100,','        "store-receipt",\n        "tools",\n        "revision",\n        "remote-execution",\n        "scheduler",\n        "cache-gc"\n      ],\n      "maximumModules": 72,'),
 E(B+"🌐️remote-execution/🚢️status/🟦️.ts",'import {remoteCacheFetchV1,resolveRemoteCacheV1} from "../../🌐️remote-cache/🛰️fleet/🟦️.ts";'+"\n"+'import {remoteExecutionPortsV1,remoteExecutionSchedulerEndpointV1,remoteExecutionWorkerApiV1} from "../🛰️sidecar/🟦️.ts";'+"\n",'import {remoteExecutionPortsV1,remoteExecutionSchedulerEndpointV1,remoteExecutionWorkerApiV1} from "../🛰️sidecar/🧰️shared/🟦️.ts";'+"\n"),
 E(B+"🌐️remote-execution/🚢️status/🟦️.ts",' const environment={...context.environment,SEMIO_WORKSPACE_ROOT:context.root},resolution=',' const {remoteCacheFetchV1,resolveRemoteCacheV1}=await import("../../🌐️remote-cache/🛰️fleet/🟦️.ts");'+"\n"+' const environment={...context.environment,SEMIO_WORKSPACE_ROOT:context.root},resolution='),
 E(B+"🩺️doctor/🔎️probes/📦️dependencies/🟦️.ts",'import {selectPackageManagerV1} from "../../../../⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/📚️catalog/🟦️.ts";'+"\n","",1,True),
 E(B+"🩺️doctor/🔎️probes/📦️dependencies/🟦️.ts",'const manager=selectPackageManagerV1("javascript"),INPUTS=[...manager.manifestFileNames(),manager.primaryLockName(),...manager.configurationNames("scope")],READY=','const dependencyInputsV1=async():Promise<readonly string[]>=>{const manager=(await import("../../../../⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/📚️catalog/🟦️.ts")).selectPackageManagerV1("javascript");return [...manager.manifestFileNames(),manager.primaryLockName(),...manager.configurationNames("scope")];};\nconst READY='),
 E(B+"🩺️doctor/🔎️probes/📦️dependencies/🟦️.ts",'  const scopes:DependencyScopeFactsV1[]=[]','  const INPUTS=names.length===0?[]:await dependencyInputsV1();\n  const scopes:DependencyScopeFactsV1[]=[]'),
]
Z=B+"📥️source/🛗️zero-touch/🧪️tests/🟦️.ts"
TESTS=[Z,B+"🚪️door/🧪️tests/🚦️startup/🟦️.ts",B+"🚢️fleet-status/🧪️tests/🟦️.ts",B+"🚢️fleet-status/🧪️tests/🛡️guard/🟦️.ts",B+"🌐️remote-execution/🛰️sidecar/🧪️tests/🟦️.ts",B+"🩺️doctor/🧪️tests/🟦️.ts"]
def sha(p):return hashlib.sha256(open(p,"rb").read()).hexdigest()
def rd(p):return open(R+"/"+p,encoding="utf8").read()
def st(e):
    t=rd(e["file"])
    if e["delete"]:return "same" if e["old"] not in t else "apply" if t.count(e["old"])==e["count"] else "REFUSED"
    if e["new"] in t and (e["old"] not in t or e["new"].count(e["old"])>0 and e["old"] not in t.replace(e["new"],"")):return "same"
    return "apply" if t.count(e["old"])==e["count"] else "REFUSED"
def targets():
    bad=[]
    from os.path import normpath,dirname,exists
    for e in EDITS:
        m=re.match(r'^(["\'])(\..*)\1$',e["new"])
        if m and not exists(normpath(R+"/"+dirname(e["file"])+"/"+m.group(2))):bad.append((e["file"],m.group(2)))
    return bad
def main():
    mode=sys.argv[1] if len(sys.argv)>1 else "--dry-run"
    rows=[(e["file"],e["old"][:60],st(e)) for e in EDITS]
    tally={};[tally.__setitem__(r[2],tally.get(r[2],0)+1) for r in rows]
    print(tally)
    for r in rows:
        if r[2]=="REFUSED":print("REFUSED",r)
    miss=targets()
    for m in miss:print("MISSING TARGET",m)
    if tally.get("REFUSED") or miss:sys.exit("refused")
    if mode=="--dry-run":print("dry-run clean");return
    if not os.path.exists(N+"/N/LAND-OK"):sys.exit("N/LAND-OK missing")
    lock=T+"/landing.lock"
    try:os.mkdir(lock)
    except FileExistsError:sys.exit("landing.lock held by "+open(lock+"/owner").read().strip())
    open(lock+"/owner","w").write("R22-%d\n"%os.getpid())
    try:
        for e in EDITS:
            p=R+"/"+e["file"];t=open(p,encoding="utf8").read()
            if st(e)=="apply":open(p+".r22tmp","w",encoding="utf8").write(t.replace(e["old"],e["new"]));os.replace(p+".r22tmp",p)
        env=dict(os.environ,BUN_RUNTIME_TRANSPILER_CACHE_PATH="0")
        for k in list(env):
            if k.startswith("SEMIO_FLEET") or k.startswith("SEMIO_REMOTE_CACHE"):del env[k]
        out=[]
        for tf in TESTS:
            d=os.path.dirname(R+"/"+tf);r=subprocess.run([BUN,"test","./"+os.path.basename(tf),"--timeout","180000"],cwd=d,env=env,capture_output=True,text=True)
            out.append((tf,r.returncode,[l for l in (r.stdout+r.stderr).splitlines() if l.startswith("(fail)")]))
        files=sorted({e["file"] for e in EDITS})
        with open(T+"/landings.md","a",encoding="utf8") as f:
            f.write("\n## R22 %s: no workspace bare specifier on any command closure; fleet-status entry loads heavy readers lazily\n"%datetime.datetime.utcnow().strftime("%Y-%m-%dT%H:%MZ"))
            f.write("- %d files edited (%d import rewrites bare-to-relative, readers lazy imports, startup fixture groups); first `%s` sha256 %s\n"%(len(files),len(EDITS),RD,sha(R+"/"+RD)[:16]))
            for tf,rc,fails in out:f.write("- test `%s` exit %d, %d red: %s\n"%(tf,rc,len(fails),"; ".join(x[:90] for x in fails)))
        print(out)
    finally:
        if open(lock+"/owner").read().startswith("R22-"):os.remove(lock+"/owner");os.rmdir(lock)
main()
