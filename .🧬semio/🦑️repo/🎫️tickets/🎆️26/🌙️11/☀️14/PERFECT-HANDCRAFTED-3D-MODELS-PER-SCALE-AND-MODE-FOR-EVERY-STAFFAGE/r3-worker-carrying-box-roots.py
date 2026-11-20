"""🌱 Root fixes of `humans/worker-carrying-box` (round 3): rig joints at the anthropometric landmarks of a 1.741 m man in safety boots, a carton that is as wide as the hands are apart and is carried by its hand holes, arm poses solved onto those hand holes, anchors, dimensions, texts and the redrawn 2D shapes.

Usage: python3 r3-worker-carrying-box-roots.py            writes the draft document `🗑️generated/r3-worker-carrying-box/asset.json`
       python3 r3-worker-carrying-box-roots.py --apply    re-reads the catalog asset and writes the same root fixes into it (every member outside `volumes`; `volumes` keeps its text and is spliced by `volumes-insert.py`)
Every value is absolute and stated here with its reason, so the script can be run again at any time. The redrawn shapes come from `🗑️generated/r3-worker-carrying-box/shapes.json` (written by `r3-worker-carrying-box-draw2d.ts`) when that file exists.
"""
import json
import math
import os
import subprocess
import sys

T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
import r3_worker_carrying_box_ik as ik
import r3_worker_carrying_box_kit as k
import r3_worker_carrying_box_texts as texts

