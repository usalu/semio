"""🦴️ Jogger, second round pass 1: the trunk of the rig on the vertebral column (lumbar `torso`, thoracic `chest`, inclined `neck`).

Usage: python3 rework-jogger-rig.py <base asset document> <candidate out> [--publish]
The base is the catalog document before this pass (one `torso` bone from the lumbar joint to C7 through the middle of the chest).
Every solid, 2D shape, anchor, pose and clip key is re-bound so that its rest placement in the world stays where it was; the
vertebral column of the structure lists is moved onto the new bones (bodies 32 mm behind the old axis, lumbar vertebrae 38.5 mm,
thoracic 25.8 mm high). All numbers are stated here by hand; frames: trunk u up, +v back, +w the figure's left.
"""
import copy
import importlib.util
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(HERE, "rework-02-lib.py"))
lib = importlib.util.module_from_spec(spec)
spec.loader.exec_module(lib)

ASSET = "humans/jogger"
J = 0.1925
VB = 0.032
C7 = 0.51
OLD_T12 = 0.15
OLD_C7 = 0.502
HEAD = (0.095, 0.016)
NECK_DEG = math.degrees(math.atan2(-(HEAD[1] - VB), HEAD[0]))
NECK_LEN = round(math.hypot(HEAD[0], HEAD[1] - VB), 4)
TONGUE = (0.015, 0.03)
TUCK = 0.003
LEAN_SHARE = 0.6
FOLD_SHARE = 0.7
WIDTH = 0.984
WAIST_SCALES = ("50", "100")
CHEST_IDS = ("process-t", "sternum", "costal", "clavicle", "scapula", "glenoid", "rib", "collar", "thoracic")
SPINE_FREE = ("scapula", "clavicle", "glenoid", "sternum", "costal", "joint-", "mass-")


def r5(x):
    return round(x, 5)


def remap(u):
    """📏️ Height of a vertebral level: lumbar vertebrae stretched to their real height, thoracic compressed."""
    if u <= OLD_T12:
        return u * J / OLD_T12
    if u <= OLD_C7:
        return J + (u - OLD_T12) * (OLD_C7 - J) / (OLD_C7 - OLD_T12)
    return u


def setback(u):
    """📏️ How far the vertebral bodies lie behind the old trunk axis at a height."""
    if u <= 0.15:
        return 0.035
    if u >= 0.30:
        return 0.02
    return 0.035 - 0.015 * (u - 0.15) / 0.15


def to_column(p, weight):
    return [r5(p[0] + (remap(p[0]) - p[0]) * weight), r5(p[1] + setback(p[0]) * weight), p[2]]


def near_column(p):
    return max(0.0, 1 - abs(p[2]) / 0.07) if p[1] > -0.01 else 0.0


def spine_onto_column(solids):
    """🦴️ Moves vertebrae, processes and rib heads of one structure list onto the vertebral column (old trunk frame)."""
    for s in solids:
        id, bone = s["id"], s.get("bone")
        if bone == "torso" and not any(k in id for k in SPINE_FREE):
            whole = id.startswith("sk-spine") or "process" in id
            if s["kind"] == "capsule":
                for k in ("p0", "p1"):
                    s[k] = to_column(s[k], 1.0 if whole else near_column(s[k]))
            elif s["kind"] == "loft" and whole:
                for sec in s["sections"]:
                    sec["at"] = [sec["at"][0], r5(sec["at"][1] + setback(sec["at"][0])), sec["at"][2]]
        elif bone == "pelvis" and id == "sk-sacrum":
            for sec in s["sections"]:
                sec["at"] = [sec["at"][0], r5(sec["at"][1] + 0.03), sec["at"][2]]
        elif bone == "neck" and id == "sk-cervical":
            for sec in s["sections"]:
                t = (sec["at"][0] + 0.012) / 0.121
                sec["at"] = [sec["at"][0], r5(sec["at"][1] + 0.028 - 0.014 * t), sec["at"][2]]


def between(a, b, u):
    t = (u - a["at"][0]) / (b["at"][0] - a["at"][0])
    out = copy.deepcopy(a)
    out["at"] = [r5(u), r5(a["at"][1] + (b["at"][1] - a["at"][1]) * t), r5(a["at"][2] + (b["at"][2] - a["at"][2]) * t)]
    for k in ("rx", "ry", "exponent"):
        out[k] = r5(a[k] + (b[k] - a[k]) * t)
    return out


