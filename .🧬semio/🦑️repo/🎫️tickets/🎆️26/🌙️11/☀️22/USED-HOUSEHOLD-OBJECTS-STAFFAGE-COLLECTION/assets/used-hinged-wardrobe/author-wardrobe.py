"""🚪 Authors the used three-door hinged wardrobe (`used-hinged-wardrobe`) from hand-chosen numbers and writes `🔣️.json` next to this file.

Every part is stated once in world metres of the rest pose (the stored pose: the carcass standing on a folded moving blanket) and converted into the
frame of its bone, so the 2D drawings and the 3D solids of one scale come from the same numbers. Each scale is its own part list.

@see ../../brief.md — the author brief
@see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-tools.md — the tools that check the output
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
LIFT = 0.012
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
DOOR_Z = (-0.266, -0.250)
CARCASS_Z = (-0.250, 0.287)
BACK_Z = (0.287, 0.290)
SHELF_Z = (-0.235, 0.285)
SHELF_HEIGHTS = (0.44, 0.80, 1.16, 1.52)
DOORS = {"door-1": (0.0015, 0.4985), "door-2": (0.5015, 0.9985), "door-3": (1.0015, 1.4985)}
HINGE_Y = (0.20, 1.03, 1.86)


def T(en, de):
    """🌐 A bilingual text, English first."""
    return {"en": en, "de": de}


def Y(y):
    """⬆️ World height of a carcass-local height in the rest (stored) pose, on the blanket."""
    return round(y + LIFT, 6)


def r(v):
    """🔢 Rounds a coordinate to a tenth of a millimetre."""
    return round(v + 0.0, 6)


BONES = [
    ("ground", None, 0, 0, 0, 0, 0, {"type": "fixed"}, T("Floor reference", "Bodenbezug")),
    ("blanket", "ground", 0.004, 0, 0, 0, 1.492, {"type": "prismatic", "axisDeg": 0, "minM": -1.70, "maxM": 0}, T("Folded moving blanket under the carcass", "Gefaltete Umzugsdecke unter dem Korpus")),
    ("lift", "ground", 0, LIFT, 0, 0, 0, {"type": "prismatic", "axisDeg": 90, "minM": -LIFT, "maxM": 0.4}, T("Lift of the carcass off its base", "Anheben des Korpus von seiner Unterlage")),
    ("carcass", "lift", 0, 0, 0, 0, 1.5, {"type": "revolute", "minDeg": -15, "maxDeg": 15}, T("Carcass, tilt about the lower left corner", "Korpus, Neigung um die untere linke Ecke")),
    ("side-l", "carcass", 0.008, 0, 0, 90, 2.0, {"type": "fixed"}, T("Left side panel", "Linke Seitenwand")),
    ("side-r", "carcass", 1.492, 0, 0, 90, 2.0, {"type": "fixed"}, T("Right side panel", "Rechte Seitenwand")),
    ("partition", "carcass", 0.5, 0.076, 0, 90, 1.908, {"type": "fixed"}, T("Middle partition", "Mittelwand")),
    ("bottom", "carcass", 0.016, 0.068, 0, 0, 1.468, {"type": "fixed"}, T("Bottom panel", "Bodenplatte")),
    ("top", "carcass", 0.016, 1.992, 0, 0, 1.468, {"type": "fixed"}, T("Top panel", "Oberboden")),
    ("plinth", "carcass", 0.016, 0.03, 0, 0, 1.468, {"type": "fixed"}, T("Plinth board", "Sockelblende")),
] + [
    (f"shelf-{k + 1}", "carcass", 0.016, h + 0.008, 0, 0, 0.476, {"type": "fixed"}, T(f"Shelf {k + 1} of the left compartment", f"Fachboden {k + 1} des linken Fachs"))
    for k, h in enumerate(SHELF_HEIGHTS)
] + [
    ("hat-shelf", "carcass", 0.508, 1.668, 0, 0, 0.976, {"type": "fixed"}, T("Hat shelf over the hanging space", "Hutboden über dem Hängeraum")),
    ("rail", "carcass", 0.508, 1.60, 0, 0, 0.976, {"type": "fixed"}, T("Oval clothes rail", "Ovale Kleiderstange")),
    ("door-1", "carcass", 0.0015, 0.062, -0.258, 90, 1.936, {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": -110, "twistMaxDeg": 0}, T("Left door, hinged on the left side panel", "Linke Tür, an der linken Seitenwand angeschlagen")),
    ("door-2", "carcass", 0.5015, 0.062, -0.258, 90, 1.936, {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": -110, "twistMaxDeg": 0}, T("Middle mirror door, hinged on the partition", "Mittlere Spiegeltür, an der Mittelwand angeschlagen")),
    ("door-3", "carcass", 1.4985, 0.062, -0.258, 90, 1.936, {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": 0, "twistMaxDeg": 110}, T("Right door, hinged on the right side panel", "Rechte Tür, an der rechten Seitenwand angeschlagen")),
    ("bracket", "top", 0.704, 0.008, 0, 0, 0.06, {"type": "fixed"}, T("Tipping restraint angle on the top", "Kippsicherungswinkel auf dem Oberboden")),
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


def frect(z0, z1, y0, y1):
    """🟦 A front rect in rest world (z, y)."""
    return {"kind": "rect", "x": r(z0), "y": r(y0), "w": r(z1 - z0), "h": r(y1 - y0)}


def prect(x0, x1, z0, z1):
    """🟩 A plan rect in rest world (x, z)."""
    return {"kind": "rect", "x": r(x0), "y": r(z0), "w": r(x1 - x0), "h": r(z1 - z0)}


def sweep(z0, z1):
    """🌀 The extruded solid placement of a part that turns with its bone (a door and what it carries)."""
    return {"solid": {"kind": "extruded", "sweep": "w", "z0": r(z0), "z1": r(z1)}}


def sliver(bone, pa, pb, width=0.0015):
    """〰️ A scratch as a thin closed quad along the world segment pa-pb (carcass-local heights), so it can turn with a door."""
    (xa, ya), (xb, yb) = pa, pb
    ya, yb = Y(ya), Y(yb)
    L = math.hypot(xb - xa, yb - ya)
    nx, ny = -(yb - ya) / L * width / 2, (xb - xa) / L * width / 2
    return epoly(bone, [(xa + nx, ya + ny), (xb + nx, yb + ny), (xb - nx, yb - ny), (xa - nx, ya - ny)])


def door_pts(door, chipped):
    """🚪 The world outline of a door leaf; the left door has its lower hinge-side corner broken out (18 x 12 mm)."""
    x0, x1 = DOORS[door]
    if chipped:
        return [(x0 + 0.018, Y(0.062)), (x1, Y(0.062)), (x1, Y(1.998)), (x0, Y(1.998)), (x0, Y(0.074)), (x0 + 0.018, Y(0.074))]
    return rect_pts(x0, x1, Y(0.062), Y(1.998))


MATERIALS = [
    {"id": "foil-chipboard", "title": T("Melamine-foiled chipboard P2, 16 mm, white, aged", "Melaminbeschichtete Spanplatte P2, 16 mm, weiß, gealtert"), "emissivity": 0.9, "solarAbsorptivity": 0.32, "densityKgM3": 650, "conductivityWmK": 0.13, "specificHeatJKgK": 1700, "elasticModulusPa": 2.3e9, "strengthPa": 11.5e6, "source": "literature", "reference": "en-312"},
    {"id": "hdf-back", "title": T("Hardboard back panel HDF, 3 mm, raw", "Hartfaser-Rückwand HDF, 3 mm, roh"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 850, "conductivityWmK": 0.14, "specificHeatJKgK": 1700, "elasticModulusPa": 3.0e9, "strengthPa": 30e6, "source": "literature", "reference": "en-622-5"},
    {"id": "mirror-glass", "title": T("Silvered float glass mirror, 4 mm", "Silberspiegel aus Floatglas, 4 mm"), "emissivity": 0.84, "solarAbsorptivity": 0.12, "densityKgM3": 2500, "conductivityWmK": 1.0, "specificHeatJKgK": 720, "elasticModulusPa": 70e9, "strengthPa": 45e6, "source": "literature", "reference": "en-572-2"},
    {"id": "chromed-steel", "title": T("Chrome-plated steel (handles, clothes rail)", "Verchromter Stahl (Griffe, Kleiderstange)"), "emissivity": 0.12, "solarAbsorptivity": 0.35, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210e9, "strengthPa": 235e6, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "zinc-steel", "title": T("Zinc-plated steel (hinges, restraint angle)", "Verzinkter Stahl (Scharniere, Kippschutzwinkel)"), "emissivity": 0.25, "solarAbsorptivity": 0.5, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210e9, "strengthPa": 235e6, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "beech-knob", "title": T("Solid beech, oiled (replacement knob)", "Buche massiv, geölt (Ersatzknopf)"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 720, "conductivityWmK": 0.16, "specificHeatJKgK": 1600, "elasticModulusPa": 14e9, "strengthPa": 100e6, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "moving-blanket", "title": T("Quilted moving blanket, cotton-polyester felt", "Gesteppte Umzugsdecke, Baumwoll-Polyester-Filz"), "emissivity": 0.9, "solarAbsorptivity": 0.75, "densityKgM3": 160, "conductivityWmK": 0.045, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "iso-10456-2007"},
]
DENSITY = {m["id"]: m["densityKgM3"] for m in MATERIALS}


def layer(lid, title, kind, material, t):
    """🧶 One board layer: resistance t/λ and the heat capacity of the whole board."""
    m = next(x for x in MATERIALS if x["id"] == material)
    return {"id": lid, "title": title, "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": round(t / m["conductivityWmK"], 6), "heatCapacityJM2K": round(m["densityKgM3"] * m["specificHeatJKgK"] * t, 1), "source": "literature", "reference": "iso-10456-2007"}


AREA = {"carcass": 4.08, "doors": 3.09, "mirror": 0.514, "plinth": 0.2, "back-panel": 2.93, "interior": 5.5, "blanket": 1.58}

REGIONS = [
    {"id": "carcass", "title": T("Side panels, top and front edges of the carcass", "Seitenwände, Oberboden und Vorderkanten des Korpus"), "kind": "surface", "model": "passive", "material": "foil-chipboard", "areaM2": AREA["carcass"], "characteristicLengthM": 0.54, "geometry": "vertical-plate", "skyViewFactor": 0.5, "layers": [layer("carcass-board", T("Chipboard 16 mm", "Spanplatte 16 mm"), "timber", "foil-chipboard", 0.016)], "heatCapacityJM2K": 17680.0, "ratedPowerW": 0, "ratedPowerBasis": "passive", "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "doors", "title": T("Door leaves with handles and the beech knob", "Türblätter mit Griffen und Buchenknopf"), "kind": "surface", "model": "passive", "material": "foil-chipboard", "areaM2": AREA["doors"], "characteristicLengthM": 0.5, "geometry": "vertical-plate", "skyViewFactor": 0.5, "layers": [layer("door-board", T("Door board 16 mm", "Türplatte 16 mm"), "timber", "foil-chipboard", 0.016)], "heatCapacityJM2K": 17680.0, "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "mirror", "title": T("Mirror on the middle door", "Spiegel auf der mittleren Tür"), "kind": "glazing", "model": "passive", "material": "mirror-glass", "areaM2": AREA["mirror"], "characteristicLengthM": 0.357, "geometry": "vertical-plate", "skyViewFactor": 0.5, "layers": [layer("mirror-pane", T("Mirror glass 4 mm", "Spiegelglas 4 mm"), "glass", "mirror-glass", 0.004)], "heatCapacityJM2K": 7200.0, "source": "literature", "reference": "en-572-2"},
    {"id": "plinth", "title": T("Scuffed plinth board", "Verschrammte Sockelblende"), "kind": "surface", "model": "passive", "material": "foil-chipboard", "areaM2": AREA["plinth"], "characteristicLengthM": 0.06, "geometry": "vertical-plate", "skyViewFactor": 0.3, "layers": [layer("plinth-board", T("Chipboard 16 mm", "Spanplatte 16 mm"), "timber", "foil-chipboard", 0.016)], "heatCapacityJM2K": 17680.0, "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "back-panel", "title": T("Hardboard back panel", "Hartfaser-Rückwand"), "kind": "surface", "model": "passive", "material": "hdf-back", "areaM2": AREA["back-panel"], "characteristicLengthM": 1.5, "geometry": "vertical-plate", "skyViewFactor": 0.5, "layers": [layer("back-board", T("HDF 3 mm", "HDF 3 mm"), "timber", "hdf-back", 0.003)], "heatCapacityJM2K": 4335.0, "source": "literature", "reference": "en-622-5"},
    {"id": "interior", "title": T("Interior: partition, shelves, hat shelf, clothes rail, hinge arms", "Innenraum: Mittelwand, Fachböden, Hutboden, Kleiderstange, Scharnierarme"), "kind": "surface", "model": "passive", "material": "foil-chipboard", "areaM2": AREA["interior"], "characteristicLengthM": 0.52, "geometry": "horizontal-plate-up", "skyViewFactor": 0.0, "layers": [layer("interior-board", T("Chipboard 16 mm", "Spanplatte 16 mm"), "timber", "foil-chipboard", 0.016)], "heatCapacityJM2K": 17680.0, "hidden": T("Inside the closed carcass behind the doors: only an open door shows it.", "Im geschlossenen Korpus hinter den Türen: nur eine offene Tür zeigt ihn."), "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "blanket", "title": T("Folded moving blanket", "Gefaltete Umzugsdecke"), "kind": "surface", "model": "passive", "material": "moving-blanket", "areaM2": AREA["blanket"], "characteristicLengthM": 0.5, "geometry": "horizontal-plate-up", "skyViewFactor": 0.3, "layers": [layer("blanket-felt", T("Quilted felt 12 mm (folded)", "Steppfilz 12 mm (gefaltet)"), "clothing", "moving-blanket", 0.012)], "heatCapacityJM2K": 2496.0, "source": "authored-estimate", "reference": "iso-10456-2007"},
]
REGION_IDS = [g["id"] for g in REGIONS]


def parts(s):
    """🧩 The hand-chosen parts of scale 1:s: body solids, thermal solids, structure solids and the 2D shapes of the three views."""
    f = FEATURE[s]
    body, thermal, struct = [], [], []
    ev, fv, pv = [], [], []
    tex = s == 50

    def th(sid, of, region):
        thermal.append({"id": f"th-{sid}", "of": of, "region": region})

    def mem(of, member, part="member", **extra):
        d = {"id": f"m-{of}", "of": of, "member": member, "part": part}
        d.update(extra)
        struct.append(d)

    board = {"sheet": True} if s in (200, 500) else {}
    inner = s != 500

    def thc(sol, region):
        thermal.append(dict(sol, id=f"th-{sol['id']}", region=region))

    if s in (50, 100, 200):
        for side, x0 in (("side-l", 0.0), ("side-r", 1.484)):
            body.append(box(side, side, (x0, x0 + 0.016, Y(0), Y(2.0), *CARCASS_Z), "foil-chipboard", 0.82, **board))
            th(side, side, "carcass")
            mem(side, side)
            ev.append(shape(side, side, erect(side, x0, x0 + 0.016, Y(0), Y(2.0)), "body", 2.0, True, 0.82, region="carcass", material="foil-chipboard"))
            if s != 200:
                pv.append(shape(side, side, prect(x0, x0 + 0.016, *CARCASS_Z), "body", 3.0, True, 0.82, region="carcass", material="foil-chipboard"))
        fv.append(shape("side-r", "side-r", frect(CARCASS_Z[0], CARCASS_Z[1] if s != 200 else BACK_Z[1], Y(0), Y(2.0)), "body", 3.0, True, 0.82, region="carcass", material="foil-chipboard"))
        for bid, y0 in (("top", 1.984), ("bottom", 0.060)):
            body.append(box(bid, bid, (0.016, 1.484, Y(y0), Y(y0 + 0.016), *CARCASS_Z), "foil-chipboard", 0.82, **board))
            th(bid, bid, "carcass")
            mem(bid, bid)
            ev.append(shape(bid, bid, erect(bid, 0.016, 1.484, Y(y0), Y(y0 + 0.016)), "body", 2.0, True, 0.82, region="carcass", material="foil-chipboard"))
        if s == 200:
            pv.append(shape("top", "top", prect(0, 1.5, CARCASS_Z[0], BACK_Z[1]), "body", 3.0, True, 0.84, region="carcass", material="foil-chipboard"))
        else:
            pv.append(shape("top", "top", prect(0.016, 1.484, *CARCASS_Z), "body", 3.0, True, 0.84, region="carcass", material="foil-chipboard"))
        if s == 50:
            pl = [(0.016, Y(0.010)), (0.044, Y(0.010)), (0.044, Y(0)), (1.484, Y(0)), (1.484, Y(0.060)), (0.016, Y(0.060))]
            body.append(extrude("plinth", "plinth", pl, -0.230, -0.214, "foil-chipboard", 0.55))
            ev.append(shape("plinth", "plinth", epoly("plinth", pl), "body", 3.0, True, 0.55, region="plinth", material="foil-chipboard"))
        else:
            pz1 = -0.214 if s == 100 else -0.200
            body.append(box("plinth", "plinth", (0.016, 1.484, Y(0), Y(0.060), -0.230, pz1), "foil-chipboard", 0.55))
            ev.append(shape("plinth", "plinth", erect("plinth", 0.016, 1.484, Y(0), Y(0.060)), "body", 3.0, True, 0.55, region="plinth", material="foil-chipboard"))
        th("plinth", "plinth", "plinth")
        mem("plinth", "plinth")
    else:
        outer = [(0, Y(0)), (1.5, Y(0)), (1.5, Y(2.0)), (0, Y(2.0))]
        hole = rect_pts(0.016, 1.484, Y(0.076), Y(1.984))
        body.append(extrude("carcass-frame", "carcass", outer, *CARCASS_Z, "foil-chipboard", 0.82, holes=[hole]))
        th("carcass-frame", "carcass-frame", "carcass")
        mem("carcass-frame", "carcass", members=["side-l", "side-r", "top", "bottom", "plinth"])
        ev.append(shape("carcass-frame", "carcass", erect("carcass", 0, 1.5, Y(0), Y(2.0)), "body", 2.0, True, 0.82, region="carcass", material="foil-chipboard"))
        fv.append(shape("carcass-frame", "carcass", frect(CARCASS_Z[0], BACK_Z[1], Y(0), Y(2.0)), "body", 3.0, True, 0.82, region="carcass", material="foil-chipboard"))
        pv.append(shape("carcass-frame", "carcass", prect(0, 1.5, CARCASS_Z[0], BACK_Z[1]), "body", 3.0, True, 0.84, region="carcass", material="foil-chipboard"))
        thermal.append(box("th-plinth", "plinth", (0.016, 1.484, Y(0), Y(0.075), CARCASS_Z[0], CARCASS_Z[0] + 0.075), "foil-chipboard", 0.55, region="plinth"))
        ev.append(shape("plinth", "carcass", erect("carcass", 0.0, 1.5, Y(0), Y(0.062)), "body", 3.0, True, 0.55, region="plinth", material="foil-chipboard"))
    part = box("partition", "partition", (0.492, 0.508, Y(0.076), Y(1.984), *CARCASS_Z), "foil-chipboard", 0.8, **board)
    body.append(part)
    thc(part, "interior")
    mem("partition", "partition")
    if inner:
        ev.append(shape("partition", "partition", erect("partition", 0.492, 0.508, Y(0.076), Y(1.984)), "body", 2.0, True, 0.8, region="interior", material="foil-chipboard"))

    body.append(box("back", "carcass", (0, 1.5, Y(0.060), Y(2.0), *BACK_Z), "hdf-back", 0.62, sheet=True))
    th("back", "back", "back-panel")
    mem("back", "carcass")
    ev.append(shape("back", "carcass", erect("carcass", 0, 1.5, Y(0.060), Y(2.0)), "body", 1.0, True, 0.62, region="back-panel", material="hdf-back"))
    if s in (50, 100):
        fv.append(shape("back", "carcass", frect(BACK_Z[0], BACK_Z[1], Y(0.060), Y(2.0)), "body", 3.1, True, 0.62, region="back-panel", material="hdf-back"))
        pv.append(shape("back", "carcass", prect(0, 1.5, *BACK_Z), "body", 3.1, True, 0.62, region="back-panel", material="hdf-back"))

    for k, h in enumerate(SHELF_HEIGHTS):
        sid = f"shelf-{k + 1}"
        sol = box(sid, sid, (0.016, 0.492, Y(h), Y(h + 0.016), *SHELF_Z), "foil-chipboard", 0.86, **board)
        body.append(sol)
        if s != 100:
            thc(sol, "interior")
        elif k == 0:
            thc(dict(sol, repeat={"count": 4, "translation": [0, r(SHELF_HEIGHTS[1] - SHELF_HEIGHTS[0]), 0]}), "interior")
        mem(sid, sid)
        if inner:
            ev.append(shape(sid, sid, erect(sid, 0.016, 0.492, Y(h), Y(h + 0.016)), "body", 2.5, True, 0.86, region="interior", material="foil-chipboard"))
    sol = box("hat-shelf", "hat-shelf", (0.508, 1.484, Y(1.66), Y(1.676), *SHELF_Z), "foil-chipboard", 0.86, **board)
    body.append(sol)
    thc(sol, "interior")
    mem("hat-shelf", "hat-shelf")
    if inner:
        ev.append(shape("hat-shelf", "hat-shelf", erect("hat-shelf", 0.508, 1.484, Y(1.66), Y(1.676)), "body", 2.5, True, 0.86, region="interior", material="foil-chipboard"))

    n = 24
    ring = [[r(0.015 * math.sin(2 * math.pi * i / n)), r(0.02 + 0.0075 * math.cos(2 * math.pi * i / n))] for i in range(n)]
    bore = [[r(0.014 * math.sin(2 * math.pi * i / n)), r(0.02 + 0.0065 * math.cos(2 * math.pi * i / n))] for i in range(n)]
    rail = {"id": "rail", "kind": "extrude", "bone": "rail", "material": "chromed-steel", "role": "body", "tone": 0.35, "profile": ring, "plane": "yz", "from": 0.0, "to": 0.976, "slender": True}
    if s in (50, 100):
        rail["holes"] = [bore]
    body.append(rail)
    thc(rail, "interior")
    mem("rail", "rail")
    if inner:
        ev.append(shape("rail", "rail", erect("rail", 0.508, 1.484, Y(1.585), Y(1.615)), "body", 2.6, True, 0.35, region="interior", material="chromed-steel"))

    if s == 50:
        for x0, sid, bone in ((0.508, "rail-socket-l", "partition"), (1.470, "rail-socket-r", "side-r")):
            sol = box(sid, bone, (x0, x0 + 0.014, Y(1.575), Y(1.625), -0.01, 0.05), "zinc-steel", 0.4, role="detail")
            body.append(sol)
            thc(sol, "interior")
            ev.append(shape(sid, bone, erect(bone, x0, x0 + 0.014, Y(1.575), Y(1.625)), "detail", 2.7, True, 0.4, region="interior", material="zinc-steel"))
        for door, (x0, bone) in {"door-1": (0.016, "side-l"), "door-2": (0.508, "partition"), "door-3": (1.472, "side-r")}.items():
            sid = f"hinge-arm-{door[-1]}"
            hb = box(sid, bone, (x0, x0 + 0.012, Y(HINGE_Y[0] - 0.016), Y(HINGE_Y[0] + 0.016), -0.25, -0.198), "zinc-steel", 0.45, role="detail")
            hb["repeat"] = {"count": 3, "translation": [r(HINGE_Y[1] - HINGE_Y[0]), 0, 0]}
            body.append(hb)
            thc(hb, "interior")
            mem(sid, door, part="joint")

    for door, (x0, x1) in DOORS.items():
        chipped = door == "door-1" and s == 50
        pts = door_pts(door, chipped)
        body.append(extrude(door, door, pts, *DOOR_Z, "foil-chipboard", 0.9, **board))
        mem(door, door)
        geo = epoly(door, pts) if chipped else erect(door, x0, x1, Y(0.062), Y(1.998))
        if s == 500:
            geo = erect(door, {"door-1": 0.0, "door-2": 0.5, "door-3": 1.0}[door], {"door-1": 0.5, "door-2": 1.0, "door-3": 1.5}[door], Y(0.062), Y(2.0))
        for views in (ev, fv, pv):
            views.append(shape(door, door, geo, "body", 4.0, True, 0.9, region="doors", material="foil-chipboard", **sweep(-0.290 if s == 500 else DOOR_Z[0], DOOR_Z[1])))
        if s == 50:
            tx0 = 0.0015 if door == "door-1" else 0.5 if door == "door-2" else 1.0
            tx1 = 0.5 if door == "door-1" else 1.0 if door == "door-2" else 1.4985
            tpts = [(tx0 + 0.018, Y(0.062)), (tx1, Y(0.062)), (tx1, Y(1.998)), (tx0, Y(1.998)), (tx0, Y(0.074)), (tx0 + 0.018, Y(0.074))] if chipped else rect_pts(tx0, tx1, Y(0.062), Y(1.998))
            thermal.append(extrude(f"th-{door}", door, tpts, *DOOR_Z, "foil-chipboard", 0.9, region="doors"))
        else:
            th(door, door, "doors")

    mirror = box("mirror", "door-2", (0.5715, 0.9285, Y(0.30), Y(1.70), -0.270, -0.266), "mirror-glass", 0.7, sheet=True)
    if s == 500:
        thermal.append(dict(mirror, id="th-mirror", region="mirror"))
    else:
        body.append(mirror)
        th("mirror", "mirror", "mirror")
    for views in ((ev, fv, pv) if s != 500 else (ev,)):
        views.append(shape("mirror", "door-2", erect("door-2", 0.5715, 0.9285, Y(0.30), Y(1.70)), "body", 4.5, True, 0.7, region="mirror", material="mirror-glass", **sweep(-0.270, -0.266)))

    if s == 50:
        for door, xc in (("door-2", 0.9635), ("door-3", 1.0365)):
            (ox, oy, oz), _ = FRAME[door]
            u0, v = loc(door, xc, Y(0.936))
            u1, _ = loc(door, xc, Y(1.064))
            ua, _ = loc(door, xc, Y(0.920))
            ub, _ = loc(door, xc, Y(1.080))
            hid = f"handle-{door[-1]}"
            for tag, uu in (("post-a", u0), ("post-b", u1)):
                body.append({"id": f"{hid}-{tag}", "kind": "capsule", "bone": door, "material": "chromed-steel", "role": "body", "tone": 0.3, "p0": [uu, v, wz(door, -0.266)], "p1": [uu, v, wz(door, -0.282)], "r0": 0.005, "r1": 0.005})
                th(f"{hid}-{tag}", f"{hid}-{tag}", "doors")
            body.append({"id": f"{hid}-bar", "kind": "capsule", "bone": door, "material": "chromed-steel", "role": "body", "tone": 0.3, "p0": [ua, v, wz(door, -0.285)], "p1": [ub, v, wz(door, -0.285)], "r0": 0.005, "r1": 0.005})
            th(f"{hid}-bar", f"{hid}-bar", "doors")
            for views in (ev, fv, pv):
                views.append(shape(f"{hid}-bar", door, erect(door, xc - 0.005, xc + 0.005, Y(0.915), Y(1.085)), "body", 5.0, True, 0.3, region="doors", material="chromed-steel", **sweep(-0.290, -0.280)))
                for tag, yc in (("post-a", 0.936), ("post-b", 1.064)):
                    views.append(shape(f"{hid}-{tag}", door, erect(door, xc - 0.005, xc + 0.005, Y(yc - 0.005), Y(yc + 0.005)), "body", 5.0, True, 0.3, region="doors", material="chromed-steel", **sweep(-0.282, -0.266)))
    elif s in (100, 200):
        hw = {100: 0.0075, 200: 0.012}[s]
        hl = 0.080
        for door, xc in (("door-2", 0.9635), ("door-3", 1.0365)):
            hid = f"handle-{door[-1]}"
            body.append(box(hid, door, (xc - hw, xc + hw, Y(1.0 - hl), Y(1.0 + hl), -0.290, -0.266), "chromed-steel", 0.3))
            th(hid, hid, "doors")
            for views in (ev, fv, pv):
                views.append(shape(hid, door, erect(door, xc - hw, xc + hw, Y(1.0 - hl), Y(1.0 + hl)), "body", 5.0, True, 0.3, region="doors", material="chromed-steel", **sweep(-0.290, -0.266)))
    if s in (50, 100):
        ku, kv = loc("door-1", 0.4635, Y(1.0))
        knob = {"id": "knob", "kind": "revolve", "bone": "door-1", "material": "beech-knob", "role": "body", "tone": 0.45, "profile": [[0.0, 0.009], [0.008, 0.009], [0.011, 0.016], [0.02, 0.016], [0.024, 0.011], [0.024, 0.0]], "axis": [0, 0, -1], "origin": [ku, kv, wz("door-1", -0.266)]}
        body.append(knob)
        th("knob", "knob", "doors")
        for views in (ev, fv, pv):
            views.append(shape("knob", "door-1", {"kind": "ellipse", "cx": ku, "cy": kv, "rx": 0.016, "ry": 0.016}, "body", 5.0, True, 0.45, region="doors", material="beech-knob", **sweep(-0.290, -0.266)))
        if s == 50:
            prof = [(0.25, 0.0), (0.29, 0.0), (0.29, 0.03), (0.287, 0.03), (0.287, 0.003), (0.25, 0.003)]
            body.append({"id": "bracket", "kind": "extrude", "bone": "bracket", "material": "zinc-steel", "role": "body", "tone": 0.4, "profile": [[r(v), r(w)] for w, v in prof], "plane": "yz", "from": 0.0, "to": 0.06})
            fv.append(shape("bracket", "bracket", {"kind": "poly", "points": [[r(w), r(Y(2.0) + v)] for w, v in prof], "closed": True}, "body", 3.5, True, 0.4, region="carcass", material="zinc-steel"))
        else:
            body.append(box("bracket", "bracket", (0.72, 0.78, Y(2.0), Y(2.03), 0.275, 0.29), "zinc-steel", 0.4))
            fv.append(shape("bracket", "bracket", frect(0.275, 0.29, Y(2.0), Y(2.03)), "body", 3.5, True, 0.4, region="carcass", material="zinc-steel"))
        th("bracket", "bracket", "carcass")
        mem("bracket", "bracket")
        ev.append(shape("bracket", "bracket", erect("bracket", 0.72, 0.78, Y(2.0), Y(2.03)), "body", 1.5, True, 0.4, region="carcass", material="zinc-steel"))
        pv.append(shape("bracket", "bracket", prect(0.72, 0.78, 0.25 if s == 50 else 0.275, 0.29), "body", 3.5, True, 0.4, region="carcass", material="zinc-steel"))

    bl = box("blanket", "blanket", (0.004, 1.496, 0.0, LIFT, -0.24, 0.28), "moving-blanket", 0.3, **({"sheet": True} if s != 50 else {}))
    body.append(bl)
    th("blanket", "blanket", "blanket")
    mem("blanket", "blanket")
    bh, bz = (0.025, 3.5) if s == 500 else (LIFT, 1.2)
    ev.append(shape("blanket", "blanket", erect("blanket", 0.004, 1.496, 0.0, bh), "body", bz, True, 0.3, region="blanket", material="moving-blanket"))
    fv.append(shape("blanket", "blanket", frect(-0.24, 0.28, 0.0, bh), "body", 2.0 if s != 500 else 3.5, True, 0.3, region="blanket", material="moving-blanket"))
    pv.append(shape("blanket", "blanket", prect(0.004, 1.496, -0.24, 0.28), "body", 1.0, True, 0.3, region="blanket", material="moving-blanket"))

    if tex:
        body.append(box("sticker", "side-r", (1.5, 1.5005, Y(1.10), Y(1.16), 0.05, 0.13), "foil-chipboard", 0.2, role="texture", texture=True))
        fv.append(shape("sticker", "side-r", frect(0.05, 0.13, Y(1.10), Y(1.16)), "texture", 3.6, True, 0.35))
        fv.append(shape("sticker-tear", "side-r", {"kind": "poly", "points": [[0.05, Y(1.13)], [0.07, Y(1.145)], [0.09, Y(1.125)], [0.11, Y(1.15)], [0.13, Y(1.14)]], "closed": False}, "texture", 3.7, False, 0.1))
        fv.append(shape("scratch-side", "side-r", {"kind": "poly", "points": [[-0.20, Y(0.55)], [-0.05, Y(0.62)], [0.12, Y(0.66)]], "closed": False}, "texture", 3.6, False, 0.2))
        for k, (x0, x1, y) in enumerate(((0.06, 0.16, 0.022), (0.32, 0.38, 0.035), (0.71, 0.84, 0.018), (1.12, 1.19, 0.03))):
            ev.append(shape(f"plinth-scuff-{k + 1}", "plinth", {"kind": "poly", "points": [list(loc("plinth", x0, Y(y))), list(loc("plinth", x1, Y(y + 0.006)))], "closed": False}, "texture", 3.2, False, 0.2))
        for k, (pa, pb) in enumerate((((1.08, 0.30), (1.21, 0.36)), ((1.15, 0.22), (1.30, 0.24)), ((0.10, 0.25), (0.20, 0.32)))):
            d = "door-1" if pa[0] < 0.5 else "door-3"
            ev.append(shape(f"door-scratch-{k + 1}", d, sliver(d, pa, pb), "detail", 4.2, True, 0.35, **sweep(-0.2661, -0.266)))
        ev.append(shape("foil-peel", "door-1", epoly("door-1", [(0.30, Y(1.998)), (0.33, Y(1.984)), (0.37, Y(1.988)), (0.40, Y(1.998))]), "texture", 4.2, True, 0.95, **sweep(-0.2661, -0.266)))
        for k, yy in enumerate((0.936, 1.064)):
            hu, hv = loc("door-1", 0.4635, Y(yy))
            ev.append(shape(f"old-screw-hole-{k + 1}", "door-1", {"kind": "ellipse", "cx": hu, "cy": hv, "rx": 0.0025, "ry": 0.0025}, "detail", 4.3, True, 0.15, **sweep(-0.2661, -0.266)))
        for k, (x, y) in enumerate(((0.60, 0.34), (0.63, 0.36), (0.88, 0.33), (0.91, 0.39), (0.59, 1.66))):
            mu, mv = loc("door-2", x, Y(y))
            ev.append(shape(f"desilvering-{k + 1}", "door-2", {"kind": "ellipse", "cx": mu, "cy": mv, "rx": 0.006, "ry": 0.004}, "detail", 4.6, True, 0.35, **sweep(-0.2701, -0.270)))
        for k, (pa, pb) in enumerate((((0.9285, 1.62), (0.89, 1.58)), ((0.89, 1.58), (0.875, 1.52)))):
            ev.append(shape(f"mirror-crack-{k + 1}", "door-2", sliver("door-2", pa, pb), "detail", 4.6, True, 0.2, **sweep(-0.2701, -0.270)))
        for k, z in enumerate((-0.15, 0.0, 0.15)):
            pv.append(shape(f"dust-edge-{k + 1}", "top", {"kind": "poly", "points": [[0.10 + 0.4 * k, z], [0.30 + 0.4 * k, z + 0.03]], "closed": False}, "texture", 3.2, False, 0.3))
    if s == 100:
        for k, (x0, x1, y) in enumerate(((0.06, 0.18, 0.025), (0.70, 0.86, 0.02))):
            ev.append(shape(f"plinth-scuff-{k + 1}", "plinth", {"kind": "poly", "points": [list(loc("plinth", x0, Y(y))), list(loc("plinth", x1, Y(y + 0.006)))], "closed": False}, "texture", 3.2, False, 0.2))

    supports = [
        ("foot-left", "carcass", 0.008, 0.0, 0.0185, (max(0.016, f), f, 0.5)),
        ("foot-right", "carcass", 1.492, 0.0, 0.0185, (max(0.016, f), f, 0.5)),
        ("grip-left", "carcass", 0.0, 0.0, 0.0185, (f, f, f)),
        ("grip-right", "carcass", 1.5, 0.0, 0.0185, (f, f, f)),
        ("blanket-floor", "blanket", 0.746, 0.0, 0.02, (0.6, max(f, 0.006) if s != 50 else 0.006, 0.4)),
        ("wall-restraint", "bracket", 0.03, 0.015, 0.29, (max(f, 0.01), max(f, 0.01), max(f, 0.01))),
    ]
    for aid, bone, ax, ay, az, (sx, sy, sz) in [row for row in supports if row[0] == "blanket-floor"]:
        (ox, oy, oz), _ = FRAME[bone]
        cx = ax + (sx / 2 if aid == "grip-left" else -sx / 2 if aid == "grip-right" else 0)
        cy = ay + sy / 2 if aid != "wall-restraint" else ay
        cz = az - sz / 2 if aid == "wall-restraint" else az
        struct.append({"id": f"support-{aid}", "kind": "box", "bone": bone, "material": "zinc-steel" if aid == "wall-restraint" else "foil-chipboard", "role": "ground", "tone": 0.2, "size": [r(sx), r(sy), r(sz)], "transform": {"translation": [r(cx), r(cy), r(cz)]}, "member": bone, "part": "support", "anchor": aid})
    for sh in fv:
        if sh["bone"].startswith("door-"):
            sh["z"] = round(4.0 + 0.1 * int(sh["bone"][-1]) + (0 if sh["id"] == sh["bone"] else 0.05), 2)
    mr = max(f, 0.03)
    struct.append({"id": "mass-carcass", "kind": "ellipsoid", "bone": "carcass", "material": "foil-chipboard", "role": "detail", "tone": 0.3, "radii": [mr, mr, mr], "transform": {"translation": [0.75, 1.02, 0.02]}, "member": "carcass", "part": "mass"})
    return body, thermal, struct, ev, fv, pv


def solid_volume(sol):
    """⚖️ Volume of a solid in cubic metres (boxes, extrusions with holes, capsules, the revolved knob)."""
    rep = sol.get("repeat", {}).get("count", 1)
    k = sol["kind"]
    if k == "box":
        a, b, c = sol["size"]
        return a * b * c * rep
    if k == "extrude":
        def area(p):
            return abs(sum(p[i][0] * p[(i + 1) % len(p)][1] - p[(i + 1) % len(p)][0] * p[i][1] for i in range(len(p)))) / 2
        return (area(sol["profile"]) - sum(area(h) for h in sol.get("holes", []))) * abs(sol["to"] - sol["from"]) * rep
    if k == "capsule":
        p0, p1 = sol["p0"], sol["p1"]
        L = math.dist(p0, p1)
        return (math.pi * sol["r0"] ** 2 * L + 4 / 3 * math.pi * sol["r0"] ** 3) * rep
    if k == "revolve":
        p = sol["profile"]
        return sum(math.pi * (p[i + 1][0] - p[i][0]) * (p[i][1] ** 2 + p[i][1] * p[i + 1][1] + p[i + 1][1] ** 2) / 3 for i in range(len(p) - 1)) * rep
    raise ValueError(k)


def solid_centre(sol):
    """🎯 The local (u, v) centre of a solid."""
    k = sol["kind"]
    if k == "box":
        t = sol["transform"]["translation"]
        base = (t[0], t[1])
    elif k == "extrude" and sol["plane"] == "xy":
        p = sol["profile"]
        base = (sum(q[0] for q in p) / len(p), sum(q[1] for q in p) / len(p))
    elif k == "extrude":
        base = ((sol["from"] + sol["to"]) / 2, sum(q[0] for q in sol["profile"]) / len(sol["profile"]))
    elif k == "capsule":
        base = ((sol["p0"][0] + sol["p1"][0]) / 2, (sol["p0"][1] + sol["p1"][1]) / 2)
    else:
        base = (sol["origin"][0], sol["origin"][1])
    rep = sol.get("repeat")
    if rep:
        n, t = rep["count"], rep.get("translation", [0, 0, 0])
        base = (base[0] + t[0] * (n - 1) / 2, base[1] + t[1] * (n - 1) / 2)
    return base


BODY50 = parts(50)[0]
MASS = {}
for sol in BODY50:
    if sol.get("texture"):
        continue
    kg = solid_volume(sol) * DENSITY[sol["material"]]
    u, v = solid_centre(sol)
    m, su, sv = MASS.get(sol["bone"], (0, 0, 0))
    MASS[sol["bone"]] = (m + kg, su + kg * u, sv + kg * v)
TOTAL_KG = round(sum(m for m, _, _ in MASS.values()), 1)

SECTIONS = {
    "side-l": (0.537, 0.016), "side-r": (0.537, 0.016), "partition": (0.537, 0.016), "top": (0.537, 0.016), "bottom": (0.537, 0.016),
    "plinth": (0.016, 0.060), "hat-shelf": (0.52, 0.016), "door-1": (0.497, 0.016), "door-2": (0.497, 0.016), "door-3": (0.497, 0.016),
    **{f"shelf-{k + 1}": (0.52, 0.016) for k in range(4)},
}


def bone_json(bid, parent, x, y, z, ang, length, joint, title):
    """🦴 One bone with its mass from the 1:50 solids and its member section."""
    b = {"id": bid, "parent": parent, "title": title, "x": r(x), "y": r(y)}
    if z:
        b["z"] = r(z)
    b.update({"angleDeg": ang, "lengthM": length, "joint": joint})
    if bid in MASS:
        m, su, sv = MASS[bid]
        b["mass"] = {"kg": round(m, 3), "comX": round(su / m, 4), "comY": round(sv / m, 4), "distribution": "point", "source": "authored-estimate", "reference": "en-312" if m > 1 else "iso-10456-2007"}
    if bid in SECTIONS:
        bw, bh = SECTIONS[bid]
        mat = "foil-chipboard"
        b["section"] = {"areaM2": r(bw * bh), "secondMomentM4": float(f"{bw * bh ** 3 / 12:.4e}"), "extremeFibreM": r(bh / 2), "material": mat, "title": T(f"Board {round(bw * 1000)} x {round(bh * 1000)} mm", f"Platte {round(bw * 1000)} x {round(bh * 1000)} mm")}
    if bid == "rail":
        a = math.pi * (0.015 * 0.0075 - 0.014 * 0.0065)
        i = math.pi / 4 * (0.0075 * 0.015 ** 3 - 0.0065 * 0.014 ** 3)
        b["section"] = {"areaM2": float(f"{a:.4e}"), "secondMomentM4": float(f"{i:.4e}"), "extremeFibreM": 0.015, "material": "chromed-steel", "title": T("Oval tube 30 x 15 x 1 mm", "Ovalrohr 30 x 15 x 1 mm")}
    return b


ANCHORS = [
    {"id": "foot-left", "bone": "carcass", "x": 0.008, "y": 0.0, "z": 0.0185, "kind": "contact", "title": T("Left side panel on its base", "Linke Seitenwand auf ihrer Unterlage"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.016, "footprintWidthM": 0.537, "frictionCoefficient": 0.5},
    {"id": "foot-right", "bone": "carcass", "x": 1.492, "y": 0.0, "z": 0.0185, "kind": "contact", "title": T("Right side panel on its base", "Rechte Seitenwand auf ihrer Unterlage"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.016, "footprintWidthM": 0.537, "frictionCoefficient": 0.5},
    {"id": "grip-left", "bone": "carcass", "x": 0.0, "y": 0.0, "z": 0.0185, "kind": "support", "title": T("First mover's hands under the left bottom edge", "Hände des ersten Trägers unter der linken Unterkante"), "support": "pin", "footprintLengthM": 0.1, "footprintWidthM": 0.3},
    {"id": "grip-right", "bone": "carcass", "x": 1.5, "y": 0.0, "z": 0.0185, "kind": "support", "title": T("Second mover's hands under the right bottom edge", "Hände des zweiten Trägers unter der rechten Unterkante"), "support": "pin", "footprintLengthM": 0.1, "footprintWidthM": 0.3},
    {"id": "blanket-floor", "bone": "blanket", "x": 0.746, "y": 0.0, "z": 0.02, "kind": "contact", "title": T("Blanket on the floor", "Decke auf dem Boden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 1.492, "footprintWidthM": 0.52, "frictionCoefficient": 0.6},
    {"id": "wall-restraint", "bone": "bracket", "x": 0.03, "y": 0.015, "z": 0.29, "kind": "support", "title": T("Restraint angle screwed to the wall", "Kippschutzwinkel an die Wand geschraubt"), "support": "roller", "normalDeg": 90, "tiltDeg": 90, "footprintLengthM": 0.06, "footprintWidthM": 0.03},
]


def binding(over=None, interior=0.0, back=1.0, blanket=None):
    """🌡 The thermal binding of a pose: every region, no emitters."""
    regions = {g: {"exposedFraction": 1.0, "sunlitFraction": 0.0} for g in REGION_IDS}
    regions["doors"]["sunlitFraction"] = 0.2
    regions["mirror"]["sunlitFraction"] = 0.2
    regions["interior"]["exposedFraction"] = interior
    regions["back-panel"]["exposedFraction"] = back
    regions["blanket"] = blanket or {"exposedFraction": 0.15, "sunlitFraction": 0.0, "contact": {"fraction": 0.85, "tempC": 18}}
    if over:
        regions.update(over)
    return {"regions": regions, "emitters": {}}


def dist_load(lid, bone, length, kg_per_m, title):
    """🧺 A distributed gravity load of stored clothes on a board or the rail."""
    return {"id": lid, "kind": "carried", "title": title, "distributed": {"bone": bone, "fromM": 0, "toM": length, "wxNPerM": 0, "wyNPerM": round(-kg_per_m * 9.80665, 1)}, "fxN": 0, "fyN": 0, "source": "authored-estimate", "reference": "en-16122"}


IN_USE_LOADS = [dist_load("clothes-on-rail", "rail", 0.976, 15.0, T("Hanging clothes on the rail, 15 kg/m", "Hängende Kleidung an der Stange, 15 kg/m")), dist_load("boxes-on-hat-shelf", "hat-shelf", 0.976, 8.0, T("Boxes on the hat shelf, 8 kg", "Kartons auf dem Hutboden, 8 kg"))] + [dist_load(f"linen-on-shelf-{k + 1}", f"shelf-{k + 1}", 0.476, 21.0, T(f"Folded linen on shelf {k + 1}, 10 kg", f"Gefaltete Wäsche auf Fachboden {k + 1}, 10 kg")) for k in range(4)]
HOME = {"blanket": {"exposedFraction": 0.6, "sunlitFraction": 0.0, "contact": {"fraction": 0.4, "tempC": 20}}}
IN_USE_BONES = {"lift": {"dy": -LIFT}, "blanket": {"dx": -1.62}}
OPEN = {"door-1": {"twistDeg": -95}, "door-2": {"twistDeg": -95}, "door-3": {"twistDeg": 95}}

POSES = [
    {"id": "stored", "title": T("Stored at the reuse depot", "Gelagert im Gebrauchtwarenlager"), "description": T("Emptied, cleaned and standing on a folded moving blanket in the depot aisle; the tipping restraint angle is still screwed on the top, its wall screw torn out at the flat clearance.", "Geleert, gereinigt und auf einer gefalteten Umzugsdecke im Lagergang stehend; der Kippschutzwinkel ist noch auf den Oberboden geschraubt, seine Wandschraube bei der Haushaltsauflösung ausgerissen."), "bones": {}, "ground": "authored", "contacts": ["foot-left", "foot-right", "blanket-floor"], "thermal": binding(), "loads": []},
    {"id": "carried", "title": T("Carried by two movers", "Von zwei Trägern getragen"), "description": T("Lifted 0.10 m off the blanket by two movers under the bottom edges of the side panels and tilted 5 degrees as on the first step of a stair; the blanket stays on the floor.", "Von zwei Trägern unter den Unterkanten der Seitenwände 0,10 m von der Decke gehoben und wie auf der ersten Treppenstufe um 5 Grad geneigt; die Decke bleibt auf dem Boden."), "bones": {"lift": {"dy": 0.10}, "carcass": {"angleDeg": 5}}, "ground": "authored", "contacts": ["grip-left", "grip-right", "blanket-floor"], "thermal": binding(blanket={"exposedFraction": 0.5, "sunlitFraction": 0.0, "contact": {"fraction": 0.5, "tempC": 18}}), "loads": []},
    {"id": "in-use", "title": T("In use in a bedroom", "In Gebrauch im Schlafzimmer"), "description": T("Set up again in a bedroom on the floor, the restraint angle screwed to the wall, clothes on the rail, linen on the shelves and boxes on the hat shelf; the moving blanket lies folded beside it.", "Im Schlafzimmer wieder auf dem Boden aufgestellt, der Kippschutzwinkel an die Wand geschraubt, Kleidung an der Stange, Wäsche in den Fächern und Kartons auf dem Hutboden; die Umzugsdecke liegt gefaltet daneben."), "bones": IN_USE_BONES, "ground": "authored", "contacts": ["foot-left", "foot-right", "blanket-floor", "wall-restraint"], "thermal": binding(over=HOME, back=0.3), "loads": IN_USE_LOADS},
    {"id": "doors-open", "title": T("In use with all doors open", "In Gebrauch mit allen Türen offen"), "description": T("The three doors swung open by 95 degrees on their cup hinges: the shelves of the left compartment and the hanging space are open; the wall angle holds the carcass against tipping forward.", "Die drei Türen auf ihren Topfbändern um 95 Grad geöffnet: die Fächer links und der Hängeraum liegen offen; der Wandwinkel hält den Korpus gegen das Vornüberkippen."), "bones": {**IN_USE_BONES, **OPEN}, "ground": "authored", "contacts": ["foot-left", "foot-right", "blanket-floor", "wall-restraint"], "thermal": binding(over=HOME, interior=0.6, back=0.3), "loads": IN_USE_LOADS},
]

CLIPS = [
    {"id": "carry", "title": T("Carry out of the depot", "Aus dem Lager tragen"), "description": T("The two movers take hold under the side panels, lift the wardrobe off the blanket and tilt it 5 degrees as they reach the stair; the blanket stays behind.", "Die zwei Träger fassen unter die Seitenwände, heben den Schrank von der Decke und neigen ihn um 5 Grad, als sie die Treppe erreichen; die Decke bleibt liegen."), "durationS": 6.0, "fps": 25, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1.0, "pose": "stored", "ease": "smooth"}, {"t": 3.0, "pose": "carried", "ease": "smooth", "bones": {"carcass": {"angleDeg": 0}}}, {"t": 5.0, "pose": "carried", "ease": "smooth"}, {"t": 6.0, "pose": "carried", "ease": "smooth"}],
     "contacts": [{"anchor": "foot-left", "fromS": 0, "toS": 1.0}, {"anchor": "foot-right", "fromS": 0, "toS": 1.0}, {"anchor": "grip-left", "fromS": 1.0, "toS": 6.0}, {"anchor": "grip-right", "fromS": 1.0, "toS": 6.0}, {"anchor": "blanket-floor", "fromS": 0, "toS": 6.0}],
     "phases": [{"id": "take-hold", "fromS": 0, "toS": 1.0, "title": T("Taking hold", "Anfassen")}, {"id": "lift", "fromS": 1.0, "toS": 3.0, "title": T("Lifting off the blanket", "Von der Decke heben")}, {"id": "tilt", "fromS": 3.0, "toS": 5.0, "title": T("Tilting for the stair", "Für die Treppe neigen")}, {"id": "carry", "fromS": 5.0, "toS": 6.0, "title": T("Carrying", "Tragen")}],
     "events": [{"t": 1.0, "kind": "contact-off", "title": T("Off the blanket", "Von der Decke"), "target": "foot-left"}, {"t": 1.0, "kind": "contact-on", "title": T("Hands take the load", "Hände übernehmen die Last"), "target": "grip-left"}, {"t": 3.0, "kind": "motion", "title": T("Lifted 0.10 m", "0,10 m angehoben")}],
     "tracks": []},
    {"id": "door-cycle", "title": T("Open and close the doors", "Türen öffnen und schließen"), "description": T("In the bedroom the right door and then the mirror door swing open by 100 degrees, stay open while clothes are taken and close again in reverse order.", "Im Schlafzimmer schwingen erst die rechte Tür und dann die Spiegeltür um 100 Grad auf, bleiben offen, während Kleidung entnommen wird, und schließen in umgekehrter Reihenfolge."), "durationS": 8.0, "fps": 25, "loop": True, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "in-use", "ease": "smooth"}, {"t": 1.5, "pose": "in-use", "ease": "smooth", "bones": {"door-3": {"twistDeg": 100}}}, {"t": 3.0, "pose": "in-use", "ease": "smooth", "bones": {"door-3": {"twistDeg": 100}, "door-2": {"twistDeg": -100}}}, {"t": 5.0, "pose": "in-use", "ease": "smooth", "bones": {"door-3": {"twistDeg": 100}, "door-2": {"twistDeg": -100}}}, {"t": 6.5, "pose": "in-use", "ease": "smooth", "bones": {"door-3": {"twistDeg": 100}}}, {"t": 8.0, "pose": "in-use", "ease": "smooth"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 8.0} for a in ("foot-left", "foot-right", "blanket-floor", "wall-restraint")],
     "phases": [{"id": "opening", "fromS": 0, "toS": 3.0, "title": T("Opening", "Öffnen")}, {"id": "open", "fromS": 3.0, "toS": 5.0, "title": T("Open", "Offen")}, {"id": "closing", "fromS": 5.0, "toS": 8.0, "title": T("Closing", "Schließen")}],
     "events": [{"t": 0.0, "kind": "motion", "title": T("Right door opens", "Rechte Tür öffnet"), "target": "door-3"}, {"t": 1.5, "kind": "motion", "title": T("Mirror door opens", "Spiegeltür öffnet"), "target": "door-2"}, {"t": 5.0, "kind": "motion", "title": T("Mirror door closes", "Spiegeltür schließt"), "target": "door-2"}, {"t": 6.5, "kind": "motion", "title": T("Right door closes", "Rechte Tür schließt"), "target": "door-3"}],
     "tracks": [{"target": "region:interior:exposedFraction", "ease": "smooth", "keys": [{"t": 0, "v": 0.0}, {"t": 1.5, "v": 0.3}, {"t": 3.0, "v": 0.6}, {"t": 5.0, "v": 0.6}, {"t": 6.5, "v": 0.3}, {"t": 8.0, "v": 0.0}]}]},
]

REFERENCES = [
    {"id": "en-312", "citation": "EN 312:2010 Particleboards - Specifications (type P2, boards for interior fitments including furniture in dry conditions: 16 mm, bending strength 11.5 N/mm2, modulus of elasticity 1600 N/mm2 minimum, typical mean 2300 N/mm2; density about 650 kg/m3)."},
    {"id": "en-622-5", "citation": "EN 622-5:2009 Fibreboards - Specifications - Part 5: Requirements for dry process boards (MDF/HDF); 3 mm HDF back panels about 850 kg/m3."},
    {"id": "en-572-2", "citation": "EN 572-2:2012 Glass in building - Basic soda lime silicate glass products - Part 2: Float glass (density 2500 kg/m3, E 70 GPa); EN 1036-1 silvered float glass mirrors for internal use."},
    {"id": "en-14749", "citation": "EN 14749:2016 Furniture - Domestic and kitchen storage units and worktops - Safety requirements and test methods (stability of units over 600 mm high, the requirement to supply wall fixings for tall units, loads on shelves and clothes rails)."},
    {"id": "en-16122", "citation": "EN 16122:2012 Domestic and non-domestic storage furniture - Test methods for the determination of strength, durability and stability (shelf, hat-shelf and clothes-rail load tests, door slam and door overload tests, overturning with extension elements open)."},
    {"id": "din-68871", "citation": "DIN 68871:2018 Möbel - Begriffe (hinged-door wardrobe, plinth, hat shelf, clothes rail as terms) and the usual German furniture-trade dimensions of a three-door wardrobe 150 x 58 x 200 cm."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (particleboard 600-650 kg/m3 0.13-0.14 W/mK, fibreboard, hardwood 0.16-0.18 W/mK, steel 50 W/mK, glass 1.0 W/mK, specific heat 1700 J/kgK for wood-based boards)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivity of paints, plastics, glass, polished and plated metals; aged and dusty surfaces)."},
    {"id": "blum-clip", "citation": "Blum CLIP top cup hinge catalogue data (35 mm cup, 11.5 mm deep, 110 degree opening angle, hinge arm on a cruciform mounting plate; three hinges for doors of 1.6-2.0 m and up to 12 kg after the manufacturer's hinge-count chart), taken as the type of the concealed hinges."},
    {"id": "used-furniture-market", "citation": "Catalogue-typical flat-pack three-door wardrobes of the German mid-range trade 2000-2015 (white foiled chipboard, 16 mm carcass, middle mirror door, bar handles at 128 mm hole spacing, 100-115 kg packed) and the intake practice of municipal second-hand department stores (Gebrauchtwarenkaufhaus): emptied, cleaned, stored upright on moving blankets; no brand data used."},
]


def scale_decisions(s):
    """📝 The bilingual design decisions of the 2D representation of 1:s."""
    D = {
        50: dict(
            silhouette=T("Full construction at 30 x 41 mm: the two side panels to the floor, the top, the middle partition, the recessed plinth board with its broken-out left corner, three 497 mm doors with 3 mm gaps, the mirror on the middle door, the bar handles, the beech knob that replaces the lost handle of the left door, the restraint angle on the top and the folded blanket under the carcass.", "Volle Konstruktion auf 30 x 41 mm: die zwei Seitenwände bis zum Boden, der Oberboden, die Mittelwand, die zurückgesetzte Sockelblende mit ausgebrochener linker Ecke, drei 497-mm-Türen mit 3 mm Fugen, der Spiegel auf der Mitteltür, die Bügelgriffe, der Buchenknopf als Ersatz für den verlorenen Griff der linken Tür, der Kippschutzwinkel auf dem Oberboden und die gefaltete Decke unter dem Korpus."),
            retained=T("Every board at its 16 mm stock, the 3 mm HDF back, the 18 x 12 mm chip of the left door, the hat shelf, rail and four shelves behind the doors (shown when a door opens), the hinge arms and rail sockets in 3D.", "Jede Platte in ihrer Stärke von 16 mm, die 3-mm-HDF-Rückwand, die 18 x 12 mm Ausbruchstelle der linken Tür, Hutboden, Stange und vier Fachböden hinter den Türen (sichtbar, wenn eine Tür aufgeht), die Scharnierarme und Stangenhalter im 3D."),
            merged=T("Each board is one shape; the edge banding is not drawn apart from the board.", "Jede Platte ist eine Form; die Kantenumleimer sind nicht von der Platte getrennt gezeichnet."),
            omitted=T("Shelf pins (5 mm), cam fittings and the screws of the hinges.", "Bodenträger (5 mm), Exzenterverbinder und die Schrauben der Scharniere."),
            thinElements=T("The 3 mm back panel and the 4 mm mirror keep their true thickness as thin filled strips in front view and plan; the restraint angle is drawn as its 3 mm L section in the front view.", "Die 3-mm-Rückwand und der 4-mm-Spiegel behalten ihre wahre Stärke als dünne gefüllte Streifen in Vorderansicht und Grundriss; der Kippschutzwinkel ist in der Vorderansicht als sein 3-mm-L-Profil gezeichnet."),
            hatching=T("No hatches: the boards read by tone, the blanket by its darker tone.", "Keine Schraffuren: die Platten lesen sich über den Ton, die Decke über ihren dunkleren Ton."),
            styles=T("Body pen 0.5 mm for the panels, texture pen 0.13 mm for the traces of use: scuffs on the plinth, scratches on the doors, the peeling foil at the top edge of the left door, the two filled screw holes of the lost handle, desilvered spots and a corner crack in the mirror, the half-torn sticker on the right side panel and dust lines on the top.", "Körperstift 0,5 mm für die Platten, Texturstift 0,13 mm für die Gebrauchsspuren: Schrammen am Sockel, Kratzer an den Türen, die sich lösende Folie an der Oberkante der linken Tür, die zwei Schraublöcher des verlorenen Griffs, blinde Stellen und ein Eckriss im Spiegel, der halb abgerissene Aufkleber an der rechten Seitenwand und Staublinien auf dem Oberboden."),
            overlays=T("Analysis views colour the same shapes by region and by member; the doors are drawn as swept solids so that every view follows them when they open.", "Die Analyseansichten färben dieselben Formen nach Region und Bauteil; die Türen sind als gezogene Körper gezeichnet, damit ihnen jede Ansicht beim Öffnen folgt.")),
        100: dict(
            silhouette=T("Wardrobe at 15 x 20 mm: panels, plinth, partition, doors, mirror, handles as 15 mm bars, the knob, the restraint angle as a 15 mm block and the blanket.", "Schrank auf 15 x 20 mm: Platten, Sockel, Mittelwand, Türen, Spiegel, Griffe als 15-mm-Stäbe, der Knopf, der Kippschutzwinkel als 15-mm-Block und die Decke."),
            retained=T("All boards at 16 mm, the interior behind the doors, the recessed plinth (20 mm).", "Alle Platten mit 16 mm, der Innenraum hinter den Türen, der zurückgesetzte Sockel (20 mm)."),
            merged=T("Handle posts and bar into one 15 mm bar; the angle's two legs into one block.", "Griffstützen und Bügel zu einem 15-mm-Stab; die zwei Schenkel des Winkels zu einem Block."),
            omitted=T("The chip of the left door and of the plinth (under 15 mm), hinge arms, rail sockets, the sticker, screw holes and mirror spots.", "Die Ausbrüche an linker Tür und Sockel (unter 15 mm), Scharnierarme, Stangenhalter, der Aufkleber, Schraublöcher und Spiegelflecken."),
            thinElements=T("Back panel and mirror stay thin strips; the rail is drawn as its 30 mm oval height.", "Rückwand und Spiegel bleiben dünne Streifen; die Stange ist mit ihrer ovalen Höhe von 30 mm gezeichnet."),
            hatching=T("No hatches.", "Keine Schraffuren."),
            styles=T("Body pen 0.35 mm, texture pen 0.13 mm for two scuffs on the plinth.", "Körperstift 0,35 mm, Texturstift 0,13 mm für zwei Schrammen am Sockel."),
            overlays=T("As at 1:50.", "Wie in 1:50.")),
        200: dict(
            silhouette=T("Wardrobe sign 7.5 x 10 mm: one carcass frame, the three door leaves, the mirror, the plinth band and the blanket line.", "Schrankzeichen 7,5 x 10 mm: ein Korpusrahmen, die drei Türblätter, der Spiegel, das Sockelband und die Deckenlinie."),
            retained=T("Overall 1.50 x 2.04 m, three doors with their gaps, the mirror, the interior boards behind the doors.", "Gesamtmaß 1,50 x 2,04 m, drei Türen mit ihren Fugen, der Spiegel, die Innenplatten hinter den Türen."),
            merged=T("Sides, top, bottom and plinth zone are one frame; in front view and plan the back panel is part of the carcass.", "Seiten, Oberboden, Boden und Sockelzone sind ein Rahmen; in Vorderansicht und Grundriss gehört die Rückwand zum Korpus."),
            omitted=T("Handles, knob, restraint angle and all traces of use.", "Griffe, Knopf, Kippschutzwinkel und alle Gebrauchsspuren."),
            thinElements=T("Doors, partition and shelves are drawn as their true 16 mm boards.", "Türen, Mittelwand und Fachböden sind als ihre wahren 16-mm-Platten gezeichnet."),
            hatching=T("No hatches.", "Keine Schraffuren."),
            styles=T("Body pen 0.25 mm.", "Körperstift 0,25 mm."),
            overlays=T("As at 1:50.", "Wie in 1:50.")),
        500: dict(
            silhouette=T("Wardrobe sign 3 x 4 mm: the carcass frame with three doors and the plinth band.", "Schrankzeichen 3 x 4 mm: der Korpusrahmen mit drei Türen und dem Sockelband."),
            retained=T("Outline 1.50 x 2.04 m, the three doors, the mirror field, the blanket line.", "Umriss 1,50 x 2,04 m, die drei Türen, das Spiegelfeld, die Deckenlinie."),
            merged=T("Carcass, plinth zone and back into one frame and block; the mirror is a field of the middle door.", "Korpus, Sockelzone und Rückwand zu einem Rahmen und Block; der Spiegel ist ein Feld der Mitteltür."),
            omitted=T("Handles, knob, angle, hinges and traces of use.", "Griffe, Knopf, Winkel, Scharniere und Gebrauchsspuren."),
            thinElements=T("Doors and interior boards keep their 16 mm stock as lines of the sign.", "Türen und Innenplatten behalten ihre 16 mm Stärke als Linien des Zeichens."),
            hatching=T("No hatches.", "Keine Schraffuren."),
            styles=T("Body pen 0.18 mm.", "Körperstift 0,18 mm."),
            overlays=T("As at 1:50.", "Wie in 1:50.")),
    }
    return D[s]


VOLUME_DECISIONS = {
    50: [T("Body 1:50 is the wardrobe as built and as used: 16 mm side panels standing on the blanket, top and bottom between them, the middle partition, the 60 mm plinth board set back 20 mm with a 28 x 10 mm piece broken out of its left corner, the 3 mm HDF back over the rear edges, four shelves left and the hat shelf right, the oval 30 x 15 x 1 mm rail as a hollow tube in its two sockets, three cup-hinge arms per door, the three 16 mm doors with the 18 x 12 mm chip at the lower hinge corner of the left door, the 4 mm mirror, two bar handles of 10 mm rod on 128 mm posts, the turned beech knob that replaced the lost handle, the 3 mm restraint angle on the top, the folded blanket and the half-torn sticker (texture).", "Körper 1:50 ist der Schrank, wie er gebaut und benutzt ist: 16-mm-Seitenwände auf der Decke, Oberboden und Boden dazwischen, die Mittelwand, die 60-mm-Sockelblende 20 mm zurückgesetzt mit einem 28 x 10 mm Ausbruch an der linken Ecke, die 3-mm-HDF-Rückwand über den Hinterkanten, vier Fachböden links und der Hutboden rechts, die ovale Stange 30 x 15 x 1 mm als hohles Rohr in ihren zwei Haltern, drei Topfbandarme je Tür, die drei 16-mm-Türen mit dem 18 x 12 mm Ausbruch an der unteren Bandecke der linken Tür, der 4-mm-Spiegel, zwei Bügelgriffe aus 10-mm-Rundstab auf 128-mm-Stützen, der gedrechselte Buchenknopf als Ersatz des verlorenen Griffs, der 3-mm-Kippschutzwinkel auf dem Oberboden, die gefaltete Decke und der halb abgerissene Aufkleber (Textur)."),
         T("Thermal 1:50 references the body solids by region; the doors are their own thermal solids that meet at the 3 mm gaps, so the closed carcass is a closed cavity and the interior is a hidden region. Structure 1:50: every board is its own member on its bone, the hinge arms are the joints of the doors, six support pads stand at the feet, the grips, the blanket and the wall angle.", "Thermik 1:50 verweist nach Regionen auf die Körper; die Türen sind eigene thermische Körper, die sich in den 3-mm-Fugen treffen, sodass der geschlossene Korpus ein geschlossener Hohlraum und der Innenraum eine verdeckte Region ist. Tragwerk 1:50: jede Platte ist ihr eigenes Bauteil auf ihrem Knochen, die Scharnierarme sind die Gelenke der Türen, sechs Auflagerklötze stehen an den Füßen, den Griffstellen, der Decke und dem Wandwinkel.")],
    100: [T("Body 1:100 keeps every 16 mm board, the recessed plinth without its chip, the back and the mirror as declared sheets, the rail as a slender hollow tube, the knob, the handles as one 15 mm bar each and the restraint angle as a 60 x 30 x 15 mm block; hinge arms, rail sockets, the door chip and the sticker are below 15 mm and left out.", "Körper 1:100 behält jede 16-mm-Platte, den zurückgesetzten Sockel ohne Ausbruch, Rückwand und Spiegel als erklärte Platten, die Stange als schlankes Hohlrohr, den Knopf, die Griffe als je einen 15-mm-Stab und den Kippschutzwinkel als Block 60 x 30 x 15 mm; Scharnierarme, Stangenhalter, der Türausbruch und der Aufkleber sind unter 15 mm und entfallen."),
          T("Thermal 1:100 references the body; the 3 mm door gaps are below a quarter feature and close the cavity. Structure 1:100: the boards as members, the door members hang on the carcass without hinge joints, six supports.", "Thermik 1:100 verweist auf den Körper; die 3-mm-Türfugen liegen unter einem Viertel Merkmal und schließen den Hohlraum. Tragwerk 1:100: die Platten als Bauteile, die Türen hängen ohne Scharniergelenke am Korpus, sechs Auflager.")],
    200: [T("Body 1:200 merges sides, top, bottom and plinth zone into one carcass frame extruded over the depth (a hollow part judged by its overall extents); the back, the partition, the shelves, the hat shelf, the doors, the mirror and the blanket are declared sheets at their true thickness because the doors open in the clip and show the interior; the rail stays a slender solid oval; handles, knob and angle are below 30 mm and left out.", "Körper 1:200 fasst Seiten, Oberboden, Boden und Sockelzone zu einem über die Tiefe gezogenen Korpusrahmen zusammen (ein Hohlteil, nach seinen Gesamtmaßen beurteilt); Rückwand, Mittelwand, Fachböden, Hutboden, Türen, Spiegel und Decke sind erklärte Platten in wahrer Stärke, weil die Türen im Clip aufgehen und den Innenraum zeigen; die Stange bleibt ein schlankes volles Oval; Griffe, Knopf und Winkel sind unter 30 mm und entfallen."),
          T("Thermal 1:200 references the body and adds the plinth as a 30 mm thermal block in the front of the frame. Structure 1:200: the frame is one member solid for the carcass, side panels, top, bottom and plinth bones; boards, doors and rail are their own members; six supports.", "Thermik 1:200 verweist auf den Körper und ergänzt den Sockel als 30-mm-Thermoblock vorn im Rahmen. Tragwerk 1:200: der Rahmen ist ein Bauteilkörper für Korpus, Seitenwände, Oberboden, Boden und Sockel; Platten, Türen und Stange sind eigene Bauteile; sechs Auflager.")],
    500: [T("Body 1:500 is the carcass frame, the back, the partition, the four shelves and the hat shelf as sheets, the rail, the three door sheets and the blanket; the mirror is no longer a body part but a field drawn on the middle door and a thermal sheet.", "Körper 1:500 ist der Korpusrahmen, die Rückwand, Mittelwand, vier Fachböden und Hutboden als Platten, die Stange, die drei Türplatten und die Decke; der Spiegel ist kein Körperteil mehr, sondern ein auf die Mitteltür gezeichnetes Feld und eine thermische Platte."),
          T("Thermal 1:500: the plinth as a 75 mm block in the frame front, the mirror as its own sheet. Structure 1:500 as 1:200 with 75 mm supports.", "Thermik 1:500: der Sockel als 75-mm-Block vorn im Rahmen, der Spiegel als eigene Platte. Tragwerk 1:500 wie 1:200 mit 75-mm-Auflagern.")],
}

PENS = {50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.7}, 100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5}, 200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35}, 500: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}

representations, volumes = {}, {}
for s in (500, 200, 100, 50):
    body, thermal, struct, ev, fv, pv = parts(s)
    representations[str(s)] = {"status": "authored", "denominator": s, "validRange": {"minDenominator": RANGES[s][0], "maxDenominator": RANGES[s][1]}, "pens": PENS[s], "hatches": [], "geometry": {"elevation": ev, "front": fv, "plan": pv}, "decisions": scale_decisions(s)}
    volumes[str(s)] = {"status": "authored", "denominator": s, "body": body, "thermal": thermal, "structure": struct, "decisions": VOLUME_DECISIONS[s]}

asset = {
    "id": "used-hinged-wardrobe",
    "category": "furniture",
    "title": T("Used Hinged-Door Wardrobe", "Gebrauchter Drehtürenkleiderschrank"),
    "summary": T(f"A second-hand three-door wardrobe 1.50 x 0.58 x 2.00 m of white foiled 16 mm chipboard, {TOTAL_KG} kg with its moving blanket: a shelf compartment behind the left door, a hanging space with rail and hat shelf behind the mirror door and the right door, a scuffed plinth with a broken corner, a chipped left door whose lost handle was replaced by a beech knob, and the tipping restraint angle still on the top; stored on a folded blanket, carried by two movers, back in use with its doors opening.", f"Ein gebrauchter dreitüriger Kleiderschrank 1,50 x 0,58 x 2,00 m aus weiß foliierter 16-mm-Spanplatte, {str(TOTAL_KG).replace('.', ',')} kg mit seiner Umzugsdecke: ein Fachteil hinter der linken Tür, ein Hängeraum mit Stange und Hutboden hinter Spiegeltür und rechter Tür, ein verschrammter Sockel mit ausgebrochener Ecke, eine angeschlagene linke Tür, deren verlorener Griff durch einen Buchenknopf ersetzt ist, und der Kippschutzwinkel noch auf dem Oberboden; gelagert auf einer gefalteten Decke, von zwei Trägern getragen, wieder in Gebrauch mit sich öffnenden Türen."),
    "tags": ["used", "household", "second-hand", "bedroom", "wardrobe", "storage", "chipboard", "mirror", "hinged-door", "interior"],
    "provenance": {"author": "Semio staffage content (used household objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES,
                   "notes": T("Type: German mid-range flat-pack three-door wardrobe of about 2005-2015, 1.50 x 0.58 x 2.00 m, white foiled P2 chipboard 16 mm, carcass depth 537 mm, three full-overlay doors 497 x 1936 mm on concealed cup hinges (three per door), 4 mm mirror 357 x 1400 mm on the middle door, bar handles at 128 mm hole spacing, 3 mm HDF back nailed over the rear edges, plinth board 60 mm set back 20 mm, four shelves 476 x 520 mm left, hat shelf 976 x 520 mm and oval rail 30 x 15 mm right. Masses are the 1:50 solid volumes times the densities of the materials (chipboard 650, HDF 850, glass 2500, steel 7850, beech 720, blanket felt 160 kg/m3). Traces of use are authored from the intake of second-hand department stores; nothing is measured.", "Typ: zerlegbarer dreitüriger Kleiderschrank des deutschen Mittelpreissegments von etwa 2005-2015, 1,50 x 0,58 x 2,00 m, weiß foliierte P2-Spanplatte 16 mm, Korpustiefe 537 mm, drei aufschlagende Türen 497 x 1936 mm auf verdeckten Topfbändern (drei je Tür), 4-mm-Spiegel 357 x 1400 mm auf der Mitteltür, Bügelgriffe mit 128 mm Lochabstand, 3-mm-HDF-Rückwand über die Hinterkanten genagelt, Sockelblende 60 mm 20 mm zurückgesetzt, vier Fachböden 476 x 520 mm links, Hutboden 976 x 520 mm und Ovalstange 30 x 15 mm rechts. Massen sind die Volumen der 1:50-Körper mal den Dichten der Werkstoffe (Spanplatte 650, HDF 850, Glas 2500, Stahl 7850, Buche 720, Deckenfilz 160 kg/m3). Gebrauchsspuren sind nach der Annahme von Gebrauchtwarenkaufhäusern geschätzt; nichts ist gemessen.")},
    "dimensions": {"heightM": 2.042, "widthM": 1.5, "depthM": 0.58, "massKg": TOTAL_KG},
    "materials": MATERIALS,
    "rig": {"kind": "mechanical", "root": "ground", "bones": [bone_json(*b) for b in BONES]},
    "anchors": ANCHORS,
    "representations": representations,
    "symbols": [],
    "poses": POSES,
    "clips": CLIPS,
    "thermal": {
        "environment": {"airTempC": 19, "meanRadiantTempC": 18, "relativeHumidity": 0.5, "windMS": 0.05, "windDirectionDeg": 0, "solarWM2": 120, "diffuseFraction": 0.4, "pressurePa": 101325},
        "passengers": [], "operating": [], "regions": REGIONS, "emitters": [],
        "assumptions": [
            T("Passive wood-based boards after ISO 10456: foiled P2 chipboard 650 kg/m3, 0.13 W/mK, 1700 J/kgK; HDF back 850 kg/m3; heat capacity of the whole 16 mm board (17.7 kJ/m2K).", "Passive Holzwerkstoffplatten nach ISO 10456: foliierte P2-Spanplatte 650 kg/m3, 0,13 W/mK, 1700 J/kgK; HDF-Rückwand 850 kg/m3; Wärmekapazität der ganzen 16-mm-Platte (17,7 kJ/m2K)."),
            T("Aged surfaces: the white melamine foil has yellowed and is scuffed, so its solar absorptance is taken as 0.32 instead of about 0.25 when new (authored estimate after Incropera, white paints 0.2-0.3); the raw brown HDF back 0.6; the silvered mirror reflects most of the sun (0.12) and radiates like glass from its front (0.84).", "Gealterte Oberflächen: die weiße Melaminfolie ist vergilbt und verschrammt, daher gilt ein solarer Absorptionsgrad von 0,32 statt etwa 0,25 im Neuzustand (Schätzung nach Incropera, weiße Lacke 0,2-0,3); die rohe braune HDF-Rückwand 0,6; der Silberspiegel reflektiert den größten Teil der Sonne (0,12) und strahlt von vorn wie Glas (0,84)."),
            T("Closed doors make the carcass a closed cavity: the interior (partition, shelves, hat shelf, rail, hinge arms) is a hidden region; an open door exposes up to 60 percent of it. The metal handles, the beech knob and the restraint angle are 0.03 m2 together and are counted in the door and carcass regions.", "Geschlossene Türen machen den Korpus zu einem geschlossenen Hohlraum: der Innenraum (Mittelwand, Fachböden, Hutboden, Stange, Scharnierarme) ist eine verdeckte Region; eine offene Tür legt bis zu 60 Prozent davon frei. Die Metallgriffe, der Buchenknopf und der Kippschutzwinkel haben zusammen 0,03 m2 und zählen zu den Regionen Türen und Korpus."),
            T("Reference case: unheated reuse depot hall at 19 degC with daylight through roof lights (120 W/m2, 40 percent diffuse); the wardrobe draws no power (passive, 0 W).", "Referenzfall: unbeheizte Lagerhalle eines Gebrauchtwarenkaufhauses bei 19 degC mit Tageslicht durch Oberlichter (120 W/m2, 40 Prozent diffus); der Schrank nimmt keine Leistung auf (passiv, 0 W)."),
        ],
    },
    "structure": {
        "gravityMS2": 9.80665, "dynamics": "quasi-static",
        "supports": ["blanket-floor"],
        "assumptions": [
            T("The carcass is a box of 16 mm P2 chipboard (EN 312, E about 2.3 GPa, bending strength 11.5 MPa) joined by cam fittings and braced against racking by the 3 mm HDF back; it stands on the floor edges of its two side panels (16 x 537 mm each); members are the boards on their bones with their true sections.", "Der Korpus ist ein Kasten aus 16-mm-P2-Spanplatte (EN 312, E etwa 2,3 GPa, Biegefestigkeit 11,5 MPa), mit Exzenterverbindern gefügt und durch die 3-mm-HDF-Rückwand gegen Verschieben ausgesteift; er steht auf den Bodenkanten seiner zwei Seitenwände (je 16 x 537 mm); Bauteile sind die Platten auf ihren Knochen mit ihren wahren Querschnitten."),
            T("In use: hanging clothes 15 kg/m on the 0.976 m rail (oval tube 30 x 15 x 1 mm, I = 8.6e-9 m4: sag 0.8 mm under 143 N), folded linen 10 kg per shelf and 8 kg of boxes on the hat shelf (authored estimates of domestic use, below the EN 16122 test loads).", "In Gebrauch: hängende Kleidung 15 kg/m an der 0,976-m-Stange (Ovalrohr 30 x 15 x 1 mm, I = 8,6e-9 m4: Durchbiegung 0,8 mm unter 143 N), gefaltete Wäsche 10 kg je Fachboden und 8 kg Kartons auf dem Hutboden (Schätzungen des Hausgebrauchs, unter den Prüflasten der EN 16122)."),
            T(f"Tipping (EN 14749, units over 600 mm): empty and with all three doors open by 95 degrees the centre of mass of the {TOTAL_KG} kg wardrobe moves about 75 mm forward, still 0.19 m behind the front edge of the side panels (restoring moment about 205 Nm); a child of 30 kg hanging at the edge of an open door (0.5 m in front of that edge, 147 Nm) is held, but a heavier pull is not, which is why the restraint angle is screwed to the wall in use. The planar solver of the archive resolves the loads in the width plane; the forward tipping check is this hand calculation.", f"Kippen (EN 14749, Möbel über 600 mm): leer und mit allen drei Türen um 95 Grad geöffnet wandert der Schwerpunkt des {str(TOTAL_KG).replace('.', ',')}-kg-Schranks etwa 75 mm nach vorn und liegt noch 0,19 m hinter der Vorderkante der Seitenwände (Rückstellmoment etwa 205 Nm); ein Kind von 30 kg am Rand einer offenen Tür (0,5 m vor dieser Kante, 147 Nm) wird gehalten, ein stärkerer Zug nicht, darum ist der Kippschutzwinkel in Gebrauch an die Wand geschraubt. Der ebene Löser des Archivs löst die Lasten in der Breitenebene; die Kippprüfung nach vorn ist diese Handrechnung."),
            T("Carried: two movers hold the bottom edges of the side panels (pins), each takes about half of the weight; the folded blanket stays on the floor on its own contact.", "Getragen: zwei Träger halten die Unterkanten der Seitenwände (Gelenke), jeder trägt etwa die Hälfte des Gewichts; die gefaltete Decke bleibt mit eigenem Kontakt auf dem Boden."),
        ],
        "clinical": False,
    },
    "volumes": volumes,
    "presentation": {"defaultPose": "stored", "defaultClip": "door-cycle", "defaultScale": 50},
    "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    "completion": "authored",
}

doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as fh:
    json.dump(doc, fh, ensure_ascii=False, indent=1)
print(f"[DEBUG] mass {TOTAL_KG} kg; bones with mass {len(MASS)}; solids per scale " + ", ".join(f"1:{s} {len(volumes[str(s)]['body'])}/{len(volumes[str(s)]['thermal'])}/{len(volumes[str(s)]['structure'])}" for s in (50, 100, 200, 500)))
