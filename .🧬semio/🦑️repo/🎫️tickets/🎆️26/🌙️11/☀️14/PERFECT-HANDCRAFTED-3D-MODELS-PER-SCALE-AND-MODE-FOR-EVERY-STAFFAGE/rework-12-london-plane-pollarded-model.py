"""🌳 Pollarded London plane (Platanus x hispanica, street tree cut back to the head every two to three years): the hand numbers of author 12. World x ahead (east), y up, z left (north)."""
import math
import os
from importlib.machinery import SourceFileLoader

A = SourceFileLoader("rework12assemble", os.path.join(os.path.dirname(os.path.abspath(__file__)), "rework-12-assemble.py")).load_module()
K = A.K

t = K.Tree("london-plane-pollarded", bark={"trunk": "plane-bark", "pollard": "plane-bark-scarred", "shoot": "plane-twig"}, wood="plane-wood", wood_density=830,
           regions={"trunk": "trunk-bark", "pollard": "pollard-bark", "shoot": "shoot-bark", "sun": "leaf-sun", "shade": "leaf-shade"},
           leaf_materials={"sun": "plane-leaf-sun", "shade": "plane-leaf-shade"}, tones={"trunk": 0.42, "pollard": 0.34, "shoot": 0.38, "sun": 0.6, "shade": 0.44})

STEM = [(0, 0, 0, 0.36), (0, 0.1, 0, 0.31), (0, 0.3, 0, 0.27), (0, 0.8, 0, 0.24), (0, 1.3, 0, 0.228), (0.01, 2.2, 0, 0.215), (0.01, 2.8, 0, 0.225), (0, 3.15, 0, 0.27), (0, 3.4, 0, 0.29), (0, 3.6, 0, 0.24)]
fy = K.heights(STEM)
t.axis("stem", STEM, kind="trunk", level=0, lobes=[(4, 0.08, 45, 0.2)], ground=True, bones=[
    ("root-plate", 0, fy(0.3), 0, "Root plate and collar", "Wurzelteller und Wurzelhals", "Root collar", "Wurzelhals"),
    ("trunk", fy(0.3), fy(2.8), 0, "Trunk (bole)", "Stamm (Schaft)", "Bole", "Schaft"),
    ("head", fy(2.8), 1.0, 0, "Pollard head", "Kopf (Kopfbaum)", "Head", "Kopf"),
])
for n, (az, reach, r0) in enumerate([(30, 0.95, 0.1), (120, 0.85, 0.09), (210, 0.95, 0.1), (300, 0.85, 0.09)], 1):
    d = K.direction(az, 0)
    t.axis(f"root-{n}", [(d[0] * 0.08, 0.32, d[2] * 0.08, r0), (d[0] * 0.4, 0.12, d[2] * 0.4, r0 * 0.8), (d[0] * reach * 0.7, 0.035, d[2] * reach * 0.7, r0 * 0.6), (d[0] * reach, 0.0, d[2] * reach, r0 * 0.45)], bone="root-plate", kind="trunk", level=2, aspect=1.4, ground=True, sides=(12, 8, 6, 6))

