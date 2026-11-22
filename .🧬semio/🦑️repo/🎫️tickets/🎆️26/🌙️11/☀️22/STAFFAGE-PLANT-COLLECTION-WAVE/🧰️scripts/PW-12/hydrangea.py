"""💙️ hydrangea: a mature bigleaf hydrangea (Hydrangea macrophylla, mophead) of nine arching canes around a central one, a dense mound
of broad leaves to the ground and about thirty mophead flower heads on the cane tips (default pose: full bloom in July). Every coordinate
below is authored by hand in world rest metres (x ahead, y up, z left); `kit.py` converts and projects, `assemble.py` builds the document.

@see ./kit.py
@see ./assemble.py"""
import math

from kit import T, Lobe, Cap
import assemble
import refs

ID = "hydrangea"
BARK, WOOD = "hydrangea-bark", "hydrangea-wood"
SUN, SHADE, FLOWER = "hydrangea-leaf-sun", "hydrangea-leaf-shade", "hydrangea-sepal"

CANES = [
    ("c1", 10, 0.55, 1.12, 0.0), ("c2", 50, 0.53, 1.16, 0.3), ("c3", 90, 0.56, 1.10, 0.9), ("c4", 130, 0.54, 1.14, 1.4), ("c5", 170, 0.55, 1.12, 2.0),
    ("c6", 210, 0.53, 1.15, 2.6), ("c7", 250, 0.56, 1.11, 3.1), ("c8", 290, 0.54, 1.13, 3.7), ("c9", 330, 0.55, 1.15, 4.3),
]
NAMES = {"c1": ("front", "vorn"), "c2": ("front left", "vorn links"), "c3": ("left", "links"), "c4": ("back left", "hinten links"), "c5": ("back", "hinten"),
         "c6": ("back right", "hinten rechts"), "c7": ("right", "rechts"), "c8": ("front right", "vorn rechts"), "c9": ("front, slightly right", "vorn, leicht rechts")}


def polar(a, rad, y):
    return (rad * math.cos(math.radians(a)), y, rad * math.sin(math.radians(a)))


def cane_pts(cid):
    if cid == "c0":
        return [(0.02, 0.06, 0.0), (0.05, 0.75, 0.03), (0.08, 1.25, 0.05)]
    _, a, R, H, _ = [c for c in CANES if c[0] == cid][0]
    return [polar(a, 0.07, 0.06), polar(a + 3, 0.30, 0.72), polar(a + 6, R, H)]


def dir_of(cid):
    _, a, _, _, _ = [c for c in CANES if c[0] == cid][0]
    return (math.cos(math.radians(a + 6)), math.sin(math.radians(a + 6)))


def along(cid, seg, t):
    p = cane_pts(cid)
    a, b = p[seg], p[seg + 1]
    return tuple(a[k] + (b[k] - a[k]) * t for k in range(3))


def at(c, d, k, dy):
    return (c[0] + k * d[0], c[1] + dy, c[2] + k * d[1])


FOLIAGE = []
for cid, a, *_ in CANES:
    en, de = NAMES[cid]
    ang = 0 if math.cos(math.radians(a + 6)) >= 0 else 180
    FOLIAGE.append({"id": "f-%s-top" % cid, "parent": "%s-b" % cid, "at": 1.0, "ang": ang, "title": T("Flowering cane top %s" % en, "Blühende Rutenspitze %s" % de)})
    FOLIAGE.append({"id": "f-%s-low" % cid, "parent": "%s-a" % cid, "at": 0.85, "ang": ang, "title": T("Lower leaf mass %s" % en, "Untere Blattmasse %s" % de)})
FOLIAGE += [
    {"id": "f-c0-top", "parent": "c0-b", "at": 1.0, "ang": 0, "title": T("Mound top with flower heads", "Kuppe mit Blütenbällen")},
    {"id": "f-core", "parent": "c0-a", "at": 0.5, "ang": 0, "title": T("Shaded core of the mound", "Beschatteter Kern des Hügels")},
]
HEAD = (0.10, 0.085, 0.10)
LEAF_TEX = [(-0.36, 0.22, 30, 0.07, 0.10), (0.20, 0.38, -20, 0.065, 0.10), (0.42, -0.05, 50, 0.06, 0.10), (-0.12, -0.30, -35, 0.07, 0.10), (0.05, 0.05, 10, 0.06, 0.08)]


