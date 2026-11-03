import json, os, sys
base = r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🔌️appliances"
a = sys.argv[1]
what = sys.argv[2:]
d = json.load(open(os.path.join(base, a, "🔣️.json"), encoding="utf-8"))["asset"]
def j(x):
    return json.dumps(x, ensure_ascii=False, separators=(",", ":"))
for w in what:
    if w.startswith("rep"):
        _, sc, *views = w.split(":")
        r = d["representations"][sc]
        for k in r:
            if k != "geometry":
                print(f"[{sc}] {k}:", j(r[k]))
        for v, shapes in r["geometry"].items():
            if views and v not in views:
                continue
            print(f"[{sc}] geometry.{v}:")
            for s in shapes:
                print("   ", j(s))
    else:
        print(w, ":", j(d[w]))
