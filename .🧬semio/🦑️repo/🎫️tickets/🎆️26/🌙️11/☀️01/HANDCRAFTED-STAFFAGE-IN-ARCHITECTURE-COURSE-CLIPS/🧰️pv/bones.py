import json,sys,glob
B="🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
d=json.load(open(glob.glob(f"{B}/*/{sys.argv[1]}/🔣️.json")[0],encoding="utf-8"))["asset"]
for b in d["rig"]["bones"]: print(b["id"],b.get("parent"),b["x"],b["y"],b.get("joint"))
for p in d["poses"]: print("POSE",p["id"],json.dumps({k:v for k,v in p.items() if k not in("thermal","title","description")},ensure_ascii=False)[:700])
print([ (e["id"],e.get("operatingKey")) for e in d["thermal"]["emitters"]], [(o["id"]) for o in d["thermal"]["operating"]])
