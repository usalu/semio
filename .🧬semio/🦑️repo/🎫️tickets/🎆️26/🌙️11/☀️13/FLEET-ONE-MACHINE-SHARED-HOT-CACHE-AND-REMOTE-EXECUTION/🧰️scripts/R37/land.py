#!/usr/bin/env python3
"""R37 landing (PREPARED ONLY, the coordinator lands it): the digest helper selection of a compacted group (the files a scheduling domain declares output-neutral and the composing module leave the helper list) plus its law test. Lands together with the prepared R29 package in ONE window (R29 edits first, then R37), or alone after R29 is in the live tree (then it bumps the inferrer stamp script-inputs/5 to /6). --dry-run (default), --land, --rollback <dir>, --merge, --no-tests, --authorized, --lock-held."""
import os,sys,json,subprocess,hashlib,time,shutil,datetime
HERE=os.path.dirname(os.path.abspath(__file__))
R29=os.path.join(os.path.dirname(HERE),"R29")
sys.path.insert(0,HERE)
import edits as e37
sys.path.pop(0)
sys.path.insert(0,R29)
import importlib.util
spec=importlib.util.spec_from_file_location("edits29",R29+"/edits.py");e29=importlib.util.module_from_spec(spec);spec.loader.exec_module(e29)
LIB=e37.LIB
def arg(name):
    return sys.argv[sys.argv.index(name)+1] if name in sys.argv else None
def root():
    if arg("--root"):return os.path.abspath(arg("--root"))
    for c in ["/Users/Shared/semio","/Users/ueli/Documents/semio"]:
        if os.path.exists(c+"/AGENTS.md") and os.path.exists(c+"/.cache/f/machine.json"):return c
    sys.exit("no worktree root found")
R=root();L=R+"/"+LIB
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
N=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
INF=LIB+"/🏗️build/🕸️graph/📥️inference"
SI=INF+"/🧭️script-inputs"
INP=LIB+"/🏗️build/📥️inputs"
GREEN=[INP+"/🧪️tests/🧱️compaction-helpers/🟦️.ts"]
SAME_AS_BEFORE=[
 SI+"/🔌️provider-closures/🧪️tests/🟦️.ts",SI+"/🔌️provider-closures/🧪️tests/🪡️tool-cover/🟦️.ts",SI+"/🔌️provider-closures/🧪️tests/🧮️blast-radius/🟦️.ts",
 SI+"/🧪️tests/🟦️.ts",SI+"/🔀️router/🧪️tests/🟦️.ts",SI+"/👓️walk-views/🧪️tests/🟦️.ts",
 INF+"/🧪️tests/🧷️closure-facts/🟦️.ts",INF+"/🧪️tests/🔖️output-stamps/🟦️.ts",INF+"/🧪️tests/🔗️late-closures/🟦️.ts",
 INF+"/🧱️project-assembly/🧪️tests/🟦️.ts",LIB+"/⚡️caching/🧪️tests/⚡️cache-contracts/🧾️command-inputs/🟦️.ts",
 LIB+"/🏗️build/📥️source/🔏️closure/🗓️scheduling/🧪️tests/🟦️.ts",INF+"/🔖️stamps/🧪️tests/🟦️.ts",
 INP+"/🧪️tests/🟦️.ts",INP+"/📋️data/🧪️tests/🟦️.ts",INP+"/🌳️owner-view/🧪️tests/🧬️inference/🟦️.ts",
]
STAMP_SCHEMA=INF+"/🔖️stamps/🧬️schema/🔣️.json"
STAMP_FIXTURE=INF+"/🧪️tests/🔖️output-stamps/🔣️.json"
BUMP=[(STAMP_SCHEMA,'"stamp": "semio.graph-inferrer.script-inputs/5"','"stamp": "semio.graph-inferrer.script-inputs/6"',1),(STAMP_FIXTURE,'"semio.graph-inferrer.script-inputs/5": {','"semio.graph-inferrer.script-inputs/6": {',1)]
def short(rel):return rel[len(LIB)+1:] if rel.startswith(LIB) else rel
def sha(b):return hashlib.sha256(b).hexdigest()
def rd(p):
    with open(p,"rb") as f:return f.read()
def satisfied(rows):
    out=True
    for rel,old,new,count in rows:
        p=R+"/"+rel
        if not os.path.exists(p) or rd(p).decode("utf8").count(new)!=count:out=False
    return out
def r29_pending():
    rows=[r for r in e29.EDITS if r[0] not in (STAMP_SCHEMA,STAMP_FIXTURE)];files_new=[d for d,_ in e29.NEWFILES]
    return not (satisfied(rows) and all(os.path.exists(R+"/"+d) for d in files_new))
def sets():
    pending=r29_pending();s=[]
    if pending:s.append(("R29",e29.EDITS,e29.NEWFILES,R29+"/new"))
    r37=list(e37.EDITS)
    if not pending and not satisfied(e37.EDITS):r37+=BUMP
    s.append(("R37",r37,e37.NEWFILES,HERE+"/new"))
    return s