def at_station(secs, u):
    for a, b in zip(secs, secs[1:]):
        if a["at"][0] <= u <= b["at"][0]:
            return between(a, b, u)
    raise ValueError(u)


def cut_at_joint(solid, lower_id, tongue):
    """✂️ Cuts a trunk loft at T12/L1; the lumbar part keeps a tucked tongue under the thoracic part so that bending opens no crack."""
    secs = solid["sections"]
    assert secs[0]["at"][0] < J < secs[-1]["at"][0], solid["id"]
    lower, upper = copy.deepcopy(solid), copy.deepcopy(solid)
    lower["id"] = lower_id
    lower["sections"] = [s for s in secs if s["at"][0] < J - 0.004] + [at_station(secs, J)]
    if tongue:
        for du in TONGUE:
            t = at_station(secs, J + du)
            t["rx"], t["ry"] = r5(t["rx"] - TUCK), r5(t["ry"] - TUCK)
            lower["sections"].append(t)
    upper["sections"] = [at_station(secs, J)] + [s for s in secs if s["at"][0] > J + 0.004]
    return upper, lower


def centre_u(s):
    if "sections" in s:
        return (s["sections"][0]["at"][0] + s["sections"][-1]["at"][0]) / 2
    if "p0" in s:
        return (s["p0"][0] + s["p1"][0]) / 2
    return s.get("transform", {}).get("translation", [0, 0, 0])[0]


def trunk_bone(id, u):
    return "chest" if any(k in id for k in CHEST_IDS) or u > J else "torso"


def neck_point(u, v):
    c, s = math.cos(math.radians(-NECK_DEG)), math.sin(math.radians(-NECK_DEG))
    x, y = u, v - VB
    return r5(c * x - s * y), r5(s * x + c * y)


def qmul(a, b):
    ax, ay, az, aw = a
    bx, by, bz, bw = b
    return [r5(aw * bx + ax * bw + ay * bz - az * by), r5(aw * by - ax * bz + ay * bw + az * bx), r5(aw * bz + ax * by - ay * bx + az * bw), r5(aw * bw - ax * bx - ay * by - az * bz)]


NECK_Q = [0, 0, math.sin(math.radians(-NECK_DEG) / 2), math.cos(math.radians(-NECK_DEG) / 2)]


def move(p, bone):
    if bone == "neck":
        u, v = neck_point(p[0], p[1])
        return [u, v, p[2]]
    return [r5(p[0] - (J if bone == "chest" else 0)), r5(p[1] - VB), p[2]]


def rebind(s):
    """🔗️ Re-expresses one solid of the old `torso` or `neck` in its new bone so that it rests where it was."""
    old = s.get("bone")
    if old not in ("torso", "neck"):
        return
    bone = "neck" if old == "neck" else trunk_bone(s["id"], centre_u(s))
    s["bone"] = bone
    if "member" in s:
        s["member"] = bone
    if s.get("part") == "joint":
        return
    if "sections" in s:
        for sec in s["sections"]:
            sec["at"] = move(sec["at"], bone)
            if bone == "neck":
                sec["rotation"] = qmul(NECK_Q, sec.get("rotation", [0, 0, 0, 1]))
    elif "p0" in s:
        s["p0"], s["p1"] = move(s["p0"], bone), move(s["p1"], bone)
    else:
        tr = s.setdefault("transform", {})
        tr["translation"] = move(tr.get("translation", [0, 0, 0]), bone)
        if bone == "neck":
            tr["rotation"] = qmul(NECK_Q, tr.get("rotation", [0, 0, 0, 1]))


def mass_marker(bone, kg, u, floor):
    r = max(floor, r5(0.02204 * (kg / 8.042) ** (1 / 3)))
    return {"id": "mass-" + bone, "kind": "ellipsoid", "bone": bone, "material": "bone", "role": "body", "tone": 0.85, "radii": [r, r, r], "transform": {"translation": [u, -VB, 0]}, "member": bone, "part": "mass"}


