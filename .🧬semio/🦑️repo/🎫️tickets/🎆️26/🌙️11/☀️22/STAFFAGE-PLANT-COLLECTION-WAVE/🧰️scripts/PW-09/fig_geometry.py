"""🍈 Fig tree geometry (PW-09): the shipped `japanese-maple` (multi-stem, low-branching) morphed to a Ficus carica of about 5 m.

Design inputs (species data in `../../📝️PW-09.md`): three to five smooth grey stems from a low root collar, a broad, dense, rounded crown
wider than high (about 7 m), leaves of 10 to 25 cm in coarse clusters at the shoot tips.

Stage 1 enlarges every foliage cluster about its own bone (denser, heavier crown with larger leaves); stage 2 is one world-diagonal map of the
whole tree (broader than high); stage 3 thickens every leaf mass vertically about its own height (`DOME`), so the flat tiers of the maple close
into the dense, rounded dome of a fig.

@see ./morph.py
"""
from morph import Morph, WorldAffine, inflate

TEMPLATE = "japanese-maple"
FOLIAGE = 1.22
STEMS = {"root-plate": (1.0, 1.25), "stem-c": (1.0, 1.3), "stem-w": (1.0, 1.0), "stem-e": (1.0, 1.0)}
A = [1.12, 0.98, 1.1]
DOME = 1.6
WOOD_DENSITY = 800 / 900


def morph(asset):
    bones = {b["id"]: (FOLIAGE, FOLIAGE) for b in asset["rig"]["bones"] if b["id"].startswith("fol-")}
    bones.update(STEMS)
    density = {b["id"]: (WOOD_DENSITY if "section" in b else 1.0) for b in asset["rig"]["bones"]}
    m = Morph(asset, k=1.0, bones=bones, density=density)
    m.apply()
    WorldAffine(asset, A).apply()
    inflate(asset, DOME, is_leafy, [b["id"] for b in asset["rig"]["bones"] if b["id"].startswith("fol-")])
    return m


def is_leafy(x):
    return str(x.get("bone", "")).startswith("fol-") or "leaf" in str(x.get("material", "")) + str(x.get("region", ""))
