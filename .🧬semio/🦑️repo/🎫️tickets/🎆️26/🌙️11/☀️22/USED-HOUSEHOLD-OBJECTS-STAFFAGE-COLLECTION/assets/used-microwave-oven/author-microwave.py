"""📡 Authors the used 20-litre solo microwave oven (`used-microwave-oven`) from hand-chosen numbers and writes `🔣️.json` next to this file.

Every part is stated once in world metres of the rest pose (the stored pose: the oven standing on a corrugated cardboard pad in the depot) and
converted into the frame of its bone, so the 2D drawings and the 3D solids of one scale come from the same numbers. Each authored scale is its
own part list; 1:200 and 1:500 are unavailable (the whole oven is 2.4 mm and 1.0 mm on paper there).

@see ../../brief.md — the author brief
@see ../used-hinged-wardrobe/author-wardrobe.py — the sibling generator whose helpers this one follows
@see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-tools.md — the tools that check the output
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
LIFT = 0.008
FEATURE = {50: 0.0075, 100: 0.015}
W, H = 0.44, 0.258
ZF, ZB = -0.135, 0.165
DOOR_Z = (-0.170, -0.135)
DOOR_X = (0.0, 0.3185)
DOOR_Y = (0.0115, 0.2565)
WIN = (0.045, 0.265, 0.058, 0.210)
GLASS_Z = (-0.160, -0.156)
CAV_IN = (0.010, 0.325, 0.028, 0.240)
CAV_OUT = (0.0095, 0.3255, 0.0275, 0.2405)
CAV_Z = (-0.135, 0.160)
BAY = (0.333, 0.432, 0.018, 0.250)
TT = (0.1675, 0.0125)
PAD = (-0.020, 0.460, -0.190, 0.205)
FEET = [(0.035, -0.105), (0.405, -0.105), (0.035, 0.135), (0.405, 0.135)]
KNOBS = {"knob-power": 0.195, "knob-timer": 0.120}
KNOB_X = 0.380
KNOB_Z = (-0.187, -0.170)
PLUG = (0.060, 0.112, 0.0, 0.028, 0.171, 0.199)
CORD_Z = 0.185


def T(en, de):
    """🌐 A bilingual text, English first."""
    return {"en": en, "de": de}


def Y(y):
    """⬆️ World height of a housing-local height in the rest (stored) pose, on the cardboard pad."""
    return round(y + LIFT, 6)


def r(v):
    """🔢 Rounds a coordinate to a micrometre."""
    return round(v + 0.0, 6)


BONES = [
    ("ground", None, 0, 0, 0, 0, 0, {"type": "fixed"}, T("Floor reference", "Bodenbezug")),
    ("pad", "ground", PAD[0], 0, 0, 0, PAD[1] - PAD[0], {"type": "prismatic", "axisDeg": 0, "minM": -0.8, "maxM": 0}, T("Corrugated cardboard pad under the oven", "Wellpappenunterlage unter dem Gerät")),
    ("lift", "ground", 0, LIFT, 0, 0, 0, {"type": "prismatic", "axisDeg": 90, "minM": -LIFT, "maxM": 0.6}, T("Lift of the oven off its base", "Anheben des Geräts von seiner Unterlage")),
    ("housing", "lift", 0, 0, 0, 0, W, {"type": "revolute", "minDeg": -10, "maxDeg": 10}, T("Housing, tilt about the lower left edge", "Gehäuse, Neigung um die untere linke Kante")),
    ("base", "housing", 0, 0.0105, 0, 0, W, {"type": "fixed"}, T("Base pan with the transformer", "Bodenwanne mit dem Transformator")),
    ("side-l", "housing", 0.00025, 0.011, 0, 90, 0.247, {"type": "fixed"}, T("Left side of the U-shaped wrapper", "Linke Seite des U-förmigen Mantels")),
    ("side-r", "housing", 0.43975, 0.011, 0, 90, 0.247, {"type": "fixed"}, T("Right side of the wrapper with the vents", "Rechte Seite des Mantels mit den Lüftungsschlitzen")),
    ("top", "housing", 0.0005, 0.25775, 0, 0, 0.439, {"type": "fixed"}, T("Top of the wrapper", "Deckel des Mantels")),
    ("cavity", "housing", 0.010, 0.028, 0, 0, 0.315, {"type": "fixed"}, T("Cooking cavity liner", "Garraum-Auskleidung")),
    ("panel", "housing", 0.380, 0.011, -0.1525, 90, 0.247, {"type": "fixed"}, T("Control panel fascia", "Bedienblende")),
    ("door", "housing", 0.0, 0.0115, ZF, 90, 0.245, {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": -110, "twistMaxDeg": 0}, T("Door, hinged on the left at the front of the housing", "Tür, links an der Gehäusefront angeschlagen")),
    ("turntable", "cavity", TT[0] - 0.010, 0.0, TT[1], 90, 0.027, {"type": "revolute", "continuousTwist": True}, T("Turntable glass plate on its roller ring", "Drehteller aus Glas auf seinem Rollenring")),
    ("knob-power", "housing", KNOB_X, KNOBS["knob-power"], KNOB_Z[1], 0, 0.01, {"type": "revolute", "minDeg": -180, "maxDeg": 0}, T("Power level knob (replacement)", "Leistungsknopf (Ersatz)")),
    ("knob-timer", "housing", KNOB_X, KNOBS["knob-timer"], KNOB_Z[1], 0, 0.01, {"type": "revolute", "minDeg": -300, "maxDeg": 0}, T("Clockwork timer knob, 0 to 30 minutes", "Uhrwerk-Zeitschaltknopf, 0 bis 30 Minuten")),
    ("cord", "housing", 0.112, 0.003, CORD_Z, 0, 0.285, {"type": "fixed"}, T("Mains cord lying on the base behind the oven", "Netzkabel, hinter dem Gerät auf der Unterlage liegend")),
]

FRAME = {}
for bid, parent, x, y, z, ang, length, joint, title in BONES:
    if parent is None:
        FRAME[bid] = ((0.0, 0.0, 0.0), False)
    else:
        (px, py, pz), pvert = FRAME[parent]
        assert not pvert
        FRAME[bid] = ((px + x, py + y, pz + z), ang == 90)


def loc(bone, x, y):
    """📐 Converts a world point (x, y) of the rest pose into the (u, v) plane of `bone`."""
    (ox, oy, _), vert = FRAME[bone]
    return (r(y - oy), r(-(x - ox))) if vert else (r(x - ox), r(y - oy))


def wz(bone, z):
    """↔️ The lateral coordinate w of a world z in the frame of `bone`."""
    return r(z - FRAME[bone][0][2])


def box(sid, bone, b, material, tone, role="body", **extra):
    """📦 A box solid from a world box (x0, x1, y0, y1, z0, z1) of the rest pose."""
    x0, x1, y0, y1, z0, z1 = b
    vert = FRAME[bone][1]
    size = [r(y1 - y0), r(x1 - x0), r(z1 - z0)] if vert else [r(x1 - x0), r(y1 - y0), r(z1 - z0)]
    u, v = loc(bone, (x0 + x1) / 2, (y0 + y1) / 2)
    solid = {"id": sid, "kind": "box", "bone": bone, "material": material, "role": role, "tone": tone, "size": size, "transform": {"translation": [u, v, wz(bone, (z0 + z1) / 2)]}}
    solid.update(extra)
    return solid


def extrude(sid, bone, pts, z0, z1, material, tone, role="body", holes=None, **extra):
    """🧱 An extrusion of a world (x, y) polygon of the rest pose from z0 to z1 (plane xy of the bone)."""
    solid = {"id": sid, "kind": "extrude", "bone": bone, "material": material, "role": role, "tone": tone, "profile": [list(loc(bone, *p)) for p in pts], "plane": "xy", "from": wz(bone, z0), "to": wz(bone, z1)}
    if holes:
        solid["holes"] = [[list(loc(bone, *p)) for p in h] for h in holes]
    solid.update(extra)
    return solid


def capsule(sid, bone, p0, p1, r0, r1, material, tone, role="body", **extra):
    """💊 A capsule between two world points (x, y, z) of the rest pose."""
    (ox, oy, oz), _ = FRAME[bone]
    solid = {"id": sid, "kind": "capsule", "bone": bone, "material": material, "role": role, "tone": tone, "p0": [r(p0[0] - ox), r(p0[1] - oy), r(p0[2] - oz)], "p1": [r(p1[0] - ox), r(p1[1] - oy), r(p1[2] - oz)], "r0": r0, "r1": r1}
    solid.update(extra)
    return solid


def rect_pts(x0, x1, y0, y1):
    """▭ The corners of a world rectangle, counter-clockwise."""
    return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]


def shape(sid, bone, geom, role, z, filled, tone, **extra):
    """✏️ One 2D shape."""
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    s.update(geom)
    s.update(extra)
    return s


def erect(bone, x0, x1, y0, y1):
    """🟥 An elevation rect of a world rectangle in the (u, v) plane of `bone`."""
    (ox, oy, _), vert = FRAME[bone]
    if vert:
        return {"kind": "rect", "x": r(y0 - oy), "y": r(-(x1 - ox)), "w": r(y1 - y0), "h": r(x1 - x0)}
    return {"kind": "rect", "x": r(x0 - ox), "y": r(y0 - oy), "w": r(x1 - x0), "h": r(y1 - y0)}


def epoly(bone, pts, closed=True):
    """🔷 An elevation polygon of world points in the (u, v) plane of `bone`."""
    return {"kind": "poly", "points": [list(loc(bone, *p)) for p in pts], "closed": closed}


def epath(bone, rings):
    """🕳️ An elevation path of world rings in the (u, v) plane of `bone`: the first ring is the outline, the others are holes."""
    d = []
    for ring in rings:
        pts = [loc(bone, *p) for p in ring]
        d.append("M " + " L ".join(f"{u} {v}" for u, v in pts) + " Z")
    return {"kind": "path", "d": " ".join(d)}


def ecircle(bone, xc, yc, rad, n=24):
    """⭕ A world circle in the (u, v) plane of `bone` as a closed polygon."""
    return epoly(bone, [(xc + rad * math.cos(2 * math.pi * k / n), yc + rad * math.sin(2 * math.pi * k / n)) for k in range(n)])


def frect(z0, z1, y0, y1):
    """🟦 A front rect in rest world (z, y)."""
    return {"kind": "rect", "x": r(z0), "y": r(y0), "w": r(z1 - z0), "h": r(y1 - y0)}


def prect(x0, x1, z0, z1):
    """🟩 A plan rect in rest world (x, z)."""
    return {"kind": "rect", "x": r(x0), "y": r(z0), "w": r(x1 - x0), "h": r(z1 - z0)}


def sweep(z0, z1):
    """🌀 The extruded solid placement of a part that turns with its bone (the door and what it carries, the knobs)."""
    return {"solid": {"kind": "extruded", "sweep": "w", "z0": r(z0), "z1": r(z1)}}


def revolved():
    """🎡 The revolved solid placement of a part of the turntable about its vertical axis."""
    return {"solid": {"kind": "revolved", "axisBone": "turntable", "axis": "u", "axisX": 0, "axisZ": 0, "mirror": True}}


def tt_half(profile):
    """🍽️ The bone-local half profile (height, -radius) of a turntable part for its revolved 2D placement, closed through the axis."""
    pts = [[r(a), r(-rad)] for a, rad in profile]
    if profile[0][1] != 0:
        pts.insert(0, [r(profile[0][0]), 0.0])
    if profile[-1][1] != 0:
        pts.append([r(profile[-1][0]), 0.0])
    return {"kind": "poly", "points": pts, "closed": True}


def ez(z):
    """👁️ Paint order of an elevation shape seen from the door side (-z): nearer parts paint later."""
    return round(5.0 - 10.0 * z, 3)


def fz(x):
    """👁️ Paint order of a front shape seen from the right (+x)."""
    return round(2.0 + 10.0 * x, 3)


def pz(y):
    """👁️ Paint order of a plan shape seen from above."""
    return round(2.0 + 10.0 * y, 3)


MATERIALS = [
    {"id": "painted-steel", "title": T("Powder-coated steel sheet 0.5 mm, white, yellowed", "Pulverbeschichtetes Stahlblech 0,5 mm, weiß, vergilbt"), "emissivity": 0.9, "solarAbsorptivity": 0.35, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210e9, "strengthPa": 235e6, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "galv-steel", "title": T("Galvanised steel base pan 1 mm, dull", "Verzinkte Stahl-Bodenwanne 1 mm, matt"), "emissivity": 0.28, "solarAbsorptivity": 0.6, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210e9, "strengthPa": 235e6, "source": "literature", "reference": "incropera-2018"},
    {"id": "cavity-paint", "title": T("Epoxy-painted steel cavity liner 0.5 mm, yellowed and splattered", "Epoxidlackierte Stahl-Garraumauskleidung 0,5 mm, vergilbt und bespritzt"), "emissivity": 0.92, "solarAbsorptivity": 0.5, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210e9, "strengthPa": 235e6, "source": "authored-estimate", "reference": "incropera-2018"},
    {"id": "abs-aged", "title": T("Moulded ABS, white, yellowed (door frame, control panel, timer knob)", "Spritzgegossenes ABS, weiß, vergilbt (Türrahmen, Bedienblende, Zeitknopf)"), "emissivity": 0.9, "solarAbsorptivity": 0.42, "densityKgM3": 1050, "conductivityWmK": 0.17, "specificHeatJKgK": 1400, "elasticModulusPa": 2.3e9, "strengthPa": 40e6, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "abs-black", "title": T("Black ABS (replacement power knob)", "Schwarzes ABS (Ersatz-Leistungsknopf)"), "emissivity": 0.92, "solarAbsorptivity": 0.92, "densityKgM3": 1050, "conductivityWmK": 0.17, "specificHeatJKgK": 1400, "elasticModulusPa": 2.3e9, "strengthPa": 40e6, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "door-glass", "title": T("Tempered door glass 4 mm with the perforated steel screen behind it, tinted", "Vorgespanntes Türglas 4 mm mit dem gelochten Stahlschirm dahinter, getönt"), "emissivity": 0.84, "solarAbsorptivity": 0.55, "densityKgM3": 2500, "conductivityWmK": 1.0, "specificHeatJKgK": 750, "elasticModulusPa": 70e9, "strengthPa": 120e6, "source": "literature", "reference": "en-12150-1"},
    {"id": "borosilicate", "title": T("Borosilicate glass turntable plate, 5 mm", "Drehteller aus Borosilikatglas, 5 mm"), "emissivity": 0.9, "solarAbsorptivity": 0.1, "densityKgM3": 2230, "conductivityWmK": 1.14, "specificHeatJKgK": 830, "elasticModulusPa": 64e9, "strengthPa": 40e6, "source": "literature", "reference": "iso-3585"},
    {"id": "pp-plastic", "title": T("Polypropylene (feet, roller ring, coupler, motor housing)", "Polypropylen (Füße, Rollenring, Mitnehmer, Motorgehäuse)"), "emissivity": 0.9, "solarAbsorptivity": 0.55, "densityKgM3": 905, "conductivityWmK": 0.22, "specificHeatJKgK": 1900, "elasticModulusPa": 1.5e9, "strengthPa": 30e6, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "pvc-cord", "title": T("PVC-sheathed mains cord H05VV-F 3G1.0 and moulded plug", "PVC-ummanteltes Netzkabel H05VV-F 3G1,0 und angespritzter Stecker"), "emissivity": 0.9, "solarAbsorptivity": 0.45, "densityKgM3": 1900, "conductivityWmK": 0.3, "specificHeatJKgK": 1000, "elasticModulusPa": 0.05e9, "strengthPa": 15e6, "source": "authored-estimate", "reference": "en-50525-2-11"},
    {"id": "corrugated-board", "title": T("Double-wall corrugated board 8 mm (flattened banana box)", "Doppelwellpappe 8 mm (flachgelegter Bananenkarton)"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "densityKgM3": 130, "conductivityWmK": 0.065, "specificHeatJKgK": 1340, "elasticModulusPa": 0.3e9, "strengthPa": 3e6, "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "electrical-iron", "title": T("Transformer and magnetron: laminated iron, copper windings, ceramic and steel", "Transformator und Magnetron: Blechpaket, Kupferwicklung, Keramik und Stahl"), "emissivity": 0.6, "solarAbsorptivity": 0.6, "densityKgM3": 7800, "conductivityWmK": 40, "specificHeatJKgK": 450, "elasticModulusPa": 200e9, "strengthPa": 200e6, "source": "authored-estimate", "reference": "iec-60335-2-25"},
]
DENSITY = {m["id"]: m["densityKgM3"] for m in MATERIALS}


def layer(lid, title, kind, material, t):
    """🧶 One layer: resistance t/λ and the heat capacity of the whole thickness."""
    m = next(x for x in MATERIALS if x["id"] == material)
    return {"id": lid, "title": title, "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": round(t / m["conductivityWmK"], 8), "heatCapacityJM2K": round(m["densityKgM3"] * m["specificHeatJKgK"] * t, 1), "source": "literature", "reference": "iso-10456-2007"}


AREA = {"casing": 0.56, "door": 0.095, "window": 0.036, "panel": 0.05, "cavity": 0.88, "turntable": 0.12, "electronics": 0.105, "pad": 0.38}
HEAT = {"casing": 35, "door": 10, "window": 5, "cavity": 60, "turntable": 15, "electronics": 40}


def region(rid, title, kind, material, lay, length, geometry, sky, op=True, **extra):
    """🔥 One thermal region with its single layer and, when it runs, the heat it releases at the rated level of the magnetron."""
    g = {"id": rid, "title": title, "kind": kind, "model": "operating" if op else "passive", "material": material, "areaM2": AREA[rid], "characteristicLengthM": length, "geometry": geometry, "skyViewFactor": sky, "layers": [lay], "heatCapacityJM2K": lay["heatCapacityJM2K"]}
    if op:
        g["operating"] = {"key": "magnetron", "heatW": HEAT[rid]}
    g.update({"source": "authored-estimate", "reference": "iec-60335-2-25"})
    g.update(extra)
    return g


REGIONS = [
    region("casing", T("Wrapper, back, base pan, feet and mains cord", "Mantel, Rückwand, Bodenwanne, Füße und Netzkabel"), "surface", "painted-steel", layer("casing-sheet", T("Steel sheet 0.5 mm", "Stahlblech 0,5 mm"), "metal", "painted-steel", 0.0005), 0.30, "vertical-plate", 0.5, ratedPowerW=1150, ratedPowerBasis="electrical-input"),
    region("door", T("Moulded door frame and hinge pins", "Spritzgegossener Türrahmen und Scharnierzapfen"), "surface", "abs-aged", layer("door-shell", T("ABS wall 2.5 mm", "ABS-Wand 2,5 mm"), "polymer", "abs-aged", 0.0025), 0.245, "vertical-plate", 0.5),
    region("window", T("Door window: tempered glass over the perforated screen", "Türfenster: vorgespanntes Glas über dem Lochschirm"), "glazing", "door-glass", layer("window-pane", T("Glass 4 mm", "Glas 4 mm"), "glass", "door-glass", 0.004), 0.152, "vertical-plate", 0.5),
    region("panel", T("Control panel, knobs and door-release button", "Bedienblende, Knöpfe und Türöffnertaste"), "surface", "abs-aged", layer("panel-shell", T("ABS wall 2.5 mm", "ABS-Wand 2,5 mm"), "polymer", "abs-aged", 0.0025), 0.12, "vertical-plate", 0.5, op=False),
    region("cavity", T("Yellowed cooking cavity liner, both faces", "Vergilbte Garraumauskleidung, beide Seiten"), "surface", "cavity-paint", layer("cavity-sheet", T("Painted steel 0.5 mm", "Lackierter Stahl 0,5 mm"), "metal", "cavity-paint", 0.0005), 0.212, "horizontal-plate-up", 0.0),
    region("turntable", T("Turntable plate, roller ring, coupler and drive motor", "Drehteller, Rollenring, Mitnehmer und Antriebsmotor"), "surface", "borosilicate", layer("plate-glass", T("Borosilicate glass 5 mm", "Borosilikatglas 5 mm"), "glass", "borosilicate", 0.005), 0.245, "horizontal-plate-up", 0.0),
    region("electronics", T("Transformer and magnetron in the technical bay", "Transformator und Magnetron im Technikraum"), "surface", "electrical-iron", layer("iron-core", T("Laminated iron 10 mm (effective)", "Blechpaket 10 mm (wirksam)"), "metal", "electrical-iron", 0.01), 0.09, "vertical-plate", 0.0,
           hidden=T("Inside the closed technical bay behind the control panel: no view of the drawings shows it.", "Im geschlossenen Technikraum hinter der Bedienblende: keine Ansicht der Zeichnungen zeigt ihn.")),
    region("pad", T("Corrugated cardboard pad", "Wellpappenunterlage"), "surface", "corrugated-board", layer("pad-board", T("Double-wall board 8 mm", "Doppelwellpappe 8 mm"), "timber", "corrugated-board", 0.008), 0.38, "horizontal-plate-up", 0.3, op=False),
]
for g in REGIONS:
    if g["id"] == "panel":
        g.update({"ratedPowerW": 0, "ratedPowerBasis": "passive"})
REGION_IDS = [g["id"] for g in REGIONS]

KNOB_PROFILE = {"knob-power": [[0.0, 0.016], [0.015, 0.016], [0.015, 0.0]], "knob-timer": [[0.0, 0.019], [0.004, 0.019], [0.006, 0.0165], [0.017, 0.0155], [0.017, 0.0]]}
KNOB_R = {"knob-power": 0.016, "knob-timer": 0.019}
KNOB_DEPTH = {"knob-power": 0.015, "knob-timer": 0.017}
PLATE = [(0.012, 0.0), (0.012, 0.100), (0.022, 0.1225), (0.027, 0.1225), (0.017, 0.100), (0.017, 0.0)]
PLATE_HUB = [(-0.004, 0.0), (-0.004, 0.015), (0.012, 0.015), (0.012, 0.100), (0.022, 0.1225), (0.027, 0.1225), (0.017, 0.100), (0.017, 0.0)]
COUPLER = [(0.0, 0.0), (0.0, 0.011), (0.012, 0.009), (0.012, 0.0)]
RING = [(r(0.006 + 0.006 * math.sin(2 * math.pi * k / 16)), r(0.085 + 0.006 * math.cos(2 * math.pi * k / 16))) for k in range(16)]
RING = RING + [RING[0]]


def parts(s):
    """🧩 The hand-chosen parts of scale 1:s: body solids, thermal solids, structure solids and the 2D shapes of the three views."""
    f = FEATURE[s]
    body, thermal, struct = [], [], []
    ev, fv, pv = [], [], []
    fine = s == 50

    def th(of, region):
        thermal.append({"id": f"th-{of}", "of": of, "region": region})

    def mem(of, member, part="member", **extra):
        d = {"id": f"m-{of}", "of": of, "member": member, "part": part}
        d.update(extra)
        struct.append(d)

    def bx(sid, bone, b, material, tone, region, views="efp", role="body", **extra):
        """📦 A box part with its body solid, its thermal reference and its three projections."""
        x0, x1, y0, y1, z0, z1 = b
        body.append(box(sid, bone, b, material, tone, role=role, **extra))
        if role != "texture":
            th(sid, region)
        kw = {"region": region, "material": material}
        if "e" in views:
            ev.append(shape(sid, bone, erect(bone, x0, x1, y0, y1), role, ez(z0), True, tone, **kw))
        if "f" in views:
            fv.append(shape(sid, bone, frect(z0, z1, y0, y1), role, fz(x1), True, tone, **kw))
        if "p" in views:
            pv.append(shape(sid, bone, prect(x0, x1, z0, z1), role, pz(y1), True, tone, **kw))

    sheet = {"sheet": True}
    bx("pad", "pad", (PAD[0], PAD[1], 0.0, LIFT, PAD[2], PAD[3]), "corrugated-board", 0.62, "pad", **({} if fine else sheet))
    mem("pad", "pad")

    for k, (xc, zc) in enumerate(FEET):
        fid = f"foot-{k + 1}"
        if fine:
            body.append({"id": fid, "kind": "revolve", "bone": "housing", "material": "pp-plastic", "role": "body", "tone": 0.15, "profile": [[0.0, 0.012], [0.010, 0.010], [0.010, 0.0]], "axis": [0, 1, 0], "origin": [r(xc), 0.0, r(zc)]})
            th(fid, "casing")
        else:
            body.append(box(fid, "housing", (xc - 0.012, xc + 0.012, Y(0), Y(0.010), zc - 0.012, zc + 0.012), "pp-plastic", 0.15))
            th(fid, "casing")
        kw = {"region": "casing", "material": "pp-plastic"}
        if zc < 0:
            ev.append(shape(fid, "housing", erect("housing", xc - 0.012, xc + 0.012, Y(0), Y(0.010)), "body", ez(zc - 0.012), True, 0.15, **kw))
        if xc > 0.2:
            fv.append(shape(fid, "housing", frect(zc - 0.012, zc + 0.012, Y(0), Y(0.010)), "body", fz(xc + 0.012), True, 0.15, **kw))

    if fine:
        bx("base", "base", (0.0, W, Y(0.010), Y(0.011), ZF, ZB), "galv-steel", 0.5, "casing", **sheet)
        mem("base", "base")
        bx("side-l", "side-l", (0.0, 0.0005, Y(0.011), Y(H), ZF, ZB), "painted-steel", 0.88, "casing", **sheet)
        mem("side-l", "side-l")
        bx("side-r", "side-r", (W - 0.0005, W, Y(0.011), Y(H), ZF, ZB), "painted-steel", 0.88, "casing", **sheet)
        mem("side-r", "side-r")
        bx("top", "top", (0.0005, W - 0.0005, Y(H - 0.0005), Y(H), ZF, ZB), "painted-steel", 0.9, "casing", **sheet)
        mem("top", "top")
        bx("back", "housing", (0.0005, W - 0.0005, Y(0.011), Y(H - 0.0005), ZB - 0.0005, ZB), "painted-steel", 0.8, "casing", **sheet)
        mem("back", "housing")
        ox0, ox1, oy0, oy1 = CAV_OUT
        ix0, ix1, iy0, iy1 = CAV_IN
        outer = rect_pts(ox0, ox1, Y(oy0), Y(oy1))
        hole = rect_pts(ix0, ix1, Y(iy0), Y(iy1))
        body.append(extrude("liner", "cavity", outer, CAV_Z[0], CAV_Z[1], "cavity-paint", 0.72, holes=[hole]))
        th("liner", "cavity")
        mem("liner", "cavity")
        ev.append(shape("liner", "cavity", epath("cavity", [outer, hole]), "body", ez(CAV_Z[0]), True, 0.72, region="cavity", material="cavity-paint"))
        fv.append(shape("liner", "cavity", frect(CAV_Z[0], CAV_Z[1], Y(oy0), Y(oy1)), "body", fz(ox1), True, 0.72, region="cavity", material="cavity-paint"))
        pv.append(shape("liner", "cavity", prect(ox0, ox1, CAV_Z[0], CAV_Z[1]), "body", pz(oy1), True, 0.72, region="cavity", material="cavity-paint"))
        bx("liner-back", "cavity", (ox0, ox1, Y(oy0), Y(oy1), CAV_Z[1], CAV_Z[1] + 0.0005), "cavity-paint", 0.66, "cavity", **sheet)
        mem("liner-back", "cavity")
        bx("motor", "cavity", (0.155, 0.180, Y(0.011), Y(0.0275), 0.0, 0.025), "pp-plastic", 0.3, "turntable", views="")
        mem("motor", "cavity")
    else:
        outer = rect_pts(0.0, W, Y(0.010), Y(H))
        ix0, ix1, iy0, iy1 = CAV_IN
        hole = rect_pts(ix0, ix1, Y(iy0), Y(iy1))
        shell_hole = rect_pts(ix0 - 0.004, ix1 + 0.004, Y(iy0 - 0.004), Y(iy1 + 0.004))
        bay = rect_pts(BAY[0], BAY[1], Y(BAY[2]), Y(BAY[3]))
        body.append(extrude("shell", "housing", outer, ZF, 0.150, "painted-steel", 0.86, holes=[shell_hole, bay]))
        th("shell", "casing")
        mem("shell", "housing", members=["base", "side-l", "side-r", "top", "cavity"])
        ev.append(shape("shell", "housing", erect("housing", 0.0, W, Y(0.010), Y(H)), "body", ez(ZF), True, 0.86, region="casing", material="painted-steel"))
        fv.append(shape("shell", "housing", frect(ZF, ZB, Y(0.010), Y(H)), "body", fz(W), True, 0.86, region="casing", material="painted-steel"))
        pv.append(shape("shell", "housing", prect(0.0, W, ZF, ZB), "body", pz(H), True, 0.88, region="casing", material="painted-steel"))
        body.append(box("back", "housing", (0.0, W, Y(0.010), Y(H), 0.150, ZB), "painted-steel", 0.8))
        th("back", "casing")
        mem("back", "housing")
        thermal.append(extrude("th-cavity-lining", "cavity", hole, CAV_Z[0], 0.150, "cavity-paint", 0.72, holes=[rect_pts(ix0 + 0.0005, ix1 - 0.0005, Y(iy0 + 0.0005), Y(iy1 - 0.0005))], region="cavity"))
        thermal.append(box("th-cavity-back", "cavity", (ix0, ix1, Y(iy0), Y(iy1), 0.1495, 0.150), "cavity-paint", 0.66, region="cavity", sheet=True))

    tt_list = [("plate", PLATE if fine else PLATE_HUB, 0.85)]
    if fine:
        tt_list += [("coupler", COUPLER, 0.4)]
    for sid, prof, tone in tt_list:
        body.append({"id": sid, "kind": "revolve", "bone": "turntable", "material": "borosilicate" if sid == "plate" else "pp-plastic", "role": "body", "tone": tone, "profile": [[r(a), r(rad)] for a, rad in prof], "axis": [1, 0, 0], "origin": [0.0, 0.0, 0.0]})
        th(sid, "turntable")
        reach = max(rad for _, rad in prof)
        mat = "borosilicate" if sid == "plate" else "pp-plastic"
        for views, zz in ((ev, ez(TT[1] - reach) - 3.0), (fv, fz(TT[0] + reach) - 3.0), (pv, pz(0.028 + max(a for a, _ in prof)) - 1.0)):
            views.append(shape(sid, "turntable", tt_half(prof), "body", zz, True, tone, region="turntable", material=mat, **revolved()))
    mem("plate", "turntable")
    if fine:
        body.append({"id": "roller-ring", "kind": "revolve", "bone": "turntable", "material": "pp-plastic", "role": "body", "tone": 0.35, "profile": [list(p) for p in RING], "axis": [1, 0, 0], "origin": [0.0, 0.0, 0.0], "slender": True})
        th("roller-ring", "turntable")
        mem("coupler", "turntable", part="joint")

    dpts = rect_pts(DOOR_X[0], DOOR_X[1], Y(DOOR_Y[0]), Y(DOOR_Y[1]))
    wpts = rect_pts(WIN[0], WIN[1], Y(WIN[2]), Y(WIN[3]))
    body.append(extrude("door-frame", "door", dpts, *DOOR_Z, "abs-aged", 0.92, holes=[wpts]))
    th("door-frame", "door")
    mem("door-frame", "door")
    for views in (ev, fv, pv):
        views.append(shape("door-frame", "door", epath("door", [dpts, wpts]), "body", 7.0, True, 0.92, region="door", material="abs-aged", **sweep(*DOOR_Z)))
    body.append(box("glass", "door", (WIN[0], WIN[1], Y(WIN[2]), Y(WIN[3]), *GLASS_Z), "door-glass", 0.3, sheet=True))
    th("glass", "window")
    for views in (ev, fv, pv):
        views.append(shape("glass", "door", erect("door", WIN[0], WIN[1], Y(WIN[2]), Y(WIN[3])), "glazing", 6.9, True, 0.3, region="window", material="door-glass", **sweep(*GLASS_Z)))
    if fine:
        for k, (y0, y1) in enumerate(((0.0115, 0.0265), (0.2415, 0.2565))):
            hid = f"hinge-{k + 1}"
            body.append(box(hid, "door", (0.001, 0.009, Y(y0), Y(y1), -0.141, -0.129), "galv-steel", 0.4, role="detail"))
            th(hid, "door")
            mem(hid, "door", part="joint")
            for views in (ev, fv, pv):
                views.append(shape(hid, "door", erect("door", 0.001, 0.009, Y(y0), Y(y1)), "detail", 6.8, True, 0.4, region="door", material="galv-steel", **sweep(-0.141, -0.129)))

    bx("fascia", "panel", (0.320, W, Y(0.011), Y(H), *DOOR_Z), "abs-aged", 0.9, "panel")
    mem("fascia", "panel")
    if fine:
        bx("button", "panel", (0.350, 0.410, Y(0.040), Y(0.062), -0.178, DOOR_Z[0]), "abs-aged", 0.75, "panel")
    for kid, yc in KNOBS.items():
        prof = KNOB_PROFILE[kid] if fine else [[0.0, KNOB_R[kid]], [KNOB_DEPTH[kid], KNOB_R[kid]], [KNOB_DEPTH[kid], 0.0]]
        mat = "abs-black" if kid == "knob-power" else "abs-aged"
        tone = 0.12 if kid == "knob-power" else 0.85
        body.append({"id": kid, "kind": "revolve", "bone": kid, "material": mat, "role": "body", "tone": tone, "profile": prof, "axis": [0, 0, -1], "origin": [0.0, 0.0, 0.0]})
        th(kid, "panel")
        rk = KNOB_R[kid]
        z0 = KNOB_Z[1] - KNOB_DEPTH[kid]
        circ = ecircle(kid, KNOB_X, Y(yc), rk)
        for views, zz in ((ev, 8.0), (fv, fz(KNOB_X + rk)), (pv, pz(yc + rk))):
            views.append(shape(kid, kid, circ, "body", zz, True, tone, region="panel", material=mat, **sweep(z0, KNOB_Z[1])))

    px0, px1, py0, py1, pz0, pz1 = PLUG
    bx("plug", "cord", (px0, px1, Y(py0), Y(py1), pz0, pz1), "pvc-cord", 0.88, "casing")
    if fine:
        body.append(capsule("cord-drop", "cord", (0.400, Y(0.028), 0.1665), (0.397, Y(0.0055), CORD_Z), 0.0045, 0.0035, "pvc-cord", 0.88))
        th("cord-drop", "casing")
        body.append(capsule("cord-run", "cord", (0.395, Y(0.003), CORD_Z), (px1, Y(0.003), CORD_Z), 0.003, 0.003, "pvc-cord", 0.88, slender=True))
        th("cord-run", "casing")
        mem("cord-run", "cord")
        kw = {"region": "casing", "material": "pvc-cord"}
        ev.append(shape("cord-run", "cord", erect("cord", px1, 0.395, Y(0.0), Y(0.006)), "body", ez(CORD_Z - 0.003), True, 0.88, **kw))
        pv.append(shape("cord-run", "cord", prect(px1, 0.395, CORD_Z - 0.003, CORD_Z + 0.003), "body", pz(0.006), True, 0.88, **kw))
        fv.append(shape("cord-drop", "cord", {"kind": "capsule", "x0": 0.1665, "y0": Y(0.028), "x1": CORD_Z, "y1": Y(0.0055), "r0": 0.0045, "r1": 0.0035}, "body", fz(0.4045), True, 0.88, **kw))
        pv.append(shape("cord-drop", "cord", prect(0.3925, 0.4045, 0.162, CORD_Z + 0.0035), "body", pz(0.0325), True, 0.88, **kw))
        body.append(capsule("cord-tape", "cord", (0.250, Y(0.003), CORD_Z), (0.272, Y(0.003), CORD_Z), 0.0037, 0.0037, "pvc-cord", 0.2, role="texture", texture=True))
        pv.append(shape("cord-tape", "cord", prect(0.250, 0.272, CORD_Z - 0.0037, CORD_Z + 0.0037), "texture", pz(0.007), True, 0.2))
    else:
        body.append(capsule("cord-run", "cord", (0.400, Y(0.028), 0.1665), (px1, Y(0.003), CORD_Z), 0.003, 0.003, "pvc-cord", 0.88, slender=True))
        th("cord-run", "casing")
        mem("cord-run", "cord")
        kw = {"region": "casing", "material": "pvc-cord"}
        ev.append(shape("cord-run", "cord", epoly("cord", [(px1, Y(0.0)), (0.400, Y(0.025)), (0.400, Y(0.031)), (px1, Y(0.006))]), "body", ez(CORD_Z - 0.003), True, 0.88, **kw))
        pv.append(shape("cord-run", "cord", {"kind": "poly", "points": [[px1, CORD_Z - 0.003], [0.400, 0.1635], [0.400, 0.1695], [px1, CORD_Z + 0.003]], "closed": True}, "body", pz(0.031), True, 0.88, **kw))
        fv.append(shape("cord-run", "cord", {"kind": "poly", "points": [[0.1635, Y(0.028)], [0.1695, Y(0.028)], [CORD_Z + 0.003, Y(0.003)], [CORD_Z - 0.003, Y(0.003)]], "closed": True}, "body", fz(0.403), True, 0.88, **kw))

    thermal.append(box("th-exhaust-vents", "side-r", (W, W + 0.0005, Y(0.150), Y(0.230), -0.060, 0.060), "painted-steel", 0.45, region="casing", tags=["exhaust"], sheet=True))
    if not fine:
        ix0, ix1, iy0, iy1 = CAV_IN
        ev.append(shape("cavity-opening", "housing", erect("housing", ix0, ix1, Y(iy0), Y(iy1)), "body", ez(ZF) + 0.1, True, 0.72, region="cavity", material="cavity-paint"))
    thermal.append(box("th-transformer", "base", (0.335, 0.425, Y(0.011 if fine else 0.020), Y(0.095), 0.020, 0.120), "electrical-iron", 0.3, region="electronics"))
    thermal.append(box("th-magnetron", "housing", (0.3255, 0.420, Y(0.120), Y(0.215), -0.050, 0.050), "electrical-iron", 0.3, region="electronics"))

    if fine:
        tex = []
        for k, (x0, y0, x1, y1) in enumerate(((0.070, 0.090, 0.130, 0.120), (0.150, 0.170, 0.205, 0.150), (0.180, 0.080, 0.240, 0.095))):
            L = math.hypot(x1 - x0, y1 - y0)
            nx, ny = -(y1 - y0) / L * 0.001, (x1 - x0) / L * 0.001
            pts = [(x0 + nx, Y(y0) + ny), (x1 + nx, Y(y1) + ny), (x1 - nx, Y(y1) - ny), (x0 - nx, Y(y0) - ny)]
            ev.append(shape(f"glass-scratch-{k + 1}", "door", epoly("door", pts), "texture", 7.2, True, 0.6, **sweep(*GLASS_Z)))
        for k, (x, y, rx) in enumerate(((0.020, 0.034, 0.005), (0.031, 0.025, 0.0035), (0.290, 0.035, 0.004), (0.300, 0.230, 0.0035))):
            ev.append(shape(f"grease-spot-{k + 1}", "door", ecircle("door", x, Y(y), rx, 12), "texture", 7.3, True, 0.55, **sweep(-0.170, -0.165)))
        for k in range(7):
            a = math.radians(90 - k * 42)
            x, y = KNOB_X + 0.026 * math.cos(a), KNOBS["knob-timer"] + 0.026 * math.sin(a)
            ev.append(shape(f"timer-scale-{k + 1}", "panel", erect("panel", x - 0.0012, x + 0.0012, Y(y) - 0.0012, Y(y) + 0.0012), "texture", 7.5, True, 0.3 if k < 4 else 0.7))
        for kid, yc in KNOBS.items():
            ev.append(shape(f"{kid}-pointer", kid, erect(kid, KNOB_X - 0.0012, KNOB_X + 0.0012, Y(yc) + 0.004, Y(yc) + KNOB_R[kid] - 0.002), "texture", 8.1, True, 0.95 if kid == "knob-power" else 0.25, **sweep(-0.187, -0.183)))
        for k in range(5):
            xs = 0.345 + 0.014 * k
            ev.append(shape(f"power-mark-{k + 1}", "panel", erect("panel", xs, xs + 0.006, Y(0.226), Y(0.2285)), "texture", 7.5, True, 0.35))
        ev.append(shape("button-yellowing", "panel", erect("panel", 0.352, 0.408, Y(0.042), Y(0.050)), "texture", 7.6, True, 0.6))
        for k in range(6):
            zs = -0.06 + 0.022 * k
            fv.append(shape(f"vent-slot-{k + 1}", "side-r", frect(zs, zs + 0.012, Y(0.150), Y(0.230)), "texture", fz(W) + 0.1, True, 0.45))
        fv.append(shape("side-dent", "side-r", {"kind": "ellipse", "cx": 0.110, "cy": Y(0.080), "rx": 0.018, "ry": 0.012}, "texture", fz(W) + 0.1, False, 0.4))
        body.append(box("price-sticker", "top", (0.250, 0.320, Y(H), Y(H) + 0.0003, -0.090, -0.040), "abs-aged", 0.95, role="texture", texture=True))
        pv.append(shape("price-sticker", "top", prect(0.250, 0.320, -0.090, -0.040), "texture", pz(H) + 0.1, True, 0.97))
        pv.append(shape("sticker-line", "top", {"kind": "poly", "points": [[0.258, -0.078], [0.310, -0.078]], "closed": False}, "texture", pz(H) + 0.2, False, 0.2))
        pv.append(shape("top-dent", "top", {"kind": "ellipse", "cx": 0.110, "cy": 0.070, "rx": 0.025, "ry": 0.015}, "texture", pz(H) + 0.1, False, 0.4))
        for k, (x0, z0, x1, z1) in enumerate(((0.05, 0.10, 0.18, 0.12), (0.30, 0.02, 0.41, 0.0))):
            pv.append(shape(f"top-scratch-{k + 1}", "top", {"kind": "poly", "points": [[x0, z0], [x1, z1]], "closed": False}, "texture", pz(H) + 0.1, False, 0.3))

    sup = {50: (0.40, 0.006, 0.30), 100: (0.40, 0.015, 0.30)}[s]
    ax, az = 0.24, 0.0075
    struct.append({"id": "support-pad-floor", "kind": "box", "bone": "pad", "material": "corrugated-board", "role": "ground", "tone": 0.2, "size": [sup[0], sup[1], sup[2]], "transform": {"translation": [r(ax), r(sup[1] / 2), r(az)]}, "member": "pad", "part": "support", "anchor": "pad-floor"})
    mr = max(f, 0.02)
    struct.append({"id": "mass-housing", "kind": "ellipsoid", "bone": "housing", "material": "electrical-iron", "role": "detail", "tone": 0.3, "radii": [mr, mr, mr], "transform": {"translation": [COM_HOUSING[0], COM_HOUSING[1], 0.02]}, "member": "housing", "part": "mass"})
    return body, thermal, struct, ev, fv, pv


def poly_area_centroid(p):
    """📏 Signed area and centroid of a closed polygon."""
    a = cx = cy = 0.0
    for i in range(len(p)):
        x0, y0 = p[i]
        x1, y1 = p[(i + 1) % len(p)]
        c = x0 * y1 - x1 * y0
        a += c
        cx += (x0 + x1) * c
        cy += (y0 + y1) * c
    a /= 2
    return a, (cx / (6 * a), cy / (6 * a)) if a else (0, 0)


def solid_volume(sol):
    """⚖️ Volume of a solid in cubic metres (boxes, extrusions with holes, capsules, revolves by Pappus)."""
    k = sol["kind"]
    if k == "box":
        a, b, c = sol["size"]
        return a * b * c
    if k == "extrude":
        area = abs(poly_area_centroid(sol["profile"])[0]) - sum(abs(poly_area_centroid(h)[0]) for h in sol.get("holes", []))
        return area * abs(sol["to"] - sol["from"])
    if k == "capsule":
        L = math.dist(sol["p0"], sol["p1"])
        rm = (sol["r0"] + sol["r1"]) / 2
        return math.pi * rm ** 2 * L + 2 / 3 * math.pi * (sol["r0"] ** 3 + sol["r1"] ** 3)
    if k == "revolve":
        p = [tuple(q) for q in sol["profile"]]
        if p[0] == p[-1]:
            loop = p[:-1]
        else:
            loop = p + ([(p[-1][0], 0.0)] if p[-1][1] else []) + ([(p[0][0], 0.0)] if p[0][1] else [])
        a, (_, rc) = poly_area_centroid(loop)
        return abs(a) * 2 * math.pi * rc
    if k == "ellipsoid":
        a, b, c = sol["radii"]
        return 4 / 3 * math.pi * a * b * c
    raise ValueError(k)


def solid_centre(sol):
    """🎯 The local (u, v) centre of a solid."""
    k = sol["kind"]
    if k == "box":
        t = sol["transform"]["translation"]
        return (t[0], t[1])
    if k == "extrude":
        p = sol["profile"]
        return (sum(q[0] for q in p) / len(p), sum(q[1] for q in p) / len(p))
    if k == "capsule":
        return ((sol["p0"][0] + sol["p1"][0]) / 2, (sol["p0"][1] + sol["p1"][1]) / 2)
    o, ax = sol["origin"], sol["axis"]
    mid = sum(q[0] for q in sol["profile"]) / len(sol["profile"])
    return (o[0] + ax[0] * mid, o[1] + ax[1] * mid)


SHELL_T = 0.0025
SHELL = {"door-frame": 0.1547, "fascia": 0.0855}
COMPONENTS = [
    ("base", 2.80, 0.380, 0.053, T("HV transformer", "Hochspannungstransformator")),
    ("base", 0.25, 0.360, 0.030, T("HV capacitor, diode and fuse", "Hochspannungskondensator, Diode und Sicherung")),
    ("housing", 0.80, 0.373, 0.168, T("Magnetron with cooling fins", "Magnetron mit Kühlrippen")),
    ("housing", 0.25, 0.400, 0.225, T("Cooling fan and its motor", "Lüfter mit Motor")),
    ("panel", 0.30, 0.380, 0.150, T("Clockwork timer, power switch, interlock switches and wiring", "Uhrwerk-Zeitschalter, Leistungsschalter, Türkontakte und Verdrahtung")),
    ("door", 0.45, 0.160, 0.134, T("Steel door plate with the choke seal and the perforated screen", "Stahl-Türblech mit Drosseldichtung und Lochschirm")),
    ("cavity", 0.10, 0.168, 0.019, T("Turntable synchronous motor", "Drehteller-Synchronmotor")),
]

COM_HOUSING = (0.0, 0.0)
BODY50 = parts(50)[0]
MASS = {}


def add_mass(bone, kg, u, v):
    """➕ Adds a point mass at local (u, v) to the bone total."""
    m, su, sv = MASS.get(bone, (0, 0, 0))
    MASS[bone] = (m + kg, su + kg * u, sv + kg * v)


for sol in BODY50:
    if sol.get("texture") or sol["id"] == "motor":
        continue
    vol = SHELL[sol["id"]] * SHELL_T if sol["id"] in SHELL else solid_volume(sol)
    u, v = solid_centre(sol)
    add_mass(sol["bone"], vol * DENSITY[sol["material"]], u, v)
for bone, kg, x, y, _ in COMPONENTS:
    add_mass(bone, kg, *loc(bone, x, Y(y)))
TOTAL_KG = round(sum(m for m, _, _ in MASS.values()), 2)
OVEN_KG = round(TOTAL_KG - MASS["pad"][0], 2)
hm = [(m, su / m, sv / m, b) for b, (m, su, sv) in MASS.items() if b not in ("pad",)]
gx = sum(m * (FRAME[b][0][0] + (cu if not FRAME[b][1] else -cv)) for m, cu, cv, b in hm) / sum(m for m, *_ in hm)
gy = sum(m * (FRAME[b][0][1] + (cv if not FRAME[b][1] else cu)) for m, cu, cv, b in hm) / sum(m for m, *_ in hm)
COM_HOUSING = (r(gx), r(gy - LIFT))


def hollow_rect(b, h, t):
    """🔲 Area and second moment of a thin-walled closed rectangle b x h with wall t (bending about the axis parallel to b)."""
    a = b * h - (b - 2 * t) * (h - 2 * t)
    i = (b * h ** 3 - (b - 2 * t) * (h - 2 * t) ** 3) / 12
    return a, i


def sec(area, inertia, fibre, material, en, de):
    """📐 A member section."""
    return {"areaM2": float(f"{area:.4e}"), "secondMomentM4": float(f"{inertia:.4e}"), "extremeFibreM": r(fibre), "material": material, "title": T(en, de)}


SECTIONS = {
    "base": sec(0.30 * 0.001, 0.30 * 0.001 ** 3 / 12, 0.0005, "galv-steel", "Base pan 300 x 1 mm", "Bodenwanne 300 x 1 mm"),
    "side-l": sec(0.30 * 0.0005, 0.30 * 0.0005 ** 3 / 12, 0.00025, "painted-steel", "Wrapper side 300 x 0.5 mm", "Mantelseite 300 x 0,5 mm"),
    "side-r": sec(0.30 * 0.0005, 0.30 * 0.0005 ** 3 / 12, 0.00025, "painted-steel", "Wrapper side 300 x 0.5 mm", "Mantelseite 300 x 0,5 mm"),
    "top": sec(0.30 * 0.0005, 0.30 * 0.0005 ** 3 / 12, 0.00025, "painted-steel", "Wrapper top 300 x 0.5 mm", "Manteldeckel 300 x 0,5 mm"),
    "cavity": sec(*hollow_rect(0.2955, 0.213, 0.0005), 0.1065, "cavity-paint", "Cavity liner tube 296 x 213 x 0.5 mm", "Garraumrohr 296 x 213 x 0,5 mm"),
    "housing": sec(*hollow_rect(0.30, 0.247, 0.0005), 0.1235, "painted-steel", "Housing box 300 x 247 x 0.5 mm (wrapper with base)", "Gehäusekasten 300 x 247 x 0,5 mm (Mantel mit Boden)"),
    "door": sec(*hollow_rect(0.035, 0.3185, SHELL_T), 0.159, "abs-aged", "Door moulding 318 x 35 mm, wall 2.5 mm", "Türformteil 318 x 35 mm, Wand 2,5 mm"),
    "panel": sec(*hollow_rect(0.035, 0.12, SHELL_T), 0.06, "abs-aged", "Panel moulding 120 x 35 mm, wall 2.5 mm", "Blendenformteil 120 x 35 mm, Wand 2,5 mm"),
    "turntable": sec(0.245 * 0.005, 0.245 * 0.005 ** 3 / 12, 0.0025, "borosilicate", "Glass plate 245 x 5 mm", "Glasteller 245 x 5 mm"),
    "cord": sec(math.pi * 0.003 ** 2, math.pi * 0.006 ** 4 / 64, 0.003, "pvc-cord", "Cord 6 mm", "Kabel 6 mm"),
    "pad": sec(0.395 * 0.008, 0.395 * 0.008 ** 3 / 12, 0.004, "corrugated-board", "Board 395 x 8 mm", "Pappe 395 x 8 mm"),
}


def bone_json(bid, parent, x, y, z, ang, length, joint, title):
    """🦴 One bone with its mass from the 1:50 solids and the component table and its member section."""
    b = {"id": bid, "parent": parent, "title": title, "x": r(x), "y": r(y)}
    if z:
        b["z"] = r(z)
    b.update({"angleDeg": ang, "lengthM": length, "joint": joint})
    if bid in MASS:
        m, su, sv = MASS[bid]
        b["mass"] = {"kg": round(m, 3), "comX": round(su / m, 4), "comY": round(sv / m, 4), "distribution": "point", "source": "authored-estimate", "reference": "iec-60705"}
    if bid in SECTIONS:
        b["section"] = SECTIONS[bid]
    return b


ANCHORS = [
    {"id": "foot-left", "bone": "housing", "x": 0.035, "y": 0.0, "z": 0.015, "kind": "contact", "title": T("Left pair of feet on the base", "Linkes Fußpaar auf der Unterlage"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.024, "footprintWidthM": 0.264, "frictionCoefficient": 0.5},
    {"id": "foot-right", "bone": "housing", "x": 0.405, "y": 0.0, "z": 0.015, "kind": "contact", "title": T("Right pair of feet on the base", "Rechtes Fußpaar auf der Unterlage"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.024, "footprintWidthM": 0.264, "frictionCoefficient": 0.5},
    {"id": "grip-left", "bone": "housing", "x": 0.0, "y": 0.010, "z": 0.015, "kind": "support", "title": T("Left hand under the left bottom edge", "Linke Hand unter der linken Unterkante"), "support": "pin", "footprintLengthM": 0.08, "footprintWidthM": 0.10},
    {"id": "grip-right", "bone": "housing", "x": W, "y": 0.010, "z": 0.015, "kind": "support", "title": T("Right hand under the right bottom edge", "Rechte Hand unter der rechten Unterkante"), "support": "pin", "footprintLengthM": 0.08, "footprintWidthM": 0.10},
    {"id": "pad-floor", "bone": "pad", "x": 0.24, "y": 0.0, "z": 0.0075, "kind": "contact", "title": T("Cardboard pad on the shelf or floor", "Pappunterlage auf Regal oder Boden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.48, "footprintWidthM": 0.395, "frictionCoefficient": 0.5},
    {"id": "exhaust", "bone": "housing", "x": W, "y": 0.19, "z": 0.05, "kind": "emitter", "title": T("Warm cooling air out of the right side vents", "Warme Kühlluft aus den Schlitzen der rechten Seite")},
]

def binding(running, door_open=False, pad=None, home=False):
    """🌡 The thermal binding of a pose: every region, the magnetron level and the exhaust."""
    regions = {g: {"exposedFraction": 1.0, "sunlitFraction": 0.0} for g in REGION_IDS}
    regions["cavity"]["exposedFraction"] = 1.0 if door_open else 0.3
    regions["turntable"]["exposedFraction"] = 1.0 if door_open else 0.3
    regions["casing"]["contact"] = {"fraction": 0.01, "tempC": 21 if home else 18}
    regions["pad"] = pad or {"exposedFraction": 0.45, "sunlitFraction": 0.0, "contact": {"fraction": 0.55, "tempC": 18}}
    return {"operating": {"magnetron": 1 if running else 0}, "regions": regions, "emitters": {"exhaust": 1 if running else 0}}


ASIDE = {"pad": {"dx": -0.60}}
HOME_PAD = {"exposedFraction": 0.5, "sunlitFraction": 0.0, "contact": {"fraction": 0.5, "tempC": 21}}
DOWN = {"lift": {"dy": -LIFT}}
G = 9.80665
FOOD = {"id": "food-on-plate", "kind": "equipment", "title": T("A plate of food, 0.6 kg, on the turntable", "Ein Teller mit Essen, 0,6 kg, auf dem Drehteller"), "bone": "turntable", "x": 0.027, "y": 0.0, "fxN": 0, "fyN": round(-0.6 * G, 2), "source": "authored-estimate", "reference": "iec-60705"}
DOOR_HAND = {"id": "hand-on-open-door", "kind": "live", "title": T("A hand resting on the open door edge, 20 N", "Eine Hand auf der offenen Türkante, 20 N"), "bone": "door", "x": 0.12, "y": 0.0, "fxN": 0, "fyN": -20.0, "source": "authored-estimate", "reference": "iec-60335-2-25"}

POSES = [
    {"id": "stored", "title": T("Stored at the reuse depot", "Gelagert im Gebrauchtwarenlager"), "description": T("Emptied, wiped and tested, the oven stands on a flattened banana box on the depot shelf with its mains cord laid out behind it; a price sticker is on the top.", "Geleert, ausgewischt und geprüft steht das Gerät auf einem flachgelegten Bananenkarton im Lagerregal, das Netzkabel liegt dahinter; oben klebt ein Preisaufkleber."), "bones": {}, "ground": "authored", "contacts": ["foot-left", "foot-right", "pad-floor"], "thermal": binding(False), "loads": []},
    {"id": "carried", "title": T("Carried by one person", "Von einer Person getragen"), "description": T("Lifted 0.30 m off the cardboard with both hands under the bottom edges of the sides and tilted 4 degrees towards the heavier transformer side; the cardboard stays on the shelf.", "Mit beiden Händen unter den Unterkanten der Seiten 0,30 m von der Pappe gehoben und um 4 Grad zur schwereren Transformatorseite geneigt; die Pappe bleibt im Regal."), "bones": {"lift": {"dy": 0.30}, "housing": {"angleDeg": 4}}, "ground": "authored", "contacts": ["grip-left", "grip-right", "pad-floor"], "thermal": binding(False), "loads": []},
    {"id": "in-use", "title": T("Running on a kitchen worktop", "In Betrieb auf einer Küchenarbeitsplatte"), "description": T("Back on a kitchen worktop at full power with the door closed: the turntable turns a plate of food, the fan blows the magnetron heat out of the right side vents; the cardboard lies folded beside it.", "Wieder auf einer Küchenarbeitsplatte bei voller Leistung mit geschlossener Tür: der Drehteller dreht einen Teller mit Essen, der Lüfter bläst die Wärme des Magnetrons aus den Schlitzen der rechten Seite; die Pappe liegt gefaltet daneben."), "bones": {**DOWN, **ASIDE}, "ground": "authored", "contacts": ["foot-left", "foot-right", "pad-floor"], "thermal": binding(True, pad=HOME_PAD, home=True), "loads": [FOOD]},
    {"id": "door-open", "title": T("Door open in the kitchen", "Tür offen in der Küche"), "description": T("Switched off by the door interlock, the door swung open 95 degrees on its left hinge pins: the yellowed cavity and the turntable are open to the kitchen air.", "Durch die Türverriegelung abgeschaltet, die Tür auf ihren linken Scharnierzapfen um 95 Grad geöffnet: der vergilbte Garraum und der Drehteller liegen offen zur Küchenluft."), "bones": {**DOWN, **ASIDE, "door": {"twistDeg": -95}}, "ground": "authored", "contacts": ["foot-left", "foot-right", "pad-floor"], "thermal": binding(False, door_open=True, pad=HOME_PAD, home=True), "loads": [FOOD, DOOR_HAND]},
    {"id": "idle", "key": True, "title": T("Off with the door closed", "Aus mit geschlossener Tür"), "description": T("Key pose of the reheat clip: on the worktop, switched off, door closed.", "Schlüsselpose des Aufwärm-Clips: auf der Arbeitsplatte, ausgeschaltet, Tür geschlossen."), "bones": {**DOWN, **ASIDE}, "ground": "authored", "contacts": ["foot-left", "foot-right", "pad-floor"], "thermal": binding(False, pad=HOME_PAD, home=True), "loads": [FOOD]},
]

HOME_CONTACTS = [{"anchor": a, "fromS": 0, "toS": 40.0} for a in ("foot-left", "foot-right", "pad-floor")]
CLIPS = [
    {"id": "carry", "title": T("Carry off the depot shelf", "Aus dem Lagerregal tragen"), "description": T("One person takes hold under both bottom edges, lifts the oven off the cardboard and tilts it slightly towards the heavy transformer side as it is carried away; the cardboard stays behind.", "Eine Person fasst unter beide Unterkanten, hebt das Gerät von der Pappe und neigt es beim Wegtragen leicht zur schweren Transformatorseite; die Pappe bleibt liegen."), "durationS": 6.0, "fps": 25, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1.0, "pose": "stored", "ease": "smooth"}, {"t": 3.0, "pose": "carried", "ease": "smooth", "bones": {"housing": {"angleDeg": 0}}}, {"t": 5.0, "pose": "carried", "ease": "smooth"}, {"t": 6.0, "pose": "carried", "ease": "smooth"}],
     "contacts": [{"anchor": "foot-left", "fromS": 0, "toS": 1.0}, {"anchor": "foot-right", "fromS": 0, "toS": 1.0}, {"anchor": "grip-left", "fromS": 1.0, "toS": 6.0}, {"anchor": "grip-right", "fromS": 1.0, "toS": 6.0}, {"anchor": "pad-floor", "fromS": 0, "toS": 6.0}],
     "phases": [{"id": "take-hold", "fromS": 0, "toS": 1.0, "title": T("Taking hold", "Anfassen")}, {"id": "lift", "fromS": 1.0, "toS": 3.0, "title": T("Lifting off the cardboard", "Von der Pappe heben")}, {"id": "tilt", "fromS": 3.0, "toS": 5.0, "title": T("Tilting to the heavy side", "Zur schweren Seite neigen")}, {"id": "carry", "fromS": 5.0, "toS": 6.0, "title": T("Carrying", "Tragen")}],
     "events": [{"t": 1.0, "kind": "contact-off", "title": T("Off the cardboard", "Von der Pappe"), "target": "foot-left"}, {"t": 1.0, "kind": "contact-on", "title": T("Hands take the load", "Hände übernehmen die Last"), "target": "grip-left"}, {"t": 3.0, "kind": "motion", "title": T("Lifted 0.30 m", "0,30 m angehoben")}],
     "tracks": []},
    {"id": "reheat", "title": T("Reheat a plate for 30 seconds", "Einen Teller 30 Sekunden aufwärmen"), "description": T("The door is opened and closed again over the plate, the timer is turned to half a minute, the magnetron runs at full power while the turntable makes two and a half turns and the timer runs back, the bell rings and the door is opened.", "Die Tür wird über dem Teller geöffnet und wieder geschlossen, der Zeitschalter auf eine halbe Minute gedreht, das Magnetron läuft mit voller Leistung, während der Drehteller zweieinhalb Umdrehungen macht und der Zeitschalter zurückläuft, die Glocke klingelt und die Tür wird geöffnet."), "durationS": 40.0, "fps": 10, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "idle", "ease": "smooth"}, {"t": 1.5, "pose": "door-open", "ease": "smooth"}, {"t": 3.0, "pose": "door-open", "ease": "smooth"}, {"t": 4.5, "pose": "idle", "ease": "smooth"}, {"t": 5.5, "pose": "idle", "ease": "linear", "bones": {"knob-timer": {"angleDeg": -5}}},
              {"t": 6.0, "pose": "in-use", "ease": "linear", "bones": {"knob-timer": {"angleDeg": -5}, "turntable": {"twistDeg": 0}}}, {"t": 36.0, "pose": "in-use", "ease": "smooth", "bones": {"knob-timer": {"angleDeg": 0}, "turntable": {"twistDeg": 900}}},
              {"t": 37.0, "pose": "idle", "ease": "smooth", "bones": {"turntable": {"twistDeg": 900}}}, {"t": 38.5, "pose": "door-open", "ease": "smooth", "bones": {"turntable": {"twistDeg": 900}}}, {"t": 40.0, "pose": "door-open", "ease": "smooth", "bones": {"turntable": {"twistDeg": 900}}}],
     "contacts": HOME_CONTACTS,
     "phases": [{"id": "load", "fromS": 0, "toS": 4.5, "title": T("Opening and closing over the plate", "Über dem Teller öffnen und schließen")}, {"id": "set", "fromS": 4.5, "toS": 6.0, "title": T("Setting the timer", "Zeit einstellen")}, {"id": "heat", "fromS": 6.0, "toS": 36.0, "title": T("Heating with the turntable turning", "Heizen mit drehendem Teller")}, {"id": "ready", "fromS": 36.0, "toS": 40.0, "title": T("Bell and door open", "Glocke und Tür auf")}],
     "events": [{"t": 0.0, "kind": "motion", "title": T("Door opens", "Tür öffnet"), "target": "door"}, {"t": 3.0, "kind": "motion", "title": T("Door closes", "Tür schließt"), "target": "door"}, {"t": 6.0, "kind": "operating", "title": T("Magnetron on", "Magnetron an"), "target": "magnetron"}, {"t": 36.0, "kind": "shutdown", "title": T("Bell rings, magnetron off", "Glocke klingelt, Magnetron aus"), "target": "magnetron"}, {"t": 37.0, "kind": "motion", "title": T("Door opens", "Tür öffnet"), "target": "door"}],
     "tracks": [{"target": "operating:magnetron", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 6.0, "v": 1}, {"t": 36.0, "v": 0}, {"t": 40.0, "v": 0}]}]},
]

REFERENCES = [
    {"id": "iec-60335-2-25", "citation": "IEC 60335-2-25:2020 Household and similar electrical appliances - Safety - Part 2-25: Particular requirements for microwave ovens, including combination microwave ovens (door interlock, door hinge and stability tests, microwave leakage limit 50 W/m2 at 5 cm; product class of the 700-800 W solo ovens with 1150-1250 W rated input)."},
    {"id": "iec-60705", "citation": "IEC 60705:2010+A1:2014 Household microwave ovens - Methods for measuring performance (microwave output power by the 1 kg water-load method, usable cavity volume, turntable dimensions)."},
    {"id": "en-12150-1", "citation": "EN 12150-1:2015 Glass in building - Thermally toughened soda lime silicate safety glass (density 2500 kg/m3, E 70 GPa, characteristic bending strength 120 MPa)."},
    {"id": "iso-3585", "citation": "ISO 3585:1998 Borosilicate glass 3.3 - Properties (density 2230 kg/m3, thermal conductivity 1.14 W/mK at 20 degC, specific heat 830 J/kgK, E 64 GPa)."},
    {"id": "en-50525-2-11", "citation": "EN 50525-2-11:2011 Electric cables - Low voltage energy cables of rated voltages up to 450/750 V - Part 2-11: Cables for general applications - Flexible cables with thermoplastic PVC insulation (H05VV-F 3G1.0: outer diameter about 6 mm)."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (steel 50 W/mK and 7850 kg/m3, plastics 0.17-0.25 W/mK, paper and board 0.05-0.07 W/mK, specific heats)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivity and solar absorptance of white paints 0.2-0.3, aged and dirty paints higher, galvanised steel 0.2-0.3 emissivity when dull, glass 0.84-0.9)."},
    {"id": "din-18022", "citation": "DIN 18022:2007 Küche, Bad, WC, Hausarbeitsraum - Planungsgrundlagen (worktop depth 600 mm and height 850-950 mm on which a countertop microwave oven stands)."},
    {"id": "used-appliance-market", "citation": "Catalogue-typical 20-litre solo microwave ovens of the German discount and mid-range trade 1995-2012 (700 W output, 1150-1200 W input, about 440 x 258 x 340 mm, cavity about 315 x 212 x 293 mm, 245 mm glass turntable, two mechanical rotary knobs, push-button door release, 10.5-11.5 kg net) and the intake practice of municipal second-hand department stores (Gebrauchtwarenkaufhaus): electrical safety test after DIN VDE 0701-0702, cleaned, priced and shelved in a flattened banana box; no brand data used."},
]


VOLUME_DECISIONS = {
    50: [T("Body 1:50 is the oven as built: four turned polypropylene feet, the 1 mm galvanised base pan, the U-shaped 0.5 mm wrapper as left side, top and right side, the 0.5 mm back, the cavity liner as a 0.5 mm steel tube with its back wall (315 x 212 x 293 mm inside = 19.6 l), the turntable motor under the cavity floor, the 245 mm borosilicate plate with its rim on a slender roller ring and its coupler, the 35 mm moulded door frame with the 220 x 152 mm window and the 4 mm glass in it, two steel hinge pins that reach into the housing, the control fascia with the push-button door release, the original white timer knob and the smaller black replacement power knob, the cord leaving the back through its strain relief and lying on the cardboard to the moulded plug, the taped repair of the cord and the price sticker (textures), and the flattened banana box.", "Körper 1:50 ist das Gerät, wie es gebaut ist: vier gedrehte Polypropylenfüße, die 1-mm-Bodenwanne aus verzinktem Stahl, der U-förmige 0,5-mm-Mantel als linke Seite, Deckel und rechte Seite, die 0,5-mm-Rückwand, die Garraumauskleidung als 0,5-mm-Stahlrohr mit Rückwand (innen 315 x 212 x 293 mm = 19,6 l), der Drehtellermotor unter dem Garraumboden, der 245-mm-Teller aus Borosilikatglas mit Rand auf einem schlanken Rollenring und seinem Mitnehmer, der 35 mm tiefe gespritzte Türrahmen mit dem 220 x 152 mm großen Fenster und dem 4-mm-Glas darin, zwei Stahl-Scharnierzapfen, die ins Gehäuse greifen, die Bedienblende mit der Türöffnertaste, der originale weiße Zeitknopf und der kleinere schwarze Ersatz-Leistungsknopf, das Kabel, das hinten durch seinen Knickschutz austritt und auf der Pappe bis zum angespritzten Stecker liegt, die geklebte Reparatur am Kabel und der Preisaufkleber (Texturen) und der flachgelegte Bananenkarton."),
         T("Thermal 1:50 references the body by region; the wrapper, back, base, door, panel and the door gaps of 1.5 mm close the housing, so the cavity liner and the turntable are contents of a closed cavity seen through the window glazing and count both faces, and the transformer and magnetron are the hidden electronics region (thermal solids only). Structure 1:50: every sheet is the member of its bone, the hinge pins are the joints of the door, the coupler the joint of the turntable, one support pad stands under the cardboard and one mass marker at the centre of mass of the oven.", "Thermik 1:50 verweist nach Regionen auf den Körper; Mantel, Rückwand, Boden, Tür, Blende und die 1,5-mm-Türfugen schließen das Gehäuse, sodass Garraumauskleidung und Drehteller Inhalt eines geschlossenen Hohlraums sind, durch die Fensterverglasung sichtbar, und beide Seiten zählen; Transformator und Magnetron sind die verdeckte Region Elektronik (nur thermische Körper). Tragwerk 1:50: jedes Blech ist das Bauteil seines Knochens, die Scharnierzapfen sind die Gelenke der Tür, der Mitnehmer das Gelenk des Drehtellers, ein Auflagerklotz steht unter der Pappe und ein Massenmarker im Schwerpunkt des Geräts.")],
    100: [T("Body 1:100 merges base pan, wrapper and cavity liner into one housing shell extruded from the front to 15 mm before the back, with the cavity as its opening (a hollow part judged by its overall extents), and a 15 mm back plate; the four feet stay as 24 x 10 x 24 mm blocks because the drawing of the housing shows them 10 mm high; the door keeps its 35 mm frame with the window and the 4 mm glass as a declared sheet; the knobs become plain cylinders of their diameters; the turntable is one plate with its rim on a 30 mm hub that stands for the roller ring and the coupler; the cord is one slender run from the back to the plug; button, hinge pins, motor and all traces of use are below 15 mm and left out.", "Körper 1:100 fasst Bodenwanne, Mantel und Garraumauskleidung zu einer Gehäuseschale zusammen, die von der Front bis 15 mm vor die Rückseite gezogen ist und den Garraum als Öffnung hat (ein Hohlteil, nach seinen Gesamtmaßen beurteilt), dazu eine 15-mm-Rückplatte; die vier Füße bleiben Blöcke von 24 x 10 x 24 mm, weil die Zeichnung des Gehäuses sie 10 mm hoch zeigt; die Tür behält ihren 35-mm-Rahmen mit Fenster und das 4-mm-Glas als erklärte Platte; die Knöpfe werden einfache Zylinder ihrer Durchmesser; der Drehteller ist ein Teller mit Rand auf einer 30-mm-Nabe, die für Rollenring und Mitnehmer steht; das Kabel ist ein schlanker Strang von der Rückseite bis zum Stecker; Taste, Scharnierzapfen, Motor und alle Gebrauchsspuren liegen unter 15 mm und entfallen."),
          T("Thermal 1:100 references the body and adds the cavity lining as a 0.5 mm thermal tube with its back sheet inside the shell opening, and the transformer and magnetron as hidden blocks. Structure 1:100: the shell is the member solid of the housing and names base, wrapper sides, top and cavity (they lie in its walls), door, fascia, plate, cord and cardboard are their own members, one support and the mass marker.", "Thermik 1:100 verweist auf den Körper und ergänzt die Garraumauskleidung als thermisches 0,5-mm-Rohr mit Rückblech in der Öffnung der Schale sowie Transformator und Magnetron als verdeckte Blöcke. Tragwerk 1:100: die Schale ist der Bauteilkörper des Gehäuses und nennt Boden, Mantelseiten, Deckel und Garraum (sie liegen in ihren Wänden), Tür, Blende, Teller, Kabel und Pappe sind eigene Bauteile, ein Auflager und der Massenmarker.")],
}


def scale_decisions(s):
    """📝 The bilingual design decisions of the 2D representation of 1:s."""
    D = {
        50: dict(
            silhouette=T("Full construction at 8.8 x 5.3 mm: feet, base pan, the wrapper edges, the door with its window and glass, the control fascia with the two knobs and the door-release button, the cord and plug behind and the cardboard under the oven; behind the glass the yellowed cavity and the turntable.", "Volle Konstruktion auf 8,8 x 5,3 mm: Füße, Bodenwanne, die Mantelkanten, die Tür mit Fenster und Glas, die Bedienblende mit den zwei Knöpfen und der Türöffnertaste, Kabel und Stecker dahinter und die Pappe unter dem Gerät; hinter dem Glas der vergilbte Garraum und der Drehteller."),
            retained=T("Every sheet at its true gauge (0.5 mm wrapper and liner, 1 mm base, 4 mm glass), the 35 mm door, the 1.5 mm door gap, the 245 mm plate with its rim, the two different knobs, the 6 mm cord.", "Jedes Blech in seiner wahren Stärke (0,5 mm Mantel und Auskleidung, 1 mm Boden, 4 mm Glas), die 35-mm-Tür, die 1,5-mm-Türfuge, der 245-mm-Teller mit Rand, die zwei verschiedenen Knöpfe, das 6-mm-Kabel."),
            merged=T("The perforated screen is drawn with the glass; the choke seal of the door is part of the door frame.", "Der Lochschirm ist mit dem Glas gezeichnet; die Drosseldichtung der Tür gehört zum Türrahmen."),
            omitted=T("Screws, the waveguide cover, the internal wiring and the fan blades.", "Schrauben, die Hohlleiterabdeckung, die innere Verdrahtung und die Lüfterflügel."),
            thinElements=T("Wrapper, back, base, liner and glass keep their true thickness as thin filled shapes; the cord is a 6 mm filled strip.", "Mantel, Rückwand, Boden, Auskleidung und Glas behalten ihre wahre Stärke als dünne gefüllte Formen; das Kabel ist ein gefüllter 6-mm-Streifen."),
            hatching=T("No hatches: the parts read by tone, the glazing by its dark glass tone.", "Keine Schraffuren: die Teile lesen sich über den Ton, die Verglasung über ihren dunklen Glaston."),
            styles=T("Body pen 0.35 mm, texture pen 0.18 mm for the traces of use: scratches on the door glass, grease spots on the door frame, the worn print of the timer scale and of the power marks, the yellowed release button, the pointer of the mismatched black power knob, the slots and a dent on the right side, the price sticker, a dent and scratches on the top and the taped repair of the cord.", "Körperstift 0,35 mm, Texturstift 0,18 mm für die Gebrauchsspuren: Kratzer im Türglas, Fettflecken am Türrahmen, der abgegriffene Aufdruck der Zeitskala und der Leistungsmarken, die vergilbte Öffnertaste, der Zeiger des nicht passenden schwarzen Leistungsknopfs, die Schlitze und eine Delle an der rechten Seite, der Preisaufkleber, eine Delle und Kratzer auf dem Deckel und die geklebte Reparatur am Kabel."),
            overlays=T("Analysis views colour the same shapes by region and member; the door, its glass and the knobs are drawn as swept solids and the turntable parts as revolved solids, so every view follows them when the door opens or the plate turns.", "Die Analyseansichten färben dieselben Formen nach Region und Bauteil; Tür, Glas und Knöpfe sind als gezogene Körper und die Drehtellerteile als Drehkörper gezeichnet, damit ihnen jede Ansicht folgt, wenn die Tür aufgeht oder der Teller dreht.")),
        100: dict(
            silhouette=T("Oven at 4.4 x 2.7 mm: the housing as one shell, the door with its window, the fascia, the two knobs, the four feet, the cord and plug and the cardboard.", "Gerät auf 4,4 x 2,7 mm: das Gehäuse als eine Schale, die Tür mit Fenster, die Blende, die zwei Knöpfe, die vier Füße, Kabel und Stecker und die Pappe."),
            retained=T("Overall 0.44 x 0.266 x 0.395 m, the window, the knobs at their diameters, the feet gap, the turntable behind the glass.", "Gesamtmaß 0,44 x 0,266 x 0,395 m, das Fenster, die Knöpfe in ihren Durchmessern, die Fußfuge, der Drehteller hinter dem Glas."),
            merged=T("Wrapper, base and cavity liner into one shell; roller ring and coupler into the hub of the plate; strain relief and cord into one strand.", "Mantel, Boden und Garraumauskleidung zu einer Schale; Rollenring und Mitnehmer zur Nabe des Tellers; Knickschutz und Kabel zu einem Strang."),
            omitted=T("Button, hinge pins and all traces of use (below 15 mm).", "Taste, Scharnierzapfen und alle Gebrauchsspuren (unter 15 mm)."),
            thinElements=T("Glass and cardboard stay thin filled strips; the cord is a 6 mm strip.", "Glas und Pappe bleiben dünne gefüllte Streifen; das Kabel ist ein 6-mm-Streifen."),
            hatching=T("No hatches.", "Keine Schraffuren."),
            styles=T("Body pen 0.25 mm.", "Körperstift 0,25 mm."),
            overlays=T("As at 1:50.", "Wie in 1:50.")),
    }
    return D[s]


PENS = {50: {"body": 0.35, "detail": 0.18, "texture": 0.18, "glazing": 0.18, "ground": 0.5}, 100: {"body": 0.25, "detail": 0.13, "texture": 0.13, "glazing": 0.18, "ground": 0.35}}
RANGES = {50: (20, 75), 100: (75, 150)}

UNAVAILABLE = {
    200: dict(reason=T("The microwave oven is 0.44 x 0.27 m = 2.2 x 1.3 mm on paper at 1:200, under the 3 mm below which a whole component is not drawn: the door window, the knobs (38 mm = 0.19 mm) and the 35 mm door would be specks.", "Die Mikrowelle misst 0,44 x 0,27 m = 2,2 x 1,3 mm auf dem Papier bei 1:200, unter den 3 mm, unter denen ein ganzes Bauteil nicht gezeichnet wird: Türfenster, Knöpfe (38 mm = 0,19 mm) und die 35-mm-Tür wären Punkte."),
              limiting=T("Knobs and door depth (0.035-0.038 m = 0.18-0.19 mm)", "Knöpfe und Türtiefe (0,035-0,038 m = 0,18-0,19 mm)"), size=0.035),
    500: dict(reason=T("The microwave oven is 0.44 x 0.27 m = 0.9 x 0.5 mm on paper at 1:500: only an anonymous speck would print.", "Die Mikrowelle misst 0,44 x 0,27 m = 0,9 x 0,5 mm auf dem Papier bei 1:500: es würde nur ein namenloser Fleck drucken."),
              limiting=T("The whole oven (0.44 m = 0.9 mm)", "Das ganze Gerät (0,44 m = 0,9 mm)"), size=0.44),
}

representations, volumes = {}, {}
for s in (200, 500):
    u = UNAVAILABLE[s]
    representations[str(s)] = {"status": "unavailable", "cause": "below-printable-feature", "denominator": s, "reason": u["reason"], "limitingFeature": u["limiting"], "limitingSizeM": u["size"], "alternatives": [{"kind": "scale", "denominator": 100}]}
    volumes[str(s)] = {"status": "unavailable", "denominator": s}
for s in (100, 50):
    body, thermal, struct, ev, fv, pv = parts(s)
    representations[str(s)] = {"status": "authored", "denominator": s, "validRange": {"minDenominator": RANGES[s][0], "maxDenominator": RANGES[s][1]}, "pens": PENS[s], "hatches": [], "geometry": {"elevation": ev, "front": fv, "plan": pv}, "decisions": scale_decisions(s)}
    volumes[str(s)] = {"status": "authored", "denominator": s, "body": body, "thermal": thermal, "structure": struct, "decisions": VOLUME_DECISIONS[s]}
representations = {k: representations[k] for k in ("50", "100", "200", "500")}
volumes = {k: volumes[k] for k in ("50", "100", "200", "500")}

DEPTH = round(PAD[3] - PAD[2], 3)
KG_DE = str(OVEN_KG).replace(".", ",")
asset = {
    "id": "used-microwave-oven",
    "category": "appliances",
    "title": T("Used Microwave Oven", "Gebrauchte Mikrowelle"),
    "summary": T(f"A second-hand 20-litre solo microwave oven 0.44 x 0.258 x 0.352 m of about 2005, {OVEN_KG} kg, 700 W output and 1150 W rated input: a 315 x 212 x 293 mm cavity whose white paint has yellowed and is splattered, a 245 mm glass turntable on its roller ring, a side-hinged door with a scratched window, a push-button door release, the original timer knob and a black replacement power knob, a dented top with a price sticker and a taped mains cord; stored on a flattened banana box at the depot, carried by one person, running again on a kitchen worktop with its turntable turning.", f"Eine gebrauchte 20-Liter-Solo-Mikrowelle 0,44 x 0,258 x 0,352 m von etwa 2005, {KG_DE} kg, 700 W Leistung und 1150 W Nennaufnahme: ein Garraum von 315 x 212 x 293 mm, dessen weißer Lack vergilbt und bespritzt ist, ein 245-mm-Glasdrehteller auf seinem Rollenring, eine seitlich angeschlagene Tür mit zerkratztem Fenster, eine Türöffnertaste, der originale Zeitknopf und ein schwarzer Ersatz-Leistungsknopf, ein eingedellter Deckel mit Preisaufkleber und ein geklebtes Netzkabel; gelagert auf einem flachgelegten Bananenkarton im Lager, von einer Person getragen, wieder in Betrieb auf einer Küchenarbeitsplatte mit drehendem Teller."),
    "tags": ["used", "household", "second-hand", "kitchen", "microwave", "appliance", "turntable", "steel", "glass", "abs", "countertop"],
    "provenance": {"author": "Semio staffage content (used household objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES,
                   "notes": T(f"Type: German discount and mid-range 20-litre solo microwave oven of about 1995-2012 (700 W output after IEC 60705, 1150 W rated input), 440 x 258 x 335 mm over housing and door, 352 mm over the knobs; cavity 315 x 212 x 293 mm; 245 mm borosilicate turntable at about 5 rpm; door hinged on the left with a push-button release on the panel; two mechanical knobs (power level, 30-minute clockwork timer). Masses: sheets, glass, cord, plug, feet and cardboard are their 1:50 solid volumes times the densities; the moulded door frame and fascia are their surface areas times a 2.5 mm wall; transformer 2.8 kg, magnetron 0.8 kg, fan 0.25 kg, HV parts 0.25 kg, timer and switches 0.3 kg, door plate and screen 0.45 kg and turntable motor 0.1 kg are catalogue-typical component masses; together {OVEN_KG} kg for the oven and {TOTAL_KG} kg with the cardboard. Traces of use are authored from the intake of second-hand department stores; nothing is measured.", f"Typ: 20-Liter-Solo-Mikrowelle des deutschen Discount- und Mittelpreissegments von etwa 1995-2012 (700 W Leistung nach IEC 60705, 1150 W Nennaufnahme), 440 x 258 x 335 mm über Gehäuse und Tür, 352 mm über die Knöpfe; Garraum 315 x 212 x 293 mm; 245-mm-Drehteller aus Borosilikatglas mit etwa 5 U/min; Tür links angeschlagen mit Öffnertaste in der Blende; zwei mechanische Knöpfe (Leistungsstufe, 30-Minuten-Uhrwerk). Massen: Bleche, Glas, Kabel, Stecker, Füße und Pappe sind ihre 1:50-Körpervolumen mal den Dichten; Türrahmen und Blende aus Spritzguss sind ihre Oberflächen mal 2,5 mm Wand; Transformator 2,8 kg, Magnetron 0,8 kg, Lüfter 0,25 kg, Hochspannungsteile 0,25 kg, Zeitschalter und Schalter 0,3 kg, Türblech und Schirm 0,45 kg und Drehtellermotor 0,1 kg sind katalogtypische Bauteilmassen; zusammen {KG_DE} kg für das Gerät und {str(TOTAL_KG).replace('.', ',')} kg mit der Pappe. Gebrauchsspuren sind nach der Annahme von Gebrauchtwarenkaufhäusern geschätzt; nichts ist gemessen.")},
    "dimensions": {"heightM": round(Y(H), 3), "widthM": round(PAD[1] - PAD[0], 3), "depthM": DEPTH, "massKg": TOTAL_KG},
    "materials": MATERIALS,
    "rig": {"kind": "mechanical", "root": "ground", "bones": [bone_json(*b) for b in BONES]},
    "anchors": ANCHORS,
    "representations": representations,
    "symbols": [],
    "poses": POSES,
    "clips": CLIPS,
    "thermal": {
        "environment": {"airTempC": 21, "meanRadiantTempC": 21, "relativeHumidity": 0.5, "windMS": 0.05, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 0, "pressurePa": 101325},
        "passengers": [],
        "operating": [{"id": "magnetron", "title": T("Magnetron", "Magnetron"), "levels": [{"v": 0, "title": T("Off", "Aus")}, {"v": 1, "title": T("Full power (700 W microwave output, 1150 W input)", "Volle Leistung (700 W Mikrowellenleistung, 1150 W Aufnahme)")}]}],
        "regions": REGIONS,
        "emitters": [{"id": "exhaust", "kind": "vent", "title": T("Cooling air blown out of the right side vents", "Kühlluft aus den Schlitzen der rechten Seite"), "anchor": "exhaust", "directionDeg": 0, "diameterM": 0.08, "operatingKey": "magnetron", "volumeFlowM3S": 0.012, "temperatureRiseK": 25, "source": "authored-estimate", "reference": "incropera-2018"}],
        "assumptions": [
            T("Rated input 1150 W (electrical input, the nameplate of the class after IEC 60335-2-25); 700 W of it reach the food as microwave power (IEC 60705) and are not a part of the model. Of the 450 W of losses the cooling fan blows about 360 W out of the right side vents (0.012 m3/s warmed by 25 K), the rest leaves through the electronics (40 W), the casing (35 W), the door (10 W) and the window (5 W); the cavity liner (60 W) and the turntable (15 W) are warmed by the food and its steam.", "Nennaufnahme 1150 W (elektrische Aufnahme, das Typenschild der Klasse nach IEC 60335-2-25); 700 W davon erreichen das Essen als Mikrowellenleistung (IEC 60705) und sind nicht Teil des Modells. Von den 450 W Verlusten bläst der Lüfter etwa 360 W aus den Schlitzen der rechten Seite (0,012 m3/s um 25 K erwärmt), der Rest geht über die Elektronik (40 W), das Gehäuse (35 W), die Tür (10 W) und das Fenster (5 W); Garraumauskleidung (60 W) und Drehteller (15 W) werden vom Essen und seinem Dampf erwärmt."),
            T("Aged surfaces: the white powder coat has yellowed, so its solar absorptance is taken as 0.35 instead of about 0.25 when new; the cavity paint is yellowed and splattered with fat (0.5 instead of about 0.3); the moulded ABS has yellowed (0.42); authored estimates after Incropera, white paints 0.2-0.3. The galvanised base pan is dull (emissivity 0.28).", "Gealterte Oberflächen: die weiße Pulverbeschichtung ist vergilbt, daher gilt ein solarer Absorptionsgrad von 0,35 statt etwa 0,25 im Neuzustand; der Garraumlack ist vergilbt und mit Fett bespritzt (0,5 statt etwa 0,3); das gespritzte ABS ist vergilbt (0,42); Schätzungen nach Incropera, weiße Lacke 0,2-0,3. Die verzinkte Bodenwanne ist matt (Emissionsgrad 0,28)."),
            T("The closed door makes the housing a closed cavity: liner and turntable are contents seen through the window and count both faces; transformer and magnetron are hidden in the technical bay. Feet, cord and plug are 0.01 m2 together and counted in the casing region, knobs and button in the panel region.", "Die geschlossene Tür macht das Gehäuse zu einem geschlossenen Hohlraum: Auskleidung und Drehteller sind durch das Fenster sichtbarer Inhalt und zählen beide Seiten; Transformator und Magnetron sind im Technikraum verdeckt. Füße, Kabel und Stecker haben zusammen 0,01 m2 und zählen zur Region Gehäuse, Knöpfe und Taste zur Region Blende."),
            T("Reference case: a kitchen at 21 degC without sun; the depot pose uses the same air. The control panel is passive (0 W).", "Referenzfall: eine Küche bei 21 degC ohne Sonne; die Lagerpose nutzt dieselbe Luft. Die Bedienblende ist passiv (0 W)."),
        ],
    },
    "structure": {
        "gravityMS2": G, "dynamics": "quasi-static",
        "supports": ["pad-floor"],
        "assumptions": [
            T("The housing is a box of 0.5 mm steel sheet (wrapper, back) on a 1 mm base pan, stiffened by the welded cavity liner; members are the sheets on their bones with their true sections, the housing bone carries the closed box section 300 x 247 x 0.5 mm. The oven stands on four feet in two pairs (contacts).", "Das Gehäuse ist ein Kasten aus 0,5-mm-Stahlblech (Mantel, Rückwand) auf einer 1-mm-Bodenwanne, ausgesteift durch die geschweißte Garraumauskleidung; Bauteile sind die Bleche auf ihren Knochen mit ihren wahren Querschnitten, der Gehäuseknochen trägt den geschlossenen Kastenquerschnitt 300 x 247 x 0,5 mm. Das Gerät steht auf vier Füßen in zwei Paaren (Kontakte)."),
            T(f"Centre of mass at x = {COM_HOUSING[0]} m from the left side (the 2.8 kg transformer pulls it towards the control side) and {COM_HOUSING[1]} m above the feet; carried by one person with both hands under the bottom edges (pins), the right hand takes the larger share.", f"Schwerpunkt bei x = {str(COM_HOUSING[0]).replace('.', ',')} m von der linken Seite (der 2,8-kg-Transformator zieht ihn zur Bedienseite) und {str(COM_HOUSING[1]).replace('.', ',')} m über den Füßen; von einer Person mit beiden Händen unter den Unterkanten getragen (Gelenke), die rechte Hand trägt den größeren Teil."),
            T(f"Tipping with the door open (IEC 60335-2-25 stability): the {OVEN_KG} kg oven tips forward about its front feet (z = -0.105 m) when a load on the open door edge (about 0.45 m in front of them) exceeds about {round(OVEN_KG * G * 0.12 / 0.345)} N; the 20 N of a hand resting on the door are held. The planar solver of the archive resolves the loads in the width plane; the forward tipping check is this hand calculation.", f"Kippen bei offener Tür (Standsicherheit nach IEC 60335-2-25): das {KG_DE}-kg-Gerät kippt um seine vorderen Füße (z = -0,105 m) nach vorn, wenn eine Last an der offenen Türkante (etwa 0,45 m davor) etwa {round(OVEN_KG * G * 0.12 / 0.345)} N übersteigt; die 20 N einer auf der Tür ruhenden Hand werden gehalten. Der ebene Löser des Archivs löst die Lasten in der Breitenebene; die Kippprüfung nach vorn ist diese Handrechnung."),
            T("The cardboard pad stays behind on its own contact when the oven is lifted and lies folded beside it in the kitchen; the solver treats the asset as one rigid body (see requests.md, used-coffee-table), so a small share of the oven weight leaks to the pad in the carried and in-use poses.", "Die Pappunterlage bleibt beim Anheben mit eigenem Kontakt liegen und liegt in der Küche gefaltet daneben; der Löser behandelt das Asset als einen starren Körper (siehe requests.md, used-coffee-table), daher fließt in den Posen Getragen und In Betrieb ein kleiner Teil des Gerätegewichts in die Pappe."),
        ],
        "clinical": False,
    },
    "volumes": volumes,
    "presentation": {"defaultPose": "stored", "defaultClip": "reheat", "defaultScale": 50},
    "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    "completion": "authored",
}

doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as fh:
    json.dump(doc, fh, ensure_ascii=False, indent=1)
print(f"[DEBUG] mass oven {OVEN_KG} kg, total {TOTAL_KG} kg; com {COM_HOUSING}; " + "; ".join(f"{b} {m:.3f}" for b, (m, _, _) in MASS.items()) + "; solids " + ", ".join(f"1:{s} {len(volumes[str(s)]['body'])}/{len(volumes[str(s)]['thermal'])}/{len(volumes[str(s)]['structure'])}" for s in (50, 100)))
