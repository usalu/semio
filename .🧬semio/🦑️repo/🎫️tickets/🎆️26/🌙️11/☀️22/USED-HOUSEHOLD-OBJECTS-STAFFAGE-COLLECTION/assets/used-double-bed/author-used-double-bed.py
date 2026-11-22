"""🛏️ Authors the staffage asset `used-double-bed` from hand-chosen numbers and writes `🔣️.json` next to this file.

Rest pose = the stored stack on a moving blanket in a second-hand store; the assembled bed and the carried mattress come from joint offsets.
World frame: x ahead (head end at small x), y up, z left; every part is a true prism, loft or box per scale, never a filtered copy.
@see ../../brief.md
@see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md
"""

import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}


def T(en, de):
    return {"en": en, "de": de}


BONES = [
    ("ground", None, 0.0, 0.0, 0.0, 0.0, {"type": "fixed"}, T("Floor of the store", "Boden des Lagers")),
    ("blanket", "ground", -0.02, 0.0, 0.0, 2.40, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 2.4}, T("Moving blanket under the stack, pulled aside when the bed is set up", "Möbeldecke unter dem Stapel, beim Aufbau beiseitegezogen")),
    ("rail-l", "ground", 0.10, 0.006, 0.0, 2.005, {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.125}, T("Left side rail with ledger and hook plates", "Linker Seitenholm mit Auflageleiste und Hakenbeschlägen")),
    ("rail-r", "ground", 0.10, 0.006, 0.0, 2.005, {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.125}, T("Right side rail with ledger and hook plates", "Rechter Seitenholm mit Auflageleiste und Hakenbeschlägen")),
    ("beam", "ground", 0.0775, 0.006, 0.0, 2.05, {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.125}, T("Centre beam between the two slatted bases", "Mittelholm zwischen den beiden Lattenrosten")),
    ("base-l", "ground", 0.1025, 0.081, 0.0, 2.0, {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.125}, T("Left slatted base 0.80 x 2.00 m", "Linker Lattenrost 0,80 x 2,00 m")),
    ("base-r", "ground", 0.1025, 0.081, 0.0, 2.0, {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.125}, T("Right slatted base 0.80 x 2.00 m with one replaced slat", "Rechter Lattenrost 0,80 x 2,00 m mit einer ersetzten Leiste")),
    ("headboard-slide", "ground", 0.10, 0.206, 0.0, 0.0, {"type": "prismatic", "axisDeg": 90, "minM": -0.206, "maxM": 0}, T("Lowering of the headboard from the stack to the floor", "Absenken des Kopfteils vom Stapel auf den Boden")),
    ("headboard", "headboard-slide", 0.0, 0.0, 0.0, 0.92, {"type": "revolute", "minDeg": 0, "maxDeg": 90}, T("Headboard: two posts, end rail, boarded panel and top rail, turning about the front edge of its post feet", "Kopfteil: zwei Pfosten, Querriegel, Brettfüllung und Abdeckleiste, drehend um die Vorderkante der Pfostenfüße")),
    ("footboard-slide", "ground", 2.105, 0.206, 0.0, 0.0, {"type": "prismatic", "axisDeg": 90, "minM": -0.206, "maxM": 0}, T("Lowering of the footboard from the stack to the floor", "Absenken des Fußteils vom Stapel auf den Boden")),
    ("footboard", "footboard-slide", 0.0, 0.0, 180.0, 0.42, {"type": "revolute", "minDeg": -90, "maxDeg": 0}, T("Footboard: two posts, end rail, panel and top rail, turning about the inner edge of its post feet", "Fußteil: zwei Pfosten, Querriegel, Füllung und Abdeckleiste, drehend um die Innenkante der Pfostenfüße")),
    ("leg-slide", "ground", 1.1025, 0.186, 0.0, 0.0, {"type": "prismatic", "axisDeg": 90, "minM": -0.186, "maxM": 0}, T("Lowering of the unscrewed centre leg", "Absenken des abgeschraubten Mittelfußes")),
    ("leg", "leg-slide", 0.0, 0.0, 0.0, 0.131, {"type": "revolute", "minDeg": 0, "maxDeg": 90}, T("Centre leg 70 x 70 mm with its hanger bolt", "Mittelfuß 70 x 70 mm mit Stockschraube")),
    ("mattress-l-carry", "ground", 1.1025, 0.376, 0.0, 0.0, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.8}, T("Walking the left mattress off the stack", "Wegtragen der linken Matratze vom Stapel")),
    ("mattress-l-lift", "mattress-l-carry", 0.0, 0.0, 0.0, 0.0, {"type": "prismatic", "axisDeg": 90, "minM": -0.02, "maxM": 0.4}, T("Lifting of the left mattress (and its sinking under a sleeper)", "Anheben der linken Matratze (und ihr Einsinken unter einer schlafenden Person)")),
    ("mattress-l", "mattress-l-lift", 0.0, 0.0, 0.0, 1.0, {"type": "revolute", "minDeg": -15, "maxDeg": 15}, T("Left pocket-spring mattress 0.80 x 2.00 x 0.20 m, pitching about its centre when carried", "Linke Taschenfederkernmatratze 0,80 x 2,00 x 0,20 m, beim Tragen um ihre Mitte geneigt")),
    ("mattress-r", "ground", 1.1025, 0.376, 0.0, 1.0, {"type": "prismatic", "axisDeg": 90, "minM": -0.02, "maxM": 0}, T("Right pocket-spring mattress 0.80 x 2.00 x 0.20 m", "Rechte Taschenfederkernmatratze 0,80 x 2,00 x 0,20 m")),
]
BONE = {b[0]: {"parent": b[1], "x": b[2], "y": b[3], "angle": b[4], "length": b[5], "joint": b[6], "title": b[7]} for b in BONES}
ROTATING = {"headboard", "footboard", "leg", "mattress-l"}


def origin(bone):
    chain = []
    while bone is not None:
        chain.append(bone)
        bone = BONE[bone]["parent"]
    x = y = a = 0.0
    for b in reversed(chain):
        c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
        x, y = x + c * BONE[b]["x"] - s * BONE[b]["y"], y + s * BONE[b]["x"] + c * BONE[b]["y"]
        a += BONE[b]["angle"]
    return x, y, a


def world(bone, u, v):
    ox, oy, a = origin(bone)
    r = math.radians(a)
    return ox + math.cos(r) * u - math.sin(r) * v, oy + math.sin(r) * u + math.cos(r) * v


def R(x):
    return round(x + 0.0, 5)


def pts(points):
    return [[R(p[0]), R(p[1])] for p in points]


def rect(u0, v0, u1, v1):
    return [(u0, v0), (u1, v0), (u1, v1), (u0, v1)]


def capped(u0, v0, u1, v1, r, n=3):
    """🍞️ A rectangle whose two corners at the far end of u (the top of a post or rail) are rounded with radius r."""
    out = [(u0, v0)]
    for k in range(n + 1):
        t = math.radians(-90 + 90 * k / n)
        out.append((u1 - r + r * math.cos(t), v0 + r + r * math.sin(t)))
    for k in range(n + 1):
        t = math.radians(90 * k / n)
        out.append((u1 - r + r * math.cos(t), v1 - r + r * math.sin(t)))
    out.append((u0, v1))
    return out


def rounded(u0, v0, u1, v1, r, n=3):
    out = []
    for cx, cy, start in ((u1 - r, v0 + r, -90), (u1 - r, v1 - r, 0), (u0 + r, v1 - r, 90), (u0 + r, v0 + r, 180)):
        for k in range(n + 1):
            t = math.radians(start + 90 * k / n)
            out.append((cx + r * math.cos(t), cy + r * math.sin(t)))
    return out


def area(poly):
    return 0.5 * abs(sum(poly[i][0] * poly[(i + 1) % len(poly)][1] - poly[(i + 1) % len(poly)][0] * poly[i][1] for i in range(len(poly))))


def centroid(poly):
    a = 0.0
    cx = cy = 0.0
    for i in range(len(poly)):
        x0, y0 = poly[i]
        x1, y1 = poly[(i + 1) % len(poly)]
        c = x0 * y1 - x1 * y0
        a += c
        cx += (x0 + x1) * c
        cy += (y0 + y1) * c
    a *= 0.5
    return cx / (6 * a), cy / (6 * a)


def dip(u, depth):
    """🛌️ The permanent lying hollow of a worn pocket-spring mattress: a cosine trough from u = -0.70 to +0.40 (head at u = -1), deepest at the hips."""
    a, b = -0.70, 0.40
    if u <= a or u >= b:
        return 0.0
    return depth * 0.5 * (1 - math.cos(2 * math.pi * (u - a) / (b - a)))


DIP = 0.018


def mattress_sections(scale, wc, shrink=0.0):
    depth = DIP if scale in (50, 100) else 0.0
    ends = [(-1.0, 0.072, 0.372), (-0.99, 0.09, 0.39), (-0.975, 0.098, 0.398), (-0.95, 0.1, 0.4)] if scale == 50 else [(-1.0, 0.08, 0.38), (-0.98, 0.097, 0.397), (-0.95, 0.1, 0.4)]
    inner = {50: [-0.8, -0.7, -0.6, -0.5, -0.4, -0.3, -0.2, -0.1, 0.0, 0.1, 0.2, 0.3, 0.4, 0.6, 0.8], 100: [-0.7, -0.5, -0.35, -0.2, -0.05, 0.1, 0.25, 0.4]}[scale]
    rows = list(ends) + [(u, 0.1, 0.4) for u in inner] + [(-u, rx, ry) for (u, rx, ry) in reversed(ends)]
    out = []
    for u, rx, ry in rows:
        d = dip(u, depth)
        uu = u * (1 - shrink / 1.0) if shrink else u
        out.append({"at": [R(uu), R(-d / 2), wc], "rx": R(rx - d / 2 - shrink), "ry": R(ry - shrink), "exponent": 20, "rotation": [0.5, 0.5, 0.5, 0.5]})
    return out


def outline_of(sections):
    bottom = [(s["at"][0], s["at"][1] - s["rx"]) for s in sections]
    top = [(s["at"][0], s["at"][1] + s["rx"]) for s in sections]
    return bottom + list(reversed(top))


def loft_volume(sections):
    n = sections[0]["exponent"]
    k = math.gamma(1 + 1 / n) ** 2 / math.gamma(1 + 2 / n)
    total = 0.0
    for a, b in zip(sections, sections[1:]):
        aa, ab = 4 * a["rx"] * a["ry"] * k, 4 * b["rx"] * b["ry"] * k
        total += 0.5 * (aa + ab) * abs(b["at"][0] - a["at"][0])
    return total


SLATS = [R(0.055 + 0.0704 * k) for k in range(28)]
PITCH = 0.0704


def W(id, bone, profile, spans, material, tone, region=None, layer=None, role="body", hatch=None, sheet=False, member=False, joint=False, mass=True, solid3d=None):
    return {"id": id, "bone": bone, "kind": "w", "profile": profile, "spans": spans, "material": material, "tone": tone, "region": region, "layer": layer, "role": role, "hatch": hatch, "sheet": sheet, "member": member, "joint": joint, "mass": mass, "solid3d": solid3d}


def U(id, bone, vw, a, b, material, tone, region=None, layer=None, member=False):
    return {"id": id, "bone": bone, "kind": "u", "vw": vw, "a": a, "b": b, "material": material, "tone": tone, "region": region, "layer": layer, "role": "body", "hatch": None, "sheet": False, "member": member, "joint": False, "mass": True}


