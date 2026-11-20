"""🪛 Round 3 root fixes of objects/waste-bin outside `volumes`: rig (tub floor), 2D shapes redrawn to the true view of the handcrafted body (ruling E1).

Usage: python3 r3-waste-bin-roots.py <out.json>   (candidate; pass the catalog file itself to write it after `r3-validate.ts --file` says VALID)
Re-reads the catalog file right before writing and changes only the listed members; idempotent.
"""
import copy
import re
import json
import sys
from pathlib import Path

CAT = Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/waste-bin/🔣️.json")

PED = "M -0.2 0.008 L 0.2 0.008 L 0.2 0.017 L 0.19 0.025 L -0.19 0.025 L -0.2 0.017 Z"
LID200 = "M 0 0 L 0.52 0 L 0.52 0.006 C 0.52 0.03 0.45 0.04 0.4 0.038 L 0.12 0.038 C 0.05 0.04 0 0.03 0 0.006 Z"
COLLAR50 = "M -0.2552 0.951 C -0.2 0.943 -0.1 0.945 0.0 0.941 C 0.1 0.945 0.2 0.943 0.2552 0.951"
COLLAR50F = "M -0.2438 0.95 C -0.2 0.943 -0.1 0.945 0.0 0.941 C 0.1 0.945 0.2 0.943 0.2438 0.95"

EDITS = {
    "50": {
        "elevation": {
            "pedestal": {"d": PED},
            "hinge-bracket": {"w": 0.021, "h": 0.062},
            "hinge-barrel": {"cy": 0.0, "rx": 0.009, "ry": 0.009},
            "liner-collar": {"d": COLLAR50},
            "lid-skirt": {"x": 0.012, "w": 0.508},
        },
        "front": {
            "pedestal": {"d": PED},
            "hinge-bracket-r": {"x": -0.22, "w": 0.06, "h": 0.062},
            "hinge-bracket-l": {"x": 0.16, "w": 0.06, "h": 0.062},
            "hinge-barrel": {"cy": 0.0, "rx": 0.009, "ry": 0.009, "solid": {"kind": "extruded", "sweep": "w", "z0": -0.155, "z1": 0.155}},
            "liner-collar": {"d": COLLAR50F},
            "lid-skirt": {"x": 0.012, "w": 0.508},
        },
        "plan": {
            "hinge-bracket-l": {"w": 0.021},
            "hinge-bracket-r": {"w": 0.021},
            "hinge-barrel": {"cy": 0.0, "rx": 0.009, "ry": 0.009, "solid": {"kind": "extruded", "sweep": "w", "z0": -0.155, "z1": 0.155}},
            "lid-skirt": {"x": 0.012, "w": 0.508},
            "lid-corner-a0": {"solid": {"kind": "extruded", "sweep": "w", "z0": -0.157, "z1": 0.157}},
        },
    },
    "100": {
        "elevation": {"hinge-bracket": {"w": 0.021, "h": 0.062}},
        "front": {
            "hinge-bracket-r": {"x": -0.22, "w": 0.06, "h": 0.062},
            "hinge-bracket-l": {"x": 0.16, "w": 0.06, "h": 0.062},
        },
        "plan": {"hinge-bracket-l": {"w": 0.021}, "hinge-bracket-r": {"w": 0.021}},
    },
    "200": {"elevation": {"lid": {"d": LID200}}, "front": {"lid": {"d": LID200}}},
}


def apply(doc):
    """🔧 Applies the rig and 2D edits to an asset document in place."""
    a = doc["asset"]
    for b in a["rig"]["bones"]:
        if b["id"] == "base-plate":
            b["y"] = 0.025
    reps = a["representations"]
    for scale, views in EDITS.items():
        for view, shapes in views.items():
            geo = reps[scale]["geometry"][view]
            ids = {s["id"]: s for s in geo}
            for sid, patch in shapes.items():
                ids[sid].update(copy.deepcopy(patch))
    plan = reps["50"]["geometry"]["plan"]
    if not any(s["id"] == "lid-corner-a0-l" for s in plan):
        i = next(k for k, s in enumerate(plan) if s["id"] == "lid-corner-a0")
        base = plan[i]
        extra = []
        for suffix, z0, z1 in (("l", 0.223, 0.24), ("r", -0.24, -0.223)):
            s = copy.deepcopy(base)
            s["id"] = f"lid-corner-a0-{suffix}"
            s["solid"] = {"kind": "extruded", "sweep": "w", "z0": z0, "z1": z1}
            extra.append(s)
        plan[i + 1:i + 1] = extra
    dec = reps["50"]["decisions"]
    for key in dec:
        for lang in ("en", "de"):
            t = dec[key][lang]
            t = t.replace("the 0.26 m hinge barrel", "the 0.31 m hinge barrel between the lugs, the lid notched over the lugs").replace("die 0,26 m lange Scharnierwalze", "die 0,31 m lange Scharnierwalze zwischen den Laschen, der Deckel über den Laschen ausgespart")
            dec[key][lang] = t
    return doc


def member_span(text, key):
    """📍 Character span of a direct member of `asset` (four-space indent) including its value."""
    m = re.search(r'\n    "' + key + r'": ', text)
    i, depth, quoted = m.end(), 0, False
    while True:
        c = text[i]
        if quoted:
            if c == "\\":
                i += 1
            elif c == '"':
                quoted = False
        elif c == '"':
            quoted = True
        elif c in "{[":
            depth += 1
        elif c in "}]":
            depth -= 1
            if depth == 0:
                return m.end(), i + 1
        i += 1


def main():
    """🚀 Reads the catalog right now, applies the edits, replaces only the rig and representations members in the text, writes the target."""
    out = Path(sys.argv[1])
    text = CAT.read_text()
    doc = json.loads(text)
    apply(doc)
    for key in ("rig", "representations"):
        a, b = member_span(text, key)
        value = json.dumps(doc["asset"][key], ensure_ascii=False, indent=2).replace("\n", "\n    ")
        text = text[:a] + value + text[b:]
    check = json.loads(text)
    if check != doc:
        raise SystemExit("textual splice does not round-trip")
    if out == CAT and CAT.read_text() != CAT.read_text():
        raise SystemExit("changed under us")
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(text)
    print(out)


if __name__ == "__main__":
    main()
