"""🌲 Lombardy poplar (Populus nigra 'Italica', open-grown, about 45 years, 26 m): the hand numbers of its author. World x ahead (east), y up, z left (north).

The chisel is `rework-12-kit.py` (tapered tube lofts and foliage lofts along the branch that carries them), the carrier into the asset document `rework-12-assemble.py`.

@see ./rework/assets/vegetation--lombardy-poplar.md
"""
import math
import os
from importlib.machinery import SourceFileLoader

HERE = os.path.dirname(os.path.abspath(__file__))
A = SourceFileLoader("rework12assemble", os.path.join(HERE, "rework-12-assemble.py")).load_module()
K = A.K
K.OUT = os.path.join(HERE, "🗑️generated", "rework-lombardy-poplar")


def flame(u, start=0.1, peak=0.6, base=0.35):
    """🔥 The foliage of one upright poplar shoot: rounded where it begins, widest at `peak`, then drawn out into an acute tip (the kit's elliptic nose reads as an egg)."""
    if u < peak:
        w = base + (1 - base) * math.sin(math.pi / 2 * u / peak)
        return w * (math.sqrt(max(0.0, 1 - (1 - u / start) ** 2)) if u < start else 1.0)
    return math.cos(math.pi / 2 * (u - peak) / (1 - peak)) ** 0.8


K.envelope = flame

t = K.Tree("lombardy-poplar", bark="poplar-bark", wood="poplar-wood", wood_density=780, regions={"trunk": "stem-bark", "limb": "branch-bark", "sun": "leaf-crown", "shade": "leaf-crown"}, leaf_materials={"sun": "poplar-leaf", "shade": "poplar-leaf"}, tones={"trunk": 0.27, "limb": 0.3, "sun": 0.55, "shade": 0.44})

STEM = [(0, 0, 0, 0.6), (0, 0.12, 0, 0.53), (0, 0.4, 0, 0.455), (0.005, 0.8, 0, 0.415), (0.01, 1.3, 0, 0.39), (0.02, 3.0, 0.01, 0.355), (0.03, 6.0, 0.02, 0.305), (0.0, 9.0, 0.03, 0.255), (-0.03, 12.0, 0.0, 0.21), (-0.01, 15.0, -0.03, 0.165), (0.03, 18.0, -0.01, 0.118), (0.03, 18.45, -0.01, 0.112)]
LEADER = [(0.03, 17.9, -0.01, 0.119), (0.035, 19.5, 0.005, 0.095), (0.04, 21.0, 0.02, 0.072), (0.01, 23.5, 0.02, 0.036), (0.0, 25.3, 0.0, 0.012)]
fy = K.heights(STEM)
fl = K.heights(LEADER)
t.axis("stem", STEM, kind="trunk", level=0, ridges=(20, 0.012), lobes=[(6, 0.15, 15, 0.32), (4, 0.05, 70, 0.6)], ground=True, bones=[
    ("root-plate", 0, fy(0.4), 0, "Root plate and fluted stem base", "Wurzelteller und gefurchter Stammfuß", "Root collar", "Wurzelhals"),
    ("trunk-1", fy(0.4), fy(6.3), 0, "Stem, lower quarter", "Stamm, unteres Viertel", "Lower stem", "Unterer Stamm"),
    ("trunk-2", fy(6.3), fy(12.3), 0, "Stem, second quarter", "Stamm, zweites Viertel", "Middle stem", "Mittlerer Stamm"),
    ("trunk-3", fy(12.3), 1.0, 0, "Stem, third quarter", "Stamm, drittes Viertel", "Upper stem", "Oberer Stamm"),
])
t.axis("leader", LEADER, kind="trunk", level=1, parent="trunk-3", bones=[
    ("trunk-4", fl(18.3), fl(23.0), 1, "Leader", "Stammverlängerung", "Leader", "Leittrieb"),
    ("fol-top", fl(23.0), 1.0, 3, "Top shoot with its foliage", "Wipfeltrieb mit Laub", "Top shoot", "Wipfeltrieb"),
])