ARMS = [("w2", 176, 28, 1.5, "Outer west arm", "Äußerer Westarm"), ("w1", 118, 46, 1.3, "North-west arm", "Nordwestarm"), ("c", 40, 74, 1.1, "Centre arm", "Mittelarm"), ("e1", -52, 46, 1.3, "South-east arm", "Südostarm"), ("e2", -6, 28, 1.5, "Outer east arm", "Äußerer Ostarm"), ("n", 62, 34, 1.4, "North arm", "Nordarm"), ("s", -118, 34, 1.4, "South arm", "Südarm")]
SHOOTS = [(0, 0, 4.2, 0.028), (34, 0, 3.8, 0.026), (34, 72, 3.7, 0.025), (34, 144, 3.9, 0.026), (34, 216, 3.6, 0.024), (34, 288, 3.8, 0.025), (62, 36, 3.0, 0.021), (62, 180, 3.1, 0.022), (62, 300, 2.9, 0.02)]
LETTERS = "abcdefghi"
for key, az, el, length, en, de in ARMS:
    d = K.direction(az, el)
    o = (0.0, 3.3, 0.0)
    at = lambda f: (o[0] + d[0] * length * f, o[1] + d[1] * length * f, o[2] + d[2] * length * f)
    t.axis(f"arm-{key}", [at(0) + (0.15,), at(0.45) + (0.115,), at(0.78) + (0.12,), at(0.95) + (0.185,), at(1.08) + (0.17,)], kind="pollard", level=1, parent="head", sides=(18, 12, 8, 6), bones=[(f"k-{key}", 0, 1.0, 1, f"{en} with its knuckle", f"{de} mit Kopfknolle", "Arm", "Arm")])
    side = K.unit(K.cross(d, (0.0, 1.0, 0.0)))
    up = K.cross(side, d)
    knuckle = at(1.0)
    for n, (tilt, roll, slen, r0) in enumerate(SHOOTS):
        lean = tilt * (0.75 if el > 60 else 1.0)
        v = K.rotate(K.rotate(d, side, -lean), d, roll + az)
        v = K.unit((v[0], abs(v[1]) * 0.6 + 0.55, v[2]))
        vaz, vel = math.degrees(math.atan2(v[2], v[0])), math.degrees(math.asin(v[1]))
        sid = f"s-{key}{LETTERS[n]}"
        start = (knuckle[0] - v[0] * 0.05, knuckle[1] - v[1] * 0.05, knuckle[2] - v[2] * 0.05)
        pts = [start + (r0 * 1.3,)]
        p = start
        for i in range(5):
            f = (i + 0.5) / 5
            dd = K.direction(vaz, vel + 14 * f)
            p = K.add(p, K.mul(dd, slen / 5))
            pts.append(p + (r0 + (0.006 - r0) * ((i + 1) / 5) ** 0.9,))
        t.axis(sid, pts, kind="shoot", level=3, parent=f"k-{key}", bones=[(sid, 0, 1.0, 3, f"Shoot {LETTERS[n]} of the {A.low(en)}", f"Trieb {LETTERS[n]}: {de}", "Shoot", "Trieb")])
        t.axes[sid]["keep"] = n < 3
        inner = tilt == 0
        t.spray(f"fol-{sid}", sid, 0.2, 0.36, 0.34, (5, 6, 0.26), over=0.2, shade=False, scales=(50, 100) if n >= 6 else (50, 100, 200), flat=0.9, peak=0.5, base=0.6, group=key)
    t.spray(f"head-{key}", f"s-{key}a", 0.1, 1.35, 1.3, (6, 4, 0.16), over=0.3, scales=(500,), flat=0.9, peak=0.55, base=0.6, group=key)
for key in ("w1", "c", "e1", "n", "s"):
    t.mass(f"inner-{key}", f"k-{key}", [(lambda q: (q[0] * 0.55, q[1] + 0.25, q[2] * 0.55))(t.axes[f"arm-{key}"]["path"].at(1.0)[0]) + (0.5, 0.4), (lambda q: (q[0] * 0.7, q[1] + 0.8, q[2] * 0.7))(t.axes[f"arm-{key}"]["path"].at(1.0)[0]) + (0.75, 0.6), (lambda q: (q[0] * 0.8, q[1] + 1.4, q[2] * 0.8))(t.axes[f"arm-{key}"]["path"].at(1.0)[0]) + (0.6, 0.5)], (5, 3, 0.18), shade=True, scales=(50, 100, 200, 500), flat=0.9)