def plan(merge):
    base=json.load(open(R29+"/baseline.json"));out={};refused=[];allsets=sets()
    rels=sorted({p for _,rows,_,_ in allsets for p,_,_,_ in rows})
    for rel in rels:
        p=R+"/"+rel
        if not os.path.exists(p):refused.append((rel,"live file is gone"));continue
        raw=rd(p);text=raw.decode("utf8");edited=text;ok=True
        for name,rows,_,_ in allsets:
            for q,old,new,count in rows:
                if q!=rel:continue
                if edited.count(new)==count:continue
                n=edited.count(old)
                if n!=count:refused.append((rel,"%s anchor occurs %d times, expected %d: %r"%(name,n,count,old[:70])));ok=False;break
                edited=edited.replace(old,new)
            if not ok:break
        if not ok:continue
        if edited==text:out[rel]=("same",raw);continue
        want=base.get(rel)
        if want is not None and sha(raw)!=want and not merge:refused.append((rel,"live differs from the R29 baseline (live %s base %s)"%(sha(raw)[:12],want[:12])));continue
        out[rel]=("copy" if (want is None or sha(raw)==want) else "merge",edited.encode("utf8"))
    for name,_,files,src in allsets:
        for dest,rel in files:
            data=rd(src+"/"+rel);p=R+"/"+dest
            if os.path.exists(p):
                if rd(p)==data:out[dest]=("same",data)
                else:refused.append((dest,"new file exists with other content"))
            else:out[dest]=("new",data)
    return out,refused,[n for n,_,_,_ in allsets]
def run_test(rel):
    cwd=os.path.dirname(R+"/"+rel);env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))}
    env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";os.makedirs(T+"/🗑️generated",exist_ok=True);env["SEMIO_TEST_ARTIFACT_DIR"]=T+"/🗑️generated"
    if not os.path.exists(R+"/"+rel):return None
    r=subprocess.run([BUN,"test","./"+os.path.basename(rel),"--timeout","180000"],cwd=cwd,env=env,capture_output=True,text=True)
    text=r.stdout+r.stderr
    return {l.split(" [")[0] for l in text.splitlines() if l.startswith("(fail)")}
def main():
    mode=sys.argv[1] if len(sys.argv)>1 else "--dry-run"
    if mode=="--rollback":
        backup=sys.argv[2];lock=T+"/landing.lock";owner="R37-rollback-%d"%os.getpid()
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
                    p=R+"/"+rel;open(p+".r37tmp","wb").write(rd(os.path.join(d,f)));os.replace(p+".r37tmp",p)
            print("restored from",backup)
        finally:
            if open(lock+"/owner").read().strip()==owner:os.remove(lock+"/owner");os.rmdir(lock)
        return
    merge="--merge" in sys.argv
    out,refused,names=plan(merge)
    print("sets:",", ".join(names)+(" (R29 already in the live tree: R37 bumps script-inputs/5 to /6)" if names==["R37"] else " (one window, stamp script-inputs/5 by R29)"))
    for rel,(how,data) in out.items():print(how.ljust(5),short(rel),sha(data)[:12])
    for rel,why in refused:print("REFUSED",short(rel),why)
    if mode!="--land":print("dry run: nothing written"+("; %d refused"%len(refused) if refused else ""));return
    if refused:sys.exit("refused: a live file changed since the payload, rebuild the payload or use --merge")
    if not os.path.exists(N+"/LAND-OK") and "--authorized" not in sys.argv:sys.exit("LAND-OK does not exist")
    before={}
    if "--no-tests" not in sys.argv:
        for rel in SAME_AS_BEFORE:before[rel]=run_test(rel)
    lock=T+"/landing.lock";owner="R37-%d-%d"%(os.getpid(),int(time.time()));held="--lock-held" in sys.argv
    os.makedirs(T,exist_ok=True)
    if held:
        if not os.path.isdir(lock):sys.exit("--lock-held, but T/landing.lock does not exist")
    else:
        try:os.mkdir(lock)
        except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip()+": wait, never remove it")
        open(lock+"/owner","w").write(owner+"\n")
    stamp=datetime.datetime.now().strftime("%Y-%m-%d %H:%M");backup=T+"/🗑️generated/R37-landing-"+datetime.datetime.now().strftime("%H%M%S")
    try:
        out,refused,names=plan(merge)
        if refused:sys.exit("refused after taking the lock: "+"; ".join(r for r,_ in refused))
        prior={}
        for rel,(how,data) in out.items():
            p=R+"/"+rel
            if how=="same":continue
            prior[rel]=sha(rd(p)) if os.path.exists(p) else "-"
            os.makedirs(os.path.dirname(backup+"/"+rel),exist_ok=True)
            if os.path.exists(p):shutil.copyfile(p,backup+"/"+rel)
            else:open(backup+"/"+rel+".absent","w").close()
            os.makedirs(os.path.dirname(p),exist_ok=True);open(p+".r37tmp","wb").write(data)
        for rel,(how,data) in out.items():
            if how!="same":os.replace(R+"/"+rel+".r37tmp",R+"/"+rel)
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
            f.write("\n## Package R37 landing (%s): digest helper selection of compacted groups obeys the scheduling table (sets: %s; numbers in WORKERS-READY result-R37.md)\n"%(stamp,", ".join(names)))
            for rel,(how,data) in out.items():f.write("- %s `%s` before %s after %s\n"%(how,short(rel) if rel.startswith(LIB) else rel,prior.get(rel,"-")[:12],sha(data)[:12]))
            for rel,failed,verdict in res:f.write("- test `LIB/%s` red %d: %s\n"%(short(rel),failed,verdict))
            f.write("- originals for `land.py --rollback`: `%s`\n"%backup)
        if any(r[2]!="as expected" for r in res):print("UNEXPECTED test result: inspect, then --rollback "+backup)
    finally:
        if not held and os.path.exists(lock+"/owner") and open(lock+"/owner").read().strip()==owner:
            os.remove(lock+"/owner");os.rmdir(lock)
main()
