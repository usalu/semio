"""💜️ common-lilac: a mature suckering Syringa vulgaris of seven upright stems in a vase, foliage in the upper two thirds and upright
panicles at the shoot tips (default pose: full bloom in May). Every coordinate below is authored by hand in world rest metres
(x ahead, y up, z left); `kit.py` converts and projects, `assemble.py` builds the physics and the document.

@see ./kit.py
@see ./assemble.py"""
import math
import sys

from kit import T, Lobe, Cap
import assemble
import refs

ID = "common-lilac"
BARK, WOOD = "lilac-bark", "lilac-wood"
SUN, SHADE, FLOWER = "lilac-leaf-sun", "lilac-leaf-shade", "lilac-flower"

STEMS = [
    ("s1", T("Front left stem", "Vorderer linker Stamm"), [(0.10, 0.10, 0.10), (0.45, 1.70, 0.32), (0.98, 2.85, 0.60)], [0.085, 0.060, 0.035]),
    ("s2", T("Back left stem", "Hinterer linker Stamm"), [(-0.10, 0.10, 0.12), (-0.48, 1.65, 0.58), (-0.72, 2.80, 1.05)], [0.080, 0.055, 0.032]),
    ("s3", T("Back right stem", "Hinterer rechter Stamm"), [(-0.12, 0.10, -0.10), (-0.50, 1.75, -0.42), (-1.00, 2.90, -1.00)], [0.080, 0.055, 0.032]),
    ("s4", T("Front right stem", "Vorderer rechter Stamm"), [(0.12, 0.10, -0.12), (0.48, 1.72, -0.46), (0.72, 2.82, -0.72)], [0.075, 0.052, 0.030]),
    ("s5", T("Central old stem", "Mittlerer alter Stamm"), [(0.00, 0.10, 0.00), (0.05, 1.95, 0.03), (0.10, 3.15, 0.06)], [0.105, 0.075, 0.045]),
    ("s6", T("Front stem", "Vorderer Stamm"), [(0.16, 0.10, 0.02), (0.62, 1.68, 0.24), (1.05, 2.60, 0.30)], [0.070, 0.050, 0.030]),
    ("s7", T("Back stem", "Hinterer Stamm"), [(-0.16, 0.10, -0.03), (-0.64, 1.66, -0.28), (-1.08, 2.58, -0.34)], [0.070, 0.050, 0.030]),
]
OUTER = ["s1", "s2", "s3", "s4", "s6", "s7"]
NAMES = {"s1": ("front left", "vorne links"), "s2": ("back left", "hinten links"), "s3": ("back right", "hinten rechts"), "s4": ("front right", "vorne rechts"), "s6": ("front", "vorne"), "s7": ("back", "hinten")}


def pts(sid):
    return [s for s in STEMS if s[0] == sid][0][2]


def along(sid, seg, t):
    p = pts(sid)
    a, b = p[seg], p[seg + 1]
    return tuple(a[k] + (b[k] - a[k]) * t for k in range(3))


def out(sid):
    x, _, z = pts(sid)[2]
    n = math.hypot(x, z)
    return (x / n, z / n)


def at(c, d, k, dy):
    return (c[0] + k * d[0], c[1] + dy, c[2] + k * d[1])


FOLIAGE = []
for sid in OUTER:
    en, de = NAMES[sid]
    ang = 0 if pts(sid)[2][0] > 0 else 180
    FOLIAGE.append({"id": "f-%s-top" % sid, "parent": "%s-b" % sid, "at": 1.0, "ang": ang, "title": T("Flowering shoot mass %s" % en, "Blühende Triebmasse %s" % de)})
    FOLIAGE.append({"id": "f-%s-mid" % sid, "parent": "%s-b" % sid, "at": 0.3, "ang": ang, "title": T("Lower foliage mass %s" % en, "Untere Laubmasse %s" % de)})
FOLIAGE += [
    {"id": "f-suckers", "parent": "root-plate", "at": 1.0, "ang": 0, "title": T("Thicket of root suckers", "Dickicht der Wurzelausläufer"), "level": "stem"},
    {"id": "f-s5-top", "parent": "s5-b", "at": 1.0, "ang": 0, "title": T("Crown top with panicles", "Kronenspitze mit Rispen")},
    {"id": "f-core-low", "parent": "s5-a", "at": 0.95, "ang": 0, "title": T("Shaded lower core", "Beschatteter Kern unten")},
    {"id": "f-core-up", "parent": "s5-b", "at": 0.45, "ang": 0, "title": T("Shaded upper core", "Beschatteter Kern oben")},
]

