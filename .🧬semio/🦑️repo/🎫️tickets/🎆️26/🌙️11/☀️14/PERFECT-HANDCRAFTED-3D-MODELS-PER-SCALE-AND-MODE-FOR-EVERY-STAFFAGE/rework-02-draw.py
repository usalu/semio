"""✏️ Redraws the 2D shapes of an asset from the stations of its reworked 3D body (author 02 rework).

Usage: python3 rework-02-draw.py <category/slug> [scale ...]
1. The side elevation outlines are ruled with `r3-project2d.ts --candidate` and written.
2. Every shape with a `solid` placement (all front shapes, most plan shapes, the arms of the elevation) is redrawn here from the very
   stations of the solids it depicts: the profile of an extruded shape is the envelope of the solids over the bone axis, its span their
   extent across; a revolved shape is the half breadth over the bone axis about the mean centre line.
3. The plain plan outlines are ruled with `r3-project2d.ts --candidate` and written.
Strokes (hems, laces, face lines) are placed by the asset's own stroke script afterwards.
"""
import glob
import importlib.util
import json
import math
import os
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(HERE, "rework-02-lib.py"))
lib = importlib.util.module_from_spec(spec)
spec.loader.exec_module(lib)
BUN = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
ALIAS = {"humans/jogger": {"hair-top": ("hair", 0.110, None)}, "humans/running-child": {"hair-top": ("hair", 0.062, None)}, "humans/runner-with-blade-prosthesis": {"hair-top": ("hair", 0.085, None)}}


DROP = {"humans/runner-with-blade-prosthesis": ("fingers-finger", "fingers-fingertip")}


def spans(solid, u, axis):
    """📏️ The interval a solid covers across `axis` (1 = v, 2 = w) at the station u, or None."""
    kind = solid["kind"]
    if kind == "loft":
        secs = solid["sections"]
        if secs[0]["at"][0] > secs[-1]["at"][0]:
            secs = secs[::-1]
        for a, b in zip(secs, secs[1:]):
            if a["at"][0] - 1e-9 <= u <= b["at"][0] + 1e-9:
                t = (u - a["at"][0]) / ((b["at"][0] - a["at"][0]) or 1)
                c = a["at"][axis] + (b["at"][axis] - a["at"][axis]) * t
                key = "ry" if axis == 1 else "rx"
                r = a[key] + (b[key] - a[key]) * t
                return c - r, c + r
        return None
    balls = []
    if kind == "capsule":
        for i in range(25):
            t = i / 24
            balls.append(([solid["p0"][k] + (solid["p1"][k] - solid["p0"][k]) * t for k in range(3)], [solid["r0"] + (solid["r1"] - solid["r0"]) * t] * 3))
    elif kind == "ellipsoid":
        balls.append((solid["transform"]["translation"], solid["radii"]))
    elif kind == "box":
        c, s = solid["transform"]["translation"], solid["size"]
        return (c[axis] - s[axis] / 2, c[axis] + s[axis] / 2) if abs(u - c[0]) <= s[0] / 2 else None
    lo = hi = None
    for c, r in balls:
        d = (u - c[0]) / r[0]
        if abs(d) < 1:
            half = r[axis] * math.sqrt(1 - d * d)
            lo = c[axis] - half if lo is None else min(lo, c[axis] - half)
            hi = c[axis] + half if hi is None else max(hi, c[axis] + half)
    return None if lo is None else (lo, hi)


def reach(solid):
    """📏️ The stations a solid spans along the bone axis."""
    if solid["kind"] == "loft":
        us = [s["at"][0] for s in solid["sections"]]
        return min(us), max(us), us
    if solid["kind"] == "capsule":
        a, b = solid["p0"][0] - solid["r0"], solid["p1"][0] + solid["r1"]
        a2, b2 = solid["p0"][0] + solid["r0"], solid["p1"][0] - solid["r1"]
        return min(a, b, a2, b2), max(a, b, a2, b2), []
    c = solid["transform"]["translation"][0]
    r = solid["radii"][0] if solid["kind"] == "ellipsoid" else solid["size"][0] / 2
    return c - r, c + r, []


def envelope(solids, axis, u0=None, u1=None):
    """🖊️ Stations with the lowest and highest reach of a group of solids across `axis`."""
    lo_u = min(reach(s)[0] for s in solids) if u0 is None else u0
    hi_u = max(reach(s)[1] for s in solids) if u1 is None else u1
    us = {lo_u + (hi_u - lo_u) * i / 96 for i in range(97)}
    for s in solids:
        us.update(u for u in reach(s)[2] if lo_u <= u <= hi_u)
    rows = []
    for u in sorted(us):
        found = [x for x in (spans(s, u, axis) for s in solids) if x]
        if found:
            rows.append((u, min(x[0] for x in found), max(x[1] for x in found)))
    return rows