def head(id, bone, c, rad, d, az, el, size, k=0.92):
    """🔵️ A mophead: a ball of sterile florets whose centre sits on the leaf mass at the given azimuth (from the outward direction) and elevation."""
    a = math.atan2(d[1], d[0]) + math.radians(az)
    e = math.radians(el)
    n = (math.cos(e) * math.cos(a), math.sin(e), math.cos(e) * math.sin(a))
    cc = (c[0] + n[0] * rad[0] * k, c[1] + n[1] * rad[1] * k, c[2] + n[2] * rad[2] * k)
    return Lobe(id, bone, cc, size, "flowers", FLOWER, 0.86, k=11, amp=0.10, phase=az * 0.01)


def cane_caps(lower=True, upper=True, thermal_upper=False):
    out = []
    for cid in ["c0"] + [c[0] for c in CANES]:
        p = cane_pts(cid)
        d = [0.026, 0.019, 0.012] if cid == "c0" else [0.024, 0.017, 0.011]
        if lower:
            out.append(Cap("%s-a" % cid, "%s-a" % cid, p[0], p[1], d[0] / 2, d[1] / 2, "stem-bark", BARK, 0.38))
        if upper:
            out.append(Cap("%s-b" % cid, "%s-b" % cid, p[1], p[2], d[1] / 2, d[2] / 2, "stem-bark", BARK, 0.38))
        elif thermal_upper:
            out.append(Cap("%s-b" % cid, "%s-b" % cid, p[1], p[2], d[1] / 2, d[2] / 2, "stem-bark", BARK, 0.38, role="thermal-only"))
    return out


def emitter_solids(k):
    sun = Lobe("emit-sun", "f-c0-top", (0.08, 1.28, 0.05), (0.16 * k, 0.10 * k, 0.16 * k), "leaf-sun", SUN, 0.6, role="detail")
    sun.tags = ["transpiration-sun"]
    shade = Lobe("emit-shade", "f-core", (0.0, 0.62, 0.0), (0.22 * k, 0.16 * k, 0.22 * k), "leaf-shade", SHADE, 0.45, role="detail")
    shade.tags = ["transpiration-shade"]
    return [sun, shade]


def scale50():
    lobes, caps = [], cane_caps()
    heads = []
    for cid, a, R, H, ph in CANES:
        t, d = cane_pts(cid)[2], dir_of(cid)
        top = at(t, d, 0.06, 0.04)
        rt = (0.34, 0.30, 0.34)
        lobes.append(Lobe("lobe-%s-top" % cid, "f-%s-top" % cid, top, rt, "leaf-sun", SUN, 0.6, k=6, amp=0.08, phase=ph, texture=True))
        low = polar(a + 4, 0.52, 0.48)
        lobes.append(Lobe("lobe-%s-low" % cid, "f-%s-low" % cid, low, (0.30, 0.30, 0.30), "leaf-sun", SUN, 0.56, k=5, amp=0.08, phase=ph + 0.7, texture=True))
        for i, (az, el) in enumerate([(-28, 48), (30, 30)]):
            heads.append(head("head-%s-%d" % (cid, i + 1), "f-%s-top" % cid, top, rt, d, az, el, HEAD))
    ctop = (0.08, 1.30, 0.05)
    lobes.append(Lobe("lobe-c0-top", "f-c0-top", ctop, (0.40, 0.24, 0.40), "leaf-sun", SUN, 0.62, k=7, amp=0.08, phase=0.5, texture=True))
    lobes.append(Lobe("lobe-core", "f-core", (0.0, 0.62, 0.0), (0.55, 0.42, 0.52), "leaf-shade", SHADE, 0.42, k=6, amp=0.05, phase=1.0, hatch="shade-lines"))
    for i, (az, el) in enumerate([(40, 75), (220, 70)]):
        heads.append(head("head-c0-%d" % (i + 1), "f-c0-top", ctop, (0.40, 0.24, 0.40), (1.0, 0.0), az, el, HEAD))
    return {"collar": (0.24, 0.17), "caps": caps, "lobes": lobes + heads, "texture": True, "thermal_only": emitter_solids(1.0), "support_r": 0.4}


