"""🦴 Stage 1 of the seated reader: lumbar spine as its own bone, shoulder and pelvic girdle on the lateral rest placement (`rig-lateral-rest.md`).

Woman of 1.68 m. Hand-chosen numbers (metres, frames of the parent bone, u along the bone, v back, w left):
- `lumbar` L5/S1 to T12/L1 0.17 m (the lower 0.17 m of the former rigid 0.477 m torso), `torso` (thorax) T12/L1 to C7/T1 0.307 m.
- `clavicle-l|r` sternoclavicular joint (21 mm beside the midline on the manubrium, 110 mm ahead of the spine line) to the acromioclavicular joint (157 mm out, 55 mm ahead of the spine line, 19 mm higher): 0.148 m.
- `ilium-l|r` sacroiliac joint (35 mm out, 60 mm above and 50 mm behind the hip axis) to the hip joint centre (85 mm out): 0.093 m.
Upper arms and thighs are re-parented without moving. A lean of the trunk is shared 65 % lumbar, 35 % thorax when bending forward and is all lumbar when reclining (the back then keeps the place the backrest contact was authored for).

Run: `python3 rework-seated-reader-1-rig.py` writes the candidate, `--apply` writes the volumes source and the catalog members `rig`, `anchors`, `poses`, `clips`, `representations`.
"""
import copy
import importlib.util
import json
import math
import os
import re
import sys

from rework_03_lib import T, catalog_path, source_path, read

spec = importlib.util.spec_from_file_location("rig_lateral", os.path.join(T, "rig-lateral-standing-adult.py"))
rig = importlib.util.module_from_spec(spec)
spec.loader.exec_module(rig)

A = "humans/seated-reader"
CANDIDATE = os.path.join(T, "🗑️generated", "rework-seated-reader", "🔣️.json")
LUMBAR = 0.17
WAIST_TOP, CHEST_BOTTOM = 0.2, 0.135
WAIST_HEIGHT = 0.768
SPLIT_SCALES = ("50", "100", "200")
FEATURE = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}
ACROSS = [0, -0.70711, 0, 0.70711]
GIRDLES = {
    "clavicle": {"parent": "torso", "limb": "arm", "from": (0.251, -0.11, 0.021), "to": (0.27, -0.055, 0.157), "low": -8, "high": 35, "solids": ("clavicle", "scapula", "acromion", "shoulder-bar"), "bar": "shoulder-bar", "drop": "shoulder-bar",
                 "title": ("{} clavicle with scapula (sternoclavicular to acromioclavicular joint)", "Schlüsselbein {} mit Schulterblatt (Sternoklavikular- bis Akromioklavikulargelenk)")},
    "ilium": {"parent": "pelvis", "limb": "thigh", "from": (0.06, 0.05, 0.035), "to": (0.0, 0.0, 0.085), "low": -5, "high": 5, "solids": ("ilium", "pubis", "ischium", "hip-bar"), "bar": "hip-bar", "drop": "pelvic-bar",
              "title": ("{} hip bone (sacroiliac joint to hip joint)", "Hüftbein {} (Iliosakralgelenk bis Hüftgelenk)")},
}
SIDES = {"l": (1, "Left", "links"), "r": (-1, "Right", "rechts")}


def share(angle):
    """📐 The lean of the trunk as (lumbar, thorax) offsets."""
    return (round(angle * 0.65, 3), round(angle * 0.35, 3)) if angle < 0 else (angle, 0)


def between(sections, x):
    """✂️ The section of a loft at the station `x` along u."""
    ordered = sorted(sections, key=lambda s: s["at"][0])
    for a, b in zip(ordered, ordered[1:]):
        if a["at"][0] <= x <= b["at"][0]:
            k = (x - a["at"][0]) / (b["at"][0] - a["at"][0])
            out = copy.deepcopy(a)
            out["at"] = [round(a["at"][i] + k * (b["at"][i] - a["at"][i]), 5) for i in range(3)]
            for key in ("rx", "ry", "exponent"):
                if key in a:
                    out[key] = round(a[key] + k * (b.get(key, a[key]) - a[key]), 5)
            return out
    raise SystemExit(f"no section at {x}")


def shift(solid, dx):
    """➡️ Moves a torso solid into the frame of the thorax bone."""
    if "transform" in solid or solid["kind"] not in ("loft", "capsule"):
        transform = solid.setdefault("transform", {})
        t = transform.get("translation", [0, 0, 0])
        transform["translation"] = [round(t[0] - dx, 5), t[1], t[2]]
    elif solid["kind"] == "loft":
        for section in solid["sections"]:
            section["at"] = [round(section["at"][0] - dx, 5), section["at"][1], section["at"][2]]
    else:
        solid["p0"] = [round(solid["p0"][0] - dx, 5), *solid["p0"][1:]]
        solid["p1"] = [round(solid["p1"][0] - dx, 5), *solid["p1"][1:]]


