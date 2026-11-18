#!/usr/bin/env python3
"""R39 landing (PREPARED ONLY, the coordinator lands it): interface identity of check-mode library units in the unit-cache wrapper. 9 anchored edits of the wrapper script and 6 new files (the interface member, its law tests, the Python model, the vectors, the wrapper-level cutoff test).
usage: land.py [--dry-run] | --land [--authorized] [--no-tests] [--lock-held] [--root DIR] | --rollback DIR [--root DIR]"""
import os,sys,subprocess,hashlib,time,shutil,datetime
HERE=os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0,HERE)
import edits
def arg(name):
    return sys.argv[sys.argv.index(name)+1] if name in sys.argv else None
def root():
    if arg("--root"):return os.path.abspath(arg("--root"))
    for c in ["/Users/ueli/Documents/semio"]:
        if os.path.exists(c+"/AGENTS.md") and os.path.exists(c+"/.cache/f/machine.json"):return c
    sys.exit("no worktree root found")
R=root()
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
N=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
UC=edits.UC
GREEN=[UC+"/🔭️interface/🧪️tests/🟦️.ts",UC+"/🧪️tests/🔭️cutoff/🟦️.ts"]
SAME_AS_BEFORE=[UC+"/"+p for p in["🧪️tests/🟦️.ts","🧪️tests/🔁️churn/🟦️.ts","🧪️tests/🕵️leaks/🟦️.ts","🧪️tests/🕰️freshness/🟦️.ts","🧪️tests/🌱️incremental/🟦️.ts","✅️complete/🧪️tests/🟦️.ts","🏁️exit/🧪️tests/🟦️.ts","🛰️fleet/🧪️tests/🟦️.ts","🪪️extern/🧪️tests/🟦️.ts","🔏️key/🧪️tests/🟦️.ts"]]
def sha(b):return hashlib.sha256(b).hexdigest()
def rd(p):
    with open(p,"rb") as f:return f.read()
def short(rel):return rel[len(UC)+1:] if rel.startswith(UC) else rel
def plan():
    out={};refused=[]
    p=R+"/"+edits.SCRIPT
    if not os.path.exists(p):refused.append((edits.SCRIPT,"live file is gone"))
    else:
        raw=rd(p);text=raw.decode("utf8")
        try:
            os.environ["R39_ROOT"]=R
            edited=edits.apply(text)
        except SystemExit as e:
            refused.append((edits.SCRIPT,str(e)));edited=None
        if edited is not None:
            if edited==text:out[edits.SCRIPT]=("same",raw)
            else:out[edits.SCRIPT]=("copy" if sha(raw).startswith(edits.BASELINE_SHA256) else "merge",edited.encode("utf8"))
    for dest,name in edits.NEWFILES:
        data=rd(HERE+"/new/"+name);p=R+"/"+dest
        if os.path.exists(p):
            if rd(p)==data:out[dest]=("same",data)
            else:refused.append((dest,"new file exists with other content"))
        else:out[dest]=("new",data)
    return out,refused
def run_test(rel):
    cwd=os.path.dirname(R+"/"+rel);env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))}
    env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";os.makedirs(T+"/🗑️generated",exist_ok=True);env["SEMIO_TEST_ARTIFACT_DIR"]=T+"/🗑️generated"
    if not os.path.exists(R+"/"+rel):return None
    r=subprocess.run([BUN,"test","./"+os.path.basename(rel),"--timeout","240000"],cwd=cwd,env=env,capture_output=True,text=True)
    text=r.stdout+r.stderr
    return {l.split(" [")[0] for l in text.splitlines() if l.startswith("(fail)")}
