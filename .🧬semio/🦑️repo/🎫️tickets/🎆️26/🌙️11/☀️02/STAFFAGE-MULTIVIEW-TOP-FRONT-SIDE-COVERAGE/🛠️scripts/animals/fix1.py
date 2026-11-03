import json, math, sys
def dump(f, d):
    open(f, "w", encoding="utf-8", newline="\n").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
def world(a):
    w = {}
    for b in a["rig"]["bones"]:
        p = w[b["parent"]] if b["parent"] else (0, 0, 0)
        c, s = math.cos(math.radians(p[2])), math.sin(math.radians(p[2]))
        w[b["id"]] = (p[0] + c * b["x"] - s * b["y"], p[1] + s * b["x"] + c * b["y"], p[2] + b["angleDeg"])
    return w
for f in sys.argv[1:]:
    doc = json.load(open(f, encoding="utf-8-sig"))
    a = doc["asset"]
    L = {b["id"]: b["lengthM"] for b in a["rig"]["bones"]}
    W = world(a)
    n = 0
    for k, r in a["representations"].items():
        if r["status"] != "authored": continue
        for s in r["geometry"]["plan"]:
            if s["kind"] != "capsule": continue
            o = W[s["bone"]]; l = L[s["bone"]]
            tx = o[0] + l * math.cos(math.radians(o[2]))
            tol = 0.15 * l + 0.02
            spans = abs(s["x0"] - o[0]) <= tol and abs(s["x1"] - tx) <= tol
            if spans and not s.get("followTip"):
                s["followTip"] = True; n += 1
    print(a["id"], "restored", n)
    dump(f, doc)
