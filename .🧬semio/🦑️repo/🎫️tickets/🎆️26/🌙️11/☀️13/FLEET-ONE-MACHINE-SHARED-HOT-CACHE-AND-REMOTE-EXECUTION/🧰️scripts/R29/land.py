#!/usr/bin/env python3
"""R29 landing (PREPARED ONLY, the coordinator lands it): shared file lists that 20 or more project nodes embedded become declared tools of the providers project that the nodes name by reference (26 tool rows, the native toolchain files by reference, schema bounds, corpus rows, inferrer stamp script-inputs/5, one new law test). --dry-run (default), --land, --rollback <dir>, --merge, --no-tests, --authorized, --lock-held."""
import os,sys,json,subprocess,hashlib,time,shutil,datetime
HERE=os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0,HERE)
import edits
def arg(name):
    return sys.argv[sys.argv.index(name)+1] if name in sys.argv else None
def root():
    if arg("--root"):return os.path.abspath(arg("--root"))
    for c in ["/Users/Shared/semio","/Users/ueli/Documents/semio"]:
        if os.path.exists(c+"/AGENTS.md") and os.path.exists(c+"/.cache/f/machine.json"):return c
    sys.exit("no worktree root found")
R=root();LIB=edits.LIB;L=R+"/"+LIB
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
N=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
def short(rel):return rel[len(LIB)+1:] if rel.startswith(LIB) else rel
INF=edits.LIB+"/🏗️build/🕸️graph/📥️inference"
SI=INF+"/🧭️script-inputs"
TESTS=[(short(rel),expected,why) for rel,expected,why in [
 (SI+"/🔌️provider-closures/🧪️tests/🟦️.ts",0,""),
 (SI+"/🔌️provider-closures/🧪️tests/🪡️tool-cover/🟦️.ts",0,""),
 (SI+"/🔌️provider-closures/🧪️tests/🧮️blast-radius/🟦️.ts",0,""),
 (SI+"/🧪️tests/🟦️.ts",0,""),
 (SI+"/🔀️router/🧪️tests/🟦️.ts",0,""),
 (SI+"/👓️walk-views/🧪️tests/🟦️.ts",0,""),
 (INF+"/🧪️tests/🧷️closure-facts/🟦️.ts",0,""),
 (INF+"/🧪️tests/🔖️output-stamps/🟦️.ts",0,""),
 (INF+"/🧪️tests/🔗️late-closures/🟦️.ts",0,""),
 (INF+"/🧱️project-assembly/🧪️tests/🟦️.ts",0,""),
 (edits.LIB+"/⚡️caching/🧪️tests/⚡️cache-contracts/🧾️command-inputs/🟦️.ts",0,""),
 (edits.LIB+"/🏗️build/📥️source/🔏️closure/🗓️scheduling/🧪️tests/🟦️.ts",0,""),
 (INF+"/🔖️stamps/🧪️tests/🟦️.ts",0,""),
]]
def short(rel):return rel[len(LIB)+1:] if rel.startswith(LIB) else rel
def sha(b):return hashlib.sha256(b).hexdigest()
def rd(p):
    with open(p,"rb") as f:return f.read()
def plan(merge):
    base=json.load(open(HERE+"/baseline.json"));out={};refused=[]
    for rel in sorted({p for p,_,_,_ in edits.EDITS}):
        p=R+"/"+rel
        if not os.path.exists(p):refused.append((rel,"live file is gone"));continue
        raw=rd(p);text=raw.decode("utf8");want=base.get(rel)
        edited=text;ok=True
        for q,old,new,count in edits.EDITS:
            if q!=rel:continue
            if edited.count(new)==count:continue
            n=edited.count(old)
            if n!=count:refused.append((rel,"anchor occurs %d times, expected %d: %r"%(n,count,old[:70])));ok=False;break
            edited=edited.replace(old,new)
        if not ok:continue
        if edited==text:out[rel]=("same",raw);continue
        if sha(raw)!=want and not merge:refused.append((rel,"live differs from the baseline (live %s base %s)"%(sha(raw)[:12],str(want)[:12])));continue
        out[rel]=("copy" if sha(raw)==want else "merge",edited.encode("utf8"))
    for dest,src in edits.NEWFILES:
        data=rd(HERE+"/new/"+src);p=R+"/"+dest
        if os.path.exists(p):
            out[dest]=("same",data) if rd(p)==data else None
            if out[dest] is None:out.pop(dest);refused.append((dest,"new file exists with other content"))
        else:out[dest]=("new",data)
    return out,refused