def scale100():
    lobes, caps = [], cane_caps()
    heads = []
    for cid, a, R, H, ph in CANES:
        t, d = cane_pts(cid)[2], dir_of(cid)
        top = at(t, d, 0.06, 0.05)
        rt = (0.35, 0.31, 0.34)
        lobes.append(Lobe("lobe-%s-top" % cid, "f-%s-top" % cid, top, rt, "leaf-sun", SUN, 0.6, k=4, amp=0.07, phase=ph + 0.3, texture=True))
        low = polar(a + 4, 0.52, 0.48)
        lobes.append(Lobe("lobe-%s-low" % cid, "f-%s-low" % cid, low, (0.31, 0.30, 0.30), "leaf-sun", SUN, 0.56, k=4, amp=0.07, phase=ph + 1.1))
        heads.append(head("head-%s" % cid, "f-%s-top" % cid, top, rt, d, 0, 40, (0.105, 0.09, 0.105)))
    ctop = (0.08, 1.30, 0.05)
    lobes.append(Lobe("lobe-c0-top", "f-c0-top", ctop, (0.41, 0.25, 0.41), "leaf-sun", SUN, 0.62, k=5, amp=0.07, phase=0.8, texture=True))
    lobes.append(Lobe("lobe-core", "f-core", (0.0, 0.62, 0.0), (0.56, 0.43, 0.53), "leaf-shade", SHADE, 0.42, k=5, amp=0.05, phase=1.4, hatch="shade-lines"))
    for i, (az, el) in enumerate([(40, 75), (220, 70)]):
        heads.append(head("head-c0-%d" % (i + 1), "f-c0-top", ctop, (0.41, 0.25, 0.41), (1.0, 0.0), az, el, (0.105, 0.09, 0.105)))
    return {"collar": (0.24, 0.17), "caps": caps, "lobes": lobes + heads, "texture": True, "thermal_only": emitter_solids(0.94), "support_r": 0.39}


def scale200():
    lobes, caps = [], cane_caps(upper=False, thermal_upper=True)
    heads = []
    for n, (cid, a, R, H, ph) in enumerate(CANES):
        t, d = cane_pts(cid)[2], dir_of(cid)
        top = at(t, d, 0.05, 0.05)
        rt = (0.35, 0.32, 0.355)
        lobes.append(Lobe("lobe-%s-top" % cid, "f-%s-top" % cid, top, rt, "leaf-sun", SUN, 0.58, k=3, amp=0.05, phase=ph))
        low = polar(a + 4, 0.52, 0.49)
        lobes.append(Lobe("lobe-%s-low" % cid, "f-%s-low" % cid, low, (0.31, 0.31, 0.30), "leaf-sun", SUN, 0.55, k=3, amp=0.05, phase=ph + 1.0))
        if n % 2 == 0:
            heads.append(head("head-%s" % cid, "f-%s-top" % cid, top, rt, d, 0, 40, (0.11, 0.095, 0.11)))
    ctop = (0.08, 1.30, 0.05)
    lobes.append(Lobe("lobe-c0-top", "f-c0-top", ctop, (0.42, 0.25, 0.42), "leaf-sun", SUN, 0.6, k=4, amp=0.05, phase=0.2))
    lobes.append(Lobe("lobe-core", "f-core", (0.0, 0.64, 0.0), (0.57, 0.44, 0.54), "leaf-shade", SHADE, 0.42, k=4, amp=0.04, phase=0.6))
    heads.append(head("head-c0", "f-c0-top", ctop, (0.42, 0.25, 0.42), (1.0, 0.0), 0, 68, (0.11, 0.095, 0.11)))
    return {"collar": (0.23, 0.17), "caps": caps, "lobes": lobes + heads, "texture": False, "thermal_only": emitter_solids(0.88), "support_r": 0.38}


