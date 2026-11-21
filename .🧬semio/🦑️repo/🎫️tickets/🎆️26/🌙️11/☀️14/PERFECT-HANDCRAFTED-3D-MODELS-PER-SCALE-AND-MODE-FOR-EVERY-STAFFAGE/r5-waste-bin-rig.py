"""🗑️ Round 5 waste bin: the root fix of the rig. The `body` bone lay on the ground and ran 0.5 m ahead from the base centre, its tip 0.25 m in the air in front of the bin, so the structural view drew the shell's member as a floor bar; the load path of the moulded bin runs up the tub wall from the feet to the hinge. The bone now stands on the tub axis (angle 90, 0.98 m to the pivot height) and every member bound to it is restated in the new frame: children (floor, lid), mass, anchors, the elevation shapes of every scale and every body-bone solid of every scale and list (the round-3 solids get the frame turn as their transform, `sectionFaces` follow the axes).

The 1:200 plan lid is redrawn as one 0.52 x 0.5 m strip: its six stepped corner strips (12 mm radius, below the 30 mm feature) left white slivers of 0.01 mm at 1:150, and the 1:200 lid solids are square in plan anyway.

Usage: python3 r5-waste-bin-rig.py [--apply]   (writes the volumes source and the candidate; --apply writes rig, anchors and representations into the catalog)
Run only once on a catalog whose `body` bone still has angle 0 (it refuses otherwise).
See ./r5-brief.md, ./r3-objects-waste-bin.md.
"""
import copy
import json
import math
import re
import sys

import r5_o02_lib as L

B = L.module("r3-waste-bin-build.py")

S2 = math.sqrt(0.5)
TURN = [0.0, 0.0, -S2, S2]
FACES = {"-u": "+v", "+u": "-v", "-v": "-u", "+v": "+u", "-w": "-w", "+w": "+w"}
HEIGHT = 0.98


def r5(v):
    return round(v + 0.0, 5) + 0.0


def local(x, y):
    """🧭 A point of the old body frame (world x ahead, y up) in the new upright frame (u up, v back)."""
    return r5(y), r5(-x)


def qmul(a, b):
    ax, ay, az, aw = a
    bx, by, bz, bw = b
    return [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz]


def turn_solid(s):
    """🔄 Prefixes a body-bone solid's transform with the frame turn (old frame -> new frame)."""
    t = s.get("transform", {})
    tr = t.get("translation", [0.0, 0.0, 0.0])
    out = {}
    if "scale" in t:
        out["scale"] = t["scale"]
    out["rotation"] = [round(c, 7) + 0.0 for c in qmul(TURN, t.get("rotation", [0.0, 0.0, 0.0, 1.0]))]
    u, v = local(tr[0], tr[1])
    out["translation"] = [u, v, r5(tr[2])]
    s["transform"] = out
    if "repeat" in s:
        raise SystemExit("repeat on a body solid: restate by hand")
    if "sectionFaces" in s:
        s["sectionFaces"] = [FACES[f] for f in s["sectionFaces"]]
    return s


def turn_volumes(volumes):
    for scale in volumes.values():
        bones = {s["id"]: s.get("bone") for s in scale.get("body", [])}
        for key in ("body", "thermal", "structure"):
            for s in scale.get(key, []):
                if s.get("bone") == "body" and "of" not in s:
                    turn_solid(s)
                elif "of" in s and bones.get(s["of"]) == "body" and "sectionFaces" in s:
                    s["sectionFaces"] = [FACES[f] for f in s["sectionFaces"]]
    return lid_200(volumes)


def lid_200(volumes):
    """🎩 1:200 lid as one solid dome over the full 0.5 m width: the 4 mm dish shell and its two 16 mm end walls are below the 30 mm feature and merge into it."""
    v = volumes["200"]
    gone = {"lid-end-l", "lid-end-r", "th-lid-end-l", "th-lid-end-r", "member-lid-end-l", "member-lid-end-r"}
    for key in ("body", "thermal", "structure"):
        v[key] = [s for s in v[key] if s["id"] not in gone]
    for s in v["body"]:
        if s["id"] == "lid-cap":
            s["from"], s["to"] = -0.25, 0.25
            s["profile"] = B.pts(B.DISH_OUTER)
    d = v["decisions"][0]
    d["en"] = re.sub(r"the lid is the domed dish with 16 mm end walls seated on that ring", "the lid is one solid dome over the full 0.5 m width seated on that ring (the 4 mm dish shell and its 16 mm end walls are below the feature and merge into it)", d["en"])
    d["de"] = re.sub(r"der Deckel ist die gewölbte Schale mit 16-mm-Stirnwänden auf diesem Ring", "der Deckel ist eine volle Wölbung über die ganze Breite von 0,5 m auf diesem Ring (die 4-mm-Schale und ihre 16-mm-Stirnwände liegen unter dem Kleinstmaß und gehen in ihr auf)", d["de"])
    if "dish with 16 mm" in d["en"] or "mit 16-mm-Stirnwänden" in d["de"]:
        raise SystemExit("decision text of 1:200 not matched")
    for d in v["decisions"][1:]:
        d["en"] = d["en"].replace("lid dish and end walls (lid)", "lid dish (lid)")
        d["de"] = d["de"].replace("Deckelschale und Stirnwände (lid)", "Deckelschale (lid)")
    return volumes


