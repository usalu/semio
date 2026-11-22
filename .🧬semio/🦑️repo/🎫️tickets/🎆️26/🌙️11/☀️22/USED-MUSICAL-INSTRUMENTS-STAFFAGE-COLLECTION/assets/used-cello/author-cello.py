"""🎻 Writes the asset document `🔣️.json` of the used 4/4 cello with its wheeled hard case: the one authoritative script of this asset.

Run: `python3 author-cello.py` in this folder. Every number is stated in `author_model.py` (cello and case), `author_parts.py` (3D solids per
scale) and `author_draw.py` (2D shapes per scale); this script adds materials, rig, anchors, poses, clips, thermal and structural models,
decisions and the exports, and writes the document next to itself.

@see ../../brief.md — the binding brief of the wave
@see ../../reports/used-cello.md — reference table, measurements and decisions
"""
import json
import math
from pathlib import Path

import author_draw as D
import author_parts as P
from author_geom import mesh_area, r6
from author_model import (BODY_U0, BRIDGE_U, CASE_TOP, CXC, CYC, ENDPIN_LEN, ENDPIN_REST_TIP, LID_ORIGIN, NUT_U, SCROLL_TOP, WALL, XP, back_v,
                          case_b, case_back_x, case_outer_areas, corpus_polyhedron, fb_top, string_v, B)

HERE = Path(__file__).parent
SCALES = (50, 100, 200, 500)
T = lambda en, de: {"en": en, "de": de}

TILT = 32.0
TIP = (1.250, 0.011)
ENDPIN_OUT = -0.300
STRING_TENSION = 650.0


def rot(deg):
    """🧭 The world (u, v) axes of a bone pointing at `deg` (counter-clockwise from +x)."""
    a = math.radians(deg)
    return (math.cos(a), math.sin(a)), (math.cos(a + math.pi / 2), math.sin(a + math.pi / 2))


def played_layout():
    """📐 The played pose: cello origin, carrier and stopper offsets in the case frame, bow carrier offset in the cello frame."""
    u, v = rot(90 + TILT)
    tip_u = ENDPIN_REST_TIP + ENDPIN_OUT
    ox, oy = TIP[0] - tip_u * u[0], TIP[1] - tip_u * u[1]
    carrier = (oy - CYC, -(ox - CXC))
    stopper = (0.0 - P.STOPPER_C[1], -(TIP[0] - P.STOPPER_C[0]))
    piv = P.bow_pivot_cello()
    contact_u = BRIDGE_U + 0.060
    d = string_v(contact_u, -0.0075) - 0.00065
    bow = (contact_u - piv[0], (d - 0.001) - piv[1])
    return {"origin": (ox, oy), "carrier": carrier, "stopper": stopper, "bow": bow, "contact": (contact_u, d)}


LAY = played_layout()


def axis_deg(d):
    """📐 The prismatic axis angle of an offset (du, dv) in its parent frame."""
    return math.degrees(math.atan2(d[1], d[0]))


def along(d):
    """➡️ A prismatic offset exactly on the rounded axis of its joint (the travel is the length of `d`)."""
    a = math.radians(round(axis_deg(d), 4))
    s = math.hypot(d[0], d[1])
    return {"dx": s * math.cos(a), "dy": s * math.sin(a)}


def materials():
    """🧱 The materials of case, cello, bow and stopper with their thermal and structural data."""
    m = lambda i, en, de, e, a, rho, k, c, E, src, ref: {"id": i, "title": T(en, de), "emissivity": e, "solarAbsorptivity": a, "densityKgM3": rho, "conductivityWmK": k, "specificHeatJKgK": c, "elasticModulusPa": E, "source": src, "reference": ref}
    return [
        m("abs-eps", "Thermoformed ABS shell over EPS foam, aged and scuffed", "Tiefgezogene ABS-Schale über EPS-Schaum, gealtert und verschrammt", 0.9, 0.85, 285, 0.044, 1400, 2.2e9, "authored-estimate", "iso-10456-2007"),
        m("webbing", "Polyester webbing over foam pads, faded", "Polyester-Gurtband über Schaumpolstern, ausgebleicht", 0.9, 0.7, 300, 0.05, 1300, 1.0e8, "authored-estimate", "iso-10456-2007"),
        m("rubber-grip", "Rubber handle grip", "Gummi-Griff", 0.92, 0.9, 1100, 0.16, 1900, 5.0e6, "literature", "iso-10456-2007"),
        m("nickel-steel", "Nickel-plated steel, tarnished", "Vernickelter Stahl, angelaufen", 0.25, 0.45, 7850, 50, 450, 2.1e11, "literature", "incropera-2018"),
        m("pu-wheel", "Polyurethane wheel, worn tread", "Polyurethan-Rad, abgelaufen", 0.9, 0.9, 1200, 0.25, 1800, 5.0e7, "authored-estimate", "iso-10456-2007"),
        m("rubber-bumper", "Rubber bumper", "Gummipuffer", 0.92, 0.9, 1100, 0.16, 1900, 5.0e6, "literature", "iso-10456-2007"),
        m("foam-lining", "Velour-covered PU foam lining", "Velourbezogene PU-Schaumpolsterung", 0.95, 0.7, 35, 0.035, 1400, 1.0e6, "authored-estimate", "iso-10456-2007"),
        m("spruce-varnish", "Spruce top under aged oil varnish (crazed, rubbed through at the upper bouts)", "Fichtendecke unter gealtertem Öllack (krakeliert, an den oberen Backen durchgerieben)", 0.9, 0.6, 420, 0.11, 1700, 1.1e10, "literature", "wood-handbook-2010"),
        m("spruce-bare", "Bare spruce where the varnish is worn through", "Blanke Fichte, wo der Lack durchgerieben ist", 0.88, 0.45, 420, 0.11, 1700, 1.1e10, "authored-estimate", "wood-handbook-2010"),
        m("maple-varnish", "Flamed maple under aged oil varnish", "Geflammter Ahorn unter gealtertem Öllack", 0.9, 0.6, 650, 0.16, 1600, 1.2e10, "literature", "wood-handbook-2010"),
        m("maple-bare", "Scratched maple of the rib (varnish scraped off)", "Zerkratzter Ahorn der Zarge (Lack abgeschabt)", 0.88, 0.5, 650, 0.16, 1600, 1.2e10, "authored-estimate", "wood-handbook-2010"),
        m("maple-bridge", "Unvarnished maple bridge", "Unlackierter Ahornsteg", 0.9, 0.45, 690, 0.17, 1600, 1.3e10, "literature", "wood-handbook-2010"),
        m("ebony", "Ebony, finger-worn", "Ebenholz, griffpoliert", 0.9, 0.9, 1150, 0.2, 1500, 1.7e10, "literature", "wood-handbook-2010"),
        m("steel-string", "Steel and tungsten wound strings", "Stahl- und wolframumsponnene Saiten", 0.3, 0.5, 7850, 50, 450, 2.0e11, "literature", "incropera-2018"),
        m("endpin-steel", "Chromed steel endpin, scratched", "Verchromter Stahlstachel, zerkratzt", 0.2, 0.4, 7850, 50, 450, 2.1e11, "literature", "incropera-2018"),
        m("rosewood", "Rosewood disc with rubber sole", "Palisanderscheibe mit Gummisohle", 0.9, 0.85, 850, 0.17, 1500, 1.2e10, "literature", "wood-handbook-2010"),
        m("pernambuco", "Pernambuco bow stick, polished", "Fernambuk-Bogenstange, poliert", 0.9, 0.75, 1000, 0.2, 1500, 2.5e10, "literature", "wood-handbook-2010"),
        m("horsehair", "Horsehair, rosined and worn thin", "Rosshaar, kolophoniert und ausgedünnt", 0.9, 0.3, 1300, 0.2, 1500, 1.0e10, "authored-estimate", "incropera-2018"),
        m("leather-wrap", "Leather thumb grip and silver lapping, worn", "Lederdaumenauflage und Silberwicklung, abgegriffen", 0.85, 0.7, 2500, 0.3, 900, 1.0e9, "authored-estimate", "incropera-2018"),
        m("rosin", "Rosin dust", "Kolophoniumstaub", 0.9, 0.3, 1070, 0.1, 1300, 1.0e9, "authored-estimate", "incropera-2018"),
    ]