def B(id, bone, c, size, material, tone, region=None, layer=None, r=0.0, count=1, du=0.0, member=False, joint=False, role="body"):
    return {"id": id, "bone": bone, "kind": "box", "c": c, "size": size, "r": r, "count": count, "du": du, "material": material, "tone": tone, "region": region, "layer": layer, "role": role, "hatch": None, "sheet": False, "member": member, "joint": joint, "mass": True}


def L(id, bone, sections, span, material, tone, region, layer):
    return {"id": id, "bone": bone, "kind": "loft", "sections": sections, "span": span, "material": material, "tone": tone, "region": region, "layer": layer, "role": "body", "hatch": None, "sheet": False, "member": True, "joint": False, "mass": True}


def rails(scale):
    out = []
    for side, s in (("l", 1), ("r", -1)):
        bone = f"rail-{side}"
        z = lambda a, b: (min(s * a, s * b), max(s * a, s * b))
        if scale == 50:
            prof = rect(0, 0, 2.005, 0.2) if side == "l" else [(0, 0), (2.005, 0), (2.005, 0.2), (1.345, 0.2), (1.337, 0.192), (1.308, 0.192), (1.30, 0.2), (0, 0.2)]
            out.append(W(bone, bone, prof, [z(0.805, 0.835)], "pine-aged", 0.55, "pine-frame", "pine", hatch="grain-h", member=True))
            out.append(W(f"ledger-{side}", bone, rect(0.07, 0.031, 1.935, 0.075), [z(0.781, 0.805)], "pine-aged", 0.5, "pine-frame", "pine", member=True))
            out.append(W(f"hooks-{side}-head", bone, rect(0.0, 0.04, 0.065, 0.16), [z(0.802, 0.805)], "steel-fitting", 0.3, role="detail", sheet=True, joint=True))
            out.append(W(f"hooks-{side}-foot", bone, rect(1.94, 0.04, 2.005, 0.16), [z(0.802, 0.805)], "steel-fitting", 0.3, role="detail", sheet=True, joint=True))
        elif scale == 100:
            vw = [(0, 0.805), (0, 0.835), (0.2, 0.835), (0.2, 0.805), (0.075, 0.805), (0.075, 0.781), (0.031, 0.781), (0.031, 0.805)]
            out.append(U(bone, bone, [(v, s * w) for v, w in (vw if s > 0 else list(reversed(vw)))], 0.0, 2.005, "pine-aged", 0.55, "pine-frame", "pine", member=True))
        else:
            out.append(W(bone, bone, rect(0, 0, 2.005, 0.2), [z(0.781, 0.835)], "pine-aged", 0.55, "pine-frame", "pine", member=True))
    return out


def bases(scale):
    out = []
    for side, s in (("l", 1), ("r", -1)):
        bone = f"base-{side}"
        z = lambda a, b: (min(s * a, s * b), max(s * a, s * b))
        if scale in (50, 100):
            out.append(W(f"{bone}-sides", bone, rect(0, 0, 2.0, 0.05), [z(0.0025, 0.0375), z(0.7675, 0.8025)], "spruce-batten", 0.55, "slatted-bases", "birch", member=True))
            out.append(W(f"{bone}-end-head", bone, rect(0, 0, 0.035, 0.05), [z(0.0375, 0.7675)], "spruce-batten", 0.55, "slatted-bases", "birch", member=True))
            out.append(W(f"{bone}-end-foot", bone, rect(1.965, 0, 2.0, 0.05), [z(0.0375, 0.7675)], "spruce-batten", 0.55, "slatted-bases", "birch", member=True))
        if scale == 50:
            for row, w in (("in", 0.02), ("out", 0.785)):
                out.append(B(f"{bone}-caps-{row}", bone, (SLATS[0], 0.056, s * w), (0.05, 0.012, 0.035), "pp-cap", 0.25, "slatted-bases", "birch", count=28, du=PITCH, joint=True))
            if side == "l":
                out.append(B(f"{bone}-slats", bone, (SLATS[0], 0.066, s * 0.4025), (0.038, 0.008, 0.785), "birch-slat", 0.62, "slatted-bases", "birch", count=28, du=PITCH, member=True))
            else:
                out.append(B(f"{bone}-slats-head", bone, (SLATS[0], 0.066, s * 0.4025), (0.038, 0.008, 0.785), "birch-slat", 0.62, "slatted-bases", "birch", count=13, du=PITCH, member=True))
                out.append(B(f"{bone}-slat-new", bone, (SLATS[13], 0.066, s * 0.4025), (0.038, 0.008, 0.785), "birch-slat-new", 0.85, "slatted-bases", "birch", member=True))
                out.append(B(f"{bone}-slats-foot", bone, (SLATS[14], 0.066, s * 0.4025), (0.038, 0.008, 0.785), "birch-slat", 0.62, "slatted-bases", "birch", count=14, du=PITCH, member=True))
        elif scale == 100:
            out.append(B(f"{bone}-deck", bone, (1.0, 0.06, s * 0.4025), (1.93, 0.02, 0.785), "birch-slat", 0.62, "slatted-bases", "birch", member=True))
        else:
            out.append(W(bone, bone, rect(0, 0, 2.0, 0.07), [z(0.0025, 0.8025)], "birch-slat", 0.6, "slatted-bases", "birch", member=True))
    return out


def boards(scale):
    out = []
    left, right, mid = (0.765, 0.835), (-0.835, -0.765), (-0.765, 0.765)
    if scale == 50:
        out.append(W("head-post-r", "headboard", [(0, 0), (0, 0.058), (0.015, 0.07), (0.914, 0.07), (0.92, 0.064), (0.92, 0.006), (0.914, 0)], [right], "pine-aged", 0.5, "pine-frame", "pine", member=True))
        out.append(W("head-post-l", "headboard", [(0, 0), (0, 0.07), (0.914, 0.07), (0.92, 0.064), (0.92, 0.006), (0.914, 0)], [left], "pine-aged", 0.5, "pine-frame", "pine", member=True))
        out.append(W("head-end-rail", "headboard", rect(0.131, 0.0225, 0.331, 0.0475), [mid], "pine-aged", 0.55, "pine-frame", "pine", member=True))
        out.append(W("head-panel", "headboard", rect(0.331, 0.025, 0.86, 0.045), [mid], "pine-aged", 0.6, "pine-frame", "pine"))
        out.append(W("head-top-rail", "headboard", capped(0.86, 0.015, 0.92, 0.055, 0.012), [mid], "pine-aged", 0.5, "pine-frame", "pine", member=True))
        out.append(W("foot-post-r", "footboard", [(0, 0), (0, -0.058), (0.012, -0.07), (0.414, -0.07), (0.42, -0.064), (0.42, -0.006), (0.414, 0)], [right], "pine-aged", 0.5, "pine-frame", "pine", member=True))
        out.append(W("foot-post-l", "footboard", [(0, 0), (0, -0.07), (0.414, -0.07), (0.42, -0.064), (0.42, -0.006), (0.414, 0)], [left], "pine-aged", 0.5, "pine-frame", "pine", member=True))
        out.append(W("foot-end-rail", "footboard", rect(0.131, -0.0475, 0.331, -0.0225), [mid], "pine-aged", 0.55, "pine-frame", "pine", member=True))
        out.append(W("foot-panel", "footboard", rect(0.331, -0.045, 0.36, -0.025), [mid], "pine-aged", 0.6, "pine-frame", "pine"))
        out.append(W("foot-top-rail", "footboard", capped(0.36, -0.055, 0.42, -0.015, 0.012), [mid], "pine-aged", 0.5, "pine-frame", "pine", member=True))
    elif scale == 100:
        out.append(W("head-posts", "headboard", rect(0, 0, 0.92, 0.07), [right, left], "pine-aged", 0.5, "pine-frame", "pine", member=True))
        out.append(W("head-end-rail", "headboard", rect(0.131, 0.0225, 0.331, 0.0475), [mid], "pine-aged", 0.55, "pine-frame", "pine", member=True))
        out.append(W("head-panel", "headboard", rect(0.331, 0.025, 0.86, 0.045), [mid], "pine-aged", 0.6, "pine-frame", "pine"))
        out.append(W("head-top-rail", "headboard", rect(0.86, 0.015, 0.92, 0.055), [mid], "pine-aged", 0.5, "pine-frame", "pine", member=True))
        out.append(W("foot-posts", "footboard", rect(0, -0.07, 0.42, 0), [right, left], "pine-aged", 0.5, "pine-frame", "pine", member=True))
        out.append(W("foot-end-rail", "footboard", rect(0.131, -0.0475, 0.331, -0.0225), [mid], "pine-aged", 0.55, "pine-frame", "pine", member=True))
        out.append(W("foot-top-rail", "footboard", rect(0.331, -0.055, 0.42, -0.015), [mid], "pine-aged", 0.5, "pine-frame", "pine", member=True))
    else:
        out.append(W("head-posts", "headboard", rect(0, 0, 0.92, 0.07), [right, left], "pine-aged", 0.5, "pine-frame", "pine", member=True))
        out.append(W("head-board", "headboard", rect(0.131, 0.015, 0.92, 0.055), [mid], "pine-aged", 0.55, "pine-frame", "pine", member=True))
        out.append(W("foot-posts", "footboard", rect(0, -0.07, 0.42, 0), [right, left], "pine-aged", 0.5, "pine-frame", "pine", member=True))
        out.append(W("foot-board", "footboard", rect(0.131, -0.055, 0.42, -0.015), [mid], "pine-aged", 0.55, "pine-frame", "pine", member=True))
    return out


def mattresses(scale):
    out = []
    for side, s in (("l", 1), ("r", -1)):
        bone = f"mattress-{side}"
        span = (0.0, 0.8) if s > 0 else (-0.8, 0.0)
        if scale in (50, 100):
            out.append(L(bone, bone, mattress_sections(scale, R(s * 0.4)), span, "ticking-faded", 0.85, "mattresses", "ticking"))
        elif scale == 200:
            out.append(W(bone, bone, rounded(-1.0, -0.1, 1.0, 0.1, 0.04, 2), [span], "ticking-faded", 0.85, "mattresses", "ticking", member=True))
        else:
            out.append(W(bone, bone, rect(-1.0025, -0.1, 1.0025, 0.1), [(0.0, 0.835) if s > 0 else (-0.835, 0.0)], "ticking-faded", 0.85, "mattresses", "ticking", member=True))
        if scale == 50:
            hz = (0.8, 0.808) if s > 0 else (-0.808, -0.8)
            out.append(W(f"{bone}-handle-head", bone, rect(-0.875, -0.015, -0.725, 0.015), [hz], "webbing", 0.7, "mattresses", "ticking"))
            out.append(W(f"{bone}-handle-foot", bone, rect(0.725, -0.015, 0.875, 0.015), [hz], "webbing", 0.7, "mattresses", "ticking"))
    return out


