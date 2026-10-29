from body import *
h = Human(1.78, 84, 1.05)
mats = [{"id": m, "title": T(m, m), "emissivity": 0.95, "source": "authored-estimate"} for m in ["skin", "cotton-clothing", "leather-shoe", "hair"]]
regs = [{"id": i, "title": T(i, i), "kind": "clothing", "material": "cotton-clothing", "model": "passive", "areaM2": 0.2, "characteristicLengthM": 0.1, "source": "authored-estimate"} for i in ["head-skin","hair","torso-clothing","arm-clothing","arm-skin","hand-skin","leg-clothing","leg-skin","shoes"]]
a = {"id": "test", "category": "humans", "title": T("t", "t"), "summary": T("t", "t"), "tags": [], "provenance": {"author": "x", "method": "handcrafted", "created": "2026-10-29", "references": []},
     "dimensions": {"heightM": 1.78, "widthM": 0.4, "depthM": 0.3, "massKg": 84}, "materials": mats, "rig": {"kind": "anatomical", "root": "pelvis", "bones": h.bones()}, "anchors": h.anchors(),
     "geometry": {"elevation": h.standard(arm_sleeve="short", legs="long", shoe="boot")},
     "poses": [{"id": "stand", "title": T("s", "s"), "description": T("s", "s"), "bones": {}, "ground": "auto", "contacts": ["contact-l", "contact-r"], "thermal": {}, "loads": []}],
     "clips": [], "thermal": {"environment": {"airTempC": 20, "meanRadiantTempC": 20, "relativeHumidity": 0.5, "windMS": 0.2, "windDirectionDeg": 180, "solarWM2": 0, "pressurePa": 101325}, "regions": regs, "emitters": [], "assumptions": []},
     "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": [], "assumptions": [], "clinical": False}, "presentation": {"defaultPose": "stand", "defaultClip": "x", "viewpoints": ["elevation"]}, "exports": [], "completion": "authored"}
json.dump({"assets": [a]}, open("out/test.json", "w"), ensure_ascii=False)
