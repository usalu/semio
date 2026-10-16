"""🧩 Union differential for the flow-flow carve: every new crate becomes a module `m_<id>` of one analysis crate (sandbox only, output /dev/null).
usage: ffu.py [cfg,cfg] ; env SHOW=n ; env WORKDIR=path (default: the scratch work tree)"""
import collections,json,os,pickle,re,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import ffo
from ffo import *
CRATES={ # id -> (module suffix, root file relative to the artifact dir)
 "model":"🧱️model/🦀️.rs",
 "document":"🧾️document/🦀️.rs",
 "copy":"🧵️retained/📑️copy/🦀️.rs",
 "mutation_laws":"🌿️vcs/⚖️laws/🦀️.rs",
 "snapshot_laws":"🚪️io/⚖️laws/🦀️.rs",
}
FACADE="🦀️.rs"
NAMES={"semio_framework_artifact_flow_flow_model":"model","semio_framework_artifact_flow_flow_document":"document","semio_framework_artifact_flow_flow_copy":"copy"}
def mounts(path):
    t=open(path,encoding='utf-8').read();L=t.split('\n');out=[]
    base=os.path.dirname(path);stack=[];pending=None;depth_marks=[]
    def cur():
        d=base
        for s in stack:d=os.path.join(d,s)
        return d
    for i,l in enumerate(L):
        pm=re.match(r'^\s*#\[path\s*=\s*"([^"]+)"\]\s*$',l)
        if pm:pending=pm.group(1);continue
        inl=re.match(r'^\s*#\[path\s*=\s*"([^"]+)"\]\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;',l)
        if inl:
            f=os.path.normpath(os.path.join(cur(),inl.group(1)));out.append(f if os.path.exists(f) else None);pending=None;continue
        m=re.match(r'^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;',l)
        if m:
            d=cur()
            if pending:f=os.path.normpath(os.path.join(d,pending))
            else:
                c=[os.path.join(d,m.group(1)+'.rs'),os.path.join(d,m.group(1),'mod.rs')];f=next((x for x in c if os.path.exists(x)),None)
            out.append(f if f and os.path.exists(f) else None);pending=None;continue
        mi=re.match(r'^(\s*)(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*\{\s*$',l)
        if mi:
            stack.append(pending if pending else mi.group(2));depth_marks.append(len(mi.group(1)));pending=None;continue
        if re.match(r'^\s*\}\s*$',l) and depth_marks and len(l)-len(l.lstrip())==depth_marks[-1]:
            stack.pop();depth_marks.pop();continue
        if l.strip() and not l.strip().startswith('#['):pending=None
    return out
def owned(root):
    seen=[];stack=[root]
    while stack:
        p=stack.pop()
        if p in seen:continue
        seen.append(p)
        for f in mounts(p):
            if f:stack.append(f)
    return seen
def transform_factory(with_facade=True):
    def transform(ff):
        ff=os.path.normpath(ff)+"/"
        owner={}
        ids=dict(CRATES)
        if with_facade and os.path.exists(ff+FACADE):ids["facade"]=FACADE
        for cid,rel in ids.items():
            for f in owned(ff+rel):
                if f in owner and owner[f]!=cid:raise SystemExit(f"file owned twice: {f} {owner[f]} {cid}")
                owner[f]=cid
        sbx=os.path.realpath(ff)+os.sep
        for f in [f for f in owner if not os.path.realpath(f).startswith(sbx) or os.path.islink(f)]:owner.pop(f)
        for f,cid in owner.items():
            t=open(f,encoding='utf-8').read()
            mod="m_"+cid
            t=re.sub(r'\bpub\(crate\)',f'pub(in crate::{mod})',t)
            t=re.sub(r'(?<![\w$])(?<!::)crate::(?!m_)',f'crate::{mod}::',t)
            t=re.sub(r'\bsemio_framework_artifact_flow_flow_(model|document|copy)\b',lambda m:'crate::m_'+m.group(1),t)
            t=re.sub(r'(?<![\w:])semio_framework_artifact_flow_flow\b',"crate::m_facade",t)
            t=re.sub(r'crate::(m_\w+)::\1::',r'crate::\1::',t)
            assert os.path.realpath(f).startswith(sbx)
            open(f,'w',encoding='utf-8').write(t)
        lines=["//! union analysis root",""]
        for cid,rel in ids.items():
            lines.append(f'#[path = "../../{rel}"]\npub mod m_{cid};')
        root=ff+"📦️packages/🦀️rust/union.rs"
        os.makedirs(os.path.dirname(root),exist_ok=True)
        open(root,'w',encoding='utf-8').write("\n".join(lines)+"\n")
        return owner
    return transform
EXT=re.compile(r"(?:module or crate|import|crate|imports|unlinked crate|macro|attribute|derive macro) `(?:semio_framework_[a-z_]+|neural_engine|protocol|pack|serde|serde_json|std|core|alloc|dsl|store|neural)(?:::\w+)*`")
def clean(s):
    s=re.sub(r'\bcrate::m_\w+::','crate::',s)
    s=re.sub(r'\bm_(\w+)',r'\1',s)
    s=re.sub(r"in `(?:model|document|copy|facade)`","in the crate root",s)
    s=re.sub(r"`crate` in `(?:model|document|copy|facade)`","`crate`",s)
    s=s.replace("in `crate`","in the crate root")
    return s
def post(key,file):
    code,msg,txt,lab=key
    if EXT.search(msg) or EXT.search(lab):return None
    return (code,clean(msg),clean(txt),clean(lab))
def compare(workdir,cfgs=None,show=80,facade=True):
    root=sandbox("cur",workdir,transform_factory(facade))
    base=pickle.load(open(SCR+"base.pkl","rb"))
    libp=root+"/"+FF+"/📦️packages/🦀️rust/union.rs"
    bad=0
    for tag,extra in CFGS.items():
        if cfgs and tag not in cfgs:continue
        err=run(libp,extra)
        open(SCR+f"err-cur-{tag}.json","w").write(err)
        cur=norm(err,post=post)
        b=norm(open(SCR+f"err-orig-{tag}.json").read(),post=post)
        a=cur-b;c=b-cur
        print(f"== {tag}: baseline {sum(b.values())} split {sum(cur.values())} only-split {sum(a.values())} only-baseline {sum(c.values())}",flush=True)
        for k,v in sorted(a.items())[:show]:print("  +split",v,k[0],k[1][:150],"|",k[2][:110],"|",k[3][:60])
        for k,v in sorted(c.items())[:show]:print("  -base ",v,k[0],k[1][:150],"|",k[2][:110],"|",k[3][:60])
        bad+=sum(a.values())+sum(c.values())
    print("TOTAL DIFF",bad)
    return bad
if __name__=="__main__":
    cfgs=None
    for a in sys.argv[1:]:
        if not a.startswith("--"):cfgs=a.split(",")
    compare(os.environ.get("WORKDIR",SCR+"work"),cfgs,int(os.environ.get("SHOW","80")),facade="--nofacade" not in sys.argv)
