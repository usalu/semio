"""🎶 Authoritative author script of the staffage asset `used-electronic-organ`: writes `🔣️.json` next to this file from the hand-chosen numbers of `organ_geometry.py` and of this file.

Run: `python3 author-used-electronic-organ.py` (no arguments). The texts, rig, poses, clips, thermal and structural data are stated here; every 2D shape and 3D solid is generated from the same part list per scale, so drawing and model agree by construction.
"""
import json
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from organ_geometry import (BONES, H_D, COVER_LEN, COVER_OPEN, P_KEY, Z0_LOWER, Z0_UPPER, KEY_C_INDEX, black_positions, local_profile, local_step, parts_of, rel_placement, to_local, to_world, rot, ol, rect)

HERE = os.path.dirname(os.path.abspath(__file__))
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.030}
SCALES = [50, 100, 200]


def T(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


def rnd(v, n=5):
    """✂️ Rounds a number for the document."""
    return round(v + 0.0, n)


def pts(points):
    """📍 Rounded point list."""
    return [[rnd(a), rnd(b)] for a, b in points]


def path_d(points, holes=()):
    """〰️ An SVG path with one subpath per ring."""
    def ring(r):
        return "M " + " L ".join(f"{rnd(a)} {rnd(b)}" for a, b in r) + " Z"
    return " ".join([ring(points)] + [ring(h) for h in holes])


MATERIALS = [
    ("veneer-aged", "Veneered chipboard, aged lacquer", "Furnierte Spanplatte, gealterter Lack", 0.9, 0.62, 650, 0.13, 1700, 2.5e9, 1.3e7, "en-312-2010"),
    ("veneer-worn", "Veneer worn through to bare wood at the hand contacts", "An den Griffstellen bis aufs Holz abgegriffenes Furnier", 0.9, 0.66, 650, 0.13, 1700, None, None, "rabek-1995"),
    ("hardboard-aged", "Hardboard back, dusty", "Hartfaser-Rückwand, verstaubt", 0.9, 0.7, 900, 0.2, 1700, 4.0e9, 3.0e7, "iso-10456-2007"),
    ("panel-faded", "Control panel: printed aluminium face, faded by light, on a box of switches and boards (effective density)", "Bedienfeld: bedruckte, vom Licht ausgeblichene Aluminiumfront auf einem Kasten mit Schaltern und Platinen (effektive Dichte)", 0.85, 0.45, 450, 0.3, 1100, 7.0e10, 1.6e8, "rabek-1995"),
    ("key-acrylic-yellowed", "Hollow-moulded keys with acrylic tops, yellowed (effective density)", "Hohl gespritzte Tasten mit Acrylbelag, vergilbt (effektive Dichte)", 0.92, 0.38, 480, 0.19, 1470, 3.0e9, 6.0e7, "rabek-1995"),
    ("key-black-aged", "Hollow black ABS keys, greyed (effective density)", "Hohle schwarze ABS-Tasten, vergraut (effektive Dichte)", 0.92, 0.88, 520, 0.17, 1400, 2.2e9, 4.0e7, "rabek-1995"),
    ("key-frame-steel", "Key action: painted steel frame, springs and contact boards (effective density)", "Tastenmechanik: lackierter Stahlrahmen, Federn und Kontaktplatinen (effektive Dichte)", 0.9, 0.8, 520, 0.3, 600, 2.1e11, 2.35e8, "douglas-1976"),
    ("grille-cloth", "Speaker grille cloth on chipboard, dusty", "Lautsprecherbespannung auf Spanplatte, verstaubt", 0.9, 0.75, 650, 0.12, 1600, None, None, "iso-10456-2007"),
    ("paper-sticker", "Old paper price sticker", "Alter Papier-Preisaufkleber", 0.92, 0.3, 800, 0.12, 1340, None, None, "iso-10456-2007"),
    ("pedal-beech", "Beech pedal frame", "Buchen-Pedalrahmen", 0.9, 0.6, 700, 0.18, 1600, 1.19e10, 1.03e8, "fpl-2010"),
    ("pedal-natural", "Beech naturals with white plastic caps, yellowed", "Buchen-Untertasten mit weißen Kunststoffkappen, vergilbt", 0.92, 0.4, 750, 0.18, 1600, 1.19e10, 1.03e8, "fpl-2010"),
    ("pedal-sharp", "Black ABS pedal sharps and swell shoe, rubber tread worn", "Schwarze ABS-Obertasten und Schwellerschuh, Gummibelag abgetreten", 0.92, 0.9, 1050, 0.17, 1400, 2.2e9, 4.0e7, "rabek-1995"),
    ("pedal-felt", "Wool felt", "Wollfilz", 0.9, 0.7, 300, 0.05, 1300, None, None, "iso-10456-2007"),
    ("castor-rubber", "Grey rubber castor wheel, worn", "Graues Gummirad, abgefahren", 0.92, 0.8, 1200, 0.16, 1800, None, None, "en-12532-1999"),
    ("castor-steel", "Zinc-plated steel, tarnished", "Verzinkter Stahl, angelaufen", 0.3, 0.6, 7850, 50, 470, 2.1e11, 2.35e8, "en-12532-1999"),
    ("dolly-beech", "Beech plywood dolly frame", "Rollbrett-Rahmen aus Buchensperrholz", 0.9, 0.6, 700, 0.17, 1600, 1.0e10, 6.0e7, "fpl-2010"),
    ("dolly-carpet", "Worn needle-felt carpet pads", "Abgenutzte Nadelfilz-Teppichauflagen", 0.9, 0.75, 200, 0.06, 1300, None, None, "iso-10456-2007"),
    ("bench-beech", "Stained beech legs, scuffed", "Gebeizte Buchenbeine, verschrammt", 0.9, 0.65, 700, 0.18, 1600, 1.19e10, 1.03e8, "fpl-2010"),
    ("bench-veneer", "Veneered bench board", "Furnierte Bankplatte", 0.9, 0.62, 650, 0.13, 1700, 2.5e9, 1.3e7, "en-312-2010"),
    ("bench-vinyl", "Black vinyl cushion, split at the front edge", "Schwarzes Kunstlederpolster, an der Vorderkante aufgeplatzt", 0.9, 0.88, 300, 0.05, 1300, None, None, "iso-10456-2007"),
    ("amp-aluminium", "Black anodised aluminium heat sink and amplifier chassis", "Schwarz eloxierter Aluminium-Kühlkörper und Verstärkerchassis", 0.8, 0.9, 2700, 160, 900, 7.0e10, 1.6e8, "incropera-2018"),
]
DENSITY = {m[0]: m[5] for m in MATERIALS}


def materials():
    """🧱 The materials of the asset."""
    out = []
    for mid, en, de, eps, alpha, rho, k, cp, E, f, ref in MATERIALS:
        m = {"id": mid, "title": T(en, de), "emissivity": eps, "solarAbsorptivity": alpha, "densityKgM3": rho, "conductivityWmK": k, "specificHeatJKgK": cp}
        if E is not None:
            m["elasticModulusPa"] = E
            m["strengthPa"] = f
        m["source"] = "authored-estimate"
        m["reference"] = ref
        out.append(m)
    return out


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
    a *= 0.5
    return abs(a), (cx / (6 * a), cy / (6 * a))


def copies(p):
    """🔁 The lateral spans of all copies of a part."""
    n, step = (p["rep"] if p["rep"] else (1, (0, 0, 0)))
    return [(p["z0"] + k * step[2], p["z1"] + k * step[2]) for k in range(n)]


EXTRA_MASS = {"cabinet": [("amplifier", 4.5, (-0.175, H_D + 0.48), (0.15, 0.16, 0.36)), ("speakers", 3.2, (-0.10, H_D + 0.30), (0.09, 0.20, 0.70)), ("tone-generator", 1.3, (-0.20, H_D + 0.75), (0.12, 0.12, 0.60))]}


def masses():
    """⚖️ Bone masses from the 1:50 solids (section x length x density) plus the electronics, and the centre of each bone in its frame."""
    acc = {}
    for p in parts_of(50):
        if p["texture"]:
            continue
        prof, _ = local_profile(p)
        a, (cu, cv) = poly_area_centroid(prof)
        for z0, z1 in copies(p):
            m = a * (z1 - z0) * DENSITY[p["mat"]]
            e = acc.setdefault(p["bone"], [0.0, 0.0, 0.0])
            e[0] += m
            e[1] += m * cu
            e[2] += m * cv
    for bone, items in EXTRA_MASS.items():
        for _, kg, c, _s in items:
            u, v = to_local(bone, c)
            e = acc.setdefault(bone, [0.0, 0.0, 0.0])
            e[0] += kg
            e[1] += kg * u
            e[2] += kg * v
    return {b: (e[0], e[1] / e[0], e[2] / e[0]) for b, e in acc.items()}


MASSES = masses()
TOTAL_KG = round(sum(m for m, _, _ in MASSES.values()), 2)

SECTIONS = {
    "cabinet": (2 * 0.018 * 0.53, 2 * 0.018 * 0.53 ** 3 / 12, 0.265, "veneer-aged", T("Two veneered chipboard cheeks 18 x 530 mm", "Zwei furnierte Spanplattenwangen 18 x 530 mm")),
    "key-cover": (1.10 * 0.012, 1.10 * 0.012 ** 3 / 12, 0.006, "veneer-aged", T("Veneered board 1100 x 12 mm", "Furnierte Platte 1100 x 12 mm")),
    "pedalboard": (0.50 * 0.012, 0.50 * 0.012 ** 3 / 12, 0.006, "pedal-beech", T("Beech base board 500 x 12 mm", "Buchen-Grundbrett 500 x 12 mm")),
    "swell-pedal": (0.095 * 0.022, 0.095 * 0.022 ** 3 / 12, 0.011, "pedal-sharp", T("ABS shoe 95 x 22 mm", "ABS-Schuh 95 x 22 mm")),
    "lower-key": (0.0225 * 0.022, 0.0225 * 0.022 ** 3 / 12, 0.011, "key-acrylic-yellowed", T("Key lever 22.5 x 22 mm", "Tastenhebel 22,5 x 22 mm")),
    "dolly": (2 * 0.095 * 0.022, 2 * 0.095 * 0.022 ** 3 / 12, 0.011, "dolly-beech", T("Two beech plywood rails 95 x 22 mm", "Zwei Buchensperrholz-Holme 95 x 22 mm")),
    "bench": (4 * 0.04 ** 2, 4 * 0.04 ** 4 / 12 + 4 * 0.0016 * 0.125 ** 2, 0.145, "bench-beech", T("Four beech legs 40 x 40 mm, 250 mm apart", "Vier Buchenbeine 40 x 40 mm, 250 mm Abstand")),
}

BONE_TITLES = {
    "floor": T("Floor datum of the shop", "Bodenbezug im Laden"),
    "carriage": T("Rolling carriage of organ and dolly", "Rollweg von Orgel und Rollbrett"),
    "dolly": T("Four-castor furniture dolly", "Möbelroller mit vier Lenkrollen"),
    "cabinet": T("Organ cabinet with both manuals", "Orgelgehäuse mit beiden Manualen"),
    "key-cover": T("Key cover, hinged at the lid, music rest when open", "Tastenklappe, am Deckel angeschlagen, offen das Notenpult"),
    "pedalboard": T("13-note pedalboard, folding up on its hinge", "13-töniges Pedal, auf seinem Scharnier hochklappbar"),
    "swell-pedal": T("Expression (swell) pedal shoe on its pivot", "Schwellerschuh auf seiner Achse"),
    "lower-key": T("Middle C of the lower manual on its balance rail", "Mittleres C des Untermanuals auf seiner Waagleiste"),
    "bench": T("Organ bench", "Orgelbank"),
}


def rig():
    """🦴 The mechanical rig."""
    bones = []
    for bid, b in BONES.items():
        x, y, ang = rel_placement(bid)
        bone = {"id": bid, "parent": b["parent"], "title": BONE_TITLES[bid], "x": rnd(x), "y": rnd(y), "angleDeg": rnd(ang, 3), "lengthM": rnd(b["length"]), "joint": b["joint"]}
        if bid in MASSES:
            kg, cu, cv = MASSES[bid]
            bone["mass"] = {"kg": round(kg, 3), "comX": rnd(cu, 4), "comY": rnd(cv, 4), "distribution": "point", "source": "authored-estimate", "reference": "douglas-1976"}
        if bid in SECTIONS:
            a, i, e, mat, title = SECTIONS[bid]
            bone["section"] = {"areaM2": rnd(a, 7), "secondMomentM4": float(f"{i:.4g}"), "extremeFibreM": e, "material": mat, "title": title}
        bones.append(bone)
    return {"kind": "mechanical", "root": "floor", "bones": bones}


def lp(bone, x, y):
    """📌 World rest point to bone-local, rounded."""
    u, v = to_local(bone, (x, y))
    return rnd(u), rnd(v)


def anchors():
    """📍 Contacts, supports, load points and the emitter outlet."""
    out = []
    for tag, cx, cz in [("back-right", -0.300, -0.250), ("back-left", -0.300, 0.250), ("front-right", 0.060, -0.250), ("front-left", 0.060, 0.250)]:
        u, v = lp("dolly", cx, 0.0)
        out.append({"id": f"castor-{tag}", "bone": "dolly", "x": u, "y": v, "z": cz, "kind": "contact", "title": T(f"Castor {tag.replace('-', ' ')} on the floor", "Lenkrolle " + {"back-right": "hinten rechts", "back-left": "hinten links", "front-right": "vorn rechts", "front-left": "vorn links"}[tag] + " auf dem Boden"), "support": "contact", "footprintLengthM": 0.024, "footprintWidthM": 0.025, "frictionCoefficient": 0.02})
    u, v = lp("bench", 0.490, 0.0)
    out.append({"id": "bench-feet", "bone": "bench", "x": u, "y": v, "z": -0.03, "kind": "contact", "title": T("Four bench feet on the floor", "Vier Bankfüße auf dem Boden"), "support": "contact", "footprintLengthM": 0.29, "footprintWidthM": 0.76, "frictionCoefficient": 0.4})
    for side, z in [("right", -0.541), ("left", 0.541)]:
        u, v = lp("cabinet", -0.015, H_D)
        out.append({"id": f"cheek-foot-{side}", "bone": "cabinet", "x": u, "y": v, "z": z, "kind": "contact", "title": T(f"Foot of the {side} cheek", "Fuß der " + ("rechten" if side == "right" else "linken") + " Wange"), "support": "contact", "footprintLengthM": 0.52, "footprintWidthM": 0.018, "frictionCoefficient": 0.4})
    out.append({"id": "pedalboard-floor", "bone": "pedalboard", "x": 0.125, "y": 0.0, "z": -0.17, "kind": "contact", "title": T("Pedalboard on the floor", "Pedal auf dem Boden"), "support": "contact", "footprintLengthM": 0.25, "footprintWidthM": 0.5, "frictionCoefficient": 0.4})
    for side, z in [("right", -0.54), ("left", 0.54)]:
        u, v = lp("cabinet", -0.26, H_D + 0.97)
        out.append({"id": f"grip-{side}", "bone": "cabinet", "x": u, "y": v, "z": z, "kind": "load", "title": T(f"Mover's hand at the {side} back top corner", "Hand des Trägers an der " + ("rechten" if side == "right" else "linken") + " hinteren oberen Ecke")})
    u, v = lp("cabinet", 0.20, H_D + 0.705)
    out.append({"id": "lower-manual-hands", "bone": "cabinet", "x": u, "y": v, "z": -0.08, "kind": "load", "title": T("Hands on the lower manual", "Hände auf dem Untermanual")})
    u, v = lp("cabinet", 0.10, H_D + 0.805)
    out.append({"id": "upper-manual-hand", "bone": "cabinet", "x": u, "y": v, "z": 0.08, "kind": "load", "title": T("Right hand on the upper manual", "Rechte Hand auf dem Obermanual")})
    out.append({"id": "swell-toe", "bone": "swell-pedal", "x": 0.12, "y": 0.022, "z": 0.2025, "kind": "load", "title": T("Right foot on the swell pedal", "Rechter Fuß auf dem Schweller")})
    out.append({"id": "pedal-toe", "bone": "pedalboard", "x": 0.20, "y": 0.045, "z": -0.37, "kind": "load", "title": T("Left foot on the low C pedal", "Linker Fuß auf dem tiefen C des Pedals")})
    out.append({"id": "key-c-top", "bone": "lower-key", "x": 0.12, "y": 0.022, "z": rnd(Z0_LOWER + (KEY_C_INDEX + 0.5) * P_KEY), "kind": "load", "title": T("Finger on middle C", "Finger auf dem mittleren C")})
    u, v = lp("bench", 0.49, 0.575)
    out.append({"id": "bench-seat", "bone": "bench", "x": u, "y": v, "z": -0.03, "kind": "load", "title": T("Player seated on the bench", "Spieler auf der Bank")})
    u, v = lp("cabinet", -0.2792, H_D + 0.56)
    out.append({"id": "rear-vent", "bone": "cabinet", "x": u, "y": v, "z": 0.0, "kind": "emitter", "title": T("Vent slots of the amplifier in the back panel", "Lüftungsschlitze des Verstärkers in der Rückwand")})
    return out


VIEW_RULES = [
    ("cheek", ("elevation", "front", "plan")), ("back-panel", ("elevation", "plan")), ("key-cover", ("elevation", "front", "plan")),
    ("lid", ("plan",)), ("tab-panel", ("front", "plan")), ("tab-strip", ("front", "plan")), ("rhythm-buttons", ("front", "plan")), ("bottom-board", ("front",)),
    ("key-bed", ("front",)), ("name-board", ("front",)), ("upper-key-bed", ("front", "plan")), ("lower-action", ()),
    ("lower-panel", ("elevation", "front", "plan")), ("upper-panel", ("front", "plan")), ("knee-panel", ("elevation", "front")), ("grille-cloth", ("front",)),
    ("key-slip", ("front", "plan")), ("lower-whites", ("elevation", "front", "plan")), ("upper-whites", ("front", "plan")), ("lower-black", ("front", "plan")),
    ("upper-black", ("front", "plan")), ("key-c", ("front", "plan")), ("swell-base", ("front", "plan")), ("swell-bracket", ("front",)),
    ("swell-shoe", ("elevation", "front", "plan")), ("pedal-sharp-riser", ()), ("pedal", ("elevation", "front", "plan")),
    ("dolly", ("elevation", "front", "plan")), ("castor", ("elevation", "front", "plan")), ("bench", ("elevation", "front", "plan")),
    ("cabinet-bottom", ("front",)), ("cabinet-console", ("front", "plan")), ("cabinet-lid", ("plan",)), ("lower-manual", ("elevation", "front", "plan")), ("upper-manual", ("front", "plan")),
]


def views_of(p):
    """👁️ The viewpoints a part is drawn in: those where it is not wholly hidden behind nearer parts of the stored pose."""
    if p["texture"]:
        return p["views"]
    if "views" in p["draw"]:
        return set(p["draw"]["views"])
    for prefix, views in VIEW_RULES:
        if p["id"].startswith(prefix):
            return set(views)
    raise KeyError(p["id"])


def world_profile(p):
    """🌍 The world rest profile of a part."""
    if p["frame"] == "local":
        return [to_world(p["bone"], q) for q in p["prof"]]
    return p["prof"]


def order(p, view, span):
    """🔢 Painter order of a part in a view: nearer parts are painted later."""
    w = world_profile(p)
    if view == "elevation":
        return -span[0]
    if view == "front":
        return max(x for x, _ in w)
    return max(y for _, y in w)




def shapes_of(scale, parts):
    """✏️ The three 2D drawings of a scale, generated from its parts (filled closed shapes with their solid placement) plus the authored strokes."""
    geo = {"elevation": [], "front": [], "plan": []}
    for p in parts:
        prof, holes = local_profile(p)
        role = ("texture" if scale == 50 else "detail") if p["texture"] else p["role"]
        for view in sorted(views_of(p)):
            spans = copies(p)
            if "span" in p["draw"]:
                spans = [p["draw"]["span"]]
            if view == "elevation":
                spans = [min(spans)]
            for k, span in enumerate(spans):
                s = {"id": p["id"] if len(spans) == 1 else f"{p['id']}-{k + 1}", "bone": p["bone"], "role": role, "z": 0.0, "filled": True, "tone": p["tone"], "region": p["region"], "material": p["mat"], "kind": "path", "d": path_d(prof, holes), "solid": {"kind": "extruded", "sweep": "w", "z0": rnd(span[0]), "z1": rnd(span[1])}}
                if view in p["hatch"]:
                    s["hatch"] = p["hatch"][view]
                geo[view].append((order(p, view, span), s))
    for view, items in strokes(scale).items():
        for s in items:
            geo[view].append((1e3, s))
    out = {}
    for view, items in geo.items():
        items.sort(key=lambda t: t[0])
        for i, (_, s) in enumerate(items):
            s["z"] = round(1 + 0.01 * i, 2)
        out[view] = [s for _, s in items]
    return out


def stroke(sid, bone, points, role="texture", region=None):
    """〰️ An open stroke in the frame of its view."""
    s = {"id": sid, "bone": bone, "role": role, "z": 0.0, "filled": False, "tone": 0.2, "kind": "poly", "points": pts(points), "closed": False}
    if region:
        s["region"] = region
    return s


def strokes(scale):
    """🖊️ The authored detail and texture strokes of a scale: key divisions, scratches, the split cushion, tab divisions."""
    out = {"elevation": [], "front": [], "plan": []}
    if scale != 50:
        if scale == 100:
            out["elevation"].append(stroke("cheek-joint", "cabinet", [lp("cabinet", -0.2700, H_D + 0.020), lp("cabinet", 0.200, H_D + 0.020)], role="detail"))
        return out
    for i, (a, b) in enumerate([((-0.21, 0.70), (-0.12, 0.66)), ((-0.05, 0.47), (0.03, 0.43)), ((-0.24, 0.20), (-0.17, 0.215))]):
        out["elevation"].append(stroke(f"cheek-scratch-{i + 1}", "cabinet", [lp("cabinet", a[0], H_D + a[1]), lp("cabinet", b[0], H_D + b[1])], region="cabinet-veneer"))
    for i, (a, b) in enumerate([((-0.21, -0.25), (-0.10, -0.18)), ((-0.18, 0.30), (-0.06, 0.34))]):
        out["plan"].append(stroke(f"lid-scratch-{i + 1}", "cabinet", [a, b], region="cabinet-veneer"))
    out["plan"].append(stroke("cushion-split", "bench", [(0.600, -0.20), (0.615, -0.12), (0.605, -0.05)], region="bench"))
    out["elevation"].append(stroke("cushion-split-edge", "bench", [lp("bench", 0.630, 0.570), lp("bench", 0.640, 0.556)], region="bench"))
    return out


def body_solid(p, scale):
    """🧊 The body solid of a part: its profile extruded laterally on its bone."""
    prof, holes = local_profile(p)
    s = {"id": p["id"], "kind": "extrude", "bone": p["bone"], "material": p["mat"], "role": ("texture" if scale == 50 else "detail") if p["texture"] else p["role"], "tone": p["tone"], "profile": pts(prof), "plane": "xy", "from": rnd(p["z0"]), "to": rnd(p["z1"])}
    if holes:
        s["holes"] = [pts(h) for h in holes]
    if p["rep"]:
        n, (du, dv, dw) = local_step(p)
        s["repeat"] = {"count": n, "translation": [rnd(du), rnd(dv), rnd(dw)]}
    if p["sheet"]:
        s["sheet"] = True
    if p["texture"]:
        s["texture"] = True
    return s


def box(sid, bone, mat, role, tone, centre, size, **extra):
    """📦 A box solid in a bone frame."""
    s = {"id": sid, "kind": "box", "bone": bone, "material": mat, "role": role, "tone": tone, "size": [rnd(x) for x in size], "transform": {"translation": [rnd(x) for x in centre]}}
    s.update(extra)
    return s


def ol_box_local(bone, x0, x1, y0, y1):
    """📦 Centre and (u, v) size in the cabinet frame of an organ-local box."""
    a = to_local(bone, (x0, y0 + H_D))
    b = to_local(bone, (x1, y1 + H_D))
    return ((a[0] + b[0]) / 2, (a[1] + b[1]) / 2), (abs(b[0] - a[0]), abs(b[1] - a[1]))


def thermal_list(scale, parts):
    """🌡️ The thermal model of a scale: every region surface as a reference to its body solid, the hidden amplifier inside the cabinet and the outlet patch of the rear plume."""
    out = [{"id": f"th-{p['id']}", "of": p["id"], "region": p["region"]} for p in parts if p["thermal"] and not p["texture"]]
    if scale == 50:
        (cu, cv), (su, sv) = ol_box_local("cabinet", -0.25, -0.13, 0.40, 0.56)
        out.append(box("th-amplifier-chassis", "cabinet", "amp-aluminium", "detail", 0.15, (cu, cv, 0.0), (su, sv, 0.36), region="amplifier"))
        (cu, cv), (su, sv) = ol_box_local("cabinet", -0.13, -0.10, 0.41, 0.55)
        out.append(box("th-amplifier-fins", "cabinet", "amp-aluminium", "detail", 0.15, (cu, cv, -0.1575), (su, sv, 0.008), region="amplifier", repeat={"count": 8, "translation": [0.0, 0.0, 0.045]}))
        (cu, cv), (su, sv) = ol_box_local("cabinet", -0.2792, -0.2782, 0.50, 0.62)
        plume = (cu, cv, 0.0), (su, sv, 0.50)
    elif scale == 100:
        (cu, cv), (su, sv) = ol_box_local("cabinet", -0.25, -0.10, 0.40, 0.56)
        out.append(box("th-amplifier", "cabinet", "amp-aluminium", "detail", 0.15, (cu, cv, 0.0), (su, sv, 0.36), region="amplifier"))
        (cu, cv), (su, sv) = ol_box_local("cabinet", -0.2802, -0.2782, 0.50, 0.62)
        plume = (cu, cv, 0.0), (su, sv, 0.50)
    else:
        (cu, cv), (su, sv) = ol_box_local("cabinet", -0.26, -0.09, 0.39, 0.57)
        out.append(box("th-amplifier", "cabinet", "amp-aluminium", "detail", 0.15, (cu, cv, 0.0), (su, sv, 0.39), region="amplifier"))
        (cu, cv), (su, sv) = ol_box_local("cabinet", -0.2812, -0.2782, 0.49, 0.63)
        plume = (cu, cv, 0.0), (su, sv, 0.51)
    out.append(box("th-rear-plume", "cabinet", "hardboard-aged", "texture", 0.3, plume[0], plume[1], tags=["rear-plume"], region="back-panel", sectionFaces=["+u", "-u", "+v", "-v", "+w", "-w"]))
    return out


MEMBERS = {
    50: {"cabinet": ["cheek-right", "cheek-left", "bottom-board", "key-bed", "lower-action", "knee-panel", "lid", "swell-base"], "key-cover": ["key-cover"], "pedalboard": ["pedal-base", "pedal-rail"], "swell-pedal": ["swell-shoe"], "lower-key": ["key-c"],
         "dolly": ["dolly-end-back", "dolly-end-front", "dolly-rail-right", "dolly-rail-left", "dolly-carpet-back", "dolly-carpet-front", "dolly-carpet-right", "dolly-carpet-left"] + [f"castor-{t}-{k}" for t, _, _ in [("bl", 0, 0), ("br", 0, 0), ("fl", 0, 0), ("fr", 0, 0)] for k in ("wheel", "leg", "fork", "plate")],
         "bench": ["bench-leg-back", "bench-leg-front", "bench-apron-back", "bench-apron-front", "bench-apron-side", "bench-seat"]},
    100: {"cabinet": ["cheek-right", "cheek-left", "bottom-board", "key-bed", "lower-manual-bass", "lower-manual-treble", "knee-panel", "lid", "swell-base"], "key-cover": ["key-cover"], "pedalboard": ["pedal-deck", "pedal-rail"], "swell-pedal": ["swell-shoe"], "lower-key": ["key-c"],
          "dolly": ["dolly-end-back", "dolly-end-front", "dolly-rail-right", "dolly-rail-left"] + [f"castor-{t}-{k}" for t in ("bl", "br", "fl", "fr") for k in ("wheel", "housing")],
          "bench": ["bench-legs-back", "bench-legs-front", "bench-frame"]},
    200: {"cabinet": ["cheek-right", "cheek-left", "cabinet-bottom", "cabinet-lid", "knee-panel", "swell-base"], "key-cover": ["key-cover"], "pedalboard": ["pedal-deck", "pedal-rail"], "swell-pedal": ["swell-shoe"],
          "dolly": ["dolly-deck"] + [f"castor-{t}" for t in ("bl", "br", "fl", "fr")], "bench": ["bench-legs", "bench-legs-far", "bench-top"]},
}


def structure_list(scale):
    """🏗️ The structural idealisation of a scale: the boards that carry (cheeks, bottom board, key bed, frame rails, legs) as members, the hinges and pivots as joints, a contact pad at every support anchor and the electronics as masses."""
    f = FEATURE[scale]
    j = {50: 0.012, 100: 0.015, 200: 0.030}[scale]
    t = {50: 0.0015, 100: 0.003, 200: 0.006}[scale]
    out = []
    for bone, ids in MEMBERS[scale].items():
        for sid in ids:
            out.append({"id": f"member-{sid}", "of": sid, "member": bone, "part": "member"})
    if scale == 200:
        (cu, cv), (su, sv) = ol_box_local("cabinet", -0.061, 0.275, 0.590, 0.620)
        out.append(box("member-key-bed", "cabinet", "veneer-aged", "body", 0.5, (cu, cv, 0.0), (su, sv, 1.04), member="cabinet", part="member"))
        zc = Z0_LOWER + (KEY_C_INDEX + 0.5) * P_KEY
        out.append(box("member-key-c", "lower-key", "key-acrylic-yellowed", "body", 0.5, (0.075, 0.011, zc), (0.15, 0.022, 0.0225), member="lower-key", part="member", slender=True))
    zc = Z0_LOWER + (KEY_C_INDEX + 0.5) * P_KEY
    out += [
        box("joint-cover-hinge", "key-cover", "castor-steel", "detail", 0.4, (0.0, 0.0, 0.0), (j, j, 1.10), member="key-cover", part="joint"),
        box("joint-pedal-hinge", "pedalboard", "castor-steel", "detail", 0.4, (j / 2, 0.044, -0.17), (j, 0.088, 0.50), member="pedalboard", part="joint"),
        box("joint-swell-pivot", "swell-pedal", "castor-steel", "detail", 0.4, (0.0, -0.01, 0.2025), (max(j, 0.02), max(0.02, j), max(0.045, j)), member="swell-pedal", part="joint"),
        box("joint-key-pivot", "lower-key", "castor-steel", "detail", 0.4, (0.0, -0.0315, zc), (j, 0.063, max(0.0225, j)), member="lower-key", part="joint"),
    ]
    pad = max(0.025, f)
    for (tag, cx, cz), anchor in zip([("bl", -0.300, -0.250), ("br", -0.300, 0.250), ("fl", 0.060, -0.250), ("fr", 0.060, 0.250)], ["castor-back-right", "castor-back-left", "castor-front-right", "castor-front-left"]):
        u, v = to_local("dolly", (cx, 0.0))
        out.append(box(f"support-{anchor}", "dolly", "castor-rubber", "ground", 0.2, (u, v - t / 2, cz), (pad, t, pad), member="dolly", part="support", anchor=anchor))
    out.append(box("support-bench-feet", "bench", "bench-beech", "ground", 0.2, (-t / 2, 0.0, -0.03), (t, 0.29, 0.76), member="bench", part="support", anchor="bench-feet"))
    for name, kg, c, size in [("amplifier", 0, (-0.175, H_D + 0.48), (0.15, 0.16, 0.36)), ("speakers", 0, (-0.10, H_D + 0.30), (0.09, 0.20, 0.70)), ("tone-generator", 0, (-0.20, H_D + 0.75), (0.12, 0.12, 0.60))]:
        u, v = to_local("cabinet", c)
        out.append(box(f"mass-{name}", "cabinet", "amp-aluminium", "detail", 0.15, (u, v, 0.0), (size[1], size[0], size[2]), member="cabinet", part="mass"))
    return out


REP_DECISIONS = {
    50: dict(
        silhouette=T("The stored organ seen from its right: the veneered cheek with its toe, the knee recess and the key cheek, the closed key cover as a 12 mm band on the sloped top edge, the 3.2 mm back panel, the furniture dolly with two visible castors underneath and the bench standing in front where the player sits; 22 mm tall on paper.", "Die eingelagerte Orgel von rechts: die furnierte Wange mit Fußspitze, Kniefreiraum und Tastenbacke, die geschlossene Tastenklappe als 12-mm-Band auf der schrägen Oberkante, die 3,2 mm starke Rückwand, darunter der Möbelroller mit zwei sichtbaren Lenkrollen und davor die Bank, wo der Spieler sitzt; 22 mm hoch auf dem Papier."),
        retained=T("Both 44-key manuals key by key (26 white and 18 black keys each) with the one-octave spinet offset, the 13-note pedalboard with eight naturals and five sharps, the swell shoe, the voice-tab strip, the rhythm and master panels, key slip, key beds, name board, grille cloth, castors with plate, fork and wheel, the bench with legs, aprons, seat and cushion; the wear: hand-worn veneer at the key cheek, an old price sticker, scratches on cheek and lid, the split cushion edge.", "Beide 44-tastigen Manuale Taste für Taste (je 26 weiße und 18 schwarze Tasten) mit dem Spinett-Versatz von einer Oktave, das 13-tönige Pedal mit acht Unter- und fünf Obertasten, der Schwellerschuh, die Registerwippenleiste, Rhythmus- und Hauptfeld, Tastenleiste, Tastenböden, Namensbrett, Bespannung, Lenkrollen mit Platte, Gabel und Rad, die Bank mit Beinen, Zargen, Sitzbrett und Polster; die Spuren: abgegriffenes Furnier an der Tastenbacke, ein alter Preisaufkleber, Kratzer an Wange und Deckel, die aufgeplatzte Polsterkante."),
        merged=T("The 16 voice tabs and the six rhythm buttons are drawn one by one; the four carpet pads of the dolly lie on its boards.", "Die 16 Registerwippen und die sechs Rhythmustaster sind einzeln gezeichnet; die vier Teppichauflagen des Rollers liegen auf seinen Brettern."),
        omitted=T("The vent slots of the back panel (5 mm, facing away from every view), screws, hinges, the contacts under the keys and the cable.", "Die Lüftungsschlitze der Rückwand (5 mm, von jeder Ansicht abgewandt), Schrauben, Scharniere, die Kontakte unter den Tasten und das Kabel."),
        thinElements=T("Back panel 3.2 mm and grille cloth 3 mm as declared sheets, scratches and the split cushion as 0.1 mm texture strokes.", "Rückwand 3,2 mm und Bespannung 3 mm als deklarierte Platten, Kratzer und das aufgeplatzte Polster als Texturstriche von 0,1 mm."),
        hatching=T("Vertical veneer grain on the near cheek and the key cover (lines 90 degrees, 1.2 mm), cross-hatched grille cloth in the front view (0.6 mm).", "Senkrechte Furniermaserung auf der nahen Wange und der Tastenklappe (Linien 90 Grad, 1,2 mm), kreuzschraffierte Bespannung in der Vorderansicht (0,6 mm)."),
        styles=T("Veneer 0.42 to 0.45, worn veneer 0.62, keys 0.9 and 0.08, faded panels 0.62 to 0.7, castors 0.15 and 0.45, bench 0.4 to 0.55, cushion 0.22; body pen 0.25 mm, detail 0.13 mm, texture 0.1 mm.", "Furnier 0,42 bis 0,45, abgegriffenes Furnier 0,62, Tasten 0,9 und 0,08, ausgeblichene Felder 0,62 bis 0,7, Rollen 0,15 und 0,45, Bank 0,4 bis 0,55, Polster 0,22; Körperstift 0,25 mm, Detail 0,13 mm, Textur 0,1 mm."),
        overlays=T("Every shape is the profile of its 3D part placed by its bone, so the closed cover hides the manuals in the stored pose and shows them standing open as music rest in the played pose; dolly, pedalboard, swell shoe, key and cover move on their own bones.", "Jede Form ist das Profil ihres 3D-Teils, platziert von ihrem Knochen, sodass die geschlossene Klappe die Manuale in der Lagerpose verdeckt und sie in der Spielpose als aufgestelltes Notenpult freigibt; Roller, Pedal, Schwellerschuh, Taste und Klappe bewegen sich auf eigenen Knochen."),
    ),
    100: dict(
        silhouette=T("The same stored organ at 11 mm height: cheek profile, cover band, back panel, dolly with castors, bench.", "Dieselbe eingelagerte Orgel mit 11 mm Höhe: Wangenprofil, Klappenband, Rückwand, Roller mit Rollen, Bank."),
        retained=T("The two manuals as yellowed key blocks (the middle C keeps its own bone and solid inside the lower block, too small to draw at this scale), side panels, key slip, name board, pedal deck with its five sharps, swell shoe, castors as wheel and housing, bench legs, frame and cushion, the price sticker.", "Die beiden Manuale als vergilbte Tastenblöcke (das mittlere C behält Knochen und Körper im unteren Block, zu klein zum Zeichnen in diesem Maßstab), Seitenfelder, Tastenleiste, Namensbrett, Pedaldeck mit seinen fünf Obertasten, Schwellerschuh, Rollen als Rad und Gehäuse, Bankbeine, Zarge und Polster, der Preisaufkleber."),
        merged=T("Black keys (11 mm, 0.11 mm on paper) into their rows, key action and key slip into one lower manual block of 85 mm, the upper keys onto their bed as one 40 mm row, grille cloth into the knee panel, pedal naturals and felt into one deck, castor plate, fork and legs into one housing, bench aprons and seat into one frame, carpet pads into the dolly boards.", "Schwarze Tasten (11 mm, 0,11 mm auf dem Papier) in ihre Reihen, Tastenmechanik und Tastenleiste in einen 85 mm hohen Block des Untermanuals, die oberen Tasten mit ihrem Boden in eine 40 mm hohe Reihe, Bespannung in das Kniebrett, Pedaluntertasten und Filz in ein Deck, Rollenplatte, Gabel und Beine in ein Gehäuse, Bankzargen und Sitz in einen Rahmen, Teppichauflagen in die Rollerbretter."),
        omitted=T("Key joints, tab strip, rhythm buttons, scratches, worn patches and the toe chamfer of the cheek (10 mm).", "Tastenfugen, Wippenleiste, Rhythmustaster, Kratzer, abgegriffene Stellen und die Fase der Wangenspitze (10 mm)."),
        thinElements=T("Back panel 3.2 mm and key cover 12 mm as declared sheets; the joint of the bottom board as one detail stroke.", "Rückwand 3,2 mm und Tastenklappe 12 mm als deklarierte Platten; die Fuge des Bodens als ein Detailstrich."),
        hatching=T("Cross-hatched grille in the front view (0.6 mm).", "Kreuzschraffierte Bespannung in der Vorderansicht (0,6 mm)."),
        styles=T("Tones as at 1:50; body pen 0.18 mm, detail 0.13 mm.", "Töne wie bei 1:50; Körperstift 0,18 mm, Detail 0,13 mm."),
        overlays=T("Shapes are the profiles of the 1:100 parts on their bones; cover, pedalboard, swell shoe and key move in every pose.", "Die Formen sind die Profile der Teile von 1:100 auf ihren Knochen; Klappe, Pedal, Schwellerschuh und Taste bewegen sich in jeder Pose."),
    ),
    200: dict(
        silhouette=T("A 5.6 mm figure: cheek with toe and key cheek, cover band, back panel, dolly deck on four castor blocks, bench as top and legs.", "Eine Figur von 5,6 mm: Wange mit Fußspitze und Tastenbacke, Klappenband, Rückwand, Rollerdeck auf vier Rollenblöcken, Bank als Platte und Beine."),
        retained=T("Cheeks, cabinet body, knee panel, the two manuals and the side panels as blocks, cover, pedal deck and rail, swell shoe, dolly, castors, bench legs and top.", "Wangen, Gehäusekörper, Kniebrett, die beiden Manuale und Seitenfelder als Blöcke, Klappe, Pedaldeck und -leiste, Schwellerschuh, Roller, Rollen, Bankbeine und -platte."),
        merged=T("Lid, tab panel, key beds, name board and the hollow inside into one cabinet body between 30 mm cheeks; panels of the upper level into the upper key bed; the pedal sharps into the deck; castors into blocks; bench seat, aprons and cushion into one top.", "Deckel, Registerbrett, Tastenböden, Namensbrett und der hohle Innenraum in einen Gehäusekörper zwischen 30 mm starken Wangen; die Felder der oberen Ebene in den oberen Tastenboden; die Pedalobertasten in das Deck; Rollen zu Blöcken; Banksitz, Zargen und Polster zu einer Platte."),
        omitted=T("The single middle C (22.5 mm), every key, all wear and the sticker.", "Das einzelne mittlere C (22,5 mm), jede Taste, alle Spuren und der Aufkleber."),
        thinElements=T("Back panel 3.2 mm as declared sheet; the 12 mm key cover is drawn and modelled 30 mm thick.", "Rückwand 3,2 mm als deklarierte Platte; die 12 mm starke Tastenklappe ist 30 mm stark gezeichnet und modelliert."),
        hatching=T("None.", "Keine."),
        styles=T("Veneer 0.42 to 0.45, keys 0.85, panels 0.62, dolly 0.6, castors 0.25, bench 0.4; body pen 0.3 mm and detail 0.18 mm, so that the 30 mm parts still print at 1:350.", "Furnier 0,42 bis 0,45, Tasten 0,85, Felder 0,62, Roller 0,6, Rollen 0,25, Bank 0,4; Körperstift 0,3 mm und Detail 0,18 mm, damit die 30-mm-Teile noch bei 1:350 drucken."),
        overlays=T("The profiles of the 1:200 parts on their bones; the cover still stands up as music rest when played.", "Die Profile der Teile von 1:200 auf ihren Knochen; die Klappe steht beim Spielen weiterhin als Notenpult."),
    ),
}

VOL_DECISIONS = {
    50: [T("Body: the organ as it is built, board by board: two 18 mm veneered chipboard cheeks with toe, knee recess and key cheek, 3.2 mm hardboard back, lid, inclined tab panel with its voice-tab strip, bottom board, knee panel with 3 mm grille cloth, lower key bed, key slip, name board, upper key bed, both 44-key manuals with every white key (repeat of 22.5 mm keys at 23.5 mm pitch) and every black key (one repeat per position in the octave), the side panels, the hinged 12 mm key cover, the swell pedal with base, bracket and shoe, the folded 13-note pedalboard (base, rail, cheeks, felt, eight naturals, five sharps on risers), the dolly of four beech boards with carpet pads on four swivel castors (plate, fork, two legs, wheel) and the bench (four tapered legs, aprons, seat board, cushion). Wear as texture solids: the hand-worn veneer of the near key cheek, the sticker and the worn front edge of the cover.", "Körper: die Orgel, wie sie gebaut ist, Brett für Brett: zwei furnierte Spanplattenwangen von 18 mm mit Fußspitze, Kniefreiraum und Tastenbacke, Hartfaserrückwand 3,2 mm, Deckel, schräges Registerbrett mit Wippenleiste, Boden, Kniebrett mit 3 mm Bespannung, unterer Tastenboden, Tastenleiste, Namensbrett, oberer Tastenboden, beide 44-tastigen Manuale mit jeder weißen Taste (Wiederholung von 22,5-mm-Tasten im Raster 23,5 mm) und jeder schwarzen Taste (eine Wiederholung je Lage in der Oktave), die Seitenfelder, die angeschlagene 12-mm-Tastenklappe, der Schweller mit Sockel, Lagerbock und Schuh, das hochgeklappte 13-tönige Pedal (Grundbrett, Leiste, Wangen, Filz, acht Unter-, fünf Obertasten auf Stegen), der Roller aus vier Buchenbrettern mit Teppich auf vier Lenkrollen (Platte, Gabel, zwei Schenkel, Rad) und die Bank (vier verjüngte Beine, Zargen, Sitzbrett, Polster). Spuren als Texturkörper: das abgegriffene Furnier der nahen Tastenbacke, der Aufkleber und die abgenutzte Vorderkante der Klappe."), T("Thermal: every body surface is its region by reference; the amplifier with its heat sink is a hidden box inside the closed cabinet; the outlet of the rear plume is a 1 mm patch on the vent field of the back panel. Structure: cheeks, bottom board, key bed, key frame, knee panel, lid and swell base carry the cabinet; cover, pedal base and rail, swell shoe and the middle C are the members of their bones; dolly boards, pads and castor parts, bench legs, aprons and seat carry the rest; hinges and pivots are joints; every permanent support (castors, bench feet) has its pad; amplifier, speakers and tone generator are masses.", "Thermik: jede Körperoberfläche ist per Verweis ihre Region; der Verstärker mit Kühlkörper ist ein verborgener Quader im geschlossenen Gehäuse; der Auslass der Abluftfahne ist ein 1-mm-Feld auf dem Lüftungsfeld der Rückwand. Tragwerk: Wangen, Boden, Tastenboden, Tastenrahmen, Kniebrett, Deckel und Schwellersockel tragen das Gehäuse; Klappe, Pedalgrundbrett und -leiste, Schwellerschuh und das mittlere C sind die Glieder ihrer Knochen; Rollerbretter, Auflagen und Rollenteile, Bankbeine, Zargen und Sitz tragen den Rest; Scharniere und Achsen sind Gelenke; jedes ständige Auflager (Rollen, Bankfüße) hat sein Lager; Verstärker, Lautsprecher und Tongenerator sind Massen.")],
    100: [T("Body: drawn anew for a 15 mm feature: the black keys, the key action and the key slip merge into an 85 mm lower manual block (the middle C keeps its bone and its column of that block), the upper keys and their bed into one 40 mm row, the grille cloth into a 19 mm knee panel, the pedal naturals with felt and base are one deck with the five sharps on it, castors are wheel and housing, the bench is legs, a seat frame and the cushion, the carpet pads are part of 25 mm dolly boards; the tab strip, the worn patches and the toe chamfer go; the sticker stays as a texture solid.", "Körper: neu für ein Merkmal von 15 mm: schwarze Tasten, Tastenmechanik und Tastenleiste gehen in einem 85 mm hohen Block des Untermanuals auf (das mittlere C behält seinen Knochen und seine Säule dieses Blocks), die oberen Tasten mit ihrem Boden in einer 40 mm hohen Reihe, die Bespannung in einem 19-mm-Kniebrett, Pedaluntertasten mit Filz und Grundbrett sind ein Deck mit den fünf Obertasten darauf, Rollen sind Rad und Gehäuse, die Bank sind Beine, ein Sitzrahmen und das Polster, die Teppichauflagen gehören zu 25 mm starken Rollerbrettern; Wippenleiste, abgegriffene Stellen und die Fase der Fußspitze entfallen; der Aufkleber bleibt als Texturkörper."), T("Thermal by reference to these solids plus the hidden amplifier and the plume outlet; structure with the same members, merged where the body merged, joints of 15 mm and 3 mm pads.", "Thermik per Verweis auf diese Körper plus verborgener Verstärker und Auslass der Abluftfahne; Tragwerk mit denselben Gliedern, zusammengefasst wo der Körper zusammengefasst ist, Gelenke von 15 mm und Lager von 3 mm.")],
    200: [T("Body: drawn anew for a 30 mm feature: 30 mm cheeks around one cabinet body that takes lid, tab panel, key beds, name board and the hollow inside; knee panel 30 mm; manuals and side panels as blocks on their beds; the key cover is generalised to 30 mm, the back panel stays a declared sheet; pedal deck and rail; a 30 mm swell shoe; the dolly as one deck on four castor blocks; the bench as two pairs of legs under one top. The single middle C is below the feature and only in the structure, as a slender member.", "Körper: neu für ein Merkmal von 30 mm: 30 mm starke Wangen um einen Gehäusekörper, der Deckel, Registerbrett, Tastenböden, Namensbrett und den hohlen Innenraum aufnimmt; Kniebrett 30 mm; Manuale und Seitenfelder als Blöcke auf ihren Böden; die Tastenklappe ist auf 30 mm verallgemeinert, die Rückwand bleibt eine deklarierte Platte; Pedaldeck und -leiste; ein 30 mm starker Schwellerschuh; der Roller als ein Deck auf vier Rollenblöcken; die Bank als zwei Beinpaare unter einer Platte. Das einzelne mittlere C liegt unter dem Merkmal und steht nur im Tragwerk, als schlankes Glied."), T("Thermal by reference plus the hidden amplifier (inside the body mass at this scale) and the plume outlet; structure: cheeks, a bottom, a key bed and a lid beam of 30 mm, knee panel and swell base for the cabinet, the merged dolly and bench members, joints of 30 mm and 6 mm pads.", "Thermik per Verweis plus verborgener Verstärker (bei diesem Maßstab in der Körpermasse) und Auslass der Abluftfahne; Tragwerk: Wangen, ein Boden-, ein Tastenboden- und ein Deckelbalken von 30 mm, Kniebrett und Schwellersockel für das Gehäuse, die zusammengefassten Glieder von Roller und Bank, Gelenke von 30 mm und Lager von 6 mm.")],
}

PENS = {50: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": None, "ground": 0.35}, 100: {"body": 0.18, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.3}, 200: {"body": 0.3, "detail": 0.18, "texture": None, "glazing": None, "ground": 0.35}}
HATCHES = {
    50: [{"id": "veneer", "title": T("Veneer grain", "Furniermaserung"), "pattern": "lines", "angleDeg": 90, "spacingMm": 1.2, "penMm": 0.1}, {"id": "cloth", "title": T("Grille cloth", "Bespannung"), "pattern": "cross", "angleDeg": 45, "spacingMm": 0.6, "penMm": 0.1}],
    100: [{"id": "cloth", "title": T("Grille cloth", "Bespannung"), "pattern": "cross", "angleDeg": 45, "spacingMm": 0.6, "penMm": 0.1}],
    200: [],
}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350)}


