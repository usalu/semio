"""🌀 Round 3 hermetic scroll compressor (Copeland K-frame class, 241 mm shell, 0.43 m): reference constants, every part of every scale stated by hand with its solid and its 2D shapes, the root members, the candidate build and `--apply`."""
import json
import math
import pathlib
import sys

T = pathlib.Path(__file__).resolve().parent
R = T.parents[6]
CATALOG = R / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
ASSET_FILE = CATALOG / "🏗️machinery/scroll-compressor/🔣️.json"
MANIFEST_FILE = CATALOG / "🔣️.json"
SOURCE_FILE = T / "volumes/machinery/scroll-compressor.json"
OUT = T / "🗑️generated/r3-scroll-compressor"
TEXTS = json.loads((T / "r3-scroll-compressor-texts.json").read_text())

RS = 0.1205
RCAP = 0.1245
Y_GROM = 0.022
Y_PLATE = 0.0295
Y_SEAM = 0.3175
SKIRT = 0.02
KNUCKLE = 0.035
CROWN = 0.25
HOLE = 0.09525
EAR = 0.021
DISC_R = 0.104
SUC_Y = 0.18
TERM = (0.112, 0.22)
TERM_BOX = (0.048, 0.078, 0.072)
WIN_Y = 0.26
TOP = 0.43
DOME_L = TOP - Y_SEAM


def r5(value):
    rounded = round(value + 0.0, 5)
    return int(rounded) if rounded == int(rounded) else rounded


def pts(points):
    return [[r5(a), r5(b)] for a, b in points]


def vec(v):
    return [r5(x) for x in v]


def crown_center():
    return SKIRT - math.sqrt((CROWN - KNUCKLE) ** 2 - (RCAP - KNUCKLE) ** 2)


def crown_y(r):
    return crown_center() + math.sqrt(CROWN ** 2 - r ** 2)


PEAK = crown_y(0)


def cap_outline(nk, nc):
    """⛑️ Top cap outer profile (radius, height above the lap) from the skirt foot over the 35 mm knuckle to the 250 mm crown at the axis."""
    kc = (RCAP - KNUCKLE, SKIRT)
    yc = crown_center()
    phi_t = math.atan2(kc[1] - yc, kc[0])
    out = [(RCAP, 0.0), (RCAP, SKIRT)]
    for i in range(1, nk + 1):
        a = phi_t * i / nk
        out.append((kc[0] + KNUCKLE * math.cos(a), kc[1] + KNUCKLE * math.sin(a)))
    for i in range(1, nc + 1):
        a = phi_t + (math.pi / 2 - phi_t) * i / nc
        out.append((CROWN * math.cos(a), yc + CROWN * math.sin(a)))
    out[-1] = (0.0, PEAK)
    return out


def plate_outline(n_arc=10, n_ear=8):
    """🦶 Stamped base plate in (x, z): a 104 mm radius disc with four mounting ears on the diagonals, ear ends round about the 190.5 mm square bolt pattern."""
    s = math.sqrt(DISC_R ** 2 - EAR ** 2)
    d = HOLE * math.sqrt(2)
    out = []
    for k in range(4):
        a = math.radians(45 + 90 * k)
        u, n = (math.cos(a), math.sin(a)), (-math.sin(a), math.cos(a))
        prev = math.radians(45 + 90 * (k - 1)) + math.asin(EAR / DISC_R)
        here = a - math.asin(EAR / DISC_R)
        for i in range(1, n_arc):
            t = prev + (here - prev) * i / n_arc
            out.append((DISC_R * math.cos(t), DISC_R * math.sin(t)))
        out.append((u[0] * s - n[0] * EAR, u[1] * s - n[1] * EAR))
        for i in range(n_ear + 1):
            t = -math.pi / 2 + math.pi * i / n_ear
            out.append((u[0] * d + EAR * (u[0] * math.cos(t) + n[0] * math.sin(t)), u[1] * d + EAR * (u[1] * math.cos(t) + n[1] * math.sin(t))))
        out.append((u[0] * s + n[0] * EAR, u[1] * s + n[1] * EAR))
    return out


def circle(cx, cy, r, n=24):
    return [(cx + r * math.cos(2 * math.pi * i / n), cy + r * math.sin(2 * math.pi * i / n)) for i in range(n)]


def spiral(r0, r1, turns, step_deg):
    n = int(turns * 360 / step_deg)
    return [((r0 + (r1 - r0) * i / n) * math.cos(math.radians(step_deg * i)), (r0 + (r1 - r0) * i / n) * math.sin(math.radians(step_deg * i))) for i in range(n + 1)]


class Scale:
    """🧱 Part collector of one scale: body solids with their thermal regions, own thermal and structure solids, and the shapes of the three viewpoints."""

    def __init__(self, denominator):
        self.d = denominator
        self.body, self.regions, self.faces, self.thermal_own, self.structure = [], {}, {}, [], []
        self.views = {"elevation": [], "front": [], "plan": []}

    def add(self, solid, region=None, faces=None):
        self.body.append(solid)
        if region:
            self.regions[solid["id"]] = region
        if faces:
            self.faces[solid["id"]] = faces
        return solid

    def own(self, solid, region, tags=None):
        solid = {**solid, "region": region}
        if tags:
            solid["tags"] = tags
        self.thermal_own.append(solid)

    def member(self, solid, member, part, anchor=None):
        solid = {**solid, "member": member, "part": part}
        if anchor:
            solid["anchor"] = anchor
        self.structure.append(solid)

    def ref(self, id, of, member, part):
        self.structure.append({"id": id, "of": of, "member": member, "part": part})

    def shape(self, view, shape):
        self.views[view].append(shape)

    def thermal(self):
        out = []
        for s in self.body:
            if s["id"] in self.regions:
                t = {"id": "th-" + s["id"], "of": s["id"], "region": self.regions[s["id"]]}
                if s["id"] in self.faces:
                    t["sectionFaces"] = self.faces[s["id"]]
                out.append(t)
        return out + self.thermal_own

    def volume(self, decisions):
        return {"status": "authored", "denominator": self.d, "body": self.body, "thermal": self.thermal(), "structure": self.structure, "decisions": decisions}


