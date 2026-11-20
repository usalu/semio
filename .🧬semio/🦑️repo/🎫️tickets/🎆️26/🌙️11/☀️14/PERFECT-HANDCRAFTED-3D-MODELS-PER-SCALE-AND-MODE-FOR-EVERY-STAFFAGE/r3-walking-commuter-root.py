"""🦴 Root fixes of `humans/walking-commuter` (round 3): rig, anchors, poses, clips, dimensions and, when present, the redrawn 2D shapes and the volumes source, written as a candidate asset document; `--commit` writes the catalog file.

Usage: python3 r3-walking-commuter-root.py [--commit]
Reads the untouched catalog copy `🗑️generated/r3-walking-commuter/original.json`, writes `🗑️generated/r3-walking-commuter/🔣️.json`.
Optional inputs: `🗑️generated/r3-walking-commuter/shapes.json` ({scale: {view: [shapes]}}), `r3-walking-commuter-texts.json` (representation decisions), `volumes/humans/walking-commuter.json`.
"""
import glob
import json
import math
import os
import re
import sys

T = os.path.dirname(os.path.abspath(__file__))
G = os.path.join(T, "🗑️generated", "r3-walking-commuter")
CATALOG = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/walking-commuter/🔣️.json")[0]
OWNED = ["dimensions", "materials", "rig", "anchors", "representations", "poses", "clips", "thermal"]
MANIFEST = os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(CATALOG))), "🔣️.json")
BAG_AREA = 0.32
RANGE_END_500 = 900
BONE = {"id": "bone", "title": {"en": "Bone", "de": "Knochen"}, "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": 1900, "source": "authored-estimate"}

LIFT = 0.024
BALL_X = 0.17
ANKLE_OLD = 0.075
ANKLE = ANKLE_OLD + LIFT
SHOULDER_Z = 0.19
BAG_Z = -0.14
TORSO_LEN = 0.495
HEAD_LEN = 0.15
GRIP_ANCHOR = (0.021, 0.0143)
GRIP = {"arm-r": -7.0, "forearm-r": 145.0, "hand-r": 28.0}
L1, L2 = 0.435, 0.425


def scalar(value):
    return not isinstance(value, (dict, list))


def inline(value):
    if isinstance(value, list):
        return all(scalar(item) for item in value) or (all(isinstance(item, list) and all(scalar(leaf) for leaf in item) for item in value) and len(json.dumps(value)) <= 160)
    return False


def dump(value, depth=0, compact=False):
    """🖨️ The file's own style: two-space indent, every list expanded, except inside `volumes` where scalar lists stay inline."""
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict):
        if not value:
            return "{}"
        return "{\n" + ",\n".join(inner + json.dumps(key, ensure_ascii=False) + ": " + dump(item, depth + 1, compact or (depth == 1 and key == "volumes")) for key, item in value.items()) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if compact and inline(value):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(item, depth + 1, compact) for item in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


def r(value, digits=4):
    return round(value, digits)


def leg(bones, side):
    """🦵 Re-solves one leg of a pose so that the sole keeps its place under the raised ankle."""
    keys = [f"{part}-{side}" for part in ("thigh", "shank", "foot")]
    if keys[0] not in bones:
        return
    dt, ds, df = (math.radians(bones[k]["angleDeg"]) if k in bones else 0.0 for k in keys)
    phi = dt + ds + df
    ax = L1 * math.sin(dt) + L2 * math.sin(dt + ds) + LIFT * (-math.sin(phi))
    ay = -L1 * math.cos(dt) - L2 * math.cos(dt + ds) + LIFT * (math.cos(phi) - 1)
    d = min(math.hypot(ax, ay), L1 + L2 - 1e-9)
    knee = math.acos(max(-1, min(1, (L1 * L1 + L2 * L2 - d * d) / (2 * L1 * L2))))
    hip = math.acos(max(-1, min(1, (L1 * L1 + d * d - L2 * L2) / (2 * L1 * d))))
    nt = math.atan2(ax, -ay) + hip
    ns = -(math.pi - knee)
    bones[keys[0]] = {**bones[keys[0]], "angleDeg": r(math.degrees(nt))}
    bones[keys[1]] = {**bones.get(keys[1], {}), "angleDeg": r(math.degrees(ns))}
    bones[keys[2]] = {**bones.get(keys[2], {}), "angleDeg": r(math.degrees(phi - nt - ns))}


def bones_fix(bones, walking):
    for side in ("l", "r"):
        leg(bones, side)
    if walking or "arm-r" in bones:
        pass
    return bones


