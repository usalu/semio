"""🔬 Patched full-resolution differential for the OS-HOST-NEURAL-DAG carve: baseline sandbox (snapshot of the original) versus the split tree with every new crate simulated as a module of the parent (union), analysis only."""
import collections,json,os,pickle,re,subprocess,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from common import *
TC="+nightly-2026-07-20"
def load_cfg(name):
    return json.load(open(TOOLS+f"/cfg-{name}.json",encoding="utf-8"))
def child_files(cfg,child):
    seen={}
    for extra in cfg.get("closureCfgs",["","test","target_arch=wasm32,target_os=unknown,target_family=wasm"]):
        for row in closure(child["root"],child["pkg"],extra):
            if row[1]=="source" and row[0].endswith(".rs") and not row[0].startswith("/"):seen[row[0]]=1
    return list(seen)
def union_transform(cfg,root):
    kids=cfg.get("children",[])
    idents=[k["ident"] for k in kids]
    parent_lib=os.path.join(root,cfg["lib"])
    owned={}
    for k in kids:
        for f in child_files(cfg,k):owned[f]=k["ident"]
    ident_rx=re.compile(r'(?<![\w:$])(?:::)?(%s)\b(?!\s*=)'%'|'.join(re.escape(i) for i in idents)) if idents else None
    def rewrite(text,ident):
        if ident:
            text=re.sub(r'(?<![\w$])crate::',f'crate::{ident}::',text)
            text=re.sub(r'pub\s*\(\s*crate\s*\)',f'pub(in crate::{ident})',text)
            text=re.sub(r'(?m)^\s*#!\[[^\n]*\]\s*$','',text) if False else text
        if ident_rx:
            text=ident_rx.sub(lambda m:f'crate::{m.group(1)}',text)
        return text
    # rewrite every real (copied) .rs file under the real dirs
    for real in cfg["real"]:
        base=os.path.join(root,real)
        for dp,dn,fn in os.walk(base):
            for f in fn:
                if not f.endswith(".rs"):continue
                p=os.path.join(dp,f);rel=os.path.relpath(p,root)
                t=open(p,encoding="utf-8",errors="ignore").read()
                ident=owned.get(rel)
                n=rewrite(t,ident)
                if ident and rel==[k["root"] for k in kids if k["ident"]==ident][0]:
                    n=re.sub(r'(?m)^#!\[[^\n]*\]\s*$','',n)
                if n!=t:open(p,"w",encoding="utf-8").write(n)
    inner=[]
    mounts=[]
    for k in kids:
        rootp=os.path.join(root,k["root"])
        for m in re.finditer(r'(?m)^#!\[[^\n]*\]\s*$',open(os.path.join(REPO,k["root"]),encoding="utf-8").read() if not cfg.get("_snap") else ""):
            inner.append(m.group(0))
        rel=os.path.relpath(rootp,os.path.dirname(parent_lib))
        mounts.append(f'#[path = "{rel}"]\npub mod {k["ident"]};\n')
    t=open(parent_lib,encoding="utf-8").read()
    for line in inner:
        if line not in t:t=line+"\n"+t
    t+= "\n"+"".join(mounts)
    open(parent_lib,"w",encoding="utf-8").write(t)
def sandbox(cfg,which):
    root=SC+f"osnd/sbx-{cfg['name']}-{which}"
    real={}
    for r in cfg["real"]:
        real[r]=(SNAP+cfg["snap"][r]) if which=="orig" else REPO+r
    mk(root,real)
    if which=="split":union_transform(cfg,root)
    files=[]
    for r in cfg["real"]:files+=rsfiles(os.path.join(root,r))
    n=neutralize(files,modnames(files))
    if which=="split" and os.environ.get("NEG"):
        for pair in os.environ["NEG"].split("||"):
            old,new=pair.split("=>")
            done=False
            for f in files:
                s=open(f,encoding="utf-8").read()
                if old in s:
                    open(f,"w",encoding="utf-8").write(s.replace(old,new,1));print("NEG patched",f[-70:]);done=True;break
            assert done,old
    return root,n
def run(cfg,root,extra):
    lib=os.path.join(root,cfg["lib"])
    p=subprocess.run(["rustc",TC,"--edition","2021","--crate-type","lib","--crate-name",cfg["crateName"],"--emit=metadata","-o","/dev/null","--error-format=json","--json=diagnostic-short",*extra,lib],capture_output=True,text=True,cwd=os.path.dirname(lib))
    return p.stderr
