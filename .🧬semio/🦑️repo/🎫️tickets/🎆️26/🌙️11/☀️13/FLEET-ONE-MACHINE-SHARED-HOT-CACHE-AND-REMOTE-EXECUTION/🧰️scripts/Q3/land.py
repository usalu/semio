#!/usr/bin/env python3
"""Q3 landing (a unit is reported done only with every output a dependant names): lands payload.tar (mirror = Q3 version, base = the live file Q3 started from) in one burst under T/landing.lock.
usage: land.py --dry-run | --land [--merge] | --rollback <backup-dir>
  waits up to 8 minutes for the lock (a lock older than 40 minutes may be taken over), refuses when a live file differs from its baseline (--merge allows a clean three-way merge by `git merge-file`, which touches no repository),
  writes all files first and replaces them back to back, keeps the originals in T/🗑️generated/Q3-landing-<time>/ and then runs the touched test files. No graph or build command is run."""
import os,sys,tarfile,tempfile,subprocess,hashlib,time,shutil,datetime
HERE=os.path.dirname(os.path.abspath(__file__))
def root():
    for c in ["/Users/Shared/semio","/Users/ueli/Documents/semio"]:
        if os.path.exists(c+"/AGENTS.md") and os.path.exists(c+"/.cache/f/machine.json"):return c
    sys.exit("no worktree root found")
R=root();L=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
U="⚡️caching/🦀️cargo/🗃️unit-cache/"
TESTS=[
 (U+"✅️complete/🧪️tests/🟦️.ts",0,""),
 (U+"🧪️tests/🟦️.ts",0,""),
 (U+"💾️store/🧪️tests/📦️contract/🟦️.ts",0,""),
 (U+"💾️store/🧪️tests/💧️spill/🟦️.ts",0,""),
 (U+"💾️store/🧪️tests/🫥️forget/🟦️.ts",0,""),
 (U+"🛰️fleet/🧪️tests/🟦️.ts",0,""),
 (U+"🛰️fleet/🪪️identity/🧪️tests/🟦️.ts",0,""),
 (U+"🔏️key/🧪️tests/🟦️.ts",0,""),
 (U+"🧪️tests/🕰️freshness/🟦️.ts",0,""),
 (U+"🧪️tests/🕵️leaks/🟦️.ts",0,""),
 (U+"🧪️tests/🪟️windows/🟦️.ts",0,""),
 (U+"🧹️gc/🪢️supersession/🧪️tests/🟦️.ts",0,""),
 (U+"🧪️tests/🌱️incremental/🟦️.ts",1,"red on the live tree before Q3 as well (Expected 2, Received 0)"),
 (U+"🧪️tests/⏳️slow/🟦️.ts",1,"red on the live tree before Q3 as well (spawn file argument undefined)"),
 ("🏗️build/🌐️remote-execution/🚀️offload/🧪️tests/🟦️.ts",0,""),
]
def sha(b):return hashlib.sha256(b).hexdigest()
def rd(p):
    with open(p,"rb") as f:return f.read()
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
    return out,refused
def take(lock,owner):
    deadline=time.time()+480
    while True:
        try:os.mkdir(lock);break
        except FileExistsError:
            age=time.time()-os.stat(lock).st_mtime
            if age>2400:
                print("lock older than 40 minutes (%d s): taking it over"%age);shutil.rmtree(lock,ignore_errors=True);continue
            if time.time()>deadline:sys.exit("landing.lock is still held by "+(open(lock+"/owner").read().strip() if os.path.exists(lock+"/owner") else "?")+": run again, never remove it")
            time.sleep(15)
    open(lock+"/owner","w").write(owner+"\n")
