"""🦴 Stage 1 of the seated office worker: the corrected rig (lumbar segment, base arms radiating at 72 degrees, desk 1200 x 800 mm) with everything bound to it moved rigidly, written as a candidate document and as the volumes source.

Run once on the catalog state of 2026-11-22: `python3 rework-03-seated-office-worker-1-rig.py`, then `r3-validate.ts --file <candidate>`, then `--write` copies the candidate into the catalog (re-read immediately before).
"""
import copy
import math
import sys

from rework_03_lib import *

A = "humans/seated-office-worker"
LUMBAR = 0.18
DIP = 4.1942
ARMS = {"side-a": 72, "back-a": 144, "back-b": -144, "side-b": -72}

if "--write" in sys.argv:
    doc = read(catalog_path(A))
    cand = read(candidate_path(A))
    assert doc["asset"]["id"] == cand["asset"]["id"]
    write(catalog_path(A), cand)
    write(source_path(A), {"volumes": cand["asset"]["volumes"]})
    print("written", catalog_path(A))
    sys.exit(0)

doc = read(catalog_path(A))
a = doc["asset"]
a["volumes"] = read(source_path(A))["volumes"]
bones = a["rig"]["bones"]
B = {b["id"]: b for b in bones}
assert "lumbar" not in B, "stage 1 already applied"

torso = B["torso"]
lumbar = {
    "id": "lumbar", "parent": "pelvis",
    "title": {"en": "Lumbar spine with abdomen", "de": "Lendenwirbelsäule mit Bauch"},
    "x": 0.1, "y": 0.035, "angleDeg": -2, "lengthM": LUMBAR,
    "joint": {"type": "revolute", "minDeg": -45, "maxDeg": 20},
    "mass": {"kg": 11.79, "comX": 0.095, "comY": 0, "distribution": "point", "source": "literature", "reference": "de-leva-1996"},
}
torso.update({"parent": "lumbar", "x": LUMBAR, "y": 0, "angleDeg": 0, "lengthM": 0.325, "title": {"en": "Thorax with shirt", "de": "Brustkorb mit Hemd"}, "joint": {"type": "revolute", "minDeg": -30, "maxDeg": 20}})
torso["mass"].update({"kg": 11.5353, "comX": 0.188})
bones.insert(bones.index(torso), lumbar)
for child in ("neck", "arm-l", "arm-r"):
    B[child]["x"] = round(B[child]["x"] - LUMBAR, 4)

front = B["chair-arm-front"]
castor = B["chair-castor-front"]
old_frames = {}
for key, phi in ARMS.items():
    arm, cas = B[f"chair-arm-{key}"], B[f"chair-castor-{key}"]
    old_frames[key] = (arm["angleDeg"], arm["lengthM"])
    arm.update({"angleDeg": front["angleDeg"], "lengthM": front["lengthM"], "abductionDeg": phi})
    side = ("left" if phi > 0 else "right", "links" if phi > 0 else "rechts")
    where = ("side" if abs(phi) == 72 else "back", "Seitenarm" if abs(phi) == 72 else "hinterer Arm")
    arm["title"] = {"en": f"{side[0].capitalize()} {where[0]} arm, {abs(phi)} degrees from the front arm", "de": f"{where[1]} {side[1]}, {abs(phi)} Grad zum vorderen Arm"}
    cas.update({"x": castor["x"], "y": castor["y"], "angleDeg": castor["angleDeg"]})
    cas.pop("z", None)

B["desk-top"].update({"lengthM": 0.8, "title": {"en": "Desk top, 1200 x 800 mm", "de": "Schreibtischplatte, 1200 x 800 mm"}})
B["desk-top"]["mass"].update({"kg": 15.6, "comX": 0.4})
B["desk-leg"].update({"x": 0.68})
B["desk-leg"]["mass"].update({"kg": 12.7})
a["dimensions"]["massKg"] = round(a["dimensions"]["massKg"] + 8.0 + 6.5, 4)

for pose in a["poses"]:
    pose["bones"]["lumbar"] = {"angleDeg": 0}
