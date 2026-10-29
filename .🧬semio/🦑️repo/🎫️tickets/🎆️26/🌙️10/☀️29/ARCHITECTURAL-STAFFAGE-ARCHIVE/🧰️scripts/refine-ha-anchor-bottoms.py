import json, re, sys
d = json.load(open("/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/🔣️.json"))
def pts(s):
    k = s["kind"]
    if k == "capsule": return [(s["x0"], s["y0"] - s["r0"]), (s["x1"], s["y1"] - s["r1"]), (s["x0"], s["y0"] + s["r0"])]
    if k == "ellipse": return [(s["cx"], s["cy"] - s["ry"]), (s["cx"], s["cy"] + s["ry"])]
    if k == "poly": return [tuple(p) for p in s["points"]]
    if k == "path":
        t = re.findall(r"-?\d+\.?\d*(?:e-?\d+)?", s["d"]); v = list(map(float, t)); return list(zip(v[0::2], v[1::2]))
    return []
for a in d["assets"]:
    if len(sys.argv) > 1 and a["id"] not in sys.argv[1:]: continue
    for an in a["anchors"]:
        if an["kind"] != "contact": continue
        low = min((y for s in a["geometry"]["elevation"] if s["bone"] == an["bone"] and s["role"] == "body" for x, y in pts(s) if abs(x - an["x"]) < max(0.03, an.get("footprintLengthM", 0.05) / 2)), default=None)
        print(a["id"], an["id"], an["bone"], "anchor", an["x"], an["y"], "shape-bottom", low)