def main():
    mode=sys.argv[1] if len(sys.argv)>1 else "--dry-run"
    if mode=="--rollback":
        backup=sys.argv[2];lock=T+"/landing.lock";owner="Q3-rollback-%d"%os.getpid();take(lock,owner)
        try:
            for d,_,fs in os.walk(backup):
                for f in fs:
                    rel=os.path.relpath(os.path.join(d,f),backup);p=L+"/"+rel
                    if f.endswith(".absent"):
                        p=L+"/"+rel[:-7]
                        if os.path.exists(p):os.remove(p)
                        continue
                    open(p+".q3tmp","wb").write(rd(os.path.join(d,f)));os.replace(p+".q3tmp",p)
            print("restored from",backup)
        finally:
            if open(lock+"/owner").read().strip()==owner:os.remove(lock+"/owner");os.rmdir(lock)
        return
    merge="--merge" in sys.argv
    tmp=tempfile.mkdtemp(prefix="q3-land-");assert os.path.realpath(tmp).startswith(os.path.realpath(tempfile.gettempdir()))
    out,refused=plan(tmp,merge)
    for rel,(how,data) in out.items():print(how.ljust(5),rel[-100:],sha(data)[:12])
    for rel,why in refused:print("REFUSED",rel[-100:],why)
    if mode!="--land":print("dry run: nothing written"+("; %d refused"%len(refused) if refused else ""));shutil.rmtree(tmp);return
    if refused:sys.exit("refused: a live file changed since the baseline, rebuild the payload or use --merge")
    lock=T+"/landing.lock";owner="Q3-%d-%d"%(os.getpid(),int(time.time()))
    take(lock,owner)
    stamp=datetime.datetime.now().strftime("%Y-%m-%d %H:%M");backup=T+"/🗑️generated/Q3-landing-"+datetime.datetime.now().strftime("%H%M%S")
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
            os.makedirs(os.path.dirname(p),exist_ok=True);open(p+".q3tmp","wb").write(data)
            if os.path.exists(p):shutil.copymode(p,p+".q3tmp")
        for rel,(how,data) in out.items():
            if how!="same":os.replace(L+"/"+rel+".q3tmp",L+"/"+rel)
        print("landed",sum(1 for h,_ in out.values() if h!="same"),"files; originals in",backup)
        res=[]
        for rel,expected,why in TESTS:
            cwd=os.path.dirname(L+"/"+rel);env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))}
            env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";os.makedirs(T+"/🗑️generated",exist_ok=True);env["SEMIO_TEST_ARTIFACT_DIR"]=T+"/🗑️generated"
            r=subprocess.run([BUN,"test","./"+os.path.basename(rel),"--timeout","180000"],cwd=cwd,env=env,capture_output=True,text=True)
            text=r.stdout+r.stderr;failed=[l for l in text.splitlines() if l.startswith("(fail)")]
            verdict="as expected" if len(failed)==expected and (r.returncode==0)==(expected==0) else "UNEXPECTED"
            res.append((rel,r.returncode,len(failed),expected,verdict,text.strip().splitlines()[-3:]));print(verdict.ljust(11),rel[-90:],"fail",len(failed),"expected",expected,*[f[:160] for f in failed],sep="\n  " if failed else " ")
        with open(T+"/landings.md","a",encoding="utf8") as f:
            f.write("\n## Package Q3 landing (%s): a unit is reported done only with every output a dependant names (`extern location for equivalent does not exist`)\n"%stamp)
            for rel,(how,data) in out.items():f.write("- %s `LIB/%s` before %s after %s\n"%(how,rel,before.get(rel,"-")[:12],sha(data)[:12]))
            for rel,code,failed,expected,verdict,tail in res:f.write("- test `LIB/%s` exit %d, %d red (%d expected): %s\n"%(rel,code,failed,expected,verdict))
            f.write("- originals for `land.py --rollback`: `%s`; report: N/result-Q3.md\n"%backup)
        if any(r[4]=="UNEXPECTED" for r in res):print("UNEXPECTED test result: inspect, then --rollback "+backup)
    finally:
        if os.path.exists(lock+"/owner") and open(lock+"/owner").read().strip()==owner:
            os.remove(lock+"/owner");os.rmdir(lock)
        shutil.rmtree(tmp,ignore_errors=True)
main()
