"""🧪 Writes the geometry-only draft of one species module for measuring and previewing (PW-09): `python3 draft.py <module> <out.json> [<measure.json>]`."""
import importlib
import sys

from morph import dumps, load

g = importlib.import_module(sys.argv[1])
doc = load(g.TEMPLATE)
g.morph(doc["asset"])
if len(sys.argv) > 3:
    import json
    from morph import measured_dimensions
    doc["asset"]["dimensions"].update(measured_dimensions(json.load(open(sys.argv[3]))))
    doc["asset"]["dimensions"]["massKg"] = round(sum(b.get("mass", {}).get("kg", 0) for b in doc["asset"]["rig"]["bones"]))
open(sys.argv[2], "w", encoding="utf-8").write(dumps(doc))