def base(id, bone, material, tone, role="body"):
    return {"id": id, "kind": None, "bone": bone, "material": material, "role": role, "tone": tone}


def rev(id, bone, profile, axis, origin, material, tone, role="body", repeat=None, **extra):
    s = {**base(id, bone, material, tone, role), "kind": "revolve", "profile": pts(profile), "axis": axis, "origin": vec(origin)}
    if repeat:
        s["repeat"] = repeat
    s.update(extra)
    return s


def box(id, bone, size, centre, material, tone, role="body", radius=0, **extra):
    s = {**base(id, bone, material, tone, role), "kind": "box", "size": vec(size)}
    if radius:
        s["cornerRadius"] = radius
    s["transform"] = {"translation": vec(centre)}
    s.update(extra)
    return s


def ext(id, bone, plane, profile, lo, hi, material, tone, role="body", **extra):
    s = {**base(id, bone, material, tone, role), "kind": "extrude", "profile": pts(profile), "plane": plane, "from": r5(lo), "to": r5(hi)}
    s.update(extra)
    return s


def loop(points):
    return list(points) + [points[0]]


def corners():
    return {"count": 4, "rotationRad": r5(math.pi / 2), "rotationAxis": [0, 1, 0], "origin": [0, 0, 0]}


def shape(id, bone, kind, role, z, tone, region=None, regions=None, material=None, filled=True, **geo):
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": kind}
    for k, v in geo.items():
        s[k] = pts(v) if k == "points" else (r5(v) if isinstance(v, float) else v)
    if kind == "poly" and "closed" not in s:
        s["closed"] = True
    if region:
        s["region"] = region
    if regions:
        s["regions"] = regions
    if material:
        s["material"] = material
    return s


def rect(id, bone, x0, y0, x1, y1, role, z, tone, **kw):
    return shape(id, bone, "rect", role, z, tone, x=r5(x0), y=r5(y0), w=r5(x1 - x0), h=r5(y1 - y0), **kw)


def poly(id, bone, points, role, z, tone, **kw):
    return shape(id, bone, "poly", role, z, tone, points=points, **kw)


def ellipse(id, bone, cx, cy, r, role, z, tone, **kw):
    return shape(id, bone, "ellipse", role, z, tone, cx=r5(cx), cy=r5(cy), rx=r5(r), ry=r5(r), **kw)


def path(id, bone, rings, role, z, tone, **kw):
    d = " ".join("M " + " L ".join(f"{r5(a)} {r5(b)}" for a, b in ring) + " Z" for ring in rings)
    return shape(id, bone, "path", role, z, tone, d=d, **kw)


def feet_side(plate_y0, grommet):
    """🦶 Side outline (x, y) of plate and two grommets, the same seen from the side and from the front."""
    ear = HOLE + EAR
    right = [(ear, Y_PLATE), (ear, plate_y0)]
    for x, y in grommet:
        right.append((HOLE + x, y))
    for x, y in reversed(grommet):
        right.append((HOLE - x, y))
    right.append((HOLE - grommet[0][0], plate_y0))
    return right + [(-x, y) for x, y in reversed(right)]


GROMMET_50 = [(0.019, Y_GROM), (0.019, 0.016), (0.0155, 0.014), (0.0155, 0.008), (0.02, 0.006), (0.02, 0.0)]
GROMMET_REV_50 = [(0, 0), (0, 0.02), (0.006, 0.02), (0.008, 0.0155), (0.014, 0.0155), (0.016, 0.019), (Y_GROM, 0.019), (Y_GROM, 0)]


def cap_pairs(nk, nc):
    return [(y, r) for r, y in cap_outline(nk, nc)]


def cap_side(nk, nc, dy=0.0):
    half = [(r, y + dy) for r, y in cap_outline(nk, nc)]
    return [(-x, y) for x, y in half[:-1]] + [half[-1]] + [(x, y) for x, y in reversed(half[:-1])]


def suction_side(boss_r, tube_r, boss_to=0.134, start=0.116):
    return [(-start, SUC_Y + boss_r), (-boss_to, SUC_Y + boss_r), (-boss_to, SUC_Y + tube_r), (-0.185, SUC_Y + tube_r), (-0.185, SUC_Y - tube_r), (-boss_to, SUC_Y - tube_r), (-boss_to, SUC_Y - boss_r), (-start, SUC_Y - boss_r)]


def discharge_side(boss_r, boss_y, tube_r, dy=0.0):
    return [(-boss_r, boss_y[0] + dy), (boss_r, boss_y[0] + dy), (boss_r, boss_y[1] + dy), (tube_r, boss_y[1] + dy), (tube_r, DOME_L + dy), (-tube_r, DOME_L + dy), (-tube_r, boss_y[1] + dy), (-boss_r, boss_y[1] + dy)]


def emitter_patch(sc, thick, rim):
    """♨️ D2 outlet patch of the dome plume: a crown-following skin around the discharge boss where the hot dome releases its plume."""
    rr = [0.017 + (rim - 0.017) * i / 6 for i in range(7)]
    outer = [(crown_y(r) + thick, r) for r in rr]
    inner = [(crown_y(r) - 0.0005, r) for r in reversed(rr)]
    sc.own(rev("th-dome-plume-outlet", "dome", loop(outer + inner), [0, 1, 0], [0, 0, 0], "steel-shell", 0.9, "detail"), "dome", ["dome-plume"])


def wrap_loft(id, r, turns, step, z=-0.131):
    secs = [{"at": vec((x, y, z)), "rx": r, "ry": r} for x, y in spiral(0.007, 0.0322, turns, step)]
    return {**base(id, "rotor", "cast-aluminium", 0.1, "detail"), "kind": "loft", "sections": secs, "capStart": True, "capEnd": True, "slender": True}


