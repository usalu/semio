"""🧺 Round 3 storage crate stack: root fixes outside `volumes` (references, dimensions, rig with turning crates, anchors, poses, clips, structure text, 2D drawings of every scale) stated by hand.

Usage: python3 r3-storage-crate-stack-roots.py <target asset file>   (the catalog file or a candidate under 🗑️generated)
"""
import importlib.util
import json
import os
import sys
from pathlib import Path

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-storage-crate-stack-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

CAT = Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/storage-crate-stack/🔣️.json")
N = 5
G = 9.80665


def text(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


REFERENCES = [
    {"id": "rotom-69751", "citation": "Rotom Deutschland GmbH, product data sheet SKU 69751: rotating stack-nest container (Drehstapelbehälter) 50 l, closed walls and floor, two open hand holes, PP; outer 600 x 400 x 300 mm, inner 490 x 332 x 295 mm, 2.14 kg (rotom.de, retrieved 2026-11-20)."},
    {"id": "transoplast-9567720765", "citation": "Transoplast, product data sheet 9567.720.765: stack-nest crate Euronorm 600 x 400 x 300 mm, HDPE, turned by 180 degrees to stack, nests when empty; inner 457 x 335 x 271 mm, 50 l, 2.16 kg, nesting ratio 72 %, maximum load 15 kg per crate, static stack load 200 kg (transoplast.de, retrieved 2026-11-20)."},
]

NOTES = text(
    "Side elevation of five rotating stack-nest crates (Drehstapelbehälter) of the 600 mm by 400 mm module (ISO 3394), 0.30 m high, injection-moulded HDPE with 3 mm walls, 2.15 kg each (Rotom 69751: 2.14 kg, Transoplast 9567.720.765: 2.16 kg) and 10.75 kg in all. The walls taper from 0.53 x 0.33 m at the foot of the wall to 0.59 x 0.37 m under the rim; the rim is an outward U-band (flange and 38 mm skirt) of 0.60 x 0.40 m; a 14 mm foot ring 0.52 x 0.32 m carries the floor; each end wall has an open hand hole of 120 x 35 mm under the rim. Three stacking ledges sit at the top inside the crate (one in each end wall, one in the far long wall), and the wall has a matching vertical channel under each ledge. Turned by half a turn against the crate below, a crate stands with its foot on the three ledges (pitch 0.30 m); in the same orientation the ledges pass through its channels and it nests, so that only 0.08 m of its height shows above the one below (Transoplast nesting ratio 72 % gives 0.084 m). Every crate turns about the vertical on its own bone; a zero-length seat bone carries the nesting travel and a lift of 0.02 m for turning. The origin is the middle of the lowest crate's base on the floor. The crates have no heat source and exchange heat with the room; a nested crate exposes only 30 percent of its surface. Loaded poses carry 10 kg per crate (Transoplast: at most 15 kg) as carried loads.",
    "Seitenansicht von fünf Drehstapelbehältern des Moduls 600 mm mal 400 mm (ISO 3394), 0,30 m hoch, PE-HD-Spritzguss mit 3 mm Wand, je 2,15 kg (Rotom 69751: 2,14 kg, Transoplast 9567.720.765: 2,16 kg) und zusammen 10,75 kg. Die Wände weiten sich von 0,53 x 0,33 m am Wandfuß auf 0,59 x 0,37 m unter dem Rand; der Rand ist ein nach außen offenes U-Band (Flansch und 38 mm Schürze) von 0,60 x 0,40 m; ein 14 mm hoher Fußring von 0,52 x 0,32 m trägt den Boden; jede Stirnwand hat unter dem Rand ein offenes Griffloch von 120 x 35 mm. Drei Stapelleisten sitzen oben innen in der Kiste (je eine in den Stirnwänden, eine in der hinteren Längswand), und die Wand hat unter jeder Leiste eine passende senkrechte Nut. Um eine halbe Drehung gegen die untere Kiste gedreht, steht eine Kiste mit dem Fuß auf den drei Leisten (Raster 0,30 m); in gleicher Ausrichtung gleiten die Leisten durch ihre Nuten und sie stellt sich hinein, sodass nur 0,08 m ihrer Höhe über der unteren herausragen (Transoplast-Nestingquote 72 % ergibt 0,084 m). Jede Kiste dreht sich auf ihrem eigenen Knochen um die Senkrechte; ein Sitzknochen ohne Länge trägt den Einstellweg und ein Anheben um 0,02 m zum Drehen. Der Ursprung ist die Mitte der Unterseite der untersten Kiste auf dem Boden. Die Kisten haben keine Wärmequelle und tauschen Wärme mit dem Raum aus; eine ineinandergestellte Kiste legt nur 30 Prozent ihrer Fläche frei. Beladene Stellungen tragen 10 kg je Kiste (Transoplast: höchstens 15 kg) als mitgeführte Lasten.",
)

SUMMARY = text(
    "Five rotating stack-nest plastic crates of 600 by 400 by 300 mm in side elevation: turned against each other and stacked loaded on their stacking ledges to 1.5 m, or nested empty to 0.62 m, with a half-high pose in between.",
    "Fünf Drehstapelbehälter aus Kunststoff von 600 mal 400 mal 300 mm in der Seitenansicht: gegeneinander gedreht und beladen auf ihren Stapelleisten 1,5 m hoch gestapelt oder leer auf 0,62 m ineinandergestellt, mit einer halbhohen Stellung dazwischen.",
)


def mass():
    """⚖️ The moulding mass of one crate at its centre (floor and foot low, rim high)."""
    return {"kg": 2.15, "comX": K.COM_U, "comY": 0, "distribution": "point", "source": "literature", "reference": "rotom-69751"}


def rig():
    """🦴 Crate 1 stands on the ground with its bone up the crate; each further crate sits on a zero-length seat bone at the rim of the one below (nesting travel and lift) and turns about the vertical on its own bone."""
    bones = [{"id": "crate-1", "parent": None, "x": 0, "y": 0, "angleDeg": 90, "title": text("Crate 1", "Kiste 1"), "lengthM": K.PITCH, "joint": {"type": "fixed"}, "mass": mass()}]
    for k in range(2, N + 1):
        bones.append({"id": f"seat-{k}", "parent": f"crate-{k - 1}", "x": K.PITCH, "y": 0, "angleDeg": 0, "title": text(f"Seat of crate {k} (nesting travel)", f"Sitz der Kiste {k} (Einstellweg)"), "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": -K.NEST, "maxM": K.LIFT}})
        bones.append({"id": f"crate-{k}", "parent": f"seat-{k}", "x": 0, "y": 0, "angleDeg": 0, "title": text(f"Crate {k} (turns about the vertical)", f"Kiste {k} (dreht um die Senkrechte)"), "lengthM": K.PITCH, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": 0, "twistMaxDeg": 180}, "mass": mass()})
    return {"kind": "mechanical", "root": "crate-1", "bones": bones}


def goods(ks):
    """📦 10 kg of goods per loaded crate, a carried load 0.1 m above its floor."""
    return [{"id": f"goods-{k}", "kind": "carried", "title": text(f"Goods in crate {k} (10 kg)", f"Ware in Kiste {k} (10 kg)"), "bone": f"crate-{k}", "x": 0.1, "y": 0, "fxN": 0, "fyN": -round(10 * G, 4), "source": "authored-estimate"} for k in ks]


def regions(nested):
    """🌡️ Exposure per crate: a nested crate shows 30 percent of its surface, the lowest crate touches the floor with its foot."""
    out = {}
    for k in range(1, N + 1):
        entry = {"exposedFraction": 0.3 if k in nested else 1, "sunlitFraction": 0}
        if k == 1:
            entry["contact"] = {"fraction": 0.05, "tempC": 20}
        out[f"crate-{k}"] = entry
    return {"regions": out, "emitters": {}}


TURN = {"twistDeg": 180}
DOWN = {"dx": -K.NEST}
UP = {"dx": K.LIFT}


def poses():
    """🧍 High: every crate turned by half a turn against the one below and stacked on its ledges. Half: crates 4 and 5 nested into crate 3. Low: all nested, same orientation."""
    return [
        {"id": "high", "title": text("High stack", "Hoher Stapel"),
         "description": text("All five crates are stacked, each turned by half a turn against the one below so that its foot rests on the three stacking ledges: 1.5 m high, each crate carrying 10 kg of goods.",
                             "Alle fünf Kisten sind gestapelt, jede um eine halbe Drehung gegen die untere gedreht, sodass ihr Fuß auf den drei Stapelleisten steht: 1,5 m hoch, jede Kiste trägt 10 kg Ware."),
         "bones": {f"crate-{k}": dict(TURN) for k in range(2, N + 1)}, "ground": "auto", "contacts": ["base-contact"], "thermal": regions(set()), "loads": goods(range(1, N + 1))},
        {"id": "half", "title": text("Half-high stack", "Halbhoher Stapel"),
         "description": text("Three crates are stacked with goods, turned against each other; the two top crates are empty, turned like the third and nested into it: 1.06 m high.",
                             "Drei Kisten sind beladen und gegeneinander gedreht gestapelt; die zwei oberen Kisten sind leer, wie die dritte ausgerichtet und in sie gestellt: 1,06 m hoch."),
         "bones": {"crate-2": dict(TURN), "crate-3": dict(TURN), "seat-4": dict(DOWN), "seat-5": dict(DOWN)}, "ground": "auto", "contacts": ["base-contact"], "thermal": regions({4, 5}), "loads": goods(range(1, 4))},
        {"id": "low", "title": text("Low stack", "Niedriger Stapel"),
         "description": text("All five empty crates face the same way and are nested into each other: 0.62 m high.",
                             "Alle fünf leeren Kisten sind gleich ausgerichtet und ineinandergestellt: 0,62 m hoch."),
         "bones": {f"seat-{k}": dict(DOWN) for k in range(2, N + 1)}, "ground": "auto", "contacts": ["base-contact"], "thermal": regions({2, 3, 4, 5}), "loads": []},
    ]


def key(t, pose, bones=None):
    """🔑 A clip key with optional bone overrides."""
    out = {"t": t, "pose": pose, "ease": "smooth"}
    if bones:
        out["bones"] = bones
    return out


def clips(old):
    """🎬 Unload: lift crate 4 (with 5 on it), turn it, nest it into 3; lift, turn and nest 5; then the same for 2 (with 3 to 5) into 1 and for 3 into 2. Load is the reverse."""
    unload = [
        key(0, "high"),
        key(1, "high", {"seat-4": UP}),
        key(2.5, "high", {"seat-4": UP, "crate-4": {"twistDeg": 0}}),
        key(3.5, "half", {"seat-5": {"dx": 0}, "crate-5": TURN}),
        key(4.25, "half", {"seat-5": UP, "crate-5": TURN}),
        key(5.25, "half", {"seat-5": UP}),
        key(6, "half"),
        key(7, "half", {"seat-2": UP}),
        key(8.5, "half", {"seat-2": UP, "crate-2": {"twistDeg": 0}}),
        key(9.5, "low", {"seat-3": {"dx": 0}, "crate-3": TURN}),
        key(10.25, "low", {"seat-3": UP, "crate-3": TURN}),
        key(11.25, "low", {"seat-3": UP}),
        key(12, "low"),
        key(14, "low"),
    ]
    load = [
        key(0, "low"),
        key(1, "low", {"seat-3": UP}),
        key(2, "low", {"seat-3": UP, "crate-3": TURN}),
        key(2.75, "low", {"seat-3": {"dx": 0}, "crate-3": TURN}),
        key(3.75, "low", {"seat-3": {"dx": 0}, "crate-3": TURN, "seat-2": UP}),
        key(5, "half", {"seat-2": UP}),
        key(6, "half"),
        key(7, "half", {"seat-5": UP}),
        key(8, "half", {"seat-5": UP, "crate-5": TURN}),
        key(8.75, "half", {"seat-5": {"dx": 0}, "crate-5": TURN}),
        key(9.75, "half", {"seat-5": {"dx": 0}, "crate-5": TURN, "seat-4": UP}),
        key(11, "high", {"seat-4": UP}),
        key(12, "high"),
        key(14, "high"),
    ]
    out = []
    for clip in old:
        c = dict(clip)
        if c["id"] == "unload-stack":
            c["keys"] = unload
            c["description"] = text("The goods are taken out and the crates are nested: crate 4 is lifted off its ledges with crate 5 on it, turned by half a turn and lowered into crate 3, then crate 5 into crate 4; the same for crate 2 into crate 1 and crate 3 into crate 2. The stack drops from 1.5 m over 1.06 m to 0.62 m in twelve seconds.",
                                    "Die Ware wird entnommen und die Kisten werden ineinandergestellt: Kiste 4 wird mit Kiste 5 darauf von ihren Leisten gehoben, um eine halbe Drehung gedreht und in Kiste 3 gesenkt, dann Kiste 5 in Kiste 4; ebenso Kiste 2 in Kiste 1 und Kiste 3 in Kiste 2. Der Stapel sinkt in zwölf Sekunden von 1,5 m über 1,06 m auf 0,62 m.")
        elif c["id"] == "load-stack":
            c["keys"] = load
            c["description"] = text("The empty nested crates are pulled out, turned by half a turn and set on the stacking ledges of the crate below, then loaded: the stack rises from 0.62 m to 1.5 m in twelve seconds.",
                                    "Die leeren ineinandergestellten Kisten werden herausgehoben, um eine halbe Drehung gedreht und auf die Stapelleisten der unteren Kiste gesetzt, dann beladen: der Stapel wächst in zwölf Sekunden von 0,62 m auf 1,5 m.")
        out.append(c)
    return out


def rect(sid, bone, role, z, filled, tone, x, y, w, h, r=None, region=True, k=None):
    """▭ A rectangle shape (region and material on filled crate shapes)."""
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": "rect", "x": K.r6(x), "y": K.r6(y), "w": K.r6(w), "h": K.r6(h)}
    if r:
        s["r"] = r
    if region:
        s["region"] = bone
        if filled:
            s["material"] = "hdpe-crate"
    return s


def poly(sid, bone, role, z, filled, tone, pts, closed=True, region=True):
    """⬠ A polygon or polyline shape."""
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": "poly", "points": [[K.r6(a), K.r6(b)] for a, b in pts], "closed": closed}
    if region:
        s["region"] = bone
        if filled:
            s["material"] = "hdpe-crate"
    return s


def tones(k):
    """🎨 Alternating crate tones so that neighbouring crates separate (body, rim, foot)."""
    return (0.45, 0.55, 0.35) if k % 2 else (0.55, 0.65, 0.45)


def solid(shape, half, half_tip=None):
    """🧊 Makes a side-elevation shape a laterally extruded solid (span +-half at the bone origin, +-half_tip at the bone tip) so that front and plan follow the turn of the crate."""
    shape["solid"] = {"kind": "extruded", "sweep": "w", "z0": K.r6(-half), "z1": K.r6(half)}
    if half_tip is not None:
        shape["solid"]["zTip"] = [K.r6(-half_tip), K.r6(half_tip)]
    return shape


def views(solids, front=(), plan=(), front_solids=True):
    """🪟 The same solid shapes in all three viewpoints (the product projects them), plus the view-only details; at 1:50 the front is drawn by hand because the end walls have the hand holes, through which the stack is seen."""
    return {"elevation": [dict(x) for x in solids], "front": ([dict(x) for x in solids] if front_solids else []) + list(front), "plan": [plan_z(x) for x in plan_rims(solids)] + [plan_z(x) for x in plan]}


def plan_rims(solids):
    """⬜ Seen from above only the rim of each crate shows: a rounded rectangle of 0.60 x 0.40 m with the moulded corner radius of 30 mm (the extruded solid would give square corners)."""
    out = []
    for x in solids:
        if x["id"].endswith("-rim"):
            out.append(rect(x["id"], x["bone"], "body", x["z"], True, x["tone"], -0.30, -0.20, 0.60, 0.40, r=0.03))
    return out


def plan_z(shape):
    """🔝 In plan the upper crate covers the lower one: the layer order of a crate's shapes is turned over (crate 5 on top)."""
    k = int(shape["bone"].split("-")[1])
    out = dict(shape)
    out["z"] = round(shape["z"] - (5 - 0.1 * k) + 4.3 + 0.1 * k, 3)
    return out


def wall_span():
    """↔️ Lateral half span of the tapering wall at the bone origin and at the bone tip (the taper is linear in u)."""
    return K.HW * K.f(0), K.HW * K.f(K.PITCH)


def end_wall(sid, k, tone, z, u0, u1):
    """🚪 Front view of a crate's end wall from u0 to u1 (it tapers like the wall) with the open hand hole as a hole of the shape, through which the stack is seen (the hole runs the other way round so that even-odd and non-zero fill agree)."""
    y0 = K.PITCH * (k - 1)
    g, a, c = K.HAND["g"], y0 + K.HAND["u0"], y0 + K.HAND["u1"]
    b0, b1 = K.HW * K.f(u0), K.HW * K.f(u1)
    ring = f"M {K.r6(-b0)} {K.r6(y0 + u0)} L {K.r6(b0)} {K.r6(y0 + u0)} L {K.r6(b1)} {K.r6(y0 + u1)} L {K.r6(-b1)} {K.r6(y0 + u1)} Z"
    hole = f"M {-g + 0.008} {K.r6(a)} L {-g} {K.r6(a + 0.008)} L {-g} {K.r6(c - 0.008)} L {-g + 0.008} {K.r6(c)} L {g - 0.008} {K.r6(c)} L {g} {K.r6(c - 0.008)} L {g} {K.r6(a + 0.008)} L {g - 0.008} {K.r6(a)} Z"
    b = f"crate-{k}"
    return {"id": sid, "bone": b, "role": "body", "z": z, "filled": True, "tone": tone, "kind": "path", "d": ring + " " + hole, "region": b, "material": "hdpe-crate"}


def geometry50():
    """✏️ 1:50: foot ring, tapering wall, rim band as solids; front: hand hole; plan: the opening inside the flange, the floor with its two ribs, three stacking ledges."""
    sol, fr, pl = [], [], []
    h0, h1 = wall_span()
    top_v = K.HV * K.f(0.262)
    for k in range(1, N + 1):
        b = f"crate-{k}"
        body, rim, foot = tones(k)
        z = 5 - 0.1 * k
        sol += [
            solid(rect(f"c{k}-foot", b, "body", round(z - 0.1, 3), True, foot, 0, -0.26, 0.014, 0.52), 0.16),
            solid(poly(f"c{k}-body", b, "body", round(z - 0.09, 3), True, body, [(K.U0, -K.HV), (0.262, -top_v), (0.262, top_v), (K.U0, K.HV)]), h0, h1),
            solid(rect(f"c{k}-rim", b, "body", round(z - 0.08, 3), True, rim, 0.262, -0.30, 0.038, 0.60), 0.20),
        ]
        y0 = K.PITCH * (k - 1)
        fr += [rect(f"c{k}-foot", b, "body", round(z - 0.1, 3), True, foot, -0.16, y0, 0.32, 0.014), end_wall(f"c{k}-body", k, body, round(z - 0.09, 3), K.U0, 0.262), rect(f"c{k}-rim", b, "body", round(z - 0.08, 3), True, rim, -0.20, y0 + 0.262, 0.40, 0.038)]
        zp = round(z - 0.05, 3)
        fl = K.f(0.02)
        e1, e3 = K.FINE["edge1"], K.FINE["edge3"]
        w0, w1, _ = K.FINE["p1"]
        v0, v1, _ = K.FINE["p3"]
        o1 = K.HV * K.f(K.FINE["ledge_u"])
        o3 = K.HW * K.f(K.FINE["ledge_u"])
        pl += [
            rect(f"c{k}-opening", b, "detail", zp, True, round(body - 0.1, 2), -K.FLANGE_HOLE["hv"], -K.FLANGE_HOLE["hw"], 2 * K.FLANGE_HOLE["hv"], 2 * K.FLANGE_HOLE["hw"], r=0.02),
            rect(f"c{k}-floor", b, "detail", round(zp + 0.005, 3), True, 0.3 if k % 2 else 0.4, -(K.HV - K.T) * fl, -(K.HW - K.T) * fl, 2 * (K.HV - K.T) * fl, 2 * (K.HW - K.T) * fl, r=0.02),
            poly(f"c{k}-floor-rib-x", b, "detail", round(zp + 0.006, 3), False, 0.3, [(-0.23, 0), (0.23, 0)], closed=False),
            poly(f"c{k}-floor-rib-y", b, "detail", round(zp + 0.006, 3), False, 0.3, [(0, -0.13), (0, 0.13)], closed=False),
            rect(f"c{k}-ledge-a", b, "detail", round(zp + 0.007, 3), True, 0.7, -o1, w0, o1 - e1, w1 - w0),
            rect(f"c{k}-ledge-b", b, "detail", round(zp + 0.007, 3), True, 0.7, e1, w0, o1 - e1, w1 - w0),
            rect(f"c{k}-ledge-c", b, "detail", round(zp + 0.007, 3), True, 0.7, v0, -o3, v1 - v0, o3 - e3),
        ]
    return views(sol, fr, pl, front_solids=False)


def geometry100():
    """✏️ 1:100: tapering body from the floor and a rim band of 25 mm as solids; plan: the opening inside the flange."""
    sol, fr, pl = [], [], []
    h0, h1 = wall_span()
    tv, bv = K.HV * K.f(0.275), K.HV * K.f(0)
    for k in range(1, N + 1):
        b = f"crate-{k}"
        body, rim, _ = tones(k)
        z = 5 - 0.1 * k
        sol += [
            solid(poly(f"c{k}-body", b, "body", round(z - 0.09, 3), True, body, [(0, -bv), (0.275, -tv), (0.275, tv), (0, bv)]), h0, h1),
            solid(rect(f"c{k}-rim", b, "body", round(z - 0.08, 3), True, rim, 0.275, -0.30, 0.025, 0.60), 0.20),
        ]
        fr += [end_wall(f"c{k}-body", k, body, round(z - 0.09, 3), 0, 0.275), rect(f"c{k}-rim", b, "body", round(z - 0.08, 3), True, rim, -0.20, K.PITCH * (k - 1) + 0.275, 0.40, 0.025)]
        pl.append(rect(f"c{k}-opening", b, "body", round(z - 0.05, 3), True, round(body - 0.1, 2), -K.FLANGE_HOLE["hv"], -K.FLANGE_HOLE["hw"], 2 * K.FLANGE_HOLE["hv"], 2 * K.FLANGE_HOLE["hw"], r=0.02))
    return views(sol, fr, pl, front_solids=False)


def geometry_tub(scale):
    """✏️ 1:200 and 1:500: per crate the tapering tub from the floor up to the rim collar and the collar of 0.60 x 0.40 m on top (30 mm at 1:200, 75 mm at 1:500), both solids."""
    sol, fr = [], []
    uc = K.COLLAR[scale]
    h0, h1 = wall_span()
    bv, tv = K.HV * K.f(0), K.HV * K.f(uc)
    for k in range(1, N + 1):
        b = f"crate-{k}"
        body, rim, _ = tones(k)
        z = 5 - 0.1 * k
        sol += [
            solid(poly(f"c{k}", b, "body", round(z - 0.09, 3), True, body, [(0, -bv), (uc, -tv), (uc, tv), (0, bv)]), h0, h1),
            solid(rect(f"c{k}-rim", b, "body", round(z - 0.08, 3), True, rim if scale == "200" else body, uc, -0.30, K.PITCH - uc, 0.60), 0.20),
        ]
        if scale == "200":
            fr += [end_wall(f"c{k}", k, body, round(z - 0.09, 3), 0, uc), rect(f"c{k}-rim", b, "body", round(z - 0.08, 3), True, rim, -0.20, K.PITCH * (k - 1) + uc, 0.40, K.PITCH - uc)]
    return views(sol, fr, (), front_solids=scale != "200")


DECISIONS = {
    "50": {
        "silhouette": text("A stack of five crates, 12 mm wide and 30 mm high on paper: each a tapering moulded tub on a foot ring under an outward rim band, stacked turned against each other on the stacking ledges, or nested so that only the rim band and 80 mm of each crate show above the one below. Front view: foot, tapering end wall with its hand hole under the rim band. Plan view: rim band, the opening inside the flange, the floor with its two ribs and the three stacking ledges; only the top crate is seen from above.",
                           "Ein Stapel aus fünf Kisten, auf Papier 12 mm breit und 30 mm hoch: jede eine konische Spritzgusswanne auf einem Fußring unter einem nach außen stehenden Randband, gegeneinander gedreht auf den Stapelleisten gestapelt oder ineinandergestellt, sodass nur das Randband und 80 mm jeder Kiste über der unteren zu sehen sind. Vorderansicht: Fuß, konische Stirnwand mit ihrem Griffloch unter dem Randband. Draufsicht: Randband, die Öffnung innerhalb des Flansches, der Boden mit seinen zwei Rippen und die drei Stapelleisten; von oben ist nur die oberste Kiste zu sehen."),
        "retained": text("Crate 0.60 x 0.40 x 0.30 m (ISO 3394 module), the wall taper from 0.53 to 0.58 m under the rim, the 38 mm rim skirt, the 14 mm foot ring 0.52 m long, the hand holes 120 x 35 mm, the floor ribs and the stacking ledges in plan; nesting steps of 0.08 m.",
                         "Kiste 0,60 x 0,40 x 0,30 m (Modul nach ISO 3394), die Konizität von 0,53 auf 0,58 m unter dem Rand, die 38 mm hohe Randschürze, der 14 mm hohe Fußring von 0,52 m Länge, die Grifflöcher 120 x 35 mm, die Bodenrippen und die Stapelleisten in der Draufsicht; Ineinanderstell-Stufen von 0,08 m."),
        "merged": text("Flange and skirt of the rim are one band; the wall channels under the stacking ledges lie inside the side-elevation outline and are not drawn.",
                       "Flansch und Schürze des Randes sind ein Band; die Wandnuten unter den Stapelleisten liegen innerhalb des Umrisses der Seitenansicht und sind nicht gezeichnet."),
        "omitted": text("Moulded text and label areas; the channels in the walls (they would show on the near or the far side depending on how the crate is turned).",
                        "Geformte Beschriftung und Etikettenflächen; die Nuten in den Wänden (sie lägen je nach Drehung der Kiste auf der nahen oder der fernen Seite)."),
        "thinElements": text("The 14 mm foot is 0.28 mm on paper; the hand hole is an opening of 2.4 x 0.7 mm in the end wall, through which the stack is seen, outlined with the body pen.",
                             "Der 14 mm hohe Fuß ist auf Papier 0,28 mm hoch; das Griffloch ist eine Öffnung von 2,4 x 0,7 mm in der Stirnwand, durch die man durch den Stapel sieht, mit dem Körperstift umrandet."),
        "hatching": text("None.", "Keine."),
        "styles": text("Crates alternate between tone 0.45 and 0.55 with a lighter rim and a darker foot so that neighbouring crates separate; ledges light in plan; body pen 0.18 mm, detail 0.13 mm, ground 0.35 mm.",
                       "Die Kisten wechseln zwischen Ton 0,45 und 0,55 mit hellerem Rand und dunklerem Fuß, damit sich benachbarte Kisten trennen; Leisten hell in der Draufsicht; Körperstift 0,18 mm, Detail 0,13 mm, Boden 0,35 mm."),
        "overlays": text("Each crate is its own region: nested crates show a smaller exposed fraction in the thermal view.", "Jede Kiste ist eine eigene Region: ineinandergestellte Kisten zeigen in der Thermalansicht einen kleineren frei liegenden Anteil."),
    },
    "100": {
        "silhouette": text("A stack of five crates, 6 mm wide and 15 mm high: tapering tubs with a rim band on top of each. Front view: five crates as a tapering end wall with its open hand hole under a separate rim band, alternating light and dark tones so that the nested crates stay apart. Plan view: per crate the rim band and the opening inside it; the lower crates are covered by the top one and show in the thermal views only.",
                           "Ein Stapel aus fünf Kisten, 6 mm breit und 15 mm hoch: konische Wannen mit einem Randband auf jeder. Vorderansicht: fünf Kisten als konische Stirnwand mit ihrem offenen Griffloch unter einem eigenen Randband, abwechselnd hell und dunkel, damit die ineinander geschobenen Kisten getrennt bleiben. Draufsicht: pro Kiste Randband und die Öffnung darin; die unteren Kisten verdeckt die oberste, sie erscheinen nur in den thermischen Ansichten."),
        "retained": text("Crate 0.60 x 0.30 m, the taper, the rim of 25 mm, the nesting step of 0.08 m and the hand holes of 120 x 35 mm (1.2 x 0.35 mm, open in the front view). Front and plan view: body, rim band and opening of every crate.",
                         "Kiste 0,60 x 0,30 m, die Konizität, der Rand von 25 mm, die Ineinanderstell-Stufe von 0,08 m und die Grifflöcher von 120 x 35 mm (1,2 x 0,35 mm, offen in der Vorderansicht). Vorder- und Draufsicht: Körper, Randband und Öffnung jeder Kiste."),
        "merged": text("The foot ring joins the body; flange and skirt are one band.", "Der Fußring geht im Körper auf; Flansch und Schürze sind ein Band."),
        "omitted": text("Floor ribs and stacking ledges in the drawing (the ledges stay in the model).", "Bodenrippen und Stapelleisten in der Zeichnung (die Leisten bleiben im Modell)."),
        "thinElements": text("The rim skirt is drawn 25 mm (0.25 mm on paper) so that fill and pen reach the feature width.", "Die Randschürze ist mit 25 mm (0,25 mm auf Papier) gezeichnet, damit Füllung und Strich die Mindestbreite erreichen."),
        "hatching": text("None.", "Keine."),
        "styles": text("Alternating tones 0.45 and 0.55, lighter rim; one body pen of 0.18 mm.", "Wechselnde Töne 0,45 und 0,55, hellerer Rand; ein Körperstift von 0,18 mm."),
        "overlays": text("Each crate keeps its region.", "Jede Kiste behält ihre Region."),
    },
    "200": {
        "silhouette": text("A stack of five trapezoids, 3 mm wide and 7.5 mm high, each crate one tapering tub under a rim collar; nested crates show as stacked collars. Front view: five tapering crates, 0.40 m wide at the collar, with the open hand holes of 120 x 35 mm (0.6 x 0.18 mm), which sink into the one below as the crates nest. Plan view: one rounded rectangle per crate; the crates lie exactly above each other, so only the top crate shows.",
                           "Ein Stapel aus fünf Trapezen, 3 mm breit und 7,5 mm hoch, jede Kiste eine konische Wanne unter einem Randkragen; ineinandergestellte Kisten zeigen sich als gestapelte Kragen. Vorderansicht: fünf konische Kisten, am Kragen 0,40 m breit, mit den offenen Grifflöchern von 120 x 35 mm (0,6 x 0,18 mm), die sich beim Ineinanderschieben in die darunter liegende Kiste senken. Draufsicht: pro Kiste ein abgerundetes Rechteck; die Kisten liegen genau übereinander, nur die oberste ist zu sehen."),
        "retained": text("Crate 0.60 x 0.30 m with the taper from 0.53 m, the stack pitch of 0.30 m and the nesting step of 0.08 m (0.4 mm). Front and plan view: five crates, each its own region.",
                         "Kiste 0,60 x 0,30 m mit der Konizität ab 0,53 m, der Stapelhöhe von 0,30 m und der Ineinanderstell-Stufe von 0,08 m (0,4 mm). Vorder- und Draufsicht: fünf Kisten, jede mit eigener Region."),
        "merged": text("Foot ring and wall are one tapering tub per crate, flange and skirt one rim collar of 30 mm.", "Fußring und Wand sind eine konische Wanne je Kiste, Flansch und Schürze ein Randkragen von 30 mm."),
        "omitted": text("Foot ring and ledges in the drawing.", "Fußring und Leisten in der Zeichnung."),
        "thinElements": text("None below the feature width; the nesting step is 0.4 mm and stays separate from the next outline.", "Nichts unterhalb der Mindestbreite; die Ineinanderstell-Stufe ist 0,4 mm hoch und bleibt vom nächsten Umriss getrennt."),
        "hatching": text("None.", "Keine."),
        "styles": text("Alternating tones 0.45 and 0.55; one body pen of 0.18 mm.", "Wechselnde Töne 0,45 und 0,55; ein Körperstift von 0,18 mm."),
        "overlays": text("Each crate keeps its region.", "Jede Kiste behält ihre Region."),
    },
    "500": {
        "silhouette": text("Five crates of 1.2 by 0.6 mm, a tapering tub under a collar, stacked to 3 mm or nested to 1.2 mm; front: five crates 0.8 mm wide; plan: one rectangle per crate.",
                           "Fünf Kisten von 1,2 mal 0,6 mm, eine konische Wanne unter einem Kragen, auf 3 mm gestapelt oder auf 1,2 mm ineinandergestellt; Vorderansicht: fünf Kisten von 0,8 mm Breite; Draufsicht: ein Rechteck je Kiste."),
        "retained": text("Crate size, the stack pitch and the nesting steps; the taper is kept because the crates nest.", "Kistengröße, Stapelraster und Ineinanderstell-Stufen; die Konizität bleibt, weil die Kisten ineinanderpassen."),
        "merged": text("Foot and wall are one tub per crate, the rim a collar of 75 mm (the feature) at the full 0.60 x 0.40 m.", "Fuß und Wand sind eine Wanne je Kiste, der Rand ein Kragen von 75 mm (die Mindestbreite) mit vollen 0,60 x 0,40 m."),
        "omitted": text("Every detail below 75 mm.", "Alle Details unter 75 mm."),
        "thinElements": text("None.", "Keine."),
        "hatching": text("None.", "Keine."),
        "styles": text("Alternating tones 0.45 and 0.55; one body pen of 0.18 mm.", "Wechselnde Töne 0,45 und 0,55; ein Körperstift von 0,18 mm."),
        "overlays": text("Each crate keeps its region.", "Jede Kiste behält ihre Region."),
    },
}


def apply(doc):
    """🛠️ Applies every root fix to the asset document in place."""
    a = doc["asset"]
    refs = a["provenance"]["references"]
    for reference in REFERENCES:
        if all(r["id"] != reference["id"] for r in refs):
            refs.append(reference)
    a["provenance"]["notes"] = NOTES
    a["summary"] = SUMMARY
    a["dimensions"].update({"heightM": 1.5, "widthM": 0.6, "depthM": 0.4, "massKg": 10.75})
    a["rig"] = rig()
    for anchor in a["anchors"]:
        if anchor["id"] == "base-contact":
            anchor["footprintLengthM"] = 0.52
            anchor["footprintWidthM"] = 0.32
    a["poses"] = poses()
    a["clips"] = clips(a["clips"])
    a["structure"]["assumptions"] = [text(
        "A column of rigid crates standing on the foot ring of the lowest crate; every crate is turned against the one below and transfers its weight and its 10 kg of goods through its foot to the three stacking ledges at the top of the crate below, and from there through the walls to its own foot.",
        "Eine Säule starrer Kisten auf dem Fußring der untersten Kiste; jede Kiste ist gegen die untere gedreht und gibt ihr Gewicht und ihre 10 kg Ware über ihren Fuß an die drei Stapelleisten oben in der unteren Kiste weiter und von dort über die Wände an deren Fuß.")]
    geometry = {"50": geometry50(), "100": geometry100(), "200": geometry_tub("200"), "500": geometry_tub("500")}
    for scale, rep in a["representations"].items():
        rep["geometry"] = geometry[scale]
        rep["decisions"] = DECISIONS[scale]


def member_span(text_, key):
    """🔎 Character span of the value of the asset member `key` in the file text (4-space indent level)."""
    import re
    m = re.search(r'\n    "' + re.escape(key) + r'": ', text_)
    i = m.end()
    depth = 0
    quoted = False
    while True:
        c = text_[i]
        if quoted:
            if c == "\\":
                i += 1
            elif c == '"':
                quoted = False
        elif c == '"':
            quoted = True
            if depth == 0:
                pass
        elif c in "{[":
            depth += 1
        elif c in "}]":
            depth -= 1
            if depth == 0:
                return m.end(), i + 1
        i += 1


MEMBERS = ("summary", "provenance", "dimensions", "rig", "anchors", "representations", "poses", "clips", "structure")


def main():
    """🚀 Reads the catalog right now, applies the edits, replaces only the changed members in the text and writes the target."""
    out = Path(sys.argv[1])
    source = CAT.read_text()
    doc = json.loads(source)
    apply(doc)
    result = source
    for name in MEMBERS:
        a, b = member_span(result, name)
        result = result[:a] + json.dumps(doc["asset"][name], ensure_ascii=False, indent=2).replace("\n", "\n    ") + result[b:]
    if json.loads(result) != doc:
        raise SystemExit("textual splice does not round-trip")
    if out == CAT and CAT.read_text() != source:
        raise SystemExit("catalog changed under us; rerun")
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(result)
    print(out)


if __name__ == "__main__":
    main()