def simplify(points, tolerance):
    """✂️ Douglas-Peucker on an open chain."""
    if len(points) < 3:
        return points
    (x0, y0), (x1, y1) = points[0], points[-1]
    length = math.hypot(x1 - x0, y1 - y0) or 1e-12
    worst, index = 0, 0
    for i in range(1, len(points) - 1):
        d = abs((x1 - x0) * (y0 - points[i][1]) - (x0 - points[i][0]) * (y1 - y0)) / length
        if d > worst:
            worst, index = d, i
    if worst <= tolerance:
        return [points[0], points[-1]]
    return simplify(points[: index + 1], tolerance)[:-1] + simplify(points[index:], tolerance)


def r4(points):
    return [[round(x, 4), round(y, 4)] for x, y in points]


def bone_z(rig):
    """🦴️ The lateral rest offset of every bone (its own `z` plus its ancestors')."""
    bones = {b["id"]: b for b in rig["bones"]}
    out = {}
    for id in bones:
        z, b = 0, bones[id]
        while b:
            z += b.get("z", 0)
            b = bones.get(b["parent"]) if b["parent"] else None
        out[id] = z
    return out


def redraw_solids(asset, scale, alias):
    """🧊️ Step 2: every shape with a `solid` placement from the stations of its solids."""
    document = lib.load_catalog(asset)
    source = lib.load_source(asset)
    body = source["volumes"][scale]["body"]
    by_id = {s["id"]: s for s in body}
    geometry = document["asset"]["representations"][scale]["geometry"]
    zs = bone_z(document["asset"]["rig"])
    tolerance = 0.15 * int(scale) / 1000 / 4
    report = []
    for view in ("elevation", "front", "plan"):
        geometry[view] = [s for s in geometry[view] if s["id"] not in DROP.get(asset, ())]
        shapes = geometry[view]
        claimed = {s["id"] for s in shapes if s["id"] in by_id} | {alias[s["id"]][0] for s in shapes if s["id"] in alias}
        largest = {}
        for s in shapes:
            if s.get("filled") and s.get("role") == "body" and s["id"] in by_id:
                size = reach(by_id[s["id"]])
                if s["bone"] not in largest or size[1] - size[0] > largest[s["bone"]][1]:
                    largest[s["bone"]] = (s["id"], size[1] - size[0])
        for shape in shapes:
            solid = shape.get("solid")
            if not solid or not shape.get("filled"):
                continue
            u0 = u1 = None
            if shape["id"] in alias:
                target, u0, u1 = alias[shape["id"]]
                group = [by_id[target]]
            elif shape["id"] in by_id:
                group = [by_id[shape["id"]]]
                if largest.get(shape["bone"], (None,))[0] == shape["id"]:
                    group += [s for s in body if s["bone"] == shape["bone"] and s["id"] not in claimed]
            elif shape["id"].endswith("-core") and shape["id"][:-5] in by_id and solid["kind"] == "extruded" and solid["sweep"] == "v":
                profile, across = envelope([by_id[shape["id"][:-5]]], 2), envelope([by_id[shape["id"][:-5]]], 1)
                shape["points"] = r4(simplify([(r[0], r[2] * 0.6) for r in profile], tolerance) + simplify([(r[0], r[1] * 0.6) for r in profile][::-1], tolerance))
                low, high = min(r[1] for r in across), max(r[2] for r in across)
                solid["z0"], solid["z1"] = round(low + (high - low) * 0.2, 4), round(high - (high - low) * 0.2, 4)
                solid.pop("zTip", None)
                continue
            else:
                report.append(f"{view}:{shape['id']} kept")
                continue
            if solid["kind"] == "extruded" and solid["sweep"] == "w":
                rows = envelope(group, 2, u0, u1)
                solid["z0"], solid["z1"] = round(zs[shape["bone"]] + min(r[1] for r in rows), 4), round(zs[shape["bone"]] + max(r[2] for r in rows), 4)
                if view != "elevation":
                    profile = envelope(group, 1, u0, u1)
                    shape["points"] = r4(simplify([(r[0], r[2]) for r in profile], tolerance) + simplify([(r[0], r[1]) for r in profile][::-1], tolerance))
            elif solid["kind"] == "extruded" and solid["sweep"] == "v":
                profile, across = envelope(group, 2, u0, u1), envelope(group, 1, u0, u1)
                shape["points"] = r4(simplify([(r[0], r[2]) for r in profile], tolerance) + simplify([(r[0], r[1]) for r in profile][::-1], tolerance))
                solid["z0"], solid["z1"] = round(min(r[1] for r in across), 4), round(max(r[2] for r in across), 4)
                if "zTip" in solid:
                    tip = across[-max(2, len(across) // 6):]
                    solid["zTip"] = [round(min(r[1] for r in tip), 4), round(max(r[2] for r in tip), 4)]
            elif solid["kind"] == "revolved" and solid["axis"] == "u":
                profile, across = envelope(group, 2, u0, u1), envelope(group, 1, u0, u1)
                axis_v = round(sum((r[1] + r[2]) / 2 for r in across) / len(across), 4)
                axis_w = round(sum((r[1] + r[2]) / 2 for r in profile) / len(profile), 4)
                half = simplify([(r[0], axis_v + (r[2] - r[1]) / 2) for r in profile], tolerance)
                shape["points"] = r4([(half[0][0], axis_v)] + half + [(half[-1][0], axis_v)])
                solid["axisX"], solid["axisZ"], solid["mirror"] = axis_v, axis_w, True
            else:
                report.append(f"{view}:{shape['id']} {solid['kind']} kept")
    fresh = lib.load_catalog(asset)
    fresh["asset"]["representations"][scale]["geometry"] = geometry
    lib.save_catalog(asset, fresh)
    return report


def rule(asset, scale, view):
    """📐️ Steps 1 and 3: the outlines of one view ruled by the shared tool and written."""
    run = subprocess.run([BUN, "r3-project2d.ts", asset, "--scale", scale, "--view", view, "--candidate", "--quiet"], cwd=HERE, capture_output=True, text=True)
    candidate = os.path.join(HERE, "🗑️generated", "r3-project2d", asset + f".s{scale}.{view}", "🔣️.json")
    with open(candidate, encoding="utf-8") as f:
        shapes = json.load(f)["asset"]["representations"][scale]["geometry"][view]
    document = lib.load_catalog(asset)
    document["asset"]["representations"][scale]["geometry"][view] = shapes
    lib.save_catalog(asset, document)
    return run.returncode


def committed(asset):
    """🗄️ The last committed catalog document: the source of shapes the ruling dropped."""
    path = os.path.relpath(lib.catalog_path(asset), "/Users/ueli/Documents/semio")
    return json.loads(subprocess.run(["git", "show", "HEAD:" + path], cwd="/Users/ueli/Documents/semio", capture_output=True, text=True, check=True).stdout)


def mend(asset, scale, regions):
    """🩹️ After a ruling: a new outline takes the id of its bone, a dropped filled shape of a solid that still exists is drawn again from its stations."""
    document = lib.load_catalog(asset)
    volume = lib.load_source(asset)["volumes"][scale]
    body = {s["id"]: s for s in volume["body"]}
    regions = {**{s["of"]: s["region"] for s in volume["thermal"] if "of" in s}, **{s["id"][3:]: s["region"] for s in volume["thermal"] if s["id"].startswith("th-")}, **regions}
    before = committed(asset)["asset"]["representations"][scale]["geometry"]
    geometry = document["asset"]["representations"][scale]["geometry"]
    tolerance = 0.15 * int(scale) / 1000 / 4
    notes = []
    for view in ("elevation", "plan"):
        shapes = geometry[view]
        for shape in shapes:
            if shape["id"].startswith("r3-new-"):
                bone = shape["id"][7:]
                solid = body.get(bone) or next(s for s in body.values() if s["bone"] == bone)
                shape.update(id=bone, role="body", tone=solid["tone"], material=solid["material"], region=regions.get(bone, shape.get("region")))
                notes.append(f"{view}:{bone} named")
        have = {s["id"] for s in shapes}
        if view == "elevation":
            for old in before[view]:
                if old["id"] not in have and old.get("filled") and old["id"] in body:
                    rows = envelope([body[old["id"]]], 1)
                    old["points"] = r4(simplify([(r[0], r[2]) for r in rows], tolerance) + simplify([(r[0], r[1]) for r in rows][::-1], tolerance))
                    shapes.append(old)
                    shapes.sort(key=lambda s: s["z"])
                    notes.append(f"{view}:{old['id']} redrawn")
    fresh = lib.load_catalog(asset)
    fresh["asset"]["representations"][scale]["geometry"] = geometry
    lib.save_catalog(asset, fresh)
    return notes


REGIONS = {}


def main():
    asset = sys.argv[1]
    for scale in sys.argv[2:] or ["50", "100", "200", "500"]:
        a = rule(asset, scale, "elevation")
        print("[DEBUG]", mend(asset, scale, REGIONS))
        report = redraw_solids(asset, scale, ALIAS.get(asset, {}))
        b = rule(asset, scale, "plan")
        print("[DEBUG]", mend(asset, scale, REGIONS))
        print(f"[DEBUG] {asset} 1:{scale}: elevation {a}, plan {b}; {'; '.join(report)}")


if __name__ == "__main__":
    main()