def parted(solid, lower_id, top, bottom, inset):
    """🪚 A loft over the whole trunk as a waist part (lumbar frame) and a chest part (still in the old frame)."""
    sections = sorted(solid["sections"], key=lambda s: s["at"][0])
    lower, upper = copy.deepcopy(solid), copy.deepcopy(solid)
    lower["id"], lower["bone"] = lower_id, "lumbar"
    lower["sections"] = [copy.deepcopy(s) for s in sections if s["at"][0] < top - 0.02] + [between(sections, top)]
    start = between(sections, bottom)
    start["rx"], start["ry"] = round(start["rx"] - inset, 5), round(start["ry"] - inset, 5)
    upper["sections"] = [start] + [copy.deepcopy(s) for s in sections if s["at"][0] > bottom + 0.02]
    return lower, upper


def clip(points, axis, limit, keep_below):
    """✂️ A polygon cut by the line `axis = limit` (Sutherland-Hodgman)."""
    inside = (lambda p: p[axis] <= limit) if keep_below else (lambda p: p[axis] >= limit)
    out = []
    for i, p in enumerate(points):
        q = points[(i + 1) % len(points)]
        if inside(p):
            out.append([round(p[0], 4), round(p[1], 4)])
        if inside(p) != inside(q):
            k = (limit - p[axis]) / (q[axis] - p[axis])
            out.append([round(p[0] + k * (q[0] - p[0]), 4), round(p[1] + k * (q[1] - p[1]), 4)])
    return out


def world(frame, p):
    """🌍 A point of a bone frame in world coordinates."""
    return [frame[0][i] + rig.apply(frame[1], p)[i] for i in range(3)]


def rebind(solid, old, new):
    """🔁 The same world solid written in the frame of another bone."""
    turn = rig.mul(rig.transpose(new[1]), old[1])
    q = rig.quaternion(turn)

    def point(p):
        return rig.r5(rig.apply(rig.transpose(new[1]), [world(old, p)[i] - new[0][i] for i in range(3)]))

    if "transform" in solid or solid["kind"] == "ellipsoid":
        transform = solid.setdefault("transform", {})
        transform["translation"] = point(transform.get("translation", [0, 0, 0]))
        transform["rotation"] = rig.r5(rig.product(q, transform.get("rotation", [0, 0, 0, 1])))
    elif solid["kind"] == "loft":
        for section in solid["sections"]:
            section["at"] = point(section["at"])
            if "rotation" in section:
                section["rotation"] = rig.r5(rig.product(q, section["rotation"]))
            elif abs(section["rx"] - section["ry"]) > 1e-9:
                raise SystemExit(f"{solid['id']}: a flat section without a rotation cannot be re-expressed")
    else:
        raise SystemExit(f"{solid['id']}: kind {solid['kind']} is not converted")


def rod(solid_id, bone, points, member=True):
    """🦴 A round bone through stations (x, y, z, r) of its bone frame."""
    out = {"id": solid_id, "kind": "loft", "bone": bone, "material": "bone", "role": "body", "tone": 0.62, "sections": [{"at": [round(c, 5) for c in p[:3]], "rx": p[3], "ry": p[3]} for p in points], "capStart": True, "capEnd": True, "member": bone}
    out["part"] = "member" if member else "joint"
    return out


def vertebrae(solid_id, bone, curve, start, count, pitch, wide, deep, origin):
    """🦴 A column of vertebral bodies with their discs: end plates wider than the waist of each body, discs narrower, along the authored curve of the spine."""
    sections = []
    for i in range(count):
        k = i / max(count - 1, 1)
        rx, ry = wide[0] + k * (wide[1] - wide[0]), deep[0] + k * (deep[1] - deep[0])
        for along, grow in ((0.08, 1.0), (0.42, 0.86), (0.76, 1.0), (0.92, 0.7)):
            x = start + (i + along) * pitch
            y = between(curve, min(max(x, curve[0]["at"][0]), curve[-1]["at"][0]))["at"][1]
            sections.append({"at": [round(x - origin, 5), round(y, 5), 0], "rx": round(rx * grow, 5), "ry": round(ry * grow, 5), "rotation": ACROSS})
    sections = sections[:-1]
    return {"id": solid_id, "kind": "loft", "bone": bone, "material": "bone", "role": "body", "tone": 0.62, "sections": sections, "capStart": True, "capEnd": True, "member": bone, "part": "member"}


