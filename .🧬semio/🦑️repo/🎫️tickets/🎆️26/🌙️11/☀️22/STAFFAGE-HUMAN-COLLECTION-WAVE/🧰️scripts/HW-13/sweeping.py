"""🧹️ Generator of `humans/person-sweeping`: the archive's 1.69 m adult in fleece sweatshirt, jeans and sneakers (body, rig and outlines of
`standing-adult`, taken over unchanged) pushing a 400 mm push broom over the floor.

New: broom handle and head bones with masses, the broom parts per scale (2D and 3D from one handcrafted prism or profile each), two
passive thermal regions, five poses solved for the grip on the handle and the bristles on the floor (kit.ts), the sweep-stroke loop and
idle breathing resting on the broom. Run: `python3 sweeping.py` (solves the poses into 🗑️generated, then writes the catalog asset).
"""
import copy
import json
import math
import sys

from lib import (GEN, HERE, SCALES, add_part, bun, find, load, mass_marker, prism_part, prism_plan_trunk, rect, rest_frames, revolved_part, save, strip_bones, t, unify_arm_views)
import texts_sweeping as TX

SLUG = "person-sweeping"
GRIP = 0.090
PALM = 0.031
L_HEAD = 1.05
TOP = -0.40
GRIP_L = -0.28
HEAD_LEN = 0.11

FIST = {
    "index": (60, 80, 40), "middle": (62, 82, 42), "ring": (65, 84, 44), "little": (68, 86, 46),
}


def fist(side):
    sg = 1 if side == "r" else -1
    out = {}
    for f, (a, b, c) in FIST.items():
        out[f"{f}-proximal-{side}"] = {"angleDeg": 0, "abductionDeg": sg * a, "twistDeg": -sg * 90}
        out[f"{f}-middle-{side}"] = {"angleDeg": b}
        out[f"{f}-distal-{side}"] = {"angleDeg": c}
    out[f"thumb-metacarpal-{side}"] = {"angleDeg": -10, "abductionDeg": sg * 30, "twistDeg": -sg * 90}
    out[f"thumb-proximal-{side}"] = {"angleDeg": 20, "abductionDeg": sg * 16}
    out[f"thumb-distal-{side}"] = {"angleDeg": 25, "abductionDeg": sg * 6}
    return out


