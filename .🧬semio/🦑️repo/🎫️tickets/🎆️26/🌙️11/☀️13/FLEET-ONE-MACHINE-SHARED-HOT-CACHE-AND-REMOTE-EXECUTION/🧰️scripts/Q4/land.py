#!/usr/bin/env python3
"""Q4 landing (a moved build-implementation closure is re-admitted by the task instead of failing it; the parent explains a failed run per task): lands the anchored edits of ops.py and the new files of new/ in ONE burst under T/landing.lock.
usage: land.py --dry-run | --land [--authorized] [--wait SECONDS] | --rollback <backup-dir>
  Edits are anchored string replacements applied to the LIVE content read under the lock (other agents edit the same files; an anchor that is missing or not unique aborts before anything is written).
  The originals go to T/🗑️generated/Q4-landing-<time>/. The tests run after the write; when one of the new tests is red or an existing test got a red it did not have before the write, every file whose content is still the one this script wrote is restored.
  The real `repo` build command of the landing rules is NOT run here (graph and build commands are forbidden to this package): the coordinator runs one after the landing."""
import os,sys,subprocess,hashlib,time,shutil,datetime,json
HERE=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,HERE)
import ops
def root():
    for c in ["/Users/Shared/semio","/Users/ueli/Documents/semio"]:
        if os.path.exists(c+"/AGENTS.md") and os.path.exists(c+"/.cache/f/machine.json"):return c
    sys.exit("no worktree root found")
R=root()
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L=ops.LIB;CL=ops.CL;FT=ops.FT;CH=ops.CHURN
NEW_TESTS=[(CL+"/🛂️readmission/🧪️tests/🟦️.ts","dir"),(CL+"/🛂️readmission/🧪️tests/🏃️tasks/🟦️.ts","root"),(FT+"/🧪️tests/🧾️summary/🟦️.ts","dir")]
OLD_TESTS=[(CH+"/🧪️tests/🧾️refusals/🟦️.ts","dir"),(FT+"/🧪️tests/🟦️.ts","dir"),(L+"/🏗️build/📖️texts/🧪️tests/🟦️.ts","dir"),(CL+"/🧪️tests/🧾️mismatch/🟦️.ts","root"),(CL+"/🚪️command/🔌️runtime/🧪️tests/✅️conformance/🟦️.ts","root"),(CL+"/🧪️tests/🧱️static/🟦️.ts","root"),(CL+"/🧪️tests/🔥️warm/🟦️.ts","root")]
def sha(b):return hashlib.sha256(b).hexdigest()
def rd(p):
    with open(p,"rb") as f:return f.read()
def new_files():
    base=HERE+"/new";out={}
    for d,_,fs in os.walk(base):
        for f in fs:
            p=os.path.join(d,f);out[os.path.relpath(p,base)]=rd(p)
    return out
def plan():
    edits={};refused=[]
    for rel in ops.EDITS:
        p=R+"/"+rel
        try:text=rd(p).decode("utf8");edits[rel]=ops.apply(rel,text).encode("utf8")
        except SystemExit as e:refused.append((rel,str(e)))
        except FileNotFoundError:refused.append((rel,"live file is gone"))
    news={}
    for rel,data in new_files().items():
        p=R+"/"+rel
        if os.path.exists(p) and rd(p)!=data:refused.append((rel,"new file exists with other content"))
        else:news[rel]=data
    return edits,news,refused
def run_test(rel,cwdmode,artifact):
    p=R+"/"+rel;cwd=R if cwdmode=="root" else os.path.dirname(p)
    env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))}
    env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";env["SEMIO_TEST_ARTIFACT_DIR"]=artifact
    target="./"+rel if cwdmode=="root" else "./"+os.path.basename(rel)
    r=subprocess.run([BUN,"test",target,"--timeout","180000"],cwd=cwd,env=env,capture_output=True,text=True)
    text=r.stdout+r.stderr
    names=sorted(set(l[len("(fail) "):].rsplit(" [",1)[0] for l in text.splitlines() if l.startswith("(fail)")))
    return r.returncode,names,text
def acquire(wait):
    lock=T+"/landing.lock";owner="Q4-%d-%d"%(os.getpid(),int(time.time()));end=time.time()+wait
    while True:
        try:os.mkdir(lock);break
        except FileExistsError:
            who=open(lock+"/owner").read().strip() if os.path.exists(lock+"/owner") else "?"
            if time.time()>=end:sys.exit("landing.lock is held by "+who+": not taken (never removed by this script)")
            time.sleep(5)
    open(lock+"/owner","w").write(owner+"\n");return lock,owner
def release(lock,owner):
    try:
        if open(lock+"/owner").read().strip()==owner:os.remove(lock+"/owner");os.rmdir(lock)
    except FileNotFoundError:pass