def parts(scale):
    band = 0.02 if scale == 500 else 0.006
    out = [W("blanket", "blanket", rect(0, 0, 2.40, band), [(-0.90, 0.90)], "moving-blanket", 0.3, "moving-blanket", "felt", sheet=True, member=True)]
    if scale != 500:
        out.append(B("hardware-bag", "blanket", (2.26, 0.026, -0.595), (0.18, 0.04, 0.13), "hardware-bag", 0.8, r=0.012 if scale == 50 else 0.0))
    out += rails(scale)
    out.append(W("beam", "beam", rect(0, 0, 2.05, 0.075), [(-0.02, 0.02)], "pine-aged", 0.5, "pine-frame", "pine", hatch="grain-h" if scale == 50 else None, member=True))
    out += bases(scale)
    out += boards(scale)
    out.append(W("leg", "leg", rect(0, -0.035, 0.131, 0.035), [(-0.035, 0.035)], "pine-aged", 0.5, "pine-frame", "pine", member=True))
    if scale == 50:
        out.append(W("leg-bolt", "leg", rect(0.131, -0.004, 0.161, 0.004), [(-0.004, 0.004)], "steel-fitting", 0.3, role="detail", joint=True, solid3d={"kind": "revolve", "profile": [[0.131, 0.0], [0.131, 0.004], [0.161, 0.004], [0.161, 0.0]], "axis": [1, 0, 0], "origin": [0, 0, 0]}))
    out += mattresses(scale)
    return out


def zord(view, p):
    if p["kind"] == "w":
        zs = [z for sp in p["spans"] for z in sp]
        xy = [world(p["bone"], u, v) for u, v in p["profile"]]
    elif p["kind"] == "u":
        zs = [w for _, w in p["vw"]]
        xy = [world(p["bone"], p["a"], v) for v, _ in p["vw"]] + [world(p["bone"], p["b"], v) for v, _ in p["vw"]]
    elif p["kind"] == "box":
        cu, cv, cw = p["c"]
        su, sv, sw = p["size"]
        zs = [cw - sw / 2, cw + sw / 2]
        xy = [world(p["bone"], cu - su / 2, cv - sv / 2), world(p["bone"], cu + su / 2 + p["du"] * (p["count"] - 1), cv + sv / 2)]
    else:
        zs = list(p["span"])
        xy = [world(p["bone"], u, v) for u, v in outline_of(p["sections"])]
    if view == "elevation":
        return R(5 - min(zs))
    if view == "front":
        return R(max(x for x, _ in xy))
    return R(max(y for _, y in xy))


def base_shape(p, sid, view, geometry):
    s = {"id": sid, "bone": p["bone"], "role": p["role"], "z": zord(view, p), "filled": True, "tone": p["tone"]}
    s.update(geometry)
    if p["region"]:
        s["region"] = p["region"]
    s["material"] = p["material"]
    if p["hatch"] and view == "elevation":
        s["hatch"] = p["hatch"]
    return s


def poly(points):
    return {"kind": "poly", "points": pts(points), "closed": True}


def path_rects(rects):
    d = " ".join(f"M {R(a)} {R(b)} L {R(c)} {R(b)} L {R(c)} {R(e)} L {R(a)} {R(e)} Z" for a, b, c, e in rects)
    return {"kind": "path", "d": d}


def shapes_of(p, view):
    bone = p["bone"]
    out = []
    if p["kind"] in ("w", "loft"):
        profile = p["profile"] if p["kind"] == "w" else outline_of(p["sections"])
        spans = p["spans"] if p["kind"] == "w" else [p["span"]]
        if view == "elevation":
            return [base_shape(p, p["id"], view, poly(profile))]
        for k, (z0, z1) in enumerate(spans):
            sid = p["id"] if len(spans) == 1 else f"{p['id']}-{k}"
            if bone in ROTATING or p["kind"] == "loft":
                g = poly(profile)
                g["solid"] = {"kind": "extruded", "sweep": "w", "z0": R(z0), "z1": R(z1)}
                out.append(base_shape(p, sid, view, g))
            else:
                xy = [world(bone, u, v) for u, v in profile]
                if view == "front":
                    ys = [y for _, y in xy]
                    out.append(base_shape(p, sid, view, poly(rect(z0, min(ys), z1, max(ys)))))
                else:
                    xs = [x for x, _ in xy]
                    out.append(base_shape(p, sid, view, poly(rect(min(xs), z0, max(xs), z1))))
        return out
    if p["kind"] == "u":
        vs = [v for v, _ in p["vw"]]
        if view == "elevation":
            return [base_shape(p, p["id"], view, poly(rect(p["a"], min(vs), p["b"], max(vs))))]
        ox, oy, _ = origin(bone)
        if view == "front":
            return [base_shape(p, p["id"], view, poly([(w, oy + v) for v, w in p["vw"]]))]
        ws = [w for _, w in p["vw"]]
        return [base_shape(p, p["id"], view, poly(rect(ox + p["a"], min(ws), ox + p["b"], max(ws))))]
    cu, cv, cw = p["c"]
    su, sv, sw = p["size"]
    copies = [(cu + k * p["du"] - su / 2, cu + k * p["du"] + su / 2) for k in range(p["count"])]
    ox, oy, _ = origin(bone)
    if view == "elevation":
        if p["count"] == 1:
            g = poly(rounded(copies[0][0], cv - sv / 2, copies[0][1], cv + sv / 2, p["r"]) if p["r"] else rect(copies[0][0], cv - sv / 2, copies[0][1], cv + sv / 2))
        else:
            g = path_rects([(a, cv - sv / 2, b, cv + sv / 2) for a, b in copies])
        return [base_shape(p, p["id"], view, g)]
    if view == "front":
        return [base_shape(p, p["id"], view, poly(rect(cw - sw / 2, oy + cv - sv / 2, cw + sw / 2, oy + cv + sv / 2)))]
    if p["count"] == 1:
        return [base_shape(p, p["id"], view, poly(rect(ox + copies[0][0], cw - sw / 2, ox + copies[0][1], cw + sw / 2)))]
    return [base_shape(p, p["id"], view, path_rects([(ox + a, cw - sw / 2, ox + b, cw + sw / 2) for a, b in copies]))]


def stroke(id, bone, role, z, points, closed=False, region=None):
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": False, "tone": 0.3, "kind": "poly", "points": pts(points), "closed": closed}
    if region:
        s["region"] = region
    return s


def textures(scale):
    el, fr, pl = [], [], []
    if scale in (50, 100):
        for side, s in (("l", 1), ("r", -1)):
            bone = f"mattress-{side}"
            secs = mattress_sections(scale, R(s * 0.4))
            top = [(q["at"][0], q["at"][1] + q["rx"] - 0.012) for q in secs if abs(q["at"][0]) <= 0.95]
            zo = 5.95 if s < 0 else 5.15
            el.append(stroke(f"{bone}-piping-top", bone, "detail", zo, top, region="mattresses"))
            el.append(stroke(f"{bone}-piping-bottom", bone, "detail", zo, [(-0.95, -0.088), (0.95, -0.088)], region="mattresses"))
        pl.append({"id": "mattress-r-stain", "bone": "mattress-r", "role": "texture", "z": 0.49, "filled": False, "tone": 0.4, "kind": "ellipse", "cx": 1.29, "cy": -0.46, "rx": 0.15, "ry": 0.09, "rotationDeg": 18, "region": "mattresses"})
    if scale == 50:
        for side, s in (("l", 1), ("r", -1)):
            bone = f"mattress-{side}"
            for k, x in enumerate([0.35, 0.6, 0.85, 1.35, 1.6, 1.85]):
                pl.append(stroke(f"{bone}-quilt-{k}", bone, "texture", 0.485, [(x, s * 0.04), (x, s * 0.76)], region="mattresses"))
        pl.append(stroke("mattress-l-price-tag", "mattress-l", "texture", 0.49, rect(1.62, 0.52, 1.72, 0.59), closed=True, region="mattresses"))
        el.append(stroke("rail-r-scratch-a", "rail-r", "texture", 5.84, [(0.42, 0.15), (0.61, 0.168)], region="pine-frame"))
        el.append(stroke("rail-r-scratch-b", "rail-r", "texture", 5.84, [(1.52, 0.058), (1.66, 0.074)], region="pine-frame"))
        el.append(stroke("rail-r-screw-holes", "rail-r", "texture", 5.84, rect(0.98, 0.095, 0.99, 0.105), closed=True, region="pine-frame"))
        el.append(stroke("head-post-r-tape", "headboard", "texture", 5.84, rect(0.40, 0.014, 0.50, 0.056), closed=True, region="pine-frame"))
        el.append(stroke("head-post-r-scuff", "headboard", "texture", 5.84, [(0.62, 0.02), (0.70, 0.035), (0.76, 0.03)], region="pine-frame"))
        el.append(stroke("foot-post-r-scuff", "footboard", "texture", 5.84, [(0.03, -0.055), (0.09, -0.045)], region="pine-frame"))
        el.append(stroke("base-r-slat-new-mark", "base-r", "texture", 3.2, rect(SLATS[13] - 0.019, 0.062, SLATS[13] + 0.019, 0.07), closed=True, region="slatted-bases"))
    return el, fr, pl


def representation(scale):
    ps = parts(scale)
    views = {"elevation": [], "front": [], "plan": []}
    for p in ps:
        for v in views:
            views[v] += shapes_of(p, v)
    el, fr, pl = textures(scale)
    views["elevation"] += el
    views["front"] += fr
    views["plan"] += pl
    pens = {50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.7}, 100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5}, 200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35}, 500: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}[scale]
    if pens["detail"] is None:
        for v in views:
            views[v] = [s for s in views[v] if s["role"] != "detail" or s.get("filled")]
            for s in views[v]:
                if s["role"] == "detail":
                    s["role"] = "body"
    hatches = [{"id": "grain-h", "title": T("Pine grain along the rail", "Kiefernmaserung längs des Holms"), "pattern": "lines", "angleDeg": 0, "spacingMm": 0.6, "penMm": 0.13}] if scale == 50 else []
    band = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": band[0], "maxDenominator": band[1]}, "pens": pens, "hatches": hatches, "geometry": views, "decisions": REP_DECISIONS[scale]}, ps


def solid_of(p):
    base = {"id": p["id"], "bone": p["bone"], "material": p["material"], "role": p["role"], "tone": p["tone"]}
    out = []
    if p["kind"] == "w" and p.get("solid3d"):
        s3 = dict(p["solid3d"])
        if s3["kind"] == "box":
            s = dict(base, kind="box", size=s3["size"], cornerRadius=s3["cornerRadius"], transform={"translation": s3["translation"]})
        else:
            s = dict(base, kind="revolve", profile=s3["profile"], axis=s3["axis"], origin=s3["origin"])
        return [s]
    if p["kind"] == "w":
        spans = p["spans"]
        widths = {R(b - a) for a, b in spans}
        if len(spans) == 2 and len(widths) == 1:
            s = dict(base, kind="extrude", profile=pts(p["profile"]), plane="xy", **{"from": R(spans[0][0]), "to": R(spans[0][1])}, repeat={"count": 2, "translation": [0.0, 0.0, R(spans[1][0] - spans[0][0])]})
            out.append(s)
        else:
            for k, (a, b) in enumerate(spans):
                s = dict(base, kind="extrude", profile=pts(p["profile"]), plane="xy", **{"from": R(a), "to": R(b)})
                if len(spans) > 1:
                    s["id"] = f"{p['id']}-{k}"
                out.append(s)
    elif p["kind"] == "u":
        out.append(dict(base, kind="extrude", profile=pts(p["vw"]), plane="yz", **{"from": R(p["a"]), "to": R(p["b"])}))
    elif p["kind"] == "box":
        s = dict(base, kind="box", size=[R(x) for x in p["size"]], transform={"translation": [R(x) for x in p["c"]]})
        if p["r"]:
            s["cornerRadius"] = p["r"]
        if p["count"] > 1:
            s["repeat"] = {"count": p["count"], "translation": [R(p["du"]), 0.0, 0.0]}
        out.append(s)
    else:
        out.append(dict(base, kind="loft", sections=p["sections"], capStart=True, capEnd=True))
    for s in out:
        if p["sheet"]:
            s["sheet"] = True
    return out


