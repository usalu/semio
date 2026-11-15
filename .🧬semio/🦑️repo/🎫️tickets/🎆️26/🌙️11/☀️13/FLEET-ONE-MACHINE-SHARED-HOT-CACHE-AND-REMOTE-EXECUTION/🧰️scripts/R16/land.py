#!/usr/bin/env python3
"""R16 landing (nothing is compiled twice, attribution: named key components per lookup with a churn report, the terminal width, the primary-package flag and the build-directory remap no longer split a unit, evictions are counted locally and at the remote cache server). The edits were made in place by anchored read-modify-write replacements on the live files (each re-read right before its edit); this script is the landing gate: under T/landing.lock it checks that every file still has the content R16 ended with (sha256 in files.json), loads every source file, runs the test files of the touched members and of their dependants, and records the landing.
usage: land.py --dry-run | --seal | --land
  --seal   writes files.json (sha256 of every R16 file as it is now); run once after the last edit.
  --dry-run  reports files that differ from files.json or are missing, and the tests that would run; takes no lock.
  --land   needs N/LAND-OK; takes T/landing.lock, verifies, loads, tests, appends to T/landings.md."""
import os,sys,json,hashlib,subprocess,time,datetime
HERE=os.path.dirname(os.path.abspath(__file__))
R="/Users/Shared/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
N=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES"
MOD=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules"
LIB=MOD+"/📚️library"
U=LIB+"/⚡️caching/🦀️cargo/🗃️unit-cache"
B=LIB+"/🏗️build"
RC=B+"/🌐️remote-cache"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
FILES=[U+p for p in ["/🧬️schema/🔣️.json","/🔏️key/🟦️.ts","/🔏️key/🧪️tests/🐍️oracle/🐍️.py","/🔏️key/🧪️tests/🔣️.json","/🔏️key/🧪️tests/🧬️components/🟦️.ts","/🔏️key/🧪️tests/🧬️components/🔣️.json","/🔏️key/🧪️tests/🧬️components/🐍️.py","/💾️store/🟦️.ts","/📜️script.ts","/🧭️remap/🟦️.ts","/🧭️remap/🧪️tests/🟦️.ts","/🧹️gc/🟦️.ts","/🧪️tests/🔁️churn/🟦️.ts"]]+[RC+p for p in ["/💾️store/🟦️.ts","/🖥️server/🟦️.ts","/🧪️tests/🟦️.ts"]]+[B+"/🏃️execution/🧬️schema/🌐️remote-cache.json"]
TESTS=[U+p for p in ["/🔏️key/🧪️tests/🟦️.ts","/🔏️key/🧪️tests/🧩️parts/🟦️.ts","/🔏️key/🧪️tests/🧬️components/🟦️.ts","/🧭️remap/🧪️tests/🟦️.ts","/🧭️remap/🔍️embedding/🧪️tests/🟦️.ts","/🛰️fleet/🪪️identity/🧪️tests/🟦️.ts","/🛰️fleet/🪪️identity/🧪️tests/🌍️machine-law/🟦️.ts","/🛰️fleet/📤️uploader/🧪️tests/🟦️.ts","/💾️store/🧪️tests/💧️spill/🟦️.ts","/💾️store/🧪️tests/📦️contract/🟦️.ts","/💾️store/🧪️tests/🫥️forget/🟦️.ts","/📖️admission/🧪️tests/🔑️key-stability/🟦️.ts","/🧹️gc/🧪️tests/🟦️.ts","/🧹️gc/🧱️units/🧪️tests/🟦️.ts","/🧹️gc/🌱️incremental/🧪️tests/🟦️.ts","/🧪️tests/🔁️churn/🟦️.ts","/🧪️tests/🟦️.ts","/🛰️fleet/🧪️tests/🟦️.ts"]]+[RC+"/🧪️tests/🟦️.ts",RC+"/🖥️server/🧪️tests/🟦️.ts",B+"/🌐️remote-execution/🧱️actions/🧪️tests/🟦️.ts",B+"/🌐️remote-execution/🚀️offload/🧪️tests/🟦️.ts",B+"/🌐️remote-execution/🚀️offload/🧪️tests/🎯️determinism/🟦️.ts",B+"/🌐️remote-execution/🚀️offload/🧪️tests/🌐️cluster/🟦️.ts"]
def sha(p):
    with open(p,"rb") as f:return hashlib.sha256(f.read()).hexdigest()
