"""🌳 Lombardy poplar (Populus nigra 'Italica', about 45 years): the hand numbers of author 12. World x ahead (east), y up, z left (north)."""
import os
from importlib.machinery import SourceFileLoader

A = SourceFileLoader("rework12assemble", os.path.join(os.path.dirname(os.path.abspath(__file__)), "rework-12-assemble.py")).load_module()
K = A.K

t = K.Tree("lombardy-poplar", bark="poplar-bark", wood="poplar-wood", wood_density=800, regions={"trunk": "stem-bark", "limb": "branch-bark", "sun": "leaf-crown"}, leaf_materials={"sun": "poplar-leaf"}, tones={"trunk": 0.3, "limb": 0.34, "sun": 0.56})

STEM = [(0, 0, 0, 0.55), (0, 0.15, 0, 0.47), (0, 0.45, 0, 0.4), (0, 0.9, 0, 0.36), (0.01, 1.3, 0, 0.34), (0.02, 3.0, 0.01, 0.315), (0.02, 6.0, 0, 0.28), (0, 9.0, -0.02, 0.245), (-0.02, 12.0, 0, 0.205), (0, 15.0, 0.02, 0.165), (0.02, 18.0, 0.01, 0.125), (0.01, 21.0, 0, 0.085), (0, 23.0, 0, 0.055), (0, 24.6, 0, 0.03), (0, 25.6, 0, 0.014), (0, 26.0, 0, 0.008)]
fy = K.heights(STEM)
t.axis("stem", STEM, kind="trunk", level=0, ridges=(20, 0.012), lobes=[(6, 0.12, 15, 0.35), (4, 0.05, 70, 0.6)], ground=True, bones=[
    ("root-plate", 0, fy(0.45), 0, "Root plate and fluted collar", "Wurzelteller und gefurchter Wurzelhals", "Root collar", "Wurzelhals"),
    ("trunk-1", fy(0.45), fy(6.0), 0, "Stem, lowest quarter", "Stamm, unterstes Viertel", "Stem", "Stamm"),
    ("trunk-2", fy(6.0), fy(12.0), 0, "Stem, second quarter", "Stamm, zweites Viertel", "Stem", "Stamm"),
    ("trunk-3", fy(12.0), fy(18.0), 0, "Stem, third quarter", "Stamm, drittes Viertel", "Stem", "Stamm"),
    ("trunk-4", fy(18.0), fy(23.0), 1, "Stem, top quarter", "Stamm, oberstes Viertel", "Stem", "Stamm"),
    ("fol-top", fy(23.0), 1.0, 3, "Leader tip with its foliage", "Wipfelspitze mit Laub", "Leader tip", "Wipfelspitze"),
])
for n, (az, reach, r0) in enumerate([(15, 1.6, 0.15), (75, 1.4, 0.13), (135, 1.5, 0.14), (195, 1.4, 0.13), (255, 1.6, 0.15), (315, 1.4, 0.13)], 1):
    d = K.direction(az, 0)
    t.axis(f"root-{n}", [(d[0] * 0.15, 0.5, d[2] * 0.15, r0), (d[0] * 0.6, 0.2, d[2] * 0.6, r0 * 0.8), (d[0] * reach * 0.7, 0.05, d[2] * reach * 0.7, r0 * 0.6), (d[0] * reach, 0.0, d[2] * reach, r0 * 0.42)], bone="root-plate", kind="trunk", level=2, aspect=1.5, ground=True, sides=(12, 8, 6, 6))

