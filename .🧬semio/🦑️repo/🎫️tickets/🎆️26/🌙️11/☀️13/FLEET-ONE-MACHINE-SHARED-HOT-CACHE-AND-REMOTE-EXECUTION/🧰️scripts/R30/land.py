# -*- coding: utf-8 -*-
"""R30 landing: the fleet read judges the machine, not the memory budget of the graph service (a Linux worker is no longer a 2 GB machine with a grant of 0 MB), the plan prints the readings it used, and the retirement plan wire shape holds the 18 declared image roles (bound 24, as the producer). Three-way merge of payload/base -> payload/new onto the live files (git merge-file on temp files, never touches the repository). usage: land.py --dry-run | --rehearse | --land [--lock-held] [--no-tests] | --rollback <dir>   (--root <dir> uses another tree; the rehearsal copies the live files into a scratch root, lands, rolls back and compares)."""
import os,sys,subprocess,hashlib,time,shutil,datetime,tempfile
HERE=os.path.dirname(os.path.abspath(__file__));P=HERE+"/payload"
def arg(n):return sys.argv[sys.argv.index(n)+1] if n in sys.argv else None
R=os.path.abspath(arg("--root")) if arg("--root") else "/Users/Shared/semio"
T="/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B=L+"🏗️build/";W=B+"🪪️graph-state/🙋️wanted/";C=L+"⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup/♻️retirement/"
TESTS=[(W+"🧪️tests/🐧️linux-host",["🟦️.ts"]),(C+"🧪️tests/📏️roster-bound",["🟦️.ts"]),(W+"🧪️tests/🍏️idle-grant",["🟦️.ts"]),(W+"🧪️tests/🌍️two-platforms",["🟦️.ts"]),(B+"🧠️memory/🧪️tests/🧹️parent-budget",["🟦️.ts"])]
def rd(p):
    with open(p,"rb") as f:return f.read()
def sha(b):return hashlib.sha256(b).hexdigest()
def files():return [l for l in open(P+"/files.txt",encoding="utf8").read().split("\n") if l]
def merge(rel):
    new=rd(P+"/new/"+rel);live=R+"/"+rel;base=P+"/base/"+rel
    if not os.path.exists(live):
        return ("new",new,b"") if not os.path.exists(base) else ("refused","live file is gone",b"")
    cur=rd(live)
    if cur==new:return ("same",new,cur)
    if not os.path.exists(base):return ("refused","new file exists with other content",cur)
    if cur==rd(base):return ("edit",new,cur)
    with tempfile.TemporaryDirectory() as d:
        open(d+"/live","wb").write(cur);r=subprocess.run(["git","merge-file","-p","-L","live","-L","base","-L","new",d+"/live",base,P+"/new/"+rel],capture_output=True)
        if r.returncode==0:return ("merge",r.stdout,cur)
        return ("refused","conflict with a concurrent edit (%d hunks)"%r.returncode,cur)
def parse_proof(rel,data):
    if rel.endswith(".json"):
        import json;json.loads(data.decode("utf8"));return None
    ext=rel.rsplit(".",1)[1];loader={"cjs":"js","mjs":"js","ts":"ts","mts":"ts"}[ext]
    code="const t=new Bun.Transpiler({loader:%r});t.transformSync(await Bun.file(process.argv[1]).text())"%loader
    with tempfile.NamedTemporaryFile(suffix="."+ext,delete=False) as f:f.write(data)
    try:
        r=subprocess.run([BUN,"-e",code,f.name],capture_output=True,text=True);return None if r.returncode==0 else r.stderr[:300]
    finally:os.remove(f.name)
def plan():
    out={};refused=[]
    for rel in files():
        how,data,raw=merge(rel)
        if how=="refused":refused.append((rel,data));continue
        err=parse_proof(rel,data)
        if err:refused.append((rel,"parse proof failed: "+err));continue
        out[rel]=(how,data,raw)
    return out,refused
def short(rel):return rel[len(L):] if rel.startswith(L) else rel
def apply(out,backup):
    for rel,(how,data,raw) in out.items():
        if how=="same":continue
        p=R+"/"+rel;os.makedirs(os.path.dirname(backup+"/"+rel),exist_ok=True)
        if how=="new":open(backup+"/"+rel+".absent","w").close()
        else:shutil.copyfile(p,backup+"/"+rel)
        os.makedirs(os.path.dirname(p),exist_ok=True);open(p+".r30tmp","wb").write(data)
    for rel,(how,data,raw) in out.items():
        if how!="same":os.replace(R+"/"+rel+".r30tmp",R+"/"+rel)
