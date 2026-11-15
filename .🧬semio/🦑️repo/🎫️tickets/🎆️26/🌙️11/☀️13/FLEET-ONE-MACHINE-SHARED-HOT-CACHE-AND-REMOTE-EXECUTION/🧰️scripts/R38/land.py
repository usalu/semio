# -*- coding: utf-8 -*-
"""R38 landing: the kernel limit of a slice load never lies below the plan guard ceiling, the service cgroup takes its limit from the machine, the typed graph memory refusal is a bilingual refusal. usage: land.py --dry-run | --land [--lock-held] | --rollback <dir> ; --root <dir> for a rehearsal root."""
import os,sys,subprocess,hashlib,time,shutil,datetime,tempfile
HERE=os.path.dirname(os.path.abspath(__file__));P=HERE+"/payload"
def arg(n):return sys.argv[sys.argv.index(n)+1] if n in sys.argv else None
R=os.path.abspath(arg("--root")) if arg("--root") else "/Users/Shared/semio"
T="/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
def rd(p):
    with open(p,"rb") as f:return f.read()
def sha(b):return hashlib.sha256(b).hexdigest()[:12]
def files():return [l.split("\t") for l in open(P+"/files.txt",encoding="utf8").read().split("\n") if l]
def merge(n,rel):
    new=rd(P+"/"+n+".new");live=R+"/"+rel;base=P+"/"+n+".base"
    if not os.path.exists(live):
        return ("new",new,b"") if not os.path.exists(base) else ("refused","live file is gone",b"")
    cur=rd(live)
    if cur==new:return ("same",new,cur)
    if not os.path.exists(base):return ("refused","new file exists with other content",cur)
    if cur==rd(base):return ("edit",new,cur)
    with tempfile.TemporaryDirectory() as d:
        open(d+"/live","wb").write(cur);r=subprocess.run(["git","merge-file","-p","-L","live","-L","base","-L","new",d+"/live",base,P+"/"+n+".new"],capture_output=True)
        if r.returncode==0:return ("merge",r.stdout,cur)
        return ("refused","conflict with a concurrent edit (%d hunks)"%r.returncode,cur)
def parse_proof(rel,data):
    if rel.endswith(".json"):
        import json;json.loads(data.decode("utf8"));return None
    ext=rel.rsplit(".",1)[1];loader={"cjs":"js","mjs":"js","ts":"ts","mts":"ts"}.get(ext)
    if loader is None:return None
    code="const t=new Bun.Transpiler({loader:%r});t.transformSync(await Bun.file(process.argv[1]).text())"%loader
    with tempfile.NamedTemporaryFile(suffix="."+ext,delete=False) as f:f.write(data)
    try:
        r=subprocess.run([BUN,"-e",code,f.name],capture_output=True,text=True);return None if r.returncode==0 else r.stderr[:300]
    finally:os.remove(f.name)
def plan():
    out={};refused=[]
    for n,rel in files():
        how,data,raw=merge(n,rel)
        if how=="refused":refused.append((rel,data));continue
        err=parse_proof(rel,data)
        if err:refused.append((rel,"parse proof failed: "+err));continue
        out[rel]=(how,data,raw)
    return out,refused
def apply(out,backup):
    for rel,(how,data,raw) in out.items():
        if how=="same":continue
        p=R+"/"+rel;os.makedirs(os.path.dirname(backup+"/"+rel),exist_ok=True) if False else None
    ix=[]
    for rel,(how,data,raw) in out.items():
        if how=="same":continue
        p=R+"/"+rel;key="%02d"%len(ix);ix.append(key+"\t"+how+"\t"+rel)
        if how=="new":open(backup+"/"+key+".absent","w").close()
        else:shutil.copyfile(p,backup+"/"+key+".orig")
        os.makedirs(os.path.dirname(p),exist_ok=True);open(p+".r38tmp","wb").write(data)
    open(backup+"/index.txt","w",encoding="utf8").write("\n".join(ix)+"\n")
    for rel,(how,data,raw) in out.items():
        if how!="same":os.replace(R+"/"+rel+".r38tmp",R+"/"+rel)
def rollback(backup):
    for line in open(backup+"/index.txt",encoding="utf8").read().split("\n"):
        if not line:continue
        key,how,rel=line.split("\t");p=R+"/"+rel
        if how=="new":
            if os.path.exists(p):os.remove(p)
        else:shutil.copyfile(backup+"/"+key+".orig",p)
    print("rolled back")
if "--rollback" in sys.argv:rollback(arg("--rollback"));sys.exit(0)
out,refused=plan()
for rel,(how,data,raw) in out.items():print("%-6s %s before %s after %s"%(how,rel[-90:],sha(raw) if raw else "-",sha(data)))
for rel,why in refused:print("REFUSED",rel[-90:],why)
if refused:sys.exit(2)
if "--dry-run" in sys.argv:print("dry run clean");sys.exit(0)
if "--land" in sys.argv:
    lock=T+"/landing.lock"
    if "--lock-held" not in sys.argv:
        os.mkdir(lock);open(lock+"/owner","w").write("R38 "+datetime.datetime.utcnow().isoformat()+"\n")
    backup=T+"/🗑️generated/R38-landing-"+time.strftime("%H%M%S");os.makedirs(backup)
    apply(out,backup);print("landed; originals in",backup)
