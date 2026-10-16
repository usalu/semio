"""🧩 Union sandbox transform (U2): every new crate becomes a module `m_<id>` of one analysis crate; analysis only, never touches the repo."""
import os,re,sys,json
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from sandbox import *
from rlex2 import lex2
FLR=OS+"🌊️flow/"
def load_registry():
    from build_crates import CR
    return CR
def mounts(p):
    """all `mod x;` file mounts of file p at any depth, with #[path] resolved -> list of files"""
    t=open(p,encoding='utf-8').read();L=t.split('\n');out=[]
    for i,l in enumerate(L):
        inl=re.match(r'^\s*#\[path\s*=\s*"([^"]+)"\]\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;',l)
        if inl:
            f=os.path.normpath(os.path.join(os.path.dirname(p),inl.group(1)))
            out.append(f if os.path.exists(f) else None);continue
        m=re.match(r'^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;',l)
        if not m:continue
        path=None
        for k in range(i-1,max(i-8,-1),-1):
            pm=re.match(r'^\s*#\[path\s*=\s*"([^"]+)"\]',L[k])
            if pm:path=pm.group(1);break
            if L[k].strip().startswith('#[') or not L[k].strip():continue
            break
        d=os.path.dirname(p)
        if path:f=os.path.normpath(os.path.join(d,path))
        else:
            c=[os.path.join(d,m.group(1)+'.rs'),os.path.join(d,m.group(1),'mod.rs')]
            f=next((x for x in c if os.path.exists(x)),None)
        if f and os.path.exists(f):out.append(f)
        else:out.append(None)
    return out
def owned_files(root):
    seen=[];stack=[root]
    while stack:
        p=stack.pop()
        if p in seen:continue
        seen.append(p)
        for f in mounts(p):
            if f:stack.append(f)
    return seen
EXC=("history_identity","flow_result")
def transform_factory(crates):
    """crates: dict id -> abs root file inside the SANDBOX flow tree (relative given as rel to FL)"""
    def transform(root):
        fl=root+"/"+FLR
        fl=os.path.normpath(fl)+"/"
        owner={}
        for cid,rel in crates.items():
            for f in owned_files(fl+rel):
                if f in owner and owner[f]!=cid:raise SystemExit(f"file owned twice: {f} {owner[f]} {cid}")
                owner[f]=cid
        sbx=os.path.realpath(root)+os.sep
        for f in [f for f in owner if not os.path.realpath(f).startswith(sbx) or os.path.islink(f)]:
            owner.pop(f)
        for f,cid in owner.items():
            t=open(f,encoding='utf-8').read()
            mod="m_"+cid.replace("-","_")
            t=re.sub(r'\bpub\(crate\)',f'pub(in crate::{mod})',t)
            t=re.sub(r'(?<![\w$])(?<!::)crate::(?!m_)',f'crate::{mod}::',t)
            t=re.sub(r'\bsemio_framework_os_flow_(\w+)\b',lambda m:'crate::m_'+m.group(1),t)
            t=re.sub(r'crate::m_\w+::(history_identity|flow_result)\b',r'crate::\1',t)
            t=re.sub(r'crate::(m_\w+)::\1::',r'crate::\1::',t)
            open(f,'w',encoding='utf-8').write(t)
        rootfile=fl+"📦️packages/🦀️rust/🦀️.rs"
        lines=["//! union analysis root",""]
        for cid,rel in crates.items():
            gate='#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]\n' if cid.startswith("wasm") else ''
            lines.append(f'{gate}#[path = "../../{rel}"]\npub mod m_{cid.replace("-","_")};')
        open(rootfile,'w',encoding='utf-8').write("\n".join(lines)+"\n")
        return owner
    return transform
def crates_for_union():
    CR=load_registry()
    return {cid:(c.node+"/"+c.lib_rel) for cid,c in CR.items()}
if __name__=="__main__":
    print(json.dumps(crates_for_union(),ensure_ascii=False,indent=1))
