#!/usr/bin/env python3
"""Q1 landing (system compiler pin, busy dependency store wait): lands payload.tar in one burst under T/landing.lock.
usage: land.py --dry-run | --land [--wait MINUTES] [--no-merge] | --rollback <backup-dir>
  paths in the payload are relative to the checkout root (base/ = the live files Q1 started from, mirror/ = the Q1 versions).
  --land refuses when a live file differs from its baseline and cannot be three-way merged by `git merge-file` (touches no repository); --no-merge refuses at any difference.
  it waits for the lock (never removes a lock it does not own), writes every file first and replaces them back to back, keeps the originals in T/🗑️generated/Q1-landing-<time>/,
  runs the touched test files and rolls its own files back when one of the NEW Q1 tests is red; --rollback restores only files that still hold the Q1 content."""
import os,sys,tarfile,tempfile,subprocess,hashlib,time,shutil,datetime
HERE=os.path.dirname(os.path.abspath(__file__))
def root():
    for c in ["/Users/Shared/semio","/Users/ueli/Documents/semio"]:
        if os.path.exists(c+"/AGENTS.md") and os.path.exists(c+"/.cache/f/machine.json"):return c
    sys.exit("no worktree root found")
R=root()
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
N=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
PUB=LIB+"⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/"
TESTS=[
 (LIB+"⚡️caching/📖️texts/🧪️tests/🟦️.ts",0,False),
 (LIB+"🏗️build/📖️texts/🧪️tests/🟦️.ts",0,False),
 (LIB+"🏗️build/🛠️cmake/🧪️tests/🍎️system-compiler/🟦️.ts",0,True),
 (LIB+"🏗️build/🛠️cmake/🧪️tests/🟦️.ts",3,False),
 (PUB+"⚙️setup/🧪️tests/⏳️busy-funding/🟦️.ts",0,True),
 (PUB+"⚙️setup/🧪️tests/🟦️.ts",0,False),
 ("🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/📥️sources/🧪️tests/🟦️.ts",1,False),
]
def sha(b):return hashlib.sha256(b).hexdigest()
def rd(p):
    with open(p,"rb") as f:return f.read()
def plan(tmp,merge):
    with tarfile.open(HERE+"/payload.tar") as t:t.extractall(tmp)
    rels=sorted(os.path.relpath(os.path.join(d,f),tmp+"/mirror") for d,_,fs in os.walk(tmp+"/mirror") for f in fs)
    out={};refused=[]
    for rel in rels:
        mine=rd(tmp+"/mirror/"+rel);live_path=R+"/"+rel;base_path=tmp+"/base/"+rel
        live=rd(live_path) if os.path.exists(live_path) else None
        if live==mine:out[rel]=("same",mine);continue
        if not os.path.exists(base_path):
            if live is None:out[rel]=("new",mine)
            else:refused.append((rel,"new file exists with other content"))
            continue
        base=rd(base_path)
        if live is None:refused.append((rel,"live file is gone"));continue
        if live==base:out[rel]=("copy",mine);continue
        if not merge:refused.append((rel,"live differs from the baseline (live %s base %s)"%(sha(live)[:12],sha(base)[:12])));continue
        lp=tmp+"/live.tmp";open(lp,"wb").write(live)
        r=subprocess.run(["git","merge-file","-p",lp,base_path,tmp+"/mirror/"+rel],capture_output=True)
        if r.returncode!=0:refused.append((rel,"three-way merge conflicts (%s)"%r.returncode))
        else:out[rel]=("merge",r.stdout)
    return out,refused
def take(lock,owner,wait):
    deadline=time.time()+wait*60
    while True:
        try:os.mkdir(lock);open(lock+"/owner","w").write(owner+"\n");return
        except FileExistsError:
            who=open(lock+"/owner").read().strip() if os.path.exists(lock+"/owner") else "?"
            if time.time()>=deadline:sys.exit("landing.lock is held by "+who+": wait, never remove it")
            print("landing.lock held by",who,"- waiting",flush=True);time.sleep(5)
def give(lock,owner):
    if os.path.exists(lock+"/owner") and open(lock+"/owner").read().strip()==owner:os.remove(lock+"/owner");os.rmdir(lock)
def restore(backup,landed):
    for d,_,fs in os.walk(backup):
        for f in fs:
            rel=os.path.relpath(os.path.join(d,f),backup);absent=f.endswith(".absent");target=rel[:-7] if absent else rel;p=R+"/"+target
            if target in landed and os.path.exists(p) and sha(rd(p))!=landed[target]:print("SKIPPED (changed since the landing)",target);continue
            if absent:
                if os.path.exists(p):os.remove(p)
                continue
            open(p+".q1tmp","wb").write(rd(os.path.join(d,f)));os.replace(p+".q1tmp",p)
