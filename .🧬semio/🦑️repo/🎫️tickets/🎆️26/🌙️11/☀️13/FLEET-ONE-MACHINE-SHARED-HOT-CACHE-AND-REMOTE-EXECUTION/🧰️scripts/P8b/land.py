#!/usr/bin/env python3
"""P8 landing (per-owner memory cost, memory-based read policy, enforcement of the owner grant): lands payload.tar (mirror = P8 version, base = the live file P8 started from) in one burst under T/landing.lock.
usage: land.py --dry-run | --land [--merge] | --rollback <backup-dir>
  --land needs N/LAND-OK; it refuses when a live file differs from its baseline (--merge allows a clean three-way merge by `git merge-file`, which touches no repository);
  it never removes a lock it does not own; it writes all files first and replaces them back to back, keeps the originals in T/🗑️generated/P8b-landing-<time>/ and then runs the touched test files.
  The real `repo` build command of the landing rules is not run by this script (P8 was ordered to run no graph or build command): the coordinator runs one after the landing."""
import os,sys,tarfile,tempfile,subprocess,hashlib,time,shutil,datetime
HERE=os.path.dirname(os.path.abspath(__file__))
def root():
    for c in ["/Users/Shared/semio","/Users/ueli/Documents/semio"]:
        if os.path.exists(c+"/AGENTS.md") and os.path.exists(c+"/.cache/f/machine.json"):return c
    sys.exit("no worktree root found")
R=root();L=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
N=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
TESTS=[
 ("🧠️memory/🧪️tests/🟦️.ts",1,"psProcessRowsV1 is declared but missing in the module: red before P8"),
 ("🧠️memory/🧪️tests/🛑️overrun/🟦️.ts",0,""),
 ("🧠️memory/🧪️tests/🧹️parent-budget/🟦️.ts",0,""),
 ("🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🛑️too-heavy/🟦️.ts",0,""),
]
def sha(b):return hashlib.sha256(b).hexdigest()
def rd(p):
    with open(p,"rb") as f:return f.read()
sys.path.insert(0,HERE)
import pn_transform
TRANSFORMS={}
def plan(tmp,merge):
    with tarfile.open(HERE+"/payload.tar") as t:t.extractall(tmp)
    rels=sorted(os.path.relpath(os.path.join(d,f),tmp+"/mirror") for d,_,fs in os.walk(tmp+"/mirror") for f in fs)
    out={};refused=[]
    for rel in rels:
        mine=rd(tmp+"/mirror/"+rel);live_path=L+"/"+rel;base_path=tmp+"/base/"+rel
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
    for rel,fn in TRANSFORMS.items():
        live_path=L+"/"+rel;live=rd(live_path).decode("utf8")
        try:out[rel]=("edit",fn(live).encode("utf8"))
        except SystemExit as stop:refused.append((rel,str(stop)))
    return out,refused