TOP = {"s1": (0.62, 0.55, 0.60), "s2": (0.60, 0.54, 0.62), "s3": (0.63, 0.56, 0.60), "s4": (0.60, 0.53, 0.60), "s6": (0.58, 0.52, 0.56), "s7": (0.58, 0.52, 0.56)}
PHASE = {"s1": 0.3, "s2": 1.1, "s3": 2.0, "s4": 2.7, "s6": 0.8, "s7": 1.7}
PANICLE_TOP = [(-38, 52), (8, 30), (48, 62)]
LEAF_TEX = [(-0.38, 0.28, 22, 0.08, 0.18), (0.18, 0.44, -14, 0.07, 0.18), (0.46, 0.02, 38, 0.075, 0.16), (-0.08, -0.24, -28, 0.08, 0.18), (0.24, -0.42, 12, 0.07, 0.14), (-0.50, -0.06, 64, 0.065, 0.16)]


def panicle(id, bone, c, rad, d, az, el, L, r0, r1, k=0.84):
    """🌸️ An upright panicle: its base sinks into the lobe at the surface direction (azimuth from the outward direction, elevation), its axis leans up."""
    a = math.atan2(d[1], d[0]) + math.radians(az)
    e = math.radians(el)
    n = (math.cos(e) * math.cos(a), math.sin(e), math.cos(e) * math.sin(a))
    p0 = (c[0] + n[0] * rad[0] * k, c[1] + n[1] * rad[1] * k, c[2] + n[2] * rad[2] * k)
    ax = (n[0] * 0.55, n[1] * 0.55 + 0.9, n[2] * 0.55)
    m = math.sqrt(sum(x * x for x in ax))
    p1 = tuple(p0[i] + L * ax[i] / m for i in range(3))
    return Cap(id, bone, p0, p1, r0, r1, "flowers", FLOWER, 0.85)


def stem_caps(scale, upper=True, lower=True, thermal_upper=False):
    out = []
    for sid, _, p, d in STEMS:
        if lower:
            out.append(Cap("%s-a" % sid, "%s-a" % sid, p[0], p[1], d[0] / 2, d[1] / 2, "stem-bark", BARK, 0.35))
        if upper:
            out.append(Cap("%s-b" % sid, "%s-b" % sid, p[1], p[2], d[1] / 2, d[2] / 2, "stem-bark", BARK, 0.35))
        elif thermal_upper:
            out.append(Cap("%s-b" % sid, "%s-b" % sid, p[1], p[2], d[1] / 2, d[2] / 2, "stem-bark", BARK, 0.35, role="thermal-only"))
    return out


def emitter_solids(k):
    sun = Lobe("emit-sun", "f-s5-top", (0.10, 3.45, 0.06), (0.22 * k, 0.16 * k, 0.22 * k), "leaf-sun", SUN, 0.6, role="detail")
    sun.tags = ["transpiration-sun"]
    shade = Lobe("emit-shade", "f-core-low", (0.0, 2.30, 0.0), (0.34 * k, 0.26 * k, 0.32 * k), "leaf-shade", SHADE, 0.45, role="detail")
    shade.tags = ["transpiration-shade"]
    return [sun, shade]