for n, (az, reach, r0) in enumerate([(22, 1.5, 0.13), (95, 1.25, 0.115), (160, 1.45, 0.125), (236, 1.2, 0.11), (305, 1.4, 0.12)], 1):
    d = K.direction(az, 0)
    t.axis(f"root-{n}", [(d[0] * 0.22, 0.42, d[2] * 0.22, r0), (d[0] * 0.6, 0.17, d[2] * 0.6, r0 * 0.8), (d[0] * reach * 0.7, 0.05, d[2] * reach * 0.7, r0 * 0.55), (d[0] * reach, 0.0, d[2] * reach, r0 * 0.34)], bone="root-plate", kind="trunk", level=2, aspect=1.4, ground=True, sides=(14, 10, 8, 6), scales=(50, 100, 200))

COMPASS = {"e": (0, "east", "Ost"), "n": (90, "north", "Nord"), "w": (180, "west", "West"), "s": (270, "south", "Süd")}
TIERS = [
    (1, 2.3, 12, 6.8, 63, 24, 0.088, "Lowest", "Unterster"),
    (2, 5.3, 57, 6.6, 64, 23, 0.082, "Second", "Zweiter"),
    (3, 8.6, 27, 6.0, 66, 22, 0.075, "Third", "Dritter"),
    (4, 12.2, 72, 5.4, 69, 19, 0.068, "Fourth", "Vierter"),
]
STAGGER = {"e": (0.0, 1.0, 9, 0.07), "n": (1.7, 0.92, -11, -0.06), "w": (0.8, 1.06, 13, 0.08), "s": (2.5, 0.96, -8, -0.07)}
TWIGS = [(0.2, 78, 2.5, 0.03), (0.33, -66, 2.4, 0.028), (0.47, 52, 2.1, 0.025), (0.6, -48, 1.8, 0.022), (0.73, 38, 1.5, 0.019), (0.85, -30, 1.1, 0.016)]
AZ = {}
for tier, y0, az0, length, el, up, r0, en, de in TIERS:
    for d, (daz, den, dde) in COMPASS.items():
        key = f"{tier}{d}"
        dy, stretch, turn, wave = STAGGER[d]
        az = az0 + daz
        AZ[key] = az
        L = length * stretch
        t.branch(key, "stem", fy(y0 + dy), az, el, L, r0, r1=0.01, up=up, turn=turn, wave=wave, level=1, bones=[
            (f"br-{key}", 0, 0.55, 1, f"{en} upright branch, {den}", f"{de} Steilast, {dde}", "Upright branch", "Steilast"),
            (f"fol-{key}", 0.55, 1.0, 2, f"{en} upright branch, {den}, end shoots with foliage", f"{de} Steilast, {dde}, Endtriebe mit Laub", "End shoot", "Endtrieb"),
        ])
        for m, (at, tdaz, tlen, tr0) in enumerate(TWIGS, 1):
            tid = f"{key}-t{m}"
            tl = tlen * L / 6.8
            t.branch(tid, key, at, az + tdaz, 60, tl, tr0, r1=0.008, up=27, level=2, scales=(50, 100))
            t.spray(f"fol-{tid}", tid, 0.22, 0.25 + 0.055 * tl, 0.25 + 0.055 * tl, (5, 4, 0.17), over=0.35, scales=(50, 100), flat=1.0, peak=0.38, base=0.5)
        t.spray(f"fol-{key}-mid", key, 0.34, 0.36, 0.36, (5, 6, 0.16), f1=0.78, over=0.0, scales=(50, 100), flat=1.0, peak=0.45, base=0.55)
        t.spray(f"fol-{key}-tip", key, 0.74, 0.4, 0.4, (5, 4, 0.17), over=0.5, scales=(50, 100), flat=1.0, peak=0.36, base=0.55)
        t.spray(f"plume-{key}", key, 0.22, 0.58, 0.58, (5, 4, 0.13), over=0.7, scales=(200,), flat=1.0, peak=0.62, base=0.2)
        t.spray(f"crown-{key}", key, 0.18, 0.74, 0.72, (4, 3, 0.1), over=0.7, scales=(500,), flat=1.0, peak=0.58, base=0.46)

