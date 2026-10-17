"""tc2: names a moved file imports from the parent crate (`parent::[a::b::]Name`): is the name reachable from outside (pub item, pub use, pub glob chain, enum variant path below a pub enum)?"""
import json,glob,os,re,sys,collections
sys.path.insert(0,os.path.dirname(__file__)); sys.path.insert(0,os.path.dirname(__file__)+"/ttc")
import mkcomp,modtree,unglob
from ttcstub import mask, expand_use
R="/Users/ueli/Documents/semio/"
tot=0;bad=0;rows=[]
for f in sorted(glob.glob(os.path.dirname(__file__)+"/specs/*.json")):
    spec=json.load(open(f)); parent=spec["parent"]; pid=mkcomp.ident(parent)
    man,mdir,lib=mkcomp.parent_info(parent)
    mods=modtree.build(lib)
    bychain={tuple(m.chain):m for m in mods if not m.test_gated}
    cache={}
    def mod_items(chain):
        k=tuple(chain)
        if k in cache: return cache[k]
        cache[k]=None
        if not chain:
            raw=open(lib).read(); mt=mask(raw)
            its=[unglob.classify(rt,mt_) for a,b,rt,mt_ in unglob.top_level_items(raw,mt,0,len(raw))]
        else:
            m=bychain.get(k)
            if m is None: return None
            its=unglob.module_items(m,mods)
        direct={}; pubg=[]; privg=[]
        for c in its:
            if not c: continue
            kind,name,vis,rt=c
            if kind=="use":
                body=re.sub(r"^\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\s*\([^)]*\))?\s+)?use\s+","",rt.strip()).rstrip(";").strip()
                ispub=bool(re.match(r"\s*(?:#\[[^\]]*\]\s*)*pub\s+use\b",rt))
                for kk,segs in expand_use(body):
                    if kk=="glob":
                        (pubg if ispub else privg).append(segs)
                    else:
                        nm=segs[-1]
                        if ispub: direct[nm]=("use","pub")
                        else: direct.setdefault(nm,("use","private"))
                for al in re.findall(r"\bas\s+([A-Za-z_]\w*)",body):
                    if ispub: direct[al]=("use","pub")
            elif name:
                direct[name]=(kind,vis or "private")
        if not chain:
            text=mask(open(lib).read())
        else:
            m=bychain.get(k)
            text=mask(open(m.file).read()) if m is not None else ""
        cache[k]=(direct,pubg,privg,text)
        return cache[k]
    def resolve_glob(segs,cur):
        if segs and segs[0]=="crate": return segs[1:]
        if segs and segs[0]=="self": return list(cur)+segs[1:]
        if segs and segs[0]=="super": return list(cur[:-1])+segs[1:]
        if tuple(list(cur)+segs[:1]) in bychain: return list(cur)+segs
        return None
    def find(chain,name,seen=None):
        seen=seen or set()
        if (tuple(chain),name) in seen: return None
        seen.add((tuple(chain),name))
        it=mod_items(chain)
        if it is None: return None
        direct,pubg,privg,text=it
        if name in direct: return direct[name]
        if re.search(r"(?<![A-Za-z0-9_])pub(?:\([^)]*\))?\s+(?:struct|enum|fn|const|static|type|trait|mod|union)\s+"+re.escape(name)+r"\b",text): return ("macro-or-nested","pub")
        for g in pubg:
            tgt=resolve_glob(g,chain)
            if tgt is not None:
                r=find(tgt,name,seen)
                if r and r[1]=="pub": return r
        for g in privg:
            tgt=resolve_glob(g,chain)
            if tgt is not None:
                r=find(tgt,name,seen)
                if r: return ("privglob","private")
        return None
    for tf in spec["files"]:
        t=mask(open(R+tf).read())
        paths=[]
        for m in re.finditer(r"(?<![A-Za-z0-9_:])"+re.escape(pid)+r"::((?:[A-Za-z0-9_]+::)*[A-Za-z0-9_]+)",t):
            paths.append(m.group(1).split("::"))
        for m in re.finditer(r"\buse\s+("+re.escape(pid)+r"::\{[^;]*\})\s*;",t):
            for kk,sg in expand_use(m.group(1)):
                if kk!="glob" and len(sg)>1: paths.append(sg[1:])
        for segs in paths:
            tot+=1
            k=0
            while k<len(segs) and tuple(segs[:k+1]) in bychain: k+=1
            ok=True;why=""
            for d in range(1,k+1):
                if bychain[tuple(segs[:d])].vis!="pub": ok=False;why=f"module {'::'.join(segs[:d])} is {bychain[tuple(segs[:d])].vis or 'private'}"
            if ok and k<len(segs):
                r=find(segs[:k],segs[k])
                if r is None: ok=False;why=f"{segs[k]} not found in {'::'.join(segs[:k]) or 'crate root'}"
                elif r[1]!="pub": ok=False;why=f"{segs[k]} is {r[1]} ({r[0]}) in {'::'.join(segs[:k]) or 'crate root'}"
            if not ok:
                bad+=1; rows.append((spec["name"][17:],tf.split("subsets/")[-1][-40:],"::".join(segs)[:70],why))
c=collections.Counter(rows)
for r,v in sorted(c.items()): print(v,*r)
print("parent paths checked",tot,"unreachable-or-unresolved",bad)
