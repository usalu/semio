import json
P = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/city-bicycle/🔣️.json"
txt = open(P).read()
asset = json.loads(txt)["asset"]
ds = set()
for sc, r in asset["representations"].items():
    for v, l in r.get("geometry", {}).items():
        for s in l:
            if s["kind"] == "path" and s["d"].count("M") == 2 and s["id"] in ("tyre-f", "tyre-r", "rim-f", "rim-r", "wheel-f", "wheel-r") and "Z" not in s["d"]:
                ds.add(s["d"])
n = 0
for d in ds:
    a, b = d.split("M")[1:]
    new = "M" + a.rstrip() + " Z M" + b.rstrip() + " Z"
    k = json.dumps(d)[1:-1]
    assert txt.count(k) >= 1
    txt = txt.replace(k, json.dumps(new)[1:-1])
    n += 1
json.loads(txt)
open(P, "w").write(txt)
print("closed", n)