FILLERS = [(1.5, 2.2, 0.032), (1.75, 2.5, 0.035), (2.05, 2.8, 0.04), (2.5, 3.0, 0.042), (2.9, 3.2, 0.045), (3.3, 3.3, 0.045), (3.7, 3.4, 0.045), (4.4, 3.6, 0.05), (5.1, 3.6, 0.05), (5.7, 3.7, 0.05), (6.3, 3.8, 0.05), (6.85, 3.8, 0.05), (7.4, 3.8, 0.05), (8.2, 3.8, 0.048), (8.9, 3.6, 0.048), (9.8, 3.6, 0.046), (10.7, 3.6, 0.045), (11.6, 3.4, 0.045), (12.5, 3.2, 0.042), (13.2, 3.3, 0.042), (13.9, 3.4, 0.042), (14.7, 3.4, 0.042), (15.15, 3.5, 0.044), (15.6, 3.6, 0.045), (16.3, 4.2, 0.052), (16.65, 3.9, 0.048), (17.0, 4.0, 0.05), (17.35, 3.8, 0.046), (17.7, 3.8, 0.048), (18.05, 3.6, 0.044), (18.4, 3.5, 0.045), (18.75, 3.3, 0.04), (19.1, 3.2, 0.04), (19.8, 2.9, 0.036), (20.5, 2.6, 0.032), (21.2, 2.2, 0.028), (21.9, 1.8, 0.024), (22.6, 1.4, 0.02)]
EXTRA = [(4.7, 3.3, 0.045, 282), (12.0, 3.3, 0.044, 276), (4.1, 3.2, 0.044, 100), (10.3, 3.4, 0.045, 195)]
SHORT = [(f"f{n:02d}", y, length, r0, 40 + 137.5 * n) for n, (y, length, r0) in enumerate(FILLERS, 1)] + [(f"g{n:02d}", y, length, r0, az) for n, (y, length, r0, az) in enumerate(EXTRA, 1)]
for n, (fid, y, length, r0, az) in enumerate(SHORT, 1):
    high = y > 16
    el, up = (64, 23) if high else (55, 31)
    t.branch(fid, "leader" if y > 18.3 else "stem", fl(y) if y > 18.3 else fy(y), az, el, length, r0, r1=0.008, up=up, turn=10 if n % 2 else -10, level=2)
    rx = 0.3 + 0.03 * length
    for m, (at, tdaz, tl, tr0) in enumerate([(0.32, 58, 0.42, 0.02), (0.58, -52, 0.34, 0.017)], 1):
        tid = f"{fid}-t{m}"
        t.branch(tid, fid, at, az + tdaz, 60, tl * length, tr0 * (r0 / 0.045) ** 0.5, r1=0.008, up=27, level=3, scales=(50, 100))
        t.spray(f"fol-{tid}", tid, 0.2, rx * 0.8, rx * 0.8, (5, 3, 0.17), over=0.3, scales=(50, 100), flat=1.0, peak=0.38, base=0.5)
    t.spray(f"fol-{fid}", fid, 0.34, rx, rx, (5, 5, 0.16), over=0.45, scales=(50, 100), flat=1.0, peak=0.36, base=0.55)
    t.spray(f"fol-{fid}", fid, 0.3, rx * 1.1, rx * 1.1, (4, 3, 0.12), over=0.5, scales=(200,), flat=1.0, peak=0.42, base=0.4)

