"""🌳 English oak (Quercus robur, open-grown, about 130 years): the hand numbers of author 12. World x ahead (east), y up, z left (north)."""
import os
from importlib.machinery import SourceFileLoader

A = SourceFileLoader("rework12assemble", os.path.join(os.path.dirname(os.path.abspath(__file__)), "rework-12-assemble.py")).load_module()
K = A.K

t = K.Tree("english-oak", bark="oak-bark", wood="oak-wood", wood_density=950, regions={"trunk": "trunk-bark", "limb": "limb-bark", "sun": "leaf-sun", "shade": "leaf-shade"}, leaf_materials={"sun": "oak-leaf-sun", "shade": "oak-leaf-shade"}, tones={"trunk": 0.27, "limb": 0.3, "sun": 0.58, "shade": 0.42})

TRUNK = [(0, 0, 0, 0.74), (0, 0.14, 0, 0.66), (0, 0.4, 0, 0.575), (0.01, 0.9, 0, 0.515), (0.02, 1.3, 0, 0.49), (0.03, 2.4, 0.01, 0.465), (0.02, 3.2, 0, 0.475), (0, 3.9, -0.02, 0.44), (-0.05, 5.0, -0.05, 0.37), (-0.12, 6.2, -0.02, 0.31), (-0.05, 7.4, 0.04, 0.27), (0.05, 8.6, 0.08, 0.235), (0.35, 10.2, 0, 0.18), (0.5, 11.9, -0.25, 0.13), (0.3, 13.4, -0.4, 0.09), (0.15, 14.8, -0.3, 0.055), (0.2, 16.0, -0.15, 0.028), (0.3, 16.9, 0, 0.012)]
fy = K.heights(TRUNK)
t.axis("stem", TRUNK, kind="trunk", level=0, ridges=(22, 0.011), lobes=[(5, 0.13, 20, 0.3), (3, 0.05, 100, 0.45)], ground=True, bones=[
    ("root-plate", 0, fy(0.4), 0, "Root plate and buttress flare", "Wurzelteller und Wurzelanlauf", "Root collar", "Wurzelhals"),
    ("trunk", fy(0.4), fy(3.4), 0, "Trunk (bole)", "Stamm (Schaft)", "Bole", "Schaft"),
    ("trunk-upper", fy(3.4), fy(6.2), 0, "Upper trunk", "Oberer Stamm", "Upper trunk", "Oberer Stamm"),
    ("leader", fy(6.2), fy(8.6), 0, "Stem head below the crown fork", "Stammkopf unter der Kronengabel", "Stem head", "Stammkopf"),
    ("leader-top", fy(8.6), fy(14.0), 1, "Central crown limb", "Mittlerer Kronenast", "Crown limb", "Kronenast"),
    ("fol-top", fy(14.0), 1.0, 3, "Top shoot with its foliage", "Wipfeltrieb mit Laub", "Top shoot", "Wipfeltrieb"),
])

for n, (az, reach, r0) in enumerate([(4, 2.3, 0.2), (76, 1.9, 0.17), (148, 2.2, 0.19), (220, 1.8, 0.16), (292, 2.1, 0.18)], 1):
    d = K.direction(az, 0)
    t.axis(f"root-{n}", [(d[0] * 0.2, 0.62, d[2] * 0.2, r0), (d[0] * 0.8, 0.24, d[2] * 0.8, r0 * 0.78), (d[0] * reach * 0.65, 0.07, d[2] * reach * 0.65, r0 * 0.55), (d[0] * reach, 0.0, d[2] * reach, r0 * 0.36)], bone="root-plate", kind="trunk", level=2, aspect=1.5, ground=True, sides=(14, 10, 8, 6))

