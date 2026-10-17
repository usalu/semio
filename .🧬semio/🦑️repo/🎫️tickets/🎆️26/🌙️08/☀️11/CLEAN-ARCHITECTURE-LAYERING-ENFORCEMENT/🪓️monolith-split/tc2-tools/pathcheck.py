"""tc2: static resolution of the parent-crate paths used by the moved files: every `parent_ident::a::b::Name` must reach a pub module chain and a pub item / pub use / macro of the last module (or an enum / nested path below it)."""
import json,glob,os,re,sys,collections
sys.path.insert(0,os.path.dirname(__file__)); sys.path.insert(0,os.path.dirname(__file__)+"/ttc")
import mkcomp,modtree,unglob
from ttcstub import mask, expand_use
R="/Users/ueli/Documents/semio/"
import fixrt
ORIG={}
for l in open(fixrt.ORIG+"/index.tsv"):
    h,sha,p=l.rstrip("\n").split("\t"); ORIG.setdefault(p,fixrt.ORIG+f"/{h}.rs")
tot=0;bad=0;pre=0;new=0
for f in sorted(glob.glob(os.path.dirname(__file__)+"/specs/*.json")):
    spec=json.load(open(f)); parent=spec["parent"]; pid=mkcomp.ident(parent)
    man,mdir,lib=mkcomp.parent_info(parent)
    mods=modtree.build(lib)
    bychain={tuple(m.chain):m for m in mods if not m.test_gated}
    cache={}
    def items_of(chain):
        k=tuple(chain)
        if k in cache: return cache[k]
        if not chain:
            raw=open(lib).read(); mt=mask(raw); lo,hi=0,len(raw)
            its=[unglob.classify(a_,b_) for a,b,a_,b_ in unglob.top_level_items(raw,mt,lo,hi)]
        else:
            m=bychain.get(k)
            if m is None: cache[k]=None; return None
            its=unglob.module_items(m,mods)
            its=[x for x in its]
        out={}
        globs=[]
        for c in its:
            if not c: continue
            kind,name,vis,rt=c
            if kind=="use":
                if vis.startswith("pub"):
                    body=re.sub(r"^\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\s*\([^)]*\))?\s+)?use\s+","",rt.strip()).rstrip(";").strip()
                    for kk,segs in expand_use(body):
                        if kk=="glob": globs.append(segs)
                        else: out[segs[-1]]=("use",vis)
                    for al in re.findall(r"\bas\s+([A-Za-z_]\w*)",body): out[al]=("use",vis)
            elif name: out[name]=(kind,vis)
        cache[k]=(out,globs)
        return cache[k]
    for tf in spec["files"]:
        txt=mask(open(R+tf).read())
        for m in re.finditer(r"(?<![A-Za-z0-9_:])"+re.escape(pid)+r"::((?:[A-Za-z0-9_]+::)*[A-Za-z0-9_]+)",txt):
            segs=m.group(1).split("::")
            tot+=1
            # longest module prefix
            k=0
            while k<len(segs) and tuple(segs[:k+1]) in bychain: k+=1
            ok=True;why=""
            for d in range(1,k+1):
                if bychain[tuple(segs[:d])].vis!="pub": ok=False;why=f"module {'::'.join(segs[:d])} is {bychain[tuple(segs[:d])].vis or 'private'}"
            if k<len(segs) and ok:
                it=items_of(segs[:k])
                if it is None: ok=False;why="no module"
                else:
                    out,globs=it
                    name=segs[k]
                    if name in out:
                        if out[name][1]!="pub" and out[name][0]!="use": ok=False;why=f"item {name} is {out[name][1] or 'private'}"
                    elif globs:
                        pass
                    else:
                        ok=False;why=f"{name} not found in {'::'.join(segs[:k]) or 'crate root'}"
            if not ok:
                bad+=1
                otxt=open(ORIG.get(R+tf,R+tf)).read()
                was=("crate::"+"::".join(segs)) in otxt or ("::".join(segs)) in otxt
                if was: pre+=1; continue
                new+=1
                print("UNRESOLVED-NEW",spec["name"].replace("semio-s-artifact-",""),tf.split("subsets/")[-1][-45:],"::".join(segs),"|",why)
print("parent paths checked",tot,"unresolved",bad,"of which the same path text already in the pre-edit file",pre,"not found there",new)
