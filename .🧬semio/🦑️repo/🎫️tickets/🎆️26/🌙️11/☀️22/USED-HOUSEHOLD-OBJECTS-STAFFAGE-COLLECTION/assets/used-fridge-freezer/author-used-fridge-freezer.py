"""🧊 Authoritative chisel of the used fridge-freezer: states every hand-chosen number and writes `🔣️.json` next to this file.

A freestanding bottom-freezer fridge-freezer of the common German mid-range type of about 2005-2012 (600 x 650 x 1850 mm, A+), emptied,
cleaned and stored in a reuse depot on a flattened carton with its power cable coiled and taped to the back; a tall appliance hand truck
stands parked behind it. Rest = default pose `stored`. World frame of the staffage: x ahead (the door side), y up, z left.

@see ../../brief.md
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md
"""
import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE / "🔣️.json"

G = 9.80665
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
SCALES = (50, 100, 200, 500)


def r6(v):
    """🔢 Rounds every float of a nested value to six decimals."""
    if isinstance(v, float):
        return float(f"{v:.6g}") if 0 < abs(v) < 0.01 else round(v, 6) + 0.0
    if isinstance(v, (list, tuple)):
        return [r6(x) for x in v]
    if isinstance(v, dict):
        return {k: r6(x) for k, x in v.items()}
    return v


def T(en, de):
    """🗣 A bilingual text."""
    return {"en": en, "de": de}


CB = 0.003
FT = 0.018
XF, XG, XD = 0.530, 0.538, 0.588
YTOP = 1.838
WALL = 0.050
TOPW = 0.045
FZ = (0.090, 0.772)
FR = (0.776, 1.836)
DIV = (0.737, 0.807)
FRC = (0.807, YTOP - TOPW)
NICHE = (0.170, 0.237)
PLINTH = (0.505, 0.088)
ZC, ZI = 0.300, 0.250
ZD = 0.299
XA0 = -0.69
RW, WW, ZW = 0.130, 0.085, 0.2425
TUBE_X, TUBE_Z, TUBE_R, TOP_Y = 0.090, 0.19, 0.014, 1.366
XPE = -0.030 - 0.005 - (TUBE_X + TUBE_R)
D_E = XPE - XA0
PRE = 1.6
TILT_T = 38.0
TILT_F = TILT_T - PRE
ROLL_C = 1.30
ROLL_U = 1.90
DROP = -0.004
DOOR_OPEN = -100.0


def Y(yl):
    """↕ World rest height (stored, on the carton) of a fridge-local height above the foot soles."""
    return yl + CB


BONES = {
    "ground": ((0.0, 0.0, 0.0), "H"),
    "cardboard": ((-0.030, 0.0, 0.0), "H"),
    "truck-base": ((XA0, RW, 0.0), "H"),
    "truck-frame": ((XA0, RW, 0.0), "V"),
    "carry-roll": ((0.0, 0.0, 0.0), "H"),
    "carry-lift": ((0.0, 0.0, 0.0), "H"),
    "carry-tilt": ((XPE, RW, 0.0), "H"),
    "cabinet": ((0.265, Y(FT), 0.0), "V"),
    "condenser": ((-0.020, Y(0.330), 0.0), "V"),
    "fridge-door": ((XD, Y(FR[0]), -ZD), "V"),
    "freezer-door": ((XD, Y(FZ[0]), -ZD), "V"),
}


def L3(bone, x, y, z=0.0):
    """📍 World rest point to the (u, v, w) frame of a bone (horizontal: u ahead; vertical: u up, v backwards)."""
    (ox, oy, oz), k = BONES[bone]
    dx, dy, dz = x - ox, y - oy, z - oz
    return (dx, dy, dz) if k == "H" else (dy, -dx, dz)


def L2(bone, x, y):
    """📍 World rest (x, y) to the elevation frame (u, v) of a bone."""
    u, v, _ = L3(bone, x, y, BONES[bone][0][2])
    return (u, v)


def AX(bone, axis):
    """🧭 A world axis ('x', 'y', 'z') in the frame of a bone."""
    k = BONES[bone][1]
    if axis == "z":
        return [0, 0, 1]
    if k == "H":
        return [1, 0, 0] if axis == "x" else [0, 1, 0]
    return [0, -1, 0] if axis == "x" else [1, 0, 0]


# ---------------------------------------------------------------- materials and densities

MATS = [
    ("cabinet-wall", T("Foamed cabinet wall: white enamelled steel skin 0.5 mm, PU foam, HIPS liner 1.2 mm, yellowed (effective over the 50 mm wall)", "Geschäumte Gehäusewand: weiß einbrennlackiertes Stahlblech 0,5 mm, PU-Schaum, HIPS-Innenbehälter 1,2 mm, vergilbt (wirksam über die 50-mm-Wand)"), 0.90, 0.30, 140, 0.026, 1450, 2.1e11, 2.4e8, "authored-estimate", "en-62552-2013"),
    ("divider-foam", T("Foamed divider: two HIPS liners and PU foam", "Geschäumte Trennwand: zwei HIPS-Liner und PU-Schaum"), 0.90, 0.25, 90, 0.025, 1450, None, None, "authored-estimate", "iso-10456-2007"),
    ("door-panel", T("Foamed door: white enamelled steel skin 0.5 mm and PU foam, yellowed", "Geschäumte Tür: weiß einbrennlackiertes Stahlblech 0,5 mm und PU-Schaum, vergilbt"), 0.90, 0.30, 115, 0.026, 1300, 2.1e11, 2.4e8, "authored-estimate", "en-62552-2013"),
    ("door-liner", T("Door liner relief, white HIPS over PU foam", "Türinnenverkleidung, weißes HIPS auf PU-Schaum"), 0.90, 0.25, 90, 0.04, 1400, None, None, "authored-estimate", "iso-10456-2007"),
    ("pvc-gasket", T("Magnetic PVC door gasket, yellowed and stiff", "Magnetische PVC-Türdichtung, vergilbt und verhärtet"), 0.92, 0.55, 1800, 0.17, 1000, None, None, "authored-estimate", "iso-10456-2007"),
    ("abs-black", T("Black ABS ventilation grille", "Schwarzes ABS-Lüftungsgitter"), 0.92, 0.90, 340, 0.17, 1400, None, None, "authored-estimate", "iso-10456-2007"),
    ("abs-white", T("White ABS trims, hinge covers and handle, yellowed", "Weiße ABS-Blenden, Scharnierkappen und Griff, vergilbt"), 0.90, 0.35, 1050, 0.17, 1400, None, None, "authored-estimate", "iso-10456-2007"),
    ("hinge-steel", T("Zinc-plated steel hinge bracket under an ABS cap", "Verzinkter Stahl-Scharnierwinkel unter ABS-Kappe"), 0.90, 0.40, 2000, 50, 460, 2.1e11, 2.4e8, "authored-estimate", "en-10346-2015"),
    ("glass-shelf", T("Tempered glass shelf 4 mm", "Einscheiben-Sicherheitsglas 4 mm"), 0.92, 0.10, 2500, 1.0, 750, 7.0e10, 1.2e8, "literature", "iso-10456-2007"),
    ("ps-clear", T("Clear polystyrene bins, drawers and trays (effective over the bin volume)", "Transparente Polystyrol-Schalen, -Schubladen und -Tropfschalen (wirksam über das Schalenvolumen)"), 0.90, 0.10, 30, 0.16, 1300, None, None, "authored-estimate", "iso-10456-2007"),
    ("aluminium-plate", T("Aluminium roll-bond evaporator plate 1.5 mm with its channels (effective over 4 mm)", "Aluminium-Rollbond-Verdampferplatte 1,5 mm mit ihren Kanälen (wirksam über 4 mm)"), 0.30, 0.30, 1000, 160, 900, 7.0e10, 1.0e8, "literature", "iso-10456-2007"),
    ("aluminium-bar", T("Anodised aluminium bar handle, scratched (replacement)", "Eloxierter Aluminium-Stangengriff, zerkratzt (Ersatzteil)"), 0.77, 0.40, 2700, 160, 900, 7.0e10, 1.5e8, "literature", "incropera-2018"),
    ("compressor-steel", T("Black painted hermetic compressor shell (effective over the shell volume)", "Schwarz lackiertes hermetisches Verdichtergehäuse (wirksam über das Gehäusevolumen)"), 0.90, 0.90, 3770, 50, 480, None, None, "authored-estimate", "en-60335-2-24"),
    ("galvanised-steel", T("Galvanised steel sheet", "Verzinktes Stahlblech"), 0.28, 0.65, 7850, 50, 460, 2.1e11, 2.4e8, "literature", "en-10346-2015"),
    ("condenser-tube", T("Black painted steel condenser tube 5 x 0.6 mm (effective over the full section)", "Schwarz lackiertes Stahlrohr des Verflüssigers 5 x 0,6 mm (wirksam über den Vollquerschnitt)"), 0.95, 0.95, 3315, 50, 460, None, None, "authored-estimate", "incropera-2018"),
    ("condenser-wire", T("Black painted steel wire 1.5 mm", "Schwarz lackierter Stahldraht 1,5 mm"), 0.95, 0.95, 7850, 50, 460, None, None, "literature", "incropera-2018"),
    ("copper-pipe", T("Copper refrigerant pipe 6 x 0.7 mm (effective over the full section)", "Kupfer-Kältemittelleitung 6 x 0,7 mm (wirksam über den Vollquerschnitt)"), 0.60, 0.65, 3670, 390, 385, None, None, "authored-estimate", "incropera-2018"),
    ("power-cable", T("PVC-sheathed power cable 3 x 0.75 mm2, repaired with tape", "PVC-Anschlussleitung 3 x 0,75 mm2, mit Isolierband geflickt"), 0.92, 0.85, 1800, 0.3, 1000, None, None, "authored-estimate", "iso-10456-2007"),
    ("pp-feet", T("Polypropylene feet and rollers", "Füße und Rollen aus Polypropylen"), 0.90, 0.80, 950, 0.22, 1700, None, None, "authored-estimate", "iso-10456-2007"),
    ("magnet", T("Ferrite fridge magnet with printed plastic face", "Ferrit-Kühlschrankmagnet mit bedruckter Kunststofffront"), 0.90, 0.60, 3500, 4, 700, None, None, "authored-estimate", "incropera-2018"),
    ("corrugated-board", T("Flattened corrugated carton, single wall 3 mm, water-stained", "Flachgelegter Wellpappkarton, einwellig 3 mm, mit Wasserflecken"), 0.90, 0.75, 150, 0.06, 1340, None, None, "authored-estimate", "iso-10456-2007"),
    ("truck-tube", T("Red powder-coated steel tube of the hand truck (effective over the full section)", "Rot pulverbeschichtetes Stahlrohr der Sackkarre (wirksam über den Vollquerschnitt)"), 0.90, 0.70, 2100, 50, 460, 2.1e11, 2.35e8, "authored-estimate", "en-10219-2-2019"),
    ("truck-plate", T("Steel nose plate and axle, scuffed", "Stählerne Schaufel und Achse, abgeschabt"), 0.80, 0.70, 7850, 50, 460, 2.1e11, 2.35e8, "literature", "en-10219-2-2019"),
    ("tyre-rubber", T("Pneumatic rubber tyre 260 mm (effective over the tyre volume)", "Luftbereifung aus Gummi 260 mm (wirksam über das Reifenvolumen)"), 0.92, 0.92, 500, 0.2, 1900, None, None, "authored-estimate", "iso-10456-2007"),
    ("rim-steel", T("Pressed steel rim (effective over the rim disc)", "Gepresste Stahlfelge (wirksam über die Felgenscheibe)"), 0.60, 0.60, 580, 50, 460, None, None, "authored-estimate", "en-10219-2-2019"),
    ("pvc-grip", T("Black PVC grip sleeve", "Schwarze PVC-Griffhülse"), 0.92, 0.90, 1300, 0.17, 1000, None, None, "authored-estimate", "iso-10456-2007"),
    ("glass-bulb", T("Incandescent lamp 15 W, E14", "Glühlampe 15 W, E14"), 0.90, 0.10, 2500, 1.0, 750, None, None, "authored-estimate", "incropera-2018"),
]
RHO = {m[0]: m[4] for m in MATS}


def materials():
    """🧪 The materials with their aged surface data and the effective densities of the modelled solids."""
    out = []
    for mid, title, eps, alpha, rho, lam, cp, E, f, src, ref in MATS:
        m = {"id": mid, "title": title, "emissivity": eps, "solarAbsorptivity": alpha, "densityKgM3": rho, "conductivityWmK": lam, "specificHeatJKgK": cp}
        if E:
            m["elasticModulusPa"] = E
            m["strengthPa"] = f
        m.update({"source": src, "reference": ref})
        out.append(m)
    return out


# ---------------------------------------------------------------- solid builders (with volume and centroid for the masses)

MASS = []


def track(bone, material, vol, c):
    """⚖ Records the volume and world centroid of a 1:50 body solid."""
    MASS.append((bone, material, vol, c))


def box(sid, bone, lo, hi, material, tone, role="body", **extra):
    """📦 An axis-aligned world box as a box solid in the frame of its bone."""
    a = L3(bone, *lo)
    b = L3(bone, *hi)
    centre = [(a[i] + b[i]) / 2 for i in range(3)]
    size = [abs(a[i] - b[i]) for i in range(3)]
    s = {"id": sid, "kind": "box", "bone": bone, "material": material, "role": role, "tone": tone, "size": size, "transform": {"translation": centre}}
    s.update(extra)
    s["_v"] = (abs(hi[0] - lo[0]) * abs(hi[1] - lo[1]) * abs(hi[2] - lo[2]), [(lo[i] + hi[i]) / 2 for i in range(3)])
    return s


def poly_area_c(pts):
    """📐 Shoelace area and centroid of a simple polygon."""
    a = cx = cy = 0.0
    for i in range(len(pts)):
        x0, y0 = pts[i]
        x1, y1 = pts[(i + 1) % len(pts)]
        cr = x0 * y1 - x1 * y0
        a += cr
        cx += (x0 + x1) * cr
        cy += (y0 + y1) * cr
    a /= 2
    return abs(a), (cx / (6 * a), cy / (6 * a))


def ext_xy(sid, bone, pts_xy, z0, z1, material, tone, role="body", holes=None, **extra):
    """🧱 A prism of a world side-elevation profile (x, y) swept laterally from z0 to z1."""
    oz = BONES[bone][0][2]
    prof = [list(L2(bone, x, y)) for x, y in pts_xy]
    s = {"id": sid, "kind": "extrude", "bone": bone, "material": material, "role": role, "tone": tone, "plane": "xy", "profile": prof, "from": z0 - oz, "to": z1 - oz}
    area, (cx, cy) = poly_area_c(pts_xy)
    if holes:
        s["holes"] = [[list(L2(bone, x, y)) for x, y in h] for h in holes]
        for h in holes:
            area -= poly_area_c(h)[0]
    s.update(extra)
    s["_v"] = (area * (z1 - z0), [cx, cy, (z0 + z1) / 2])
    return s


def ext_zy(sid, bone, pts_zy, x0, x1, material, tone, role="body", holes=None, **extra):
    """🧱 A prism of a world front profile (z, y) swept along x from x0 to x1 (vertical bones)."""
    (ox, oy, oz), k = BONES[bone]
    assert k == "V"
    prof = [[z - oz, y - oy] for z, y in pts_zy]
    a, b = sorted((ox - x0, ox - x1))
    s = {"id": sid, "kind": "extrude", "bone": bone, "material": material, "role": role, "tone": tone, "plane": "zx", "profile": prof, "from": a, "to": b}
    area, (cz, cy) = poly_area_c(pts_zy)
    if holes:
        s["holes"] = [[[z - oz, y - oy] for z, y in h] for h in holes]
        for h in holes:
            area -= poly_area_c(h)[0]
    s.update(extra)
    s["_v"] = (area * abs(x1 - x0), [(x0 + x1) / 2, cy, cz])
    return s


def ext_xz(sid, bone, pts_xz, y0, y1, material, tone, role="body", **extra):
    """🧱 A prism of a world plan profile (x, z) swept up from y0 to y1 (vertical bones)."""
    (ox, oy, oz), k = BONES[bone]
    assert k == "V"
    prof = [[ox - x, z - oz] for x, z in pts_xz]
    s = {"id": sid, "kind": "extrude", "bone": bone, "material": material, "role": role, "tone": tone, "plane": "yz", "profile": prof, "from": y0 - oy, "to": y1 - oy}
    area, (cx, cz) = poly_area_c(pts_xz)
    s.update(extra)
    s["_v"] = (area * (y1 - y0), [cx, (y0 + y1) / 2, cz])
    return s


def revolve(sid, bone, centre, axis, profile, material, tone, role="body", closed_loop=False, **extra):
    """⭕ A body of revolution about a world axis through a world point; `profile` is [axial, radial]."""
    o = L3(bone, *centre)
    written = list(profile) + [profile[0]] if closed_loop else list(profile)
    s = {"id": sid, "kind": "revolve", "bone": bone, "material": material, "role": role, "tone": tone, "profile": [list(p) for p in written], "axis": AX(bone, axis), "origin": list(o)}
    s.update(extra)
    pts = list(profile) if closed_loop else list(profile) + [(profile[-1][0], 0.0), (profile[0][0], 0.0)]
    area, (ca, cr) = poly_area_c(pts)
    vol = 2 * math.pi * cr * area
    c = list(centre)
    i = "xyz".index(axis)
    sign = -1 if (axis == "x" and BONES[bone][1] == "V") else 1
    c[i] += sign * ca
    s["_v"] = (vol, c)
    return s


def capsule(sid, bone, p0, p1, r, material, tone, role="body", rep=None, **extra):
    """💊 A capsule between two world points; `rep` = (count, world step)."""
    a = L3(bone, *p0)
    b = L3(bone, *p1)
    s = {"id": sid, "kind": "capsule", "bone": bone, "material": material, "role": role, "tone": tone, "p0": list(a), "p1": list(b), "r0": r, "r1": r}
    n = 1
    c = [(p0[i] + p1[i]) / 2 for i in range(3)]
    if rep:
        n, step = rep
        (ox, oy, oz), k = BONES[bone]
        ls = step if k == "H" else (step[1], -step[0], step[2])
        s["repeat"] = {"count": n, "translation": list(ls)}
        c = [c[i] + step[i] * (n - 1) / 2 for i in range(3)]
    s.update(extra)
    s["_v"] = (n * (math.pi * r * r * math.dist(p0, p1) + 4 / 3 * math.pi * r ** 3), c)
    return s