t.spray("fol-top-shoot", "leader", fl(22.7), 0.4, 0.4, (5, 4, 0.16), over=0.5, scales=(50, 100), bone="fol-top", flat=1.0, peak=0.3, base=0.6)
t.spray("fol-top-shoot", "leader", fl(22.0), 0.6, 0.6, (4, 3, 0.1), over=0.5, scales=(200,), bone="fol-top", flat=1.0, peak=0.3, base=0.6)
t.spray("fol-top-shoot", "leader", fl(21.3), 0.9, 0.9, (4, 3, 0.08), over=0.5, scales=(500,), bone="fol-top", flat=1.0, peak=0.22, base=0.85)
CORES = [
    ("trunk-1", [(1.7, 0.3), (2.3, 0.8), (3.2, 1.05), (5.0, 1.25), (7.2, 1.45)]),
    ("trunk-2", [(6.0, 1.25), (7.2, 1.45), (9.0, 1.5), (11.0, 1.5), (13.2, 1.4)]),
    ("trunk-3", [(12.0, 1.25), (13.2, 1.4), (15.0, 1.32), (17.0, 1.27), (19.3, 1.2)]),
    ("trunk-4", [(18.0, 1.05), (19.3, 1.2), (20.3, 1.17), (21.2, 1.05), (22.3, 0.8), (23.2, 0.6)]),
]
for bone, rows in CORES:
    t.mass(f"crown-{bone}", bone, [(0, y, 0, r, r) for y, r in rows], (5, 4, 0.1), scales=(500,), flat=1.0, peak=0.9, base=1.0)

cfg = {
    "leaf_density": 0.5,
    "emitters": {"transpiration-crown": {"on": [f"fol-{k}-tip" for k in ("1e", "1w", "2n", "2s", "3e", "3w", "4n", "4s")] + ["fol-top-shoot"], "diameter": 0.5, "cls": "sun", "prefix": "tp-crown", "title": ("Transpiration of the crown", "Transpiration der Krone")}},
    "scaffolds": ["1e", "1n", "1w", "1s"],
    "stems": [["trunk-1", "root-plate", "trunk-2", "trunk-3", "trunk-4", "fol-top"]],
    "habit": {
        50: ("the poplar is built as it grows. A fluted stem base 1.2 m across with five shallow surface roots carries a furrowed stem of 0.78 m at breast height that runs through to the tip at 25.3 m. Sixteen upright scaffold branches in four tiers leave it all round the compass at 18 to 27 degrees from the stem and turn up until they run parallel to it, 5 to 7 m long; between them thirty-eight shorter branches fill the column from 1.5 m above the ground to the leader. Every scaffold branch carries six upright side shoots, every short branch two. The foliage sits as it does on the tree, as slender vertical flames with acute tips around each shoot, so the crown is a column of overlapping plumes with vertical grooves and a serrated outline, 4.5 m across at a third of the height and pointed at the top.", "Die Pappel ist gebaut, wie sie wächst. Ein gefurchter Stammfuß von 1,2 m mit fünf flachen Oberflächenwurzeln trägt einen rissigen Stamm von 0,78 m in Brusthöhe, der bis zur Spitze in 25,3 m durchläuft. Sechzehn Steiläste in vier Etagen verlassen ihn rundum unter 18 bis 27 Grad zum Stamm und richten sich auf, bis sie parallel zu ihm stehen, 5 bis 7 m lang; dazwischen füllen achtunddreißig kürzere Äste die Säule von 1,5 m über dem Boden bis zum Leittrieb. Jeder Steilast trägt sechs aufrechte Seitentriebe, jeder kurze Ast zwei. Das Laub sitzt wie am Baum als schlanke senkrechte Flammen mit spitzem Ende um jeden Trieb, sodass die Krone eine Säule aus überlappenden Fahnen mit senkrechten Rinnen und gezacktem Umriss ist, 4,5 m breit auf einem Drittel der Höhe und oben spitz."),
        100: ("the same skeleton with stem, sixteen scaffold branches, thirty-eight short branches and their side shoots down to 32 mm; the bark furrows and the fine relief of the foliage are gone; every shoot keeps its own flame (0.5 to 0.9 m across, far above the 15 mm feature), meshed more coarsely.", "Dasselbe Skelett mit Stamm, sechzehn Steilästen, achtunddreißig kurzen Ästen und ihren Seitentrieben bis 32 mm; Rindenfurchen und das feine Relief des Laubs entfallen; jeder Trieb behält seine eigene Flamme (0,5 bis 0,9 m breit, weit über dem Merkmal von 15 mm), gröber vernetzt."),
        200: ("stem, scaffold branches and the short branches of at least 66 mm remain as wood; every scaffold branch and every short branch is one upright plume, so the column keeps its vertical grooves and its serrated outline.", "Stamm, Steiläste und die kurzen Äste ab 66 mm bleiben als Holz; jeder Steilast und jeder kurze Ast ist eine aufrechte Fahne, sodass die Säule ihre senkrechten Rinnen und den gezackten Umriss behält."),
        500: ("stem with its base and the sixteen scaffold branches, none thinner than 160 mm, one plume per scaffold branch around four inner crown masses that follow the spindle of the column along the stem, and the top shoot drawn out into the tip: the narrow column, widest at a third of its height and pointed.", "Stamm mit Fuß und die sechzehn Steiläste, keiner dünner als 160 mm, je Steilast eine Fahne um vier innere Kronenmassen, die der Spindel der Säule am Stamm folgen, und der zur Spitze ausgezogene Wipfeltrieb: die schmale Säule, am breitesten auf einem Drittel der Höhe und oben spitz."),
    },
    "drawing": {
        50: ("furrowed stem on its fluted base, the upright scaffold and short branches with their side shoots, and a separate outline for every foliage flame, so that near flames overlap far ones and the column reads as plumes.", "rissiger Stamm auf gefurchtem Fuß, die Steiläste und kurzen Äste mit ihren Seitentrieben und ein eigener Umriss für jede Laubflamme, sodass nahe Flammen ferne überdecken und die Säule als Fahnen lesbar ist."),
        100: ("stem, scaffold and short branches as tapered outlines per bone and the flame of every shoot as its own outline.", "Stamm, Steiläste und kurze Äste als verjüngte Umrisse je Knochen und die Flamme jedes Triebs als eigener Umriss."),
        200: ("stem, scaffold branches and one plume per branch.", "Stamm, Steiläste und eine Fahne je Ast."),
        500: ("stem and the stepped column of sixteen plumes around the inner crown masses.", "Stamm und die gestufte Säule aus sechzehn Fahnen um die inneren Kronenmassen."),
    },
}
t.offsets = lambda bone, kind, deg: {"angleDeg": deg}