for anchor in a["anchors"]:
    if anchor["id"] == "backrest":
        anchor["x"] = round(anchor["x"] - LUMBAR, 4)


def shift(solid, du):
    if "of" in solid:
        return
    t = solid.setdefault("transform", {})
    tr = t.get("translation", [0, 0, 0])
    t["translation"] = [round(tr[0] + du, 5), tr[1], tr[2]]


def desk(scale):
    leg_r = 0.003 if scale == 50 else 0.002
    out = [box("desk-top", "desk-top", "desk-board", 0.62, [0.8, 0.025, 1.2], [0.4, 0.0125, 0], 0.003)]
    for name, v, w in (("fl", 0.35, 0.55), ("fr", 0.35, -0.55), ("bl", -0.35, 0.55), ("br", -0.35, -0.55)):
        if scale == 50:
            out.append(box(f"desk-leg-{name}", "desk-leg", "desk-steel", 0.42, [0.683, 0.04, 0.04], [0.3535, v, w], leg_r))
            out.append(cap(f"desk-glide-{name}", "desk-leg", "desk-steel", 0.2, [0.004, v, w], [0.012, v, w], 0.014))
        else:
            out.append(box(f"desk-leg-{name}", "desk-leg", "desk-steel", 0.42, [0.695, 0.04, 0.04], [0.3475, v, w], leg_r))
    if scale <= 200:
        thick = 0.02 if scale <= 100 else 0.03
        out.append(box("desk-rail-l", "desk-leg", "desk-steel", 0.42, [0.04, 0.66, thick], [0.675, 0, 0.55], 0.002))
        out.append(box("desk-rail-r", "desk-leg", "desk-steel", 0.42, [0.04, 0.66, thick], [0.675, 0, -0.55], 0.002))
        out.append(box("desk-rail-back", "desk-leg", "desk-steel", 0.42, [0.04, thick, 1.06], [0.675, -0.35, 0], 0.002))
        out.append(box("desk-rail-front", "desk-leg", "desk-steel", 0.42, [0.04, thick, 1.06], [0.675, 0.35, 0], 0.002))
    return out


def spine_split(scale, structure):
    r = {50: (0.024, 0.021, 0.013), 100: (0.024, 0.021, 0.014), 200: (0.024, 0.02, 0.016), 500: (0.0377, 0.0377, 0.0377)}[scale]
    S = by_id(structure)
    spine = S["spine"]
    lum = loft("lumbar-spine", "lumbar", spine["material"], spine["tone"], [(-0.004, 0, 0, r[0], r[0]), (0.06, -0.02, 0, r[0], r[0]), (0.125, -0.02, 0, r[1], r[1]), (0.186, -0.01, 0, r[1], r[1])], across=False, member="lumbar", part="member")
    tho = loft("spine", "torso", spine["material"], spine["tone"], [(-0.006, -0.01, 0, r[1], r[1]), (0.05, 0.006, 0, r[1], r[1]), (0.14, 0.024, 0, (r[1] + r[2]) / 2, (r[1] + r[2]) / 2), (0.24, 0.02, 0, r[2], r[2]), (0.325, 0, 0, r[2], r[2])], across=False, member="torso", part="member")
    i = structure.index(spine)
    structure[i : i + 1] = [lum, tho]
    for kind, at in (("joint", [0, 0, 0]), ("mass", [0.095, 0, 0])):
        t = S.get(f"{kind}-torso")
        if t is None:
            continue
        c = copy.deepcopy(t)
        c.update({"id": f"{kind}-lumbar", "bone": "lumbar", "member": "lumbar"})
        c["transform"]["translation"] = at
        structure.insert(structure.index(t), c)
        if kind == "mass":
            t["transform"]["translation"] = [0.188, 0, 0]
        else:
            t["transform"]["translation"] = [0, 0, 0]


