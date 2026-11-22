"""📦️ Authoritative chisel of the asset `used-moving-boxes`: every hand-chosen number of the stack of three second-hand double-wall moving boxes on a flattened fourth one, and the writer of `🔣️.json` next to this file.

Run: `python3 author-used-moving-boxes.py` (writes `🔣️.json`, prints the masses and centres it derived).

World frame of the staffage (x ahead = along the 0.614 m length of the boxes, y up, z left), metres. Box bones rest at angle 0, so their bone-local point is the world point minus the bone origin; the four lid flaps of the top box rest closed on their hinges (major flaps laid sideways by `abductionDeg`), and every flap solid is converted from world rest bounds through the rest frame of its bone.

@see ../../brief.md — the brief of the wave
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — the contract
"""
import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
G = 9.80665


def T(en, de):
    """🗣️ One bilingual text, English first."""
    return {"en": en, "de": de}


def r6(v):
    """🎯️ Rounds a coordinate to the micrometre."""
    return round(v + 0.0, 6)


L, W, H, H1, TB = 0.614, 0.344, 0.354, 0.342, 0.007
FLAP = W / 2 - TB
FX, FZ, FT = 0.96, 0.70, 0.014
Y1 = FT
Y2 = Y1 + H1
Y3 = Y2 + H
TOP = Y3 + H
HOLE_DROP, HOLE_A, HOLE_R = 0.07, 0.05, 0.015
TAPE_W, TAPE_TAIL, FILM = 0.05, 0.06, 0.0003
BOARD_KG_M2 = 0.82
RHO_BOARD = round(BOARD_KG_M2 / TB, 1)
RHO_FLAT = RHO_BOARD
GRIP_Y = 0.85
CARRY_DX = 0.9

BOXES = {"box-1": (Y1, H1), "box-2": (Y2, H), "box-3": (Y3, H)}


def rz(a):
    """🔄️ Rotation about the joint axis w (u towards v)."""
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    return [[c, -s, 0.0], [s, c, 0.0], [0.0, 0.0, 1.0]]


def ry(a):
    """↪️ Rotation about the bone normal v (u towards w)."""
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    return [[c, 0.0, -s], [0.0, 1.0, 0.0], [s, 0.0, c]]


def mm(a, b):
    """✖️ 3 x 3 matrix product."""
    return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def mv(a, v):
    """➡️ Matrix times vector."""
    return [sum(a[i][k] * v[k] for k in range(3)) for i in range(3)]


def tr(a):
    """🔁️ Transpose."""
    return [[a[j][i] for j in range(3)] for i in range(3)]