def scale50():
    lobes, caps = [], stem_caps("50")
    for sid in OUTER:
        t, d = pts(sid)[2], out(sid)
        rt = TOP[sid]
        top = at(t, d, 0.12, 0.30)
        lobes.append(Lobe("lobe-%s-top" % sid, "f-%s-top" % sid, top, rt, "leaf-sun", SUN, 0.62, k=7, amp=0.07, phase=PHASE[sid], texture=True))
        sat = at(top, d, 0.30, 0.20)
        lobes.append(Lobe("lobe-%s-sat" % sid, "f-%s-top" % sid, sat, (0.34, 0.30, 0.33), "leaf-sun", SUN, 0.64, k=5, amp=0.07, phase=PHASE[sid] + 0.5))
        mid = at(along(sid, 1, 0.3), d, 0.42, -0.02)
        lobes.append(Lobe("lobe-%s-mid" % sid, "f-%s-mid" % sid, mid, (0.52, 0.46, 0.50), "leaf-sun", SUN, 0.58, k=6, amp=0.07, phase=PHASE[sid] + 1.0, texture=True))
        for i, (az, el) in enumerate(PANICLE_TOP):
            caps.append(panicle("pan-%s-%d" % (sid, i + 1), "f-%s-top" % sid, top, rt, d, az, el, 0.20, 0.05, 0.012))
        caps.append(panicle("pan-%s-sat" % sid, "f-%s-top" % sid, sat, (0.34, 0.30, 0.33), d, 0, 55, 0.18, 0.045, 0.011))
        caps.append(panicle("pan-%s-mid" % sid, "f-%s-mid" % sid, mid, (0.52, 0.46, 0.50), d, 0, 18, 0.18, 0.045, 0.011))
    top = (0.10, 3.45, 0.06)
    lobes.append(Lobe("lobe-s5-top", "f-s5-top", top, (0.72, 0.52, 0.70), "leaf-sun", SUN, 0.64, k=8, amp=0.07, phase=0.4, texture=True))
    lobes.append(Lobe("lobe-s5-sat-a", "f-s5-top", (0.42, 3.62, 0.30), (0.34, 0.28, 0.32), "leaf-sun", SUN, 0.66, k=5, amp=0.07, phase=0.9))
    lobes.append(Lobe("lobe-s5-sat-b", "f-s5-top", (-0.30, 3.60, -0.28), (0.36, 0.30, 0.34), "leaf-sun", SUN, 0.66, k=5, amp=0.07, phase=1.9))
    lobes.append(Lobe("lobe-core-low", "f-core-low", (0.0, 2.30, 0.0), (0.95, 0.70, 0.90), "leaf-shade", SHADE, 0.42, k=8, amp=0.05, phase=0.2, hatch="shade-lines"))
    lobes.append(Lobe("lobe-core-up", "f-core-up", (0.05, 2.95, 0.02), (0.85, 0.55, 0.80), "leaf-shade", SHADE, 0.44, k=7, amp=0.05, phase=1.2, hatch="shade-lines"))
    for i, (az, el) in enumerate([(0, 82), (95, 55), (200, 58)]):
        caps.append(panicle("pan-s5-%d" % (i + 1), "f-s5-top", top, (0.72, 0.52, 0.70), (1.0, 0.0), az, el, 0.20, 0.05, 0.012))
    lobes.append(Lobe("lobe-suckers", "f-suckers", (0.0, 0.42, 0.0), (0.62, 0.42, 0.56), "leaf-sun", SUN, 0.56, texture=True))
    return {"collar": (0.36, 0.26), "caps": caps, "lobes": lobes, "texture": True, "thermal_only": emitter_solids(1.0), "support_r": 0.55}


def scale100():
    lobes, caps = [], stem_caps("100")
    for sid in OUTER:
        t, d = pts(sid)[2], out(sid)
        rt = TOP[sid]
        top = at(t, d, 0.13, 0.31)
        rr = (rt[0] + 0.05, rt[1] + 0.04, rt[2] - 0.01)
        lobes.append(Lobe("lobe-%s-top" % sid, "f-%s-top" % sid, top, rr, "leaf-sun", SUN, 0.62, k=5, amp=0.06, phase=PHASE[sid] + 0.2, texture=True))
        mid = at(along(sid, 1, 0.3), d, 0.42, -0.02)
        lobes.append(Lobe("lobe-%s-mid" % sid, "f-%s-mid" % sid, mid, (0.53, 0.47, 0.51), "leaf-sun", SUN, 0.58, k=4, amp=0.06, phase=PHASE[sid] + 1.3))
        for i, (az, el) in enumerate([(-30, 45), (35, 58)]):
            caps.append(panicle("pan-%s-%d" % (sid, i + 1), "f-%s-top" % sid, top, rr, d, az, el, 0.21, 0.055, 0.014))
    top = (0.10, 3.47, 0.06)
    lobes.append(Lobe("lobe-s5-top", "f-s5-top", top, (0.80, 0.56, 0.78), "leaf-sun", SUN, 0.64, k=6, amp=0.06, phase=0.6, texture=True))
    lobes.append(Lobe("lobe-core-low", "f-core-low", (0.0, 2.30, 0.0), (0.96, 0.71, 0.91), "leaf-shade", SHADE, 0.42, k=6, amp=0.05, phase=0.5, hatch="shade-lines"))
    lobes.append(Lobe("lobe-core-up", "f-core-up", (0.05, 2.95, 0.02), (0.86, 0.56, 0.81), "leaf-shade", SHADE, 0.44, k=5, amp=0.05, phase=1.5, hatch="shade-lines"))
    for i, (az, el) in enumerate([(0, 80), (120, 50), (240, 50)]):
        caps.append(panicle("pan-s5-%d" % (i + 1), "f-s5-top", top, (0.80, 0.56, 0.78), (1.0, 0.0), az, el, 0.18, 0.055, 0.014))
    lobes.append(Lobe("lobe-suckers", "f-suckers", (0.0, 0.42, 0.0), (0.63, 0.42, 0.57), "leaf-sun", SUN, 0.56))
    return {"collar": (0.36, 0.26), "caps": caps, "lobes": lobes, "texture": True, "thermal_only": emitter_solids(0.94), "support_r": 0.54}