for scale, vol in a["volumes"].items():
    old_body = by_id(vol["body"])
    tone = {s["id"]: (s["material"], s["tone"]) for s in vol["body"] if s["bone"].startswith("desk")}
    new_desk = desk(int(scale))
    for s in new_desk:
        ref = tone.get(s["id"]) or tone.get("desk-rail-back") or tone.get("desk-leg-fl")
        s["material"], s["tone"] = ref
    body = [s for s in vol["body"] if not s["bone"].startswith("desk")]
    at = next(i for i, s in enumerate(vol["body"]) if s["bone"].startswith("desk"))
    front_solids = [s for s in body if s["bone"] in ("chair-arm-front", "chair-castor-front")]
    body = [s for s in body if not any(s["bone"] in (f"chair-arm-{k}", f"chair-castor-{k}") for k in ARMS)]
    for key in ARMS:
        for s in front_solids:
            c = copy.deepcopy(s)
            c["id"] = c["id"].replace("front", key)
            c["bone"] = c["bone"].replace("front", key)
            assert c["id"] in old_body, c["id"]
            body.append(c)
    order = {s["id"]: i for i, s in enumerate(vol["body"])}
    body.sort(key=lambda s: order.get(s["id"], 0))
    first_desk = next((i for i, s in enumerate(body) if order.get(s["id"], 0) > at), len(body))
    body[first_desk:first_desk] = new_desk
    for s in body:
        if s["bone"] == "torso":
            shift(s, -LUMBAR)
    vol["body"] = body
    for name in ("thermal", "structure"):
        lst = vol[name]
        for s in lst:
            if s.get("bone") == "torso":
                shift(s, -LUMBAR)
            if s["id"].startswith("mass-desk-leg-"):
                k = int(s["id"][-1]) - 1
                s["transform"]["translation"] = [0.4, (0.35, 0.35, -0.35, -0.35)[k], (0.55, -0.55, 0.55, -0.55)[k]]
            if s["id"] == "mass-desk-top":
                s["transform"]["translation"] = [0.4, 0.0125, 0]
            if s["id"].startswith("floor-desk-"):
                tr = s["transform"]["translation"]
                s["transform"]["translation"] = [0.33 if s["id"][-2] == "f" else 1.03, tr[1], 0.55 if s["id"][-1] == "l" else -0.55]
        if "desk-rail-front" in by_id(body):
            for i, s in enumerate(list(lst)):
                if s.get("of") == "desk-rail-back":
                    c = copy.deepcopy(s)
                    c["id"] = c["id"].replace("back", "front")
                    c["of"] = "desk-rail-front"
                    lst.insert(lst.index(s) + 1, c)
        ids = by_id(body)
        vol[name] = [s for s in lst if "of" not in s or s["of"] in ids]
    spine_split(int(scale), vol["structure"])


def remap_arm(points, key):
    old_angle, _ = old_frames[key]
    ca, sa = math.cos(math.radians(old_angle)), math.sin(math.radians(old_angle))
    phi = math.radians(ARMS[key])
    d = math.radians(DIP)
    ux, uy, vx, vy = math.cos(d) * math.cos(phi), -math.sin(d), math.sin(d), math.cos(d)
    det = ux * vy - uy * vx
    out = []
    for x, y in points:
        wx, wy = x * ca - y * sa, x * sa + y * ca
        out.append([round((wx * vy - wy * vx) / det, 4), round((ux * wy - uy * wx) / det, 4)])
    return out


for rep in a["representations"].values():
    for s in rep["geometry"]["elevation"]:
        if s["bone"] == "torso":
            s["points"] = [[round(p[0] - LUMBAR, 4), p[1]] for p in s["points"]]
        for key, phi in ARMS.items():
            if s["bone"] == f"chair-arm-{key}":
                s["points"] = remap_arm(s["points"], key)
            if s["bone"] == f"chair-castor-{key}":
                s["points"] = [[round(p[0] / math.cos(math.radians(phi)), 4), p[1]] for p in s["points"]]

for region in a["thermal"]["regions"]:
    if region["id"] == "desk":
        region["areaM2"] = 2.87
text = a["structure"]["assumptions"][1]
text["en"] = text["en"].replace("(1.7 kg)", "(1.8 kg)").replace("(14.6 kg)", "(28.3 kg)")
text["de"] = text["de"].replace("(1,7 kg)", "(1,8 kg)").replace("(14,6 kg)", "(28,3 kg)")
write(candidate_path(A), doc)
print("candidate", candidate_path(A))