AZ = {}
LIMBS = [
    ("w-low", 3.3, 172, 12, 10.2, 0.22, 24, -14, 0.35, "Lower west limb", "Unterer Westast"),
    ("e-low", 3.6, -10, 16, 9.8, 0.2, 22, 12, -0.3, "Lower east limb", "Unterer Ostast"),
    ("n-low", 3.9, 62, 20, 9.0, 0.185, 20, 14, 0.3, "Lower north limb", "Unterer Nordast"),
    ("s-low", 4.2, -118, 18, 8.8, 0.175, 24, -12, -0.3, "Lower south limb", "Unterer Südast"),
    ("w-mid", 5.6, -150, 34, 8.6, 0.17, 10, 10, 0.25, "Middle south-west limb", "Mittlerer Südwestast"),
    ("e-mid", 6.1, 38, 36, 8.4, 0.165, 8, -12, -0.25, "Middle north-east limb", "Mittlerer Nordostast"),
    ("nw-mid", 6.6, 120, 38, 7.8, 0.15, 8, 12, 0.2, "Middle north-west limb", "Mittlerer Nordwestast"),
    ("se-mid", 7.0, -55, 40, 7.6, 0.145, 6, -10, -0.2, "Middle south-east limb", "Mittlerer Südostast"),
]
for key, y, az, el, length, r0, up, turn, wave, en, de in LIMBS:
    AZ[key] = az
    t.branch(key, "stem", fy(y), az, el, length, r0, r1=0.012, up=up, turn=turn, wave=wave, level=1, socket=True, bones=[
        (f"limb-{key}", 0, 0.5, 1, en, de, "Limb", "Starkast"),
        (f"br-{key}", 0.5, 0.84, 2, f"{en}, outer branch", f"{de}, Außenast", "Branch", "Ast"),
        (f"fol-{key}", 0.84, 1.0, 3, f"{en}, end shoots with foliage", f"{de}, Endtriebe mit Laub", "End shoot", "Endtrieb"),
    ])
UPPER = [
    ("leader-s", 7.7, -118, 44, 8.0, 0.14, 4, 14, "Southern crown limb", "Südlicher Kronenast"),
    ("leader-w", 8.1, 165, 46, 7.8, 0.135, 4, 18, "Western crown limb", "Westlicher Kronenast"),
    ("leader-e", 8.4, 10, 48, 7.6, 0.13, 2, -16, "Eastern crown limb", "Östlicher Kronenast"),
    ("leader-n", 8.6, 64, 50, 7.4, 0.125, 2, 10, "Northern crown limb", "Nördlicher Kronenast"),
]
for key, y, az, el, length, r0, up, turn, en, de in UPPER:
    AZ[key] = az
    t.branch(key, "stem", fy(y), az, el, length, r0, r1=0.012, up=up, turn=turn, wave=0.15, level=1, socket=True, bones=[
        (f"br-{key}", 0, 0.7, 2, en, de, "Branch", "Ast"),
        (f"fol-{key}", 0.7, 1.0, 3, f"{en}, end shoots with foliage", f"{de}, Endtriebe mit Laub", "End shoot", "Endtrieb"),
    ])

