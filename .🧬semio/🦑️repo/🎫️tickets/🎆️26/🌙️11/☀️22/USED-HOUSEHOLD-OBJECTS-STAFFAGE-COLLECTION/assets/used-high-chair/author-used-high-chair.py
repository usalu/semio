"""🪑️ Authoritative chisel of the staffage asset `used-high-chair`: a second-hand step high chair of lacquered beech that grows with the child, with its clip-on tray bow and the EVA foam floor tile it came with.

Every number of the real object is stated here once (world metres of the rest pose `stored`: x ahead, y up, z left) and the parts are written per scale as prisms, so the 2D drawings of the three views and the three volume lists of each scale come from the same handcrafted numbers. Run `python3 author-used-high-chair.py` to rewrite `🔣️.json` next to this file.

@see ../../brief.md — the author brief of the wave
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — the asset contract
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "🔣️.json")

TM = 0.012
GL = 0.008
YR0 = TM + GL
YR1 = 0.070
XR0, XR1 = -0.27, 0.22
ZI, ZO = 0.203, 0.230
THETA = math.radians(70.0)
COT = math.cos(THETA) / math.sin(THETA)
BW = 0.075 / math.sin(THETA)
XF0 = 0.14
YT = TM + 0.79
SLAT_H = 0.018 / math.sin(THETA)
SLATS = [(0.565, 0.645), (0.696, 0.776)]
SEAT = dict(x0=-0.07, x1=0.13, y0=0.464, y1=0.482, z=0.21, r=0.02)
FOOT = dict(x0=-0.015, x1=0.205, y0=0.244, y1=0.262, z=0.21, r=0.015)
BOW = dict(x0=-0.10, x1=0.13, bar=0.03, arm=0.025, zo=0.2025, y0=0.482, y1=0.512)
TRAY = dict(x0=-0.01, x1=0.23, z=0.19, r=0.025, wall=0.003, y0=0.512, y1=0.542)
MAT = dict(x0=-0.31, x1=0.29, z0=-0.30, z1=0.30, pitch=0.06, root=0.024, tip=0.032, depth=0.012)
ROD = dict(y=0.112, r=0.0125, z=0.215)
BOW_LIFT = 0.145
BOW_SLIDE = 0.10
CARRY_LIFT = 0.16
CARRY_TILT = 8.0

RHO = {"beech-lacquered": 710, "beech-plywood": 690, "polypropylene": 905, "pp-glide": 905, "wool-felt": 280, "eva-foam": 90, "zinc-steel": 7850}


def F(y):
    """📐️ The front edge of a side upright at height y (world x)."""
    return XF0 - (y - YR0) * COT


def R(y):
    """📏️ The rear edge of a side upright at height y (world x)."""
    return F(y) - BW


def rnd(v):
    """🎯️ A coordinate rounded to a tenth of a millimetre, never negative zero."""
    return round(float(v), 5) + 0.0


def arc(cx, cy, r, a0, a1, n):
    """🌙️ Points on a circular arc from angle a0 to a1 (degrees), both ends included."""
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * k / n)), cy + r * math.sin(math.radians(a0 + (a1 - a0) * k / n))) for k in range(n + 1)]


def circle(cx, cy, r, n=16):
    """⭕️ A closed polygon approximating a circle."""
    return [(cx + r * math.cos(2 * math.pi * k / n), cy + r * math.sin(2 * math.pi * k / n)) for k in range(n)]


def rrect(x0, x1, y0, y1, r0=0.0, r1=0.0, r2=0.0, r3=0.0, n=4):
    """⬜️ A rectangle with individual corner radii (rear-right, front-right, front-left, rear-left in a plan of x ahead and z left)."""
    pts = []
    pts += [(x0, y0 + r0)] + arc(x0 + r0, y0 + r0, r0, 180, 270, n)[1:-1] + [(x0 + r0, y0)] if r0 else [(x0, y0)]
    pts += [(x1 - r1, y0)] + arc(x1 - r1, y0 + r1, r1, 270, 360, n)[1:-1] + [(x1, y0 + r1)] if r1 else [(x1, y0)]
    pts += [(x1, y1 - r2)] + arc(x1 - r2, y1 - r2, r2, 0, 90, n)[1:-1] + [(x1 - r2, y1)] if r2 else [(x1, y1)]
    pts += [(x0 + r3, y1)] + arc(x0 + r3, y1 - r3, r3, 90, 180, n)[1:-1] + [(x0, y1 - r3)] if r3 else [(x0, y1)]
    out = []
    for p in pts:
        if not out or abs(out[-1][0] - p[0]) > 1e-9 or abs(out[-1][1] - p[1]) > 1e-9:
            out.append(p)
    if abs(out[0][0] - out[-1][0]) < 1e-9 and abs(out[0][1] - out[-1][1]) < 1e-9:
        out.pop()
    return out


def area_centroid(poly):
    """⚖️ Signed-area-free area and centroid of a simple polygon."""
    a = cx = cy = 0.0
    for i in range(len(poly)):
        x0, y0 = poly[i]
        x1, y1 = poly[(i + 1) % len(poly)]
        c = x0 * y1 - x1 * y0
        a += c
        cx += (x0 + x1) * c
        cy += (y0 + y1) * c
    a *= 0.5
    return abs(a), (cx / (6 * a), cy / (6 * a))


def perimeter(poly):
    """➰️ The length of the closed outline of a polygon."""
    return sum(math.dist(poly[i], poly[(i + 1) % len(poly)]) for i in range(len(poly)))


def mat_outline(teeth=True, bitten=None):
    """🧩️ The plan outline (x, z) of the EVA puzzle tile: dovetail tabs on the front and left edge, matching slots on the rear and right edge."""
    m = MAT
    if not teeth:
        e = m["depth"] * (m["root"] + m["tip"]) / 2 / m["pitch"]
        return [(m["x0"] + e, m["z0"] + e), (m["x1"] + e, m["z0"] + e), (m["x1"] + e, m["z1"] + e), (m["x0"] + e, m["z1"] + e)]
    n = int(round((m["x1"] - m["x0"]) / m["pitch"]))
    hr, ht, d = m["root"] / 2, m["tip"] / 2, m["depth"]
    centres = [m["pitch"] / 2 + m["pitch"] * k for k in range(n)]
    pts = []
    for k, c in enumerate(centres):
        x = m["x0"] + c
        pts += [(x - hr, m["z0"]), (x - ht, m["z0"] + d), (x + ht, m["z0"] + d), (x + hr, m["z0"])]
    pts.append((m["x1"], m["z0"]))
    for c in centres:
        z = m["z0"] + c
        pts += [(m["x1"], z - hr), (m["x1"] + d, z - ht), (m["x1"] + d, z + ht), (m["x1"], z + hr)]
    pts.append((m["x1"], m["z1"]))
    for k, c in enumerate(reversed(centres)):
        x = m["x0"] + c
        if bitten is not None and k == bitten:
            pts += [(x + hr, m["z1"]), (x + hr * 0.6, m["z1"] + d * 0.35), (x - hr * 0.3, m["z1"] + d * 0.5), (x - hr, m["z1"])]
        else:
            pts += [(x + hr, m["z1"]), (x + ht, m["z1"] + d), (x - ht, m["z1"] + d), (x - hr, m["z1"])]
    pts.append((m["x0"], m["z1"]))
    for c in reversed(centres):
        z = m["z0"] + c
        pts += [(m["x0"], z + hr), (m["x0"] + d, z + ht), (m["x0"] + d, z - ht), (m["x0"], z - hr)]
    pts.append((m["x0"], m["z0"]))
    return pts[:-1]


def upright_profile():
    """🪜️ The side profile of one upright: the inclined beech board standing on the runner up to its chamfered top."""
    return [(F(YR1), YR1), (F(YT - 0.012), YT - 0.012), (F(YT) - 0.010, YT), (R(YT) + 0.002, YT), (R(YR1), YR1)]


def runner_profile(feet):
    """🛷️ The side profile of a base runner with its rounded rear end and chamfered front; `feet` "glides" leaves the glide gap, "merged" runs down to the tile."""
    yb = YR0 if feet == "glides" else TM
    rear = [(XR0 + 0.025 - 0.025 * math.cos(math.radians(a)), YR1 - 0.025 + 0.025 * math.sin(math.radians(a))) for a in (90, 60, 30)]
    pts = [(XR0, yb + 0.004), (XR0 + 0.004, yb), (XR1 - 0.008, yb), (XR1, yb + 0.010), (XR1, YR0 + 0.026), (XR1 - 0.028, YR1)]
    pts += [(XR0 + 0.025, YR1)] + rear[1:] + [(XR0, YR1 - 0.025)]
    return pts


def slat_profile(y0, y1, extra=0.0):
    """🪵️ The side profile of a back slat flush with the rear edge of the uprights, `extra` thickening it forward."""
    return [(R(y0), y0), (R(y0) + SLAT_H + extra, y0), (R(y1) + SLAT_H + extra, y1), (R(y1), y1)]


def bow_outline(arm):
    """🧷️ The plan outline (x, z) of the tray bow: a front bar and two arms that clip into the inner faces of the uprights."""
    b = BOW
    x0, x1, zo = b["x0"], b["x1"], b["zo"]
    xi, zi = x1 - b["bar"], zo - arm
    ro, ri = 0.03, 0.01
    pts = [(x0, -zo), (x1 - ro, -zo)] + arc(x1 - ro, -zo + ro, ro, 270, 360, 4)[1:-1] + [(x1, -zo + ro), (x1, zo - ro)] + arc(x1 - ro, zo - ro, ro, 0, 90, 4)[1:-1]
    pts += [(x1 - ro, zo), (x0, zo), (x0, zi), (xi - ri, zi)] + arc(xi - ri, zi - ri, ri, 90, 0, 3)[1:-1] + [(xi, zi - ri), (xi, -zi + ri)] + arc(xi - ri, -zi + ri, ri, 360, 270, 3)[1:-1] + [(xi - ri, -zi), (x0, -zi)]
    return pts


def tray_outline(inset=0.0):
    """🍽️ The plan outline (x, z) of the clip-on polypropylene tray, optionally inset by the wall."""
    t = TRAY
    r = t["r"] - inset
    return rrect(t["x0"] + inset, t["x1"] - inset, -t["z"] + inset, t["z"] - inset, r, r, r, r, 4)


def plate_outline(p, chip=False):
    """🟫️ The plan outline (x, z) of the seat plate or footplate with rounded front corners, the footplate with its chipped front-right corner."""
    pts = rrect(p["x0"], p["x1"], -p["z"], p["z"], 0, p["r"], p["r"], 0, 4)
    if not chip:
        return pts
    out = []
    for q in pts:
        if q[0] > p["x1"] - p["r"] - 1e-9 and q[1] < -p["z"] + p["r"] + 1e-9:
            continue
        if abs(q[0] - p["x0"]) < 1e-9 and abs(q[1] + p["z"]) < 1e-9:
            out.append(q)
            out += [(p["x1"] - 0.028, -p["z"]), (p["x1"] - 0.018, -p["z"] + 0.009), (p["x1"] - 0.006, -p["z"] + 0.012), (p["x1"], -p["z"] + 0.022)]
            continue
        out.append(q)
    return out


BONES = {}
ORDER = []


def bone(id, parent, x, y, angle, length, joint, title, mass=None, section=None):
    """🦴️ A rig bone placed in its parent frame; the rest world frame is kept for the geometry."""
    if parent is None:
        o, a = (x, y), angle
    else:
        p = BONES[parent]
        c, s = math.cos(math.radians(p["a"])), math.sin(math.radians(p["a"]))
        o, a = (p["o"][0] + c * x - s * y, p["o"][1] + s * x + c * y), p["a"] + angle
    BONES[id] = {"id": id, "parent": parent, "x": x, "y": y, "angleDeg": angle, "lengthM": length, "joint": joint, "title": title, "o": o, "a": a, "mass": mass, "section": section}
    ORDER.append(id)


def local(b, p):
    """🧭️ A rest world point (x, y) in the frame (u, v) of bone b."""
    f = BONES[b]
    c, s = math.cos(math.radians(f["a"])), math.sin(math.radians(f["a"]))
    dx, dy = p[0] - f["o"][0], p[1] - f["o"][1]
    return (c * dx + s * dy, -s * dx + c * dy)


def T(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


bone("ground", None, 0, 0, 0, 0, {"type": "fixed"}, T("Floor marker below the middle of the high chair", "Bodenmarke unter der Mitte des Hochstuhls"))
bone("mat", "ground", MAT["x0"], 0, 0, MAT["x1"] - MAT["x0"], {"type": "fixed"}, T("EVA foam puzzle tile under the chair (stays on the floor when the chair is carried)", "EVA-Puzzlematte unter dem Stuhl (bleibt beim Tragen am Boden liegen)"))
bone("lift", "ground", XR0, TM, 0, 0, {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.25}, T("Lift of the chair off the tile when it is carried (rear end of the runners)", "Anheben des Stuhls von der Matte beim Tragen (hinteres Ende der Kufen)"))
bone("chair", "lift", 0, 0, 0, 0, {"type": "revolute", "minDeg": -15, "maxDeg": 15}, T("Tilt of the whole chair about the rear ends of the runners", "Neigen des ganzen Stuhls um die hinteren Kufenenden"))
bone("runner-r", "chair", 0, GL, 0, XR1 - XR0, {"type": "fixed"}, T("Right base runner with its two glides", "Rechte Bodenkufe mit ihren zwei Gleitern"))
bone("runner-l", "chair", 0, GL, 0, XR1 - XR0, {"type": "fixed"}, T("Left base runner, front glide and the felt pad that replaced the lost rear glide", "Linke Bodenkufe, vorderer Gleiter und das Filzpolster anstelle des verlorenen hinteren Gleiters"))
UX = (F(YR0) + R(YR0)) / 2
bone("upright-r", "chair", UX - XR0, GL, 110, (YT - YR0) / math.sin(THETA), {"type": "fixed"}, T("Right upright: the inclined grooved beech board", "Rechte Wange: das geneigte genutete Buchenbrett"))
bone("upright-l", "chair", UX - XR0, GL, 110, (YT - YR0) / math.sin(THETA), {"type": "fixed"}, T("Left upright: the inclined grooved beech board", "Linke Wange: das geneigte genutete Buchenbrett"))
BX = R(SLATS[0][0]) + SLAT_H / 2
bone("backrest", "chair", BX - XR0, SLATS[0][0] - TM, 110, (SLATS[1][1] - SLATS[0][0]) / math.sin(THETA), {"type": "fixed"}, T("Backrest: the two beech slats bolted between the uprights", "Rückenlehne: die zwei zwischen die Wangen geschraubten Buchenleisten"))
RX = (F(ROD["y"]) + R(ROD["y"])) / 2
bone("stretcher", "chair", RX - 0.012 - XR0, ROD["y"] - TM, 0, 0.024, {"type": "fixed"}, T("Round beech stretcher that ties the uprights together low down", "Runde Buchenstrebe, die die Wangen unten verbindet"))
bone("seat", "chair", SEAT["x0"] - XR0, SEAT["y0"] - TM, 0, SEAT["x1"] - SEAT["x0"], {"type": "prismatic", "axisDeg": 110, "minM": -0.14, "maxM": 0}, T("Seat plate in the top groove pair (slides along the uprights to lower grooves)", "Sitzplatte im obersten Nutenpaar (wandert entlang der Wangen in tiefere Nuten)"))
bone("footplate", "chair", FOOT["x0"] - XR0, FOOT["y0"] - TM, 0, FOOT["x1"] - FOOT["x0"], {"type": "prismatic", "axisDeg": 110, "minM": -0.1, "maxM": 0.1}, T("Footplate in its groove pair", "Fußplatte in ihrem Nutenpaar"))
bone("bow-lift", "seat", BOW["x0"] - SEAT["x0"], SEAT["y1"] - SEAT["y0"], 0, 0, {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": BOW_LIFT}, T("Height of the tray bow: lying on the seat or clipped in at tray height", "Höhe des Essbügels: auf dem Sitz liegend oder auf Tischhöhe eingeklipst"))
bone("bow", "bow-lift", 0, 0, 0, BOW["x1"] - BOW["x0"], {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": BOW_SLIDE}, T("Tray bow with the clipped-on tray (slides forward off its clips)", "Essbügel mit aufgeklipstem Tablett (gleitet nach vorn von seinen Clips)"))


class Part:
    """🧱️ One prismatic part of a scale: a side, plan or front profile in rest world metres, extruded across its span, with its bone, material, region and drawing role."""

    def __init__(self, id, bone, kind, profile, a, b, material, region, tone, role="body", holes=None, texture=False, sheet=False, slender=False, draw=("elevation", "front", "plan"), detail=None):
        self.id, self.bone, self.kind, self.profile, self.a, self.b = id, bone, kind, profile, a, b
        self.material, self.region, self.tone, self.role, self.holes = material, region, tone, role, holes or []
        self.texture, self.sheet, self.slender, self.draw, self.detail = texture, sheet, slender, draw, detail or {}

    def volume(self):
        """📦️ The volume and the world centroid of the prism."""
        area, c = area_centroid(self.profile)
        for h in self.holes:
            ha, hc = area_centroid(h)
            c = ((c[0] * area - hc[0] * ha) / (area - ha), (c[1] * area - hc[1] * ha) / (area - ha))
            area -= ha
        depth = self.b - self.a
        mid = (self.a + self.b) / 2
        if self.kind == "side":
            return area * depth, (c[0], c[1], mid)
        if self.kind == "plan":
            return area * depth, (c[0], mid, c[1])
        return area * depth, (mid, c[1], c[0])

    def surface(self):
        """🧴️ The full surface of the prism (both caps and the mantle)."""
        area, _ = area_centroid(self.profile)
        per = perimeter(self.profile)
        for h in self.holes:
            area -= area_centroid(h)[0]
            per += perimeter(h)
        return 2 * area + per * (self.b - self.a)

    def ranges(self):
        """📐️ The world box (x0, x1, y0, y1, z0, z1) of the prism."""
        xs = [p[0] for p in self.profile]
        ys = [p[1] for p in self.profile]
        if self.kind == "side":
            return min(xs), max(xs), min(ys), max(ys), self.a, self.b
        if self.kind == "plan":
            return min(xs), max(xs), self.a, self.b, min(ys), max(ys)
        return self.a, self.b, min(ys), max(ys), min(xs), max(xs)

    def solid(self):
        """🧊️ The volume solid of the prism in the rest frame of its bone."""
        f = BONES[self.bone]
        s = {"id": self.id, "kind": "extrude", "bone": self.bone, "material": self.material, "role": self.role, "tone": self.tone}
        if self.kind == "side":
            s["profile"] = [[rnd(u), rnd(v)] for u, v in (local(self.bone, p) for p in self.profile)]
            if self.holes:
                s["holes"] = [[[rnd(u), rnd(v)] for u, v in (local(self.bone, p) for p in h)] for h in self.holes]
            s.update({"plane": "xy", "from": rnd(self.a), "to": rnd(self.b)})
        elif self.kind == "plan":
            assert abs(f["a"]) < 1e-9
            s["profile"] = [[rnd(z), rnd(x - f["o"][0])] for x, z in self.profile]
            if self.holes:
                s["holes"] = [[[rnd(z), rnd(x - f["o"][0])] for x, z in h] for h in self.holes]
            s.update({"plane": "zx", "from": rnd(self.a - f["o"][1]), "to": rnd(self.b - f["o"][1])})
        else:
            assert abs(f["a"]) < 1e-9
            s["profile"] = [[rnd(y - f["o"][1]), rnd(z)] for z, y in self.profile]
            s.update({"plane": "yz", "from": rnd(self.a - f["o"][0]), "to": rnd(self.b - f["o"][0])})
        if self.texture:
            s["texture"] = True
        if self.sheet:
            s["sheet"] = True
        if self.slender:
            s["slender"] = True
        return s

    def shapes(self, view, z):
        """✏️ The filled 2D shape of the prism in one view: elevation in bone-local metres, front (z, y) and plan (x, z) in rest world metres."""
        x0, x1, y0, y1, z0, z1 = self.ranges()
        base = {"id": self.id, "bone": self.bone, "role": self.role, "z": z, "filled": True, "tone": self.tone, "region": self.region, "material": self.material}
        if view == "elevation":
            if self.kind == "side":
                pts = [local(self.bone, p) for p in self.profile]
            else:
                pts = [local(self.bone, p) for p in [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]]
            return dict(base, kind="poly", points=[[rnd(u), rnd(v)] for u, v in pts], closed=True)
        if view == "front":
            if self.kind == "front":
                pts = self.profile
            else:
                pts = [(z0, y0), (z1, y0), (z1, y1), (z0, y1)]
            return dict(base, kind="poly", points=[[rnd(a), rnd(b)] for a, b in pts], closed=True)
        if self.kind == "plan":
            if self.holes:
                d = " ".join("M " + " L ".join(f"{rnd(x)} {rnd(zz)}" for x, zz in ring) + " Z" for ring in [self.profile] + self.holes)
                return dict(base, kind="path", d=d)
            pts = self.profile
        else:
            pts = [(x0, z0), (x1, z0), (x1, z1), (x0, z1)]
        return dict(base, kind="poly", points=[[rnd(a), rnd(b)] for a, b in pts], closed=True)


def side_parts(scale):
    """🪜️ The side frames of one scale: runners with glides or merged feet, the inclined uprights and their bolt caps."""
    zt = {50: 0.0, 100: 0.0, 200: 0.0015}[scale]
    zi, zo = ZI - zt, ZO + zt
    feet = "glides" if scale == 50 else "merged"
    out = []
    for side, sgn in (("r", -1), ("l", 1)):
        za, zb = sorted((sgn * zi, sgn * zo))
        out.append(Part(f"runner-{side}", f"runner-{side}", "side", runner_profile(feet), za, zb, "beech-lacquered", "frame", 0.68))
        out.append(Part(f"upright-{side}", f"upright-{side}", "side", upright_profile(), za, zb, "beech-lacquered", "frame", 0.74))
        if scale == 50:
            out.append(Part(f"glide-f{side}", f"runner-{side}", "side", [(XR1 - 0.07, TM), (XR1 - 0.012, TM), (XR1 - 0.012, YR0), (XR1 - 0.07, YR0)], za, zb, "pp-glide", "frame", 0.18))
            if side == "r":
                out.append(Part("glide-rr", "runner-r", "side", [(XR0 + 0.01, TM), (XR0 + 0.068, TM), (XR0 + 0.068, YR0), (XR0 + 0.01, YR0)], za, zb, "pp-glide", "frame", 0.18))
            else:
                out.append(Part("felt-rl", "runner-l", "plan", circle(XR0 + 0.04, sgn * (ZI + ZO) / 2, 0.02, 16), TM, YR0, "wool-felt", "frame", 0.55))
            caps = [((R(y0) + R(y1)) / 2 + SLAT_H / 2, (y0 + y1) / 2) for y0, y1 in SLATS] + [(RX, ROD["y"])]
            for k, (cx, cy) in enumerate(caps):
                za2, zb2 = sorted((sgn * ZO, sgn * (ZO + 0.003)))
                out.append(Part(f"cap-{side}-{k}", f"upright-{side}", "side", circle(cx, cy, 0.007, 12), za2, zb2, "zinc-steel", "frame", 0.25, role="detail", texture=True, draw=("elevation",) if side == "r" else ()))
    return out


def inner_parts(scale):
    """🪑️ The parts between the uprights at one scale: back slats, stretcher, seat plate, footplate, tray bow, tray and the floor tile."""
    out = []
    zs = 0.21
    if scale in (50, 100):
        for k, (y0, y1) in enumerate(SLATS):
            out.append(Part(f"slat-{k}", "backrest", "side", slat_profile(y0, y1), -zs, zs, "beech-lacquered", "frame", 0.72, draw=("front", "plan")))
        out.append(Part("stretcher", "stretcher", "side", circle(RX, ROD["y"], ROD["r"], 16), -ROD["z"], ROD["z"], "beech-lacquered", "frame", 0.66, draw=("front", "plan")))
    else:
        out.append(Part("backrest-band", "backrest", "side", slat_profile(SLATS[0][0], SLATS[1][1], 0.032 / math.sin(THETA) - SLAT_H), -zs, zs, "beech-lacquered", "frame", 0.72, draw=("front", "plan")))
        out.append(Part("stretcher", "stretcher", "side", circle(RX, ROD["y"], 0.0155, 16), -ROD["z"], ROD["z"], "beech-lacquered", "frame", 0.66, draw=("front", "plan")))
    th = 0.0 if scale != 200 else 0.012
    out.append(Part("seat-plate", "seat", "plan", plate_outline(SEAT), SEAT["y0"] - th, SEAT["y1"], "beech-plywood", "seat-plate", 0.62))
    out.append(Part("footplate", "footplate", "plan", plate_outline(FOOT, chip=scale == 50), FOOT["y0"] - th, FOOT["y1"], "beech-plywood", "footplate", 0.6))
    arm = BOW["arm"] if scale != 200 else 0.03
    out.append(Part("bow", "bow", "plan", bow_outline(arm), BOW["y0"], BOW["y1"], "beech-lacquered", "bow", 0.7))
    if scale == 50:
        out.append(Part("tray-rim", "bow", "plan", tray_outline(), TRAY["y0"], TRAY["y1"], "polypropylene", "tray", 0.9, holes=[tray_outline(TRAY["wall"])]))
        out.append(Part("tray-floor", "bow", "plan", tray_outline(TRAY["wall"]), TRAY["y0"], TRAY["y0"] + 0.003, "polypropylene", "tray", 0.86, sheet=True, draw=()))
        out.append(Part("tile", "mat", "plan", mat_outline(True, bitten=6), 0.0, TM, "eva-foam", "mat", 0.42))
    else:
        out.append(Part("tray", "bow", "plan", tray_outline(), TRAY["y0"], TRAY["y1"], "polypropylene", "tray", 0.9))
        out.append(Part("tile", "mat", "plan", mat_outline(False), 0.0, TM, "eva-foam", "mat", 0.42, sheet=True))
    return out


def parts_of(scale):
    """🗂️ Every part of one scale in a fixed order."""
    return side_parts(scale) + inner_parts(scale)


def depth_rank(parts, view):
    """🎚️ Painting rank of the parts in one view: the nearer part is painted later."""
    def key(p):
        x0, x1, y0, y1, z0, z1 = p.ranges()
        if view == "elevation":
            return -z0
        if view == "front":
            return x1
        return y1
    ranked = sorted([p for p in parts if view in p.draw], key=key)
    return {p.id: 1 + k for k, p in enumerate(ranked)}


def poly(id, bone, role, z, pts, closed, pen=None, region=None, material=None, tone=0.3, filled=False):
    """🖊️ A hand-placed open or closed polyline of one view."""
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": "poly", "points": [[rnd(a), rnd(b)] for a, b in pts], "closed": closed}
    if region:
        s["region"] = region
    if material:
        s["material"] = material
    if pen:
        s["penMm"] = pen
    return s


def ellipse(id, bone, role, z, cx, cy, rx, ry, pen=None, region=None, tone=0.3):
    """🥚️ A hand-placed unfilled ellipse of one view."""
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": False, "tone": tone, "kind": "ellipse", "cx": rnd(cx), "cy": rnd(cy), "rx": rnd(rx), "ry": rnd(ry)}
    if region:
        s["region"] = region
    if pen:
        s["penMm"] = pen
    return s


def details(scale):
    """🔍️ The detail and texture strokes of one scale: grooves seen at the plate ends, bolt caps, the sticker, stains, rings, scratches, teeth marks and the tray rim."""
    E, Fr, P = [], [], []
    if scale == 200:
        P.append(poly("tray-rim-line", "bow", "detail", 90, tray_outline(0.012) + [tray_outline(0.012)[0]], False, region="tray"))
        return E, Fr, P
    lb = lambda b, pts: [local(b, p) for p in pts]
    if scale == 50:
        for k, (y0, y1) in enumerate(SLATS):
            cx, cy = (R(y0) + R(y1)) / 2 + SLAT_H / 2, (y0 + y1) / 2
            u, v = local("upright-r", (cx, cy))
            E.append(ellipse(f"cap-ring-{k}", "upright-r", "detail", 95, u, v, 0.007, 0.007, region="frame"))
        u, v = local("upright-r", (RX, ROD["y"]))
        E.append(ellipse("cap-ring-2", "upright-r", "detail", 95, u, v, 0.007, 0.007, region="frame"))
        sx, sy = (F(0.34) + R(0.34)) / 2, 0.34
        E.append(poly("sticker", "upright-r", "texture", 96, lb("upright-r", [(sx - 0.022, sy - 0.018), (sx + 0.018, sy - 0.022), (sx + 0.022, sy + 0.016), (sx - 0.008, sy + 0.02), (sx - 0.02, sy + 0.012)]), True, region="frame"))
        E.append(poly("sticker-torn", "upright-r", "texture", 96, lb("upright-r", [(sx - 0.008, sy + 0.02), (sx + 0.002, sy + 0.004), (sx + 0.022, sy + 0.016)]), False, region="frame"))
        for k, y in enumerate((0.15, 0.26, 0.40, 0.55, 0.68)):
            a, b = (F(y) - 0.006, y), (R(y) + 0.006, y + 0.012)
            E.append(poly(f"wear-{k}", "upright-r", "texture", 96, lb("upright-r", [a, (a[0] - 0.012, a[1] + 0.006)]), False, region="frame"))
        E.append(poly("runner-scuff", "runner-r", "texture", 96, lb("runner-r", [(XR1 - 0.05, 0.03), (XR1 - 0.02, 0.034), (XR1 - 0.035, 0.05)]), False, region="frame"))
        E.append(poly("footplate-edge", "footplate", "detail", 97, lb("footplate", [(FOOT["x1"] - 0.03, FOOT["y1"] - 0.004), (FOOT["x1"] - 0.004, FOOT["y1"] - 0.004)]), False, region="footplate"))
        E.append(poly("tray-lip", "bow", "detail", 97, lb("bow", [(TRAY["x0"], TRAY["y1"] - 0.006), (TRAY["x1"], TRAY["y1"] - 0.006)]), False, region="tray"))
        zb = BOW["zo"]
        Fr.append(poly("tray-wall-l", "bow", "detail", 90, [(TRAY["z"] - TRAY["wall"], TRAY["y0"] + 0.003), (TRAY["z"] - TRAY["wall"], TRAY["y1"])], False, region="tray"))
        Fr.append(poly("tray-wall-r", "bow", "detail", 90, [(-TRAY["z"] + TRAY["wall"], TRAY["y0"] + 0.003), (-TRAY["z"] + TRAY["wall"], TRAY["y1"])], False, region="tray"))
        for k, zz in enumerate((-0.04, -0.028, -0.015, 0.012, 0.03)):
            Fr.append(poly(f"teeth-{k}", "bow", "texture", 91, [(zz, BOW["y1"] - 0.004), (zz + 0.004, BOW["y1"] - 0.011)], False, region="bow"))
        Fr.append(poly("slat-grain-0", "backrest", "texture", 60, [(-0.18, 0.59), (-0.05, 0.592), (0.08, 0.588), (0.18, 0.59)], False, region="frame"))
        Fr.append(poly("slat-grain-1", "backrest", "texture", 60, [(-0.18, 0.735), (-0.02, 0.733), (0.12, 0.737), (0.18, 0.735)], False, region="frame"))
        for k, (cx, cz, rx, rz) in enumerate([(0.11, 0.07, 0.012, 0.009), (0.06, -0.09, 0.016, 0.01), (0.17, -0.02, 0.009, 0.013), (0.03, 0.13, 0.01, 0.007)]):
            P.append(ellipse(f"stain-{k}", "bow", "texture", 92, cx, cz, rx, rz, region="tray"))
        P.append(ellipse("cup-ring", "bow", "detail", 92, 0.16, 0.09, 0.034, 0.034, region="tray"))
        P.append(poly("tray-scratch-0", "bow", "texture", 92, [(0.02, -0.15), (0.09, -0.12)], False, region="tray"))
        P.append(poly("tray-scratch-1", "bow", "texture", 92, [(0.12, -0.16), (0.2, -0.13), (0.21, -0.11)], False, region="tray"))
        P.append(poly("tray-inner", "bow", "detail", 91, tray_outline(TRAY["wall"]) + [tray_outline(TRAY["wall"])[0]], False, region="tray"))
        for k, (cx, cz) in enumerate([(0.02, 0.15), (0.16, -0.17), (0.09, 0.17)]):
            P.append(ellipse(f"seat-stain-{k}", "seat", "texture", 40, cx, cz, 0.011, 0.008, region="seat-plate"))
        P.append(poly("footplate-wear", "footplate", "texture", 30, [(0.17, -0.12), (0.19, -0.02), (0.17, 0.08), (0.185, 0.15)], False, region="footplate"))
        P.append(poly("tile-grain-0", "mat", "texture", 2, [(-0.29, -0.27), (-0.29, 0.27)], False, region="mat"))
        P.append(poly("tile-grain-1", "mat", "texture", 2, [(0.27, -0.27), (0.27, 0.27)], False, region="mat"))
        P.append(poly("tile-dent-0", "mat", "texture", 2, [(XR1 - 0.06, -0.25), (XR1 - 0.02, -0.25)], False, region="mat"))
        P.append(poly("tile-dent-1", "mat", "texture", 2, [(XR0 + 0.02, 0.25), (XR0 + 0.06, 0.25)], False, region="mat"))
    else:
        u, v = local("upright-r", ((F(0.34) + R(0.34)) / 2, 0.34))
        E.append(poly("tray-lip", "bow", "detail", 97, lb("bow", [(TRAY["x0"], TRAY["y1"] - 0.008), (TRAY["x1"], TRAY["y1"] - 0.008)]), False, region="tray"))
        for k, (cx, cz, rx, rz) in enumerate([(0.11, 0.07, 0.014, 0.011), (0.06, -0.09, 0.018, 0.012)]):
            P.append(ellipse(f"stain-{k}", "bow", "texture", 92, cx, cz, rx, rz, region="tray"))
        P.append(ellipse("cup-ring", "bow", "detail", 92, 0.16, 0.09, 0.034, 0.034, region="tray"))
        P.append(poly("tray-inner", "bow", "detail", 91, tray_outline(0.008) + [tray_outline(0.008)[0]], False, region="tray"))
    return E, Fr, P


PENS = {50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.7}, 100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5}, 200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350)}
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03}


def geometry(scale):
    """🗺️ The three views of one scale: the filled parts in painting order and the hand-placed details."""
    parts = parts_of(scale)
    g = {}
    for view in ("elevation", "front", "plan"):
        rank = depth_rank(parts, view)
        shapes = [p.shapes(view, rank[p.id]) for p in parts if view in p.draw]
        shapes.sort(key=lambda s: s["z"])
        g[view] = shapes
    E, Fr, P = details(scale)
    if scale == 50:
        for s in g["elevation"]:
            if s["id"] in ("upright-r", "runner-r", "upright-l", "runner-l"):
                s["hatch"] = "beech-grain"
    g["elevation"] += E
    g["front"] += Fr
    g["plan"] += P
    return g


def decisions(scale):
    """📝️ The authored drawing decisions of one scale."""
    if scale == 50:
        return {
            "silhouette": T("The true views of the 1:50 model, 12.2 by 16.0 mm: on the floor tile the two inclined beech uprights (70 degrees, 75 by 27 mm) stand on their runners with two glides each (the lost rear-left glide replaced by a felt pad), the seat plate and the footplate sit in their grooves, the two back slats are bolted between the uprights at the rear edge, the round stretcher ties the uprights low down, and in the stored state the tray bow with its clipped-on tray lies on the seat; the front shows the slats, plates, stretcher, bow and tray between the uprights, the plan the toothed EVA tile under everything.", "Die wahren Ansichten des 1:50-Modells, 12,2 mal 16,0 mm: auf der Bodenmatte stehen die zwei geneigten Buchenwangen (70 Grad, 75 mal 27 mm) auf ihren Kufen mit je zwei Gleitern (der verlorene hintere linke Gleiter durch ein Filzpolster ersetzt), Sitz- und Fußplatte stecken in ihren Nuten, die zwei Rückenleisten sind an der Hinterkante zwischen die Wangen geschraubt, die runde Strebe verbindet die Wangen unten, und im Lagerzustand liegt der Essbügel mit aufgeklipstem Tablett auf dem Sitz; die Vorderansicht zeigt Leisten, Platten, Strebe, Bügel und Tablett zwischen den Wangen, die Draufsicht die gezahnte EVA-Matte unter allem."),
            "retained": T("Every part of the real chair: runners, glides and felt pad, uprights, slats, stretcher, both plates, bow, tray rim and the puzzle teeth of the tile; traces of use as geometry where they exceed 7.5 mm: the chipped front-right corner of the footplate and the bitten-off tooth of the tile.", "Jedes Teil des echten Stuhls: Kufen, Gleiter und Filzpolster, Wangen, Leisten, Strebe, beide Platten, Bügel, Tablettrand und die Puzzlezähne der Matte; Gebrauchsspuren als Geometrie, wo sie 7,5 mm überschreiten: die abgeplatzte vordere rechte Ecke der Fußplatte und der abgebissene Zahn der Matte."),
            "merged": T("Nothing is merged; the left side frame lies exactly behind the right one in the side elevation and is drawn below it.", "Nichts ist zusammengefasst; die linke Seitenwange liegt in der Seitenansicht genau hinter der rechten und ist darunter gezeichnet."),
            "omitted": T("The grooves on the inner faces of the uprights are hidden by the plates and are not drawn; the slats and the stretcher lie behind the right upright in the side elevation and are drawn only in front and plan.", "Die Nuten auf den Innenseiten der Wangen sind durch die Platten verdeckt und nicht gezeichnet; Leisten und Strebe liegen in der Seitenansicht hinter der rechten Wange und erscheinen nur in Vorderansicht und Draufsicht."),
            "thinElements": T("The 3 mm tray wall is drawn as the inner rim line; the 0.6 mm zinc bolt caps are detail circles on the outer face of the right upright.", "Die 3-mm-Tablettwand ist als innere Randlinie gezeichnet; die verzinkten Schraubkappen sind Detailkreise auf der Außenseite der rechten Wange."),
            "hatching": T("A line hatch in the direction of the grain marks the beech of runners and uprights in the side elevation.", "Eine Linienschraffur in Faserrichtung kennzeichnet das Buchenholz von Kufen und Wangen in der Seitenansicht."),
            "styles": T("Body pen 0.5 mm, detail 0.25 mm for bolt caps, rims and edges, texture 0.13 mm for the worn lacquer, the half-scratched sticker, food stains, scratches and the teeth marks on the bow.", "Körperstift 0,5 mm, Detail 0,25 mm für Schraubkappen, Ränder und Kanten, Textur 0,13 mm für den abgegriffenen Lack, den halb abgekratzten Aufkleber, Essensflecken, Kratzer und die Zahnabdrücke am Bügel."),
            "overlays": T("Thermal and structural overlays bind to the frame, plates, bow, tray and tile regions; the occupant loads of the toddler sit on the seat and footplate anchors.", "Thermische und statische Überlagerungen binden an die Regionen Gestell, Platten, Bügel, Tablett und Matte; die Lasten des Kleinkinds sitzen an den Ankern von Sitz und Fußplatte."),
        }
    if scale == 100:
        return {
            "silhouette": T("The chair at 6.1 by 8.0 mm: side frames, slats, stretcher, plates, bow and tray as at 1:50, the glides merged into the runners down to the tile and the toothed tile edge generalised to its mean line.", "Der Stuhl mit 6,1 mal 8,0 mm: Seitenwangen, Leisten, Strebe, Platten, Bügel und Tablett wie bei 1:50, die Gleiter in die Kufen bis zur Matte einbezogen und der gezahnte Mattenrand auf seine Mittellinie verallgemeinert."),
            "retained": T("Both slats with their 51 mm gap, the stretcher, the 18 mm plates and the bow arms stay separate parts.", "Beide Leisten mit ihrem Abstand von 51 mm, die Strebe, die 18-mm-Platten und die Bügelarme bleiben getrennte Teile."),
            "merged": T("Glides and the felt pad into the runners; tray rim and floor into one tray body.", "Gleiter und Filzpolster in die Kufen; Tablettrand und -boden zu einem Tablettkörper."),
            "omitted": T("Bolt caps, the sticker, the wear marks on the uprights, the teeth marks, the footplate chip and the puzzle teeth (all below 15 mm).", "Schraubkappen, Aufkleber, Abriebspuren an den Wangen, Zahnabdrücke, die Absplitterung der Fußplatte und die Puzzlezähne (alle unter 15 mm)."),
            "thinElements": T("The tray rim is an inner detail line 8 mm inside the edge.", "Der Tablettrand ist eine innere Detaillinie 8 mm innerhalb der Kante."),
            "hatching": T("No hatch at 1:100.", "Keine Schraffur bei 1:100."),
            "styles": T("Body pen 0.35 mm, detail 0.18 mm, texture 0.13 mm for two food stains on the tray.", "Körperstift 0,35 mm, Detail 0,18 mm, Textur 0,13 mm für zwei Essensflecken auf dem Tablett."),
            "overlays": T("As at 1:50.", "Wie bei 1:50."),
        }
    return {
        "silhouette": T("The chair at 3.1 by 4.0 mm: side frames, the backrest as one band, seat, footplate, bow and tray, the tile under it; plates, frames and bow thickened to the 30 mm feature.", "Der Stuhl mit 3,1 mal 4,0 mm: Seitenwangen, die Rückenlehne als ein Band, Sitz, Fußplatte, Bügel und Tablett, darunter die Matte; Platten, Wangen und Bügel auf das Merkmal von 30 mm verstärkt."),
        "retained": T("The inclined uprights, runners, both plates at their groove heights, the bow arms and the tray.", "Die geneigten Wangen, Kufen, beide Platten auf ihrer Nutenhöhe, die Bügelarme und das Tablett."),
        "merged": T("The two slats and their gap into one backrest band from 0.565 to 0.776 m; glides into the runners.", "Die zwei Leisten mit ihrem Abstand zu einem Lehnenband von 0,565 bis 0,776 m; Gleiter in die Kufen."),
        "omitted": T("Every texture and the puzzle teeth; the 25 mm stretcher is drawn at the 31 mm the scale can print.", "Jede Textur und die Puzzlezähne; die 25-mm-Strebe ist mit den 31 mm gezeichnet, die der Maßstab drucken kann."),
        "thinElements": T("The 12 mm tile is a declared sheet; the tray rim is one detail line.", "Die 12-mm-Matte ist ein erklärtes dünnes Blatt; der Tablettrand ist eine Detaillinie."),
        "hatching": T("No hatch at 1:200.", "Keine Schraffur bei 1:200."),
        "styles": T("Body pen 0.25 mm, detail 0.13 mm, no texture pen.", "Körperstift 0,25 mm, Detail 0,13 mm, kein Texturstift."),
        "overlays": T("As at 1:50, on the generalised parts.", "Wie bei 1:50, an den verallgemeinerten Teilen."),
    }


def representation(scale):
    """🖼️ The authored representation of one scale."""
    rep = {"status": "authored", "denominator": scale, "validRange": {"minDenominator": RANGES[scale][0], "maxDenominator": RANGES[scale][1]}, "pens": PENS[scale], "hatches": [], "geometry": geometry(scale), "decisions": decisions(scale)}
    if scale == 50:
        rep["hatches"] = [{"id": "beech-grain", "title": T("Beech grain along the boards", "Buchenmaserung entlang der Bretter"), "pattern": "lines", "angleDeg": 110, "spacingMm": 0.8, "penMm": 0.13}]
    return rep


MASS_PARTS = {}


def bone_masses():
    """⚖️ Mass and centre of mass of every bone from the volumes and densities of its 1:50 parts (texture caps included as 12 mm M6 bolts)."""
    out = {}
    for p in parts_of(50):
        v, c = p.volume()
        m = v * RHO[p.material]
        if p.texture:
            m = 0.006
        e = out.setdefault(p.bone, [0.0, 0.0, 0.0, 0.0])
        e[0] += m
        e[1] += m * c[0]
        e[2] += m * c[1]
        e[3] += m * c[2]
    return {b: (e[0], (e[1] / e[0], e[2] / e[0], e[3] / e[0])) for b, e in out.items()}


MASSES = bone_masses()


def rect_section(w, h, material, en, de):
    """🎡️ A rectangular member section: area, second moment about the bending axis and extreme fibre."""
    return {"areaM2": float(f"{w * h:.6g}"), "secondMomentM4": float(f"{w * h ** 3 / 12:.6g}"), "extremeFibreM": rnd(h / 2), "material": material, "title": T(en, de)}


SECTIONS = {
    "runner-r": rect_section(0.027, 0.05, "beech-lacquered", "Runner 27 x 50 mm beech", "Kufe 27 x 50 mm Buche"),
    "runner-l": rect_section(0.027, 0.05, "beech-lacquered", "Runner 27 x 50 mm beech", "Kufe 27 x 50 mm Buche"),
    "upright-r": rect_section(0.027, 0.075, "beech-lacquered", "Upright 27 x 75 mm beech", "Wange 27 x 75 mm Buche"),
    "upright-l": rect_section(0.027, 0.075, "beech-lacquered", "Upright 27 x 75 mm beech", "Wange 27 x 75 mm Buche"),
    "backrest": rect_section(0.42, 0.018, "beech-lacquered", "Back slat 420 x 18 mm beech (bending of the slat plane)", "Rückenleiste 420 x 18 mm Buche (Biegung der Leistenebene)"),
    "seat": rect_section(0.42, 0.018, "beech-plywood", "Seat plate 420 x 18 mm beech plywood", "Sitzplatte 420 x 18 mm Buchensperrholz"),
    "footplate": rect_section(0.42, 0.018, "beech-plywood", "Footplate 420 x 18 mm beech plywood", "Fußplatte 420 x 18 mm Buchensperrholz"),
    "bow": rect_section(0.05, 0.03, "beech-lacquered", "Two bow arms 25 x 30 mm beech", "Zwei Bügelarme 25 x 30 mm Buche"),
    "mat": rect_section(0.6, 0.012, "eva-foam", "EVA tile 600 x 12 mm", "EVA-Matte 600 x 12 mm"),
}
SECTIONS["stretcher"] = {"areaM2": float(f"{math.pi * 0.0125 ** 2:.6g}"), "secondMomentM4": float(f"{math.pi * 0.025 ** 4 / 64:.6g}"), "extremeFibreM": 0.0125, "material": "beech-lacquered", "title": T("Round stretcher 25 mm beech", "Runde Strebe 25 mm Buche")}

MASS_REF = {"mat": "eva-foam-tile", "bow": "pp-and-beech", "seat": "iso-10456-2007", "footplate": "iso-10456-2007"}


def rig():
    """🌳️ The rig with the masses of the 1:50 parts and the member sections."""
    bones = []
    for id in ORDER:
        b = BONES[id]
        out = {k: b[k] for k in ("id", "parent", "title")}
        out.update({"x": rnd(b["x"]), "y": rnd(b["y"]), "angleDeg": b["angleDeg"], "lengthM": rnd(b["lengthM"]), "joint": b["joint"]})
        if id in MASSES:
            m, c = MASSES[id]
            u, v = local(id, (c[0], c[1]))
            out["mass"] = {"kg": rnd(m), "comX": rnd(u), "comY": rnd(v), "distribution": "point", "source": "authored-estimate", "reference": "iso-10456-2007"}
        if id in SECTIONS:
            out["section"] = SECTIONS[id]
        bones.append(out)
    return {"kind": "mechanical", "root": "ground", "bones": bones}


def anchor(id, bone, p, kind, en, de, z=0.0, **extra):
    """📍️ An anchor at a rest world point on a bone."""
    u, v = local(bone, p)
    a = {"id": id, "bone": bone, "x": rnd(u), "y": rnd(v), "z": rnd(z), "kind": kind, "title": T(en, de)}
    a.update(extra)
    return a


ZC = (ZI + ZO) / 2
FLOOR = [("floor-fr", XR1 - 0.041, -ZC, "Floor under the front right glide", "Boden unter dem vorderen rechten Gleiter"), ("floor-fl", XR1 - 0.041, ZC, "Floor under the front left glide", "Boden unter dem vorderen linken Gleiter"), ("floor-rr", XR0 + 0.039, -ZC, "Floor under the rear right glide", "Boden unter dem hinteren rechten Gleiter"), ("floor-rl", XR0 + 0.04, ZC, "Floor under the felt pad at the rear left", "Boden unter dem Filzpolster hinten links")]
GRIPS = ["grip-slat", "grip-seat"]


def anchors():
    """🪝️ Floor contacts under the tile, the two carrying grips and the load points of toddler and bowl."""
    out = [anchor(i, "mat", (x, 0.0), "contact", en, de, z=z, support="contact", normalDeg=90, footprintLengthM=0.08, footprintWidthM=0.05, frictionCoefficient=0.6) for i, x, z, en, de in FLOOR]
    out.append(anchor("grip-slat", "backrest", (R(SLATS[1][1]) + SLAT_H / 2, SLATS[1][1]), "support", "Carrying hand over the top edge of the upper back slat", "Tragende Hand über der Oberkante der oberen Rückenleiste", support="pin"))
    out.append(anchor("grip-seat", "seat", (SEAT["x1"] - 0.02, SEAT["y0"]), "support", "Carrying hand under the front edge of the seat plate", "Tragende Hand unter der Vorderkante der Sitzplatte", support="pin"))
    out.append(anchor("seat-top", "seat", (-0.02, SEAT["y1"]), "load", "Sitting point of the toddler on the seat plate", "Sitzpunkt des Kleinkinds auf der Sitzplatte"))
    out.append(anchor("feet-top", "footplate", (0.11, FOOT["y1"]), "load", "Feet of the toddler on the footplate", "Füße des Kleinkinds auf der Fußplatte"))
    out.append(anchor("tray-top", "bow", (0.14, TRAY["y0"] + 0.003), "load", "Bowl on the tray", "Schale auf dem Tablett"))
    return out


REGIONS = ["frame", "seat-plate", "footplate", "bow", "tray", "mat"]


def binding(seat=None, foot=None, tray=None, bow=None, mat_floor=True, seat_cover=0.0, carried=False):
    """🌡️ The thermal binding of a pose: every region with its exposure and conductive contact."""
    r = {k: {"exposedFraction": 1, "sunlitFraction": 0} for k in REGIONS}
    if seat_cover:
        r["seat-plate"] = {"exposedFraction": rnd(1 - seat_cover), "sunlitFraction": 0, "contact": {"fraction": rnd(seat_cover), "tempC": 21}}
        r["bow"] = {"exposedFraction": 0.86, "sunlitFraction": 0, "contact": {"fraction": 0.14, "tempC": 21}}
    if seat:
        r["seat-plate"] = {"exposedFraction": rnd(1 - seat), "sunlitFraction": 0, "contact": {"fraction": seat, "tempC": 31}}
    if foot:
        r["footplate"] = {"exposedFraction": rnd(1 - foot), "sunlitFraction": 0, "contact": {"fraction": foot, "tempC": 29}}
    if tray:
        r["tray"] = {"exposedFraction": rnd(1 - tray), "sunlitFraction": 0, "contact": {"fraction": tray, "tempC": 32}}
    if bow:
        r["bow"] = {"exposedFraction": rnd(1 - bow), "sunlitFraction": 0, "contact": {"fraction": bow, "tempC": 31}}
    r["mat"] = {"exposedFraction": 0.5, "sunlitFraction": 0, "contact": {"fraction": 0.5, "tempC": 20}}
    if carried:
        r["frame"] = {"exposedFraction": 0.97, "sunlitFraction": 0, "contact": {"fraction": 0.03, "tempC": 32}}
    return {"regions": r, "emitters": {}}


G = 9.80665
TODDLER = 12.0
LOADS_TODDLER = [
    {"id": "toddler-seat", "kind": "occupant", "title": T("Toddler on the seat plate (85 % of 12 kg)", "Kleinkind auf der Sitzplatte (85 % von 12 kg)"), "anchor": "seat-top", "fxN": 0, "fyN": rnd(-0.85 * TODDLER * G), "source": "authored-estimate", "reference": "who-growth-2006"},
    {"id": "toddler-feet", "kind": "occupant", "title": T("Toddler's feet on the footplate (15 % of 12 kg)", "Füße des Kleinkinds auf der Fußplatte (15 % von 12 kg)"), "anchor": "feet-top", "fxN": 0, "fyN": rnd(-0.15 * TODDLER * G), "source": "authored-estimate", "reference": "who-growth-2006"},
]
LOAD_BOWL = {"id": "bowl", "kind": "equipment", "title": T("Plastic bowl of porridge on the tray (0.25 kg)", "Kunststoffschale mit Brei auf dem Tablett (0,25 kg)"), "anchor": "tray-top", "fxN": 0, "fyN": rnd(-0.25 * G), "source": "authored-estimate", "reference": "en-14988"}
FLOOR_IDS = [f[0] for f in FLOOR]
SLIDE = (rnd(BOW_SLIDE), 0)


def pose(id, en_t, de_t, en_d, de_d, bones, contacts, thermal, loads, key=False):
    """🧍️ A pose of the chair with authored ground (the tile never leaves the floor)."""
    p = {"id": id, "title": T(en_t, de_t), "description": T(en_d, de_d)}
    if key:
        p["key"] = True
    p.update({"bones": bones, "ground": "authored", "contacts": contacts, "thermal": thermal, "loads": loads})
    return p


def poses():
    """🎭️ The named and key poses: stored, carried, ready, in use and the in-between keys of the clips."""
    up = {"bow-lift": {"dy": BOW_LIFT}}
    out_ = {"bow-lift": {"dy": BOW_LIFT}, "bow": {"dx": BOW_SLIDE}}
    carry = {"lift": {"dy": CARRY_LIFT}, "chair": {"angleDeg": CARRY_TILT}}
    return [
        pose("stored", "Stored at the reuse depot", "Gelagert im Gebrauchtwarenlager", "Standing on the EVA puzzle tile it came with from the flat clearance, the tray bow unclipped and laid on the seat with the stained tray on top.", "Auf der EVA-Puzzlematte, mit der er aus der Haushaltsauflösung kam, der Essbügel ausgeklipst und mit dem fleckigen Tablett obenauf auf den Sitz gelegt.", {}, FLOOR_IDS, binding(seat_cover=0.3), []),
        pose("carried", "Carried by one person", "Von einer Person getragen", "Lifted 0.16 m off the tile that stays on the floor, one hand over the top back slat and the other under the front edge of the seat, tilted back 8 degrees so the loose bow stays on the seat.", "0,16 m von der Matte gehoben, die am Boden bleibt, eine Hand über der oberen Rückenleiste, die andere unter der Vorderkante des Sitzes, 8 Grad nach hinten geneigt, damit der lose Bügel auf dem Sitz bleibt.", carry, GRIPS, binding(seat_cover=0.3, carried=True), []),
        pose("ready", "Ready at the table", "Bereit am Tisch", "Back in a kitchen on its tile, the bow clipped in at tray height, waiting for the child.", "Wieder in einer Küche auf seiner Matte, der Bügel auf Tablethöhe eingeklipst, bereit für das Kind.", up, FLOOR_IDS, binding(), []),
        pose("in-use", "In use with a toddler", "In Gebrauch mit einem Kleinkind", "A toddler of 12 kg sits on the seat plate with the feet on the footplate, the bow clipped in and a bowl of porridge on the tray.", "Ein Kleinkind von 12 kg sitzt auf der Sitzplatte, die Füße auf der Fußplatte, der Bügel eingeklipst und eine Schale Brei auf dem Tablett.", up, FLOOR_IDS, binding(seat=0.3, foot=0.15, tray=0.12, bow=0.1), LOADS_TODDLER + [LOAD_BOWL]),
        pose("bow-out-key", "Bow slid forward (key)", "Bügel nach vorn gezogen (Schlüssel)", "The bow is pulled 0.10 m forward off its clips, held by the parent's hand.", "Der Bügel ist 0,10 m nach vorn von seinen Clips gezogen und wird von der Hand der Eltern gehalten.", out_, FLOOR_IDS, binding(), [], key=True),
        pose("seated-bow-out-key", "Toddler seated, bow still out (key)", "Kleinkind sitzt, Bügel noch vorn (Schlüssel)", "The toddler has been put on the seat; the bow is still held forward.", "Das Kleinkind wurde auf den Sitz gesetzt; der Bügel wird noch vorn gehalten.", out_, FLOOR_IDS, binding(seat=0.3, foot=0.15), LOADS_TODDLER, key=True),
        pose("lifting-key", "Lifting off the tile (key)", "Abheben von der Matte (Schlüssel)", "Both hands grip; the chair has just left the tile.", "Beide Hände greifen; der Stuhl hat die Matte gerade verlassen.", {"lift": {"dy": 0.04}, "chair": {"angleDeg": 3}}, GRIPS, binding(seat_cover=0.3, carried=True), [], key=True),
    ]


def clips():
    """🎞️ The clips: carrying the chair off its tile, setting up the bow and a mealtime in use."""
    return [
        {"id": "carry", "title": T("Carry away", "Wegtragen"), "description": T("One person grips the top back slat and the front of the seat, lifts the chair off its tile, tilts it back 8 degrees and holds it 0.16 m above the floor; the tile stays behind.", "Eine Person greift die obere Rückenleiste und die Vorderkante des Sitzes, hebt den Stuhl von seiner Matte, neigt ihn 8 Grad nach hinten und hält ihn 0,16 m über dem Boden; die Matte bleibt liegen."), "durationS": 4, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 0.8, "pose": "stored", "ease": "smooth"}, {"t": 1.4, "pose": "lifting-key", "ease": "smooth"}, {"t": 2.6, "pose": "carried", "ease": "smooth"}, {"t": 4, "pose": "carried", "ease": "smooth"}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 1.2} for a in FLOOR_IDS] + [{"anchor": a, "fromS": 0.6, "toS": 4} for a in GRIPS],
         "phases": [{"id": "grip", "fromS": 0, "toS": 0.8, "title": T("Grip", "Greifen")}, {"id": "lift", "fromS": 0.8, "toS": 2.6, "title": T("Lift and tilt", "Heben und Neigen")}, {"id": "hold", "fromS": 2.6, "toS": 4, "title": T("Carry", "Tragen")}],
         "events": [{"t": 0.6, "kind": "contact-on", "title": T("Hands take the weight", "Hände übernehmen das Gewicht"), "target": "grip-slat"}, {"t": 1.2, "kind": "contact-off", "title": T("Glides leave the tile", "Gleiter verlassen die Matte"), "target": "floor-fr"}],
         "tracks": []},
        {"id": "set-up", "title": T("Clip in the bow", "Bügel einklipsen"), "description": T("The bow is lifted from the seat with its tray, brought forward to tray height and pushed back into its clips on the uprights.", "Der Bügel wird mit seinem Tablett vom Sitz gehoben, nach vorn auf Tablethöhe gebracht und nach hinten in seine Clips an den Wangen geschoben."), "durationS": 4, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1.8, "pose": "bow-out-key", "ease": "smooth"}, {"t": 3.2, "pose": "ready", "ease": "smooth"}, {"t": 4, "pose": "ready", "ease": "smooth"}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 4} for a in FLOOR_IDS],
         "phases": [{"id": "lift-bow", "fromS": 0, "toS": 1.8, "title": T("Lift the bow", "Bügel heben")}, {"id": "clip-in", "fromS": 1.8, "toS": 3.2, "title": T("Clip in", "Einklipsen")}],
         "events": [{"t": 3.2, "kind": "motion", "title": T("Bow clicks into its clips", "Bügel rastet in seine Clips ein")}],
         "tracks": []},
        {"id": "mealtime", "title": T("Mealtime", "Essenszeit"), "description": T("The bow is pulled forward, the toddler is put on the seat, the bow is pushed back in and a bowl of porridge is set on the tray; the seat and footplate take the child's warmth.", "Der Bügel wird nach vorn gezogen, das Kleinkind auf den Sitz gesetzt, der Bügel wieder eingeschoben und eine Schale Brei auf das Tablett gestellt; Sitz und Fußplatte nehmen die Wärme des Kindes auf."), "durationS": 8, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "ready", "ease": "smooth"}, {"t": 1.0, "pose": "bow-out-key", "ease": "smooth"}, {"t": 2.6, "pose": "seated-bow-out-key", "ease": "smooth"}, {"t": 4.0, "pose": "in-use", "ease": "smooth"}, {"t": 8, "pose": "in-use", "ease": "smooth"}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 8} for a in FLOOR_IDS],
         "phases": [{"id": "open", "fromS": 0, "toS": 1.0, "title": T("Open the bow", "Bügel öffnen")}, {"id": "seat-child", "fromS": 1.0, "toS": 2.6, "title": T("Seat the child", "Kind hinsetzen")}, {"id": "close", "fromS": 2.6, "toS": 4.0, "title": T("Close the bow", "Bügel schließen")}, {"id": "eat", "fromS": 4.0, "toS": 8, "title": T("Eat", "Essen")}],
         "events": [{"t": 2.2, "kind": "load-on", "title": T("Toddler sits down", "Kleinkind setzt sich"), "target": "toddler-seat"}, {"t": 4.6, "kind": "load-on", "title": T("Bowl on the tray", "Schale auf dem Tablett"), "target": "bowl"}, {"t": 2.2, "kind": "thermal", "title": T("Seat starts taking body heat", "Sitz nimmt Körperwärme auf")}],
         "tracks": [{"target": "load:toddler-seat", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 1.6, "v": 0}, {"t": 2.4, "v": 1}, {"t": 8, "v": 1}]}, {"target": "load:toddler-feet", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 1.8, "v": 0}, {"t": 2.6, "v": 1}, {"t": 8, "v": 1}]}, {"target": "load:bowl", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 4.6, "v": 1}, {"t": 8, "v": 1}]}]},
    ]


def ref(id, citation):
    """📚️ A provenance reference."""
    return {"id": id, "citation": citation}


REFERENCES = [
    ref("en-14988", "EN 14988:2017+A1:2020 Children's high chairs - Requirements and test methods (stability, static and dynamic strength of seat, footrest and tray, restraint)."),
    ref("en-12520", "EN 12520:2015 Furniture - Strength, durability and safety - Requirements for domestic seating (order of the static seat loads)."),
    ref("en-1728", "EN 1728:2012 Furniture - Seating - Test methods for the determination of strength and durability."),
    ref("en-338", "EN 338:2016 Structural timber - Strength classes (hardwood class D40: bending strength 40 MPa, mean modulus 13 GPa, mean density 700 kg/m3)."),
    ref("iso-10456-2007", "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (hardwood 700 kg/m3: 0.18 W/mK, 1600 J/kgK; plywood 700 kg/m3: 0.17 W/mK, 1600 J/kgK; polypropylene 910 kg/m3: 0.22 W/mK, 1800 J/kgK)."),
    ref("wood-handbook-2021", "Forest Products Laboratory (2021). Wood Handbook - Wood as an Engineering Material, FPL-GTR-282, USDA (European beech density 640-720 kg/m3 at 12 % moisture; emissivity of finished wood about 0.9)."),
    ref("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (radiative properties of wood, paints and plastics, Table A.11)."),
    ref("who-growth-2006", "WHO Multicentre Growth Reference Study Group (2006). WHO Child Growth Standards: weight-for-age, median 11-12.5 kg at 18-24 months."),
    ref("din-18011", "DIN 18011:2020-12 Stellflächen, Abstände und Bewegungsflächen im Wohnungsbau (dining table height and space beside a table)."),
    ref("eva-foam-tile", "Catalogue-typical data of EVA foam puzzle play mats: 60 x 60 cm tiles, 10-12 mm thick, about 0.3-0.4 kg per tile (closed-cell ethylene vinyl acetate foam of about 90 kg/m3, conductivity about 0.04 W/mK)."),
    ref("pp-and-beech", "Catalogue-typical data of step high chairs of solid beech that grow with the child (height 79-80 cm, width 46-50 cm, depth 49-55 cm, 6.5-7.5 kg without accessories; clip-on tray bow of beech with a polypropylene tray, 0.8-1.0 kg); no brand is modelled."),
    ref("semio-used-household", "Semio staffage content, used household objects wave 2026-11-22: dimensions, masses and traces of use are authored from typical second-hand products of the type; parameters tagged authored-estimate are not taken from a datasheet."),
]


def materials():
    """🧱️ The materials with their thermal and structural properties, aged where the surface has aged."""
    return [
        {"id": "beech-lacquered", "title": T("Solid beech with aged, yellowed clear lacquer", "Massive Buche mit gealtertem, vergilbtem Klarlack"), "emissivity": 0.9, "solarAbsorptivity": 0.55, "densityKgM3": 710, "conductivityWmK": 0.18, "specificHeatJKgK": 1600, "elasticModulusPa": 13e9, "strengthPa": 40e6, "source": "literature", "reference": "en-338"},
        {"id": "beech-plywood", "title": T("Beech plywood 18 mm, lacquer worn through in places", "Buchensperrholz 18 mm, Lack stellenweise durchgescheuert"), "emissivity": 0.9, "solarAbsorptivity": 0.58, "densityKgM3": 690, "conductivityWmK": 0.17, "specificHeatJKgK": 1600, "elasticModulusPa": 9e9, "strengthPa": 35e6, "source": "literature", "reference": "iso-10456-2007"},
        {"id": "polypropylene", "title": T("Polypropylene tray, scratched and stained", "Polypropylen-Tablett, zerkratzt und fleckig"), "emissivity": 0.92, "solarAbsorptivity": 0.45, "densityKgM3": 905, "conductivityWmK": 0.22, "specificHeatJKgK": 1800, "elasticModulusPa": 1.4e9, "strengthPa": 30e6, "source": "literature", "reference": "iso-10456-2007"},
        {"id": "pp-glide", "title": T("Black polypropylene glide", "Schwarzer Polypropylen-Gleiter"), "emissivity": 0.92, "solarAbsorptivity": 0.9, "densityKgM3": 905, "conductivityWmK": 0.22, "specificHeatJKgK": 1800, "source": "literature", "reference": "iso-10456-2007"},
        {"id": "wool-felt", "title": T("Grey wool felt pad (replacement)", "Graues Wollfilzpolster (Ersatz)"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "densityKgM3": 280, "conductivityWmK": 0.05, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "incropera-2018"},
        {"id": "eva-foam", "title": T("EVA foam tile 12 mm, faded blue", "EVA-Schaummatte 12 mm, verblasstes Blau"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": 90, "conductivityWmK": 0.04, "specificHeatJKgK": 1900, "elasticModulusPa": 2e6, "strengthPa": 0.4e6, "source": "authored-estimate", "reference": "eva-foam-tile"},
        {"id": "zinc-steel", "title": T("Zinc-plated M6 bolt caps, dulled", "Verzinkte M6-Schraubkappen, matt geworden"), "emissivity": 0.25, "solarAbsorptivity": 0.6, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 450, "source": "literature", "reference": "incropera-2018"},
    ]


def layer(id, en, de, kind, material, t, k, rho, c, source="literature", reference="iso-10456-2007"):
    """🧶️ One thermal layer with resistance and areal heat capacity from its thickness."""
    return {"id": id, "title": T(en, de), "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": rnd(t / k), "heatCapacityJM2K": rnd(t * rho * c), "source": source, "reference": reference}


AREA = {}


def regions():
    """🔥️ The six thermal regions with their outer areas from the 1:50 model."""
    def reg(id, en, de, mat, area, L, geom, svf, lay, extra=None):
        r = {"id": id, "title": T(en, de), "kind": "surface", "model": "passive", "material": mat, "areaM2": area, "characteristicLengthM": L, "geometry": geom, "skyViewFactor": svf, "layers": [lay], "heatCapacityJM2K": lay["heatCapacityJM2K"], "source": "authored-estimate", "reference": "iso-10456-2007"}
        r.update(extra or {})
        return r
    return [
        reg("frame", "Side frames, back slats and stretcher (aged lacquered beech)", "Seitenwangen, Rückenleisten und Strebe (gealterte lackierte Buche)", "beech-lacquered", AREA["frame"], 0.075, "vertical-plate", 0.4, layer("frame-beech", "Beech 27 mm (half thickness for each face)", "Buche 27 mm (halbe Dicke je Seite)", "timber", "beech-lacquered", 0.0135, 0.18, 710, 1600), {"ratedPowerW": 0, "ratedPowerBasis": "passive"}),
        reg("seat-plate", "Seat plate, stained and worn", "Sitzplatte, fleckig und abgegriffen", "beech-plywood", AREA["seat-plate"], 0.2, "horizontal-plate-up", 0.5, layer("seat-ply", "Beech plywood 18 mm (half thickness for each face)", "Buchensperrholz 18 mm (halbe Dicke je Seite)", "timber", "beech-plywood", 0.009, 0.17, 690, 1600)),
        reg("footplate", "Footplate, worn by shoes", "Fußplatte, von Schuhen abgetreten", "beech-plywood", AREA["footplate"], 0.22, "horizontal-plate-up", 0.4, layer("foot-ply", "Beech plywood 18 mm (half thickness for each face)", "Buchensperrholz 18 mm (halbe Dicke je Seite)", "timber", "beech-plywood", 0.009, 0.17, 690, 1600)),
        reg("bow", "Tray bow with teeth marks", "Essbügel mit Zahnabdrücken", "beech-lacquered", AREA["bow"], 0.03, "cylinder", 0.5, layer("bow-beech", "Beech 25 mm (half thickness)", "Buche 25 mm (halbe Dicke)", "timber", "beech-lacquered", 0.0125, 0.18, 710, 1600)),
        reg("tray", "Clip-on tray, scratched polypropylene with food stains", "Aufgeklipstes Tablett, zerkratztes Polypropylen mit Essensflecken", "polypropylene", AREA["tray"], 0.24, "horizontal-plate-up", 0.6, layer("tray-pp", "Polypropylene wall 3 mm", "Polypropylenwand 3 mm", "polymer", "polypropylene", 0.003, 0.22, 905, 1800)),
        reg("mat", "EVA foam floor tile", "EVA-Schaum-Bodenmatte", "eva-foam", AREA["mat"], 0.6, "horizontal-plate-up", 0.3, layer("mat-eva", "EVA foam 12 mm", "EVA-Schaum 12 mm", "insulation", "eva-foam", 0.012, 0.04, 90, 1900, source="authored-estimate", reference="eva-foam-tile")),
    ]


def hand_areas():
    """🧮️ Outer areas by hand from the 1:50 prisms: each region's prism surfaces less the faces two prisms of the same region share."""
    ps = {p.id: p for p in parts_of(50)}
    s = lambda *ids: sum(ps[i].surface() for i in ids)
    up = ps["upright-r"]
    overlap_up_runner = 2 * ((F(YR0) - R(YR0)) * 0.027 + (YR1 - YR0) * 0.027) * 0
    slat_into = 2 * 2 * (0.007 * (SLATS[0][1] - SLATS[0][0]) / math.sin(THETA) + 0.007 * 0.018)
    frame = s("runner-r", "runner-l", "upright-r", "upright-l", "slat-0", "slat-1", "stretcher", "glide-fr", "glide-fl", "glide-rr", "felt-rl")
    frame -= 2 * 2 * ((F(YR0) - R(YR0)) * (YR1 - YR0)) + 2 * 2 * 0.027 * (YR1 - YR0) * 0.8
    frame -= 2 * (2 * (0.018 * 0.08 / math.sin(THETA)) * 2) + 2 * (2 * math.pi * 0.0125 ** 2)
    frame -= 3 * 2 * 0.058 * 0.027 + 0.0013 * 2
    AREA["frame"] = round(frame, 3)
    AREA["seat-plate"] = round(s("seat-plate"), 3)
    AREA["footplate"] = round(s("footplate"), 3)
    AREA["bow"] = round(s("bow"), 3)
    AREA["tray"] = round(s("tray-rim") + s("tray-floor") - 2 * perimeter(tray_outline(TRAY["wall"])) * 0.003, 3)
    AREA["mat"] = round(s("tile"), 3)


