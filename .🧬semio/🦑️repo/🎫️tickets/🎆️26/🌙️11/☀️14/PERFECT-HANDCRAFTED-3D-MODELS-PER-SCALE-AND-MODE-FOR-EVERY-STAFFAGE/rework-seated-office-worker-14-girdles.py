"""🦴 Stage 14 of the seated office worker: shoulder and pelvic girdle on the lateral rest placement (`rig-lateral-rest.md`).

`clavicle-l|r` run from the sternoclavicular joint (22 mm beside the midline, on the manubrium) to the acromioclavicular joint (165 mm out, 56 mm further back, 31 mm higher along the thorax): 0.157 m. `ilium-l|r` run from the sacroiliac joint (35 mm out, 55 mm above and behind the hip axis) to the hip joint centre (88 mm out): 0.094 m. Upper arms and thighs are re-parented to them without moving (their world rest frames are checked to 2e-5), so their solids, drawings, anchors, masses, poses and clips stay as they are. The structure solids of each girdle half (clavicle, scapula, acromion, glenoid, shoulder bar; ilium, its body, pubis, ischium, hip bar) are re-expressed in the frame of their bone at every scale and become its members; where a scale has no such solid a bar from joint to joint is stated.

Run: `python3 rework-seated-office-worker-14-girdles.py` (candidate), `--apply` writes the volumes source and `rig` of the catalog file.
"""
import copy
import importlib.util
import math
import os
import sys

from rework_03_lib import *

spec = importlib.util.spec_from_file_location("rig_lateral", os.path.join(T, "rig-lateral-standing-adult.py"))
rig = importlib.util.module_from_spec(spec)
spec.loader.exec_module(rig)

A = "humans/seated-office-worker"
GIRDLES = {
    "clavicle": {"parent": "torso", "limb": "arm", "from": (0.25, -0.106, 0.022), "to": (0.281, -0.05, 0.165), "low": -8, "high": 35, "solids": ("clavicle", "scapula", "acromion", "glenoid", "shoulder-bar"), "bar": "shoulder-bar",
                 "title": ("{} clavicle with scapula (sternoclavicular to acromioclavicular joint)", "Schlüsselbein {} mit Schulterblatt (Sternoklavikular- bis Akromioklavikulargelenk)")},
    "ilium": {"parent": "pelvis", "limb": "thigh", "from": (0.055, 0.055, 0.035), "to": (0.0, 0.0, 0.088), "low": -5, "high": 5, "solids": ("ilium", "ilium-body", "pubis", "ischium", "hip-bar"), "bar": "hip-bar",
              "title": ("{} hip bone (sacroiliac joint to hip joint)", "Hüftbein {} (Iliosakralgelenk bis Hüftgelenk)")},
}
SIDES = {"l": (1, "Left", "links"), "r": (-1, "Right", "rechts")}
FEATURE = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}


def world(frame, p):
    """🌍 A point of a bone frame in world coordinates."""
    return [frame[0][i] + rig.apply(frame[1], p)[i] for i in range(3)]


def rebind(solid, old, new):
    """🔁 The same world solid written in the frame of another bone."""
    turn = rig.mul(rig.transpose(new[1]), old[1])
    q = rig.quaternion(turn)

    def point(p):
        return rig.r5(rig.apply(rig.transpose(new[1]), [world(old, p)[i] - new[0][i] for i in range(3)]))

    if solid["kind"] == "loft":
        for section in solid["sections"]:
            section["at"] = point(section["at"])
            if "rotation" in section:
                section["rotation"] = rig.r5(rig.product(q, section["rotation"]))
            elif abs(section["rx"] - section["ry"]) > 1e-9:
                raise SystemExit(f"{solid['id']}: a flat section without a rotation cannot be re-expressed")
    elif solid["kind"] == "ellipsoid":
        transform = solid.setdefault("transform", {})
        transform["translation"] = point(transform.get("translation", [0, 0, 0]))
        transform["rotation"] = rig.r5(rig.product(q, transform.get("rotation", [0, 0, 0, 1])))
    elif solid["kind"] == "capsule":
        solid["p0"], solid["p1"] = point(solid["p0"]), point(solid["p1"])
    else:
        raise SystemExit(f"{solid['id']}: kind {solid['kind']} is not converted")


