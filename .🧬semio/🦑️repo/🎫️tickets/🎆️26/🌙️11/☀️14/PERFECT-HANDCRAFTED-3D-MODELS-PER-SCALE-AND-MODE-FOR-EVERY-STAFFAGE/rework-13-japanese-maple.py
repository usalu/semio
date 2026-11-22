"""🍁 Japanese maple (Acer palmatum), multi-stem, 4.5 m: every solid of the four scales from hand-stated numbers.

The stated radii are fitted per scale by one factor per half space (printed as `fit`) so that every scale has the declared extent of 5.646 x 4.499 x 5.39 m.
Usage: `python3 rework-13-japanese-maple.py [--publish]` (needs `bun rework-13-frames.ts vegetation/japanese-maple` first).
World: x ahead (east), y up, z left. Azimuth 0 = +x, 90 = +z. A pad row is (ring, azimuth, radius from the tier centre, height above the tier level, a radial, b tangential, rise); the outer ring is widened tangentially by `outer_b` of the scale so that neighbours overlap and every scale keeps the same crown outline.
"""
import importlib.util
import math
import os
import sys

spec = importlib.util.spec_from_file_location("kit", os.path.join(os.path.dirname(os.path.abspath(__file__)), "rework-13-kit.py"))
kit = importlib.util.module_from_spec(spec)
spec.loader.exec_module(kit)
P = kit.Plant("vegetation", "🌳️vegetation", "japanese-maple")
BARK, YOUNG, SUN, SHADE, WOOD = "maple-bark", "maple-bark-young", "maple-leaf-sun", "maple-leaf-shade", "maple-wood"
KZ = 0.955
WARP = {"t1": (0.1, 40), "t2": (0.09, 200), "t3": (0.07, 110), "top": (0.0, 0)}


def at(bone, u, v=0.0, w=0.0):
    return P.world(bone, (u, v, w))


