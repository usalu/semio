"""🧭️ Writes the committed `(before, mutation, after)` specification vectors of the stdio semio model subset's relative
placement leaves (`drag-elements`, `rotate-elements`, `scale-elements`, design §12/§17.6/§20.15) into the S4-FLOWCAD stage tree.

The before-snapshot is the subset's existing committed demo-building vector (`🎛️set-element/⬅️before`); each after-snapshot
is derived here from the leaf semantics alone — translation plus offset, the Hamilton product `delta ⊗ rotation` of the unit
axis/angle turn, per-axis scale times factor — so the vectors are a statement independent of both the Rust subject and the
Python oracle of `🏛️mutate-semio-model`.

Usage (cwd = repository root): python3 T/🧪️s4-flowcad-model-vectors.py [--check]
"""

import copy
import json
import math
import os
import sys

TICKET = os.path.dirname(os.path.abspath(__file__))
SUBSET = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model"
CASE = f"{SUBSET}/🧫️fixtures/🏛️mutate-semio-model"
STAGE = os.path.join(TICKET, "🗑️generated", "s4-flowcad", "stage-model", "tree")

VECTORS = {
    "✋️drag-elements": {"mutation": "dragElements", "targets": ["wall-1"], "offset": [3.5, -1.25, 0.5]},
    "🔄️rotate-elements": {"mutation": "rotateElements", "targets": ["wall-1"], "axis": [0.0, 0.0, 1.0], "angle": math.pi / 2},
    "🔍️scale-elements": {"mutation": "scaleElements", "targets": ["wall-1"], "factors": [2.0, 1.0, 0.5]},
}


def turned(rotation, axis, angle):
    length = math.sqrt(axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2])
    sin, cos = math.sin(angle * 0.5), math.cos(angle * 0.5)
    lx, ly, lz, lw = axis[0] / length * sin, axis[1] / length * sin, axis[2] / length * sin, cos
    rx, ry, rz, rw = rotation["x"], rotation["y"], rotation["z"], rotation["w"]
    return {"x": lw * rx + lx * rw + ly * rz - lz * ry, "y": lw * ry - lx * rz + ly * rw + lz * rx, "z": lw * rz + lx * ry - ly * rx + lz * rw, "w": lw * rw - lx * rx - ly * ry - lz * rz}


def applied(before, mutation):
    after = copy.deepcopy(before)
    for element in after["elements"]:
        if element["id"] not in mutation["targets"]:
            continue
        placement = element["placement"]
        if mutation["mutation"] == "dragElements":
            for axis, delta in zip("xyz", mutation["offset"]):
                placement["translation"][axis] = placement["translation"][axis] + delta
        elif mutation["mutation"] == "rotateElements":
            placement["rotation"] = turned(placement["rotation"], mutation["axis"], mutation["angle"])
        else:
            for axis, factor in zip("xyz", mutation["factors"]):
                placement["scale"][axis] = placement["scale"][axis] * factor
    return after


def main():
    check = "--check" in sys.argv
    before = json.load(open(f"{CASE}/🎛️set-element/⬅️before/🔣️.json"))
    stale = []
    for directory, mutation in VECTORS.items():
        for role, value in (("⬅️before", before), ("🦠️mutation", mutation), ("➡️after", applied(before, mutation))):
            path = os.path.join(STAGE, CASE, directory, role, "🔣️.json")
            text = json.dumps(value, indent=2, ensure_ascii=False) + "\n"
            if check:
                if not os.path.exists(path) or open(path).read() != text:
                    stale.append(path)
            else:
                os.makedirs(os.path.dirname(path), exist_ok=True)
                open(path, "w").write(text)
    if stale:
        sys.exit("stale vectors:\n  " + "\n  ".join(stale))
    print(("checked" if check else "wrote"), len(VECTORS) * 3, "vector files")


if __name__ == "__main__":
    main()
