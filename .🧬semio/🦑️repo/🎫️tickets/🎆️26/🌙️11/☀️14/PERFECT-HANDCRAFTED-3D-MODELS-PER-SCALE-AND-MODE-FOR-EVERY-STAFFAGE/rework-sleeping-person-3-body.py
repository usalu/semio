"""🛌 Hand-chosen body of humans/sleeping-person under the duvet (second round, pass 1, continuation).

Usage: python3 rework-sleeping-person-3-body.py candidate|publish
candidate writes 🗑️generated/rework-sleeping-person/body/🔣️.json (catalog document with the edited volumes);
publish writes the solids into volumes/humans/sleeping-person.json (then volumes-insert.py splices the catalog).
Frames: a loft station is (u along the bone, v, w, half width along w, half depth along v, exponent).
Pelvis: u towards the head from the hip line, v posterior. Torso: the same. Thigh and shank: u towards the foot, v anterior.
"""
import copy
import json
import os
import sys

T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/sleeping-person/🔣️.json"
SOURCE = T + "/volumes/humans/sleeping-person.json"
CANDIDATE = T + "/🗑️generated/rework-sleeping-person/body/🔣️.json"
ACROSS = [0, -0.70711, 0, 0.70711]
PYJAMA = {"material": "cotton-underwear", "role": "body", "tone": 0.62}
SKIN = {"material": "skin", "role": "body", "tone": 0.82}

PELVIS = {
    "50": [(-0.108, 0.02, 0.045, 0.028, 2.2), (-0.098, 0.02, 0.105, 0.054, 2.3), (-0.075, 0.016, 0.148, 0.072, 2.4), (-0.04, 0.011, 0.172, 0.082, 2.5), (0, 0.008, 0.18, 0.085, 2.6), (0.04, 0.0055, 0.17, 0.0875, 2.6), (0.1, 0, 0.14, 0.0925, 2.5)],
    "100": [(-0.105, 0.02, 0.06, 0.035, 2.2), (-0.08, 0.017, 0.142, 0.07, 2.4), (-0.03, 0.01, 0.176, 0.083, 2.5), (0.04, 0.0055, 0.17, 0.0875, 2.6), (0.1, 0, 0.14, 0.0925, 2.5)],
    "200": [(-0.1, 0.02, 0.08, 0.045, 2.2), (-0.06, 0.014, 0.16, 0.078, 2.4), (0.02, 0.007, 0.178, 0.086, 2.5), (0.1, 0, 0.14, 0.0925, 2.5)],
    "500": [(-0.095, 0.018, 0.1, 0.055, 2.2), (-0.02, 0.009, 0.178, 0.084, 2.4), (0.1, 0, 0.14, 0.0925, 2.5)],
}
TRUNK_LOW = {
    "100": [(-0.02, 0.0005, 0.14, 0.0925, 2.5), (0.08, 0.003, 0.13, 0.09, 2.5), (0.2, -0.01, 0.145, 0.103, 2.4)],
    "200": [(-0.02, 0.0005, 0.14, 0.0925, 2.5), (0.1, 0.002, 0.131, 0.091, 2.5), (0.22, -0.012, 0.148, 0.105, 2.4)],
    "500": [(-0.02, 0.0005, 0.14, 0.0925, 2.5), (0.14, -0.002, 0.136, 0.095, 2.4)],
}
THIGH_HEAD = [(-0.07, 0, 0.03, 0.03, 2), (-0.055, 0, 0.06, 0.058, 2), (-0.03, 0, 0.08, 0.076, 2)]
SHORTS_HEAD = [(-0.074, 0, 0.032, 0.032, 2), (-0.058, 0, 0.063, 0.061, 2), (-0.032, 0, 0.083, 0.079, 2)]
KNEE_END = [(0.445, 0.004, 0.042, 0.04, 2), (0.462, 0.004, 0.022, 0.02, 2)]
SHANK_HEAD = [(-0.052, 0.004, 0.024, 0.022, 2), (-0.04, 0.004, 0.04, 0.038, 2)]
THIGH = {
    "100": [(-0.065, 0, 0.04, 0.038, 2), (-0.03, 0, 0.08, 0.076, 2), (0.06, 0, 0.092, 0.088, 2.1), (0.22, 0.003, 0.078, 0.075, 2.2), (0.4, 0.004, 0.052, 0.052, 2), (0.45, 0.004, 0.036, 0.034, 2)],
    "200": [(-0.06, 0, 0.05, 0.048, 2), (0.03, 0, 0.092, 0.088, 2.1), (0.24, 0.003, 0.075, 0.072, 2.2), (0.42, 0.004, 0.05, 0.05, 2), (0.455, 0.004, 0.03, 0.03, 2)],
    "500": [(-0.05, 0, 0.06, 0.058, 2), (0.05, 0, 0.092, 0.088, 2.1), (0.44, 0.004, 0.05, 0.05, 2)],
}
SHANK = {
    "100": [(-0.045, 0.004, 0.034, 0.032, 2), (0, 0, 0.051, 0.051, 2), (0.09, -0.008, 0.055, 0.057, 2), (0.22, -0.004, 0.045, 0.044, 2), (0.345, 0, 0.034, 0.033, 2)],
    "200": [(-0.04, 0.003, 0.04, 0.04, 2), (0.09, -0.008, 0.055, 0.057, 2), (0.345, 0, 0.035, 0.034, 2)],
    "500": [(-0.03, 0.002, 0.046, 0.046, 2), (0.1, -0.007, 0.054, 0.056, 2), (0.345, 0, 0.0375, 0.0375, 2)],
}