BOUGHS = {
    "w-low": [(0.26, 62, 26, 3.6, 0.078), (0.4, -58, 20, 4.2, 0.074), (0.54, 54, 28, 3.8, 0.064), (0.68, -50, 24, 3.2, 0.054), (0.8, 44, 30, 2.6, 0.044)],
    "e-low": [(0.25, -64, 24, 3.4, 0.074), (0.4, 56, 22, 4.1, 0.07), (0.55, -52, 28, 3.6, 0.06), (0.69, 48, 26, 3.0, 0.05), (0.81, -42, 30, 2.4, 0.042)],
    "n-low": [(0.27, 60, 24, 3.3, 0.07), (0.42, -62, 22, 3.8, 0.066), (0.56, 50, 28, 3.3, 0.056), (0.69, -46, 26, 2.8, 0.048), (0.81, 40, 30, 2.2, 0.04)],
    "s-low": [(0.26, -58, 26, 3.2, 0.068), (0.41, 60, 20, 3.7, 0.064), (0.55, -52, 28, 3.2, 0.054), (0.69, 44, 26, 2.7, 0.046), (0.81, -40, 30, 2.2, 0.04)],
    "w-mid": [(0.28, 66, 28, 3.2, 0.064), (0.44, -60, 30, 3.5, 0.06), (0.6, 52, 34, 3.0, 0.05), (0.76, -44, 36, 2.4, 0.042)],
    "e-mid": [(0.28, -66, 28, 3.1, 0.062), (0.44, 58, 30, 3.4, 0.058), (0.6, -50, 34, 2.9, 0.048), (0.76, 42, 36, 2.3, 0.04)],
    "nw-mid": [(0.3, 62, 30, 3.0, 0.058), (0.46, -58, 32, 3.2, 0.054), (0.62, 48, 36, 2.7, 0.046), (0.78, -42, 38, 2.1, 0.038)],
    "se-mid": [(0.3, -62, 30, 2.9, 0.056), (0.46, 56, 32, 3.1, 0.052), (0.62, -48, 36, 2.6, 0.044), (0.78, 40, 38, 2.0, 0.038)],
    "leader-s": [(0.36, 64, 30, 3.0, 0.054), (0.54, -58, 34, 2.9, 0.05), (0.72, 48, 38, 2.4, 0.042)],
    "leader-w": [(0.36, -62, 32, 2.9, 0.052), (0.54, 56, 36, 2.8, 0.048), (0.72, -46, 40, 2.3, 0.04)],
    "leader-e": [(0.38, 60, 32, 2.8, 0.05), (0.56, -54, 36, 2.7, 0.046), (0.74, 44, 40, 2.2, 0.04)],
    "leader-n": [(0.38, -58, 34, 2.7, 0.048), (0.56, 52, 38, 2.6, 0.044), (0.74, -42, 42, 2.1, 0.038)],
}
TWIGS = [(0.36, 54, 20, 1.8, 0.026), (0.58, -50, 24, 1.6, 0.024), (0.8, 42, 28, 1.3, 0.02)]
SHADE = set()
for key, rows in BOUGHS.items():
    for n, (at, daz, el, length, r0) in enumerate(rows, 1):
        bid = f"{key}-b{n}"
        t.branch(bid, key, at, AZ[key] + daz, el, length, r0, r1=0.009, up=14, turn=-daz * 0.2, level=2, wave=0.12 if n % 2 else -0.12, scales=(50, 100, 200) if r0 >= 0.05 else (50, 100))
        inner = at <= 0.34 and key.endswith(("low", "mid"))
        if inner:
            SHADE.add(bid)
        for m, (tat, tdaz, tel, tlen, tr0) in enumerate(TWIGS, 1):
            sign = 1 if (n + m) % 2 else -1
            tid = f"{bid}-t{m}"
            t.branch(tid, bid, tat, AZ[key] + daz + sign * tdaz, tel + 4 * m, tlen * length / 3.4, tr0 * (r0 / 0.06) ** 0.5, r1=0.008, up=10, level=3, scales=(50, 100))
            t.spray(f"fol-{tid}", tid, 0.3, 0.8 * length / 3.4 + 0.22, 0.52, (5, 2, 0.18), over=0.25, lift=0.1, shade=inner, scales=(50,), group=key)
        t.spray(f"fol-{bid}", bid, 0.7, 1.05 * length / 3.4 + 0.25, 0.7, (6, 3, 0.16), over=0.35, lift=0.14, shade=inner, scales=(50,), group=key)
        t.spray(f"fol-{bid}", bid, 0.26, 1.75 * length / 3.4 + 0.2, 0.95, (6, 3, 0.15), over=0.4, lift=0.2, shade=inner, scales=(100,), group=key)
for key in list(BOUGHS):
    low = key.endswith("low")
    t.spray(f"fol-{key}-tip", key, 0.82 if key in AZ and not key.startswith("leader") else 0.68, 1.5 if low else 1.3, 0.95 if low else 0.9, (7, 3, 0.15), over=0.5, lift=0.2, scales=(50, 100), group=key)
    reach = 3.0 if low else 2.6 if key.endswith("mid") else 2.3
    t.spray(f"fol-{key}-inner", key, 0.26, reach * 0.92, 1.35 if low else 1.2, (6, 3, 0.14), f1=0.62, over=0.0, lift=0.5, shade=key.endswith(("low", "mid")), scales=(200,), flat=0.5, peak=0.5, base=0.5, group=key)
    t.spray(f"fol-{key}-outer", key, 0.56, reach, 1.5 if low else 1.3, (6, 3, 0.14), over=0.9, lift=0.5, scales=(200,), flat=0.5, group=key)
    t.spray(f"crown-{key}", key, 0.28, reach * 1.12, 1.55 if low else 1.3, (5, 3, 0.12), over=1.0, lift=0.6, scales=(500,), flat=0.5, peak=0.55, base=0.5, group=key)