def masses():
    """⚖️ Bone masses from sections, areas and densities (kg) with their breakdown, the single source of `dimensions.massKg`."""
    back_area, lid_area = case_outer_areas(24)
    shell_kg = lambda area: area * (0.002 * 1050 + 0.006 * 30)
    sp = P.strap_profile(P.STRAP_Z, P.STRAP_Y0, P.STRAP_Y1, 14)
    strap_len = sum(math.hypot(b[0] - a[0], b[1] - a[1]) for a, b in zip(sp[:15], sp[1:15]))
    straps = 2 * strap_len * P.STRAP_W * P.STRAP_T * 300
    handle = math.pi * 0.0125 ** 2 * P.HANDLE_L * 1100 + 2 * 0.022 * 0.026 * 0.036 * 1100 * 0.5
    latches = 3 * 0.040
    lining = (back_area + lid_area) * 0.006 * 35 + 0.24 * 0.10 * 0.004 * 35 + 2 * 0.08 * 0.12 * 0.012 * 35 + 0.08 * 0.067 * 0.05 * 35
    housings = 2 * 0.056 * 0.05 * 0.04 * 285
    bumper = 0.009 * 0.03 * 0.14 * 1100
    hinges = 2 * 0.030
    spinners = 2 * 0.006
    wheels = 2 * math.pi * 0.032 ** 2 * 0.024 * 1200 * 0.55 + math.pi * 0.006 ** 2 * 0.24 * 7850
    case_kg = shell_kg(back_area) + straps + handle + latches + housings + bumper + lining * 0.55
    lid_kg = shell_kg(lid_area) + hinges + spinners + lining * 0.45
    tp = P.plate_polyhedron(60, 9, True, 0.004)[3]
    bp = P.plate_polyhedron(60, 9, False, 0.004)[3]
    ribs = 2.05 * 0.120 * 0.0014 * 650
    blocks = 0.12
    bar = 0.60 * 0.025 * 0.012 * 420
    post = math.pi * 0.0055 ** 2 * 0.16 * 420
    neck = 0.00030 * 650
    pegbox = 0.00034 * 650
    fb = 0.58 * 0.033 * 0.0105 * 1150
    tail = 0.232 * 0.049 * 0.016 * 1150 * 0.55
    fittings = 4 * 0.012 + 0.020 + 4 * 0.012 + 0.010
    bridge = 0.0045 * 1.0 * 0.0042 * 690 * 1.0
    strings = 0.068
    collar = math.pi * 0.016 ** 2 * 0.012 * 1150
    cello_kg = tp * 420 + bp * 650 + ribs + blocks + bar + post + neck + pegbox + fb + tail + fittings + bridge + strings + collar + 0.06
    endpin_kg = math.pi * 0.005 ** 2 * ENDPIN_LEN * 7850 * 1.0 + 0.010
    stopper_kg = math.pi * 0.0425 ** 2 * 0.018 * 850 * 0.92
    bow_kg = 0.0815
    out = {"case": case_kg, "case-lid": lid_kg, "case-wheels": wheels, "cello": cello_kg, "endpin": endpin_kg, "stopper": stopper_kg, "bow": bow_kg}
    breakdown = {"shell back": shell_kg(back_area), "shell lid": shell_kg(lid_area), "straps": straps, "handle": handle, "lining": lining, "top plate": tp * 420, "back plate": bp * 650, "ribs": ribs, "neck+pegbox": neck + pegbox, "fingerboard": fb, "tailpiece": tail}
    return {k: round(v, 4) for k, v in out.items()}, breakdown


MASS, MASS_BREAKDOWN = masses()
TOTAL_KG = round(sum(MASS.values()), 3)


def bone(i, parent, en, de, x, y, angle, length, joint, kg=None, com=(0.0, 0.0), z=None, abduction=None, section=None, ref="wood-handbook-2010"):
    """🦴 A rig bone."""
    out = {"id": i, "parent": parent, "title": T(en, de), "x": r6(x), "y": r6(y)}
    if z is not None:
        out["z"] = r6(z)
    out["angleDeg"] = angle
    if abduction is not None:
        out["abductionDeg"] = abduction
    out["lengthM"] = length
    out["joint"] = joint
    if kg is not None:
        out["mass"] = {"kg": kg, "comX": r6(com[0]), "comY": r6(com[1]), "distribution": "point", "source": "authored-estimate", "reference": ref}
    if section:
        out["section"] = section
    return out


