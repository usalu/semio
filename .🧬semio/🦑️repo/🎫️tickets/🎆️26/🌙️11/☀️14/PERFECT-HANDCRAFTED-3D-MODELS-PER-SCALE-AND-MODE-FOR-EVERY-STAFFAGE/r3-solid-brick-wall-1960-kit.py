"""🧱 Round 3 solid brick wall 1960: the real build-up of the 1 m x 1 m sample (NF solid brick in the 36.5 cm block bond, 1960) and the writers every script states its parts with.

Frame of the single bone `root`: u = thickness from the room face (0) to the render face (0.40), v = height (0 .. 1), w = length of the wall (-0.5 .. 0.5, left +).
References: DIN 105-1 / DIN 4172 (NF 240 x 115 x 71 mm, course 1/12 m, head joint 10 mm, bed joint 12.3 mm), DIN 1053-1 (block bond of the 36.5 cm wall), EN ISO 10456 (densities).
"""
import importlib.util
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
SLUG = "solid-brick-wall-1960"
ASSET = "components/" + SLUG
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/solid-brick-wall-1960/🔣️.json"
SOURCE = os.path.join(T, "volumes", "components", SLUG + ".json")
GEN = os.path.join(T, "🗑️generated", "r3-" + SLUG)

PLASTER = (0.0, 0.015)
BRICK = (0.015, 0.38)
RENDER = (0.38, 0.40)
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

LAYER_MASS = {"plaster": (0.015 * 1400, 0.0075), "brick": (0.365 * 1800, 0.1975), "render": (0.02 * 1700, 0.39)}
MASS_KG = sum(m for m, _ in LAYER_MASS.values())
COM_U = sum(m * u for m, u in LAYER_MASS.values()) / MASS_KG
COM_V = 0.5


def r5(x):
    return round(x + 0.0, 9) + 0.0


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
