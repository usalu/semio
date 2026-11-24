"""🗒️ The drawn shapes of the views of a person at a scale beside the body solids: per shape its bone, kind, sweep and whether a body solid carries its id; then the body solids no view draws under their id. `python3 shapes.py <slug> <scale> [view]`"""
import sys

import lib

slug, scale = sys.argv[1:3]
asset = lib.asset_of(slug)
body = {s["id"]: s for s in asset["volumes"][scale]["body"]}
for view in ([sys.argv[3]] if len(sys.argv) > 3 else ["elevation", "front", "plan"]):
    shapes = asset["representations"][scale]["geometry"][view]
    print(f"== {view}: {len(shapes)} shapes")
    for s in shapes:
        solid = s.get("solid")
        sweep = "flat" if solid is None else solid["kind"] + (" " + solid["sweep"] + f" {solid['z0']}..{solid['z1']}" if solid["kind"] == "extruded" else " " + solid.get("axis", "") + "@" + solid.get("axisBone", ""))
        print(f"  {s['id']}[{s.get('bone')}] {s['role']} {s['kind']} {'filled' if s.get('filled', True) is not False else 'open pen ' + str(s.get('penMm'))} z {s.get('z')} {sweep} {'=solid ' + body[s['id']]['kind'] if s['id'] in body else ''}")
    drawn = {s["id"] for s in shapes}
    print("  undrawn solids: " + " ".join(f"{i}[{b['bone']}:{b['kind']}{'~' if b.get('slender') else ''}{' tex' if b.get('texture') else ''}]" for i, b in body.items() if i not in drawn))