def rig():
    """🦴 The rig: the case shell as root with lid, wheel axle and the carriers that take the cello and its stopper out of the case."""
    piv = P.bow_pivot_cello()
    return {"kind": "mechanical", "root": "case", "bones": [
        bone("case", None, "Back shell of the hard case standing on its bottom end", "Unterschale des Hartschalenkoffers, auf dem Fußende stehend", 0.0, 0.0, 90, CASE_TOP, {"type": "fixed"}, MASS["case"], (0.62, 0.07), ref="case-catalogue",
             section={"areaM2": 0.0098, "secondMomentM4": 2.1e-4, "extremeFibreM": 0.235, "material": "abs-eps", "title": T("Oval shell 470 x 290 mm, 8 mm ABS over EPS", "Ovale Schale 470 x 290 mm, 8 mm ABS über EPS")}),
        bone("case-lid", "case", "Lid on its hinge line along the right rim", "Deckel auf seiner Scharnierlinie am rechten Rand", LID_ORIGIN[1], -LID_ORIGIN[0], 0, 1.10, {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": -110, "twistMaxDeg": 0}, MASS["case-lid"], (0.42, -0.09), z=LID_ORIGIN[2], ref="case-catalogue"),
        bone("case-wheels", "case", "Wheel axle with two 64 mm wheels at the bottom of the back shell", "Radachse mit zwei 64-mm-Rädern unten an der Unterschale", P.WHEEL_C[1], -P.WHEEL_C[0], 0, 0.05, {"type": "revolute", "minDeg": -720, "maxDeg": 720}, MASS["case-wheels"], ref="case-catalogue"),
        bone("cello-carrier", "case", "Carrier that takes the cello out of the case to its playing place", "Träger, der das Cello aus dem Koffer an seinen Spielplatz bringt", CYC, -CXC, 0, 0.05, {"type": "prismatic", "axisDeg": round(axis_deg(LAY["carrier"]), 4), "minM": 0, "maxM": 1.25}),
        bone("cello", "cello-carrier", "Cello from the endpin collar to the scroll, leaning back when played", "Cello vom Stachelhalter bis zur Schnecke, beim Spielen zurückgelehnt", 0.0, 0.0, 0, SCROLL_TOP, {"type": "revolute", "minDeg": -5, "maxDeg": 45}, MASS["cello"], (0.40, 0.0),
             section={"areaM2": 0.00128, "secondMomentM4": 4.1e-6, "extremeFibreM": 0.085, "material": "maple-varnish", "title": T("Box of top, back and ribs at the waist", "Kasten aus Decke, Boden und Zargen an der Taille")}),
        bone("endpin", "cello", "Endpin, sliding out of the collar", "Stachel, aus dem Stachelhalter ausziehbar", ENDPIN_REST_TIP, 0.0, 0, ENDPIN_LEN, {"type": "prismatic", "axisDeg": 0, "minM": -0.45, "maxM": 0}, MASS["endpin"], (0.25, 0.0),
             section={"areaM2": 7.85e-5, "secondMomentM4": 4.9e-10, "extremeFibreM": 0.005, "material": "endpin-steel", "title": T("Steel rod 10 mm", "Stahlstab 10 mm")}, ref="incropera-2018"),
        bone("stopper-carrier", "case", "Carrier of the endpin stopper from its pocket to the floor", "Träger des Stachelhalters aus seiner Tasche auf den Boden", P.STOPPER_C[1], -P.STOPPER_C[0], 0, 0.05, {"type": "prismatic", "axisDeg": round(axis_deg(LAY["stopper"]), 4), "minM": 0, "maxM": 1.35}),
        bone("stopper", "stopper-carrier", "Endpin stopper disc", "Stachelhalterscheibe", 0.0, 0.0, 0, P.STOPPER_H, {"type": "fixed"}, MASS["stopper"], (0.009, 0.0)),
        bone("bow-carrier", "cello", "Carrier that brings the bow from the lid clips to the strings", "Träger, der den Bogen aus den Deckelklammern an die Saiten bringt", piv[0], piv[1], 0, 0.05, {"type": "prismatic", "axisDeg": round(axis_deg(LAY["bow"]), 4), "minM": 0, "maxM": 0.2}, z=piv[2]),
        bone("bow-angle", "bow-carrier", "Bow pivot at the contact point: turns the bow across the strings", "Bogendrehpunkt am Kontaktpunkt: dreht den Bogen quer zu den Saiten", 0.0, 0.0, 0, 0.05, {"type": "revolute", "minDeg": -20, "maxDeg": 20}, abduction=P.BOW_PHI),
        bone("bow", "bow-angle", "Bow (pernambuco stick, frog, hair), stroking along its own axis", "Bogen (Fernambukstange, Frosch, Bezug), entlang der eigenen Achse streichend", 0.0, 0.0, 0, P.BOW_TIP, {"type": "prismatic", "axisDeg": 0, "minM": -0.3, "maxM": 0.3}, MASS["bow"], (-0.17, -0.012),
             section={"areaM2": 4.0e-5, "secondMomentM4": 1.6e-10, "extremeFibreM": 0.0036, "material": "pernambuco", "title": T("Round stick 7.2 mm at mid-length", "Runde Stange 7,2 mm in der Mitte")}),
    ]}


def anchors():
    """📍 Floor contacts of the case, carrying grips, the playing contacts and the load points of strings, bow and fingers."""
    a = []
    add = lambda i, b, x, y, kind, en, de, **kw: a.append({"id": i, "bone": b, "x": r6(x), "y": r6(y), **({"z": r6(kw.pop("z"))} if "z" in kw else {}), "kind": kind, "title": T(en, de), **kw})
    add("wheel-left", "case-wheels", -P.WHEEL_R, 0.0, "contact", "Left wheel on the floor", "Linkes Rad auf dem Boden", z=P.WHEEL_Z, support="contact", footprintLengthM=0.012, footprintWidthM=0.024, frictionCoefficient=0.6)
    add("wheel-right", "case-wheels", -P.WHEEL_R, 0.0, "contact", "Right wheel on the floor", "Rechtes Rad auf dem Boden", z=-P.WHEEL_Z, support="contact", footprintLengthM=0.012, footprintWidthM=0.024, frictionCoefficient=0.6)
    add("foot-front", "case", 0.0, -0.095, "contact", "Front bumper on the floor", "Vorderer Puffer auf dem Boden", support="contact", footprintLengthM=0.030, footprintWidthM=0.140, frictionCoefficient=0.7)
    add("handle-side", "case", P.HANDLE_Y, -XP, "support", "Hand on the side handle", "Hand am Seitengriff", z=-case_b(P.HANDLE_Y) - 0.034, support="pin")
    for side, z in (("left", P.STRAP_Z), ("right", -P.STRAP_Z)):
        add(f"strap-top-{side}", "case", P.STRAP_Y1 - 0.02, -case_back_x(P.STRAP_Y1, z) + P.STRAP_T, "support", f"Shoulder strap {side}, upper attachment", f"Schultergurt {'links' if side == 'left' else 'rechts'}, obere Befestigung", z=z, support="pin")
        add(f"strap-bottom-{side}", "case", P.STRAP_Y0 + 0.02, -case_back_x(P.STRAP_Y0, z) + P.STRAP_T, "support", f"Shoulder strap {side}, lower end against the back", f"Schultergurt {'links' if side == 'left' else 'rechts'}, unteres Ende am Rücken", z=z, support="roller", normalDeg=0)
    add("stopper-floor", "stopper", 0.0, 0.0, "contact", "Rubber sole of the stopper on the floor", "Gummisohle des Stachelhalters auf dem Boden", support="contact", footprintLengthM=0.085, footprintWidthM=0.085, frictionCoefficient=0.9)
    s_knee = 0.300
    add("knee-left", "cello", BODY_U0 + s_knee, 0.0, "contact", "Player's left knee against the lower bout", "Linkes Knie des Spielers am Unterbug", z=B(s_knee), support="contact", tiltDeg=-90, footprintLengthM=0.08, footprintWidthM=0.06, frictionCoefficient=0.5)
    add("knee-right", "cello", BODY_U0 + s_knee, 0.0, "contact", "Player's right knee against the lower bout", "Rechtes Knie des Spielers am Unterbug", z=-B(s_knee), support="contact", tiltDeg=90, footprintLengthM=0.08, footprintWidthM=0.06, frictionCoefficient=0.5)
    add("chest", "cello", BODY_U0 + 0.700, back_v(0.700, 0.0), "contact", "Player's chest against the upper back", "Brust des Spielers am oberen Boden", support="contact", normalDeg=TILT, footprintLengthM=0.06, footprintWidthM=0.10, frictionCoefficient=0.5)
    add("neck-hand", "cello", 0.950, fb_top(0.950) + 0.050, "support", "Left hand round the neck while the endpin is set", "Linke Hand am Hals, während der Stachel eingestellt wird", support="pin")
    add("bow-hand", "bow", -0.390, -0.012, "support", "Right hand at the frog", "Rechte Hand am Frosch", support="fixed")
    add("string-nut", "cello", NUT_U, fb_top(NUT_U) - 0.002, "load", "Strings over the nut", "Saiten über dem Sattel")
    add("string-tail", "cello", 0.240, -0.151, "load", "Strings at the fine tuners", "Saiten an den Feinstimmern")
    add("bridge-top", "cello", BRIDGE_U, -0.174, "load", "Strings over the bridge", "Saiten über dem Steg")
    add("bow-contact", "cello", LAY["contact"][0], LAY["contact"][1], "load", "Bow hair on the D string", "Bogenhaar auf der D-Saite", z=-0.0075)
    add("finger-stop", "cello", 0.950, fb_top(0.950), "load", "Fingers stopping the strings", "Finger, die die Saiten greifen")
    return a


def string_forces():
    """🎶 The string forces on the cello in cello (u, v): pull at the nut and at the tailpiece, push of the bridge (self-equilibrated)."""
    pb, pn, pt_ = (BRIDGE_U, -0.174), (NUT_U, fb_top(NUT_U) - 0.002), (0.240, -0.151)
    unit = lambda a, b: ((b[0] - a[0]) / math.hypot(b[0] - a[0], b[1] - a[1]), (b[1] - a[1]) / math.hypot(b[0] - a[0], b[1] - a[1]))
    fn = tuple(STRING_TENSION * c for c in unit(pn, pb))
    ft = tuple(STRING_TENSION * c for c in unit(pt_, pb))
    fb = (-(fn[0] + ft[0]), -(fn[1] + ft[1]))
    return {"string-nut": fn, "string-tail": ft, "bridge-top": fb}


def world(f, cello_deg):
    """🌍 A cello-frame force (fu, fv) in world newtons for a cello pointing at `cello_deg`."""
    u, v = rot(cello_deg)
    return (f[0] * u[0] + f[1] * v[0], f[0] * u[1] + f[1] * v[1])


def loads(cello_deg, playing):
    """🧲 The loads of a pose: string prestress always, bow pressure and finger force when played."""
    out = []
    titles = {"string-nut": T("String tension at the nut", "Saitenzug am Sattel"), "string-tail": T("String tension at the tailpiece", "Saitenzug am Saitenhalter"), "bridge-top": T("Bridge down-bearing on the top", "Stegdruck auf die Decke")}
    for anchor, f in string_forces().items():
        fx, fy = world(f, cello_deg)
        out.append({"id": anchor.replace("string-", "strings-") if anchor != "bridge-top" else "bridge-down-bearing", "kind": "imposed", "title": titles[anchor], "anchor": anchor, "fxN": round(fx, 2), "fyN": round(fy, 2), "source": "literature", "reference": "string-tension-chart"})
    if playing:
        fx, fy = world((0.0, 1.5), cello_deg)
        out.append({"id": "bow-pressure", "kind": "live", "title": T("Bow pressure on the string", "Bogendruck auf die Saite"), "anchor": "bow-contact", "fxN": round(fx, 3), "fyN": round(fy, 3), "source": "literature", "reference": "askenfelt-1989"})
        out.append({"id": "bow-reaction", "kind": "live", "title": T("String reaction on the bow hair", "Saitenreaktion auf das Bogenhaar"), "bone": "bow", "x": -0.0955, "y": 0.001, "fxN": round(-fx, 3), "fyN": round(-fy, 3), "source": "literature", "reference": "askenfelt-1989"})
        fx, fy = world((0.0, 6.0), cello_deg)
        out.append({"id": "finger-force", "kind": "live", "title": T("Fingers pressing the strings onto the fingerboard", "Finger drücken die Saiten auf das Griffbrett"), "anchor": "finger-stop", "fxN": round(fx, 2), "fyN": round(fy, 2), "source": "authored-estimate", "reference": "fletcher-rossing-1998"})
    return out


REGIONS = ["case-shell", "case-straps", "case-handle", "case-hardware", "case-wheels", "case-feet", "cello-corpus", "cello-neck", "cello-fingerboard", "cello-fittings", "cello-bridge", "cello-strings", "endpin", "stopper", "bow-stick", "bow-frog", "bow-hair"]
HIDDEN = set(REGIONS[6:])


def binding(pose):
    """🌡️ The thermal binding of a pose: every region with its exposure and contacts."""
    regions = {}
    for r in REGIONS:
        inside = r in HIDDEN and pose in ("stored", "carried", "carry-lift")
        b = {"exposedFraction": 0.0 if inside else 1.0, "sunlitFraction": 0.0}
        regions[r] = b
    if pose in ("stored", "played"):
        regions["case-wheels"]["contact"] = {"fraction": 0.04, "tempC": 19.0}
        regions["case-feet"]["contact"] = {"fraction": 0.3, "tempC": 19.0}
    if pose == "carried":
        regions["case-straps"]["contact"] = {"fraction": 0.45, "tempC": 31.0}
    if pose == "carry-lift":
        regions["case-handle"]["contact"] = {"fraction": 0.5, "tempC": 33.0}
    if pose == "played":
        regions["cello-corpus"]["contact"] = {"fraction": 0.05, "tempC": 32.0}
        regions["cello-neck"]["contact"] = {"fraction": 0.12, "tempC": 33.0}
        regions["cello-fingerboard"]["contact"] = {"fraction": 0.04, "tempC": 33.0}
        regions["bow-frog"]["contact"] = {"fraction": 0.35, "tempC": 33.0}
        regions["bow-stick"]["contact"] = {"fraction": 0.05, "tempC": 33.0}
        regions["stopper"]["contact"] = {"fraction": 0.4, "tempC": 19.0}
        regions["endpin"]["exposedFraction"] = 1.0
    return {"regions": regions, "emitters": {}}


def poses():
    """🧍 Stored (default), carried on the back, played seated, and the key pose of lifting by the side handle."""
    lay = LAY
    played_bones = {"cello-carrier": along(lay["carrier"]), "cello": {"angleDeg": TILT}, "endpin": {"dx": ENDPIN_OUT},
                    "stopper-carrier": along(lay["stopper"]), "bow-carrier": along(lay["bow"]),
                    "bow-angle": {"abductionDeg": 90 - P.BOW_PHI}}
    return [
        {"id": "stored", "title": T("Stored in its case", "Im Koffer gelagert"), "description": T("The closed hard case stands upright in the dealer's depot on its two wheels and the front bumper; inside the cello lies in its padding with the endpin pushed in, the stopper in the endpin pocket and the bow in the lid clips.", "Der geschlossene Hartschalenkoffer steht aufrecht im Lager des Händlers auf seinen zwei Rädern und dem vorderen Puffer; darin liegt das Cello in seiner Polsterung mit eingeschobenem Stachel, der Stachelhalter in der Stacheltasche und der Bogen in den Deckelklammern."),
         "bones": {}, "ground": "auto", "contacts": ["wheel-left", "wheel-right", "foot-front"], "thermal": binding("stored"), "loads": loads(90, False)},
        {"id": "carried", "title": T("Carried on the back", "Auf dem Rücken getragen"), "description": T("The case hangs on both shoulder straps of a walking adult (towards -x): the upper strap ends carry it at shoulder height, the lower ends lean against the lower back; the top leans 5 degrees away from the back.", "Der Koffer hängt an beiden Schultergurten eines gehenden Erwachsenen (Richtung -x): die oberen Gurtenden tragen ihn auf Schulterhöhe, die unteren lehnen am unteren Rücken; das Kopfende neigt sich 5 Grad vom Rücken weg."),
         "bones": {}, "root": {"x": 0.0, "y": 0.40, "angleDeg": -5}, "ground": "authored", "contacts": ["strap-top-left", "strap-top-right", "strap-bottom-left", "strap-bottom-right"], "thermal": binding("carried"), "loads": loads(85, False)},
        {"id": "played", "title": T("Played seated", "Im Sitzen gespielt"), "description": T("Taken out and played seated: the endpin is pulled out 300 mm and stands in the stopper on the floor, the cello leans back 32 degrees between the knees against the chest, the bow crosses the D and G strings 60 mm above the bridge; the closed case stays standing where it was stored.", "Herausgenommen und im Sitzen gespielt: der Stachel ist 300 mm ausgezogen und steht im Stachelhalter auf dem Boden, das Cello lehnt 32 Grad zurück zwischen den Knien an der Brust, der Bogen streicht D- und G-Saite 60 mm über dem Steg; der geschlossene Koffer bleibt stehen, wo er gelagert war."),
         "bones": played_bones, "ground": "auto", "contacts": ["wheel-left", "wheel-right", "foot-front", "stopper-floor", "knee-left", "knee-right", "chest", "bow-hand"], "thermal": binding("played"), "loads": loads(90 + TILT, True)},
        {"id": "carry-lift", "title": T("Lifted by the side handle", "Am Seitengriff angehoben"), "description": T("Key pose of the carry clip: the case is lifted 180 mm by its side handle before it is swung onto the back.", "Schlüsselpose des Trage-Clips: der Koffer ist am Seitengriff 180 mm angehoben, bevor er auf den Rücken geschwungen wird."), "key": True,
         "bones": {}, "root": {"x": 0.0, "y": 0.18, "angleDeg": -2}, "ground": "authored", "contacts": ["handle-side"], "thermal": binding("carry-lift"), "loads": loads(88, False)},
    ]


def clips():
    """🎞️ Carry (stored to carried), endpin extension while seated, bowing loop."""
    played_contacts = ["wheel-left", "wheel-right", "foot-front", "stopper-floor", "knee-left", "knee-right", "chest", "bow-hand"]
    span = lambda ids, a, b: [{"anchor": i, "fromS": a, "toS": b} for i in ids]
    return [
        {"id": "carry", "title": T("Lift onto the back", "Auf den Rücken heben"), "description": T("The cellist grips the side handle, lifts the case off its wheels and swings it onto the back, where the straps take it.", "Die Cellistin fasst den Seitengriff, hebt den Koffer von den Rädern und schwingt ihn auf den Rücken, wo die Gurte ihn übernehmen."),
         "durationS": 2.4, "fps": 25, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1.0, "pose": "carry-lift", "ease": "smooth"}, {"t": 2.4, "pose": "carried", "ease": "smooth"}],
         "contacts": span(["wheel-left", "wheel-right", "foot-front"], 0, 0.25) + span(["handle-side"], 0.15, 1.9) + span(["strap-top-left", "strap-top-right", "strap-bottom-left", "strap-bottom-right"], 1.6, 2.4),
         "phases": [{"id": "lift", "fromS": 0, "toS": 1.0, "title": T("Lifting by the handle", "Anheben am Griff")}, {"id": "shoulder", "fromS": 1.0, "toS": 2.4, "title": T("Onto the shoulders", "Auf die Schultern")}],
         "events": [{"t": 0.25, "kind": "contact-off", "title": T("Wheels leave the floor", "Räder verlassen den Boden"), "target": "wheel-left"}, {"t": 1.6, "kind": "contact-on", "title": T("Straps take the load", "Gurte übernehmen die Last"), "target": "strap-top-left"}],
         "tracks": []},
        {"id": "endpin-extend", "title": T("Pull out the endpin", "Stachel ausziehen"), "description": T("Seated with the cello held between the knees and by the neck, the endpin is pulled out 300 mm until its point sits in the stopper cup.", "Sitzend, das Cello zwischen den Knien und am Hals gehalten, wird der Stachel 300 mm ausgezogen, bis seine Spitze in der Mulde des Stachelhalters sitzt."),
         "durationS": 2.5, "fps": 25, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "played", "ease": "smooth", "bones": {"endpin": {"dx": 0.0}}}, {"t": 2.0, "pose": "played", "ease": "smooth"}, {"t": 2.5, "pose": "played", "ease": "step"}],
         "contacts": span(played_contacts, 0, 2.5) + span(["neck-hand"], 0, 2.1),
         "phases": [{"id": "pull", "fromS": 0, "toS": 2.0, "title": T("Loosen the screw and pull", "Schraube lösen und ziehen")}, {"id": "set", "fromS": 2.0, "toS": 2.5, "title": T("Point in the stopper", "Spitze im Stachelhalter")}],
         "events": [{"t": 2.0, "kind": "contact-on", "title": T("Endpin point in the stopper cup", "Stachelspitze in der Mulde"), "target": "stopper-floor"}],
         "tracks": []},
        {"id": "bowing", "title": T("Long bows on D and G", "Lange Bogenstriche auf D und G"), "description": T("A down-bow and an up-bow of 440 mm each at 60 mm above the bridge, the hair on the D and G strings.", "Ein Abstrich und ein Aufstrich von je 440 mm 60 mm über dem Steg, das Haar auf D- und G-Saite."),
         "durationS": 4.0, "fps": 25, "loop": True, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "played", "ease": "smooth", "bones": {"bow": {"dx": 0.22}}}, {"t": 2.0, "pose": "played", "ease": "smooth", "bones": {"bow": {"dx": -0.22}}}, {"t": 4.0, "pose": "played", "ease": "smooth", "bones": {"bow": {"dx": 0.22}}}],
         "contacts": span(played_contacts, 0, 4.0),
         "phases": [{"id": "down-bow", "fromS": 0, "toS": 2.0, "title": T("Down-bow", "Abstrich")}, {"id": "up-bow", "fromS": 2.0, "toS": 4.0, "title": T("Up-bow", "Aufstrich")}],
         "events": [{"t": 0.0, "kind": "motion", "title": T("Down-bow starts at the frog", "Abstrich beginnt am Frosch")}, {"t": 2.0, "kind": "motion", "title": T("Bow change at the tip", "Bogenwechsel an der Spitze")}],
         "tracks": []},
    ]