cfg = {
    "like": dict({f"k-{key}": {"n": "k-w1", "s": "k-e1"}.get(key, f"k-{key}") for key, *_ in ARMS}, **{f"s-{key}{LETTERS[n]}": "fol-top" if n == 0 else "fol-w1" for key, *_ in ARMS for n in range(len(SHOOTS))}),
    "leaf_density": 0.5,
    "group_bone": {key: f"s-{key}a" for key, *_ in ARMS},
    "emitters": {
        "transpiration-sun": {"on": ["fol-s-w2a", "fol-s-w1a", "fol-s-ca", "fol-s-e1a", "fol-s-e2a", "fol-s-na", "fol-s-sa"], "diameter": 0.6, "cls": "sun", "prefix": "tp-sun", "title": ("Transpiration of the sun leaves", "Transpiration der Sonnenblätter")},
        "transpiration-shade": {"on": ["inner-w1", "inner-c", "inner-e1", "inner-n", "inner-s"], "diameter": 0.6, "cls": "shade", "prefix": "tp-shade", "title": ("Transpiration of the shade leaves", "Transpiration der Schattenblätter")},
    },
    "scaffolds": ["arm-w2", "arm-e2", "arm-n", "arm-s"],
    "symbol_extra": lambda view, shape, tree: dict({k: v for k, v in shape.items() if k not in ("points",)}, points=[[round(1.75 * math.cos(math.radians(a)), 4), round(1.75 * math.sin(math.radians(a)), 4)] for a in range(0, 360, 15)]) if shape["id"] == "head-ring" else None,
    "habit": {
        50: ("the plane is built as a pollard is. A collar of 0.72 m with four root spurs carries a straight, smooth bole of 0.46 m at breast height that swells into the head at 3.3 m; seven short arms of 0.23 m leave the head all round and each ends in its knuckle, the swollen callus of decades of cutting, 0.37 m across. From every knuckle nine straight shoots of two seasons spring like a broom, the middle one upright, the others leaning out and turning up, 2.9 to 4.2 m long and 40 to 56 mm thick at the base, each a bone of its own. The large leaves sit along the upper three quarters of every shoot as one loose mass per shoot; shaded leaf masses fill the middle above five knuckles.", "Die Platane ist gebaut, wie ein Kopfbaum es ist. Ein Wurzelhals von 0,72 m mit vier Wurzelanläufen trägt einen geraden, glatten Schaft von 0,46 m in Brusthöhe, der in 3,3 m zum Kopf anschwillt; sieben kurze Arme von 0,23 m verlassen den Kopf rundum, und jeder endet in seiner Kopfknolle, dem verdickten Kallus jahrzehntelangen Schnitts, 0,37 m stark. Aus jeder Knolle entspringen besenartig neun gerade zweijährige Triebe, der mittlere aufrecht, die anderen nach außen geneigt und aufgebogen, 2,9 bis 4,2 m lang und an der Basis 40 bis 56 mm dick, jeder ein eigener Knochen. Die großen Blätter sitzen entlang der oberen drei Viertel jedes Triebes als eine lockere Masse je Trieb; beschattete Laubmassen füllen die Mitte über fünf Knollen."),
        100: ("the same bole, head, seven arms with knuckles and sixty-three shoots, none thinner than 37 mm, each shoot with its leaf mass; relief and tessellation are those of the coarser scale.", "Derselbe Schaft, Kopf, sieben Arme mit Knollen und dreiundsechzig Triebe, keiner dünner als 37 mm, jeder Trieb mit seiner Laubmasse; Relief und Auflösung sind die des gröberen Maßstabs."),
        200: ("bole, head, arms with knuckles and six shoots per knuckle, each with its leaf mass, and the shaded masses over the knuckles.", "Schaft, Kopf, Arme mit Knollen und sechs Triebe je Knolle, jeder mit seiner Laubmasse, dazu die beschatteten Massen über den Knollen."),
        500: ("bole, head, arms with knuckles and three stout shoots per knuckle; the foliage of each knuckle is one mass along its middle shoot around the shaded masses.", "Schaft, Kopf, Arme mit Knollen und drei kräftige Triebe je Knolle; das Laub jeder Knolle ist eine Masse entlang des mittleren Triebes um die beschatteten Massen."),
    },
    "drawing": {
        50: ("smooth bole, swollen head, seven arms with knuckles, sixty-three shoots and the leaf masses of the shoots of every knuckle as one outline with its openings.", "glatter Schaft, verdickter Kopf, sieben Arme mit Knollen, dreiundsechzig Triebe und die Laubmassen der Triebe jeder Knolle als ein Umriss mit seinen Lücken."),
        100: ("bole, head, arms, shoots and the leaf mass of every shoot.", "Schaft, Kopf, Arme, Triebe und die Laubmasse jedes Triebes."),
        200: ("bole, head, arms and six leafy shoots per knuckle.", "Schaft, Kopf, Arme und sechs belaubte Triebe je Knolle."),
        500: ("bole, head with arms and one foliage mass per knuckle.", "Schaft, Kopf mit Armen und eine Laubmasse je Knolle."),
    },
}
A.assemble(t, cfg)
