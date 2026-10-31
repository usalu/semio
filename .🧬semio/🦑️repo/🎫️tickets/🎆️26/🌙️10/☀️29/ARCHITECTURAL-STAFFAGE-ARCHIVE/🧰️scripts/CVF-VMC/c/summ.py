import json,sys
S="/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/"
a=json.load(open(S+sys.argv[1]+"/🔣️.json"))["asset"]
print(a["id"],a["dimensions"],a["presentation"])
print("TAGS",a["tags"])
print("RIG",a["rig"]["kind"],a["rig"]["root"])
for b in a["rig"]["bones"]:
    print(" B",b["id"],b["parent"],b["x"],b["y"],b["angleDeg"],b["lengthM"],json.dumps(b["joint"]),json.dumps(b.get("mass")) if "mass" in b else "", json.dumps(b.get("section")) if "section" in b else "")
for x in a["anchors"]:
    print(" A",json.dumps({k:v for k,v in x.items() if k!="title"}))
print("MAT",[ (m["id"],m["emissivity"],m.get("solarAbsorptivity")) for m in a["materials"]])
for s in a["geometry"]["elevation"]:
    d={k:v for k,v in s.items() if k not in("id","bone","role","lod","z","filled","tone","region","material")}
    print(" S",s["id"],s["bone"],s["role"],"L%d"%s["lod"],"z%s"%s["z"],"F" if s["filled"] else "o",s["tone"],s.get("region"),json.dumps(d,ensure_ascii=False))
print("PLAN" , "plan" in a["geometry"])
for p in a["poses"]:
    print(" P",p["id"],json.dumps(p["bones"]),p["contacts"],[ (l["id"],l["kind"],l["fxN"],l["fyN"],l.get("distributed"),l.get("anchor"),l.get("bone"),l.get("x"),l.get("y")) for l in p["loads"]])
for c in a["clips"]:
    print(" C",c["id"],c["durationS"],c["loop"],[(k["t"],k["pose"],k.get("bones")) for k in c["keys"]],c["tracks"])
print("THERMAL",json.dumps({k:v for k,v in a["thermal"].items() if k not in("assumptions",)},ensure_ascii=False)[:2500])
