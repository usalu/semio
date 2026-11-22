"""🔧️ HW-09 adjustments of the city-bicycle part inside `person-pushing-bicycle` (design input of build_person_pushing_bicycle.py):
the bicycle moves CENTRE to the right so the figure is centred on z = 0, every bicycle shape keeps the pen of its own drawing, the
1:500 drawing turns member strokes into filled capsules, and solids below the smallest feature of their scale are typified.
The three fork shapes of 1:500 (26 to 32 mm wide, 0.194 to 0.199 mm with the 0.15 mm pen) get a 0.16 mm pen and so keep the 0.2 mm print
minimum; the right chainstay is not drawn in the 1:50 front view, where it runs end-on behind the bottom bracket and chain case and only
a 0.06 mm sliver of it showed beside them when the bicycle is lifted onto the stand.
"""
import copy
import math
from hw09lib import shift_rest, SCALES, VIEWS, LISTS

LIMIT = {"100": {"*": 0.0151}, "200": {"*": 0.0302}, "500": {"wheel-r": 0.0375, "wheel-f": 0.0375, "frame": 0.029, "*": 0.0302}}
NUTS = ("nut-r-l", "nut-r-r", "nut-f-l", "nut-f-r")


def rest_frames(bic):
    """🦴️ Rest world origin and angle of the planar bicycle bones (frame, steer, wheels, crank, pedals, stand)."""
    out, by = {}, {b["id"]: b for b in bic["rig"]["bones"]}
    def frame(i):
        if i in out:
            return out[i]
        b = by[i]
        if b["parent"] is None:
            out[i] = (b["x"], b["y"], b.get("z", 0), math.radians(b["angleDeg"]))
        else:
            px, py, pz, pa = frame(b["parent"])
            x, y = b["x"], b["y"]
            out[i] = (px + x * math.cos(pa) - y * math.sin(pa), py + x * math.sin(pa) + y * math.cos(pa), pz + b.get("z", 0), pa + math.radians(b["angleDeg"]))
        return out[i]
    for i in by:
        frame(i)
    return out


def capsule_of(shape, view, frames):
    """💊️ A member stroke as a filled capsule in the drawing coordinates of its view."""
    def world(end):
        x0, y0, z0, a = frames[end.get("bone", shape["bone"])]
        return (x0 + end["x"] * math.cos(a) - end["y"] * math.sin(a), y0 + end["x"] * math.sin(a) + end["y"] * math.cos(a), z0 + end["z"])
    sec = shape["section"]
    r = round((sec["d"] if sec["kind"] == "circle" else max(sec["w"], sec["h"])) / 2, 5)
    if view == "elevation":
        x0, y0, z0, a = frames[shape["bone"]]
        p, q = shape["from"], shape["to"]
        c = (p["x"], p["y"], q["x"], q["y"])
    else:
        P, Q = world(shape["from"]), world(shape["to"])
        c = (P[2], P[1], Q[2], Q[1]) if view == "front" else (P[0], P[2], Q[0], Q[2])
    out = {k: v for k, v in shape.items() if k not in ("from", "to", "section", "kind")}
    out.update(kind="capsule", x0=round(c[0], 5), y0=round(c[1], 5), x1=round(c[2], 5), y1=round(c[3], 5), r0=r, r1=r, filled=True)
    return out


def thicken(x, limit):
    """🧱️ Raises the thin dimension of a bicycle solid to the limit of its scale (or declares it slender when it is long enough)."""
    k = x["kind"]
    if k == "capsule":
        length = math.dist(x["p0"], x["p1"])
        r = max(x["r0"], x["r1"])
        if length >= 20 * 2 * r:
            x["slender"] = True
        else:
            x["r0"] = x["r1"] = round(limit / 2, 5)
    elif k == "revolve":
        axial = [p[0] for p in x["profile"]]
        radial = [p[1] for p in x["profile"]]
        if max(radial) < limit / 2:
            x["profile"] = [[p[0], round(limit / 2, 5) if p[1] > 0 else 0] for p in x["profile"]]
        else:
            span = max(axial) - min(axial)
            if span < limit:
                c = (max(axial) + min(axial)) / 2
                f = limit / span
                x["profile"] = [[round(c + (p[0] - c) * f, 5), p[1]] for p in x["profile"]]


def fix_bike(bike, bic, centre):
    shift_rest(bike, dz=-centre)
    frames = {k: (x, y, z - centre, a) for k, (x, y, z, a) in rest_frames(bic).items()}
    for s in SCALES:
        src = bic["representations"][s]
        pens = src["pens"] if src["status"] == "authored" else {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.13}
        for v in VIEWS:
            shapes = []
            for x in bike["shapes"][s][v]:
                if s == "500" and x["kind"] == "member":
                    x = capsule_of(x, v, frames)
                if "penMm" not in x and pens.get(x["role"]) is not None:
                    x["penMm"] = pens[x["role"]]
                if s == "500" and x["id"] in ("fork-l", "fork-r", "fork-crown"):
                    x["penMm"] = 0.16
                if s == "500" and x["id"] in ("tyre-r", "tyre-f", "wheel-r", "wheel-f") and "hub-brakes" not in x.get("regions", [x.get("region")]):
                    regs = x.pop("regions", None) or [x.pop("region")]
                    x["regions"] = regs + ["hub-brakes"]
                shapes.append(x)
            if s == "50" and v == "front":
                shapes = [x for x in shapes if x["id"] != "chainstay-r"]
            bike["shapes"][s][v] = shapes
        vol = bike["solids"][s]
        vol["body"] = [x for x in vol["body"] if not (s in LIMIT and x["id"] in NUTS)]
        for lst in ["thermal", "structure"]:
            vol[lst] = [x for x in vol[lst] if not (s in LIMIT and x.get("of") in NUTS)]
        for lst in LISTS:
            for x in vol[lst]:
                if x.get("part") == "support":
                    x["part"] = "member"
                    x.pop("anchor", None)
                if s in LIMIT and "of" not in x and "kind" in x:
                    lim = LIMIT[s].get(x["bone"], LIMIT[s]["*"])
                    if x["id"].startswith(("axle", "rim-", "bearing")) or (s == "500" and x["bone"] == "frame" and x["kind"] == "capsule"):
                        thicken(x, lim)
    regions = {r for s in SCALES for v in VIEWS for x in bike["shapes"][s][v] for r in x.get("regions", [x.get("region")])}
    print("bike regions drawn at 500:", sorted(r for r in {r for v in VIEWS for x in bike["shapes"]["500"][v] for r in x.get("regions", [x.get("region")])} if r))