WOOD = {"trunk": 0.92, "limb": 0.94}
CORE_SIDES = {50: (40, 12, 8), 100: (28, 10, 6), 200: (18, 8, 6), 500: (12, 6, 6)}
kit_build = t.build
WOODS = {}


def build(scale, *args, **kwargs):
    """🪵 The kit's meshes with the member of every wood solid restated as the load-bearing section: the wood inside the bark (stem bark 30 mm on 390 mm at breast height, branch bark 6 %), with the flutes and the oval of the body it lies in."""
    wood, leaves = kit_build(scale, *args, **kwargs)
    for g in wood:
        a = t.axes[g["axis"]]
        samples = g["samples"]
        thin = samples if len(samples) < 5 else [q for i, q in enumerate(samples) if i % 2 == 0 or i == len(samples) - 1]
        core = [(p, (v[0] * WOOD[g["kind"]],), tan) for p, v, tan in thin]
        sides = 6 if g["slender"] else CORE_SIDES[scale][min(g["level"], 2)]
        mesh = K.tube(core, sides, a["aspect"], None, None if g["slender"] else a["lobes"])
        g["core"] = mesh.ground() if a["ground"] else mesh
    WOODS[scale] = wood
    return wood, leaves


t.build = build
doc = A.assemble(t, cfg)
asset = doc["asset"]
import re
MASS_FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
for bone in asset["rig"]["bones"]:
    f = WOOD["trunk" if bone["id"].startswith(("trunk", "root")) else "limb"]
    sec = bone["section"]
    r = sec["extremeFibreM"] * f
    sec.update(areaM2=float(f"{math.pi * r * r:.4g}"), secondMomentM4=float(f"{math.pi * r ** 4 / 4:.4g}"), extremeFibreM=round(r, 4))
    sec["title"] = {"en": re.sub(r"d = [0-9.]+ m", f"d = {2 * r:.2f} m", sec["title"]["en"]), "de": re.sub(r"d = [0-9,]+ m", f"d = {2 * r:.2f} m".replace(".", ","), sec["title"]["de"])}
