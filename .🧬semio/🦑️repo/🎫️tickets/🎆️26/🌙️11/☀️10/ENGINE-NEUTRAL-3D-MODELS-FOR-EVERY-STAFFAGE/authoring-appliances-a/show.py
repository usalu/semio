import json,glob,sys
def rnd(v):
    if isinstance(v,float): return round(v,4)
    if isinstance(v,list): return [rnd(x) for x in v]
    if isinstance(v,dict): return {k:rnd(x) for k,x in v.items()}
    return v
slug=sys.argv[1]
f=glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*appliances/%s/*.json"%slug)[0]
a=json.load(open(f))["asset"]
print("DIM",a["dimensions"])
print("MAT",[m["id"] for m in a["materials"]])
print("RIG",a["rig"]["kind"],a["rig"]["root"])
for b in a["rig"]["bones"]:
    print(" bone",b["id"],"parent",b["parent"],"x",b["x"],"y",b["y"],"ang",b["angleDeg"],"len",b.get("lengthM"),"joint",json.dumps(b.get("joint")))
print("REGIONS",[r["id"] for r in a["thermal"]["regions"]])
for p in a["poses"]:
    print("POSE",p["id"],json.dumps(rnd(p.get("bones")),ensure_ascii=False)[:300])
for c in a["clips"]:
    print("CLIP",c["id"],c["durationS"],c.get("rootMotion"),[ (k["t"],k.get("pose")) for k in c["keys"]][:8])
print("DEFAULTPOSE",a["presentation"]["defaultPose"])
skip={"id","bone","role","filled","kind","region","material","tone","z"}
for sc in ["50","100","200","500"]:
    r=a["representations"][sc]
    print("== SCALE",sc,r.get("status"))
    if "geometry" not in r: continue
    for vw,sh in r["geometry"].items():
        if sc!="50":
            print(" ",vw,[s["id"] for s in sh]); continue
        print(" --",vw)
        for s in sh:
            d={k:rnd(v) for k,v in s.items() if k not in skip}
            print("   ",s["id"],"|",s["bone"],s["role"],s["kind"],"t%s"%s.get("tone"),s.get("region"),s.get("material"),json.dumps(d,ensure_ascii=False))
