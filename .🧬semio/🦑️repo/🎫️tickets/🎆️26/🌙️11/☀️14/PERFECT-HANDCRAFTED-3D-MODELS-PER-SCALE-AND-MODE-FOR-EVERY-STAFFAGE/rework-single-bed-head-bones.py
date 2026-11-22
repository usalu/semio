"""🛏️ single-bed: turn the bones `slat-head` and `mat-head` round so that they run from the hinge (x = 0.78, y = 0.23) to the head end.

Hand-chosen change: rest `angleDeg` of `slat-head` 0 -> 180 (u towards the head board, v down, w left as before). Every coordinate that
is written in the frame of one of the two bones is turned by the same half turn about w, (x, y, z) -> (-x, -y, z), so nothing moves in the
world. `mat-head` keeps `angleDeg` 0 under its parent; its prismatic travel is along the parent's v, which now points down into the slats:
`axisDeg` 90, travel 0..0.06 m, pose `dy` positive = sinking.

usage: python3 rework-single-bed-head-bones.py candidate|write   (after write: volumes-insert.py puts the volumes back into the catalog file)
"""
import glob
import json
import re
import sys

T = __file__.rsplit("/", 1)[0]
CATALOG = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/single-bed/*.json")[0]
SOURCE = T + "/volumes/furniture/single-bed.json"
CANDIDATE = T + "/🗑️generated/r3-single-bed/🔣️.json"
BONES = ("slat-head", "mat-head")


def r(value):
    """🔢 A negated coordinate without float noise or a negative zero."""
    out = round(-value, 6)
    return 0.0 if out == 0 else out


def turn_point(p):
    """🔄 (x, y, z) -> (-x, -y, z)."""
    return [r(p[0]), r(p[1])] + list(p[2:])


def turn_path(d):
    """✏️ An absolute M/L/Q/C/Z path with every coordinate negated."""
    tokens = d.split()
    assert all(t in "MLQCZ" or re.fullmatch(r"-?[0-9.]+", t) for t in tokens), d
    return " ".join(t if t in "MLQCZ" else repr(r(float(t))) for t in tokens)


def turn_solid(s):
    """🧊 One solid of a turned bone."""
    kind = s["kind"]
    assert "rotation" not in s and "rotation" not in s.get("transform", {}), s["id"]
    if "repeat" in s:
        assert set(s["repeat"]) == {"count", "translation"}, s["id"]
        s["repeat"]["translation"] = turn_point(s["repeat"]["translation"])
    if kind == "loft":
        for section in s["sections"]:
            assert set(section) <= {"at", "rx", "ry", "exponent"}, s["id"]
            section["at"] = turn_point(section["at"])
    elif kind in ("box", "ellipsoid"):
        assert set(s["transform"]) == {"translation"}, s["id"]
        s["transform"]["translation"] = turn_point(s["transform"]["translation"])
    elif kind == "extrude":
        assert s["plane"] == "xy" and "holes" not in s, s["id"]
        s["profile"] = [turn_point(p) for p in s["profile"]]
    else:
        raise AssertionError(kind + " " + s["id"])


def turn_shape(s):
    """📐 One 2D shape drawn in the frame of a turned bone (elevation, or a `solid` profile in front and plan)."""
    kind = s["kind"]
    if kind == "rect":
        s["x"], s["y"] = r(s["x"] + s["w"]), r(s["y"] + s["h"])
    elif kind == "poly":
        s["points"] = [turn_point(p) for p in s["points"]]
    elif kind == "path":
        s["d"] = turn_path(s["d"])
    else:
        raise AssertionError(kind + " " + s["id"])
    if "solid" in s:
        assert s["solid"]["kind"] == "extruded" and s["solid"]["sweep"] == "w", s["id"]


def main(mode):
    document = json.load(open(CATALOG))
    asset = document["asset"]
    if asset["volumes"] != json.load(open(SOURCE))["volumes"]:
        raise SystemExit("source and catalog volumes differ: publish first")
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    if bones["slat-head"]["angleDeg"] == 180:
        raise SystemExit("already turned")
    slat, mat = bones["slat-head"], bones["mat-head"]
    slat["angleDeg"] = 180
    slat["mass"]["comX"], slat["mass"]["comY"] = 0.36, -0.015
    mat["y"] = -0.055
    mat["joint"] = {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.06}
    mat["mass"]["comX"], mat["mass"]["comY"] = 0.36, -0.1
    for anchor in asset["anchors"]:
        assert anchor["bone"] not in BONES, anchor["id"]
    for pose in asset["poses"]:
        offset = pose["bones"].get("mat-head")
        if offset is not None:
            assert set(offset) == {"dy"}
            offset["dy"] = r(offset["dy"])
        for load in pose["loads"]:
            span = load.get("distributed")
            if span is not None and span["bone"] in BONES:
                span["fromM"], span["toM"] = r(span["toM"]), r(span["fromM"])
            assert load.get("bone") not in BONES, load["id"]
    for clip in asset["clips"]:
        for track in clip["tracks"]:
            assert not any(b in track["target"] for b in BONES), track["target"]
    for representation in asset["representations"].values():
        if "geometry" not in representation:
            continue
        for view in ("elevation", "front", "plan"):
            for shape in representation["geometry"][view]:
                if shape.get("bone") in BONES:
                    assert view == "elevation" or "solid" in shape, shape["id"]
                    turn_shape(shape)
    for scale in asset["volumes"].values():
        for name in ("body", "thermal", "structure"):
            for solid in scale.get(name, []):
                if solid.get("bone") in BONES and "of" not in solid:
                    turn_solid(solid)
    volumes = asset.pop("volumes")
    if mode == "candidate":
        asset["volumes"] = volumes
        open(CANDIDATE, "w").write(json.dumps(document, ensure_ascii=False, indent=2) + "\n")
    elif mode == "write":
        open(SOURCE, "w").write(json.dumps({"volumes": volumes}, ensure_ascii=False, indent=1))
        open(CATALOG, "w").write(json.dumps(document, ensure_ascii=False, indent=2) + "\n")
    else:
        raise SystemExit(__doc__)
    print("turned", mode)


if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else "")