STEMS = {
    "stem-c": [(0.0, 0.0, 0.0, 0.085), (0.55, 0.035, -0.01, 0.079), (1.2, -0.03, -0.06, 0.071), (1.9, 0.025, -0.07, 0.061), (2.7, 0.0, -0.1093, 0.048)],
    "stem-w": [(0.0, 0.0, 0.0, 0.075), (0.5, -0.03, 0.09, 0.07), (1.3, 0.0, 0.27, 0.06), (2.0, 0.03, 0.43, 0.046), (2.4, 0.0, 0.5, 0.032)],
    "stem-e": [(0.0, 0.0, 0.0, 0.07), (0.6, 0.035, -0.07, 0.065), (1.5, 0.0, -0.2, 0.055), (2.2, -0.03, -0.31, 0.042), (2.6, 0.0, -0.36, 0.03)],
    "stem-c-top": [(0.0, 0.0, 0.0, 0.046), (0.35, 0.02, 0.015, 0.036), (0.7, -0.015, -0.01, 0.026), (1.0, 0.0, 0.0, 0.016)],
}
ROOTS = [(20, 0.46), (95, 0.4), (165, 0.44), (235, 0.38), (305, 0.42)]
TIERS = {
    "t1": {"centre": (0.05, 1.86, 0.0), "west": "t1w", "east": "t1e", "core": "fol-core-low", "limbs": [("t1w", 178, 2.2), ("t1w", 132, 2.1), ("t1w", 226, 2.05), ("t1e", 2, 2.25), ("t1e", 48, 2.1), ("t1e", 312, 2.1), ("stem-c", 88, 1.95), ("stem-c", 268, 1.9)]},
    "t2": {"centre": (0.05, 2.56, -0.02), "west": "t2w", "east": "t2e", "core": "fol-core-up", "limbs": [("t2w", 176, 1.9), ("t2w", 124, 1.75), ("t2w", 232, 1.8), ("t2e", 6, 1.95), ("t2e", 56, 1.8), ("t2e", 304, 1.75), ("stem-c", 92, 1.5), ("stem-c", 266, 1.5)]},
    "t3": {"centre": (0.08, 3.2, -0.06), "west": "t3w", "east": "t3e", "core": None, "limbs": [("t3w", 172, 1.4), ("t3w", 112, 1.25), ("t3w", 238, 1.3), ("t3e", 8, 1.45), ("t3e", 66, 1.25), ("t3e", 296, 1.3)]},
    "top": {"centre": (0.09, 3.7, -0.1), "west": "top", "east": "top", "core": None, "limbs": []},
}
PADS = {
    50: {
        "t1": [
            ("o", 5, 2.2, -0.06, 0.6, 0.52, 0.2), ("o", 33, 2.14, -0.02, 0.56, 0.5, 0.17), ("o", 62, 2.22, -0.08, 0.58, 0.55, 0.19), ("o", 95, 2.18, -0.03, 0.62, 0.5, 0.18),
            ("o", 124, 2.1, -0.07, 0.55, 0.52, 0.2), ("o", 150, 2.2, -0.04, 0.6, 0.48, 0.17), ("o", 182, 2.22, -0.09, 0.58, 0.54, 0.2), ("o", 212, 2.12, -0.03, 0.56, 0.5, 0.18),
            ("o", 240, 2.2, -0.07, 0.6, 0.52, 0.19), ("o", 268, 2.16, -0.04, 0.62, 0.5, 0.17), ("o", 300, 2.2, -0.08, 0.57, 0.53, 0.2), ("o", 332, 2.14, -0.03, 0.58, 0.5, 0.18),
            ("m", 18, 1.42, 0.07, 0.55, 0.5, 0.2), ("m", 50, 1.5, 0.04, 0.5, 0.48, 0.18), ("m", 85, 1.38, 0.08, 0.56, 0.46, 0.21), ("m", 118, 1.48, 0.05, 0.52, 0.5, 0.18), ("m", 160, 1.4, 0.09, 0.55, 0.48, 0.2),
            ("m", 196, 1.5, 0.04, 0.5, 0.5, 0.19), ("m", 230, 1.42, 0.08, 0.54, 0.46, 0.2), ("m", 262, 1.46, 0.05, 0.52, 0.5, 0.18), ("m", 296, 1.4, 0.09, 0.55, 0.47, 0.21), ("m", 338, 1.5, 0.05, 0.5, 0.5, 0.18),
            ("i", 30, 0.7, 0.14, 0.48, 0.44, 0.18), ("i", 90, 0.66, 0.11, 0.46, 0.42, 0.17), ("i", 150, 0.72, 0.15, 0.5, 0.42, 0.18), ("i", 215, 0.68, 0.11, 0.46, 0.44, 0.17), ("i", 275, 0.7, 0.14, 0.48, 0.42, 0.18), ("i", 330, 0.66, 0.12, 0.46, 0.44, 0.17),
        ],
        "t2": [
            ("o", 15, 1.92, -0.05, 0.56, 0.5, 0.2), ("o", 48, 1.86, -0.01, 0.52, 0.5, 0.18), ("o", 84, 1.94, -0.06, 0.55, 0.48, 0.19), ("o", 120, 1.88, -0.02, 0.54, 0.5, 0.18), ("o", 158, 1.95, -0.06, 0.56, 0.48, 0.2),
            ("o", 190, 1.9, -0.02, 0.54, 0.5, 0.18), ("o", 228, 1.94, -0.06, 0.55, 0.5, 0.19), ("o", 262, 1.86, -0.01, 0.52, 0.48, 0.18), ("o", 300, 1.93, -0.05, 0.56, 0.5, 0.2), ("o", 335, 1.88, -0.02, 0.54, 0.5, 0.18),
            ("m", 0, 1.12, 0.08, 0.52, 0.48, 0.2), ("m", 42, 1.2, 0.05, 0.5, 0.46, 0.18), ("m", 95, 1.1, 0.09, 0.52, 0.46, 0.2), ("m", 140, 1.18, 0.05, 0.5, 0.48, 0.18),
            ("m", 180, 1.12, 0.09, 0.52, 0.46, 0.2), ("m", 225, 1.2, 0.05, 0.5, 0.48, 0.18), ("m", 270, 1.1, 0.08, 0.52, 0.46, 0.2), ("m", 318, 1.18, 0.05, 0.5, 0.46, 0.18),
            ("i", 60, 0.42, 0.14, 0.44, 0.4, 0.17), ("i", 150, 0.44, 0.12, 0.42, 0.4, 0.16), ("i", 240, 0.42, 0.14, 0.44, 0.4, 0.17), ("i", 330, 0.44, 0.12, 0.42, 0.4, 0.16),
        ],
        "t3": [
            ("o", 10, 1.42, -0.06, 0.56, 0.5, 0.2), ("o", 55, 1.3, 0.0, 0.5, 0.5, 0.18), ("o", 100, 1.4, -0.05, 0.54, 0.48, 0.19), ("o", 145, 1.3, 0.0, 0.5, 0.5, 0.18),
            ("o", 190, 1.42, -0.06, 0.56, 0.5, 0.2), ("o", 235, 1.3, 0.0, 0.5, 0.48, 0.18), ("o", 280, 1.4, -0.05, 0.54, 0.5, 0.19), ("o", 325, 1.3, 0.0, 0.5, 0.5, 0.18),
            ("i", 30, 0.55, 0.1, 0.52, 0.46, 0.2), ("i", 120, 0.5, 0.13, 0.5, 0.46, 0.19), ("i", 210, 0.55, 0.1, 0.52, 0.46, 0.2), ("i", 300, 0.5, 0.13, 0.5, 0.46, 0.19),
        ],
        "top": [
            ("r", 20, 0.95, -0.05, 0.52, 0.5, 0.21), ("r", 80, 0.86, 0.02, 0.5, 0.48, 0.2), ("r", 140, 0.95, -0.04, 0.52, 0.48, 0.21), ("r", 200, 0.86, 0.02, 0.5, 0.5, 0.2), ("r", 260, 0.95, -0.05, 0.52, 0.48, 0.21), ("r", 320, 0.86, 0.02, 0.5, 0.48, 0.2),
            ("m", 50, 0.45, 0.28, 0.5, 0.46, 0.21), ("m", 140, 0.4, 0.33, 0.48, 0.46, 0.2), ("m", 230, 0.45, 0.28, 0.5, 0.46, 0.21), ("m", 320, 0.4, 0.33, 0.48, 0.46, 0.2),
            ("f", 0, 0.0, 0.565, 0.5, 0.46, 0.234),
        ],
    },
    100: {
        "t1": [("o", 5, 2.2, -0.06, 0.6, 0.52, 0.22), ("o", 33, 2.14, -0.02, 0.56, 0.5, 0.19), ("o", 62, 2.22, -0.08, 0.58, 0.55, 0.21), ("o", 95, 2.18, -0.03, 0.62, 0.5, 0.20), ("o", 124, 2.1, -0.07, 0.55, 0.52, 0.22), ("o", 150, 2.2, -0.04, 0.6, 0.48, 0.19), ("o", 182, 2.22, -0.09, 0.58, 0.54, 0.22), ("o", 212, 2.12, -0.03, 0.56, 0.5, 0.20), ("o", 240, 2.2, -0.07, 0.6, 0.52, 0.21), ("o", 268, 2.16, -0.04, 0.62, 0.5, 0.19), ("o", 300, 2.2, -0.08, 0.57, 0.53, 0.22), ("o", 332, 2.14, -0.03, 0.58, 0.5, 0.20), ("m", 30, 1.1, 0.08, 0.7, 0.66, 0.22), ("m", 90, 1.14, 0.06, 0.68, 0.66, 0.2), ("m", 150, 1.1, 0.08, 0.7, 0.66, 0.22), ("m", 210, 1.14, 0.06, 0.68, 0.66, 0.2), ("m", 270, 1.1, 0.08, 0.7, 0.66, 0.22), ("m", 330, 1.14, 0.06, 0.68, 0.66, 0.2), ("i", 0, 0.42, 0.14, 0.52, 0.5, 0.18), ("i", 120, 0.42, 0.12, 0.5, 0.5, 0.17), ("i", 240, 0.42, 0.14, 0.52, 0.5, 0.18)],
        "t2": [("o", 15, 1.92, -0.05, 0.56, 0.5, 0.22), ("o", 48, 1.86, -0.01, 0.52, 0.5, 0.20), ("o", 84, 1.94, -0.06, 0.55, 0.48, 0.21), ("o", 120, 1.88, -0.02, 0.54, 0.5, 0.20), ("o", 158, 1.95, -0.06, 0.56, 0.48, 0.22), ("o", 190, 1.9, -0.02, 0.54, 0.5, 0.20), ("o", 228, 1.94, -0.06, 0.55, 0.5, 0.21), ("o", 262, 1.86, -0.01, 0.52, 0.48, 0.20), ("o", 300, 1.93, -0.05, 0.56, 0.5, 0.22), ("o", 335, 1.88, -0.02, 0.54, 0.5, 0.20), ("m", 46, 0.86, 0.08, 0.64, 0.6, 0.21), ("m", 118, 0.9, 0.06, 0.62, 0.6, 0.2), ("m", 190, 0.86, 0.08, 0.64, 0.6, 0.21), ("m", 262, 0.9, 0.06, 0.62, 0.6, 0.2), ("m", 334, 0.86, 0.08, 0.64, 0.6, 0.21), ("i", 90, 0.2, 0.14, 0.48, 0.46, 0.17), ("i", 270, 0.2, 0.13, 0.48, 0.46, 0.17)],
        "t3": [("o", 10, 1.42, -0.06, 0.56, 0.5, 0.22), ("o", 55, 1.3, 0.0, 0.5, 0.5, 0.20), ("o", 100, 1.4, -0.05, 0.54, 0.48, 0.21), ("o", 145, 1.3, 0.0, 0.5, 0.5, 0.20), ("o", 190, 1.42, -0.06, 0.56, 0.5, 0.22), ("o", 235, 1.3, 0.0, 0.5, 0.48, 0.20), ("o", 280, 1.4, -0.05, 0.54, 0.5, 0.21), ("o", 325, 1.3, 0.0, 0.5, 0.5, 0.20), ("i", 90, 0.24, 0.11, 0.54, 0.5, 0.2), ("i", 270, 0.24, 0.12, 0.54, 0.5, 0.2)],
        "top": [("r", 20, 0.95, -0.05, 0.52, 0.5, 0.23), ("r", 80, 0.86, 0.02, 0.5, 0.48, 0.22), ("r", 140, 0.95, -0.04, 0.52, 0.48, 0.23), ("r", 200, 0.86, 0.02, 0.5, 0.5, 0.22), ("r", 260, 0.95, -0.05, 0.52, 0.48, 0.23), ("r", 320, 0.86, 0.02, 0.5, 0.48, 0.22), ("m", 80, 0.22, 0.3, 0.5, 0.48, 0.22), ("m", 260, 0.22, 0.32, 0.5, 0.48, 0.22), ("f", 0, 0.0, 0.565, 0.46, 0.44, 0.234)],
    },
    200: {
        "t1": [("o", 5, 2.16, -0.06, 0.64, 0.52, 0.24), ("o", 33, 2.10, -0.02, 0.60, 0.5, 0.21), ("o", 62, 2.18, -0.08, 0.62, 0.55, 0.23), ("o", 95, 2.14, -0.03, 0.66, 0.5, 0.22), ("o", 124, 2.06, -0.07, 0.59, 0.52, 0.24), ("o", 150, 2.16, -0.04, 0.64, 0.48, 0.21), ("o", 182, 2.18, -0.09, 0.62, 0.54, 0.24), ("o", 212, 2.08, -0.03, 0.60, 0.5, 0.22), ("o", 240, 2.16, -0.07, 0.64, 0.52, 0.23), ("o", 268, 2.12, -0.04, 0.66, 0.5, 0.21), ("o", 300, 2.16, -0.08, 0.61, 0.53, 0.24), ("o", 332, 2.10, -0.03, 0.62, 0.5, 0.22), ("m", 60, 0.75, 0.1, 0.85, 0.85, 0.24), ("m", 180, 0.75, 0.09, 0.85, 0.85, 0.24), ("m", 300, 0.75, 0.1, 0.85, 0.85, 0.24), ("i", 0, 0.1, 0.15, 0.6, 0.6, 0.20)],
        "t2": [("o", 15, 1.88, -0.05, 0.60, 0.5, 0.24), ("o", 48, 1.82, -0.01, 0.56, 0.5, 0.22), ("o", 84, 1.90, -0.06, 0.59, 0.48, 0.23), ("o", 120, 1.84, -0.02, 0.58, 0.5, 0.22), ("o", 158, 1.91, -0.06, 0.60, 0.48, 0.24), ("o", 190, 1.86, -0.02, 0.58, 0.5, 0.22), ("o", 228, 1.90, -0.06, 0.59, 0.5, 0.23), ("o", 262, 1.82, -0.01, 0.56, 0.48, 0.22), ("o", 300, 1.89, -0.05, 0.60, 0.5, 0.24), ("o", 335, 1.84, -0.02, 0.58, 0.5, 0.22), ("m", 90, 0.5, 0.1, 0.8, 0.78, 0.24), ("m", 270, 0.5, 0.1, 0.8, 0.78, 0.24), ("i", 0, 0.05, 0.16, 0.55, 0.55, 0.18)],
        "t3": [("o", 10, 1.38, -0.06, 0.60, 0.5, 0.24), ("o", 55, 1.26, 0.0, 0.54, 0.5, 0.22), ("o", 100, 1.36, -0.05, 0.58, 0.48, 0.23), ("o", 145, 1.26, 0.0, 0.54, 0.5, 0.22), ("o", 190, 1.38, -0.06, 0.60, 0.5, 0.24), ("o", 235, 1.26, 0.0, 0.54, 0.48, 0.22), ("o", 280, 1.36, -0.05, 0.58, 0.5, 0.23), ("o", 325, 1.26, 0.0, 0.54, 0.5, 0.22)],
        "top": [("r", 60, 0.5, 0.0, 0.66, 0.66, 0.24), ("r", 180, 0.5, 0.03, 0.66, 0.66, 0.24), ("r", 300, 0.5, 0.0, 0.66, 0.66, 0.24), ("f", 0, 0.0, 0.5, 0.6, 0.58, 0.30)],
    },
    500: {
        "t1": [("o", 3, 1.7, -0.04, 1.08, 1.72, 0.4), ("o", 94, 1.7, -0.05, 1.08, 1.72, 0.4), ("o", 181, 1.7, -0.05, 1.08, 1.72, 0.4), ("o", 269, 1.7, -0.05, 1.08, 1.72, 0.4), ("i", 0, 0.0, 0.12, 1.0, 1.0, 0.34)],
        "t2": [("o", 15, 1.44, -0.03, 1.02, 1.5, 0.38), ("o", 102, 1.44, -0.04, 1.02, 1.5, 0.38), ("o", 190, 1.44, -0.04, 1.02, 1.5, 0.38), ("o", 281, 1.44, -0.04, 1.02, 1.5, 0.38), ("i", 0, 0.0, 0.12, 0.9, 0.9, 0.32)],
        "t3": [("o", 32, 0.88, -0.03, 0.9, 0.98, 0.36), ("o", 122, 0.88, -0.02, 0.9, 0.98, 0.36), ("o", 212, 0.88, -0.03, 0.9, 0.98, 0.36), ("o", 302, 0.88, -0.02, 0.9, 0.98, 0.36)],
        "top": [("r", 90, 0.42, 0.0, 0.7, 0.72, 0.34), ("r", 270, 0.42, 0.02, 0.7, 0.72, 0.34), ("f", 0, 0.0, 0.33, 0.8, 0.78, 0.469)],
    },
}
STYLE = {50: {"teeth": 26, "depth": 0.085, "sides": 52, "lobe": 0.1, "limb_sides": 10, "twig": 0.008, "under": 0.42, "dip": 7, "outer_b": 1.2, "limb": (0.03, 0.012)}, 100: {"teeth": 13, "depth": 0.09, "sides": 39, "lobe": 0.1, "limb_sides": 8, "twig": 0.011, "under": 0.5, "dip": 6, "outer_b": 1.2, "limb": (0.03, 0.012)},
         200: {"teeth": 9, "depth": 0.08, "sides": 36, "lobe": 0.08, "limb_sides": 8, "twig": 0.022, "under": 0.5, "dip": 5, "outer_b": 1.24, "limb": (0.03, 0.022)}, 500: {"teeth": 6, "depth": 0.045, "sides": 36, "lobe": 0.08, "limb_sides": 8, "twig": 0.055, "under": 0.55, "dip": 3, "outer_b": 1.0, "limb": (0.046, 0.045)}}
