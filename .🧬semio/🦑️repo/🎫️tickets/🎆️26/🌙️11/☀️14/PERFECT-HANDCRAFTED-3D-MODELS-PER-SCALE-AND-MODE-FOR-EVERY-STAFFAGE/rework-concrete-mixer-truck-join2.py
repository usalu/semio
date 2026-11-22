"""🪞 Author pass 1 (second round) on machinery/concrete-mixer-truck, after the probe of rework-concrete-mixer-truck-join.py: mirror arms reach the cab corner at 1:100 and 1:200, the 1:500 platform stay runs from the rear frame to the platform, and the hopper gussets of 1:100 leave the structure list (the hopper is no member there).

Usage: python3 rework-concrete-mixer-truck-join2.py   (source only; publish with volumes-insert.py)
"""
import json

PATH = "volumes/machinery/concrete-mixer-truck.json"
doc = json.load(open(PATH))
V = doc["volumes"]
get = lambda scale, sid: next(s for s in V[str(scale)]["body"] if s["id"] == sid)
for s, k in (("l", 1), ("r", -1)):
    get(100, f"mirror-arm-{s}")["p0"] = [4.32, 2.82, k * 0.99]
    arm = get(200, f"mirror-arm-{s}")
    for key in ("size", "transform"):
        arm.pop(key, None)
    arm.update(kind="capsule", p0=[4.3, 2.8, k * 0.98], p1=[4.44, 2.74, k * 1.13], r0=0.025, r1=0.025)
stay = get(500, "platform-stay")
for key in ("size", "transform"):
    stay.pop(key, None)
stay.update(kind="capsule", p0=[-3.7, 2.5, 0.54], p1=[-3.98, 2.5, 0.82], r0=0.04, r1=0.04)
V["100"]["structure"] = [s for s in V["100"]["structure"] if s["id"] not in ("st-hopper-bracket-l", "st-hopper-bracket-r")]
json.dump(doc, open(PATH, "w"), ensure_ascii=False, indent=1)
print("joined 2")