def strip(solids):
    """🧽 Removes the private volume bookkeeping before writing."""
    return [{k: v for k, v in s.items() if not k.startswith("_")} for s in solids]


# ---------------------------------------------------------------- the fridge-freezer (fridge-local x, world y via Y(), z)

def rounded_door_profile(z0, z1, x0, x1, r):
    """🚪 Plan section (x, z) of a door slab with its two front vertical edges rounded."""
    pts = [(x0, z0)]
    for k in range(5):
        a = math.pi / 2 * k / 4
        pts.append((x1 - r + r * math.sin(a), z0 + r - r * math.cos(a)))
    for k in range(5):
        a = math.pi / 2 * k / 4
        pts.append((x1 - r + r * math.cos(a), z1 - r + r * math.sin(a)))
    pts.append((x0, z1))
    return pts


def lower_block_profile(scale):
    """🧱 Side section (x, y) of the foamed bottom: floor, plinth step, machine recess roof and lower back wall."""
    yb = 0.0 if scale >= 200 else FT
    xp = 0.500 if scale == 200 else PLINTH[0]
    pts = [(NICHE[0], Y(yb)), (xp, Y(yb)), (xp, Y(PLINTH[1])), (XF, Y(PLINTH[1])), (XF, Y(0.152)), (0.230, Y(0.152)), (0.230, Y(0.297)), (0.0, Y(0.297)), (0.0, Y(NICHE[1])), (NICHE[0], Y(NICHE[1]))]
    return pts


def fridge_body(scale):
    """🧊 The body solids of the fridge-freezer at one scale."""
    s50, s100, s200, s500 = (scale == x for x in SCALES)
    out = []
    if s500:
        prof = [(-0.030, Y(0.0)), (XF, Y(0.0)), (XF, Y(YTOP)), (-0.030, Y(YTOP))]
        out.append(ext_xy("cabinet-block", "cabinet", prof, -ZC, ZC, "cabinet-wall", 0.92))
        for door, (y0, y1) in (("fridge-door", FR), ("freezer-door", FZ)):
            out.append(box(f"{door}-slab", door, (XF, Y(y0), -ZD), (XD, Y(y1), ZD), "door-panel", 0.95))
        return out
    yb = 0.0 if s200 else FT
    for side, sz in (("r", -1), ("l", 1)):
        out.append(box(f"side-wall-{side}", "cabinet", (0.0, Y(yb), sz * ZI), (XF, Y(YTOP), sz * ZC), "cabinet-wall", 0.92))
    out.append(box("top-wall", "cabinet", (0.0, Y(YTOP - TOPW), -ZI), (XF, Y(YTOP), ZI), "cabinet-wall", 0.92))
    out.append(box("back-wall", "cabinet", (0.0, Y(0.297), -ZI), (WALL, Y(YTOP - TOPW), ZI), "cabinet-wall", 0.92))
    out.append(ext_xy("lower-block", "cabinet", lower_block_profile(scale), -ZI, ZI, "cabinet-wall", 0.92))
    out.append(box("divider", "cabinet", (WALL, Y(DIV[0]), -ZI), (XF, Y(DIV[1]), ZI), "divider-foam", 0.9))
    out.append(box("base-plate", "cabinet", (0.0, Y(yb), -ZI), (NICHE[0], Y(yb + 0.002), ZI), "galvanised-steel", 0.5, sheet=True))
    xp = 0.500 if s200 else PLINTH[0]
    out.append(box("plinth", "cabinet", (xp, Y(yb), -ZI), (XF, Y(PLINTH[1]), ZI), "abs-black", 0.15))
    if s50 or s100:
        for side, sz in (("r", -1), ("l", 1)):
            if s50:
                out.append(revolve(f"foot-f{side}", "cabinet", (0.490, Y(0.0), sz * 0.255), "y", [(0.0, 0.0), (0.0, 0.018), (0.004, 0.018), (0.004, 0.012), (FT, 0.012), (FT, 0.0)], "pp-feet", 0.25))
            else:
                out.append(box(f"foot-f{side}", "cabinet", (0.472, Y(0.0), sz * 0.255 - 0.018), (0.508, Y(FT), sz * 0.255 + 0.018), "pp-feet", 0.25))
            out.append(revolve(f"roller-r{side}", "cabinet", (0.045, Y(0.015), sz * 0.255), "z", [(-0.0075, 0.0), (-0.0075, 0.015), (0.0075, 0.015), (0.0075, 0.0)], "pp-feet", 0.25))
    if s50:
        out.append(box("hinge-top", "cabinet", (0.480, Y(YTOP), -ZC), (0.596, Y(1.850), -0.240), "abs-white", 0.85))
        out.append(box("hinge-bottom", "cabinet", (XF, Y(0.076), -ZC), (0.596, Y(FZ[0]), -0.255), "hinge-steel", 0.55))
    if s50 or s100:
        xs = 0.410
        for k, yg in enumerate((1.090, 1.310, 1.530), start=1):
            out.append(box(f"shelf-{k}", "cabinet", (0.065 if s50 else 0.060, Y(yg), -ZI), (xs - (0.010 if s50 else 0.0), Y(yg + 0.004), ZI), "glass-shelf", 0.7, role="glazing", sheet=True))
            if s50:
                out.append(box(f"trim-{k}", "cabinet", (xs - 0.010, Y(yg - 0.010), -ZI), (xs, Y(yg + 0.010), ZI), "abs-white", 0.85))
        if s50:
            out.append(box("crisper-glass", "cabinet", (0.060, Y(0.977), -ZI), (xs, Y(0.981), ZI), "glass-shelf", 0.7, role="glazing", sheet=True))
            for side, (za, zb) in (("r", (-0.245, -0.005)), ("l", (0.005, 0.245))):
                out.append(box(f"crisper-{side}", "cabinet", (0.065, Y(DIV[1]), za), (0.405, Y(0.977), zb), "ps-clear", 0.75))
        else:
            out.append(box("crisper", "cabinet", (0.060, Y(DIV[1]), -ZI), (xs, Y(0.981), ZI), "ps-clear", 0.75))
    if not s500:
        out.append(box("lamp-housing", "cabinet", (0.300, Y(FRC[1] - 0.030), -0.200), (0.380, Y(FRC[1]), -0.120), "abs-white", 0.85))
    if s50:
        for k, (y0, y1) in enumerate(((0.296, 0.300), (0.505, 0.509)), start=1):
            out.append(box(f"freezer-plate-{k}", "cabinet", (WALL, Y(y0), -ZI), (XF - 0.025, Y(y1), ZI), "aluminium-plate", 0.6, sheet=True))
        for name, (x0, y0, y1) in (("a", (0.235, 0.152, 0.290)), ("b", (0.060, 0.300, 0.500)), ("c", (0.060, 0.509, 0.730))):
            out.append(box(f"drawer-{name}", "cabinet", (x0, Y(y0), -0.235), (0.505, Y(y1), 0.235), "ps-clear", 0.75))
    cz = -0.080
    if s50:
        out.append(box("compressor-rail", "cabinet", (0.020, Y(FT + 0.002), cz - 0.065), (0.150, Y(0.042), cz + 0.065), "hinge-steel", 0.4))
        out.append(revolve("compressor", "cabinet", (0.085, Y(0.042), cz), "y", [(0.0, 0.0), (0.0, 0.066), (0.012, 0.072), (0.115, 0.072), (0.140, 0.062), (0.152, 0.040), (0.155, 0.0)], "compressor-steel", 0.2))
        out.append(box("relay", "cabinet", (0.060, Y(0.090), cz + 0.072), (0.110, Y(0.130), 0.040), "abs-black", 0.2))
        out.append(box("drip-tray", "cabinet", (0.015, Y(0.197), -0.200), (0.165, Y(0.215), 0.050), "ps-clear", 0.8))
        out.append(capsule("pipe-a", "cabinet", (0.030, Y(0.120), -0.150), (-0.008, Y(0.200), -0.200), 0.004, "copper-pipe", 0.4))
        out.append(capsule("pipe-b", "cabinet", (-0.008, Y(0.200), -0.200), (-0.020, Y(0.330), -0.2425), 0.004, "copper-pipe", 0.4))
    elif s100:
        out.append(revolve("compressor", "cabinet", (0.085, Y(FT + 0.002), cz), "y", [(0.0, 0.0), (0.0, 0.072), (0.135, 0.072), (0.177, 0.0)], "compressor-steel", 0.2))
        out.append(box("drip-tray", "cabinet", (0.015, Y(0.197), -0.200), (0.165, Y(0.215), 0.050), "ps-clear", 0.8))
        out.append(capsule("pipe-a", "cabinet", (0.030, Y(0.120), -0.150), (-0.020, Y(0.330), -0.2425), 0.004, "copper-pipe", 0.4, slender=True))
    else:
        out.append(revolve("compressor", "cabinet", (0.085, Y(0.002), cz), "y", [(0.0, 0.0), (0.0, 0.072), (0.150, 0.072), (0.195, 0.0)], "compressor-steel", 0.2))
    if s50 or s100:
        out.append(revolve("cable-coil", "cabinet", (-0.009, Y(0.240), 0.150), "x", [(-0.003, 0.050), (0.0, 0.047), (0.003, 0.050), (0.0, 0.053)], "power-cable", 0.2, closed_loop=True, slender=True))
        out.append(capsule("cable-lead", "cabinet", (-0.009, Y(0.190), 0.150), (0.060, Y(0.110), -0.010), 0.003, "power-cable", 0.2, slender=True))
        out.append(box("plug", "cabinet", (-0.030, Y(0.130), 0.130), (-0.002, Y(0.190), 0.170), "power-cable", 0.2))
        if s50:
            out.append(box("cable-tape", "cabinet", (-0.012, Y(0.270), 0.140), (0.0, Y(0.300), 0.160), "power-cable", 0.1))
    if not s200:
        out.append(box("wall-spacer-r", "cabinet", (-0.030, Y(1.790), -0.210), (0.0, Y(1.820), -0.180), "abs-white", 0.85))
        out.append(box("wall-spacer-l", "cabinet", (-0.030, Y(1.790), 0.180), (0.0, Y(1.820), 0.210), "abs-white", 0.85))
    else:
        out.append(box("wall-spacer-r", "cabinet", (-0.030, Y(1.790), -0.215), (0.0, Y(1.820), -0.185), "abs-white", 0.85))
        out.append(box("wall-spacer-l", "cabinet", (-0.030, Y(1.790), 0.185), (0.0, Y(1.820), 0.215), "abs-white", 0.85))
    out += condenser_body(scale)
    out += door_body("fridge-door", FR, scale)
    out += door_body("freezer-door", FZ, scale)
    return out


def condenser_body(scale):
    """🌀 The wire-on-tube condenser on the back."""
    b = "condenser"
    out = []
    if scale == 200:
        out.append(box("condenser-grid", b, (-0.024, Y(0.325), -0.245), (-0.016, Y(1.790), 0.245), "condenser-wire", 0.12, sheet=True, envelope=True))
        return out
    out.append(capsule("cond-tubes", b, (-0.020, Y(0.330), -0.240), (-0.020, Y(0.330), 0.240), 0.0025, "condenser-tube", 0.12, rep=(25, (0.0, 0.060, 0.0)), slender=True))
    for side, sz in (("r", -1), ("l", 1)):
        out.append(capsule(f"cond-bend-{side}", b, (-0.020, Y(0.330), sz * 0.2425), (-0.020, Y(1.770), sz * 0.2425), 0.0025, "condenser-tube", 0.12, slender=True))
    if scale == 50:
        for side, xw in (("back", -0.02325), ("front", -0.01675)):
            out.append(capsule(f"cond-wires-{side}", b, (xw, Y(0.325), -0.238), (xw, Y(1.775), -0.238), 0.00075, "condenser-wire", 0.12, rep=(69, (0.0, 0.0, 0.007)), slender=True))
    xa = -0.0175
    for k, (yy, zz) in enumerate(((0.380, -0.235), (0.380, 0.215), (1.700, -0.235), (1.700, 0.215))):
        out.append(box(f"cond-bracket-{k}", b, (xa, Y(yy), zz), (0.0, Y(yy + 0.020), zz + 0.020), "galvanised-steel", 0.4))
    return out


def door_body(door, span, scale):
    """🚪 Door slab, gasket, liner relief, bins and handles of one door."""
    y0, y1 = span
    H = y1 - y0
    fridge = door == "fridge-door"
    out = []
    if scale == 50:
        out.append(ext_xz(f"{door}-slab", door, rounded_door_profile(-ZD, ZD, XG, XD, 0.010), Y(y0), Y(y1), "door-panel", 0.95))
    else:
        out.append(box(f"{door}-slab", door, (XG, Y(y0), -ZD), (XD, Y(y1), ZD), "door-panel", 0.95))
    ring = [(-0.292, Y(y0 + 0.007)), (0.292, Y(y0 + 0.007)), (0.292, Y(y1 - 0.007)), (-0.292, Y(y1 - 0.007))]
    hole = [(-0.272, Y(y0 + 0.027)), (-0.272, Y(y1 - 0.027)), (0.272, Y(y1 - 0.027)), (0.272, Y(y0 + 0.027))]
    out.append(ext_zy(f"{door}-gasket", door, ring, XF, XG, "pvc-gasket", 0.55, holes=[hole]))
    if fridge:
        out.append(box(f"{door}-plug", door, (0.490, Y(y0 + 0.045), -0.240), (XG, Y(y0 + 1.005), 0.240), "door-liner", 0.9))
    elif scale != 200:
        out.append(box(f"{door}-plug", door, (0.510, Y(y0 + 0.070), -0.235), (XG, Y(y0 + 0.640), 0.235), "door-liner", 0.9))
    else:
        out.append(box(f"{door}-plug", door, (0.508, Y(y0 + 0.070), -0.235), (XG, Y(y0 + 0.640), 0.235), "door-liner", 0.9))
    if fridge:
        if scale == 200:
            out.append(box("balconies", door, (0.420, Y(y0 + 0.100), -0.225), (0.490, Y(y0 + 0.960), 0.225), "ps-clear", 0.8))
        else:
            for k, (ua, ub) in enumerate(((0.10, 0.20), (0.40, 0.49), (0.70, 0.79)), start=1):
                out.append(box(f"balcony-{k}", door, (0.420, Y(y0 + ua), -0.225), (0.490, Y(y0 + ub), 0.225), "ps-clear", 0.8))
            if scale == 50:
                out.append(box("butter-box", door, (0.440, Y(y0 + 0.880), -0.180), (0.490, Y(y0 + 0.960), 0.180), "ps-clear", 0.85))
        if scale == 50:
            out.append(box("handle-bar", door, (0.608, Y(y0 + 0.300), 0.245), (0.620, Y(y0 + 0.650), 0.270), "aluminium-bar", 0.55))
            for k, uu in enumerate((0.320, 0.630), start=1):
                out.append(revolve(f"standoff-{k}", door, (XD, Y(y0 + uu), 0.2575), "x", [(0.0, 0.0), (0.0, 0.007), (0.020, 0.007), (0.020, 0.0)], "aluminium-bar", 0.5))
            out.append(box("magnet", door, (XD, Y(y0 + 0.700), 0.030), (0.596, Y(y0 + 0.740), 0.070), "magnet", 0.35))
        elif scale == 100:
            out.append(box("handle-bar", door, (XD, Y(y0 + 0.300), 0.245), (0.620, Y(y0 + 0.650), 0.270), "aluminium-bar", 0.55))
    else:
        if scale in (50, 100):
            out.append(box("freezer-handle", door, (XD, Y(y0 + 0.600), 0.100), (0.606, Y(y0 + 0.665), 0.280), "abs-white", 0.85))
    return out


# ---------------------------------------------------------------- the carton and the hand truck

def cardboard_body(scale):
    """📦 The flattened carton under the fridge (true 3 mm, a 22 mm strip at 1:500 so it prints at 1:1000)."""
    t = 0.022 if scale == 500 else CB
    extra = {} if scale == 500 else {"sheet": True}
    return [box("cardboard", "cardboard", (-0.030, 0.0, -0.330), (0.660, t, 0.330), "corrugated-board", 0.7, **extra)]


def truck_point(tx, ty, tz=0.0):
    """🛒 Truck-local (ahead of the axle, above the floor) to world rest."""
    return (XA0 + tx, ty, tz)


def truck_body(scale):
    """🛒 The tall appliance hand truck parked behind the fridge."""
    s50, s100, s200, s500 = (scale == x for x in SCALES)
    out = []
    for side, sz in (("r", -1), ("l", 1)):
        c = truck_point(0.0, RW, sz * ZW)
        if s500:
            out.append(revolve(f"wheel-{side}", "truck-base", c, "z", [(-WW / 2, 0.0), (-WW / 2, RW), (WW / 2, RW), (WW / 2, 0.0)], "tyre-rubber", 0.15))
        elif s200:
            out.append(revolve(f"wheel-{side}", "truck-base", c, "z", [(-WW / 2, 0.0), (-WW / 2, 0.112), (-0.025, RW), (0.025, RW), (WW / 2, 0.112), (WW / 2, 0.0)], "tyre-rubber", 0.15))
        else:
            rr = 0.02
            loop = [(-WW / 2, 0.078), (-WW / 2, RW - rr)]
            for k in range(1, 4):
                a = math.pi / 2 * k / 4
                loop.append((-WW / 2 + rr - rr * math.cos(a), RW - rr + rr * math.sin(a)))
            loop += [(-WW / 2 + rr, RW), (WW / 2 - rr, RW)]
            for k in range(1, 4):
                a = math.pi / 2 * k / 4
                loop.append((WW / 2 - rr + rr * math.sin(a), RW - rr + rr * math.cos(a)))
            loop += [(WW / 2, RW - rr), (WW / 2, 0.078)]
            out.append(revolve(f"tyre-{side}", "truck-base", c, "z", loop, "tyre-rubber", 0.15, closed_loop=True))
            out.append(revolve(f"rim-{side}", "truck-base", c, "z", [(-0.030, 0.0), (-0.030, 0.078), (0.030, 0.078), (0.030, 0.0)], "rim-steel", 0.6))
    out.append(capsule("axle", "truck-base", truck_point(0.0, RW, -0.290), truck_point(0.0, RW, 0.290), 0.010, "truck-plate", 0.4, slender=True))
    f = "truck-frame"
    for side, sz in (("r", -1), ("l", 1)):
        out.append(capsule(f"tube-{side}", f, truck_point(TUBE_X, 0.008 + TUBE_R, sz * TUBE_Z), truck_point(TUBE_X, TOP_Y, sz * TUBE_Z), TUBE_R, "truck-tube", 0.35, slender=True))
    if not s500:
        out.append(capsule("top-bar", f, truck_point(TUBE_X, TOP_Y, -TUBE_Z), truck_point(TUBE_X, TOP_Y, TUBE_Z), 0.015 if s200 else TUBE_R, "truck-tube", 0.35))
        for k, yy in enumerate((0.33, 0.68, 1.03), start=1):
            out.append(capsule(f"cross-bar-{k}", f, truck_point(TUBE_X, yy, -0.2035), truck_point(TUBE_X, yy, 0.2035), 0.010, "truck-tube", 0.35, slender=True))
    if s50 or s100:
        out.append(capsule("grip", f, truck_point(TUBE_X, TOP_Y, -0.120), truck_point(TUBE_X, TOP_Y, 0.120), 0.017, "pvc-grip", 0.1))
    if s500:
        out.append(box("bracket", f, truck_point(-0.0375, RW - 0.0375, -0.1975), truck_point(TUBE_X, RW + 0.0375, 0.1975), "truck-plate", 0.4))
    for side, sz in (("r", -1), ("l", 1)):
        if not s500:
            rs = 0.015 if s200 else 0.010
            out.append(capsule(f"strut-{side}", f, truck_point(TUBE_X, 0.230, sz * TUBE_Z), truck_point(0.0, RW, sz * TUBE_Z), rs, "truck-tube", 0.35))
            if not s200:
                out.append(capsule(f"eye-{side}", f, truck_point(0.0, RW, sz * 0.180), truck_point(0.0, RW, sz * 0.190), 0.016, "truck-plate", 0.4))
    t = 0.022 if s500 else 0.008
    extra = {} if s500 else {"sheet": True}
    out.append(box("nose-plate", f, truck_point(0.076, 0.0, -0.200), truck_point(0.300, t, 0.200), "truck-plate", 0.4, **extra))
    return out


