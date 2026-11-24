"""🪢️ person-on-crutches, 1:50: each hoodie drawstring, one bowed loft through five sections, becomes two straight lofts through its first, middle and last section (the same cord in two straight pieces, 4.8 mm thick so that each piece is at least 20 times as long as thick) with a thermal entry each, because the guard widens the one straight stroke it finds for a bowed slender solid to the bow (28.8 mm for a 6 mm cord); the side elevation draws the cord as the front already does. Writes the ops into ops/person-on-crutches.json (idempotent: an earlier copy of these ops is replaced).
python3 crutches-drawstrings.py"""
import json, os

HERE = os.path.dirname(os.path.abspath(__file__))
OPS = os.path.join(HERE, "ops", "person-on-crutches.json")
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/person-on-crutches/🔣️.json"
SECTIONS = {"l": [[0.29576, -0.07016, 0.026], [0.25443, -0.10282, 0.0286], [0.2066, -0.12357, 0.0312], [0.15744, -0.13341, 0.0338], [0.10774, -0.13897, 0.0364]]}
SECTIONS["r"] = [[u, v, -w] for u, v, w in SECTIONS["l"]]
ops = [o for o in (json.load(open(OPS, encoding="utf-8")) if os.path.exists(OPS) else []) if o.get("group") != "drawstrings"]
body = {s["id"]: s for s in json.load(open(CAT, encoding="utf-8"))["asset"]["volumes"]["50"]["body"]}
for side in "lr":
    base = {k: v for k, v in body[f"drawstring-{side}"].items() if k not in ("id", "sections")}
    points = SECTIONS[side]
    previous, previous_thermal = f"drawstring-{side}", f"th-drawstring-{side}"
    for i in range(2):
        sections = [{"at": points[2 * i], "rx": 0.0024, "ry": 0.0024}, {"at": points[2 * i + 2], "rx": 0.0024, "ry": 0.0024}]
        if i == 0:
            ops.append({"op": "solid-set", "scale": "50", "list": "body", "id": previous, "set": {"sections": sections}, "group": "drawstrings"})
            continue
        sid = f"drawstring-{side}-{i}"
        ops.append({"op": "solid-add", "scale": "50", "list": "body", "after": previous, "solid": {"id": sid, **base, "sections": sections}, "group": "drawstrings"})
        ops.append({"op": "solid-add", "scale": "50", "list": "thermal", "after": previous_thermal, "solid": {"id": f"th-{sid}", "of": sid, "region": "hoodie", "layer": "hoodie-sweat"}, "group": "drawstrings"})
        previous, previous_thermal = sid, f"th-{sid}"
for side in "lr":
    shape = {"id": f"drawstring-{side}", "bone": "torso", "role": "detail", "z": 5.3, "filled": False, "tone": 0.85, "region": "hoodie", "kind": "poly", "points": [SECTIONS["l"][k][:2] for k in (0, 2, 4)], "closed": False}
    ops.append({"op": "shape-add", "scale": "50", "view": "elevation", "after": "torso" if side == "l" else "drawstring-l", "shape": shape, "group": "drawstrings"})
open(OPS, "w", encoding="utf-8").write("[\n" + ",\n".join(json.dumps(o, ensure_ascii=False) for o in ops) + "\n]\n")
print(f"person-on-crutches: {sum(1 for o in ops if o.get('group') == 'drawstrings')} drawstring ops")
