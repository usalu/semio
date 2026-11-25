"""🌬️ Ops writer of `used-air-water-heat-pump-monobloc`: at 1:500 the unit is one block of 0.56 m depth while the stands under it are 0.60 m long and the fan grille, 40 mm proud of the casing, carries the emitter `fan-outlet`: the emitter lay 40 mm off the block and, once the pallet is gone, the plan ended 40 mm short of the side elevation. The band merges every unit region into the block and omits details below 75 mm, so the block takes the grille in: 0.60 m deep in side elevation, plan and in the body of the 3D model of that scale, as long as the stands; the decision text says so: `python3 make/used-air-water-heat-pump-monobloc.py` writes its ops."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from mc import *
write("used-air-water-heat-pump-monobloc", [
    {"op": "shape-set", "scale": "500", "view": "elevation", "id": "unit-block", "set": {"w": 0.6}},
    {"op": "shape-set", "scale": "500", "view": "plan", "id": "unit-block", "set": {"w": 0.6}},
    {"op": "solid-set", "scale": "500", "id": "unit-block", "set": {"size": [0.6, 1.3, 1.12], "transform": {"translation": [0.3, 0.65, 0.0]}}},
    {"op": "note", "scale": "500", "decision": "merged", "en": "The block takes in the fan grille, 40 mm proud of the casing: 0.60 m deep, as long as the stands.", "de": "Der Block schließt das 40 mm vorstehende Lüftergitter ein: 0,60 m tief, so lang wie die Böcke."},
])
