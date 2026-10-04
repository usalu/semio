#!/usr/bin/env python3
"""📐️ S4-PUZZLE: completes the `x-semio-ui` of the puzzle 2d edge geometry inputs (`connect-handles`, `replace-edge-geometry`)
with the units of the semio connection they carry into puzzle 3d/5d: gap/shift/rise in metres, rotation/turn/tilt as degree dials
with quarter-turn detents (the same facets as puzzle 3d `connect-vortices`). Labels stay. Idempotent; `--check` only reports."""
import json
import sys

ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
KEY_ORDER = ("widget", "role", "label", "description", "ref", "unit", "displayUnit", "displayFactor", "step", "precision", "softMin", "softMax", "scale", "snaps", "snapSource", "options", "group", "order")
LEAVES = {"🪢️connect-handles": ("gap", "shift", "rise", "rotation", "turn", "tilt"), "🧮replace-edge-geometry": ("newGap", "newShift", "newRise", "newRotation", "newTurn", "newTilt")}
LENGTH = {"widget": "stepper", "unit": "m", "step": 0.01, "precision": 3}
ANGLE = {"widget": "dial", "unit": "deg", "step": 1, "precision": 1, "softMin": -180, "softMax": 180, "snaps": [-180, -90, 0, 90, 180]}


def facets(ui, extra):
    body = {**ui, **extra}
    return {key: body[key] for key in KEY_ORDER if key in body}


def main():
    check = "--check" in sys.argv
    pending = 0
    for leaf, fields in LEAVES.items():
        path = f"{ROOT}/{leaf}/🧬️schema/🔣️.json"
        text = open(path, encoding="utf-8").read()
        schema = json.loads(text)
        for index, field in enumerate(fields):
            node = schema["properties"][field]
            node["x-semio-ui"] = facets(node["x-semio-ui"], LENGTH if index < 3 else ANGLE)
        written = json.dumps(schema, indent=2, ensure_ascii=False) + "\n"
        if written != text:
            pending += 1
            print(f"{'pending' if check else 'wrote'} {leaf}")
            if not check:
                open(path, "w", encoding="utf-8").write(written)
    print(f"{pending} leaf schema(s) {'pending' if check else 'rewritten'}")
    sys.exit(1 if check and pending else 0)


if __name__ == "__main__":
    main()
