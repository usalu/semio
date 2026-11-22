"""🌲 Stone pine geometry (PW-09): the shipped `scots-pine` morphed to a Pinus pinea of about 20 m with an umbrella crown.

Design inputs (species data in `../../📝️PW-09.md`): a long clear bole of about 0.7 m that forks at 12 to 14 m into a few heavy, steeply
rising limbs; a dense, flat-topped umbrella crown about 13 to 14 m across and only about 6 to 7 m deep; plate-like red-brown bark.

Stage 1 lengthens and thickens the bole (the trunk bones are vertical, so a local scale is exact) and enlarges the needle pads; stage 2a scales the
whole tree to the height, stage 2b flattens and widens the crown subtree from the top of the bole (the needle pads are carried to their new
places but keep their own shape, so they stay dense discs) (the upper trunk bones narrow in w by
stage 1 so that the stage 2b widening keeps them round).

@see ./morph.py
"""
from morph import Morph, WorldAffine

TEMPLATE = "scots-pine"
CROWN_A = [1.1, 0.55, 1.4]
PADS = ["pl-w3", "pl-e3", "pl-w1", "pl-e1", "pl-w2", "pl-e2", "pl-top"]
WIDEN = CROWN_A[0] / CROWN_A[2]
BONES = {
    "root-plate": (1.0, 1.3, 1.3),
    "trunk-1": (1.5, 1.3, 1.3),
    "trunk-2": (1.0, 1.15, 1.15 * WIDEN),
    "trunk-3": (1.0, 1.1, 1.1 * WIDEN),
    "leader": (1.0, 1.0, WIDEN),
    "pl-w3": (1.25, 1.25), "pl-e3": (1.25, 1.25), "pl-w1": (1.3, 1.3), "pl-e1": (1.3, 1.3),
    "pl-w2": (1.3, 1.3), "pl-e2": (1.3, 1.3), "pl-top": (1.35, 1.35),
}
WOOD_DENSITY = 1.0


def crown_symbol(item):
    return item.get("id") in {"crown"} or str(item.get("id", "")).startswith("scaffold-")


def morph(asset):
    m = Morph(asset, k=1.0, bones=BONES)
    m.apply()
    WorldAffine(asset, [1.0, 1.0, 1.0]).apply()
    WorldAffine(asset, CROWN_A, pivot="trunk-2", extra=crown_symbol, keep=PADS).apply()
    return m
