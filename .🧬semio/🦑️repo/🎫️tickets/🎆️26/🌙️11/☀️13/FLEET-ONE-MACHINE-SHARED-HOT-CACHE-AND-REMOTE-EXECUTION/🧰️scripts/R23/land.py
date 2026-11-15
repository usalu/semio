#!/usr/bin/env python3
"""R23 landing (one project record serves macOS, Linux and Windows machines: the record is published in a neutral form of the host's spelling and taken in the taker's, records of every platform share the neutral tag, a machine of any platform class serves a request except its host-bound owners, which it names and leaves to a machine of the class, and the heartbeat and the request lines say how many projects were served across classes and how many were left): replaces the files of payload.json (whole files; every live file must still be the one the payload was made from, or already the new one) in one burst under T/landing.lock.
usage: land.py --dry-run | --parse | --land [--lock-held] [--authorized] [--no-tests] | --rollback <backup-dir>      (--root <dir> uses another tree, for rehearsals)
  --parse runs the parse proof on the payload files without touching the worktree: every .ts/.mjs through the Bun transpiler, every .json through a JSON parser, every .py through py_compile.
  --land needs N/LAND-OK (or --authorized) and refuses when a live file differs from payload.json (base = the file the payload was made from, new = the payload); with --lock-held it expects T/landing.lock to be held by the caller, otherwise it takes it and never removes a lock it does not own; it writes all files first and replaces them back to back, keeps the originals in T/🗑️generated/R23-landing-<time>/ and then runs the test files of TESTS (live), appends to T/landings.md.
  The real `repo` build command of the landing rules is not run here: the coordinator runs it after the landing (no graph command on the machine that made the payload)."""
import os,sys,json,subprocess,hashlib,time,shutil,datetime
HERE=os.path.dirname(os.path.abspath(__file__))
def arg(name):return sys.argv[sys.argv.index(name)+1] if name in sys.argv else None
def root():
    if arg("--root"):return os.path.abspath(arg("--root"))
    for c in ["/Users/Shared/semio","/Users/ueli/Documents/semio"]:
        if os.path.exists(c+"/AGENTS.md") and os.path.exists(c+"/.cache/f/machine.json"):return c
    sys.exit("no worktree root found")
R=root()
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
GS=LIB+"/🪪️graph-state"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
N=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
FILES=json.load(open(HERE+"/payload.json",encoding="utf8"))
TESTS=[
 GS+"/🌍️platform-class/🧪️tests/🟦️.ts",
 GS+"/📦️project-records/🧳️portable/🧪️tests/🟦️.ts",GS+"/📦️project-records/🧳️portable/🧪️tests/🌍️real/🟦️.ts",
 GS+"/📦️project-records/🧪️tests/🟦️.ts",GS+"/📦️project-records/🧪️tests/🌍️two-platforms/🟦️.ts",
 GS+"/🙋️wanted/🧪️tests/🟦️.ts",GS+"/🙋️wanted/🧪️tests/🛠️server/🟦️.ts",GS+"/🙋️wanted/🧪️tests/🛠️server/💰️priority/🟦️.ts",GS+"/🙋️wanted/🧪️tests/🛠️server/🌊️wave-rows/🟦️.ts",
 GS+"/🙋️wanted/🧪️tests/📖️read/🟦️.ts",GS+"/🙋️wanted/🧪️tests/🌍️two-platforms/🟦️.ts",GS+"/🙋️wanted/💰️cost/🧪️tests/🟦️.ts",GS+"/🙋️wanted/💰️cost/🧪️tests/🌍️measured-only/🟦️.ts",
 LIB+"/🕸️graph/🧪️tests/🙋️fleet-read/🟦️.ts",LIB+"/🕸️graph/🧪️tests/🙋️fleet-read/🌍️small-booking/🟦️.ts",LIB+"/🕸️graph/🧪️tests/🙋️fleet-read/🍏️macos-reading/🟦️.ts",
 LIB+"/🌐️remote-execution/🛰️sidecar/🧪️tests/🕸️graph/🟦️.ts",LIB+"/🌐️remote-execution/🛰️sidecar/🧪️tests/🕸️graph/🔬️stall/🟦️.ts",LIB+"/🌐️remote-execution/🛰️sidecar/🧪️tests/🕸️graph/🌊️wave-rows/🟦️.ts",LIB+"/🌐️remote-execution/🛰️sidecar/🧪️tests/🕸️graph/📡️wave-upload/🟦️.ts",LIB+"/🌐️remote-execution/🛰️sidecar/🧪️tests/🕸️graph/💰️wave-costs/🟦️.ts",
 LIB+"/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🛑️too-heavy/🟦️.ts",
]
BASELINE_RED={LIB+"/🕸️graph/🧪️tests/🙋️fleet-read/🌍️small-booking/🟦️.ts":3,LIB+"/🕸️graph/🧪️tests/🙋️fleet-read/🍏️macos-reading/🟦️.ts":3}
def sha(b):return hashlib.sha256(b).hexdigest()
def rd(p):
    with open(p,"rb") as f:return f.read()
def short(rel):return rel[len(LIB)+1:] if rel.startswith(LIB) else rel
def plan():
    out={};refused=[]
    for f in FILES:
        p=R+"/"+f["dest"];data=rd(HERE+"/new/"+f["flat"])
        if sha(data)!=f["new"]:refused.append((f["dest"],"payload file damaged"));continue
        live=sha(rd(p)) if os.path.exists(p) else None
        if live==f["new"]:out[f["dest"]]=("same",data,live);continue
        if f["kind"]=="new":
            if live is not None:refused.append((f["dest"],"new file exists with other content"));continue
            out[f["dest"]]=("new",data,live);continue
        if live!=f["base"]:refused.append((f["dest"],"live differs from the file the payload was made from (live %s base %s)"%(str(live)[:12],f["base"][:12])));continue
        out[f["dest"]]=("edit",data,live)
    return out,refused
