"""🫕 Round 5 cooking pot: the round-3 volumes with the round-5 corrections (bubbles on their own sinking bone so that the cold pot shows a still surface, 1:500 handles as loops with a round grip end instead of square planks) and the root fixes (bubbles bone, pose offsets, plan bubbles rebound).

Usage: python3 r5-cooking-pot-model.py [--apply]   (always writes the volumes source and the candidate; --apply writes rig, poses and representations into the catalog)
See ./r5-brief.md, ./r3-objects-cooking-pot.md.
"""
import copy
import math
import sys

import r5_o02_lib as L

V = L.module("r3-cooking-pot-volumes.py")
K = V.K
K.ORIGIN["bubbles"] = K.FILL

SINK = 0.012
SIMMER = 0.003
GRIP_R = 0.025
V.LUG_500 = [(0.07726, 0.025), (0.1, 0.025)] + [(0.1 + GRIP_R * math.cos(a), GRIP_R * math.sin(a)) for a in [math.pi / 2 - math.pi * k / 8 for k in range(1, 8)]] + [(0.1, -0.025), (0.07726, -0.025), (0.08023, -0.0125), (0.0812, 0.0), (0.08023, 0.0125)]

BUBBLES_BONE = {
    "id": "bubbles",
    "parent": "surface",
    "x": 0,
    "y": 0,
    "angleDeg": 0,
    "title": {"en": "Boiling bubbles, rise through the surface when the water boils", "de": "Siedeblasen, steigen beim Kochen durch die Oberfläche"},
    "lengthM": 0,
    "joint": {"type": "prismatic", "axisDeg": 0, "minM": -SINK, "maxM": 0},
}


def volumes():
    """🫧 Every scale of the round-3 model with the round-5 changes."""
    out = V.volumes()
    for s in out["50"]["body"]:
        if s["id"].startswith("bubble-"):
            s["bone"] = "bubbles"
    out["50"]["decisions"][0]["en"] += " The bubbles sit on their own bone that sinks them 12 mm into the water in the cold pose and lets only their tops break through while heating."
    out["50"]["decisions"][0]["de"] += " Die Blasen sitzen an einem eigenen Knochen, der sie in der kalten Pose 12 mm ins Wasser absenkt und beim Aufheizen nur ihre Kuppen durchbrechen lässt."
    d = out["500"]["decisions"][0]
    d["en"] = d["en"].replace("two square 50 x 10 mm lugs for the handles", "two 50 x 10 mm lugs with a round grip end for the loop handles").replace("lugs as members", "lugs as members")
    d["de"] = d["de"].replace("zwei eckige 50 x 10 mm große Laschen für die Griffe", "zwei 50 x 10 mm große Laschen mit rundem Griffende für die Bügelgriffe")
    return out


def roots(asset):
    """🦴 Adds the bubbles bone, its pose offsets and rebinds the plan bubbles of 1:50."""
    bones = [b for b in asset["rig"]["bones"] if b["id"] != "bubbles"]
    asset["rig"]["bones"] = bones + [copy.deepcopy(BUBBLES_BONE)]
    for pose in asset["poses"]:
        pose["bones"].pop("bubbles", None)
        if pose["id"] == "cold":
            pose["bones"]["bubbles"] = {"dx": -SINK}
        if pose["id"] == "heating":
            pose["bones"]["bubbles"] = {"dx": -SIMMER}
    for shape in asset["representations"]["50"]["geometry"]["plan"]:
        if shape["id"].startswith("bubble-"):
            shape["bone"] = "bubbles"
    for view in asset["representations"]["200"]["geometry"].values():
        for shape in view:
            if shape["id"] in PRINT_200:
                shape["penMm"] = 0.2
    return asset


PRINT_200 = {"base", "base-end", "wall-far", "water"}


if __name__ == "__main__":
    vols = volumes()
    print(L.write_source("cooking-pot", vols))
    doc = L.catalog("cooking-pot")
    roots(doc["asset"])
    doc["asset"]["volumes"] = vols
    print(L.write_candidate("cooking-pot", doc))
    if "--apply" in sys.argv:
        print("written:", L.apply_members("cooking-pot", doc["asset"], ["rig", "poses", "representations"]))
