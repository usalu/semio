"""📐 Author pass 1 (second round) on machinery/concrete-mixer-truck: the side elevations of 1:50 and 1:100 get the steering linkage that shows between the first and second axle under the frame (relay rod, relay lever, steering box).

Usage: python3 rework-concrete-mixer-truck-elev.py   (writes the catalog file; re-read right before)
"""
import json

C = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/concrete-mixer-truck/🔣️.json"
doc = json.load(open(C))
for scale in ("50", "100"):
    elevation = doc["asset"]["representations"][scale]["geometry"]["elevation"]
    assert not any(s["id"] == "relay-rod" for s in elevation)
    base = {"bone": "chassis", "role": "detail", "z": 0.6, "filled": True, "tone": 0.4, "region": "running-gear", "material": "paint-steel"}
    elevation[:0] = [
        {"id": "relay-rod", **base, "kind": "poly", "points": [[1.75, 0.7], [3.7, 0.68], [3.7, 0.72], [1.75, 0.74]], "closed": True},
        {"id": "relay-lever", **base, "kind": "rect", "x": 1.72, "y": 0.61, "w": 0.06, "h": 0.26},
        {"id": "steering-box", **base, "kind": "rect", "x": 3.62, "y": 0.69, "w": 0.2, "h": 0.22},
    ]
json.dump(doc, open(C, "w"), ensure_ascii=False, indent=1)
print("elevations redrawn")
