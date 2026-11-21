"""📐 Round 5 V03: measures the two-wheelers' rigs against their reference tables (wheelbase, wheel radii, bottom bracket and axle heights, head angle, trail, stand and crank lengths) in the rest frame, from the catalog file.

Usage: python3 r5-v03-measure.py"""
import json
import math

CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/{}/🔣️.json"


def frames(bones):
    out = {}
    by = {b["id"]: b for b in bones}

    def get(i):
        if i in out:
            return out[i]
        b = by[i]
        if b.get("parent"):
            px, py, pa = get(b["parent"])
            a = math.radians(pa)
            x = px + b.get("x", 0) * math.cos(a) - b.get("y", 0) * math.sin(a)
            y = py + b.get("x", 0) * math.sin(a) + b.get("y", 0) * math.cos(a)
            out[i] = (x, y, pa + b.get("angleDeg", 0))
        else:
            out[i] = (b.get("x", 0), b.get("y", 0), b.get("angleDeg", 0))
        return out[i]

    for b in bones:
        get(b["id"])
    return out, by


for slug, steer in (("city-bicycle", "steer"), ("cargo-bike", "fork"), ("motor-scooter", "steer")):
    a = json.load(open(CAT.format(slug), encoding="utf8"))["asset"]
    f, by = frames(a["rig"]["bones"])
    wf, wr = f["wheel-f"], f["wheel-r"]
    s = f[steer]
    head = -s[2]
    ang = math.radians(s[2])
    t = (wf[1]) / math.tan(math.radians(head))
    xa = s[0] + (s[1] - 0) * math.cos(math.radians(head)) / math.sin(math.radians(head))
    trail = xa - (wf[0])
    row = {"wheelbase": round(wf[0] - wr[0], 4), "axle-f y": round(wf[1], 4), "axle-r y": round(wr[1], 4), "r-f": by["wheel-f"]["lengthM"], "r-r": by["wheel-r"]["lengthM"], "head angle": round(head, 2), "trail": round(trail, 4), "dims": a["dimensions"]}
    for k in ("crank", "motor", "stand", "seat", "swing"):
        if k in f:
            row[k] = (round(f[k][0], 4), round(f[k][1], 4), by[k].get("lengthM"))
    print(slug, row)
