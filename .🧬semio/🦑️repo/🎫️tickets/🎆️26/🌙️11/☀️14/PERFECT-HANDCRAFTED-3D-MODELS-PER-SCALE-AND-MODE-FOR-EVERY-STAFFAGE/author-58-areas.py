import re, sys, json
slug = sys.argv[1]
a = json.load(open(f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/{slug}/🔣️.json"))["asset"]
decl = {r["id"]: r["areaM2"] for r in a["thermal"]["regions"]}
for d in sys.argv[2:]:
    t = open(f"🗑️generated/author-58/{slug}.s{d}.check.txt").read()
    ls = [x for x in t.splitlines() if "thermal-coverage" in x]
    if not ls: continue
    l = ls[0]
    outer = dict(re.findall(r"outerAreaM2:(\S+)=(\S+)", l))
    tot = sum(float(v) for v in outer.values()); dt = sum(decl[k] for k in outer)
    print(d, " ".join(f"{k}:{float(v):.3f}/{decl[k]}({float(v)/decl[k]:.2f})" for k, v in outer.items()), f"TOTAL {tot:.3f}/{dt:.3f} ({tot/dt:.2f})")