def station(row):
    u, v, half_width, half_depth, exponent = row
    return {"at": [u, v, 0], "rx": half_width, "ry": half_depth, "rotation": ACROSS, "exponent": exponent}


def loft(identity, bone, rows, look):
    return {"id": identity, "kind": "loft", "bone": bone, **look, "sections": [station(r) for r in rows], "capStart": True, "capEnd": True}


def put(solids, solid, after=None):
    """🪛 Replace the solid of the same id, or insert the new one after the solid called `after`."""
    for i, old in enumerate(solids):
        if old["id"] == solid["id"]:
            solids[i] = solid
            return
    at = next((i + 1 for i, old in enumerate(solids) if old["id"] == after), len(solids))
    solids.insert(at, solid)


def find(solids, identity):
    return next((s for s in solids if s["id"] == identity), None)


def extend(solid, head, tail):
    """🦵 Rounded ends in front of and behind the stations a limb already has (idempotent by station position)."""
    rows = solid["sections"]
    first, last = rows[0]["at"][0], rows[-1]["at"][0]
    solid["sections"] = [station(r) for r in head if r[0] < first - 1e-6] + rows + [station(r) for r in tail if r[0] > last + 1e-6]


def mirrored(solid, identity, bone):
    twin = copy.deepcopy(solid)
    twin["id"], twin["bone"] = identity, bone
    for row in twin.get("sections", []):
        row["at"][2] = -row["at"][2]
    for key in ("p0", "p1", "center", "centre"):
        if key in twin:
            twin[key][2] = -twin[key][2]
    if "transform" in twin and "translation" in twin["transform"]:
        twin["transform"]["translation"][2] = -twin["transform"]["translation"][2]
    return twin


def fine(body):
    put(body, loft("shorts", "pelvis", PELVIS["50"], PYJAMA))
    for side in ("l", "r"):
        extend(find(body, "thigh-" + side), THIGH_HEAD, KNEE_END)
        extend(find(body, "shorts-leg-" + side), SHORTS_HEAD, [])
        extend(find(body, "shank-" + side), SHANK_HEAD, [])


def coarse(scale, body):
    trunk = find(body, "trunk") or find(body, "shoulders")
    first = trunk["sections"][0]["at"][0]
    trunk["sections"] = [station(r) for r in TRUNK_LOW[scale] if r[0] < first - 0.03] + trunk["sections"]
    put(body, loft("shorts", "pelvis", PELVIS[scale], PYJAMA), after=trunk["id"])
    for side in ("l", "r"):
        put(body, loft("thigh-" + side, "thigh-" + side, THIGH[scale], SKIN), after="shorts" if side == "l" else "thigh-l")
        put(body, loft("shank-" + side, "shank-" + side, SHANK[scale], SKIN), after="thigh-" + side)
    for part in ("hand", "thumb"):
        right = find(body, part + "-r")
        if right:
            left = mirrored(right, part + "-l", "hand-l")
            for row in left.get("sections", []) if scale == "500" else []:
                row["ry"] = max(row["ry"], 0.038)
            put(body, left, after="foresleeve-l")


def edit(volumes):
    fine(volumes["50"]["body"])
    for scale in ("100", "200", "500"):
        coarse(scale, volumes[scale]["body"])


def main(mode):
    source = json.load(open(SOURCE))
    edit(source["volumes"])
    if mode == "publish":
        json.dump(source, open(SOURCE, "w"), ensure_ascii=False)
        return
    document = json.load(open(CATALOG))
    for scale, lists in source["volumes"].items():
        document["asset"]["volumes"][scale] = lists
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    json.dump(document, open(CANDIDATE, "w"), ensure_ascii=False)


main(sys.argv[1] if len(sys.argv) > 1 else "candidate")
