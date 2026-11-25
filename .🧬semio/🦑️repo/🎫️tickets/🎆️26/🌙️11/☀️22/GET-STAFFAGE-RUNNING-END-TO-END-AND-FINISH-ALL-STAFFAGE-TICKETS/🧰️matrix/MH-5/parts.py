"""🔎️ What draws and what models one bone: parts.py <id|file.json> <scale> <bone>… prints per view the shapes of the bone (kind, solid, role, z, ranges) and the body solids of the band on it (kind, weights, transform)."""
import json,sys,glob,os
p=sys.argv[1]
if not os.path.exists(p): p=glob.glob(f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/{p}/🔣️.json")[0]
a=json.load(open(p))["asset"]; s=sys.argv[2]; bones=sys.argv[3:]
r=a["representations"][s]; v=a.get("volumes",{}).get(s,{})
if not bones: bones=[b["id"] for b in a["rig"]["bones"]]
def pts(o):
    if isinstance(o,list):
        if len(o)>=2 and all(isinstance(x,(int,float)) for x in o): yield o[:2]
        else:
            for x in o: yield from pts(x)
    elif isinstance(o,dict):
        for x in o.values(): yield from pts(x)
for b in bones:
    B=next(x for x in a["rig"]["bones"] if x["id"]==b)
    print("==",b,{k:B[k] for k in B if k not in("id","joint")})
    for w in("elevation","front","plan"):
        for sh in r["geometry"][w]:
            if sh.get("bone")!=b: continue
            q=list(pts(sh.get("geometry",sh.get("points",sh.get("rings",[]))))) 
            rng=f"a[{min(x[0] for x in q):.3f},{max(x[0] for x in q):.3f}] b[{min(x[1] for x in q):.3f},{max(x[1] for x in q):.3f}] n{len(q)}" if q else ""
            print("  ",w,sh["id"],sh["kind"],json.dumps(sh.get("solid")),sh.get("role"),"z",sh.get("z"),"tip" if sh.get("followTip") else "",rng, [k for k in sh if k not in("id","kind","solid","role","z","bone","geometry","points","rings")])
    for d in v.get("body",[]):
        if d.get("bone")==b: print("   VOL",d["id"],d["kind"],"weights" if d.get("weights") else "",json.dumps({k:x for k,x in d.items() if k not in("id","kind","bone","sections","profile","material","weights","tone")})[:260], "sections %d"%len(d["sections"]) if "sections" in d else "")
