"""🚛 Round 3 kit of `machinery/concrete-mixer-truck`: the reference dimensions of the real 8x4 truck mixer (one table of numbers that the 2D drawings and the solids of every scale both read), the rest frames of the corrected rig, and small writers for solids (through author 36's `Volume`) and 2D shapes."""

import glob
import importlib.util
import json
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(T, "🗑️generated", "r3-concrete-mixer-truck")
CATALOG = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/concrete-mixer-truck/*.json")[0]
SOURCE = os.path.join(T, "volumes", "machinery", "concrete-mixer-truck.json")
CANDIDATE = os.path.join(OUT, "candidate", "🔣️.json")

_spec = importlib.util.spec_from_file_location("lib36", os.path.join(T, "author-36-lib.py"))
lib = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(lib)

sin, cos, pi, rad = math.sin, math.cos, math.pi, math.radians

LEN = 9.0
FRONT = 4.5
REAR = -4.5
HEIGHT = 3.65
WIDTH = 2.55
AX = [3.025, 1.23, -1.275, -2.675]
WR = 0.52
TW = 0.315
ZF = 1.02
ZI, ZO = 0.745, 1.095
RAIL_Z = 0.34
RAIL_Y0, RAIL_Y1 = 0.81, 1.08
RAIL_X0, RAIL_X1 = -3.75, 4.10
SUB_Y1 = 1.16
SUB_X0, SUB_X1 = -4.25, 2.45
CAB_R = 2.58
CAB_HW = 1.14
ARCH_R = 0.66
DRUM_O = (1.80, 2.04)
DRUM_A = 169.0
DRUM_L = 5.4
DRUM = [(0.0, 0.30), (0.03, 0.62), (0.09, 0.76), (0.18, 0.83), (1.10, 1.15), (1.90, 1.15), (5.40, 0.56)]
RING_S0, RING_S1, RING_R = 4.20, 4.32, 0.80
RING_IN = 0.72
ROLLER = 0.91
LIP_R = 0.595
SW = (-3.95, 1.95)
CHUTE_A = 202.0
CHUTE_L, CHUTE_HW, CHUTE_D = 1.25, 0.26, 0.20
EXT_L, EXT_HW, EXT_D = 0.85, 0.24, 0.18
STACK = (2.46, -0.95)
INTAKE = (2.46, 0.95)
PITCH = 0.85


def rot(deg):
    a = rad(deg)
    return [cos(a), sin(a), 0.0], [-sin(a), cos(a), 0.0], [0.0, 0.0, 1.0]


def frame(o, deg):
    u, v, w = rot(deg)
    return {"o": [o[0], o[1], 0.0], "u": u, "v": v, "w": w}


CU, CV, _ = rot(CHUTE_A)
EXT_O = (SW[0] + CHUTE_L * CU[0], SW[1] + CHUTE_L * CU[1])
FRAMES = {
    "chassis": frame((0, 0), 0),
    "drum": frame(DRUM_O, DRUM_A),
    "drum-spiral": frame(DRUM_O, DRUM_A),
    "chute-swivel": frame(SW, 90),
    "chute": frame(SW, CHUTE_A),
    "chute-ext": frame(EXT_O, CHUTE_A + 180),
    "wheel-1": frame((AX[0], WR), 0),
    "wheel-2": frame((AX[1], WR), 0),
    "wheel-3": frame((AX[2], WR), 0),
    "wheel-4": frame((AX[3], WR), 0),
}
DU, DV = FRAMES["drum"]["u"], FRAMES["drum"]["v"]


def drum_r(s):
    for (a0, r0), (a1, r1) in zip(DRUM, DRUM[1:]):
        if a0 <= s <= a1:
            return r0 + (r1 - r0) * (s - a0) / (a1 - a0)
    return DRUM[-1][1]


def dw(s, v=0.0, w=0.0):
    """World point of the drum-local point (s along the axis, v towards the ground side, w lateral)."""
    return [DRUM_O[0] + s * DU[0] + v * DV[0], DRUM_O[1] + s * DU[1] + v * DV[1], w]


def arc(cx, cy, r, a0, a1, n):
    return [(cx + r * cos(rad(a0 + (a1 - a0) * i / n)), cy + r * sin(rad(a0 + (a1 - a0) * i / n))) for i in range(n + 1)]


def superellipse(cx, cz, rx, rz, e, n):
    pts = []
    for i in range(n):
        t = 2 * pi * i / n
        c, s = cos(t), sin(t)
        pts.append((cx + rx * math.copysign(abs(c) ** (2 / e), c), cz + rz * math.copysign(abs(s) ** (2 / e), s)))
    return pts


class Kit(lib.Volume):
    def __init__(self):
        self.slug = "concrete-mixer-truck"
        self.frames = FRAMES
        self.lists = {"body": [], "thermal": [], "structure": []}
        self.cur = "body"
        self.decisions = []
        self.all = {}
        self.reg = {}
        self.scale = None

    def extl(self, id, bone, plane, pts, lo, hi, holes=None, **m):
        d = self._meta(id, bone, "extrude", m)
        d["profile"] = [lib.rv(p) for p in pts]
        if holes:
            d["holes"] = [[lib.rv(p) for p in h] for h in holes]
        d["plane"] = plane
        d["from"], d["to"] = lib.r5(lo), lib.r5(hi)
        return self.put(d, {})

    def revl(self, id, bone, origin, axis, profile, sweep=None, rep=None, **m):
        d = self._meta(id, bone, "revolve", m)
        d["profile"] = [lib.rv(p) for p in profile]
        d["axis"] = lib.rv(axis)
        d["origin"] = lib.rv(origin)
        if sweep is not None:
            d["sweepRad"] = lib.r5(sweep)
        if rep:
            d["repeat"] = rep
        return self.put(d, {})

    def boxl(self, id, bone, c, size, cr=None, **m):
        d = self._meta(id, bone, "box", m)
        d["size"] = lib.rv(size)
        if cr:
            d["cornerRadius"] = lib.r5(cr)
        d["transform"] = {"translation": lib.rv(c)}
        return self.put(d, {})

    def struct(self, part, member, **kw):
        return dict(member=member, part=part, **kw)


def r4(x):
    v = round(x, 4)
    return 0.0 if v == 0 else v


def local(bone, pts):
    f = FRAMES[bone]
    out = []
    for x, y in pts:
        dx, dy = x - f["o"][0], y - f["o"][1]
        out.append([r4(dx * f["u"][0] + dy * f["u"][1]), r4(dx * f["v"][0] + dy * f["v"][1])])
    return out


def shape(kind, id, bone, role, z, tone, region, material, filled=True, **geo):
    d = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": kind}
    d.update(geo)
    if isinstance(region, (list, tuple)):
        d["regions"] = list(region)
    elif region:
        d["region"] = region
    d["material"] = material
    return d


def poly(id, bone, role, z, tone, region, material, pts, filled=True, closed=True, world=True, **extra):
    p = local(bone, pts) if world else [[r4(a), r4(b)] for a, b in pts]
    return shape("poly", id, bone, role, z, tone, region, material, filled, points=p, closed=closed, **extra)


def rect(id, bone, role, z, tone, region, material, x, y, w, h, filled=True, **extra):
    return shape("rect", id, bone, role, z, tone, region, material, filled, x=r4(x), y=r4(y), w=r4(w), h=r4(h), **extra)


def ellipse(id, bone, role, z, tone, region, material, cx, cy, rx, ry, filled=True, **extra):
    return shape("ellipse", id, bone, role, z, tone, region, material, filled, cx=r4(cx), cy=r4(cy), rx=r4(rx), ry=r4(ry), **extra)


def scalar(v):
    return not isinstance(v, (dict, list))


def inline(v):
    if isinstance(v, list):
        return all(scalar(i) for i in v) or (all(isinstance(i, list) and all(scalar(x) for x in i) for i in v) and len(v) > 0)
    return False


def dump(v, depth):
    """The catalog file's own style: one space per level, number lists and point lists on one line."""
    pad, inner = " " * depth, " " * (depth + 1)
    if isinstance(v, dict):
        if not v:
            return "{}"
        return "{\n" + ",\n".join(inner + json.dumps(k, ensure_ascii=False) + ": " + dump(i, depth + 1) for k, i in v.items()) + "\n" + pad + "}"
    if isinstance(v, list):
        if not v:
            return "[]"
        if inline(v) and not all(isinstance(i, str) for i in v):
            return json.dumps(v, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(i, depth + 1) for i in v) + "\n" + pad + "]"
    return json.dumps(v, ensure_ascii=False)


RUNNING = ("axle-", "differential", "spring-", "bogie-", "torque-rod-", "damper-", "prop-shaft", "rim-", "wheel-disc-", "wheel-dish-", "fender-", "mud-flap", "handhole-", "axles", "wheel-hub")
SUPER = ("subframe", "front-pedestal", "rear-pedestal", "pedestal", "support-roller", "roller-bracket", "rear-frame", "hinge-post", "hopper-stay", "swivel-stay", "water-tank", "ladder-", "rear-ladder", "guard-", "platform", "oil-cooler-stay")


def region_of(id, region):
    """The thermal region of a chassis part: the frame and its equipment stay `chassis`, axles, springs, rims and fenders are `running-gear`, the mixer subframe, pedestals, rear frame, water tank and ladder are `superstructure`."""
    if region != "chassis":
        return region
    if id.startswith(RUNNING):
        return "running-gear"
    if id.startswith(SUPER):
        return "superstructure"
    return region