def representations():
    """🗺️ The authored drawings of 1:50, 1:100 and 1:200; 1:500 is unavailable."""
    out = {}
    for s in SCALES:
        out[str(s)] = {"status": "authored", "denominator": s, "validRange": {"minDenominator": RANGES[s][0], "maxDenominator": RANGES[s][1]}, "pens": PENS[s], "hatches": HATCHES[s], "geometry": shapes_of(s, parts_of(s)), "decisions": REP_DECISIONS[s]}
    out["500"] = {"status": "unavailable", "denominator": 500, "cause": "below-printable-feature", "reason": T("At 1:500 the whole organ with dolly and bench is 2.2 mm on paper, under the 3 mm below which a component is not drawn; its manuals, cover and dolly cannot be told apart.", "Bei 1:500 ist die ganze Orgel mit Roller und Bank 2,2 mm auf dem Papier groß, unter den 3 mm, unter denen ein Bauteil nicht gezeichnet wird; Manuale, Klappe und Roller sind nicht zu unterscheiden."), "limitingFeature": T("Overall height of 1.12 m on the dolly", "Gesamthöhe von 1,12 m auf dem Roller"), "limitingSizeM": 1.117, "alternatives": [{"kind": "scale", "denominator": 200}]}
    return out


def volumes():
    """🧊 The handcrafted 3D models of the authored scales."""
    out = {}
    for s in SCALES:
        parts = parts_of(s)
        out[str(s)] = {"status": "authored", "denominator": s, "body": [body_solid(p, s) for p in parts], "thermal": thermal_list(s, parts), "structure": structure_list(s), "decisions": VOL_DECISIONS[s]}
    out["500"] = {"status": "unavailable", "denominator": 500}
    return out


