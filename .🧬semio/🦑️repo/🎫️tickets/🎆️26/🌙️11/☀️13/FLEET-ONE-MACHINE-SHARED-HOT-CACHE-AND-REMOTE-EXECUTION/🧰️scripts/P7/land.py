#!/usr/bin/env python3
"""P7 landing (persistent self-starting worker farm, tests of the farm brought to the current protocol): lands payload.tar (mirror/ = P7 version, base/ = the live file it replaces) in one burst under T/landing.lock.
usage: land.py --dry-run | --land [--merge] [--no-rollback] | --rollback <backup-dir> | --tests
  --land refuses when a live file differs from its baseline (--merge allows a clean three-way merge by `git merge-file`, which touches no repository); it never removes a lock it does not own;
  it writes all files first and replaces them back to back, keeps the originals in T/🗑️generated/P7-landing-<time>/, runs the touched test files one at a time with a hard timeout and, when a test of the farm is red that is not expected to be, restores the originals at once (unless --no-rollback).
  --tests only runs the test files on the live tree (after a landing)."""
import os,sys,tarfile,tempfile,subprocess,hashlib,time,shutil,datetime,signal
HERE=os.path.dirname(os.path.abspath(__file__))
def root():
    for c in ["/Users/Shared/semio","/Users/ueli/Documents/semio"]:
        if os.path.exists(c+"/AGENTS.md") and os.path.exists(c+"/.cache/f/machine.json"):return c
    sys.exit("no worktree root found")
R=root();L=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
F="🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests"
N="🕸️graph/📥️inference/📦️project-nodes/🧪️tests"
TESTS=[
 (F+"/🔓️release/🟦️.ts",0,150,True),(F+"/🪣️declined/🟦️.ts",0,150,True),(F+"/🔒️farm-close/🟦️.ts",0,200,True),(F+"/♻️retirement/🟦️.ts",0,150,True),
 (F+"/🪆️authority/🟦️.ts",0,200,True),(F+"/🪆️captured/🟦️.ts",0,300,True),(F+"/🌊️wave-tally/🟦️.ts",0,100,True),(F+"/🏘️hub/🟦️.ts",0,200,True),(F+"/🪦️orphans/🟦️.ts",0,250,True),(F+"/🟦️.ts",0,400,True),
 ("🧠️memory/🧪️tests/🚪️away/🟦️.ts",0,100,False),("🧠️memory/🧪️tests/🧹️parent-budget/🟦️.ts",0,150,False),("🧠️memory/🧪️tests/🎫️reservation/🟦️.ts",0,100,False),("🧠️memory/🧪️tests/🍎️available/🟦️.ts",0,100,False),
 ("🧠️memory/🧪️tests/🟦️.ts",1,100,False),
 (N+"/🚪️away/🟦️.ts",0,100,False),(N+"/🧹️parent-budget/🟦️.ts",0,100,False),(N+"/🚦️governed-dispatch/🟦️.ts",1,100,False),
]
def sha(b):return hashlib.sha256(b).hexdigest()
def rd(p):
    with open(p,"rb") as f:return f.read()
def plan(tmp,merge):
    with tarfile.open(HERE+"/payload.tar") as t:t.extractall(tmp,filter="data") if sys.version_info>=(3,12) else t.extractall(tmp)
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
def run_tests(selection=None):
    res=[]
    for rel,expected,limit,farm in TESTS:
        cwd=os.path.dirname(L+"/"+rel);env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))}
        env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";art=T+"/🗑️generated/P7-artifacts";os.makedirs(art,exist_ok=True);env["SEMIO_TEST_ARTIFACT_DIR"]=art
        p=subprocess.Popen([BUN,"test","./"+os.path.basename(rel),"--timeout",str((limit-10)*1000)],cwd=cwd,env=env,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True,start_new_session=True)
        try:out,_=p.communicate(timeout=limit);code=p.returncode
        except subprocess.TimeoutExpired:
            os.killpg(p.pid,signal.SIGKILL);out,_=p.communicate();code="TIMEOUT"
        failed=[l for l in out.splitlines() if l.startswith("(fail)")]
        verdict="as expected" if code!="TIMEOUT" and len(failed)==expected and (code==0)==(expected==0) else "UNEXPECTED"
        res.append((rel,code,len(failed),expected,verdict,farm));print(verdict.ljust(11),rel.split("build/")[-1][-90:],"exit",code,"fail",len(failed),"expected",expected,*failed,sep="\n  " if failed else " ",flush=True)
    shutil.rmtree(T+"/🗑️generated/P7-artifacts",ignore_errors=True)
    return res