HEIGHTS = [1.7, 2.4, 3.1, 3.8, 4.6, 5.3, 6.0, 6.7, 7.5, 8.2, 8.9, 9.6, 10.4, 11.1, 11.8, 12.5, 13.3, 14.0, 14.7, 15.4, 16.2, 16.9, 17.6, 18.3, 19.1, 19.8, 20.5, 21.2, 22.0, 22.7]
AZIMUTHS = [20, 160, 304, 64, 205, 340, 125, 245, 30, 170, 305, 60, 215, 350, 130, 240, 40, 180, 310, 55, 200, 330, 120, 250, 25, 165, 300, 65, 210, 345]
LENGTH = [(1.7, 5.4), (4.0, 6.8), (8.0, 6.6), (12.0, 5.4), (16.0, 4.0), (20.0, 2.6), (22.7, 1.5)]
RADIUS = [(1.7, 0.075), (8.0, 0.062), (14.0, 0.042), (20.0, 0.024), (22.7, 0.016)]
RISE = [(1.7, 64), (12.0, 67), (22.7, 70)]


def stated(table, y):
    """📐 The value a hand table of (height, value) states at the height y, between its rows."""
    for (y0, v0), (y1, v1) in zip(table, table[1:]):
        if y <= y1:
            return v0 + (v1 - v0) * (y - y0) / (y1 - y0)
    return table[-1][1]