def parse_proof():
    bad=0
    for f in FILES:
        path=HERE+"/new/"+f["flat"];name=f["dest"];ext=os.path.splitext(name)[1]
        if ext in(".ts",".mjs"):
            r=subprocess.run([BUN,"build","--no-bundle","--target=bun",path],capture_output=True,text=True);ok=r.returncode==0
        elif ext==".json":
            try:json.load(open(path,encoding="utf8"));ok=True
            except Exception as e:ok=False
        elif ext==".py":
            r=subprocess.run([sys.executable,"-c","import ast,sys;ast.parse(open(sys.argv[1],encoding='utf8').read())",path],capture_output=True,text=True);ok=r.returncode==0
        else:ok=True
        print("parsed" if ok else "BROKEN",short(name));bad+=0 if ok else 1
    print("parse proof:",len(FILES)-bad,"of",len(FILES),"files parse");return bad
def main():
    mode=sys.argv[1] if len(sys.argv)>1 else "--dry-run"
    if mode=="--parse":sys.exit(1 if parse_proof() else 0)
    if mode=="--rollback":
        backup=sys.argv[2];lock=T+"/landing.lock";owner="R23-rollback-%d"%os.getpid()
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
                    p=R+"/"+rel;open(p+".r23tmp","wb").write(rd(os.path.join(d,f)));os.replace(p+".r23tmp",p)
            print("restored from",backup)
        finally:
            if open(lock+"/owner").read().strip()==owner:os.remove(lock+"/owner");os.rmdir(lock)
        return
    out,refused=plan()
    for rel,(how,data,live) in out.items():print(how.ljust(5),short(rel),sha(data)[:12])
    for rel,why in refused:print("REFUSED",short(rel),why)
    if mode!="--land":print("dry run: nothing written"+("; %d refused"%len(refused) if refused else ""));return
    if refused:sys.exit("refused: a live file changed since the payload was made, regenerate the payload")
    if not os.path.exists(N+"/LAND-OK") and "--authorized" not in sys.argv:sys.exit("LAND-OK does not exist")
    lock=T+"/landing.lock";owner="R23-%d-%d"%(os.getpid(),int(time.time()));held="--lock-held" in sys.argv
    os.makedirs(T,exist_ok=True)
    if held:
        if not os.path.isdir(lock):sys.exit("--lock-held, but T/landing.lock does not exist")
    else:
        try:os.mkdir(lock)
        except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip()+": wait, never remove it")
        open(lock+"/owner","w").write(owner+"\n")
    stamp=datetime.datetime.now().strftime("%Y-%m-%d %H:%M");backup=T+"/🗑️generated/R23-landing-"+datetime.datetime.now().strftime("%H%M%S")
    try:
        out,refused=plan()
        if refused:sys.exit("refused after taking the lock: "+"; ".join(r for r,_ in refused))
        for rel,(how,data,live) in out.items():
            p=R+"/"+rel
            if how=="same":continue
            os.makedirs(os.path.dirname(backup+"/"+rel),exist_ok=True)
            if os.path.exists(p):shutil.copyfile(p,backup+"/"+rel)
            else:open(backup+"/"+rel+".absent","w").close()
            os.makedirs(os.path.dirname(p),exist_ok=True);open(p+".r23tmp","wb").write(data)
        for rel,(how,data,live) in out.items():
            if how!="same":os.replace(R+"/"+rel+".r23tmp",R+"/"+rel)
        print("landed",sum(1 for v in out.values() if v[0]!="same"),"files; originals in",backup)
        res=[]
        for rel in ([] if "--no-tests" in sys.argv else TESTS):
            cwd=os.path.dirname(R+"/"+rel);env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))}
            env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
            r=subprocess.run([BUN,"test","./"+os.path.basename(rel),"--timeout","180000"],cwd=cwd,env=env,capture_output=True,text=True)
            text=r.stdout+r.stderr;failed=[l for l in text.splitlines() if l.startswith("(fail)")]
            expected=BASELINE_RED.get(rel,0)
            verdict="as expected" if (r.returncode==0 and not failed) or (len(failed)==expected and expected>0) else "UNEXPECTED"
            res.append((rel,r.returncode,len(failed),verdict,expected));print(verdict.ljust(11),short(rel),"fail",len(failed),*failed,sep="\n  " if failed else " ")
        with open(T+"/landings.md","a",encoding="utf8") as f:
            f.write("\n## Package R23 landing (%s): one project record serves macOS, Linux and Windows machines; a machine of any class serves a request except its host-bound owners, which it names and leaves\n"%stamp)
            for rel,(how,data,live) in out.items():f.write("- %s `%s` before %s after %s\n"%(how,short(rel),(live or "-")[:12],sha(data)[:12]))
            for rel,code,failed,verdict,expected in res:f.write("- test `LIB/🏗️build/%s` exit %d, %d red (baseline %d): %s\n"%(short(rel),code,failed,expected,verdict))
            f.write("- originals for `land.py --rollback`: `%s`\n"%backup)
        if any(r[3]=="UNEXPECTED" for r in res):print("UNEXPECTED test result: inspect, then --rollback "+backup)
    finally:
        if not held and os.path.exists(lock+"/owner") and open(lock+"/owner").read().strip()==owner:
            os.remove(lock+"/owner");os.rmdir(lock)
main()