def s50():
    """🔩 1:50, smallest feature 7.5 mm: hourglass grommets with bolts, eared base plate, shell with its lap under the torispherical top cap, both fittings with their weld bosses, terminal cover with gland, the bolted window with glass and the orbiting scroll with its wrap."""
    sc = Scale(50)
    m = sc.add
    m(rev("grommets", "shell", GROMMET_REV_50, [0, 1, 0], [HOLE, 0, HOLE], "epdm-rubber", 0.15, repeat=corners()), "feet")
    m(rev("mount-bolts", "shell", [(Y_PLATE, 0), (Y_PLATE, 0.012), (Y_PLATE + 0.004, 0.012), (Y_PLATE + 0.004, 0.0075), (Y_PLATE + 0.0115, 0.0075), (Y_PLATE + 0.0115, 0)], [0, 1, 0], [HOLE, 0, HOLE], "steel-shell", 0.3, "detail", repeat=corners()), "feet")
    m(ext("base-plate", "shell", "zx", [(z, x) for x, z in plate_outline()], Y_GROM, Y_PLATE, "steel-shell", 0.35), "feet")
    m(rev("shell", "shell", [(Y_PLATE, 0), (Y_PLATE, RS - 0.002), (Y_PLATE + 0.002, RS), (Y_SEAM, RS), (Y_SEAM, 0)], [0, 1, 0], [0, 0, 0], "steel-shell", 0.5), "shell", ["+v"])
    m(rev("top-cap", "dome", [(0, 0)] + cap_pairs(8, 8), [0, 1, 0], [0, 0, 0], "steel-shell", 0.6), "dome", ["-v"])
    m(rev("discharge-boss", "dome", [(0.069, 0), (0.069, 0.016), (0.0825, 0.016), (0.0825, 0)], [0, 1, 0], [0, 0, 0], "steel-shell", 0.45), "discharge")
    m(rev("discharge-tube", "dome", [(0.08, 0), (0.08, 0.00635), (DOME_L, 0.00635), (DOME_L, 0.0053), (DOME_L - 0.01, 0.0053), (DOME_L - 0.01, 0)], [0, 1, 0], [0, 0, 0], "copper", 0.75), "discharge")
    m(rev("suction-boss", "shell", [(0.105, 0), (0.105, 0.017), (0.134, 0.017), (0.134, 0)], [-1, 0, 0], [0, SUC_Y, 0], "steel-shell", 0.45), "suction")
    m(rev("suction-tube", "shell", [(0.128, 0), (0.128, 0.0111), (0.185, 0.0111), (0.185, 0.0098), (0.175, 0.0098), (0.175, 0)], [-1, 0, 0], [0, SUC_Y, 0], "copper", 0.75), "suction")
    m(box("terminal-cover", "terminal", TERM_BOX, (TERM_BOX[0] / 2, TERM_BOX[1] / 2, 0), "pa-plastic", 0.25, radius=0.006), "terminal", ["-u"])
    m(rev("terminal-gland", "terminal", [(-0.002, 0), (-0.002, 0.0095), (0.008, 0.0095), (0.008, 0.0075), (0.016, 0.0075), (0.016, 0)], [0, -1, 0], [0.026, 0, 0], "pa-plastic", 0.2, "detail"), "terminal")
    m(rev("window-boss", "shell", loop([(0.10, 0.040), (0.10, 0.052), (0.1365, 0.052), (0.1365, 0.040)]), [0, 0, -1], [0, WIN_Y, 0], "steel-shell", 0.4), "shell")
    m(rev("window-bezel", "shell", loop([(0.1365, 0.036), (0.1365, 0.053), (0.1455, 0.053), (0.1455, 0.036)]), [0, 0, -1], [0, WIN_Y, 0], "steel-shell", 0.35), "shell")
    m(rev("sight-glass", "shell", [(0.1335, 0), (0.1335, 0.0399), (0.1365, 0.0399), (0.1365, 0)], [0, 0, -1], [0, WIN_Y, 0], "borosilicate-glass", 0.9, "glazing"))
    m(rev("scroll-plate", "rotor", [(0.121, 0), (0.121, 0.038), (0.1285, 0.038), (0.1285, 0)], [0, 0, -1], [0, 0, 0], "cast-aluminium", 0.75), "scroll")
    m(wrap_loft("scroll-wrap", 0.0025, 1.8, 15), "scroll")
    emitter_patch(sc, 0.002, 0.06)
    st = sc.member
    st(rev("shell-vessel", "shell", loop([(Y_PLATE, RS - 0.0035), (Y_PLATE, RS), (Y_SEAM, RS), (Y_SEAM, RS - 0.0035)]), [0, 1, 0], [0, 0, 0], "steel-shell", 0.5), "shell", "member")
    st(rev("lower-cap", "shell", [(Y_PLATE, 0), (Y_PLATE, RS - 0.0035), (Y_PLATE + 0.012, RS - 0.0035), (Y_PLATE + 0.012, RS - 0.02), (Y_PLATE + 0.0045, RS - 0.03), (Y_PLATE + 0.0045, 0)], [0, 1, 0], [0, 0, 0], "steel-shell", 0.5), "shell", "member")
    sc.ref("base-plate-member", "base-plate", "shell", "member")
    st(rev("bearing-frame", "shell", [(0.232, 0), (0.232, 0.03), (0.24, 0.03), (0.24, RS - 0.0035), (0.256, RS - 0.0035), (0.256, 0.04), (0.268, 0.04), (0.268, 0)], [0, 1, 0], [0, 0, 0], "steel-shell", 0.45), "shell", "member")
    st(rev("fixed-scroll", "shell", [(0.28, 0), (0.28, RS - 0.0035), (0.296, RS - 0.0035), (0.296, 0)], [0, 1, 0], [0, 0, 0], "cast-aluminium", 0.55), "shell", "member")
    st(rev("crankshaft", "shell", [(0.05, 0), (0.05, 0.0125), (0.268, 0.0125), (0.268, 0)], [0, 1, 0], [0, 0, 0], "steel-shell", 0.3), "shell", "member")
    st(box("lower-bearing-bridge", "shell", (2 * (RS - 0.0035), 0.012, 0.03), (0, 0.06, 0), "steel-shell", 0.4, repeat={"count": 2, "rotationRad": r5(math.pi / 2), "rotationAxis": [0, 1, 0], "origin": [0, 0, 0]}), "shell", "member")
    st(rev("joint-base-weld", "shell", loop([(Y_PLATE, RS - 0.004), (Y_PLATE, RS + 0.004), (Y_PLATE + 0.008, RS)]), [0, 1, 0], [0, 0, 0], "steel-shell", 0.3), "shell", "joint")
    st(rev("top-cap-vessel", "dome", loop([(y, r) for r, y in cap_outline(8, 8)] + [(PEAK - 0.0035, 0)] + [(y - 0.0035, max(r - 0.0035, 0.0)) for r, y in reversed(cap_outline(8, 8)[1:-1])] + [(0, RCAP - 0.0035)]), [0, 1, 0], [0, 0, 0], "steel-shell", 0.6), "dome", "member")
    st(rev("joint-lap-weld", "dome", loop([(-0.004, RCAP), (0.004, RCAP), (0, RCAP + 0.004)]), [0, 1, 0], [0, 0, 0], "steel-shell", 0.3), "dome", "joint")
    st(box("terminal-fence", "terminal", (0.012, 0.06, 0.056), (0.003, TERM_BOX[1] / 2, 0), "steel-shell", 0.35), "terminal", "member")
    sc.ref("orbiting-scroll", "scroll-plate", "rotor", "member")
    st(rev("joint-scroll-bearing", "rotor", [(0.10, 0), (0.10, 0.0125), (0.121, 0.0125), (0.121, 0)], [0, 0, -1], [0, 0, 0], "steel-shell", 0.3), "rotor", "joint")
    st(rev("mass-motor", "shell", [(0.11, 0), (0.11, 0.1), (0.23, 0.1), (0.23, 0)], [0, 1, 0], [0, 0, 0], "steel-shell", 0.6), "shell", "mass")
    st(rev("contact-grommets", "shell", [(0, 0), (0, 0.02), (0.0075, 0.02), (0.0075, 0)], [0, 1, 0], [HOLE, 0, HOLE], "epdm-rubber", 0.2, repeat=corners()), "shell", "support", "base-contact")
    e = lambda s: sc.shape("elevation", s)
    feet = feet_side(Y_GROM, GROMMET_50)
    e(poly("feet", "shell", feet, "body", 3.1, 0.35, region="feet", material="steel-shell"))
    e(path("mount-bolts", "shell", [[(sx * HOLE - 0.012, Y_PLATE), (sx * HOLE + 0.012, Y_PLATE), (sx * HOLE + 0.012, Y_PLATE + 0.004), (sx * HOLE + 0.0075, Y_PLATE + 0.004), (sx * HOLE + 0.0075, Y_PLATE + 0.0115), (sx * HOLE - 0.0075, Y_PLATE + 0.0115), (sx * HOLE - 0.0075, Y_PLATE + 0.004), (sx * HOLE - 0.012, Y_PLATE + 0.004)] for sx in (-1, 1)], "detail", 3.6, 0.3, region="feet", material="steel-shell"))
    e(rect("shell", "shell", -RS, Y_PLATE, RS, Y_SEAM, "body", 3.4, 0.5, region="shell", material="steel-shell"))
    e(poly("suction-stub", "shell", suction_side(0.017, 0.0111), "body", 3.2, 0.7, region="suction", material="copper"))
    e(ellipse("sight-ring", "shell", 0, WIN_Y, 0.053, "body", 3.6, 0.35, region="shell", material="steel-shell"))
    e(ellipse("scroll-disc", "rotor", 0, 0, 0.036, "glazing", 3.7, 0.8, region="scroll", material="cast-aluminium"))
    e(poly("spiral", "rotor", spiral(0.007, 0.0322, 1.8, 15), "detail", 3.8, 0.1, region="scroll", material="cast-aluminium", filled=False, closed=False))
    e(rect("terminal", "terminal", 0, 0, TERM_BOX[0], TERM_BOX[1], "body", 3.5, 0.25, region="terminal", material="pa-plastic"))
    e(poly("terminal-gland", "terminal", [(0.0165, 0.001), (0.0355, 0.001), (0.0355, -0.012), (0.0335, -0.012), (0.0335, -0.016), (0.0185, -0.016), (0.0185, -0.012), (0.0165, -0.012)], "detail", 3.55, 0.2, region="terminal", material="pa-plastic"))
    e(poly("dome", "dome", cap_side(4, 4), "body", 3.3, 0.6, region="dome", material="steel-shell"))
    e(poly("discharge-stub", "dome", discharge_side(0.016, (0.07, 0.0825), 0.00635), "body", 3.2, 0.75, region="discharge", material="copper"))
    f = lambda s: sc.shape("front", s)
    f(poly("feet", "shell", feet, "body", 3.1, 0.35, region="feet", material="steel-shell"))
    f(path("mount-bolts", "shell", [[(sx * HOLE - 0.012, Y_PLATE), (sx * HOLE + 0.012, Y_PLATE), (sx * HOLE + 0.012, Y_PLATE + 0.004), (sx * HOLE + 0.0075, Y_PLATE + 0.004), (sx * HOLE + 0.0075, Y_PLATE + 0.0115), (sx * HOLE - 0.0075, Y_PLATE + 0.0115), (sx * HOLE - 0.0075, Y_PLATE + 0.004), (sx * HOLE - 0.012, Y_PLATE + 0.004)] for sx in (-1, 1)], "detail", 3.6, 0.3, region="feet", material="steel-shell"))
    f(rect("shell", "shell", -RS, Y_PLATE, RS, Y_SEAM, "body", 3.4, 0.5, region="shell", material="steel-shell"))
    f(poly("sight-ring", "shell", [(-0.116, WIN_Y - 0.052), (-0.1365, WIN_Y - 0.052), (-0.1365, WIN_Y - 0.053), (-0.1455, WIN_Y - 0.053), (-0.1455, WIN_Y + 0.053), (-0.1365, WIN_Y + 0.053), (-0.1365, WIN_Y + 0.052), (-0.116, WIN_Y + 0.052)], "body", 3.6, 0.35, region="shell", material="steel-shell"))
    f(rect("terminal", "terminal", -TERM_BOX[2] / 2, TERM[1], TERM_BOX[2] / 2, TERM[1] + TERM_BOX[1], "body", 3.5, 0.25, region="terminal", material="pa-plastic"))
    f(poly("terminal-gland", "terminal", [(-0.0095, TERM[1] + 0.001), (0.0095, TERM[1] + 0.001), (0.0095, TERM[1] - 0.012), (0.0075, TERM[1] - 0.012), (0.0075, TERM[1] - 0.016), (-0.0075, TERM[1] - 0.016), (-0.0075, TERM[1] - 0.012), (-0.0095, TERM[1] - 0.012)], "detail", 3.55, 0.2, region="terminal", material="pa-plastic"))
    f(poly("dome", "dome", cap_side(4, 4, Y_SEAM), "body", 3.3, 0.6, region="dome", material="steel-shell"))
    f(poly("discharge-stub", "dome", discharge_side(0.016, (0.07, 0.0825), 0.00635, Y_SEAM), "body", 3.2, 0.75, region="discharge", material="copper"))
    p = lambda s: sc.shape("plan", s)
    p(poly("feet", "shell", plate_outline(4, 6), "body", 3.1, 0.35, region="feet", material="steel-shell"))
    p(path("mount-bolts", "shell", [circle(sx * HOLE, sz * HOLE, 0.012, 12) for sx in (-1, 1) for sz in (-1, 1)], "detail", 3.2, 0.3, region="feet", material="steel-shell"))
    p(ellipse("shell", "shell", 0, 0, RS, "body", 3.3, 0.5, region="shell", material="steel-shell"))
    p(poly("suction-stub", "shell", [(x, y - SUC_Y) for x, y in suction_side(0.017, 0.0111)], "body", 3.45, 0.7, region="suction", material="copper"))
    p(poly("sight-ring", "shell", [(-0.052, -0.116), (-0.052, -0.1365), (-0.053, -0.1365), (-0.053, -0.1455), (0.053, -0.1455), (0.053, -0.1365), (0.052, -0.1365), (0.052, -0.116)], "body", 3.4, 0.35, region="shell", material="steel-shell"))
    p(rect("terminal", "terminal", TERM[0], -TERM_BOX[2] / 2, TERM[0] + TERM_BOX[0], TERM_BOX[2] / 2, "body", 3.5, 0.25, region="terminal", material="pa-plastic"))
    p(ellipse("dome", "dome", 0, 0, RCAP, "body", 3.8, 0.6, region="dome", material="steel-shell"))
    p(ellipse("discharge-stub", "dome", 0, 0, 0.016, "body", 3.9, 0.45, region="discharge", material="steel-shell"))
    p(ellipse("discharge-tube", "dome", 0, 0, 0.00635, "detail", 3.95, 0.75, region="discharge", material="copper"))
    return sc


