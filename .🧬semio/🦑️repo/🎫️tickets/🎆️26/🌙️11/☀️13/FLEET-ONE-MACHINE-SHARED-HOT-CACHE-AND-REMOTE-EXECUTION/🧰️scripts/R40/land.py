#!/usr/bin/env python3
"""R40 landing (PREPARED ONLY, the coordinator lands it): ONE window for R29 + R37 + R39 + R40. R40 adds the interface closure for check-class Rust targets (a dependency crate of a `check` task is keyed by what a type check can observe of its sources, not by their bytes). A set already in the live tree is skipped, so the script lands the remaining ones; a second run is a no-op. The inferrer stamp script-inputs/N moves once per window (R29 owns the 4 to 5 step; without R29 in the window the live N becomes N+1).
usage: land.py [--dry-run] | --land [--authorized] [--merge] [--no-tests] [--lock-held] [--root DIR] | --rollback DIR [--root DIR] | --overlay DIR [--root DIR]"""
import os,sys,re,json,subprocess,hashlib,time,shutil,datetime,importlib.util
sys.dont_write_bytecode=True
HERE=os.path.dirname(os.path.abspath(__file__))
SCRIPTS=os.path.dirname(HERE)
def load(name,path):
    spec=importlib.util.spec_from_file_location(name,path);mod=importlib.util.module_from_spec(spec)
    sys.modules[name]=mod
    spec.loader.exec_module(mod);return mod
def arg(name):
    return sys.argv[sys.argv.index(name)+1] if name in sys.argv else None
def root():
    if arg("--root"):return os.path.abspath(arg("--root"))
    for c in ["/Users/Shared/semio","/Users/ueli/Documents/semio"]:
        if os.path.exists(c+"/AGENTS.md") and os.path.exists(c+"/.cache/f/machine.json"):return c
    sys.exit("no worktree root found")
R=root()
os.environ["R39_ROOT"]=R
e29=load("edits29",SCRIPTS+"/R29/edits.py");e37=load("edits37",SCRIPTS+"/R37/edits.py");e39=load("edits39",SCRIPTS+"/R39/edits.py");e40=load("edits40",HERE+"/edits.py")
LIB=e37.LIB;L=R+"/"+LIB
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
N=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
INF=LIB+"/🏗️build/🕸️graph/📥️inference";SI=INF+"/🧭️script-inputs";INP=LIB+"/🏗️build/📥️inputs";UC=e39.UC
STAMP_SCHEMA=INF+"/🔖️stamps/🧬️schema/🔣️.json";STAMP_FIXTURE=INF+"/🧪️tests/🔖️output-stamps/🔣️.json"
STAMP_ROWS=(STAMP_SCHEMA,STAMP_FIXTURE)
SETS=[("R29",e29.EDITS,e29.NEWFILES,SCRIPTS+"/R29/new"),("R37",e37.EDITS,e37.NEWFILES,SCRIPTS+"/R37/new"),("R39",e39.EDITS,e39.NEWFILES,SCRIPTS+"/R39/new"),("R40",e40.EDITS,e40.NEWFILES,HERE+"/new")]
GREEN=[INP+"/🧪️tests/🧱️compaction-helpers/🟦️.ts",UC+"/🔭️interface/🧪️tests/🟦️.ts",UC+"/🧪️tests/🔭️cutoff/🟦️.ts"]+list(e40.GREEN)
SAME_AS_BEFORE=[
 SI+"/🔌️provider-closures/🧪️tests/🟦️.ts",SI+"/🔌️provider-closures/🧪️tests/🪡️tool-cover/🟦️.ts",SI+"/🔌️provider-closures/🧪️tests/🧮️blast-radius/🟦️.ts",
 SI+"/🧪️tests/🟦️.ts",SI+"/🔀️router/🧪️tests/🟦️.ts",SI+"/👓️walk-views/🧪️tests/🟦️.ts",
 INF+"/🧪️tests/🧷️closure-facts/🟦️.ts",INF+"/🧪️tests/🔖️output-stamps/🟦️.ts",INF+"/🧪️tests/🔗️late-closures/🟦️.ts",
 INF+"/🧱️project-assembly/🧪️tests/🟦️.ts",LIB+"/⚡️caching/🧪️tests/⚡️cache-contracts/🧾️command-inputs/🟦️.ts",
 LIB+"/🏗️build/📥️source/🔏️closure/🗓️scheduling/🧪️tests/🟦️.ts",INF+"/🔖️stamps/🧪️tests/🟦️.ts",
 INP+"/🧪️tests/🟦️.ts",INP+"/📋️data/🧪️tests/🟦️.ts",INP+"/🌳️owner-view/🧪️tests/🧬️inference/🟦️.ts",INP+"/🧮️digest/🧪️tests/🟦️.ts",
]+[UC+"/"+p for p in["🧪️tests/🟦️.ts","🧪️tests/🔁️churn/🟦️.ts","🧪️tests/🕵️leaks/🟦️.ts","🧪️tests/🕰️freshness/🟦️.ts","🧪️tests/🌱️incremental/🟦️.ts","✅️complete/🧪️tests/🟦️.ts","🏁️exit/🧪️tests/🟦️.ts","🛰️fleet/🧪️tests/🟦️.ts","🪪️extern/🧪️tests/🟦️.ts","🔏️key/🧪️tests/🟦️.ts"]]+list(e40.SAME_AS_BEFORE)
SAME_AS_BEFORE=list(dict.fromkeys(SAME_AS_BEFORE))
def short(rel):return rel[len(LIB)+1:] if rel.startswith(LIB) else rel
def sha(b):return hashlib.sha256(b).hexdigest()
def rd(p):
    with open(p,"rb") as f:return f.read()
