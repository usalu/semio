"""🚜 Forklift 2D drawings, pass 1 (second round): the drive axle housing drawn in elevation (behind the wheels), front and plan (in the slot between front frame and mast), the lift chains wrapped over their sheaves, fork shanks 45 mm.

Run from the ticket folder: python3 rework-forklift-p1-2d.py
"""
import math
exec(open("rework-10-lib.py").read())
C = Catalog("forklift")
AY = 0.29575


def chain_profile():
    c = (1.94, -0.035)
    ring = lambda r, angles: [[round(c[0] + r * math.sin(math.radians(a)), 5), round(c[1] - r * math.cos(math.radians(a)), 5)] for a in angles]
    return [[0.44, -0.106], [1.94, -0.106]] + ring(0.071, range(15, 180, 15)) + [[1.94, 0.036], [1.3, 0.036], [1.3, 0.02], [1.94, 0.02]] + ring(0.055, range(165, 0, -15)) + [[1.94, -0.09], [0.44, -0.09]]


def shape(sid, z, tone, geometry):
    return {"id": sid, "bone": "axle-f", "role": "body", "z": z, "filled": True, "tone": tone, "region": "chassis-paint", "material": "steel", **geometry}


def rect(x0, y0, x1, y1):
    return {"kind": "poly", "points": [[x0, y0], [x1, y0], [x1, y1], [x0, y1]], "closed": True}


for scale in ("50", "100", "200", "500"):
    g = C.asset["representations"][scale]["geometry"]
    for view in g:
        g[view] = [s for s in g[view] if not s["id"].startswith("axle-motor") and not s["id"].startswith("axle-gear")]
    if scale == "500":
        continue
    half = 0.3 if scale in ("50", "100") else 0.39
    g["elevation"].insert(0, shape("axle-motor", 1.9, 0.3, {"kind": "ellipse", "cx": 0, "cy": 0, "rx": 0.13, "ry": 0.13}))
    g["front"].insert(0, shape("axle-motor", 1.9, 0.3, rect(-half, AY - 0.13, half, AY + 0.13)))
    g["plan"].insert(0, shape("axle-motor", 1.9, 0.3, rect(-0.13, -half, 0.13, half)))
    if scale in ("50", "100"):
        g["elevation"].insert(1, shape("axle-gear", 1.91, 0.35, {"kind": "ellipse", "cx": 0, "cy": 0, "rx": 0.11, "ry": 0.11}))
        for side, sign in (("l", 1), ("r", -1)):
            lo, hi = sorted((sign * 0.3, sign * 0.385))
            g["front"].insert(1, shape("axle-gear-" + side, 1.92 if side == "l" else 1.93, 0.35, rect(lo, AY - 0.11, hi, AY + 0.11)))
            g["plan"].insert(1, shape("axle-gear-" + side, 1.92 if side == "l" else 1.93, 0.35, rect(-0.11, lo, 0.11, hi)))
        for view in g:
            for s in g[view]:
                if s["id"] in ("chain", "chain-l", "chain-r") and s.get("points") == [[0.44, -0.077], [0.44, -0.093], [1.94, -0.093], [1.94, -0.077]]:
                    s["points"] = chain_profile()
                    print("[DEBUG] chain", scale, view, s["id"])
                if s["id"] in ("tine-l", "tine-r") and s.get("kind") == "poly" and [0, 0.462] in s["points"]:
                    s["points"] = [[round(x + 0.005, 5), y] if (0 <= x <= 0.015 and y >= 0) else [x, y] for x, y in s["points"]]
                    print("[DEBUG] tine", scale, view, s["id"])
C.touch("representations")
print("[DEBUG] wrote", C.write())
