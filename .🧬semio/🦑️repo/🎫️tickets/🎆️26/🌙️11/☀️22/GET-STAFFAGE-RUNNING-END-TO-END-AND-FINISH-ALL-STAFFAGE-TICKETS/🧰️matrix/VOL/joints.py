#!/usr/bin/env python3
"""🪡️ The joints between repeated boxes whose copies shared whole faces (guard 🧊️model-catalog, check `closed`): every copy gives up 1 mm at its joint, as the butted parts do (push nipple joint of two boiler sections, seam of two radiator covers, seam of two carton flaps). Idempotent, writes only the size numbers of the named solids: joints.py <asset document>"""
import json, re, sys
FIX = {
    "used-cast-iron-sectional-boiler": (("th-section-middle", 0, 0.12, 0.119), ("member-section-middle", 0, 0.12, 0.119)),
    "used-panel-radiator-pallet": (("stack-cover-left", 2, 0.1, 0.099), ("stack-cover-right", 2, 0.1, 0.099)),
    "used-moving-boxes": (("b1-major-flaps", 2, 0.165, 0.164), ("b2-major-flaps", 2, 0.165, 0.164)),
}
path = sys.argv[1]
text = before = open(path, encoding="utf-8").read()
asset = json.loads(text)["asset"]
for name, axis, old, new in FIX[asset["id"]]:
    def put(m):
        numbers = re.findall(r"-?[\d.]+", m[2])
        repeat = text[m.end():m.end() + 260]
        if float(numbers[axis]) != old or '"repeat"' not in repeat.split('"id"')[0]: return m[0]
        parts = re.split(r"(-?[\d.]+)", m[2]); parts[1 + 2 * axis] = repr(new)
        return m[1] + "".join(parts) + m[3]
    text = re.sub(r'("id": "%s",(?:(?!"id")[^\[])*?"size": \[)([^\]]*)(\])' % re.escape(name), put, text)
if text != before: open(path, "w", encoding="utf-8").write(text)
print(asset["id"], sum(a != b for a, b in zip(before, text)) if len(before) == len(text) else f"{len(before)} > {len(text)} bytes", "written" if text != before else "unchanged")
