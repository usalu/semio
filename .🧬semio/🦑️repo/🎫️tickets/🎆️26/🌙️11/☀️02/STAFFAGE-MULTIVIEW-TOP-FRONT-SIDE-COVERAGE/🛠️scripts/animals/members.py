import json, sys, copy
C = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/"
KEEP = ("id", "bone", "role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm")
def member(base, id, bone, f, t, d, zs=0.0):
    s = {k: base[k] for k in KEEP if k in base}
    s["id"] = id; s["bone"] = bone
    s["kind"] = "member"
    s["from"] = {"x": f[0], "y": f[1], "z": zs}
    s["to"] = {"x": t[0], "y": t[1], "z": zs}
    s["section"] = {"kind": "circle", "d": d}
    return s
def apply(slug, scales, bone, replace_ids, parts):
    p = C + slug + "/🔣️.json"
    d = json.load(open(p, encoding="utf-8")); n = 0
    for sc in scales:
        r = d["asset"]["representations"][str(sc)]
        for view in ("front", "plan"):
            geo = r["geometry"][view]
            base = None; out = []; done = False
            for s in geo:
                if s["bone"] == bone and s["id"] in replace_ids and s["role"] == "body":
                    base = base or s
                    if not done:
                        for pid, f, t, dd in parts:
                            out.append(member(base, pid, bone, f, t, dd))
                        done = True
                    n += 1
                else:
                    out.append(s)
            r["geometry"][view] = out
    open(p, "w", encoding="utf-8", newline="\n").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
    print(slug, bone, "replaced", n)
if __name__ == "__main__":
    apply("labrador-dog", [50], "head", {"head", "nose"}, [("head-cranium", (0.0, 0.018), (0.13, 0.018), 0.115), ("head-muzzle", (0.13, 0.015), (0.25, 0.015), 0.06)])