def solid_area(s):
    """📐 The surface area of a solid of this asset, analytically or from its mesh."""
    k = s["kind"]
    if k == "polyhedron":
        return mesh_area(s["vertices"], s["triangles"])
    if k == "box":
        a, b, c = s["size"]
        return 2 * (a * b + b * c + c * a)
    if k == "capsule":
        L = math.dist(s["p0"], s["p1"])
        r0, r1 = s["r0"], s["r1"]
        return math.pi * (r0 + r1) * math.hypot(L, r0 - r1) + 2 * math.pi * (r0 * r0 + r1 * r1)
    if k == "ellipsoid":
        a, b, c = s["radii"]
        p = 1.6075
        return 4 * math.pi * (((a * b) ** p + (a * c) ** p + (b * c) ** p) / 3) ** (1 / p)
    if k == "loft":
        secs = s["sections"]
        area = 0.0
        for x, y in zip(secs, secs[1:]):
            per = lambda q: math.pi * (3 * (q["rx"] + q["ry"]) - math.sqrt((3 * q["rx"] + q["ry"]) * (q["rx"] + 3 * q["ry"])))
            L = math.dist(x["at"], y["at"])
            area += (per(x) + per(y)) / 2 * L
        ell = lambda q: math.pi * q["rx"] * q["ry"]
        return area + ell(secs[0]) + ell(secs[-1])
    if k == "revolve":
        prof = s["profile"]
        area = 0.0
        for a, b in zip(prof, prof[1:]):
            area += math.pi * (a[1] + b[1]) * math.dist(a, b)
        return area
    if k == "extrude":
        prof = s["profile"]
        h = s["to"] - s["from"]
        per = sum(math.dist(prof[i], prof[(i + 1) % len(prof)]) for i in range(len(prof)))
        ar = abs(sum(prof[i][0] * prof[(i + 1) % len(prof)][1] - prof[(i + 1) % len(prof)][0] * prof[i][1] for i in range(len(prof))) / 2)
        for hole in s.get("holes", []):
            per += sum(math.dist(hole[i], hole[(i + 1) % len(hole)]) for i in range(len(hole)))
            ar -= abs(sum(hole[i][0] * hole[(i + 1) % len(hole)][1] - hole[(i + 1) % len(hole)][0] * hole[i][1] for i in range(len(hole))) / 2)
        return 2 * ar + per * h
    raise ValueError(k)