def body_of(scale):
    """🧊 The whole body list of one scale."""
    return cardboard_body(scale) + truck_body(scale) + fridge_body(scale)


# ---------------------------------------------------------------- masses

def masses():
    """⚖ Bone masses and centres from the 1:50 body solids (volume x effective density), local (u, v)."""
    acc = {}
    for s in body_of(50):
        vol, c = s["_v"]
        m = vol * RHO[s["material"]]
        b = s["bone"]
        a = acc.setdefault(b, [0.0, 0.0, 0.0, 0.0])
        a[0] += m
        for i in range(3):
            a[1 + i] += m * c[i]
    out = {}
    for b, (m, sx, sy, sz) in acc.items():
        c = (sx / m, sy / m, sz / m)
        u, v, _ = L3(b, *c)
        out[b] = (m, (u, v), c)
    return out


# ---------------------------------------------------------------- anchors

CARD_C = ["carton-corner-fr", "carton-corner-fl", "carton-corner-rr", "carton-corner-rl"]
WHEEL_C = ["truck-wheel-r", "truck-wheel-l"]
NOSE_C = ["truck-nose"]
GRIP_C = ["grip-r", "grip-l"]
FEET_C = ["fridge-foot-fr", "fridge-foot-fl", "fridge-roller-rr", "fridge-roller-rl"]


