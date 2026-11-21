"""🛌 Round 5 red fox: the curled-sleeping pose authored anew from world pitch angles of every bone (degrees from the horizontal, forward = 0, in the plane of its parent) and lateral bends, converted to offsets from rest down the chain.

The fox lies on its belly with brisket and loin on the ground (hip joint 0.15 m, the depth of the chest), the hind legs folded beside the belly (thigh down and forward and splayed out, shank back along the ground, metatarsus forward, paw flat), the forelegs stretched forward along the ground, the spine bent to its left in lumbar and thoracic segments, the neck and head turned back onto the left flank and the brush wrapped forward round the left side over the nose.

Usage: python3 r5-red-fox-poses.py [--apply]   (without --apply: prints the offsets)
"""
import importlib.util
import json
import sys

spec = importlib.util.spec_from_file_location("io", "r3-red-fox-io.py")
io = importlib.util.module_from_spec(spec)
spec.loader.exec_module(io)

WORLD = {
    "pelvis": 26.565, "lumbar": 1.309, "thorax": -12.791,
    "neck": -26.0, "head": -14.0, "jaw": -17.7, "ear-": 22.0,
    "tail-1": 192.0, "tail-2": 182.0, "tail-3": 178.0,
    "thigh-h": -72.0, "shank-h": -172.0, "metatarsus-h": -2.0, "paw-h": -6.0,
    "scapula-f": -57.0, "humerus-f": -124.0, "forearm-f": -6.0, "metacarpus-f": -3.0, "paw-f": -7.0,
}
LATERAL = {"lumbar": 32, "thorax": 22, "neck": 92, "head": 58, "tail-1": 62, "tail-2": 66, "tail-3": 56}
TWIST = {"humerus-fn": -38, "humerus-ff": -48}


def stem(bone):
    for s in WORLD:
        if bone == s or (bone.startswith(s) and bone[len(s):] in ("n", "f")):
            return s
    return None


def offsets(asset):
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    world = {}
    out = {}
    for b in asset["rig"]["bones"]:
        s = stem(b["id"])
        parent = world.get(b.get("parent"), 0.0)
        rest = b["angleDeg"]
        target = WORLD[s] if s else parent + rest
        d = (target - parent - rest + 180.0) % 360.0 - 180.0
        lo, hi = b["joint"]["minDeg"], b["joint"]["maxDeg"]
        if d < lo:
            d += 360.0
        if d > hi:
            d -= 360.0
        if not lo <= d <= hi:
            raise SystemExit(f"{b['id']} offset {d:.1f} outside {lo}..{hi}")
        world[b["id"]] = parent + rest + d
        entry = {}
        if abs(d) > 1e-6:
            entry["angleDeg"] = round(d, 3)
        if b["id"] in LATERAL:
            entry["abductionDeg"] = LATERAL[b["id"]]
        if b["id"] in TWIST:
            entry["twistDeg"] = TWIST[b["id"]]
        if entry:
            out[b["id"]] = entry
    del bones
    return out


asset = io.load()
bones = offsets(asset)
print(json.dumps(bones))
if "--apply" in sys.argv:
    poses = asset["poses"]
    p = next(p for p in poses if p["id"] == "curled-sleeping")
    p["bones"] = bones
    p["root"] = {"y": 0.152}
    p["description"] = {
        "en": "Curled up asleep on its belly: hind legs folded beside the belly, forelegs stretched forward, the spine bent to the left, head turned back onto the left flank and the brush wrapped forward round the left side over the nose.",
        "de": "Auf dem Bauch eingerollt schlafend: Hinterbeine neben dem Bauch gefaltet, Vorderbeine nach vorn gestreckt, Rücken nach links gebogen, Kopf zurück auf die linke Flanke gelegt und die Rute links herum nach vorn über die Nase geschlungen.",
    }
    io.put({"poses": poses})
    print("applied")
