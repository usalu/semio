"""tc2: duplicate top-level `use` bindings (E0252) inside the moved test files."""
import json,glob,os,re,sys,collections
sys.path.insert(0,os.path.dirname(__file__)); sys.path.insert(0,os.path.dirname(__file__)+"/ttc")
import unglob
from ttcstub import mask, expand_use
R="/Users/ueli/Documents/semio/"
bad=0;tot=0
for f in sorted(glob.glob(os.path.dirname(__file__)+"/specs/*.json")):
    s=json.load(open(f))
    for tf in s["files"]:
        raw=open(R+tf).read(); m=mask(raw)
        binds=collections.defaultdict(list)
        defs=set()
        for a,b,rt,mt in unglob.top_level_items(raw,m,0,len(raw)):
            c=unglob.classify(rt,mt)
            if not c: continue
            if c[0]=="use":
                body=re.sub(r"^\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\s*\([^)]*\))?\s+)?use\s+","",rt.strip()).rstrip(";").strip()
                cfg=bool(re.match(r"\s*#\[\s*cfg",rt))
                for kind,segs in expand_use(body):
                    if kind=="glob": continue
                    nm=segs[-1]
                    am=re.search(r"\bas\s+(\w+)\s*$",body) if len(segs)==1 and False else None
                    binds[nm].append((cfg,"::".join(segs)))
                for al in re.findall(r"\bas\s+([A-Za-z_]\w*)",body):
                    binds[al].append((cfg,"alias"))
            elif c[1]: defs.add(c[1])
        for nm,v in binds.items():
            tot+=1
            paths={p for c,p in v if not c}
            if (len(v)>1 and len({p for c,p in v})>1 and nm!="_") or nm in defs:
                bad+=1; print("DUP",tf.split("subsets/")[-1][-50:],nm,v[:3])
print("bindings",tot,"conflicts",bad)