def anchors():
    """🪝 Contacts of the carton, the fridge feet and rollers, the hand truck's wheels, nose plate and grips, the condenser plume outlet and the door-shelf load point."""
    out = []
    side = {"r": T("right", "rechts"), "l": T("left", "links")}
    for aid in CARD_C:
        f = aid[-2] == "f"
        sz = 1 if aid[-1] == "l" else -1
        x = 0.640 if f else -0.010
        u, v, w = L3("cardboard", x, 0.0, sz * 0.310)
        n = (T("front", "vorne") if f else T("rear", "hinten"))
        out.append({"id": aid, "bone": "cardboard", "x": u, "y": v, "z": w, "kind": "contact", "title": T(f"Carton corner on the depot floor, {n['en']} {side[aid[-1]]['en']}", f"Kartonecke auf dem Lagerboden, {n['de']} {side[aid[-1]]['de']}"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.04, "footprintWidthM": 0.04, "frictionCoefficient": 0.5})
    for aid in WHEEL_C:
        sz = 1 if aid[-1] == "l" else -1
        u, v, w = L3("truck-base", XA0, 0.0, sz * ZW)
        out.append({"id": aid, "bone": "truck-base", "x": u, "y": v, "z": w, "kind": "contact", "title": T(f"Hand-truck wheel on the floor, {side[aid[-1]]['en']}", f"Rad der Sackkarre auf dem Boden, {side[aid[-1]]['de']}"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.06, "footprintWidthM": 0.07, "frictionCoefficient": 0.6})
    u, v, w = L3("truck-frame", *truck_point(0.290, 0.0, 0.0))
    out.append({"id": "truck-nose", "bone": "truck-frame", "x": u, "y": v, "z": w, "kind": "contact", "title": T("Front edge of the nose plate on the floor", "Vorderkante der Schaufel auf dem Boden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.02, "footprintWidthM": 0.40, "frictionCoefficient": 0.4})
    for aid in GRIP_C:
        sz = 1 if aid[-1] == "l" else -1
        u, v, w = L3("truck-frame", *truck_point(TUBE_X, TOP_Y, sz * 0.100))
        out.append({"id": aid, "bone": "truck-frame", "x": u, "y": v, "z": w, "kind": "contact", "title": T(f"Mover's hand on the grip, {side[aid[-1]]['en']}", f"Hand des Trägers am Griff, {side[aid[-1]]['de']}"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.09, "footprintWidthM": 0.035, "frictionCoefficient": 0.8})
    for aid in FEET_C:
        sz = 1 if aid[-1] == "l" else -1
        foot = "foot" in aid
        u, v, w = L3("cabinet", 0.490 if foot else 0.045, Y(0.0), sz * 0.255)
        t = T(f"Adjustable front foot, {side[aid[-1]]['en']}", f"Verstellbarer Vorderfuß, {side[aid[-1]]['de']}") if foot else T(f"Rear transport roller, {side[aid[-1]]['en']}", f"Hintere Transportrolle, {side[aid[-1]]['de']}")
        out.append({"id": aid, "bone": "cabinet", "x": u, "y": v, "z": w, "kind": "contact", "title": t, "support": "contact", "normalDeg": 90, "footprintLengthM": 0.036 if foot else 0.010, "footprintWidthM": 0.036 if foot else 0.015, "frictionCoefficient": 0.5})
    u, v, w = L3("condenser", -0.020, Y(1.790), 0.0)
    out.append({"id": "condenser-top", "bone": "condenser", "x": u, "y": v, "z": w, "kind": "emitter", "title": T("Top of the wire condenser", "Oberkante des Drahtverflüssigers")})
    u, v, w = L3("fridge-door", 0.455, Y(FR[0] + 0.445), 0.0)
    out.append({"id": "door-shelf", "bone": "fridge-door", "x": u, "y": v, "z": w, "kind": "load", "title": T("Middle door balcony with bottles", "Mittleres Türabstellfach mit Flaschen")})
    u, v, w = L3("cabinet", 0.24, Y(1.30), 0.0)
    out.append({"id": "contents", "bone": "cabinet", "x": u, "y": v, "z": w, "kind": "load", "title": T("Centre of the food in both compartments", "Schwerpunkt der Lebensmittel in beiden Fächern")})
    return out


ANCHORS = anchors()


def anchor(aid):
    """🪝 One anchor by id."""
    return next(a for a in ANCHORS if a["id"] == aid)


# ---------------------------------------------------------------- volumes per scale

REGION_OF = {
    "cardboard": "cardboard", "axle": "truck", "nose-plate": "truck", "top-bar": "truck", "grip": "truck",
    "cabinet-block": "cabinet", "side-wall-r": "cabinet", "side-wall-l": "cabinet", "top-wall": "cabinet", "back-wall": "cabinet", "lower-block": "cabinet", "divider": "cabinet", "base-plate": "cabinet", "hinge-top": "cabinet", "hinge-bottom": "cabinet", "wall-spacer-r": "cabinet", "wall-spacer-l": "cabinet",
    "handle-bar": "doors", "standoff-1": "doors", "standoff-2": "doors", "magnet": "doors", "freezer-handle": "doors",
}
STRUCT_SKIP = ("shelf-", "trim-", "crisper", "lamp-housing", "freezer-plate", "drawer-", "relay", "drip-tray", "pipe-", "cable-", "plug", "balcon", "butter-box", "magnet", "grip", "hinge-top", "standoff", "handle-bar", "freezer-handle", "fridge-door-gasket", "freezer-door-gasket", "fridge-door-plug", "freezer-door-plug", "compressor", "base-plate")


def region_of(sid):
    """🔥 The thermal region a body solid belongs to (None: inside a closed compartment or not a region surface)."""
    if sid in REGION_OF:
        return REGION_OF[sid]
    for p, r in (("foot-", "cabinet"), ("roller-", "cabinet"), ("tyre-", "truck-wheels"), ("wheel-", "truck-wheels"), ("rim-", "truck"), ("tube-", "truck"), ("cross-bar-", "truck"), ("strut-", "truck"), ("eye-", "truck"), ("bracket", "truck"), ("cond-", "condenser"), ("condenser-grid", "condenser")):
        if sid.startswith(p):
            return r
    if sid.endswith("-slab"):
        return "doors"
    if sid.endswith("-gasket"):
        return "gasket"
    return None


def mass_marker(sid, bone, at, r, material):
    """⚖ A centre-of-mass marker (world point)."""
    u, v, w = L3(bone, *at)
    return {"id": sid, "kind": "ellipsoid", "bone": bone, "material": material, "role": "detail", "tone": 0.3, "radii": [r, r, r], "transform": {"translation": [u, v, w]}, "member": bone, "part": "mass"}


def pad(sid, bone, aid, r, h):
    """🛞 A support pad on a support anchor, standing on the floor."""
    a = anchor(aid)
    return {"id": sid, "kind": "revolve", "bone": bone, "material": "tyre-rubber", "role": "ground", "tone": 0.2, "profile": [[0.0, 0.0], [0.0, r], [h, r], [h, 0.0]], "axis": AX(bone, "y"), "origin": [a["x"], a["y"], a["z"]], "member": bone, "part": "support", "anchor": aid}


def hidden_solids(scale):
    """🫥 The own solids of the hidden regions: compressor shell, the two evaporators and the lamp (never body surfaces)."""
    f = FEATURE[scale]
    out = []
    cz = -0.080
    if scale == 500:
        out.append(box("th-compressor", "cabinet", (0.015, Y(0.040), cz - 0.0375), (0.155, Y(0.195), cz + 0.0375), "compressor-steel", 0.2, region="compressor"))
    else:
        out.append(revolve("th-compressor", "cabinet", (0.085, Y(0.042 if scale < 200 else 0.004), cz), "y", [(0.0, 0.0), (0.0, 0.072), (0.150, 0.072), (0.155, 0.0)], "compressor-steel", 0.2, region="compressor"))
    out.append(box("th-fridge-evaporator", "cabinet", (WALL - 0.004 if scale < 500 else 0.020, Y(0.950), -0.220), (WALL if scale < 500 else 0.024, Y(1.700), 0.220), "aluminium-plate", 0.6, region="fridge-evaporator", sheet=True))
    for k, (y0, y1) in enumerate(((0.296, 0.300), (0.505, 0.509)), start=1):
        out.append(box(f"th-freezer-evaporator-{k}", "cabinet", (WALL, Y(y0), -ZI + 0.005), (XF - 0.030, Y(y1), ZI - 0.005), "aluminium-plate", 0.6, region="freezer-evaporator", sheet=True))
    lc = 0.040 if scale <= 100 else f
    out.append(box("th-lamp", "cabinet", (0.340 - lc / 2, Y(FRC[1] - 0.030 - lc), -0.160 - lc / 2), (0.340 + lc / 2, Y(FRC[1] - 0.030), -0.160 + lc / 2), "glass-bulb", 0.9, region="lamp"))
    return out


def emitter_solid(scale):
    """💨 The outlet patch of the condenser plume at the top edge of the condenser."""
    f = FEATURE[scale]
    a = anchor("condenser-top")
    return {"id": "th-condenser-plume", "kind": "revolve", "bone": "condenser", "material": "condenser-tube", "role": "detail", "tone": 0.2, "profile": [[0.0, 0.0], [0.0, max(0.02, f)], [f, max(0.02, f)], [f, 0.0]], "axis": [1, 0, 0], "origin": [a["x"] - f, a["y"], a["z"]], "region": "condenser", "tags": ["condenser-plume"]}


def lists(scale, body):
    """🧊 Thermal and structural lists of one scale from its body list."""
    f = FEATURE[scale]
    thermal = []
    for s in body:
        reg = region_of(s["id"])
        if reg:
            thermal.append({"id": f"th-{s['id']}", "of": s["id"], "region": reg})
    if scale == 500:
        thermal.append(strip([box("th-gasket", "fridge-door", (XF, Y(FR[0] + 0.005), -ZD), (XD, Y(FR[0] + 1.055), -ZD + 0.058), "pvc-gasket", 0.55, region="gasket")])[0])
        thermal.append(strip([box("th-plinth", "cabinet", (XF - 0.075, Y(0.0), -ZI), (XF, Y(0.075), ZI), "abs-black", 0.15, region="plinth")])[0])
        thermal.append(strip([box("th-condenser", "condenser", (-0.024, Y(0.325), -0.245), (-0.016, Y(1.775), 0.245), "condenser-wire", 0.12, region="condenser", sheet=True, envelope=True)])[0])
    if scale < 500:
        xp = 0.500 if scale == 200 else PLINTH[0]
        thermal.append(strip([box("th-plinth", "cabinet", (xp, Y(0.0 if scale == 200 else FT), -ZI), (XF, Y(PLINTH[1]), ZI), "abs-black", 0.15, region="plinth")])[0])
    thermal += strip(hidden_solids(scale))
    thermal.append(emitter_solid(scale))
    structure = []
    for s in body:
        sid = s["id"]
        if any(sid.startswith(p) for p in STRUCT_SKIP) or sid.startswith("cond-wires"):
            continue
        structure.append({"id": f"m-{sid}", "of": sid, "member": s["bone"], "part": "member"})
    if scale == 500:
        structure.append(strip([capsule("m-condenser", "condenser", (-0.020, Y(0.330), 0.0), (-0.020, Y(1.770), 0.0), 0.0025, "condenser-tube", 0.12, member="condenser", part="member", slender=True)])[0])
    j = max(f, 0.012)
    for door, y0 in (("fridge-door", FR[0]), ("freezer-door", FZ[0])):
        for k, yy in enumerate((y0 + 0.004, (FR[1] if door == "fridge-door" else FZ[1]) - 0.004 - j)):
            structure.append(strip([box(f"j-{door}-hinge-{k}", door, (XF, Y(yy), -ZC), (XD, Y(yy + j), -ZC + j), "hinge-steel", 0.3, role="detail", member=door, part="joint")])[0])
    jr = max(f / 2, 0.012)
    structure.append(strip([capsule("j-axle", "truck-frame", truck_point(0.0, RW, -0.19), truck_point(0.0, RW, 0.19), jr, "truck-plate", 0.3, role="detail", member="truck-frame", part="joint")])[0])
    pr = f if scale < 500 else 0.0375
    for aid in WHEEL_C:
        structure.append(pad(f"support-{aid}", "truck-base", aid, pr, f))
    M = masses()
    mr = f if scale < 500 else 0.0375
    for bone in ("cardboard", "truck-base", "truck-frame", "cabinet", "fridge-door", "freezer-door"):
        kg, (u, v), c = M[bone]
        at = c
        if bone == "cardboard":
            at = (c[0], max(c[1], mr) if scale == 500 else c[1], c[2])
        if bone == "truck-frame":
            at = (XA0 + TUBE_X, c[1], 0.0)
        if bone in ("fridge-door", "freezer-door"):
            at = ((XG + XD) / 2 if scale < 500 else (XF + XD) / 2, c[1], c[2])
        structure.append(mass_marker(f"mass-{bone}", bone, at, mr if bone != "cardboard" or scale == 500 else 0.0015, "cabinet-wall"))
    return thermal, structure


VOLUME_DECISIONS = {
    50: [T("Body, authored for 7.5 mm: the foamed cabinet as its walls (50 mm sides and back, 45 mm top, the bottom with the plinth step and the roof of the machine recess as one section, the 70 mm divider), the 2 mm base plate of the recess, the plinth grille, two turned front feet and two rear rollers, the hinge cover on the top and the bottom hinge bracket; inside: three 4 mm glass shelves with their trims, the crisper cover and two crisper drawers, the lamp housing, the two evaporator shelves and three drawers of the freezer; in the recess the compressor dome on its rail with relay, drip tray and the copper discharge line; behind: the wire-on-tube condenser (25 tube runs, 2 x 69 wires of 1.5 mm, true sections, slender) on four brackets, the two wall spacers, the coiled power cable taped to the back with its plug. Doors: slabs with rounded front edges, the yellowed magnetic gaskets as rings, the liner reliefs, three balconies and the butter box, the replaced aluminium bar handle on two standoffs, a forgotten fridge magnet, the original freezer handle. Carton 3 mm (sheet); hand truck with tyres, rims, axle, frame tubes, top bar, three cross bars, grip, struts, axle eyes and the 8 mm nose plate (sheet).", "Körper für 7,5 mm: das geschäumte Gehäuse als seine Wände (50-mm-Seiten und Rückwand, 45-mm-Decke, der Boden mit Sockelstufe und Dach der Maschinennische als ein Querschnitt, die 70-mm-Trennwand), die 2-mm-Bodenplatte der Nische, das Sockelgitter, zwei gedrehte Vorderfüße und zwei hintere Rollen, die Scharnierkappe oben und der untere Scharnierwinkel; innen: drei 4-mm-Glasböden mit Blenden, die Gemüsefachabdeckung und zwei Gemüseschalen, das Lampengehäuse, die beiden Verdampferböden und drei Schubladen des Gefrierteils; in der Nische die Verdichterkuppel auf ihrer Schiene mit Relais, Tropfschale und kupferner Druckleitung; hinten: der Drahtrohrverflüssiger (25 Rohrzüge, 2 x 69 Drähte zu 1,5 mm, echte Querschnitte, schlank) auf vier Haltern, die beiden Wandabstandshalter, das aufgewickelte, an die Rückwand geklebte Netzkabel mit Stecker. Türen: Türblätter mit gerundeten Vorderkanten, die vergilbten Magnetdichtungen als Ringe, die Innenverkleidungen, drei Abstellfächer und das Butterfach, der ersetzte Aluminium-Stangengriff auf zwei Abstandshaltern, ein vergessener Kühlschrankmagnet, der originale Gefriergriff. Karton 3 mm (Blatt); Sackkarre mit Reifen, Felgen, Achse, Rahmenrohren, Griffholm, drei Querstreben, Griff, Streben, Achsaugen und der 8-mm-Schaufel (Blatt)."),
         T("Thermal: every outer body part by reference in its region (cabinet, plinth, doors with handles, gaskets, condenser, carton, hand truck, tyres); the hidden regions have their own solids inside: compressor shell, the cold-wall evaporator plate behind the fridge liner, the two freezer evaporator plates and the lamp; the condenser plume leaves from a patch on the top edge of the condenser. Structure: the cabinet walls, feet and rollers, door slabs, carton, frame tubes, bars, struts, eyes, nose plate, tyres, rims and axle as members; door hinges and the axle bearing as joints; one pad under each hand-truck wheel (the permanent supports); masses at the centres of the six massive bones.", "Thermik: jedes äußere Körperteil als Verweis in seiner Region (Gehäuse, Sockel, Türen mit Griffen, Dichtungen, Verflüssiger, Karton, Sackkarre, Reifen); die verdeckten Regionen haben eigene Körper im Inneren: Verdichtergehäuse, die Kaltwand-Verdampferplatte hinter dem Kühlteil-Liner, die beiden Gefrierverdampferplatten und die Lampe; die Verflüssigerfahne steigt von einer Fläche an der Oberkante des Verflüssigers auf. Tragwerk: Gehäusewände, Füße und Rollen, Türblätter, Karton, Rahmenrohre, Streben, Achsaugen, Schaufel, Reifen, Felgen und Achse als Stäbe; Türscharniere und Achslager als Knoten; je ein Auflager unter jedem Sackkarrenrad (die ständigen Auflager); Massen in den Schwerpunkten der sechs schweren Knochen.")],
    100: [T("Body, authored anew for 15 mm: walls, bottom, divider and plinth as at 1:50; the feet as 36 mm blocks, the rollers kept; hinge cover and bottom hinge bracket (12 and 14 mm) left out; shelves as glass sheets without trims, crisper drawers and cover merged into one bin, the freezer drawers and evaporator shelves left out of the body (never seen behind the closed freezer door); the compressor as one dome without rail and relay, one straight discharge line; cable coil and plug kept, the tape left out; the condenser as its 25 tube runs and bends without the 1.5 mm wires; door slabs square-edged, gaskets kept at their drawn 8 mm (C13), three balconies, the replaced handle as one bar block with its standoffs, magnet left out; hand truck as at 1:50.", "Körper für 15 mm neu gebaut: Wände, Boden, Trennwand und Sockel wie bei 1:50; die Füße als 36-mm-Klötze, die Rollen bleiben; Scharnierkappe und unterer Scharnierwinkel (12 und 14 mm) entfallen; Böden als Glasblätter ohne Blenden, Gemüseschalen und Abdeckung zu einer Schale verschmolzen, Gefrierschubladen und Verdampferböden entfallen im Körper (hinter der geschlossenen Gefriertür nie sichtbar); der Verdichter als eine Kuppel ohne Schiene und Relais, eine gerade Druckleitung; Kabelrolle und Stecker bleiben, das Klebeband entfällt; der Verflüssiger als seine 25 Rohrzüge und Bögen ohne die 1,5-mm-Drähte; Türblätter scharfkantig, Dichtungen in ihren gezeichneten 8 mm (C13), drei Abstellfächer, der Ersatzgriff als ein Stangenblock mit seinen Abstandshaltern, Magnet entfällt; Sackkarre wie bei 1:50."),
          T("Thermal: outer parts by reference, hidden regions as own solids, plume patch at 15 mm. Structure: members by reference, hinge and axle joints and wheel pads at 15 mm, the six masses.", "Thermik: äußere Teile als Verweis, verdeckte Regionen als eigene Körper, Fahnenfläche bei 15 mm. Tragwerk: Stäbe als Verweis, Scharnier- und Achsknoten und Radauflager bei 15 mm, die sechs Massen.")],
    200: [T("Body, authored anew for 30 mm: the feet and rollers (18 and 30 mm) merge into the cabinet, which now reaches down to the carton; the plinth grille deepened to 30 mm; the interior empty apart from the lamp housing (shelves, bins and drawers are below what the scale shows behind a door); the door bins as one block; handles left out; the condenser as one 8 mm sheet of its grid (envelope, typified) reaching the wall spacers; the compressor a plain dome; hand truck with merged wheels (one turned tyre with its rim), 30 mm struts, no eyes or grip; carton and nose plate keep their true 3 and 8 mm as declared sheets.", "Körper für 30 mm neu gebaut: Füße und Rollen (18 und 30 mm) gehen im Gehäuse auf, das nun bis auf den Karton reicht; das Sockelgitter auf 30 mm vertieft; der Innenraum bis auf das Lampengehäuse leer (Böden, Schalen und Schubladen sind hinter einer Tür unter dem, was der Maßstab zeigt); die Türfächer als ein Block; Griffe entfallen; der Verflüssiger als ein 8-mm-Blatt seines Gitters (Hülle, typisiert) bis an die Wandabstandshalter; der Verdichter eine schlichte Kuppel; Sackkarre mit verschmolzenen Rädern (ein gedrehter Reifen mit Felge), 30-mm-Streben, ohne Achsaugen und Griff; Karton und Schaufel behalten ihre echten 3 und 8 mm als deklarierte Blätter."),
          T("Thermal: outer parts by reference, the condenser sheet as an envelope, hidden regions as own solids. Structure: members by reference, joints and pads at 30 mm, masses.", "Thermik: äußere Teile als Verweis, das Verflüssigerblatt als Hülle, verdeckte Regionen als eigene Körper. Tragwerk: Stäbe als Verweis, Knoten und Auflager bei 30 mm, Massen.")],
    500: [T("Body, authored anew for 75 mm: the sign of the stored appliance. The fridge-freezer as one cabinet block 560 x 600 x 1838 mm with the condenser and wall spacers merged into its back, the two doors as 58 mm slabs (door and gasket merged, as drawn: C13) on their hinge bones; the carton as a 22 mm strip and the nose plate as a 22 mm plate so they print at 1:1000; the hand truck as two wheel discs, the axle, two 75 mm axle brackets, the frame tubes and the top bar at their true 28 mm (slender).", "Körper für 75 mm neu gebaut: das Zeichen des gelagerten Geräts. Die Kühl-Gefrier-Kombination als ein Gehäuseblock 560 x 600 x 1838 mm, in dessen Rückseite Verflüssiger und Wandabstandshalter aufgehen, die beiden Türen als 58-mm-Blätter (Tür und Dichtung verschmolzen, wie gezeichnet: C13) an ihren Scharnierknochen; der Karton als 22-mm-Streifen und die Schaufel als 22-mm-Platte, damit sie bei 1:1000 drucken; die Sackkarre als zwei Radscheiben, die Achse, zwei 75-mm-Achshalter, die Rahmenrohre und der Griffholm in ihren echten 28 mm (schlank)."),
          T("Thermal: block, slabs, carton and truck by reference; the gaskets, merged into the doors, keep their true surface as one 58 mm strip along the hinge edge of the fridge door, the plinth and the condenser keep own solids at the front foot of the block and behind its back (condenser as an envelope sheet), the hidden regions own solids inside the block. Structure: members by reference, joints, pads and masses at 75 mm.", "Thermik: Block, Türblätter, Karton und Karre als Verweis; die in die Türen verschmolzenen Dichtungen behalten ihre echte Oberfläche als ein 58-mm-Streifen an der Scharnierkante der Kühltür, Sockel und Verflüssiger behalten eigene Körper am vorderen Fuß des Blocks und hinter seiner Rückseite (Verflüssiger als Hüllblatt), die verdeckten Regionen eigene Körper im Block. Tragwerk: Stäbe als Verweis, Knoten, Auflager und Massen bei 75 mm.")],
}


def build_volumes():
    """🧊 The four individually authored volumes."""
    vols = {}
    for scale in SCALES:
        body = body_of(scale)
        thermal, structure = lists(scale, body)
        vols[str(scale)] = {"status": "authored", "denominator": scale, "body": strip(body), "thermal": thermal, "structure": structure, "decisions": VOLUME_DECISIONS[scale]}
    return vols


# ---------------------------------------------------------------- 2D shapes

def shape(sid, bone, role, z, tone, geom, region=None, material=None, hatch=None, filled=True, regions=None, solid=None, pen=None):
    """✏️ One 2D shape with its line-hierarchy role, paint order and region."""
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    s.update(geom)
    if region:
        s["region"] = region
    if regions:
        s["regions"] = regions
    if material:
        s["material"] = material
    if hatch:
        s["hatch"] = hatch
    if pen:
        s["penMm"] = pen
    if solid:
        s["solid"] = solid
    return s


def rect(x0, y0, x1, y1):
    """▭ A rect geometry from two corners."""
    return {"kind": "rect", "x": min(x0, x1), "y": min(y0, y1), "w": abs(x1 - x0), "h": abs(y1 - y0)}


def rect_e(bone, x0, y0, x1, y1):
    """▭ A rect in the elevation frame of a bone from two world corners."""
    a = L2(bone, x0, y0)
    b = L2(bone, x1, y1)
    return rect(a[0], a[1], b[0], b[1])


def poly(points, closed=True):
    """⬠ A poly geometry."""
    return {"kind": "poly", "points": [list(p) for p in points], "closed": closed}


def poly_e(bone, points, closed=True):
    """⬠ A poly in the elevation frame of a bone from world points."""
    return poly([L2(bone, *p) for p in points], closed)


def circle(cx, cy, r):
    """⭕ A circle as an ellipse geometry."""
    return {"kind": "ellipse", "cx": cx, "cy": cy, "rx": r, "ry": r}


def caps(x0, y0, x1, y1, r):
    """💊 A capsule geometry."""
    return {"kind": "capsule", "x0": x0, "y0": y0, "x1": x1, "y1": y1, "r0": r, "r1": r}


def caps_e(bone, p0, p1, r):
    """💊 A capsule in the elevation frame of a bone from world (x, y) points."""
    a = L2(bone, *p0)
    b = L2(bone, *p1)
    return caps(a[0], a[1], b[0], b[1], r)


def lines(segments):
    """〰 A path of straight strokes."""
    return {"kind": "path", "d": " ".join(f"M {a[0]:.5f} {a[1]:.5f} L {b[0]:.5f} {b[1]:.5f}" for a, b in segments)}


def lines_e(bone, segments):
    """〰 Straight strokes in the elevation frame of a bone from world points."""
    return lines([(L2(bone, *a), L2(bone, *b)) for a, b in segments])


def circles(cs):
    """⭕ A path of circles."""
    return {"kind": "path", "d": " ".join(f"M {cx + r:.5f} {cy:.5f} A {r:.5f} {r:.5f} 0 1 0 {cx - r:.5f} {cy:.5f} A {r:.5f} {r:.5f} 0 1 0 {cx + r:.5f} {cy:.5f}" for cx, cy, r in cs)}


def door_shape(sid, door, role, z, tone, x0, y0, x1, y1, z0, z1, region=None, material=None, regions=None, filled=True):
    """🚪 A door part as a solid placement: its side profile (world x, y) on the door bone swept laterally from z0 to z1."""
    return shape(sid, door, role, z, tone, rect_e(door, x0, y0, x1, y1), region, material, regions=regions, filled=filled, solid={"kind": "extruded", "sweep": "w", "z0": z0, "z1": z1})


def scratch(sid, door, a, b, width=0.0012):
    """🪡 A scratch in the door face as a thin solid placed in the face plane (profile u along the door, w across it)."""
    oy, oz = BONES[door][0][1], BONES[door][0][2]
    (za, ya), (zb, yb) = a, b
    dz, dy = zb - za, yb - ya
    n = math.hypot(dz, dy)
    nz, ny = -dy / n * width / 2, dz / n * width / 2
    pts = [(ya + ny - oy + CB, za + nz - oz), (yb + ny - oy + CB, zb + nz - oz), (yb - ny - oy + CB, zb - nz - oz), (ya - ny - oy + CB, za - nz - oz)]
    return shape(sid, door, "detail", 5.6, 0.45, poly(pts), "doors", solid={"kind": "extruded", "sweep": "v", "z0": -0.0004, "z1": 0.0})


def door_parts(scale):
    """🚪 The drawn parts of both doors at one scale, as (id, door, role, tone, x0, y0, x1, y1, z0, z1, region, material, regions, views)."""
    s50, s100, s200, s500 = (scale == x for x in SCALES)
    P = []
    for door, (y0, y1) in (("fridge-door", FR), ("freezer-door", FZ)):
        if s500:
            P.append((f"{door}-slab", door, "body", 0.95, XF, Y(y0), XD, Y(y1), -ZD, ZD, None, "door-panel", ["doors", "gasket"], "efp"))
            continue
        P.append((f"{door}-slab", door, "body", 0.95, XG, Y(y0), XD, Y(y1), -ZD, ZD, "doors", "door-panel", None, "efp"))
        P.append((f"{door}-gasket", door, "body", 0.5, XF, Y(y0 + 0.007), XG, Y(y1 - 0.007), -0.292, 0.292, "gasket", "pvc-gasket", None, "efp"))
        if door == "fridge-door":
            P.append((f"{door}-plug", door, "body", 0.9, 0.490, Y(y0 + 0.045), XG, Y(y0 + 1.005), -0.240, 0.240, None, "door-liner", None, "fp"))
            if s200:
                P.append(("balconies", door, "body", 0.8, 0.420, Y(y0 + 0.100), 0.490, Y(y0 + 0.960), -0.225, 0.225, None, "ps-clear", None, "fp"))
            else:
                for k, (ua, ub) in enumerate(((0.10, 0.20), (0.40, 0.49), (0.70, 0.79)), start=1):
                    P.append((f"balcony-{k}", door, "body", 0.8, 0.420, Y(y0 + ua), 0.490, Y(y0 + ub), -0.225, 0.225, None, "ps-clear", None, "fp"))
                if s50:
                    P.append(("butter-box", door, "body", 0.85, 0.440, Y(y0 + 0.880), 0.490, Y(y0 + 0.960), -0.180, 0.180, None, "abs-white", None, "fp"))
            if s50:
                P.append(("handle-bar", door, "body", 0.55, 0.608, Y(y0 + 0.300), 0.620, Y(y0 + 0.650), 0.245, 0.270, "doors", "aluminium-bar", None, "efp"))
                for k, uu in enumerate((0.320, 0.630), start=1):
                    P.append((f"standoff-{k}", door, "body", 0.5, XD, Y(y0 + uu - 0.007), 0.608, Y(y0 + uu + 0.007), 0.2505, 0.2645, "doors", "aluminium-bar", None, "efp"))
                P.append(("magnet", door, "body", 0.35, XD, Y(y0 + 0.700), 0.596, Y(y0 + 0.740), 0.030, 0.070, "doors", "magnet", None, "efp"))
            elif s100:
                P.append(("handle-bar", door, "body", 0.55, XD, Y(y0 + 0.300), 0.620, Y(y0 + 0.650), 0.245, 0.270, "doors", "aluminium-bar", None, "efp"))
        else:
            fx = 0.508 if s200 else 0.510
            P.append((f"{door}-plug", door, "body", 0.9, fx, Y(y0 + 0.070), XG, Y(y0 + 0.640), -0.235, 0.235, None, "door-liner", None, "fp"))
            if s50 or s100:
                P.append(("freezer-handle", door, "body", 0.85, XD, Y(y0 + 0.600), 0.606, Y(y0 + 0.665), 0.100, 0.280, "doors", "abs-white", None, "efp"))
    return P


def door_details_front(scale):
    """🏷 The traces of use on the door fronts, drawn in the front view as thin solids proud of the door face."""
    if scale != 50:
        if scale == 100:
            return [("price-label", "fridge-door", "detail", 0.98, Y(FR[0] + 0.880), Y(FR[0] + 0.925), -0.200, -0.130)]
        return []
    return [
        ("price-label", "fridge-door", "detail", 0.98, Y(FR[0] + 0.880), Y(FR[0] + 0.925), -0.200, -0.130),
        ("sticker-scrap-1", "fridge-door", "texture", 0.6, Y(FR[0] + 0.520), Y(FR[0] + 0.545), -0.080, -0.050),
        ("sticker-scrap-2", "fridge-door", "texture", 0.6, Y(FR[0] + 0.470), Y(FR[0] + 0.490), -0.020, 0.010),
        ("screw-hole-1", "fridge-door", "detail", 0.1, Y(FR[0] + 0.284), Y(FR[0] + 0.294), 0.232, 0.242),
        ("screw-hole-2", "fridge-door", "detail", 0.1, Y(FR[0] + 0.656), Y(FR[0] + 0.666), 0.232, 0.242),
        ("rust-spot", "freezer-door", "detail", 0.3, Y(FZ[0]), Y(FZ[0] + 0.012), -0.290, -0.262),
    ]


def geometry(scale):
    """🖼 The individually drawn elevation, front and plan of one scale."""
    s50, s100, s200, s500 = (scale == x for x in SCALES)
    E, F, P = [], [], []
    ct = 0.022 if s500 else CB
    E.append(shape("cardboard", "cardboard", "body", 1, 0.7, rect_e("cardboard", -0.030, 0.0, 0.660, ct), "cardboard", "corrugated-board"))
    F.append(shape("cardboard", "cardboard", "body", 2.5, 0.7, rect(-0.330, 0.0, 0.330, ct), "cardboard", "corrugated-board"))
    P.append(shape("cardboard", "cardboard", "body", 1, 0.7, rect(-0.030, -0.330, 0.660, 0.330), "cardboard", "corrugated-board", hatch="corrugation" if s50 else None))
    if s50:
        P.append(shape("carton-stain", "cardboard", "texture", 1.1, 0.5, circles([(0.600, 0.0, 0.035), (0.625, -0.07, 0.020)]), "cardboard", filled=False))
    truck_2d(scale, E, F, P)
    yb = 0.0 if scale >= 200 else FT
    xb = -0.030 if s500 else 0.0
    E.append(shape("side-panel", "cabinet", "body", 3, 0.92, rect_e("cabinet", xb, Y(yb), XF, Y(YTOP)), "cabinet" if not s500 else None, "cabinet-wall", regions=["cabinet", "condenser"] if s500 else None))
    F.append(shape("cabinet-front", "cabinet", "body", 3, 0.92, rect(-ZC, Y(yb), ZC, Y(YTOP)), "cabinet" if not s500 else None, "cabinet-wall", regions=["cabinet", "plinth"] if s500 else None))
    P.append(shape("cabinet-top", "cabinet", "body", 3, 0.92, rect(xb, -ZC, XF, ZC), "cabinet" if not s500 else None, "cabinet-wall", regions=["cabinet", "condenser"] if s500 else None))
    if not s500:
        xp = 0.500 if s200 else PLINTH[0]
        F.append(shape("plinth", "cabinet", "body", 3.2, 0.15, rect(-ZI, Y(yb), ZI, Y(PLINTH[1])), "plinth", "abs-black"))
        if s50:
            F.append(shape("grille-slots", "cabinet", "texture", 3.25, 0.4, lines([((zz, Y(FT + 0.012)), (zz, Y(PLINTH[1] - 0.012))) for zz in [(-0.225 + 0.025 * k) for k in range(19)]]), "plinth", filled=False))
    if s50 or s100:
        for side, sz in (("r", -1), ("l", 1)):
            F.append(shape(f"foot-f{side}", "cabinet", "body", 2.9, 0.25, rect(sz * 0.255 - 0.018, Y(0.0), sz * 0.255 + 0.018, Y(FT)), "cabinet", "pp-feet"))
        E.append(shape("foot-fr", "cabinet", "body", 2.9, 0.25, rect_e("cabinet", 0.472, Y(0.0), 0.508, Y(FT)), "cabinet", "pp-feet"))
        E.append(shape("roller-rr", "cabinet", "body", 2.9, 0.25, {"kind": "ellipse", **dict(zip(("cx", "cy"), L2("cabinet", 0.045, Y(0.015)))), "rx": 0.015, "ry": 0.015}, "cabinet", "pp-feet"))
    if s50:
        E.append(shape("hinge-top", "cabinet", "body", 3.4, 0.85, rect_e("cabinet", 0.480, Y(YTOP), 0.596, Y(1.850)), "cabinet", "abs-white"))
        F.append(shape("hinge-top", "cabinet", "body", 3.4, 0.85, rect(-ZC, Y(YTOP), -0.240, Y(1.850)), "cabinet", "abs-white"))
        P.append(shape("hinge-top", "cabinet", "body", 5.5, 0.85, rect(0.480, -ZC, 0.596, -0.240), "cabinet", "abs-white"))
        E.append(shape("hinge-bottom", "cabinet", "body", 3.4, 0.55, rect_e("cabinet", XF, Y(0.076), 0.596, Y(FZ[0])), "cabinet", "hinge-steel"))
        F.append(shape("hinge-bottom", "cabinet", "body", 3.4, 0.55, rect(-ZC, Y(0.076), -0.255, Y(FZ[0])), "cabinet", "hinge-steel"))
        E.append(shape("side-dent", "cabinet", "texture", 3.1, 0.5, {"kind": "ellipse", **dict(zip(("cx", "cy"), L2("cabinet", 0.300, Y(1.210)))), "rx": 0.030, "ry": 0.022}, "cabinet", filled=False))
        E.append(shape("side-scuffs", "cabinet", "texture", 3.1, 0.5, lines_e("cabinet", [((0.060, Y(0.140)), (0.140, Y(0.150))), ((0.080, Y(0.110)), (0.190, Y(0.118))), ((0.400, Y(0.420)), (0.460, Y(0.430)))]), "cabinet", filled=False))
        E.append(shape("rating-plate-residue", "cabinet", "texture", 3.1, 0.5, rect_e("cabinet", 0.040, Y(1.560), 0.110, Y(1.600)), "cabinet", filled=False))
        P.append(shape("microwave-marks", "cabinet", "texture", 3.1, 0.5, circles([(0.110, 0.190, 0.012), (0.420, 0.190, 0.012), (0.110, -0.190, 0.012), (0.420, -0.190, 0.012)]), "cabinet", filled=False))
        P.append(shape("dust-edge", "cabinet", "texture", 3.1, 0.5, rect(0.080, -0.220, 0.450, 0.220), "cabinet", filled=False))
    if s50 or s100:
        F.append(shape("fridge-liner", "cabinet", "body", 3.3, 0.97, rect(-ZI, Y(FRC[0]), ZI, Y(FRC[1])), None, "cabinet-wall"))
        for k, yg in enumerate((1.090, 1.310, 1.530), start=1):
            F.append(shape(f"shelf-{k}", "cabinet", "glazing" if s50 else "detail", 3.4, 0.7, rect(-ZI, Y(yg), ZI, Y(yg + 0.004)), None, "glass-shelf"))
            if s50:
                F.append(shape(f"trim-{k}", "cabinet", "body", 3.45, 0.85, rect(-ZI, Y(yg - 0.010), ZI, Y(yg + 0.010)), None, "abs-white"))
        F.append(shape("crisper", "cabinet", "body", 3.4, 0.75, rect(-0.245, Y(DIV[1]), 0.245, Y(0.977 if s50 else 0.981)), None, "ps-clear"))
        if s50:
            F.append(shape("crisper-split", "cabinet", "detail", 3.42, 0.5, lines([((0.0, Y(DIV[1])), (0.0, Y(0.977)))]), None, filled=False))
            F.append(shape("crisper-glass", "cabinet", "glazing", 3.45, 0.7, rect(-ZI, Y(0.977), ZI, Y(0.981)), None, "glass-shelf"))
        F.append(shape("lamp-housing", "cabinet", "body", 3.4, 0.85, rect(-0.200, Y(FRC[1] - 0.030), -0.120, Y(FRC[1])), None, "abs-white"))
    elif s200:
        F.append(shape("fridge-liner", "cabinet", "body", 3.3, 0.97, rect(-ZI, Y(FRC[0]), ZI, Y(FRC[1])), None, "cabinet-wall"))
        F.append(shape("lamp-housing", "cabinet", "body", 3.4, 0.85, rect(-0.200, Y(FRC[1] - 0.030), -0.120, Y(FRC[1])), None, "abs-white"))
    if not s500:
        bone = "condenser"
        if s200:
            E.append(shape("condenser", bone, "body", 2.8, 0.12, rect_e(bone, -0.024, Y(0.325), -0.016, Y(1.790)), "condenser", "condenser-wire"))
            P.append(shape("condenser", bone, "body", 2.8, 0.12, rect(-0.024, -0.245, -0.016, 0.245), "condenser", "condenser-wire"))
        else:
            E.append(shape("condenser", bone, "body", 2.8, 0.12, rect_e(bone, -0.024 if s50 else -0.0225, Y(0.325), -0.016 if s50 else -0.0175, Y(1.775)), "condenser", "condenser-wire"))
            P.append(shape("condenser", bone, "body", 2.8, 0.12, rect(-0.024 if s50 else -0.0225, -0.245, -0.016 if s50 else -0.0175, 0.245), "condenser", "condenser-wire"))
            xa = -0.0175
            for k, (yy, zz) in enumerate(((0.380, -0.235), (1.700, -0.235))):
                E.append(shape(f"cond-bracket-{k}", bone, "body", 2.85, 0.4, rect_e(bone, xa, Y(yy), 0.0, Y(yy + 0.020)), "condenser", "galvanised-steel"))
            for k, (zz) in enumerate((-0.235, 0.215)):
                P.append(shape(f"cond-bracket-{k}", bone, "body", 2.85, 0.4, rect(xa, zz, 0.0, zz + 0.020), "condenser", "galvanised-steel"))
            if s50:
                E.append(shape("condenser-tubes", bone, "texture", 2.9, 0.3, circles([(*L2(bone, -0.020, Y(0.330 + 0.12 * k)), 0.0025) for k in range(13)]), "condenser", filled=False))
        for zr in ((-0.210, -0.180), (0.180, 0.210)) if not s200 else ((-0.215, -0.185), (0.185, 0.215)):
            P.append(shape(f"wall-spacer-{'r' if zr[0] < 0 else 'l'}", "cabinet", "body", 2.9, 0.85, rect(-0.030, zr[0], 0.0, zr[1]), "cabinet", "abs-white"))
        E.append(shape("wall-spacer", "cabinet", "body", 2.9, 0.85, rect_e("cabinet", -0.030, Y(1.790), 0.0, Y(1.820)), "cabinet", "abs-white"))
        if s50 or s100:
            E.append(shape("cable-coil", "cabinet", "detail", 2.95, 0.2, lines_e("cabinet", [((-0.009, Y(0.187)), (-0.009, Y(0.293)))]), None, filled=False))
            P.append(shape("cable-coil", "cabinet", "detail", 2.95, 0.2, lines([((-0.009, 0.097), (-0.009, 0.203))]), None, filled=False))
            E.append(shape("plug", "cabinet", "body", 2.96, 0.2, rect_e("cabinet", -0.030, Y(0.130), -0.002, Y(0.190)), None, "power-cable"))
            P.append(shape("plug", "cabinet", "body", 2.96, 0.2, rect(-0.030, 0.130, -0.002, 0.170), None, "power-cable"))
            if s50:
                E.append(shape("cable-tape", "cabinet", "body", 2.97, 0.1, rect_e("cabinet", -0.012, Y(0.270), 0.0, Y(0.300)), None, "power-cable"))
    for sid, door, role, tone, x0, y0, x1, y1, z0, z1, reg, mat, regs, views in door_parts(scale):
        kind = next(k for k in ("slab", "gasket", "plug", "balcon", "butter", "handle", "standoff", "magnet") if k in sid)
        ze, zf, zp = {"slab": (5.0, 5.0, 5.0), "gasket": (4.5, 4.5, 4.5), "plug": (3.7, 3.7, 2.6), "balcon": (3.8, 3.8, 2.65), "butter": (3.85, 3.85, 2.66), "handle": (6.0, 6.0, 6.0), "standoff": (5.9, 5.9, 5.9), "magnet": (6.1, 6.1, 6.1)}[kind]
        dz = 0.0 if door == "fridge-door" else -0.05
        if "e" in views:
            E.append(door_shape(sid, door, role, ze + dz, tone, x0, y0, x1, y1, z0, z1, reg, mat, regs))
        if "f" in views:
            F.append(door_shape(sid, door, role, zf + dz, tone, x0, y0, x1, y1, z0, z1, reg, mat, regs))
        if "p" in views:
            P.append(door_shape(sid, door, role, zp + dz + (0.5 if kind == "slab" and door == "fridge-door" else 0.0), tone, x0, y0, x1, y1, z0, z1, reg, mat, regs))
    for sid, door, role, tone, y0, y1, z0, z1 in door_details_front(scale):
        F.append(door_shape(sid, door, role, 5.5, tone, XD - 0.0004, y0, XD, y1, z0, z1, "doors"))
    if s50:
        F.append(door_shape("old-handle-ghost", "fridge-door", "texture", 5.6, 0.9, XD - 0.0004, Y(FR[0] + 0.280), XD, Y(FR[0] + 0.670), 0.228, 0.274, "doors", filled=False))
        for k, (a, b) in enumerate((((0.180, FR[0] + 0.500), (0.215, FR[0] + 0.460)), ((0.190, FR[0] + 0.560), (0.230, FR[0] + 0.540)), ((0.150, FR[0] + 0.420), (0.175, FR[0] + 0.380))), start=1):
            F.append(scratch(f"door-scratch-{k}", "fridge-door", a, b))
    return {"elevation": E, "front": F, "plan": P}


def truck_2d(scale, E, F, P):
    """🛒 The hand truck drawn in the three views (it stands behind the fridge, so its front view is hidden at rest)."""
    s50, s100, s200, s500 = (scale == x for x in SCALES)
    tb, tf = "truck-base", "truck-frame"
    ax, ay = L2(tb, XA0, RW)
    if s500 or s200:
        E.append(shape("wheel", tb, "body", 2, 0.15, circle(ax, ay, RW), "truck-wheels", "tyre-rubber"))
    else:
        E.append(shape("tyre", tb, "body", 2, 0.15, circle(ax, ay, RW), "truck-wheels", "tyre-rubber"))
        E.append(shape("rim", tb, "body", 2.1, 0.6, circle(ax, ay, 0.078), "truck", "rim-steel"))
    E.append(shape("axle", tb, "body", 2.2, 0.4, circle(ax, ay, 0.010 if s200 or s500 else 0.016), "truck", "truck-plate"))
    t = 0.022 if s500 else 0.008
    x0, x1 = XA0 + 0.076, XA0 + 0.300
    E.append(shape("nose-plate", tf, "body", 2.3, 0.4, rect_e(tf, x0, 0.0, x1, t), "truck", "truck-plate"))
    E.append(shape("tube", tf, "body", 2.4, 0.35, caps_e(tf, (XA0 + TUBE_X, 0.008 + TUBE_R), (XA0 + TUBE_X, TOP_Y), TUBE_R), "truck", "truck-tube"))
    if s500:
        E.append(shape("bracket", tf, "body", 2.35, 0.4, rect_e(tf, XA0 - 0.0375, RW - 0.0375, XA0 + TUBE_X, RW + 0.0375), "truck", "truck-plate"))
    else:
        rs = 0.015 if s200 else 0.010
        E.append(shape("strut", tf, "body", 2.35, 0.35, caps_e(tf, (XA0 + TUBE_X, 0.230), (XA0, RW), rs), "truck", "truck-tube"))
    if s50 or s100:
        E.append(shape("grip", tf, "body", 2.45, 0.1, circle(*L2(tf, XA0 + TUBE_X, TOP_Y), 0.017), "truck", "pvc-grip"))
    wheel_z = [(-ZW - WW / 2, -ZW + WW / 2), (ZW - WW / 2, ZW + WW / 2)]
    for k, (za, zb) in enumerate(wheel_z):
        nm = "rl"[k]
        F.append(shape(f"wheel-{nm}", tb, "body", 1.0, 0.15, rect(za, 0.0, zb, 2 * RW), "truck-wheels", "tyre-rubber"))
        P.append(shape(f"wheel-{nm}", tb, "body", 2.0, 0.15, rect(XA0 - RW, za, XA0 + RW, zb), "truck-wheels", "tyre-rubber"))
    F.append(shape("axle", tb, "body", 1.1, 0.4, caps(-0.290, RW, 0.290, RW, 0.010), "truck", "truck-plate"))
    if s500:
        F.append(shape("bracket", tf, "body", 1.25, 0.4, rect(-0.1975, RW - 0.0375, 0.1975, RW + 0.0375), "truck", "truck-plate"))
        P.append(shape("bracket", tf, "body", 2.25, 0.4, rect(XA0 - 0.0375, -0.1975, XA0 + TUBE_X, 0.1975), "truck", "truck-plate"))
    P.append(shape("axle", tb, "body", 2.05, 0.4, caps(XA0, -0.290, XA0, 0.290, 0.010), "truck", "truck-plate"))
    F.append(shape("nose-plate", tf, "body", 1.2, 0.4, rect(-0.200, 0.0, 0.200, t), "truck", "truck-plate"))
    P.append(shape("nose-plate", tf, "body", 2.1, 0.4, rect(x0, -0.200, x1, 0.200), "truck", "truck-plate"))
    for k, sz in enumerate((-1, 1)):
        nm = "rl"[k]
        F.append(shape(f"tube-{nm}", tf, "body", 1.3, 0.35, caps(sz * TUBE_Z, 0.008 + TUBE_R, sz * TUBE_Z, TOP_Y, TUBE_R), "truck", "truck-tube"))
        P.append(shape(f"tube-{nm}", tf, "body", 2.3, 0.35, circle(XA0 + TUBE_X, sz * TUBE_Z, TUBE_R), "truck", "truck-tube"))
        if not s500:
            rs = 0.015 if s200 else 0.010
            P.append(shape(f"strut-{nm}", tf, "body", 2.25, 0.35, caps(XA0 + TUBE_X, sz * TUBE_Z, XA0, sz * TUBE_Z, rs), "truck", "truck-tube"))
    if not s500:
        rb = 0.015 if s200 else TUBE_R
        F.append(shape("top-bar", tf, "body", 1.4, 0.35, caps(-TUBE_Z, TOP_Y, TUBE_Z, TOP_Y, rb), "truck", "truck-tube"))
        P.append(shape("top-bar", tf, "body", 2.4, 0.35, caps(XA0 + TUBE_X, -TUBE_Z, XA0 + TUBE_X, TUBE_Z, rb), "truck", "truck-tube"))
        for k, yy in enumerate((0.33, 0.68, 1.03), start=1):
            F.append(shape(f"cross-bar-{k}", tf, "body", 1.35, 0.35, caps(-0.2035, yy, 0.2035, yy, 0.010), "truck", "truck-tube"))
    if s50 or s100:
        F.append(shape("grip", tf, "body", 1.45, 0.1, caps(-0.120, TOP_Y, 0.120, TOP_Y, 0.017), "truck", "pvc-grip"))
        P.append(shape("grip", tf, "body", 2.45, 0.1, caps(XA0 + TUBE_X, -0.120, XA0 + TUBE_X, 0.120, 0.017), "truck", "pvc-grip"))


PENS = {50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.7}, 100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5}, 200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35}, 500: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}
BANDS = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}
HATCHES = [{"id": "corrugation", "title": T("Corrugation of the flattened carton", "Wellung des flachgelegten Kartons"), "pattern": "lines", "angleDeg": 90, "spacingMm": 0.8, "penMm": 0.13}]


def decisions(scale):
    """📝 The drawing decisions of one scale."""
    D = {
        50: dict(
            silhouette=T("Fridge-freezer 13 x 37 mm on paper on its carton, the hand truck parked 0.36 m behind it. Side elevation: the right side panel with the feet, a rear roller and the bottom hinge, the yellowed gaskets as 8 mm strips between cabinet and doors, the 50 mm door edges, the replaced bar handle on its standoffs, the freezer handle and a fridge magnet standing proud of the doors; behind, the condenser strip on its brackets, the wall spacer, the coiled cable and its plug; the hand truck with tyre, rim, axle eye, strut, frame tube, grip and nose plate. Front elevation: the two doors with the price label, sticker scraps, the ghost and screw holes of the original handle, scratches, the rust spot on the freezer door, the plinth grille with its slots, the feet and the hinges; behind the doors the fridge liner, three shelves with trims, the crisper and the lamp housing (seen when the door opens). Plan: carton, condenser, cabinet top with the marks of a microwave and its dust edge, gasket, doors, handles, hand truck.", "Kühl-Gefrier-Kombination 13 x 37 mm auf dem Papier auf ihrem Karton, die Sackkarre 0,36 m dahinter abgestellt. Seitenansicht: die rechte Seitenwand mit Füßen, hinterer Rolle und unterem Scharnier, die vergilbten Dichtungen als 8-mm-Streifen zwischen Gehäuse und Türen, die 50-mm-Türkanten, der ersetzte Stangengriff auf seinen Abstandshaltern, der Gefriergriff und ein Kühlschrankmagnet vor den Türen; hinten der Verflüssigerstreifen auf seinen Haltern, der Wandabstandshalter, das aufgewickelte Kabel und sein Stecker; die Sackkarre mit Reifen, Felge, Achsauge, Strebe, Rahmenrohr, Griff und Schaufel. Vorderansicht: die beiden Türen mit Preisschild, Aufkleberresten, dem Schatten und den Schraublöchern des Originalgriffs, Kratzern, dem Roststreifen an der Gefriertür, das Sockelgitter mit Schlitzen, Füße und Scharniere; hinter den Türen der Kühlteil-Liner, drei Böden mit Blenden, das Gemüsefach und das Lampengehäuse (sichtbar, wenn die Tür aufgeht). Grundriss: Karton, Verflüssiger, Gehäusedecke mit den Abdrücken einer Mikrowelle und ihrem Staubrand, Dichtung, Türen, Griffe, Sackkarre."),
            retained=T("The 8 mm gasket gap, the 4 mm door gap, the 10 mm rounded door edges, the replaced handle with its old screw holes, the price label, the rust spot, the dent and scuffs of the side panel, the microwave marks, the condenser tubes, the taped cable coil and the water stains of the carton.", "Der 8-mm-Dichtungsspalt, die 4-mm-Türfuge, die 10-mm-gerundeten Türkanten, der ersetzte Griff mit den alten Schraublöchern, das Preisschild, der Roststreifen, Delle und Schrammen der Seitenwand, die Mikrowellenabdrücke, die Verflüssigerrohre, die geklebte Kabelrolle und die Wasserflecken des Kartons."),
            merged=T("Left wheel, left tube and the far door edges coincide with the near ones in the side elevation; the hand truck is hidden behind the fridge in the front elevation and is drawn under it.", "Linkes Rad, linkes Rohr und die hinteren Türkanten decken sich in der Seitenansicht mit den vorderen; die Sackkarre steht in der Vorderansicht verdeckt hinter dem Gerät und ist darunter gezeichnet."),
            omitted=T("The 0.5 mm skins and the foam, the 1.5 mm wires of the condenser (a filled strip), the compressor, pipes and relay in the recess behind the side panels.", "Die 0,5-mm-Bleche und der Schaum, die 1,5-mm-Drähte des Verflüssigers (ein gefüllter Streifen), Verdichter, Leitungen und Relais in der Nische hinter den Seitenwänden."),
            thinElements=T("The 3 mm carton and the 4 mm glass shelves are filled strips carried by their pens; the cable coil seen edge-on is a detail stroke.", "Der 3-mm-Karton und die 4-mm-Glasböden sind gefüllte Streifen, die ihr Strich trägt; die hochkant gesehene Kabelrolle ist ein Detailstrich."),
            hatching=T("Corrugation lines on the carton in plan.", "Wellungslinien auf dem Karton im Grundriss."),
            styles=T("Cabinet and doors 0.92-0.95 (yellowed white), gaskets 0.50, plinth 0.15, condenser 0.12, carton 0.70, hand truck 0.35, tyres 0.15.", "Gehäuse und Türen 0,92-0,95 (vergilbtes Weiß), Dichtungen 0,50, Sockel 0,15, Verflüssiger 0,12, Karton 0,70, Sackkarre 0,35, Reifen 0,15."),
            overlays=T("Every outer part carries its thermal region: cabinet, plinth, doors, gaskets, condenser, carton, hand truck and tyres; compressor, evaporators and lamp are hidden.", "Jedes äußere Teil trägt seine Thermikregion: Gehäuse, Sockel, Türen, Dichtungen, Verflüssiger, Karton, Sackkarre und Reifen; Verdichter, Verdampfer und Lampe sind verdeckt.")),
        100: dict(
            silhouette=T("Fridge-freezer 6.5 x 18.5 mm: the same parts without texture: feet, rollers, gaskets, doors, the replaced handle as one bar, the freezer handle, condenser, spacers, cable coil and plug, hand truck; the interior behind the doors with liner, three shelves, the crisper and the lamp housing.", "Kühl-Gefrier-Kombination 6,5 x 18,5 mm: dieselben Teile ohne Textur: Füße, Rollen, Dichtungen, Türen, der Ersatzgriff als eine Stange, der Gefriergriff, Verflüssiger, Abstandshalter, Kabelrolle und Stecker, Sackkarre; der Innenraum hinter den Türen mit Liner, drei Böden, Gemüsefach und Lampengehäuse."),
            retained=T("The gasket and door gaps, the price label, the replaced handle, the plinth band.", "Dichtungs- und Türfuge, das Preisschild, der Ersatzgriff, das Sockelband."),
            merged=T("Handle bar and standoffs into one bar; crisper drawers and cover into one bin; condenser wires into its strip.", "Griffstange und Abstandshalter zu einer Stange; Gemüseschalen und Abdeckung zu einer Schale; Verflüssigerdrähte in seinen Streifen."),
            omitted=T("Hinge cover, bottom hinge, magnet, tape, stickers, scratches, dent, grille slots, the microwave marks and the carton hatch.", "Scharnierkappe, unteres Scharnier, Magnet, Klebeband, Aufkleber, Kratzer, Delle, Gitterschlitze, die Mikrowellenabdrücke und die Kartonschraffur."),
            thinElements=T("Gaskets 8 mm and the carton 3 mm stay at their true size as filled strips carried by the body pen.", "Dichtungen 8 mm und Karton 3 mm bleiben in echter Größe als gefüllte Streifen, die der Körperstrich trägt."),
            hatching=T("None.", "Keine."),
            styles=T("As 1:50.", "Wie 1:50."),
            overlays=T("All eight drawn regions.", "Alle acht gezeichneten Regionen.")),
        200: dict(
            silhouette=T("Fridge-freezer 3.3 x 9.3 mm: cabinet reaching down to the carton, plinth band, gaskets, two door slabs, the condenser strip with the wall spacer; behind the doors the liner and lamp housing; hand truck with wheel, strut, tube and nose plate.", "Kühl-Gefrier-Kombination 3,3 x 9,3 mm: Gehäuse bis auf den Karton, Sockelband, Dichtungen, zwei Türblätter, der Verflüssigerstreifen mit dem Wandabstandshalter; hinter den Türen Liner und Lampengehäuse; Sackkarre mit Rad, Strebe, Rohr und Schaufel."),
            retained=T("Door division, gasket strips, plinth band, condenser at the back, the parked hand truck.", "Türteilung, Dichtungsstreifen, Sockelband, Verflüssiger hinten, die abgestellte Sackkarre."),
            merged=T("Feet and rollers into the cabinet; tyre and rim into one wheel; door bins into one block.", "Füße und Rollen ins Gehäuse; Reifen und Felge zu einem Rad; Türfächer zu einem Block."),
            omitted=T("Handles, hinges, cable, plug, shelves, crisper, grip, eyes and every trace of use under 30 mm.", "Griffe, Scharniere, Kabel, Stecker, Böden, Gemüsefach, Griff, Achsaugen und jede Gebrauchsspur unter 30 mm."),
            thinElements=T("Gaskets 8 mm, carton 3 mm and nose plate 8 mm stay true as filled strips carried by the 0.25 mm body pen.", "Dichtungen 8 mm, Karton 3 mm und Schaufel 8 mm bleiben echt als gefüllte Streifen, die der 0,25-mm-Körperstrich trägt."),
            hatching=T("None.", "Keine."),
            styles=T("As 1:50.", "Wie 1:50."),
            overlays=T("All eight drawn regions.", "Alle acht gezeichneten Regionen.")),
        500: dict(
            silhouette=T("Sign 1.3 x 3.7 mm: the cabinet block with the condenser merged into its back, two door slabs, the carton strip, the hand truck as wheel, bracket, tube and nose plate.", "Zeichen 1,3 x 3,7 mm: der Gehäuseblock mit dem in die Rückseite verschmolzenen Verflüssiger, zwei Türblätter, der Kartonstreifen, die Sackkarre als Rad, Halter, Rohr und Schaufel."),
            retained=T("Height, depth and the two doors of the fridge-freezer, the hand truck behind it.", "Höhe, Tiefe und die beiden Türen der Kühl-Gefrier-Kombination, die Sackkarre dahinter."),
            merged=T("Gaskets into the doors, plinth into the cabinet front, condenser into the cabinet back; struts and eyes into one 75 mm bracket.", "Dichtungen in die Türen, Sockel in die Gehäusefront, Verflüssiger in die Gehäuserückseite; Streben und Achsaugen zu einem 75-mm-Halter."),
            omitted=T("Interior, handles, feet, cross bars, grip and every detail under 75 mm.", "Innenraum, Griffe, Füße, Querstreben, Griff und jedes Detail unter 75 mm."),
            thinElements=T("Carton and nose plate are 22 mm strips so that with the 0.18 mm body pen they still print 0.2 mm wide at 1:1000; the 28 mm tubes keep their true width.", "Karton und Schaufel sind 22-mm-Streifen, damit sie mit dem 0,18-mm-Körperstrich bei 1:1000 noch 0,2 mm breit drucken; die 28-mm-Rohre behalten ihre echte Breite."),
            hatching=T("None.", "Keine."),
            styles=T("As 1:50.", "Wie 1:50."),
            overlays=T("Cabinet with condenser and plinth merged, doors with gaskets merged, carton, hand truck and tyres.", "Gehäuse mit verschmolzenem Verflüssiger und Sockel, Türen mit verschmolzenen Dichtungen, Karton, Sackkarre und Reifen.")),
    }
    return D[scale]


def representations():
    """🖨 The four authored drawings."""
    out = {}
    for scale in SCALES:
        lo, hi = BANDS[scale]
        out[str(scale)] = {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": HATCHES if scale == 50 else [], "geometry": geometry(scale), "decisions": decisions(scale)}
    return out


# ---------------------------------------------------------------- rig

def rig():
    """🦴 The rig: carton fixed, the hand truck rolling and tilting on its own chain, the fridge rolled, lowered and tilted about the engaged axle, two hinged doors."""
    M = masses()

    def mass(bone, ref):
        kg, (u, v), _ = M[bone]
        return {"kg": kg, "comX": u, "comY": v, "distribution": "point", "source": "authored-estimate", "reference": ref}

    def rel(child, parent):
        (cx, cy, cz), _ = BONES[child]
        u, v, _w = L3(parent, cx, cy, BONES[parent][0][2])
        return u, v

    cab = rel("cabinet", "carry-tilt")
    fd = rel("fridge-door", "cabinet")
    zd = rel("freezer-door", "cabinet")
    cs = rel("condenser", "cabinet")
    tube_a = 2 * math.pi * (0.014 ** 2 - 0.012 ** 2)
    tube_i = 2 * math.pi / 64 * (0.028 ** 4 - 0.024 ** 4)
    return {"kind": "mechanical", "root": "ground", "bones": [
        {"id": "ground", "parent": None, "title": T("Floor reference", "Bodenbezug"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
        {"id": "cardboard", "parent": "ground", "title": T("Flattened carton under the fridge (storage aid, stays behind)", "Flachgelegter Karton unter dem Gerät (Lagerhilfe, bleibt liegen)"), "x": -0.030, "y": 0, "angleDeg": 0, "lengthM": 0.69, "joint": {"type": "fixed"}, "mass": mass("cardboard", "reuse-depot-practice")},
        {"id": "truck-base", "parent": "ground", "title": T("Hand truck: wheels and axle rolling on the floor", "Sackkarre: Räder und Achse, auf dem Boden rollend"), "x": XA0, "y": RW, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 2.6}, "mass": mass("truck-base", "hand-truck-catalogue")},
        {"id": "truck-frame", "parent": "truck-base", "title": T("Hand truck frame tilting about the axle", "Rahmen der Sackkarre, um die Achse kippend"), "x": 0, "y": 0, "angleDeg": 90, "lengthM": TOP_Y - RW, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 50}, "mass": mass("truck-frame", "hand-truck-catalogue"), "section": {"areaM2": tube_a, "secondMomentM4": tube_i, "extremeFibreM": 0.014, "material": "truck-plate", "title": T("Two steel tubes 28 x 2 mm", "Zwei Stahlrohre 28 x 2 mm")}},
        {"id": "carry-roll", "parent": "ground", "title": T("Fridge rolled with the hand truck", "Gerät mit der Sackkarre gerollt"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 2.0}},
        {"id": "carry-lift", "parent": "carry-roll", "title": T("Fridge set onto the nose plate or off the carton", "Gerät auf die Schaufel oder vom Karton gesetzt"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.01, "maxM": 0.01}},
        {"id": "carry-tilt", "parent": "carry-lift", "title": T("Tilting back about the engaged hand-truck axle", "Zurückkippen um die angesetzte Achse der Sackkarre"), "x": XPE, "y": RW, "angleDeg": 0, "lengthM": 0, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 45}},
        {"id": "cabinet", "parent": "carry-tilt", "title": T("Foamed cabinet with interior, machine recess and cable", "Geschäumtes Gehäuse mit Innenraum, Maschinennische und Kabel"), "x": cab[0], "y": cab[1], "angleDeg": 90, "lengthM": YTOP - FT, "joint": {"type": "fixed"}, "mass": mass("cabinet", "fridge-freezer-catalogue"), "section": {"areaM2": 2 * 0.53 * 0.0005 + 2 * 0.6 * 0.0005, "secondMomentM4": 2 * 0.0005 * 0.53 ** 3 / 12 + 2 * 0.6 * 0.0005 * 0.265 ** 2, "extremeFibreM": 0.265, "material": "cabinet-wall", "title": T("Steel skins 0.5 mm of the foamed box 600 x 530 mm", "Stahlbleche 0,5 mm des geschäumten Kastens 600 x 530 mm")}},
        {"id": "condenser", "parent": "cabinet", "title": T("Wire-on-tube condenser on the back", "Drahtrohrverflüssiger an der Rückwand"), "x": cs[0], "y": cs[1], "angleDeg": 0, "lengthM": 1.445, "joint": {"type": "fixed"}, "mass": mass("condenser", "fridge-freezer-catalogue")},
        {"id": "fridge-door", "parent": "cabinet", "title": T("Fridge door on its right-hand hinges", "Kühlteiltür an ihren rechten Scharnieren"), "x": fd[0], "y": fd[1], "z": -ZD, "angleDeg": 0, "lengthM": FR[1] - FR[0], "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": -115, "twistMaxDeg": 0}, "mass": mass("fridge-door", "fridge-freezer-catalogue"), "section": {"areaM2": 0.598 * 0.0005 * 2, "secondMomentM4": 2 * 0.598 * 0.0005 * 0.025 ** 2, "extremeFibreM": 0.025, "material": "door-panel", "title": T("Steel skin and liner of the 50 mm door", "Stahlblech und Liner der 50-mm-Tür")}},
        {"id": "freezer-door", "parent": "cabinet", "title": T("Freezer door on its right-hand hinges", "Gefrierteiltür an ihren rechten Scharnieren"), "x": zd[0], "y": zd[1], "z": -ZD, "angleDeg": 0, "lengthM": FZ[1] - FZ[0], "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": -115, "twistMaxDeg": 0}, "mass": mass("freezer-door", "fridge-freezer-catalogue"), "section": {"areaM2": 0.598 * 0.0005 * 2, "secondMomentM4": 2 * 0.598 * 0.0005 * 0.025 ** 2, "extremeFibreM": 0.025, "material": "door-panel", "title": T("Steel skin and liner of the 50 mm door", "Stahlblech und Liner der 50-mm-Tür")}},
    ]}


