#!/usr/bin/env python3
"""R64 landing 2: drive-root access ladder; earlier: Windows relative-open failure names code, origin, root, entry, role, access; root open retries with reduced access after ERROR_ACCESS_DENIED. Stage in scratch, lock, verify base sha, replace, run tests, record."""
import os,sys,subprocess,hashlib,datetime,shutil
R="/Users/ueli/Documents/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
ST="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R64/stage/"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/"
O="🧰️framework/🔨️modules/📁️filesystem/🧾️observation/"
D=R+"/"+O+"📁️descriptor/"
FILES=[(D+"🟦️.ts",ST+"ts"),(D+"🧬️schema/🔣️.json",ST+"schema")]
PREFLIGHT=[B+"🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight",B+"🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests"]
TESTS=[O+"🧪️tests/🩺️windows-failure",O+"🧪️tests/🧭️denied-entry",O+"🧪️tests/📁️descriptor"]
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()[:12]
BASE=sys.argv[1:] if False else None
def tests(dirs):
    res=[]
    for rel in dirs:
        env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";env["SEMIO_TEST_ARTIFACT_DIR"]="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R64/artifacts"
        r=subprocess.run([BUN,"test","./🟦️.ts","--timeout","180000"],cwd=R+"/"+rel,env=env,capture_output=True,text=True)
        out=r.stdout+r.stderr;failed=[l[:160] for l in out.splitlines() if l.startswith("(fail)")]
        res.append((rel,r.returncode,failed));print("ok  " if r.returncode==0 else "RED ",rel,failed,flush=True)
    return res
base=open(ST+"base.sha").read().split()
lock=T+"/landing.lock"
try:os.mkdir(lock)
except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
open(lock+"/owner","w").write("R64-%d %s\n"%(os.getpid(),datetime.datetime.now().isoformat()))
try:
    for (live,_),b in zip(FILES,base):
        if sha(live)!=b:sys.exit("live file changed since staging: "+live)
    before=[sha(l) for l,_ in FILES]
    for live,staged in FILES:
        tmp=live+".r64.tmp";shutil.copyfile(staged,tmp);os.replace(tmp,live)
    res=tests(TESTS)+tests(PREFLIGHT)
    if any(x[1]!=0 for x in res):
        print("RED: restoring is manual; staged originals in",ST+"orig");
        for (live,_),i in zip(FILES,range(2)):
            tmp=live+".r64.tmp";shutil.copyfile(ST+"orig%d"%i,tmp);os.replace(tmp,live)
        sys.exit(1)
    with open(T+"/landings.md","a",encoding="utf8") as f:
        f.write("\n## Package R64 landing (%s): Windows relative-open failure names code, origin, root, entry, role and access; drive root opens read-only after ERROR_ACCESS_DENIED on a write open\n"%datetime.datetime.now().strftime("%Y-%m-%d %H:%M"))
        f.write("- disclosed: new test folder `FS/🧪️tests/🩺️windows-failure` (🟦️.ts, 🔣️.json, 🐍️oracle.py) was created before the lock; it is referenced by nothing else\n")
        for (live,_),b,a in zip(FILES,before,[sha(l) for l,_ in FILES]):f.write("- replaced `%s` %s -> %s\n"%(live[len(R)+1:],b,a))
        for rel,code,failed in res:f.write("- test `%s` exit %d, %d red%s\n"%(rel,code,len(failed),": "+"; ".join(failed) if failed else ""))
finally:
    shutil.rmtree(lock,ignore_errors=True)
