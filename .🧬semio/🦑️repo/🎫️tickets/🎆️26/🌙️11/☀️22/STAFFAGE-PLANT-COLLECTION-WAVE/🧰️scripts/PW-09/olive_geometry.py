"""🫒 Olive tree geometry (PW-09): the shipped `apple-tree` morphed to an old Olea europaea of about 7 m.

Design inputs (species data in `../../📝️PW-09.md`): a massive, short, fluted stem of about 0.7 m diameter that forks at 1.4 m into
heavy, spreading scaffolds; a crown wider than high (about 8 m) with a low, rounded top; silvery foliage; drupes of 2 to 3.5 cm.

@see ./morph.py
"""
from morph import Morph, shrink

TEMPLATE = "apple-tree"
K = 1.10
BONES = {
    "root-plate": (1.0, 1.75),
    "trunk": (0.85, 1.85),
    "ld": (0.95, 1.45),
    "ld-b": (0.8, 1.15),
    "sc-w1a": (1.25, 1.25), "sc-e1a": (1.25, 1.25),
    "sc-w1b": (1.2, 1.2), "sc-e1b": (1.2, 1.2),
    "sc-w2a": (1.18, 1.18), "sc-e2a": (1.18, 1.18),
    "sc-w2b": (1.12, 1.12), "sc-e2b": (1.12, 1.12),
    "spur-w1": (1.2, 1.2), "spur-e1": (1.2, 1.2), "spur-w2": (1.12, 1.12), "spur-e2": (1.12, 1.12),
    "fol-core-low": (1.18, 1.18), "fol-core-up": (1.1, 1.1),
    "fol-w1": (1.2, 1.2), "fol-e1": (1.2, 1.2), "fol-w2": (1.15, 1.15), "fol-e2": (1.15, 1.15),
    "fol-top": (1.0, 1.0),
}
CROWN = [1.2, 1.15, 1.25]
WOOD_DENSITY = 1100 / 900
FRUIT = {"50": 0.45, "100": 0.5, "200": 0.6, "500": 1.0}


def is_fruit(x):
    return x.get("region") == "fruit" or str(x.get("material", "")).endswith("-fruit")


LEAFY = ("leaf", "fruit")
CROWN_IDS = {"crown"}


def is_crown(x):
    return x.get("id") in CROWN_IDS or str(x.get("bone", "")).startswith("fol-") or any(w in str(x.get("material", "")) + str(x.get("region", "")) for w in LEAFY)


def morph(asset):
    density = {b["id"]: (WOOD_DENSITY if "section" in b else 1.0) for b in asset["rig"]["bones"]}
    crown = {"bone": "ld", "A": CROWN, "bones": [b["id"] for b in asset["rig"]["bones"] if b["id"].startswith("fol-")]}
    m = Morph(asset, k=K, bones=BONES, density=density, crown=crown, is_blended=is_crown)
    m.apply()
    shrink(asset, m, is_fruit, FRUIT)
    return m