hand_areas()


def volume_lists(scale):
    """🧊️ Body, thermal and structure lists of one scale from its parts."""
    parts = parts_of(scale)
    body = [p.solid() for p in parts]
    thermal = [{"id": f"th-{p.id}", "of": p.id, "region": p.region} for p in parts if not p.texture]
    f = FEATURE[scale]
    members = {"runner-r": ["runner-r"], "runner-l": ["runner-l"], "upright-r": ["upright-r"], "upright-l": ["upright-l"], "seat": ["seat-plate"], "footplate": ["footplate"], "bow": ["bow"], "mat": ["tile"]}
    members["backrest"] = ["slat-0", "slat-1"] if scale != 200 else ["backrest-band"]
    members["stretcher"] = ["stretcher"]
    structure = []
    for b, ids in members.items():
        for i in ids:
            structure.append({"id": f"m-{i}", "of": i, "member": b, "part": "member"})
    if scale == 50:
        for gid, b in (("glide-fr", "runner-r"), ("glide-rr", "runner-r"), ("glide-fl", "runner-l"), ("felt-rl", "runner-l")):
            structure.append({"id": f"j-{gid}", "of": gid, "member": b, "part": "joint"})
    def jbox(id, b, p, size, member):
        u, v = local(b, (p[0], p[1]))
        return {"id": id, "kind": "box", "bone": b, "material": "beech-lacquered", "role": "detail", "tone": 0.3, "size": [rnd(size[0]), rnd(size[1]), rnd(size[2])], "transform": {"translation": [rnd(u), rnd(v), rnd(p[2])]}, "member": member, "part": "joint"}
    js = {50: 0.02, 100: 0.024, 200: 0.036}[scale]
    for side, sgn in (("r", -1), ("l", 1)):
        zc = sgn * ZC
        structure.append(jbox(f"j-foot-{side}", f"upright-{side}", ((F(YR1) + R(YR0)) / 2, (YR0 + YR1) / 2, zc), (0.05 + js, 0.04, 0.03 if scale == 200 else 0.027), f"upright-{side}"))
        structure.append(jbox(f"j-seat-{side}", "seat", ((F(SEAT["y1"]) + R(SEAT["y1"])) / 2 + 0.02, (SEAT["y0"] + SEAT["y1"]) / 2 - (0.006 if scale == 200 else 0), sgn * 0.2065), (0.04, max(0.018, js), js), "seat"))
        structure.append(jbox(f"j-foot-plate-{side}", "footplate", ((F(FOOT["y1"]) + R(FOOT["y1"])) / 2 + 0.02, (FOOT["y0"] + FOOT["y1"]) / 2 - (0.006 if scale == 200 else 0), sgn * 0.2065), (0.04, max(0.018, js), js), "footplate"))
        structure.append(jbox(f"j-back-{side}", "backrest", (R((SLATS[0][0] + SLATS[1][1]) / 2) + 0.012, (SLATS[0][0] + SLATS[1][1]) / 2, sgn * 0.2065), (0.2, js, js), "backrest"))
        structure.append(jbox(f"j-bow-{side}", "bow", (BOW["x0"] + 0.015, (BOW["y0"] + BOW["y1"]) / 2, sgn * (BOW["zo"] - 0.012)), (js, 0.03, js if scale != 200 else 0.03), "bow"))
        structure.append(jbox(f"j-rod-{side}", "stretcher", (RX, ROD["y"], sgn * 0.206), (js, js, js), "stretcher"))
    rr = {50: 0.012, 100: 0.016, 200: 0.022}[scale]
    marks = [(f"mass-{b}", b, c) for b, (m, c) in MASSES.items() if b != "backrest" or scale == 200]
    if scale != 200:
        marks += [(f"mass-backrest-{k}", "backrest", ((R(y0) + R(y1)) / 2 + SLAT_H / 2, (y0 + y1) / 2, 0.0)) for k, (y0, y1) in enumerate(SLATS)]
    for id, b, c in marks:
        u, v = local(b, (c[0], c[1]))
        structure.append({"id": id, "kind": "ellipsoid", "bone": b, "material": "beech-lacquered", "role": "detail", "tone": 0.4, "radii": [rr, rr, rr], "transform": {"translation": [rnd(u), rnd(v), rnd(c[2])]}, "member": b, "part": "mass"})
    return body, thermal, structure


