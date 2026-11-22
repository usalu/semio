"""🗺️ Author pass 1 (second round) on machinery/concrete-mixer-truck: the 1:50 plan drawing gets the parts the model shows from above and the drawing lacked (battery box, air tanks, lamp beam at its true depth, ladder foot, rear frame gussets, relay rod and second drag link of the steering).

Usage: python3 rework-concrete-mixer-truck-plan50.py   (writes the catalog file; re-read right before)
"""
import json

C = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/concrete-mixer-truck/🔣️.json"
doc = json.load(open(C))
plan = doc["asset"]["representations"]["50"]["geometry"]["plan"]
ids = {s["id"] for s in plan}


def rect(sid, z, x, y, w, h, tone=0.3, region="chassis", material="paint-steel"):
    assert sid not in ids, sid
    return {"id": sid, "bone": "chassis", "role": "detail", "z": z, "filled": True, "tone": tone, "kind": "rect", "x": x, "y": y, "w": w, "h": h, "region": region, "material": material}


beam = next(s for s in plan if s["id"] == "tail-lamp-beam")
beam.update(x=-4.43, w=0.1)
plan[:0] = [
    rect("battery-box", 1.55, 0.35, 0.5, 0.3, 0.56),
    rect("air-tanks", 1.55, -0.625, -0.865, 1.07, 0.25),
    rect("ladder-foot", 1.3, -4.39, 0.78, 0.2, 0.46),
    rect("rear-frame-gusset-left", 1.42, -3.66, 0.29, 0.3, 0.3),
    rect("rear-frame-gusset-right", 1.42, -3.66, -0.59, 0.3, 0.3),
    rect("relay-rod", 0.6, 1.75, 0.48, 1.95, 0.085, tone=0.4, region="running-gear"),
    {"id": "drag-link-2", "bone": "chassis", "role": "detail", "z": 0.6, "filled": True, "tone": 0.4, "kind": "poly", "points": [[1.75, 0.48], [1.75, 0.52], [1.44, 0.68], [1.42, 0.64]], "closed": True, "region": "running-gear", "material": "paint-steel"},
]
json.dump(doc, open(C, "w"), ensure_ascii=False, indent=1)
print("plan 1:50 redrawn")