def s100():
    """🔧 1:100, smallest feature 15 mm: grommets as plain pads, a 15 mm plate, shell, five-point cap, fittings with merged bosses, the terminal cover alone, the window as one ring with the scroll disc and its wrap."""
    sc = Scale(100)
    m = sc.add
    m(rev("grommets", "shell", [(0, 0), (0, 0.02), (0.0225, 0.02), (0.0225, 0)], [0, 1, 0], [HOLE, 0, HOLE], "epdm-rubber", 0.15, repeat=corners()), "feet")
    m(ext("base-plate", "shell", "zx", [(z, x) for x, z in plate_outline(5, 4)], 0.0145, Y_PLATE, "steel-shell", 0.35), "feet")
    m(rev("shell", "shell", [(Y_PLATE, 0), (Y_PLATE, RS), (Y_SEAM, RS), (Y_SEAM, 0)], [0, 1, 0], [0, 0, 0], "steel-shell", 0.5), "shell", ["+v"])
    m(rev("top-cap", "dome", [(0, 0)] + cap_pairs(3, 3), [0, 1, 0], [0, 0, 0], "steel-shell", 0.6), "dome", ["-v"])
    m(rev("discharge-stub", "dome", [(0.069, 0), (0.069, 0.016), (0.0825, 0.016), (0.0825, 0.0075), (DOME_L, 0.0075), (DOME_L, 0)], [0, 1, 0], [0, 0, 0], "copper", 0.75), "discharge")
    m(rev("suction-stub", "shell", [(0.105, 0), (0.105, 0.017), (0.134, 0.017), (0.134, 0.0111), (0.185, 0.0111), (0.185, 0)], [-1, 0, 0], [0, SUC_Y, 0], "copper", 0.75), "suction")
    m(box("terminal-cover", "terminal", TERM_BOX, (TERM_BOX[0] / 2, TERM_BOX[1] / 2, 0), "pa-plastic", 0.25, radius=0.006), "terminal", ["-u"])
    m(rev("window-ring", "shell", loop([(0.10, 0.036), (0.10, 0.053), (0.1455, 0.053), (0.1455, 0.036)]), [0, 0, -1], [0, WIN_Y, 0], "steel-shell", 0.35), "shell")
    m(rev("scroll-plate", "rotor", [(0.1215, 0), (0.1215, 0.038), (0.1365, 0.038), (0.1365, 0)], [0, 0, -1], [0, 0, 0], "cast-aluminium", 0.75), "scroll")
    m(wrap_loft("scroll-wrap", 0.0025, 1.3, 20, -0.139), "scroll")
    emitter_patch(sc, 0.002, 0.09)
    st = sc.member
    st(rev("shell-vessel", "shell", loop([(Y_PLATE, RS - 0.0035), (Y_PLATE, RS), (Y_SEAM, RS), (Y_SEAM, RS - 0.0035)]), [0, 1, 0], [0, 0, 0], "steel-shell", 0.5), "shell", "member")
    st(rev("lower-cap", "shell", [(Y_PLATE, 0), (Y_PLATE, RS - 0.0035), (Y_PLATE + 0.015, RS - 0.0035), (Y_PLATE + 0.015, 0)], [0, 1, 0], [0, 0, 0], "steel-shell", 0.5), "shell", "member")
    sc.ref("base-plate-member", "base-plate", "shell", "member")
    st(rev("bearing-frame", "shell", [(0.236, 0), (0.236, RS - 0.0035), (0.256, RS - 0.0035), (0.256, 0)], [0, 1, 0], [0, 0, 0], "steel-shell", 0.45), "shell", "member")
    st(rev("crankshaft", "shell", [(0.05, 0), (0.05, 0.0125), (0.256, 0.0125), (0.256, 0)], [0, 1, 0], [0, 0, 0], "steel-shell", 0.3), "shell", "member")
    st(rev("top-cap-vessel", "dome", loop([(y, r) for r, y in cap_outline(3, 3)] + [(PEAK - 0.0035, 0)] + [(y - 0.0035, max(r - 0.0035, 0.0)) for r, y in reversed(cap_outline(3, 3)[1:-1])] + [(0, RCAP - 0.0035)]), [0, 1, 0], [0, 0, 0], "steel-shell", 0.6), "dome", "member")
    st(box("terminal-fence", "terminal", (0.015, 0.06, 0.056), (0.0045, TERM_BOX[1] / 2, 0), "steel-shell", 0.35), "terminal", "member")
    sc.ref("orbiting-scroll", "scroll-plate", "rotor", "member")
    st(rev("joint-scroll-bearing", "rotor", [(0.10, 0), (0.10, 0.015), (0.121, 0.015), (0.121, 0)], [0, 0, -1], [0, 0, 0], "steel-shell", 0.3), "rotor", "joint")
    st(rev("mass-motor", "shell", [(0.11, 0), (0.11, 0.1), (0.23, 0.1), (0.23, 0)], [0, 1, 0], [0, 0, 0], "steel-shell", 0.6), "shell", "mass")
    st(rev("contact-grommets", "shell", [(0, 0), (0, 0.0225), (0.015, 0.0225), (0.015, 0)], [0, 1, 0], [HOLE, 0, HOLE], "epdm-rubber", 0.2, repeat=corners()), "shell", "support", "base-contact")
    grom = [(0.0225, 0.0145), (0.0225, 0.0)]
    feet = feet_side(0.0145, grom)
    e = lambda s: sc.shape("elevation", s)
    e(poly("feet", "shell", feet, "body", 3.1, 0.35, region="feet", material="steel-shell"))
    e(rect("shell", "shell", -RS, Y_PLATE, RS, Y_SEAM, "body", 3.4, 0.5, region="shell", material="steel-shell"))
    e(poly("suction-stub", "shell", suction_side(0.017, 0.0111), "body", 3.2, 0.7, region="suction", material="copper"))
    e(ellipse("sight-ring", "shell", 0, WIN_Y, 0.053, "body", 3.6, 0.35, region="shell", material="steel-shell"))
    e(ellipse("scroll-disc", "rotor", 0, 0, 0.036, "body", 3.7, 0.8, region="scroll", material="cast-aluminium"))
    e(poly("spiral", "rotor", spiral(0.007, 0.0322, 1.3, 20), "detail", 3.8, 0.1, region="scroll", material="cast-aluminium", filled=False, closed=False))
    e(rect("terminal", "terminal", 0, 0, TERM_BOX[0], TERM_BOX[1], "body", 3.5, 0.25, region="terminal", material="pa-plastic"))
    e(poly("dome", "dome", cap_side(2, 2), "body", 3.3, 0.6, region="dome", material="steel-shell"))
    e(poly("discharge-stub", "dome", discharge_side(0.016, (0.07, 0.0825), 0.0075), "body", 3.2, 0.75, region="discharge", material="copper"))
    f = lambda s: sc.shape("front", s)
    f(poly("feet", "shell", feet, "body", 3.1, 0.35, region="feet", material="steel-shell"))
    f(rect("shell", "shell", -RS, Y_PLATE, RS, Y_SEAM, "body", 3.4, 0.5, region="shell", material="steel-shell"))
    f(rect("sight-ring", "shell", -0.1455, WIN_Y - 0.053, -0.116, WIN_Y + 0.053, "body", 3.6, 0.35, region="shell", material="steel-shell"))
    f(rect("terminal", "terminal", -TERM_BOX[2] / 2, TERM[1], TERM_BOX[2] / 2, TERM[1] + TERM_BOX[1], "body", 3.5, 0.25, region="terminal", material="pa-plastic"))
    f(poly("dome", "dome", cap_side(2, 2, Y_SEAM), "body", 3.3, 0.6, region="dome", material="steel-shell"))
    f(poly("discharge-stub", "dome", discharge_side(0.016, (0.07, 0.0825), 0.0075, Y_SEAM), "body", 3.2, 0.75, region="discharge", material="copper"))
    p = lambda s: sc.shape("plan", s)
    p(poly("feet", "shell", plate_outline(3, 4), "body", 3.1, 0.35, region="feet", material="steel-shell"))
    p(ellipse("shell", "shell", 0, 0, RS, "body", 3.3, 0.5, region="shell", material="steel-shell"))
    p(poly("suction-stub", "shell", [(x, y - SUC_Y) for x, y in suction_side(0.017, 0.0111)], "body", 3.45, 0.7, region="suction", material="copper"))
    p(rect("sight-ring", "shell", -0.053, -0.1455, 0.053, -0.116, "body", 3.4, 0.35, region="shell", material="steel-shell"))
    p(rect("terminal", "terminal", TERM[0], -TERM_BOX[2] / 2, TERM[0] + TERM_BOX[0], TERM_BOX[2] / 2, "body", 3.5, 0.25, region="terminal", material="pa-plastic"))
    p(ellipse("dome", "dome", 0, 0, RCAP, "body", 3.8, 0.6, region="dome", material="steel-shell"))
    p(ellipse("discharge-stub", "dome", 0, 0, 0.016, "body", 3.9, 0.45, region="discharge", material="copper"))
    return sc