def rollback(backup):
    for d,_,fs in os.walk(backup):
        for f in fs:
            rel=os.path.relpath(os.path.join(d,f),backup)
            if f.endswith(".absent"):
                p=R+"/"+rel[:-7]
                if os.path.exists(p):os.remove(p)
                continue
            p=R+"/"+rel;open(p+".r30tmp","wb").write(rd(os.path.join(d,f)));os.replace(p+".r30tmp",p)
def run_tests():
    res=[]
    env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
    art=tempfile.mkdtemp(prefix="r30art");env["SEMIO_TEST_ARTIFACT_DIR"]=art
    for rel,names in TESTS:
        r=subprocess.run([BUN,"test"]+["./"+n for n in names]+["--timeout","180000"],cwd=R+"/"+rel,env=env,capture_output=True,text=True)
        text=r.stdout+r.stderr;failed=[l for l in text.splitlines() if l.startswith("(fail)")]
        res.append((rel,r.returncode,failed));print("ok   " if r.returncode==0 else "RED  ",short(rel),*failed,sep="\n  " if failed else " ")
    shutil.rmtree(art,ignore_errors=True);return res
def main():
    mode=sys.argv[1] if len(sys.argv)>1 else "--dry-run"
    if mode=="--rollback":rollback(sys.argv[2]);print("restored from",sys.argv[2]);return
    if mode=="--rehearse":
        global R
        live="/Users/Shared/semio";scratch=tempfile.mkdtemp(prefix="r30rehearsal",dir="/private/tmp/claude-501")
        for rel in files():
            if os.path.exists(live+"/"+rel):os.makedirs(os.path.dirname(scratch+"/"+rel),exist_ok=True);shutil.copyfile(live+"/"+rel,scratch+"/"+rel)
        R=scratch;out,refused=plan();assert not refused,refused
        before={rel:rd(R+"/"+rel) if os.path.exists(R+"/"+rel) else None for rel in files()}
        backup=scratch+"/.backup";apply(out,backup)
        for rel in files():assert rd(R+"/"+rel)==rd(P+"/new/"+rel),rel
        out2,ref2=plan();assert not ref2 and all(v[0]=="same" for v in out2.values()),"second landing is not idempotent"
        rollback(backup)
        for rel in files():
            now=rd(R+"/"+rel) if os.path.exists(R+"/"+rel) else None
            if before[rel] is None:
                assert now is None,rel
            else:assert now==before[rel],rel
        shutil.rmtree(scratch);print("rehearsal ok: landed, idempotent, rolled back byte-identical,",len(files()),"files");return
    out,refused=plan()
    for rel,(how,data,_) in out.items():print(how.ljust(5),short(rel),sha(data)[:12])
    for rel,why in refused:print("REFUSED",short(rel),why)
    if mode!="--land":print("dry run: nothing written"+("; %d refused"%len(refused) if refused else ""));return
    if refused:sys.exit("refused")
    lock=T+"/landing.lock";owner="R30-%d-%d"%(os.getpid(),int(time.time()));held="--lock-held" in sys.argv
    if held:
        if not os.path.isdir(lock):sys.exit("--lock-held, but landing.lock does not exist")
    else:
        try:os.mkdir(lock)
        except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip()+": wait, never remove it")
        open(lock+"/owner","w").write(owner+"\n")
    stamp=datetime.datetime.now().strftime("%Y-%m-%d %H:%M");backup=T+"/🗑️generated/R30-landing-"+datetime.datetime.now().strftime("%H%M%S")
    try:
        out,refused=plan()
        if refused:sys.exit("refused after taking the lock")
        apply(out,backup);print("landed; originals in",backup)
        res=[] if "--no-tests" in sys.argv else run_tests()
        with open(T+"/landings.md","a",encoding="utf8") as f:
            f.write("\n## Package R30 landing (%s): fleet read judges the machine and prints its readings, retirement plan wire bound equals the producer bound\n"%stamp)
            for rel,(how,data,raw) in out.items():f.write("- %s `%s` before %s after %s\n"%(how,short(rel),sha(raw)[:12] if raw else "-",sha(data)[:12]))
            for rel,code,failed in res:f.write("- test `LIB/%s` exit %d, %d red%s\n"%(short(rel),code,len(failed),": "+"; ".join(failed) if failed else ""))
            f.write("- originals for `land.py --rollback`: `%s`\n"%backup)
    finally:
        if not held and os.path.exists(lock+"/owner") and open(lock+"/owner").read().strip()==owner:os.remove(lock+"/owner");os.rmdir(lock)
main()