LEAF = [(1.0, 0.0), (0.5, 0.13), (0.86, 0.5), (0.36, 0.3), (0.42, 0.9), (0.1, 0.34), (-0.25, 0.72), (-0.12, 0.16), (-0.42, 0.0), (-0.12, -0.16), (-0.25, -0.72), (0.1, -0.34), (0.42, -0.9), (0.36, -0.3), (0.86, -0.5), (0.5, -0.13)]


TARGET = {"x": (-2.8, 2.846), "z": (-2.695, 2.695), "y": 4.499}


def build(scale, fit, measure=False):
    style = STYLE[scale]
    P.begin(scale)
    wood = {"role": "body", "tone": 0.4}
    P.solid("root-collar", "root-plate", kit.tube([(0, 0.0, 0, 0.3), (0, 0.045, 0, 0.215), (0, 0.12, 0, 0.155), (0, 0.24, 0, 0.128), (0, 0.33, 0, 0.11)], sides=16 if scale < 200 else 10, start="flat"), BARK, tone=0.36, region="stem-bark", member="member", draw=wood)
    if scale <= 100:
        for index, (azimuth, reach) in enumerate(ROOTS):
            mesh = kit.tube([kit.polar(azimuth, 0.1, 0.13) + (0.06,), kit.polar(azimuth, 0.24, 0.07) + (0.05,), kit.polar(azimuth + 6, reach, 0.028) + (0.027,)], sides=8)
            P.solid("root-%d" % (index + 1), "root-plate", mesh, BARK, tone=0.36, region="stem-bark", member="member", draw=wood)
    for bone, rows in STEMS.items():
        mesh = kit.tube([at(bone, u, v, w) + (r,) for u, v, w, r in rows], sides=style["limb_sides"] + 2, sub_steps=4 if scale <= 100 else 2)
        P.solid("wood-" + bone, bone, mesh, BARK, tone=0.36, region="stem-bark", member="member", draw=wood)
        P.mass_at[bone] = at(bone, *rows[2][:3])
    nodes = {}
    for tier, spec in TIERS.items():
        nodes[tier] = []
        cx, cy, cz = spec["centre"]
        for index, (owner, azimuth, reach) in enumerate(spec["limbs"]):
            if owner == "stem-c":
                start = at("stem-c", cy - 0.25 - 0.42, 0.0, -0.0405 * (cy - 0.67))
            else:
                start = tuple(P.frames[owner]["o"])
            r0, thin = style["limb"]
            end = kit.polar(azimuth, reach, cy - 0.1, (cx, 0, cz))
            end = (end[0], end[1], end[2] * KZ)
            mid1 = (start[0] + (end[0] - start[0]) * 0.35, start[1] + (end[1] - start[1]) * 0.75 + 0.04, start[2] + (end[2] - start[2]) * 0.3 + 0.05 * math.sin(index * 2.1))
            mid2 = (start[0] + (end[0] - start[0]) * 0.7, end[1] + 0.03, start[2] + (end[2] - start[2]) * 0.72 - 0.05 * math.sin(index * 2.1))
            stations = [start + (r0,), mid1 + (r0 * 0.8,), mid2 + (max(r0 * 0.55, thin),), end + (thin,)]
            slender = 2 * r0 < kit.FEATURE[scale]
            ident = "limb-%s" % owner if index in (0, 3) and owner != "stem-c" else "limb-%s-%s" % (tier, "abcdefgh"[index])
            P.solid(ident, owner, kit.tube(stations, sides=style["limb_sides"], sub_steps=3 if scale <= 100 else 2), YOUNG, role="detail" if slender else "body", tone=0.38, region="limb-bark", member="member", slender=slender,
                    draw=None if slender else {"role": "body", "tone": 0.38})
            if slender:
                P.stroke("ink-" + ident, owner, [s[:3] for s in stations], role="detail", tone=0.3, material=YOUNG, region="limb-bark")
            nodes[tier].extend(s[:3] for s in kit.spline(stations, 4))
            if ident == "limb-" + owner:
                P.mass_at[owner] = start if slender else mid1
            if scale <= 100:
                for side, turn in (("l", 24), ("r", -22)):
                    base = kit.spline(stations, 4)[6]
                    tip = kit.polar(azimuth + turn, reach * 0.86, cy - 0.08, (cx, 0, cz))
                    tip = (tip[0], tip[1], tip[2] * KZ)
                    bend = ((base[0] + tip[0]) / 2, (base[1] + tip[1]) / 2 + 0.035, (base[2] + tip[2]) / 2)
                    fork = [base[:3] + (max(base[3] * 0.7, thin),), bend + (thin * 1.15,), tip + (thin * 0.8,)]
                    P.solid("%s-%s" % (ident, side), owner, kit.tube(fork, sides=6, sub_steps=3), YOUNG, role="detail", tone=0.38, region="limb-bark", member="member")
                    P.stroke("ink-%s-%s" % (ident, side), owner, [s[:3] for s in kit.spline(fork, 3)], role="detail", tone=0.3, material=YOUNG, region="limb-bark")
                    nodes[tier].extend(s[:3] for s in kit.spline(fork, 4))
    nodes["top"] = [s[:3] for s in kit.spline([at("stem-c-top", u, v, w) + (r,) for u, v, w, r in STEMS["stem-c-top"]], 6)]
    nodes["t3"].extend(nodes["top"][:8])
    stem_nodes = [s[:3] for bone in ("stem-c", "stem-w", "stem-e") for s in kit.spline([at(bone, u, v, w) + (r,) for u, v, w, r in STEMS[bone]], 6)]
    picked = {}
    for tier, spec in TIERS.items():
        cx, cy, cz = spec["centre"]
        counts = {}
        for row_index, (ring, azimuth, radius, dy, a, b, rise) in enumerate(PADS[scale][tier]):
            counts[ring] = counts.get(ring, 0) + 1
            warp = WARP[tier][0] * math.cos(math.radians(azimuth - WARP[tier][1])) * radius / 2.2
            centre = kit.polar(azimuth, radius, cy + dy + warp, (cx, 0, cz))
            centre = ((centre[0] - cx) * fit["e" if centre[0] > cx else "w"] + cx, centre[1] + (fit["top"] if ring == "f" else 0.0), centre[2] * KZ * fit["n" if centre[2] > 0 else "s"])
            shade = ring == "i" and spec["core"] is not None
            if tier == "top":
                bone = "fol-top"
            elif shade:
                bone = spec["core"]
            else:
                bone = "fol-" + (spec["west"] if centre[0] < cx else spec["east"])
            ident = "%s-%s%d" % (tier, ring, counts[ring])
            outer = ring == "o"
            if outer:
                b *= style["outer_b"]
            lobes = ((2, style["lobe"] * 0.7, 71 * row_index), (3, style["lobe"], 47 * row_index + 10), (5, style["lobe"] * 0.6, 83 * row_index))
            bumps = ((3, 0.13, 59 * row_index), (4, 0.09, 101 * row_index + 30))
            rim = []
            mesh = kit.pad(centre, a, b, rise, rise * style["under"], yaw=azimuth, dip=style["dip"] if outer else (3 if ring in ("m", "r") else 0), lobes=lobes, teeth=(style["teeth"], style["depth"]), sides=style["sides"], droop=0.05 if outer else 0.02, bumps=bumps, power=0.62, rim=rim)
            material = SHADE if shade else SUN
            region = "leaf-shade" if shade else "leaf-sun"
            draw = {"role": "body" if scale >= 100 else "detail", "tone": 0.45 if shade else 0.6}
            if shade:
                draw["hatch"] = "shade-lines"
            P.solid(ident, bone, mesh, material, tone=0.5 if shade else 0.62, region=region, draw=draw)
            picked.setdefault(bone, []).append((ident, centre, rise, a, b))
            under = (centre[0] - 0.25 * a * math.cos(math.radians(azimuth)), centre[1] - rise * 0.2, centre[2] - 0.25 * a * math.sin(math.radians(azimuth)))
            pool = nodes[tier] + (stem_nodes if ring == "i" or not nodes[tier] else [])
            near = min(pool, key=lambda n: (n[0] - under[0]) ** 2 + (n[1] - under[1]) ** 2 * 4 + (n[2] - under[2]) ** 2)
            if scale >= 200:
                under = centre
            elif kit.dot(kit.sub(under, near), kit.sub(under, near)) < 0.2 ** 2:
                under = (under[0], near[1] + 0.2, under[2])
            twig = [near + (style["twig"] * 1.2,), under + (max(style["twig"] * 0.7, 0.6 * kit.FEATURE[scale]),)]
            P.mass_at.setdefault(bone, under)
            P.joint_at.setdefault(bone, near)
            P.solid("twig-" + ident, bone, kit.tube(twig, sides=6, sub_steps=2), YOUNG, role="detail", tone=0.38, member="member")
            if scale <= 100:
                P.stroke("ink-twig-" + ident, bone, [s[:3] for s in twig], role="texture", tone=0.3, material=YOUNG, region="limb-bark")
            if scale == 50:
                leaves(ident, bone, rim, row_index, material)
    if measure:
        return P.box
    emitters(picked)
    P.extra_structure({"id": "support-base-support", "kind": "revolve", "bone": "root-plate", "material": WOOD, "role": "detail", "tone": 0.25, "profile": [[-0.04, 0.6], [0, 0.6]], "axis": [1, 0, 0], "origin": [0, 0, 0], "member": "root-plate", "part": "support", "anchor": "base-support"})
    P.skeleton_marks(WOOD, WOOD)
    P.end(DECISIONS[scale])
    return picked