def turn_path(d):
    """✏️ Restates the coordinate pairs of an SVG-like path (M, L, C, Z only) in the new frame."""
    tokens = re.findall(r"[MLCZ]|-?\d*\.?\d+(?:e-?\d+)?", d)
    out, nums = [], []
    for tok in tokens:
        if tok in "MLCZ":
            out.append(tok)
            continue
        nums.append(float(tok))
        if len(nums) == 2:
            u, v = local(*nums)
            out += ["%g" % u, "%g" % v]
            nums = []
    return " ".join(out)


def turn_shape(s):
    k = s["kind"]
    if k == "rect":
        u0, v1 = local(s["x"], s["y"])
        u1, v0 = local(s["x"] + s["w"], s["y"] + s["h"])
        s["x"], s["y"], s["w"], s["h"] = min(u0, u1), min(v0, v1), r5(abs(u1 - u0)), r5(abs(v1 - v0))
    elif k == "poly":
        s["points"] = [list(local(x, y)) for x, y in s["points"]]
    elif k == "path":
        s["d"] = turn_path(s["d"])
    elif k == "ellipse":
        s["cx"], s["cy"] = local(s["cx"], s["cy"])
        s["rx"], s["ry"] = s["ry"], s["rx"]
    else:
        raise SystemExit("shape kind %s not handled" % k)
    if "solid" in s:
        raise SystemExit("solid member on %s: restate by hand" % s["id"])
    return s


def roots(asset):
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    body = bones["body"]
    if body["angleDeg"] != 0:
        raise SystemExit("body bone already turned")
    body.update({"angleDeg": 90, "lengthM": HEIGHT})
    body["title"] = {"en": "Bin shell and pedestal, up the tub axis from the base to the hinge pin", "de": "Behälterschale und Sockel, entlang der Behälterachse vom Boden bis zum Scharnierbolzen"}
    m = body["mass"]
    m["comX"], m["comY"] = local(m["comX"], m["comY"])
    for b in asset["rig"]["bones"]:
        if b.get("parent") == "body":
            b["x"], b["y"] = local(b["x"], b["y"])
            b["angleDeg"] = b["angleDeg"] - 90
    for a in asset["anchors"]:
        if a["bone"] == "body":
            a["x"], a["y"] = local(a["x"], a["y"])
    for rep in asset["representations"].values():
        if rep["status"] != "authored":
            continue
        for s in rep["geometry"]["elevation"]:
            if s.get("bone") == "body":
                turn_shape(s)
    plan = asset["representations"]["200"]["geometry"]["plan"]
    corners = {"lid-corner-a0", "lid-corner-b0", "lid-corner-a1", "lid-corner-b1", "lid-corner-a2", "lid-corner-b2"}
    plan[:] = [s for s in plan if s["id"] not in corners]
    for s in plan:
        if s["id"] == "lid-span":
            s.update({"x": 0, "w": 0.52})
    for load in [l for p in asset["poses"] for l in p.get("loads", [])]:
        if load.get("bone") == "body":
            raise SystemExit("load on body: restate by hand")
    return asset


if __name__ == "__main__":
    doc = L.catalog("waste-bin")
    roots(doc["asset"])
    vols = turn_volumes(copy.deepcopy(doc["asset"]["volumes"]))
    doc["asset"]["volumes"] = vols
    print(L.write_candidate("waste-bin", doc))
    if "--apply" in sys.argv:
        print(L.write_source("waste-bin", vols))
        print("written:", L.apply_members("waste-bin", doc["asset"], ["rig", "anchors", "representations"]))
