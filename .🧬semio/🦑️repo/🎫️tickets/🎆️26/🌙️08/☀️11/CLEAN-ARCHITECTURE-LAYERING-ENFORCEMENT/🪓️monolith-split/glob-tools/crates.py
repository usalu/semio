import sys,os,re,pickle
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from rs import *
def parse_toml(text):
    d={}; cur=d; 
    for raw in text.split("\n"):
        l=raw.strip()
        if not l or l.startswith("#"): continue
        m=re.match(r'^\[\[([^\]]+)\]\]$',l)
        if m:
            cur={}; d.setdefault(m.group(1).strip(),[]).append(cur); continue
        m=re.match(r'^\[([^\]]+)\]$',l)
        if m:
            cur=d
            for part in [x.strip().strip('"') for x in re.split(r'\.(?![^"]*")',m.group(1))]:
                cur=cur.setdefault(part,{})
            continue
        m=re.match(r'^("?[\w.\-]+"?)\s*=\s*(.*)$',l)
        if m:
            k=m.group(1).strip('"'); v=m.group(2).strip()
            if v.startswith('"'):
                mm=re.match(r'"([^"]*)"',v); cur[k]=mm.group(1) if mm else v
            elif v.startswith('{'):
                inl={}
                for kk,vv in re.findall(r'(\w[\w\-]*)\s*=\s*"([^"]*)"',v): inl[kk]=vv
                cur[k]=inl
            else: cur[k]=v
    return d
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad"
def manifests():
    return [l.strip() for l in open(S+"/cargos.txt") if l.strip()]
def load():
    res=[]
    for m in manifests():
        p=os.path.join(R,m)
        try: d=parse_toml(open(p,encoding='utf-8').read())
        except Exception as e: continue
        res.append((p,d))
    return res
def targets(p,d):
    base=os.path.dirname(p); out=[]
    pk=d.get('package',{}); name=pk.get('name')
    if not name: return out
    lib=d.get('lib',{})
    lp=lib.get('path') or ('src/lib.rs' if os.path.exists(os.path.join(base,'src/lib.rs')) else None)
    if lp:
        out.append(('lib',lib.get('name') or name.replace('-','_'),os.path.normpath(os.path.join(base,lp)),name))
    for kind in ('test','bin','example','bench'):
        for t in d.get(kind,[]):
            if t.get('path'): out.append((kind,t.get('name'),os.path.normpath(os.path.join(base,t['path'])),name))
    return out
if __name__=="__main__":
    ms=load()
    tg=[]
    for p,d in ms: tg+= [(p,)+x for x in targets(p,d)]
    deps={}
    for p,d in ms:
        pk=d.get('package',{}).get('name')
        if not pk: continue
        s=set()
        for sec in ('dependencies','dev-dependencies','build-dependencies'):
            for k,v in d.get(sec,{}).items():
                s.add(v.get('package',k) if isinstance(v,dict) else k)
        deps[pk]=s
    pickle.dump((tg,deps),open(S+"/crates.pkl","wb"))
    print(len(ms),len(tg),sys.version)