AREAS = {"cabinet-veneer": 3.97, "back-panel": 1.18, "grille-panel": 0.68, "keys": 0.51, "control-panels": 0.39, "pedals": 0.55, "dolly": 0.44, "bench": 1.17, "amplifier": 0.33}
HEAT = {"cabinet-veneer": 14, "back-panel": 44, "grille-panel": 8, "control-panels": 6, "amplifier": 18}

REGIONS = [
    ("cabinet-veneer", "Veneered cabinet: cheeks, lid, tab panel, key cover, key beds and slip", "Furniertes Gehäuse: Wangen, Deckel, Registerbrett, Tastenklappe, Tastenböden und -leiste", "surface", "veneer-aged", 0.98, "vertical-plate", 0.5, ("veneered-chipboard", "Veneered chipboard 18 mm", "Furnierte Spanplatte 18 mm", "timber", 0.018, 0.13, 650 * 1700), "en-312-2010"),
    ("back-panel", "Hardboard back panel with the vent slots of the amplifier", "Hartfaser-Rückwand mit den Lüftungsschlitzen des Verstärkers", "surface", "hardboard-aged", 0.98, "vertical-plate", 0.5, ("hardboard", "Hardboard 3.2 mm", "Hartfaserplatte 3,2 mm", "timber", 0.0032, 0.20, 900 * 1700), "douglas-1976"),
    ("grille-panel", "Knee panel with the speaker grille cloth", "Kniebrett mit der Lautsprecherbespannung", "surface", "grille-cloth", 0.58, "vertical-plate", 0.4, ("cloth-on-board", "Grille cloth on chipboard 16 mm", "Bespannung auf Spanplatte 16 mm", "timber", 0.019, 0.12, 650 * 1600), "douglas-1976"),
    ("keys", "Both manuals, yellowed white and greyed black keys", "Beide Manuale, vergilbte weiße und vergraute schwarze Tasten", "surface", "key-acrylic-yellowed", 0.15, "horizontal-plate-up", 0.6, ("key-shell", "Acrylic key shell 2.5 mm", "Acryl-Tastenschale 2,5 mm", "polymer", 0.0025, 0.19, 1180 * 1470), "rabek-1995"),
    ("control-panels", "Faded rhythm, master and preset panels with their lamps", "Ausgeblichene Rhythmus-, Haupt- und Registerfelder mit ihren Lämpchen", "surface", "panel-faded", 0.3, "horizontal-plate-up", 0.6, ("printed-aluminium", "Printed aluminium 1 mm", "Bedrucktes Aluminium 1 mm", "metal", 0.001, 160, 2700 * 900), "rabek-1995"),
    ("pedals", "13-note pedalboard and swell pedal", "13-töniges Pedal und Schweller", "surface", "pedal-beech", 0.25, "horizontal-plate-up", 0.4, ("beech", "Beech 20 mm", "Buche 20 mm", "timber", 0.020, 0.18, 700 * 1600), "fpl-2010"),
    ("dolly", "Furniture dolly with castors", "Möbelroller mit Lenkrollen", "surface", "dolly-beech", 0.45, "horizontal-plate-up", 0.3, ("beech-plywood", "Beech plywood 22 mm", "Buchensperrholz 22 mm", "timber", 0.022, 0.17, 700 * 1600), "en-12532-1999"),
    ("bench", "Organ bench with vinyl cushion", "Orgelbank mit Kunstlederpolster", "surface", "bench-veneer", 0.32, "horizontal-plate-up", 0.6, ("bench-board", "Veneered board 25 mm", "Furnierte Platte 25 mm", "timber", 0.025, 0.13, 650 * 1700), "fpl-2010"),
    ("amplifier", "Amplifier with heat sink inside the cabinet", "Verstärker mit Kühlkörper im Gehäuse", "heater", "amp-aluminium", 0.16, "vertical-plate", 0.0, ("heat-sink", "Aluminium heat sink 3 mm", "Aluminium-Kühlkörper 3 mm", "metal", 0.003, 160, 2700 * 900), "incropera-2018"),
]