def scale200():
    lobes, caps = [], stem_caps("200", upper=False, thermal_upper=True)
    for sid in OUTER:
        t, d = pts(sid)[2], out(sid)
        rt = TOP[sid]
        top = at(t, d, 0.12, 0.30)
        rr = (rt[0] + 0.07, rt[1] + 0.05, rt[2] + 0.0)
        lobes.append(Lobe("lobe-%s-top" % sid, "f-%s-top" % sid, top, rr, "leaf-sun", SUN, 0.6, k=3, amp=0.05, phase=PHASE[sid]))
        mid = at(along(sid, 1, 0.3), d, 0.40, 0.0)
        lobes.append(Lobe("lobe-%s-mid" % sid, "f-%s-mid" % sid, mid, (0.55, 0.50, 0.52), "leaf-sun", SUN, 0.58, k=3, amp=0.05, phase=PHASE[sid] + 1.0))
        caps.append(panicle("pan-%s" % sid, "f-%s-top" % sid, top, rr, d, 0, 52, 0.24, 0.07, 0.02, k=0.86))
    top = (0.10, 3.42, 0.06)
    lobes.append(Lobe("lobe-s5-top", "f-s5-top", top, (0.80, 0.58, 0.78), "leaf-sun", SUN, 0.62, k=4, amp=0.05, phase=0.3))
    lobes.append(Lobe("lobe-core", "f-core-low", (0.02, 2.55, 0.0), (0.96, 0.95, 0.90), "leaf-shade", SHADE, 0.42, k=4, amp=0.04, phase=0.9))
    for i, (az, el) in enumerate([(30, 70), (210, 62)]):
        caps.append(panicle("pan-s5-%d" % (i + 1), "f-s5-top", top, (0.80, 0.58, 0.78), (1.0, 0.0), az, el, 0.22, 0.07, 0.02))
    lobes.append(Lobe("lobe-suckers", "f-suckers", (0.0, 0.42, 0.0), (0.64, 0.42, 0.58), "leaf-sun", SUN, 0.56))
    return {"collar": (0.34, 0.25), "caps": caps, "lobes": lobes, "texture": False, "thermal_only": emitter_solids(0.88), "support_r": 0.52}