# ---------------------------------------------------------------- thermal

ENV = {"airTempC": 20, "meanRadiantTempC": 20, "relativeHumidity": 0.5, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 0.15, "pressurePa": 101325}

AREAS = {"cabinet": 4.15, "doors": 1.50, "gasket": 0.250, "plinth": 0.0985, "condenser": 1.18, "compressor": 0.100, "fridge-evaporator": 0.670, "freezer-evaporator": 0.897, "lamp": 0.0096, "cardboard": 0.919, "truck": 0.711, "truck-wheels": 0.327}


def lay(lid, title, kind, material, R, hc, src, ref, t=None):
    """🧶 One layer (no gauge unless stated, so no shell is required at 1:50)."""
    out = {"id": lid, "title": title, "kind": kind, "material": material, "resistanceM2KW": R, "heatCapacityJM2K": hc, "source": src, "reference": ref}
    if t is not None:
        out["thicknessM"] = t
    return out


def region(rid, title, kind, model, material, L, geom, layers, hc, ref, extra=None):
    """🔥 A thermal region."""
    r = {"id": rid, "title": title, "kind": kind, "model": model, "material": material, "areaM2": AREAS[rid], "characteristicLengthM": L, "geometry": geom, "skyViewFactor": 0.3, "layers": layers, "heatCapacityJM2K": hc, "source": "authored-estimate", "reference": ref}
    if extra:
        r.update(extra)
    return r