def rig(a):
    bones = a["rig"]["bones"]
    i = next(k for k, b in enumerate(bones) if b["id"] == "toe-r") + 1
    bones[i:i] = [
        {"id": "broom", "parent": "hand-r", "title": t("Broom handle (held in the right fist)", "Besenstiel (in der rechten Faust)"), "x": GRIP, "y": 0, "z": PALM, "angleDeg": -90, "lengthM": L_HEAD,
         "joint": {"type": "revolute", "minDeg": -150, "maxDeg": 150, "twistMinDeg": -180, "twistMaxDeg": 180},
         "mass": {"kg": 0.52, "comX": round((TOP + L_HEAD) / 2, 4), "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "en-338-2016"}},
        {"id": "broom-head", "parent": "broom", "title": t("Broom head on its swivel", "Besenkopf am Gelenk"), "x": L_HEAD, "y": 0, "angleDeg": 0, "lengthM": HEAD_LEN,
         "joint": {"type": "revolute", "minDeg": -110, "maxDeg": 110},
         "mass": {"kg": 0.68, "comX": 0.045, "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "en-338-2016"}},
    ]
    a["anchors"] += [
        {"id": "broom-tip", "bone": "broom-head", "x": HEAD_LEN, "y": 0, "kind": "contact", "title": t("Bristle tips on the floor", "Borstenspitzen auf dem Boden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.05, "footprintWidthM": 0.38, "frictionCoefficient": 0.45},
        {"id": "grip-r", "bone": "hand-r", "x": GRIP, "y": 0, "z": PALM, "kind": "attachment", "title": t("Right fist on the handle", "Rechte Faust am Stiel")},
        {"id": "grip-l", "bone": "hand-l", "x": GRIP, "y": 0, "z": -PALM, "kind": "attachment", "title": t("Left fist on the handle", "Linke Faust am Stiel")},
    ]


# ------------------------------------------------------------------ poses
FEET = ["heel-l", "ball-l", "heel-r", "ball-r"]

# id: head (x, z), handle elevation deg, pivot lift, base offsets (legs, trunk, head), contacts with the broom
STROKES = {
    "push-start": dict(head=(1.02, -0.20), elev=48, lift=0.0, trunk=(-7, -5), neck=(-6, -10)),
    "pushing": dict(head=(1.15, -0.20), elev=44, lift=0.0, trunk=(-10, -6), neck=(-6, -12)),
    "push-end": dict(head=(1.27, -0.20), elev=40, lift=0.0, trunk=(-14, -10), neck=(-5, -14)),
    "broom-lifted": dict(head=(1.08, -0.20), elev=46, lift=0.05, trunk=(-9, -5), neck=(-6, -11)),
    "resting-on-broom": dict(head=(0.30, -0.30), elev=84, lift=0.0, trunk=(0, 0), neck=(-1, 1)),
}
LEGS = {
    "sweep": {"thigh-l": {"angleDeg": 6}, "shank-l": {"angleDeg": -10}, "foot-l": {"angleDeg": 4}, "toe-l": {"angleDeg": 0},
              "thigh-r": {"angleDeg": -4}, "shank-r": {"angleDeg": -8}, "foot-r": {"angleDeg": 12}, "toe-r": {"angleDeg": 0}},
    "rest": {"thigh-l": {"angleDeg": -2.5}, "shank-l": {"angleDeg": -2.5}, "foot-l": {"angleDeg": 5}, "toe-l": {"angleDeg": 0},
             "thigh-r": {"angleDeg": -2.5}, "shank-r": {"angleDeg": -2.5}, "foot-r": {"angleDeg": 5}, "toe-r": {"angleDeg": 0}},
}


def unit(v):
    n = math.sqrt(sum(x * x for x in v))
    return [x / n for x in v]


def spec_of(pid, s, seeds):
    hx, hz = s["head"]
    e = math.radians(s["elev"])
    pivot = [hx, HEAD_LEN + s["lift"], hz]
    rest = pid == "resting-on-broom"
    lateral = 0.10 if not rest else 0.0
    horiz = math.cos(e)
    up = unit([-horiz, math.sin(e), lateral * horiz])
    down = [-x for x in up]
    pr = [pivot[k] + L_HEAD * up[k] for k in range(3)]
    pl = [pivot[k] + (L_HEAD - GRIP_L) * up[k] for k in range(3)]
    legs = LEGS["rest" if rest else "sweep"]
    lum, tor = s["trunk"]
    nk, hd = s["neck"]
    bones = {**legs, "lumbar": {"angleDeg": lum}, "torso": {"angleDeg": tor}, "neck": {"angleDeg": nk}, "head": {"angleDeg": hd}, **fist("r"), **fist("l" if not rest else "l")}
    seed = seeds.get(pid, {})
    for k, v in seed.items():
        bones[k] = v
    if rest:
        bones.update({"arm-l": {"angleDeg": 0, "abductionDeg": 6}, "forearm-l": {"angleDeg": 7}, "hand-l": {"angleDeg": 3}})
        for f in ("index", "middle", "ring", "little"):
            bones[f"{f}-proximal-l"] = {"angleDeg": 0, "abductionDeg": -9, "twistDeg": 90}
            bones[f"{f}-middle-l"] = {"angleDeg": 17}
            bones[f"{f}-distal-l"] = {"angleDeg": 11}
    legs_stage = {"vars": [["thigh-r", "angleDeg", -30, 60], ["shank-r", "angleDeg", -60, 0], ["foot-r", "angleDeg", -30, 30]], "terms": [
        {"kind": "pair", "bone": "foot-r", "local": [0.015, -0.0904, 0], "other": "foot-l", "otherLocal": [0.015, -0.0904, 0]},
        {"kind": "pair", "bone": "foot-r", "local": [0.17251, 0, 0], "other": "foot-l", "otherLocal": [0.17251, 0, 0]}], "regularise": 1e-4}
    arm_r = [["clavicle-r", "angleDeg", -8, 30], ["arm-r", "angleDeg", -60, 170], ["arm-r", "abductionDeg", -60, 30], ["arm-r", "twistDeg", -120, 120], ["forearm-r", "angleDeg", 0, 150], ["forearm-r", "twistDeg", -80, 80], ["hand-r", "angleDeg", -60, 60], ["hand-r", "abductionDeg", -30, 30]]
    stages = ([] if rest else [legs_stage]) + [
        {"vars": arm_r + [["broom", "angleDeg", -60, 60], ["broom", "abductionDeg", -40, 40]], "terms": [
            {"kind": "point", "bone": "hand-r", "local": [GRIP, 0, PALM], "target": pr},
            {"kind": "point", "bone": "broom", "local": [L_HEAD, 0, 0], "target": pivot}], "regularise": 2e-4},
        {"vars": [["broom", "twistDeg", -180, 180], ["broom-head", "angleDeg", -110, 110]], "terms": [{"kind": "level", "bone": "broom", "axis": "w", "direction": [0, 1, 0]}, {"kind": "axis", "bone": "broom-head", "axis": "u", "direction": [0, -1, 0], "weight": 10}], "regularise": 1e-8},
    ]
    if not rest:
        stages.append({"vars": [["clavicle-l", "angleDeg", -8, 30], ["arm-l", "angleDeg", -60, 170], ["arm-l", "abductionDeg", -30, 60], ["arm-l", "twistDeg", -120, 120], ["forearm-l", "angleDeg", 0, 150], ["forearm-l", "twistDeg", -80, 80], ["hand-l", "angleDeg", -60, 60], ["hand-l", "abductionDeg", -30, 30]],
                       "terms": [{"kind": "point", "bone": "hand-l", "local": [GRIP, 0, -PALM], "target": pl}, {"kind": "axis", "bone": "hand-l", "axis": "v", "direction": up, "weight": 6}], "regularise": 2e-4})
    return {"id": pid, "bones": bones, "contacts": FEET, "stages": stages}


SEEDS = {
    "push-start": {"arm-r": {"angleDeg": 30, "abductionDeg": -5, "twistDeg": 0}, "forearm-r": {"angleDeg": 40}, "arm-l": {"angleDeg": 20, "abductionDeg": 10, "twistDeg": 0}, "forearm-l": {"angleDeg": 70}},
    "pushing": {"arm-r": {"angleDeg": 40, "abductionDeg": -5, "twistDeg": 0}, "forearm-r": {"angleDeg": 25}, "arm-l": {"angleDeg": 25, "abductionDeg": 10, "twistDeg": 0}, "forearm-l": {"angleDeg": 60}},
    "push-end": {"arm-r": {"angleDeg": 55, "abductionDeg": -5, "twistDeg": 0}, "forearm-r": {"angleDeg": 10}, "arm-l": {"angleDeg": 35, "abductionDeg": 10, "twistDeg": 0}, "forearm-l": {"angleDeg": 45}},
    "broom-lifted": {"arm-r": {"angleDeg": 35, "abductionDeg": -5, "twistDeg": 0}, "forearm-r": {"angleDeg": 35}, "arm-l": {"angleDeg": 22, "abductionDeg": 10, "twistDeg": 0}, "forearm-l": {"angleDeg": 65}},
    "resting-on-broom": {"arm-r": {"angleDeg": 10, "abductionDeg": -8, "twistDeg": 0}, "forearm-r": {"angleDeg": 80}},
}


def solved_poses(draft_path, redo):
    out = f"{GEN}/sweeping-solved.json"
    if redo:
        spec = {"poses": [spec_of(pid, s, SEEDS) for pid, s in STROKES.items()]}
        with open(f"{GEN}/sweeping-spec.json", "w") as f:
            json.dump(spec, f)
        bun("kit.ts", "solve", draft_path, f"{GEN}/sweeping-spec.json", out)
    with open(out) as f:
        return json.load(f)


def thermal_of(a, pid):
    base = copy.deepcopy(find(a["poses"], "relaxed")["thermal"])
    rest = pid == "resting-on-broom"
    base["activityMet"] = 1.3 if rest else 3.3
    base["breathing"] = "quiet-standing" if rest else "sweeping"
    base["regions"]["broom-wood"] = {"exposedFraction": 0.92, "sunlitFraction": 0}
    base["regions"]["broom-bristles"] = {"exposedFraction": 0.75, "sunlitFraction": 0, **({} if pid == "broom-lifted" else {"contact": {"fraction": 0.2, "tempC": 21}})}
    base["regions"]["hand-skin"] = {"exposedFraction": 0.75 if not rest else 0.9, "sunlitFraction": 0}
    base["emitters"] = {"body-plume": 0.7 if not rest else 0.6}
    return base


def poses(a, solved):
    out = []
    for pid in STROKES:
        sol = solved[pid]["bones"]
        contacts = FEET + ([] if pid == "broom-lifted" else ["broom-tip"])
        out.append({"id": pid, "title": TX.POSES[pid][0], "description": TX.POSES[pid][1], "bones": sol, "ground": "auto", "contacts": contacts, "thermal": thermal_of(a, pid), "loads": []})
    return out


def clips(a):
    stroke = 1.6
    sweep = {"id": "sweep-stroke", "title": TX.CLIPS["sweep-stroke"][0], "description": TX.CLIPS["sweep-stroke"][1], "durationS": stroke, "fps": 30, "loop": True, "rootMotion": "none",
             "keys": [{"t": 0, "pose": "push-start", "ease": "smooth"}, {"t": 0.45, "pose": "pushing", "ease": "smooth"}, {"t": 0.9, "pose": "push-end", "ease": "smooth"}, {"t": 1.25, "pose": "broom-lifted", "ease": "smooth"}, {"t": stroke, "pose": "push-start", "ease": "smooth"}],
             "contacts": [{"anchor": x, "fromS": 0, "toS": stroke} for x in FEET],
             "phases": [{"id": "push", "fromS": 0, "toS": 0.9, "title": t("Push stroke", "Schubstrich")}, {"id": "lift-and-return", "fromS": 0.9, "toS": stroke, "title": t("Lift and return", "Anheben und zurückführen")}],
             "events": [{"t": 1.0, "kind": "motion", "title": t("Bristles lift off", "Borsten heben ab"), "target": "broom-head"}, {"t": 1.48, "kind": "motion", "title": t("Bristles set down", "Borsten setzen auf"), "target": "broom-head"}],
             "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": stroke, "v": 1}]}, {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 3.3}, {"t": stroke, "v": 3.3}]}]}
    rest = find(a["poses"], "resting-on-broom")["bones"]

    def nudge(**kw):
        return {k: {**rest[k], "angleDeg": round(rest[k]["angleDeg"] + v, 2)} for k, v in kw.items()}
    idle = {"id": "idle-breathing", "title": TX.CLIPS["idle-breathing"][0], "description": TX.CLIPS["idle-breathing"][1], "durationS": 5, "fps": 30, "loop": True, "rootMotion": "none",
            "keys": [{"t": 0, "pose": "resting-on-broom", "ease": "smooth"}, {"t": 1.25, "pose": "resting-on-broom", "ease": "smooth", "bones": nudge(torso=-0.4, head=1.5, lumbar=-0.2)},
                     {"t": 2.5, "pose": "resting-on-broom", "ease": "smooth", "bones": nudge(torso=0.3, head=0.4, lumbar=0.2)}, {"t": 3.75, "pose": "resting-on-broom", "ease": "smooth", "bones": nudge(torso=-0.3, head=1.2, lumbar=-0.15)},
                     {"t": 5, "pose": "resting-on-broom", "ease": "smooth"}],
            "contacts": [{"anchor": x, "fromS": 0, "toS": 5} for x in FEET + ["broom-tip"]], "phases": [{"id": "rest", "fromS": 0, "toS": 5, "title": t("Resting on the broom", "Auf den Besen gestützt")}], "events": [],
            "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 5, "v": 0}]}]}
    return [sweep, idle]


# ------------------------------------------------------------------ parts per scale
WOOD = dict(region="broom-wood", material="beech-handle")
BLOCK = dict(region="broom-wood", material="beech-block")
BRISTLE = dict(region="broom-bristles", material="pp-bristle")


def broom_parts(scale):
    """🧹️ The broom of one scale: handle (solid of revolution), socket, head block and bristle pad (prisms swept along the head axis u)."""
    parts = []
    if scale == "50":
        handle = [[TOP, 0], [TOP + 0.001, 0.0062], [TOP + 0.004, 0.0098], [TOP + 0.009, 0.0117], [TOP + 0.014, 0.012], [1.07, 0.012], [1.07, 0]]
        parts.append(("revolve", "broom-handle", "broom", handle, WOOD, 0.62, 6.6, {}))
        parts.append(("revolve", "broom-hang-cap", "broom", [[TOP - 0.004, 0], [TOP - 0.004, 0.0125], [TOP + 0.012, 0.0125], [TOP + 0.012, 0]], dict(region="broom-wood", material="pp-cap"), 0.25, 6.62, {"role": "detail"}))
        parts.append(("revolve", "broom-socket", "broom", [[1.02, 0], [1.02, 0.0135], [1.035, 0.0165], [L_HEAD, 0.0165], [L_HEAD, 0]], dict(region="broom-wood", material="pp-cap"), 0.25, 6.64, {}))
        parts.append(("prism-u", "broom-block", "broom-head", (rect(-0.030, 0.030, -0.200, 0.200, 0.008), (0.0, 0.040)), BLOCK, 0.48, 6.5, {}))
        parts.append(("prism-u", "broom-bristles", "broom-head", (rect(-0.025, 0.025, -0.192, 0.192, 0.004), (0.038, HEAD_LEN)), BRISTLE, 0.18, 6.48, {}))
    elif scale == "100":
        parts.append(("revolve", "broom-handle", "broom", [[TOP, 0], [TOP, 0.0115], [TOP + 0.006, 0.012], [1.06, 0.012], [1.06, 0]], WOOD, 0.62, 6.6, {}))
        parts.append(("revolve", "broom-socket", "broom", [[1.03, 0], [1.03, 0.016], [L_HEAD, 0.016], [L_HEAD, 0]], dict(region="broom-wood", material="pp-cap"), 0.25, 6.64, {}))
        parts.append(("prism-u", "broom-block", "broom-head", (rect(-0.030, 0.030, -0.200, 0.200, 0.005), (0.0, 0.040)), BLOCK, 0.48, 6.5, {}))
        parts.append(("prism-u", "broom-bristles", "broom-head", (rect(-0.025, 0.025, -0.192, 0.192), (0.038, HEAD_LEN)), BRISTLE, 0.18, 6.48, {}))
    elif scale == "200":
        parts.append(("revolve", "broom-handle", "broom", [[TOP, 0], [TOP, 0.0125], [L_HEAD, 0.0125], [L_HEAD, 0]], WOOD, 0.6, 6.6, {"slender": True}))
        parts.append(("prism-u", "broom-block", "broom-head", (rect(-0.030, 0.030, -0.200, 0.200), (0.0, 0.040)), BLOCK, 0.45, 6.5, {}))
        parts.append(("prism-u", "broom-bristles", "broom-head", (rect(-0.026, 0.026, -0.194, 0.194), (0.038, HEAD_LEN)), BRISTLE, 0.2, 6.48, {}))
    else:
        parts.append(("revolve", "broom-handle", "broom", [[TOP, 0], [TOP, 0.02], [L_HEAD, 0.02], [L_HEAD, 0]], WOOD, 0.55, 6.6, {"slender": True}))
        parts.append(("prism-u", "broom-head", "broom-head", (rect(-0.0375, 0.0375, -0.200, 0.200), (0.0, HEAD_LEN)), BRISTLE, 0.3, 6.5, {"regions": ["broom-wood", "broom-bristles"]}))
    return parts


def fist_hands(a, scale, frames):
    """✊️ At 1:200 and 1:500 the hands of the source are open single forms 0.18 m long; a hand round a broom handle is a fist 0.12 m long, thicker towards the palm: the drawing (a prism of the side outline across the fist) and the body solid are replaced together."""
    half = {"200": (0.0395, 0.016), "500": (0.044, 0.0378)}[scale]
    prof = [[-0.015, -0.027], [0.05, -half[0]], [0.100, -half[0] + 0.002], [0.12, -0.02], [0.12, 0.02], [0.100, half[0] - 0.002], [0.05, half[0]], [-0.015, 0.027]]
    vol = a["volumes"][scale]
    for side, sg in (("l", -1), ("r", 1)):
        bone = f"hand-{side}"
        oz = frames[bone]["o"][2]
        lo, hi = (-0.016, 0.048) if scale == "200" else (-0.026, 0.05)
        w0, w1 = (lo, hi) if sg > 0 else (-hi, -lo)
        for v in ("elevation", "front", "plan"):
            for sh in a["representations"][scale]["geometry"][v]:
                if sh["id"] == f"palm-{side}":
                    sh["points"] = prof
                    sh["solid"] = {"kind": "extruded", "sweep": "w", "z0": round(oz + w0, 4), "z1": round(oz + w1, 4)}
        solid = find(vol["body"], bone)
        mid = (w0 + w1) / 2
        rw = (w1 - w0) / 2
        rot = [0.0, -0.70711, 0.0, 0.70711]
        solid["sections"] = [{"at": [-0.015, 0, mid * 0.3], "rx": max(rw * 0.55, half[1] if scale == "500" else 0.016), "ry": 0.027, "exponent": 2.3, "rotation": rot},
                             {"at": [0.05, 0, mid], "rx": rw, "ry": half[0], "exponent": 2.6, "rotation": rot},
                             {"at": [0.100, 0, mid], "rx": rw, "ry": half[0] - 0.002, "exponent": 2.6, "rotation": rot},
                             {"at": [0.12, 0, mid], "rx": rw * 0.8, "ry": 0.02, "exponent": 2.6, "rotation": rot}]
        solid["capStart"] = True
        solid["capEnd"] = True


def add_broom(a, scale):
    for kind, pid, bone, prof, reg, tone, z, extra in broom_parts(scale):
        role = extra.get("role", "body")
        if kind == "revolve":
            shapes, solid = revolved_part(pid, bone, prof, tone=tone, z=z, role=role, slender=extra.get("slender", False), **reg)
            if pid == "broom-handle" and scale in ("50", "100"):
                solid["slender"] = True
        else:
            shapes, solid = prism_part(pid, bone, prof[0], "u", *prof[1], tone=tone, z=z, role=role, **reg)
        if "regions" in extra:
            for sh in shapes.values():
                del sh["region"]
                sh["regions"] = extra["regions"]
        member = {"broom-handle": "broom", "broom-block": "broom-head", "broom-head": "broom-head"}.get(pid)
        add_part(a, scale, shapes, solid, reg["region"], structure=member)
    vol = a["volumes"][scale]
    vol["structure"].append(mass_marker(a, scale, "broom", 0.52, [round((TOP + L_HEAD) / 2, 4), 0, 0], "beech-handle"))
    vol["structure"].append(mass_marker(a, scale, "broom-head", 0.68, [0.045, 0, 0], "beech-block"))
    if scale in ("50", "100"):
        vol["structure"].append({"id": "joint-broom-swivel", "kind": "ellipsoid", "bone": "broom-head", "material": "pp-cap", "role": "body", "tone": 0.4, "radii": [0.012, 0.012, 0.012], "member": "broom-head", "part": "joint"})




# ------------------------------------------------------------------ assembly


def base():
    a = load("standing-adult")
    a["id"] = SLUG
    a["title"] = TX.TITLE
    a["summary"] = TX.SUMMARY
    a["tags"] = ["human", "adult", "standing", "chores", "domestic", "cleaning", "sweeping", "broom", "push-broom"]
    a["materials"] += TX.MATERIALS
    pv = a["provenance"]
    pv["author"] = "Semio staffage human collection worker HW-13"
    pv["created"] = "2026-11-22"
    pv["references"] += TX.REFERENCES
    pv["notes"] = TX.NOTES
    rig(a)
    a["thermal"]["regions"] += TX.REGIONS
    a["thermal"]["respiration"]["patterns"] = [p for p in a["thermal"]["respiration"]["patterns"] if p["id"] == "quiet-standing"] + [TX.SWEEP_BREATH]
    a["thermal"]["assumptions"] = TX.THERMAL_ASSUMPTIONS
    a["structure"]["supports"] = ["heel-l", "ball-l", "heel-r", "ball-r"]
    a["structure"]["assumptions"] = TX.STRUCTURE_ASSUMPTIONS
    frames = rest_frames(save(a, SLUG, f"{GEN}/draft/{SLUG}/🔣️.json"))
    for sc in SCALES:
        add_broom(a, sc)
        if sc in ("200", "500"):
            fist_hands(a, sc, frames)
        rep = a["representations"][sc]
        rep["decisions"] = TX.DECISIONS[sc]
        a["volumes"][sc]["decisions"] = a["volumes"][sc]["decisions"] + TX.VOLUME_DECISIONS[sc]
    unify_arm_views(a)
    prism_plan_trunk(a, frames, extra={"hoodie": ["hem-rib"]}, lift={"500": {"sil-body": 0.05}})
    a["dimensions"] = dict(TX.DIMENSIONS)
    a["dimensions"]["massKg"] = round(sum(b.get("mass", {}).get("kg", 0) for b in a["rig"]["bones"]), 3)
    return a


def main():
    redo = "--solve" in sys.argv
    a = base()
    draft = save(a, SLUG, f"{GEN}/draft/{SLUG}/🔣️.json")
    solved = solved_poses(draft, redo)
    a["poses"] = poses(a, solved)
    a["clips"] = clips(a)
    a["presentation"] = {"defaultPose": "pushing", "defaultClip": "sweep-stroke", "defaultScale": 100}
    for p in solved.values():
        print("[DEBUG] residuals", [f"{x:.1e}" for x in p["residual"]])
    print(save(a, SLUG))


if __name__ == "__main__":
    main()
