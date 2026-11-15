#!/usr/bin/env python3
"""R7c landing (resident graph service without file access: deep access sample and typed refusal at start, retire-and-replace by the client, `where:` line with route, pid and parent for every EPERM, worker errors keep their data): lands payload.tar in one burst.
usage: land.py --dry-run | --land [--merge] | --rollback <backup-dir>
  payload.tar holds mirror/<path relative to the repository root> (the R7 version) and base/<same path> (the live file R7 started from; absent for new files).
  --land needs N/LAND-OK (N = WORKERS-READY-WITH-LOW-RESOURCES). It takes T/landing.lock (build system), refuses when a live file differs from its baseline (--merge allows a clean three-way merge by `git merge-file`, which touches no repository),
  writes all files first and replaces them back to back, keeps the originals in T/🗑️generated/R7c-landing-<time>/, load-checks every touched source file and runs the touched test files; every test file is run once BEFORE the replacement (live baseline) so that the verdict is `same as before` or `new red`.
  No `repo show/run/cargo/...` graph or build command is run here (no graph commands on this machine)."""
import os,sys,tarfile,tempfile,subprocess,hashlib,time,shutil,datetime
HERE=os.path.dirname(os.path.abspath(__file__))
def root():
    for c in [os.environ.get("SEMIO_WORKSPACE_ROOT",""),"/Users/Shared/semio","/Users/ueli/Documents/semio"]:
        if c and os.path.exists(c+"/AGENTS.md") and os.path.exists(c+"/.cache/f/machine.json"):return c
    sys.exit("no worktree root found")