def region_areas(lists50):
    """📐 The 1:50 areas of every region: the outer skin of the case regions, the full mesh area of the hidden instrument regions."""
    body, thermal = lists50
    by_id = {s["id"]: s for s in body}
    areas = {r: 0.0 for r in REGIONS}
    back_area, lid_area = case_outer_areas(24)
    areas["case-shell"] = back_area + lid_area
    for s in thermal:
        geo = by_id[s["of"]] if "of" in s else s
        if s["region"] == "case-shell":
            if s["id"].startswith("th-wheel-housing"):
                a, b, c = geo["size"]
                areas["case-shell"] += a * b + 2 * b * c + 2 * a * c - 0.0
            continue
        areas[s["region"]] += solid_area(geo)
    areas.update(MEASURED_1_50)
    return areas


MEASURED_1_50 = {"case-straps": 0.128, "case-handle": 0.017, "case-hardware": 0.019, "case-wheels": 0.009, "case-feet": 0.007}


REGION_META = {
    "case-shell": ("Case shell (ABS over EPS)", "Kofferschale (ABS über EPS)", "surface", "abs-eps", 0.47, "vertical-plate", 0.5),
    "case-straps": ("Shoulder straps", "Schultergurte", "surface", "webbing", 0.05, "vertical-plate", 0.4),
    "case-handle": ("Side handle", "Seitengriff", "surface", "rubber-grip", 0.025, "cylinder", 0.4),
    "case-hardware": ("Latches, hinges and axle", "Verschlüsse, Scharniere und Achse", "surface", "nickel-steel", 0.04, "vertical-plate", 0.4),
    "case-wheels": ("Wheels", "Räder", "tyre", "pu-wheel", 0.064, "cylinder", 0.2),
    "case-feet": ("Front bumper", "Vorderer Puffer", "surface", "rubber-bumper", 0.03, "horizontal-plate-down", 0.0),
    "cello-corpus": ("Cello body: spruce top, maple back and ribs, aged varnish", "Cellokorpus: Fichtendecke, Ahornboden und Zargen, gealterter Lack", "surface", "spruce-varnish", 0.44, "vertical-plate", 0.5),
    "cello-neck": ("Neck, pegbox and scroll", "Hals, Wirbelkasten und Schnecke", "surface", "maple-varnish", 0.04, "cylinder", 0.5),
    "cello-fingerboard": ("Fingerboard, finger-worn", "Griffbrett, abgespielt", "surface", "ebony", 0.033, "vertical-plate", 0.5),
    "cello-fittings": ("Tailpiece, pegs, nut, saddle, collar and fine tuners", "Saitenhalter, Wirbel, Sattel, Untersattel, Stachelhalter und Feinstimmer", "surface", "ebony", 0.05, "vertical-plate", 0.5),
    "cello-bridge": ("Bridge", "Steg", "surface", "maple-bridge", 0.09, "vertical-plate", 0.5),
    "cello-strings": ("Strings", "Saiten", "surface", "steel-string", 0.0015, "cylinder", 0.5),
    "endpin": ("Endpin", "Stachel", "surface", "endpin-steel", 0.01, "cylinder", 0.5),
    "stopper": ("Endpin stopper", "Stachelhalter", "surface", "rosewood", 0.085, "horizontal-plate-up", 0.6),
    "bow-stick": ("Bow stick and head", "Bogenstange und Spitze", "surface", "pernambuco", 0.008, "cylinder", 0.5),
    "bow-frog": ("Bow frog and button", "Frosch und Schraube", "surface", "ebony", 0.025, "vertical-plate", 0.5),
    "bow-hair": ("Bow hair", "Bogenbezug", "surface", "horsehair", 0.012, "vertical-plate", 0.5),
}


def thermal_model(areas):
    """🌡️ The thermal model: inert passive surfaces in an indoor depot; the instrument is hidden in the closed case when stored."""
    regions = []
    for r in REGIONS:
        en, de, kind, mat, L, geo, sky = REGION_META[r]
        reg = {"id": r, "title": T(en, de), "kind": kind, "model": "passive", "material": mat, "areaM2": round(areas[r], 4), "characteristicLengthM": L, "geometry": geo, "skyViewFactor": sky, "layers": [], "heatCapacityJM2K": 3000, "source": "authored-estimate", "reference": "incropera-2018"}
        if r == "case-shell":
            reg["layers"] = [{"id": "abs-eps-wall", "title": T("2 mm ABS skin over 6 mm EPS foam", "2 mm ABS-Haut über 6 mm EPS-Schaum"), "kind": "shell", "material": "abs-eps", "thicknessM": 0.008, "resistanceM2KW": 0.183, "heatCapacityJM2K": 3174, "source": "authored-estimate", "reference": "iso-10456-2007"}]
            reg["heatCapacityJM2K"] = 3174
            reg["ratedPowerW"] = 0
            reg["ratedPowerBasis"] = "passive"
        if r in HIDDEN:
            reg["hidden"] = T("Inside the closed case in the stored default pose; exposed only when the cello is taken out and played.", "Im geschlossenen Koffer in der gelagerten Standardpose; nur freiliegend, wenn das Cello herausgenommen und gespielt wird.")
        regions.append(reg)
    return {"environment": {"airTempC": 20, "meanRadiantTempC": 19.5, "relativeHumidity": 0.5, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325},
            "passengers": [], "operating": [], "regions": regions, "emitters": [],
            "assumptions": [T("Every part is an inert passive surface: radiation and convection in an indoor depot at 20 degC and 50 % relative humidity (the humidity a string dealer keeps); no internal heat and no electrical power (passive, 0 W).", "Jedes Teil ist eine inerte passive Fläche: Strahlung und Konvektion in einem Innenlager bei 20 °C und 50 % relativer Feuchte (die Feuchte, die ein Streichinstrumentenhändler hält); keine innere Wärme und keine elektrische Leistung (passiv, 0 W)."),
                            T("In the stored and carried poses the instrument, bow and stopper are enclosed by the 8 mm ABS-EPS shell (R about 0.18 m2K/W) and do not exchange heat with the room directly; when played, the knees, chest and hands warm the contact patches towards 32-33 degC.", "In den Posen gelagert und getragen sind Instrument, Bogen und Stachelhalter von der 8-mm-ABS-EPS-Schale (R etwa 0,18 m2K/W) umschlossen und tauschen keine Wärme direkt mit dem Raum; beim Spielen erwärmen Knie, Brust und Hände die Kontaktflächen auf 32-33 °C."),
                            T("Aged surfaces: the crazed oil varnish and the rubbed-through bare spruce at the upper bouts, the scraped rib and the chalky, scuffed ABS have emissivities of 0.88-0.9 (authored estimates after Incropera 2018); tarnished nickel plating 0.25 instead of 0.1 for bright nickel.", "Gealterte Oberflächen: der krakelierte Öllack und die durchgeriebene blanke Fichte an den oberen Backen, die abgeschabte Zarge und das kreidige, verschrammte ABS haben Emissionsgrade von 0,88-0,9 (Schätzwerte nach Incropera 2018); angelaufene Vernickelung 0,25 statt 0,1 für blankes Nickel.")]}


def structure_model():
    """🏗️ The structural model: supports come from the contacts of each pose (floor, straps, handle, the player's knees, chest and hands)."""
    return {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": [], "clinical": False, "assumptions": [
        T("Rigid body on elastic supports per pose: stored on two wheels and the front bumper; carried on the two upper strap ends (pins) with the lower ends against the back (rollers, horizontal); lifted by the side handle; played on the stopper sole, both knees (lateral contacts), the chest (normal at 32 degrees) and the right hand at the frog (fixed), with the closed case standing on its own floor contacts.", "Starrer Körper auf elastischen Auflagern je Pose: gelagert auf zwei Rädern und dem vorderen Puffer; getragen an den zwei oberen Gurtenden (Gelenke) mit den unteren Enden am Rücken (Rollen, horizontal); am Seitengriff angehoben; gespielt auf der Sohle des Stachelhalters, beiden Knien (seitliche Kontakte), der Brust (Normale unter 32 Grad) und der rechten Hand am Frosch (eingespannt), der geschlossene Koffer auf seinen eigenen Bodenkontakten."),
        T(f"String tension {STRING_TENSION:.0f} N in total (four strings, catalogue-typical medium tension) pulls the nut and the tailpiece towards the bridge; the bridge bears {string_forces()['bridge-top'][1]:.0f} N into the top. The three loads are self-equilibrated and act in every pose, the instrument is always strung.", f"Der Saitenzug von insgesamt {STRING_TENSION:.0f} N (vier Saiten, katalogtypische mittlere Spannung) zieht Sattel und Saitenhalter zum Steg; der Steg drückt mit {string_forces()['bridge-top'][1]:.0f} N auf die Decke. Die drei Lasten sind im Gleichgewicht und wirken in jeder Pose, das Instrument ist stets besaitet."),
        T("Bow pressure 1.5 N on the strings (with its reaction on the hair) and 6 N of the stopping fingers when played (Askenfelt 1989; Fletcher and Rossing 1998). The player is not part of the asset: knees, chest and hands are supports, the case carrier is the pair of straps.", "Bogendruck 1,5 N auf die Saiten (mit seiner Reaktion auf das Haar) und 6 N der greifenden Finger beim Spielen (Askenfelt 1989; Fletcher und Rossing 1998). Die spielende Person ist nicht Teil des Assets: Knie, Brust und Hände sind Auflager, die tragende Person ist das Gurtpaar."),
        T(f"Masses from sections and densities: case {MASS['case']:.2f} kg (shell, straps, handle, latches, lining), lid {MASS['case-lid']:.2f} kg, wheels {MASS['case-wheels']:.2f} kg, cello {MASS['cello']:.2f} kg (4 mm spruce top, 4 mm maple back, ribs, blocks, bass bar, sound post, neck, ebony fingerboard and fittings), endpin {MASS['endpin']:.2f} kg, stopper {MASS['stopper']:.2f} kg, bow {MASS['bow']:.3f} kg; together {TOTAL_KG:.2f} kg.", f"Massen aus Querschnitten und Dichten: Koffer {MASS['case']:.2f} kg (Schale, Gurte, Griff, Verschlüsse, Polster), Deckel {MASS['case-lid']:.2f} kg, Räder {MASS['case-wheels']:.2f} kg, Cello {MASS['cello']:.2f} kg (4 mm Fichtendecke, 4 mm Ahornboden, Zargen, Klötze, Bassbalken, Stimmstock, Hals, Ebenholzgriffbrett und Garnitur), Stachel {MASS['endpin']:.2f} kg, Stachelhalter {MASS['stopper']:.2f} kg, Bogen {MASS['bow']:.3f} kg; zusammen {TOTAL_KG:.2f} kg.")]}


