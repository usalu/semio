"""🦴 standing-adult: the symmetric rig. Adds the lumbar spine between pelvis and thorax, a clavicle (shoulder girdle) per side between thorax and upper arm, the left hand with the same fifteen finger bones as the right; binds poses, clips and 2D shapes to it.

`apply(asset)` changes the asset in place and returns True; it does nothing when the rig already has `lumbar`.
"""
import copy
import math

LUMBAR_M = 0.17
STERNOCLAVICULAR = (0.2247, -0.0773, 0.02)
LUMBAR_SHARE = 0.4
FINGERS = ["thumb-metacarpal", "thumb-proximal", "thumb-distal"] + [f"{f}-{p}" for f in ["index", "middle", "ring", "little"] for p in ["proximal", "middle", "distal"]]
RELAXED_LEG = {"thigh": -2.5, "shank": -2.5, "foot": 5}
MITTEN_SCALES = ("200", "500")
MITTEN_SPAN_M = {"200": (0.016, 0.016), "500": (0.0535, 0.022)}
MITTEN_OUTLINE = {"200": [(-0.015, -0.027), (0.05, -0.039), (0.115, -0.037), (0.17, -0.026), (0.18, 0), (0.17, 0.026), (0.115, 0.038), (0.05, 0.04), (-0.015, 0.027)], "500": [(-0.01, -0.044), (0.17, -0.044), (0.182, 0), (0.17, 0.044), (-0.01, 0.044)]}
SHRUG_DEG = {"arms-crossed": 7, "hands-warming": 6}


def left_title(title):
    return {"en": title["en"].replace("Right", "Left").replace("right", "left"), "de": title["de"].replace("Rechter", "Linker").replace("Rechtes", "Linkes").replace("Rechte", "Linke").replace("rechter", "linker").replace("rechtes", "linkes").replace("rechte", "linke")}


def mirror_offset(offset):
    out = dict(offset)
    for key in ("abductionDeg", "twistDeg"):
        if key in out:
            out[key] = -out[key]
    return out


def split_spine(bones):
    if "torso" not in bones or "lumbar" in bones:
        return
    total = bones["torso"].get("angleDeg", 0)
    bones["lumbar"] = {"angleDeg": round(total * LUMBAR_SHARE, 3)}
    bones["torso"] = {**bones["torso"], "angleDeg": round(total * (1 - LUMBAR_SHARE), 3)}


def left_fingers(bones):
    for finger in FINGERS:
        if f"{finger}-r" in bones:
            bones[f"{finger}-l"] = mirror_offset(bones[f"{finger}-r"])


def shrug(bones, deg):
    bones["clavicle-l"] = {"twistDeg": -deg}
    bones["clavicle-r"] = {"twistDeg": deg}
    if "arm-l" in bones:
        bones["arm-l"] = {**bones["arm-l"], "abductionDeg": bones["arm-l"].get("abductionDeg", 0) - deg}
    if "arm-r" in bones:
        bones["arm-r"] = {**bones["arm-r"], "abductionDeg": bones["arm-r"].get("abductionDeg", 0) + deg}


def shift_shape(shape, dx):
    kind = shape["kind"]
    if kind == "poly":
        shape["points"] = [[round(p[0] + dx, 5), p[1]] for p in shape["points"]]
    elif kind == "capsule":
        shape["x0"], shape["x1"] = round(shape["x0"] + dx, 5), round(shape["x1"] + dx, 5)
    elif kind == "ellipse":
        shape["cx"] = round(shape["cx"] + dx, 5)
    elif kind == "rect":
        shape["x"] = round(shape["x"] + dx, 5)
    else:
        raise ValueError(f"shape kind {kind} of {shape['id']} is not shifted")


def mirror_shape(shape, z):
    out = copy.deepcopy(shape)
    out["id"] = shape["id"][:-2] + "-l"
    out["bone"] = shape["bone"][:-2] + "-l"
    out["z"] = z
    solid = out["solid"]
    solid["z0"], solid["z1"] = round(-shape["solid"]["z1"], 6), round(-shape["solid"]["z0"], 6)
    return out