def regions():
    """🔥 The thermal regions with their layers, heat release and nameplates."""
    out = []
    for rid, en, de, kind, mat, lc, geo, sky, (lid, len_, lde, lkind, th, k, rc), ref in REGIONS:
        hc = round(rc * th)
        r = {"id": rid, "title": T(en, de), "kind": kind, "model": "operating" if rid in HEAT else "passive", "material": mat, "areaM2": AREAS[rid], "characteristicLengthM": lc, "geometry": geo, "skyViewFactor": sky, "layers": [{"id": lid, "title": T(len_, lde), "kind": lkind, "material": mat, "thicknessM": th, "resistanceM2KW": round(th / k, 6), "heatCapacityJM2K": hc, "source": "authored-estimate", "reference": "iso-10456-2007"}], "heatCapacityJM2K": hc}
        if rid in HEAT:
            r["operating"] = {"key": "power", "heatW": HEAT[rid]}
        if rid == "back-panel":
            r["ratedPowerW"] = 90
            r["ratedPowerBasis"] = "electrical-input"
        if rid in ("dolly", "bench"):
            r["ratedPowerW"] = 0
            r["ratedPowerBasis"] = "passive"
        if rid == "amplifier":
            r["hidden"] = T("The amplifier and its heat sink sit inside the closed cabinet behind the back panel; no view sees them.", "Verstärker und Kühlkörper sitzen im geschlossenen Gehäuse hinter der Rückwand; keine Ansicht sieht sie.")
        r["source"] = "authored-estimate"
        r["reference"] = ref
        out.append(r)
    return out


