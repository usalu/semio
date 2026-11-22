"""🖊️ Multi-stem birch: the crown shapes of every 2D view redrawn over the veil crown. Per scale the elevation first, then front and plan (they are carried at the depth of their elevation twin). The outline of each bone's foliage is measured with `r3-project2d.ts` and goes over the bone's crown shape; the other filled crown shapes of that bone (the old front and back paddles) and the leaf strokes that hatched them (`-fine`, `-arcs`, `-leaves`, `-arc`) go. In front and plan the islands of one bone are one shape (they are carried through a pose together). Stems, branches, bark texture stay byte for byte. Usage: python3 rework-multi-stem-birch-2d.py 50 100 200 500"""
import glob
import json
import os
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/multi-stem-birch/🔣️.json"
BUN = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
LEAF_STROKE = re.compile(r"^(fol|f|p)-.*-(fine|arcs|leaves|arc)$")


def array_range(text, keys):
    at = 0
    for key in keys:
        at = re.compile(r'"' + key + r'"\s*:\s*').search(text, at).end()
    depth, k, quoted = 0, at, False
    while True:
        c = text[k]
        if quoted:
            k += 1 if c == "\\" else 0
            quoted = c != '"'
        elif c == '"':
            quoted = True
        elif c == "[":
            depth += 1
        elif c == "]":
            depth -= 1
            if depth == 0:
                return at, k + 1
        k += 1


def one_shape(shapes):
    """🧩 The islands of one bone's foliage as one shape with a subpath per ring: front and plan shapes are carried through a pose at the depth of the elevation shape of their id, so islands under ids of their own would drift apart from the island that has that twin."""
    rings = []
    for shape in shapes:
        rings.append(shape["d"] if shape["kind"] == "path" else "M " + " L ".join(f"{x} {y}" for x, y in shape["points"]) + " Z")
    merged = {k: v for k, v in shapes[0].items() if k not in ("points", "closed", "d", "kind")}
    merged["kind"] = "path"
    merged["d"] = " ".join(rings)
    return merged


def redraw(scale, view):
    subprocess.run([BUN, "r3-project2d.ts", "vegetation/multi-stem-birch", "--scale", scale, "--view", view, "--quiet"], cwd=HERE, check=True, stdout=subprocess.DEVNULL)
    measured = json.load(open(os.path.join(HERE, "🗑️generated", "r3-project2d", "vegetation", f"multi-stem-birch.s{scale}.{view}.json")))
    text = open(CATALOG).read()
    doc = json.loads(text)
    shapes = doc["asset"]["representations"][scale]["geometry"][view]
    crown = {s["id"] for s in shapes if s.get("region") == "leaf-crown" and s.get("filled")}
    outlines = {b["replaces"]: b for b in measured["bones"] if b["replaces"] in crown}
    if view != "elevation":
        for b in outlines.values():
            b["shapes"] = [one_shape(b["shapes"])]
    bones = {b["bone"] for b in outlines.values()}
    result = []
    for shape in shapes:
        if shape["id"] in outlines:
            result.extend(outlines[shape["id"]]["shapes"])
        elif shape["id"] in crown and shape.get("bone") in bones:
            continue
        elif not shape.get("filled") and LEAF_STROKE.match(shape["id"]):
            continue
        else:
            result.append(shape)
    doc["asset"]["representations"][scale]["geometry"][view] = result
    start, end = array_range(text, ["representations", scale, "geometry", view])
    spliced = text[:start] + json.dumps(result, ensure_ascii=False, separators=(",", ":")) + text[end:]
    assert json.loads(spliced) == doc
    open(CATALOG, "w", encoding="utf-8").write(spliced)
    print("[DEBUG]", scale, view, len(shapes), "->", len(result), "bones", len(outlines), "points", sum(len(json.dumps(s)) for b in outlines.values() for s in b["shapes"]))


if __name__ == "__main__":
    for scale in sys.argv[1:]:
        for view in ("elevation", "front", "plan"):
            redraw(scale, view)