def volume_decisions(scale):
    """📝️ What each volume scale models and generalises."""
    if scale == 50:
        return [
            T("Body: the full construction for the 7.5 mm feature: both side frames as prisms of their true side profiles (27 mm beech, runner 50 mm high with a rounded rear end and chamfered front, upright 75 mm wide at 70 degrees with a chamfered top), the three 8 mm polypropylene glides and the 40 mm felt pad that replaced the lost rear-left glide, two 18 mm back slats flush with the rear edge and 7 mm deep in the uprights, the 25 mm round stretcher, the 18 mm seat plate and footplate in their 7 mm grooves (the footplate with its chipped front-right corner), the tray bow lying on the seat with the 3 mm polypropylene tray rim and floor clipped on it, and the EVA puzzle tile with its dovetail teeth and one bitten-off tooth; the six zinc bolt caps are texture.", "Körper: die volle Konstruktion für das Merkmal von 7,5 mm: beide Seitenwangen als Prismen ihrer wahren Seitenprofile (27 mm Buche, Kufe 50 mm hoch mit gerundetem hinterem Ende und abgeschrägter Front, Wange 75 mm breit unter 70 Grad mit abgeschrägtem Kopf), die drei 8-mm-Polypropylengleiter und das 40-mm-Filzpolster anstelle des verlorenen hinteren linken Gleiters, zwei 18-mm-Rückenleisten bündig mit der Hinterkante und 7 mm tief in den Wangen, die runde 25-mm-Strebe, Sitz- und Fußplatte von 18 mm in ihren 7-mm-Nuten (die Fußplatte mit abgeplatzter vorderer rechter Ecke), der auf dem Sitz liegende Essbügel mit aufgeklipstem 3-mm-Polypropylen-Tablettrand und -boden sowie die EVA-Puzzlematte mit Schwalbenschwanzzähnen und einem abgebissenen Zahn; die sechs verzinkten Schraubkappen sind Textur."),
            T("Thermal: every body solid except the bolt caps names its region (frame, seat plate, footplate, bow, tray, tile); the areas are the outer areas of these solids.", "Thermik: jeder Körper außer den Schraubkappen nennt seine Region (Gestell, Sitzplatte, Fußplatte, Bügel, Tablett, Matte); die Flächen sind die Außenflächen dieser Körper."),
            T("Structure: runners, uprights, slats, stretcher, plates, bow and tile are members of their true sections; joints at the glides, the upright feet, the plate grooves, the slat and stretcher bolts and the bow clips; masses at the centres of the bones; the supports are the floor contacts under the tile or the two carrying hands of the pose.", "Tragwerk: Kufen, Wangen, Leisten, Strebe, Platten, Bügel und Matte sind Glieder ihrer wahren Querschnitte; Knoten an den Gleitern, den Wangenfüßen, den Plattennuten, den Leisten- und Strebenschrauben und den Bügelclips; Massen in den Knochenschwerpunkten; Auflager sind die Bodenkontakte unter der Matte oder die zwei tragenden Hände der Pose."),
        ]
    if scale == 100:
        return [
            T("Body: a new model for the 15 mm feature: the glides and the felt pad are merged into the runners, which reach down to the tile; the tray is one polypropylene body of its outline and 30 mm height; the tile edge is its mean line without the 12 mm teeth and is a declared 12 mm sheet; slats, stretcher, plates and bow keep their true sections; bolt caps and the footplate chip are left out.", "Körper: ein neues Modell für das Merkmal von 15 mm: Gleiter und Filzpolster gehen in die Kufen auf, die bis zur Matte reichen; das Tablett ist ein Polypropylenkörper seines Umrisses und 30 mm Höhe; der Mattenrand ist seine Mittellinie ohne die 12-mm-Zähne und eine erklärte 12-mm-Platte; Leisten, Strebe, Platten und Bügel behalten ihre wahren Querschnitte; Schraubkappen und die Absplitterung der Fußplatte entfallen."),
            T("Thermal and structure follow the body; the glide joints are now inside the runner members.", "Thermik und Tragwerk folgen dem Körper; die Gleiterknoten liegen nun in den Kufengliedern."),
        ]
    return [
        T("Body: a new model for the 30 mm feature: side frames 30 mm thick with the runners down to the tile, the two slats and their gap as one 32 mm backrest band, seat plate and footplate thickened downwards to 30 mm with their top faces at the true groove heights, bow arms 30 mm, the 25 mm stretcher 31 mm, the tray one 30 mm body and the tile a declared 12 mm sheet.", "Körper: ein neues Modell für das Merkmal von 30 mm: Seitenwangen 30 mm dick mit den Kufen bis zur Matte, die zwei Leisten und ihr Abstand als ein 32-mm-Lehnenband, Sitz- und Fußplatte nach unten auf 30 mm verstärkt mit den Oberseiten auf den wahren Nutenhöhen, Bügelarme 30 mm, die 25-mm-Strebe 31 mm, das Tablett ein 30-mm-Körper und die Matte eine erklärte 12-mm-Platte."),
        T("Structure: the members follow the generalised body (the backrest band is the backrest member, the 31 mm rod the stretcher); joints and mass markers are sized for the 30 mm feature.", "Tragwerk: die Glieder folgen dem verallgemeinerten Körper (das Lehnenband ist das Glied der Rückenlehne, der 31-mm-Stab die Strebe); Knoten und Massenmarken sind für das Merkmal von 30 mm bemessen."),
    ]