def references():
    """📚 Organology, standards and catalogue-typical data used for the numbers."""
    R = lambda i, c, url=None: {"id": i, "citation": c, **({"url": url} if url else {})}
    return [
        R("hornbostel-sachs-1914", "von Hornbostel, E. M., Sachs, C. (1914). Systematik der Musikinstrumente. Zeitschrift für Ethnologie 46, 553-590: the violoncello is 321.322-71 (composite chordophone, box lute with neck, sounded by a bow)."),
        R("strad-davidov", "The Strad (poster and measurements), Antonio Stradivari violoncello 'Davidov' 1712: body length 755 mm, upper bout 342 mm, middle bout 232 mm, lower bout 439 mm, rib height about 120 mm; the 4/4 pattern of modern workshop and factory cellos."),
        R("cello-setup-standards", "Catalogue-typical 4/4 cello setup data of violin-making workshops: vibrating string length 690-695 mm, stop 400 mm, neck 280 mm, fingerboard 580 mm (24 mm at the nut, 42 mm at the end), bridge 90 mm high and 92 mm wide, projection about 80 mm at the bridge, total length 1230-1250 mm without the endpin, mass 2.6-3.2 kg."),
        R("string-tension-chart", "Catalogue-typical tension charts of medium-tension 4/4 cello string sets (synthetic or steel core): A about 190 N, D 160 N, G 145 N, C 155 N, together about 650 N."),
        R("bow-dimensions", "Catalogue-typical 4/4 cello bow data: length 715-735 mm, mass 78-86 g, hair ribbon about 12 mm wide and 650 mm long, frog about 47 x 25 mm."),
        R("case-catalogue", "Retailer catalogue data of thermoformed ABS-over-EPS 4/4 cello hard cases with wheels and backpack straps (2023-2025): external 1350-1370 x 450-470 x 290-310 mm, 4-5 kg, two wheels of about 60 mm at the bottom, three latches, side handle at the balance point."),
        R("bunting-1982", "Bunting, C. (1982). Essay on the Craft of 'Cello-Playing, Vol. 1. Cambridge University Press: seated posture, endpin length, inclination of the instrument and contact with the knees and chest."),
        R("askenfelt-1989", "Askenfelt, A. (1989). Measurement of the bowing parameters in violin playing II: bow-bridge distance, dynamic range, and limits of bow force. Journal of the Acoustical Society of America 86(2), 503-516."),
        R("fletcher-rossing-1998", "Fletcher, N. H., Rossing, T. D. (1998). The Physics of Musical Instruments, 2nd ed. Springer, chapter 10 (bowed string instruments: bridge, bass bar, sound post, string forces)."),
        R("wood-handbook-2010", "Forest Products Laboratory (2010). Wood Handbook - Wood as an Engineering Material, FPL-GTR-190: densities and elastic moduli of spruce, maple, ebony and tropical hardwoods."),
        R("iso-10456-2007", "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (plastics, foams, rubber, wood)."),
        R("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed. Wiley (emissivities of varnished wood, plastics and metals)."),
    ]


PENS = {50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": None, "ground": 0.7}, 100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": None, "ground": 0.5},
        200: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": None, "ground": 0.35}, 500: {"body": 0.18, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.25}}
BANDS = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}


def rep_decisions(scale):
    """📝 The design decisions of the drawing at one scale."""
    paper = lambda m: f"{m * 1000 / scale:.1f}".replace(".", ",")
    h, w, d = CASE_TOP, 0.30, 0.49
    if scale == 50:
        return {
            "silhouette": T(f"Side elevation {h * 1000 / scale:.0f} mm high: the oval case on edge with the deep back shell and the shallower lid meeting at the parting line, the two padded straps standing proud of the back, the wheel in its housing, the front bumper and on the near side the handle grip with its mounts and two hinges. Front view: the cello-shaped lid face with its crown line, three latches on the left rim, handle and hinges on the right, the wheels showing under the bottom, two old stickers and scuffed corners. Plan: the lower-bout section of shell and lid with the narrower neck and head sections inside, straps at the back, handle and latches at the rims.", f"Seitenansicht {h * 1000 / scale:.0f} mm hoch: der ovale Koffer hochkant mit der tiefen Unterschale und dem flacheren Deckel an der Trennfuge, die zwei gepolsterten Gurte vor dem Rücken, das Rad im Radkasten, der vordere Puffer und auf der nahen Seite der Griff mit seinen Haltern und zwei Scharnieren. Vorderansicht: die cellöförmige Deckelfläche mit ihrer Wölbungslinie, drei Verschlüsse am linken Rand, Griff und Scharniere rechts, die Räder unter dem Boden, zwei alte Aufkleber und verschrammte Ecken. Draufsicht: der Unterbugquerschnitt von Schale und Deckel mit dem schmaleren Hals- und Kopfquerschnitt darin, Gurte hinten, Griff und Verschlüsse an den Rändern."),
            "retained": T("Every part of case and instrument at true size: shell, lid, straps, handle, latches, hinges, wheels, bumper; inside (drawn for the played pose) the cello body with arched top and back, neck, fingerboard, pegbox, scroll, four pegs, tailpiece with fine tuners, collar, bridge, the four strings, the endpin with its point, the stopper and the bow with stick, hair, frog and head.", "Jedes Teil von Koffer und Instrument in wahrer Größe: Schale, Deckel, Gurte, Griff, Verschlüsse, Scharniere, Räder, Puffer; innen (für die Spielpose gezeichnet) der Cellokorpus mit gewölbter Decke und Boden, Hals, Griffbrett, Wirbelkasten, Schnecke, vier Wirbel, Saitenhalter mit Feinstimmern, Stachelhalter, Steg, die vier Saiten, der Stachel mit Spitze, der Stachelhalter und der Bogen mit Stange, Haar, Frosch und Spitze."),
            "merged": T("Latch hasps, hinge pins and the strap buckles are merged into their plates; the far strap and the far wheel lie behind their near twins in the side elevation.", "Verschlussbügel, Scharnierstifte und Gurtschnallen sind in ihre Platten gezogen; der ferne Gurt und das ferne Rad liegen in der Seitenansicht hinter ihren nahen Zwillingen."),
            "omitted": T("The velour lining, the accessory pocket and the pegbox opening are not drawn; the inner contour of the shell is hidden by the closed case.", "Das Velourfutter, die Zubehörtasche und die Öffnung des Wirbelkastens sind nicht gezeichnet; die Innenkontur der Schale ist vom geschlossenen Koffer verdeckt."),
            "thinElements": T("Strings, the bow hair and the scuffs are single strokes at the texture pen (0.13 mm = 6.5 mm); the parting line, crown line and rib edges at the detail pen.", "Saiten, Bogenhaar und Schrammen sind Einzelstriche mit dem Texturstift (0,13 mm = 6,5 mm); Trennfuge, Wölbungslinie und Zargenkanten mit dem Detailstift."),
            "hatching": T("No hatches: the shell reads by its dark tone, the varnished wood by mid tones, ebony and rubber near black.", "Keine Schraffuren: die Schale liest sich durch ihren dunklen Ton, das lackierte Holz durch Mitteltöne, Ebenholz und Gummi fast schwarz."),
            "styles": T("Body pen 0.5 mm for the case outline, 0.35 mm for the instrument parts, detail 0.25 mm, texture 0.13 mm; tones from 0.06 (ebony frog) to 0.9 (paper label).", "Körperstift 0,5 mm für den Koffer, 0,35 mm für die Instrumententeile, Detail 0,25 mm, Textur 0,13 mm; Töne von 0,06 (Ebenholzfrosch) bis 0,9 (Papieretikett)."),
            "overlays": T("Traces of use: whitened scuffs at the bottom and head corners, a round and a rectangular old sticker on the lid, faded straps; on the instrument four scratches on the right lower rib, the varnish rubbed through at the left upper bout and rosin dust on the top under the strings.", "Gebrauchsspuren: weiß verschrammte Ecken am Fuß- und Kopfende, ein runder und ein rechteckiger alter Aufkleber auf dem Deckel, ausgebleichte Gurte; am Instrument vier Kratzer an der rechten unteren Zarge, der Lack an der linken oberen Backe durchgerieben und Kolophoniumstaub auf der Decke unter den Saiten."),
        }
    drop = {100: ("hinges, the hub circle, stickers, scuffs, fine tuners, nut, saddle, f-holes, rib scratches and the bow head", "Scharniere, Nabenkreis, Aufkleber, Schrammen, Feinstimmer, Sattel, Untersattel, F-Löcher, Zargenkratzer und Bogenspitze"),
            200: ("straps, handle, latches, scroll and pegs as parts, tailpiece, afterlengths, collar, stopper and frog", "Gurte, Griff, Verschlüsse, Schnecke und Wirbel als Einzelteile, Saitenhalter, Nachlängen, Stachelhalter, Stachelscheibe und Frosch"),
            500: ("wheels (merged into the foot block), bridge, strings, fingerboard overhang, pegbox and the bow hair", "Räder (in den Fußblock gezogen), Steg, Saiten, Griffbrettüberstand, Wirbelkasten und Bogenhaar")}[scale]
    return {
        "silhouette": T(f"Side elevation {h * 1000 / scale:.1f} mm high and {0.30 * 1000 / scale:.1f} mm deep: the oval case on edge, back shell and lid; front {d * 1000 / scale:.1f} mm wide: the cello-shaped lid face; plan: the lower-bout section.", f"Seitenansicht {h * 1000 / scale:.1f} mm hoch und {0.30 * 1000 / scale:.1f} mm tief: der ovale Koffer hochkant, Unterschale und Deckel; Vorderansicht {d * 1000 / scale:.1f} mm breit: die cellöförmige Deckelfläche; Draufsicht: der Unterbugquerschnitt."),
        "retained": T("Shell and lid with their true outlines at fewer stations, the wheels or foot, the bumper, the instrument body with neck and fingerboard, the endpin and the bow stick.", "Schale und Deckel mit ihren wahren Umrissen an weniger Stationen, Räder oder Fuß, Puffer, der Instrumentenkorpus mit Hals und Griffbrett, der Stachel und die Bogenstange."),
        "merged": T(f"Merged or left out at this scale: {drop[0]}.", f"Bei diesem Maßstab zusammengezogen oder weggelassen: {drop[1]}."),
        "omitted": T("Lining and interior pads are not drawn; the cello parts are drawn for the played pose and hidden by the case when stored.", "Futter und Innenpolster sind nicht gezeichnet; die Celloteile sind für die Spielpose gezeichnet und im gelagerten Zustand vom Koffer verdeckt."),
        "thinElements": T("Strings, endpin, bow stick and hair are single strokes at the detail pen where they are drawn; nothing is thickened beyond its pen.", "Saiten, Stachel, Bogenstange und Haar sind Einzelstriche mit dem Detailstift, wo sie gezeichnet sind; nichts ist über seinen Stift hinaus verdickt."),
        "hatching": T("No hatches, tones only.", "Keine Schraffuren, nur Töne."),
        "styles": T(f"Body pen {PENS[scale]['body']} mm, detail {PENS[scale]['detail']} mm.", f"Körperstift {PENS[scale]['body']} mm, Detail {PENS[scale]['detail']} mm."),
        "overlays": T("The traces of use are below the printable feature at this scale and live in the 1:50 drawing and model.", "Die Gebrauchsspuren liegen bei diesem Maßstab unter der druckbaren Einzelheit und leben in Zeichnung und Modell 1:50."),
    }