def processes(prefix, bone, curve, start, count, pitch, reach, origin):
    """🦴 The spinous processes: one short spur per vertebra from the arch back and down."""
    out = []
    for i in range(count):
        x = start + (i + 0.5) * pitch
        y = between(curve, min(max(x, curve[0]["at"][0]), curve[-1]["at"][0]))["at"][1]
        k = i / max(count - 1, 1)
        r = reach[0] + k * (reach[1] - reach[0])
        out.append(rod(f"{prefix}-{i + 1}", bone, [(x - origin, y + 0.012, 0, 0.0055), (x - origin - 0.35 * r, y + 0.012 + r, 0, 0.004)]))
    return out


def convert(a):
    """🛠️ The whole stage on an asset whose `volumes` are the source."""
    bones = a["rig"]["bones"]
    if any(b["id"] == "lumbar" for b in bones):
        raise SystemExit("already converted")
    torso = next(b for b in bones if b["id"] == "torso")
    lumbar = {"id": "lumbar", "parent": "pelvis", "title": {"en": "Lumbar spine and abdomen (L5/S1 to T12/L1)", "de": "Lendenwirbelsäule und Bauch (L5/S1 bis Th12/L1)"}, "x": torso["x"], "y": torso["y"], "angleDeg": torso["angleDeg"], "lengthM": LUMBAR, "joint": {"type": "revolute", "minDeg": -45, "maxDeg": 20}, "mass": {"kg": 9.23, "comX": 0.085, "comY": 0, "distribution": "point", "source": "literature", "reference": "de-leva-1996"}}
    bones.insert(bones.index(torso), lumbar)
    torso.update({"parent": "lumbar", "title": {"en": "Thorax with sweater (T12/L1 to C7/T1)", "de": "Brustkorb mit Pullover (Th12/L1 bis C7/Th1)"}, "x": LUMBAR, "y": 0, "angleDeg": 0, "lengthM": round(torso["lengthM"] - LUMBAR, 4), "joint": {"type": "revolute", "minDeg": -25, "maxDeg": 15}})
    torso["mass"].update({"kg": 9.73, "comX": 0.173})
    for bone in bones:
        if bone["parent"] == "torso":
            bone["x"] = round(bone["x"] - LUMBAR, 4)
    for anchor in a["anchors"]:
        if anchor["bone"] == "torso":
            anchor["x"] = round(anchor["x"] - LUMBAR, 4)

    def keys(offsets):
        if "torso" in offsets:
            low, high = share(offsets["torso"]["angleDeg"])
            fresh = {}
            for key, value in offsets.items():
                if key == "torso":
                    fresh["lumbar"] = {"angleDeg": low}
                    fresh["torso"] = {"angleDeg": high}
                else:
                    fresh[key] = value
            offsets.clear()
            offsets.update(fresh)

    for pose in a["poses"]:
        keys(pose["bones"])
    for clip_ in a["clips"]:
        for key in clip_["keys"]:
            keys(key.get("bones", {}))

    for scale, rep in a["representations"].items():
        geometry = rep.get("geometry", {})
        for view, shapes in geometry.items():
            fresh = []
            for shape in shapes:
                if shape["bone"] == "torso" and shape["id"] == "sweater-hem":
                    shape["bone"] = "lumbar"
                elif shape["bone"] == "torso" and view == "elevation":
                    if shape["kind"] != "poly":
                        raise SystemExit(f"{scale} {shape['id']}: elevation shape kind {shape['kind']}")
                    if shape["id"] == "sweater" and scale in SPLIT_SCALES:
                        waist = copy.deepcopy(shape)
                        waist.update({"id": "sweater-waist", "bone": "lumbar", "z": round(shape["z"] - 0.02, 3), "points": clip(shape["points"], 0, WAIST_TOP, True)})
                        fresh.append(waist)
                        shape["points"] = clip(shape["points"], 0, CHEST_BOTTOM, False)
                    shape["points"] = [[round(p[0] - LUMBAR, 4), p[1]] for p in shape["points"]]
                elif shape["bone"] == "torso" and view == "front" and shape["id"] == "sweater" and scale in SPLIT_SCALES:
                    waist = copy.deepcopy(shape)
                    waist.update({"id": "sweater-waist", "bone": "lumbar", "z": round(shape["z"] - 0.02, 3), "points": clip(shape["points"], 1, WAIST_HEIGHT + 0.03, True)})
                    fresh.append(waist)
                    shape["points"] = clip(shape["points"], 1, WAIST_HEIGHT - 0.035, False)
                fresh.append(shape)
            geometry[view] = fresh

    curve = sorted(copy.deepcopy(next(s for s in a["volumes"]["50"]["structure"] if s["id"] == "spine")["sections"]), key=lambda s: s["at"][0])
    for scale, lists in a["volumes"].items():
        for name in ("body", "thermal", "structure"):
            fresh = []
            for solid in lists[name]:
                if solid.get("bone") != "torso":
                    if solid.get("of") == "sweater" and scale in SPLIT_SCALES:
                        twin = copy.deepcopy(solid)
                        twin["id"], twin["of"] = solid["id"].replace("sweater", "sweater-waist", 1), "sweater-waist"
                        fresh.append(twin)
                    fresh.append(solid)
                    continue
                if solid["id"].endswith("sweater-hem") or solid["id"] == "sweater-hem":
                    solid["bone"] = "lumbar"
                    fresh.append(solid)
                    continue
                if solid.get("part") == "joint":
                    twin = copy.deepcopy(solid)
                    twin.update({"id": "joint-lumbar", "bone": "lumbar", "member": "lumbar"})
                    fresh += [twin, solid]
                    continue
                if solid.get("part") == "mass":
                    twin = copy.deepcopy(solid)
                    twin.update({"id": "mass-lumbar", "bone": "lumbar", "member": "lumbar"})
                    twin["transform"]["translation"] = [0.085, 0, 0]
                    solid["transform"]["translation"] = [0.173, 0, 0]
                    fresh += [twin, solid]
                    continue
                if solid["id"] == "spine":
                    if scale == "50":
                        fresh.append(vertebrae("lumbar-spine", "lumbar", curve, -0.004, 5, 0.0352, (0.024, 0.021), (0.018, 0.016), 0))
                        fresh += processes("lumbar-process", "lumbar", curve, -0.004, 5, 0.0352, (0.03, 0.03), 0)
                        fresh.append(vertebrae("spine", "torso", curve, 0.168, 12, 0.02575, (0.019, 0.013), (0.015, 0.011), LUMBAR))
                        fresh += processes("thoracic-process", "torso", curve, 0.168, 12, 0.02575, (0.03, 0.022), LUMBAR)
                        continue
                    low = copy.deepcopy(solid)
                    low.update({"id": "lumbar-spine", "bone": "lumbar", "member": "lumbar"})
                    low["sections"] = [s for s in solid["sections"] if s["at"][0] < LUMBAR - 0.01] + [between(solid["sections"], LUMBAR + 0.004)]
                    solid["sections"] = [between(solid["sections"], LUMBAR - 0.004)] + [s for s in solid["sections"] if s["at"][0] > LUMBAR + 0.01]
                    shift(solid, LUMBAR)
                    fresh += [low, solid]
                    continue
                if solid["kind"] == "loft" and "transform" not in solid and scale in SPLIT_SCALES and min(s["at"][0] for s in solid["sections"]) < 0.08 and max(s["at"][0] for s in solid["sections"]) > 0.3:
                    low, solid = parted(solid, solid["id"].replace("sweater", "sweater-waist", 1), WAIST_TOP, CHEST_BOTTOM, 0.003)
                    fresh.append(low)
                shift(solid, LUMBAR)
                fresh.append(solid)
            lists[name] = fresh

    old = rig.frames(copy.deepcopy(bones))
    for name, g in GIRDLES.items():
        parent = old[g["parent"]]
        for side, (sign, en, de) in SIDES.items():
            start = world(parent, [g["from"][0], g["from"][1], sign * g["from"][2]])
            end = world(parent, [g["to"][0], g["to"][1], sign * g["to"][2]])
            u = [end[i] - start[i] for i in range(3)]
            length = round(math.sqrt(sum(c * c for c in u)), 5)
            placement = rig.placed(parent, start, u, (-1.0 * sign, 0.0, 0.0))
            bone = {"id": f"{name}-{side}", "parent": g["parent"], "title": {"en": g["title"][0].format(en), "de": g["title"][1].format(de)}, **placement, "lengthM": length, "joint": {"type": "revolute", "minDeg": g["low"], "maxDeg": g["high"]}}
            limb = next(b for b in bones if b["id"] == f"{g['limb']}-{side}")
            bones.insert(bones.index(limb), bone)
            now = rig.frames(bones)
            hung = rig.fields(now[bone["id"]], old[limb["id"]][0], old[limb["id"]][1])
            limb["parent"] = bone["id"]
            for key in ("z", "abductionDeg", "twistDeg"):
                limb.pop(key, None)
            limb.update({k: v for k, v in hung.items() if k not in ("abductionDeg", "twistDeg") or abs(v) > 1e-9})
            ordered = {k: limb[k] for k in ("id", "parent", "title", "x", "y", "z", "angleDeg", "abductionDeg", "twistDeg", "lengthM", "joint") if k in limb}
            ordered.update({k: v for k, v in limb.items() if k not in ordered})
            limb.clear()
            limb.update(ordered)
    new = rig.frames(bones)
    for bone_id, (origin, basis) in old.items():
        drift = max(abs(new[bone_id][0][i] - origin[i]) for i in range(3))
        turned = max(abs(new[bone_id][1][i][j] - basis[i][j]) for i in range(3) for j in range(3))
        if drift > 2e-5 or turned > 2e-5:
            raise SystemExit(f"{bone_id} moved by {drift} m / {turned}")
    for name, g in GIRDLES.items():
        for scale, lists in a["volumes"].items():
            lists["structure"] = [s for s in lists["structure"] if s["id"] != g["drop"] or s["id"].endswith(("-l", "-r"))]
        for side in SIDES:
            bone_id = f"{name}-{side}"
            length = next(b for b in bones if b["id"] == bone_id)["lengthM"]
            for scale, lists in a["volumes"].items():
                members = [s for s in lists["structure"] if s.get("bone") == g["parent"] and s["id"] in {f"{stem}-{side}" for stem in g["solids"]}]
                for solid in members:
                    rebind(solid, old[g["parent"]], new[bone_id])
                    solid["bone"] = solid["member"] = bone_id
                at = next(i for i, s in enumerate(lists["structure"]) if s.get("bone") == g["parent"]) + 1
                if scale == "500":
                    r = round(FEATURE[scale] / 2 + 0.0002, 5)
                    lists["structure"].insert(at, rod(f"{g['bar']}-{side}", bone_id, [(0, 0, 0, r), (length, 0, 0, r)]))
                if scale in ("50", "100"):
                    r = round(FEATURE[scale] * 1.2, 5)
                    lists["structure"].insert(at, {"id": f"joint-{bone_id}", "kind": "ellipsoid", "bone": bone_id, "material": "bone", "role": "body", "tone": 0.62, "radii": [r, r, r], "transform": {"translation": [0, 0, 0]}, "member": bone_id, "part": "joint"})
    return a