BONES = [
    {"id": "ground", "parent": None, "title": T("Depot floor reference", "Bodenbezug im Lager"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
    {"id": "flat-box", "parent": "ground", "title": T("Fourth box folded flat under the stack (stays behind)", "Vierter Karton flach gefaltet unter dem Stapel (bleibt zurück)"), "x": -FX / 2, "y": 0, "angleDeg": 0, "lengthM": FX, "joint": {"type": "fixed"}},
    {"id": "box-1", "parent": "ground", "title": T("Bottom box, compressed by earlier stacking (stays behind)", "Unterer Karton, vom früheren Stapeln gestaucht (bleibt zurück)"), "x": -L / 2, "y": Y1, "angleDeg": 0, "lengthM": L, "joint": {"type": "fixed"}},
    {"id": "box-2", "parent": "box-1", "title": T("Middle box with the taped hand hole (stays behind)", "Mittlerer Karton mit geklebtem Griffloch (bleibt zurück)"), "x": 0, "y": H1, "angleDeg": 0, "lengthM": L, "joint": {"type": "fixed"}},
    {"id": "travel", "parent": "ground", "title": T("Carrying travel along the floor", "Tragweg über den Boden"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 1.2}},
    {"id": "lift", "parent": "travel", "title": T("Lift of the carrying hands", "Hub der tragenden Hände"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.72, "maxM": 0.05}},
    {"id": "box-3", "parent": "lift", "title": T("Top box, the one carried away", "Oberer Karton, der weggetragen wird"), "x": -L / 2, "y": Y3, "angleDeg": 0, "lengthM": L, "joint": {"type": "revolute", "minDeg": -10, "maxDeg": 10}},
    {"id": "flap-r", "parent": "box-3", "title": T("Right major lid flap (hinge along the right top edge)", "Rechte große Deckelklappe (Rill an der rechten Oberkante)"), "x": L / 2, "y": H - TB, "z": -(W / 2 - TB), "angleDeg": 0, "abductionDeg": 90, "lengthM": r6(FLAP), "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 115}},
    {"id": "flap-l", "parent": "box-3", "title": T("Left major lid flap (hinge along the left top edge)", "Linke große Deckelklappe (Rill an der linken Oberkante)"), "x": L / 2, "y": H - TB, "z": W / 2 - TB, "angleDeg": 0, "abductionDeg": -90, "lengthM": r6(FLAP), "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 115}},
    {"id": "flap-f", "parent": "box-3", "title": T("Front minor lid flap (hinge on the front end wall)", "Vordere kleine Deckelklappe (Rill an der vorderen Stirnwand)"), "x": L - TB, "y": H - 2 * TB, "angleDeg": 180, "lengthM": r6(FLAP), "joint": {"type": "revolute", "minDeg": -115, "maxDeg": 0}},
    {"id": "flap-b", "parent": "box-3", "title": T("Back minor lid flap (hinge on the back end wall)", "Hintere kleine Deckelklappe (Rill an der hinteren Stirnwand)"), "x": TB, "y": H - 2 * TB, "angleDeg": 0, "lengthM": r6(FLAP), "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 115}},
]
BONE = {b["id"]: b for b in BONES}


def frames():
    """🧭️ Rest frames (origin, rotation with the columns u, v, w) of every bone in world metres."""
    out = {}
    for b in BONES:
        rest = mm(mm(rz(b["angleDeg"]), ry(b.get("abductionDeg", 0))), [[1, 0, 0], [0, 1, 0], [0, 0, 1]])
        if b["parent"] is None:
            out[b["id"]] = ([b["x"], b["y"], b.get("z", 0.0)], rest)
            continue
        po, pr = out[b["parent"]]
        off = mv(pr, [b["x"], b["y"], b.get("z", 0.0)])
        out[b["id"]] = ([po[i] + off[i] for i in range(3)], mm(pr, rest))
    return out


FR = frames()


def local(bone, p):
    """📍️ World rest point to the bone-local (u, v, w) of `bone`."""
    o, rot = FR[bone]
    return [r6(c) for c in mv(tr(rot), [p[i] - o[i] for i in range(3)])]


def loc(bone, x, y):
    """📍️ World rest point (x, y) to the bone-local (u, v) of a sagittal bone."""
    u, v, _ = local(bone, (x, y, FR[bone][0][2]))
    return (u, v)


def box(id, bone, x0, x1, y0, y1, z0, z1, material, role="body", tone=0.6, **kw):
    """📦️ An axis-aligned box solid from world rest bounds, written in the frame of its bone."""
    _, rot = FR[bone]
    c = local(bone, ((x0 + x1) / 2, (y0 + y1) / 2, (z0 + z1) / 2))
    size = [abs(x) for x in mv(tr(rot), [x1 - x0, y1 - y0, z1 - z0])]
    s = {"id": id, "kind": "box", "bone": bone, "material": material, "role": role, "tone": tone, "size": [r6(a) for a in size], "transform": {"translation": c}}
    s.update(kw)
    return s


def arc(cx, cy, r, a0, a1, n):
    """⭕️ Points of a circular arc from angle a0 to a1 (degrees), both ends included."""
    return [(r6(cx + r * math.cos(math.radians(a0 + (a1 - a0) * k / n))), r6(cy + r * math.sin(math.radians(a0 + (a1 - a0) * k / n)))) for k in range(n + 1)]


def stadium(cx, cy, a, r, n=6):
    """💊️ A slot of half length `a` (along x) and radius `r` (the die-cut hand hole)."""
    return arc(cx + a - r, cy, r, -90, 90, n) + arc(cx - a + r, cy, r, 90, 270, n)


def slot(cx, cy, a, r):
    """▭️ The same hand hole as a plain rectangle (coarser scale)."""
    return [(cx - a, cy - r), (cx + a, cy - r), (cx + a, cy + r), (cx - a, cy + r)]


def poly_area(pts):
    """🧮️ Shoelace area of a ring."""
    return abs(sum(x0 * y1 - x1 * y0 for (x0, y0), (x1, y1) in zip(pts, pts[1:] + pts[:1]))) / 2


def end_walls(id, bone, hole, tone, **kw):
    """🚪️ Both end walls of a box as one board profile in the (v, w) plane swept through the board thickness, the hand hole as a hole, the back wall a repeat copy."""
    yb, h = BOXES[bone]
    zi = W / 2 - TB
    prof = [[0.0, r6(-zi)], [0.0, r6(zi)], [r6(h - TB), r6(zi)], [r6(h - TB), r6(-zi)]]
    s = {"id": id, "kind": "extrude", "bone": bone, "material": MAT[bone], "role": "body", "tone": tone, "plane": "yz", "profile": prof, "from": r6(L - TB), "to": r6(L), "repeat": {"count": 2, "translation": [r6(-(L - TB)), 0, 0]}, "sheet": True}
    if hole is not None:
        s["holes"] = [[[r6(v), r6(w)] for w, v in hole(0.0, h - HOLE_DROP, HOLE_A, HOLE_R)]]
    s.update(kw)
    return s


MAT = {"box-1": "kraft-board-worn", "box-2": "kraft-board", "box-3": "kraft-board", "flat-box": "kraft-board-worn"}


def box_shell(prefix, bone, hole, tone, top=True, tex=True):
    """🗃️ One closed box of the 1:50 or 1:100 body: two long walls, two end walls with their hand holes, the double bottom and (for the boxes that stay) the lid of minor and major flaps."""
    yb, h = BOXES[bone]
    m = MAT[bone]
    zi = W / 2 - TB
    s = [
        box(f"{prefix}-long-walls", bone, -L / 2, L / 2, yb, yb + h, zi, W / 2, m, tone=tone, repeat={"count": 2, "translation": [0, 0, r6(-(W - TB))]}, sheet=True),
        end_walls(f"{prefix}-end-walls", bone, hole, tone - 0.04),
        box(f"{prefix}-bottom", bone, -L / 2 + TB, L / 2 - TB, yb, yb + 2 * TB, -zi, zi, m, tone=tone - 0.1, **({} if tex else {"sheet": True})),
    ]
    if top and tex:
        s += [
            box(f"{prefix}-minor-flaps", bone, L / 2 - TB - FLAP, L / 2 - TB, yb + h - 2 * TB, yb + h - TB, -zi, zi, m, tone=tone - 0.06, repeat={"count": 2, "translation": [r6(-(L - 2 * TB - FLAP)), 0, 0]}, sheet=True),
            box(f"{prefix}-major-flaps", bone, -L / 2, L / 2, yb + h - TB, yb + h, 0.0, zi, m, tone=tone + 0.04, repeat={"count": 2, "translation": [0, 0, r6(-zi)]}, sheet=True),
        ]
    elif top:
        s.append(box(f"{prefix}-lid", bone, -L / 2, L / 2, yb + h - 2 * TB, yb + h, -zi, zi, m, tone=tone + 0.04, sheet=True))
    return s


def flaps(prefix, tone):
    """🪽️ The four lid flaps of the top box on their own hinge bones, closed at rest."""
    zi = W / 2 - TB
    return [
        box(f"{prefix}-flap-r", "flap-r", -L / 2, L / 2, TOP - TB, TOP, -zi, 0.0, "kraft-board", tone=tone + 0.04, sheet=True),
        box(f"{prefix}-flap-l", "flap-l", -L / 2, L / 2, TOP - TB, TOP, 0.0, zi, "kraft-board", tone=tone + 0.04, sheet=True),
        box(f"{prefix}-flap-f", "flap-f", L / 2 - TB - FLAP, L / 2 - TB, TOP - 2 * TB, TOP - TB, -zi, zi, "kraft-board", tone=tone - 0.06, sheet=True),
        box(f"{prefix}-flap-b", "flap-b", -L / 2 + TB, -L / 2 + TB + FLAP, TOP - 2 * TB, TOP - TB, -zi, zi, "kraft-board", tone=tone - 0.06, sheet=True),
    ]


def tex(id, bone, x0, x1, y0, y1, z0, z1, material, tone):
    """🏷️ A texture solid (tape, label, sticker): a film on a face, excluded from the feature and silhouette rules."""
    return box(id, bone, x0, x1, y0, y1, z0, z1, material, role="detail", tone=tone, texture=True)


def tapes(prefix, bone, lid_on_flaps=False):
    """🩹️ The brown packing tape over the lid seam and down both end walls."""
    yb, h = BOXES[bone]
    top = yb + h
    out = [tex(f"{prefix}-tape-tails", bone, L / 2, L / 2 + FILM, top - TAPE_TAIL, top, -TAPE_W / 2, TAPE_W / 2, "packing-tape", 0.45)]
    out[0]["repeat"] = {"count": 2, "translation": [r6(-(L + FILM)), 0, 0]}
    if lid_on_flaps:
        out += [tex(f"{prefix}-tape-r", "flap-r", -L / 2, L / 2, top, top + FILM, -TAPE_W / 2, 0.0, "packing-tape", 0.45),
                tex(f"{prefix}-tape-l", "flap-l", -L / 2, L / 2, top, top + FILM, 0.0, TAPE_W / 2, "packing-tape", 0.45)]
    else:
        out.append(tex(f"{prefix}-tape-top", bone, -L / 2, L / 2, top, top + FILM, -TAPE_W / 2, TAPE_W / 2, "packing-tape", 0.45))
    return out


LABELS = {"box-1": (-0.17, 0.03, 0.12, 0.22), "box-2": (-0.05, 0.15, 0.15, 0.24), "box-3": (-0.21, -0.03, 0.14, 0.23)}
CREASE = [(-0.25, 0.0), (0.0, -0.006), (0.25, 0.0)]


def labels(prefix):
    """✍️ Old paper labels and stickers on the right long walls (the face the side elevation sees)."""
    out = []
    for bone, (x0, x1, a, b) in LABELS.items():
        yb, _ = BOXES[bone]
        out.append(tex(f"{prefix}-label-{bone}", bone, x0, x1, yb + a, yb + b, -W / 2 - FILM, -W / 2, "paper-label", 0.88))
    return out


def body50():
    """📦️ The 1:50 body: the flattened box, every board of the three boxes with their die-cut hand holes, the four lid flaps of the top box on their hinges, tape, the taped repair of a torn hand hole and the old labels."""
    s = [box("flat-box", "flat-box", -FX / 2, FX / 2, 0.0, FT, -FZ / 2, FZ / 2, "kraft-board-worn", tone=0.5, sheet=True)]
    s += box_shell("b1", "box-1", stadium, 0.6)
    s += box_shell("b2", "box-2", stadium, 0.66)
    s += box_shell("b3", "box-3", stadium, 0.7, top=False)
    s += flaps("b3", 0.7)
    s += tapes("b1", "box-1") + tapes("b2", "box-2") + tapes("b3", "box-3", lid_on_flaps=True)
    hc = Y2 + H - HOLE_DROP
    s.append(tex("b2-repair-tape", "box-2", L / 2 + FILM, L / 2 + 2 * FILM, hc + HOLE_R, hc + HOLE_R + 0.045, -0.075, 0.075, "packing-tape", 0.42))
    s += labels("t50")
    return s


def body100():
    """🪚️ The 1:100 body: the flattened box, each box of its walls, end walls with a plain rectangular hand hole, a double bottom sheet and (stacked boxes) one 14 mm lid sheet; the four flaps of the top box on their hinges."""
    s = [box("flat-box-100", "flat-box", -FX / 2, FX / 2, 0.0, FT, -FZ / 2, FZ / 2, "kraft-board-worn", tone=0.5, sheet=True)]
    s += box_shell("b1-100", "box-1", slot, 0.6, tex=False)
    s += box_shell("b2-100", "box-2", slot, 0.66, tex=False)
    s += box_shell("b3-100", "box-3", slot, 0.7, top=False, tex=False)
    s += flaps("b3-100", 0.7)
    return s


def tube(id, bone, tone):
    """🧱️ The four walls of a box as one hollow profile in plan (w, u) swept up its height."""
    yb, h = BOXES[bone]
    out = [[r6(-W / 2), 0.0], [r6(W / 2), 0.0], [r6(W / 2), r6(L)], [r6(-W / 2), r6(L)]]
    hole = [[r6(-W / 2 + TB), r6(TB)], [r6(W / 2 - TB), r6(TB)], [r6(W / 2 - TB), r6(L - TB)], [r6(-W / 2 + TB), r6(L - TB)]]
    return {"id": id, "kind": "extrude", "bone": bone, "material": MAT[bone], "role": "body", "tone": tone, "plane": "zx", "profile": out, "holes": [hole], "from": 0.0, "to": r6(h)}


def body200():
    """🧱️ The 1:200 body: the flattened box, the two stacked boxes as closed blocks, the top box as its wall tube on a double bottom with the four flaps."""
    s = [box("flat-box-200", "flat-box", -FX / 2, FX / 2, 0.0, FT, -FZ / 2, FZ / 2, "kraft-board-worn", tone=0.5, sheet=True)]
    for bone, tone in (("box-1", 0.6), ("box-2", 0.66)):
        yb, h = BOXES[bone]
        s.append(box(f"{bone}-200", bone, -L / 2, L / 2, yb, yb + h, -W / 2, W / 2, MAT[bone], tone=tone))
    s.append(tube("b3-tube-200", "box-3", 0.7))
    s.append(box("b3-bottom-200", "box-3", -L / 2 + TB, L / 2 - TB, Y3, Y3 + 2 * TB, -W / 2 + TB, W / 2 - TB, "kraft-board", tone=0.6, sheet=True))
    s += flaps("b3-200", 0.7)
    return s


def solid_volume(s):
    """📐️ Volume of a solid times its repeat count (box or extrude)."""
    n = s.get("repeat", {}).get("count", 1)
    if s["kind"] == "box":
        a, b, c = s["size"]
        return a * b * c * n
    a = poly_area([tuple(p) for p in s["profile"]]) - sum(poly_area([tuple(p) for p in h]) for h in s.get("holes", []))
    return a * (s["to"] - s["from"]) * n


def centre(s):
    """🎯️ The local centre of a solid and its repeat copies."""
    if s["kind"] == "box":
        c = list(s["transform"]["translation"])
    else:
        prof = [tuple(p) for p in s["profile"]]
        a2 = sum(x0 * y1 - x1 * y0 for (x0, y0), (x1, y1) in zip(prof, prof[1:] + prof[:1]))
        cx = sum((x0 + x1) * (x0 * y1 - x1 * y0) for (x0, y0), (x1, y1) in zip(prof, prof[1:] + prof[:1])) / (3 * a2)
        cy = sum((y0 + y1) * (x0 * y1 - x1 * y0) for (x0, y0), (x1, y1) in zip(prof, prof[1:] + prof[:1])) / (3 * a2)
        m = (s["from"] + s["to"]) / 2
        c = {"xy": [cx, cy, m], "yz": [m, cx, cy], "zx": [cy, m, cx]}[s["plane"]]
    rep = s.get("repeat")
    if rep:
        n = rep["count"]
        t = rep["translation"]
        c = [c[i] + t[i] * (n - 1) / 2 for i in range(3)]
    return c


DENS = {"kraft-board": RHO_BOARD, "kraft-board-worn": RHO_BOARD}


def mass_of(solids):
    """⚖️ Masses and centres per bone from section × density (tape and labels are films of a few grams, left out)."""
    out = {}
    for s in solids:
        if s.get("texture"):
            continue
        kg = solid_volume(s) * DENS[s["material"]]
        c = centre(s)
        acc = out.setdefault(s["bone"], [0.0, 0.0, 0.0, 0.0, {}])
        acc[0] += kg
        for i in range(3):
            acc[i + 1] += kg * c[i]
        acc[4][s["id"]] = round(kg, 4)
    return {b: {"kg": a[0], "comX": a[1] / a[0], "comY": a[2] / a[0], "comZ": a[3] / a[0], "parts": a[4]} for b, a in out.items()}


B50 = body50()
MASS = mass_of(B50)
TOTAL_KG = sum(m["kg"] for m in MASS.values())
CARRIED_BONES = ("box-3", "flap-r", "flap-l", "flap-f", "flap-b")
STAY_BONES = ("flat-box", "box-1", "box-2")
CARRIED_KG = sum(MASS[b]["kg"] for b in CARRIED_BONES)
BOX_KG = {"box-1": MASS["box-1"]["kg"], "box-2": MASS["box-2"]["kg"], "box-3": CARRIED_KG, "flat-box": MASS["flat-box"]["kg"]}


def ref(id, of, **kw):
    """🔗️ A thermal or structural reference to a body solid of the same scale."""
    s = {"id": id, "of": of}
    s.update(kw)
    return s


def marker(id, bone, r):
    """⚫️ A compact mass marker at the centre of mass of a bone."""
    m = MASS[bone]
    return {"id": id, "kind": "ellipsoid", "bone": bone, "material": MAT.get(bone, "kraft-board"), "role": "detail", "tone": 0.3, "radii": [r, r, r], "transform": {"translation": [r6(m["comX"]), r6(m["comY"]), r6(m["comZ"])]}, "member": bone, "part": "mass"}


def glue_laps(prefix, d):
    """🧷️ The glued manufacturer's joint of every box: the 35 mm lap of the long wall glued inside the back end wall, a joint solid at the back right corner."""
    out = []
    for bone in ("box-1", "box-2", "box-3"):
        yb, h = BOXES[bone]
        out.append(dict(box(f"{prefix}-lap-{bone}", bone, -L / 2 + TB, -L / 2 + TB + d, yb + 2 * TB, yb + h - 2 * TB, -W / 2 + TB, -W / 2 + TB + 0.035, MAT[bone], role="detail", tone=0.3), member=bone, part="joint"))
    return out


def structure_of(body, sfx, r, lap):
    """🏗️ The load path of a scale: every board as a member on its bone, the glued laps as joints, one mass marker per bone."""
    st = [ref(f"m-{s['id']}", s["id"], member=s["bone"], part="member") for s in body if not s.get("texture")]
    if lap:
        st += glue_laps(f"j{sfx}", lap)
    st += [marker(f"mass-{b}{sfx}", b, r) for b in ("flat-box", "box-1", "box-2", "box-3", "flap-r", "flap-l", "flap-f", "flap-b")]
    return st


REGION_OF = {"flat-box": "flat-box", "box-1": "stacked-boxes", "box-2": "stacked-boxes", "box-3": "carried-box", "flap-r": "carried-box", "flap-l": "carried-box", "flap-f": "carried-box", "flap-b": "carried-box"}


def thermal100(body):
    """🌡️ The 1:100 thermal model: the four walls of each box as one closed wall tube (no hand holes), its bottom and lid or flaps by reference, the flattened box."""
    th = []
    for s in body:
        if s.get("texture") or s["id"].endswith(("-long-walls", "-end-walls")):
            continue
        th.append(ref("th-" + s["id"], s["id"], region=REGION_OF[s["bone"]]))
    for bone, tone in (("box-1", 0.6), ("box-2", 0.66), ("box-3", 0.7)):
        th.append(dict(tube(f"th-{bone}-tube-100", bone, tone), region=REGION_OF[bone]))
    return th


def thermal_of(body, sfx):
    """🌡️ The thermal model of a scale: every board in the region of its box, the end walls as closed boards (the 100 x 30 mm hand holes are left closed: the inside of an empty taped box is still air behind them and not part of the exchange surface)."""
    th = []
    for s in body:
        if s.get("texture"):
            continue
        reg = REGION_OF[s["bone"]]
        if s["kind"] == "extrude" and s.get("holes") and s["plane"] == "yz":
            c = {k: v for k, v in s.items() if k != "holes"}
            c["id"] = "th-" + s["id"]
            c["region"] = reg
            th.append(c)
        else:
            th.append(ref("th-" + s["id"], s["id"], region=reg))
    return th


def sh(id, bone, role, z, filled, tone, geom, region=None, material=None, hatch=None, pen=None, solid=None):
    """✏️ One 2D shape."""
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    s.update(geom)
    if region:
        s["region"] = region
    if material:
        s["material"] = material
    if hatch:
        s["hatch"] = hatch
    if pen is not None:
        s["penMm"] = pen
    if solid:
        s["solid"] = solid
    return s


def erect(bone, x0, y0, x1, y1):
    """⬛️ An elevation rectangle from world rest corners, in the frame of its bone."""
    u0, v0 = loc(bone, x0, y0)
    return {"kind": "rect", "x": u0, "y": v0, "w": r6(x1 - x0), "h": r6(y1 - y0)}


def epoly(bone, pts, closed=True):
    """🔷️ An elevation polygon from world rest points, in the frame of its bone."""
    return {"kind": "poly", "points": [list(loc(bone, x, y)) for x, y in pts], "closed": closed}


def wrect(a0, b0, a1, b1):
    """⬜️ A front or plan rectangle in world rest coordinates (front: z, y; plan: x, z)."""
    return {"kind": "rect", "x": r6(a0), "y": r6(b0), "w": r6(a1 - a0), "h": r6(b1 - b0)}


def wpoly(pts, closed=True):
    """🔶️ A front or plan polygon in world rest coordinates."""
    return {"kind": "poly", "points": [[r6(a), r6(b)] for a, b in pts], "closed": closed}


def wpath(lines):
    """〰️ Open strokes as one path of polylines in world rest coordinates."""
    return {"kind": "path", "d": " ".join("M " + " L ".join(f"{r6(a)} {r6(b)}" for a, b in ln) for ln in lines)}


def epath(bone, lines):
    """〰️ Open strokes as one path of polylines from world rest points, in the frame of a sagittal bone."""
    return {"kind": "path", "d": " ".join("M " + " L ".join("{} {}".format(*loc(bone, a, b)) for a, b in ln) for ln in lines)}


def ring_path(rings):
    """⭕️ A path of several closed rings."""
    return {"kind": "path", "d": " ".join("M " + " L ".join(f"{r6(a)} {r6(b)}" for a, b in ring) + " Z" for ring in rings)}


def end_face(yb, h, hole):
    """🚪️ The end face of a box in the front view with its hand hole as a hole: the hand holes of both end walls are in line, so the empty box is seen through."""
    return ring_path([[(-W / 2, yb), (W / 2, yb), (W / 2, yb + h), (-W / 2, yb + h)], list(reversed(hole))])


def flap_shapes(z0, tone, pen=None):
    """🪽️ The four lid flaps as solid-placed shapes in every view: the profile in the frame of the flap bone, placed with its full 3D frame, so open, closing and closed flaps read right in elevation, front and plan."""
    zi = W / 2 - TB
    major = {"kind": "rect", "x": 0.0, "y": r6(-L / 2), "w": r6(FLAP), "h": r6(L)}
    return [
        sh("flap-r", "flap-r", "body", z0, True, tone + 0.04, major, region="carried-box", material="kraft-board", pen=pen, solid={"kind": "extruded", "sweep": "v", "z0": 0.0, "z1": TB}),
        sh("flap-l", "flap-l", "body", z0, True, tone + 0.04, major, region="carried-box", material="kraft-board", pen=pen, solid={"kind": "extruded", "sweep": "v", "z0": 0.0, "z1": TB}),
        sh("flap-f", "flap-f", "body", z0 - 0.5, True, tone - 0.06, {"kind": "rect", "x": 0.0, "y": -TB, "w": r6(FLAP), "h": TB}, region="carried-box", material="kraft-board", pen=pen, solid={"kind": "extruded", "sweep": "w", "z0": r6(-zi), "z1": r6(zi)}),
        sh("flap-b", "flap-b", "body", z0 - 0.5, True, tone - 0.06, {"kind": "rect", "x": 0.0, "y": 0.0, "w": r6(FLAP), "h": TB}, region="carried-box", material="kraft-board", pen=pen, solid={"kind": "extruded", "sweep": "w", "z0": r6(-zi), "z1": r6(zi)}),
    ]


def tape_on_flaps(z0):
    """🩹️ The cut halves of the lid tape on the major flaps (plan and front), placed by the flap frames."""
    return [
        sh("tape-r", "flap-r", "detail", z0, True, 0.45, {"kind": "rect", "x": r6(FLAP - TAPE_W / 2), "y": r6(-L / 2), "w": r6(TAPE_W / 2), "h": r6(L)}, region="carried-box", material="packing-tape", solid={"kind": "extruded", "sweep": "v", "z0": TB, "z1": r6(TB + FILM)}),
        sh("tape-l", "flap-l", "detail", z0, True, 0.45, {"kind": "rect", "x": r6(FLAP - TAPE_W / 2), "y": r6(-L / 2), "w": r6(TAPE_W / 2), "h": r6(L)}, region="carried-box", material="packing-tape", solid={"kind": "extruded", "sweep": "v", "z0": TB, "z1": r6(TB + FILM)}),
    ]


TONES = {"box-1": 0.6, "box-2": 0.66, "box-3": 0.7}
REG = {"box-1": "stacked-boxes", "box-2": "stacked-boxes", "box-3": "carried-box"}


def scribble(x0, y0, w, h, n):
    """✍️ A marker scrawl of n letters as a zigzag inside a box (world x, y)."""
    pts = []
    for k in range(2 * n + 1):
        pts.append((x0 + w * k / (2 * n), y0 + (h if k % 2 else 0.0) + (0.004 if k % 4 == 1 else 0.0)))
    return pts


def rep50():
    """🖋️ The 1:50 drawing: every box with its seams, hand holes, tape, labels, crease and dent, the flaps of the top box, the score lines and stain of the flattened box."""
    el = [sh("flat-box", "flat-box", "body", 1, True, 0.5, erect("flat-box", -FX / 2, 0, FX / 2, FT), region="flat-box", material="kraft-board-worn"),
          sh("flat-box-layers", "flat-box", "texture", 1.5, False, 0.3, epath("flat-box", [[(-FX / 2 + 0.01, FT / 2), (FX / 2 - 0.01, FT / 2)]]), region="flat-box")]
    for bone in ("box-1", "box-2", "box-3"):
        yb, h = BOXES[bone]
        el.append(sh(bone, bone, "body", 2, True, TONES[bone], erect(bone, -L / 2, yb, L / 2, yb + h), region=REG[bone], material=MAT[bone]))
        x0, x1, a, b = LABELS[bone]
        el.append(sh(f"label-{bone}", bone, "detail", 2.4, True, 0.88, erect(bone, x0, yb + a, x1, yb + b), region=REG[bone], material="paper-label"))
        el.append(sh(f"writing-{bone}", bone, "texture", 2.6, False, 0.2, epath(bone, [scribble(x0 + 0.02, yb + a + 0.035, x1 - x0 - 0.05, 0.03, 5)] + ([[(x0 + 0.01, yb + a + 0.05), (x1 - 0.015, yb + a + 0.055)]] if bone != "box-2" else [])), region=REG[bone]))
    el += [sh("crease-box-1", "box-1", "detail", 2.8, False, 0.3, epath("box-1", [[(x, Y1 + H1 / 2 + dy) for x, dy in CREASE]]), region="stacked-boxes"),
           sh("dent-box-1", "box-1", "detail", 2.8, False, 0.3, epath("box-1", [[(L / 2 - 0.06, Y2), (L / 2 - 0.035, Y2 - 0.02), (L / 2, Y2 - 0.028)]]), region="stacked-boxes"),
           sh("sticker-box-2", "box-2", "detail", 2.8, True, 0.9, epoly("box-2", [(-0.26, Y2 + 0.05), (-0.17, Y2 + 0.05), (-0.18, Y2 + 0.09), (-0.26, Y2 + 0.11)]), region="stacked-boxes", material="paper-label"),
           sh("scuffs-box-3", "box-3", "texture", 2.8, False, 0.3, epath("box-3", [[(0.2, Y3 + 0.01), (0.27, Y3 + 0.03)], [(0.23, Y3 + 0.008), (0.29, Y3 + 0.022)]]), region="carried-box")]
    el += flap_shapes(3, 0.7)
    fr = [sh("flat-box", "flat-box", "body", 1, True, 0.5, wrect(-FZ / 2, 0, FZ / 2, FT), region="flat-box", material="kraft-board-worn")]
    for bone in ("box-1", "box-2", "box-3"):
        yb, h = BOXES[bone]
        hc = yb + h - HOLE_DROP
        fr += [sh(bone, bone, "body", 2, True, TONES[bone] - 0.04, end_face(yb, h, stadium(0.0, hc, HOLE_A, HOLE_R)), region=REG[bone], material=MAT[bone]),
               sh(f"tape-tail-{bone}", bone, "detail", 2.6, True, 0.45, wrect(-TAPE_W / 2, yb + h - TAPE_TAIL, TAPE_W / 2, yb + h), region=REG[bone], material="packing-tape")]
    fr += [sh("repair-tape-box-2", "box-2", "detail", 2.8, True, 0.42, wrect(-0.075, Y2 + H - HOLE_DROP + HOLE_R, 0.075, Y2 + H - HOLE_DROP + HOLE_R + 0.045), region="stacked-boxes", material="packing-tape"),
           sh("repair-tear-box-2", "box-2", "texture", 2.9, False, 0.3, wpath([[(-0.02, Y2 + H - HOLE_DROP + HOLE_R), (-0.012, Y2 + H - HOLE_DROP + HOLE_R + 0.03), (0.0, Y2 + H - HOLE_DROP + HOLE_R + 0.04)]]), region="stacked-boxes"),
           sh("crease-end-box-1", "box-1", "detail", 2.8, False, 0.3, wpath([[(-W / 2, Y1 + H1 / 2), (0.0, Y1 + H1 / 2 - 0.005), (W / 2, Y1 + H1 / 2)]]), region="stacked-boxes")]
    fr += flap_shapes(3, 0.66)
    pl = [sh("flat-box", "flat-box", "body", 1, True, 0.5, wrect(-FX / 2, -FZ / 2, FX / 2, FZ / 2), region="flat-box", material="kraft-board-worn"),
          sh("flat-box-scores", "flat-box", "texture", 1.5, False, 0.3, wpath([[(-FX / 2, FZ / 2 - FLAP - TB), (FX / 2, FZ / 2 - FLAP - TB)], [(-FX / 2, -FZ / 2 + FLAP + TB), (FX / 2, -FZ / 2 + FLAP + TB)], [(-FX / 2 + L, -FZ / 2), (-FX / 2 + L, FZ / 2)]]), region="flat-box"),
          sh("flat-box-stain", "flat-box", "texture", 1.6, False, 0.3, {"kind": "ellipse", "cx": 0.39, "cy": -0.24, "rx": 0.05, "ry": 0.035, "rotationDeg": 20}, region="flat-box")]
    for bone in ("box-1", "box-2"):
        pl += [sh(f"{bone}", bone, "body", 2, True, TONES[bone] + 0.04, wrect(-L / 2, -W / 2, L / 2, W / 2), region=REG[bone], material=MAT[bone]),
               sh(f"tape-{bone}", bone, "detail", 2.2, True, 0.45, wrect(-L / 2, -TAPE_W / 2, L / 2, TAPE_W / 2), region=REG[bone], material="packing-tape")]
    pl += [sh("box-3-inside", "box-3", "body", 3, True, 0.32, wrect(-L / 2 + TB, -W / 2 + TB, L / 2 - TB, W / 2 - TB), region="carried-box", material="kraft-board"),
           sh("box-3-rim", "box-3", "body", 3.2, True, 0.7, ring_path([[(-L / 2, -W / 2), (L / 2, -W / 2), (L / 2, -W / 2 + TB), (-L / 2, -W / 2 + TB)], [(-L / 2, W / 2 - TB), (L / 2, W / 2 - TB), (L / 2, W / 2), (-L / 2, W / 2)], [(-L / 2, -W / 2 + TB), (-L / 2 + TB, -W / 2 + TB), (-L / 2 + TB, W / 2 - TB), (-L / 2, W / 2 - TB)], [(L / 2 - TB, -W / 2 + TB), (L / 2, -W / 2 + TB), (L / 2, W / 2 - TB), (L / 2 - TB, W / 2 - TB)]]), region="carried-box", material="kraft-board")]
    pl += flap_shapes(4, 0.7) + tape_on_flaps(5)
    return {"status": "authored", "denominator": 50, "validRange": {"minDenominator": 20, "maxDenominator": 75}, "pens": {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": None, "ground": 0.7},
            "hatches": [], "geometry": {"elevation": el, "front": fr, "plan": pl},
            "decisions": {
                "silhouette": T("Stack 12.3 x 21.0 mm on the 19.2 mm flattened box: three boxes of 12.3 x 7.1 mm (the bottom one 6.8 mm, compressed by 12 mm), each drawn on its own so the joints between them read; the lid flaps of the top box as their own shapes placed by their hinges. Front view: the three end walls with their die-cut hand holes (2.0 x 0.6 mm) and the tape tails; plan: the taped lid of the top box over the 19.2 x 14 mm flattened box with its score lines.", "Stapel 12,3 x 21,0 mm auf dem 19,2 mm breiten flachen Karton: drei Kartons zu 12,3 x 7,1 mm (der untere 6,8 mm, um 12 mm gestaucht), jeder für sich gezeichnet, damit die Fugen zwischen ihnen lesbar sind; die Deckelklappen des oberen Kartons als eigene Formen an ihren Rillen. Vorderansicht: die drei Stirnwände mit ihren gestanzten Grifflöchern (2,0 x 0,6 mm) und den Klebebandenden; Grundriss: der zugeklebte Deckel des oberen Kartons über dem 19,2 x 14 mm großen flachen Karton mit seinen Rilllinien."),
                "retained": T("The traces of use: the 12 mm compressed bottom box with its crease across both faces and a dented front top corner, the torn hand hole of the middle box taped over, old paper labels with crossed-out marker writing, a half torn-off sticker, scuffs on the top box, the water stain on the flattened box.", "Die Gebrauchsspuren: der um 12 mm gestauchte untere Karton mit seiner Knickfalte über beide Seiten und einer eingedrückten vorderen oberen Ecke, das eingerissene und überklebte Griffloch des mittleren Kartons, alte Papieretiketten mit durchgestrichener Filzstiftschrift, ein halb abgerissener Aufkleber, Schrammen am oberen Karton, der Wasserfleck auf dem flachen Karton."),
                "merged": T("The two long walls coincide in the side view and the two end walls in the front view; the flaps closed inside the box outline.", "Die beiden Längswände fallen in der Seitenansicht zusammen, die beiden Stirnwände in der Vorderansicht; die geschlossenen Klappen liegen im Umriss des Kartons."),
                "omitted": T("Corrugation flutes (4 and 3 mm, inside the board), the 7 mm board edges and score beads, the glued manufacturer's joint inside the back corner (in the structure list only), the printed handling marks.", "Wellen der Pappe (4 und 3 mm, im Inneren der Platte), die 7-mm-Plattenkanten und Rillwülste, die geklebte Herstellerfuge innen in der hinteren Ecke (nur in der Tragwerksliste), die aufgedruckten Handhabungszeichen."),
                "thinElements": T("The 7 mm double-wall board is drawn as the edge of the box outline (0.14 mm); the 14 mm flattened box keeps its two layers with a texture line.", "Die 7-mm-Doppelwellpappe ist als Kante des Kartonumrisses gezeichnet (0,14 mm); der 14 mm dicke flache Karton behält seine zwei Lagen mit einer Texturlinie."),
                "hatching": T("None: kraft board is a plain tone; the tape is a darker band, the paper labels a light one.", "Keine: Kraftpappe ist ein glatter Ton; das Klebeband ist ein dunkleres Band, die Papieretiketten ein helles."),
                "styles": T("Top box 0.70, middle 0.66, worn bottom box 0.60, flattened box 0.50, inside of the open box 0.32, tape 0.45, labels 0.88-0.90, hand holes 0.12 (the dark inside of the box seen through them).", "Oberer Karton 0,70, mittlerer 0,66, abgenutzter unterer 0,60, flacher Karton 0,50, Innenseite des offenen Kartons 0,32, Klebeband 0,45, Etiketten 0,88-0,90, Grifflöcher 0,12 (das dunkle Innere des Kartons dahinter)."),
                "overlays": T("One shape per box and view; the flaps of the top box, the tape halves on them and the inside of the box are their own shapes, so the open lid of the in-use pose shows the empty box inside.", "Eine Form je Karton und Ansicht; die Klappen des oberen Kartons, die Klebebandhälften darauf und die Innenseite des Kartons sind eigene Formen, sodass der offene Deckel der Gebrauchspose das leere Innere zeigt."),
            }}


def rep100():
    """🖋️ The 1:100 drawing: the boxes as outlines, labels as light fields, hand holes as slots, tape tails, the flaps of the top box."""
    el = [sh("flat-box", "flat-box", "body", 1, True, 0.5, erect("flat-box", -FX / 2, 0, FX / 2, FT), region="flat-box", material="kraft-board-worn")]
    for bone in ("box-1", "box-2", "box-3"):
        yb, h = BOXES[bone]
        x0, x1, a, b = LABELS[bone]
        el += [sh(bone, bone, "body", 2, True, TONES[bone], erect(bone, -L / 2, yb, L / 2, yb + h), region=REG[bone], material=MAT[bone]),
               sh(f"label-{bone}", bone, "detail", 2.4, True, 0.88, erect(bone, x0, yb + a, x1, yb + b), region=REG[bone], material="paper-label")]
    el.append(sh("crease-box-1", "box-1", "detail", 2.8, False, 0.3, epath("box-1", [[(x, Y1 + H1 / 2 + dy) for x, dy in CREASE]]), region="stacked-boxes"))
    el += flap_shapes(3, 0.7)
    fr = [sh("flat-box", "flat-box", "body", 1, True, 0.5, wrect(-FZ / 2, 0, FZ / 2, FT), region="flat-box", material="kraft-board-worn")]
    for bone in ("box-1", "box-2", "box-3"):
        yb, h = BOXES[bone]
        hc = yb + h - HOLE_DROP
        fr += [sh(bone, bone, "body", 2, True, TONES[bone] - 0.04, end_face(yb, h, slot(0.0, hc, HOLE_A, HOLE_R)), region=REG[bone], material=MAT[bone]),
               sh(f"tape-tail-{bone}", bone, "detail", 2.6, True, 0.45, wrect(-TAPE_W / 2, yb + h - TAPE_TAIL, TAPE_W / 2, yb + h), region=REG[bone], material="packing-tape")]
    fr += flap_shapes(3, 0.66)
    pl = [sh("flat-box", "flat-box", "body", 1, True, 0.5, wrect(-FX / 2, -FZ / 2, FX / 2, FZ / 2), region="flat-box", material="kraft-board-worn"),
          sh("flat-box-scores", "flat-box", "texture", 1.5, False, 0.3, wpath([[(-FX / 2, FZ / 2 - FLAP - TB), (FX / 2, FZ / 2 - FLAP - TB)], [(-FX / 2, -FZ / 2 + FLAP + TB), (FX / 2, -FZ / 2 + FLAP + TB)]]), region="flat-box")]
    for bone in ("box-1", "box-2"):
        pl.append(sh(f"{bone}", bone, "body", 2, True, TONES[bone] + 0.04, wrect(-L / 2, -W / 2, L / 2, W / 2), region=REG[bone], material=MAT[bone]))
    pl += [sh("box-3-inside", "box-3", "body", 3, True, 0.32, wrect(-L / 2 + TB, -W / 2 + TB, L / 2 - TB, W / 2 - TB), region="carried-box", material="kraft-board"),
           sh("box-3-rim", "box-3", "body", 3.2, False, 0.7, wrect(-L / 2 + TB / 2, -W / 2 + TB / 2, L / 2 - TB / 2, W / 2 - TB / 2), region="carried-box", material="kraft-board", pen=0.35)]
    pl += flap_shapes(4, 0.7) + tape_on_flaps(5)
    return {"status": "authored", "denominator": 100, "validRange": {"minDenominator": 75, "maxDenominator": 150}, "pens": {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": None, "ground": 0.5},
            "hatches": [], "geometry": {"elevation": el, "front": fr, "plan": pl},
            "decisions": {
                "silhouette": T("Stack 6.1 x 10.6 mm on the 9.6 mm flattened box: three box outlines with the joints between them, the flaps of the top box; front: end walls with the hand holes as 1.0 x 0.3 mm slots and the tape tails; plan: the taped lid over the flattened box.", "Stapel 6,1 x 10,6 mm auf dem 9,6 mm breiten flachen Karton: drei Kartonumrisse mit den Fugen dazwischen, die Klappen des oberen Kartons; vorn: Stirnwände mit den Grifflöchern als Schlitze 1,0 x 0,3 mm und den Klebebandenden; Grundriss: der zugeklebte Deckel über dem flachen Karton."),
                "retained": T("The compressed bottom box (12 mm lower) with its crease, the light label fields, the hand holes, the tape and the two flap score lines of the flattened box.", "Der gestauchte untere Karton (12 mm niedriger) mit seiner Knickfalte, die hellen Etikettenfelder, die Grifflöcher, das Klebeband und die beiden Klapprillen des flachen Kartons."),
                "merged": T("The rounded ends of the hand holes become plain slots; the tape of the stacked boxes merges with their lids.", "Die runden Enden der Grifflöcher werden zu einfachen Schlitzen; das Klebeband der gestapelten Kartons verschmilzt mit ihren Deckeln."),
                "omitted": T("Marker writing, sticker, dent, scuffs, the repair tape, the stain and the panel score of the flattened box (all under 0.15 mm or too fine).", "Filzstiftschrift, Aufkleber, Delle, Schrammen, das Reparaturklebeband, der Fleck und die Felderrille des flachen Kartons (alle unter 0,15 mm oder zu fein)."),
                "thinElements": T("The 7 mm board and the 14 mm flattened box are carried by the 0.35 mm pen; the rim of the open box is a centred outline.", "Die 7-mm-Pappe und der 14 mm dicke flache Karton werden vom 0,35-mm-Stift getragen; der Rand des offenen Kartons ist ein mittiger Umriss."),
                "hatching": T("None.", "Keine."),
                "styles": T("Top box 0.70, middle 0.66, bottom 0.60, flattened box 0.50, inside 0.32, tape 0.45, labels 0.88, hand holes 0.12.", "Oberer Karton 0,70, mittlerer 0,66, unterer 0,60, flacher Karton 0,50, Innenseite 0,32, Klebeband 0,45, Etiketten 0,88, Grifflöcher 0,12."),
                "overlays": T("One shape per box and view, the flaps and tape halves of the top box placed by their hinges.", "Eine Form je Karton und Ansicht, Klappen und Klebebandhälften des oberen Kartons an ihren Rillen platziert."),
            }}


def rep200():
    """🖋️ The 1:200 drawing: three box outlines on the flattened box, the four flaps of the top box, no hand holes, tape or labels."""
    el = [sh("flat-box", "flat-box", "body", 1, True, 0.5, erect("flat-box", -FX / 2, 0, FX / 2, FT), region="flat-box", material="kraft-board-worn")]
    fr = [sh("flat-box", "flat-box", "body", 1, True, 0.5, wrect(-FZ / 2, 0, FZ / 2, FT), region="flat-box", material="kraft-board-worn")]
    pl = [sh("flat-box", "flat-box", "body", 1, True, 0.5, wrect(-FX / 2, -FZ / 2, FX / 2, FZ / 2), region="flat-box", material="kraft-board-worn")]
    for bone in ("box-1", "box-2", "box-3"):
        yb, h = BOXES[bone]
        el.append(sh(bone, bone, "body", 2, True, TONES[bone], erect(bone, -L / 2, yb, L / 2, yb + h), region=REG[bone], material=MAT[bone]))
        fr.append(sh(bone, bone, "body", 2, True, TONES[bone] - 0.04, wrect(-W / 2, yb, W / 2, yb + h), region=REG[bone], material=MAT[bone]))
    for bone in ("box-1", "box-2"):
        pl.append(sh(bone, bone, "body", 2, True, TONES[bone] + 0.04, wrect(-L / 2, -W / 2, L / 2, W / 2), region=REG[bone], material=MAT[bone]))
    pl.append(sh("box-3-inside", "box-3", "body", 3, True, 0.32, wrect(-L / 2, -W / 2, L / 2, W / 2), region="carried-box", material="kraft-board"))
    el += flap_shapes(3, 0.7)
    fr += flap_shapes(3, 0.66)
    pl += flap_shapes(4, 0.7)
    return {"status": "authored", "denominator": 200, "validRange": {"minDenominator": 150, "maxDenominator": 350}, "pens": {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35},
            "hatches": [], "geometry": {"elevation": el, "front": fr, "plan": pl},
            "decisions": {
                "silhouette": T("Stack 3.1 x 5.3 mm on the 4.8 mm flattened box: three box outlines whose joints still read, the flaps of the top box so that the open lid of the in-use pose is drawn.", "Stapel 3,1 x 5,3 mm auf dem 4,8 mm breiten flachen Karton: drei Kartonumrisse, deren Fugen noch lesbar sind, die Klappen des oberen Kartons, damit der offene Deckel der Gebrauchspose gezeichnet wird."),
                "retained": T("The three boxes as separate bodies, the lower height of the compressed bottom box (1.71 against 1.77 mm), the flattened box under the stack.", "Die drei Kartons als getrennte Körper, die geringere Höhe des gestauchten unteren Kartons (1,71 gegen 1,77 mm), der flache Karton unter dem Stapel."),
                "merged": T("Each stacked box is one closed body; the top box is its wall tube with the flaps.", "Jeder gestapelte Karton ist ein geschlossener Körper; der obere Karton ist sein Wandrohr mit den Klappen."),
                "omitted": T("Hand holes (0.5 x 0.15 mm), tape, labels, crease, score lines and every other trace of use (under 0.15 mm at 1:200).", "Grifflöcher (0,5 x 0,15 mm), Klebeband, Etiketten, Knickfalte, Rilllinien und alle übrigen Gebrauchsspuren (unter 0,15 mm bei 1:200)."),
                "thinElements": T("The 14 mm flattened box is carried by the 0.25 mm pen.", "Der 14 mm dicke flache Karton wird vom 0,25-mm-Stift getragen."),
                "hatching": T("None.", "Keine."),
                "styles": T("Top box 0.70, middle 0.66, bottom 0.60, flattened box 0.50, inside of the open box 0.32.", "Oberer Karton 0,70, mittlerer 0,66, unterer 0,60, flacher Karton 0,50, Innenseite des offenen Kartons 0,32."),
                "overlays": T("One shape per box and view and one per flap.", "Eine Form je Karton und Ansicht und eine je Klappe."),
            }}


def rep500():
    """🚫️ 1:500 is not drawn: the whole stack is about two millimetres of paper."""
    return {"status": "unavailable", "denominator": 500, "cause": "below-printable-feature",
            "reason": T("The stack is 0.96 x 0.70 x 1.06 m = 1.9 x 1.4 x 2.1 mm on paper at 1:500, under 3 mm: the single boxes (0.34 m = 0.7 mm), their joints, the flattened box (14 mm = 0.03 mm) and the lid flaps would merge into one anonymous speck.", "Der Stapel misst 0,96 x 0,70 x 1,06 m = 1,9 x 1,4 x 2,1 mm auf dem Papier bei 1:500, unter 3 mm: die einzelnen Kartons (0,34 m = 0,7 mm), ihre Fugen, der flache Karton (14 mm = 0,03 mm) und die Deckelklappen würden zu einem namenlosen Fleck verschmelzen."),
            "limitingFeature": T("Flattened box under the stack, 14 mm (0.03 mm on paper)", "Flacher Karton unter dem Stapel, 14 mm (0,03 mm auf dem Papier)"),
            "limitingSizeM": FT, "alternatives": [{"kind": "scale", "denominator": 200}]}


def vol(den, body, thermal, structure, decisions):
    """🧊️ One authored scale volume."""
    return {"status": "authored", "denominator": den, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


B100, B200 = body100(), body200()
VOLUMES = {
    "50": vol(50, B50, thermal_of(B50, ""), structure_of(B50, "", 0.012, 0.008), [
        T("Body for the 7.5 mm feature: every board of the three 614 x 344 x 354 mm double-wall boxes at its true 7 mm gauge (declared sheets): the long walls, both end walls with the die-cut 100 x 30 mm hand hole as a hole, the 14 mm double bottom of minor and major flaps, the lid of minor flaps under major flaps; the bottom box 12 mm lower (342 mm) from earlier stacking; the four lid flaps of the top box on their own hinge bones; the 0.96 x 0.70 m flattened fourth box of two 7 mm layers under the stack.", "Körper für das 7,5-mm-Merkmal: jede Platte der drei Doppelwellkartons 614 x 344 x 354 mm in ihrer wahren Stärke von 7 mm (erklärte Platten): die Längswände, beide Stirnwände mit dem gestanzten Griffloch 100 x 30 mm als Loch, der 14-mm-Doppelboden aus kleinen und großen Klappen, der Deckel aus kleinen Klappen unter großen; der untere Karton vom früheren Stapeln 12 mm niedriger (342 mm); die vier Deckelklappen des oberen Kartons auf eigenen Rillknochen; der flach gefaltete vierte Karton 0,96 x 0,70 m aus zwei 7-mm-Lagen unter dem Stapel."),
        T("Traces of use as films (texture solids): the 50 mm packing tape over every lid seam and down both end walls (on the top box cut in two halves on the major flaps), the tape over the torn hand hole of the middle box, the old paper labels on the right long walls; the 12 mm compression is real geometry.", "Gebrauchsspuren als Folien (Texturkörper): das 50-mm-Packband über jeder Deckelfuge und an beiden Stirnwänden hinab (am oberen Karton in zwei Hälften auf den großen Klappen geschnitten), das Klebeband über dem eingerissenen Griffloch des mittleren Kartons, die alten Papieretiketten an den rechten Längswänden; die Stauchung um 12 mm ist echte Geometrie."),
        T("Thermal: every board in the region of its box; the end walls are closed boards in the thermal list, because the inside of an empty taped box behind a 30 mm hand hole is still air and no exchange surface, so each region is the outer surface of its box. Structure: every board a member on the bone of its box, the glued 35 mm manufacturer's lap of each box as a joint, eight mass markers; no permanent support: the pose contacts carry (corners of the flattened box, hand holes, bottom corners).", "Thermik: jede Platte in der Region ihres Kartons; die Stirnwände sind in der Thermikliste geschlossene Platten, denn das Innere eines leeren zugeklebten Kartons hinter einem 30-mm-Griffloch ist ruhende Luft und keine Austauschfläche, so ist jede Region die Außenfläche ihres Kartons. Tragwerk: jede Platte ein Stab auf dem Knochen ihres Kartons, die geklebte 35-mm-Herstellerfuge jedes Kartons als Gelenk, acht Massenmarken; kein dauerndes Auflager: die Kontakte der Posen tragen (Ecken des flachen Kartons, Grifflöcher, untere Ecken)."),
    ]),
    "100": vol(100, B100, thermal100(B100), structure_of(B100, "-100", 0.009, 0.016), [
        T("Body for the 15 mm feature: the walls of each box at their 7 mm gauge (sheets), the end walls with a plain 100 x 30 mm rectangular hand hole, the double bottom as one 14 mm sheet, the lid of each stacked box as one 14 mm sheet (minor and major flaps merged), the four flaps of the top box on their hinges, the flattened box as one 14 mm sheet.", "Körper für das 15-mm-Merkmal: die Wände jedes Kartons in ihrer Stärke von 7 mm (Platten), die Stirnwände mit einem einfachen rechteckigen Griffloch 100 x 30 mm, der Doppelboden als eine 14-mm-Platte, der Deckel jedes gestapelten Kartons als eine 14-mm-Platte (kleine und große Klappen vereinigt), die vier Klappen des oberen Kartons an ihren Rillen, der flache Karton als eine 14-mm-Platte."),
        T("Left out against 1:50: tape, labels and the repair tape (films the 1:100 drawing does not draw), the rounded ends of the hand holes.", "Gegenüber 1:50 weggelassen: Klebeband, Etiketten und das Reparaturband (Folien, die die 1:100-Zeichnung nicht zeichnet), die runden Enden der Grifflöcher."),
        T("Thermal: the four walls of each box as one closed wall tube without hand holes, bottom, lid and flaps by reference, all in the region of their box; structure: boards as members, glued laps as 16 mm joints, mass markers 18 mm.", "Thermik: die vier Wände jedes Kartons als ein geschlossenes Wandrohr ohne Grifflöcher, Boden, Deckel und Klappen als Verweis, alle in der Region ihres Kartons; Tragwerk: Platten als Stäbe, geklebte Fugen als 16-mm-Gelenke, Massenmarken 18 mm."),
    ]),
    "200": vol(200, B200, thermal_of(B200, "-200"), structure_of(B200, "-200", 0.016, 0.03), [
        T("Body for the 30 mm feature: each stacked box as one closed block of its outer size (the 7 mm board and the hand holes are below the feature), the top box as one hollow wall tube with its 14 mm double bottom and the four 7 mm flaps on their hinges, so the in-use pose still opens the lid; the flattened box as a 14 mm sheet.", "Körper für das 30-mm-Merkmal: jeder gestapelte Karton als ein geschlossener Block seiner Außenmaße (die 7-mm-Pappe und die Grifflöcher liegen unter dem Merkmal), der obere Karton als ein hohles Wandrohr mit seinem 14-mm-Doppelboden und den vier 7-mm-Klappen an ihren Rillen, damit die Gebrauchspose den Deckel noch öffnet; der flache Karton als 14-mm-Platte."),
        T("Thermal: blocks, tube, bottom, flaps and flattened box in their regions; structure: the same solids as members, the glued laps as 30 mm joints, mass markers 32 mm.", "Thermik: Blöcke, Rohr, Boden, Klappen und flacher Karton in ihren Regionen; Tragwerk: dieselben Körper als Stäbe, die geklebten Fugen als 30-mm-Gelenke, Massenmarken 32 mm."),
    ]),
    "500": {"status": "unavailable", "denominator": 500},
}


def mat(id, en, de, eps, alpha, rho, k, c, E=None, f=None, source="literature", reference="iso-10456-2007"):
    """🧱️ One material."""
    m = {"id": id, "title": T(en, de), "emissivity": eps, "solarAbsorptivity": alpha}
    if rho is not None:
        m["densityKgM3"] = rho
    m.update({"conductivityWmK": k, "specificHeatJKgK": c})
    if E is not None:
        m["elasticModulusPa"] = E
    if f is not None:
        m["strengthPa"] = f
    m.update({"source": source, "reference": reference})
    return m


MATERIALS = [
    mat("kraft-board", "Double-wall corrugated board BC 7 mm, brown kraft, reused", "Doppelwellpappe BC 7 mm, braunes Kraftpapier, wiederverwendet", 0.9, 0.75, RHO_BOARD, 0.064, 1340, 0.4e9, 1.0e6, "literature", "din-55468-1-2015"),
    mat("kraft-board-worn", "Double-wall corrugated board BC 7 mm, scuffed, damp-darkened and compressed", "Doppelwellpappe BC 7 mm, verschrammt, feuchtigkeitsdunkel und gestaucht", 0.91, 0.8, RHO_BOARD, 0.064, 1340, 0.24e9, 0.6e6, "authored-estimate", "mckee-1963"),
    mat("packing-tape", "Brown polypropylene packing tape with acrylic adhesive", "Braunes Polypropylen-Packband mit Acrylatkleber", 0.85, 0.65, 905, 0.2, 1700, None, None, "literature", "incropera-2018"),
    mat("paper-label", "Paper label, yellowed, with marker writing", "Papieretikett, vergilbt, mit Filzstiftschrift", 0.93, 0.35, 800, 0.12, 1340, None, None, "literature", "incropera-2018"),
]


def layer(id, en, de, kind, material, t, k, rho, c):
    """🧶️ One single layer of a region with its resistance and areal heat capacity rho c min(t, sqrt(alpha 3600 s))."""
    depth = min(t, math.sqrt(k / (rho * c) * 3600))
    return {"id": id, "title": T(en, de), "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": round(t / k, 6), "heatCapacityJM2K": round(rho * c * depth), "source": "literature", "reference": "iso-10456-2007"}


def outer(l, w, h):
    """📏️ Outer surface of a closed box."""
    return 2 * (l * w + l * h + w * h)


AREA = {"carried-box": round(outer(L, W, H) - L * W, 4), "stacked-boxes": round(outer(L, W, H1) + outer(L, W, H) - 3 * L * W, 4), "flat-box": round(2 * FX * FZ + 2 * (FX + FZ) * FT, 4)}


def region(id, en, de, material, geometry, Lc, sky, lay, extra=None):
    """🔥️ One passive thermal region."""
    r = {"id": id, "title": T(en, de), "kind": "surface", "model": "passive", "material": material, "areaM2": AREA[id], "characteristicLengthM": Lc, "geometry": geometry, "skyViewFactor": sky, "layers": [lay], "heatCapacityJM2K": lay["heatCapacityJM2K"], "source": "authored-estimate", "reference": "iso-10456-2007"}
    if extra:
        r.update(extra)
    return r


REGIONS = [
    region("carried-box", "Top box with its lid flaps", "Oberer Karton mit Deckelklappen", "kraft-board", "vertical-plate", H, 0.5, layer("carried-board", "Double-wall board 7 mm", "Doppelwellpappe 7 mm", "timber", "kraft-board", TB, 0.064, RHO_BOARD, 1340), {"ratedPowerW": 0, "ratedPowerBasis": "passive"}),
    region("stacked-boxes", "Middle and compressed bottom box", "Mittlerer und gestauchter unterer Karton", "kraft-board-worn", "vertical-plate", H + H1, 0.3, layer("stacked-board", "Double-wall board 7 mm", "Doppelwellpappe 7 mm", "timber", "kraft-board-worn", TB, 0.064, RHO_BOARD, 1340)),
    region("flat-box", "Fourth box folded flat", "Vierter Karton flach gefaltet", "kraft-board-worn", "horizontal-plate-up", FZ, 0.2, layer("flat-board", "Two layers of board 14 mm", "Zwei Lagen Pappe 14 mm", "timber", "kraft-board-worn", FT, 0.064, RHO_FLAT, 1340)),
]

HC3 = H - HOLE_DROP
ANCHORS = []
for name, x in (("rear", -0.43), ("front", 0.43)):
    for side, z in (("l", 0.3), ("r", -0.3)):
        u, v = loc("flat-box", x, 0.0)
        ANCHORS.append({"id": f"floor-{name}-{side}", "bone": "flat-box", "x": u, "y": v, "z": z, "kind": "contact", "title": T(f"Floor under the {name} {'left' if side == 'l' else 'right'} corner of the flattened box", f"Boden unter der {'hinteren' if name == 'rear' else 'vorderen'} {'linken' if side == 'l' else 'rechten'} Ecke des flachen Kartons"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.1, "footprintWidthM": 0.1, "frictionCoefficient": 0.5})
for name, u in (("rear", 0.03), ("front", L - 0.03)):
    for side, z in (("l", W / 2 - 0.03), ("r", -(W / 2 - 0.03))):
        ANCHORS.append({"id": f"corner-{name}-{side}", "bone": "box-3", "x": r6(u), "y": 0.0, "z": r6(z), "kind": "contact", "title": T(f"{'Rear' if name == 'rear' else 'Front'} {'left' if side == 'l' else 'right'} bottom corner of the top box", f"{'Hintere' if name == 'rear' else 'Vordere'} {'linke' if side == 'l' else 'rechte'} untere Ecke des oberen Kartons"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.06, "footprintWidthM": 0.06, "frictionCoefficient": 0.5})
ANCHORS += [
    {"id": "grip-front", "bone": "box-3", "x": r6(L - TB / 2), "y": r6(HC3), "z": 0.0, "kind": "support", "title": T("Hand in the front hand hole", "Hand im vorderen Griffloch"), "support": "pin"},
    {"id": "grip-rear", "bone": "box-3", "x": r6(TB / 2), "y": r6(HC3), "z": 0.0, "kind": "support", "title": T("Hand in the rear hand hole", "Hand im hinteren Griffloch"), "support": "pin"},
    {"id": "contents", "bone": "box-3", "x": r6(L / 2), "y": 0.12, "z": 0.0, "kind": "load", "title": T("Books packed in the box", "In den Karton gepackte Bücher"), "support": None},
]
ANCHORS[-1].pop("support")

FLOOR = ["floor-rear-l", "floor-rear-r", "floor-front-l", "floor-front-r"]
CORNERS = ["corner-rear-l", "corner-rear-r", "corner-front-l", "corner-front-r"]
GRIPS = ["grip-front", "grip-rear"]
LIFT = round(GRIP_Y - (Y3 + HC3), 4)
DOWN = round(-Y3, 4)


def stay_loads():
    """🪨️ Floor reactions under the flattened box and the two stacked boxes, which stay on the depot floor on their own."""
    out = []
    for b in STAY_BONES:
        m = MASS[b]
        out.append({"id": f"floor-under-{b}", "kind": "imposed", "title": T(f"Floor reaction under the {'flattened box' if b == 'flat-box' else b.replace('-', ' ')}, which stays behind", f"Bodenreaktion unter {'dem flachen Karton' if b == 'flat-box' else 'Karton ' + b[-1]}, der zurückbleibt"), "bone": b, "x": round(m["comX"], 5), "y": round(m["comY"], 5), "fxN": 0, "fyN": round(m["kg"] * G, 3), "source": "literature", "reference": "en-1991-1-1"})
    return out


BOOKS_KG = 15.0
BOOKS = {"id": "books", "kind": "equipment", "title": T(f"Books packed in the box, {BOOKS_KG:.0f} kg", f"In den Karton gepackte Bücher, {BOOKS_KG:.0f} kg"), "anchor": "contents", "fxN": 0, "fyN": round(-BOOKS_KG * G, 3), "source": "authored-estimate", "reference": "removal-box-type"}


def binding(carried_contact=0.0, sun=0.0):
    """🌬️ The thermal binding of every region of a pose."""
    return {"regions": {"carried-box": {"exposedFraction": 1, "sunlitFraction": sun} | ({"contact": {"fraction": carried_contact, "tempC": 18}} if carried_contact else {}),
                        "stacked-boxes": {"exposedFraction": 1, "sunlitFraction": 0},
                        "flat-box": {"exposedFraction": 1, "sunlitFraction": 0, "contact": {"fraction": 0.48, "tempC": 16}}}, "emitters": {}}


OPEN = {"flap-r": {"angleDeg": 100}, "flap-l": {"angleDeg": 100}, "flap-f": {"angleDeg": -100}, "flap-b": {"angleDeg": 100}}
MINORS_IN = {"flap-r": {"angleDeg": 100}, "flap-l": {"angleDeg": 100}}
HOME = {"travel": {"dx": CARRY_DX}, "lift": {"dy": DOWN}}
FLOOR_FRACTION = round(L * W / AREA["carried-box"], 3)

POSES = [
    {"id": "stored", "title": T("Stacked in the depot", "Im Lager gestapelt"), "description": T("Three emptied, taped moving boxes stacked on a fourth one folded flat on the floor of a second-hand store; the bottom box is compressed by 12 mm.", "Drei geleerte, zugeklebte Umzugskartons auf einem vierten, flach gefalteten Karton auf dem Boden eines Gebrauchtwarenkaufhauses gestapelt; der untere Karton ist um 12 mm gestaucht."), "bones": {}, "ground": "authored", "contacts": FLOOR, "thermal": binding(), "loads": []},
    {"id": "carried", "title": T("Top box carried by its hand holes", "Oberer Karton an den Grifflöchern getragen"), "description": T(f"The top box lifted off the stack and carried level with both hands in its hand holes at {GRIP_Y:.2f} m; the other boxes and the flattened one stay behind.", f"Der obere Karton vom Stapel gehoben und mit beiden Händen in den Grifflöchern waagerecht in {GRIP_Y:.2f} m getragen; die übrigen Kartons und der flache bleiben zurück."), "bones": {"travel": {"dx": CARRY_DX}, "lift": {"dy": LIFT}}, "ground": "authored", "contacts": GRIPS, "thermal": binding(), "loads": stay_loads()},
    {"id": "in-use", "title": T("Being packed again", "Wird wieder gepackt"), "description": T(f"Standing on the floor of a flat with the lid flaps folded open, {BOOKS_KG:.0f} kg of books packed in; a window sun patch reaches it.", f"Auf dem Boden einer Wohnung mit aufgeklappten Deckelklappen, {BOOKS_KG:.0f} kg Bücher eingepackt; ein Fensterlichtfleck erreicht ihn."), "bones": dict(HOME, **OPEN), "ground": "authored", "contacts": CORNERS, "thermal": binding(FLOOR_FRACTION, 0.2), "loads": stay_loads() + [BOOKS]},
    {"id": "packed", "title": T("Packed and closed", "Gepackt und geschlossen"), "description": T("The flaps folded shut over the books, ready to be taped and carried.", "Die Klappen über den Büchern geschlossen, bereit zum Zukleben und Tragen."), "bones": dict(HOME), "ground": "authored", "contacts": CORNERS, "thermal": binding(FLOOR_FRACTION, 0.2), "loads": stay_loads() + [BOOKS]},
    {"id": "minors-in-key", "key": True, "title": T("Minor flaps folded in", "Kleine Klappen eingeschlagen"), "description": T("The two minor flaps folded in, the major flaps still standing open.", "Die beiden kleinen Klappen eingeschlagen, die großen Klappen stehen noch offen."), "bones": dict(HOME, **MINORS_IN), "ground": "authored", "contacts": CORNERS, "thermal": binding(FLOOR_FRACTION, 0.2), "loads": stay_loads() + [BOOKS]},
    {"id": "lift-off-key", "key": True, "title": T("Lifting off the stack", "Vom Stapel abheben"), "description": T("Both hands in the hand holes lift the top box 20 mm off the middle box.", "Beide Hände in den Grifflöchern heben den oberen Karton 20 mm vom mittleren ab."), "bones": {"lift": {"dy": 0.02}}, "ground": "authored", "contacts": GRIPS, "thermal": binding(), "loads": stay_loads()},
    {"id": "clear-key", "key": True, "title": T("Drawn clear of the stack", "Vom Stapel weggezogen"), "description": T("The top box drawn forward until its rear end has cleared the stack, still 20 mm high.", "Der obere Karton nach vorn gezogen, bis sein hinteres Ende den Stapel freigibt, noch 20 mm hoch."), "bones": {"travel": {"dx": 0.64}, "lift": {"dy": 0.02}}, "ground": "authored", "contacts": GRIPS, "thermal": binding(), "loads": stay_loads()},
    {"id": "set-down-key", "key": True, "title": T("Lowered to the floor", "Auf den Boden abgesenkt"), "description": T("The carried box lowered to just above the floor.", "Der getragene Karton bis knapp über den Boden abgesenkt."), "bones": {"travel": {"dx": CARRY_DX}, "lift": {"dy": round(DOWN + 0.01, 4)}}, "ground": "authored", "contacts": GRIPS, "thermal": binding(), "loads": stay_loads()},
]

CLIPS = [
    {"id": "carry", "title": T("Take the top box off the stack", "Den oberen Karton vom Stapel nehmen"), "description": T("Both hands reach into the hand holes, lift the top box off the stack, draw it forward clear of the stack and lower it to carrying height; the rest of the stack stays.", "Beide Hände greifen in die Grifflöcher, heben den oberen Karton vom Stapel, ziehen ihn nach vorn vom Stapel weg und senken ihn auf Traghöhe; der Rest des Stapels bleibt."), "durationS": 3.0, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 0.6, "pose": "lift-off-key", "ease": "smooth"}, {"t": 1.8, "pose": "clear-key", "ease": "smooth"}, {"t": 2.6, "pose": "carried", "ease": "smooth"}, {"t": 3.0, "pose": "carried", "ease": "smooth"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 3.0} for a in FLOOR] + [{"anchor": a, "fromS": 0.2, "toS": 3.0} for a in GRIPS],
     "phases": [{"id": "grip", "fromS": 0, "toS": 0.6, "title": T("Grip and lift", "Greifen und anheben")}, {"id": "draw", "fromS": 0.6, "toS": 1.8, "title": T("Draw clear", "Wegziehen")}, {"id": "lower", "fromS": 1.8, "toS": 3.0, "title": T("Lower to carrying height", "Auf Traghöhe senken")}],
     "events": [{"t": 0.3, "kind": "contact-off", "title": T("The top box leaves the stack", "Der obere Karton verlässt den Stapel")}, {"t": 2.6, "kind": "motion", "title": T("Carried level", "Waagerecht getragen")}], "tracks": []},
    {"id": "close-lid", "title": T("Fold the lid shut and open again", "Deckel zuklappen und wieder öffnen"), "description": T("While packing: the minor flaps are folded in, the major flaps over them, the lid held shut, then opened again in reverse order.", "Beim Packen: die kleinen Klappen werden eingeschlagen, die großen darüber, der Deckel zugehalten und in umgekehrter Folge wieder geöffnet."), "durationS": 4.0, "fps": 30, "loop": True, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "in-use", "ease": "smooth"}, {"t": 0.9, "pose": "minors-in-key", "ease": "smooth"}, {"t": 1.8, "pose": "packed", "ease": "smooth"}, {"t": 2.4, "pose": "packed", "ease": "smooth"}, {"t": 3.2, "pose": "minors-in-key", "ease": "smooth"}, {"t": 4.0, "pose": "in-use", "ease": "smooth"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 4.0} for a in CORNERS],
     "phases": [{"id": "minors", "fromS": 0, "toS": 0.9, "title": T("Minor flaps in", "Kleine Klappen ein")}, {"id": "majors", "fromS": 0.9, "toS": 1.8, "title": T("Major flaps over", "Große Klappen darüber")}, {"id": "hold", "fromS": 1.8, "toS": 2.4, "title": T("Shut", "Zu")}, {"id": "open", "fromS": 2.4, "toS": 4.0, "title": T("Open again", "Wieder öffnen")}],
     "events": [{"t": 1.8, "kind": "motion", "title": T("Lid shut", "Deckel zu")}, {"t": 4.0, "kind": "motion", "title": T("Lid open", "Deckel offen")}], "tracks": []},
    {"id": "set-down", "title": T("Set it down in the flat", "In der Wohnung abstellen"), "description": T("The carried box is lowered onto its bottom corners and let go.", "Der getragene Karton wird auf seine unteren Ecken abgesenkt und losgelassen."), "durationS": 2.0, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "carried", "ease": "smooth"}, {"t": 1.4, "pose": "set-down-key", "ease": "smooth"}, {"t": 2.0, "pose": "packed", "ease": "smooth"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 1.9} for a in GRIPS] + [{"anchor": a, "fromS": 1.7, "toS": 2.0} for a in CORNERS],
     "phases": [{"id": "lower", "fromS": 0, "toS": 1.4, "title": T("Lower", "Absenken")}, {"id": "release", "fromS": 1.4, "toS": 2.0, "title": T("Set down and let go", "Absetzen und loslassen")}],
     "events": [{"t": 1.7, "kind": "contact-on", "title": T("Corners touch the floor", "Ecken berühren den Boden")}, {"t": 1.9, "kind": "contact-off", "title": T("Hands let go", "Hände lassen los")}], "tracks": []},
]

SYMBOL = {"id": "used-moving-boxes-symbol", "title": T("Stack of moving boxes, true extent", "Umzugskartonstapel, wahre Ausdehnung"), "description": T("The stored stack in all three views: three boxes 0.614 x 0.344 m to 1.06 m on the flattened 0.96 x 0.70 m box.", "Der gelagerte Stapel in allen drei Ansichten: drei Kartons 0,614 x 0,344 m bis 1,06 m auf dem flachen Karton 0,96 x 0,70 m."),
          "semantics": "true-extent", "units": "model-m", "denominators": [200, 100, 50], "pens": {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35}, "hatches": [],
          "geometry": {
              "elevation": [sh("flat-box", "ground", "body", 1, True, 0.5, wrect(-FX / 2, 0, FX / 2, FT)), sh("stack", "ground", "body", 2, True, 0.7, wrect(-L / 2, Y1, L / 2, TOP)), sh("joints", "ground", "detail", 3, False, 0.3, wpath([[(-L / 2, Y2), (L / 2, Y2)], [(-L / 2, Y3), (L / 2, Y3)]]))],
              "front": [sh("flat-box", "ground", "body", 1, True, 0.5, wrect(-FZ / 2, 0, FZ / 2, FT)), sh("stack", "ground", "body", 2, True, 0.7, wrect(-W / 2, Y1, W / 2, TOP)), sh("joints", "ground", "detail", 3, False, 0.3, wpath([[(-W / 2, Y2), (W / 2, Y2)], [(-W / 2, Y3), (W / 2, Y3)]]))],
              "plan": [sh("flat-box", "ground", "body", 1, True, 0.5, wrect(-FX / 2, -FZ / 2, FX / 2, FZ / 2)), sh("stack", "ground", "body", 2, True, 0.7, wrect(-L / 2, -W / 2, L / 2, W / 2))],
          },
          "dimensionalNote": T("Model metres at real size in the frame of the asset (x ahead along the boxes, lateral positive to the left, y up): the side view is seen from the right, the front view from ahead, the top view from above; all three show the stored stack.", "Modellmeter in wahrer Größe im Koordinatensystem des Objekts (x nach vorn längs der Kartons, seitlich positiv nach links, y nach oben): die Seitenansicht blickt von rechts, die Vorderansicht von vorn, die Draufsicht von oben; alle drei zeigen den gelagerten Stapel.")}

REFERENCES = [
    {"id": "fefco-code-2021", "citation": "FEFCO / ESBO (2021). International Fibreboard Case Code, 12th ed., FEFCO, Brussels (style 0201 regular slotted container: four panels with a glued manufacturer's joint, top and bottom flaps each half the width deep, the major flaps meeting at the centre)."},
    {"id": "din-55468-1-2015", "citation": "DIN 55468-1:2015-05 Packstoffe - Wellpappe - Teil 1: Anforderungen, Prüfung (double-wall corrugated board BC about 6.5-7 mm, grades with grammage about 700-900 g/m2 and edge crush resistance ECT about 6-9 kN/m)."},
    {"id": "din-en-iso-3037-2013", "citation": "DIN EN ISO 3037:2013 Corrugated fibreboard - Determination of edgewise crush resistance (ECT), the board property that the box compression strength follows."},
    {"id": "mckee-1963", "citation": "McKee, R. C., Gander, J. W., Wachuta, J. R. (1963). Compression strength formula for corrugated boxes. Paperboard Packaging 48(8), 149-159 (BCT = 5.87 ECT sqrt(t Z): with ECT 7 kN/m, t 7 mm and perimeter 1.92 m about 4.8 kN new; humidity and reuse lower it by 30-50 %)."},
    {"id": "din-en-iso-12048-2001", "citation": "DIN EN ISO 12048:2001 Packaging - Complete, filled transport packages - Compression and stacking tests using a compression tester."},
    {"id": "removal-box-type", "citation": "Type data of German removal boxes (Umzugskartons) as removal companies and DIY stores offer them: double-wall BC corrugated board, inner 600 x 330 x 340 mm (about 67-68 l), outer about 614 x 344 x 354 mm, regular slotted style with two die-cut 100 x 30 mm hand holes, printed label field, 1.0-1.2 kg empty, rated for 30-40 kg; no single product, no brand."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (paper and cardboard products about 0.05-0.13 W/mK depending on density, specific heat about 1300-1400 J/kgK)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (total emissivity of paper 0.92-0.97 and of plastic films about 0.85; solar absorptance of brown kraft surfaces)."},
    {"id": "en-1991-1-1", "citation": "EN 1991-1-1:2002 Eurocode 1: Actions on structures - Part 1-1: General actions - Densities, self-weight, imposed loads for buildings (self-weight as the product of volume and density; books about 6-8 kN/m3 bulk)."},
    {"id": "din-33402-2-2020", "citation": "DIN 33402-2:2020-12 Ergonomie - Körpermaße des Menschen - Teil 2: Werte (elbow height of standing adults about 1.0-1.1 m; a box held in front with bent forearms puts the hands at about 0.85 m)."},
    {"id": "din-18011-2020", "citation": "DIN 18011:2020-04 Stellflächen, Abstände und Bewegungsflächen im Wohnungsbau (movement areas of at least 0.9 m in front of furniture, through which a 0.614 m box is carried)."},
]

MASS_NOTE = ", ".join(f"{b} {BOX_KG[b]:.2f} kg" for b in ("flat-box", "box-1", "box-2", "box-3"))
ASSET = {
    "id": "used-moving-boxes", "category": "objects",
    "title": T("Used Moving Box Stack", "Gebrauchter Umzugskartonstapel"),
    "summary": T(f"Three second-hand double-wall moving boxes of 614 x 344 x 354 mm, emptied and taped shut, stacked to 1.06 m on a fourth box folded flat (0.96 x 0.70 m); the bottom box compressed by 12 mm with a crease, a torn hand hole taped over, old labels with crossed-out marker writing. The top box is carried away level by both hand holes and packed again with its lid flaps folded open; {TOTAL_KG:.1f} kg in all.", f"Drei gebrauchte Doppelwell-Umzugskartons von 614 x 344 x 354 mm, geleert und zugeklebt, 1,06 m hoch auf einem vierten, flach gefalteten Karton (0,96 x 0,70 m) gestapelt; der untere Karton um 12 mm gestaucht und geknickt, ein eingerissenes Griffloch überklebt, alte Etiketten mit durchgestrichener Filzstiftschrift. Der obere Karton wird an beiden Grifflöchern waagerecht weggetragen und mit aufgeklappten Deckelklappen wieder gepackt; zusammen {TOTAL_KG:.1f} kg."),
    "tags": ["used", "household", "second-hand", "hallway", "moving-box", "corrugated-board", "cardboard", "box-stack", "removal", "storage", "object"],
    "provenance": {"author": "Semio staffage content (used household objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES,
                   "notes": T(f"German removal boxes of the regular slotted type (FEFCO 0201) as a second-hand store or a flat clearance passes them on: double-wall BC kraft board 7 mm, {BOARD_KG_M2 * 1000:.0f} g/m2, inner 600 x 330 x 340 mm, outer 614 x 344 x 354 mm; long walls and end walls, die-cut 100 x 30 mm hand holes 70 mm below the top of each end wall, bottom and lid of two minor flaps under two major flaps 165 mm deep, a glued 35 mm manufacturer's joint. Three boxes taped shut and stacked; the bottom one has crept 12 mm under earlier loads (342 mm) and carries a crease, the middle one a torn hand hole taped over, all old paper labels with marker writing; the fourth box is folded flat (0.96 x 0.70 m, two layers) and lies under the stack. Masses from the modelled 1:50 boards x density: {MASS_NOTE}.", f"Deutsche Umzugskartons der Bauart Faltschachtel (FEFCO 0201), wie ein Gebrauchtwarenkaufhaus oder eine Haushaltsauflösung sie weitergibt: Doppelwellpappe BC aus Kraftpapier 7 mm, {BOARD_KG_M2 * 1000:.0f} g/m2, innen 600 x 330 x 340 mm, außen 614 x 344 x 354 mm; Längs- und Stirnwände, gestanzte Grifflöcher 100 x 30 mm 70 mm unter der Oberkante jeder Stirnwand, Boden und Deckel aus zwei kleinen Klappen unter zwei großen, 165 mm tief, eine geklebte 35-mm-Herstellerfuge. Drei Kartons zugeklebt und gestapelt; der untere ist unter früheren Lasten um 12 mm gekrochen (342 mm) und geknickt, der mittlere hat ein eingerissenes, überklebtes Griffloch, alle tragen alte Papieretiketten mit Filzstiftschrift; der vierte Karton ist flach gefaltet (0,96 x 0,70 m, zwei Lagen) und liegt unter dem Stapel. Massen aus den 1:50-Platten x Dichte: {MASS_NOTE}.")},
    "dimensions": {"heightM": round(TOP, 4), "widthM": FX, "depthM": FZ, "massKg": round(TOTAL_KG, 5)},
    "materials": MATERIALS,
    "rig": {"kind": "mechanical", "root": "ground", "bones": [dict(b, **({"mass": {"kg": round(MASS[b["id"]]["kg"], 5), "comX": round(MASS[b["id"]]["comX"], 5), "comY": round(MASS[b["id"]]["comY"], 5), "distribution": "point", "source": "authored-estimate", "reference": "removal-box-type"}} if b["id"] in MASS else {})) for b in BONES]},
    "anchors": ANCHORS,
    "representations": {"50": rep50(), "100": rep100(), "200": rep200(), "500": rep500()},
    "symbols": [SYMBOL],
    "poses": POSES,
    "clips": CLIPS,
    "thermal": {"environment": {"airTempC": 18, "meanRadiantTempC": 18, "relativeHumidity": 0.5, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 300, "diffuseFraction": 0.2, "pressurePa": 101325}, "operating": [], "regions": REGIONS, "emitters": [], "passengers": [],
                "assumptions": [
                    T(f"Corrugated board BC 7 mm of {BOARD_KG_M2 * 1000:.0f} g/m2 ({RHO_BOARD} kg/m3 over its thickness), 0.064 W/mK across the board with its air-filled flutes and 1340 J/kgK (paper after ISO 10456); areal heat capacity rho c t, the whole board takes part within an hour.", f"Wellpappe BC 7 mm mit {BOARD_KG_M2 * 1000:.0f} g/m2 ({RHO_BOARD} kg/m3 über die Dicke), 0,064 W/mK quer zur Platte mit ihren luftgefüllten Wellen und 1340 J/kgK (Papier nach ISO 10456); flächenbezogene Wärmekapazität rho c t, die ganze Platte nimmt innerhalb einer Stunde teil."),
                    T("Aged surfaces: the scuffed, damp-darkened kraft of the bottom box and the flattened box absorbs more sun (0.80) than the cleaner reused kraft of the upper boxes (0.75); paper labels are lighter (0.35); authored estimates from the range of paper surfaces in Incropera.", "Gealterte Oberflächen: das verschrammte, feuchtigkeitsdunkle Kraftpapier des unteren und des flachen Kartons absorbiert mehr Sonne (0,80) als das sauberere wiederverwendete der oberen Kartons (0,75); Papieretiketten sind heller (0,35); Schätzwerte aus dem Bereich von Papieroberflächen nach Incropera."),
                    T("Each region is the outer surface of its boxes: the inside of an empty taped box is still air behind two 30 mm hand holes and is not counted; a face resting on the closed lid of the box below (the bottom of the top box, the bottom of the middle box on the bottom one) lies between two boards and is not counted, so areaM2 is the outer surface less these resting faces (top box 1.10 less 0.21 m2); the flattened box lies with 0.48 of its surface on a 16 degC depot floor.", "Jede Region ist die Außenfläche ihrer Kartons: das Innere eines leeren zugeklebten Kartons ist ruhende Luft hinter zwei 30-mm-Grifflöchern und wird nicht gezählt; eine Fläche, die auf dem geschlossenen Deckel des Kartons darunter aufliegt (der Boden des oberen, der Boden des mittleren auf dem unteren), liegt zwischen zwei Platten und wird nicht gezählt, so ist areaM2 die Außenfläche ohne diese Auflageflächen (oberer Karton 1,10 ohne 0,21 m2); der flache Karton liegt mit 0,48 seiner Oberfläche auf einem 16 degC kühlen Lagerboden."),
                    T("No electrical element: the boxes are passive objects, 0 W stated explicitly on the region of the top box; the packed books are a load, not part of the asset.", "Kein elektrisches Element: die Kartons sind passive Gegenstände, 0 W ausdrücklich an der Region des oberen Kartons angegeben; die gepackten Bücher sind eine Last, nicht Teil des Assets."),
                ]},
    "structure": {"gravityMS2": G, "dynamics": "quasi-static", "supports": [],
                  "assumptions": [
                      T("One rigid body on the contacts of each pose: stored, the stack stands through the flattened box on the depot floor (four corner contacts); carried, the two hands in the hand holes are pins at the end walls; in use, the top box stands on its four bottom corners.", "Ein starrer Körper auf den Kontakten jeder Pose: gelagert steht der Stapel über den flachen Karton auf dem Lagerboden (vier Eckkontakte); getragen sind die beiden Hände in den Grifflöchern Gelenklager an den Stirnwänden; in Gebrauch steht der obere Karton auf seinen vier unteren Ecken."),
                      T(f"The flattened box and the two stacked boxes ({BOX_KG['flat-box'] + BOX_KG['box-1'] + BOX_KG['box-2']:.2f} kg) lie on their own when the top box is lifted: their floor reactions are entered as upward loads at their centres in the carried and in-use poses, so the hands carry the top box alone ({BOX_KG['box-3']:.2f} kg = {BOX_KG['box-3'] * G:.1f} N).", f"Der flache Karton und die beiden gestapelten Kartons ({BOX_KG['flat-box'] + BOX_KG['box-1'] + BOX_KG['box-2']:.2f} kg) liegen für sich, wenn der obere Karton gehoben wird: ihre Bodenreaktionen sind in der Trage- und Gebrauchspose als aufwärts gerichtete Lasten in ihrer Mitte angesetzt, so tragen die Hände nur den oberen Karton ({BOX_KG['box-3']:.2f} kg = {BOX_KG['box-3'] * G:.1f} N)."),
                      T("Members: the 7 mm double-wall boards as plates (homogenised in-plane modulus about 0.4 GPa, compressive strength ECT / t about 1.0 MPa after DIN 55468-1 and ISO 3037; worn board of the bottom box 40 % lower); box compression strength after McKee about 4.8 kN new and 2.4-3.4 kN reused, against 22 N of the two empty boxes above the bottom one and 150 N of books in use. Masses are section x density of the 1:50 boards.", "Stäbe: die 7-mm-Doppelwellplatten als Scheiben (homogenisierter Modul in der Ebene etwa 0,4 GPa, Druckfestigkeit ECT / t etwa 1,0 MPa nach DIN 55468-1 und ISO 3037; abgenutzte Pappe des unteren Kartons 40 % weniger); Stauchwiderstand der Schachtel nach McKee etwa 4,8 kN neu und 2,4-3,4 kN wiederverwendet, gegen 22 N der beiden leeren Kartons über dem unteren und 150 N Bücher in Gebrauch. Massen sind Querschnitt x Dichte der 1:50-Platten."),
                      T(f"In use: {BOOKS_KG:.0f} kg of books (about half the 30-40 kg the box type is rated for; authored estimate) rest on the double bottom, which carries them to the bottom corners.", f"In Gebrauch: {BOOKS_KG:.0f} kg Bücher (etwa die Hälfte der 30-40 kg, für die der Kartontyp ausgelegt ist; Schätzwert) liegen auf dem Doppelboden, der sie zu den unteren Ecken trägt."),
                  ], "clinical": False},
    "volumes": VOLUMES,
    "presentation": {"defaultPose": "stored", "defaultClip": "carry", "defaultScale": 50},
    "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    "completion": "authored",
}

ORDER = ["id", "category", "title", "summary", "tags", "provenance", "dimensions", "materials", "rig", "anchors", "representations", "symbols", "poses", "clips", "thermal", "structure", "volumes", "presentation", "exports", "completion"]
DOC = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": {k: ASSET[k] for k in ORDER}}

if __name__ == "__main__":
    (HERE / "🔣️.json").write_text(json.dumps(DOC, ensure_ascii=False, indent=1) + "\n")
    for b, m in MASS.items():
        print(f"[DEBUG] {b}: {m['kg']:.4f} kg com ({m['comX']:.4f}, {m['comY']:.4f}, {m['comZ']:.4f})")
    print(f"[DEBUG] total {TOTAL_KG:.4f} kg, boxes {BOX_KG}, board density {RHO_BOARD}, lift {LIFT}, areas {AREA}")
    for b in ("flap-r", "flap-l", "flap-f", "flap-b"):
        o, rot = FR[b]
        print(f"[DEBUG] frame {b}: o {[round(c, 4) for c in o]} u {[round(rot[i][0], 3) for i in range(3)]} v {[round(rot[i][1], 3) for i in range(3)]} w {[round(rot[i][2], 3) for i in range(3)]}")