def representation(scale):
    """🖨️ The authored representation at one scale."""
    ce, cf, cp = D.case_drawing(scale)
    ve, vf, vp = D.cello_drawing(scale)
    be, bf, bp = D.bow_drawing(scale)
    lo, hi = BANDS[scale]
    for s in ve + vf + vp + be + bf + bp + ce + cf + cp:
        thin = s["id"].startswith(("string-", "after-")) or s["id"] in ("bow-hair", "bow-stick", "straps")
        if thin and s["filled"]:
            s["penMm"] = max(s.get("penMm", 0.0), 0.2)
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": [],
            "geometry": {"elevation": ve + be + ce, "front": vf + bf + cf, "plan": vp + bp + cp}, "decisions": rep_decisions(scale)}


def volume_decisions(scale, counts):
    """🧊 What the 3D model of one scale keeps, merges and why."""
    f = P.FEATURE[scale] * 1000
    texts = {
        50: T(f"1:50 (smallest feature {f:.1f} mm): the case as two closed half-shell polyhedra (8 mm ABS over EPS) of 19 superelliptic stations each, meeting at the parting plane, with both padded straps following the back, handle grip and mounts, three latches, two hinges, two wheels on their axle in housings and the front bumper; inside the lower-bout cradle, two endpin-pocket blocks, the neck rest and two lid spinners holding the bow. The cello body is one closed polyhedron of 42 stations through the outline of the 755 mm body (lower bout 440, waist 232, upper bout 342 mm) with 120 mm ribs and 24/26 mm arching; neck, fingerboard and pegbox are lofts, the scroll a turned disc, four pegs with thumb pieces, the tailpiece with four fine tuners, collar, nut, saddle, the 4 mm bridge as a declared sheet, eight slender string capsules, the endpin rod with its turned point, the stopper with its cup and the bow with stick (one slender loft), hair, frog, head, button and lapping. Texture solids: f-holes, rosin dust, worn varnish, rib scratch.", f"1:50 (kleinste Einzelheit {f:.1f} mm): der Koffer als zwei geschlossene Halbschalen-Polyeder (8 mm ABS über EPS) aus je 19 superelliptischen Stationen, die sich in der Trennebene treffen, mit beiden gepolsterten Gurten dem Rücken folgend, Griff und Haltern, drei Verschlüssen, zwei Scharnieren, zwei Rädern auf ihrer Achse in Radkästen und dem vorderen Puffer; innen die Unterbugwiege, zwei Stacheltaschenblöcke, die Halsauflage und zwei Deckelklammern für den Bogen. Der Cellokorpus ist ein geschlossenes Polyeder aus 42 Stationen durch den Umriss des 755-mm-Korpus (Unterbug 440, Taille 232, Oberbug 342 mm) mit 120-mm-Zargen und 24/26 mm Wölbung; Hals, Griffbrett und Wirbelkasten sind Lofts, die Schnecke eine gedrehte Scheibe, vier Wirbel mit Griffplatten, der Saitenhalter mit vier Feinstimmern, Stachelhalter, Sattel, Untersattel, der 4-mm-Steg als erklärte Platte, acht schlanke Saitenkapseln, der Stachelstab mit gedrehter Spitze, der Stachelhalter mit Mulde und der Bogen mit Stange (ein schlanker Loft), Haar, Frosch, Spitze, Schraube und Wicklung. Texturkörper: F-Löcher, Kolophoniumstaub, durchgeriebener Lack, Zargenkratzer."),
        100: T(f"1:100 (feature {f:.0f} mm): shells of 16 points per half-section, straps, handle and three latches kept (latches 15 mm proud), hinges and the hub dropped; cello body of 30 stations, pegs at 15 mm, tailpiece, collar 15 mm, bridge sheet, eight strings; fine tuners, nut, saddle and all texture solids merged or dropped; endpin one slender rod with a round end, stopper without cup, bow stick, hair and frog.", f"1:100 (Einzelheit {f:.0f} mm): Schalen mit 16 Punkten je Halbquerschnitt, Gurte, Griff und drei Verschlüsse behalten (Verschlüsse 15 mm vorstehend), Scharniere und Nabe entfallen; Cellokorpus aus 30 Stationen, Wirbel mit 15 mm, Saitenhalter, Stachelhalter 15 mm, Stegplatte, acht Saiten; Feinstimmer, Sattel, Untersattel und alle Texturkörper zusammengezogen oder entfallen; Stachel ein schlanker Stab mit runder Spitze, Stachelhalter ohne Mulde, Bogenstange, Haar und Frosch."),
        200: T(f"1:200 (feature {f:.0f} mm): shells of 13 stations and 12 points, wheels 30 mm wide, bumper 32 mm; straps, handle and latches only as thermal surfaces; cello body of 20 stations, neck at least 30 mm wide, one pegbox-and-scroll loft, bridge sheet, four speaking strings; pegs, tailpiece, collar and stopper merged or thermal only; bow stick and hair.", f"1:200 (Einzelheit {f:.0f} mm): Schalen aus 13 Stationen und 12 Punkten, Räder 30 mm breit, Puffer 32 mm; Gurte, Griff und Verschlüsse nur als thermische Flächen; Cellokorpus aus 20 Stationen, Hals mindestens 30 mm breit, ein Loft für Wirbelkasten und Schnecke, Stegplatte, vier schwingende Saiten; Wirbel, Saitenhalter, Stachelhalter und Stachelscheibe zusammengezogen oder nur thermisch; Bogenstange und Haar."),
        500: T(f"1:500 (feature {f:.0f} mm): shells of 10 stations and 8 points, wheels and bumper merged into one 80 mm foot block; cello body of 14 stations, neck and scroll one slender 23 mm rod, endpin and bow stick slender rods; everything below 75 mm is a thermal surface only.", f"1:500 (Einzelheit {f:.0f} mm): Schalen aus 10 Stationen und 8 Punkten, Räder und Puffer in einen 80-mm-Fußblock gezogen; Cellokorpus aus 14 Stationen, Hals und Schnecke ein schlanker 23-mm-Stab, Stachel und Bogenstange schlanke Stäbe; alles unter 75 mm ist nur noch thermische Fläche."),
    }
    structure = {
        50: T("Structure: the two shells are the monocoque of the case (members of case and lid), with the hinges as joints, the wheels on their axle joint, the bumper and the interior pads as the load path from the cello into the shell; the cello as its real load-bearing parts: 4 mm arched top and back plates, the 1.4 mm rib garland, lower and upper blocks, bass bar, sound post, neck, fingerboard, tailpiece, bridge and the four strings, with the neck joint and the endpin collar as joints; endpin rod, stopper disc; bow stick and hair with the frog as joint; one mass marker per bone at its centre of mass.", "Tragwerk: die zwei Schalen sind die Monocoque des Koffers (Stäbe von Koffer und Deckel), mit den Scharnieren als Gelenke, den Rädern auf ihrem Achsgelenk, dem Puffer und den Innenpolstern als Lastweg vom Cello in die Schale; das Cello mit seinen wirklich tragenden Teilen: 4 mm gewölbte Decke und Boden, der 1,4-mm-Zargenkranz, Unter- und Oberklotz, Bassbalken, Stimmstock, Hals, Griffbrett, Saitenhalter, Steg und die vier Saiten, mit Halsfuß und Stachelhalter als Gelenke; Stachelstab, Stachelscheibe; Bogenstange und Haar mit dem Frosch als Gelenk; je Knochen eine Massenmarke im Schwerpunkt."),
        100: T("Structure as at 1:50 with the hinges merged into the shell; bass bar 15 mm wide.", "Tragwerk wie 1:50, Scharniere in die Schale gezogen; Bassbalken 15 mm breit."),
        200: T("Structure: shells, wheels, pads, plates, ribs, blocks, bass bar, sound post, neck, fingerboard, bridge, strings, endpin, bow stick and hair; tailpiece and stopper only as masses.", "Tragwerk: Schalen, Räder, Polster, Decke, Boden, Zargen, Klötze, Bassbalken, Stimmstock, Hals, Griffbrett, Steg, Saiten, Stachel, Bogenstange und Haar; Saitenhalter und Stachelhalter nur noch als Massen."),
        500: T("Structure: shells and foot block, pads, the rib garland and the lower block for the cello box (the 4 mm plates are below 75 mm and merged into it), the slender neck rod, the endpin and the bow stick.", "Tragwerk: Schalen und Fußblock, Polster, der Zargenkranz und der Unterklotz für den Cellokasten (die 4-mm-Platten liegen unter 75 mm und sind darin aufgegangen), der schlanke Halsstab, der Stachel und die Bogenstange."),
    }
    tail = T(f"Solids at this scale: body {counts[0]}, thermal {counts[1]}, structure {counts[2]}.", f"Körper bei diesem Maßstab: Körperliste {counts[0]}, thermisch {counts[1]}, Tragwerk {counts[2]}.")
    return [texts[scale], structure[scale], tail]