def spliced(text, key, before, after):
    """🧵 One member of the asset replaced in the catalog text: in its own layout where that is known, else in the layout of `volumes-insert.py`."""
    try:
        return rig.splice(text, key, before, after)
    except SystemExit:
        insert = rig.insert_module()
        indent = re.search(r'\n( +)"id"\s*:', text).group(1)
        start, end = insert.member_range(text, indent, key)
        return text[:start] + indent + json.dumps(key) + ": " + insert.dump(after, len(indent) // 2) + text[end:]


def main():
    """🚀 Candidate, or with `--apply` the source and the catalog members."""
    catalog = catalog_path(A)
    original = text = open(catalog, encoding="utf8").read()
    document = json.loads(text)
    before = copy.deepcopy(document["asset"])
    source_text = open(source_path(A), encoding="utf8").read()
    document["asset"]["volumes"] = json.loads(source_text)["volumes"]
    convert(document["asset"])
    for bone in document["asset"]["rig"]["bones"]:
        if bone["id"] in ("lumbar", "torso", "clavicle-l", "arm-l", "ilium-l", "thigh-l", "clavicle-r", "arm-r"):
            print("[DEBUG]", json.dumps({k: v for k, v in bone.items() if k not in ("title", "mass", "joint")}))
    if "--apply" not in sys.argv:
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        json.dump(document, open(CANDIDATE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
        print(CANDIDATE)
        return
    for key in ("rig", "anchors", "poses", "clips", "representations"):
        text = spliced(text, key, before[key], document["asset"][key])
    expected = copy.deepcopy(document)
    expected["asset"]["volumes"] = before["volumes"]
    if json.loads(text) != expected:
        raise SystemExit("splice did not round-trip")
    if open(catalog, encoding="utf8").read() != original or open(source_path(A), encoding="utf8").read() != source_text:
        raise SystemExit("a file changed while converting: run again")
    open(catalog, "w", encoding="utf8").write(text)
    json.dump({"volumes": document["asset"]["volumes"]}, open(source_path(A), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    open(source_path(A), "a", encoding="utf8").write("\n")
    print("applied")


if __name__ == "__main__":
    main()