def env():
    e={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};e["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";return e
def run_test(path):
    if not os.path.exists(path):return None
    r=subprocess.run([BUN,"test","./"+os.path.basename(path),"--timeout","180000"],cwd=os.path.dirname(path),env=env(),capture_output=True,text=True)
    text=r.stdout+r.stderr;return r.returncode,[l for l in text.splitlines() if l.startswith("(fail)")]
def main():
    mode=sys.argv[1] if len(sys.argv)>1 else "--dry-run"
    if mode=="--seal":
        json.dump({os.path.relpath(p,R):sha(p) for p in FILES},open(HERE+"/files.json","w",encoding="utf8"),ensure_ascii=False,indent=1);print("sealed",len(FILES));return
    sealed=json.load(open(HERE+"/files.json",encoding="utf8"));bad=[]
    for p in FILES:
        rel=os.path.relpath(p,R)
        if not os.path.exists(p):bad.append((rel,"missing"))
        elif sha(p)!=sealed.get(rel):bad.append((rel,"differs from the sealed content (live %s sealed %s)"%(sha(p)[:12],str(sealed.get(rel))[:12])))
        else:print("same ",rel[-100:],sha(p)[:12])
    for rel,why in bad:print("REFUSED",rel[-100:],why)
    if mode!="--land":print("dry run: nothing written;",len(TESTS),"test files would run;",len(bad),"refused");return
    if bad:sys.exit("refused: a file changed since R16 sealed it")
    if not os.path.exists(N+"/LAND-OK"):sys.exit("LAND-OK does not exist")
    lock=T+"/landing.lock";owner="R16-%d-%d"%(os.getpid(),int(time.time()))
    try:os.mkdir(lock)
    except FileExistsError:sys.exit(lock+" is held by "+open(lock+"/owner").read().strip()+": wait, never remove it")
    open(lock+"/owner","w").write(owner+"\n")
    try:
        stamp=datetime.datetime.now().strftime("%Y-%m-%d %H:%M");smoke=[];res=[]
        for p in FILES:
            if "/🧪️tests/" in p or not p.endswith(".ts"):continue
            r=subprocess.run([BUN,"-e","await import(process.argv[1])",p],capture_output=True,text=True,env=env(),cwd=R);smoke.append((p,r.returncode));print("load","ok  " if r.returncode==0 else "FAIL",p[-90:])
        for p in TESTS:
            now=run_test(p)
            if now is None:continue
            code,failed=now;res.append((p,code,failed));print(("ok" if code==0 and not failed else "RED").ljust(4),os.path.relpath(p,MOD)[-100:],*failed)
        with open(T+"/landings.md","a",encoding="utf8") as f:
            f.write("\n## Package R16 landing (%s): attribution of unit lookups (named key components, churn report), terminal width / primary-package flag / build-directory remap no longer split a unit, evictions counted (unit cache and remote cache server)\n"%stamp)
            for p in FILES:f.write("- file `%s` sha256 %s\n"%(os.path.relpath(p,R),sha(p)[:12]))
            for p,code,failed in res:f.write("- test `%s` exit %d, %d red: %s\n"%(os.path.relpath(p,R),code,len(failed),"ok" if code==0 and not failed else "RED"))
            f.write("- edits were made in place (anchored read-modify-write, files re-read before each edit); no backup directory exists, the content before is the previous landing's\n")
        if any(c!=0 for _,c in smoke) or any(c!=0 or fl for _,c,fl in res):print("UNEXPECTED result: inspect")
    finally:
        if os.path.exists(lock+"/owner") and open(lock+"/owner").read().strip()==owner:os.remove(lock+"/owner");os.rmdir(lock)
main()