def mass_markers(scale):
    """⚖️ One compact mass marker per bone with a mass, at its centre of mass."""
    r = max(P.FEATURE[scale] * 0.6, 0.012)
    out = []
    for b in rig()["bones"]:
        if "mass" not in b:
            continue
        cx, cy = b["mass"]["comX"], b["mass"]["comY"]
        mat = {"case": "abs-eps", "case-lid": "abs-eps", "case-wheels": "pu-wheel", "cello": "maple-varnish", "endpin": "endpin-steel", "stopper": "rosewood", "bow": "pernambuco"}[b["id"]]
        w = -b.get("z", 0.0) if b["id"] == "case-lid" else 0.0
        out.append(P.ellipsoid(f"mass-{b['id']}", b["id"], mat, 0.2, (cx, cy, w), (r, r, r), member=b["id"], part="mass"))
    return out


def volume(scale):
    """🧊 The authored volume at one scale."""
    cb, ct, cs = P.case_parts(scale)
    vb, vt, vs, _ = P.cello_parts(scale)
    bb, bt, bs = P.bow_parts(scale)
    ib, is_ = P.interior_parts(scale)
    body = cb + ib + vb + bb
    thermal = ct + vt + bt
    structure = cs + is_ + vs + bs + mass_markers(scale)
    return {"status": "authored", "denominator": scale, "body": body, "thermal": thermal, "structure": structure, "decisions": volume_decisions(scale, (len(body), len(thermal), len(structure)))}


def drawn_extent(rep):
    """📏 The extents of the default (rest) drawing: height from the elevation, x from the plan, z from the front."""
    ys, xs, zs = [], [], []
    for s in rep["geometry"]["front"]:
        if s["bone"].startswith("case") and s["role"] != "ground":
            for p in s["points"]:
                zs.append(p[0])
                ys.append(p[1])
    for s in rep["geometry"]["plan"]:
        if s["bone"].startswith("case"):
            for p in s["points"]:
                xs.append(p[0])
    return max(ys) - min(ys), max(xs) - min(xs), max(zs) - min(zs)


def build():
    """🏗️ Assembles and writes the asset document."""
    reps = {str(s): representation(s) for s in SCALES}
    vols = {str(s): volume(s) for s in SCALES}
    areas = region_areas((vols["50"]["body"], vols["50"]["thermal"]))
    h, w, d = drawn_extent(reps["50"])
    asset = {
        "id": "used-cello", "category": "objects",
        "title": T("Used Cello with Hard Case", "Gebrauchtes Cello mit Hartschalenkoffer"),
        "summary": T(f"A second-hand 4/4 workshop cello of 1.24 m (body 755 mm, lower bout 440 mm, string length 692 mm) with bow, 500 mm steel endpin and rosewood endpin stopper, in a wheeled ABS-over-EPS hard case of 1.36 x 0.47 x 0.29 m with backpack straps; the varnish is rubbed through at the upper bout, the right rib is scratched, rosin dust lies under the strings, the case corners are scuffed and two old stickers stay on the lid. Stored upright in the closed case, carried on the back, and played seated with the endpin pulled out 300 mm and the cello leaning back 32 degrees.", f"Ein gebrauchtes 4/4-Werkstattcello von 1,24 m (Korpus 755 mm, Unterbug 440 mm, Mensur 692 mm) mit Bogen, 500-mm-Stahlstachel und Stachelhalter aus Palisander, in einem ABS-über-EPS-Hartschalenkoffer mit Rädern von 1,36 x 0,47 x 0,29 m mit Rucksackgurten; der Lack ist an der oberen Backe durchgerieben, die rechte Zarge zerkratzt, Kolophoniumstaub liegt unter den Saiten, die Kofferecken sind verschrammt und zwei alte Aufkleber kleben auf dem Deckel. Aufrecht im geschlossenen Koffer gelagert, auf dem Rücken getragen und im Sitzen gespielt mit 300 mm ausgezogenem Stachel und 32 Grad zurückgelehntem Cello."),
        "tags": ["used", "instrument", "music", "second-hand", "strings", "cello", "violoncello", "bowed", "chordophone", "bow", "hard-case", "wheeled-case", "endpin", "spruce", "maple", "ebony", "abs"],
        "provenance": {"author": "Semio staffage content (used-instruments wave, row 06)", "method": "handcrafted", "created": "2026-11-22", "references": references(),
                       "notes": T("Hornbostel-Sachs 321.322-71. A 4/4 workshop cello of the common Stradivari pattern as dealers and music-school depots offer it second-hand, cleaned and set up to be played again, with its student hard case. Stored pose: the case stands upright on its wheels and front bumper with the lid towards +x; the instrument, its bow and stopper are inside. The player and the carrying person are not part of the asset: their knees, chest, hands and shoulders are supports. The lid hinge is a revolute bone about the hinge line (twist), not used by a pose: the case stays closed while the cello is played, as it stands in a depot.", "Hornbostel-Sachs 321.322-71. Ein 4/4-Werkstattcello des verbreiteten Stradivari-Modells, wie Händler und Musikschullager es gebraucht anbieten, gereinigt und wieder spielbar eingerichtet, mit seinem Schülerhartschalenkoffer. Lagerpose: der Koffer steht aufrecht auf Rädern und vorderem Puffer, der Deckel nach +x; Instrument, Bogen und Stachelhalter liegen darin. Spielende und tragende Person sind nicht Teil des Assets: ihre Knie, Brust, Hände und Schultern sind Auflager. Das Deckelscharnier ist ein Drehknochen um die Scharnierlinie (Verdrehung), von keiner Pose benutzt: der Koffer bleibt geschlossen, während das Cello gespielt wird, wie er in einem Lager steht.")},
        "dimensions": {"heightM": round(h, 3), "widthM": round(w, 3), "depthM": round(d, 3), "massKg": TOTAL_KG},
        "materials": materials(),
        "rig": rig(),
        "anchors": anchors(),
        "representations": reps,
        "symbols": [],
        "poses": poses(),
        "clips": clips(),
        "thermal": thermal_model(areas),
        "structure": structure_model(),
        "volumes": vols,
        "presentation": {"defaultPose": "stored", "defaultClip": "bowing", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    (HERE / "🔣️.json").write_text(json.dumps(doc, ensure_ascii=False, separators=(",", ":")) + "\n")
    print("[DEBUG] written", (HERE / "🔣️.json").stat().st_size, "bytes; dims", asset["dimensions"], "mass", MASS)
    print("[DEBUG] areas", {k: round(v, 4) for k, v in areas.items()})


if __name__ == "__main__":
    build()
