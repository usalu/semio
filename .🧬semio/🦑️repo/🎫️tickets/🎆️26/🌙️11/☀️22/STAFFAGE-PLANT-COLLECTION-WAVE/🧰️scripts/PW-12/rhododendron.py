"""🌺️ rhododendron: a mature evergreen Rhododendron ponticum hybrid, a dense dome 3 m high on eight crooked stems around a central one,
leathery leaves to 0.25 m above the ground and domed flower trusses at the shoot tips (default pose: full bloom in May). Every coordinate
below is authored by hand in world rest metres (x ahead, y up, z left); `kit.py` converts and projects, `assemble.py` builds the document.

@see ./kit.py
@see ./assemble.py"""
import math

from kit import T, Lobe, Cap
import assemble
import refs

ID = "rhododendron"
BARK, WOOD = "rhododendron-bark", "rhododendron-wood"
SUN, SHADE, FLOWER = "rhododendron-leaf-sun", "rhododendron-leaf-shade", "rhododendron-flower"

STEMS = [
    ("s1", 20, 1.02, 2.12, 0.2), ("s2", 65, 0.98, 2.18, 0.9), ("s3", 115, 1.04, 2.10, 1.6), ("s4", 160, 1.00, 2.16, 2.3),
    ("s5", 200, 1.03, 2.08, 3.0), ("s6", 245, 0.99, 2.15, 3.7), ("s7", 290, 1.02, 2.11, 4.4), ("s8", 335, 1.00, 2.17, 5.1),
]
NAMES = {"s1": ("front", "vorn"), "s2": ("front left", "vorn links"), "s3": ("back left", "hinten links"), "s4": ("back", "hinten"),
         "s5": ("back, slightly right", "hinten, leicht rechts"), "s6": ("back right", "hinten rechts"), "s7": ("front right", "vorn rechts"), "s8": ("front, slightly right", "vorn, leicht rechts")}
D_OUT = [0.075, 0.060, 0.042, 0.026]
D_C = [0.085, 0.066, 0.046, 0.030]


def polar(a, rad, y):
    return (rad * math.cos(math.radians(a)), y, rad * math.sin(math.radians(a)))


def stem_pts(sid):
    if sid == "s0":
        return [(0.0, 0.08, 0.0), (0.06, 0.90, 0.04), (0.10, 1.80, 0.07), (0.14, 2.55, 0.10)]
    _, a, R, H, _ = [s for s in STEMS if s[0] == sid][0]
    return [polar(a, 0.08, 0.08), polar(a + 4, 0.38, 0.48), polar(a + 8, 0.78, 1.30), polar(a + 10, R, H)]


def dir_of(sid):
    _, a, _, _, _ = [s for s in STEMS if s[0] == sid][0]
    return (math.cos(math.radians(a + 10)), math.sin(math.radians(a + 10)))


def at(c, d, k, dy):
    return (c[0] + k * d[0], c[1] + dy, c[2] + k * d[1])


FOLIAGE = []
for sid, a, *_ in STEMS:
    en, de = NAMES[sid]
    ang = 0 if math.cos(math.radians(a + 10)) >= 0 else 180
    FOLIAGE.append({"id": "f-%s-top" % sid, "parent": "%s-c" % sid, "at": 1.0, "ang": ang, "title": T("Flowering shoot mass %s" % en, "Blühende Triebmasse %s" % de)})
    FOLIAGE.append({"id": "f-%s-mid" % sid, "parent": "%s-b" % sid, "at": 1.0, "ang": ang, "title": T("Middle leaf mass %s" % en, "Mittlere Blattmasse %s" % de)})
    FOLIAGE.append({"id": "f-%s-skirt" % sid, "parent": "%s-a" % sid, "at": 1.0, "ang": ang, "title": T("Skirt leaf mass %s" % en, "Saum-Blattmasse %s" % de)})