HIDE_RECESS = T("In the machine recess at the bottom of the back, between the two side panels, so neither elevation nor plan shows it.", "In der Maschinennische unten an der Rückseite zwischen den beiden Seitenwänden, daher in keiner Ansicht und keinem Grundriss sichtbar.")
HIDE_PLINTH = T("Set back between the two side panels below the freezer door and facing forward, so the side elevation sees the side panel in front of it; the front elevation draws it.", "Zwischen den beiden Seitenwänden unter der Gefriertür zurückgesetzt und nach vorn gerichtet, daher sieht die Seitenansicht die Seitenwand davor; die Vorderansicht zeichnet es.")
HIDE_INSIDE = T("Inside the closed compartment behind the door, so no view shows it.", "Im geschlossenen Fach hinter der Tür, daher in keiner Ansicht sichtbar.")


def thermal_model():
    """🌡 Twelve regions: the passive shell, the refrigerant circuit as two hydronic evaporators and a hydronic condenser keyed to the compressor, the warm compressor shell and the door lamp."""
    foam = lay("pu-foam", T("PU foam 47 mm", "PU-Schaum 47 mm"), "insulation", "cabinet-wall", 0.047 / 0.024, 1900, "literature", "iso-10456-2007")
    skin = lay("steel-skin", T("Enamelled steel skin 0.5 mm", "Einbrennlackiertes Stahlblech 0,5 mm"), "metal", "cabinet-wall", 0.0005 / 50, 1800, "literature", "iso-10456-2007")
    dfoam = lay("door-foam", T("PU foam 47 mm", "PU-Schaum 47 mm"), "insulation", "door-panel", 0.047 / 0.024, 1900, "literature", "iso-10456-2007")
    dskin = lay("door-skin", T("Enamelled steel skin 0.5 mm", "Einbrennlackiertes Stahlblech 0,5 mm"), "metal", "door-panel", 0.0005 / 50, 1800, "literature", "iso-10456-2007")
    regions = [
        region("cabinet", T("Cabinet sides, top and back, yellowed white enamel with a dent", "Gehäuseseiten, Decke und Rückwand, vergilbte weiße Einbrennlackierung mit Delle"), "surface", "passive", "cabinet-wall", 1.82, "vertical-plate", [foam, skin], 4000, "en-62552-2013", {"ratedPowerW": 0, "ratedPowerBasis": "passive"}),
        region("doors", T("Fridge and freezer door fronts with the replaced handle", "Fronten der Kühl- und Gefriertür mit dem Ersatzgriff"), "surface", "passive", "door-panel", 1.06, "vertical-plate", [dfoam, dskin], 3700, "en-62552-2013"),
        region("gasket", T("Magnetic door gaskets, yellowed", "Magnetische Türdichtungen, vergilbt"), "surface", "passive", "pvc-gasket", 0.02, "vertical-plate", [lay("gasket-wall", T("PVC bellows with magnet strip", "PVC-Balg mit Magnetband"), "polymer", "pvc-gasket", 0.05, 4000, "authored-estimate", "iso-10456-2007")], 4000, "iso-10456-2007"),
        region("plinth", T("Black plinth grille", "Schwarzes Sockelgitter"), "surface", "passive", "abs-black", 0.07, "vertical-plate", [lay("grille", T("ABS 2.5 mm", "ABS 2,5 mm"), "polymer", "abs-black", 0.015, 3500, "literature", "iso-10456-2007")], 3500, "iso-10456-2007", {"hidden": HIDE_PLINTH}),
        region("condenser", T("Wire-on-tube condenser, condensing at about 45 degC when running", "Drahtrohrverflüssiger, im Betrieb bei etwa 45 degC kondensierend"), "heater", "hydronic", "condenser-wire", 0.005, "cylinder", [lay("cond-wall", T("Painted steel tube wall 0.6 mm", "Lackierte Stahlrohrwand 0,6 mm"), "metal", "condenser-tube", 0.0001, 2000, "literature", "incropera-2018")], 2000, "en-62552-2013", {"areaBasis": "exchange", "hydronic": {"key": "compressor", "rows": [{"v": 1, "supplyTempC": 50, "returnTempC": 32}], "flowPosition": 0.5, "filmM2KW": 0.004}, "ratedPowerW": 0, "ratedPowerBasis": "passive"}),
        region("compressor", T("Hermetic compressor shell", "Hermetisches Verdichtergehäuse"), "engine", "operating", "compressor-steel", 0.14, "cylinder", [lay("shell", T("Steel shell 3 mm", "Stahlgehäuse 3 mm"), "metal", "compressor-steel", 0.00006, 15000, "literature", "incropera-2018")], 15000, "en-60335-2-24", {"hidden": HIDE_RECESS, "operating": {"key": "compressor", "heatW": 35}, "ratedPowerW": 100, "ratedPowerBasis": "electrical-input"}),
        region("fridge-evaporator", T("Cold-wall evaporator behind the fridge liner", "Kaltwandverdampfer hinter dem Kühlteil-Liner"), "surface", "hydronic", "aluminium-plate", 0.75, "vertical-plate", [lay("frost", T("Aluminium plate and thin frost", "Aluminiumplatte und dünner Reif"), "metal", "aluminium-plate", 0.004, 3000, "authored-estimate", "incropera-2018")], 3000, "en-62552-2013", {"hidden": HIDE_INSIDE, "hydronic": {"key": "compressor", "rows": [{"v": 1, "supplyTempC": -18, "returnTempC": -24}], "flowPosition": 0.5, "filmM2KW": 0.005}, "ratedPowerW": 0, "ratedPowerBasis": "passive"}),
        region("freezer-evaporator", T("Freezer evaporator shelves", "Verdampferböden des Gefrierteils"), "surface", "hydronic", "aluminium-plate", 0.45, "horizontal-plate-up", [lay("frost-freezer", T("Aluminium plate and frost 2 mm", "Aluminiumplatte und Reif 2 mm"), "metal", "aluminium-plate", 0.01, 3000, "authored-estimate", "incropera-2018")], 3000, "en-62552-2013", {"hidden": HIDE_INSIDE, "hydronic": {"key": "compressor", "rows": [{"v": 1, "supplyTempC": -24, "returnTempC": -28}], "flowPosition": 0.5, "filmM2KW": 0.005}, "ratedPowerW": 0, "ratedPowerBasis": "passive"}),
        region("lamp", T("Interior lamp, incandescent 15 W", "Innenbeleuchtung, Glühlampe 15 W"), "lamp", "operating", "glass-bulb", 0.04, "sphere", [lay("bulb", T("Glass bulb 0.5 mm", "Glaskolben 0,5 mm"), "glass", "glass-bulb", 0.0005, 900, "literature", "incropera-2018")], 900, "en-60335-2-24", {"hidden": HIDE_INSIDE, "operating": {"key": "lamp", "heatW": 15}, "ratedPowerW": 15, "ratedPowerBasis": "electrical-input"}),
        region("cardboard", T("Flattened carton, water-stained", "Flachgelegter Karton mit Wasserflecken"), "surface", "passive", "corrugated-board", 0.69, "horizontal-plate-up", [lay("board", T("Corrugated board 3 mm", "Wellpappe 3 mm"), "polymer", "corrugated-board", 0.05, 600, "authored-estimate", "iso-10456-2007")], 600, "iso-10456-2007"),
        region("truck", T("Hand-truck frame, nose plate and axle, powder-coated steel", "Rahmen, Schaufel und Achse der Sackkarre, pulverbeschichteter Stahl"), "surface", "passive", "truck-tube", 0.028, "cylinder", [lay("truck-steel", T("Steel wall 2 mm", "Stahlwand 2 mm"), "metal", "truck-tube", 0.00004, 7000, "literature", "en-10219-2-2019")], 7000, "en-10219-2-2019"),
        region("truck-wheels", T("Pneumatic tyres", "Luftreifen"), "tyre", "passive", "tyre-rubber", 0.26, "cylinder", [lay("rubber", T("Rubber 6 mm", "Gummi 6 mm"), "polymer", "tyre-rubber", 0.03, 6000, "authored-estimate", "iso-10456-2007")], 6000, "iso-10456-2007"),
    ]
    return {"environment": ENV, "operating": [
        {"id": "compressor", "title": T("Compressor and refrigerant circuit", "Verdichter und Kältekreis"), "levels": [{"v": 0, "title": T("Off or unplugged", "Aus oder ausgesteckt")}, {"v": 1, "title": T("Running (about 85 W input, evaporating -25 degC, condensing 45 degC)", "In Betrieb (etwa 85 W Aufnahme, Verdampfung -25 degC, Kondensation 45 degC)")}]},
        {"id": "lamp", "title": T("Interior lamp (door switch)", "Innenbeleuchtung (Türschalter)"), "levels": [{"v": 0, "title": T("Off, door closed", "Aus, Tür zu")}, {"v": 1, "title": T("On, door open (15 W)", "An, Tür offen (15 W)")}]},
    ], "regions": regions, "emitters": [
        {"id": "condenser-plume", "kind": "plume", "title": T("Warm air rising along the condenser behind the fridge", "Warme Luft, die am Verflüssiger hinter dem Gerät aufsteigt"), "anchor": "condenser-top", "directionDeg": 90, "diameterM": 0.40, "region": "condenser", "source": "authored-estimate", "reference": "incropera-2018"},
    ], "passengers": [], "assumptions": [
        T("Refrigerant circuit as temperatures, not as a cycle model: while the compressor runs (level 1) the condenser tubes hold the condensing refrigerant at 50 degC falling to 32 degC at the liquid outlet, the cold-wall evaporator -24 to -18 degC and the freezer shelves -28 to -24 degC (hydronic circuits keyed to the compressor, catalogue-typical R600a values for an A+ appliance of this size); unplugged they follow the room air. The compressor shell releases 35 W of its about 85 W input directly; the nameplate is 100 W (compressor) plus the 15 W lamp, 115 W in total.", "Kältekreis als Temperaturen, nicht als Kreisprozessmodell: läuft der Verdichter (Stufe 1), halten die Verflüssigerrohre das kondensierende Kältemittel bei 50 degC, fallend auf 32 degC am Flüssigkeitsaustritt, der Kaltwandverdampfer -24 bis -18 degC und die Gefrierböden -28 bis -24 degC (hydronische Kreise an der Verdichterstufe, katalogtypische R600a-Werte eines A+-Geräts dieser Größe); ausgesteckt folgen sie der Raumluft. Das Verdichtergehäuse gibt 35 W seiner etwa 85 W Aufnahme direkt ab; das Typenschild nennt 100 W (Verdichter) plus 15 W Lampe, zusammen 115 W."),
        T("Aged surfaces: the white enamel of cabinet and doors has yellowed, solar absorptance 0.30 instead of about 0.20 when new; the PVC gaskets have yellowed and stiffened (0.55 instead of 0.35); the emissivity of enamel and PVC stays 0.90-0.92 (Incropera; authored-estimate).", "Gealterte Oberflächen: der weiße Einbrennlack von Gehäuse und Türen ist vergilbt, solarer Absorptionsgrad 0,30 statt etwa 0,20 im Neuzustand; die PVC-Dichtungen sind vergilbt und verhärtet (0,55 statt 0,35); der Emissionsgrad von Lack und PVC bleibt 0,90-0,92 (Incropera; Schätzwert)."),
        T("Reference case: indoors at 20 degC, still air, no sun, in the depot as in the kitchen. Areas are the outer areas of the 1:50 model (validator flood); the condenser states its tube and wire surface as an exchange area.", "Referenzfall: innen bei 20 degC, ruhige Luft, keine Sonne, im Lager wie in der Küche. Flächen sind die äußeren Flächen des 1:50-Modells (Flutung des Validators); der Verflüssiger nennt seine Rohr- und Drahtoberfläche als Austauschfläche."),
    ]}