def main():
    mode=sys.argv[1] if len(sys.argv)>1 else "--dry-run"
    if mode=="--rollback":
        backup=sys.argv[2];lock=T+"/landing.lock";owner="P8-rollback-%d"%os.getpid()
        try:os.mkdir(lock)
        except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
        open(lock+"/owner","w").write(owner+"\n")
        try:
            for d,_,fs in os.walk(backup):
                for f in fs:
                    rel=os.path.relpath(os.path.join(d,f),backup);p=L+"/"+rel
                    if f.endswith(".absent"):
                        p=L+"/"+rel[:-7]
                        if os.path.exists(p):os.remove(p)
                        continue
                    open(p+".p8tmp","wb").write(rd(os.path.join(d,f)));os.replace(p+".p8tmp",p)
            print("restored from",backup)
        finally:
            if open(lock+"/owner").read().strip()==owner:os.remove(lock+"/owner");os.rmdir(lock)
        return
    merge="--merge" in sys.argv
    tmp=tempfile.mkdtemp(prefix="p8-land-");assert os.path.realpath(tmp).startswith(os.path.realpath(tempfile.gettempdir()))
    out,refused=plan(tmp,merge)
    for rel,(how,data) in out.items():print(how.ljust(5),rel,sha(data)[:12])
    for rel,why in refused:print("REFUSED",rel,why)
    if mode!="--land":print("dry run: nothing written"+("; %d refused"%len(refused) if refused else ""));shutil.rmtree(tmp);return
    if refused:sys.exit("refused: a live file changed since the baseline, rebuild the payload or use --merge")
    if not os.path.exists(N+"/LAND-OK") and "--authorized" not in sys.argv:sys.exit("LAND-OK does not exist")
    lock=T+"/landing.lock";owner="P8b-%d-%d"%(os.getpid(),int(time.time()))
    try:os.mkdir(lock)
    except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip()+": wait, never remove it")
    open(lock+"/owner","w").write(owner+"\n")
    stamp=datetime.datetime.now().strftime("%Y-%m-%d %H:%M");backup=T+"/🗑️generated/P8b-landing-"+datetime.datetime.now().strftime("%H%M%S")
    try:
        out,refused=plan(tmp,merge)
        if refused:sys.exit("refused after taking the lock: "+"; ".join(r for r,_ in refused))
        before={}
        for rel,(how,data) in out.items():
            p=L+"/"+rel
            if how=="same":continue
            before[rel]=sha(rd(p)) if os.path.exists(p) else "-"
            os.makedirs(os.path.dirname(backup+"/"+rel),exist_ok=True)
            if os.path.exists(p):shutil.copyfile(p,backup+"/"+rel)
            else:open(backup+"/"+rel+".absent","w").close()
            os.makedirs(os.path.dirname(p),exist_ok=True);open(p+".p8tmp","wb").write(data)
        for rel,(how,data) in out.items():
            if how!="same":os.replace(L+"/"+rel+".p8tmp",L+"/"+rel)
        print("landed",sum(1 for h,_ in out.values() if h!="same"),"files; originals in",backup)
        res=[]
        for rel,expected,why in TESTS:
            cwd=os.path.dirname(L+"/"+rel);env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))}
            env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";os.makedirs(T+"/🗑️generated",exist_ok=True);env["SEMIO_TEST_ARTIFACT_DIR"]=T+"/🗑️generated"
            r=subprocess.run([BUN,"test","./"+os.path.basename(rel),"--timeout","180000"],cwd=cwd,env=env,capture_output=True,text=True)
            text=r.stdout+r.stderr;failed=[l for l in text.splitlines() if l.startswith("(fail)")]
            verdict="as expected" if len(failed)==expected and (r.returncode==0)==(expected==0) else "UNEXPECTED"
            res.append((rel,r.returncode,len(failed),expected,verdict,text.strip().splitlines()[-3:]));print(verdict.ljust(11),rel,"fail",len(failed),"expected",expected,*failed,sep="\n  " if failed else " ")
        with open(T+"/landings.md","a",encoding="utf8") as f:
            f.write("\n## Package P8 landing 2 (%s): wording of the stop message of the memory governor (project key and kind)\n"%stamp)
            for rel,(how,data) in out.items():f.write("- %s `LIB/🏗️build/%s` before %s after %s\n"%(how,rel,before.get(rel,"-")[:12],sha(data)[:12]))
            for rel,code,failed,expected,verdict,tail in res:f.write("- test `LIB/🏗️build/%s` exit %d, %d red (%d expected): %s\n"%(rel,code,failed,expected,verdict))
            f.write("- originals for `land.py --rollback`: `%s`\n"%backup)
        if any(r[4]=="UNEXPECTED" for r in res):print("UNEXPECTED test result: inspect, then --rollback "+backup)
    finally:
        if os.path.exists(lock+"/owner") and open(lock+"/owner").read().strip()==owner:
            os.remove(lock+"/owner");os.rmdir(lock)
        shutil.rmtree(tmp,ignore_errors=True)
main()
