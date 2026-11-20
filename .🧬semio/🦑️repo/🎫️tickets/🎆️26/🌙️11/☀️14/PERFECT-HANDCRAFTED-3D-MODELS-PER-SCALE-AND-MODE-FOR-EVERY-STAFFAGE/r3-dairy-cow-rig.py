"""🦴 Root fix of the dairy cow rig: the trunk chain pelvis, lumbar, thorax ran at the height of the hip joint (1.10 m), 0.27 m below the vertebral column; the three bones now follow the ilium, the lumbar spine and the thoracic spine. Every child bone, mass and anchor keeps its rest position in the world.

Usage: python3 r3-dairy-cow-rig.py [--apply]   (without --apply: prints the old and new values and the re-levelled grazing pose)
"""
import copy
import json
import math
import sys

import r3_dairy_cow_io as io

LUMBOSACRAL = (0.17, 0.27)
THORACOLUMBAR = (0.62, 0.27)
FIRST_THORACIC = (1.30, 0.08)
STERNUM = (1.01, -0.484)
BACK = (0.75, 0.336)


def frames(bones, pose=None, root=None):
    by = {b["id"]: b for b in bones}
    out = {}

    def solve(i):
        if i in out:
            return out[i]
        b = by[i]
        d = (pose or {}).get(i, {})
        if b["parent"] is None:
            r = root or {}
            out[i] = (b["x"] + r.get("x", 0) + d.get("x", 0), b["y"] + r.get("y", 0) + d.get("y", 0), b["angleDeg"] + r.get("angleDeg", 0) + d.get("angleDeg", 0))
        else:
            px, py, pa = solve(b["parent"])
            c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
            x, y = b["x"] + d.get("x", 0), b["y"] + d.get("y", 0)
            out[i] = (px + x * c - y * s, py + x * s + y * c, pa + b["angleDeg"] + d.get("angleDeg", 0))
        return out[i]

    for b in bones:
        solve(b["id"])
    return out


def to_world(f, x, y):
    c, s = math.cos(math.radians(f[2])), math.sin(math.radians(f[2]))
    return (f[0] + x * c - y * s, f[1] + x * s + y * c)


def to_local(f, wx, wy):
    c, s = math.cos(math.radians(f[2])), math.sin(math.radians(f[2]))
    dx, dy = wx - f[0], wy - f[1]
    return (dx * c + dy * s, -dx * s + dy * c)


def r5(v):
    return round(v, 5)


def new_rig(asset):
    bones = copy.deepcopy(asset["rig"]["bones"])
    old = frames(bones)
    target = dict(old)
    target["pelvis"] = (0.0, 0.0, math.degrees(math.atan2(LUMBOSACRAL[1], LUMBOSACRAL[0])))
    target["lumbar"] = (LUMBOSACRAL[0], LUMBOSACRAL[1], 0.0)
    target["thorax"] = (THORACOLUMBAR[0], THORACOLUMBAR[1], math.degrees(math.atan2(FIRST_THORACIC[1] - THORACOLUMBAR[1], FIRST_THORACIC[0] - THORACOLUMBAR[0])))
    lengths = {"pelvis": math.hypot(*LUMBOSACRAL), "lumbar": THORACOLUMBAR[0] - LUMBOSACRAL[0], "thorax": math.hypot(FIRST_THORACIC[0] - THORACOLUMBAR[0], FIRST_THORACIC[1] - THORACOLUMBAR[1])}
    for b in bones:
        i = b["id"]
        if "mass" in b and i in lengths:
            wx, wy = to_world(old[i], b["mass"]["comX"], b["mass"]["comY"])
            b["mass"]["comX"], b["mass"]["comY"] = map(r5, to_local(target[i], wx, wy))
        if i in lengths:
            b["lengthM"] = r5(lengths[i])
        if b["parent"] is None:
            b["x"], b["y"], b["angleDeg"] = r5(target[i][0]), r5(target[i][1]), r5(target[i][2])
        elif b["parent"] in lengths or i in lengths:
            p = target[b["parent"]]
            x, y = to_local(p, target[i][0], target[i][1])
            b["x"], b["y"], b["angleDeg"] = r5(x), r5(y), r5(target[i][2] - p[2])
    anchors = copy.deepcopy(asset["anchors"])
    for a in anchors:
        if a["bone"] in lengths:
            wx, wy = to_world(old[a["bone"]], a["x"], a["y"])
            if a["id"] == "contact-sternum":
                wx, wy = STERNUM
            if a["id"] == "back-plume":
                wx, wy = BACK
            a["x"], a["y"] = map(r5, to_local(target[a["bone"]], wx, wy))
    return bones, anchors


def anchor_world(bones, anchors, pose, root):
    f = frames(bones, pose, root)
    return {a["id"]: to_world(f[a["bone"]], a["x"], a["y"]) for a in anchors}


def level_grazing(asset, bones, anchors):
    """⚖️ The grazing pose tilts the thorax; with the pivot at the thoracolumbar joint the fore hooves would hang 1.8 mm above the ground of the hind hooves, so the tilt is solved again until both pairs stand level."""
    poses = copy.deepcopy(asset["poses"])
    pose = next(p for p in poses if p["id"] == "grazing-head-down")

    def gap(delta):
        w = anchor_world(bones, anchors, {**pose["bones"], "thorax": {"angleDeg": delta}}, pose.get("root"))
        return (w["contact-fn"][1] + w["contact-ff"][1]) / 2 - (w["contact-hn"][1] + w["contact-hf"][1]) / 2

    lo, hi = -4.0, -2.0
    for _ in range(60):
        mid = (lo + hi) / 2
        if gap(lo) * gap(mid) <= 0:
            hi = mid
        else:
            lo = mid
    pose["bones"]["thorax"] = {"angleDeg": round((lo + hi) / 2, 3)}
    return poses


def main():
    text, asset = io.read()
    bones, anchors = new_rig(asset)
    for b0, b1 in zip(asset["rig"]["bones"], bones):
        if b0 != b1:
            print("bone", b0["id"], {k: (b0[k], b1[k]) for k in ("x", "y", "angleDeg", "lengthM") if b0[k] != b1[k]}, "com", (b0["mass"]["comX"], b0["mass"]["comY"]), "->", (b1["mass"]["comX"], b1["mass"]["comY"]))
    for a0, a1 in zip(asset["anchors"], anchors):
        if a0 != a1:
            print("anchor", a0["id"], (a0["x"], a0["y"]), "->", (a1["x"], a1["y"]))
    poses = level_grazing(asset, bones, anchors)
    for p0, p1 in zip(asset["poses"], poses):
        if p0 != p1:
            print("pose", p0["id"], "thorax", p0["bones"]["thorax"], "->", p1["bones"]["thorax"])
    if "--apply" in sys.argv:
        def change(a):
            b, an = new_rig(a)
            return {"rig": {**a["rig"], "bones": b}, "anchors": an, "poses": level_grazing(a, b, an)}
        print("written", io.write(change))


main()