def scale500():
    merged = ["leaf-sun", "leaf-shade", "flowers"]
    lobes = [
        Lobe("mound", "f-core", (0.0, 0.85, 0.0), (0.94, 0.66, 0.955), "leaf-sun", SUN, 0.6, k=3, amp=0.03, phase=0.3),
        Lobe("mound-top", "f-c0-top", (0.05, 1.30, 0.04), (0.56, 0.285, 0.53), "leaf-sun", SUN, 0.6, k=3, amp=0.03, phase=1.3),
    ]
    for lb in lobes:
        lb.regions = merged
    shade = Lobe("th-core", "f-core", (0.0, 0.66, 0.0), (0.5, 0.4, 0.48), "leaf-shade", SHADE, 0.42)
    fl = [Lobe("th-head-%d" % (i + 1), "f-c0-top", (0.05 + 0.4 * math.cos(a), 1.36, 0.04 + 0.4 * math.sin(a)), (0.1, 0.085, 0.1), "flowers", FLOWER, 0.86) for i, a in enumerate([0.3, 1.9, 3.5, 5.0])]
    caps = cane_caps(lower=False, upper=False)
    th = [Cap(c.id, c.bone, c.p0, c.p1, c.r0, c.r1, c.region, c.material, c.tone, role="thermal-only") for c in cane_caps()]
    return {"collar": (0.17, 0.22, 0.22), "caps": caps + th, "lobes": lobes, "texture": False, "thermal_only": [shade] + fl + emitter_solids(0.82), "support_r": 0.37}


SCALES = {"50": scale50(), "100": scale100(), "200": scale200(), "500": scale500()}
CROWN_PROJ = math.pi * 0.95 * 0.92
LEAF = {"leaf-sun": round(2.0 * CROWN_PROJ, 1), "leaf-shade": round(1.2 * CROWN_PROJ, 1)}


def rain_offsets(sp, rig):
    o = {}
    for id in rig.order:
        b = rig.bones[id]
        out = math.cos(math.radians(b.ang)) > 0.05
        inn = math.cos(math.radians(b.ang)) < -0.05
        if b.kind == "wood" and not id.startswith("c0"):
            if out or inn:
                o[id] = (-1 if out else 1) * (2.0 if b.level == "stem" else 4.5)
        elif b.kind == "foliage" and id.endswith("-top") and id != "f-c0-top":
            o[id] = -7.0 if math.cos(math.radians(b.ang)) > 0 else 7.0
    return o


def rain_loads(sp, rig):
    per = {}
    for id, la in sp["_leaf_of"].items():
        fl = len([lb for lb in SCALES["50"]["lobes"] if lb.bone == id and lb.region == "flowers"])
        per[id] = la * 0.2 + fl * sp["flowers_per_drawn"] * 0.15
    return assemble.carried(sp, rig, "rain", per, ("Intercepted rain", "Zurückgehaltener Regen"), "keim-2006")