SQ = 0.11


def s200():
    """🪨 1:200, smallest feature 30 mm: the plate and grommets one 30 mm foot block whose corners are the ears, shell, three-point cap, 30 mm stubs and the terminal box; the window is below the scale and lives in thermal only."""
    sc = Scale(200)
    m = sc.add
    foot = [(SQ, SQ - 0.012), (SQ, -SQ + 0.012), (SQ - 0.012, -SQ), (-SQ + 0.012, -SQ), (-SQ, -SQ + 0.012), (-SQ, SQ - 0.012), (-SQ + 0.012, SQ), (SQ - 0.012, SQ)]
    m(ext("feet", "shell", "zx", [(z, x) for x, z in foot], 0, Y_PLATE, "steel-shell", 0.35), "feet")
    m(rev("shell", "shell", [(Y_PLATE, 0), (Y_PLATE, RS), (Y_SEAM, RS), (Y_SEAM, 0)], [0, 1, 0], [0, 0, 0], "steel-shell", 0.5), "shell", ["+v"])
    m(rev("top-cap", "dome", [(0, 0)] + cap_pairs(2, 2), [0, 1, 0], [0, 0, 0], "steel-shell", 0.6), "dome", ["-v"])
    m(rev("discharge-stub", "dome", [(0.06, 0), (0.06, 0.015), (DOME_L, 0.015), (DOME_L, 0)], [0, 1, 0], [0, 0, 0], "copper", 0.75), "discharge")
    m(rev("suction-stub", "shell", [(0.11, 0), (0.11, 0.015), (0.185, 0.015), (0.185, 0)], [-1, 0, 0], [0, SUC_Y, 0], "copper", 0.75), "suction")
    m(box("terminal", "terminal", TERM_BOX, (TERM_BOX[0] / 2, TERM_BOX[1] / 2, 0), "pa-plastic", 0.25), "terminal", ["-u"])
    m(rev("window", "shell", [(0.10, 0), (0.10, 0.053), (0.1455, 0.053), (0.1455, 0)], [0, 0, -1], [0, WIN_Y, 0], "steel-shell", 0.35), "shell")
    sc.own(rev("th-scroll-window", "rotor", [(0.1165, 0), (0.1165, 0.045), (0.1465, 0.045), (0.1465, 0)], [0, 0, -1], [0, 0, 0], "cast-aluminium", 0.8), "scroll")
    emitter_patch(sc, 0.009, 0.104)
    st = sc.member
    st(rev("shell-vessel", "shell", loop([(Y_PLATE, RS - 0.0035), (Y_PLATE, RS), (Y_SEAM + SKIRT, RS), (Y_SEAM + SKIRT, RS - 0.0035)]), [0, 1, 0], [0, 0, 0], "steel-shell", 0.5), "shell", "member")
    st(ext("foot-frame", "shell", "zx", [(z, x) for x, z in foot], 0.0, Y_PLATE + 0.03, "steel-shell", 0.35, holes=[[(z, x) for x, z in circle(0, 0, 0.07, 16)]]), "shell", "member")
    st(rev("bearing-frame", "shell", [(0.236, 0), (0.236, RS - 0.0035), (0.266, RS - 0.0035), (0.266, 0)], [0, 1, 0], [0, 0, 0], "steel-shell", 0.45), "shell", "member")
    st(rev("crankshaft", "shell", [(0.06, 0), (0.06, 0.015), (0.236, 0.015), (0.236, 0)], [0, 1, 0], [0, 0, 0], "steel-shell", 0.3), "shell", "member")
    st(rev("top-cap-shell", "dome", loop([(y, r) for r, y in cap_outline(2, 2)[1:]] + [(PEAK - 0.0035, 0)] + [(y - 0.0035, max(r - 0.0035, 0.0)) for r, y in reversed(cap_outline(2, 2)[1:-1])]), [0, 1, 0], [0, 0, 0], "steel-shell", 0.6), "dome", "member")
    st(box("terminal-fence", "terminal", (0.03, 0.06, 0.056), (0.0135, TERM_BOX[1] / 2, 0), "steel-shell", 0.35), "terminal", "member")
    st(rev("orbiting-scroll", "rotor", [(0.09, 0), (0.09, 0.04), (0.12, 0.04), (0.12, 0)], [0, 0, -1], [0, 0, 0], "cast-aluminium", 0.75), "rotor", "member")
    st(rev("mass-motor", "shell", [(0.11, 0), (0.11, 0.1), (0.23, 0.1), (0.23, 0)], [0, 1, 0], [0, 0, 0], "steel-shell", 0.6), "shell", "mass")
    st(box("contact-pad", "shell", (2 * SQ, Y_PLATE, 2 * SQ), (0, Y_PLATE / 2, 0), "epdm-rubber", 0.2), "shell", "support", "base-contact")
    e = lambda s: sc.shape("elevation", s)
    e(rect("feet", "shell", -SQ, 0, SQ, Y_PLATE, "body", 3.5, 0.35, region="feet", material="steel-shell"))
    e(rect("shell", "shell", -RS, Y_PLATE, RS, Y_SEAM, "body", 3.4, 0.5, region="shell", material="steel-shell"))
    e(rect("suction-stub", "shell", -0.185, SUC_Y - 0.015, -0.116, SUC_Y + 0.015, "body", 3.2, 0.7, region="suction", material="copper"))
    e(rect("terminal", "terminal", 0, 0, TERM_BOX[0], TERM_BOX[1], "body", 3.5, 0.25, region="terminal", material="pa-plastic"))
    e(ellipse("sight-ring", "shell", 0, WIN_Y, 0.053, "body", 3.6, 0.4, region="scroll", material="steel-shell"))
    cap3 = cap_side(2, 2)
    e(poly("dome", "dome", cap3, "body", 3.3, 0.6, region="dome", material="steel-shell"))
    e(rect("discharge-stub", "dome", -0.015, 0.06, 0.015, DOME_L, "body", 3.2, 0.75, region="discharge", material="copper"))
    f = lambda s: sc.shape("front", s)
    f(rect("feet", "shell", -SQ, 0, SQ, Y_PLATE, "body", 3.5, 0.35, region="feet", material="steel-shell"))
    f(rect("shell", "shell", -RS, Y_PLATE, RS, Y_SEAM, "body", 3.4, 0.5, region="shell", material="steel-shell"))
    f(rect("terminal", "terminal", -TERM_BOX[2] / 2, TERM[1], TERM_BOX[2] / 2, TERM[1] + TERM_BOX[1], "body", 3.5, 0.25, region="terminal", material="pa-plastic"))
    f(rect("sight-ring", "shell", -0.1455, WIN_Y - 0.053, -0.116, WIN_Y + 0.053, "body", 3.6, 0.4, region="scroll", material="steel-shell"))
    f(poly("dome", "dome", [(x, y + Y_SEAM) for x, y in cap3], "body", 3.3, 0.6, region="dome", material="steel-shell"))
    f(rect("discharge-stub", "dome", -0.015, Y_SEAM + 0.06, 0.015, TOP, "body", 3.2, 0.75, region="discharge", material="copper"))
    p = lambda s: sc.shape("plan", s)
    p(poly("feet", "shell", foot, "body", 3.1, 0.35, region="feet", material="steel-shell"))
    p(ellipse("shell", "shell", 0, 0, RS, "body", 3.4, 0.5, region="shell", material="steel-shell"))
    p(rect("suction-stub", "shell", -0.185, -0.015, -0.116, 0.015, "body", 3.45, 0.7, region="suction", material="copper"))
    p(rect("terminal", "terminal", TERM[0], -TERM_BOX[2] / 2, TERM[0] + TERM_BOX[0], TERM_BOX[2] / 2, "body", 3.5, 0.25, region="terminal", material="pa-plastic"))
    p(rect("sight-ring", "shell", -0.053, -0.1455, 0.053, -0.116, "body", 3.4, 0.4, region="scroll", material="steel-shell"))
    p(ellipse("dome", "dome", 0, 0, RCAP, "body", 3.8, 0.6, region="dome", material="steel-shell"))
    p(ellipse("discharge-stub", "dome", 0, 0, 0.015, "body", 3.9, 0.75, region="discharge", material="copper"))
    return sc


