import json, os, sys
root = r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🪑️objects"
i = sys.argv[1]
scales = sys.argv[2:] or ["500", "200", "100", "50"]
d = json.load(open(os.path.join(root, i, "🔣️.json"), encoding="utf-8"))["asset"]
if "full" in scales:
    scales = ["500", "200", "100", "50"]
    for b in d["rig"]["bones"]:
        print("BONE", json.dumps({k: v for k, v in b.items() if k not in ("mass", "title")}, ensure_ascii=False))
    print("ANCHORS", json.dumps(d["anchors"], ensure_ascii=False)[:600])
    for p in d["poses"]:
        print("POSE", json.dumps(p, ensure_ascii=False)[:500])
    for c in d["clips"]:
        print("CLIP", json.dumps(c, ensure_ascii=False)[:700])
    print("THERMAL", json.dumps(d["thermal"], ensure_ascii=False)[:1200])
    print("MAT", json.dumps(d["materials"], ensure_ascii=False)[:600])
for s in scales:
    if s == "full":
        continue
    r = d["representations"][s]
    print("#####", s, r["status"], json.dumps({k: v for k, v in r.items() if k not in ("geometry", "decisions")}, ensure_ascii=False))
    for v, shapes in r.get("geometry", {}).items():
        print(" --", v)
        for sh in shapes:
            print("  ", json.dumps(sh, ensure_ascii=False))