def volumes():
    """🗃️ The volumes of the authored scales; 1:500 is unavailable like its drawing."""
    out = {}
    for scale in (50, 100, 200):
        body, thermal, structure = volume_lists(scale)
        out[str(scale)] = {"status": "authored", "denominator": scale, "body": body, "thermal": thermal, "structure": structure, "decisions": volume_decisions(scale)}
    out["500"] = {"status": "unavailable", "denominator": 500}
    return out


def unavailable500():
    """🚫️ Why 1:500 is not drawn."""
    return {"status": "unavailable", "denominator": 500, "cause": "below-printable-feature", "reason": T("At 1:500 the high chair is 1.2 by 1.6 mm on paper, under the 3 mm a recognisable drawing needs; its 27 mm side frames would be 0.05 mm, a quarter of the 0.2 mm office-laser minimum feature, and the 18 mm plates 0.04 mm; draw it at 1:200 or use the symbol used-high-chair-symbol.", "Bei 1:500 misst der Hochstuhl 1,2 mal 1,6 mm auf dem Papier, unter den 3 mm, die eine erkennbare Zeichnung braucht; seine 27-mm-Seitenwangen wären 0,05 mm, ein Viertel des Mindestmerkmals von 0,2 mm des Büro-Laserdrucks, und die 18-mm-Platten 0,04 mm; bei 1:200 zeichnen oder das Symbol used-high-chair-symbol verwenden."), "limitingFeature": T("Side frames of 27 mm beech", "Seitenwangen aus 27 mm Buche"), "limitingSizeM": 0.027, "alternatives": [{"kind": "scale", "denominator": 200}, {"kind": "symbol", "symbol": "used-high-chair-symbol"}]}