def scale500():
    merged = ["leaf-sun", "leaf-shade", "flowers"]
    lobes = []
    for sid in OUTER:
        t, d = pts(sid)[2], out(sid)
        rt = TOP[sid]
        top = at(t, d, 0.12, 0.30)
        lobes.append(Lobe("crown-%s" % sid, "f-%s-top" % sid, (top[0], top[1] - 0.14, top[2]), (rt[0] + 0.12, rt[1] + 0.16, rt[2] + 0.06), "leaf-sun", SUN, 0.6))
    lobes.append(Lobe("crown-top", "f-s5-top", (0.10, 3.42, 0.06), (0.80, 0.58, 0.78), "leaf-sun", SUN, 0.6))
    lobes.append(Lobe("crown-core", "f-core-low", (0.02, 2.46, 0.0), (1.05, 1.06, 1.0), "leaf-sun", SUN, 0.6))
    lobes.append(Lobe("crown-suckers", "f-suckers", (0.0, 0.42, 0.0), (0.62, 0.42, 0.56), "leaf-sun", SUN, 0.56))
    for lb in lobes:
        lb.regions = merged
    shade = Lobe("th-core", "f-core-low", (0.0, 2.45, 0.0), (0.9, 0.8, 0.85), "leaf-shade", SHADE, 0.42)
    fl = [panicle("th-pan-%d" % (i + 1), "f-s5-top", (0.08, 3.35, 0.05), (0.95, 0.66, 1.05), (1.0, 0.0), az, el, 0.22, 0.06, 0.03, k=0.8) for i, (az, el) in enumerate([(0, 75), (90, 50), (180, 50), (270, 50)])]
    for f in fl:
        f.role = "thermal-only"
        f.slender = False
    return {"collar": (0.24, 0.60, 1.45), "caps": fl, "lobes": lobes, "texture": False, "thermal_only": [shade] + emitter_solids(0.82), "support_r": 0.5}


SCALES = {"50": scale50(), "100": scale100(), "200": scale200(), "500": scale500()}

CROWN_PROJ = math.pi * 1.75 * 1.5
LEAF = {"leaf-sun": round(2.1 * CROWN_PROJ, 1), "leaf-shade": round(1.4 * CROWN_PROJ, 1)}


def rain_offsets(sp, rig):
    o = {}
    for id in rig.order:
        b = rig.bones[id]
        if b.kind == "wood" and not id.startswith("s5"):
            sgn = -1 if math.cos(math.radians(b.ang)) > 0 else 1
            o[id] = sgn * (1.2 if b.level == "stem" else 2.6)
        elif b.kind == "foliage" and not id.startswith("f-core") and id != "f-s5-top":
            o[id] = -4.0 if math.cos(math.radians(b.ang)) > 0 else 4.0
    return o


def rain_loads(sp, rig):
    per = {}
    for id, la in sp["_leaf_of"].items():
        fl = len([c for c in SCALES["50"]["caps"] if c.bone == id and c.region == "flowers"])
        per[id] = la * 0.2 + fl * sp["flowers_per_drawn"] * 0.04
    return assemble.carried(sp, rig, "rain", per, ("Intercepted rain", "Zurückgehaltener Regen"), "keim-2006")


