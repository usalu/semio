"""🦿 Forward kinematics of the staffage rig (port of the rig module's chain) for humans/skateboarder: joint positions per pose."""
import json, math, sys

ASSET = sys.argv[1] if len(sys.argv) > 1 and sys.argv[1].endswith(".json") else "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/skateboarder/🔣️.json"


def sc(deg):
    r = math.radians(deg)
    return math.sin(r), math.cos(r)


def comb(a, s, b, t):
    return [a[i] * s + b[i] * t for i in range(3)]


def clamp(v, lo, hi):
    if lo is not None:
        v = max(v, lo)
    if hi is not None:
        v = min(v, hi)
    return v


def chain(rig, offsets, root):
    rs, rc = sc(root.get("angleDeg", 0))
    base = {"o": [root.get("x", 0), root.get("y", 0), 0], "u": [rc, rs, 0], "v": [-rs, rc, 0], "w": [0, 0, 1]}
    out = {}
    byid = {b["id"]: b for b in rig["bones"]}
    done = set()

    def visit(b):
        if b["id"] in done:
            return
        if b["parent"]:
            visit(byid[b["parent"]])
        p = out[b["parent"]] if b["parent"] else base
        off = offsets.get(b["id"], {})
        j = b["joint"]
        a = ab = tw = dx = dy = 0
        if j["type"] == "revolute":
            a = clamp(off.get("angleDeg", 0), j.get("minDeg"), j.get("maxDeg"))
            ab = off.get("abductionDeg", 0)
            tw = off.get("twistDeg", 0)
        elif j["type"] == "prismatic":
            s, c = sc(j.get("axisDeg", 0))
            t = clamp(off.get("dx", 0) * c + off.get("dy", 0) * s, j.get("minM"), j.get("maxM"))
            dx, dy = t * c, t * s
        along = comb(p["u"], b["x"] + dx, p["v"], b["y"] + dy)
        z = b.get("z", 0)
        o = [p["o"][i] + along[i] + p["w"][i] * z for i in range(3)]
        ts, tc = sc(b["angleDeg"] + a)
        u1 = comb(p["u"], tc, p["v"], ts)
        v1 = comb(p["u"], -ts, p["v"], tc)
        ps, pc = sc(ab)
        u2 = comb(u1, pc, p["w"], ps)
        w2 = comb(u1, -ps, p["w"], pc)
        ws, wc = sc(tw)
        out[b["id"]] = {"o": o, "u": u2, "v": comb(v1, wc, w2, -ws), "w": comb(v1, ws, w2, wc), "len": b["lengthM"]}
        done.add(b["id"])

    for b in rig["bones"]:
        visit(b)
    return out


def pt(f, x, y=0, z=0):
    return [f["o"][i] + f["u"][i] * x + f["v"][i] * y + f["w"][i] * z for i in range(3)]


if __name__ == "__main__":
    a = json.load(open(ASSET))["asset"]
    names = [x for x in sys.argv[1:] if not x.endswith(".json")] or [p["id"] for p in a["poses"]]
    for p in a["poses"]:
        if p["id"] not in names:
            continue
        F = chain(a["rig"], p["bones"], p.get("root", {}))
        print("==", p["id"], "root", p.get("root"), "ground", p.get("ground"), "contacts", p.get("contacts"))
        for b in [b["id"] for b in a["rig"]["bones"]]:
            f = F[b]
            tip = pt(f, f["len"])
            ang = math.degrees(math.atan2(f["u"][1], f["u"][0]))
            print(f"  {b:11s} o=({f['o'][0]:+.3f},{f['o'][1]:+.3f},{f['o'][2]:+.3f}) tip=({tip[0]:+.3f},{tip[1]:+.3f},{tip[2]:+.3f}) dir={ang:+7.1f}  off={p['bones'].get(b)}")
        for an in a["anchors"]:
            q = pt(F[an["bone"]], an["x"], an["y"], an.get("z", 0))
            print(f"    anchor {an['id']:20s} ({q[0]:+.3f},{q[1]:+.3f},{q[2]:+.3f})")