def row_satisfied(rel,new,count):
    p=R+"/"+rel
    return os.path.exists(p) and rd(p).decode("utf8").count(new)==count
def satisfied(name,rows,files,src):
    for rel,old,new,count in rows:
        if name=="R29" and rel in STAMP_ROWS:continue
        if not row_satisfied(rel,new,count):return False
    for dest,srcname in files:
        p=R+"/"+dest
        if not os.path.exists(p) or rd(p)!=rd(src+"/"+srcname):return False
    return True
def window():
    skip=set(filter(None,os.environ.get("LAND_SKIP","").split(",")))
    pending=[(n,rows,files,src) for n,rows,files,src in SETS if n not in skip and not satisfied(n,rows,files,src)]
    names=[n for n,_,_,_ in pending]
    return pending,names
def stamp_bump(names):
    if "R29" in names or not ({"R37","R40"}&set(names)):return []
    out=[]
    for rel,pattern,template in((STAMP_SCHEMA,r'"stamp": "semio\.graph-inferrer\.script-inputs/(\d+)"','"stamp": "semio.graph-inferrer.script-inputs/%d"'),(STAMP_FIXTURE,r'"semio\.graph-inferrer\.script-inputs/(\d+)": \{','"semio.graph-inferrer.script-inputs/%d": {')):
        text=rd(R+"/"+rel).decode("utf8");found=re.findall(pattern,text)
        if len(found)!=1:sys.exit("stamp row not unique in "+rel)
        n=int(found[0]);out.append((rel,template%n,template%(n+1),1))
    return out