def norm(cfg,root,stderr):
    out=collections.Counter();ext=collections.Counter()
    idents=[k["ident"] for k in cfg.get("children",[])]
    for ln in stderr.splitlines():
        if not ln.startswith("{"):continue
        try:d=json.loads(ln)
        except Exception:continue
        if "message" not in d or "level" not in d or d["level"] not in("error","warning"):continue
        spans=[s for s in d.get("spans",[]) if s.get("is_primary")]
        txt="";lab=""
        if spans:
            s=spans[0]
            txt=" ".join(" ".join(t["text"] for t in s.get("text",[])).split())
            lab=s.get("label") or ""
        code=(d.get("code") or {}).get("code") or ""
        msg=re.sub(r'\s+',' ',d["message"])
        for k in cfg.get("children",[]):
            i=k["ident"];al=k.get("alias","CHILD")
            msg=msg.replace(i,"CHILD");txt=txt.replace(i,"CHILD");lab=lab.replace(i,"CHILD")
            txt=re.sub(r'pub\(in crate::CHILD\)','pub(crate)',txt)
            txt=txt.replace("crate::CHILD::","crate::").replace("crate::CHILD","crate")
            msg=msg.replace("crate::CHILD::","crate::").replace("crate::CHILD","crate")
            lab=lab.replace("`CHILD`","`%s`"%al).replace(" CHILD"," "+al)
            msg=msg.replace("`CHILD`","`%s`"%al)
        if re.match(r'aborting due to \d+ previous error',msg):continue
        if msg.startswith("aborting due to"):continue
        for rx,rep in cfg.get("textNormalize",[]):
            txt=re.sub(rx,rep,txt);msg=re.sub(rx,rep,msg)
        if "unresolved module or unlinked crate" in lab or "no external crate" in lab or "list of imported crates" in lab or "unresolved module or unlinked crate" in msg:
            m=re.search(r"`([A-Za-z_][\w]*)`",msg)
            ext[(m.group(1) if m else msg[:40])]+=1;continue
        out[(d["level"],code,msg,txt,lab)]+=1
    return out,ext
def main():
    name=sys.argv[1];mode=sys.argv[2];only=sys.argv[3:]
    cfg=load_cfg(name)
    os.makedirs(SC+"osnd",exist_ok=True)
    pk=SC+f"osnd/base-{name}.pkl"
    if mode=="base":
        root,n=sandbox(cfg,"orig");print("neutralized",n)
        base={}
        for tag,extra in cfg["cfgs"].items():
            if only and tag not in only:continue
            err=run(cfg,root,extra);open(SC+f"osnd/{name}-{tag}-orig.json","w").write(err)
            base[tag]=norm(cfg,root,err);print(tag,"errors",sum(v for k,v in base[tag][0].items() if k[0]=="error")+sum(base[tag][1].values()),"internal",sum(v for k,v in base[tag][0].items() if k[0]=="error"),"warnings",sum(v for k,v in base[tag][0].items() if k[0]=="warning"))
        old=pickle.load(open(pk,"rb")) if os.path.exists(pk) else {}
        old.update(base);pickle.dump(old,open(pk,"wb"))
    else:
        base=pickle.load(open(pk,"rb"))
        root,n=sandbox(cfg,"split");print("neutralized",n)
        bad=0
        for tag,extra in cfg["cfgs"].items():
            if only and tag not in only:continue
            err=run(cfg,root,extra);open(SC+f"osnd/{name}-{tag}-split.json","w").write(err)
            (c,ec),(b,eb_)=norm(cfg,root,err),base[tag]
            os_=c-b;ob=b-c
            ext_delta={k:(eb_.get(k,0),ec.get(k,0)) for k in set(ec)|set(eb_) if ec.get(k,0)!=eb_.get(k,0)}
            ea=sum(v for k,v in b.items() if k[0]=="error");eb=sum(v for k,v in c.items() if k[0]=="error")
            wa=sum(v for k,v in b.items() if k[0]=="warning");wb=sum(v for k,v in c.items() if k[0]=="warning")
            print(f"== {name}/{tag}: baseline errors={ea} warnings={wa} | split errors={eb} warnings={wb} | only-split={sum(os_.values())} only-baseline={sum(ob.values())}")
            for k,v in sorted(os_.items())[:int(os.environ.get('MAXROWS','60'))]:print("  +split",v,k[0],k[1],k[2][:130],"|",k[3][:90],"|",k[4][:50])
            for k,v in sorted(ob.items())[:int(os.environ.get('MAXROWS','60'))]:print("  -base ",v,k[0],k[1],k[2][:130],"|",k[3][:90],"|",k[4][:50])
            if ext_delta:print("  external-missing-crate deltas (baseline->split):",dict(sorted(ext_delta.items())))
            bad+=sum(os_.values())+sum(ob.values())
        print("TOTAL DIFF",bad)
main()