FOLIAGE += [
    {"id": "f-s0-top", "parent": "s0-c", "at": 1.0, "ang": 0, "title": T("Dome top with trusses", "Kuppel mit Blütenständen")},
    {"id": "f-core", "parent": "s0-b", "at": 0.3, "ang": 0, "title": T("Shaded core of the dome", "Beschatteter Kern der Kuppel")},
]
TRUSS = (0.085, 0.07, 0.085)
LEAF_TEX = [(-0.40, 0.20, 15, 0.10, 0.06), (0.15, 0.40, -10, 0.09, 0.06), (0.45, -0.02, 30, 0.09, 0.05), (-0.10, -0.28, -20, 0.10, 0.06), (0.22, -0.45, 5, 0.09, 0.05), (-0.48, -0.15, 40, 0.08, 0.05)]


def truss(id, bone, c, rad, d, az, el, size, k=0.93):
    """🌺️ A truss: a domed head of ten to fifteen funnel flowers whose centre sits on the leaf mass at the given azimuth (from the outward direction) and elevation."""
    a = math.atan2(d[1], d[0]) + math.radians(az)
    e = math.radians(el)
    n = (math.cos(e) * math.cos(a), math.sin(e), math.cos(e) * math.sin(a))
    cc = (c[0] + n[0] * rad[0] * k, c[1] + n[1] * rad[1] * k, c[2] + n[2] * rad[2] * k)
    return Lobe(id, bone, cc, size, "flowers", FLOWER, 0.84, k=9, amp=0.12, phase=az * 0.013)


def stem_caps(segs=(0, 1, 2), thermal=()):
    out = []
    for sid in ["s0"] + [s[0] for s in STEMS]:
        p = stem_pts(sid)
        d = D_C if sid == "s0" else D_OUT
        for i in range(3):
            if i in segs:
                out.append(Cap("%s-%s" % (sid, "abc"[i]), "%s-%s" % (sid, "abc"[i]), p[i], p[i + 1], d[i] / 2, d[i + 1] / 2, "stem-bark", BARK, 0.3))
            elif i in thermal:
                out.append(Cap("%s-%s" % (sid, "abc"[i]), "%s-%s" % (sid, "abc"[i]), p[i], p[i + 1], d[i] / 2, d[i + 1] / 2, "stem-bark", BARK, 0.3, role="thermal-only"))
    return out


def emitter_solids(k):
    sun = Lobe("emit-sun", "f-s0-top", (0.14, 2.62, 0.10), (0.24 * k, 0.14 * k, 0.24 * k), "leaf-sun", SUN, 0.55, role="detail")
    sun.tags = ["transpiration-sun"]
    shade = Lobe("emit-shade", "f-core", (0.0, 1.50, 0.0), (0.38 * k, 0.30 * k, 0.36 * k), "leaf-shade", SHADE, 0.4, role="detail")
    shade.tags = ["transpiration-shade"]
    return [sun, shade]


