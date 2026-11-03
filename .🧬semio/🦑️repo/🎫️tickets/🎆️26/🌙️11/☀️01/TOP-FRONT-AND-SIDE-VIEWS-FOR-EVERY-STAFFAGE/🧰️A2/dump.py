import json, math, os, sys

ROOT = r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🐕️animals"


def load(a):
    return json.load(open(os.path.join(ROOT, a, "🔣️.json"), encoding="utf-8"))["asset"]


def rest_world(asset):
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    out = {}

    def solve(i):
        if i in out:
            return out[i]
        b = bones[i]
        if b.get("parent") is None:
            px, py, pa = 0.0, 0.0, 0.0
        else:
            px, py, pa = solve(b["parent"])
        c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
        x = px + c * b["x"] - s * b["y"]
        y = py + s * b["x"] + c * b["y"]
        a = pa + b["angleDeg"]
        out[i] = (x, y, a)
        return out[i]

    for i in bones:
        solve(i)
    return out, bones


def fmt(shape):
    k = shape["kind"]
    if k == "capsule":
        g = "cap(%.3f,%.3f->%.3f,%.3f r%.3f/%.3f)" % (shape["x0"], shape["y0"], shape["x1"], shape["y1"], shape["r0"], shape["r1"])
    elif k == "ellipse":
        g = "ell(%.3f,%.3f %.3f,%.3f)" % (shape["cx"], shape["cy"], shape["rx"], shape["ry"])
    elif k == "rect":
        g = "rect(%.3f,%.3f %.3f,%.3f)" % (shape["x"], shape["y"], shape["w"], shape["h"])
    elif k == "poly":
        g = "poly%d" % len(shape["points"])
    else:
        g = "path(%s)" % shape["d"][:60]
    return "%-22s %-10s %-7s z%-3s %s %s %s" % (shape["id"], shape["bone"], shape["role"], shape["z"], g, shape.get("region") or shape.get("regions") or "", shape.get("material", ""))


if __name__ == "__main__":
    a = sys.argv[1]
    scales = sys.argv[2].split(",") if len(sys.argv) > 2 else []
    d = load(a)
    w, bones = rest_world(d)
    print("dims", d["dimensions"], "pres", d["presentation"])
    print("BONES (rest world x,y,angle; local x,y,ang,len)")
    for b in d["rig"]["bones"]:
        x, y, ang = w[b["id"]]
        print("  %-12s par=%-10s world(%.3f,%.3f,%6.1f) loc(%.3f,%.3f,%6.1f) len=%.3f" % (b["id"], b.get("parent"), x, y, ang, b["x"], b["y"], b["angleDeg"], b["lengthM"]))
    print("POSES", [(p["id"], p.get("key", False)) for p in d["poses"]])
    print("CLIPS", [(c["id"], c["durationS"]) for c in d["clips"]])
    print("REGIONS", [r["id"] for r in d["thermal"]["regions"]])
    print("MATERIALS", [m["id"] for m in d["materials"]])
    for s, r in d["representations"].items():
        print("REP", s, r["status"], "pens", r.get("pens"), "hatches", [h["id"] for h in r.get("hatches", [])], {k: len(v) for k, v in r.get("geometry", {}).items()})
        if s in scales:
            for sh in r.get("geometry", {}).get("elevation", []):
                print("   ", fmt(sh))
