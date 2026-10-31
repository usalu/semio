# [scratch] NW-A1 local emulation of the suitability gap check on the named poses (visible outline point sets, hidden-line removal, join rule)
import json, math, sys, importlib
from lib import *


def inside(pt, ring):
    x, y = pt
    c = False
    n = len(ring)
    for i in range(n):
        x1, y1 = ring[i]
        x2, y2 = ring[(i + 1) % n]
        if (y1 > y) != (y2 > y) and x < (x2 - x1) * (y - y1) / (y2 - y1 + 1e-12) + x1:
            c = not c
    return c


def dens(r, closed, step=0.06):
    out = []
    n = len(r)
    for i in range(n if closed else n - 1):
        a, b = r[i], r[(i + 1) % n]
        L = math.hypot(b[0] - a[0], b[1] - a[1])
        k = max(1, int(L / step))
        for j in range(k):
            out.append((a[0] + (b[0] - a[0]) * j / k, a[1] + (b[1] - a[1]) * j / k))
    out.append(r[-1] if not closed else r[0])
    return out


def scan(asset, sk, den, thr=0.15):
    rep = asset["representations"][str(den)]
    shapes = rep["geometry"]["elevation"]
    mm = 1000.0 / den
    out = []
    parent = {b["id"]: b["parent"] for b in sk.bones}
    for p in asset["poses"]:
        if p.get("key"):
            continue
        offs = {b: v.get("angleDeg", 0.0) for b, v in p["bones"].items()}
        root = (sk.hip[0], p["root"]["y"]) if p.get("root") and "y" in p["root"] else None
        fr = sk.frames(offs, root)
        if p["ground"] == "auto" and p["contacts"]:
            drop = min(sk.anchor_w(fr, c)[1] for c in p["contacts"])
            fr = {kk: ((o[0], o[1] - drop), a) for kk, (o, a) in fr.items()}
        items = []
        for s, r in world_rings(sk, shapes, fr):
            if s["role"] == "ground":
                continue
            closed = s["kind"] != "poly" or s["closed"]
            ring = [(x * mm, y * mm) for x, y in r]
            pen = s.get("penMm") or rep["pens"][s["role"]] or 0
            items.append(dict(s=s, ring=ring, closed=closed, filled=s["filled"] and closed, pen=pen))

        def joined(a, b):
            sa, sb = a["s"], b["s"]
            ra = sa.get("region") or None
            rb = sb.get("region") or None
            return ra is not None and ra == rb and (sa["bone"] == sb["bone"] or parent.get(sa["bone"]) == sb["bone"] or parent.get(sb["bone"]) == sa["bone"])

        for a in items:
            pts = dens(a["ring"], a["closed"])
            vis = []
            for pt in pts:
                hid = False
                for b in items:
                    if b is a or not b["filled"]:
                        continue
                    if b["s"]["z"] > a["s"]["z"] or joined(a, b):
                        if inside(pt, b["ring"]):
                            hid = True
                            break
                if not hid:
                    vis.append(pt)
            a["vis"] = vis
        for i in range(len(items)):
            for j in range(i + 1, len(items)):
                a, b = items[i], items[j]
                if not a["vis"] or not b["vis"]:
                    continue
                best = 1e9
                for x in a["vis"][::2]:
                    for y in b["vis"][::2]:
                        d = math.hypot(x[0] - y[0], x[1] - y[1])
                        if d < best:
                            best = d
                gap = best - (a["pen"] + b["pen"]) / 2
                if 0.0 < gap < thr and best > 0.09:
                    out.append((round(gap, 3), p["id"], a["s"]["id"], b["s"]["id"]))
    return sorted(set(out))


def run(name, mod, dims=None):
    m = importlib.import_module(mod)
    sk = m.make_skel()
    a = json.load(open("/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/%s/🔣️.json" % name))["asset"]
    tot = 0
    for den in (500, 200, 100, 50):
        if a["representations"][str(den)]["status"] != "authored":
            continue
        res = scan(a, sk, den)
        tot += len(res)
        print("1:%d" % den, len(res))
        for r in res[:30]:
            print("   ", r)
    return tot


if __name__ == "__main__":
    run(sys.argv[1], sys.argv[2])