def apply(asset):
    if all(m["id"] != "bone" for m in asset["materials"]):
        asset["materials"].append(BONE)
    rig = {b["id"]: b for b in asset["rig"]["bones"]}
    rig["pelvis"]["y"] = r(rig["pelvis"]["y"] + LIFT)
    rig["torso"]["lengthM"] = TORSO_LEN
    rig["neck"]["x"] = TORSO_LEN
    shift = rig["head"]["lengthM"] - HEAD_LEN
    rig["head"]["lengthM"] = HEAD_LEN
    rig["head"]["mass"]["comX"] = r(rig["head"]["mass"]["comX"] - shift)
    foot_angle = math.degrees(math.atan2(BALL_X, ANKLE))
    foot_len = math.hypot(BALL_X, ANKLE)
    old = math.radians(66.2)
    new = math.radians(foot_angle)

    def refoot(x, y, lift):
        wx = x * math.sin(old) + y * math.cos(old)
        wy = -x * math.cos(old) + y * math.sin(old) - lift
        return r(wx * math.sin(new) - wy * math.cos(new)), r(wx * math.cos(new) + wy * math.sin(new))

    for side in ("l", "r"):
        foot, toe = rig[f"foot-{side}"], rig[f"toe-{side}"]
        foot["angleDeg"] = r(foot_angle)
        foot["lengthM"] = r(foot_len)
        foot["mass"]["comX"], foot["mass"]["comY"] = refoot(foot["mass"]["comX"], foot["mass"]["comY"], 0)
        toe["x"] = r(foot_len)
        toe["angleDeg"] = r(90 - foot_angle)
        rig[f"arm-{side}"]["z"] = SHOULDER_Z if side == "l" else -SHOULDER_Z
    bag = rig["bag"]
    rebuilt = {}
    for key, value in bag.items():
        rebuilt[key] = value
        if key == "y":
            rebuilt["z"] = BAG_Z
    bag.clear()
    bag.update(rebuilt)
    for anchor in asset["anchors"]:
        if anchor["id"].startswith("heel-"):
            anchor["x"], anchor["y"] = refoot(anchor["x"], anchor["y"], LIFT)
        if anchor["id"].startswith("ball-"):
            anchor["x"] = r(foot_len)
        if anchor["bone"] == "head":
            anchor["x"] = r(anchor["x"] - shift)
        if anchor["id"] == "strap-grip":
            anchor["x"], anchor["y"] = GRIP_ANCHOR
    walking = {"contact-left", "loading-left", "passing-left", "contact-right", "loading-right", "passing-right"}
    for pose in asset["poses"]:
        for side in ("l", "r"):
            leg(pose["bones"], side)
        if pose["id"] in walking or pose["id"] == "standing":
            for bone, angle in GRIP.items():
                pose["bones"][bone] = {**pose["bones"].get(bone, {}), "angleDeg": angle}
    for clip in asset["clips"]:
        for key in clip["keys"]:
            if "bones" not in key:
                continue
            for side in ("l", "r"):
                leg(key["bones"], side)
            if clip["id"] == "walk-cycle":
                for bone, angle in GRIP.items():
                    if bone in key["bones"]:
                        key["bones"][bone] = {**key["bones"][bone], "angleDeg": angle}
    for region in asset["thermal"]["regions"]:
        if region["id"] == "bag-leather":
            region["areaM2"] = BAG_AREA
    dims = os.path.join(T, "r3-walking-commuter-dimensions.json")
    if os.path.exists(dims):
        asset["dimensions"].update(json.load(open(dims, encoding="utf8")))
    shapes = os.path.join(G, "shapes.json")
    if os.path.exists(shapes):
        drawn = json.load(open(shapes, encoding="utf8"))
        for scale, views in drawn.items():
            for view, items in views.items():
                asset["representations"][scale]["geometry"][view] = items
    texts = os.path.join(T, "r3-walking-commuter-texts.json")
    if os.path.exists(texts):
        for scale, decisions in json.load(open(texts, encoding="utf8")).get("representations", {}).items():
            asset["representations"][scale]["decisions"].update(decisions)
    asset["representations"]["500"]["validRange"]["maxDenominator"] = RANGE_END_500
    source = os.path.join(T, "volumes", "humans", "walking-commuter.json")
    if "--with-volumes" in sys.argv and os.path.exists(source):
        asset["volumes"] = json.load(open(source, encoding="utf8"))["volumes"]


def main():
    raw = open(os.path.join(G, "original.json"), encoding="utf8").read()
    doc = json.loads(raw)
    if dump(doc) + "\n" != raw:
        print("[DEBUG] style does not round-trip the original", len(dump(doc)) + 1, len(raw))
    before = json.loads(raw)
    apply(doc["asset"])
    out = os.path.join(G, "🔣️.json")
    open(out, "w", encoding="utf8").write(dump(doc) + "\n")
    print(out)
    if "--commit" in sys.argv:
        live_raw = open(CATALOG, encoding="utf8").read()
        live = json.loads(live_raw)
        last = os.path.join(G, "committed.json")
        mine = json.load(open(last, encoding="utf8")) if os.path.exists(last) else before
        for member in OWNED:
            if live["asset"][member] not in (before["asset"][member], mine["asset"][member], doc["asset"][member]):
                raise SystemExit(f"catalog member {member} changed under us; merge by hand")
        for member in OWNED:
            live["asset"][member] = doc["asset"][member]
        text = dump(live) + "\n"
        open(CATALOG, "w", encoding="utf8").write(text)
        open(last, "w", encoding="utf8").write(json.dumps({"asset": {member: doc["asset"][member] for member in OWNED}}, ensure_ascii=False))
        print("committed", CATALOG, len(text))
        manifest = open(MANIFEST, encoding="utf8").read()
        at = manifest.index('"id": "walking-commuter",\n      "category": "humans"')
        end = manifest.index('"powerW"', at)
        row = manifest[at:end]
        for key in ("heightM", "widthM"):
            row = re.sub(r'("' + key + r'": )[0-9.]+', lambda m: m.group(1) + json.dumps(doc["asset"]["dimensions"][key]), row)
        if row != manifest[at:end]:
            open(MANIFEST, "w", encoding="utf8").write(manifest[:at] + row + manifest[end:])
            print("manifest row updated")


main()