CATALOG = os.path.normpath(os.path.join(T, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"))
ASSET = os.path.join(CATALOG, "👤️humans/worker-carrying-box/🔣️.json")
MANIFEST = os.path.join(CATALOG, "🔣️.json")
OUT = k.OUT
SHAPES = os.path.join(OUT, "shapes.json")

STATURE = 1.741
SOLE = 0.030
HIP = 0.9406
THIGH = 0.420
SHANK = 0.4248
ANKLE = round(HIP - THIGH - SHANK, 4)
TOE = 0.090
S1 = (-0.045, HIP + 0.1011)
C7 = (-0.030, round(0.8605 * STATURE + SOLE, 3))
VERTEX = (-0.005, STATURE + SOLE)
SHOULDER = (-0.010, round(0.818 * STATURE + SOLE, 3))
SHOULDER_Z = 0.180
GRIP_Z = 0.217
BOX = (0.30, 0.30, 0.40)
HOLE_BELOW_TOP = 0.065
BOX_CENTRE_BELOW_GRIP = BOX[1] / 2 - HOLE_BELOW_TOP
DIMENSIONS = {"heightM": 1.82, "widthM": 0.65, "depthM": 0.52}

ARMS = {
    "carrying-box-low": {"grip": (0.310, 0.910), "phi": 14, "psi": 4},
    "squat-grip": {"grip": None, "phi": 14, "psi": 3},
    "lift-mid": {"grip": (0.620, 0.460), "phi": 15, "psi": 3},
    "setting-down": {"grip": (0.560, 0.530), "phi": 16, "psi": 3},
    "carrying-box-chest": {"grip": (0.300, 1.160), "phi": 5, "psi": 50, "elbow": (0.0, -0.5, 1.0)},
}
SQUAT = {"thigh": 72, "shank": -102, "foot": 30, "torso": -66}
PELVIS_TILT = {"squat-grip": -38, "lift-mid": -30, "setting-down": -25}


def fix_rig(asset):
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    torso = bones["torso"]
    share = 0.2124 / 0.4652
    lean = math.atan2(C7[1] - S1[1], C7[0] - S1[0])
    torso["x"], torso["y"] = 0.1011, round(-S1[0], 5)
    torso["angleDeg"] = round(math.degrees(lean) - 90, 3)
    torso["lengthM"] = round(math.hypot(C7[0] - S1[0], C7[1] - S1[1]), 5)
    torso["mass"]["comX"], torso["mass"]["comY"] = round(share * torso["lengthM"], 4), 0
    u, v = (math.cos(lean), math.sin(lean)), (-math.sin(lean), math.cos(lean))
    into = lambda p: (round((p[0] - S1[0]) * u[0] + (p[1] - S1[1]) * u[1], 5), round((p[0] - S1[0]) * v[0] + (p[1] - S1[1]) * v[1], 5))
    head = bones["head"]
    rise = math.atan2(VERTEX[1] - C7[1], VERTEX[0] - C7[0])
    head["x"], head["y"] = torso["lengthM"], 0
    head["angleDeg"] = round(math.degrees(rise - lean), 3)
    head["lengthM"] = round(math.hypot(VERTEX[0] - C7[0], VERTEX[1] - C7[1]), 5)
    head["mass"]["comX"] = round(0.5002 * head["lengthM"], 4)
    for side, sign in (("l", 1), ("r", -1)):
        arm = bones["upper-arm-" + side]
        arm["x"], arm["y"] = into(SHOULDER)
        arm["z"] = sign * SHOULDER_Z
        arm["angleDeg"] = round(-90 - math.degrees(lean), 3)
        thigh = bones["thigh-" + side]
        thigh["lengthM"] = THIGH
        thigh["mass"]["comX"] = round(0.4095 * THIGH, 4)
        bones["shank-" + side]["x"] = THIGH
        foot = bones["foot-" + side]
        foot["mass"]["comY"] = round(-ANKLE / 2, 4)
        bones["toe-" + side]["y"] = -ANKLE
        bones["toe-" + side]["lengthM"] = TOE
        box = bones["box-" + side]
        box["angleDeg"] = 90
        box["lengthM"] = BOX[0] / 2
        box["title"] = texts.BOX_BONE[side]


def fix_anchors(asset):
    anchors = {a["id"]: a for a in asset["anchors"]}
    for side in ("l", "r"):
        anchors["contact-" + side]["y"] = -ANKLE
        anchors["contact-" + side].update({"footprintLengthM": 0.10, "footprintWidthM": 0.108})
        anchors["heel-" + side].update({"x": -0.040, "y": -ANKLE, "footprintLengthM": 0.085, "footprintWidthM": 0.088})
    base = anchors["box-base"]
    ordered = {}
    for key, value in base.items():
        ordered[key] = value
        if key == "y":
            ordered["z"] = GRIP_Z
    ordered.update({"x": 0, "y": -(BOX_CENTRE_BELOW_GRIP + BOX[1] / 2), "footprintLengthM": BOX[0], "footprintWidthM": BOX[2]})
    base.clear()
    base.update(ordered)
    head = {b["id"]: b for b in asset["rig"]["bones"]}["head"]
    frames = k.pose_frames(asset)
    space = k.Space(frames)
    top = STATURE + SOLE
    for anchor, world in (("mouth", (0.099, top - 0.188, 0)), ("nose", (0.105, top - 0.166, 0)), ("crown", (-0.012, 1.815, 0))):
        p = space.point("head", world)
        anchors[anchor]["x"], anchors[anchor]["y"] = round(p[0], 4), round(p[1], 4)
    assert head["lengthM"] > 0


def fix_poses(asset):
    solved = {}
    for pose in asset["poses"]:
        bones = pose["bones"]
        own = "carrying-box-low" if pose["id"].startswith("walk-") else pose["id"]
        spec = ARMS[own]
        if own == "squat-grip":
            for part, deg in SQUAT.items():
                for bone in ([part] if part in bones else [part + "-l", part + "-r"]):
                    bones[bone] = {"angleDeg": deg}
        if pose["id"] in PELVIS_TILT and "pelvis" not in bones:
            tilt = PELVIS_TILT[pose["id"]]
            for bone in ("torso", "thigh-l", "thigh-r"):
                bones[bone] = {"angleDeg": round(bones[bone]["angleDeg"] - tilt, 3)}
            bones["pelvis"] = {"angleDeg": tilt}
        rest = {b: {"angleDeg": o["angleDeg"]} for b, o in bones.items() if not b.startswith(("upper-arm", "forearm", "hand", "box"))}
        if own not in solved:
            base = next(p for p in asset["poses"] if p["id"] == own)["bones"]
            stand = {b: {"angleDeg": o["angleDeg"]} for b, o in base.items() if not b.startswith(("upper-arm", "forearm", "hand", "box"))}
            grip = spec["grip"]
            if grip is None:
                frames = k.grounded(asset, stand, ik.FEET)
                toe = frames["toe-l"]
                tip = toe["o"][0] + TOE * toe["u"][0]
                grip = (round(tip + 0.02 + BOX[0] / 2, 3), round(BOX[1] - HOLE_BELOW_TOP, 3))
            solved[own] = ik.solve_arms(asset, stand, (grip[0], grip[1], GRIP_Z), spec["phi"], spec["psi"], spec.get("elbow", (-1.0, -0.5, 0.3)))
        for bone in list(bones):
            if bone in solved[own]:
                bones[bone] = dict(solved[own][bone])
        if pose["id"] in texts.POSES:
            pose["description"] = texts.POSES[pose["id"]]
    return solved


def fix_clips(asset, solved):
    for clip in asset["clips"]:
        for key in clip["keys"]:
            for bone in list(key.get("bones", {})):
                if bone in solved["carrying-box-low"]:
                    key["bones"][bone] = dict(solved["carrying-box-low"][bone])
        if clip["id"] in texts.CLIPS:
            clip["description"] = texts.CLIPS[clip["id"]]


def fix_texts(asset):
    asset["dimensions"].update(DIMENSIONS)
    asset["summary"] = texts.SUMMARY
    asset["provenance"]["notes"] = texts.NOTES
    known = {r["id"] for r in asset["provenance"]["references"]}
    asset["provenance"]["references"] += [r for r in texts.REFERENCES if r["id"] not in known]
    asset["structure"]["assumptions"] = texts.ASSUMPTIONS
    for material in asset["materials"]:
        if material["id"] in texts.MATERIALS:
            material["title"] = texts.MATERIALS[material["id"]]
    for scale, decisions in texts.DECISIONS_2D.items():
        asset["representations"][scale]["decisions"].update(decisions)


def fix_shapes(asset):
    if not os.path.exists(SHAPES):
        return False
    shapes = json.load(open(SHAPES, encoding="utf-8"))
    for scale, rep in shapes.items():
        asset["representations"][scale]["geometry"] = rep["geometry"]
        if "hatches" in rep:
            asset["representations"][scale]["hatches"] = rep["hatches"]
    return True


def fixed(document):
    asset = document["asset"]
    fix_rig(asset)
    fix_anchors(asset)
    solved = fix_poses(asset)
    fix_clips(asset, solved)
    fix_texts(asset)
    drawn = fix_shapes(asset)
    return solved, drawn


def advance(document):
    """🚶 Sets `advanceM` of the walking clip to the planted travel the product measures on the draft."""
    bun = sorted(__import__("glob").glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
    tmp = os.path.join(OUT, "advance.json")
    json.dump(document, open(tmp, "w", encoding="utf-8"), ensure_ascii=False)
    done = subprocess.run([bun, "r3-worker-carrying-box-probe.ts", tmp], cwd=T, capture_output=True, text=True)
    os.remove(tmp)
    if done.returncode != 0:
        sys.exit(done.stdout + done.stderr)
    probe = json.loads(done.stdout)
    for clip in document["asset"]["clips"]:
        if clip["id"] in probe["travel"]:
            clip["advanceM"] = round(probe["travel"][clip["id"]], 6)
    return probe


def volumes_span(text):
    start = text.index('\n    "volumes": {') + 1
    end = text.index('\n    "presentation": {')
    return start, end


def render(document, volumes_text):
    """🧾 The document in the catalog's own style: two-space indent everywhere, the `volumes` member kept as the text `volumes-insert.py` wrote."""
    marker = "@@R3-WORKER-VOLUMES@@"
    copy = json.loads(json.dumps(document))
    copy["asset"]["volumes"] = marker
    text = json.dumps(copy, indent=2, ensure_ascii=False) + "\n"
    return text.replace('    "volumes": "%s",\n' % marker, volumes_text + "\n")


def main():
    os.makedirs(OUT, exist_ok=True)
    text = open(ASSET, encoding="utf-8").read()
    start, end = volumes_span(text)
    document = json.loads(text)
    if render(document, text[start:end]) != text:
        sys.exit("the catalog file is not in the expected style: nothing written")
    solved, drawn = fixed(document)
    probe = advance(document)
    json.dump(document, open(k.DRAFT, "w", encoding="utf-8"), ensure_ascii=False)
    print("draft", k.DRAFT, "shapes" if drawn else "old shapes", "advance", probe["travel"])
    if "--print" in sys.argv:
        for pose, arms in solved.items():
            print(pose, json.dumps(arms))
    if "--apply" in sys.argv:
        fresh = open(ASSET, encoding="utf-8").read()
        s, e = volumes_span(fresh)
        again = json.loads(fresh)
        fixed(again)
        advance(again)
        tmp = ASSET + ".r3.tmp"
        open(tmp, "w", encoding="utf-8").write(render(again, fresh[s:e]))
        os.replace(tmp, ASSET)
        manifest = open(MANIFEST, encoding="utf-8").read()
        at = manifest.index('"id": "worker-carrying-box"')
        head, tail = manifest[:at], manifest[at:]
        import re

        tail = re.sub(r'"heightM": [0-9.]+', '"heightM": %s' % DIMENSIONS["heightM"], tail, count=1)
        tail = re.sub(r'"widthM": [0-9.]+', '"widthM": %s' % DIMENSIONS["widthM"], tail, count=1)
        if head + tail != manifest:
            open(MANIFEST, "w", encoding="utf-8").write(head + tail)
        print("applied", ASSET)


main()
