#!/usr/bin/env python3
"""R48 follow-up landing. Everything is staged in scratch: STAGE/edits2.py (anchored hunks old->new, relative to the build module) and STAGE/new/<relative path> (new files). Under landing.lock: apply, run TESTS from edits2.py, roll back everything on the first red test, else record in landings.md.
usage: land2.py STAGE [--dry-run]
"""
import os,sys,subprocess,hashlib,time,datetime,importlib.util
R="/Users/ueli/Documents/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
PIDS=["31945","31946"]
stage=os.path.abspath(sys.argv[1])
spec=importlib.util.spec_from_file_location("e2",stage+"/edits2.py");e=importlib.util.module_from_spec(spec);spec.loader.exec_module(e)
def rd(p):
    with open(p,"rb") as f:return f.read()
def sha(b):return hashlib.sha256(b).hexdigest()[:12]
def alive():
    out=subprocess.run(["ps","-axo","pid"],capture_output=True,text=True).stdout.split();return [p for p in PIDS if p in out]
newrel=[]
for d,_,fs in os.walk(stage+"/new"):
    for f in fs:newrel.append(os.path.relpath(os.path.join(d,f),stage+"/new"))
newrel.sort()
backup={}
def plan():
    out={};bad=[]
    for rel in sorted({x[0] for x in e.E}):
        t=open(R+"/"+L+rel,encoding="utf8").read()
        for f,o,n,c in e.E:
            if f!=rel:continue
            if t.count(o)!=c:bad.append("%s: anchor x%d (want %d): %r"%(rel,t.count(o),c,o[:60]));break
            t=t.replace(o,n)
        else:out[rel]=t
    for rel in newrel:
        if os.path.exists(R+"/"+L+rel) and rd(R+"/"+L+rel)!=rd(stage+"/new/"+rel):bad.append(rel+": exists with other content")
    return out,bad
def main():
    out,bad=plan()
    for b in bad:print("BAD",b)
    if bad or "--dry-run" in sys.argv:print("dry run" if not bad else "refused");return
    lock=T+"/landing.lock";owner="R48b-%d-%d"%(os.getpid(),int(time.time()))
    try:os.mkdir(lock)
    except FileExistsError:sys.exit("landing.lock held by "+open(lock+"/owner").read().strip())
    open(lock+"/owner","w").write(owner+"\n");before=alive();res=[];touched=[]
    try:
        out,bad=plan()
        if bad:sys.exit("refused under lock: "+str(bad))
        for rel,t in out.items():
            p=R+"/"+L+rel;backup[p]=rd(p);open(p,"w",encoding="utf8").write(t);touched.append(p)
        for rel in newrel:
            p=R+"/"+L+rel
            if not os.path.exists(p):os.makedirs(os.path.dirname(p),exist_ok=True);backup[p]=None;open(p,"wb").write(rd(stage+"/new/"+rel));touched.append(p)
        red=False
        for rel in e.TESTS:
            env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
            r=subprocess.run([BUN,"test","./🟦️.ts","--timeout","180000"],cwd=R+"/"+L+rel,env=env,capture_output=True,text=True)
            failed=[l for l in (r.stdout+r.stderr).splitlines() if l.startswith("(fail)")]
            res.append((rel,r.returncode,failed));print("ok   " if not failed and r.returncode==0 else "RED  ",rel,*failed)
            if failed or r.returncode!=0:red=True
        if red and "--keep-red" not in sys.argv:
            for p,b in backup.items():
                if b is None:os.remove(p)
                else:open(p,"wb").write(b)
            print("ROLLED BACK (red test)");return
        with open(T+"/landings.md","a",encoding="utf8") as f:
            f.write("\n## Package R48 follow-up landing (%s): %s\n"%(datetime.datetime.now().strftime("%Y-%m-%d %H:%M"),e.TITLE))
            for p in touched:f.write("- `LIB/🏗️build/%s` %s\n"%(p[len(R+"/"+L):],sha(rd(p))))
            for rel,code,failed in res:f.write("- test `LIB/🏗️build/%s` exit %d, %d red%s\n"%(rel,code,len(failed),": "+"; ".join(failed) if failed else ""))
            f.write("- pids %s alive before %s after %s\n"%(",".join(PIDS),before,alive()))
        print("landed",len(touched),"files")
    finally:
        if os.path.exists(lock+"/owner") and open(lock+"/owner").read().strip()==owner:os.remove(lock+"/owner");os.rmdir(lock)
main()