AREAS = TEXTS["areas"]
MATERIALS = [
    {"id": "epdm-rubber", "title": {"en": "Rubber grommet", "de": "Gummitülle"}, "emissivity": 0.9, "solarAbsorptivity": 0.9, "densityKgM3": 1100, "conductivityWmK": 0.13, "specificHeatJKgK": 2010, "source": "literature", "reference": "incropera-2018"},
    {"id": "borosilicate-glass", "title": {"en": "Borosilicate sight glass", "de": "Borosilikat-Schauglas"}, "emissivity": 0.9, "solarAbsorptivity": 0.1, "densityKgM3": 2225, "conductivityWmK": 1.4, "specificHeatJKgK": 835, "source": "literature", "reference": "incropera-2018"},
]
BONES = {
    "shell": {"x": 0, "y": 0, "lengthM": Y_SEAM, "mass": {"kg": 22.6, "comX": 0, "comY": 0.17}},
    "dome": {"x": 0, "y": Y_SEAM, "lengthM": r5(DOME_L), "mass": {"kg": 2.2, "comX": 0, "comY": 0.04}},
    "terminal": {"x": TERM[0], "y": TERM[1], "lengthM": TERM_BOX[0], "mass": {"kg": 0.4, "comX": 0.024, "comY": 0.039}},
    "rotor": {"x": 0, "y": WIN_Y, "lengthM": 0.036, "mass": {"kg": 0.8, "comX": 0, "comY": 0}},
}