def main():
    mode=sys.argv[1] if len(sys.argv)>1 else "--dry-run"
    if mode=="--rollback":
        backup=sys.argv[2];lock=T+"/landing.lock";owner="R39-rollback-%d"%os.getpid()
        try:os.mkdir(lock)
        except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
        open(lock+"/owner","w").write(owner+"\n")
        try:
            for d,_,fs in os.walk(backup):
                for f in fs:
                    rel=os.path.relpath(os.path.join(d,f),backup)
                    if f.endswith(".absent"):
                        p=R+"/"+rel[:-7]
                        if os.path.exists(p):os.remove(p)
                        continue
                    p=R+"/"+rel;open(p+".r39tmp","wb").write(rd(os.path.join(d,f)));os.replace(p+".r39tmp",p)
            print("restored from",backup)
        finally:
            if os.path.exists(lock+"/owner") and open(lock+"/owner").read().strip()==owner:os.remove(lock+"/owner");os.rmdir(lock)
        return
    out,refused=plan()
    for rel,(how,data) in out.items():print(how.ljust(5),short(rel),sha(data)[:12])
    for rel,why in refused:print("REFUSED",short(rel),why)
    landing=sum(1 for h,_ in out.values() if h!="same")
    print("plan: %d files to write, %d unchanged, %d refused"%(landing,len(out)-landing,len(refused)))
    if mode!="--land":print("dry run: nothing written");return
    if refused:sys.exit("refused: a live file changed since the payload, rebuild the payload")
    if landing==0:print("nothing to land");return
    if not os.path.exists(N+"/LAND-OK") and "--authorized" not in sys.argv:sys.exit("LAND-OK does not exist")
    before={}
    if "--no-tests" not in sys.argv:
        for rel in SAME_AS_BEFORE:before[rel]=run_test(rel)
    lock=T+"/landing.lock";owner="R39-%d-%d"%(os.getpid(),int(time.time()));held="--lock-held" in sys.argv
    os.makedirs(T,exist_ok=True)
    if held:
        if not os.path.isdir(lock):sys.exit("--lock-held, but T/landing.lock does not exist")
    else:
        try:os.mkdir(lock)
        except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip()+": wait, never remove it")
        open(lock+"/owner","w").write(owner+"\n")
    stamp=datetime.datetime.now().strftime("%Y-%m-%d %H:%M");backup=T+"/🗑️generated/R39-landing-"+datetime.datetime.now().strftime("%H%M%S")
    try:
        out,refused=plan()
        if refused:sys.exit("refused after taking the lock: "+"; ".join(r for r,_ in refused))
        prior={}
        for rel,(how,data) in out.items():
            p=R+"/"+rel
            if how=="same":continue
            prior[rel]=sha(rd(p)) if os.path.exists(p) else "-"
            os.makedirs(os.path.dirname(backup+"/"+rel),exist_ok=True)
            if os.path.exists(p):shutil.copyfile(p,backup+"/"+rel)
            else:open(backup+"/"+rel+".absent","w").close()
            os.makedirs(os.path.dirname(p),exist_ok=True);open(p+".r39tmp","wb").write(data)
        for rel,(how,data) in out.items():
            if how!="same":os.replace(R+"/"+rel+".r39tmp",R+"/"+rel)
        print("landed",sum(1 for h,_ in out.values() if h!="same"),"files; originals in",backup)
        res=[]
        if "--no-tests" not in sys.argv:
            for rel in GREEN+SAME_AS_BEFORE:
                now=run_test(rel);was=before.get(rel)
                if now is None:verdict="MISSING";failed=set()
                elif rel in GREEN:failed=now;verdict="as expected" if not now else "UNEXPECTED"
                else:failed=now;new=now-(was or set());verdict="as expected" if not new else "UNEXPECTED"
                res.append((rel,len(failed),verdict));print(verdict.ljust(11),short(rel),"red now",len(failed),"red before",len(was) if was is not None else "-",*sorted(failed)[:3],sep=" | ")
        with open(T+"/landings.md","a",encoding="utf8") as f:
            f.write("\n## Package R39 landing (%s): interface identity of check-mode library units in the unit-cache wrapper (numbers in WORKERS-READY result-R39.md)\n"%stamp)
            for rel,(how,data) in out.items():f.write("- %s `%s` before %s after %s\n"%(how,short(rel) if rel.startswith(UC) else rel,prior.get(rel,"-")[:12],sha(data)[:12]))
            for rel,failed,verdict in res:f.write("- test `UC/%s` red %d: %s\n"%(short(rel),failed,verdict))
            f.write("- originals for `land.py --rollback`: `%s`\n"%backup)
        if any(r[2]!="as expected" for r in res):print("UNEXPECTED test result: inspect, then --rollback "+backup)
    finally:
        if not held and os.path.exists(lock+"/owner") and open(lock+"/owner").read().strip()==owner:
            os.remove(lock+"/owner");os.rmdir(lock)
main()
