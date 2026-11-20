"""🧱 Round 3 ETICS brick wall 30: the real build-up of the 1 m x 1 m sample (NF solid brick in the 36.5 cm block bond, 1960) and the writers every script states its parts with.

Frame of the single bone `root`: u = thickness from the room face (0) to the render face (0.70), v = height (0 .. 1), w = length of the wall (-0.5 .. 0.5, left +).
References: DIN 105-1 / DIN 4172 (NF 240 x 115 x 71 mm, course 1/12 m, head joint 10 mm, bed joint 12.3 mm), DIN 1053-1 (block bond of the 36.5 cm wall), EAD 040083 / ETAG 004 and ETAG 014 (mineral wool ETICS boards 800 x 625 mm, plate anchors 60 mm, shank 8 mm, T scheme).
"""
import importlib.util
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
SLUG = "etics-brick-wall-30"
ASSET = "components/" + SLUG
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/etics-brick-wall-30/🔣️.json"
SOURCE = os.path.join(T, "volumes", "components", SLUG + ".json")
GEN = os.path.join(T, "🗑️generated", "r3-" + SLUG)

PLASTER = (0.0, 0.015)
BRICK = (0.015, 0.38)
WOOL = (0.38, 0.68)
RENDER = (0.68, 0.70)
WALL_W = (-0.5, 0.5)
HEIGHT = 1.0

COURSE = 1.0 / 12.0
BRICK_H = 0.071
BED = COURSE - BRICK_H
HEAD = 0.010
NF_L = 0.240
NF_W = 0.115
HEADER_PITCH = NF_W + HEAD
STRETCHER_PITCH = NF_L + HEAD
HEADER_JOINT0 = 0.03
STRETCHER_JOINT_A = HEADER_JOINT0 + 0.0625
STRETCHER_JOINT_B = HEADER_JOINT0 + 0.1875

LAYERS = {
    "A": {"stretcher": (BRICK[0], BRICK[0] + NF_W), "collar": (BRICK[0] + NF_W, BRICK[0] + NF_W + HEAD), "header": (BRICK[0] + NF_W + HEAD, BRICK[1]), "stretcherJoint": STRETCHER_JOINT_A},
    "B": {"header": (BRICK[0], BRICK[0] + NF_L), "collar": (BRICK[0] + NF_L, BRICK[0] + NF_L + HEAD), "stretcher": (BRICK[0] + NF_L + HEAD, BRICK[1]), "stretcherJoint": STRETCHER_JOINT_B},
}

BOARD_JOINT_V = 0.40
BOARD_JOINT_LOW_W = 0.15
BOARD_JOINT_HIGH_W = -0.25
DOWELS = [(BOARD_JOINT_LOW_W, BOARD_JOINT_V), (BOARD_JOINT_HIGH_W, BOARD_JOINT_V), (BOARD_JOINT_HIGH_W, 0.0875), (BOARD_JOINT_LOW_W, 0.7125)]
DOWEL_TIP_U = BRICK[1] - 0.05
DOWEL_PLATE_R = 0.03
DOWEL_SHANK_R = 0.004
DOWEL_PLATE_T = 0.008

LAYER_MASS = {"plaster": (0.015 * 1400, 0.0075), "brick": (0.365 * 1800, 0.1975), "insulation": (0.3 * 60, 0.53), "render": (0.02 * 1700, 0.69)}
MASS_KG = sum(m for m, _ in LAYER_MASS.values())
COM_U = sum(m * u for m, u in LAYER_MASS.values()) / MASS_KG
COM_V = 0.5


def r5(x):
    return round(x + 0.0, 5) + 0.0


def course_kind(k):
    return "A" if k % 2 == 0 else "B"


def bed_v(k):
    return (k * COURSE, k * COURSE + BED)


def brick_v(k):
    return (k * COURSE + BED, (k + 1) * COURSE)


def joints(first, pitch):
    """📏 Centres of the head joints of one brick row inside the sample length."""
    out = []
    c = first
    while c - pitch > WALL_W[0]:
        c -= pitch
    while c < WALL_W[1]:
        if c > WALL_W[0]:
            out.append(c)
        c += pitch
    return out


def pieces(centres):
    """🧩 The brick lengths along w between the head joints of a row (cut at the sample ends)."""
    edges = [WALL_W[0]] + [x for c in centres for x in (c - HEAD / 2, c + HEAD / 2)] + [WALL_W[1]]
    return [(edges[i], edges[i + 1]) for i in range(0, len(edges), 2)]


def box(id_, u, v, w, material, tone, role="body", **extra):
    """📦 An axis-aligned box from three intervals in the bone frame."""
    out = {"id": id_, "kind": "box", "bone": "root", "material": material, "role": role, "tone": tone,
           "size": [r5(u[1] - u[0]), r5(v[1] - v[0]), r5(w[1] - w[0])],
           "transform": {"translation": [r5((u[0] + u[1]) / 2), r5((v[0] + v[1]) / 2), r5((w[0] + w[1]) / 2)]}}
    out.update(extra)
    return out


def cuts(u, v, w):
    """✂️ The faces of a box that lie on the cut boundary of the sample (top, bottom, both ends): section faces (C25 a)."""
    faces = []
    if abs(v[0]) < 1e-9:
        faces.append("-v")
    if abs(v[1] - HEIGHT) < 1e-9:
        faces.append("+v")
    if abs(w[0] - WALL_W[0]) < 1e-9:
        faces.append("-w")
    if abs(w[1] - WALL_W[1]) < 1e-9:
        faces.append("+w")
    return faces


def load(path):
    with open(path, encoding="utf-8") as f:
        return json.load(f)


def kit_module(name):
    spec = importlib.util.spec_from_file_location(name, os.path.join(T, name + ".py"))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m
