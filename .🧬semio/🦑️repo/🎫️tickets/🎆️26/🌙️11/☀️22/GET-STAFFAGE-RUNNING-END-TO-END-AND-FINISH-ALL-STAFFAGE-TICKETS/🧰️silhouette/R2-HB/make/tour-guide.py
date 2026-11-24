"""🚩️ Volume part of the repair of `tour-guide` 1:50: the flag pole (a capsule of 24 mm, three smallest features) loses its `slender` mark and is compared by its own outline; merged into ops/tour-guide.json. `python3 make/tour-guide.py`"""
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import lib

asset = lib.asset_of("tour-guide")
pole = dict(next(s for s in asset["volumes"]["50"]["body"] if s["id"] == "pole"))
pole.pop("slender", None)
path = os.path.join(lib.HERE, "ops", "tour-guide.json")
ops = [op for op in json.load(open(path, encoding="utf8")) if not (op["op"] == "solid" and op.get("id") == "pole")]
ops.append({"op": "solid", "scale": "50", "id": "pole", "solid": pole})
lib.write_ops("tour-guide", ops)
