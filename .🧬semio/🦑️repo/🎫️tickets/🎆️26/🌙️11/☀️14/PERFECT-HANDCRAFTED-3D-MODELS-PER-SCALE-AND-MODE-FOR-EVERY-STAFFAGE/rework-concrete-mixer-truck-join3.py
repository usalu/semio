"""🧲 Author pass 1 (second round) on machinery/concrete-mixer-truck, 1:500 after the cross-scale proof: silencer and air filter keep the footprint of 1:200 and hang on brackets, the stack stands on its outlet and the intake on its elbow, the lamp beam sits where 1:200 has it and the ladder stands on its foot.

Usage: python3 rework-concrete-mixer-truck-join3.py   (source only; publish with volumes-insert.py)
"""
import json

PATH = "volumes/machinery/concrete-mixer-truck.json"
doc = json.load(open(PATH))
vol = doc["volumes"]["500"]
get = lambda sid: next(s for s in vol["body"] if s["id"] == sid)
region = lambda sid: next(t["region"] for t in vol["thermal"] if t.get("of") == sid)
P = "paint-steel"


def put(solid, like):
    assert not any(s["id"] == solid["id"] for s in vol["body"]), solid["id"]
    vol["body"].append(solid)
    vol["thermal"].append({"id": "th-" + solid["id"], "of": solid["id"], "region": like if like == "chassis" else region(like)})


def box(sid, size, at):
    return {"id": sid, "kind": "box", "bone": "chassis", "material": P, "role": "detail", "tone": 0.3, "size": size, "transform": {"translation": at}}


get("silencer").update(size=[0.42, 0.56, 0.6], transform={"translation": [2.09, 0.72, -0.83]})
get("air-filter").update(size=[0.42, 0.62, 0.5], transform={"translation": [2.09, 0.98, 0.83]})
get("tail-lamp-beam").update(size=[0.14, 0.14, 2.36], transform={"translation": [-4.4, 0.8, 0]})
put(box("silencer-bracket", [0.3, 0.08, 0.2], [2.09, 0.9, -0.47]), "chassis")
put(box("air-filter-bracket", [0.3, 0.08, 0.24], [2.09, 0.95, 0.48]), "chassis")
put(box("ladder-foot", [0.3, 0.08, 0.46], [-4.32, 0.8, 1.01]), "ladder-stile-1")
put({"id": "exhaust-outlet", "kind": "capsule", "bone": "chassis", "material": get("exhaust-stack")["material"], "role": "detail", "tone": 0.2, "p0": [2.22, 0.93, -0.95], "p1": [2.42, 1.12, -0.95], "r0": 0.055, "r1": 0.055}, "exhaust-stack")
put({"id": "air-intake-elbow", "kind": "revolve", "bone": "chassis", "material": get("air-intake")["material"], "role": "detail", "tone": 0.3, "profile": [[0, 0.075], [0.26, 0.075]], "axis": [0.77104, 0.59969, 0.21418], "origin": [2.28, 1.1, 0.9]}, "air-intake")
json.dump(doc, open(PATH, "w"), ensure_ascii=False, indent=1)
print("joined 3")
