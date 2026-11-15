#!/usr/bin/env python3
"""R14 landing (the fleet read waits as long as the serving machine's progress record advances, a request is idempotent and incremental, the server orders by demand): three-way merges new/ over the live files of files.json (base/ is the original, files.json carries its sha256) and lands them in one burst under T/landing.lock.
usage: land.py --dry-run | --land [--lock-held] [--no-tests] | --rollback <backup-dir>      (--root <dir> uses another tree, for rehearsals)
  A live file equal to the baseline is replaced by the new file; a changed live file is merged with `git merge-file -p` on temporary copies outside the worktree (a read-only use of git), a conflict refuses the landing. Every merged output must parse (TypeScript through Bun.Transpiler, JSON through JSON.parse) before anything is written. --land needs N/LAND-OK, writes all files first and replaces them back to back, keeps the originals in T/🗑️generated/R14-landing-<time>/ and runs the test files of TESTS with BUN_RUNTIME_TRANSPILER_CACHE_PATH=0."""
import os,sys,json,subprocess,hashlib,time,shutil,datetime,tempfile
HERE=os.path.dirname(os.path.abspath(__file__))
def arg(name):
    return sys.argv[sys.argv.index(name)+1] if name in sys.argv else None
def root():
    if arg("--root"):return os.path.abspath(arg("--root"))
    for c in ["/Users/Shared/semio","/Users/ueli/Documents/semio"]:
        if os.path.exists(c+"/AGENTS.md") and os.path.exists(c+"/.cache/f/machine.json"):return c
    sys.exit("no worktree root found")
R=root()
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
N=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
W=LIB+"/🪪️graph-state/🙋️wanted"
CONF=json.load(open(HERE+"/files.json",encoding="utf8"))
FILES=CONF["files"];BASE=CONF["baseline"]
TESTS=[W+"/🧪️tests/🟦️.ts",W+"/🧪️tests/📖️read/🟦️.ts",W+"/🧪️tests/🛠️server/🟦️.ts",W+"/🧪️tests/🛠️server/💰️priority/🟦️.ts",W+"/🧪️tests/🛠️server/🌊️wave-rows/🟦️.ts",W+"/🧪️tests/🌍️two-platforms/🟦️.ts",W+"/🧪️tests/🍏️idle-grant/🟦️.ts",W+"/💰️cost/🧪️tests/🟦️.ts",W+"/💰️cost/🧪️tests/🌍️measured-only/🟦️.ts",
 LIB+"/🌐️remote-execution/🛰️sidecar/🧪️tests/🕸️graph/🟦️.ts",LIB+"/🌐️remote-execution/🛰️sidecar/🧪️tests/🕸️graph/💰️wave-costs/🟦️.ts",LIB+"/🌐️remote-execution/🛰️sidecar/🧪️tests/🕸️graph/🌊️wave-rows/🟦️.ts",LIB+"/🌐️remote-execution/🛰️sidecar/🧪️tests/🕸️graph/📡️wave-upload/🟦️.ts",
 LIB+"/🕸️graph/🕰️generations/🌐️remote-exchange/🧪️tests/🏁️head-rank/🟦️.ts",LIB+"/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🛑️too-heavy/🟦️.ts",LIB+"/🕸️graph/🧪️tests/🙋️fleet-read/🟦️.ts",LIB+"/🕸️graph/🧪️tests/🙋️fleet-read/🌍️small-booking/🟦️.ts",LIB+"/🕸️graph/🧪️tests/🙋️fleet-read/🍏️macos-reading/🟦️.ts"]
def sha(b):return hashlib.sha256(b).hexdigest()
def rd(p):
    with open(p,"rb") as f:return f.read()
def merge(short,live):
    with tempfile.TemporaryDirectory(prefix="r14-merge-") as d:
        for name,data in (("live",live),("base",rd(HERE+"/base/"+short)),("new",rd(HERE+"/new/"+short))):open(d+"/"+name,"wb").write(data)
        r=subprocess.run(["git","merge-file","-p","-L","live","-L","base","-L","new",d+"/live",d+"/base",d+"/new"],capture_output=True)
        return (r.stdout if r.returncode==0 else None),r.returncode
def parses(rel,data):
    if rel.endswith(".json"):
        try:json.loads(data.decode("utf8"));return True,""
        except Exception as e:return False,str(e)
    with tempfile.TemporaryDirectory(prefix="r14-parse-") as d:
        open(d+"/in.ts","wb").write(data)
        code='const t=new Bun.Transpiler({loader:"ts"});t.transformSync(await Bun.file(process.env.R14_PARSE).text());'
        r=subprocess.run([BUN,"-e",code],capture_output=True,text=True,cwd=d,env={**os.environ,"R14_PARSE":d+"/in.ts"})
        return r.returncode==0,(r.stderr or r.stdout)[:300]