def symbol():
    """🔣️ A true-extent symbol of the chair on its tile in all three views."""
    pen = {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35}
    side = [(XR0, TM), (XR1, TM), (XR1, YR1), (F(YR1), YR1), (F(FOOT["y0"]), FOOT["y0"]), (FOOT["x1"], FOOT["y0"]), (FOOT["x1"], FOOT["y1"]), (F(FOOT["y1"]), FOOT["y1"]), (F(SEAT["y0"]), SEAT["y0"]), (SEAT["x1"], SEAT["y0"]), (SEAT["x1"], SEAT["y1"]), (F(SEAT["y1"]), SEAT["y1"]), (F(YT), YT), (R(YT), YT), (R(YR1), YR1), (XR0, YR1)]
    g = {
        "elevation": [poly("sym-tile", "ground", "body", 1, [(MAT["x0"], 0), (MAT["x1"] + 0.012, 0), (MAT["x1"] + 0.012, TM), (MAT["x0"], TM)], True, filled=True, tone=0.42), poly("sym-frame", "ground", "body", 2, side, True, filled=True, tone=0.72), poly("sym-tray", "ground", "body", 3, [(TRAY["x0"], BOW["y0"]), (TRAY["x1"], BOW["y0"]), (TRAY["x1"], TRAY["y1"]), (TRAY["x0"], TRAY["y1"])], True, filled=True, tone=0.9)],
        "front": [poly("sym-tile", "ground", "body", 1, [(MAT["z0"], 0), (MAT["z1"] + 0.012, 0), (MAT["z1"] + 0.012, TM), (MAT["z0"], TM)], True, filled=True, tone=0.42), poly("sym-side-r", "ground", "body", 2, [(-ZO, TM), (-ZI, TM), (-ZI, YT), (-ZO, YT)], True, filled=True, tone=0.72), poly("sym-side-l", "ground", "body", 3, [(ZI, TM), (ZO, TM), (ZO, YT), (ZI, YT)], True, filled=True, tone=0.72), poly("sym-seat", "ground", "body", 4, [(-ZI, SEAT["y0"]), (ZI, SEAT["y0"]), (ZI, TRAY["y1"]), (-ZI, TRAY["y1"])], True, filled=True, tone=0.8), poly("sym-foot", "ground", "body", 5, [(-ZI, FOOT["y0"]), (ZI, FOOT["y0"]), (ZI, FOOT["y1"]), (-ZI, FOOT["y1"])], True, filled=True, tone=0.6), poly("sym-back", "ground", "body", 6, [(-ZI, SLATS[0][0]), (ZI, SLATS[0][0]), (ZI, SLATS[1][1]), (-ZI, SLATS[1][1])], True, filled=True, tone=0.72)],
        "plan": [poly("sym-tile", "ground", "body", 1, [(MAT["x0"], MAT["z0"]), (MAT["x1"] + 0.012, MAT["z0"]), (MAT["x1"] + 0.012, MAT["z1"] + 0.012), (MAT["x0"], MAT["z1"] + 0.012)], True, filled=True, tone=0.42), poly("sym-chair", "ground", "body", 2, [(XR0, -ZO), (XR1, -ZO), (XR1, ZO), (XR0, ZO)], True, filled=True, tone=0.72), poly("sym-tray", "ground", "body", 3, [(TRAY["x0"], -TRAY["z"]), (TRAY["x1"], -TRAY["z"]), (TRAY["x1"], TRAY["z"]), (TRAY["x0"], TRAY["z"])], True, filled=True, tone=0.9)],
    }
    return [{"id": "used-high-chair-symbol", "title": T("High chair on its tile, true extent", "Hochstuhl auf seiner Matte, wahre Ausdehnung"), "description": T("The step high chair as a pictogram in three views: side frame with seat and footplate on the 0.61 m floor tile, the two uprights and the plates in front, the chair and tray over the tile in plan.", "Der Treppenhochstuhl als Piktogramm in drei Ansichten: Seitenwange mit Sitz und Fußplatte auf der 0,61-m-Bodenmatte, die zwei Wangen und die Platten von vorn, Stuhl und Tablett über der Matte in der Draufsicht."), "semantics": "true-extent", "units": "model-m", "denominators": [500, 200], "pens": pen, "hatches": [], "geometry": g, "dimensionalNote": T("Model metres at real size in the frame of the asset (x ahead, lateral positive to the left, y up), stored pose: chair 0.49 m deep, 0.46 m wide and 0.79 m high on the 12 mm tile of 0.61 m.", "Modellmeter in wahrer Größe im Koordinatensystem des Objekts (x nach vorn, seitlich positiv nach links, y nach oben), Lagerpose: Stuhl 0,49 m tief, 0,46 m breit und 0,79 m hoch auf der 12-mm-Matte von 0,61 m.")}]


