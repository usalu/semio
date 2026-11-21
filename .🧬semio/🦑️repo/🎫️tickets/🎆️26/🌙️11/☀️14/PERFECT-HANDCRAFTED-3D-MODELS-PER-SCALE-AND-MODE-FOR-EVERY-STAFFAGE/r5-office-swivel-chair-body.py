"""🪑 Round 5, furniture/office-swivel-chair: the round-3 handcrafted body with the round-5 corrections stated by hand.

The backrest carrier cover is a moulded polyamide cap over the flat-steel strut (tapered, domed, rounded ends) instead of a box; the 1:50 and 1:100 bodies carry it, the coarser scales keep the bare strut.
@see ./r3-office-swivel-chair-body.py
@see ./r5-brief.md
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("chair_body_r3", os.path.join(HERE, "r3-office-swivel-chair-body.py"))
R3 = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(R3)
globals().update({k: v for k, v in vars(R3).items() if not k.startswith("__")})

COVER = {50: [(0.500, -0.2990, 0.030, 0.0060), (0.506, -0.2992, 0.041, 0.0110), (0.520, -0.2995, 0.046, 0.0135), (0.600, -0.3000, 0.050, 0.0145), (0.680, -0.2998, 0.047, 0.0135), (0.694, -0.2996, 0.042, 0.0110), (0.700, -0.2995, 0.031, 0.0060)],
         100: [(0.502, -0.2992, 0.036, 0.0080), (0.520, -0.2995, 0.046, 0.0135), (0.680, -0.2998, 0.047, 0.0135), (0.698, -0.2996, 0.036, 0.0080)]}


def cover(scale):
    """🛡 The carrier cover: horizontal domed sections (y, centre x, half width, half depth) up the strut, closed by rounded ends."""
    return K.loft("backrest-bracket", "backrest", PA, "body", 0.35, [((x, y, 0), hw, hd, LEFT, AHEAD) for y, x, hw, hd in COVER[scale]], 3.0)


def _with_cover(solids, scale):
    return [cover(scale) if s["id"] == "backrest-bracket" else s for s in solids]


def body50():
    return _with_cover(R3.body50(), 50)


def body100():
    return _with_cover(R3.body100(), 100)