def plan():
    out={};refused=[]
    for short,rel in FILES:
        p=R+"/"+rel
        if not os.path.exists(p):refused.append((rel,"live file is gone"));continue
        live=rd(p);new=rd(HERE+"/new/"+short)
        if live==new:out[rel]=("same",live);continue
        if sha(live)==BASE[rel]:data,how=new,"copy"
        else:
            data,code=merge(short,live)
            if data is None:refused.append((rel,"merge conflict (%d)"%code));continue
            how="merge"
        ok,why=parses(rel,data)
        if not ok:refused.append((rel,"does not parse: "+why));continue
        out[rel]=(how,data)
    return out,refused
def short(rel):return rel[len(LIB)+1:] if rel.startswith(LIB) else rel
def main():
    mode=sys.argv[1] if len(sys.argv)>1 else "--dry-run"
    if mode=="--rollback":
        backup=sys.argv[2];lock=T+"/landing.lock";owner="R14-rollback-%d"%os.getpid()
        try:os.mkdir(lock)
        except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
        open(lock+"/owner","w").write(owner+"\n")
        try:
            for d,_,fs in os.walk(backup):
                for f in fs:
                    rel=os.path.relpath(os.path.join(d,f),backup);p=R+"/"+rel
                    open(p+".r14tmp","wb").write(rd(os.path.join(d,f)));os.replace(p+".r14tmp",p)
            print("restored from",backup)
        finally:
            if open(lock+"/owner").read().strip()==owner:os.remove(lock+"/owner");os.rmdir(lock)
        return
    out,refused=plan()
    for rel,(how,data) in out.items():print(how.ljust(5),short(rel),sha(data)[:12])
    for rel,why in refused:print("REFUSED",short(rel),why)
    if mode!="--land":print("dry run: nothing written"+("; %d refused"%len(refused) if refused else ""));return
    if refused:sys.exit("refused")
    if not os.path.exists(N+"/LAND-OK"):sys.exit("LAND-OK does not exist")
    lock=T+"/landing.lock";owner="R14-%d-%d"%(os.getpid(),int(time.time()));held="--lock-held" in sys.argv
    if held:
        if not os.path.isdir(lock):sys.exit("--lock-held, but T/landing.lock does not exist")
    else:
        try:os.mkdir(lock)
        except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip()+": wait, never remove it")
        open(lock+"/owner","w").write(owner+"\n")
    stamp=datetime.datetime.now().strftime("%Y-%m-%d %H:%M");backup=T+"/🗑️generated/R14-landing-"+datetime.datetime.now().strftime("%H%M%S")
    try:
        out,refused=plan()
        if refused:sys.exit("refused after taking the lock: "+"; ".join(r for r,_ in refused))
        before={};after={}
        for rel,(how,data) in out.items():
            p=R+"/"+rel
            if how=="same":continue
            before[rel]=sha(rd(p));after[rel]=sha(data)
            os.makedirs(os.path.dirname(backup+"/"+rel),exist_ok=True);shutil.copyfile(p,backup+"/"+rel)
            open(p+".r14tmp","wb").write(data)
        for rel,(how,data) in out.items():
            if how!="same":os.replace(R+"/"+rel+".r14tmp",R+"/"+rel)
        print("landed",len(after),"files; originals in",backup)
        res=[]
        for rel in ([] if "--no-tests" in sys.argv else TESTS):
            cwd=os.path.dirname(R+"/"+rel);env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))}
            env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
            r=subprocess.run([BUN,"test","./"+os.path.basename(rel),"--timeout","180000"],cwd=cwd,env=env,capture_output=True,text=True)
            text=r.stdout+r.stderr;failed=[l for l in text.splitlines() if l.startswith("(fail)")]
            res.append((rel,r.returncode,len(failed)));print(("ok " if r.returncode==0 and not failed else "RED").ljust(4),short(rel),*failed,sep="\n  " if failed else " ")
        with open(T+"/landings.md","a",encoding="utf8") as f:
            f.write("\n## Package R14 landing (%s): the fleet read waits while the serving machine's progress record advances (typed refusal only after a bound without progress or without a sign of life), a request is idempotent and renewed, the server orders by demand and renews its claims\n"%stamp)
            for rel in after:f.write("- file `%s` before %s after %s\n"%(rel,before[rel][:12],after[rel][:12]))
            for rel,code,failed in res:f.write("- test `%s` exit %d, %d red\n"%(rel,code,failed))
        print("backup kept at",backup,"(delete it when the landing is verified)")
        if any(code!=0 for _,code,_ in res):print("SOME TESTS ARE RED: compare with the baseline before deciding on --rollback",backup)
    finally:
        if not held and os.path.isdir(lock) and open(lock+"/owner").read().strip()==owner:os.remove(lock+"/owner");os.rmdir(lock)
main()