def thermal():
    """🌡️ The thermal model: a kitchen with still air, six passive regions, no emitters."""
    return {
        "environment": {"airTempC": 21, "meanRadiantTempC": 21, "relativeHumidity": 0.5, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 300, "diffuseFraction": 0.2, "pressurePa": 101325},
        "passengers": [], "operating": [], "regions": regions(), "emitters": [],
        "assumptions": [
            T("Lacquered beech: emissivity 0.9 (Incropera Table A.11, finished wood); the clear lacquer has yellowed over about fifteen years of use, which raises the solar absorptance from about 0.45 of new light beech to 0.55 (authored-estimate); the plates, where the lacquer is worn through and stained, are taken at 0.58.", "Lackierte Buche: Emissionsgrad 0,9 (Incropera Tabelle A.11, behandeltes Holz); der Klarlack ist in etwa fünfzehn Jahren Gebrauch vergilbt, was die solare Absorption von etwa 0,45 neuer heller Buche auf 0,55 hebt (Schätzwert); die Platten, wo der Lack durchgescheuert und fleckig ist, mit 0,58."),
            T("The polypropylene tray is scratched and stained (solar absorptance 0.45 instead of 0.35 for new white PP, authored-estimate); the EVA tile is faded blue (0.65).", "Das Polypropylen-Tablett ist zerkratzt und fleckig (solare Absorption 0,45 statt 0,35 für neues weißes PP, Schätzwert); die EVA-Matte ist verblasst blau (0,65)."),
            T("A toddler in use warms 30 % of the seat plate at 31 degC, 15 % of the footplate at 29 degC (socks), 12 % of the tray (forearms) and 10 % of the bow (belly) at 31-32 degC (clothing surface temperatures of ISO 7730 order, authored-estimate); the tile lies with half its outer area on a floor at 20 degC; furniture draws no power (passive, 0 W).", "Ein Kleinkind in Gebrauch erwärmt 30 % der Sitzplatte auf 31 degC, 15 % der Fußplatte auf 29 degC (Socken), 12 % des Tabletts (Unterarme) und 10 % des Bügels (Bauch) auf 31-32 degC (Bekleidungsoberflächen in der Größenordnung von ISO 7730, Schätzwert); die Matte liegt mit der Hälfte ihrer Außenfläche auf einem Boden von 20 degC; Möbel nehmen keine Leistung auf (passiv, 0 W)."),
            T("Reference case: a kitchen at 21 degC with still air and a window sun patch of 300 W/m2 that does not reach the chair by default.", "Referenzfall: eine Küche mit 21 degC, ruhiger Luft und einem Fenster-Sonnenfleck von 300 W/m2, der den Stuhl standardmäßig nicht erreicht."),
        ],
    }


