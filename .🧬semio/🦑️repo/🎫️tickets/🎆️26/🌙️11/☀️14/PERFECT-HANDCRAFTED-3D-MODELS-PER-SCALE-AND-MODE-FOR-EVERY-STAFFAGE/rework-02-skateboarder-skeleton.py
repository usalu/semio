"""🦴️ Hand corrections of the skeleton of `humans/skateboarder` where bones did not meet: temporal bones carry the mandible at the
temporomandibular joints, the patella lies on the femoral condyles, the fibula has its head on the lateral tibial condyle and its
malleolus on the talus, the coarse sternum reaches the sternoclavicular joints and the coarse heel stands on the deck.
`apply(asset)` is called by `rework-02-skateboarder-rig.py`; all coordinates are bone-local metres (u along the bone, v, w to the left).
"""
import copy
import importlib.util
import os

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib02", os.path.join(T, "rework-02-lib.py"))
lib = importlib.util.module_from_spec(spec)
spec.loader.exec_module(lib)
BONE = {"material": "bone", "tone": 0.9}


def fibula(solid, sign, scale):
    """🦵️ Fibula: head, neck, shaft and lateral malleolus (station, v, w, radius)."""
    shaft = 0.006 if scale == 50 else 0.0077
    rows = [(0.03, -0.010, 0.022, 0.011), (0.062, -0.013, 0.026, shaft), (0.25, -0.012, 0.025, shaft), (0.40, -0.009, 0.028, shaft), (0.44, -0.006, 0.026, 0.0095)]
    template = solid["sections"][0]
    solid["sections"] = [{**copy.deepcopy(template), "at": [u, v, sign * w], "rx": r, "ry": r} for u, v, w, r in rows]


def apply(asset):
    """🔨️ Applies the corrections to the structure lists."""
    for scale in (50, 100, 200, 500):
        structure = asset["volumes"][str(scale)]["structure"]
        by = {s["id"]: s for s in structure}
        for side, sign in (("l", 1), ("r", -1)):
            if f"patella-{side}" in by:
                by[f"patella-{side}"]["transform"]["translation"][1] = 0.0395
            if f"fibula-{side}" in by and scale in (50, 100):
                fibula(by[f"fibula-{side}"], sign, scale)
            if scale in (50, 100) and "mandible" in by:
                at = next(i for i, s in enumerate(structure) if s["id"] == "mandible")
                structure.insert(at, lib.capsule(f"temporal-{side}", "head", BONE["material"], BONE["tone"], [0.022, -0.028, sign * 0.054], [-0.003, -0.039, sign * 0.048], 0.009, 0.008, member="head", part="member"))
            if scale in (200, 500):
                by[f"clavicle-{side}"]["p0"] = [-0.0225, 0, 0]
            if scale == 200:
                heel = by[f"tarsus-{side}"]["sections"][0]
                heel["at"], heel["rx"], heel["ry"] = [-0.0188, -0.082, 0], 0.02, 0.02
        if scale in (200, 500):
            sternum = by["open-sternum"]
            sternum["sections"].insert(0, {**copy.deepcopy(sternum["sections"][0]), "at": [0.413, -0.066, 0]})
