"""🚪️ Ops writer of `sliding-door`: the attachment `carriage`, one anchor at the middle of the track for the two trolleys the leaf hangs from, 100 mm off every drawn part of the leaf, becomes one anchor per trolley on its hanger (D45): `python3 make/sliding-door.py` writes `ops/sliding-door.json`."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from mc import *
a = asset("sliding-door")
parts = [
    {"id": "carriage-west", "bone": "leaf", "x": 0.075, "y": -0.35, "kind": "attachment", "title": {"en": "West trolley on the track (prismatic joint)", "de": "Westlicher Laufwagen an der Schiene (Prismengelenk)"}},
    {"id": "carriage-east", "bone": "leaf", "x": 0.075, "y": 0.35, "kind": "attachment", "title": {"en": "East trolley on the track (prismatic joint)", "de": "Östlicher Laufwagen an der Schiene (Prismengelenk)"}},
]
write("sliding-door", [put(["anchors"], split_anchor(a, "carriage", parts))])