LATERALS = [(0.3, 40, 68, 1.5, 0.018), (0.48, -40, 70, 1.4, 0.017), (0.64, 36, 72, 1.2, 0.015), (0.8, -36, 74, 1.0, 0.013)]
like = {}
for n, (y, az) in enumerate(zip(HEIGHTS, AZIMUTHS), 1):
    key = f"{n:02d}"
    length, r0, el = stated(LENGTH, y), stated(RADIUS, y), stated(RISE, y)
    quarter = min(4, 1 + (n - 1) // 8)
    like[f"br-{key}"] = f"br-{'l' if n % 2 else 'r'}{quarter}"
    t.branch(f"b{key}", "stem", fy(y), az, el, length, r0, r1=0.008, up=16, turn=10 if n % 2 else -10, level=1, socket=True, bones=[
        (f"br-{key}", 0, 1.0, 2, f"Upright branch {n} at {y:g} m", f"Steilast {n} in {str(y).replace('.', ',')} m", "Branch", "Ast"),
    ])
    k = length / 6.8
    for m, (at, daz, lel, llen, lr0) in enumerate(LATERALS, 1):
        lid = f"b{key}-l{m}"
        t.branch(lid, f"b{key}", at, az + daz, lel, llen * k + 0.3, max(0.0095, lr0 * k ** 0.5), r1=0.008, up=10, level=3, scales=(50, 100))
        t.spray(f"fol-{lid}", lid, 0.15, 0.42 + 0.14 * k, 0.4 + 0.12 * k, (5, 3, 0.18), over=0.25, scales=(50,), flat=1.0, group=f"b{key}")
    t.spray(f"fol-b{key}", f"b{key}", 0.5, 0.5 + 0.25 * k, 0.48 + 0.24 * k, (5, 3, 0.17), over=0.45, scales=(50,), flat=1.0, group=f"b{key}")
    t.spray(f"fol-b{key}", f"b{key}", 0.18, 0.6 + 0.45 * k, 0.58 + 0.42 * k, (5, 4, 0.16), over=0.5, scales=(100,), flat=1.0, peak=0.5, base=0.55, group=f"b{key}")
    t.spray(f"fol-b{key}", f"b{key}", 0.16, 0.65 + 0.45 * k, 0.62 + 0.42 * k, (5, 3, 0.15), over=0.5, scales=(200,), flat=1.0, peak=0.5, base=0.55, group=f"b{key}")
    if n % 2:
        t.spray(f"crown-b{key}", f"b{key}", 0.1, 0.8 + 0.6 * k, 0.8 + 0.55 * k, (5, 3, 0.13), over=0.7, scales=(500,), flat=1.0, peak=0.5, base=0.6, group=f"b{key}")
t.spray("fol-top-shoot", "stem", fy(22.8), 0.5, 0.5, (5, 4, 0.16), over=0.2, scales=(50, 100), bone="fol-top", flat=1.0, peak=0.35, base=0.7, group="top")
t.spray("fol-top-shoot", "stem", fy(22.2), 0.7, 0.7, (5, 3, 0.14), over=0.2, scales=(200, 500), bone="fol-top", flat=1.0, peak=0.35, base=0.7, group="top")

cfg = {
    "like": like,
    "leaf_density": 0.4,
    "group_bone": dict({f"b{n:02d}": f"br-{n:02d}" for n in range(1, 31)}, top="fol-top"),
    "emitters": {"transpiration-crown": {"on": [f"fol-b{n:02d}" for n in (2, 5, 8, 11, 14, 17, 20, 23, 26)], "diameter": 0.5, "cls": "sun", "prefix": "tp-crown", "title": ("Transpiration of the crown leaves", "Transpiration der Kronenblätter")}},
    "scaffolds": ["b04", "b05", "b06", "b07"],
    "habit": {
        50: ("the poplar is built as it grows. A fluted collar of 1.1 m with six buttress roots carries a furrowed stem of 0.68 m at breast height that runs as one axis to the tip at 26 m. Thirty branches leave it from 1.7 m upwards in a spiral, each at 64 to 70 degrees and at once turning up to more than 80, so that they stand almost parallel to the stem, 6.8 m long in the lower third and ever shorter above; each carries four upright laterals. The leaves clothe the upper part of every branch and every lateral as slender upright masses, so the crown is a column of 4 m of overlapping flames with narrow vertical clefts, pointed at the top.", "Die Pappel ist gebaut, wie sie wächst. Ein gefurchter Wurzelhals von 1,1 m mit sechs Wurzelanläufen trägt einen gefurchten Stamm von 0,68 m in Brusthöhe, der als eine Achse bis zur Spitze in 26 m durchläuft. Dreißig Äste verlassen ihn ab 1,7 m in einer Spirale, jeder mit 64 bis 70 Grad und sofort auf über 80 Grad aufgerichtet, sodass sie fast parallel zum Stamm stehen, im unteren Drittel 6,8 m lang und nach oben immer kürzer; jeder trägt vier aufrechte Seitenäste. Die Blätter bekleiden den oberen Teil jedes Astes und jedes Seitenastes als schlanke aufrechte Massen, die Krone ist so eine Säule von 4 m aus sich überdeckenden Flammen mit schmalen senkrechten Spalten, oben spitz."),
        100: ("the same stem and thirty upright branches with their laterals down to 37 mm; each branch carries one upright foliage mass from its first lateral to beyond its tip.", "Derselbe Stamm und dreißig Steiläste mit ihren Seitenästen bis 37 mm; jeder Ast trägt eine aufrechte Laubmasse vom ersten Seitenast bis über die Spitze hinaus."),
        200: ("stem and the thirty upright branches (thinner than 85 mm as slender solids of true thickness), each with one upright foliage mass.", "Stamm und die dreißig Steiläste (dünner als 85 mm als schlanke Körper in wahrer Dicke), jeder mit einer aufrechten Laubmasse."),
        500: ("stem and upright branches (as slender solids of true thickness), and the foliage of every second branch as one broad upright mass: the closed column with its pointed top.", "Stamm und Steiläste (als schlanke Körper in wahrer Dicke), und das Laub jedes zweiten Astes als eine breite aufrechte Masse: die geschlossene Säule mit spitzem Wipfel."),
    },
    "drawing": {
        50: ("fluted collar and stem, thirty upright branches with laterals and the upright leaf masses of every branch as one outline.", "gefurchter Wurzelhals und Stamm, dreißig Steiläste mit Seitenästen und die aufrechten Laubmassen jedes Astes als ein Umriss."),
        100: ("stem, upright branches with laterals and one foliage outline per branch bone.", "Stamm, Steiläste mit Seitenästen und ein Laubumriss je Astknochen."),
        200: ("stem, upright branches and one foliage mass per branch.", "Stamm, Steiläste und eine Laubmasse je Ast."),
        500: ("stem and the column of fifteen upright foliage masses with the top.", "Stamm und die Säule aus fünfzehn aufrechten Laubmassen mit dem Wipfel."),
    },
}
A.assemble(t, cfg)