def binding(power, contacts):
    """🌬️ The thermal binding of a pose: all regions indoors without sun, the given contact shares, the operating level and the plume."""
    regs = {}
    for rid, *_ in REGIONS:
        b = {"exposedFraction": 1, "sunlitFraction": 0}
        if rid in contacts:
            b["exposedFraction"] = round(1 - contacts[rid], 3)
            b["contact"] = {"fraction": contacts[rid], "tempC": 20}
        regs[rid] = b
    return {"operating": {"power": power}, "regions": regs, "emitters": {"rear-plume": 1 if power > 0 else 0}}


CASTOR_ANCHORS = ["castor-back-right", "castor-back-left", "castor-front-right", "castor-front-left"]
STORED_CONTACTS = {"dolly": 0.002, "bench": 0.005, "cabinet-veneer": 0.03}
PLAYED_CONTACTS = {"dolly": 0.002, "bench": 0.005, "cabinet-veneer": 0.01, "pedals": 0.15}


def load(lid, kind, en, de, anchor, fx, fy, ref="douglas-1976"):
    """🧲 An applied load at an anchor."""
    return {"id": lid, "kind": kind, "title": T(en, de), "anchor": anchor, "fxN": fx, "fyN": fy, "source": "authored-estimate", "reference": ref}


def poses():
    """🧍 Stored (default), on show and warm, carried, played."""
    open_cover = {"key-cover": {"angleDeg": rnd(COVER_OPEN, 3)}}
    return [
        {"id": "stored", "title": T("Stored on the dolly", "Auf dem Roller eingelagert"), "description": T("As the dealer keeps it: on a four-castor furniture dolly, key cover closed over both manuals, pedalboard folded up against the knee panel, switched off; the bench stands in front where the player will sit.", "Wie der Händler sie aufbewahrt: auf einem Möbelroller mit vier Lenkrollen, die Tastenklappe über beiden Manualen geschlossen, das Pedal an das Kniebrett hochgeklappt, ausgeschaltet; die Bank steht davor, wo der Spieler sitzen wird."), "bones": {}, "ground": "auto", "contacts": CASTOR_ANCHORS + ["bench-feet"], "thermal": binding(0, STORED_CONTACTS), "loads": []},
        {"id": "on-show-warm", "title": T("On show, running warm", "Vorgeführt, warmgelaufen"), "description": T("Still on the dolly, the key cover stands up as music rest and the organ has been switched on for buyers for half an hour: 40 W of idle amplifier and lamps warm the back panel and its vent field.", "Noch auf dem Roller, die Tastenklappe steht als Notenpult und die Orgel ist seit einer halben Stunde für Kaufinteressenten eingeschaltet: 40 W Ruheleistung von Verstärker und Lämpchen erwärmen die Rückwand und ihr Lüftungsfeld."), "bones": open_cover, "ground": "auto", "contacts": CASTOR_ANCHORS + ["bench-feet"], "thermal": binding(0.45, STORED_CONTACTS), "loads": []},
        {"id": "carried", "title": T("Rolled away on the dolly", "Auf dem Roller weggerollt"), "description": T("Two movers roll the organ one metre back on its dolly, each pulling 6 N at a back top corner, which is what the rolling resistance of the castors (0.02 of the 692 N on them) asks at walking pace; the bench stays where it stood.", "Zwei Träger rollen die Orgel auf ihrem Roller einen Meter zurück, jeder zieht mit 6 N an einer hinteren oberen Ecke, so viel wie der Rollwiderstand der Lenkrollen (0,02 der 692 N auf ihnen) im Schritttempo verlangt; die Bank bleibt stehen."), "bones": {"carriage": {"dx": -1.0}}, "ground": "auto", "contacts": CASTOR_ANCHORS + ["bench-feet"], "thermal": binding(0, STORED_CONTACTS), "loads": [load("pull-right", "carried", "Right mover pulling", "Rechter Träger zieht", "grip-right", -6, 0, "en-12532-1999"), load("pull-left", "carried", "Left mover pulling", "Linker Träger zieht", "grip-left", -6, 0, "en-12532-1999")]},
        {"id": "played", "title": T("Played from the bench", "Von der Bank gespielt"), "description": T("Lifted off the dolly onto its cheeks, the dolly left behind it, pedalboard folded down, key cover standing as music rest, switched on at playing volume (54 W): both hands on the manuals, the left foot on low C, the right foot on the swell pedal, the player on the bench.", "Vom Roller auf die Wangen gehoben, der Roller bleibt dahinter, Pedal heruntergeklappt, Tastenklappe als Notenpult, auf Spiellautstärke eingeschaltet (54 W): beide Hände auf den Manualen, der linke Fuß auf dem tiefen C, der rechte auf dem Schweller, der Spieler auf der Bank."), "bones": {"cabinet": {"dy": -H_D}, "dolly": {"dx": -0.75}, "key-cover": {"angleDeg": rnd(COVER_OPEN, 3)}, "pedalboard": {"angleDeg": -90}, "swell-pedal": {"angleDeg": -5}}, "ground": "auto", "contacts": ["cheek-foot-right", "cheek-foot-left", "pedalboard-floor", "bench-feet"] + CASTOR_ANCHORS, "thermal": binding(0.6, PLAYED_CONTACTS), "loads": [
            load("hands-lower", "live", "Hands resting and playing on the lower manual", "Hände ruhend und spielend auf dem Untermanual", "lower-manual-hands", 0, -15),
            load("hand-upper", "live", "Right hand on the upper manual", "Rechte Hand auf dem Obermanual", "upper-manual-hand", 0, -8),
            load("finger-c", "live", "Finger pressing middle C", "Finger drückt das mittlere C", "key-c-top", 0, -1),
            load("foot-swell", "live", "Right foot on the swell pedal", "Rechter Fuß auf dem Schweller", "swell-toe", 0, -40),
            load("foot-pedal", "live", "Left foot on low C", "Linker Fuß auf dem tiefen C", "pedal-toe", 0, -30),
            load("player", "occupant", "Player seated on the bench (60 kg on the seat)", "Spieler auf der Bank (60 kg auf dem Sitz)", "bench-seat", 0, -588, "fpl-2010")]},
    ]