def restore(backup,out):
    for rel,(how,data) in out.items():
        if how=="same":continue
        p=R+"/"+rel;b=backup+"/"+rel
        if os.path.exists(b):shutil.copyfile(b,p+".p7tmp");os.replace(p+".p7tmp",p)
        elif os.path.exists(b+".absent") and os.path.exists(p):os.remove(p)
def main():
    mode=sys.argv[1] if len(sys.argv)>1 else "--dry-run"
    if mode=="--tests":run_tests();return
    lock=T+"/landing.lock"
    if mode=="--rollback":
        backup=sys.argv[2];owner="P7-rollback-%d"%os.getpid()
        try:os.mkdir(lock)
        except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
        open(lock+"/owner","w").write(owner+"\n")
        try:
            for d,_,fs in os.walk(backup):
                for f in fs:
                    rel=os.path.relpath(os.path.join(d,f),backup);p=R+"/"+rel
                    if f.endswith(".absent"):
                        p=R+"/"+rel[:-7]
                        if os.path.exists(p):os.remove(p)
                        continue
                    open(p+".p7tmp","wb").write(rd(os.path.join(d,f)));os.replace(p+".p7tmp",p)
            print("restored from",backup)
        finally:
            if open(lock+"/owner").read().strip()==owner:os.remove(lock+"/owner");os.rmdir(lock)
        return
    merge="--merge" in sys.argv
    tmp=tempfile.mkdtemp(prefix="p7-land-");assert os.path.realpath(tmp).startswith(os.path.realpath(tempfile.gettempdir()))
    out,refused=plan(tmp,merge)
    for rel,(how,data) in out.items():print(how.ljust(5),rel.split("build/")[-1][-100:],sha(data)[:12])
    for rel,why in refused:print("REFUSED",rel.split("build/")[-1][-100:],why)
    if mode!="--land":print("dry run: nothing written"+("; %d refused"%len(refused) if refused else ""));shutil.rmtree(tmp);return
    if refused:sys.exit("refused: a live file changed since the baseline, rebuild the payload or use --merge")
    owner="P7-%d-%d"%(os.getpid(),int(time.time()))
    try:os.mkdir(lock)
    except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip()+": wait, never remove it")
    open(lock+"/owner","w").write(owner+"\n")
    stamp=datetime.datetime.now().strftime("%Y-%m-%d %H:%M");backup=T+"/🗑️generated/P7-landing-"+datetime.datetime.now().strftime("%H%M%S")
    try:
        out,refused=plan(tmp,merge)
        if refused:sys.exit("refused after taking the lock: "+"; ".join(r for r,_ in refused))
        before={}
        for rel,(how,data) in out.items():
            p=R+"/"+rel
            if how=="same":continue
            before[rel]=sha(rd(p)) if os.path.exists(p) else "-"
            os.makedirs(os.path.dirname(backup+"/"+rel),exist_ok=True)
            if os.path.exists(p):shutil.copyfile(p,backup+"/"+rel)
            else:open(backup+"/"+rel+".absent","w").close()
            os.makedirs(os.path.dirname(p),exist_ok=True);open(p+".p7tmp","wb").write(data)
        for rel,(how,data) in out.items():
            if how!="same":os.replace(R+"/"+rel+".p7tmp",R+"/"+rel)
        print("landed",sum(1 for h,_ in out.values() if h!="same"),"files; originals in",backup,flush=True)
        res=run_tests()
        bad=[r for r in res if r[4]=="UNEXPECTED"]
        rolled=False
        if any(r[5] for r in bad) and "--no-rollback" not in sys.argv:
            restore(backup,out);rolled=True;print("ROLLED BACK: an expected-green test of the farm is red; originals restored from",backup)
        with open(T+"/landings.md","a",encoding="utf8") as f:
            f.write("\n## Package P7 landing (%s): persistent self-starting worker farm, farm tests on the current protocol%s\n"%(stamp," (ROLLED BACK)" if rolled else ""))
            for rel,(how,data) in out.items():f.write("- %s `LIB/🏗️build/%s` before %s after %s\n"%(how,rel,before.get(rel,"-")[:12],sha(data)[:12]))
            for rel,code,failed,expected,verdict,farm in res:f.write("- test `LIB/🏗️build/%s` exit %s, %d red (%d expected): %s\n"%(rel,code,failed,expected,verdict))
            f.write("- originals for `land.py --rollback`: `%s`\n"%backup)
        if bad and not rolled:print("UNEXPECTED test result: inspect, then --rollback "+backup)
    finally:
        if os.path.exists(lock+"/owner") and open(lock+"/owner").read().strip()==owner:
            os.remove(lock+"/owner");os.rmdir(lock)
        shutil.rmtree(tmp,ignore_errors=True)
main()