def main():
    mode=sys.argv[1] if len(sys.argv)>1 else "--dry-run"
    if mode=="--rollback":
        backup=sys.argv[2];lock=T+"/landing.lock";owner="R29-rollback-%d"%os.getpid()
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
                    p=R+"/"+rel;open(p+".r29tmp","wb").write(rd(os.path.join(d,f)));os.replace(p+".r29tmp",p)
            print("restored from",backup)
        finally:
            if open(lock+"/owner").read().strip()==owner:os.remove(lock+"/owner");os.rmdir(lock)
        return
    merge="--merge" in sys.argv
    out,refused=plan(merge)
    for rel,(how,data) in out.items():print(how.ljust(5),short(rel),sha(data)[:12])
    for rel,why in refused:print("REFUSED",short(rel),why)
    if mode!="--land":print("dry run: nothing written"+("; %d refused"%len(refused) if refused else ""));return
    if refused:sys.exit("refused: a live file changed since the baseline, rebuild the payload or use --merge")
    if not os.path.exists(N+"/LAND-OK") and "--authorized" not in sys.argv:sys.exit("LAND-OK does not exist")
    lock=T+"/landing.lock";owner="R29-%d-%d"%(os.getpid(),int(time.time()));held="--lock-held" in sys.argv
    os.makedirs(T,exist_ok=True)
    if held:
        if not os.path.isdir(lock):sys.exit("--lock-held, but T/landing.lock does not exist")
    else:
        try:os.mkdir(lock)
        except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip()+": wait, never remove it")
        open(lock+"/owner","w").write(owner+"\n")
    stamp=datetime.datetime.now().strftime("%Y-%m-%d %H:%M");backup=T+"/🗑️generated/R29-landing-"+datetime.datetime.now().strftime("%H%M%S")
    try:
        out,refused=plan(merge)
        if refused:sys.exit("refused after taking the lock: "+"; ".join(r for r,_ in refused))
        before={}
        for rel,(how,data) in out.items():
            p=R+"/"+rel
            if how=="same":continue
            before[rel]=sha(rd(p)) if os.path.exists(p) else "-"
            os.makedirs(os.path.dirname(backup+"/"+rel),exist_ok=True)
            if os.path.exists(p):shutil.copyfile(p,backup+"/"+rel)
            else:open(backup+"/"+rel+".absent","w").close()
            os.makedirs(os.path.dirname(p),exist_ok=True);open(p+".r29tmp","wb").write(data)
        for rel,(how,data) in out.items():
            if how!="same":os.replace(R+"/"+rel+".r29tmp",R+"/"+rel)
        print("landed",sum(1 for h,_ in out.values() if h!="same"),"files; originals in",backup)
        res=[]
        for rel,expected,why in ([] if "--no-tests" in sys.argv else TESTS):
            cwd=os.path.dirname(L+"/"+rel);env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))}
            env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";os.makedirs(T+"/🗑️generated",exist_ok=True);env["SEMIO_TEST_ARTIFACT_DIR"]=T+"/🗑️generated"
            r=subprocess.run([BUN,"test","./"+os.path.basename(rel),"--timeout","180000"],cwd=cwd,env=env,capture_output=True,text=True)
            text=r.stdout+r.stderr;failed=[l for l in text.splitlines() if l.startswith("(fail)")]
            verdict="as expected" if len(failed)==expected and (r.returncode==0)==(expected==0) else "UNEXPECTED"
            res.append((rel,r.returncode,len(failed),expected,verdict,text.strip().splitlines()[-3:]));print(verdict.ljust(11),rel,"fail",len(failed),"expected",expected,*failed,sep="\n  " if failed else " ")
        with open(T+"/landings.md","a",encoding="utf8") as f:
            f.write("\n## Package R29 landing (%s): shared file lists of 20 or more nodes become declared tools named by reference (26 tool rows, native toolchain files, stamp script-inputs/5; numbers in WORKERS-READY result-R29.md)\n"%stamp)
            for rel,(how,data) in out.items():f.write("- %s `%s` before %s after %s\n"%(how,short(rel) if rel.startswith(LIB) else rel,before.get(rel,"-")[:12],sha(data)[:12]))
            for rel,code,failed,expected,verdict,tail in res:f.write("- test `LIB/🏗️build/%s` exit %d, %d red (%d expected): %s\n"%(rel,code,failed,expected,verdict))
            f.write("- originals for `land.py --rollback`: `%s`\n"%backup)
        if any(r[4]=="UNEXPECTED" for r in res):print("UNEXPECTED test result: inspect, then --rollback "+backup)
    finally:
        if not held and os.path.exists(lock+"/owner") and open(lock+"/owner").read().strip()==owner:
            os.remove(lock+"/owner");os.rmdir(lock)
main()