def clips():
    """🎞️ Carry, open and warm up, and a looping phrase."""
    all_s = CASTOR_ANCHORS + ["bench-feet"]
    played_c = ["cheek-foot-right", "cheek-foot-left", "pedalboard-floor", "bench-feet"] + CASTOR_ANCHORS
    return [
        {"id": "carry", "title": T("Roll away on the dolly", "Auf dem Roller wegrollen"), "description": T("The movers take hold for a second and a half, roll the organ one metre back in five and a half seconds and stop; the bench stays.", "Die Träger fassen anderthalb Sekunden an, rollen die Orgel in fünfeinhalb Sekunden einen Meter zurück und halten an; die Bank bleibt stehen."), "durationS": 8, "fps": 25, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1.5, "pose": "stored", "ease": "smooth"}, {"t": 7, "pose": "carried", "ease": "smooth"}, {"t": 8, "pose": "carried", "ease": "step"}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 8} for a in all_s], "phases": [{"id": "grip", "fromS": 0, "toS": 1.5, "title": T("Taking hold", "Anfassen")}, {"id": "roll", "fromS": 1.5, "toS": 7, "title": T("Rolling", "Rollen")}, {"id": "stop", "fromS": 7, "toS": 8, "title": T("Stopped", "Angehalten")}],
         "events": [{"t": 1.5, "kind": "load-on", "title": T("Movers pull", "Träger ziehen")}, {"t": 7, "kind": "motion", "title": T("Dolly stops", "Roller hält")}], "tracks": []},
        {"id": "open-and-warm-up", "title": T("Open the cover and switch on", "Klappe öffnen und einschalten"), "description": T("The dealer lifts the key cover in three seconds until it stands as music rest, switches the organ on at four seconds and leaves it idling while it warms up.", "Der Händler hebt die Tastenklappe in drei Sekunden, bis sie als Notenpult steht, schaltet die Orgel nach vier Sekunden ein und lässt sie im Leerlauf warmlaufen."), "durationS": 60, "fps": 5, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 3, "pose": "on-show-warm", "ease": "step"}, {"t": 60, "pose": "on-show-warm", "ease": "step"}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 60} for a in all_s], "phases": [{"id": "lift", "fromS": 0, "toS": 3, "title": T("Cover lifted", "Klappe gehoben")}, {"id": "idle", "fromS": 3, "toS": 60, "title": T("Idling, warming up", "Leerlauf, warmlaufend")}],
         "events": [{"t": 0, "kind": "motion", "title": T("Cover lifted", "Klappe gehoben"), "target": "key-cover"}, {"t": 4, "kind": "operating", "title": T("Switched on", "Eingeschaltet"), "target": "power"}],
         "tracks": [{"target": "operating:power", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 4, "v": 0.45}]}, {"target": "emitter:rear-plume", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 4, "v": 1}]}]},
        {"id": "phrase", "title": T("Play a phrase with swell", "Eine Phrase mit Schweller spielen"), "description": T("A two-second loop: middle C goes down 3.8 degrees and up twice while the right foot opens the swell from 5 to 12 degrees and back.", "Eine Schleife von zwei Sekunden: das mittlere C geht zweimal um 3,8 Grad hinunter und hinauf, während der rechte Fuß den Schweller von 5 auf 12 Grad und zurück öffnet."), "durationS": 2, "fps": 25, "loop": True, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "played", "ease": "smooth", "bones": {"lower-key": {"angleDeg": 0}, "swell-pedal": {"angleDeg": -5}}}, {"t": 0.5, "pose": "played", "ease": "smooth", "bones": {"lower-key": {"angleDeg": -3.8}, "swell-pedal": {"angleDeg": -9}}}, {"t": 1.0, "pose": "played", "ease": "smooth", "bones": {"lower-key": {"angleDeg": 0}, "swell-pedal": {"angleDeg": -12}}}, {"t": 1.5, "pose": "played", "ease": "smooth", "bones": {"lower-key": {"angleDeg": -3.8}, "swell-pedal": {"angleDeg": -8}}}, {"t": 2, "pose": "played", "ease": "smooth", "bones": {"lower-key": {"angleDeg": 0}, "swell-pedal": {"angleDeg": -5}}}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 2} for a in played_c], "phases": [{"id": "phrase", "fromS": 0, "toS": 2, "title": T("Phrase", "Phrase")}],
         "events": [{"t": 0.5, "kind": "motion", "title": T("Key down", "Taste unten"), "target": "lower-key"}, {"t": 1.5, "kind": "motion", "title": T("Key down", "Taste unten"), "target": "lower-key"}], "tracks": []},
    ]