def scale50():
    lobes, caps, fl = [], stem_caps(), []
    for sid, a, R, H, ph in STEMS:
        t, d = stem_pts(sid)[3], dir_of(sid)
        top = at(t, d, 0.10, 0.12)
        rt = (0.55, 0.48, 0.52)
        lobes.append(Lobe("lobe-%s-top" % sid, "f-%s-top" % sid, top, rt, "leaf-sun", SUN, 0.5, k=7, amp=0.06, phase=ph, texture=True))
        sat = at(top, d, 0.30, 0.24)
        lobes.append(Lobe("lobe-%s-sat" % sid, "f-%s-top" % sid, sat, (0.30, 0.26, 0.28), "leaf-sun", SUN, 0.52, k=5, amp=0.06, phase=ph + 0.4))
        mid = polar(a + 9, 1.20, 1.42)
        lobes.append(Lobe("lobe-%s-mid" % sid, "f-%s-mid" % sid, mid, (0.52, 0.55, 0.50), "leaf-sun", SUN, 0.48, k=7, amp=0.06, phase=ph + 0.9, texture=True))
        skirt = polar(a + 6, 1.06, 0.68)
        lobes.append(Lobe("lobe-%s-skirt" % sid, "f-%s-skirt" % sid, skirt, (0.52, 0.44, 0.50), "leaf-sun", SUN, 0.46, k=6, amp=0.06, phase=ph + 1.5))
        for i, (az, el) in enumerate([(-35, 45), (5, 28), (40, 55)]):
            fl.append(truss("truss-%s-%d" % (sid, i + 1), "f-%s-top" % sid, top, rt, d, az, el, TRUSS))
        fl.append(truss("truss-%s-sat" % sid, "f-%s-top" % sid, sat, (0.30, 0.26, 0.28), d, 0, 50, TRUSS))
        for i, (az, el) in enumerate([(-25, 15), (25, 22)]):
            fl.append(truss("truss-%s-mid-%d" % (sid, i + 1), "f-%s-mid" % sid, mid, (0.52, 0.55, 0.50), d, az, el, TRUSS))
    ctop = (0.14, 2.62, 0.10)
    lobes.append(Lobe("lobe-s0-top", "f-s0-top", ctop, (0.90, 0.42, 0.85), "leaf-sun", SUN, 0.52, k=9, amp=0.06, phase=0.5, texture=True))
    lobes.append(Lobe("lobe-core", "f-core", (0.0, 1.50, 0.0), (1.20, 1.00, 1.10), "leaf-shade", SHADE, 0.38, k=8, amp=0.04, phase=1.0, hatch="shade-lines"))
    for i, (az, el) in enumerate([(0, 80), (90, 60), (180, 62), (270, 58)]):
        fl.append(truss("truss-s0-%d" % (i + 1), "f-s0-top", ctop, (0.90, 0.42, 0.85), (1.0, 0.0), az, el, TRUSS))
    return {"collar": (0.32, 0.24), "caps": caps, "lobes": lobes + fl, "texture": True, "thermal_only": emitter_solids(1.0), "support_r": 0.6}


def scale100():
    lobes, caps, fl = [], stem_caps(), []
    for sid, a, R, H, ph in STEMS:
        t, d = stem_pts(sid)[3], dir_of(sid)
        top = at(t, d, 0.10, 0.15)
        rt = (0.565, 0.51, 0.53)
        lobes.append(Lobe("lobe-%s-top" % sid, "f-%s-top" % sid, top, rt, "leaf-sun", SUN, 0.5, k=5, amp=0.05, phase=ph + 0.3, texture=True))
        mid = polar(a + 9, 1.20, 1.42)
        lobes.append(Lobe("lobe-%s-mid" % sid, "f-%s-mid" % sid, mid, (0.53, 0.56, 0.50), "leaf-sun", SUN, 0.48, k=5, amp=0.05, phase=ph + 1.2))
        skirt = polar(a + 6, 1.06, 0.68)
        lobes.append(Lobe("lobe-%s-skirt" % sid, "f-%s-skirt" % sid, skirt, (0.53, 0.45, 0.50), "leaf-sun", SUN, 0.46, k=4, amp=0.05, phase=ph + 1.8))
        for i, (az, el) in enumerate([(-25, 40), (30, 50)]):
            fl.append(truss("truss-%s-%d" % (sid, i + 1), "f-%s-top" % sid, top, rt, d, az, el, (0.09, 0.075, 0.09)))
        fl.append(truss("truss-%s-mid" % sid, "f-%s-mid" % sid, mid, (0.53, 0.56, 0.50), d, 0, 18, (0.09, 0.075, 0.09)))
    ctop = (0.14, 2.62, 0.10)
    lobes.append(Lobe("lobe-s0-top", "f-s0-top", ctop, (0.92, 0.43, 0.87), "leaf-sun", SUN, 0.52, k=7, amp=0.05, phase=0.8, texture=True))
    lobes.append(Lobe("lobe-core", "f-core", (0.0, 1.50, 0.0), (1.21, 1.01, 1.11), "leaf-shade", SHADE, 0.38, k=6, amp=0.04, phase=1.4, hatch="shade-lines"))
    for i, (az, el) in enumerate([(30, 78), (150, 60), (270, 60)]):
        fl.append(truss("truss-s0-%d" % (i + 1), "f-s0-top", ctop, (0.92, 0.43, 0.87), (1.0, 0.0), az, el, (0.09, 0.075, 0.09)))
    return {"collar": (0.32, 0.24), "caps": caps, "lobes": lobes + fl, "texture": True, "thermal_only": emitter_solids(0.94), "support_r": 0.59}