def plan(merge):
    skip=set(filter(None,os.environ.get("LAND_SKIP","").split(",")))
    base=json.load(open(SCRIPTS+"/R29/baseline.json"));base40=json.load(open(HERE+"/baseline.json")) if os.path.exists(HERE+"/baseline.json") else {}
    pending,names=window();allsets=[(n,rows,files,src) for n,rows,files,src in pending]
    if "R40" in names and "R39" in skip:sys.exit("R40 imports the unit-cache masker of R39: land them together")
    bump=stamp_bump(names)
    out={};refused=[]
    rels=sorted({p for _,rows,_,_ in allsets for p,_,_,_ in rows}|{p for p,_,_,_ in bump})
    for rel in rels:
        p=R+"/"+rel
        if not os.path.exists(p):refused.append((rel,"live file is gone"));continue
        raw=rd(p);text=raw.decode("utf8");edited=text;ok=True
        for name,rows,_,_ in allsets+[("stamp",bump,[],"")]:
            for q,old,new,count in rows:
                if q!=rel:continue
                if name=="R29" and q in STAMP_ROWS and "R29" in names:pass
                if edited.count(new)==count and name!="stamp":continue
                n=edited.count(old)
                if n!=count:refused.append((rel,"%s anchor occurs %d times, expected %d: %r"%(name,n,count,old[:70])));ok=False;break
                edited=edited.replace(old,new)
            if not ok:break
        if not ok:continue
        if edited==text:out[rel]=("same",raw);continue
        want=base.get(rel) or base40.get(rel)
        if want is not None and sha(raw)!=want and not merge:refused.append((rel,"live differs from the prepared baseline (live %s base %s)"%(sha(raw)[:12],want[:12])));continue
        out[rel]=("copy" if (want is None or sha(raw)==want) else "merge",edited.encode("utf8"))
    for name,_,files,src in allsets:
        for dest,srcname in files:
            data=rd(src+"/"+srcname);p=R+"/"+dest
            if os.path.exists(p):
                if rd(p)==data:out[dest]=("same",data)
                else:refused.append((dest,"new file exists with other content"))
            else:out[dest]=("new",data)
    return out,refused,names
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
        backup=sys.argv[2];lock=T+"/landing.lock";owner="R40-rollback-%d"%os.getpid()
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
                        parent=os.path.dirname(p)
                        while parent.startswith(R+"/") and len(parent)>len(R)+1:
                            try:os.rmdir(parent)
                            except OSError:break
                            parent=os.path.dirname(parent)
                        continue
                    p=R+"/"+rel;open(p+".r40tmp","wb").write(rd(os.path.join(d,f)));os.replace(p+".r40tmp",p)
            print("restored from",backup)
        finally:
            if open(lock+"/owner").read().strip()==owner:os.remove(lock+"/owner");os.rmdir(lock)
        return
    merge="--merge" in sys.argv
    out,refused,names=plan(merge)
    print("sets:",", ".join(names) if names else "none (everything is in the live tree)")
    for rel,(how,data) in out.items():print(how.ljust(5),short(rel),sha(data)[:12])
    for rel,why in refused:print("REFUSED",short(rel),why)
    if mode=="--overlay":
        dest=os.path.abspath(arg("--overlay"))
        if not os.path.realpath(os.path.dirname(dest)).startswith("/private/tmp/claude-501/"):sys.exit("overlay must live under the session scratch")
        if os.path.exists(dest):sys.exit("overlay target exists: "+dest)
        for rel,(how,data) in out.items():
            if how=="same":continue
            p=dest+"/"+rel;os.makedirs(os.path.dirname(p),exist_ok=True);open(p,"wb").write(data)
        print("overlay written:",sum(1 for h,_ in out.values() if h!="same"),"files");return
    if mode!="--land":print("dry run: nothing written"+("; %d refused"%len(refused) if refused else ""));return
    if refused:sys.exit("refused: a live file changed since the payload, rebuild the payload or use --merge")
    if all(h=="same" for h,_ in out.values()):print("nothing to land");return
    if not os.path.exists(N+"/LAND-OK") and "--authorized" not in sys.argv:sys.exit("LAND-OK does not exist")
    before={}
    if "--no-tests" not in sys.argv:
        for rel in SAME_AS_BEFORE:before[rel]=run_test(rel)
    lock=T+"/landing.lock";owner="R40-%d-%d"%(os.getpid(),int(time.time()));held="--lock-held" in sys.argv
    os.makedirs(T,exist_ok=True)
    if held:
        if not os.path.isdir(lock):sys.exit("--lock-held, but T/landing.lock does not exist")
    else:
        try:os.mkdir(lock)
        except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip()+": wait, never remove it")
        open(lock+"/owner","w").write(owner+"\n")
    stamp=datetime.datetime.now().strftime("%Y-%m-%d %H:%M");backup=T+"/🗑️generated/R40-landing-"+datetime.datetime.now().strftime("%H%M%S")+"-%d"%os.getpid()
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
            os.makedirs(os.path.dirname(p),exist_ok=True);open(p+".r40tmp","wb").write(data)
        for rel,(how,data) in out.items():
            if how!="same":os.replace(R+"/"+rel+".r40tmp",R+"/"+rel)
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
            f.write("\n## Package R40 landing (%s): window of sets %s (R29 blast radius of shared lists, R37 digest helper selection, R39 interface identity in the unit-cache wrapper, R40 interface closure of check-class Rust targets; numbers in WORKERS-READY result-R29/R37/R39/R40.md)\n"%(stamp,", ".join(names)))
            for rel,(how,data) in out.items():f.write("- %s `%s` before %s after %s\n"%(how,short(rel) if rel.startswith(LIB) else rel,prior.get(rel,"-")[:12],sha(data)[:12]))
            for rel,failed,verdict in res:f.write("- test `LIB/%s` red %d: %s\n"%(short(rel),failed,verdict))
            f.write("- originals for `land.py --rollback`: `%s`\n"%backup)
        if any(r[2]!="as expected" for r in res):print("UNEXPECTED test result: inspect, then --rollback "+backup)
    finally:
        if not held and os.path.exists(lock+"/owner") and open(lock+"/owner").read().strip()==owner:
            os.remove(lock+"/owner");os.rmdir(lock)
main()