def structure():
    """🏗️ The structural model: supports come from the pose, the floor contacts under the tile or the two carrying hands."""
    return {"gravityMS2": G, "dynamics": "quasi-static", "supports": [], "clinical": False, "assumptions": [
        T("Supports follow the pose: standing on the tile the chair rests on four floor contacts under the glides and the felt pad (through 12 mm of EVA foam); carried, its weight hangs on two pins, one hand over the top back slat and one under the front edge of the seat, so the two hands form the only load path and no floor contact is active.", "Die Auflager folgen der Pose: auf der Matte stehend ruht der Stuhl auf vier Bodenkontakten unter den Gleitern und dem Filzpolster (durch 12 mm EVA-Schaum); getragen hängt sein Gewicht an zwei Gelenken, einer Hand über der oberen Rückenleiste und einer unter der Vorderkante des Sitzes, die zwei Hände sind dann der einzige Lastweg und kein Bodenkontakt ist aktiv."),
        T("The quasi-static solver treats the asset as one rigid body, so in the carried pose the 0.4 kg tile that stays on the floor is counted on the hands as well (4 N of about 80 N); the chair itself is 7.4 kg with bow and tray.", "Der quasistatische Löser behandelt das Objekt als einen starren Körper, daher wird in der Tragepose auch die am Boden bleibende Matte von 0,4 kg an den Händen gezählt (4 N von etwa 80 N); der Stuhl selbst wiegt mit Bügel und Tablett 7,4 kg."),
        T("Members: beech of strength class D40 after EN 338 (runners 27 x 50 mm, uprights 27 x 75 mm, slats 18 x 80 mm, stretcher 25 mm), plates of 18 mm beech plywood in 7 mm grooves; the toddler of 12 kg (WHO median at 18-24 months) puts 85 % on the seat and 15 % on the footplate, a 0.25 kg bowl stands on the tray; EN 14988 sets the test loads for high chairs, which are several times these service loads and are not modelled here.", "Glieder: Buche der Festigkeitsklasse D40 nach EN 338 (Kufen 27 x 50 mm, Wangen 27 x 75 mm, Leisten 18 x 80 mm, Strebe 25 mm), Platten aus 18-mm-Buchensperrholz in 7-mm-Nuten; das Kleinkind von 12 kg (WHO-Median mit 18-24 Monaten) gibt 85 % auf den Sitz und 15 % auf die Fußplatte, eine Schale von 0,25 kg steht auf dem Tablett; EN 14988 legt die Prüflasten für Hochstühle fest, die ein Mehrfaches dieser Gebrauchslasten betragen und hier nicht modelliert sind."),
    ]}