def thermal():
    """🌡️ The thermal model: a showroom, one power key, nine regions and the rear plume."""
    return {
        "environment": {"airTempC": 20, "meanRadiantTempC": 20, "relativeHumidity": 0.45, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 0, "pressurePa": 101325},
        "passengers": [],
        "operating": [{"id": "power", "title": T("Mains power", "Netzleistung"), "levels": [{"v": 0, "title": T("Off", "Aus")}, {"v": 0.45, "title": T("Idling, on show (40 W)", "Leerlauf, vorgeführt (40 W)")}, {"v": 0.6, "title": T("Playing at room volume (54 W)", "Spiel in Zimmerlautstärke (54 W)")}, {"v": 1, "title": T("Full volume, rated (90 W)", "Volle Lautstärke, Nennleistung (90 W)")}]}],
        "regions": regions(),
        "emitters": [{"id": "rear-plume", "kind": "plume", "title": T("Warm air rising from the vent slots of the back panel", "Warme Luft aus den Lüftungsschlitzen der Rückwand"), "anchor": "rear-vent", "directionDeg": 0, "diameterM": 0.25, "region": "back-panel", "source": "authored-estimate", "reference": "incropera-2018"}],
        "assumptions": [
            T("Every watt ends as heat in the room. At the rated 90 W: 44 W through the back panel and its vent field (the amplifier stands behind it), 18 W from the amplifier heat sink inside, 14 W through the veneered cabinet, 8 W from the speakers through the grille, 6 W from panel lamps and the rhythm unit; the keys, pedals, dolly and bench release nothing of their own. Idle on show is level 0.45 (40 W), playing at room volume 0.6 (54 W).", "Jedes Watt endet als Wärme im Raum. Bei den Nenn-90 W: 44 W über die Rückwand und ihr Lüftungsfeld (der Verstärker steht dahinter), 18 W vom Kühlkörper innen, 14 W über das furnierte Gehäuse, 8 W von den Lautsprechern über die Bespannung, 6 W von Lämpchen und Rhythmusgerät; Tasten, Pedal, Roller und Bank geben nichts Eigenes ab. Leerlauf beim Vorführen ist Stufe 0,45 (40 W), Spiel in Zimmerlautstärke 0,6 (54 W)."),
            T("Aged surfaces: the lacquer of the veneer has faded (solar absorptivity 0.62 instead of about 0.75 new), the white keys have yellowed (0.38 instead of 0.25), the printed panels have faded (0.45 instead of 0.6), the hand contacts are worn to bare wood; emissivities stay near 0.9. Authored estimates after the photodegradation literature.", "Gealterte Oberflächen: der Lack des Furniers ist ausgeblichen (solare Absorption 0,62 statt neu etwa 0,75), die weißen Tasten sind vergilbt (0,38 statt 0,25), die bedruckten Felder ausgeblichen (0,45 statt 0,6), die Griffstellen bis aufs Holz abgegriffen; die Emissionsgrade bleiben nahe 0,9. Schätzwerte nach der Literatur zur Photodegradation."),
            T("The organ has no fan: the plume of the vent field rises from the back panel by free convection.", "Die Orgel hat keinen Lüfter: die Fahne des Lüftungsfelds steigt durch freie Konvektion von der Rückwand auf."),
        ],
    }