TOP = [(10.4, 200, 30, 3.4, 0.056), (11.4, 20, 32, 3.3, 0.052), (12.5, 110, 34, 3.0, 0.046), (13.5, -70, 36, 2.7, 0.04), (14.5, 170, 38, 2.3, 0.034), (15.4, -10, 40, 1.9, 0.028)]
for n, (y, az, el, length, r0) in enumerate(TOP, 1):
    bid = f"top-b{n}"
    t.branch(bid, "stem", fy(y), az, el, length, r0, r1=0.009, up=12, level=2, scales=(50, 100, 200) if r0 >= 0.045 else (50, 100))
    for m, (tat, tdaz, tel, tlen, tr0) in enumerate(TWIGS[:2], 1):
        tid = f"{bid}-t{m}"
        t.branch(tid, bid, tat, az + (tdaz if (n + m) % 2 else -tdaz), tel + 6, tlen * length / 3.4, tr0, r1=0.008, up=10, level=3, scales=(50, 100))
        t.spray(f"fol-{tid}", tid, 0.3, 0.8 * length / 3.4 + 0.22, 0.52, (5, 2, 0.18), over=0.25, lift=0.1, scales=(50,), group="top")
    t.spray(f"fol-{bid}", bid, 0.7, 1.05 * length / 3.4 + 0.25, 0.7, (6, 3, 0.16), over=0.35, lift=0.14, scales=(50,), group="top")
    t.spray(f"fol-{bid}", bid, 0.26, 1.75 * length / 3.4 + 0.2, 0.95, (6, 3, 0.15), over=0.4, lift=0.2, scales=(100,), group="top")
t.spray("fol-top-shoot", "stem", fy(15.7), 1.2, 1.0, (6, 3, 0.16), over=0.4, scales=(50, 100), bone="fol-top", flat=1.0, group="top")
t.spray("crown-top", "stem", fy(10.2), 2.9, 2.7, (6, 4, 0.14), f1=fy(13.8), over=0.0, scales=(200, 500), bone="leader-top", flat=1.0, peak=0.5, base=0.55, group="top")
t.spray("crown-core", "stem", fy(5.6), 2.4, 2.2, (6, 4, 0.12), f1=fy(9.6), over=0.0, shade=True, scales=(500,), bone="leader", flat=1.0, peak=0.5, base=0.7, group="core")
t.spray("fol-top-shoot", "stem", fy(13.6), 2.2, 2.0, (6, 3, 0.13), over=0.5, scales=(200, 500), bone="fol-top", flat=1.0, group="top")