def volumes(asset):
    for scale, v in asset["volumes"].items():
        if "body" not in v:
            continue
        spine_onto_column(v["structure"])
        for name in ("body", "thermal", "structure"):
            out = []
            for s in v[name]:
                spans = s.get("bone") == "torso" and s.get("kind") == "loft" and s["sections"][0]["at"][0] < J - 0.03 and s["sections"][-1]["at"][0] > J + 0.05
                if spans and s["id"] == "sk-spine":
                    upper, lower = cut_at_joint(s, "sk-lumbar-spine", False)
                    upper["id"] = "sk-thoracic-spine"
                    out += [lower, upper]
                elif spans and scale in WAIST_SCALES:
                    upper, lower = cut_at_joint(s, s["id"].replace("shirt", "shirt-waist", 1), True)
                    out += [upper, lower]
                elif s.get("of") == "shirt" and scale in WAIST_SCALES:
                    out += [s, {**s, "id": s["id"].replace("shirt", "shirt-waist", 1), "of": "shirt-waist"}]
                else:
                    out.append(s)
            for s in out:
                rebind(s)
            v[name] = out
        structure = []
        for s in v["structure"]:
            if s["id"] == "mass-torso":
                marker = s
                structure += [{**mass_marker("torso", 11.76, 0.105, marker["radii"][0] if scale == "500" else 0), "material": marker["material"], "tone": marker["tone"]}, {**mass_marker("chest", 11.489, r5(0.365 - J), marker["radii"][0] if scale == "500" else 0), "material": marker["material"], "tone": marker["tone"]}]
            elif s["id"] == "joint-torso":
                structure += [s, {**copy.deepcopy(s), "id": "joint-chest", "bone": "chest", "member": "chest"}]
            else:
                structure.append(s)
        v["structure"] = structure


def clip(points, c, keep_below):
    """✂️ Sutherland-Hodgman against the line x = c."""
    out = []
    inside = (lambda p: p[0] <= c) if keep_below else (lambda p: p[0] >= c)
    for a, b in zip(points, points[1:] + points[:1]):
        if inside(a):
            out.append(a)
        if inside(a) != inside(b):
            t = (c - a[0]) / (b[0] - a[0])
            out.append([c, round(a[1] + (b[1] - a[1]) * t, 5)])
    return out


def shift_solid(shape, dv):
    solid = shape["solid"]
    for k in ("z0", "z1"):
        solid[k] = round(solid[k] + dv, 4)
    if "zTip" in solid:
        solid["zTip"] = [round(z + dv, 4) for z in solid["zTip"]]


def drawings(asset):
    """✏️ Re-binds the 2D shapes of the old `torso` and `neck`; the shirt outline is cut at the waist joint like its solid."""
    for scale, rep in asset["representations"].items():
        geometry = rep.get("geometry") or {}
        home = {}
        for view in ("elevation", "front", "plan"):
            out = []
            for shape in geometry.get(view, []):
                bone = shape.get("bone")
                if bone not in ("torso", "neck"):
                    out.append(shape)
                    continue
                local = view == "elevation" or "solid" in shape
                if bone == "neck":
                    if view == "elevation":
                        shape["points"] = [list(neck_point(*p)) for p in shape["points"]]
                    elif "solid" in shape:
                        solid = shape["solid"]
                        mid = (solid["z0"] + solid["z1"]) / 2
                        us = [p[0] for p in shape["points"]]
                        dv = neck_point((min(us) + max(us)) / 2, mid)[1] - mid
                        shape["points"] = [[neck_point(p[0], mid)[0], p[1]] for p in shape["points"]]
                        shift_solid(shape, dv)
                    out.append(shape)
                    continue
                if not local:
                    shape["bone"] = home.get(shape["id"], "chest")
                    out.append(shape)
                    continue
                us = [p[0] for p in shape["points"]]
                if scale in WAIST_SCALES and shape.get("closed", True) and shape.get("filled") and min(us) < J - 0.03 and max(us) > J + 0.05:
                    waist = copy.deepcopy(shape)
                    waist["id"] = shape["id"].replace("shirt", "shirt-waist", 1)
                    waist["z"] = round(shape["z"] - 0.005, 4)
                    waist["points"] = clip(shape["points"], J + TONGUE[1], True)
                    shape["points"] = clip(shape["points"], J, False)
                    parts = [(waist, "torso"), (shape, "chest")]
                else:
                    parts = [(shape, trunk_bone(shape["id"], (min(us) + max(us)) / 2))]
                for part, target in parts:
                    part["bone"] = target
                    home.setdefault(part["id"], target)
                    du = J if target == "chest" else 0
                    if view == "elevation":
                        part["points"] = [[r5(p[0] - du), r5(p[1] - VB)] for p in part["points"]]
                    else:
                        part["points"] = [[r5(p[0] - du), p[1]] for p in part["points"]]
                        shift_solid(part, -VB)
                    out.append(part)
            if view in geometry:
                geometry[view] = sorted(out, key=lambda s: s["z"]) if out != geometry[view] and all("z" in s for s in out) and geometry[view] == sorted(geometry[view], key=lambda s: s["z"]) else out