def apply(asset):
    rig = asset["rig"]
    by_id = {b["id"]: b for b in rig["bones"]}
    if "lumbar" in by_id:
        return False
    torso, arm_l, arm_r, hand_l, hand_r = by_id["torso"], by_id["arm-l"], by_id["arm-r"], by_id["hand-l"], by_id["hand-r"]
    whole = torso["mass"]
    lumbar_kg, lumbar_com = 8.638, (0.093, -0.02)
    thorax_kg = round(whole["kg"] - lumbar_kg, 3)
    thorax_x = (whole["kg"] * whole["comX"] - lumbar_kg * lumbar_com[0]) / thorax_kg - LUMBAR_M
    thorax_y = (whole["kg"] * whole["comY"] - lumbar_kg * lumbar_com[1]) / thorax_kg
    lumbar = {"id": "lumbar", "parent": "pelvis", "title": {"en": "Lumbar spine and abdomen", "de": "Lendenwirbelsäule und Bauch"}, "x": torso["x"], "y": torso["y"], "angleDeg": torso["angleDeg"], "lengthM": LUMBAR_M, "joint": {"type": "revolute", "minDeg": -25, "maxDeg": 15}, "mass": {"kg": lumbar_kg, "comX": lumbar_com[0], "comY": lumbar_com[1], "distribution": "point", "source": "literature", "reference": "de-leva-1996"}}
    shoulder = (arm_l["x"] - LUMBAR_M, arm_l["y"])
    torso.update({"parent": "lumbar", "title": {"en": "Thorax with sweatshirt", "de": "Brustkorb mit Sweatshirt"}, "x": LUMBAR_M, "y": 0, "angleDeg": 0, "lengthM": round(torso["lengthM"] - LUMBAR_M, 5), "joint": {"type": "revolute", "minDeg": -15, "maxDeg": 10}, "mass": {**whole, "kg": thorax_kg, "comX": round(thorax_x, 4), "comY": round(thorax_y, 4)}})
    by_id["neck"]["x"] = torso["lengthM"]
    dx, dy = shoulder[0] - STERNOCLAVICULAR[0], shoulder[1] - STERNOCLAVICULAR[1]
    clavicle_deg, clavicle_m = math.degrees(math.atan2(dy, dx)), math.hypot(dx, dy)
    clavicles = []
    for side, arm, name in ((1, arm_l, ("Left", "Linkes")), (-1, arm_r, ("Right", "Rechtes"))):
        suffix = "l" if side > 0 else "r"
        twist = {"twistMinDeg": -30, "twistMaxDeg": 10} if side > 0 else {"twistMinDeg": -10, "twistMaxDeg": 30}
        clavicles.append({"id": f"clavicle-{suffix}", "parent": "torso", "title": {"en": f"{name[0]} clavicle (shoulder girdle)", "de": f"{name[1]} Schlüsselbein (Schultergürtel)"}, "x": STERNOCLAVICULAR[0], "y": STERNOCLAVICULAR[1], "z": side * STERNOCLAVICULAR[2], "angleDeg": round(clavicle_deg, 4), "lengthM": round(clavicle_m, 5), "joint": {"type": "revolute", "minDeg": -5, "maxDeg": 5, **twist}})
        lateral = arm["z"] - side * STERNOCLAVICULAR[2]
        arm.update({"parent": f"clavicle-{suffix}", "x": round(clavicle_m, 5), "y": 0, "z": round(lateral, 5), "angleDeg": round(arm["angleDeg"] - clavicle_deg, 4)})
    hand_l["lengthM"] = hand_r["lengthM"]
    hand_l["mass"] = copy.deepcopy(hand_r["mass"])
    fingers_l = []
    for finger in FINGERS:
        right = by_id[f"{finger}-r"]
        left = copy.deepcopy(right)
        left["id"] = f"{finger}-l"
        left["parent"] = right["parent"][:-2] + "-l"
        left["title"] = left_title(right["title"])
        if "z" in left:
            left["z"] = -right["z"]
        joint = left["joint"]
        if "twistMinDeg" in joint:
            joint["twistMinDeg"], joint["twistMaxDeg"] = -right["joint"]["twistMaxDeg"], -right["joint"]["twistMinDeg"]
        fingers_l.append(left)
    ordered = []
    for bone in rig["bones"]:
        if bone["id"] == "torso":
            ordered.append(lumbar)
        if bone["id"] in ("arm-l", "arm-r"):
            ordered.append(clavicles[0] if bone["id"] == "arm-l" else clavicles[1])
        ordered.append(bone)
        if bone["id"] == "hand-l":
            ordered.extend(fingers_l)
    rig["bones"] = ordered

    for pose in asset["poses"]:
        bones = pose["bones"]
        split_spine(bones)
        left_fingers(bones)
        for side in ("l", "r"):
            for part, deg in RELAXED_LEG.items():
                bones[f"{part}-{side}"] = {**bones.get(f"{part}-{side}", {}), "angleDeg": deg}
        if pose["id"] in SHRUG_DEG:
            shrug(bones, SHRUG_DEG[pose["id"]])
    poses = {p["id"]: p for p in asset["poses"]}
    for clip in asset["clips"]:
        for key in clip["keys"]:
            bones = key.get("bones")
            if bones is None:
                continue
            split_spine(bones)
            base = poses[key["pose"]]["bones"] if "pose" in key else {}
            for side, sign in (("l", -1), ("r", 1)):
                arm = f"arm-{side}"
                if arm in bones and key.get("pose") in SHRUG_DEG and "abductionDeg" in bones[arm]:
                    bones[arm] = {**bones[arm], "abductionDeg": base[arm]["abductionDeg"]}

    for scale, rep in asset["representations"].items():
        if rep.get("status") != "authored":
            continue
        for view, shapes in rep["geometry"].items():
            for shape in shapes:
                if shape["bone"] == "torso" and (view == "elevation" or "solid" in shape):
                    shift_shape(shape, -LUMBAR_M)
            right = [s for s in shapes if s["bone"] == "hand-r" or (s["bone"].endswith("-r") and s["bone"][:-2] in FINGERS)]
            old = [i for i, s in enumerate(shapes) if s["bone"] == "hand-l"]
            at = old[0] if old else shapes.index(right[0])
            z = shapes[at]["z"] if old else right[0]["z"] - 1
            left = [mirror_shape(s, round(z + 0.001 * k, 4)) for k, s in enumerate(right)]
            kept = [s for s in shapes if s["bone"] != "hand-l"]
            kept[at:at] = left
            if scale in MITTEN_SCALES:
                kept = [s for s in kept if s["bone"][:-2] not in FINGERS]
                for shape in kept:
                    if shape["bone"] in ("hand-l", "hand-r"):
                        side = 1 if shape["bone"] == "hand-l" else -1
                        medial, lateral = MITTEN_SPAN_M[scale]
                        shape["points"] = [list(p) for p in MITTEN_OUTLINE[scale]]
                        faces = sorted([side * (0.153 - medial), side * (0.153 + lateral)])
                        shape["solid"]["z0"], shape["solid"]["z1"] = round(faces[0], 4), round(faces[1], 4)
            rep["geometry"][view] = kept
    return True
