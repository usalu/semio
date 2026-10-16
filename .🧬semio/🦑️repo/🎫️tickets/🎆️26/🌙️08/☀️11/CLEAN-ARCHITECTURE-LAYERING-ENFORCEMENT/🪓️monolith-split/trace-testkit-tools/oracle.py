import json,os,re,shutil,subprocess,sys,collections
from concurrent.futures import ThreadPoolExecutor
R="/Users/ueli/Documents/semio"
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad"
here=os.path.dirname(os.path.abspath(__file__))
units=json.load(open(here+"/units.json"))
HELPER="🧰️framework/🔨️modules/⏱️trace/🧮️memory/🧪️testing/📥️requests"
TOOLCHAIN="+nightly-2026-07-20"
def snap_new(i,u):
    p=subprocess.run(["bun",here+"/closure.ts",R+"/"+u["entry"],R+"/"+u["pkg"],"list","test"],capture_output=True,text=True,cwd=R)
    if p.returncode!=0: return None
    files=[l.split("\t")[0] for l in p.stdout.splitlines() if l and not l.startswith("TOTAL")]
    dst=f"{S}/sb/new/{i:02d}"
    shutil.rmtree(dst,ignore_errors=True)
    for f in files:
        d=os.path.join(dst,f); os.makedirs(os.path.dirname(d),exist_ok=True); shutil.copyfile(os.path.join(R,f),d)
    for rel in ["🦀️.rs","🧪️tests/🦀️.rs","🧪️tests/🧫️fixtures/🔣️.json"]:
        d=os.path.join(dst,HELPER,rel); os.makedirs(os.path.dirname(d),exist_ok=True); shutil.copyfile(os.path.join(R,HELPER,rel),d)
    return dst,files
def patch_new(dst,u,files):
    helper_rel=os.path.relpath(os.path.join(dst,HELPER,"🦀️.rs"),os.path.dirname(os.path.join(dst,u["entry"])))
    for f in files:
        if not f.endswith(".rs"): continue
        p=os.path.join(dst,f); s=open(p,encoding="utf8").read()
        t=s.replace("use semio_framework_trace_memory_testing as","use crate::semio_framework_trace_memory_testing as")
        if t!=s: open(p,"w",encoding="utf8").write(t)
    root=os.path.join(dst,u["entry"]); s=open(root,encoding="utf8").read()
    s+=f'\n#[cfg(test)]\n#[path="{helper_rel}"]\npub mod semio_framework_trace_memory_testing;\n'
    open(root,"w",encoding="utf8").write(s)
def patch_base(dst,files):
    for f in files:
        if f.endswith("📥️requests/🦀️.rs"):
            p=os.path.join(dst,f); s=open(p,encoding="utf8").read()
            open(p,"w",encoding="utf8").write(s.replace("pub(in crate::m_geometry)","pub(crate)"))
def run(dst,u,flags):
    root=os.path.join(dst,u["entry"])
    crate=re.sub(r"\W","_",u["name"])
    p=subprocess.run(["rustc",TOOLCHAIN,"--edition","2021","--crate-type","lib","--crate-name",crate,"--emit=metadata","-o","/dev/null",*flags,root],capture_output=True,text=True,cwd=dst)
    return p.stderr
def summarize(err,dst):
    rows=collections.Counter()
    for blk in re.split(r"\n(?=error)",err):
        if not blk.startswith("error"): continue
        head=blk.splitlines()[0]
        m=re.search(r"--> (\S+?):\d+:\d+",blk)
        path=os.path.normpath(m.group(1).replace(dst+"/","")) if m else ""
        head=re.sub(r"`[^`]*📥️requests[^`]*`","`HELPER`",head)
        rows[(head,path)]+=1
    return rows
def one(i):
    u=units[i]
    d0=f"{S}/sb/base/{i:02d}"
    if not os.path.exists(d0+"/_unit.json"): return i,"skip",None
    files=json.load(open(d0+"/_unit.json"))["files"]
    patch_base(d0,files)
    sn=snap_new(i,u)
    if sn is None: return i,"closure-error",None
    dst,nf=sn
    patch_new(dst,u,nf)
    out={}
    for label,flags in [("test",["--test"])]:
        eb=run(d0,u,flags); en=run(dst,u,flags)
        open(f"{S}/sb/log-{i:02d}-base.txt","w").write(eb); open(f"{S}/sb/log-{i:02d}-new.txt","w").write(en)
        rb,rn=summarize(eb,d0),summarize(en,dst)
        only_new=rn-rb; only_base=rb-rn
        out[label]=dict(base=sum(rb.values()),new=sum(rn.values()),only_new=[(k,v) for k,v in only_new.items()],only_base=[(k,v) for k,v in only_base.items()])
    return i,"ok",out
if __name__=="__main__":
    only=set(map(int,sys.argv[1].split(","))) if len(sys.argv)>1 else set(range(len(units)))
    with ThreadPoolExecutor(4) as ex:
        for i,st,out in ex.map(one,sorted(only)):
            u=units[i]
            print(f"[{i:02d}] {u['name']} {os.path.basename(os.path.dirname(u['entry']))} {st}",flush=True)
            if out:
                for label,r in out.items():
                    print(f"    {label}: base errors {r['base']}, new errors {r['new']}, only-new {sum(v for _,v in r['only_new'])}, only-base {sum(v for _,v in r['only_base'])}")
                    for k,v in r["only_new"][:8]: print("      +",v,k)
                    for k,v in r["only_base"][:8]: print("      -",v,k)