def rig(asset):
    bones = []
    for b in asset["rig"]["bones"]:
        id = b["id"]
        if id == "torso":
            mass = b["mass"]
            b.update(title={"en": "Lumbar spine and abdomen", "de": "Lendenwirbelsäule und Bauch"}, y=VB, lengthM=J, joint={"type": "revolute", "minDeg": -60, "maxDeg": 25}, mass={**mass, "kg": 11.76, "comX": 0.105, "comY": -VB})
            chest = {"id": "chest", "parent": "torso", "title": {"en": "Thoracic spine and ribcage", "de": "Brustwirbelsäule und Brustkorb"}, "x": J, "y": 0, "angleDeg": 0, "lengthM": round(C7 - J, 4), "joint": {"type": "revolute", "minDeg": -35, "maxDeg": 20}, "mass": {**mass, "kg": 11.489, "comX": round(0.365 - J, 4), "comY": -VB}}
            bones += [b, chest]
            continue
        if id == "neck":
            cx, cy = neck_point(b["mass"]["comX"], b["mass"]["comY"])
            b.update(parent="chest", x=round(C7 - J, 4), y=0, angleDeg=round(NECK_DEG, 4), lengthM=NECK_LEN)
            b["mass"].update(comX=round(cx, 4), comY=round(cy, 4))
        elif id == "head":
            b.update(x=NECK_LEN, y=0, angleDeg=round(-NECK_DEG, 4))
        elif id.startswith("upper-arm"):
            b.update(parent="chest", x=round(b["x"] - J, 4), y=-VB)
        bones.append(b)
    asset["rig"]["bones"] = bones


def split_lean(bones):
    if "torso" in bones and "chest" not in bones:
        angle = bones["torso"]["angleDeg"]
        lumbar = round(angle * (LEAN_SHARE + (FOLD_SHARE - LEAN_SHARE) * min(1, max(0, (abs(angle) - 20) / 40))), 4)
        rebuilt = {}
        for k, v in bones.items():
            rebuilt[k] = v
            if k == "torso":
                rebuilt[k] = {**v, "angleDeg": lumbar}
                rebuilt["chest"] = {"angleDeg": round(angle - lumbar, 4)}
        bones.clear()
        bones.update(rebuilt)


def motion(asset):
    for pose in asset["poses"]:
        split_lean(pose["bones"])
    for c in asset["clips"]:
        for key in c["keys"]:
            if "bones" in key:
                split_lean(key["bones"])
    for pattern in asset["thermal"]["respiration"].get("patterns", []):
        for m in pattern.get("motion", []):
            if m.get("bone") == "torso":
                m["bone"] = "chest"
    for m in asset["thermal"]["respiration"].get("motion", []):
        if m.get("bone") == "torso":
            m["bone"] = "chest"


def main():
    base, out = sys.argv[1], sys.argv[2]
    with open(base, encoding="utf-8") as f:
        document = json.load(f)
    asset = document["asset"]
    assert all(b["id"] != "chest" for b in asset["rig"]["bones"]), "already on the vertebral column"
    volumes(asset)
    drawings(asset)
    rig(asset)
    motion(asset)
    asset["dimensions"]["widthM"] = WIDTH
    os.makedirs(os.path.dirname(out), exist_ok=True)
    with open(out, "w", encoding="utf-8") as f:
        json.dump(document, f, ensure_ascii=False, indent=2)
        f.write("\n")
    if "--publish" in sys.argv:
        current = lib.load_catalog(ASSET)
        with open(base, encoding="utf-8") as f:
            assert current == json.load(f), "the catalog document changed since the base was taken"
        lib.save_source(ASSET, {"volumes": asset["volumes"]})
        lib.save_catalog(ASSET, document)
    print("neck", NECK_DEG, NECK_LEN, {s: {n: len(v[n]) for n in ("body", "thermal", "structure")} for s, v in asset["volumes"].items() if "body" in v})


main()