for scale, volume in asset["volumes"].items():
    structure = volume["structure"] = [s for s in volume["structure"] if s.get("part") != "mass"]
    have = set()
    for s in structure:
        if s.get("part") == "joint":
            centre = t.frames[s["bone"]]["o"]
            radius = min(centre[1] - 0.01, 1.06 * max(q[1][0] * WOOD[g["kind"]] for g in WOODS[int(scale)] for q in g["samples"] if K.norm(K.sub(q[0], centre)) <= q[1][0] * 0.5 + 0.03))
            s.update(t.solid(s["id"], s["bone"], K.sphere(centre, radius, 10 if scale == "50" else 8 if scale == "100" else 6), t.wood, 0.3, role="detail", extra={"member": s["bone"], "part": "joint"}))
    for bone in asset["rig"]["bones"]:
        kg = bone["mass"]["kg"]
        radius = 0.5 * (3 * kg / (4 * math.pi * 780)) ** (1 / 3)
        if f"mass-{bone['id']}" in have or radius < MASS_FEATURE[int(scale)]:
            continue
        fr = t.frames[bone["id"]]
        centre = K.add(fr["o"], K.add(K.mul(fr["u"], bone["mass"]["comX"]), K.mul(fr["v"], bone["mass"]["comY"])))
        structure.append(t.solid(f"mass-{bone['id']}", bone["id"], K.sphere(centre, radius, 10 if scale == "50" else 8 if scale != "500" else 6), t.wood, 0.3, role="detail", extra={"member": bone["id"], "part": "mass"}))
    for text in volume["decisions"]:
        if text["en"].startswith("Structure"):
            text["en"] = re.sub(r"\(\d+ solids\)", f"({len(structure)} solids)", text["en"]).replace("at 0.55 of the bark radius", "with the load-bearing section, the wood inside the bark (0.92 of the bark radius on the stem, 0.94 on the branches, fluted at the stem base as the body is)").replace("joint spheres sit at the rig joints", "a joint sphere sits at every rig joint, a little larger than the wood it lies in (a branch joint lies in the stem, so its sphere shows as a bead on the stem)").replace("the heaviest segments carry a mass sphere on their axis", "every bone carries a mass sphere at its centre of mass, sized by the cube root of its mass (wood and foliage)")
            text["de"] = re.sub(r"\(\d+ Körper\)", f"({len(structure)} Körper)", text["de"]).replace("mit dem 0,55-fachen Rindenradius", "mit dem tragenden Querschnitt, dem Holz unter der Rinde (0,92 des Rindenradius am Stamm, 0,94 an den Ästen, am Stammfuß gefurcht wie der Körper)").replace("Gelenkkugeln sitzen an den Gelenken des Skeletts", "an jedem Gelenk des Skeletts sitzt eine Gelenkkugel, etwas größer als das Holz, in dem es liegt (ein Astgelenk liegt im Stamm, seine Kugel zeigt sich als Wulst am Stamm)").replace("die schwersten Abschnitte tragen eine Massenkugel auf ihrer Achse", "jeder Knochen trägt eine Massenkugel in seinem Schwerpunkt, bemessen nach der dritten Wurzel seiner Masse (Holz und Laub)")
            print("[DEBUG] " + text["en"][-330:])
for moved in [pose["bones"] for pose in asset["poses"]] + [key["bones"] for clip in asset["clips"] for key in clip["keys"] if "bones" in key]:
    for bone, offset in moved.items():
        if abs(offset.pop("abductionDeg", 0)) > 0.5:
            raise SystemExit(f"{bone} would leave its drawing plane")
for pose in asset["poses"]:
    for load in pose.get("loads", []):
        if "distributed" not in load and load["bone"].startswith("trunk"):
            load["id"] = f"wind-crown-{load['bone']}"