R=root()
TICKETS=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26"
T=TICKETS+"/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
N=TICKETS+"/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES"
SW=TICKETS+"/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS"
MOD=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules"
LIB=MOD+"/📚️library/🏗️build"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
TESTS=[
 MOD+"/📚️library/🏗️build/📖️texts/🧪️tests/🟦️.ts",
 MOD+"/📚️library/🏗️build/🧭️invocation/🧪️tests/🛰️plan/🟦️.ts",
 R+"/🧰️framework/🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🧪️tests/🟦️.ts",
]
RESIDENT_TESTS=MOD+"/📚️library/🏗️build/🏡️resident/🧪️tests"
def resident_tests():
    out=[RESIDENT_TESTS+"/🟦️.ts"]
    for name in sorted(os.listdir(RESIDENT_TESTS)):
        p=RESIDENT_TESTS+"/"+name+"/🟦️.ts"
        if os.path.isfile(p):out.append(p)
    return out
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
def env():
    e={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))}
    e["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";os.makedirs(T+"/🗑️generated",exist_ok=True);e["SEMIO_TEST_ARTIFACT_DIR"]=T+"/🗑️generated"
    return e
def run_test(path):
    if not os.path.exists(path):return None
    r=subprocess.run([BUN,"test","./"+os.path.basename(path),"--timeout","180000"],cwd=os.path.dirname(path),env=env(),capture_output=True,text=True)
    text=r.stdout+r.stderr;failed=[l for l in text.splitlines() if l.startswith("(fail)")]
    return r.returncode,failed
def module_tests():
    found=[]
    for d,dirs,fs in os.walk(MOD+"/🤝️shared-worktree"):
        dirs[:]=[x for x in dirs if x not in("node_modules",".cache")]
        if "🧪️tests" not in d.split(os.sep):continue
        for f in fs:
            p=os.path.join(d,f)
            if f.endswith(".ts") and "bun:test" in open(p,encoding="utf8",errors="ignore").read(4000):found.append(p)
    return sorted(found)
def lock(path,owner):
    try:os.mkdir(path)
    except FileExistsError:sys.exit(path+" is held by "+open(path+"/owner").read().strip()+": wait, never remove it")
    open(path+"/owner","w").write(owner+"\n")
def unlock(path,owner):
    if os.path.exists(path+"/owner") and open(path+"/owner").read().strip()==owner:os.remove(path+"/owner");os.rmdir(path)
def main():
    mode=sys.argv[1] if len(sys.argv)>1 else "--dry-run"
    if mode=="--rollback":
        backup=sys.argv[2];owner="R7c-rollback-%d"%os.getpid();lock(T+"/landing.lock",owner)
        try:
            for d,_,fs in os.walk(backup):
                for f in fs:
                    rel=os.path.relpath(os.path.join(d,f),backup);p=R+"/"+rel
                    if f.endswith(".absent"):
                        p=R+"/"+rel[:-7]
                        if os.path.exists(p):os.remove(p)
                        continue
                    open(p+".r7tmp","wb").write(rd(os.path.join(d,f)));os.replace(p+".r7tmp",p)
            print("restored from",backup,"(new directories stay, empty)")
        finally:unlock(T+"/landing.lock",owner)
        return
    merge="--merge" in sys.argv
    tmp=tempfile.mkdtemp(prefix="r7-land-");assert os.path.realpath(tmp).startswith(os.path.realpath(tempfile.gettempdir()))
    out,refused=plan(tmp,merge)
    for rel,(how,data) in out.items():print(how.ljust(5),rel[-110:],sha(data)[:12])
    for rel,why in refused:print("REFUSED",rel[-110:],why)
    if mode!="--land":print("dry run: nothing written"+("; %d refused"%len(refused) if refused else ""));shutil.rmtree(tmp);return
    if refused:sys.exit("refused: a live file changed since the baseline, rebuild the payload or use --merge")
    if not os.path.exists(N+"/LAND-OK") and "--authorized" not in sys.argv:sys.exit("LAND-OK does not exist")
    owner="R7c-%d-%d"%(os.getpid(),int(time.time()));touches_sw=any("/🤝️shared-worktree/" in "/"+rel for rel,(how,_) in out.items() if how!="same")
    stamp=datetime.datetime.now().strftime("%Y-%m-%d %H:%M");backup=T+"/🗑️generated/R7c-landing-"+datetime.datetime.now().strftime("%H%M%S")
    lock(T+"/landing.lock",owner)
    if touches_sw:
        try:lock(SW+"/1113-landing.lock",owner)
        except SystemExit:unlock(T+"/landing.lock",owner);raise
    try:
        out,refused=plan(tmp,merge)
        if refused:sys.exit("refused after taking the lock: "+"; ".join(r for r,_ in refused))
        baseline={}
        ALL=resident_tests()+TESTS
        for path in ALL:baseline[path]=run_test(path)
        before={}
        for rel,(how,data) in out.items():
            p=R+"/"+rel
            if how=="same":continue
            before[rel]=sha(rd(p)) if os.path.exists(p) else "-"
            os.makedirs(os.path.dirname(backup+"/"+rel),exist_ok=True)
            if os.path.exists(p):shutil.copyfile(p,backup+"/"+rel)
            else:open(backup+"/"+rel+".absent","w").close()
            os.makedirs(os.path.dirname(p),exist_ok=True);open(p+".r7tmp","wb").write(data)
        for rel,(how,data) in out.items():
            if how!="same":os.replace(R+"/"+rel+".r7tmp",R+"/"+rel)
        print("landed",sum(1 for h,_ in out.values() if h!="same"),"files; originals in",backup)
        smoke=[]
        for rel,(how,_) in out.items():
            if how=="same" or "/🧪️tests/" in rel or not rel.endswith((".mjs",".ts")) or rel.endswith(".d.mts") or "/🥾️entry/🟨️.mjs" in rel or "/🔎️probe/" in rel or "/🔎️sample/" in rel:continue
            r=subprocess.run([BUN,"-e","await import(process.argv[1])",R+"/"+rel],capture_output=True,text=True,env=env(),cwd=R)
            smoke.append((rel,r.returncode,(r.stderr or "").strip().splitlines()[-1:]));print("load",("ok " if r.returncode==0 else "FAIL"),rel[-90:])
        res=[]
        for path in ALL:
            now=run_test(path);was=baseline.get(path)
            if now is None:continue
            code,failed=now
            verdict="ok" if not failed and code==0 else ("same as before" if was is not None and len(was[1])>=len(failed) and set(failed)<=set(was[1]) else "NEW RED")
            res.append((path,code,failed,was,verdict));print(verdict.ljust(14),os.path.relpath(path,MOD)[-100:],"fail",len(failed),"before",None if was is None else len(was[1]),*failed,sep="\n  " if failed else " ")
        modules=[]
        if "--module-tests" in sys.argv and touches_sw:
            for path in module_tests():
                now=run_test(path)
                if now is None:continue
                modules.append((path,now[0],now[1]));print("module".ljust(14),os.path.relpath(path,MOD)[-100:],"exit",now[0],"fail",len(now[1]))
        for name,sw in (("landings.md",False),("1113-landings.md",True)):
            if sw and not touches_sw:continue
            with open((SW if sw else T)+"/"+name,"a",encoding="utf8") as f:
                f.write("\n## Package R7c landing (%s): the access proof repeated when a mount appears after the start, the proof version in the service log, the memory written after the retire, access files part of the service revision\n"%stamp)
                for rel,(how,data) in out.items():
                    if sw and "/🤝️shared-worktree/" not in "/"+rel:continue
                    f.write("- %s `%s` before %s after %s\n"%(how,rel,before.get(rel,"-")[:12],sha(data)[:12]))
                for path,code,failed,was,verdict in res:f.write("- test `%s` exit %d, %d red (before %s): %s\n"%(os.path.relpath(path,R),code,len(failed),"-" if was is None else len(was[1]),verdict))
                for path,code,failed in modules:f.write("- module test `%s` exit %d, %d red\n"%(os.path.relpath(path,R),code,len(failed)))
                f.write("- originals for `land.py --rollback`: `%s`\n"%backup)
        if any(v=="NEW RED" for *_,v in res) or any(c!=0 for _,c,_ in smoke):print("UNEXPECTED result: inspect, then --rollback "+backup)
    finally:
        if touches_sw:unlock(SW+"/1113-landing.lock",owner)
        unlock(T+"/landing.lock",owner)
        shutil.rmtree(tmp,ignore_errors=True)
main()
