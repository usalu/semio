#!/usr/bin/env python3
"""R53 landing: fine-grained closure invalidation (and, in wave 2, the closure walk farm).
The files are developed outside the worktree in SCRATCH/orig (pristine copies in SCRATCH/base). This script takes landing.lock, then for every file: live == base -> copy orig over it; live == orig -> nothing; else a three-way `git merge-file` (a pure file operation, no repository state) and a refusal on conflicts. Anchored patches (PATCHES) edit big live files in place. It then runs the targeted law tests (only files named 🟦️.ts), checks that the long running read pids survive and appends the entry to landings.md.
usage: land.py WAVE [--tests-only] [--no-lock]"""
import os,sys,subprocess,hashlib,datetime,shutil,tempfile,time,json
WAVE=sys.argv[1]
R="/Users/ueli/Documents/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
L=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R53"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
spec=json.load(open(S+"/waves.json",encoding="utf8"))[WAVE]
def sha(p):return hashlib.sha256(open(p,"rb").read()).hexdigest()[:12]
def same(a,b):return os.path.exists(a) and os.path.exists(b) and open(a,"rb").read()==open(b,"rb").read()
def pids(ids):
    out=subprocess.run(["ps","-o","pid=","-p",",".join(ids)],capture_output=True,text=True).stdout.split();return sorted(out)
def land_file(rel):
    live,orig,base=L+rel,S+"/orig/"+rel,S+"/base/"+rel
    if same(live,orig):return "unchanged"
    if not os.path.exists(live):
        os.makedirs(os.path.dirname(live),exist_ok=True);shutil.copyfile(orig,live);return "new"
    if os.path.exists(base) and same(live,base):shutil.copyfile(orig,live);return "replaced"
    tmp=tempfile.NamedTemporaryFile(delete=False);tmp.close()
    r=subprocess.run(["git","merge-file","-p",live,base,orig],capture_output=True)
    if r.returncode!=0:sys.exit("CONFLICT (live changed under me) in "+rel)
    open(live,"wb").write(r.stdout);return "merged"
def patch(rel,old,new):
    p=L+rel;t=open(p,encoding="utf8").read()
    if new in t:return "already"
    if t.count(old)!=1:sys.exit("anchor not found exactly once in "+rel)
    open(p,"w",encoding="utf8").write(t.replace(old,new));return "patched"
lock=T+"/landing.lock"
if "--no-lock" not in sys.argv:
    waited=0
    while True:
        try:os.mkdir(lock);break
        except FileExistsError:
            try:age=time.time()-os.path.getmtime(lock)
            except OSError:age=0
            if age>40*60:sys.exit("stale lock held by "+open(lock+"/owner").read().strip())
            if waited>900:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
            time.sleep(15);waited+=15
    open(lock+"/owner","w").write("R53-%d %s\n"%(os.getpid(),datetime.datetime.now().isoformat()))
try:
    before=pids(spec["pids"]);lines=[]
    if "--tests-only" not in sys.argv:
        for rel in spec["files"]:lines.append("- %s `LIB/%s`"%(land_file(rel),rel))
        for rel,old,new in [(r,open(S+"/"+o,encoding="utf8").read(),open(S+"/"+n,encoding="utf8").read()) for r,o,n in spec.get("patches",[])]:lines.append("- %s `LIB/%s`"%(patch(rel,old,new),rel))
    env={k:v for k,v in os.environ.items() if not k.startswith("SEMIO_")};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
    for t in spec["tests"]:
        r=subprocess.run([BUN,"test","./🟦️.ts","--timeout","180000"],cwd=L+t,env=env,capture_output=True,text=True)
        red=[l for l in (r.stdout+r.stderr).splitlines() if l.startswith("(fail)")]
        lines.append("- test `LIB/%s` exit %d, %d red%s"%(t,r.returncode,len(red),(": "+"; ".join(x[:140] for x in red)) if red else ""));print(lines[-1],flush=True)
    after=pids(spec["pids"])
    if "--tests-only" in sys.argv:sys.exit(0)
    stamp=datetime.datetime.now().strftime("%Y-%m-%d %H:%M")
    entry="\n## Package R53 %s (%s): %s\n"%(WAVE,stamp,spec["title"])
    entry+="\n".join(lines)+"\n"
    entry+="\n".join("- after-sha `%s` %s"%(rel[-60:],sha(L+rel)) for rel in spec["files"] if os.path.exists(L+rel))+"\n"
    entry+="- pids %s alive before %s after %s\n"%(",".join(spec["pids"]),before,after)
    open(T+"/landings.md","a",encoding="utf8").write(entry)
finally:
    if "--no-lock" not in sys.argv and os.path.exists(lock+"/owner"):
        os.remove(lock+"/owner");os.rmdir(lock)
