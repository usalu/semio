import json, sys, pathlib
root = pathlib.Path("🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
p = next(root.rglob(sys.argv[1] + "/🔣️.json"))
d = json.loads(p.read_text(encoding="utf-8"))["asset"]
for q in d["poses"]:
    print(q["id"], json.dumps({k: v for k, v in q.items() if k not in ("id", "joints", "angles", "rig", "state", "values")}, ensure_ascii=False)[:700])
print(json.dumps(d["thermal"]["operating"], ensure_ascii=False)[:1500])