def leaves(ident, bone, rim, row_index, material):
    """🍁 The fringe of a cushion: a seven-lobed leaf of 10 cm at every third rim point, its base in the cushion, its blade leaning out and alternately up and down; one of them is drawn."""
    size = 0.05
    for index in range(row_index % 3, len(rim), 3):
        point, out = rim[index]
        lean = (0.5, -0.35, 0.15)[(index // 3) % 3]
        normal = kit.norm((-out[0] * lean, 1.0, -out[2] * lean))
        position = kit.add(point, kit.mul(out, -0.3 * size))
        mesh = kit.plate([(x * size, y * size) for x, y in LEAF], position, normal, out, 0.006)
        P.solid("leaf-%s-%d" % (ident, index // 3 + 1), bone, mesh, material, role="texture", tone=0.68, texture=True, slender=True)
        if index // 3 == 2:
            ahead = kit.norm(kit.sub(out, kit.mul(normal, kit.dot(out, normal))))
            across = kit.cross(normal, ahead)
            ring = [kit.add(position, kit.add(kit.mul(ahead, x * size), kit.mul(across, y * size))) for x, y in LEAF]
            P.stroke("ink-leaf-" + ident, bone, ring + ring[:1], role="texture", tone=0.25, material=material, after=ident)


ANCHOR_PAD = {}


def emitters(picked):
    """💧 Every transpiration anchor sits on the crest of a cushion of its own bone: a lens patch there is the emitter solid."""
    used = {}
    for anchor in P.asset["anchors"]:
        if anchor.get("kind") != "emitter":
            continue
        pads = picked[anchor["bone"]]
        index = used.get(anchor["bone"], 0)
        used[anchor["bone"]] = index + 1
        ident, centre, rise, a, b = pads[(index * 3) % len(pads)]
        crest = (centre[0], centre[1] + rise * 0.93, centre[2])
        shade = anchor["id"].startswith("tp-shade")
        P.lens("th-" + anchor["id"], anchor["bone"], crest, min(a, b) * 0.42, rise * 0.12, SHADE if shade else SUN, "leaf-shade" if shade else "leaf-sun", "transpiration-shade" if shade else "transpiration-sun")
        if P.scale == 50:
            ANCHOR_PAD[anchor["id"]] = P.local(anchor["bone"], (crest[0], crest[1] + rise * 0.1, crest[2]))


def text(en, de):
    return {"en": en, "de": de}


DECISIONS = {
    50: [
        text("Body at 1:50: a flared root collar with five surface roots; three sinuous stems of 0.17, 0.15 and 0.14 m that splay from it and taper to 6 to 10 cm, the leader above the centre stem. Each of the three tiers is carried by limbs that fan from the tier joints and from the centre stem to every quarter (eight, eight and six), each forking once into an opposite pair; a twig runs from the nearest limb into the underside of every cushion. The crown is seventy-three leaf cushions in three flat, slightly drooping tiers and a domed top: every cushion is a low dome over a flatter underside with a rim serrated at leaf size, and carries a fringe of seven-lobed leaves of 10 cm, one at every third tooth of its rim.",
             "Körper bei 1:50: ein ausgestellter Wurzelhals mit fünf Oberflächenwurzeln; drei geschwungene Stämme von 0,17, 0,15 und 0,14 m, die sich daraus spreizen und auf 6 bis 10 cm verjüngen, darüber der Leittrieb des Mittelstamms. Jede der drei Etagen wird von Ästen getragen, die von den Etagengelenken und vom Mittelstamm in jedes Viertel fächern (acht, acht und sechs) und sich je einmal in ein gegenständiges Paar gabeln; ein Zweig führt vom nächsten Ast in die Unterseite jedes Polsters. Die Krone besteht aus dreiundsiebzig Laubpolstern in drei flachen, leicht hängenden Etagen und einem gewölbten Wipfel: jedes Polster ist eine flache Kuppel über einer flacheren Unterseite mit einem in Blattgröße gezähnten Rand und trägt einen Saum aus siebenlappigen Blättern von 10 cm, eines an jedem dritten Zahn seines Rands."),
        text("Thermal at 1:50: stem bark is the collar, the roots, the three stems and the leader; limb bark the limbs and their forks; sun leaves the cushions of the tiers and the top, shade leaves the inner cushions round the stems, all by reference to the body. The twelve transpiration outlets are lens patches on the crest of the cushion that carries each anchor.",
             "Thermik bei 1:50: Stammrinde sind Wurzelhals, Wurzeln, die drei Stämme und der Leittrieb; Astrinde die Äste und ihre Gabeln; Sonnenblätter die Polster der Etagen und des Wipfels, Schattenblätter die inneren Polster um die Stämme, alles als Verweis auf den Körper. Die zwölf Transpirationsauslässe sind Linsenflächen auf dem Scheitel des Polsters, das den jeweiligen Anker trägt."),
        text("Structure at 1:50: the wood itself is the load path, by reference to the body: collar, stems, leader, limbs, forks and the twigs that carry the cushions. A ball marks the joint of every bone, the base disc the fixed root plate, a ball the mass of every bone at its centre of mass.",
             "Tragwerk bei 1:50: das Holz selbst ist der Lastpfad, als Verweis auf den Körper: Wurzelhals, Stämme, Leittrieb, Äste, Gabeln und die Zweige, die die Polster tragen. Eine Kugel markiert das Gelenk jedes Knochens, die Fußscheibe den eingespannten Wurzelteller, eine Kugel die Masse jedes Knochens in ihrem Schwerpunkt."),
        text("Drawings at 1:50: every cushion, stem and limb is drawn as the outline of its own solid, layered by depth towards the viewer; thin forks and twigs are single lines, one leaf per cushion is drawn at its rim.",
             "Zeichnungen bei 1:50: jedes Polster, jeder Stamm und jeder Ast ist als Umriss des eigenen Körpers gezeichnet, nach Tiefe zum Betrachter geschichtet; dünne Gabeln und Zweige sind einzelne Linien, je Polster ist ein Blatt am Rand gezeichnet."),
    ],
    100: [
        text("Body at 1:100: collar with five roots, the three sinuous stems and the leader as at 1:50; the same limbs with their forks, twigs of 2 cm into the cushions. The crown is fifty-seven cushions in the same three tiers and top (the outer rings as at 1:50, the inner rings merged to half as many larger cushions), their rims scalloped at the size of a leaf cluster (about 12 cm); single leaves are below the smallest feature of 15 mm and are left out.",
             "Körper bei 1:100: Wurzelhals mit fünf Wurzeln, die drei geschwungenen Stämme und der Leittrieb wie bei 1:50; dieselben Äste mit ihren Gabeln, Zweige von 2 cm in die Polster. Die Krone besteht aus siebenundfünfzig Polstern in denselben drei Etagen und dem Wipfel (die Außenringe wie bei 1:50, die Innenringe zu halb so vielen größeren Polstern zusammengefasst), ihre Ränder in der Größe eines Blattbüschels gebuchtet (etwa 12 cm); einzelne Blätter liegen unter dem kleinsten Merkmal von 15 mm und entfallen."),
        text("Thermal at 1:100: the same four regions by reference to the body; the twelve outlets are lens patches on the crests of the cushions that carry the anchors.", "Thermik bei 1:100: dieselben vier Regionen als Verweis auf den Körper; die zwölf Auslässe sind Linsenflächen auf den Scheiteln der Polster, die die Anker tragen."),
        text("Structure at 1:100: collar, stems, leader, limbs, forks and twigs by reference to the body; joint balls, the base disc and one mass ball per bone.", "Tragwerk bei 1:100: Wurzelhals, Stämme, Leittrieb, Äste, Gabeln und Zweige als Verweis auf den Körper; Gelenkkugeln, die Fußscheibe und je Knochen eine Massenkugel."),
        text("Drawings at 1:100: outlines of the cushions, stems and limbs layered by depth; forks and twigs as single lines.", "Zeichnungen bei 1:100: Umrisse der Polster, Stämme und Äste nach Tiefe geschichtet; Gabeln und Zweige als einzelne Linien."),
    ],
    200: [
        text("Body at 1:200: collar without separate roots, three stems and leader with the same sweep; the tier limbs without forks. The crown keeps the outer cushions of the three tiers (twelve, ten and eight, each a little longer and deeper than at 1:100) because they are the outline of the tree, and merges everything inside them into three, three and no inner cushions; the top is three cushions and a crest; the rims are scalloped at 30 cm, the size of a spray.",
             "Körper bei 1:200: Wurzelhals ohne einzelne Wurzeln, drei Stämme und Leittrieb mit demselben Schwung; die Etagenäste ohne Gabeln. Die Krone behält die äußeren Polster der drei Etagen (zwölf, zehn und acht, jedes etwas länger und tiefer als bei 1:100), weil sie der Umriss des Baums sind, und fasst alles innerhalb davon zu drei, drei und keinem inneren Polster zusammen; der Wipfel besteht aus drei Polstern und einem Scheitel; die Ränder sind in 30 cm gebuchtet, der Größe eines Triebs."),
        text("Thermal at 1:200: the four regions by reference to the body, twelve lens outlets on the cushions of the anchors.", "Thermik bei 1:200: die vier Regionen als Verweis auf den Körper, zwölf Linsenauslässe auf den Polstern der Anker."),
        text("Structure at 1:200: collar, stems, leader, limbs and the carrying branches of the cushions by reference; joint balls, base disc, mass balls.", "Tragwerk bei 1:200: Wurzelhals, Stämme, Leittrieb, Äste und die Tragäste der Polster als Verweis; Gelenkkugeln, Fußscheibe, Massenkugeln."),
        text("Drawings at 1:200: outlines of the cushions and of the wood; the thin tier limbs are single lines.", "Zeichnungen bei 1:200: Umrisse der Polster und des Holzes; die dünnen Etagenäste sind einzelne Linien."),
    ],
    500: [
        text("Body at 1:500: the collar, the three splayed stems and the leader; the tier limbs are thinner than the smallest feature of 75 mm and stay as slender lines. The crown is generalised to its true layered habit in eighteen deep cushions: five round the first tier and four round the second, each with a shaded cushion at the stems, three in the third tier, two flanks and a crest dome in the top; every cushion is as deep below as it is domed above, its outline lobed.",
             "Körper bei 1:500: der Wurzelhals, die drei gespreizten Stämme und der Leittrieb; die Etagenäste sind dünner als das kleinste Merkmal von 75 mm und bleiben schlanke Linien. Die Krone ist auf ihren wahren geschichteten Wuchs in achtzehn tiefen Polstern verallgemeinert: fünf um die erste und vier um die zweite Etage, je mit einem beschatteten Polster an den Stämmen, drei in der dritten Etage, zwei Flanken und eine Scheitelkuppel im Wipfel; jedes Polster ist unten so tief wie oben gewölbt, sein Umriss gelappt."),
        text("Thermal at 1:500: the four regions by reference to the body, the outlets as lens patches on the cushion of each anchor.", "Thermik bei 1:500: die vier Regionen als Verweis auf den Körper, die Auslässe als Linsenflächen auf dem Polster des jeweiligen Ankers."),
        text("Structure at 1:500: collar, stems, leader and the slender tier limbs and carrying branches by reference; joint balls, base disc, mass balls.", "Tragwerk bei 1:500: Wurzelhals, Stämme, Leittrieb und die schlanken Etagenäste und Tragäste als Verweis; Gelenkkugeln, Fußscheibe, Massenkugeln."),
        text("Drawings at 1:500: one outline per cushion and stem, nothing finer.", "Zeichnungen bei 1:500: je Polster und Stamm ein Umriss, nichts Feineres."),
    ],
}

for scale in (50, 100, 200, 500):
    fit = {"w": 1.0, "e": 1.0, "n": 1.0, "s": 1.0, "top": 0.0}
    for _ in range(4):
        low, high = build(scale, fit, measure=True)
        fit["w"] *= TARGET["x"][0] / low[0]
        fit["e"] *= TARGET["x"][1] / high[0]
        fit["s"] *= TARGET["z"][0] / low[2]
        fit["n"] *= TARGET["z"][1] / high[2]
        fit["top"] += TARGET["y"] - high[1]
    print("fit", scale, {k: round(v, 4) for k, v in fit.items()})
    build(scale, fit)
P.save_source()


AREA = {"limb-bark": 6.506}
DRAWN = {
    50: ("At 1:50 (1 m is 20 mm) the three drawings are the true views of the 1:50 body: the flared collar with its surface roots, the three sinuous stems, the limbs that fan under each tier and the seventy-three leaf cushions with their leaf-toothed rims, each drawn as the outline of its own solid and layered by depth.",
         "Bei 1:50 (1 m sind 20 mm) sind die drei Zeichnungen die wahren Ansichten des Körpers 1:50: der ausgestellte Wurzelhals mit seinen Oberflächenwurzeln, die drei geschwungenen Stämme, die Äste, die unter jeder Etage fächern, und die dreiundsiebzig Laubpolster mit ihren blattgezähnten Rändern, jedes als Umriss des eigenen Körpers gezeichnet und nach Tiefe geschichtet."),
    100: ("At 1:100 (1 m is 10 mm) the drawings are the true views of the 1:100 body: collar, stems, limbs and fifty-seven cushions with rims scalloped at the size of a leaf cluster.",
          "Bei 1:100 (1 m sind 10 mm) sind die Zeichnungen die wahren Ansichten des Körpers 1:100: Wurzelhals, Stämme, Äste und siebenundfünfzig Polster mit in Büschelgröße gebuchteten Rändern."),
    200: ("At 1:200 (1 m is 5 mm) the drawings are the true views of the 1:200 body: collar, stems, tier limbs and forty cushions in three tiers and a top.",
          "Bei 1:200 (1 m sind 5 mm) sind die Zeichnungen die wahren Ansichten des Körpers 1:200: Wurzelhals, Stämme, Etagenäste und vierzig Polster in drei Etagen und einem Wipfel."),
    500: ("At 1:500 (1 m is 2 mm) the drawings are the true views of the 1:500 body: the collar, the three splayed stems and eighteen deep cushions that keep the layered habit.",
          "Bei 1:500 (1 m sind 2 mm) sind die Zeichnungen die wahren Ansichten des Körpers 1:500: der Wurzelhals, die drei gespreizten Stämme und achtzehn tiefe Polster, die den geschichteten Wuchs bewahren."),
}


def edit(asset):
    for region in asset["thermal"]["regions"]:
        if region["id"] in AREA:
            region["areaM2"] = AREA[region["id"]]
    for scale, (en, de) in DRAWN.items():
        asset["representations"][str(scale)]["decisions"]["silhouette"] = text(en, de)
    for anchor in asset["anchors"]:
        if anchor["id"] in ANCHOR_PAD:
            anchor["x"], anchor["y"], anchor["z"] = ANCHOR_PAD[anchor["id"]]


P.candidate(edit)
if "--publish" in sys.argv:
    P.publish_root(["anchors", "representations", "thermal"], edit)
