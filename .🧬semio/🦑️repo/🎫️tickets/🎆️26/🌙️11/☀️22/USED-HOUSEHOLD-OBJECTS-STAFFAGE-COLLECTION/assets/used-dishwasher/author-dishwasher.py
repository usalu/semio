"""🍽️ Authors the used 60 cm freestanding dishwasher (`used-dishwasher`) from hand-chosen numbers and writes `🔣️.json` next to this file.

Every part is stated once in world metres of the rest pose (the stored pose: the machine standing on a Düsseldorf half pallet, its hoses and
mains cable coiled and taped to the back) and converted into the frame of its bone, so the 2D drawings and the 3D solids of one scale come
from the same numbers. World: x ahead (the door faces +x), y up, z to the machine's left. Heights `h` of the machine are measured from the
underside of its feet; `Y(h)` lifts them onto the pallet. Each scale is its own part list.

@see ../../brief.md — the author brief
@see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-tools.md — the tools that check the output
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
PAL = 0.144
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03}
SIDE_T = 0.0006
TUB_T = 0.0005
G = 9.80665


def T(en, de):
    """🌐 A bilingual text, English first."""
    return {"en": en, "de": de}


def r(v):
    """🔢 Rounds a coordinate to a micrometre."""
    return round(v + 0.0, 6)


def Y(h):
    """⬆️ World height of a machine height `h` (above the underside of the feet) in the rest pose, on the pallet."""
    return r(h + PAL)


BONES = [
    ("ground", None, 0, 0, 0, 0, 0.0, {"type": "fixed"}, T("Floor reference", "Bodenbezug")),
    ("pallet", "ground", -0.10, 0, 0, 0, 0.80, {"type": "prismatic", "axisDeg": 0, "minM": -1.2, "maxM": 0}, T("Düsseldorf half pallet under the machine", "Düsseldorfer Halbpalette unter der Maschine")),
    ("lift", "ground", 0, PAL, 0, 0, 0.0, {"type": "prismatic", "axisDeg": 90, "minM": -PAL, "maxM": 0.4}, T("Lift of the machine off its base", "Anheben der Maschine von ihrer Unterlage")),
    ("body", "lift", 0, 0, 0, 0, 0.04, {"type": "revolute", "minDeg": -15, "maxDeg": 15}, T("Machine, tilt about the rear bottom edge", "Maschine, Neigung um die hintere Unterkante")),
    ("base", "body", 0, 0, 0, 0, 0.525, {"type": "fixed"}, T("Base pan with pumps, heater and feet", "Bodenwanne mit Pumpen, Heizung und Füßen")),
    ("side-l", "body", 0, 0.02, 0.2997, 90, 0.80, {"type": "fixed"}, T("Left side panel", "Linke Seitenwand")),
    ("side-r", "body", 0, 0.02, -0.2997, 90, 0.80, {"type": "fixed"}, T("Right side panel", "Rechte Seitenwand")),
    ("back", "body", 0.0003, 0.02, 0, 90, 0.80, {"type": "fixed"}, T("Back panel", "Rückwand")),
    ("worktop", "body", 0, 0.82, 0, 0, 0.60, {"type": "fixed"}, T("Removable worktop", "Abnehmbare Arbeitsplatte")),
    ("plinth", "body", 0.525, 0.010, 0, 0, 0.04, {"type": "fixed"}, T("Kick plinth clipped under the door", "Sockelblende unter der Tür eingeklipst")),
    ("tub", "body", 0.045, 0.125, 0, 0, 0.50, {"type": "fixed"}, T("Stainless wash tub with its front frame and rails", "Edelstahl-Spülbehälter mit Frontrahmen und Schienen")),
    ("door", "body", 0.545, 0.125, 0, 90, 0.69, {"type": "revolute", "minDeg": -90, "maxDeg": 0}, T("Drop-down door, hinged at its lower inner edge", "Klapptür, an ihrer unteren Innenkante angeschlagen")),
    ("basket-lower", "body", 0.065, 0.125, 0, 0, 0.465, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.42}, T("Lower basket on its wheels", "Unterkorb auf seinen Rollen")),
    ("basket-upper", "body", 0.065, 0.455, 0, 0, 0.455, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.33}, T("Upper basket in its rails", "Oberkorb in seinen Schienen")),
    ("spray-lower", "tub", 0.246, 0.0, 0, 90, 0.03, {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": -360, "twistMaxDeg": 360}, T("Lower spray arm on its hub", "Unterer Sprüharm auf seiner Nabe")),
    ("spray-upper", "basket-upper", 0.226, -0.030, 0, 90, 0.03, {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": -360, "twistMaxDeg": 360}, T("Upper spray arm under the upper basket", "Oberer Sprüharm unter dem Oberkorb")),
    ("hoses", "body", 0.0, 0.30, 0, 0, 0.03, {"type": "fixed"}, T("Hose and cable coils taped to the back", "An die Rückwand geklebte Schlauch- und Kabelringe")),
]

FRAME = {}
for bid, parent, x, y, z, ang, length, joint, title in BONES:
    if parent is None:
        FRAME[bid] = ((0.0, 0.0, 0.0), False)
    else:
        (px, py, pz), pvert = FRAME[parent]
        assert not pvert
        FRAME[bid] = ((px + x, py + y, pz + z), ang == 90)


def L(bone, x, y, z=None):
    """📐 A world point of the rest pose in the (u, v[, w]) frame of `bone`."""
    (ox, oy, oz), vert = FRAME[bone]
    uv = [r(y - oy), r(-(x - ox))] if vert else [r(x - ox), r(y - oy)]
    return uv if z is None else uv + [r(z - oz)]


def D(bone, dx, dy, dz):
    """🧭 A world direction in the frame of `bone`."""
    return [dy, -dx, dz] if FRAME[bone][1] else [dx, dy, dz]


def wz(bone, z):
    """↔️ The lateral coordinate w of a world z in the frame of `bone`."""
    return r(z - FRAME[bone][0][2])


def box(sid, bone, b, material, tone, role="body", **extra):
    """📦 A box solid from a world box (x0, x1, y0, y1, z0, z1) of the rest pose."""
    x0, x1, y0, y1, z0, z1 = b
    vert = FRAME[bone][1]
    size = [r(y1 - y0), r(x1 - x0), r(z1 - z0)] if vert else [r(x1 - x0), r(y1 - y0), r(z1 - z0)]
    u, v = L(bone, (x0 + x1) / 2, (y0 + y1) / 2)
    s = {"id": sid, "kind": "box", "bone": bone, "material": material, "role": role, "tone": tone, "size": size, "transform": {"translation": [u, v, wz(bone, (z0 + z1) / 2)]}}
    s.update(extra)
    return s


def extrude_xy(sid, bone, pts, z0, z1, material, tone, role="body", **extra):
    """🧱 An extrusion of a world (x, y) polygon of the rest pose from z0 to z1 (plane xy of the bone)."""
    s = {"id": sid, "kind": "extrude", "bone": bone, "material": material, "role": role, "tone": tone, "profile": [L(bone, *p) for p in pts], "plane": "xy", "from": wz(bone, z0), "to": wz(bone, z1)}
    s.update(extra)
    return s


def capsule(sid, bone, a, b, rad, material, tone, role="detail", **extra):
    """💊 A rod between two world points of the rest pose."""
    s = {"id": sid, "kind": "capsule", "bone": bone, "material": material, "role": role, "tone": tone, "p0": L(bone, *a), "p1": L(bone, *b), "r0": rad, "r1": rad}
    s.update(extra)
    return s


def revolve(sid, bone, profile, axis, origin, material, tone, role="body", **extra):
    """🌀 A revolved solid about the world direction `axis` through the world point `origin`."""
    s = {"id": sid, "kind": "revolve", "bone": bone, "material": material, "role": role, "tone": tone, "profile": [[r(a), r(b)] for a, b in profile], "axis": D(bone, *axis), "origin": L(bone, *origin)}
    s.update(extra)
    return s


def torus_profile(R, t, n=20):
    """🍩 The closed [axial, radial] loop of a tube of radius t on a coil of radius R."""
    pts = [(t * math.cos(2 * math.pi * k / n), R + t * math.sin(2 * math.pi * k / n)) for k in range(n)]
    return pts + [pts[0]]


def rep(solid, count, step):
    """🔁 Adds a repeat of `count` copies with the world step (dx, dy, dz) converted into the bone frame."""
    d = D(solid["bone"], *step)
    solid["repeat"] = {"count": count, "translation": [r(c) for c in d]}
    return solid


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
    """🔷 An elevation polygon of world (x, y) points in the (u, v) plane of `bone`."""
    return {"kind": "poly", "points": [L(bone, *p) for p in pts], "closed": closed}


def eline(bone, pts):
    """〰️ An open elevation stroke of world (x, y) points."""
    return epoly(bone, pts, closed=False)


def ecircle(bone, x, y, rad):
    """⚪ An elevation circle about a world point."""
    u, v = L(bone, x, y)
    return {"kind": "ellipse", "cx": u, "cy": v, "rx": rad, "ry": rad}


def frect(z0, z1, y0, y1):
    """🟦 A front rect in rest world (z, y)."""
    return {"kind": "rect", "x": r(z0), "y": r(y0), "w": r(z1 - z0), "h": r(y1 - y0)}


def fpoly(pts, closed=True):
    """🔹 A front polygon of rest world (z, y) points."""
    return {"kind": "poly", "points": [[r(a), r(b)] for a, b in pts], "closed": closed}


def prect(x0, x1, z0, z1):
    """🟩 A plan rect in rest world (x, z)."""
    return {"kind": "rect", "x": r(x0), "y": r(z0), "w": r(x1 - x0), "h": r(z1 - z0)}


def ppoly(pts, closed=True):
    """🔸 A plan polygon of rest world (x, z) points."""
    return {"kind": "poly", "points": [[r(a), r(b)] for a, b in pts], "closed": closed}


def sweep_w(z0, z1):
    """🌀 The placement of a door part drawn by its side profile and swept across the door."""
    return {"solid": {"kind": "extruded", "sweep": "w", "z0": r(z0), "z1": r(z1)}}


def sweep_v(x0, x1):
    """🧲 The placement of a door part drawn by its face (height and lateral) and swept through the door thickness (world x0..x1)."""
    return {"solid": {"kind": "extruded", "sweep": "v", "z0": r(-(x1 - 0.545)), "z1": r(-(x0 - 0.545))}}


def dface(h0, h1, z0, z1):
    """🚪 A door-face rect in the (u, w) plane of the door from machine heights and world z."""
    return {"kind": "rect", "x": r(h0 - 0.125), "y": r(z0), "w": r(h1 - h0), "h": r(z1 - z0)}


def dface_circle(h, z, rad):
    """🔘 A door-face circle in the (u, w) plane of the door."""
    return {"kind": "ellipse", "cx": r(h - 0.125), "cy": r(z), "rx": rad, "ry": rad}


def dface_line(pts):
    """✒️ An open door-face stroke of (machine height, world z) points."""
    return {"kind": "poly", "points": [[r(h - 0.125), r(z)] for h, z in pts], "closed": False}


MATERIALS = [
    {"id": "painted-steel", "title": T("White powder-coated steel sheet 0.6 mm, yellowed and scratched", "Weiß pulverbeschichtetes Stahlblech 0,6 mm, vergilbt und verkratzt"), "emissivity": 0.9, "solarAbsorptivity": 0.33, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210e9, "strengthPa": 235e6, "source": "literature", "reference": "en-10130"},
    {"id": "stainless-steel", "title": T("Stainless steel 1.4016 sheet 0.5 mm, limescaled", "Edelstahlblech 1.4016 0,5 mm, verkalkt"), "emissivity": 0.35, "solarAbsorptivity": 0.45, "densityKgM3": 7700, "conductivityWmK": 25, "specificHeatJKgK": 460, "elasticModulusPa": 200e9, "strengthPa": 260e6, "source": "literature", "reference": "en-10088-2"},
    {"id": "tub-frame", "title": T("Folded stainless front frame with door seal (8 mm envelope)", "Gekanteter Edelstahl-Frontrahmen mit Türdichtung (8-mm-Hülle)"), "emissivity": 0.4, "solarAbsorptivity": 0.45, "densityKgM3": 1200, "conductivityWmK": 15, "specificHeatJKgK": 500, "elasticModulusPa": 200e9, "strengthPa": 260e6, "source": "authored-estimate", "reference": "en-10088-2"},
    {"id": "bitumen-mat", "title": T("Bitumen sound-damping mat 4 mm", "Bitumen-Schalldämmmatte 4 mm"), "emissivity": 0.92, "solarAbsorptivity": 0.9, "densityKgM3": 1500, "conductivityWmK": 0.17, "specificHeatJKgK": 1000, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "base-unit", "title": T("Base pan of polypropylene with circulation pump, drain pump, heater, sump and softener (effective density)", "Bodenwanne aus Polypropylen mit Umwälzpumpe, Laugenpumpe, Heizung, Sumpf und Enthärter (wirksame Dichte)"), "emissivity": 0.92, "solarAbsorptivity": 0.6, "densityKgM3": 320, "conductivityWmK": 0.22, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "used-appliance-market"},
    {"id": "door-composite", "title": T("Door: painted steel outer panel, bitumen and felt, stainless inner liner (effective density)", "Tür: lackiertes Stahl-Außenblech, Bitumen und Filz, Edelstahl-Innenblech (wirksame Dichte)"), "emissivity": 0.9, "solarAbsorptivity": 0.33, "densityKgM3": 300, "conductivityWmK": 0.06, "specificHeatJKgK": 900, "elasticModulusPa": 210e9, "strengthPa": 235e6, "source": "authored-estimate", "reference": "used-appliance-market"},
    {"id": "panel-abs", "title": T("Control panel moulding of ABS with electronics, yellowed (effective density)", "Bedienblende aus ABS mit Elektronik, vergilbt (wirksame Dichte)"), "emissivity": 0.92, "solarAbsorptivity": 0.4, "densityKgM3": 400, "conductivityWmK": 0.17, "specificHeatJKgK": 1400, "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "knob-abs", "title": T("Grey replacement programme knob of ABS", "Grauer Ersatz-Programmknebel aus ABS"), "emissivity": 0.92, "solarAbsorptivity": 0.6, "densityKgM3": 600, "conductivityWmK": 0.17, "specificHeatJKgK": 1400, "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "laminate-chipboard", "title": T("Worktop: chipboard P2 with plastic laminate, 30 mm", "Arbeitsplatte: Spanplatte P2 mit Schichtstoff, 30 mm"), "emissivity": 0.9, "solarAbsorptivity": 0.45, "densityKgM3": 680, "conductivityWmK": 0.14, "specificHeatJKgK": 1700, "elasticModulusPa": 2.5e9, "strengthPa": 12e6, "source": "literature", "reference": "en-312"},
    {"id": "pp-plinth", "title": T("Polypropylene kick plinth, white", "Sockelblende aus Polypropylen, weiß"), "emissivity": 0.92, "solarAbsorptivity": 0.35, "densityKgM3": 900, "conductivityWmK": 0.22, "specificHeatJKgK": 1700, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "pp-grey", "title": T("Grey polypropylene (feet, spray arms, cutlery basket, wheels, Aquastop box, plug)", "Graues Polypropylen (Füße, Sprüharme, Besteckkorb, Rollen, Aquastop-Gehäuse, Stecker)"), "emissivity": 0.92, "solarAbsorptivity": 0.6, "densityKgM3": 900, "conductivityWmK": 0.22, "specificHeatJKgK": 1700, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "cutlery-basket", "title": T("Perforated polypropylene cutlery basket (effective density of its envelope)", "Durchbrochener Besteckkorb aus Polypropylen (wirksame Dichte seiner Hülle)"), "emissivity": 0.92, "solarAbsorptivity": 0.6, "densityKgM3": 200, "conductivityWmK": 0.22, "specificHeatJKgK": 1700, "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "coated-wire", "title": T("PVC-coated steel wire of the baskets, coating chipped", "PVC-beschichteter Stahldraht der Körbe, Beschichtung abgeplatzt"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": 7000, "conductivityWmK": 40, "specificHeatJKgK": 500, "elasticModulusPa": 200e9, "strengthPa": 400e6, "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "rail-steel", "title": T("Telescopic rail of folded zinc-plated steel with rollers (effective density)", "Teleskopschiene aus gekantetem verzinktem Stahl mit Rollen (wirksame Dichte)"), "emissivity": 0.3, "solarAbsorptivity": 0.5, "densityKgM3": 2600, "conductivityWmK": 50, "specificHeatJKgK": 460, "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "pvc-hose", "title": T("Grey PVC and polypropylene hose, corrugated", "Grauer PVC- und Polypropylenschlauch, gewellt"), "emissivity": 0.92, "solarAbsorptivity": 0.6, "densityKgM3": 450, "conductivityWmK": 0.17, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "pvc-cable", "title": T("PVC-sheathed mains cable 3 x 1.5 mm2", "PVC-Netzleitung 3 x 1,5 mm2"), "emissivity": 0.92, "solarAbsorptivity": 0.6, "densityKgM3": 1800, "conductivityWmK": 0.3, "specificHeatJKgK": 1000, "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "packing-tape", "title": T("Brown polypropylene packing tape", "Braunes Polypropylen-Packband"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "densityKgM3": 900, "conductivityWmK": 0.2, "specificHeatJKgK": 1700, "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "pallet-spruce", "title": T("Spruce pallet timber, weathered grey", "Paletten-Fichtenholz, grau verwittert"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": 420, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11e9, "strengthPa": 24e6, "source": "literature", "reference": "en-338"},
]
DENSITY = {m["id"]: m["densityKgM3"] for m in MATERIALS}

ZC = 0.2994
DOOR_Z = (-0.299, 0.299)
FEET_X = (0.050, 0.505)
TOP_BOARDS = (-0.25, -0.125, 0.0, 0.125, 0.25)
CROSS_X = (-0.10, 0.2275, 0.555)
PALLET_Z = (-0.25, 0.0, 0.25)
PANEL_PTS = [(0.548, 0.735), (0.578, 0.735), (0.578, 0.760), (0.600, 0.760), (0.600, 0.815), (0.548, 0.815)]
KNOB = (0.787, 0.200, 0.021)
COILS = {"drain-coil": (-0.012, 0.600, -0.060, 0.135, 0.012), "inlet-coil": (-0.010, 0.320, 0.060, 0.115, 0.010), "cable-coil": (-0.004, 0.640, 0.200, 0.070, 0.004)}
AQUASTOP = (-0.045, 0.0, 0.160, 0.260, -0.240, -0.170)
PLUG = (-0.036, 0.0, 0.610, 0.670, 0.182, 0.218)
WORKTOP_PTS = [(0.0, -0.30), (0.582, -0.30), (0.582, -0.288), (0.600, -0.288), (0.600, 0.30), (0.0, 0.30)]
PLINTH_PTS = [(0.537, 0.010), (0.540, 0.010), (0.540, 0.110), (0.545, 0.110), (0.545, 0.125), (0.525, 0.125), (0.525, 0.110), (0.537, 0.110)]


def lower_basket(s, body, thermal, ev, fv, pv):
    """🧺 The lower basket at scale 1:s: wire rims, floor rods, side wires, plate tines, wheels and the cutlery basket."""
    b = "basket-lower"
    x0, x1, zb = 0.073, 0.522, 0.258
    hf, ht = 0.157, 0.320
    out = []
    if s in (50, 100):
        wr = 0.0035
        out.append(capsule("bl-floor-rail", b, (x0, Y(hf), -zb), (x1, Y(hf), -zb), wr, "coated-wire", 0.45, slender=True))
        rep(out[-1], 2, (0, 0, 2 * zb))
        out.append(capsule("bl-floor-end", b, (x0, Y(hf), -zb), (x0, Y(hf), zb), wr, "coated-wire", 0.45, slender=True))
        rep(out[-1], 2, (0, 0, 0))
        out[-1]["repeat"]["translation"] = [r(x1 - x0), 0, 0]
        out.append(capsule("bl-top-rail", b, (x0, Y(ht), -zb), (x1, Y(ht), -zb), wr, "coated-wire", 0.45, slender=True))
        rep(out[-1], 2, (0, 0, 2 * zb))
        out.append(capsule("bl-top-end", b, (x0, Y(ht), -zb), (x0, Y(ht), zb), wr, "coated-wire", 0.45, slender=True))
        out[-1]["repeat"] = {"count": 2, "translation": [r(x1 - x0), 0, 0]}
        for k, z in enumerate((-zb, zb)):
            out.append(capsule(f"bl-post-{'r' if z < 0 else 'l'}", b, (x0, Y(hf), z), (x0, Y(ht), z), 0.0025, "coated-wire", 0.45, slender=True))
            out[-1]["repeat"] = {"count": 6 if s == 50 else 4, "translation": [r((x1 - x0) / (5 if s == 50 else 3)), 0, 0]}
        n = 12 if s == 50 else 6
        out.append(capsule("bl-floor-rods", b, (x0, Y(hf), -0.235), (x1, Y(hf), -0.235), 0.002, "coated-wire", 0.45, slender=True))
        rep(out[-1], n, (0, 0, 0.47 / (n - 1)))
        nt = 10 if s == 50 else 5
        for z, side in ((-0.10, "r"), (0.10, "l")):
            out.append(capsule(f"bl-tine-bar-{side}", b, (x0, Y(hf), z), (x1, Y(hf), z), 0.002, "coated-wire", 0.45, slender=True))
            out.append(capsule(f"bl-tines-{side}", b, (0.105, Y(hf), z), (0.105, Y(0.290), z), 0.0015 if s == 50 else 0.002, "coated-wire", 0.45, slender=True))
            rep(out[-1], nt, (0.38 / (nt - 1), 0, 0))
        ww = 0.010 if s == 50 else 0.015
        for side, z in (("r", -0.262), ("l", 0.262)):
            out.append(revolve(f"bl-wheel-{side}", b, [(-ww / 2, 0.015), (ww / 2, 0.015)], (0, 0, 1), (0.095, Y(0.140), z), "pp-grey", 0.3))
            out[-1]["repeat"] = {"count": 2, "translation": [0.405, 0, 0]}
        out.append(box("bl-cutlery", b, (0.390, 0.505, Y(0.160), Y(0.285), -0.240, -0.125), "cutlery-basket", 0.55, role="body"))
    else:
        out.append(capsule("bl-top-rail", b, (x0, Y(ht), -zb), (x1, Y(ht), -zb), 0.0035, "coated-wire", 0.45, slender=True))
        rep(out[-1], 2, (0, 0, 2 * zb))
        out.append(capsule("bl-top-end", b, (x0, Y(ht), -zb), (x0, Y(ht), zb), 0.0035, "coated-wire", 0.45, slender=True))
        out[-1]["repeat"] = {"count": 2, "translation": [r(x1 - x0), 0, 0]}
        out.append(capsule("bl-floor-rail", b, (x0, Y(hf), -zb), (x1, Y(hf), -zb), 0.0035, "coated-wire", 0.45, slender=True))
        rep(out[-1], 2, (0, 0, 2 * zb))
        for side, z in (("r", -zb), ("l", zb)):
            out.append(capsule(f"bl-post-{side}", b, (x0, Y(0.1285), z), (x0, Y(ht), z), 0.0035, "coated-wire", 0.45, slender=True))
            out[-1]["repeat"] = {"count": 2, "translation": [r(x1 - x0), 0, 0]}
        out.append(box("bl-cutlery", b, (0.390, 0.505, Y(0.160), Y(0.285), -0.2545, -0.125), "cutlery-basket", 0.55, role="body"))
    body.extend(out)
    for sol in out:
        thermal.append(dict(sol, id=f"th-{sol['id']}", region="racks"))
    pen = {50: 0.18, 100: 0.13, 200: 0.13}[s]
    ev.append(shape("bl-side", b, epoly(b, [(x0, Y(hf)), (x1, Y(hf)), (x1, Y(ht)), (x0, Y(ht))]), "detail", 0.5, False, 0.45, region="racks", material="coated-wire", penMm=pen))
    if s in (50, 100):
        nw = 6 if s == 50 else 4
        for k in range(1, nw - 1):
            x = x0 + (x1 - x0) * k / (nw - 1)
            ev.append(shape(f"bl-wire-{k}", b, eline(b, [(x, Y(hf)), (x, Y(ht))]), "detail", 0.5, False, 0.45, penMm=pen))
        for k, x in enumerate((0.095, 0.500)):
            ev.append(shape(f"bl-wheel-{k + 1}", b, ecircle(b, x, Y(0.140), 0.015), "detail", 0.6, True, 0.3, region="racks", material="pp-grey"))
        ev.append(shape("bl-cutlery", b, erect(b, 0.390, 0.505, Y(0.160), Y(0.285)), "detail", 0.55, True, 0.55, region="racks", material="pp-grey"))
        fv.append(shape("bl-front", b, frect(-zb, zb, Y(hf), Y(ht)), "detail", 2.5, False, 0.45, region="racks", material="coated-wire", penMm=pen))
        for k in range(1, 6):
            z = -zb + 2 * zb * k / 6
            fv.append(shape(f"bl-front-wire-{k}", b, fpoly([(z, Y(hf)), (z, Y(ht))], closed=False), "detail", 2.5, False, 0.45, penMm=pen))
        pv.append(shape("bl-plan", b, prect(x0, x1, -zb, zb), "detail", 4.5, False, 0.45, region="racks", material="coated-wire", penMm=pen))
        pv.append(shape("bl-plan-cutlery", b, prect(0.390, 0.505, -0.240, -0.125), "detail", 4.55, True, 0.55, region="racks", material="pp-grey"))
    else:
        fv.append(shape("bl-front", b, frect(-zb, zb, Y(hf), Y(ht)), "detail", 2.5, False, 0.45, region="racks", material="coated-wire", penMm=pen))
        pv.append(shape("bl-plan", b, prect(x0, x1, -zb, zb), "detail", 4.5, False, 0.45, region="racks", material="coated-wire", penMm=pen))
    return out


def upper_basket(s, body, thermal, ev, fv, pv):
    """🥛 The upper basket at scale 1:s: runner rods in the rails, rims, floor rods and cup tines."""
    b = "basket-upper"
    x0, x1, zb = 0.073, 0.515, 0.250
    hf, ht = 0.457, 0.625
    out = []
    zr = 0.258 if s in (50, 100) else 0.271
    out.append(capsule("bu-runner", b, (x0, Y(0.500), -zr), (x1, Y(0.500), -zr), 0.004, "coated-wire", 0.45, slender=True))
    rep(out[-1], 2, (0, 0, 2 * zr))
    out.append(capsule("bu-top-rail", b, (x0, Y(ht), -zb), (x1, Y(ht), -zb), 0.0035, "coated-wire", 0.45, slender=True))
    rep(out[-1], 2, (0, 0, 2 * zb))
    out.append(capsule("bu-top-end", b, (x0, Y(ht), -zb), (x0, Y(ht), zb), 0.0035, "coated-wire", 0.45, slender=True))
    out[-1]["repeat"] = {"count": 2, "translation": [r(x1 - x0), 0, 0]}
    out.append(capsule("bu-floor-rail", b, (x0, Y(hf), -zb), (x1, Y(hf), -zb), 0.0035, "coated-wire", 0.45, slender=True))
    rep(out[-1], 2, (0, 0, 2 * zb))
    out.append(capsule("bu-floor-end", b, (x0, Y(hf), -zr), (x0, Y(hf), zr), 0.0035, "coated-wire", 0.45, slender=True))
    out[-1]["repeat"] = {"count": 2, "translation": [r(x1 - x0), 0, 0]}
    for side, z in (("r", -zb), ("l", zb)):
        out.append(capsule(f"bu-post-{side}", b, (x0, Y(hf), z), (x0, Y(ht), z), 0.0025, "coated-wire", 0.45, slender=True))
        out[-1]["repeat"] = {"count": {50: 5, 100: 3, 200: 2}[s], "translation": [r((x1 - x0) / {50: 4, 100: 2, 200: 1}[s]), 0, 0]}
    for side, z in (("r", -zr), ("l", zr)):
        out.append(capsule(f"bu-hanger-{side}", b, (x0, Y(hf), z), (x0, Y(ht), z), 0.0035, "coated-wire", 0.45, slender=True))
        out[-1]["repeat"] = {"count": 2, "translation": [r(x1 - x0), 0, 0]}
    if s in (50, 100):
        n = 10 if s == 50 else 5
        out.append(capsule("bu-floor-rods", b, (x0, Y(hf), -0.225), (x1, Y(hf), -0.225), 0.002, "coated-wire", 0.45, slender=True))
        rep(out[-1], n, (0, 0, 0.45 / (n - 1)))
        if s == 50:
            out.append(capsule("bu-tine-bar", b, (x0, Y(hf), 0.0), (x1, Y(hf), 0.0), 0.002, "coated-wire", 0.45, slender=True))
            out.append(capsule("bu-tines", b, (0.10, Y(hf), 0.0), (0.10, Y(0.560), 0.0), 0.0015, "coated-wire", 0.45, slender=True))
            rep(out[-1], 9, (0.04, 0, 0))
    body.extend(out)
    for sol in out:
        thermal.append(dict(sol, id=f"th-{sol['id']}", region="racks"))
    pen = {50: 0.18, 100: 0.13, 200: 0.13}[s]
    ev.append(shape("bu-side", b, epoly(b, [(x0, Y(hf)), (x1, Y(hf)), (x1, Y(ht)), (x0, Y(ht))]), "detail", 0.5, False, 0.45, region="racks", material="coated-wire", penMm=pen))
    ev.append(shape("bu-runner", b, eline(b, [(x0, Y(0.500)), (x1, Y(0.500))]), "detail", 0.5, False, 0.45, penMm=pen))
    fv.append(shape("bu-front", b, frect(-zb, zb, Y(hf), Y(ht)), "detail", 2.5, False, 0.45, region="racks", material="coated-wire", penMm=pen))
    pv.append(shape("bu-plan", b, prect(x0, x1, -zb, zb), "detail", 4.6, False, 0.45, region="racks", material="coated-wire", penMm=pen))
    if s in (50, 100):
        for k in range(1, 4):
            x = x0 + (x1 - x0) * k / 4
            ev.append(shape(f"bu-wire-{k}", b, eline(b, [(x, Y(hf)), (x, Y(ht))]), "detail", 0.5, False, 0.45, penMm=pen))
    return out


def spray_arms(s, body, thermal):
    """💦 The two spray arms with their hubs (1:50 and 1:100 only; under the feature of 1:200)."""
    if s == 200:
        return []
    out = []
    xc = 0.291
    out.append(revolve("sl-hub", "spray-lower", [(0, 0.020), (0.025, 0.020)], (0, 1, 0), (xc, Y(0.125), 0), "pp-grey", 0.35))
    out.append(box("sl-arm", "spray-lower", (xc - 0.21, xc + 0.21, Y(0.132), Y(0.150), -0.0175, 0.0175), "pp-grey", 0.35))
    out.append(revolve("su-hub", "spray-upper", [(0, 0.018), (0.030, 0.018)], (0, 1, 0), (xc, Y(0.425), 0), "pp-grey", 0.35))
    out.append(box("su-arm", "spray-upper", (xc - 0.18, xc + 0.18, Y(0.425), Y(0.441), -0.016, 0.016), "pp-grey", 0.35))
    body.extend(out)
    for sol in out:
        thermal.append(dict(sol, id=f"th-{sol['id']}", region="racks"))
    return out


def parts(s):
    """🧩 The hand-chosen parts of scale 1:s: body solids, thermal solids, structure solids and the 2D shapes of the three views."""
    f = FEATURE[s]
    body, thermal, struct = [], [], []
    ev, fv, pv = [], [], []

    def th(of, region):
        thermal.append({"id": f"th-{of}", "of": of, "region": region})

    def thc(sol, region):
        thermal.append(dict(sol, id=f"th-{sol['id']}", region=region))

    def mem(of, member, part="member", **extra):
        d = {"id": f"m-{of}", "of": of, "member": member, "part": part}
        d.update(extra)
        struct.append(d)

    P = "pallet"
    if s in (50, 100):
        tb = rep(box("pallet-top", P, (-0.10, 0.70, 0.122, 0.144, -0.30, -0.20), "pallet-spruce", 0.62), 5, (0, 0, 0.125))
        cb = rep(box("pallet-cross", P, (-0.10, 0.045, 0.100, 0.122, -0.30, 0.30), "pallet-spruce", 0.55), 3, (0.3275, 0, 0))
        bk = [rep(box(f"pallet-blocks-{k + 1}", P, (-0.10, 0.045, 0.022, 0.100, z - 0.05, z + 0.05), "pallet-spruce", 0.5), 3, (0.3275, 0, 0)) for k, z in enumerate(PALLET_Z)]
        bb = rep(box("pallet-bottom", P, (-0.10, 0.70, 0.0, 0.022, -0.30, -0.20), "pallet-spruce", 0.6), 3, (0, 0, 0.25))
        for sol in [tb, cb, *bk, bb]:
            body.append(sol)
            th(sol["id"], "pallet")
        mem("pallet-top", P)
        mem("pallet-bottom", P)
        for sol in [cb, *bk]:
            mem(sol["id"], P, part="joint")
        ev.append(shape("pallet-bottom", P, erect(P, -0.10, 0.70, 0.0, 0.022), "body", 1.0, True, 0.6, region="pallet", material="pallet-spruce"))
        for k, x in enumerate(CROSS_X):
            ev.append(shape(f"pallet-block-{k + 1}", P, erect(P, x, x + 0.145, 0.022, 0.100), "body", 1.0, True, 0.5, region="pallet", material="pallet-spruce"))
            ev.append(shape(f"pallet-cross-{k + 1}", P, erect(P, x, x + 0.145, 0.100, 0.122), "body", 1.1, True, 0.55, region="pallet", material="pallet-spruce"))
        ev.append(shape("pallet-top", P, erect(P, -0.10, 0.70, 0.122, 0.144), "body", 1.2, True, 0.62, region="pallet", material="pallet-spruce"))
        for k, z in enumerate(PALLET_Z):
            fv.append(shape(f"pallet-bottom-{k + 1}", P, frect(z - 0.05, z + 0.05, 0.0, 0.022), "body", 1.0, True, 0.6, region="pallet", material="pallet-spruce"))
            fv.append(shape(f"pallet-block-{k + 1}", P, frect(z - 0.05, z + 0.05, 0.022, 0.100), "body", 1.0, True, 0.5, region="pallet", material="pallet-spruce"))
        fv.append(shape("pallet-cross", P, frect(-0.30, 0.30, 0.100, 0.122), "body", 1.1, True, 0.55, region="pallet", material="pallet-spruce"))
        for k, z in enumerate(TOP_BOARDS):
            fv.append(shape(f"pallet-top-{k + 1}", P, frect(z - 0.05, z + 0.05, 0.122, 0.144), "body", 1.2, True, 0.62, region="pallet", material="pallet-spruce"))
            pv.append(shape(f"pallet-top-{k + 1}", P, prect(-0.10, 0.70, z - 0.05, z + 0.05), "body", 1.0, True, 0.62, region="pallet", material="pallet-spruce"))
        for k, x in enumerate(CROSS_X):
            pv.append(shape(f"pallet-cross-{k + 1}", P, prect(x, x + 0.145, -0.30, 0.30), "body", 0.9, True, 0.4, region="pallet", material="pallet-spruce"))
        if s == 50:
            for k, (x, z) in enumerate(((-0.07, -0.25), (0.62, 0.0), (0.62, 0.25))):
                pv.append(shape(f"pallet-stamp-{k + 1}", P, prect(x, x + 0.05, z - 0.02, z + 0.02), "texture", 1.1, False, 0.3))
            for k, z in enumerate(TOP_BOARDS):
                pv.append(shape(f"pallet-grain-{k + 1}", P, ppoly([(-0.09, z - 0.02), (-0.06, z - 0.015), (-0.03, z - 0.022)], closed=False), "texture", 1.1, False, 0.3))
    else:
        dk = box("pallet-deck", P, (-0.10, 0.70, 0.100, 0.144, -0.30, 0.30), "pallet-spruce", 0.6)
        bk = [rep(box(f"pallet-blocks-{k + 1}", P, (-0.10, 0.045, 0.0, 0.100, z - 0.05, z + 0.05), "pallet-spruce", 0.5), 3, (0.3275, 0, 0)) for k, z in enumerate(PALLET_Z)]
        for sol in [dk, *bk]:
            body.append(sol)
            th(sol["id"], "pallet")
        mem("pallet-deck", P)
        for sol in bk:
            mem(sol["id"], P, part="joint")
        for k, x in enumerate(CROSS_X):
            ev.append(shape(f"pallet-block-{k + 1}", P, erect(P, x, x + 0.145, 0.0, 0.100), "body", 1.0, True, 0.5, region="pallet", material="pallet-spruce"))
        ev.append(shape("pallet-deck", P, erect(P, -0.10, 0.70, 0.100, 0.144), "body", 1.2, True, 0.6, region="pallet", material="pallet-spruce"))
        for k, z in enumerate(PALLET_Z):
            fv.append(shape(f"pallet-block-{k + 1}", P, frect(z - 0.05, z + 0.05, 0.0, 0.100), "body", 1.0, True, 0.5, region="pallet", material="pallet-spruce"))
        fv.append(shape("pallet-deck", P, frect(-0.30, 0.30, 0.100, 0.144), "body", 1.2, True, 0.6, region="pallet", material="pallet-spruce"))
        pv.append(shape("pallet-deck", P, prect(-0.10, 0.70, -0.30, 0.30), "body", 1.0, True, 0.6, region="pallet", material="pallet-spruce"))
    sx = {50: 0.40, 100: 0.40, 200: 0.40}[s]
    struct.append({"id": "support-pallet-floor", "kind": "box", "bone": P, "material": "pallet-spruce", "role": "ground", "tone": 0.2, "size": [0.6, max(f, 0.008), 0.5], "transform": {"translation": [sx, r(max(f, 0.008) / 2), 0.0]}, "member": P, "part": "support", "anchor": "pallet-floor"})

    if s in (50, 100):
        bp = box("base-pan", "base", (0.0006, 0.525, Y(0.020), Y(0.110), -ZC, ZC), "base-unit", 0.4)
        body.append(bp)
        th("base-pan", "base")
        mem("base-pan", "base")
        for k, x in enumerate(FEET_X):
            ft = revolve(f"foot-{'rear' if k == 0 else 'front'}", "base", [(0, 0.016), (0.020, 0.016)], (0, 1, 0), (x, Y(0.0), -0.250), "pp-grey", 0.3)
            ft["repeat"] = {"count": 2, "translation": [0, 0, 0.5]}
            body.append(ft)
            th(ft["id"], "base")
            mem(ft["id"], "base", part="joint")
            ev.append(shape(ft["id"], "base", erect("base", x - 0.016, x + 0.016, Y(0.0), Y(0.020)), "body", 1.5, True, 0.3, region="base", material="pp-grey"))
        for z in (-0.25, 0.25):
            fv.append(shape(f"foot-front-{'r' if z < 0 else 'l'}", "base", frect(z - 0.016, z + 0.016, Y(0.0), Y(0.020)), "body", 1.5, True, 0.3, region="base", material="pp-grey"))
        ev.append(shape("base-pan", "base", erect("base", 0.0006, 0.525, Y(0.020), Y(0.110)), "body", 1.4, True, 0.4, region="base", material="base-unit"))
        ppts = [(x, Y(h)) for x, h in PLINTH_PTS]
        pl = extrude_xy("plinth", "plinth", ppts, -ZC, ZC, "pp-plinth", 0.85)
        body.append(pl)
        ev.append(shape("plinth", "plinth", epoly("plinth", ppts), "body", 1.6, True, 0.85, region="plinth", material="pp-plinth"))
    else:
        bp = box("base-pan", "base", (0.0006, 0.510, Y(0.0), Y(0.110), -ZC, ZC), "base-unit", 0.4)
        body.append(bp)
        th("base-pan", "base")
        mem("base-pan", "base")
        ev.append(shape("base-pan", "base", erect("base", 0.0006, 0.510, Y(0.0), Y(0.110)), "body", 1.4, True, 0.4, region="base", material="base-unit"))
        fv.append(shape("base-pan", "base", frect(-ZC, ZC, Y(0.0), Y(0.110)), "body", 1.4, True, 0.4, region="base", material="base-unit"))
        pl = box("plinth", "plinth", (0.510, 0.545, Y(0.010), Y(0.125), -ZC, ZC), "pp-plinth", 0.85)
        body.append(pl)
        ev.append(shape("plinth", "plinth", erect("plinth", 0.510, 0.545, Y(0.010), Y(0.125)), "body", 1.6, True, 0.85, region="plinth", material="pp-plinth"))
    th("plinth", "plinth")
    fv.append(shape("plinth", "plinth", frect(-ZC, ZC, Y(0.010), Y(0.125)), "body", 3.0, True, 0.85, region="plinth", material="pp-plinth"))

    for side, z0, z1 in (("side-l", ZC, 0.300), ("side-r", -0.300, -ZC)):
        sp = box(side, side, (0.0, 0.545, Y(0.020), Y(0.820), z0, z1), "painted-steel", 0.88, sheet=True)
        body.append(sp)
        th(side, "cabinet")
        mem(side, side)
    ev.append(shape("side-r", "side-r", erect("side-r", 0.0, 0.545, Y(0.020), Y(0.820)), "body", 2.0, True, 0.88, region="cabinet", material="painted-steel"))
    bk_ = box("back", "back", (0.0, 0.0006, Y(0.020), Y(0.820), -ZC, ZC), "painted-steel", 0.8, sheet=True)
    body.append(bk_)
    th("back", "cabinet")
    mem("back", "back")
    fv.append(shape("cabinet", "back", frect(-0.300, 0.300, Y(0.020), Y(0.820)), "body", 2.0, True, 0.88, region="cabinet", material="painted-steel"))

    if s == 50:
        wt = {"id": "worktop", "kind": "extrude", "bone": "worktop", "material": "laminate-chipboard", "role": "body", "tone": 0.75, "profile": [[r(z), r(x)] for x, z in WORKTOP_PTS], "plane": "zx", "from": 0.0, "to": 0.03}
        pv.append(shape("worktop", "worktop", ppoly(WORKTOP_PTS), "body", 5.0, True, 0.75, region="worktop", material="laminate-chipboard"))
    else:
        wt = box("worktop", "worktop", (0.0, 0.600, Y(0.820), Y(0.850), -0.300, 0.300), "laminate-chipboard", 0.75)
        pv.append(shape("worktop", "worktop", prect(0.0, 0.600, -0.300, 0.300), "body", 5.0, True, 0.75, region="worktop", material="laminate-chipboard"))
    body.append(wt)
    th("worktop", "worktop")
    mem("worktop", "worktop")
    ev.append(shape("worktop", "worktop", erect("worktop", 0.0, 0.600, Y(0.820), Y(0.850)), "body", 3.0, True, 0.75, region="worktop", material="laminate-chipboard"))
    fv.append(shape("worktop", "worktop", frect(-0.300, 0.300, Y(0.820), Y(0.850)), "body", 5.0, True, 0.75, region="worktop", material="laminate-chipboard"))

    tub = "tub"
    fx0 = {50: 0.537, 100: 0.530, 200: 0.515}[s]
    walls = [
        ("tub-floor", (0.045, fx0, Y(0.1245), Y(0.125), -0.2755, 0.2755)),
        ("tub-top", (0.045, fx0, Y(0.780), Y(0.7805), -0.2755, 0.2755)),
        ("tub-side-r", (0.045, fx0, Y(0.1245), Y(0.7805), -0.2755, -0.275)),
        ("tub-side-l", (0.045, fx0, Y(0.1245), Y(0.7805), 0.275, 0.2755)),
        ("tub-back", (0.0445, 0.045, Y(0.1245), Y(0.7805), -0.2755, 0.2755)),
    ]
    for sid, bb_ in walls:
        sol = box(sid, tub, bb_, "stainless-steel", 0.55, sheet=True)
        body.append(sol)
        thc(sol, "tub")
    frame_pts = [(-ZC, 0.125), (-0.275, 0.125), (-0.275, 0.780), (0.275, 0.780), (0.275, 0.125), (ZC, 0.125), (ZC, 0.820), (-ZC, 0.820)]
    fr = {"id": "tub-frame", "kind": "extrude", "bone": tub, "material": "tub-frame", "role": "body", "tone": 0.5, "profile": [[r(Y(h) - Y(0.125)), r(z)] for z, h in frame_pts], "plane": "yz", "from": r(fx0 - 0.045), "to": 0.5}
    body.append(fr)
    thc(fr, "tub")
    mem("tub-floor", tub)
    mem("tub-side-r", tub)
    mem("tub-side-l", tub)
    mem("tub-frame", tub)
    fv.append(shape("tub-back", tub, frect(-0.275, 0.275, Y(0.125), Y(0.780)), "body", 2.2, True, 0.3, region="tub", material="stainless-steel"))
    fv.append(shape("tub-frame", tub, fpoly([(z, Y(h)) for z, h in frame_pts]), "body", 2.3, True, 0.5, region="tub", material="tub-frame"))
    if s in (50, 100):
        rz = 0.262 if s == 50 else 0.260
        for side, z0, z1 in (("r", -0.275, -rz), ("l", rz, 0.275)):
            rl = box(f"rail-{side}", tub, (0.060, 0.520, Y(0.490), Y(0.510), z0, z1), "rail-steel", 0.4, role="detail")
            body.append(rl)
            thc(rl, "tub")
            mem(rl["id"], tub)
        fv.append(shape("rail-r", tub, frect(-0.275, -rz, Y(0.490), Y(0.510)), "detail", 2.4, True, 0.4, region="tub", material="rail-steel"))
        fv.append(shape("rail-l", tub, frect(rz, 0.275, Y(0.490), Y(0.510)), "detail", 2.4, True, 0.4, region="tub", material="rail-steel"))
    if s in (50, 100):
        for side, z0, z1 in (("r", -0.2795, -0.2755), ("l", 0.2755, 0.2795)):
            ins = box(f"mat-{side}", tub, (0.060, 0.520, Y(0.150), Y(0.760), z0, z1), "bitumen-mat", 0.15, sheet=True)
            body.append(ins)
            thc(ins, "tub")
        ins = box("mat-top", tub, (0.060, 0.520, Y(0.7805), Y(0.7845), -0.250, 0.250), "bitumen-mat", 0.15, sheet=True)
        body.append(ins)
        thc(ins, "tub")

    lower_basket(s, body, thermal, ev, fv, pv)
    upper_basket(s, body, thermal, ev, fv, pv)
    spray_arms(s, body, thermal)
    for sid in ("bl-floor-rail" if s != 200 else "bl-floor-rail",):
        mem(sid, "basket-lower")
    mem("bl-top-rail", "basket-lower")
    if s != 200:
        mem("bl-post-r", "basket-lower")
        mem("bl-post-l", "basket-lower")
        mem("bl-wheel-r", "basket-lower", part="joint")
        mem("bl-wheel-l", "basket-lower", part="joint")
    else:
        mem("bl-post-r", "basket-lower", part="joint")
        mem("bl-post-l", "basket-lower", part="joint")
    mem("bu-runner", "basket-upper")
    mem("bu-floor-rail", "basket-upper")
    mem("bu-floor-end", "basket-upper")
    mem("bu-hanger-r", "basket-upper", part="joint")
    mem("bu-hanger-l", "basket-upper", part="joint")

    door = "door"
    leaf = box("door-leaf", door, (0.545, 0.600, Y(0.125), Y(0.735), *DOOR_Z), "door-composite", 0.92)
    body.append(leaf)
    th("door-leaf", "door")
    mem("door-leaf", door)
    ev.append(shape("door-leaf", door, erect(door, 0.545, 0.600, Y(0.125), Y(0.735)), "body", 4.0, True, 0.92, region="door", material="door-composite"))
    fv.append(shape("door-leaf", door, erect(door, 0.545, 0.600, Y(0.125), Y(0.735)), "body", 4.0, True, 0.92, region="door", material="door-composite", **sweep_w(*DOOR_Z)))
    pv.append(shape("door-leaf", door, erect(door, 0.545, 0.600, Y(0.125), Y(0.815)), "body", 4.0, True, 0.92, region="door", material="door-composite", **sweep_w(*DOOR_Z)))
    if s in (50, 100):
        pts = PANEL_PTS
    else:
        pts = [(0.548, 0.735), (0.600, 0.735), (0.600, 0.815), (0.548, 0.815)]
    pn = extrude_xy("panel", door, [(x, Y(h)) for x, h in pts], *DOOR_Z, "panel-abs", 0.8)
    body.append(pn)
    th("panel", "door")
    ev.append(shape("panel", door, epoly(door, [(x, Y(h)) for x, h in pts]), "body", 4.1, True, 0.8, region="door", material="panel-abs"))
    fv.append(shape("panel", door, epoly(door, [(x, Y(h)) for x, h in pts]), "body", 4.1, True, 0.8, region="door", material="panel-abs", **sweep_w(*DOOR_Z)))
    if s == 50:
        fv.append(shape("grip-recess", door, dface(0.735, 0.760, -0.27, 0.27), "detail", 4.2, True, 0.25, region="door", material="panel-abs", **sweep_v(0.5995, 0.600)))
    if s in (50, 100):
        kh, kz, kr = KNOB
        kn = revolve("knob", door, [(0, kr), (0.018, kr), (0.022, kr - 0.004)], (1, 0, 0), (0.600, Y(kh), kz), "knob-abs", 0.45)
        body.append(kn)
        th("knob", "door")
        ev.append(shape("knob", door, erect(door, 0.600, 0.622, Y(kh - kr), Y(kh + kr)), "body", 4.3, True, 0.45, region="door", material="knob-abs"))
        fv.append(shape("knob", door, dface_circle(kh, kz, kr), "body", 4.3, True, 0.45, region="door", material="knob-abs", **sweep_v(0.600, 0.622)))
        pv.append(shape("knob", door, dface_circle(kh, kz, kr), "body", 4.3, True, 0.45, region="door", material="knob-abs", **sweep_v(0.600, 0.622)))
    for side, z0, z1 in (("r", -0.296, -0.284), ("l", 0.284, 0.296)):
        hz = max(f, 0.012)
        jt = box(f"hinge-{side}", door, (0.545 - hz / 2, 0.545 + hz / 2, Y(0.125 - hz / 2), Y(0.125 + hz / 2), z0 if side == "r" else z1 - hz, z0 + hz if side == "r" else z1), "painted-steel", 0.4, role="detail")
        jt.update({"member": door, "part": "joint", "id": f"j-hinge-{side}"})
        struct.append(jt)

    ems = box("th-door-steam", door, (0.550, 0.550 + max(f, 0.03), Y(0.815 - f), Y(0.815), -0.20, 0.20), "door-composite", 0.5, role="detail", tags=["door-steam"])
    ems["region"] = "door"
    thermal.append(ems)

    for cid, (xc, hc, zc, R, t) in COILS.items():
        mat = "pvc-cable" if cid == "cable-coil" else "pvc-hose"
        co = revolve(cid, "hoses", torus_profile(R, t), (1, 0, 0), (xc, Y(hc), zc), mat, 0.45, slender=True)
        body.append(co)
        th(cid, "hoses")
        if s != 200:
            ev.append(shape(cid, "hoses", erect("hoses", xc - t, 0.0, Y(hc - R - t), Y(hc + R + t)), "body", 2.5, True, 0.45, region="hoses", material=mat))
            pv.append(shape(cid, "hoses", prect(xc - t, 0.0, zc - R - t, zc + R + t), "body", 2.0, True, 0.45, region="hoses", material=mat))
    for sid, (x0, x1, h0, h1, z0, z1) in (("aquastop", AQUASTOP), ("plug", PLUG)):
        sol = box(sid, "hoses", (x0, x1, Y(h0), Y(h1), z0, z1), "pp-grey", 0.4)
        body.append(sol)
        th(sid, "hoses")
        if s != 200:
            ev.append(shape(sid, "hoses", erect("hoses", x0, x1, Y(h0), Y(h1)), "body", 2.6, True, 0.4, region="hoses", material="pp-grey"))
            pv.append(shape(sid, "hoses", prect(x0, x1, z0, z1), "body", 2.6, True, 0.4, region="hoses", material="pp-grey"))
    if s == 200:
        ev.append(shape("hose-bundle", "hoses", epoly("hoses", [(0.0, Y(0.160)), (-0.045, Y(0.160)), (-0.045, Y(0.260)), (-0.024, Y(0.260)), (-0.024, Y(0.747)), (0.0, Y(0.747))]), "body", 2.5, True, 0.45, region="hoses", material="pvc-hose"))
        pv.append(shape("hose-bundle", "hoses", ppoly([(0.0, -0.240), (-0.045, -0.240), (-0.045, -0.170), (-0.024, -0.170), (-0.024, 0.274), (0.0, 0.274)]), "body", 2.0, True, 0.45, region="hoses", material="pvc-hose"))

    if s == 50:
        for k, (x0, x1, h, z0, z1) in enumerate(((-0.025, 0.0, 0.747, -0.085, -0.035), (-0.009, 0.0, 0.714, 0.180, 0.220))):
            tp = box(f"tape-{k + 1}", "hoses", (x0, x1, Y(h), Y(h + 0.0003), z0, z1), "packing-tape", 0.55, role="texture", texture=True)
            body.append(tp)
            pv.append(shape(f"tape-{k + 1}", "hoses", prect(x0, x1, z0, z1), "texture", 2.9, True, 0.55))
        for k, (zz, hh) in enumerate(((-0.26, 0.79), (-0.20, 0.79), (-0.14, 0.79), (-0.08, 0.79))):
            fv.append(shape(f"button-{k + 1}", door, dface(hh - 0.006, hh + 0.006, zz - 0.015, zz + 0.015), "detail", 4.4, True, 0.6, **sweep_v(0.5995, 0.600)))
        fv.append(shape("display", door, dface(0.770, 0.805, 0.02, 0.12), "detail", 4.4, True, 0.2, **sweep_v(0.5995, 0.600)))
        for k, (hh, zz) in enumerate(((0.14, -0.21), (0.145, -0.17), (0.138, 0.05), (0.15, 0.19), (0.142, 0.23))):
            fv.append(shape(f"rust-{k + 1}", door, dface_line([(hh, zz - 0.006), (hh + 0.004, zz), (hh - 0.002, zz + 0.006)]), "texture", 4.5, False, 0.15, **sweep_v(0.5995, 0.600)))
        fv.append(shape("scratch-grip", door, dface_line([(0.70, -0.06), (0.715, 0.02), (0.705, 0.09)]), "texture", 4.5, False, 0.2, **sweep_v(0.5995, 0.600)))
        fv.append(shape("limescale-run", door, dface_line([(0.20, 0.12), (0.17, 0.125), (0.13, 0.12)]), "texture", 4.5, False, 0.3, **sweep_v(0.5995, 0.600)))
        ev.append(shape("dent-1", "side-r", eline("side-r", [(0.26, Y(0.52)), (0.30, Y(0.545)), (0.34, Y(0.52))]), "texture", 2.1, False, 0.3))
        ev.append(shape("dent-2", "side-r", eline("side-r", [(0.275, Y(0.51)), (0.30, Y(0.525)), (0.325, Y(0.51))]), "texture", 2.1, False, 0.3))
        for k, (xa, xb, h) in enumerate(((0.42, 0.50, 0.06), (0.05, 0.12, 0.09), (0.18, 0.24, 0.07))):
            ev.append(shape(f"scuff-{k + 1}", "side-r", eline("side-r", [(xa, Y(h)), (xb, Y(h + 0.008))]), "texture", 2.1, False, 0.3))
        for k, (x, h) in enumerate(((0.52, 0.035), (0.535, 0.048), (0.505, 0.030))):
            ev.append(shape(f"side-rust-{k + 1}", "side-r", eline("side-r", [(x - 0.006, Y(h)), (x, Y(h + 0.004)), (x + 0.006, Y(h - 0.002))]), "texture", 2.1, False, 0.15))
        pv.append(shape("ring-mark-1", "worktop", {"kind": "ellipse", "cx": 0.34, "cy": 0.12, "rx": 0.055, "ry": 0.055}, "texture", 5.1, False, 0.3))
        pv.append(shape("ring-mark-2", "worktop", ppoly([(0.40, 0.06), (0.43, 0.09), (0.44, 0.13), (0.42, 0.17)], closed=False), "texture", 5.1, False, 0.3))
        for k, ((xa, za), (xb, zb)) in enumerate((((0.10, -0.20), (0.28, -0.12)), ((0.15, 0.20), (0.30, 0.24)), ((0.48, 0.05), (0.56, -0.02)))):
            pv.append(shape(f"top-scratch-{k + 1}", "worktop", ppoly([(xa, za), (xb, zb)], closed=False), "texture", 5.1, False, 0.25))
        pv.append(shape("label-remnant", "worktop", prect(0.08, 0.18, -0.27, -0.21), "texture", 5.1, True, 0.9))
        pv.append(shape("price-tag", "worktop", prect(0.47, 0.55, 0.16, 0.26), "texture", 5.1, True, 0.97))
        pv.append(shape("price-tag-line", "worktop", ppoly([(0.49, 0.18), (0.49, 0.24)], closed=False), "texture", 5.2, False, 0.2))
        pv.append(shape("label-tear", "worktop", ppoly([(0.18, -0.27), (0.165, -0.25), (0.18, -0.235), (0.165, -0.21)], closed=False), "texture", 5.2, False, 0.2))
    elif s == 100:
        pv.append(shape("ring-mark-1", "worktop", {"kind": "ellipse", "cx": 0.34, "cy": 0.12, "rx": 0.055, "ry": 0.055}, "texture", 5.1, False, 0.3))
        ev.append(shape("dent-1", "side-r", eline("side-r", [(0.26, Y(0.52)), (0.30, Y(0.545)), (0.34, Y(0.52))]), "texture", 2.1, False, 0.3))

    mr = max(f, 0.03)
    struct.append({"id": "mass-machine", "kind": "ellipsoid", "bone": "body", "material": "base-unit", "role": "detail", "tone": 0.3, "radii": [mr, mr, mr], "transform": {"translation": [0.30, 0.36, 0.0]}, "member": "base", "part": "mass"})
    return body, thermal, struct, ev, fv, pv


def poly_area_centroid(p):
    """📐 Area and centroid of a simple polygon."""
    a = cx = cy = 0.0
    for i in range(len(p)):
        x0, y0 = p[i]
        x1, y1 = p[(i + 1) % len(p)]
        c = x0 * y1 - x1 * y0
        a += c
        cx += (x0 + x1) * c
        cy += (y0 + y1) * c
    a /= 2
    return abs(a), (cx / (6 * a), cy / (6 * a)) if a else (0, 0)


def solid_volume(sol):
    """⚖️ Volume of a solid in cubic metres (boxes, extrusions, capsules, revolves by Pappus)."""
    n = sol.get("repeat", {}).get("count", 1)
    k = sol["kind"]
    if k == "box":
        a, b, c = sol["size"]
        return a * b * c * n
    if k == "extrude":
        return poly_area_centroid(sol["profile"])[0] * abs(sol["to"] - sol["from"]) * n
    if k == "capsule":
        L_ = math.dist(sol["p0"], sol["p1"])
        return (math.pi * sol["r0"] ** 2 * L_ + 4 / 3 * math.pi * sol["r0"] ** 3) * n
    if k == "ellipsoid":
        a, b, c = sol["radii"]
        return 4 / 3 * math.pi * a * b * c * n
    if k == "revolve":
        p = [tuple(q) for q in sol["profile"]]
        if p[0] == p[-1]:
            ring = p[:-1]
        else:
            ring = p + [(p[-1][0], 0.0), (p[0][0], 0.0)]
        a, (_, rc) = poly_area_centroid(ring)
        return 2 * math.pi * abs(rc) * a * n
    raise ValueError(k)


def solid_centre(sol):
    """🎯 The local (u, v) centre of a solid (copies averaged)."""
    k = sol["kind"]
    if k in ("box", "ellipsoid"):
        t = sol["transform"]["translation"]
        base = (t[0], t[1])
    elif k == "extrude" and sol["plane"] == "xy":
        base = poly_area_centroid(sol["profile"])[1]
    elif k == "extrude" and sol["plane"] == "yz":
        base = ((sol["from"] + sol["to"]) / 2, poly_area_centroid(sol["profile"])[1][0])
    elif k == "extrude":
        c = poly_area_centroid(sol["profile"])[1]
        base = (c[1], (sol["from"] + sol["to"]) / 2)
    elif k == "capsule":
        base = ((sol["p0"][0] + sol["p1"][0]) / 2, (sol["p0"][1] + sol["p1"][1]) / 2)
    else:
        o, ax = sol["origin"], sol["axis"]
        p = sol["profile"]
        mid = (min(q[0] for q in p) + max(q[0] for q in p)) / 2
        base = (o[0] + ax[0] * mid, o[1] + ax[1] * mid)
    rp = sol.get("repeat")
    if rp:
        n, t = rp["count"], rp.get("translation", [0, 0, 0])
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
MACHINE_KG = round(TOTAL_KG - MASS["pallet"][0], 1)
PALLET_KG = round(MASS["pallet"][0], 1)


def sec_rect(b, h, material, title):
    """▭ A rectangular member section b x h (h in the bending plane)."""
    return {"areaM2": float(f"{b * h:.4e}"), "secondMomentM4": float(f"{b * h ** 3 / 12:.4e}"), "extremeFibreM": r(h / 2), "material": material, "title": title}


def sec_flanges(area_each, d, material, title):
    """🪜 A section of two flanges of `area_each` a distance `d` apart."""
    return {"areaM2": float(f"{2 * area_each:.4e}"), "secondMomentM4": float(f"{2 * area_each * (d / 2) ** 2:.4e}"), "extremeFibreM": r(d / 2), "material": material, "title": title}


SECTIONS = {
    "side-l": sec_rect(0.545, SIDE_T, "painted-steel", T("Side panel 545 x 0.6 mm", "Seitenwand 545 x 0,6 mm")),
    "side-r": sec_rect(0.545, SIDE_T, "painted-steel", T("Side panel 545 x 0.6 mm", "Seitenwand 545 x 0,6 mm")),
    "back": sec_rect(0.5988, SIDE_T, "painted-steel", T("Back panel 599 x 0.6 mm", "Rückwand 599 x 0,6 mm")),
    "worktop": sec_rect(0.60, 0.03, "laminate-chipboard", T("Worktop 600 x 30 mm", "Arbeitsplatte 600 x 30 mm")),
    "base": sec_flanges(1.2e-4, 0.02, "painted-steel", T("Two folded base rails 40 x 20 x 1.5 mm", "Zwei gekantete Bodenschienen 40 x 20 x 1,5 mm")),
    "tub": sec_flanges(0.551 * TUB_T + 0.656 * TUB_T, 0.656, "stainless-steel", T("Welded tub box 551 x 656 mm, walls 0.5 mm", "Geschweißter Behälterkasten 551 x 656 mm, Wände 0,5 mm")),
    "door": sec_flanges(0.598 * 0.0005, 0.055, "door-composite", T("Door sandwich: two skins 598 x 0.5 mm, 55 mm apart", "Türsandwich: zwei Bleche 598 x 0,5 mm im Abstand 55 mm")),
    "basket-lower": sec_flanges(2 * math.pi * 0.0035 ** 2, 0.163, "coated-wire", T("Basket rims: two pairs of 7 mm wires 163 mm apart", "Korbränder: zwei Paare 7-mm-Drähte im Abstand 163 mm")),
    "basket-upper": sec_flanges(2 * math.pi * 0.0035 ** 2, 0.168, "coated-wire", T("Basket rims: two pairs of 7 mm wires 168 mm apart", "Korbränder: zwei Paare 7-mm-Drähte im Abstand 168 mm")),
    "pallet": sec_flanges(0.5 * 0.022, 0.122, "pallet-spruce", T("Pallet decks: top 5 x 100 x 22 mm and bottom boards, 122 mm apart", "Palettendecks: oben 5 x 100 x 22 mm und Bodenbretter im Abstand 122 mm")),
}


def bone_json(bid, parent, x, y, z, ang, length, joint, title):
    """🦴 One bone with its mass from the 1:50 solids and its member section."""
    b = {"id": bid, "parent": parent, "title": title, "x": r(x), "y": r(y)}
    if z:
        b["z"] = r(z)
    b.update({"angleDeg": ang, "lengthM": length, "joint": joint})
    if bid in MASS:
        m, su, sv = MASS[bid]
        b["mass"] = {"kg": round(m, 3), "comX": round(su / m, 4), "comY": round(sv / m, 4), "distribution": "point", "source": "authored-estimate", "reference": "used-appliance-market"}
    if bid in SECTIONS:
        b["section"] = SECTIONS[bid]
    return b


ANCHORS = [
    {"id": "foot-rear", "bone": "body", "x": FEET_X[0], "y": 0.0, "kind": "contact", "title": T("Rear feet on their base", "Hintere Füße auf ihrer Unterlage"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.032, "footprintWidthM": 0.532, "frictionCoefficient": 0.5},
    {"id": "foot-front", "bone": "body", "x": FEET_X[1], "y": 0.0, "kind": "contact", "title": T("Adjustable front feet on their base", "Verstellbare vordere Füße auf ihrer Unterlage"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.032, "footprintWidthM": 0.532, "frictionCoefficient": 0.5},
    {"id": "grip-rear", "bone": "body", "x": 0.0, "y": 0.0, "kind": "support", "title": T("Rear mover's hands under the back edge", "Hände des hinteren Trägers unter der Hinterkante"), "support": "pin", "footprintLengthM": 0.1, "footprintWidthM": 0.4},
    {"id": "grip-front", "bone": "body", "x": 0.525, "y": 0.0, "kind": "support", "title": T("Front mover's hands under the base behind the plinth", "Hände des vorderen Trägers unter der Bodenwanne hinter dem Sockel"), "support": "pin", "footprintLengthM": 0.1, "footprintWidthM": 0.4},
    {"id": "pallet-floor", "bone": "pallet", "x": 0.40, "y": 0.0, "kind": "contact", "title": T("Pallet on the floor", "Palette auf dem Boden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.8, "footprintWidthM": 0.3, "frictionCoefficient": 0.6},
    {"id": "door-gap", "bone": "door", "x": 0.69, "y": -0.02, "kind": "emitter", "title": T("Vapour leaving over the top of the ajar door", "Dampf, der über der Oberkante der angelehnten Tür austritt")},
]

REGION_DATA = [
    ("cabinet", T("Side panels and back panel (powder-coated steel)", "Seitenwände und Rückwand (pulverbeschichteter Stahl)"), "surface", "operating", "painted-steel", 1.36, 0.55, "vertical-plate", 0.5, ("sheet", T("Steel sheet 0.6 mm", "Stahlblech 0,6 mm"), "metal", SIDE_T), {"key": "heater", "heatW": 45}, "en-60335-2-5"),
    ("worktop", T("Removable laminate worktop", "Abnehmbare Schichtstoff-Arbeitsplatte"), "surface", "passive", "laminate-chipboard", 0.47, 0.6, "horizontal-plate-up", 0.8, ("board", T("Chipboard 30 mm", "Spanplatte 30 mm"), "timber", 0.03), None, "en-312"),
    ("door", T("Door front, control panel and programme knob", "Türfront, Bedienblende und Programmknebel"), "surface", "operating", "door-composite", 0.62, 0.69, "vertical-plate", 0.5, ("door-skin", T("Insulated door 55 mm", "Gedämmte Tür 55 mm"), "insulation", 0.055), {"key": "heater", "heatW": 25}, "en-60335-2-5"),
    ("plinth", T("Kick plinth", "Sockelblende"), "surface", "passive", "pp-plinth", 0.16, 0.115, "vertical-plate", 0.2, ("plinth-wall", T("Polypropylene 3 mm", "Polypropylen 3 mm"), "polymer", 0.003), None, "iso-10456-2007"),
    ("base", T("Base pan underside and feet", "Unterseite der Bodenwanne und Füße"), "surface", "passive", "base-unit", 0.52, 0.52, "horizontal-plate-down", 0.0, ("pan-wall", T("Polypropylene base pan 3 mm", "Bodenwanne aus Polypropylen 3 mm"), "polymer", 0.003), None, "iso-10456-2007"),
    ("tub", T("Wash tub, front frame, rails and sound-damping mats", "Spülbehälter, Frontrahmen, Schienen und Dämmmatten"), "surface", "operating", "stainless-steel", 4.9, 0.55, "vertical-plate", 0.0, ("tub-wall", T("Stainless steel 0.5 mm", "Edelstahl 0,5 mm"), "metal", TUB_T), {"key": "drying", "heatW": 250}, "en-60335-2-5"),
    ("racks", T("Baskets, cutlery basket and spray arms", "Körbe, Besteckkorb und Sprüharme"), "surface", "passive", "coated-wire", 0.61, 0.2, "cylinder", 0.0, ("wire", T("Coated wire 4 mm", "Beschichteter Draht 4 mm"), "metal", 0.004), None, "iso-10456-2007"),
    ("hoses", T("Coiled drain and inlet hoses, mains cable, Aquastop box and plug", "Aufgerollte Ablauf- und Zulaufschläuche, Netzleitung, Aquastop-Gehäuse und Stecker"), "surface", "passive", "pvc-hose", 0.16, 0.024, "cylinder", 0.3, ("hose-wall", T("Hose wall 2 mm", "Schlauchwand 2 mm"), "polymer", 0.002), None, "iso-10456-2007"),
    ("pallet", T("Düsseldorf half pallet", "Düsseldorfer Halbpalette"), "surface", "passive", "pallet-spruce", 1.87, 0.1, "horizontal-plate-up", 0.3, ("pallet-board", T("Spruce board 22 mm", "Fichtenbrett 22 mm"), "timber", 0.022), None, "en-13698-2"),
]
HIDDEN = {
    "tub": T("Inside the closed machine behind the door: only the open or ajar door shows it.", "Im Inneren der geschlossenen Maschine hinter der Tür: nur die offene oder angelehnte Tür zeigt ihn."),
    "racks": T("Inside the wash tub behind the closed door: the baskets show when the door is open and they are pulled out.", "Im Spülbehälter hinter der geschlossenen Tür: die Körbe zeigen sich bei offener Tür und herausgezogen."),
}
MAT = {m["id"]: m for m in MATERIALS}


def region(row):
    """🌡 One thermal region with its single layer."""
    rid, title, kind, model, mat, area, cl, geo, svf, (lid, ltitle, lkind, t), op, ref = row
    m = MAT[mat]
    layer = {"id": lid, "title": ltitle, "kind": lkind, "material": mat, "thicknessM": t, "resistanceM2KW": round(t / m["conductivityWmK"], 6), "heatCapacityJM2K": round(m["densityKgM3"] * m["specificHeatJKgK"] * t, 1), "source": "literature" if m["source"] == "literature" else "authored-estimate", "reference": "iso-10456-2007"}
    g = {"id": rid, "title": title, "kind": kind, "model": model, "material": mat, "areaM2": area, "characteristicLengthM": cl, "geometry": geo, "skyViewFactor": svf, "layers": [layer], "heatCapacityJM2K": layer["heatCapacityJM2K"]}
    if op:
        g["operating"] = op
    if rid == "cabinet":
        g["ratedPowerW"] = 2200
        g["ratedPowerBasis"] = "electrical-input"
    if rid in HIDDEN:
        g["hidden"] = HIDDEN[rid]
    g["source"] = "authored-estimate"
    g["reference"] = ref
    return g


REGIONS = [region(row) for row in REGION_DATA]
REGION_IDS = [g["id"] for g in REGIONS]


def binding(heater=0, drying=0, tub=0.0, racks=0.0, steam=0, pallet=None, base=None):
    """🌡 The thermal binding of a pose: every region and the emitter."""
    regions = {g: {"exposedFraction": 1.0, "sunlitFraction": 0.0} for g in REGION_IDS}
    regions["tub"] = {"exposedFraction": tub, "sunlitFraction": 0.0}
    regions["racks"] = {"exposedFraction": racks, "sunlitFraction": 0.0}
    regions["base"] = base or {"exposedFraction": 0.9, "sunlitFraction": 0.0}
    regions["pallet"] = pallet or {"exposedFraction": 0.85, "sunlitFraction": 0.0, "contact": {"fraction": 0.15, "tempC": 19}}
    regions["hoses"] = {"exposedFraction": 0.9, "sunlitFraction": 0.0}
    return {"operating": {"heater": heater, "drying": drying}, "regions": regions, "emitters": {"door-steam": steam}}


def dist_load(lid, bone, length, kg, kind, title, ref):
    """🍽 A distributed gravity load along a bone."""
    return {"id": lid, "kind": kind, "title": title, "distributed": {"bone": bone, "fromM": 0, "toM": length, "wxNPerM": 0, "wyNPerM": round(-kg * G / length, 1)}, "fxN": 0, "fyN": 0, "source": "authored-estimate", "reference": ref}


DISHES = [dist_load("dishes-lower", "basket-lower", 0.465, 7.0, "live", T("Plates, pots and the cutlery of 12 place settings in the lower basket, 7 kg", "Teller, Töpfe und das Besteck von 12 Maßgedecken im Unterkorb, 7 kg"), "en-50242"),
          dist_load("dishes-upper", "basket-upper", 0.455, 4.0, "live", T("Cups, glasses and bowls in the upper basket, 4 kg", "Tassen, Gläser und Schalen im Oberkorb, 4 kg"), "en-50242")]
WATER = {"id": "wash-water", "kind": "equipment", "title": T("Wash water in the sump and in circulation, 3.5 kg", "Spülwasser im Sumpf und im Umlauf, 3,5 kg"), "bone": "base", "x": 0.30, "y": 0.10, "fxN": 0, "fyN": round(-3.5 * G, 1), "source": "authored-estimate", "reference": "en-50242"}
HOME = {"lift": {"dy": -PAL}, "pallet": {"dx": -0.95}}
HOME_PALLET = {"exposedFraction": 0.85, "sunlitFraction": 0.0, "contact": {"fraction": 0.15, "tempC": 21}}
HOME_BASE = {"exposedFraction": 0.6, "sunlitFraction": 0.0, "contact": {"fraction": 0.05, "tempC": 21}}
OPEN = {"door": {"angleDeg": -90}, "basket-lower": {"dx": 0.40}, "basket-upper": {"dx": 0.30}}

POSES = [
    {"id": "stored", "title": T("Stored at the reuse depot", "Gelagert im Gebrauchtwarenlager"), "description": T("Emptied, dried, wiped and standing on a Düsseldorf half pallet in the depot aisle; the drain hose, the inlet hose with its Aquastop box and the mains cable are coiled and taped to the back panel, the door is shut on its latch.", "Geleert, getrocknet, abgewischt und auf einer Düsseldorfer Halbpalette im Lagergang stehend; Ablaufschlauch, Zulaufschlauch mit Aquastop-Gehäuse und Netzleitung sind aufgerollt und an die Rückwand geklebt, die Tür ist eingerastet."), "bones": {}, "ground": "authored", "contacts": ["foot-rear", "foot-front", "pallet-floor"], "thermal": binding(), "loads": []},
    {"id": "carried", "title": T("Carried by two movers", "Von zwei Trägern getragen"), "description": T("Lifted 0.10 m off the pallet by two movers, one with the hands under the back edge, one under the base pan behind the plinth, and tilted 6 degrees back so that the last water in the sump stays inside; the pallet stays on the floor.", "Von zwei Trägern 0,10 m von der Palette gehoben, einer mit den Händen unter der Hinterkante, einer unter der Bodenwanne hinter dem Sockel, und 6 Grad nach hinten geneigt, damit das letzte Wasser im Sumpf drinbleibt; die Palette bleibt auf dem Boden."), "bones": {"lift": {"dy": 0.10}, "body": {"angleDeg": 6}}, "ground": "authored", "contacts": ["grip-rear", "grip-front", "pallet-floor"], "thermal": binding(), "loads": []},
    {"id": "in-use", "title": T("Washing in a kitchen", "Spült in einer Küche"), "description": T("Back in a kitchen on its own feet, connected and loaded, the door closed: the heater brings the wash water to 50 degC while the spray arms turn; the coiled excess hose stays taped behind the machine, the pallet stands behind it.", "Wieder in einer Küche auf eigenen Füßen, angeschlossen und beladen, die Tür geschlossen: die Heizung bringt das Spülwasser auf 50 degC, während die Sprüharme drehen; der aufgerollte Schlauchüberstand bleibt hinter der Maschine geklebt, die Palette steht dahinter."), "bones": HOME, "ground": "authored", "contacts": ["foot-rear", "foot-front", "pallet-floor"], "thermal": binding(heater=1, base=HOME_BASE, pallet=HOME_PALLET), "loads": DISHES + [WATER]},
    {"id": "door-open", "title": T("Loading with the door down", "Beladen mit offener Tür"), "description": T("The door is let down flat on its hinges and both baskets are pulled out, the lower one rolling onto the inner face of the door, to load the dishes.", "Die Tür ist flach auf ihren Scharnieren heruntergeklappt und beide Körbe sind herausgezogen, der untere rollt auf die Innenseite der Tür, um das Geschirr einzuräumen."), "bones": {**HOME, **OPEN}, "ground": "authored", "contacts": ["foot-rear", "foot-front", "pallet-floor"], "thermal": binding(tub=0.6, racks=0.8, base=HOME_BASE, pallet=HOME_PALLET), "loads": DISHES},
    {"id": "drying-ajar", "title": T("Drying with the door ajar", "Trocknen mit angelehnter Tür"), "description": T("After the programme the door is opened 10 degrees: the residual heat of the tub and the dishes dries them and warm vapour leaves over the top of the door.", "Nach dem Programm ist die Tür 10 Grad geöffnet: die Restwärme von Behälter und Geschirr trocknet sie, und warmer Dampf tritt über der Oberkante der Tür aus."), "bones": {**HOME, "door": {"angleDeg": -10}}, "ground": "authored", "contacts": ["foot-rear", "foot-front", "pallet-floor"], "thermal": binding(drying=1, tub=0.15, racks=0.1, steam=1, base=HOME_BASE, pallet=HOME_PALLET), "loads": DISHES},
]

CLOSED_KEY = {"door": {"angleDeg": 0}, "basket-lower": {"dx": 0}, "basket-upper": {"dx": 0}}
HOME_C = ["foot-rear", "foot-front", "pallet-floor"]
CLIPS = [
    {"id": "carry", "title": T("Carry out of the depot", "Aus dem Lager tragen"), "description": T("The two movers take hold under the back edge and under the base pan, lift the machine off the pallet and tilt it back 6 degrees for the walk to the van; the pallet stays behind.", "Die zwei Träger fassen unter die Hinterkante und unter die Bodenwanne, heben die Maschine von der Palette und neigen sie für den Weg zum Transporter 6 Grad nach hinten; die Palette bleibt stehen."), "durationS": 6.0, "fps": 25, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1.0, "pose": "stored", "ease": "smooth"}, {"t": 3.0, "pose": "carried", "ease": "smooth", "bones": {"body": {"angleDeg": 0}}}, {"t": 5.0, "pose": "carried", "ease": "smooth"}, {"t": 6.0, "pose": "carried", "ease": "smooth"}],
     "contacts": [{"anchor": "foot-rear", "fromS": 0, "toS": 1.0}, {"anchor": "foot-front", "fromS": 0, "toS": 1.0}, {"anchor": "grip-rear", "fromS": 1.0, "toS": 6.0}, {"anchor": "grip-front", "fromS": 1.0, "toS": 6.0}, {"anchor": "pallet-floor", "fromS": 0, "toS": 6.0}],
     "phases": [{"id": "take-hold", "fromS": 0, "toS": 1.0, "title": T("Taking hold", "Anfassen")}, {"id": "lift", "fromS": 1.0, "toS": 3.0, "title": T("Lifting off the pallet", "Von der Palette heben")}, {"id": "tilt", "fromS": 3.0, "toS": 5.0, "title": T("Tilting back", "Nach hinten neigen")}, {"id": "carry", "fromS": 5.0, "toS": 6.0, "title": T("Carrying", "Tragen")}],
     "events": [{"t": 1.0, "kind": "contact-off", "title": T("Off the pallet", "Von der Palette"), "target": "foot-front"}, {"t": 1.0, "kind": "contact-on", "title": T("Hands take the load", "Hände übernehmen die Last"), "target": "grip-front"}, {"t": 3.0, "kind": "motion", "title": T("Lifted 0.10 m", "0,10 m angehoben")}],
     "tracks": []},
    {"id": "load-cycle", "title": T("Door down, baskets out and back", "Tür auf, Körbe heraus und zurück"), "description": T("In the kitchen the door is let down, the lower and then the upper basket are pulled out for loading, pushed back in and the door is closed again.", "In der Küche wird die Tür heruntergeklappt, erst der Unter- und dann der Oberkorb zum Beladen herausgezogen, wieder hineingeschoben und die Tür geschlossen."), "durationS": 10.0, "fps": 25, "loop": True, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "door-open", "ease": "smooth", "bones": CLOSED_KEY}, {"t": 2.0, "pose": "door-open", "ease": "smooth", "bones": {"basket-lower": {"dx": 0}, "basket-upper": {"dx": 0}}}, {"t": 3.5, "pose": "door-open", "ease": "smooth", "bones": {"basket-upper": {"dx": 0}}}, {"t": 5.0, "pose": "door-open", "ease": "smooth"}, {"t": 6.5, "pose": "door-open", "ease": "smooth"}, {"t": 8.0, "pose": "door-open", "ease": "smooth", "bones": {"basket-lower": {"dx": 0}, "basket-upper": {"dx": 0}}}, {"t": 10.0, "pose": "door-open", "ease": "smooth", "bones": CLOSED_KEY}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 10.0} for a in HOME_C],
     "phases": [{"id": "door-down", "fromS": 0, "toS": 2.0, "title": T("Letting the door down", "Tür herunterklappen")}, {"id": "baskets-out", "fromS": 2.0, "toS": 5.0, "title": T("Pulling the baskets out", "Körbe herausziehen")}, {"id": "loading", "fromS": 5.0, "toS": 6.5, "title": T("Loading", "Beladen")}, {"id": "baskets-in", "fromS": 6.5, "toS": 8.0, "title": T("Pushing the baskets in", "Körbe hineinschieben")}, {"id": "door-up", "fromS": 8.0, "toS": 10.0, "title": T("Closing the door", "Tür schließen")}],
     "events": [{"t": 0.0, "kind": "motion", "title": T("Door unlatched", "Tür entriegelt"), "target": "door"}, {"t": 2.0, "kind": "motion", "title": T("Lower basket rolls out", "Unterkorb rollt heraus"), "target": "basket-lower"}, {"t": 3.5, "kind": "motion", "title": T("Upper basket out", "Oberkorb heraus"), "target": "basket-upper"}, {"t": 10.0, "kind": "motion", "title": T("Door latched", "Tür eingerastet"), "target": "door"}],
     "tracks": [{"target": "region:tub:exposedFraction", "ease": "smooth", "keys": [{"t": 0, "v": 0.0}, {"t": 2.0, "v": 0.6}, {"t": 8.0, "v": 0.6}, {"t": 10.0, "v": 0.0}]}, {"target": "region:racks:exposedFraction", "ease": "smooth", "keys": [{"t": 0, "v": 0.0}, {"t": 2.0, "v": 0.4}, {"t": 5.0, "v": 0.8}, {"t": 6.5, "v": 0.8}, {"t": 8.0, "v": 0.4}, {"t": 10.0, "v": 0.0}]}]},
    {"id": "wash-spray", "title": T("Main wash: heating and spraying", "Hauptspülgang: Heizen und Sprühen"), "description": T("Door closed, the heater draws its 2 kW and the circulation pump drives the two spray arms one full turn in 4 s in opposite senses (15 turns a minute, the slow start of the wash phase).", "Tür geschlossen, die Heizung nimmt ihre 2 kW auf und die Umwälzpumpe dreht die zwei Sprüharme gegensinnig um eine volle Umdrehung in 4 s (15 Umdrehungen je Minute, der langsame Anlauf des Spülgangs)."), "durationS": 4.0, "fps": 25, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "in-use", "ease": "linear", "bones": {"spray-lower": {"twistDeg": 0}, "spray-upper": {"twistDeg": 0}}}, {"t": 2.0, "pose": "in-use", "ease": "linear", "bones": {"spray-lower": {"twistDeg": 180}, "spray-upper": {"twistDeg": -180}}}, {"t": 4.0, "pose": "in-use", "ease": "linear", "bones": {"spray-lower": {"twistDeg": 360}, "spray-upper": {"twistDeg": -360}}}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 4.0} for a in HOME_C],
     "phases": [{"id": "spraying", "fromS": 0, "toS": 4.0, "title": T("Heating and spraying", "Heizen und Sprühen")}],
     "events": [{"t": 0.0, "kind": "operating", "title": T("Heater on", "Heizung an"), "target": "heater"}],
     "tracks": [{"target": "operating:heater", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 4.0, "v": 1}]}]},
    {"id": "drying-open", "title": T("End of programme: door ajar, drying", "Programmende: Tür angelehnt, Trocknen"), "description": T("The last rinse has heated the tub; the heater is off, at 30 s the door is opened 10 degrees and the residual heat dries the dishes for ten minutes while warm vapour leaves over the door.", "Der letzte Klarspülgang hat den Behälter aufgeheizt; die Heizung ist aus, nach 30 s wird die Tür 10 Grad geöffnet und die Restwärme trocknet das Geschirr zehn Minuten lang, während warmer Dampf über der Tür austritt."), "durationS": 600.0, "fps": 1, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "drying-ajar", "ease": "smooth", "bones": {"door": {"angleDeg": 0}}}, {"t": 30.0, "pose": "drying-ajar", "ease": "smooth", "bones": {"door": {"angleDeg": 0}}}, {"t": 35.0, "pose": "drying-ajar", "ease": "smooth"}, {"t": 600.0, "pose": "drying-ajar", "ease": "smooth"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 600.0} for a in HOME_C],
     "phases": [{"id": "closed", "fromS": 0, "toS": 30.0, "title": T("Programme ended, door closed", "Programm beendet, Tür zu")}, {"id": "ajar", "fromS": 30.0, "toS": 600.0, "title": T("Door ajar, drying", "Tür angelehnt, Trocknen")}],
     "events": [{"t": 30.0, "kind": "motion", "title": T("Door opened 10 degrees", "Tür 10 Grad geöffnet"), "target": "door"}, {"t": 30.0, "kind": "thermal", "title": T("Vapour escapes", "Dampf tritt aus"), "target": "door-steam"}],
     "tracks": [{"target": "operating:drying", "ease": "smooth", "keys": [{"t": 0, "v": 1.0}, {"t": 30.0, "v": 1.0}, {"t": 300.0, "v": 0.5}, {"t": 600.0, "v": 0.2}]}, {"target": "emitter:door-steam", "ease": "smooth", "keys": [{"t": 0, "v": 0.0}, {"t": 30.0, "v": 0.0}, {"t": 40.0, "v": 1.0}, {"t": 300.0, "v": 0.5}, {"t": 600.0, "v": 0.15}]}, {"target": "region:tub:exposedFraction", "ease": "smooth", "keys": [{"t": 0, "v": 0.0}, {"t": 30.0, "v": 0.0}, {"t": 35.0, "v": 0.15}, {"t": 600.0, "v": 0.15}]}]},
]

REFERENCES = [
    {"id": "en-60335-2-5", "citation": "EN 60335-2-5 (IEC 60335-2-5): Household and similar electrical appliances - Safety - Part 2-5: Particular requirements for dishwashers (rated input, heating element, water connection, stability of appliances with a bottom-hinged door)."},
    {"id": "en-50242", "citation": "EN 50242 / EN 60436: Electric dishwashers for household use - Methods for measuring the performance (place settings, load of 12 standard place settings, programme phases, drying assessment)."},
    {"id": "din-18022", "citation": "DIN 18022:2007 Küchen, Bad, WC und Hausarbeitsraum im Wohnungsbau - Planungsgrundlagen (600 mm appliance module, worktop height 850-910 mm, 600 mm depth)."},
    {"id": "en-1116", "citation": "EN 1116:2004 Kitchen furniture - Coordinating sizes for kitchen furniture and kitchen appliances (60 cm module, 820-870 mm under-counter height, 600 mm depth)."},
    {"id": "en-10130", "citation": "EN 10130 cold-rolled low-carbon steel flat products for cold forming (DC01 0.6 mm cabinet sheet, 7850 kg/m3, E 210 GPa) with polyester powder coating."},
    {"id": "en-10088-2", "citation": "EN 10088-2 stainless steels - sheet and strip for general purposes (ferritic 1.4016 for wash tubs, 7700 kg/m3, E 200 GPa, 25 W/mK)."},
    {"id": "en-312", "citation": "EN 312:2010 Particleboards - Specifications (P2 boards, about 650-680 kg/m3, bending strength about 12 N/mm2, E about 2500 N/mm2) faced with high-pressure laminate for the removable worktop."},
    {"id": "en-338", "citation": "EN 338:2016 Structural timber - Strength classes (C24 spruce mean density 420 kg/m3, E 11 GPa, f_m 24 N/mm2), the timber of pallet boards and blocks."},
    {"id": "en-13698-2", "citation": "EN 13698-2:2003 Pallet production specification - Part 2: Construction specification for 1000 x 1200 mm flat wooden pallets, taken for the block-and-board build of the 800 x 600 mm Düsseldorf half pallet (5 top boards, 3 cross boards, 9 blocks, 3 bottom boards, 144 mm high)."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (steel 50 W/mK, stainless 17-25 W/mK, polypropylene and PVC 0.17-0.22 W/mK, bitumen 0.17 W/mK, particleboard 0.14 W/mK, softwood 0.13 W/mK, specific heats)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivity and solar absorptance of white paints, oxidised and polished stainless steel, plastics; free convection on vertical plates)."},
    {"id": "used-appliance-market", "citation": "Catalogue-typical 60 cm freestanding dishwashers of the German mid-range trade 2005-2015 (850 x 600 x 600 mm with removable worktop, 12 place settings, 2.0-2.4 kW connected load with a 2 kW heater, 44-50 kg, drop-down door with front control panel and programme knob, two baskets on wheels and rails, drain and inlet hoses 1.5 m with Aquastop) and the intake practice of municipal second-hand department stores (Gebrauchtwarenkaufhaus): tested, emptied, dried, hoses and cable taped to the back, stored on pallets; no brand data used."},
]


def scale_decisions(s):
    """📝 The bilingual design decisions of the 2D representation of 1:s."""
    D_ = {
        50: dict(
            silhouette=T("Full construction at 16 x 20 mm: the side panel with the 30 mm worktop over it, the 55 mm drop-down door with the control panel, its grip recess and the grey replacement knob, the recessed kick plinth and the feet, the hose coils, the Aquastop box and the plug proud of the back, and the Düsseldorf half pallet with its boards, cross boards and blocks.", "Volle Konstruktion auf 16 x 20 mm: die Seitenwand mit der 30-mm-Arbeitsplatte darüber, die 55-mm-Klapptür mit Bedienblende, Griffmulde und grauem Ersatzknebel, die zurückgesetzte Sockelblende und die Füße, die Schlauchringe, das Aquastop-Gehäuse und der Stecker hinter der Rückwand und die Düsseldorfer Halbpalette mit Deckbrettern, Querbrettern und Klötzen."),
            retained=T("Every panel at its true place, the 18 x 12 mm chip at the front right corner of the worktop, the tub with its front frame and rails, both wire baskets with wheels, rods and tines (they show when the door is down), the five top boards of the pallet with their 25 mm gaps.", "Jede Platte an ihrem wahren Ort, die 18 x 12 mm Abplatzung an der vorderen rechten Ecke der Arbeitsplatte, der Behälter mit Frontrahmen und Schienen, beide Drahtkörbe mit Rollen, Stäben und Zinken (sichtbar bei offener Tür), die fünf Deckbretter der Palette mit ihren 25-mm-Fugen."),
            merged=T("Base pan, pumps, heater and softener are one block; the door is one leaf with its control panel.", "Bodenwanne, Pumpen, Heizung und Enthärter sind ein Block; die Tür ist ein Blatt mit ihrer Bedienblende."),
            omitted=T("Screws, door seal, the inner door liner relief, the dosing compartment and the filter.", "Schrauben, Türdichtung, das Relief des Türinnenblechs, die Dosierkammer und das Sieb."),
            thinElements=T("The 0.6 mm side and back panels and the 0.5 mm tub walls keep their true thickness as declared sheets; wires and coils are drawn as strokes and strips at their true widths.", "Die 0,6-mm-Seiten- und Rückwände und die 0,5-mm-Behälterwände behalten als erklärte Platten ihre wahre Stärke; Drähte und Ringe sind als Striche und Streifen in wahrer Breite gezeichnet."),
            hatching=T("No hatches: panels read by tone, the pallet by its darker wood tone.", "Keine Schraffuren: die Platten lesen sich über den Ton, die Palette über ihren dunkleren Holzton."),
            styles=T("Body pen 0.5 mm, detail pen 0.25 mm for buttons, display, grip recess and the baskets, texture pen 0.13 mm for the traces of use: rust spots and a limescale run at the lower door edge, scratches by the grip, the half-torn price sticker of the depot, a dent, scuffs and rust on the right side panel, ring marks, scratches and the remnant of the energy label on the worktop, the parcel tape over the coils and the stamps of the pallet.", "Körperstift 0,5 mm, Detailstift 0,25 mm für Tasten, Anzeige, Griffmulde und die Körbe, Texturstift 0,13 mm für die Gebrauchsspuren: Rostpunkte und eine Kalkspur an der unteren Türkante, Kratzer an der Griffmulde, das halb abgerissene Preisetikett des Lagers, eine Delle, Schrammen und Rost an der rechten Seitenwand, Wasserränder, Kratzer und der Rest des Energielabels auf der Arbeitsplatte, das Packband über den Ringen und die Stempel der Palette."),
            overlays=T("Analysis views colour the same shapes by region and member; the door shapes are swept solids, so front and plan follow the door down.", "Die Analyseansichten färben dieselben Formen nach Region und Bauteil; die Türformen sind gezogene Körper, sodass Vorderansicht und Grundriss der Tür nach unten folgen.")),
        100: dict(
            silhouette=T("Dishwasher at 8 x 10 mm: cabinet, worktop, door with control panel, grip recess and knob, plinth, feet, coils behind and the pallet.", "Geschirrspüler auf 8 x 10 mm: Korpus, Arbeitsplatte, Tür mit Bedienblende, Griffmulde und Knebel, Sockel, Füße, Ringe dahinter und die Palette."),
            retained=T("All pallet boards at 22 mm, the 25 mm grip recess, the 42 mm knob, the baskets with fewer wires.", "Alle Palettenbretter mit 22 mm, die 25-mm-Griffmulde, der 42-mm-Knebel, die Körbe mit weniger Drähten."),
            merged=T("Basket wires reduced to every second one; rails widened to 15 mm.", "Korbdrähte auf jeden zweiten reduziert; Schienen auf 15 mm verbreitert."),
            omitted=T("Worktop chip, buttons, display, stickers, tape and most traces of use (under 15 mm).", "Abplatzung der Arbeitsplatte, Tasten, Anzeige, Etiketten, Klebeband und die meisten Gebrauchsspuren (unter 15 mm)."),
            thinElements=T("Panels stay declared sheets; wires and coils stay strokes and strips.", "Platten bleiben erklärte Platten; Drähte und Ringe bleiben Striche und Streifen."),
            hatching=T("No hatches.", "Keine Schraffuren."),
            styles=T("Body pen 0.35 mm, detail 0.18 mm, texture 0.13 mm for the ring mark and the dent.", "Körperstift 0,35 mm, Detail 0,18 mm, Textur 0,13 mm für den Wasserrand und die Delle."),
            overlays=T("As at 1:50.", "Wie in 1:50.")),
        200: dict(
            silhouette=T("Dishwasher sign 4 x 5 mm: cabinet with worktop, door with panel band, plinth block, base band, the coils behind and the pallet as deck and blocks.", "Geschirrspülerzeichen 4 x 5 mm: Korpus mit Arbeitsplatte, Tür mit Blendenband, Sockelblock, Bodenband, die Ringe dahinter und die Palette als Deck und Klötze."),
            retained=T("Outline 0.80 x 0.99 m with the pallet, the door and its panel, the baskets as rims.", "Umriss 0,80 x 0,99 m mit Palette, die Tür und ihre Blende, die Körbe als Ränder."),
            merged=T("Top boards and cross boards of the pallet into one deck, bottom boards into the blocks; feet into the base band; plinth into a 30 mm block.", "Deck- und Querbretter der Palette zu einem Deck, Bodenbretter in die Klötze; Füße in das Bodenband; Sockel zu einem 30-mm-Block."),
            omitted=T("Knob, grip recess, spray arms, rails, wheels and all traces of use.", "Knebel, Griffmulde, Sprüharme, Schienen, Rollen und alle Gebrauchsspuren."),
            thinElements=T("Panels and tub walls stay true sheets so the open door still shows the tub.", "Platten und Behälterwände bleiben wahre Platten, damit die offene Tür noch den Behälter zeigt."),
            hatching=T("No hatches.", "Keine Schraffuren."),
            styles=T("Body pen 0.25 mm, detail 0.13 mm.", "Körperstift 0,25 mm, Detail 0,13 mm."),
            overlays=T("As at 1:50.", "Wie in 1:50.")),
    }
    return D_[s]


VOLUME_DECISIONS = {
    50: [T("Body 1:50 is the machine as built and as used: the Düsseldorf half pallet board by board (five top boards, three cross boards, nine blocks, three bottom boards), four feet under the base block, the 0.6 mm side and back panels, the 30 mm worktop with its broken front corner, the 0.5 mm stainless tub with its 8 mm front frame, rails and bitumen mats, the L-section kick plinth, the 55 mm door with the control panel, its grip recess and the revolved replacement knob, both wire baskets rod by rod with wheels and tines, the cutlery basket, both spray arms on their hubs, the hose and cable coils as tori, the Aquastop box, the plug and the parcel tape (texture).", "Körper 1:50 ist die Maschine, wie sie gebaut und benutzt ist: die Düsseldorfer Halbpalette Brett für Brett (fünf Deckbretter, drei Querbretter, neun Klötze, drei Bodenbretter), vier Füße unter dem Bodenblock, die 0,6-mm-Seiten- und Rückwände, die 30-mm-Arbeitsplatte mit abgebrochener Vorderecke, der 0,5-mm-Edelstahlbehälter mit 8-mm-Frontrahmen, Schienen und Bitumenmatten, die L-förmige Sockelblende, die 55-mm-Tür mit Bedienblende, Griffmulde und gedrehtem Ersatzknebel, beide Drahtkörbe Stab für Stab mit Rollen und Zinken, der Besteckkorb, beide Sprüharme auf ihren Naben, die Schlauch- und Kabelringe als Tori, das Aquastop-Gehäuse, der Stecker und das Packband (Textur)."),
         T("Thermal 1:50 references the outer parts by region; tub and racks are their own solids (hidden regions inside the closed door). Structure 1:50: panels, worktop, base block, tub and door leaf are members on their bones, the basket rims are the basket members on their wheels, the pallet decks are its members on its blocks, the door hinges are joints, one support pad stands under the pallet.", "Thermik 1:50 verweist nach Regionen auf die Außenteile; Behälter und Körbe sind eigene Körper (verdeckte Regionen hinter der geschlossenen Tür). Tragwerk 1:50: Platten, Arbeitsplatte, Bodenblock, Behälter und Türblatt sind Bauteile auf ihren Knochen, die Korbränder sind die Korbbauteile auf ihren Rollen, die Palettendecks sind ihre Bauteile auf ihren Klötzen, die Türscharniere sind Gelenke, ein Auflagerklotz steht unter der Palette.")],
    100: [T("Body 1:100 keeps every pallet board (22 mm), the feet, the panels as declared sheets, the plinth, the tub with a 15 mm front frame, 15 mm rails and its bitumen mats as declared sheets, the door with its grip recess and knob, the baskets with half the rods and 15 mm wheels, the spray arms and the coils; the worktop chip, the buttons and the tape are below 15 mm and left out.", "Körper 1:100 behält jedes Palettenbrett (22 mm), die Füße, die Platten als erklärte Platten, den Sockel, den Behälter mit 15-mm-Frontrahmen, 15-mm-Schienen und seinen Bitumenmatten als erklärte Platten, die Tür mit Griffmulde und Knebel, die Körbe mit der Hälfte der Stäbe und 15-mm-Rollen, die Sprüharme und die Ringe; die Abplatzung der Arbeitsplatte, die Tasten und das Klebeband sind unter 15 mm und entfallen."),
          T("Thermal and structure 1:100 as 1:50 on the merged parts.", "Thermik und Tragwerk 1:100 wie 1:50 auf den zusammengefassten Teilen.")],
    200: [T("Body 1:200 merges the pallet into a 44 mm deck on nine blocks that take the bottom boards in, the feet into a base block down to the pallet, the plinth into a 30 mm block and the front frame into 30 mm; panels and tub walls stay true sheets because the door opens in the clips and shows the tub; the baskets are their rims and corner posts; knob, recess, spray arms, rails and wheels are below 30 mm and left out.", "Körper 1:200 fasst die Palette zu einem 44-mm-Deck auf neun Klötzen zusammen, die die Bodenbretter aufnehmen, die Füße zu einem Bodenblock bis zur Palette, den Sockel zu einem 30-mm-Block und den Frontrahmen auf 30 mm; Platten und Behälterwände bleiben wahre Platten, weil die Tür in den Clips aufgeht und den Behälter zeigt; die Körbe sind ihre Ränder und Eckstäbe; Knebel, Mulde, Sprüharme, Schienen und Rollen sind unter 30 mm und entfallen."),
          T("Thermal 1:200 references the merged body; structure 1:200 keeps the members of every load-bearing bone, the basket posts stand on the tub floor.", "Thermik 1:200 verweist auf den zusammengefassten Körper; Tragwerk 1:200 behält die Bauteile jedes tragenden Knochens, die Korbstäbe stehen auf dem Behälterboden.")],
}

PENS = {50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.7}, 100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5}, 200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350)}

representations, volumes = {}, {}
for s in (200, 100, 50):
    body, thermal, struct, ev, fv, pv = parts(s)
    representations[str(s)] = {"status": "authored", "denominator": s, "validRange": {"minDenominator": RANGES[s][0], "maxDenominator": RANGES[s][1]}, "pens": PENS[s], "hatches": [], "geometry": {"elevation": ev, "front": fv, "plan": pv}, "decisions": scale_decisions(s)}
    volumes[str(s)] = {"status": "authored", "denominator": s, "body": body, "thermal": thermal, "structure": struct, "decisions": VOLUME_DECISIONS[s]}
representations["500"] = {"status": "unavailable", "denominator": 500, "cause": "below-printable-feature",
                          "reason": T("At 1:500 the dishwasher on its pallet is 1.6 by 2.0 mm on paper, under the 3 mm a recognisable drawing needs; its 30 mm worktop and 55 mm door would be 0.06 and 0.11 mm, under the 0.2 mm office-laser minimum feature; draw it at 1:200.", "Bei 1:500 misst der Geschirrspüler auf seiner Palette 1,6 mal 2,0 mm auf dem Papier, unter den 3 mm, die eine erkennbare Zeichnung braucht; seine 30-mm-Arbeitsplatte und 55-mm-Tür wären 0,06 und 0,11 mm, unter dem Mindestmerkmal von 0,2 mm des Büro-Laserdrucks; bei 1:200 zeichnen."),
                          "limitingFeature": T("Door of 55 mm, the thickest part that tells the machine apart", "Tür von 55 mm, das dickste Teil, das die Maschine kenntlich macht"), "limitingSizeM": 0.055, "alternatives": [{"kind": "scale", "denominator": 200}]}
volumes["500"] = {"status": "unavailable", "denominator": 500}
representations = {k: representations[k] for k in ("50", "100", "200", "500")}
volumes = {k: volumes[k] for k in ("50", "100", "200", "500")}

de = lambda v: str(v).replace(".", ",")
asset = {
    "id": "used-dishwasher",
    "category": "appliances",
    "title": T("Used Dishwasher", "Gebrauchter Geschirrspüler"),
    "summary": T(f"A second-hand 60 cm freestanding dishwasher 850 x 600 x 600 mm of the German mid-range trade, {MACHINE_KG} kg, on a {PALLET_KG} kg Düsseldorf half pallet: white powder-coated cabinet with a dent and rust at the lower edges, a removable laminate worktop with ring marks and a broken corner, a drop-down door with a yellowed control panel and a grey replacement programme knob, two wire baskets with chipped coating, a limescaled stainless tub, and the drain hose, inlet hose and mains cable coiled and taped to the back; stored on the pallet, carried by two movers, washing at 2.2 kW, loaded with the door down and drying with the door ajar.", f"Ein gebrauchter 60-cm-Standgeschirrspüler 850 x 600 x 600 mm des deutschen Mittelpreissegments, {de(MACHINE_KG)} kg, auf einer {de(PALLET_KG)} kg schweren Düsseldorfer Halbpalette: weiß pulverbeschichteter Korpus mit Delle und Rost an den Unterkanten, eine abnehmbare Schichtstoff-Arbeitsplatte mit Wasserrändern und abgebrochener Ecke, eine Klapptür mit vergilbter Bedienblende und grauem Ersatz-Programmknebel, zwei Drahtkörbe mit abgeplatzter Beschichtung, ein verkalkter Edelstahlbehälter, und Ablaufschlauch, Zulaufschlauch und Netzleitung aufgerollt und an die Rückwand geklebt; gelagert auf der Palette, von zwei Trägern getragen, spült mit 2,2 kW, beladen bei offener Tür und trocknend mit angelehnter Tür."),
    "tags": ["used", "household", "second-hand", "kitchen", "appliance", "dishwasher", "electric", "internal-gain", "heat-source", "steel", "pallet", "hoses-taped"],
    "provenance": {"author": "Semio staffage content (used household objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES,
                   "notes": T(f"Type: German mid-range 60 cm freestanding dishwasher of about 2005-2015, 12 place settings, 850 x 600 x 600 mm with removable 30 mm worktop, 2.2 kW nameplate (2 kW heater, circulation and drain pumps), white powder-coated 0.6 mm steel cabinet, 0.5 mm stainless tub 495 x 655 x 550 mm, 55 mm drop-down door hinged at its lower inner edge with a front control panel, kick plinth recessed 60 mm, four feet of 20 mm. Masses are the 1:50 solid volumes times the densities (steel 7850, stainless 7700, chipboard 680, bitumen 1500, spruce 420 kg/m3; base unit, door and control panel as effective densities of their assemblies): machine {MACHINE_KG} kg, pallet {PALLET_KG} kg, total {TOTAL_KG} kg. Traces of use are authored from the intake of second-hand department stores; nothing is measured.", f"Typ: 60-cm-Standgeschirrspüler des deutschen Mittelpreissegments von etwa 2005-2015, 12 Maßgedecke, 850 x 600 x 600 mm mit abnehmbarer 30-mm-Arbeitsplatte, 2,2 kW Anschlusswert (2-kW-Heizung, Umwälz- und Laugenpumpe), weiß pulverbeschichteter 0,6-mm-Stahlkorpus, 0,5-mm-Edelstahlbehälter 495 x 655 x 550 mm, 55-mm-Klapptür an der unteren Innenkante angeschlagen mit Bedienblende vorn, Sockelblende 60 mm zurückgesetzt, vier 20-mm-Füße. Massen sind die Volumen der 1:50-Körper mal den Dichten (Stahl 7850, Edelstahl 7700, Spanplatte 680, Bitumen 1500, Fichte 420 kg/m3; Bodeneinheit, Tür und Blende als wirksame Dichten ihrer Baugruppen): Maschine {de(MACHINE_KG)} kg, Palette {de(PALLET_KG)} kg, zusammen {de(TOTAL_KG)} kg. Gebrauchsspuren sind nach der Annahme von Gebrauchtwarenkaufhäusern geschätzt; nichts ist gemessen.")},
    "dimensions": {"heightM": Y(0.850), "widthM": 0.80, "depthM": 0.60, "massKg": TOTAL_KG},
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
        "operating": [{"id": "heater", "title": T("Wash heater and pumps", "Spülheizung und Pumpen"), "levels": [{"v": 0, "title": T("Off", "Aus")}, {"v": 1, "title": T("Heating the wash water (2.0 kW)", "Heizt das Spülwasser (2,0 kW)")}]},
                      {"id": "drying", "title": T("Residual-heat drying", "Trocknen mit Restwärme"), "levels": [{"v": 0, "title": T("Cold", "Kalt")}, {"v": 1, "title": T("Hot tub and dishes after the final rinse", "Heißer Behälter und Geschirr nach dem Klarspülen")}]}],
        "regions": REGIONS,
        "emitters": [{"id": "door-steam", "kind": "vent", "title": T("Warm vapour over the ajar door", "Warmer Dampf über der angelehnten Tür"), "anchor": "door-gap", "directionDeg": 90, "diameterM": 0.05, "operatingKey": "drying", "volumeFlowM3S": 0.002, "temperatureRiseK": 18, "vapourKgPerKg": 0.012, "source": "authored-estimate", "reference": "incropera-2018"}],
        "assumptions": [
            T("Nameplate 2.2 kW electrical input (2 kW heater, circulation pump about 60 W, drain pump about 30 W, controls), stated on the cabinet region. While heating, about 45 W leave through the side and back panels and 25 W through the door (insulated by bitumen mats and felt), so the outer surfaces stay at 25 to 28 degC in a 21 degC kitchen; the rest goes into the water and the dishes.", "Typenschild 2,2 kW elektrische Aufnahme (2-kW-Heizung, Umwälzpumpe etwa 60 W, Laugenpumpe etwa 30 W, Steuerung), an der Region Korpus angegeben. Beim Heizen verlassen etwa 45 W die Seiten- und Rückwände und 25 W die Tür (gedämmt durch Bitumenmatten und Filz), sodass die Außenflächen in einer 21-degC-Küche 25 bis 28 degC warm bleiben; der Rest geht in Wasser und Geschirr."),
            T("Drying: after the final rinse at about 65 degC the tub and the dishes hold some 0.4 kWh; with the door ajar they release up to 250 W, decaying over ten minutes, partly as warm vapour over the door edge (2 l/s, 18 K above the room, 12 g/kg).", "Trocknen: nach dem Klarspülen bei etwa 65 degC halten Behälter und Geschirr rund 0,4 kWh; bei angelehnter Tür geben sie bis zu 250 W ab, abklingend über zehn Minuten, zum Teil als warmer Dampf über der Türkante (2 l/s, 18 K über dem Raum, 12 g/kg)."),
            T("Aged surfaces: the white powder coating has yellowed and is scratched, so its solar absorptance is taken as 0.33 instead of about 0.25 when new; the limescaled stainless tub radiates with 0.35 instead of about 0.15 when bright (authored estimates after Incropera).", "Gealterte Oberflächen: die weiße Pulverbeschichtung ist vergilbt und verkratzt, daher gilt ein solarer Absorptionsgrad von 0,33 statt etwa 0,25 im Neuzustand; der verkalkte Edelstahlbehälter strahlt mit 0,35 statt etwa 0,15 im blanken Zustand (Schätzungen nach Incropera)."),
            T("Reference case: a domestic kitchen at 21 degC without sun; the depot hall of the stored pose is about 19 degC. Tub and racks are hidden inside the closed door; the open and ajar door expose them by the fractions of the poses.", "Referenzfall: eine Wohnküche bei 21 degC ohne Sonne; die Lagerhalle der gelagerten Pose hat etwa 19 degC. Behälter und Körbe liegen verdeckt hinter der geschlossenen Tür; die offene und die angelehnte Tür legen sie mit den Anteilen der Posen frei."),
        ],
    },
    "structure": {
        "gravityMS2": G, "dynamics": "quasi-static",
        "supports": ["pallet-floor"],
        "assumptions": [
            T("The machine is a box of 0.6 mm steel side and back panels screwed to a base pan with two folded base rails and to the welded stainless tub, closed by the 30 mm worktop; it stands on four feet, the two front ones adjustable. Members are the panels, the worktop, the base, the tub and the door on their bones with their true sections.", "Die Maschine ist ein Kasten aus 0,6-mm-Stahlseiten- und Rückwänden, verschraubt mit einer Bodenwanne mit zwei gekanteten Bodenschienen und mit dem geschweißten Edelstahlbehälter, oben geschlossen von der 30-mm-Arbeitsplatte; sie steht auf vier Füßen, die zwei vorderen verstellbar. Bauteile sind die Platten, die Arbeitsplatte, der Boden, der Behälter und die Tür auf ihren Knochen mit ihren wahren Querschnitten."),
            T(f"In use: 12 place settings, 7 kg in the lower and 4 kg in the upper basket (after EN 50242), and 3.5 kg of wash water in the sump while washing.", "In Gebrauch: 12 Maßgedecke, 7 kg im Unter- und 4 kg im Oberkorb (nach EN 50242), und beim Spülen 3,5 kg Spülwasser im Sumpf."),
            T(f"Tipping over the front feet with the door down (IEC 60335-2-5 asks for stability of appliances with a bottom-hinged door): the empty machine without its door weighs about {round(MACHINE_KG - 6.9, 1)} kg with its centre of mass 0.26 m behind the front feet' line at x 0.505 m less 0.25 m, about 95 Nm restoring; the open door (6.9 kg at 0.39 m beyond the feet) and the pulled-out loaded lower basket (8.4 kg at 0.20 m beyond) overturn with about 43 Nm, so loading is safe, but a child of more than about 13 kg sitting on the door edge would tip a machine that is not fixed under a worktop. The planar solver of the archive resolves the loads in the side plane; this tipping check is a hand calculation.", f"Kippen über die vorderen Füße bei offener Tür (IEC 60335-2-5 verlangt Standsicherheit für Geräte mit unten angeschlagener Tür): die leere Maschine ohne Tür wiegt etwa {de(round(MACHINE_KG - 6.9, 1))} kg mit dem Schwerpunkt 0,25 m hinter der Linie der vorderen Füße bei x 0,505 m, etwa 95 Nm Rückstellmoment; die offene Tür (6,9 kg 0,39 m vor den Füßen) und der herausgezogene beladene Unterkorb (8,4 kg 0,20 m davor) kippen mit etwa 43 Nm, das Beladen ist also sicher, aber ein Kind von mehr als etwa 13 kg auf der Türkante würde eine nicht unter einer Arbeitsplatte befestigte Maschine kippen. Der ebene Löser des Archivs löst die Lasten in der Seitenebene; diese Kippprüfung ist eine Handrechnung."),
            T("Carried: two movers hold the back edge and the base pan behind the plinth (pins) and each takes about half of the weight; the pallet stays on the floor on its own contact. The rigid-body solver treats the pallet and the machine as one body (see requests of this wave), so in the carried and home poses a share of the pallet's reactions is redistributed.", "Getragen: zwei Träger halten die Hinterkante und die Bodenwanne hinter dem Sockel (Gelenke), jeder trägt etwa die Hälfte des Gewichts; die Palette bleibt mit eigenem Kontakt auf dem Boden. Der Starrkörperlöser behandelt Palette und Maschine als einen Körper (siehe Anfragen dieser Welle), daher wird in den getragenen und häuslichen Posen ein Teil der Auflagerkräfte der Palette umverteilt."),
        ],
        "clinical": False,
    },
    "volumes": volumes,
    "presentation": {"defaultPose": "stored", "defaultClip": "load-cycle", "defaultScale": 50},
    "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    "completion": "authored",
}

doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as fh:
    json.dump(doc, fh, ensure_ascii=False, indent=1)
print(f"[DEBUG] mass {TOTAL_KG} kg (machine {MACHINE_KG}, pallet {PALLET_KG}); per bone " + ", ".join(f"{b} {m:.2f}" for b, (m, _, _) in MASS.items()))
print("[DEBUG] solids " + ", ".join(f"1:{s} {len(volumes[str(s)]['body'])}/{len(volumes[str(s)]['thermal'])}/{len(volumes[str(s)]['structure'])}" for s in (50, 100, 200)))
