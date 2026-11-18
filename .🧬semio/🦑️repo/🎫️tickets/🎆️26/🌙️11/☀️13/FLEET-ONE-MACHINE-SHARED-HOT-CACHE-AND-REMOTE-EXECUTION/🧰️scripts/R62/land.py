#!/usr/bin/env python3
"""R62 landing record. The edits to the ready role, its schema, fixtures, oracle, tests and the Windows relative-open message were made in place before the lock (disclosed in result-R62.md); this script takes landing.lock, switches native-bootstrap.sh to the pid-carrying human marker by atomic rename (new inode, never an in-place edit), runs the standing preflight and the member tests, and records the landing. Usage: land.py [--dry-run]"""
import os,sys,subprocess,hashlib,datetime,shutil
R="/Users/ueli/Documents/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
W=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES"
NEW_SH="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/native-bootstrap.sh.new"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/"
RD=B+"🌐️remote-execution/🛰️sidecar/🚦️ready/"
PREFLIGHT=[B+"🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight",B+"🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests"]
CK=B+"🕸️graph/🧮️derived-nodes/🧩️closure-kinds/"
TESTS=[RD+"🧪️tests",B+"🌐️remote-execution/🛰️sidecar/🧪️tests/🔁️restart",CK+"🏭️walk-farm/🧪️tests",CK+"🧪️tests/⏱️throughput"]
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()[:12]
def tests(dirs):
    res=[]
    for rel in dirs:
        env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
        r=subprocess.run([BUN,"test","./🟦️.ts","--timeout","180000"],cwd=R+"/"+rel,env=env,capture_output=True,text=True)
        out=r.stdout+r.stderr;failed=[l[:160] for l in out.splitlines() if l.startswith("(fail)")]
        res.append((rel,r.returncode,failed));print("ok  " if r.returncode==0 else "RED ",rel[len(L):],failed,flush=True)
    return res
if "--dry-run" in sys.argv:print("dry ok",os.path.exists(NEW_SH));sys.exit(0)
lock=T+"/landing.lock"
try:os.mkdir(lock)
except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
open(lock+"/owner","w").write("R62-%d %s\n"%(os.getpid(),datetime.datetime.now().isoformat()))
try:
    res=tests(TESTS)+tests(PREFLIGHT)
    if any(x[1]!=0 for x in res):print("RED: nothing switched");sys.exit(1)
    target=W+"/native-bootstrap.sh";before=sha(target);staged=target+".r62.tmp"
    shutil.copyfile(NEW_SH,staged);shutil.copymode(target,staged);os.replace(staged,target)
    with open(T+"/landings.md","a",encoding="utf8") as f:
        f.write("\n## Package R62 landing (%s): human marker carries pid and start time and expires on evidence; retry exit 75 keeps the first back-off; workspace stage cap 4 h; Windows relative-open message names the status; the closure walk farm obeys the memory plan and the cut of the load\n"%datetime.datetime.now().strftime("%Y-%m-%d %H:%M"))
        f.write("- edited in place before the lock (disclosed): `LIB/%s🟦️.ts`, `LIB/%s🧬️schema/🔣️.json`, `LIB/%s🧪️tests/🟦️.ts` plus `🧫️fixtures/🔣️.json` and `🐍️oracle.py`, and `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts` (message only), `LIB/%s🏭️walk-farm/🟨️.mjs` and `LIB/%s🟨️.mjs` (farm cap), `LIB/%s🏭️walk-farm/🧪️tests/🟦️.ts`\n"%(RD[len(L):],RD[len(L):],RD[len(L):],CK[len(L):],CK[len(L):],CK[len(L):]))
        f.write("- `native-bootstrap.sh` replaced by atomic rename (new inode; running sessions keep the old file) before %s after %s\n"%(before,sha(target)))
        for rel,code,failed in res:f.write("- test `LIB/%s` exit %d, %d red%s\n"%(rel[len(L):],code,len(failed),": "+"; ".join(failed) if failed else ""))
finally:
    os.remove(lock+"/owner");os.rmdir(lock)
