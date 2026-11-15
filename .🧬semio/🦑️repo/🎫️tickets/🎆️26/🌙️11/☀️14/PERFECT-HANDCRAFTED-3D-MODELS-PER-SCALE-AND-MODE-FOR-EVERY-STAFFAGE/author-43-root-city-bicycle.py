import json, re, sys
P = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/city-bicycle/🔣️.json"
txt = open(P).read()
asset = json.loads(txt)["asset"]
ds = set()
for sc, r in asset["representations"].items():
    for v, l in r.get("geometry", {}).items():
        for s in l:
            if s["kind"] == "path" and s["d"].count("M") == 2 and s["id"] in ("tyre-f", "tyre-r", "rim-f", "rim-r", "wheel-f", "wheel-r"):
                ds.add(s["d"])
n = 0
for d in ds:
    a, b = d.split("M")[1:]
    toks = re.findall(r"[ML]\s*-?[\d.e-]+,-?[\d.e-]+", b.strip().join(["M", ""]))
    pts = re.findall(r"-?[\d.e-]+,-?[\d.e-]+", b)
    rev = [pts[0]] + pts[1:][::-1]
    new = "M" + a.rstrip() + " M " + " L ".join(rev)
    if new == d or txt.count(json.dumps(d)[1:-1]) == 0:
        continue
    txt = txt.replace(json.dumps(d)[1:-1], json.dumps(new)[1:-1])
    n += 1
json.loads(txt)
open(P, "w").write(txt)
print("replaced", n)
