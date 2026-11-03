import math, json, sys
from kit import *
D = math.pi / 180
AZ = {"R1": -10, "R4": 40, "R5": 62, "R2": 85, "L5": 108, "L3": 128, "L1": 168, "L4": 220, "L2": 272, "R3": 308}

def dirv(e, az):
    return (math.cos(e*D)*math.cos(az*D), math.cos(e*D)*math.sin(az*D), math.sin(e*D))

def dot(a, b): return sum(x*y for x, y in zip(a, b))

def solve():
    doc = load("agave")
    a_ = doc["asset"]
    r, bones = rest(doc)
    out = {}
    for n, az in AZ.items():
        single = n in ("R5", "L5")
        ia = f"leaf-{n}" if single else f"leaf-{n}-a"
        side = 1 if n[0] == "R" else -1
        al0 = r[ia][2] % 360
        ea = al0 if side > 0 else 180 - al0
        da = dirv(ea, az)
        alpha = math.degrees(math.atan2(da[2], da[0]))
        phi = math.degrees(math.asin(max(-1, min(1, da[1]))))
        res = {ia: (alpha - 90, phi)}
        if not single:
            ib = f"leaf-{n}-b"
            alb = r[ib][2] % 360
            eb = alb if side > 0 else 180 - alb
            if eb > 180: eb -= 360
            db = dirv(eb, az)
            ca, sa = math.cos(alpha*D), math.sin(alpha*D)
            cp, sp = math.cos(phi*D), math.sin(phi*D)
            ua = (cp*ca, sp, cp*sa)
            va = (-sa, 0, ca)
            wa = (-sp*ca, cp, -sp*sa)
            phb = math.degrees(math.asin(max(-1, min(1, dot(db, wa)))))
            dl = math.degrees(math.atan2(dot(db, va), dot(db, ua)))
            res[ib] = (dl, phb)
        out[n] = res
    return doc, out

if __name__ == "__main__":
    doc, out = solve()
    for n, v in out.items():
        print(n, {k: (round(a, 1), round(p, 1)) for k, (a, p) in v.items()})