def runtests(first_only=None):
    res=[]
    for rel,expected,new in TESTS:
        cwd=os.path.dirname(R+"/"+rel);env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))}
        env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";os.makedirs(T+"/🗑️generated",exist_ok=True);env["SEMIO_TEST_ARTIFACT_DIR"]=T+"/🗑️generated"
        r=subprocess.run([BUN,"test","./"+os.path.basename(rel),"--timeout","180000"],cwd=cwd,env=env,capture_output=True,text=True)
        text=r.stdout+r.stderr;failed=[l for l in text.splitlines() if l.startswith("(fail)")]
        verdict="as expected" if len(failed)==expected and (r.returncode==0)==(expected==0) else "UNEXPECTED"
        res.append((rel,r.returncode,len(failed),expected,verdict,new));print(verdict.ljust(11),rel.split("📚️library/")[-1],"fail",len(failed),"expected",expected,*failed,sep="\n  " if failed else " ")
    return res
def main():
    mode=sys.argv[1] if len(sys.argv)>1 else "--dry-run"
    wait=float(sys.argv[sys.argv.index("--wait")+1]) if "--wait" in sys.argv else 0
    if mode=="--rollback":
        lock=T+"/landing.lock";owner="Q1-rollback-%d"%os.getpid();take(lock,owner,wait)
        try:restore(sys.argv[2],{});print("restored from",sys.argv[2])
        finally:give(lock,owner)
        return
    merge="--no-merge" not in sys.argv
    tmp=tempfile.mkdtemp(prefix="q1-land-");assert os.path.realpath(tmp).startswith(os.path.realpath(tempfile.gettempdir()))
    out,refused=plan(tmp,merge)
    for rel,(how,data) in out.items():print(how.ljust(5),rel.split("📚️library/")[-1],sha(data)[:12])
    for rel,why in refused:print("REFUSED",rel,why)
    if mode!="--land":print("dry run: nothing written"+("; %d refused"%len(refused) if refused else ""));shutil.rmtree(tmp);return
    if refused:sys.exit("refused: a live file changed since the baseline and does not merge, rebuild the payload")
    if not os.path.exists(N+"/LAND-OK") and "--authorized" not in sys.argv:sys.exit("LAND-OK does not exist")
    lock=T+"/landing.lock";owner="Q1-%d-%d"%(os.getpid(),int(time.time()));take(lock,owner,wait)
    stamp=datetime.datetime.now().strftime("%Y-%m-%d %H:%M");backup=T+"/🗑️generated/Q1-landing-"+datetime.datetime.now().strftime("%H%M%S")
    try:
        out,refused=plan(tmp,merge)
        if refused:sys.exit("refused after taking the lock: "+"; ".join(r for r,_ in refused))
        before={};landed={}
        for rel,(how,data) in out.items():
            p=R+"/"+rel
            if how=="same":continue
            before[rel]=sha(rd(p)) if os.path.exists(p) else "-";landed[rel]=sha(data)
            os.makedirs(os.path.dirname(backup+"/"+rel),exist_ok=True)
            if os.path.exists(p):shutil.copyfile(p,backup+"/"+rel)
            else:open(backup+"/"+rel+".absent","w").close()
            os.makedirs(os.path.dirname(p),exist_ok=True);open(p+".q1tmp","wb").write(data)
        for rel,(how,data) in out.items():
            if how!="same":os.replace(R+"/"+rel+".q1tmp",R+"/"+rel)
        print("landed",len(landed),"files; originals in",backup,flush=True)
        res=runtests()
        rolled=False
        if any(v=="UNEXPECTED" and new for _,_,_,_,v,new in res):
            print("a NEW Q1 test is red: rolling the Q1 files back");restore(backup,landed);rolled=True
        with open(T+"/landings.md","a",encoding="utf8") as f:
            f.write("\n## Package Q1 landing (%s): system compiler pin on macOS, busy dependency store waits%s\n"%(stamp," - ROLLED BACK" if rolled else ""))
            for rel,(how,data) in out.items():f.write("- %s `%s` before %s after %s\n"%(how,rel.replace(LIB,"LIB/"),before.get(rel,"-")[:12],sha(data)[:12]))
            for rel,code,failed,expected,verdict,new in res:f.write("- test `%s` exit %d, %d red (%d expected): %s\n"%(rel.replace(LIB,"LIB/"),code,failed,expected,verdict))
            f.write("- originals for `land.py --rollback`: `%s`\n"%backup)
        if any(r[4]=="UNEXPECTED" for r in res):print("UNEXPECTED test result: inspect; originals in "+backup)
    finally:
        give(lock,owner);shutil.rmtree(tmp,ignore_errors=True)
main()