cfg = {
    "like": {"limb-nw-mid": "limb-w-mid", "limb-se-mid": "limb-e-mid", "br-nw-mid": "br-w-mid", "br-se-mid": "br-e-mid", "fol-nw-mid": "fol-w-mid", "fol-se-mid": "fol-e-mid", "limb-n-low": "limb-w-low", "limb-s-low": "limb-e-low", "br-n-low": "br-w-low", "br-s-low": "br-e-low", "fol-n-low": "fol-w-low", "fol-s-low": "fol-e-low", "br-leader-s": "br-leader-e", "br-leader-n": "br-leader-w", "fol-leader-s": "fol-leader-e", "fol-leader-n": "fol-leader-w"},
    "droop_poses": ["wet-snow"],
    "group_bone": dict({key: f"br-{key}" for key in BOUGHS}, top="leader-top", core="leader"),
    "leaf_density": 0.45,
    "emitters": {
        "transpiration-sun": {"on": ["fol-w-low-tip", "fol-e-low-tip", "fol-n-low-tip", "fol-s-low-tip", "fol-w-mid-tip", "fol-e-mid-tip", "fol-top-shoot"], "diameter": 1.6, "cls": "sun", "prefix": "tp-sun", "title": ("Transpiration of the sun leaves", "Transpiration der Sonnenblätter")},
        "transpiration-shade": {"on": ["fol-w-low-b1", "fol-e-low-b1", "fol-n-low-b1", "fol-s-low-b1", "fol-w-mid-b1"], "diameter": 1.2, "cls": "shade", "prefix": "tp-shade", "title": ("Transpiration of the shade leaves", "Transpiration der Schattenblätter")},
    },
    "scaffolds": ["w-low", "e-low", "n-low", "s-low"],
    "habit": {
        50: ("the oak is built as it grows. A root collar 1.5 m across with five buttress roots carries a fissured bole of 0.98 m at breast height that swells below the first limbs at 3.3 m and dissolves at 8.6 m into five ascending crown limbs, the strongest of which carries the top to 17 m. Eight scaffold limbs leave the stem all round the compass, the four lowest only 12 to 20 degrees above the horizon and up to 10 m long, elbowed and turning up at the tips; every limb gives off three to five boughs alternately to both sides and every bough three branches. The foliage sits where oak leaves sit, crowded at the shoot ends: a flat-bottomed tuft on every branch, bough end and limb end, sun leaves outside, shade leaves on the innermost boughs, with the sky open between the limb systems and tiers.", "Die Eiche ist gebaut, wie sie wächst. Ein Wurzelhals von 1,5 m mit fünf Wurzelanläufen trägt einen rissigen Schaft von 0,98 m in Brusthöhe, der unter den ersten Starkästen in 3,3 m anschwillt und sich in 8,6 m in fünf aufsteigende Kronenäste auflöst, deren stärkster den Wipfel bis 17 m trägt. Acht Starkäste verlassen den Stamm rundum, die vier untersten nur 12 bis 20 Grad über der Waagrechten und bis 10 m lang, gekniet und an den Spitzen aufgerichtet; jeder Starkast trägt wechselständig drei bis fünf Äste und jeder Ast drei Zweige. Das Laub sitzt, wo Eichenlaub sitzt, gedrängt an den Triebenden: ein unten flaches Büschel an jedem Zweig, Astende und Starkastende, Sonnenblätter außen, Schattenblätter an den innersten Ästen, mit offenem Himmel zwischen den Astsystemen und Etagen."),
        100: ("the same skeleton with stem, eight scaffold limbs, five crown limbs, their boughs and branches down to 37 mm; the bark fissures and the leaf tufts of the single branches are gone, each bough carries one foliage mass from its first branch to beyond its tip and each limb end one.", "Dasselbe Skelett mit Stamm, acht Starkästen, fünf Kronenästen, ihren Ästen und Zweigen bis 37 mm; Rindenrisse und die Laubbüschel der einzelnen Zweige entfallen, jeder Ast trägt eine Laubmasse von seinem ersten Zweig bis über die Spitze hinaus und jedes Starkastende eine."),
        200: ("stem, limbs and the boughs of at least 100 mm base diameter remain as wood (thinner ends of the limbs as slender solids of true thickness); every limb system is two foliage masses, a shaded inner one over its first boughs and a sunlit outer one over its outer boughs and tip, so the tiers and the gaps between the limb systems stay open.", "Stamm, Starkäste und die Äste ab 100 mm Basisdurchmesser bleiben als Holz (dünnere Astenden als schlanke Körper in wahrer Dicke); jedes Astsystem besteht aus zwei Laubmassen, einer beschatteten inneren über den ersten Ästen und einer besonnten äußeren über den äußeren Ästen und der Spitze, sodass Etagen und Lücken zwischen den Astsystemen offen bleiben."),
        500: ("stem with root collar and the thirteen limbs, none thinner than 240 mm, and one foliage mass per limb system around a shaded core, plus the top: fifteen lobes that give the broad, irregular dome with its notches.", "Stamm mit Wurzelhals und die dreizehn Starkäste, keiner dünner als 240 mm, und je Astsystem eine Laubmasse um einen beschatteten Kern sowie der Wipfel: fünfzehn Lappen, die die breite, unregelmäßige Kuppel mit ihren Einschnitten ergeben."),
    },
    "drawing": {
        50: ("fissured bole on its buttress roots, eight scaffold limbs and five crown limbs with boughs and branches, and the leaf tufts of every limb system as one lobed outline with its sky openings, so that near systems overlap far ones.", "rissiger Schaft auf seinen Wurzelanläufen, acht Starkäste und fünf Kronenäste mit Ästen und Zweigen und die Laubbüschel jedes Astsystems als ein gelappter Umriss mit seinen Himmelslücken, sodass nahe Systeme ferne überdecken."),
        100: ("bole, limbs, boughs and branches as tapered outlines per bone and the foliage of each bone as one lobed outline.", "Schaft, Starkäste, Äste und Zweige als verjüngte Umrisse je Knochen und das Laub jedes Knochens als ein gelappter Umriss."),
        200: ("bole, limbs and main boughs, and two foliage masses per limb system, the inner one shaded.", "Schaft, Starkäste und Hauptäste sowie zwei Laubmassen je Astsystem, die innere beschattet."),
        500: ("stem, limb stubs and the crown lobes around the shaded core.", "Stamm, Astansätze und die Kronenlappen um den beschatteten Kern."),
    },
}
A.assemble(t, cfg)