ALL_REGIONS = ["cabinet", "doors", "gasket", "plinth", "condenser", "compressor", "fridge-evaporator", "freezer-evaporator", "lamp", "cardboard", "truck", "truck-wheels"]


def binding(kind):
    """🌬 The thermal binding of one pose family."""
    r = {k: {"exposedFraction": 1, "sunlitFraction": 0} for k in ALL_REGIONS}
    r["cardboard"] = {"exposedFraction": 0.52, "sunlitFraction": 0, "contact": {"fraction": 0.48, "tempC": 18}}
    r["truck-wheels"] = {"exposedFraction": 0.98, "sunlitFraction": 0, "contact": {"fraction": 0.02, "tempC": 18}}
    op = {"compressor": 0, "lamp": 0}
    em = {"condenser-plume": 0}
    if kind == "carried":
        r["truck"] = {"exposedFraction": 0.99, "sunlitFraction": 0, "contact": {"fraction": 0.01, "tempC": 32}}
    if kind in ("in-use", "door-open"):
        op["compressor"] = 1
        em["condenser-plume"] = 1
    if kind == "door-open":
        op["lamp"] = 1
    return {"operating": op, "regions": r, "emitters": em}


# ---------------------------------------------------------------- poses and clips

CONTENTS = {"id": "contents", "kind": "live", "title": T("Food in both compartments (25 kg)", "Lebensmittel in beiden Fächern (25 kg)"), "anchor": "contents", "fxN": 0, "fyN": -25 * G, "source": "authored-estimate", "reference": "en-62552-2013"}
BOTTLES = {"id": "bottles", "kind": "live", "title": T("Bottles and jars in the door balconies (6 kg)", "Flaschen und Gläser in den Türfächern (6 kg)"), "anchor": "door-shelf", "fxN": 0, "fyN": -6 * G, "source": "authored-estimate", "reference": "en-60335-2-24"}

STORED_C = CARD_C + WHEEL_C + NOSE_C
ENGAGED = {"truck-base": {"dx": D_E}, "truck-frame": {"angleDeg": PRE}}
TILTED = {"truck-base": {"dx": D_E}, "truck-frame": {"angleDeg": TILT_T}, "carry-lift": {"dy": DROP}, "carry-tilt": {"angleDeg": TILT_F}}
CARRIED = {"truck-base": {"dx": D_E + ROLL_C}, "truck-frame": {"angleDeg": TILT_T}, "carry-roll": {"dx": ROLL_C}, "carry-lift": {"dy": DROP}, "carry-tilt": {"angleDeg": TILT_F}}
ARRIVED = {"truck-base": {"dx": D_E + ROLL_U}, "truck-frame": {"angleDeg": TILT_T}, "carry-roll": {"dx": ROLL_U}, "carry-lift": {"dy": DROP}, "carry-tilt": {"angleDeg": TILT_F}}
UPRIGHT = {"truck-base": {"dx": D_E + ROLL_U}, "carry-roll": {"dx": ROLL_U}, "carry-lift": {"dy": -CB}}
IN_USE = {"truck-base": {"dx": ROLL_U}, "carry-roll": {"dx": ROLL_U}, "carry-lift": {"dy": -CB}}
DOOR = dict(IN_USE, **{"fridge-door": {"twistDeg": DOOR_OPEN}})


def pose(pid, title, desc, bones, contacts, kind, loads=(), key=False):
    """🧍 One pose."""
    p = {"id": pid, "title": title, "description": desc}
    if key:
        p["key"] = True
    p.update({"bones": bones, "ground": "auto", "contacts": contacts, "thermal": binding(kind), "loads": list(loads)})
    return p


def poses():
    """🧍 Stored, carried, in use, door open and the key poses of the carry and the set-down."""
    return [
        pose("stored", T("Stored on a carton in the depot", "Auf einem Karton im Lager abgestellt"), T("Emptied, cleaned and unplugged, the fridge-freezer stands on a flattened carton that catches the last defrost water, its cable coiled and taped to the back, a price label on the door; the mover's hand truck is parked behind it.", "Geleert, gereinigt und ausgesteckt steht die Kühl-Gefrier-Kombination auf einem flachgelegten Karton, der das letzte Abtauwasser auffängt, das Kabel aufgewickelt und an die Rückwand geklebt, ein Preisschild an der Tür; die Sackkarre des Trägers steht dahinter."), {}, STORED_C, "stored"),
        pose("carried", T("Carried upright on the hand truck", "Aufrecht auf der Sackkarre gefahren"), T("Strapped upright to the tall hand truck, tilted back 38 degrees onto its wheels and wheeled away by one mover holding the grip; the carton stays behind.", "Aufrecht an die hohe Sackkarre gegurtet, um 38 Grad auf die Räder zurückgekippt und von einem Träger am Griff weggefahren; der Karton bleibt liegen."), CARRIED, WHEEL_C + GRIP_C, "carried"),
        pose("in-use", T("Running in a kitchen", "In einer Küche in Betrieb"), T("Set down on its feet in a kitchen and plugged in: the compressor runs, the condenser at the back is warm, both compartments are filled; the hand truck stands behind it.", "In einer Küche auf die Füße gestellt und eingesteckt: der Verdichter läuft, der Verflüssiger hinten ist warm, beide Fächer sind gefüllt; die Sackkarre steht dahinter."), IN_USE, FEET_C + WHEEL_C + NOSE_C, "in-use", [CONTENTS, BOTTLES]),
        pose("door-open", T("Fridge door open", "Kühlteiltür offen"), T("Running, the fridge door swung open 100 degrees with the lamp on and the filled door balconies hanging on the hinges.", "In Betrieb, die Kühlteiltür um 100 Grad geöffnet, die Lampe an und die gefüllten Türfächer an den Scharnieren."), DOOR, FEET_C + WHEEL_C + NOSE_C, "door-open", [CONTENTS, BOTTLES]),
        pose("engaged-key", T("Nose plate slid under (key)", "Schaufel untergeschoben (Schlüssel)"), T("The hand truck rolled in behind the fridge, its frame 5 mm off the wall spacers, the nose plate slid between the feet over the carton edge.", "Die Sackkarre hinter das Gerät gerollt, ihr Rahmen 5 mm vor den Wandabstandshaltern, die Schaufel zwischen den Füßen über die Kartonkante geschoben."), ENGAGED, CARD_C + WHEEL_C, "stored", key=True),
        pose("tilted-key", T("Tilted back onto the wheels (key)", "Auf die Räder zurückgekippt (Schlüssel)"), T("Strapped and tilted back 38 degrees about the axle, the fridge resting on the nose plate and against the frame.", "Gegurtet und um 38 Grad um die Achse zurückgekippt, das Gerät auf der Schaufel und am Rahmen anliegend."), TILTED, WHEEL_C + GRIP_C, "carried", key=True),
        pose("arrived-key", T("Arrived in the kitchen (key)", "In der Küche angekommen (Schlüssel)"), T("Wheeled to its place, still tilted.", "An seinen Platz gefahren, noch gekippt."), ARRIVED, WHEEL_C + GRIP_C, "carried", key=True),
        pose("upright-key", T("Stood up again (key)", "Wieder aufgestellt (Schlüssel)"), T("Tilted forward onto its feet, the nose plate still under it.", "Nach vorn auf die Füße gekippt, die Schaufel noch darunter."), UPRIGHT, FEET_C + WHEEL_C + NOSE_C, "carried", key=True),
    ]


def spans(ids, a, b):
    """🦶 Contact spans."""
    return [{"anchor": x, "fromS": a, "toS": b} for x in ids]