EMITTER_DOWN = 0.45
for anchor in asset["anchors"]:
    if anchor["kind"] == "emitter":
        anchor["x"] = round(anchor["x"] - EMITTER_DOWN, 4)
for volume in asset["volumes"].values():
    for solid in volume["thermal"]:
        if "emitter" in solid.get("tags", ()):
            solid["vertices"] = [[round(x - EMITTER_DOWN, 4), y, z] for x, y, z in solid["vertices"]]
points = [load["id"] for load in asset["poses"][0]["loads"]]
for clip in asset["clips"]:
    tracks = clip.get("tracks", [])
    factor = next((track for track in tracks if track["target"].startswith("load:wind-fol-")), None)
    if factor is not None:
        clip["tracks"] = [track for track in tracks if not track["target"].startswith("load:")] + [dict(factor, target=f"load:{i}") for i in points]


def profile(leaves):
    """📏 The crown radius per metre of height and 30 degree sector, measured on the foliage meshes."""
    bands = {}
    for g in leaves:
        for x, y, z in g["mesh"].v:
            row = bands.setdefault(int(y), [0.0] * 12)
            k = int((math.degrees(math.atan2(z, x)) % 360) / 30) % 12
            row[k] = max(row[k], math.hypot(x, z))
    return bands


K.write(os.path.join(K.OUT, "lombardy-poplar", "pre.json"), doc)
for scale in K.SCALES:
    wood, leaves = t.build(scale)
    pts = [p for g in wood + leaves for p in g["mesh"].v]
    print(f"  [DEBUG] 1:{scale} profile (height m: least / mean / largest crown radius over 12 sectors) " + "  ".join(f"{y}:{min(r):.1f}/{sum(r) / 12:.1f}/{max(r):.1f}" for y, r in sorted(profile(leaves).items())))
    print(f"  [DEBUG] 1:{scale} extent x {max(p[0] for p in pts) - min(p[0] for p in pts):.3f} z {max(p[2] for p in pts) - min(p[2] for p in pts):.3f} y {max(p[1] for p in pts):.3f}")


def silhouette(scale, axis, cell=0.1):
    """🖤 The filled side view of one scale as rows of cells (lateral axis 0 = x for the elevation, 2 = z for the front), for comparing two scales by eye."""
    wood, leaves = t.build(scale)
    grid = [[False] * 60 for _ in range(262)]
    for g in wood + leaves:
        v = [(p[axis] / cell + 30, p[1] / cell) for p in g["mesh"].v]
        for a, b, c in g["mesh"].f:
            (ax, ay), (bx, by), (cx, cy) = v[a], v[b], v[c]
            d = (by - cy) * (ax - cx) + (cx - bx) * (ay - cy)
            if abs(d) < 1e-12:
                continue
            for j in range(max(0, int(min(ay, by, cy))), min(261, int(max(ay, by, cy)) + 1)):
                for i in range(max(0, int(min(ax, bx, cx))), min(59, int(max(ax, bx, cx)) + 1)):
                    l1 = ((by - cy) * (i + 0.5 - cx) + (cx - bx) * (j + 0.5 - cy)) / d
                    l2 = ((cy - ay) * (i + 0.5 - cx) + (ax - cx) * (j + 0.5 - cy)) / d
                    if l1 >= 0 and l2 >= 0 and l1 + l2 <= 1:
                        grid[j][i] = True
    return grid


if os.environ.get("SILHOUETTE"):
    fine, coarse = (int(s) for s in os.environ["SILHOUETTE"].split(","))
    for axis in (0, 2):
        a, b = silhouette(fine, axis), silhouette(coarse, axis)
        print(f"[DEBUG] silhouette axis {'xz'[axis // 2]}: '#' both, '+' only 1:{coarse}, '-' only 1:{fine}; rows every 0.3 m from the top")
        for j in range(258, -1, -3):
            print(f"{j / 10:5.1f} " + "".join("#" if a[j][i] and b[j][i] else "+" if b[j][i] else "-" if a[j][i] else " " for i in range(60)))