doc = read(catalog_path(A))
a = doc["asset"]
a["volumes"] = read(source_path(A))["volumes"]
bones = a["rig"]["bones"]
if any(b["id"] == "clavicle-l" for b in bones):
    raise SystemExit("the girdles are already on their bones")
old = rig.frames(copy.deepcopy(bones))
for name, g in GIRDLES.items():
    parent = old[g["parent"]]
    for side, (sign, en, de) in SIDES.items():
        start = world(parent, [g["from"][0], g["from"][1], sign * g["from"][2]])
        end = world(parent, [g["to"][0], g["to"][1], sign * g["to"][2]])
        u = [end[i] - start[i] for i in range(3)]
        length = round(math.sqrt(sum(c * c for c in u)), 5)
        fields = rig.placed(parent, start, u, (-1.0 * sign, 0.0, 0.0))
        bone = {"id": f"{name}-{side}", "parent": g["parent"], "title": {"en": g["title"][0].format(en), "de": g["title"][1].format(de)}, **fields, "lengthM": length, "joint": {"type": "revolute", "minDeg": g["low"], "maxDeg": g["high"]}}
        limb = next(b for b in bones if b["id"] == f"{g['limb']}-{side}")
        bones.insert(bones.index(limb), bone)
        now = rig.frames(bones)
        placement = rig.fields(now[bone["id"]], old[limb["id"]][0], old[limb["id"]][1])
        limb["parent"] = bone["id"]
        for key in ("z", "abductionDeg", "twistDeg"):
            limb.pop(key, None)
        limb.update({k: v for k, v in placement.items() if k not in ("abductionDeg", "twistDeg") or abs(v) > 1e-9})
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
    for side in SIDES:
        bone_id = f"{name}-{side}"
        length = next(b for b in bones if b["id"] == bone_id)["lengthM"]
        for scale, lists in a["volumes"].items():
            members = [s for s in lists["structure"] if s.get("bone") == g["parent"] and s["id"] in {f"{stem}-{side}" for stem in g["solids"]}]
            for solid in members:
                rebind(solid, old[g["parent"]], new[bone_id])
                solid["bone"] = solid["member"] = bone_id
            if not members:
                r = round(FEATURE[scale] / 2 + 0.0002, 5)
                at = next(i for i, s in enumerate(lists["structure"]) if s.get("bone") == g["parent"]) + 1
                lists["structure"].insert(at, {"id": f"{g['bar']}-{side}", "kind": "loft", "bone": bone_id, "material": "bone", "role": "body", "tone": 0.62, "sections": [{"at": [0, 0, 0], "rx": r, "ry": r, "rotation": [0.5, 0.5, 0.5, 0.5]}, {"at": [length, 0, 0], "rx": r, "ry": r, "rotation": [0.5, 0.5, 0.5, 0.5]}], "capStart": True, "capEnd": True, "member": bone_id, "part": "member"})
            print("[DEBUG]", scale, bone_id, [s["id"] for s in members] or "bar stated")
for bone_id in ("clavicle-l", "arm-l", "ilium-l", "thigh-l", "clavicle-r", "thigh-r"):
    b = next(x for x in bones if x["id"] == bone_id)
    print("[DEBUG]", {k: b[k] for k in ("id", "parent", "x", "y", "z", "angleDeg", "abductionDeg", "twistDeg", "lengthM") if k in b})
write(candidate_path(A), doc)
if "--apply" in sys.argv:
    write(source_path(A), {"volumes": a["volumes"]})
    live = read(catalog_path(A))
    live["asset"]["rig"] = a["rig"]
    write(catalog_path(A), live)
    print("applied")