def build():
    """🏗️ Candidate asset document: catalog asset with every root member and the volumes replaced."""
    doc = json.loads(ASSET_FILE.read_text())
    a = doc["asset"]
    scales = {"50": s50(), "100": s100(), "200": s200()}
    a["summary"] = TEXTS["summary"]
    prov = a["provenance"]
    prov["references"] = [r for r in prov["references"] if r["id"] != "copeland-zr-k5"] + [{"id": "copeland-zr-k5", "citation": TEXTS["copelandRef"]}]
    prov["notes"] = TEXTS["provenanceNotes"]
    a["dimensions"] = TEXTS["dimensions"]
    a["materials"] = [x for x in a["materials"] if x["id"] not in {m["id"] for m in MATERIALS}] + MATERIALS
    for b in a["rig"]["bones"]:
        spec = BONES[b["id"]]
        b.update({k: spec[k] for k in ("x", "y", "lengthM")})
        b["mass"].update(spec["mass"])
        if b["id"] in TEXTS["boneTitles"]:
            b["title"] = TEXTS["boneTitles"][b["id"]]
    for anchor in a["anchors"]:
        if anchor["id"] == "base-contact":
            anchor.update({"footprintLengthM": 0.23, "footprintWidthM": 0.23})
        if anchor["id"] == "dome-top":
            anchor.update({"x": 0, "y": r5(DOME_L)})
    a["structure"]["assumptions"] = TEXTS["structureAssumptions"]
    for region in a["thermal"]["regions"]:
        if region["id"] in AREAS:
            region["areaM2"] = AREAS[region["id"]]
    reps = a["representations"]
    for key, sc in scales.items():
        reps[key]["geometry"] = sc.views
        reps[key]["decisions"] = TEXTS["representation"][key]
    reps["500"].update(TEXTS["unavailable500"])
    volumes = {key: sc.volume(TEXTS["volumes"][key]) for key, sc in scales.items()}
    volumes["500"] = {"status": "unavailable", "denominator": 500}
    a["volumes"] = volumes
    return doc, volumes