def structure():
    """🏗️ The structural model."""
    return {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": CASTOR_ANCHORS + ["bench-feet"], "assumptions": [
        T(f"Rigid bodies on unilateral contacts. Stored and carried, the {MASSES['cabinet'][0] + MASSES['key-cover'][0] + MASSES['pedalboard'][0] + MASSES['swell-pedal'][0] + MASSES['lower-key'][0]:.1f} kg organ stands with its bottom board on the dolly and the dolly on four castors 0.36 m by 0.50 m apart; played, it stands on its two cheeks and the pedalboard on the floor.", "Starre Körper auf einseitigen Kontakten. Eingelagert und gerollt steht die " + f"{MASSES['cabinet'][0] + MASSES['key-cover'][0] + MASSES['pedalboard'][0] + MASSES['swell-pedal'][0] + MASSES['lower-key'][0]:.1f}".replace(".", ",") + f" kg schwere Orgel mit dem Boden auf dem Roller und der Roller auf vier Lenkrollen im Abstand von 0,36 m mal 0,50 m; gespielt steht sie auf ihren beiden Wangen und das Pedal auf dem Boden."),
        T("Masses are section times length times density of the 1:50 solids plus 9.0 kg of electronics (amplifier with transformer 4.5 kg, two speakers 3.2 kg, tone generator 1.3 kg); playing loads are hand forces of 1 to 15 N, foot forces of 30 and 40 N and 60 kg of the player on the bench; the movers pull 6 N each, the rolling resistance of the castors at constant pace.", "Die Massen sind Querschnitt mal Länge mal Dichte der Körper von 1:50 plus 9,0 kg Elektronik (Verstärker mit Transformator 4,5 kg, zwei Lautsprecher 3,2 kg, Tongenerator 1,3 kg); die Spiellasten sind Handkräfte von 1 bis 15 N, Fußkräfte von 30 und 40 N und 60 kg des Spielers auf der Bank; die Träger ziehen je 6 N, den Rollwiderstand der Lenkrollen bei gleichmäßigem Tempo."),
        T("An electronic organ has no string tension: the only loads of the frame are its own weight, the key and pedal forces and the handling forces.", "Eine elektronische Orgel hat keinen Saitenzug: das Gestell trägt nur sein Eigengewicht, die Tasten- und Pedalkräfte und die Kräfte beim Bewegen."),
    ], "clinical": False}


REFERENCES = [
    ("hornbostel-sachs-1914", "Hornbostel, E. M. von, Sachs, C. (1914). Systematik der Musikinstrumente. Ein Versuch. Zeitschrift für Ethnologie 46(4-5), 553-590 (the classification; electrophones were added as class 5 by Sachs 1940)."),
    ("mimo-2011", "MIMO Consortium (2011). Revision of the Hornbostel-Sachs Classification of Musical Instruments by the MIMO Consortium, class 53 (radioelectric, analogue electronic instruments), keyboard-operated."),
    ("douglas-1976", "Douglas, A. (1976). The Electronic Musical Instrument Manual, 6th ed. Pitman, London (construction of electronic home organs: spinet manuals of 44 keys from F with the one-octave offset, 13-note pedal clavier, amplifier and speaker cabinet, power consumption)."),
    ("dorf-1968", "Dorf, R. H. (1968). Electronic Musical Instruments, 3rd ed. Radiofile, New York (console layout and dimensions of spinet and console organs, expression pedal)."),
    ("ago-console-1961", "American Guild of Organists (1961, rev.). AGO Standard Console Specifications (relation of manuals, bench and pedal clavier; the 13-note flat spinet pedalboard is the domestic short form)."),
    ("iec-60065-1985", "IEC 60065:1985 Safety requirements for mains operated electronic and related apparatus for household and similar general use (permissible temperature rise of accessible surfaces)."),
    ("en-312-2010", "EN 312:2010 Particleboards - Specifications (density about 650 kg/m3, bending strength and modulus of furniture boards)."),
    ("en-12532-1999", "EN 12532:1999 Castors and wheels - Castors and wheels for applications up to 1.1 m/s (load and rolling resistance of swivel castors of furniture dollies)."),
    ("fpl-2010", "Forest Products Laboratory (2010). Wood Handbook - Wood as an Engineering Material, FPL-GTR-190, USDA (density, modulus and strength of beech and plywood)."),
    ("iso-10456-2007", "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (conductivity, density and heat capacity of wood-based panels, polymers, metals)."),
    ("rabek-1995", "Rabek, J. F. (1995). Polymer Photodegradation: Mechanisms and Experimental Methods. Chapman & Hall, London (yellowing of acrylic and ABS, fading of lacquers and prints)."),
    ("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed. Wiley (free convection from vertical plates, plumes, radiative properties)."),
]


def dims():
    """📏 The rest-pose envelope from the 1:50 parts, including the key cover band."""
    xs, ys, zs = [], [], []
    for p in [q for q in parts_of(50) if not q["texture"]]:
        w = world_profile(p)
        xs += [x for x, _ in w]
        ys += [y for _, y in w]
        for z0, z1 in copies(p):
            zs += [z0, z1]
    return round(max(ys) - min(ys), 3), round(max(xs) - min(xs), 3), round(max(zs) - min(zs), 3)


def asset():
    """🎹 The whole asset."""
    h, w, d = dims()
    organ_kg = sum(MASSES[b][0] for b in ("cabinet", "key-cover", "pedalboard", "swell-pedal", "lower-key"))
    return {
        "id": "used-electronic-organ", "category": "appliances",
        "title": T("Used Home Organ", "Gebrauchte Heimorgel"),
        "summary": T(f"A second-hand two-manual home organ of the 1980s, 1.10 m wide, 0.55 m deep and 0.98 m high, {organ_kg:.0f} kg: two 44-key manuals with the spinet offset of one octave, a 13-note pedalboard, rhythm unit, swell pedal and a 90 W amplifier in a veneered chipboard cabinet. Thirty years have yellowed the keys, faded the panels and the lacquer and worn the veneer at the key cheeks. At the dealer it stands on a furniture dolly with its bench in front, is switched on warm for buyers and is played from the bench again.", f"Eine gebrauchte zweimanualige Heimorgel der 1980er Jahre, 1,10 m breit, 0,55 m tief und 0,98 m hoch, {organ_kg:.0f} kg: zwei 44-tastige Manuale mit dem Spinett-Versatz von einer Oktave, ein 13-töniges Pedal, Rhythmusgerät, Schweller und ein 90-W-Verstärker in einem furnierten Spanplattengehäuse. Dreißig Jahre haben die Tasten vergilbt, Felder und Lack ausgebleicht und das Furnier an den Tastenbacken abgegriffen. Beim Händler steht sie auf einem Möbelroller mit der Bank davor, wird für Kaufinteressenten warmgelaufen vorgeführt und von der Bank aus wieder gespielt."),
        "tags": ["used", "instrument", "music", "second-hand", "keyboard", "amplification", "electronic-organ", "home-organ", "spinet", "electrophone", "veneer", "chipboard", "dolly", "bench", "electrical", "heat-source", "internal-gain"],
        "provenance": {"author": "Semio staffage content (used instruments wave)", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": i, "citation": c} for i, c in REFERENCES], "notes": T(
            f"A two-manual spinet home organ of the 1980s as a type (no single product, no brand), Hornbostel-Sachs 53 (electrophone, analogue electronic, keyboard-operated). Cabinet 1.10 m wide (z), 0.55 m deep (x) and 0.983 m high to the lid; lower manual top 0.705 m and upper manual top 0.805 m above the floor, both 44 keys F to C (26 white keys of 22.5 mm at 23.5 mm pitch, 18 black keys), the lower one an octave (164.5 mm) to the bass side; 13-note flat pedalboard C to C of 0.50 m with naturals at 57.5 mm pitch; swell pedal right of it; 12 mm hinged key cover that stands up as music rest; bench 0.80 x 0.32 m, seat 0.575 m high. Organ {organ_kg:.1f} kg, dolly {MASSES['dolly'][0]:.1f} kg, bench {MASSES['bench'][0]:.1f} kg, together {TOTAL_KG} kg (section x length x density of every 1:50 solid plus 9.0 kg of electronics). Rated input 90 W (catalogue range of the class 60 to 120 W). Dimensions are catalogue-typical authored estimates of the class; nothing is measured.",
            f"Eine zweimanualige Spinett-Heimorgel der 1980er Jahre als Typ (kein einzelnes Produkt, keine Marke), Hornbostel-Sachs 53 (Elektrophon, analog elektronisch, mit Tastatur). Gehäuse 1,10 m breit (z), 0,55 m tief (x) und bis zum Deckel 0,983 m hoch; Oberkante Untermanual 0,705 m und Obermanual 0,805 m über dem Boden, beide mit 44 Tasten von F bis C (26 weiße Tasten von 22,5 mm im Raster 23,5 mm, 18 schwarze), das untere um eine Oktave (164,5 mm) zum Bass versetzt; 13-töniges flaches Pedal C bis C von 0,50 m mit Untertasten im Raster 57,5 mm; der Schweller rechts davon; 12 mm starke angeschlagene Tastenklappe, die als Notenpult aufgestellt wird; Bank 0,80 x 0,32 m, Sitzhöhe 0,575 m. Orgel {organ_kg:.1f} kg, Roller {MASSES['dolly'][0]:.1f} kg, Bank {MASSES['bench'][0]:.1f} kg, zusammen {TOTAL_KG} kg (Querschnitt mal Länge mal Dichte jedes Körpers von 1:50 plus 9,0 kg Elektronik). Nennaufnahme 90 W (Katalogbereich der Klasse 60 bis 120 W). Die Maße sind katalogtypische Schätzwerte der Klasse; nichts ist gemessen.".replace(f"{organ_kg:.1f}", f"{organ_kg:.1f}".replace(".", ",")).replace(f"{MASSES['dolly'][0]:.1f}", f"{MASSES['dolly'][0]:.1f}".replace(".", ",")).replace(f"{MASSES['bench'][0]:.1f}", f"{MASSES['bench'][0]:.1f}".replace(".", ",")).replace(f"{TOTAL_KG}", f"{TOTAL_KG}".replace(".", ",")))},
        "dimensions": {"heightM": h, "widthM": w, "depthM": d, "massKg": TOTAL_KG},
        "materials": materials(), "rig": rig(), "anchors": anchors(), "representations": representations(), "symbols": [], "poses": poses(), "clips": clips(), "thermal": thermal(), "structure": structure(), "volumes": volumes(),
        "presentation": {"defaultPose": "stored", "defaultClip": "carry", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }


if __name__ == "__main__":
    a = asset()
    order_keys = ["id", "category", "title", "summary", "tags", "provenance", "dimensions", "materials", "rig", "anchors", "representations", "symbols", "poses", "clips", "thermal", "structure", "volumes", "presentation", "exports", "completion"]
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": {k: a[k] for k in order_keys}}
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as fh:
        json.dump(doc, fh, ensure_ascii=False, indent=1)
        fh.write("\n")
    print("[DEBUG] dims", a["dimensions"], "masses", {b: round(m[0], 2) for b, m in MASSES.items()})