def clips():
    """🎞 Carry out of the depot, set down in a kitchen and the door opening cycle."""
    return [
        {"id": "carry", "title": T("Carry out of the depot", "Aus dem Lager fahren"), "description": T("The mover rolls the hand truck in behind the fridge, slides the nose plate under it, straps it, tilts it back onto the wheels and wheels it away; the carton stays on the floor.", "Der Träger rollt die Sackkarre hinter das Gerät, schiebt die Schaufel darunter, gurtet es, kippt es auf die Räder zurück und fährt es weg; der Karton bleibt auf dem Boden."), "durationS": 7, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1.6, "pose": "engaged-key", "ease": "smooth"}, {"t": 2.4, "pose": "engaged-key", "ease": "smooth"}, {"t": 4.0, "pose": "tilted-key", "ease": "smooth"}, {"t": 7, "pose": "carried", "ease": "smooth"}],
         "contacts": spans(CARD_C, 0, 7) + spans(WHEEL_C, 0, 7) + spans(NOSE_C, 0, 0.4) + spans(GRIP_C, 1.0, 7),
         "phases": [{"id": "roll-in", "fromS": 0, "toS": 1.6, "title": T("Roll in", "Heranrollen")}, {"id": "strap", "fromS": 1.6, "toS": 2.4, "title": T("Strap", "Gurten")}, {"id": "tilt", "fromS": 2.4, "toS": 4.0, "title": T("Tilt back", "Zurückkippen")}, {"id": "wheel-away", "fromS": 4.0, "toS": 7, "title": T("Wheel away", "Wegfahren")}],
         "events": [{"t": 0.4, "kind": "contact-off", "title": T("Nose plate leaves the floor", "Schaufel hebt vom Boden ab"), "target": "truck-nose"}, {"t": 1.0, "kind": "contact-on", "title": T("Mover takes the grip", "Träger fasst den Griff"), "target": "grip-r"}, {"t": 2.6, "kind": "motion", "title": T("Feet leave the carton", "Füße lösen sich vom Karton")}],
         "tracks": []},
        {"id": "set-down", "title": T("Set down in a kitchen", "In einer Küche abstellen"), "description": T("Wheeled to its place, tilted forward onto its feet, the hand truck pulled back out.", "An seinen Platz gefahren, nach vorn auf die Füße gekippt, die Sackkarre zurückgezogen."), "durationS": 6, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "carried", "ease": "smooth"}, {"t": 2.0, "pose": "arrived-key", "ease": "smooth"}, {"t": 3.8, "pose": "upright-key", "ease": "smooth"}, {"t": 6, "pose": "in-use", "ease": "smooth"}],
         "contacts": spans(CARD_C, 0, 6) + spans(WHEEL_C, 0, 6) + spans(GRIP_C, 0, 5.2) + spans(FEET_C, 3.6, 6) + spans(NOSE_C, 3.8, 6),
         "phases": [{"id": "wheel-in", "fromS": 0, "toS": 2.0, "title": T("Wheel in", "Heranfahren")}, {"id": "stand-up", "fromS": 2.0, "toS": 3.8, "title": T("Stand up", "Aufstellen")}, {"id": "pull-out", "fromS": 3.8, "toS": 6, "title": T("Pull the hand truck out", "Sackkarre herausziehen")}],
         "events": [{"t": 3.6, "kind": "contact-on", "title": T("Feet on the floor", "Füße auf dem Boden"), "target": "fridge-foot-fr"}, {"t": 5.2, "kind": "contact-off", "title": T("Mover lets go", "Träger lässt los"), "target": "grip-r"}],
         "tracks": [{"target": "load:contents", "ease": "step", "keys": [{"t": 0, "v": 0}]}, {"target": "load:bottles", "ease": "step", "keys": [{"t": 0, "v": 0}]}, {"target": "operating:compressor", "ease": "step", "keys": [{"t": 0, "v": 0}]}, {"target": "emitter:condenser-plume", "ease": "step", "keys": [{"t": 0, "v": 0}]}]},
        {"id": "door-cycle", "title": T("Opening the fridge door", "Kühlteiltür öffnen"), "description": T("Running in the kitchen: the door is pulled open to 100 degrees by the replaced bar handle, the lamp comes on, milk is taken from the door balcony, and the door swings shut against its magnetic gasket.", "In der Küche in Betrieb: die Tür wird am ersetzten Stangengriff auf 100 Grad aufgezogen, die Lampe geht an, Milch wird aus dem Türfach genommen, und die Tür schwingt gegen ihre Magnetdichtung zu."), "durationS": 8, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "in-use", "ease": "out"}, {"t": 1.2, "pose": "door-open", "ease": "linear"}, {"t": 6.0, "pose": "door-open", "ease": "in"}, {"t": 7.4, "pose": "in-use", "ease": "linear"}, {"t": 8, "pose": "in-use", "ease": "linear"}],
         "contacts": spans(FEET_C, 0, 8) + spans(WHEEL_C, 0, 8) + spans(NOSE_C, 0, 8),
         "phases": [{"id": "open", "fromS": 0, "toS": 1.2, "title": T("Open", "Öffnen")}, {"id": "take", "fromS": 1.2, "toS": 6.0, "title": T("Take the milk", "Milch nehmen")}, {"id": "close", "fromS": 6.0, "toS": 7.4, "title": T("Close", "Schließen")}, {"id": "sealed", "fromS": 7.4, "toS": 8, "title": T("Sealed", "Dicht")}],
         "events": [{"t": 0.15, "kind": "operating", "title": T("Lamp on", "Lampe an"), "target": "lamp"}, {"t": 3.5, "kind": "load-off", "title": T("Milk taken out", "Milch entnommen"), "target": "bottles"}, {"t": 7.35, "kind": "operating", "title": T("Lamp off", "Lampe aus"), "target": "lamp"}, {"t": 7.4, "kind": "motion", "title": T("Gasket pulls the door shut", "Dichtung zieht die Tür zu")}],
         "tracks": [{"target": "operating:lamp", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 0.15, "v": 1}, {"t": 7.35, "v": 0}]}, {"target": "load:bottles", "ease": "linear", "keys": [{"t": 0, "v": 1}, {"t": 3.4, "v": 1}, {"t": 3.6, "v": 0.83}]}]},
    ]


def structure_model():
    """🏗 Supports and assumptions."""
    M = masses()
    fr = sum(M[b][0] for b in ("cabinet", "condenser", "fridge-door", "freezer-door"))
    tr = M["truck-base"][0] + M["truck-frame"][0]
    return {"gravityMS2": G, "dynamics": "quasi-static", "supports": WHEEL_C, "clinical": False, "assumptions": [
        T(f"Stored: the fridge-freezer ({fr:.1f} kg) stands on its two front feet and two rear rollers on the 3 mm carton, the carton on the depot floor (four corner contacts, friction 0.5); the hand truck ({tr:.1f} kg) stands apart on its two wheels and the front edge of its nose plate.", f"Gelagert: die Kühl-Gefrier-Kombination ({fr:.1f} kg) steht mit zwei Vorderfüßen und zwei hinteren Rollen auf dem 3-mm-Karton, der Karton auf dem Lagerboden (vier Eckkontakte, Reibung 0,5); die Sackkarre ({tr:.1f} kg) steht daneben auf ihren zwei Rädern und der Vorderkante der Schaufel."),
        T("Carried: strapped to the frame and tilted back 38 degrees (fridge 36.4 degrees, resting on the nose plate) so that the common centre of mass lies a little behind the axle; the two pneumatic wheels carry most of the weight and the mover's hands on the grip the rest (compression contacts pushing up, within the 25 kg one person may hold per DGUV 208-033). The truck and the strapped fridge are one rigid body; the carton is off the load path.", "Gefahren: an den Rahmen gegurtet und um 38 Grad zurückgekippt (Gerät 36,4 Grad, auf der Schaufel aufliegend), so dass der gemeinsame Schwerpunkt etwas hinter der Achse liegt; die zwei Luftreifen tragen den größten Teil, die Hände des Trägers am Griff den Rest (nach oben drückende Kontakte, innerhalb der 25 kg, die eine Person nach DGUV 208-033 halten darf). Karre und gegurtetes Gerät sind ein starrer Körper; der Karton liegt außerhalb des Lastpfads."),
        T("In use: on its feet and rollers in a kitchen, 25 kg of food at the centre of both compartments and 6 kg of bottles in the door balconies (authored-estimate after the load cases of EN 62552 storage volumes); door open 100 degrees with the filled balconies is the stability case of EN 60335-2-24 clause 20 (tilting of a free-standing appliance with an open loaded door; magnitude an authored-estimate).", "In Gebrauch: auf Füßen und Rollen in einer Küche, 25 kg Lebensmittel im Schwerpunkt beider Fächer und 6 kg Flaschen in den Türfächern (Schätzwerte nach den Lagervolumina der EN 62552); die um 100 Grad geöffnete Tür mit gefüllten Fächern ist der Standsicherheitsfall der EN 60335-2-24 Abschnitt 20 (Kippen eines freistehenden Geräts mit offener, beladener Tür; Betrag ein Schätzwert)."),
        T("Limitation of the solver: the whole asset is one instantaneously rigid body on springs of equal stiffness, so where the fridge and the parked hand truck stand apart (stored, in use) their reactions are shared linearly over all active contacts instead of each body carrying its own weight; the carried pose (one strapped body on wheels and hands) is exact.", "Grenze des Lösers: das ganze Objekt ist ein momentan starrer Körper auf gleich steifen Federn, wo Gerät und abgestellte Sackkarre getrennt stehen (gelagert, in Gebrauch), teilen sich ihre Reaktionen daher linear über alle aktiven Kontakte, statt dass jeder Körper sein eigenes Gewicht trägt; die gefahrene Pose (ein gegurteter Körper auf Rädern und Händen) ist exakt."),
        T("Members: the foamed cabinet acts as a box of 0.5 mm steel skins (E 210 GPa), the doors as sandwich panels of 0.5 mm skins, the hand truck frame as two 28 x 2 mm steel tubes (EN 10219, S235); masses are modelled volume x effective density of every solid (composite walls 140 kg/m3, doors 115, compressor shell 3770, tubes over their full section) and sum to the stated mass.", "Stäbe: das geschäumte Gehäuse wirkt als Kasten aus 0,5-mm-Stahlblechen (E 210 GPa), die Türen als Sandwichplatten mit 0,5-mm-Blechen, der Rahmen der Sackkarre als zwei Stahlrohre 28 x 2 mm (EN 10219, S235); die Massen sind modelliertes Volumen x wirksame Dichte jedes Körpers (Verbundwände 140 kg/m3, Türen 115, Verdichtergehäuse 3770, Rohre über ihren Vollquerschnitt) und ergeben die angegebene Masse."),
    ]}


# ---------------------------------------------------------------- document

REFS = [
    ("en-62552-2013", "EN 62552:2013 Household refrigerating appliances - Characteristics and test methods (storage volumes, compartment temperatures +4 / -18 degC, energy consumption; used for the compartment layout and load order of magnitude)."),
    ("en-60335-2-24", "EN 60335-2-24:2010+A2 Household and similar electrical appliances - Safety - Part 2-24: Particular requirements for refrigerating appliances (clause 20 stability with doors open; rated input of compressor and lamp)."),
    ("din-18022-2007", "DIN 18022:2007-11 Küche, Bad und WC im Wohnungsbau (placing area of a fridge-freezer 60 x 60 cm beside the kitchen run)."),
    ("iso-10456-2007", "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (PU foam 0.024-0.028 W/mK and 30-40 kg/m3, polymers, steel 50 W/mK, aluminium 160 W/mK, paper and board)."),
    ("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivity of enamel, lacquer, PVC, anodised aluminium and black paint; free convection along plates and tubes)."),
    ("en-10219-2-2019", "EN 10219-2:2019 Cold formed welded steel structural hollow sections - Tolerances, dimensions and sectional properties (circular 28 x 2 mm about 1.63 cm2)."),
    ("en-10346-2015", "EN 10346:2015 Continuously hot-dip coated steel flat products for cold forming (galvanised base plates and brackets)."),
    ("fridge-freezer-catalogue", "Catalogue-typical data of freestanding bottom-freezer fridge-freezers of German mid-range makes sold 2005-2012, 185 x 60 x 65 cm, about 220 l fridge and 90 l freezer, A+ about 300 kWh/a, R600a, rated input 100-120 W, net weight 60-70 kg, wire-on-tube condenser on the back, compressor in the bottom rear recess, three freezer drawers, three glass shelves and a crisper, door balconies, adjustable front feet and rear rollers (authored-estimate from listings of the type, no brand)."),
    ("hand-truck-catalogue", "Catalogue-typical data of tall appliance hand trucks (Kühlschrankkarren): height 1.3-1.6 m, nose plate 400 x 220-250 mm in 6-8 mm steel, pneumatic wheels 260 x 85 mm, frame of 28-30 mm steel tube, 15-25 kg, load 150-250 kg, with strap (authored-estimate from listings of the type, no brand)."),
    ("reuse-depot-practice", "Practice of municipal second-hand department stores and reuse depots in Germany (Gebrauchtwarenkaufhaus, Haushaltsauflösung): white goods are emptied, cleaned, tested and priced, stood unplugged on a piece of carton against leaking defrost water with the cable taped to the back, and moved upright on an appliance hand truck (authored-estimate from the practice described in reuse programmes, no single source)."),
    ("dguv-208-033", "DGUV Information 208-033 (2014) Muskel-Skelett-Belastungen - erkennen und beurteilen (manual handling: holding and pushing forces, loads up to about 25 kg for one adult)."),
]


def balance():
    """⚖ [DEBUG] Common centre of mass of fridge and truck against the axle in the carried pose (x behind the axle negative)."""
    M = masses()

    def rot(p, piv, deg):
        a = math.radians(deg)
        dx, dy = p[0] - piv[0], p[1] - piv[1]
        return (piv[0] + dx * math.cos(a) - dy * math.sin(a), piv[1] + dx * math.sin(a) + dy * math.cos(a))

    tot = mx = 0.0
    for b, (kg, _, c) in M.items():
        if b in ("cabinet", "condenser", "fridge-door", "freezer-door"):
            p = rot((c[0], c[1] + DROP), (XPE, RW + DROP), TILT_F)
        elif b == "truck-frame":
            p = rot((c[0] + D_E, c[1]), (XPE, RW), TILT_T)
        elif b == "truck-base":
            p = (c[0] + D_E, c[1])
        else:
            continue
        tot += kg
        mx += kg * (p[0] - XPE)
    grip = rot((XA0 + TUBE_X + D_E, TOP_Y), (XPE, RW), TILT_T)
    lever = grip[0] - XPE
    return tot, mx / tot, lever, tot * (mx / tot) / lever


def document():
    """📄 The whole asset document."""
    M = masses()
    total = sum(v[0] for v in M.values())
    asset = {
        "id": "used-fridge-freezer",
        "category": "appliances",
        "title": T("Used Fridge-Freezer", "Gebrauchte Kühl-Gefrier-Kombination"),
        "summary": T("A second-hand freestanding bottom-freezer fridge-freezer 0.60 x 0.65 x 1.85 m with yellowed door gaskets, a replaced bar handle, door balconies and the compressor and wire condenser at the back, stored unplugged on a carton in a reuse depot, wheeled upright on a tall hand truck and running again in a kitchen with its door opening.", "Eine gebrauchte freistehende Kühl-Gefrier-Kombination mit unten liegendem Gefrierteil, 0,60 x 0,65 x 1,85 m, mit vergilbten Türdichtungen, ersetztem Stangengriff, Türabstellfächern und Verdichter und Drahtverflüssiger an der Rückseite, ausgesteckt auf einem Karton im Gebrauchtwarenlager abgestellt, aufrecht auf einer hohen Sackkarre gefahren und in einer Küche wieder in Betrieb, mit sich öffnender Tür."),
        "tags": ["used", "household", "second-hand", "kitchen", "appliance", "refrigeration", "fridge-freezer", "enamelled-steel", "pu-foam", "hand-truck", "reuse-depot"],
        "provenance": {"author": "Semio staffage content (used household objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": i, "citation": c} for i, c in REFS], "notes": T(
            "Fridge-freezer of the common German mid-range type, about 2005-2012: cabinet 600 x 530 x 1838 mm of foamed walls (enamelled steel skin 0.5 mm, PU foam, HIPS liner 1.2 mm; sides and back 50 mm, top 45, divider 70) on two adjustable front feet and two rear rollers (18 mm), freezer door 682 mm with three drawers, fridge door 1060 mm with three glass shelves, crisper and three balconies, doors 50 mm on right-hand hinges with magnetic gaskets, compressor in the bottom rear recess, wire-on-tube condenser on the back on 30 mm wall spacers; overall 600 x 650 x 1850 mm. Traces of use: yellowed enamel and gaskets, the original fridge handle replaced by an aluminium bar handle leaving two screw holes and a paler ghost, a dent and scuffs in the right side panel, sticker scraps and a forgotten magnet on the door, a rust spot on the bottom edge of the freezer door, rubber-foot marks of a microwave on the top, the cable insulation repaired with tape. Storage aids: a flattened carton under it; carrying aid: a tall appliance hand truck parked behind it. Masses are modelled volume x effective density of every solid.",
            "Kühl-Gefrier-Kombination der gängigen deutschen Mittelklasse um 2005-2012: Gehäuse 600 x 530 x 1838 mm aus geschäumten Wänden (einbrennlackiertes Stahlblech 0,5 mm, PU-Schaum, HIPS-Innenbehälter 1,2 mm; Seiten und Rückwand 50 mm, Decke 45, Trennwand 70) auf zwei verstellbaren Vorderfüßen und zwei hinteren Rollen (18 mm), Gefriertür 682 mm mit drei Schubladen, Kühltür 1060 mm mit drei Glasböden, Gemüsefach und drei Abstellfächern, 50-mm-Türen an rechten Scharnieren mit Magnetdichtungen, Verdichter in der unteren hinteren Nische, Drahtrohrverflüssiger an der Rückwand auf 30-mm-Wandabstandshaltern; gesamt 600 x 650 x 1850 mm. Gebrauchsspuren: vergilbter Lack und vergilbte Dichtungen, der Originalgriff der Kühltür durch einen Aluminium-Stangengriff ersetzt, mit zwei Schraublöchern und einem helleren Schatten, eine Delle und Schrammen in der rechten Seitenwand, Aufkleberreste und ein vergessener Magnet an der Tür, ein Roststreifen an der Unterkante der Gefriertür, Gummifußabdrücke einer Mikrowelle auf der Decke, die Kabelisolierung mit Isolierband geflickt. Lagerhilfe: ein flachgelegter Karton darunter; Tragehilfe: eine hohe Gerätesackkarre dahinter. Massen aus modelliertem Volumen x wirksamer Dichte jedes Körpers.")},
        "dimensions": {"heightM": Y(1.850), "widthM": 0.660 - (XA0 - RW), "depthM": 0.660, "massKg": total},
        "materials": materials(),
        "rig": rig(),
        "anchors": ANCHORS,
        "representations": representations(),
        "symbols": [],
        "poses": poses(),
        "clips": clips(),
        "thermal": thermal_model(),
        "structure": structure_model(),
        "volumes": build_volumes(),
        "presentation": {"defaultPose": "stored", "defaultClip": "carry", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": r6(asset)}


if __name__ == "__main__":
    OUT.write_text(json.dumps(document(), ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    M = masses()
    print("[DEBUG] masses", {b: round(v[0], 2) for b, v in M.items()}, "total", round(sum(v[0] for v in M.values()), 2))
    print("[DEBUG] balance (kg, com x from axle, grip lever, hand kg)", [round(x, 3) for x in balance()])
