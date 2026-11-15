#!/usr/bin/env python3
"""R8 landing record (a missing build input closure leaf is a cache miss, never an error).
usage: land.py --dry-run | --land
R8 edited the files below IN PLACE in the live tree before this script existed (a deviation from the copy-and-land rule, disclosed in result-R8.md and in landings.md); there is therefore no payload to replace. This script
  --dry-run  : lists every file with its sha256 and whether it still has the sha256 R8 finished with (EXPECTED), prints the test files it would run; writes nothing.
  --land     : needs N/LAND-OK, takes T/landing.lock, parses (loads) every touched source file, runs the touched test files, appends the record to T/landings.md, releases the lock.
No `repo` graph or build command is run (none on this machine)."""
import os,sys,subprocess,hashlib,time,datetime,glob
def root():
    for c in [os.environ.get("SEMIO_WORKSPACE_ROOT",""),"/Users/Shared/semio","/Users/ueli/Documents/semio"]:
        if c and os.path.exists(c+"/AGENTS.md") and os.path.exists(c+"/.cache/f/machine.json"):return c
    sys.exit("no worktree root found")
R=root()
TICKETS=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26"
T=TICKETS+"/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
N=TICKETS+"/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES"
LIB=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
G=LIB+"/🕸️graph"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
SOURCES=[G+"/"+p for p in ["🩹️closure-repair/🟨️.mjs","📁️workspace/🟦️.ts","🕰️generations/🌐️remote-exchange/🟦️.ts","🕰️generations/🔌️load/🟨️.mjs","🕰️host/🟦️.ts","🤝️prefetch/🟦️.ts"]]+[LIB+"/📥️inputs/🧮️digest/🟨️.mjs",LIB+"/🔧️tools/🏗️bazel/🔗️closure/🟦️.ts"]
OTHERS=[G+"/🩹️closure-repair/🟨️.d.mts",G+"/🕰️generations/🌐️remote-exchange/🧬️schema/🔣️.json"]
TESTS=[G+"/"+p for p in ["🩹️closure-repair/🧪️tests/🟦️.ts","🕰️generations/🌐️remote-exchange/🧪️tests/🟦️.ts","🕰️generations/🌐️remote-exchange/🧪️tests/🧲️closure-leaves/🟦️.ts","🕰️generations/🌐️remote-exchange/🧪️tests/📡️wave-delta/🟦️.ts","🕰️generations/🌐️remote-exchange/🧪️tests/🏁️head-rank/🟦️.ts","🕰️generations/📡️wave-upload/🧪️tests/🟦️.ts","🕰️generations/🔌️load/🧪️tests/🟦️.ts","🧪️tests/🗑️reset-closure/🟦️.ts","🧪️tests/🙋️fleet-read/🟦️.ts"]]+[LIB+"/🔧️tools/🏗️bazel/🧪️tests/📥️closure/🟦️.ts",LIB+"/👁️observation/🧪️tests/📥️reuse/🟦️.ts"]
EXPECTED=os.path.join(os.path.dirname(os.path.abspath(__file__)),"expected.sha")
def sha(p):
    with open(p,"rb") as f:return hashlib.sha256(f.read()).hexdigest()
def env():
    e={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))}
    e["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";return e
def run_test(path):
    if not os.path.exists(path):return None
    r=subprocess.run([BUN,"test","./"+os.path.basename(path),"--timeout","180000"],cwd=os.path.dirname(path),env=env(),capture_output=True,text=True)
    text=r.stdout+r.stderr;return r.returncode,[l for l in text.splitlines() if l.startswith("(fail)")]
def expected():
    return dict(l.split("\t")[::-1] for l in open(EXPECTED,encoding="utf8").read().splitlines()) if os.path.exists(EXPECTED) else {}
def main():
    mode=sys.argv[1] if len(sys.argv)>1 else "--dry-run"
    want=expected();files=SOURCES+OTHERS+TESTS
    for p in files:
        rel=os.path.relpath(p,R);now=sha(p) if os.path.exists(p) else "-"
        print(("ok     " if want.get(rel)==now else "CHANGED" if rel in want else "new    "),now[:12],rel[-100:])
    if mode!="--land":print("dry run: nothing written;",len(TESTS),"test files would run");return
    if not os.path.exists(N+"/LAND-OK"):sys.exit("LAND-OK does not exist")
    owner="R8-%d-%d"%(os.getpid(),int(time.time()));lockp=T+"/landing.lock"
    try:os.mkdir(lockp)
    except FileExistsError:sys.exit(lockp+" is held by "+open(lockp+"/owner").read().strip()+": wait, never remove it")
    open(lockp+"/owner","w").write(owner+"\n")
    try:
        smoke=[]
        for p in SOURCES:
            r=subprocess.run([BUN,"-e","await import(process.argv[1])",p],capture_output=True,text=True,env=env(),cwd=R)
            smoke.append((p,r.returncode));print("load","ok  " if r.returncode==0 else "FAIL",os.path.relpath(p,R)[-90:])
        res=[]
        for p in TESTS:
            now=run_test(p)
            if now is None:continue
            res.append((p,)+now);print("test",os.path.relpath(p,R)[-90:],"exit",now[0],"red",len(now[1]),*now[1])
        stamp=datetime.datetime.now().strftime("%Y-%m-%d %H:%M")
        with open(T+"/landings.md","a",encoding="utf8") as f:
            f.write("\n## Package R8 landing (%s): a missing build input closure leaf is a cache miss (fleet refetch, else the owners that name it are marked stale and derived again, heavy ones asked of the fleet); leaves travel with the generation\n"%stamp)
            f.write("- note: R8 edited these files in place before taking the lock (no payload, no baseline sha); the lock was taken for the test run and the record only.\n")
            for p in SOURCES+OTHERS+TESTS:f.write("- file `%s` after %s\n"%(os.path.relpath(p,R),sha(p)[:12] if os.path.exists(p) else "-"))
            for p,c in smoke:f.write("- load `%s` %s\n"%(os.path.relpath(p,R),"ok" if c==0 else "FAIL"))
            for p,c,failed in res:f.write("- test `%s` exit %d, %d red%s\n"%(os.path.relpath(p,R),c,len(failed),"" if not failed else ": "+"; ".join(x[:90] for x in failed)))
    finally:
        if os.path.exists(lockp+"/owner") and open(lockp+"/owner").read().strip()==owner:os.remove(lockp+"/owner");os.rmdir(lockp)
main()