SP = {
    "id": ID,
    "title": T("Common Lilac, Multi-Stem", "Gewöhnlicher Flieder, mehrstämmig"),
    "summary": T("A mature common lilac (Syringa vulgaris) of seven upright suckering stems that open into a vase about 4 m high; the foliage of heart-shaped leaves fills the upper two thirds, and in May upright cone-shaped panicles stand at every shoot tip.",
                 "Ein ausgewachsener Gewöhnlicher Flieder (Syringa vulgaris) aus sieben aufrechten Ausläuferstämmen, die sich zu einer etwa 4 m hohen Vase öffnen; das Laub aus herzförmigen Blättern füllt die oberen zwei Drittel, und im Mai stehen an jeder Triebspitze aufrechte kegelförmige Rispen."),
    "tags": ["shrub", "deciduous", "flowering-shrub", "multi-stem", "garden", "panicles", "wind", "rain", "transpiration"],
    "distinction": T("A suckering multi-stem flowering shrub with a bare-legged vase of seven stems and upright panicles at the shoot tips, whose rain pose bows the stems under soaked flower heads.",
                     "Ein ausläufertreibender, mehrstämmiger Blütenstrauch mit kahler Stammvase aus sieben Stämmen und aufrechten Rispen an den Triebspitzen; in der Regenpose biegen die vollgesogenen Blütenstände die Stämme nach außen."),
    "references": refs.common + [refs.rhs("rhs-syringa-vulgaris", "Syringa vulgaris (common lilac)", "eventual height 4 to 8 m, spread 2.5 to 4 m, flowers in late spring"), refs.hillier, refs.koerner, refs.keim],
    "notes": T("Rig, stems, foliage masses and panicles are authored by hand for a 25-year-old garden lilac on its own roots: seven stems of 7 to 10.5 cm at the base, crown about 3.6 m by 3.0 m and 4 m high, inside the RHS band of 4 to 8 m height and 2.5 to 4 m spread. Masses are green-wood volumes times an authored density and leaf areas from a leaf area index of 3.5 over the crown projection; about 160 panicles of about 30 g are an authored estimate, the drawn panicles are a sample of them, not a count.",
               "Rig, Stämme, Laubmassen und Rispen sind von Hand für einen 25-jährigen wurzelechten Gartenflieder entworfen: sieben Stämme von 7 bis 10,5 cm am Fuß, Krone etwa 3,6 m mal 3,0 m und 4 m hoch, innerhalb der RHS-Spanne von 4 bis 8 m Höhe und 2,5 bis 4 m Breite. Massen sind Frischholzvolumen mal einer geschätzten Dichte, Blattflächen folgen aus einem Blattflächenindex von 3,5 über der Kronenprojektion; etwa 160 Rispen zu je etwa 30 g sind eine Autorenschätzung, die gezeichneten Rispen eine Auswahl, keine Zählung."),
    "wood": WOOD, "bark": BARK, "density": 1000, "leaf_ref": "jones-2014", "leaf_kg_m2": 0.18, "twig_factor": 1.6, "flower_kg": 0.03, "flowers_per_drawn": round(160 / 33, 3),
    "leaf_area": LEAF, "porosity": 0.8, "cd": 0.45, "trunk": "s5-a", "footprint": 0.8, "foliage_len": 0.12,
    "collar": {"h": 0.18, "r1": 0.26, "kg": 30, "title": T("Root crown of the suckering clump", "Wurzelstock des Ausläuferhorsts")},
    "stems": [{"id": sid, "title": t, "pts": p, "d": d, "limits": [6, 8]} for sid, t, p, d in STEMS],
    "foliage": FOLIAGE,
    "scales": SCALES,
    "texture": {"50": LEAF_TEX, "100": [(-0.3, 0.25, 20, 0.12, 0.16), (0.3, 0.3, -20, 0.11, 0.16), (0.0, -0.3, 35, 0.12, 0.14)]},
    "materials": [
        {"id": BARK, "title": T("Lilac bark, grey and finely fissured", "Fliederrinde, grau und fein rissig"), "emissivity": 0.95, "solarAbsorptivity": 0.68, "densityKgM3": 600, "conductivityWmK": 0.12, "source": "authored-estimate", "reference": "wood-handbook-2021"},
        {"id": WOOD, "title": T("Green lilac wood (dense, fine-grained)", "Frisches Fliederholz (dicht, feinfaserig)"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": 1000, "conductivityWmK": 0.17, "elasticModulusPa": 8000000000, "strengthPa": 55000000, "source": "authored-estimate", "reference": "wood-handbook-2021"},
        {"id": SUN, "title": T("Lilac sun leaves", "Flieder-Sonnenblätter"), "emissivity": 0.97, "solarAbsorptivity": 0.5, "source": "literature", "reference": "jones-2014"},
        {"id": SHADE, "title": T("Lilac shade leaves (effective)", "Flieder-Schattenblätter (wirksam)"), "emissivity": 0.97, "solarAbsorptivity": 0.28, "source": "authored-estimate", "reference": "jones-2014"},
        {"id": FLOWER, "title": T("Lilac florets (pale violet petals)", "Fliederblüten (blassviolette Kronblätter)"), "emissivity": 0.96, "solarAbsorptivity": 0.35, "source": "authored-estimate", "reference": "jones-2014"},
    ],
    "regions": [
        {"id": "stem-bark", "title": T("Grey bark of the seven stems and the root crown", "Graue Rinde der sieben Stämme und des Wurzelstocks"), "kind": "bark", "model": "passive", "material": BARK, "len": 0.07, "geom": "cylinder", "sky": 0.4, "hc": 40000, "ref": "wood-handbook-2021", "sun": 0.3},
        {"id": "leaf-sun", "title": T("Sun leaves of the outer crown", "Sonnenblätter der Außenkrone"), "kind": "foliage", "model": "leaf", "material": SUN, "area": LEAF["leaf-sun"], "exchange": True, "len": 0.07, "geom": "vertical-plate", "sky": 0.7, "hc": 800, "gs": 0.006, "lai": 2.1, "ref": "koerner-1979", "sun": 0.85},
        {"id": "leaf-shade", "title": T("Shade leaves of the inner crown", "Schattenblätter der Innenkrone"), "kind": "foliage", "model": "leaf", "material": SHADE, "area": LEAF["leaf-shade"], "exchange": True, "len": 0.08, "geom": "vertical-plate", "sky": 0.3, "hc": 650, "gs": 0.0035, "lai": 1.4, "ref": "koerner-1979", "sun": 0.12},
        {"id": "flowers", "title": T("Petals of the flower panicles", "Kronblätter der Blütenrispen"), "kind": "surface", "model": "passive", "material": FLOWER, "area": 6.0, "exchange": True, "len": 0.012, "geom": "sphere", "sky": 0.75, "hc": 1200, "ref": "jones-2014", "sun": 0.9},
    ],
    "emitters": [
        {"id": "transpiration-sun", "aid": "tp-sun", "title": T("Transpiration of the sun leaves", "Transpiration der Sonnenblätter"), "region": "leaf-sun", "diam": 1.2, "strength": 1.0,
         "anchors": [("f-%s-top" % s, None) for s in OUTER] + [("f-s5-top", None)]},
        {"id": "transpiration-shade", "aid": "tp-shade", "title": T("Transpiration of the shade leaves", "Transpiration der Schattenblätter"), "region": "leaf-shade", "diam": 1.4, "strength": 0.6,
         "anchors": [("f-core-low", (0.55, 2.25, 0.45)), ("f-core-low", (-0.55, 2.25, -0.45)), ("f-core-low", (0.0, 2.10, 0.0)), ("f-core-up", (0.4, 3.0, -0.35)), ("f-core-up", (-0.4, 2.95, 0.35))]},
    ],
    "environment": {"airTempC": 19, "meanRadiantTempC": 23, "relativeHumidity": 0.6, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 650, "pressurePa": 101325, "diffuseFraction": 0.2},
    "lean_ref": {"root": 0.0, "stem": 0.8, "stemtop": 1.6, "fol": 3.0},
    "sway_amp": {"root": 0.0, "stem": 0.5, "stemtop": 0.6, "fol": 0.7},
    "sway_lag": 0.45, "sway_period": 1.4,
    "gust_tau": {"root": 0.5, "stem": 0.4, "stemtop": 0.3, "fol": 0.2},
    "sway_desc": ("A deterministic %s s sway cycle about the breeze lean of the flowering lilac (5.5 m/s at 10 m; estimated first mode of the 4 m stems about 0.7 Hz). The upper stems swing behind the lower ones and the panicle-laden shoot masses nod about 2 degrees.",
                  "Ein deterministischer Schwingungszyklus von %s s um die Neigung des blühenden Flieders bei Brise (5,5 m/s in 10 m Höhe; geschätzte erste Eigenform der 4 m hohen Stämme etwa 0,7 Hz). Die oberen Stammteile schwingen den unteren nach, die rispenbeladenen Triebmassen nicken etwa 2 Grad."),
    "gust_desc": ("A single gust over 8 s from %.1f to %.1f m/s at 10 m: the wind-load tracks rise with the reconfigured drag law, the shoot masses lean first and the stems follow with a level-wise lag.",
                  "Eine einzelne Böe über 8 s von %.1f auf %.1f m/s in 10 m Höhe: Die Windlast-Spuren steigen mit dem umgeformten Widerstandsgesetz, zuerst neigen sich die Triebmassen, die Stämme folgen ebenenweise verzögert."),
    "breeze_pose": "flowering-breeze", "breeze_v10": 5.5, "gust_peak": 16.0, "default_pose": "flowering-calm", "default_scale": 100,
    "symbol_crown_base": 1.45, "symbol_k": 9, "symbol_stem": (0.24, 0.6),
}

SP["poses"] = [
    {"id": "flowering-calm", "title": T("Full bloom, calm", "Vollblüte, windstill"), "desc": T("A still late-May noon (1.5 m/s at 10 m) in full bloom: about 160 panicles stand upright on the shoot tips, the leaves transpire at 80 % of their maximum.",
                                                                                                "Ein stiller Mittag Ende Mai (1,5 m/s in 10 m Höhe) in Vollblüte: etwa 160 Rispen stehen aufrecht auf den Triebspitzen, die Blätter transpirieren mit 80 % ihres Maximums."), "v10": 1.5, "emit": 0.8},
    {"id": "flowering-breeze", "title": T("Full bloom in a breeze", "Vollblüte bei Brise"), "desc": T("A fresh breeze of 5.5 m/s at 10 m in full bloom: the shoot masses lean about 1 degree downwind, the scent drifts off with the transpired vapour.",
                                                                                                  "Eine frische Brise von 5,5 m/s in 10 m Höhe in Vollblüte: Die Triebmassen neigen sich etwa 1 Grad in Windrichtung, der Duft zieht mit dem transpirierten Dampf ab."), "v10": 5.5, "cd": 0.4},
    {"id": "storm", "title": T("Spring storm", "Frühjahrssturm"), "desc": T("A gale of 20 m/s at 10 m under an overcast sky: the leaves fold and the drag coefficient drops to 0.25 (reconfiguration after Vogel 1989); the stems lean about 1 degree, the shoot masses about 3 degrees.",
                                                                            "Ein Sturm von 20 m/s in 10 m Höhe unter bedecktem Himmel: Die Blätter falten sich, der Widerstandsbeiwert sinkt auf 0,25 (Umformung nach Vogel 1989); die Stämme neigen sich etwa 1 Grad, die Triebmassen etwa 3 Grad."), "v10": 20.0, "cd": 0.25, "sun": 0.3, "emit": 0.35},
    {"id": "rain-laden", "title": T("Soaked after a downpour", "Nach einem Platzregen vollgesogen"), "desc": T("Calm air just after a heavy shower in full bloom: the leaves hold about 0.2 kg of water per m2 and each panicle about 40 g, so the outer stems bow outwards by 1 to 3 degrees and the shoot masses hang 4 degrees lower; the wet surfaces barely transpire.",
                                                                                                          "Windstille kurz nach einem kräftigen Schauer in Vollblüte: Die Blätter halten etwa 0,2 kg Wasser je m2 und jede Rispe etwa 40 g, daher biegen sich die äußeren Stämme um 1 bis 3 Grad nach außen und die Triebmassen hängen 4 Grad tiefer; die nassen Flächen transpirieren kaum."), "v10": 1.0, "sun": 0.15, "emit": 0.06,
     "wet": {"leaf-sun": 1.0, "leaf-shade": 1.0, "flowers": 1.0, "stem-bark": 0.6}, "offsets": rain_offsets, "extra": rain_loads},
]

SP["thermal_assumptions"] = refs.thermal_assumptions("lilac", ("Flowers are a passive exchange surface: the petals of about 160 panicles (6 m2 of corolla, an authored estimate) take part in radiation and convection but are not given a stomatal model.",
                                                               "Blüten sind eine passive Austauschfläche: Die Kronblätter von etwa 160 Rispen (6 m2 Krone, eine Autorenschätzung) nehmen an Strahlung und Konvektion teil, erhalten aber kein Stomatamodell."))
SP["structure_assumptions"] = refs.structure_assumptions("seven stems", "sieben Stämme")
# The rest boxes give 3.454 m of depth; in the default pose (the stems lean a little in the light air) the product measures 3.440 m on the 1:50 body (FIN-B/probe.ts).
SP["measured_dims"] = {"depthM": 3.44}
SP["decisions"] = refs.decisions_lilac()
SP["volume_decisions"] = refs.volume_decisions_lilac()

if __name__ == "__main__":
    for scale, S in SCALES.items():
        for c in S.get("caps", []):
            if c.slender is None and 2 * min(c.r0, c.r1) >= 0.15 * int(scale) / 1000:
                c.slender = False
    for e in SP["emitters"]:
        e["anchors"] = [(b, p if p is not None else None) for b, p in e["anchors"]]
    tops = {lb.bone: lb.c for lb in SCALES["50"]["lobes"] if lb.id.endswith("-top")}
    for e in SP["emitters"]:
        e["anchors"] = [(b, p if p is not None else tops[b]) for b, p in e["anchors"]]
    doc, path, rig = assemble.build(SP)
    assemble.report(doc, rig, SP)
    print("[DEBUG] wrote", path)