SP = {
    "id": ID,
    "title": T("Bigleaf Hydrangea, Mophead", "Bauernhortensie, Ballhortensie"),
    "summary": T("A mature bigleaf hydrangea (Hydrangea macrophylla) of the mophead type: ten canes arch out of the crown into a dense mound 1.5 m high and 1.9 m wide, clothed to the ground in broad toothed leaves and topped in summer by about thirty ball-shaped flower heads.",
                 "Eine ausgewachsene Bauernhortensie (Hydrangea macrophylla) vom Typ Ballhortensie: zehn Ruten wölben sich aus dem Wurzelstock zu einem dichten Hügel von 1,5 m Höhe und 1,9 m Breite, bis zum Boden mit breiten gezähnten Blättern bekleidet und im Sommer von etwa dreißig ballförmigen Blütenständen gekrönt."),
    "tags": ["shrub", "deciduous", "flowering-shrub", "mound", "garden", "mophead", "wind", "rain", "transpiration"],
    "distinction": T("A low mound shrub clothed to the ground whose ball-shaped mopheads soak up rain and pull the arching canes outwards in the rain pose.",
                     "Ein niedriger, bis zum Boden belaubter Hügelstrauch, dessen ballförmige Blütenstände Regen aufsaugen und in der Regenpose die gewölbten Ruten nach außen ziehen."),
    "references": refs.common + [refs.rhs("rhs-hydrangea-macrophylla", "Hydrangea macrophylla (bigleaf hydrangea, mophead)", "eventual height 1 to 1.5 m, spread 1.5 to 2.5 m, flowers mid to late summer"), refs.hillier, refs.koerner, refs.keim],
    "notes": T("Rig, canes, leaf masses and flower heads are authored by hand for a ten-year-old garden mophead: ten canes of 2.4 to 2.6 cm at the base, mound 1.9 m by 1.85 m and 1.55 m high with the heads, inside the RHS band of 1 to 1.5 m height and 1.5 to 2.5 m spread. About thirty heads of 20 cm (about 120 g each, an authored estimate) carry the bloom; the drawn heads are a sample of them, not a count.",
               "Rig, Ruten, Blattmassen und Blütenstände sind von Hand für eine zehnjährige Gartenballhortensie entworfen: zehn Ruten von 2,4 bis 2,6 cm am Fuß, Hügel 1,9 m mal 1,85 m und mit den Blütenbällen 1,55 m hoch, innerhalb der RHS-Spanne von 1 bis 1,5 m Höhe und 1,5 bis 2,5 m Breite. Etwa dreißig Bälle von 20 cm (je etwa 120 g, eine Autorenschätzung) tragen die Blüte; die gezeichneten Bälle sind eine Auswahl, keine Zählung."),
    "wood": WOOD, "bark": BARK, "density": 800, "leaf_ref": "jones-2014", "leaf_kg_m2": 0.15, "twig_factor": 1.4, "flower_kg": 0.12, "flowers_per_drawn": 1.5,
    "leaf_area": LEAF, "porosity": 0.75, "cd": 0.5, "trunk": "c0-a", "footprint": 0.5, "foliage_len": 0.06,
    "collar": {"h": 0.10, "r1": 0.17, "kg": 6, "title": T("Woody crown of the canes", "Verholzter Wurzelstock der Ruten")},
    "stems": [{"id": cid, "title": T("Central cane" if cid == "c0" else "Cane %s" % NAMES[cid][0], "Mittlere Rute" if cid == "c0" else "Rute %s" % NAMES[cid][1]), "pts": cane_pts(cid),
               "d": [0.026, 0.019, 0.012] if cid == "c0" else [0.024, 0.017, 0.011], "limits": [8, 10]} for cid in ["c0"] + [c[0] for c in CANES]],
    "foliage": FOLIAGE,
    "scales": SCALES,
    "texture": {"50": LEAF_TEX, "100": [(-0.3, 0.2, 30, 0.10, 0.10), (0.28, 0.25, -25, 0.09, 0.10), (0.0, -0.28, 40, 0.10, 0.08)]},
    "materials": [
        {"id": BARK, "title": T("Hydrangea bark, pale brown and peeling", "Hortensienrinde, hellbraun und abblätternd"), "emissivity": 0.95, "solarAbsorptivity": 0.6, "densityKgM3": 550, "conductivityWmK": 0.12, "source": "authored-estimate", "reference": "wood-handbook-2021"},
        {"id": WOOD, "title": T("Green hydrangea wood (pithy canes)", "Frisches Hortensienholz (markreiche Ruten)"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 800, "conductivityWmK": 0.15, "elasticModulusPa": 3000000000, "strengthPa": 25000000, "source": "authored-estimate", "reference": "wood-handbook-2021"},
        {"id": SUN, "title": T("Hydrangea sun leaves (broad, glossy)", "Hortensien-Sonnenblätter (breit, glänzend)"), "emissivity": 0.97, "solarAbsorptivity": 0.5, "source": "literature", "reference": "jones-2014"},
        {"id": SHADE, "title": T("Hydrangea shade leaves (effective)", "Hortensien-Schattenblätter (wirksam)"), "emissivity": 0.97, "solarAbsorptivity": 0.28, "source": "authored-estimate", "reference": "jones-2014"},
        {"id": FLOWER, "title": T("Mophead sepals (blue to pink)", "Kelchblätter der Blütenbälle (blau bis rosa)"), "emissivity": 0.96, "solarAbsorptivity": 0.45, "source": "authored-estimate", "reference": "jones-2014"},
    ],
    "regions": [
        {"id": "stem-bark", "title": T("Bark of the canes and the woody crown", "Rinde der Ruten und des Wurzelstocks"), "kind": "bark", "model": "passive", "material": BARK, "len": 0.02, "geom": "cylinder", "sky": 0.2, "hc": 15000, "ref": "wood-handbook-2021", "sun": 0.1},
        {"id": "leaf-sun", "title": T("Sun leaves of the outer mound", "Sonnenblätter der Hügeloberfläche"), "kind": "foliage", "model": "leaf", "material": SUN, "area": LEAF["leaf-sun"], "exchange": True, "len": 0.10, "geom": "vertical-plate", "sky": 0.7, "hc": 700, "gs": 0.008, "lai": 2.0, "ref": "koerner-1979", "sun": 0.85},
        {"id": "leaf-shade", "title": T("Shade leaves of the inner mound", "Schattenblätter im Inneren des Hügels"), "kind": "foliage", "model": "leaf", "material": SHADE, "area": LEAF["leaf-shade"], "exchange": True, "len": 0.11, "geom": "vertical-plate", "sky": 0.25, "hc": 600, "gs": 0.0045, "lai": 1.2, "ref": "koerner-1979", "sun": 0.1},
        {"id": "flowers", "title": T("Sepals of the mophead flower heads", "Kelchblätter der Blütenbälle"), "kind": "surface", "model": "passive", "material": FLOWER, "area": 3.6, "exchange": True, "len": 0.03, "geom": "sphere", "sky": 0.8, "hc": 1500, "ref": "jones-2014", "sun": 0.9},
    ],
    "emitters": [
        {"id": "transpiration-sun", "aid": "tp-sun", "title": T("Transpiration of the sun leaves", "Transpiration der Sonnenblätter"), "region": "leaf-sun", "diam": 0.6, "strength": 1.0,
         "anchors": [("f-c1-top", None), ("f-c3-top", None), ("f-c5-top", None), ("f-c7-top", None), ("f-c9-top", None), ("f-c0-top", None)]},
        {"id": "transpiration-shade", "aid": "tp-shade", "title": T("Transpiration of the shade leaves", "Transpiration der Schattenblätter"), "region": "leaf-shade", "diam": 0.7, "strength": 0.6,
         "anchors": [("f-core", (0.3, 0.62, 0.25)), ("f-core", (-0.3, 0.62, -0.25)), ("f-core", (0.0, 0.80, 0.0)), ("f-core", (0.25, 0.55, -0.3)), ("f-core", (-0.25, 0.55, 0.3))]},
    ],
    "environment": {"airTempC": 24, "meanRadiantTempC": 28, "relativeHumidity": 0.55, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 750, "pressurePa": 101325, "diffuseFraction": 0.15},
    "lean_ref": {"root": 0.0, "stem": 1.0, "stemtop": 2.0, "fol": 3.5},
    "sway_amp": {"root": 0.0, "stem": 0.5, "stemtop": 0.65, "fol": 0.75},
    "sway_lag": 0.4, "sway_period": 0.9,
    "gust_tau": {"root": 0.4, "stem": 0.3, "stemtop": 0.22, "fol": 0.15},
    "sway_desc": ("A deterministic %s s sway cycle about the breeze lean of the hydrangea in bloom (5.5 m/s at 10 m; estimated first mode of the arching canes about 1.1 Hz). The flower-laden cane tops nod behind the cane bases, the leaf masses flutter about 2 degrees.",
                  "Ein deterministischer Schwingungszyklus von %s s um die Neigung der blühenden Hortensie bei Brise (5,5 m/s in 10 m Höhe; geschätzte erste Eigenform der gewölbten Ruten etwa 1,1 Hz). Die blütenschweren Rutenspitzen nicken den Rutenfüßen nach, die Blattmassen flattern etwa 2 Grad."),
    "gust_desc": ("A single gust over 8 s from %.1f to %.1f m/s at 10 m: the wind-load tracks rise with the reconfigured drag law, the heavy heads lean first and the canes follow with a level-wise lag.",
                  "Eine einzelne Böe über 8 s von %.1f auf %.1f m/s in 10 m Höhe: Die Windlast-Spuren steigen mit dem umgeformten Widerstandsgesetz, zuerst neigen sich die schweren Blütenbälle, die Ruten folgen ebenenweise verzögert."),
    "breeze_pose": "flowering-breeze", "breeze_v10": 5.5, "gust_peak": 14.0, "default_pose": "flowering-calm", "default_scale": 50,
    "symbol_crown_base": 0.18, "symbol_k": 8, "symbol_stem": (0.2, 0.3),
}

SP["poses"] = [
    {"id": "flowering-calm", "title": T("Full bloom, calm", "Vollblüte, windstill"), "desc": T("A still July noon (1.5 m/s at 10 m) in full bloom: about thirty mopheads crown the canes, the broad leaves transpire at 80 % of their maximum.",
                                                                                                "Ein stiller Julimittag (1,5 m/s in 10 m Höhe) in Vollblüte: etwa dreißig Blütenbälle krönen die Ruten, die breiten Blätter transpirieren mit 80 % ihres Maximums."), "v10": 1.5, "emit": 0.8},
    {"id": "flowering-breeze", "title": T("Full bloom in a breeze", "Vollblüte bei Brise"), "desc": T("A fresh breeze of 5.5 m/s at 10 m: the mound leans about 1 degree downwind at the cane tops, the leaves turn their pale undersides.",
                                                                                                  "Eine frische Brise von 5,5 m/s in 10 m Höhe: Der Hügel neigt sich an den Rutenspitzen etwa 1 Grad in Windrichtung, die Blätter zeigen ihre hellen Unterseiten."), "v10": 5.5, "cd": 0.45},
    {"id": "storm", "title": T("Summer thunderstorm gust front", "Böenfront eines Sommergewitters"), "desc": T("A gust front of 18 m/s at 10 m under a dark sky: the leaves fold and the drag coefficient drops to 0.3 (reconfiguration after Vogel 1989); the canes lean about 2 degrees, the heads about 3 degrees.",
                                                                                                                  "Eine Böenfront von 18 m/s in 10 m Höhe unter dunklem Himmel: Die Blätter falten sich, der Widerstandsbeiwert sinkt auf 0,3 (Umformung nach Vogel 1989); die Ruten neigen sich etwa 2 Grad, die Blütenbälle etwa 3 Grad."), "v10": 18.0, "cd": 0.3, "sun": 0.25, "emit": 0.3},
    {"id": "rain-laden", "title": T("Heads soaked after rain", "Blütenbälle nach Regen vollgesogen"), "desc": T("Calm air after a long summer rain: every mophead holds about 150 g of water and the leaves about 0.2 kg per m2, so the outer canes splay outwards by 2 to 4.5 degrees and the heads hang 7 degrees lower, the typical flop of wet hydrangeas; the wet surfaces barely transpire.",
                                                                                                         "Windstille nach langem Sommerregen: Jeder Blütenball hält etwa 150 g Wasser und die Blätter etwa 0,2 kg je m2, daher spreizen sich die äußeren Ruten um 2 bis 4,5 Grad nach außen und die Bälle hängen 7 Grad tiefer, das typische Umkippen nasser Hortensien; die nassen Flächen transpirieren kaum."), "v10": 1.0, "sun": 0.15, "emit": 0.06,
     "wet": {"leaf-sun": 1.0, "leaf-shade": 1.0, "flowers": 1.0, "stem-bark": 0.5}, "offsets": rain_offsets, "extra": rain_loads},
]
SP["thermal_assumptions"] = refs.thermal_assumptions("hydrangea", ("Flowers are a passive exchange surface: the sepals of about thirty mopheads (3.6 m2, an authored estimate) take part in radiation and convection but are not given a stomatal model.",
                                                                   "Blüten sind eine passive Austauschfläche: Die Kelchblätter von etwa dreißig Blütenbällen (3,6 m2, eine Autorenschätzung) nehmen an Strahlung und Konvektion teil, erhalten aber kein Stomatamodell."))
SP["structure_assumptions"] = refs.structure_assumptions("ten canes", "zehn Ruten")
SP["decisions"] = refs.decisions_hydrangea()
SP["volume_decisions"] = refs.volume_decisions_hydrangea()

if __name__ == "__main__":
    tops = {lb.bone: lb.c for lb in SCALES["50"]["lobes"] if lb.id.endswith("-top")}
    for e in SP["emitters"]:
        e["anchors"] = [(b, p if p is not None else tops[b]) for b, p in e["anchors"]]
    doc, path, rig = assemble.build(SP)
    assemble.report(doc, rig, SP)
    print("[DEBUG] wrote", path)
