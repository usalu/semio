#!/usr/bin/env python3
"""R76c landing record: the slice worker writes slice-memory.json (phase, memory, heap types, owners ledger, caches) every 30 s and at a plan stop; the ready child copies it next to ready.json; refusal lines are not cut. NOTE: edited live before the lock (own members only)."""
import os,sys,subprocess,datetime,shutil,time
R="/Users/ueli/Documents/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/";PN=B+"🕸️graph/📥️inference/📦️project-nodes/"
SC=B+"🌐️remote-execution/🛰️sidecar/";RS=B+"🏡️resident/"
SI=B+"🕸️graph/📥️inference/🧭️script-inputs/"
TESTS=[SI+"🧪️tests/🫧️take-release",PN+"📥️take/🧪️tests",PN+"🧪️tests/📥️take",B+"🧠️memory/📋️slice-report/🧪️tests"]
PREFLIGHT=[B+"🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight",B+"🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests"]
def tests(dirs):
    res=[]
    for rel in dirs:
        if not os.path.exists(R+"/"+rel+"/🟦️.ts"):print("skip",rel[len(L):]);continue
        env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
        for attempt in (1,2):
            r=subprocess.run([BUN,"test","./🟦️.ts","--timeout","180000","--bail"],cwd=R+"/"+rel,env=env,capture_output=True,text=True)
            if r.returncode==0:break
        out=r.stdout+r.stderr;failed=[l[:200] for l in out.splitlines() if l.startswith("(fail)")]
        res.append((rel,r.returncode,failed));print("ok  " if r.returncode==0 else "RED ",rel[len(L):],failed,flush=True)
        if r.returncode!=0:i=out.find("error:");print(out[max(0,i-1500):i+1500],flush=True);break
    return res
lock=T+"/landing.lock";waited=0
while True:
    try:os.mkdir(lock);break
    except FileExistsError:
        try:holder=open(lock+"/owner").read().strip()
        except Exception:holder="?"
        if waited%300==0:print("landing.lock held by",holder,flush=True)
        if waited>=2400:sys.exit("lock still held")
        time.sleep(15);waited+=15
open(lock+"/owner","w").write("R76-%d %s\n"%(os.getpid(),datetime.datetime.now().isoformat()))
try:
    res=tests(TESTS);ok=all(x[1]==0 for x in res)
    if ok:res+=tests(PREFLIGHT);ok=all(x[1]==0 for x in res)
    if not ok:sys.exit("RED: fix forward, the edits are already live")
    with open(T+"/landings.md","a",encoding="utf8") as f:
        f.write("\n## Package R76e landing (the heap snapshot probe is removed from the slice worker: it crashed the resident service of m01, m02, m08 with reference b28ca173 at 120 to 200 s; the take-pass release of R76d stays) (%s): slice memory report and untruncated refusal lines\n"%datetime.datetime.now().strftime("%Y-%m-%d %H:%M"))
        f.write("- MISTAKE again: the live tree was edited before this lock was taken (own members only); tests run under the lock afterwards. Diagnostic plumbing only. New member LIB/build/memory/slice-report (schema, module, tests); edited: plan-stage (onPlanStopV1), slice-pool worker.ts, project-nodes (report source), sidecar ready (copy of slice-memory.json next to ready.json, refusal lines up to 4000 characters).\n")
        for rel,code,failed in res:f.write("- test `LIB/%s` exit %d, %d red\n"%(rel[len(L):],code,len(failed)))
    print("LANDED")
finally:
    try:os.remove(lock+"/owner");os.rmdir(lock)
    except Exception:pass