def restore(backup,written):
    force=not written
    for d,_,fs in os.walk(backup):
        for f in fs:
            rel=os.path.relpath(os.path.join(d,f),backup)
            if f.endswith(".absent"):
                rel=rel[:-7];p=R+"/"+rel
                if os.path.exists(p) and (force or sha(rd(p))==written.get(rel)):os.remove(p);print("removed",rel)
                continue
            p=R+"/"+rel
            if os.path.exists(p) and not force and sha(rd(p))!=written.get(rel):print("SKIPPED (changed since the landing)",rel);continue
            open(p+".q4tmp","wb").write(rd(os.path.join(d,f)));os.replace(p+".q4tmp",p);print("restored",rel)
def main():
    mode=sys.argv[1] if len(sys.argv)>1 else "--dry-run"
    if mode=="--rollback":
        lock,owner=acquire(600)
        try:restore(sys.argv[2],{})
        finally:release(lock,owner)
        return
    wait=int(sys.argv[sys.argv.index("--wait")+1]) if "--wait" in sys.argv else 0
    edits,news,refused=plan()
    for rel,data in edits.items():print("edit ",rel.split("/")[-2][:12].ljust(12),sha(data)[:12],rel[-60:])
    for rel,data in news.items():print("new  ",sha(data)[:12],rel[-70:])
    for rel,why in refused:print("REFUSED",rel[-60:],why)
    if mode!="--land":print("dry run: nothing written"+("; %d refused"%len(refused) if refused else ""));return
    if "--authorized" not in sys.argv:sys.exit("--authorized (the coordinator's go) is required")
    artifact=R+"/.nx/q4a";lock,owner=acquire(wait)
    stamp=datetime.datetime.now().strftime("%Y-%m-%d %H:%M");backup=T+"/🗑️generated/Q4-landing-"+datetime.datetime.now().strftime("%H%M%S");written={};before={}
    try:
        edits,news,refused=plan()
        if refused:sys.exit("refused after taking the lock: "+"; ".join(r+" "+w for r,w in refused))
        os.makedirs(artifact,exist_ok=True);assert os.path.realpath(artifact).startswith(os.path.realpath(R+"/.nx/"))
        base={}
        for rel,cw in OLD_TESTS:
            code,names,_=run_test(rel,cw,artifact);base[rel]=(code,names);print("baseline",code,len(names),rel[-60:])
        allfiles={**edits,**news}
        for rel,data in allfiles.items():
            p=R+"/"+rel
            before[rel]=sha(rd(p)) if os.path.exists(p) else "-"
            os.makedirs(os.path.dirname(backup+"/"+rel),exist_ok=True)
            if os.path.exists(p):shutil.copyfile(p,backup+"/"+rel)
            else:open(backup+"/"+rel+".absent","w").close()
            os.makedirs(os.path.dirname(p),exist_ok=True);open(p+".q4tmp","wb").write(data)
        for rel,data in allfiles.items():os.replace(R+"/"+rel+".q4tmp",R+"/"+rel);written[rel]=sha(data)
        print("landed",len(allfiles),"files; originals in",backup)
        res=[];bad=False
        for rel,cw in NEW_TESTS:
            code,names,text=run_test(rel,cw,artifact);ok=code==0 and not names;bad|=not ok;res.append((rel,code,names,[],"as expected" if ok else "UNEXPECTED"));print("new ",code,names,rel[-60:])
            if not ok:print(text[-3000:])
        for rel,cw in OLD_TESTS:
            code,names,text=run_test(rel,cw,artifact);extra=[n for n in names if n not in base[rel][1]];ok=not extra;bad|=not ok;res.append((rel,code,names,base[rel][1],"as before" if ok else "UNEXPECTED"));print("old ",code,names,"baseline",base[rel][1],rel[-60:])
            if not ok:print(text[-3000:])
        with open(T+"/landings.md","a",encoding="utf8") as f:
            f.write("\n## Package Q4 landing (%s): a moved build-implementation closure is re-admitted by the task; the parent explains a failed Nx run per task\n"%stamp)
            for rel in allfiles:f.write("- %s `%s` before %s after %s\n"%("edit" if rel in edits else "new",rel.replace(L,"LIB"),before[rel][:12],written[rel][:12]))
            for rel,code,names,basenames,verdict in res:f.write("- test `%s` exit %d, red %d (before the landing %d): %s\n"%(rel.replace(L,"LIB"),code,len(names),len(basenames),verdict))
            f.write("- originals for `land.py --rollback`: `%s`\n"%backup)
        if bad:
            print("UNEXPECTED test result: restoring every file that still holds what this script wrote");restore(backup,written)
            with open(T+"/landings.md","a",encoding="utf8") as f:f.write("- Q4 landing ROLLED BACK after an unexpected test result (files changed since were left alone)\n")
            sys.exit(1)
    finally:
        shutil.rmtree(artifact,ignore_errors=True) if os.path.realpath(artifact).startswith(os.path.realpath(R+"/.nx/")) else None
        release(lock,owner)
main()
