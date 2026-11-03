import json

C = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/"
GEOM = ("kind", "points", "closed", "x", "y", "w", "h", "r", "x0", "y0", "x1", "y1", "r0", "r1", "cx", "cy", "rx", "ry", "rotationDeg", "d")


def twin(bone):
    for a, b in (("-ff", "-fn"), ("-fn", "-ff"), ("-rf", "-rn"), ("-rn", "-rf"), ("-hf", "-hn"), ("-hn", "-hf"), ("-f", "-n"), ("-n", "-f")):
        if bone.endswith(a):
            return bone[: -len(a)] + b
    return None


def load(slug):
    p = C + slug + "/🔣️.json"
    return p, json.load(open(p, encoding="utf-8-sig"))


def save(p, d):
    open(p, "w", encoding="utf-8", newline="\n").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")


def fallbacks(slug):
    p, d = load(slug)
    a = d["asset"]
    L = {b["id"]: b["lengthM"] for b in a["rig"]["bones"]}
    n = 0
    for sc, r in a["representations"].items():
        if r["status"] != "authored":
            continue
        for v in ("front", "plan"):
            for s in r["geometry"][v]:
                sol = s.get("solid", {})
                if sol.get("kind") == "extruded" and s["kind"] == "capsule" and s["x0"] == 0 and s["y0"] == 0 and s["x1"] == L[s["bone"]] and s["y1"] == 0:
                    t = twin(s["bone"])
                    elev = [e for e in r["geometry"]["elevation"] if e["bone"] == t and e["role"] != "ground"]
                    if not elev:
                        continue
                    for k in GEOM:
                        s.pop(k, None)
                    s.update({k: elev[0][k] for k in GEOM if k in elev[0]})
                    n += 1
    save(p, d)
    print(slug, "twin profiles", n)


def fox():
    p, d = load("red-fox")
    a = d["asset"]
    n = 0
    for sc, r in a["representations"].items():
        if r["status"] != "authored":
            continue
        elev = {e["id"]: e for e in r["geometry"]["elevation"]}
        for v in ("front", "plan"):
            for s in r["geometry"][v]:
                if s.get("solid", {}).get("kind") == "revolved" and s["id"] in ("hindbody", "chest", "neck", "head"):
                    z = elev[s["id"]]["solid"]
                    s["solid"] = {"kind": "extruded", "z0": z["z0"], "z1": z["z1"]}
                    n += 1
    save(p, d)
    print("red-fox revolved to extruded", n)


for slug in ("hedgehog", "horse", "red-fox"):
    fallbacks(slug)
fox()