def scale200():
    lobes, caps, fl = [], stem_caps(segs=(0,), thermal=(1, 2)), []
    for sid, a, R, H, ph in STEMS:
        t, d = stem_pts(sid)[3], dir_of(sid)
        up = polar(a + 10, 1.135, 2.28)
        lobes.append(Lobe("lobe-%s-up" % sid, "f-%s-top" % sid, up, (0.57, 0.56, 0.55), "leaf-sun", SUN, 0.5, k=4, amp=0.05, phase=ph))
        lo = polar(a + 8, 1.145, 1.06)
        lobes.append(Lobe("lobe-%s-low" % sid, "f-%s-mid" % sid, lo, (0.55, 0.82, 0.53), "leaf-sun", SUN, 0.47, k=4, amp=0.05, phase=ph + 1.0))
        fl.append(truss("truss-%s" % sid, "f-%s-top" % sid, up, (0.57, 0.56, 0.55), d, 0, 42, (0.10, 0.085, 0.10)))
    ctop = (0.14, 2.62, 0.10)
    lobes.append(Lobe("lobe-s0-top", "f-s0-top", ctop, (0.94, 0.44, 0.88), "leaf-sun", SUN, 0.52, k=5, amp=0.05, phase=0.2))
    lobes.append(Lobe("lobe-core", "f-core", (0.0, 1.52, 0.0), (1.22, 1.03, 1.12), "leaf-shade", SHADE, 0.38, k=5, amp=0.04, phase=0.6))
    for i, (az, el) in enumerate([(0, 80), (120, 55), (240, 55)]):
        fl.append(truss("truss-s0-%d" % (i + 1), "f-s0-top", ctop, (0.94, 0.44, 0.88), (1.0, 0.0), az, el, (0.10, 0.085, 0.10)))
    return {"collar": (0.31, 0.24), "caps": caps, "lobes": lobes + fl, "texture": False, "thermal_only": emitter_solids(0.88), "support_r": 0.58}


def scale500():
    merged = ["leaf-sun", "leaf-shade", "flowers"]
    lobes = [
        Lobe("dome-front", "f-s1-mid", (0.52, 1.55, 0.0), (1.17, 1.30, 1.645), "leaf-sun", SUN, 0.5, k=3, amp=0.03, phase=0.4),
        Lobe("dome-back", "f-s4-mid", (-0.52, 1.55, 0.0), (1.17, 1.30, 1.645), "leaf-sun", SUN, 0.5, k=3, amp=0.03, phase=1.4),
        Lobe("dome-top", "f-s0-top", (0.10, 2.58, 0.06), (1.05, 0.48, 1.05), "leaf-sun", SUN, 0.5, k=3, amp=0.03, phase=2.4),
    ]
    for lb in lobes:
        lb.regions = merged
    shade = Lobe("th-core", "f-core", (0.0, 1.50, 0.0), (1.1, 0.95, 1.0), "leaf-shade", SHADE, 0.38)
    fl = [Lobe("th-truss-%d" % (i + 1), "f-s0-top", (0.10 + 0.55 * math.cos(a), 2.85, 0.06 + 0.55 * math.sin(a)), (0.09, 0.08, 0.09), "flowers", FLOWER, 0.84) for i, a in enumerate([0.4, 2.0, 3.6, 5.1])]
    th = []
    for sid in ["s0"] + [s[0] for s in STEMS]:
        p = stem_pts(sid)
        d = D_C if sid == "s0" else D_OUT
        th.append(Cap("bark-%s" % sid, "%s-b" % sid, p[1], p[3], d[1] / 2, d[2] / 2, "stem-bark", BARK, 0.3, role="thermal-only"))
    return {"collar": (0.30, 0.46, 0.36), "caps": th, "lobes": lobes, "texture": False, "thermal_only": [shade] + fl + emitter_solids(0.82), "support_r": 0.57}


