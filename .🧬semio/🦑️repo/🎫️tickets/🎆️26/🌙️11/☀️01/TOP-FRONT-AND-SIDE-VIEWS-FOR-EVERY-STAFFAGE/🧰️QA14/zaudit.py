import json, sys
sys.stdout.reconfigure(encoding="utf8")
ROOT = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/"
IDS = "steel-column sliding-door tilt-window tied-canopy wheelchair-ramp stair-flight spiral-stair".split()


def bbox(s):
    k = s["kind"]
    if k == "rect":
        return (s["x"], s["y"], s["x"] + s["w"], s["y"] + s["h"])
    if k == "poly":
        xs = [p[0] for p in s["points"]]; ys = [p[1] for p in s["points"]]
        return (min(xs), min(ys), max(xs), max(ys))
    if k == "ellipse":
        return (s["cx"] - s["rx"], s["cy"] - s["ry"], s["cx"] + s["rx"], s["cy"] + s["ry"])
    if k == "capsule":
        r = max(s["r0"], s["r1"])
        return (min(s["x0"], s["x1"]) - r, min(s["y0"], s["y1"]) - r, max(s["x0"], s["x1"]) + r, max(s["y0"], s["y1"]) + r)
    return None


def regs(s):
    return set(s.get("regions") or ([s["region"]] if s.get("region") else []))


def inside(a, b):
    return a[0] >= b[0] - 1e-6 and a[1] >= b[1] - 1e-6 and a[2] <= b[2] + 1e-6 and a[3] <= b[3] + 1e-6


n = 0
for aid in IDS:
    d = json.load(open(ROOT + aid + "/🔣️.json", encoding="utf8"))["asset"]
    for sc, rep in d["representations"].items():
        if rep.get("status") != "authored":
            continue
        for view in ("elevation", "front", "plan"):
            sh = rep["geometry"][view]
            for s in sh:
                if s.get("role") == "ground" or s["kind"] == "path" or s.get("solid"):
                    continue
                bs = bbox(s)
                if bs is None:
                    continue
                for t in sh:
                    if t is s or not t.get("filled") or t.get("role") == "ground" or t["kind"] == "path" or t.get("solid"):
                        continue
                    if t["bone"] != s["bone"] or t["z"] <= s["z"] and not (t["z"] == s["z"]):
                        pass
                    if t["bone"] != s["bone"] or not (regs(s) & regs(t)) or t["z"] <= s["z"]:
                        continue
                    bt = bbox(t)
                    if bt and inside(bs, bt) and (not s.get("filled") or s["role"] in ("detail", "texture")):
                        print(aid, sc, view, "detail", s["id"], "z", s["z"], "under filled", t["id"], "z", t["z"], sorted(regs(s) & regs(t)))
                        n += 1
print("hits", n)