def mass_of(p):
    rho = DENSITY["pocket-spring" if p["material"] == "ticking-faded" else p["material"]]
    if p["kind"] == "w" and p.get("solid3d", {}) and p["solid3d"]["kind"] == "revolve":
        return math.pi * 0.004 ** 2 * 0.03 * rho, (0.146, 0.0)
    if p["kind"] == "w":
        a = area(p["profile"])
        L = sum(b - c for c, b in p["spans"])
        return a * L * rho, centroid(p["profile"])
    if p["kind"] == "u":
        return area(p["vw"]) * (p["b"] - p["a"]) * rho, ((p["a"] + p["b"]) / 2, centroid(p["vw"])[0])
    if p["kind"] == "box":
        su, sv, sw = p["size"]
        return su * sv * sw * p["count"] * rho, (p["c"][0] + p["du"] * (p["count"] - 1) / 2, p["c"][1])
    return loft_volume(p["sections"]) * rho, (0.0, sum(s["at"][1] for s in p["sections"]) / len(p["sections"]))


def bone_masses():
    ps = parts(50)
    agg = {}
    for p in ps:
        m, (cu, cv) = mass_of(p)
        a = agg.setdefault(p["bone"], [0.0, 0.0, 0.0])
        a[0] += m
        a[1] += m * cu
        a[2] += m * cv
    return {b: (a[0], a[1] / a[0], a[2] / a[0]) for b, a in agg.items()}