SCALES = {"50": scale50(), "100": scale100(), "200": scale200(), "500": scale500()}
CROWN_PROJ = math.pi * 1.75 * 1.68
LEAF = {"leaf-sun": round(2.4 * CROWN_PROJ, 1), "leaf-shade": round(2.0 * CROWN_PROJ, 1)}


def frost_offsets(sp, rig):
    o = {}
    for id in rig.order:
        b = rig.bones[id]
        if b.kind == "foliage" and id not in ("f-s0-top", "f-core"):
            k = 3.0 if id.endswith("-skirt") else 6.0
            o[id] = -k if math.cos(math.radians(b.ang)) > 0 else k
    return o


SP = {
    "id": ID,
    "title": T("Rhododendron, Evergreen Dome", "Rhododendron, immergrüne Kuppel"),
    "summary": T("A mature evergreen rhododendron (Rhododendron ponticum hybrid) grown as a dense dome 3 m high and 3.5 m wide on nine crooked stems; leathery dark leaves clothe it to 25 cm above the ground, and in May domed trusses of funnel-shaped flowers stand at every shoot tip.",
                 "Ein ausgewachsener immergrüner Rhododendron (Rhododendron-ponticum-Hybride), gewachsen als dichte Kuppel von 3 m Höhe und 3,5 m Breite auf neun knorrigen Stämmen; ledrige dunkle Blätter bekleiden ihn bis 25 cm über dem Boden, und im Mai stehen an jeder Triebspitze gewölbte Stände trichterförmiger Blüten."),
    "tags": ["shrub", "evergreen", "flowering-shrub", "dome", "garden", "trusses", "wind", "frost", "transpiration"],
    "distinction": T("A dense evergreen dome shrub clothed almost to the ground with flower trusses on the shoot tips, whose late-frost pose lets the leathery leaves droop and close their stomata.",
                     "Ein dichter immergrüner Kuppelstrauch, fast bis zum Boden belaubt, mit Blütenständen an den Triebspitzen; in der Spätfrostpose hängen die ledrigen Blätter und schließen ihre Spaltöffnungen."),
    "references": refs.common + [refs.rhs("rhs-rhododendron-ponticum", "Rhododendron ponticum and its hardy hybrids (for example 'Roseum Elegans', 'Cunningham's White')", "evergreen, eventual height 2.5 to 4 m and spread 2.5 to 4 m for the garden hybrids, flowers in late spring"), refs.hillier, refs.koerner, refs.larcher],
    "notes": T("Rig, stems, leaf masses and trusses are authored by hand for a 30-year-old garden hybrid of the ponticum group: nine crooked stems of 7.5 to 8.5 cm at the base that branch low, dome 3.5 m by 3.4 m and 3.0 m high, inside the RHS band of the hardy hybrids. About 150 trusses of about 60 g are an authored estimate; the drawn trusses are a sample of them, not a count. The species itself becomes larger and is invasive in parts of Britain and Ireland; the drawn plant is a garden hybrid.",
               "Rig, Stämme, Blattmassen und Blütenstände sind von Hand für eine 30-jährige Gartenhybride der Ponticum-Gruppe entworfen: neun knorrige, tief verzweigte Stämme von 7,5 bis 8,5 cm am Fuß, Kuppel 3,5 m mal 3,4 m und 3,0 m hoch, innerhalb der RHS-Spanne der winterharten Hybriden. Etwa 150 Blütenstände zu je etwa 60 g sind eine Autorenschätzung; die gezeichneten Stände sind eine Auswahl, keine Zählung. Die Art selbst wird größer und ist in Teilen Großbritanniens und Irlands invasiv; die gezeichnete Pflanze ist eine Gartenhybride."),
    "wood": WOOD, "bark": BARK, "density": 900, "leaf_ref": "jones-2014", "leaf_kg_m2": 0.35, "twig_factor": 1.5, "flower_kg": 0.06, "flowers_per_drawn": round(150 / 52, 3),
    "leaf_area": LEAF, "porosity": 0.9, "cd": 0.5, "trunk": "s0-a", "footprint": 0.9, "foliage_len": 0.1,
    "collar": {"h": 0.15, "r1": 0.24, "kg": 25, "title": T("Root crown of the stem cluster", "Wurzelstock der Stammgruppe")},
    "stems": [{"id": sid, "title": T("Central stem" if sid == "s0" else "Stem %s" % NAMES[sid][0], "Mittelstamm" if sid == "s0" else "Stamm %s" % NAMES[sid][1]), "pts": stem_pts(sid),
               "d": D_C if sid == "s0" else D_OUT, "limits": [5, 7, 9]} for sid in ["s0"] + [s[0] for s in STEMS]],
    "foliage": FOLIAGE,
    "scales": SCALES,
    "texture": {"50": LEAF_TEX, "100": [(-0.32, 0.20, 15, 0.14, 0.05), (0.28, 0.28, -12, 0.13, 0.05), (0.0, -0.30, 25, 0.14, 0.05)]},
    "materials": [
        {"id": BARK, "title": T("Rhododendron bark, red-brown and flaking", "Rhododendronrinde, rotbraun und schuppig"), "emissivity": 0.95, "solarAbsorptivity": 0.7, "densityKgM3": 600, "conductivityWmK": 0.12, "source": "authored-estimate", "reference": "wood-handbook-2021"},
        {"id": WOOD, "title": T("Green rhododendron wood (hard, crooked)", "Frisches Rhododendronholz (hart, knorrig)"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": 900, "conductivityWmK": 0.16, "elasticModulusPa": 6000000000, "strengthPa": 45000000, "source": "authored-estimate", "reference": "wood-handbook-2021"},
        {"id": SUN, "title": T("Rhododendron sun leaves (leathery, dark glossy)", "Rhododendron-Sonnenblätter (ledrig, dunkel glänzend)"), "emissivity": 0.97, "solarAbsorptivity": 0.55, "source": "literature", "reference": "jones-2014"},
        {"id": SHADE, "title": T("Rhododendron shade leaves (effective)", "Rhododendron-Schattenblätter (wirksam)"), "emissivity": 0.97, "solarAbsorptivity": 0.3, "source": "authored-estimate", "reference": "jones-2014"},
        {"id": FLOWER, "title": T("Rhododendron corollas (mauve funnels)", "Rhododendronkronen (malvenfarbene Trichter)"), "emissivity": 0.96, "solarAbsorptivity": 0.45, "source": "authored-estimate", "reference": "jones-2014"},
    ],
    "regions": [
        {"id": "stem-bark", "title": T("Bark of the crooked stems and the root crown", "Rinde der knorrigen Stämme und des Wurzelstocks"), "kind": "bark", "model": "passive", "material": BARK, "len": 0.05, "geom": "cylinder", "sky": 0.1, "hc": 35000, "ref": "wood-handbook-2021", "sun": 0.05},
        {"id": "leaf-sun", "title": T("Sun leaves of the dome surface", "Sonnenblätter der Kuppeloberfläche"), "kind": "foliage", "model": "leaf", "material": SUN, "area": LEAF["leaf-sun"], "exchange": True, "len": 0.05, "geom": "vertical-plate", "sky": 0.7, "hc": 1500, "gs": 0.004, "lai": 2.4, "ref": "koerner-1979", "sun": 0.85},
        {"id": "leaf-shade", "title": T("Shade leaves inside the dome", "Schattenblätter im Inneren der Kuppel"), "kind": "foliage", "model": "leaf", "material": SHADE, "area": LEAF["leaf-shade"], "exchange": True, "len": 0.05, "geom": "vertical-plate", "sky": 0.2, "hc": 1300, "gs": 0.0025, "lai": 2.0, "ref": "koerner-1979", "sun": 0.08},
        {"id": "flowers", "title": T("Corollas of the flower trusses", "Blütenkronen der Blütenstände"), "kind": "surface", "model": "passive", "material": FLOWER, "area": 9.0, "exchange": True, "len": 0.05, "geom": "sphere", "sky": 0.75, "hc": 1200, "ref": "jones-2014", "sun": 0.9},
    ],
    "emitters": [
        {"id": "transpiration-sun", "aid": "tp-sun", "title": T("Transpiration of the sun leaves", "Transpiration der Sonnenblätter"), "region": "leaf-sun", "diam": 1.0, "strength": 1.0,
         "anchors": [("f-s1-top", None), ("f-s3-top", None), ("f-s5-top", None), ("f-s7-top", None), ("f-s2-mid", None), ("f-s6-mid", None), ("f-s0-top", None)]},
        {"id": "transpiration-shade", "aid": "tp-shade", "title": T("Transpiration of the shade leaves", "Transpiration der Schattenblätter"), "region": "leaf-shade", "diam": 1.2, "strength": 0.5,
         "anchors": [("f-core", (0.6, 1.5, 0.5)), ("f-core", (-0.6, 1.5, -0.5)), ("f-core", (0.0, 1.9, 0.0)), ("f-core", (0.5, 1.2, -0.6)), ("f-core", (-0.5, 1.2, 0.6))]},
    ],
    "environment": {"airTempC": 17, "meanRadiantTempC": 21, "relativeHumidity": 0.65, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 600, "pressurePa": 101325, "diffuseFraction": 0.25},
    "lean_ref": {"root": 0.0, "stem": 0.5, "stemtop": 1.0, "fol": 2.2},
    "sway_amp": {"root": 0.0, "stem": 0.45, "stemtop": 0.55, "fol": 0.65},
    "sway_lag": 0.4, "sway_period": 1.2,
    "gust_tau": {"root": 0.5, "stem": 0.4, "stemtop": 0.3, "fol": 0.2},
    "sway_desc": ("A deterministic %s s sway cycle about the breeze lean of the flowering rhododendron (5.5 m/s at 10 m; estimated first mode of the dense dome about 0.8 Hz). The stiff stems barely move, the leaf masses on the shoot tips rock about 1.5 degrees behind them.",
                  "Ein deterministischer Schwingungszyklus von %s s um die Neigung des blühenden Rhododendrons bei Brise (5,5 m/s in 10 m Höhe; geschätzte erste Eigenform der dichten Kuppel etwa 0,8 Hz). Die steifen Stämme bewegen sich kaum, die Blattmassen an den Triebspitzen schaukeln etwa 1,5 Grad hinterher."),
    "gust_desc": ("A single gust over 8 s from %.1f to %.1f m/s at 10 m: the wind-load tracks rise with the reconfigured drag law, the shoot masses lean first and the stiff stems follow with a level-wise lag.",
                  "Eine einzelne Böe über 8 s von %.1f auf %.1f m/s in 10 m Höhe: Die Windlast-Spuren steigen mit dem umgeformten Widerstandsgesetz, zuerst neigen sich die Triebmassen, die steifen Stämme folgen ebenenweise verzögert."),
    "breeze_pose": "flowering-breeze", "breeze_v10": 5.5, "gust_peak": 16.0, "default_pose": "flowering-calm", "default_scale": 100,
    "symbol_crown_base": 0.25, "symbol_k": 9, "symbol_stem": (0.3, 0.46),
}

SP["poses"] = [
    {"id": "flowering-calm", "title": T("Full bloom, calm", "Vollblüte, windstill"), "desc": T("A still mid-May noon (1.5 m/s at 10 m) in full bloom: about 150 trusses crown the dome, the leathery leaves transpire at 80 % of their low maximum.",
                                                                                                "Ein stiller Mittag Mitte Mai (1,5 m/s in 10 m Höhe) in Vollblüte: etwa 150 Blütenstände krönen die Kuppel, die ledrigen Blätter transpirieren mit 80 % ihres niedrigen Maximums."), "v10": 1.5, "emit": 0.8},
    {"id": "flowering-breeze", "title": T("Full bloom in a breeze", "Vollblüte bei Brise"), "desc": T("A fresh breeze of 5.5 m/s at 10 m: the dense dome hardly yields, the shoot masses lean about half a degree downwind.",
                                                                                                  "Eine frische Brise von 5,5 m/s in 10 m Höhe: Die dichte Kuppel gibt kaum nach, die Triebmassen neigen sich etwa ein halbes Grad in Windrichtung."), "v10": 5.5, "cd": 0.45},
    {"id": "storm", "title": T("Gale", "Sturm"), "desc": T("A gale of 20 m/s at 10 m under an overcast sky: the stiff leaves turn edge-on and the drag coefficient drops to 0.3 (reconfiguration after Vogel 1989); the stems lean well under 1 degree, the shoot masses about 2.5 degrees.",
                                                           "Ein Sturm von 20 m/s in 10 m Höhe unter bedecktem Himmel: Die steifen Blätter drehen sich in den Wind, der Widerstandsbeiwert sinkt auf 0,3 (Umformung nach Vogel 1989); die Stämme neigen sich deutlich unter 1 Grad, die Triebmassen etwa 2,5 Grad."), "v10": 20.0, "cd": 0.3, "sun": 0.3, "emit": 0.3},
    {"id": "late-frost", "title": T("Late frost in bloom", "Spätfrost in der Blüte"), "desc": T("A clear night of late frost in May at about -5 °C (pair it with the environment preset winter-clear-night): the leaves roll and hang down by thermonasty, so the shoot masses drop 6 degrees and the skirt 3 degrees; the stomata are closed and nothing is sunlit; the open flowers are frost-bitten.",
                                                                                                   "Eine klare Spätfrostnacht im Mai bei etwa -5 °C (zusammen mit der Umgebungsvorgabe winter-clear-night zu lesen): Die Blätter rollen sich und hängen durch Thermonastie herab, daher sinken die Triebmassen um 6 Grad und der Saum um 3 Grad; die Spaltöffnungen sind geschlossen, nichts ist besonnt; die offenen Blüten erfrieren."), "v10": 0.5, "sun": 0.0, "emit": 0.02,
     "sunlit": {"stem-bark": 0.0, "leaf-sun": 0.0, "leaf-shade": 0.0, "flowers": 0.0}, "offsets": frost_offsets},
]
SP["thermal_assumptions"] = refs.thermal_assumptions("rhododendron", ("Flowers are a passive exchange surface: the corollas of about 150 trusses (9 m2, an authored estimate) take part in radiation and convection but are not given a stomatal model. The leaves are evergreen and thick: their areal heat capacity of 1.3 to 1.5 kJ/(m2 K) is about twice that of thin deciduous leaves.",
                                                                       "Blüten sind eine passive Austauschfläche: Die Blütenkronen von etwa 150 Ständen (9 m2, eine Autorenschätzung) nehmen an Strahlung und Konvektion teil, erhalten aber kein Stomatamodell. Die Blätter sind immergrün und dick: Ihre flächenbezogene Wärmekapazität von 1,3 bis 1,5 kJ/(m2 K) ist etwa doppelt so groß wie die dünner sommergrüner Blätter."))
SP["structure_assumptions"] = refs.structure_assumptions("nine stems", "neun Stämme")
SP["decisions"] = refs.decisions_rhododendron()
SP["volume_decisions"] = refs.volume_decisions_rhododendron()

if __name__ == "__main__":
    tops = {lb.bone: lb.c for lb in SCALES["50"]["lobes"] if lb.id.endswith("-top") or lb.id.endswith("-mid")}
    for e in SP["emitters"]:
        e["anchors"] = [(b, p if p is not None else tops[b]) for b, p in e["anchors"]]
    doc, path, rig = assemble.build(SP)
    assemble.report(doc, rig, SP)
    print("[DEBUG] wrote", path)