def dims():
    """📏️ The real envelope of the rest pose from the 1:50 parts and the total mass."""
    xs, ys, zs = [], [], []
    for p in parts_of(50):
        x0, x1, y0, y1, z0, z1 = p.ranges()
        xs += [x0, x1]
        ys += [y0, y1]
        zs += [z0, z1]
    return {"heightM": rnd(max(ys) - min(ys)), "widthM": rnd(max(xs) - min(xs)), "depthM": rnd(max(zs) - min(zs)), "massKg": rnd(sum(m for m, _ in MASSES.values()))}


def asset():
    """🏺️ The whole asset document."""
    d = dims()
    a = {
        "id": "used-high-chair",
        "category": "furniture",
        "title": T("Used Child High Chair", "Gebrauchter Kinderhochstuhl"),
        "summary": T("A second-hand step high chair of lacquered beech that grows with the child: two inclined grooved uprights on runners, seat plate and footplate in their grooves, two back slats and a clip-on tray bow, with food stains, teeth marks, yellowed lacquer and a felt pad where a glide was lost; stored on the EVA floor tile it came with, carried by one person, and in use with a toddler at mealtime.", "Ein gebrauchter, mitwachsender Treppenhochstuhl aus lackierter Buche: zwei geneigte genutete Wangen auf Kufen, Sitz- und Fußplatte in ihren Nuten, zwei Rückenleisten und ein aufklipsbarer Essbügel, mit Essensflecken, Zahnabdrücken, vergilbtem Lack und einem Filzpolster anstelle eines verlorenen Gleiters; gelagert auf der EVA-Bodenmatte, mit der er kam, von einer Person getragen und in Gebrauch mit einem Kleinkind beim Essen."),
        "tags": ["used", "household", "second-hand", "children", "kitchen", "high-chair", "seating", "beech", "timber", "grows-with-child", "tray"],
        "provenance": {"author": "Semio staffage content (used household objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": T(
            f"Step high chair of the common solid-beech type that grows with the child (no brand): 0.79 m high, 0.46 m wide, runners 0.49 m long, uprights inclined at 70 degrees, seat top at 0.47 m and footplate top at 0.25 m above the floor the chair stands on, both in their top groove pairs for a toddler; tray bow with a polypropylene tray of 0.40 x 0.24 m. Masses from the volumes of the 1:50 parts and the densities of the materials: {d['massKg']:.2f} kg in all, of which the tile 0.39 kg. Traces of use: yellowed lacquer worn on the edges, food stains and a cup ring on the tray and the seat, teeth marks on the bow, a chipped footplate corner, a half scratched-off sticker on the right upright and a felt pad instead of the lost rear-left glide.",
            f"Treppenhochstuhl des verbreiteten mitwachsenden Typs aus massiver Buche (keine Marke): 0,79 m hoch, 0,46 m breit, Kufen 0,49 m lang, Wangen unter 70 Grad geneigt, Sitzoberkante 0,47 m und Fußplattenoberkante 0,25 m über dem Boden, auf dem der Stuhl steht, beide im obersten Nutenpaar für ein Kleinkind; Essbügel mit Polypropylen-Tablett von 0,40 x 0,24 m. Massen aus den Volumen der 1:50-Teile und den Dichten der Werkstoffe: insgesamt {d['massKg']:.2f} kg, davon die Matte 0,39 kg. Gebrauchsspuren: vergilbter, an den Kanten abgegriffener Lack, Essensflecken und ein Tassenring auf Tablett und Sitz, Zahnabdrücke am Bügel, eine abgeplatzte Ecke der Fußplatte, ein halb abgekratzter Aufkleber an der rechten Wange und ein Filzpolster anstelle des verlorenen hinteren linken Gleiters.")},
        "dimensions": d,
        "materials": materials(),
        "rig": rig(),
        "anchors": anchors(),
        "representations": {"50": representation(50), "100": representation(100), "200": representation(200), "500": unavailable500()},
        "symbols": symbol(),
        "poses": poses(),
        "clips": clips(),
        "thermal": thermal(),
        "structure": structure(),
        "volumes": volumes(),
        "presentation": {"defaultPose": "stored", "defaultClip": "carry", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": a}


if __name__ == "__main__":
    doc = asset()
    with open(OUT, "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
    print("[DEBUG] dims", doc["asset"]["dimensions"])
    print("[DEBUG] masses", {b: round(m, 3) for b, (m, _) in MASSES.items()})
    print("[DEBUG] areas", AREA)