MATERIALS = [
    {"id": "pine-aged", "title": T("Solid Scots pine, waxed, yellowed by twenty years of light", "Kiefer massiv, gewachst, durch zwanzig Jahre Licht vergilbt"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 500, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000.0, "strengthPa": 24000000.0, "source": "authored-estimate", "reference": "hon-2001-photodegradation"},
    {"id": "spruce-batten", "title": T("Spruce battens of the slatted-base frames", "Fichtenleisten der Lattenrostrahmen"), "emissivity": 0.9, "solarAbsorptivity": 0.55, "densityKgM3": 450, "conductivityWmK": 0.12, "specificHeatJKgK": 1600, "elasticModulusPa": 10000000000.0, "strengthPa": 22000000.0, "source": "literature", "reference": "en-338-2016"},
    {"id": "birch-slat", "title": T("Laminated birch slats 38 x 8 mm, greyed", "Birkenschichtholz-Leisten 38 x 8 mm, vergraut"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 650, "conductivityWmK": 0.15, "specificHeatJKgK": 1600, "elasticModulusPa": 9000000000.0, "strengthPa": 40000000.0, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "birch-slat-new", "title": T("Replacement birch slat, unweathered", "Ersatzleiste aus Birke, unverwittert"), "emissivity": 0.9, "solarAbsorptivity": 0.45, "densityKgM3": 650, "conductivityWmK": 0.15, "specificHeatJKgK": 1600, "elasticModulusPa": 9000000000.0, "strengthPa": 40000000.0, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "pp-cap", "title": T("Polypropylene slat holders (hollow caps)", "Leistenträger aus Polypropylen (Hohlkappen)"), "emissivity": 0.92, "solarAbsorptivity": 0.85, "densityKgM3": 300, "conductivityWmK": 0.22, "specificHeatJKgK": 1800, "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "steel-fitting", "title": T("Zinc-plated steel hook plates and hanger bolt", "Verzinkte Stahl-Hakenbeschläge und Stockschraube"), "emissivity": 0.25, "solarAbsorptivity": 0.5, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210000000000.0, "strengthPa": 235000000.0, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "ticking-faded", "title": T("Quilted polyester-cotton ticking, sun-faded", "Gesteppter Polyester-Baumwoll-Drell, ausgeblichen"), "emissivity": 0.9, "solarAbsorptivity": 0.42, "densityKgM3": 150, "conductivityWmK": 0.045, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "iso-105-b02"},
    {"id": "pocket-spring", "title": T("Pocket-spring unit with felt and foam layers (effective)", "Taschenfederkern mit Filz- und Schaumauflagen (effektiv)"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": 70, "conductivityWmK": 0.06, "specificHeatJKgK": 1000, "elasticModulusPa": 30000.0, "strengthPa": 10000.0, "source": "authored-estimate", "reference": "catalogue-pocket-spring-mattress"},
    {"id": "webbing", "title": T("Polyester webbing handles", "Tragegriffe aus Polyestergurtband"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 300, "conductivityWmK": 0.1, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "moving-blanket", "title": T("Quilted moving blanket of recycled fibre felt", "Gesteppte Möbeldecke aus Recyclingfaservlies"), "emissivity": 0.9, "solarAbsorptivity": 0.75, "densityKgM3": 77, "conductivityWmK": 0.045, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "catalogue-moving-blanket"},
    {"id": "hardware-bag", "title": T("Polythene bag with bed bolts, screws and hook plates (effective)", "Polyethylenbeutel mit Bettbolzen, Schrauben und Beschlägen (effektiv)"), "emissivity": 0.9, "solarAbsorptivity": 0.3, "densityKgM3": 640, "conductivityWmK": 0.3, "specificHeatJKgK": 600, "source": "authored-estimate", "reference": "catalogue-pine-double-bed"},
]
DENSITY = {m["id"]: m["densityKgM3"] for m in MATERIALS}

SECTIONS = {
    "rail-l": (0.005, 1.6667e-05, 0.1, T("Side rail 25 x 200 mm, C24 pine", "Seitenholm 25 x 200 mm, Kiefer C24")),
    "rail-r": (0.005, 1.6667e-05, 0.1, T("Side rail 25 x 200 mm, C24 pine", "Seitenholm 25 x 200 mm, Kiefer C24")),
    "beam": (0.003, 1.40625e-06, 0.0375, T("Centre beam 40 x 75 mm", "Mittelholm 40 x 75 mm")),
    "leg": (0.0049, 2.0008e-06, 0.035, T("Centre leg 70 x 70 mm", "Mittelfuß 70 x 70 mm")),
    "headboard": (0.0098, 4.0017e-06, 0.035, T("Two head posts 70 x 70 mm", "Zwei Kopfpfosten 70 x 70 mm")),
    "footboard": (0.0098, 4.0017e-06, 0.035, T("Two foot posts 70 x 70 mm", "Zwei Fußpfosten 70 x 70 mm")),
    "base-l": (0.0035, 7.2917e-07, 0.025, T("Two frame battens 35 x 50 mm", "Zwei Rahmenleisten 35 x 50 mm")),
    "base-r": (0.0035, 7.2917e-07, 0.025, T("Two frame battens 35 x 50 mm", "Zwei Rahmenleisten 35 x 50 mm")),
}
MATERIAL_OF_SECTION = {"base-l": "spruce-batten", "base-r": "spruce-batten"}
MASS_REF = {"mattress-l": "catalogue-pocket-spring-mattress", "mattress-r": "catalogue-pocket-spring-mattress", "blanket": "catalogue-moving-blanket", "base-l": "catalogue-slatted-base", "base-r": "catalogue-slatted-base"}


def rig():
    masses = bone_masses()
    out = []
    for bid, parent, x, y, angle, length, joint, title in BONES:
        b = {"id": bid, "parent": parent, "title": title, "x": x, "y": y, "angleDeg": angle, "lengthM": length, "joint": joint}
        if bid in masses:
            m, cu, cv = masses[bid]
            b["mass"] = {"kg": round(m, 3), "comX": R(cu), "comY": R(cv), "distribution": "point", "source": "authored-estimate", "reference": MASS_REF.get(bid, "iso-10456-2007")}
        if bid in SECTIONS:
            a, i, c, t = SECTIONS[bid]
            b["section"] = {"areaM2": a, "secondMomentM4": i, "extremeFibreM": c, "material": MATERIAL_OF_SECTION.get(bid, "pine-aged"), "title": t}
        out.append(b)
    return {"kind": "mechanical", "root": "ground", "bones": out}


def contact(id, bone, x, y, z, title, length, width, mu, kind="contact", support="contact"):
    a = {"id": id, "bone": bone, "x": x, "y": y, "kind": kind, "title": title, "support": support, "normalDeg": 90, "footprintLengthM": length, "footprintWidthM": width, "frictionCoefficient": mu}
    if z:
        a["z"] = z
    return a


ANCHORS = [
    contact("floor-rail-l", "blanket", 1.1225, 0.0, 0.82, T("Left side rail on the blanket", "Linker Seitenholm auf der Decke"), 2.005, 0.03, 0.5),
    contact("floor-rail-r", "blanket", 1.1225, 0.0, -0.82, T("Right side rail on the blanket", "Rechter Seitenholm auf der Decke"), 2.005, 0.03, 0.5),
    contact("floor-beam", "blanket", 1.1225, 0.0, 0.0, T("Centre beam on the blanket", "Mittelholm auf der Decke"), 2.05, 0.04, 0.5),
    contact("foot-head-l", "headboard", 0.0, 0.035, 0.8, T("Left head post foot", "Linker Kopfpfostenfuß"), 0.07, 0.07, 0.4),
    contact("foot-head-r", "headboard", 0.0, 0.035, -0.8, T("Right head post foot", "Rechter Kopfpfostenfuß"), 0.07, 0.07, 0.4),
    contact("foot-foot-l", "footboard", 0.0, -0.035, 0.8, T("Left foot post foot", "Linker Fußpfostenfuß"), 0.07, 0.07, 0.4),
    contact("foot-foot-r", "footboard", 0.0, -0.035, -0.8, T("Right foot post foot", "Rechter Fußpfostenfuß"), 0.07, 0.07, 0.4),
    contact("foot-mid", "leg", 0.0, 0.0, 0.0, T("Centre leg foot", "Fuß des Mittelfußes"), 0.07, 0.07, 0.4),
    {"id": "grip-head", "bone": "mattress-l", "x": -0.8, "y": 0.0, "z": 0.808, "kind": "support", "title": T("Hands of the carrier at the head handle", "Hände der tragenden Person am Kopfgriff"), "support": "pin"},
    {"id": "grip-foot", "bone": "mattress-l", "x": 0.8, "y": 0.0, "z": 0.808, "kind": "support", "title": T("Hands of the carrier at the foot handle", "Hände der tragenden Person am Fußgriff"), "support": "pin"},
]
FLOOR = ["floor-rail-l", "floor-rail-r", "floor-beam"]
FEET = ["foot-head-l", "foot-head-r", "foot-foot-l", "foot-foot-r", "foot-mid"]
GRIPS = ["grip-head", "grip-foot"]
ASSEMBLED = {"blanket": {"dx": 2.25}, "rail-l": {"dy": 0.125}, "rail-r": {"dy": 0.125}, "beam": {"dy": 0.125}, "base-l": {"dy": 0.125}, "base-r": {"dy": 0.125}, "headboard-slide": {"dy": -0.206}, "headboard": {"angleDeg": 90}, "footboard-slide": {"dy": -0.206}, "footboard": {"angleDeg": -90}, "leg-slide": {"dy": -0.186}, "leg": {"angleDeg": 90}}
SINK = -0.015
WY = -round(75 * 9.80665 / 1.8, 1)


def binding(mat, frame, slats, blanket, contact_fraction=0.0):
    m = {"exposedFraction": mat, "sunlitFraction": 0}
    if contact_fraction:
        m["contact"] = {"fraction": contact_fraction, "tempC": 31}
    return {"regions": {"mattresses": m, "pine-frame": {"exposedFraction": frame, "sunlitFraction": 0}, "slatted-bases": {"exposedFraction": slats, "sunlitFraction": 0}, "moving-blanket": {"exposedFraction": blanket, "sunlitFraction": 0}}, "emitters": {}}


def sleeper(side):
    word = {"l": ("left", "linken"), "r": ("right", "rechten")}[side]
    return {"id": f"sleeper-{side}", "kind": "occupant", "title": T(f"Sleeper of 75 kg lying on the {word[0]} mattress (head to hips to heels over 1.80 m)", f"Schlafende Person von 75 kg auf der {word[1]} Matratze (Kopf bis Fersen über 1,80 m)"), "distributed": {"bone": f"mattress-{side}", "fromM": -0.95, "toM": 0.85, "wxNPerM": 0, "wyNPerM": WY}, "fxN": 0, "fyN": 0, "source": "authored-estimate", "reference": "en-1991-1-1"}


def pose(id, title, description, bones, contacts, thermal, loads=(), key=False):
    p = {"id": id, "title": title, "description": description}
    if key:
        p["key"] = True
    p.update({"bones": bones, "ground": "auto", "contacts": contacts, "thermal": thermal, "loads": list(loads)})
    return p


POSES = [
    pose("stored", T("Stored on a moving blanket", "Auf einer Möbeldecke gelagert"), T("Dismantled as the flat clearance delivered it: side rails on edge with their ledgers, the centre beam and the two slatted bases between them on the blanket, head- and footboard laid face down across the rails, the unscrewed centre leg on the bases and the two mattresses on top; the bag of bolts lies beside the stack.", "Zerlegt, wie die Haushaltsauflösung es brachte: Seitenholme hochkant mit ihren Auflageleisten, Mittelholm und die beiden Lattenroste dazwischen auf der Decke, Kopf- und Fußteil mit der Schauseite nach unten quer über den Holmen, der abgeschraubte Mittelfuß auf den Rosten und die beiden Matratzen obenauf; der Beutel mit den Bolzen liegt neben dem Stapel."), {}, FLOOR, binding(1.0, 0.8, 0.3, 0.4)),
    pose("lift-key", T("Mattress lifted off the stack (key)", "Matratze vom Stapel gehoben (Schlüssel)"), T("Two people have lifted the left mattress 0.12 m by its side handles.", "Zwei Personen haben die linke Matratze an den Seitengriffen 0,12 m angehoben."), {"mattress-l-lift": {"dy": 0.12}}, FLOOR + GRIPS, binding(1.0, 0.8, 0.3, 0.4), key=True),
    pose("carried", T("Mattress carried by two people", "Matratze von zwei Personen getragen"), T("The left mattress is carried flat at hand height (handles at 0.75 m) by two people at its head and foot handles, walked 0.6 m towards the door and tipped 8 degrees as the front carrier steps down; the rest of the bed stays on the blanket.", "Die linke Matratze wird flach in Handhöhe (Griffe bei 0,75 m) von zwei Personen an Kopf- und Fußgriff getragen, 0,6 m zur Tür bewegt und um 8 Grad geneigt, weil die vordere Person eine Stufe hinabgeht; der Rest des Betts bleibt auf der Decke."), {"mattress-l-carry": {"dx": 0.6}, "mattress-l-lift": {"dy": 0.374}, "mattress-l": {"angleDeg": -8}}, FLOOR + GRIPS, binding(1.0, 0.8, 0.5, 0.4)),
    pose("assembled", T("Set up again in a bedroom", "Wieder aufgebaut im Schlafzimmer"), T("Head- and footboard stand on their posts, the side rails and the centre beam hang in their hook plates at 0.13 m, the centre leg is screwed under the beam, the slatted bases lie on the ledgers and the mattresses on the bases; the moving blanket lies pulled out beyond the foot end.", "Kopf- und Fußteil stehen auf ihren Pfosten, Seitenholme und Mittelholm hängen in ihren Hakenbeschlägen bei 0,13 m, der Mittelfuß ist unter den Holm geschraubt, die Lattenroste liegen auf den Auflageleisten und die Matratzen auf den Rosten; die Möbeldecke liegt hinter dem Fußende ausgezogen."), ASSEMBLED, FEET, binding(1.0, 1.0, 0.2, 0.5)),
    pose("one-in-key", T("First sleeper lying (key)", "Erste Person liegt (Schlüssel)"), T("The left sleeper lies down; the worn left mattress gives 15 mm.", "Die linke Person legt sich hin; die durchgelegene linke Matratze gibt 15 mm nach."), dict(ASSEMBLED, **{"mattress-l-lift": {"dy": SINK}}), FEET, binding(1.0, 1.0, 0.2, 0.5, 0.075), [sleeper("l")], key=True),
    pose("in-use", T("In use with two sleepers", "In Gebrauch mit zwei Schlafenden"), T("Two adults of 75 kg lie on the two mattresses; the worn pocket springs give 15 mm and the covers warm up where the bodies lie.", "Zwei Erwachsene von 75 kg liegen auf den beiden Matratzen; die durchgelegenen Taschenfedern geben 15 mm nach und die Bezüge erwärmen sich, wo die Körper liegen."), dict(ASSEMBLED, **{"mattress-l-lift": {"dy": SINK}, "mattress-r": {"dy": SINK}}), FEET, binding(1.0, 1.0, 0.2, 0.5, 0.15), [sleeper("l"), sleeper("r")]),
]

CLIPS = [
    {"id": "carry", "title": T("Carry a mattress out", "Eine Matratze hinaustragen"), "description": T("Two people take the left mattress by its handles, lift it off the stack and walk it towards the door, the front carrier stepping down; the frame bundle stays on the blanket.", "Zwei Personen fassen die linke Matratze an den Griffen, heben sie vom Stapel und tragen sie zur Tür, die vordere Person geht eine Stufe hinab; das Rahmenbündel bleibt auf der Decke."), "durationS": 4, "fps": 25, "loop": False, "rootMotion": "none", "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1.2, "pose": "lift-key", "ease": "smooth"}, {"t": 4, "pose": "carried", "ease": "smooth"}], "contacts": [{"anchor": a, "fromS": 0, "toS": 4} for a in FLOOR] + [{"anchor": a, "fromS": 0.4, "toS": 4} for a in GRIPS], "phases": [{"id": "lift", "fromS": 0, "toS": 1.2, "title": T("Lifting off the stack", "Vom Stapel heben")}, {"id": "walk", "fromS": 1.2, "toS": 4, "title": T("Walking it out", "Hinaustragen")}], "events": [{"t": 0.4, "kind": "contact-on", "title": T("Hands take the handles", "Hände fassen die Griffe"), "target": "grip-head"}, {"t": 1.2, "kind": "motion", "title": T("Mattress clear of the stack", "Matratze frei vom Stapel")}], "tracks": []},
    {"id": "get-into-bed", "title": T("Two sleepers go to bed", "Zwei Personen gehen zu Bett"), "description": T("In the set-up bed the left and then the right sleeper lie down; each worn mattress gives 15 mm and its cover takes up body heat.", "Im aufgebauten Bett legen sich erst die linke, dann die rechte Person hin; jede durchgelegene Matratze gibt 15 mm nach und ihr Bezug nimmt Körperwärme auf."), "durationS": 6, "fps": 25, "loop": False, "rootMotion": "none", "keys": [{"t": 0, "pose": "assembled", "ease": "smooth"}, {"t": 3, "pose": "one-in-key", "ease": "smooth"}, {"t": 6, "pose": "in-use", "ease": "smooth"}], "contacts": [{"anchor": a, "fromS": 0, "toS": 6} for a in FEET], "phases": [{"id": "left-lies-down", "fromS": 0, "toS": 3, "title": T("Left sleeper lies down", "Linke Person legt sich hin")}, {"id": "right-lies-down", "fromS": 3, "toS": 6, "title": T("Right sleeper lies down", "Rechte Person legt sich hin")}], "events": [{"t": 1.6, "kind": "load-on", "title": T("Left sleeper on the mattress", "Linke Person auf der Matratze"), "target": "sleeper-l"}, {"t": 4.4, "kind": "load-on", "title": T("Right sleeper on the mattress", "Rechte Person auf der Matratze"), "target": "sleeper-r"}, {"t": 4.4, "kind": "thermal", "title": T("Both covers take up 31 degC contact heat", "Beide Bezüge nehmen 31-degC-Kontaktwärme auf")}], "tracks": [{"target": "load:sleeper-l", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 0.8, "v": 0}, {"t": 2.4, "v": 1}, {"t": 6, "v": 1}]}, {"target": "load:sleeper-r", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 3.6, "v": 0}, {"t": 5.2, "v": 1}, {"t": 6, "v": 1}]}]},
]


def layer(id, title, kind, material, thickness, k, rho, cp, source, ref):
    return {"id": id, "title": title, "kind": kind, "material": material, "thicknessM": thickness, "resistanceM2KW": round(thickness / k, 6), "heatCapacityJM2K": round(thickness * rho * cp), "source": source, "reference": ref}


AREAS = {"mattresses": 7.8, "pine-frame": 6.9, "slatted-bases": 6.0, "moving-blanket": 8.69}


def thermal_model():
    core = layer("spring-core", T("Pocket springs in felt and foam 0.17 m", "Taschenfedern in Filz und Schaum 0,17 m"), "insulation", "pocket-spring", 0.17, 0.06, 70, 1000, "authored-estimate", "catalogue-pocket-spring-mattress")
    tick = layer("ticking", T("Quilted ticking 15 mm", "Steppdrell 15 mm"), "clothing", "ticking-faded", 0.015, 0.045, 150, 1300, "authored-estimate", "iso-10456-2007")
    pine = layer("pine", T("Pine 40 mm, half thickness 20 mm", "Kiefer 40 mm, halbe Dicke 20 mm"), "timber", "pine-aged", 0.02, 0.13, 500, 1600, "literature", "iso-10456-2007")
    birch = layer("birch", T("Birch slat 8 mm", "Birkenleiste 8 mm"), "timber", "birch-slat", 0.008, 0.15, 650, 1600, "literature", "iso-10456-2007")
    felt = layer("felt", T("Fibre felt 6 mm", "Faservlies 6 mm"), "clothing", "moving-blanket", 0.006, 0.045, 77, 1300, "authored-estimate", "catalogue-moving-blanket")
    regions = [
        {"id": "mattresses", "title": T("Covers of the two mattresses", "Bezüge der beiden Matratzen"), "kind": "surface", "model": "passive", "material": "ticking-faded", "areaM2": AREAS["mattresses"], "characteristicLengthM": 0.8, "geometry": "horizontal-plate-up", "skyViewFactor": 0.7, "layers": [core, tick], "heatCapacityJM2K": core["heatCapacityJM2K"] + tick["heatCapacityJM2K"], "source": "authored-estimate", "reference": "catalogue-pocket-spring-mattress"},
        {"id": "pine-frame", "title": T("Posts, boards, rails, beam and leg", "Pfosten, Bretter, Holme, Mittelholm und Fuß"), "kind": "surface", "model": "passive", "material": "pine-aged", "areaM2": AREAS["pine-frame"], "characteristicLengthM": 0.2, "geometry": "vertical-plate", "skyViewFactor": 0.5, "layers": [pine], "heatCapacityJM2K": pine["heatCapacityJM2K"], "ratedPowerW": 0, "ratedPowerBasis": "passive", "source": "authored-estimate", "reference": "iso-10456-2007"},
        {"id": "slatted-bases", "title": T("Two slatted bases", "Zwei Lattenroste"), "kind": "surface", "model": "passive", "material": "birch-slat", "areaM2": AREAS["slatted-bases"], "characteristicLengthM": 0.04, "geometry": "horizontal-plate-up", "skyViewFactor": 0.2, "layers": [birch], "heatCapacityJM2K": birch["heatCapacityJM2K"], "source": "literature", "reference": "iso-10456-2007"},
        {"id": "moving-blanket", "title": T("Moving blanket", "Möbeldecke"), "kind": "surface", "model": "passive", "material": "moving-blanket", "areaM2": AREAS["moving-blanket"], "characteristicLengthM": 1.8, "geometry": "horizontal-plate-up", "skyViewFactor": 0.4, "layers": [felt], "heatCapacityJM2K": felt["heatCapacityJM2K"], "source": "authored-estimate", "reference": "catalogue-moving-blanket"},
    ]
    return {"environment": {"airTempC": 18, "meanRadiantTempC": 17, "relativeHumidity": 0.5, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325}, "passengers": [], "operating": [], "regions": regions, "emitters": [], "assumptions": [
        T("Reference case: an unheated store or a cool bedroom at 18 degC air and 17 degC walls, still air, no sun; the bed is a passive object with no power (0 W, stated on the frame region).", "Referenzfall: ein unbeheiztes Lager oder ein kühles Schlafzimmer bei 18 degC Luft und 17 degC Wänden, ruhige Luft, keine Sonne; das Bett ist ein passives Objekt ohne Leistung (0 W, an der Rahmenregion angegeben)."),
        T("Aged surfaces: the waxed pine has yellowed under twenty years of indoor light (solar absorptance 0.60 instead of about 0.50 when new, after the photo-yellowing of lignin, Hon 2001); the mattress ticking is sun-faded (0.42 instead of about 0.55, colour loss after ISO 105-B02); emissivity of wood and textiles stays 0.90 (ISO 10456).", "Gealterte Oberflächen: die gewachste Kiefer ist unter zwanzig Jahren Innenlicht vergilbt (solarer Absorptionsgrad 0,60 statt etwa 0,50 im Neuzustand, nach der Lichtvergilbung des Lignins, Hon 2001); der Matratzendrell ist ausgeblichen (0,42 statt etwa 0,55, Farbverlust nach ISO 105-B02); der Emissionsgrad von Holz und Textil bleibt 0,90 (ISO 10456)."),
        T("A mattress is a 0.17 m pocket-spring core (effective 0.06 W/mK, 70 kg/m3, authored-estimate for steel springs in felt and foam) inside 15 mm quilted ticking; a sleeper covers 0.6 m2 of the 3.2 m2 tops of both mattresses with a clothed body at 31 degC (ISO 7730 clothing surface), so 15 % of the cover area is in contact in use.", "Eine Matratze ist ein 0,17 m Taschenfederkern (effektiv 0,06 W/mK, 70 kg/m3, Schätzwert für Stahlfedern in Filz und Schaum) in 15 mm gestepptem Drell; eine schlafende Person bedeckt 0,6 m2 der 3,2 m2 Oberseiten beider Matratzen mit einem bekleideten Körper bei 31 degC (Bekleidungsoberfläche nach ISO 7730), im Gebrauch sind daher 15 % der Bezugsfläche in Kontakt."),
    ]}


MASS_R = {50: 0.03, 100: 0.035, 200: 0.045, 500: 0.06}


def volumes(scale, ps):
    body, thermal, structure = [], [], []
    with_layers = scale in (50, 100)
    for p in ps:
        solids = solid_of(p)
        body += solids
        for s in solids:
            if p["region"]:
                t = {"id": f"th-{s['id']}", "of": s["id"], "region": p["region"]}
                if with_layers and p["layer"]:
                    t["layer"] = p["layer"]
                thermal.append(t)
            if p["member"] or p["joint"]:
                structure.append({"id": f"st-{s['id']}", "of": s["id"], "member": p["bone"], "part": "joint" if p["joint"] else "member"})
    for side, s in (("l", 1), ("r", -1)):
        bone = f"mattress-{side}"
        if with_layers:
            thermal.append({"id": f"th-{bone}-core", "kind": "loft", "bone": bone, "material": "pocket-spring", "role": "body", "tone": 0.7, "sections": mattress_sections(scale, R(s * 0.4), shrink=0.015), "capStart": True, "capEnd": True, "region": "mattresses", "layer": "spring-core"})
        if scale != 500:
            structure.append({"id": f"st-{bone}-mass", "kind": "ellipsoid", "bone": bone, "material": "pocket-spring", "role": "detail", "tone": 0.3, "radii": [MASS_R[scale]] * 3, "transform": {"translation": [0.0, 0.0, R(s * 0.4)]}, "member": bone, "part": "mass"})
    return {"status": "authored", "denominator": scale, "body": body, "thermal": thermal, "structure": structure, "decisions": VOL_DECISIONS[scale]}

OVERLAYS = T("Posts, boards, rails, beam and leg = pine-frame; frames, holders and slats of the bases = slatted-bases; covers and handles = mattresses; blanket = moving-blanket; hook plates, hanger bolt and the bag of bolts carry no region.", "Pfosten, Bretter, Holme, Mittelholm und Fuß = pine-frame; Rahmen, Träger und Leisten der Roste = slatted-bases; Bezüge und Griffe = mattresses; Decke = moving-blanket; Hakenbeschläge, Stockschraube und Beutel tragen keine Region.")
REP_DECISIONS = {
    50: {
        "silhouette": T("True orthographic views of the 1:50 model of the stored stack, 48 x 9.5 mm in the side elevation: the moving blanket, the near side rail with its chipped top edge, head- and footboard laid face down on the rails with the unscrewed centre leg between them, and the two pocket-spring mattresses with their lying hollow and side handles on top. Front elevation from the foot end, 36 mm wide: blanket, rails with ledgers and hook plates, the ends of the slatted bases with holders and slats, the beam, the lying footboard and the mattress ends. Plan: blanket, the bag of bolts, the rail edges and post ends beside the mattresses.", "Wahre orthogonale Ansichten des Modells 1:50 des gelagerten Stapels, 48 x 9,5 mm in der Seitenansicht: die Möbeldecke, der vordere Seitenholm mit der abgeplatzten Oberkante, Kopf- und Fußteil mit der Schauseite nach unten auf den Holmen, dazwischen der abgeschraubte Mittelfuß, und obenauf die beiden Taschenfederkernmatratzen mit Liegekuhle und Seitengriffen. Vorderansicht vom Fußende, 36 mm breit: Decke, Holme mit Auflageleisten und Hakenbeschlägen, die Enden der Lattenroste mit Trägern und Leisten, der Mittelholm, das liegende Fußteil und die Matratzenenden. Draufsicht: Decke, der Beutel mit den Bolzen, die Holmkanten und Pfostenenden neben den Matratzen."),
        "retained": T("The 18 mm lying hollow of both mattresses, the 8 mm chip in the right rail, the chipped feet of the right head and foot posts, post chamfers, rounded top rails, all 28 slats of each base in their holders with the one replaced, paler slat, piping seams, quilting lines, the water stain and the price tag, scratches and an old tape label.", "Die 18 mm tiefe Liegekuhle beider Matratzen, die 8 mm Abplatzung im rechten Holm, die bestoßenen Füße des rechten Kopf- und Fußpfostens, Pfostenfasen, gerundete Abdeckleisten, alle 28 Leisten jedes Rosts in ihren Trägern mit der einen ersetzten, helleren Leiste, Kedernähte, Steppnähte, der Wasserfleck und das Preisschild, Kratzer und ein altes Klebeband-Etikett."),
        "merged": T("The far rail, boards and mattress coincide with the near ones in the side elevation; the slats of a base are one comb outline in the side elevation and one row in plan.", "Hinterer Holm, Bretter und Matratze decken sich in der Seitenansicht mit den vorderen; die Leisten eines Rosts sind in der Seitenansicht ein Kammumriss und in der Draufsicht eine Reihe."),
        "omitted": T("Screws and dowels, the bed bolts inside the bag, the inner handles pressed flat between the mattresses, the zips of the covers.", "Schrauben und Dübel, die Bettbolzen im Beutel, die zwischen den Matratzen flachgedrückten inneren Griffe, die Reißverschlüsse der Bezüge."),
        "thinElements": T("Nothing is thickened: the 6 mm blanket, the 3 mm hook plates, 8 mm slats and 20 mm panels are filled shapes at their true size; seams, quilting, scratches, tape and tag are strokes.", "Nichts ist verdickt: die 6 mm Decke, die 3 mm Hakenbeschläge, 8 mm Leisten und 20 mm Füllungen sind gefüllte Formen in wahrer Größe; Nähte, Steppung, Kratzer, Klebeband und Etikett sind Striche."),
        "hatching": T("Grain lines at 0.6 mm, pen 0.13, along the side rails and the beam, which never turn.", "Maserungslinien im Abstand 0,6 mm, Stift 0,13, längs der Seitenholme und des Mittelholms, die sich nie drehen."),
        "styles": T("Mattresses 0.85, replaced slat 0.85, bag 0.8, slats 0.62, panels 0.6, rails 0.55, posts and beam 0.5, blanket and steel 0.3.", "Matratzen 0,85, ersetzte Leiste 0,85, Beutel 0,8, Leisten 0,62, Füllungen 0,6, Holme 0,55, Pfosten und Mittelholm 0,5, Decke und Stahl 0,3."),
        "overlays": OVERLAYS,
    },
    100: {
        "silhouette": T("True orthographic views of the 1:100 model, 24 x 4.8 mm: blanket, side rail, the two lying boards with posts, end rail, panel and top rail, the leg between them and the two mattresses with their hollow. Front elevation: rails as L sections, base frames with their slat decks, beam, footboard and mattress ends. Plan: blanket, bag, rails, post ends and mattresses.", "Wahre orthogonale Ansichten des Modells 1:100, 24 x 4,8 mm: Decke, Seitenholm, die beiden liegenden Teile mit Pfosten, Querriegel, Füllung und Abdeckleiste, dazwischen der Fuß, und die beiden Matratzen mit ihrer Kuhle. Vorderansicht: Holme als L-Querschnitte, Rostrahmen mit ihren Leistendecks, Mittelholm, Fußteil und Matratzenenden. Draufsicht: Decke, Beutel, Holme, Pfostenenden und Matratzen."),
        "retained": T("The 18 mm lying hollow, the frames of both bases, end rails, panels and top rails of the boards, piping seams and the water stain.", "Die 18 mm Liegekuhle, die Rahmen beider Roste, Querriegel, Füllungen und Abdeckleisten der Teile, Kedernähte und der Wasserfleck."),
        "merged": T("Rail and ledger are one L section; slats and holders of a base one 20 mm deck; the two posts of a board one pair of equal posts.", "Holm und Auflageleiste sind ein L-Querschnitt; Leisten und Träger eines Rosts ein 20 mm Deck; die beiden Pfosten eines Teils ein Paar gleicher Pfosten."),
        "omitted": T("Hook plates, chamfers and chips, handles, hanger bolt, quilting lines, scratches, tape and price tag (below 15 mm).", "Hakenbeschläge, Fasen und Abplatzungen, Griffe, Stockschraube, Steppnähte, Kratzer, Klebeband und Preisschild (unter 15 mm)."),
        "thinElements": T("Nothing is thickened: the 6 mm blanket, 20 mm panels and 25 mm rails are filled shapes at their true size.", "Nichts ist verdickt: die 6 mm Decke, 20 mm Füllungen und 25 mm Holme sind gefüllte Formen in wahrer Größe."),
        "hatching": T("None.", "Keine."),
        "styles": T("Mattresses 0.85, bag 0.8, slat decks 0.62, panels 0.6, rails 0.55, posts and beam 0.5, blanket 0.3.", "Matratzen 0,85, Beutel 0,8, Leistendecks 0,62, Füllungen 0,6, Holme 0,55, Pfosten und Mittelholm 0,5, Decke 0,3."),
        "overlays": OVERLAYS,
    },
    200: {
        "silhouette": T("True orthographic views of the 1:200 model, 12 x 2.4 mm: blanket, side rail, the two lying boards as posts and one board each, the leg between them and the two mattresses as rounded blocks. Front elevation: rails, the two base slabs, beam, footboard and mattress ends. Plan: blanket, bag, rails, post ends and mattresses.", "Wahre orthogonale Ansichten des Modells 1:200, 12 x 2,4 mm: Decke, Seitenholm, die beiden liegenden Teile als Pfosten und je ein Brett, dazwischen der Fuß, und die beiden Matratzen als gerundete Blöcke. Vorderansicht: Holme, die beiden Rostplatten, Mittelholm, Fußteil und Matratzenenden. Draufsicht: Decke, Beutel, Holme, Pfostenenden und Matratzen."),
        "retained": T("The heights of the stack (rails 0.2 m, boards 0.27 m, mattress top 0.48 m), the open slot between the two lying boards with the leg, the rounded mattress edges, the bag beside the stack.", "Die Höhen des Stapels (Holme 0,2 m, Teile 0,27 m, Matratzenoberkante 0,48 m), der offene Schlitz zwischen den beiden liegenden Teilen mit dem Fuß, die gerundeten Matratzenkanten, der Beutel neben dem Stapel."),
        "merged": T("Rail and ledger one 54 mm board; each slatted base one 70 mm slab; end rail, panel and top rail of each board one 40 mm board.", "Holm und Auflageleiste ein 54 mm Brett; jeder Lattenrost eine 70 mm Platte; Querriegel, Füllung und Abdeckleiste jedes Teils ein 40 mm Brett."),
        "omitted": T("The 18 mm lying hollow, seams, stain, handles and fittings (below 30 mm).", "Die 18 mm Liegekuhle, Nähte, Fleck, Griffe und Beschläge (unter 30 mm)."),
        "thinElements": T("Nothing is thickened: the 6 mm blanket, 40 mm boards and 54 mm rails are filled shapes at their true size.", "Nichts ist verdickt: die 6 mm Decke, 40 mm Bretter und 54 mm Holme sind gefüllte Formen in wahrer Größe."),
        "hatching": T("None.", "Keine."),
        "styles": T("Mattresses 0.85, bag 0.8, base slabs 0.6, boards and rails 0.55, posts and beam 0.5, blanket 0.3.", "Matratzen 0,85, Beutel 0,8, Rostplatten 0,6, Bretter und Holme 0,55, Pfosten und Mittelholm 0,5, Decke 0,3."),
        "overlays": OVERLAYS,
    },
    500: {
        "silhouette": T("True orthographic views of the 1:500 model, 4.8 x 1 mm: the stack as blanket, rails, the two lying boards, the leg and the two mattresses as plain blocks. Front elevation and plan the same masses.", "Wahre orthogonale Ansichten des Modells 1:500, 4,8 x 1 mm: der Stapel als Decke, Holme, die beiden liegenden Teile, der Fuß und die beiden Matratzen als einfache Blöcke. Vorderansicht und Draufsicht dieselben Massen."),
        "retained": T("Overall stack 2.40 x 1.80 x 0.48 m, the two mattress blocks of 0.80 m, the boards that turn up into head- and footboard when the bed is set up.", "Gesamtstapel 2,40 x 1,80 x 0,48 m, die beiden Matratzenblöcke von 0,80 m, die Teile, die sich beim Aufbau zu Kopf- und Fußteil aufrichten."),
        "merged": T("As at 1:200: rail and ledger one board, each base one slab, each board one panel on its posts; the mattresses are plain blocks.", "Wie bei 1:200: Holm und Auflageleiste ein Brett, jeder Rost eine Platte, jedes Teil eine Platte auf seinen Pfosten; die Matratzen sind einfache Blöcke."),
        "omitted": T("The bag of bolts (40 mm high) and the rounded mattress edges (below 75 mm).", "Der Beutel mit den Bolzen (40 mm hoch) und die gerundeten Matratzenkanten (unter 75 mm)."),
        "thinElements": T("Nothing is thickened: blanket 6 mm, rails 54 mm, boards and beam 40 mm, leg and posts 70 mm are filled shapes at their true size and read as the pen line.", "Nichts ist verdickt: Decke 6 mm, Holme 54 mm, Bretter und Mittelholm 40 mm, Fuß und Pfosten 70 mm sind gefüllte Formen in wahrer Größe und lesen sich als Stiftlinie."),
        "hatching": T("None.", "Keine."),
        "styles": T("Mattresses 0.85, base slabs 0.6, boards and rails 0.55, posts, leg and beam 0.5, blanket 0.3.", "Matratzen 0,85, Rostplatten 0,6, Bretter und Holme 0,55, Pfosten, Fuß und Mittelholm 0,5, Decke 0,3."),
        "overlays": OVERLAYS,
    },
}
VOL_DECISIONS = {
    50: [T("Body, authored for 7.5 mm: the 2.40 x 1.80 m moving blanket as a 6 mm sheet with the bag of bolts beside the stack; two 25 x 200 mm side rails with 24 x 44 mm ledgers and four 3 mm hook plates each, the right rail with an 8 mm chip; the 40 x 75 mm centre beam; two framed slatted bases with 35 x 50 mm battens, 56 polypropylene holders and 28 birch slats 38 x 8 mm each, one slat of the right base replaced; head- and footboard as two chamfered 70 mm posts (right feet chipped), a 25 mm end rail, a 20 mm panel and a rounded 40 mm top rail; the 70 mm centre leg with its M8 hanger bolt; two 0.80 x 2.00 x 0.20 m mattresses as lofts with rounded piped edges and an 18 mm lying hollow at the hips, and two webbing handles on each outer side.", "Körper, für 7,5 mm gebaut: die Möbeldecke 2,40 x 1,80 m als 6 mm Bahn mit dem Beutel neben dem Stapel; zwei Seitenholme 25 x 200 mm mit Auflageleisten 24 x 44 mm und je vier 3 mm Hakenbeschlägen, der rechte Holm mit 8 mm Abplatzung; der Mittelholm 40 x 75 mm; zwei Rahmenlattenroste mit Leisten 35 x 50 mm, 56 Leistenträgern aus Polypropylen und je 28 Birkenleisten 38 x 8 mm, eine Leiste des rechten Rosts ersetzt; Kopf- und Fußteil aus zwei gefasten 70 mm Pfosten (rechte Füße bestoßen), 25 mm Querriegel, 20 mm Füllung und gerundeter 40 mm Abdeckleiste; der 70 mm Mittelfuß mit M8-Stockschraube; zwei Matratzen 0,80 x 2,00 x 0,20 m als Lofts mit gerundeten Kederkanten und 18 mm Liegekuhle an der Hüfte sowie je zwei Gurtgriffen an der Außenseite."),
         T("Thermal: the covers are the body lofts (layer ticking) around the spring cores 15 mm inside (layer spring-core); frame, bases and blanket reference their body solids; steel and the bag carry no region. Structure: every load-bearing part is its own member along its bone (rails, ledgers, beam, leg, posts, end and top rails, battens, slats, blanket), the hook plates, slat holders and hanger bolt are joints, each mattress body is its own member (the spring unit carries and spreads the sleeper) with a mass marker at its centre; there are no fixed supports, the contacts of each pose carry the bed.", "Thermik: die Bezüge sind die Körper-Lofts (Schicht ticking) um die 15 mm innen liegenden Federkerne (Schicht spring-core); Rahmen, Roste und Decke verweisen auf ihre Körper; Stahl und Beutel tragen keine Region. Struktur: jedes tragende Teil ist ein eigener Stab längs seines Knochens (Holme, Auflageleisten, Mittelholm, Fuß, Pfosten, Quer- und Abdeckleisten, Rahmenleisten, Leisten, Decke), Hakenbeschläge, Leistenträger und Stockschraube sind Gelenke, jeder Matratzenkörper ist sein eigener Stab (der Federkern trägt und verteilt die Last der Person) mit einer Massenmarke in seiner Mitte; feste Auflager gibt es keine, die Kontakte jeder Pose tragen das Bett.")],
    100: [T("Body, authored anew for 15 mm: rail and ledger as one L-section extrusion along the rail; base frames of 35 x 50 mm battens with one 20 mm slat deck instead of slats and holders; the boards with plain 70 mm posts, end rail, panel and top rail; the leg without its bolt; the mattresses as lofts with fewer sections, keeping the 18 mm hollow, without handles; blanket and bag as at 1:50.", "Körper, für 15 mm neu gebaut: Holm und Auflageleiste als eine L-Profil-Extrusion längs des Holms; Rostrahmen aus Leisten 35 x 50 mm mit einem 20 mm Leistendeck statt Leisten und Trägern; die Teile mit glatten 70 mm Pfosten, Querriegel, Füllung und Abdeckleiste; der Fuß ohne Bolzen; die Matratzen als Lofts mit weniger Schnitten, mit der 18 mm Kuhle, ohne Griffe; Decke und Beutel wie bei 1:50."),
         T("Thermal: covers around spring cores 15 mm inside, the rest by reference. Structure: members for rails, beam, leg, posts, end and top rails, battens, decks and blanket; each mattress body as its member with a mass marker; no joints are shown below 15 mm.", "Thermik: Bezüge um 15 mm innen liegende Federkerne, der Rest per Verweis. Struktur: Stäbe für Holme, Mittelholm, Fuß, Pfosten, Quer- und Abdeckleisten, Rahmenleisten, Decks und Decke; jeder Matratzenkörper als Stab mit Massenmarke; Gelenke unter 15 mm entfallen.")],
    200: [T("Body, authored anew for 30 mm: rail and ledger as one 54 mm board, each slatted base one 70 mm slab, each end of the bed two posts and one 40 mm board from the end rail to the top, the leg, the mattresses as 0.80 x 2.00 x 0.20 m blocks with 40 mm rounded edges in the side view (the 18 mm hollow is below the feature), blanket and bag.", "Körper, für 30 mm neu gebaut: Holm und Auflageleiste als ein 54 mm Brett, jeder Lattenrost eine 70 mm Platte, jedes Bettende zwei Pfosten und ein 40 mm Brett vom Querriegel bis oben, der Fuß, die Matratzen als Blöcke 0,80 x 2,00 x 0,20 m mit 40 mm gerundeten Kanten in der Seitenansicht (die 18 mm Kuhle liegt unter dem Merkmal), Decke und Beutel."),
         T("Thermal by reference without layers (merged into one surface per region). Structure: members for rails, beam, leg, posts, boards, slabs and blanket; each mattress block as its member with a mass marker.", "Thermik per Verweis ohne Schichten (je Region eine Oberfläche). Struktur: Stäbe für Holme, Mittelholm, Fuß, Pfosten, Bretter, Platten und Decke; jeder Matratzenblock als Stab mit Massenmarke.")],
    500: [T("Body, authored anew for 75 mm: blanket, the two 54 mm rails, the beam, the two base slabs, posts and one board for each end, the leg and the mattresses as plain blocks; the bag (40 mm) is left out. Parts thinner than 75 mm keep their true size because the 1:500 drawing draws them so. The model is minimal: nothing more can be merged without losing the parts that become the assembled bed.", "Körper, für 75 mm neu gebaut: Decke, die beiden 54 mm Holme, der Mittelholm, die beiden Rostplatten, Pfosten und ein Brett je Ende, der Fuß und die Matratzen als einfache Blöcke; der Beutel (40 mm) entfällt. Teile unter 75 mm behalten ihre wahre Größe, weil die Zeichnung 1:500 sie so zeichnet. Das Modell ist minimal: mehr lässt sich nicht zusammenfassen, ohne die Teile zu verlieren, aus denen das aufgebaute Bett wird."),
         T("Thermal by reference without layers. Structure: members for rails, beam, leg, posts, boards, slabs and blanket; each mattress block as its member; mass markers are left out at this scale.", "Thermik per Verweis ohne Schichten. Struktur: Stäbe für Holme, Mittelholm, Fuß, Pfosten, Bretter, Platten und Decke; jeder Matratzenblock als Stab; Massenmarken entfallen bei diesem Maßstab.")],
}

REFERENCES = [
    ("en-1725-1998", "EN 1725:1998 Domestic furniture - Beds and mattresses - Safety requirements and test methods"),
    ("en-1334-1996", "EN 1334:1996 Domestic furniture - Beds and mattresses - Methods of measurement and recommended tolerances"),
    ("en-1957-2012", "EN 1957:2012 Furniture - Beds and mattresses - Test methods for the determination of functional characteristics and assessment criteria"),
    ("en-1991-1-1", "EN 1991-1-1:2002 Eurocode 1: Actions on structures - Part 1-1: General actions - Densities, self-weight, imposed loads for buildings"),
    ("en-338-2016", "EN 338:2016 Structural timber - Strength classes"),
    ("iso-10456-2007", "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values and procedures for determining declared and design thermal values"),
    ("iso-7730-2005", "ISO 7730:2005 Ergonomics of the thermal environment - Analytical determination and interpretation of thermal comfort"),
    ("iso-105-b02", "ISO 105-B02:2014 Textiles - Tests for colour fastness - Part B02: Colour fastness to artificial light: Xenon arc fading lamp test"),
    ("hon-2001-photodegradation", "Hon, D. N.-S. (2001) Weathering and photochemistry of wood. In: Hon, D. N.-S., Shiraishi, N. (eds.) Wood and Cellulosic Chemistry, 2nd ed. Marcel Dekker, New York, 513-546"),
    ("din-18011", "DIN 18011 Stellflächen, Abstände und Bewegungsflächen im Wohnungsbau (footprint of a double bed and the access space beside it)"),
    ("catalogue-pine-double-bed", "Catalogue-typical data of flat-pack solid-pine double beds 160 x 200 cm of the 1995-2015 mass market: outer 167-170 x 208-215 cm, headboard 0.85-0.95 m, footboard 0.30-0.45 m, side rails 20-25 x 180-200 mm on hook plates, centre beam on one leg, 38-55 kg; authored from retail catalogues, no brand"),
    ("catalogue-slatted-base", "Catalogue-typical framed slatted bases 80 x 200 cm: 28 laminated birch slats 38 x 8 mm in polypropylene holders on a 35 x 50 mm frame, 7-10 kg; authored from retail catalogues, no brand"),
    ("catalogue-pocket-spring-mattress", "Catalogue-typical 7-zone pocket-spring mattresses 80 x 200 x 20 cm (about 400 springs, 20-25 kg, sizes after EN 1334); authored from retail catalogues, no brand"),
    ("catalogue-moving-blanket", "Catalogue-typical quilted moving blankets of recycled fibre felt, 180 x 200 to 180 x 240 cm, 0.4-0.5 kg/m2; authored from removal-supply catalogues, no brand"),
]


def asset():
    reps, vols = {}, {}
    for scale in (500, 200, 100, 50):
        rep, ps = representation(scale)
        reps[str(scale)] = rep
        vols[str(scale)] = volumes(scale, ps)
    masses = bone_masses()
    total = round(sum(m for m, _, _ in masses.values()), 1)
    return {
        "id": "used-double-bed",
        "category": "furniture",
        "title": T("Used Double Bed with Mattress", "Gebrauchtes Doppelbett mit Matratze"),
        "summary": T("A used solid-pine double bed 160 x 200 cm with two framed slatted bases and two worn pocket-spring mattresses, stored dismantled on a moving blanket as a second-hand store takes it in from a flat clearance: rails on edge, head- and footboard laid across, the unscrewed centre leg and a bag of bolts; one mattress is carried out by two people, then the bed is set up again for two sleepers.", "Ein gebrauchtes Doppelbett aus massiver Kiefer, 160 x 200 cm, mit zwei Rahmenlattenrosten und zwei durchgelegenen Taschenfederkernmatratzen, zerlegt auf einer Möbeldecke gelagert, wie ein Gebrauchtwarenkaufhaus es aus einer Haushaltsauflösung annimmt: Holme hochkant, Kopf- und Fußteil quer darüber, der abgeschraubte Mittelfuß und ein Beutel mit Bolzen; eine Matratze wird von zwei Personen hinausgetragen, dann wird das Bett für zwei Schlafende wieder aufgebaut."),
        "tags": ["used", "household", "second-hand", "bedroom", "furniture", "bed", "double-bed", "mattress", "pocket-spring-mattress", "slatted-base", "pine", "solid-wood", "dismantled", "moving-blanket", "flat-clearance"],
        "provenance": {"author": "Semio staffage content (used household objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": i, "citation": c} for i, c in REFERENCES], "notes": T("Every dimension, section, density and mass is catalogue-typical or from the cited standards and tagged literature or authored-estimate; nothing is measured. The traces of use (lying hollow, chips, scuffs, replaced slat, faded and stained covers, tape label, price tag) are authored as typical of a bed used for about twenty years.", "Alle Maße, Querschnitte, Dichten und Massen sind katalogtypisch oder aus den genannten Normen und als Literaturwert oder Schätzwert gekennzeichnet; nichts ist gemessen. Die Gebrauchsspuren (Liegekuhle, Abplatzungen, Stoßstellen, ersetzte Leiste, ausgeblichene und fleckige Bezüge, Klebeband-Etikett, Preisschild) sind als typisch für ein etwa zwanzig Jahre benutztes Bett gesetzt.")},
        "dimensions": {"heightM": 0.476, "widthM": 2.4, "depthM": 1.8, "massKg": total},
        "materials": MATERIALS,
        "rig": rig(),
        "volumes": {k: vols[k] for k in ("50", "100", "200", "500")},
        "anchors": ANCHORS,
        "representations": {k: reps[k] for k in ("500", "200", "100", "50")},
        "symbols": [],
        "poses": POSES,
        "clips": CLIPS,
        "thermal": thermal_model(),
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": [], "assumptions": [
            T("The product treats the asset as one rigid body on the contacts of the pose: stored, the stack stands through the blanket on the floor under the two rails and the beam; carried, the two carriers' hands at the mattress handles are pin supports beside those floor contacts; set up, the bed stands on its four post feet and the centre leg (EN 1725 static and durability tests of bed bases are the scope; no test load is modelled).", "Das Produkt behandelt das Asset als einen starren Körper auf den Kontakten der Pose: gelagert steht der Stapel über die Decke unter den beiden Holmen und dem Mittelholm auf dem Boden; getragen sind die Hände der beiden Tragenden an den Matratzengriffen Gelenklager neben diesen Bodenkontakten; aufgebaut steht das Bett auf seinen vier Pfostenfüßen und dem Mittelfuß (Rahmen: statische und Dauerprüfungen von Bettunterbauten nach EN 1725; keine Prüflast modelliert)."),
            T("Members: side rails 25 x 200 mm and centre beam 40 x 75 mm of C24-like pine (EN 338: f_m,k 24 MPa, E_0,mean 11 GPa), posts 70 x 70 mm, slatted-base battens 35 x 50 mm spruce; each sleeper is 75 kg spread over 1.80 m of the mattress (EN 1991-1-1 imposed load as a person, 409 N/m); a worn mattress gives 15 mm under a sleeper and keeps an 18 mm lying hollow unloaded (height and firmness loss after EN 1957 durability cycling, authored-estimate).", "Stäbe: Seitenholme 25 x 200 mm und Mittelholm 40 x 75 mm aus Kiefer etwa C24 (EN 338: f_m,k 24 MPa, E_0,mean 11 GPa), Pfosten 70 x 70 mm, Rostrahmenleisten 35 x 50 mm Fichte; jede schlafende Person ist 75 kg auf 1,80 m der Matratze (Nutzlast als Person nach EN 1991-1-1, 409 N/m); eine durchgelegene Matratze gibt unter einer Person 15 mm nach und behält unbelastet eine 18 mm Liegekuhle (Höhen- und Härteverlust nach EN-1957-Dauerprüfung, Schätzwert)."),
            T("Masses are section x length x density of every part of the 1:50 model (pine 500, spruce 450, birch 650 kg/m3 after ISO 10456; a mattress 22 kg, a slatted base 9 kg, the frame 42 kg, catalogue-typical), placed at their centres on their bones.", "Massen sind Querschnitt x Länge x Dichte jedes Teils des Modells 1:50 (Kiefer 500, Fichte 450, Birke 650 kg/m3 nach ISO 10456; eine Matratze 22 kg, ein Lattenrost 9 kg, der Rahmen 42 kg, katalogtypisch), in ihren Schwerpunkten an ihren Knochen."),
        ], "clinical": False},
        "presentation": {"defaultPose": "stored", "defaultClip": "carry", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }


if __name__ == "__main__":
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset()}
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
        f.write("\n")
    a = doc["asset"]
    print("[DEBUG] mass", a["dimensions"]["massKg"], {b["id"]: b.get("mass", {}).get("kg") for b in a["rig"]["bones"]})
    print("[DEBUG] solids", {k: (len(v["body"]), len(v["thermal"]), len(v["structure"])) for k, v in a["volumes"].items()})
