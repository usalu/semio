import json,sys,math
base=r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🪑️objects"
a=sys.argv[1]; scale=sys.argv[2] if len(sys.argv)>2 else None
d=json.load(open(f"{base}\{a}\🔣️.json",encoding="utf-8"))["asset"]
bones={b["id"]:b for b in d["rig"]["bones"]}
W={}
def world(i):
    if i in W: return W[i]
    b=bones[i]
    if b["parent"] is None: W[i]=(b["x"],b["y"],b["angleDeg"]); return W[i]
    px,py,pa=world(b["parent"]); r=math.radians(pa)
    W[i]=(px+b["x"]*math.cos(r)-b["y"]*math.sin(r),py+b["x"]*math.sin(r)+b["y"]*math.cos(r),pa+b["angleDeg"]); return W[i]
print("dims",d["dimensions"])
for i,b in bones.items():
    x,y,an=world(i); print(f"bone {i} parent={b['parent']} world=({x:.3f},{y:.3f}) ang={an:.1f} len={b.get('lengthM')} joint={b.get('joint')}")
print("regions",[r.get("id") for r in d.get("regions",[])] if isinstance(d.get("regions"),list) else list(d.get("regions",{}))[:30])
for p in d["poses"]: print("pose",p["id"],json.dumps(p.get("joints",p),ensure_ascii=False)[:300])
for c in d["clips"]: print("clip",c["id"],c.get("durationS"),json.dumps({k:v for k,v in c.items() if k not in("title","description")},ensure_ascii=False)[:500])
if scale:
    r=d["representations"][scale]
    print("pens",r.get("pens"),"hatches",r.get("hatches"))
    print("decisions",json.dumps(r.get("decisions"),ensure_ascii=False,indent=1))
    for s in r["geometry"]["elevation"]:
        k={x:v for x,v in s.items() if x not in("filled",)}
        print(json.dumps(k,ensure_ascii=False))
