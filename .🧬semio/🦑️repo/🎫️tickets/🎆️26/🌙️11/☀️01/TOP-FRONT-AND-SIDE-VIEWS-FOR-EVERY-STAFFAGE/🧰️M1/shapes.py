import json, sys, io, os
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf8")
S = r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🏗️machinery"
aid, scale, view = sys.argv[1], sys.argv[2], sys.argv[3]
d = json.load(open(os.path.join(S, aid, "🔣️.json"), encoding="utf8"))
r = d["asset"]["representations"][scale]
if len(sys.argv) > 4 and sys.argv[4] == "meta":
    for k, v in r.items():
        if k != "geometry":
            print(k, json.dumps(v, ensure_ascii=False)[:1500])
    sys.exit()
for s in r["geometry"][view]:
    o = dict(s)
    print(json.dumps(o, ensure_ascii=False, separators=(",", ":")))