def apply(doc, volumes):
    """✍️ Writes the root members into the catalog file (re-read right before, member order kept), the manifest entry by a scoped text edit and the volumes source."""
    current = json.loads(ASSET_FILE.read_text())
    new = doc["asset"]
    for key in list(current["asset"].keys()):
        if key != "volumes":
            current["asset"][key] = new[key]
    ASSET_FILE.write_text(json.dumps(current, ensure_ascii=False, indent=2) + "\n")
    SOURCE_FILE.write_text(json.dumps({"volumes": volumes}, ensure_ascii=False, indent=1) + "\n")
    text = MANIFEST_FILE.read_text()
    start = text.index('"id": "scroll-compressor"')
    end = text.index('"completion"', start)
    old = next(e for e in json.loads(text)["entries"] if e["id"] == "scroll-compressor")
    part = text[start:end]
    part = part.replace(f'"widthM": {old["widthM"]},', f'"widthM": {new["dimensions"]["widthM"]},')
    for lang in ("en", "de"):
        part = part.replace(json.dumps(old["distinction"][lang], ensure_ascii=False), json.dumps(TEXTS["distinction"][lang], ensure_ascii=False))
    MANIFEST_FILE.write_text(text[:start] + part + text[end:])
    check = next(e for e in json.loads(MANIFEST_FILE.read_text())["entries"] if e["id"] == "scroll-compressor")
    assert check["widthM"] == new["dimensions"]["widthM"] and check["distinction"] == TEXTS["distinction"]


if __name__ == "__main__":
    doc, volumes = build()
    OUT.mkdir(parents=True, exist_ok=True)
    (OUT / "🔣️.json").write_text(json.dumps(doc, ensure_ascii=False, indent=2) + "\n")
    print("candidate", OUT / "🔣️.json")
    if "--apply" in sys.argv:
        apply(doc, volumes)
        print("applied")
