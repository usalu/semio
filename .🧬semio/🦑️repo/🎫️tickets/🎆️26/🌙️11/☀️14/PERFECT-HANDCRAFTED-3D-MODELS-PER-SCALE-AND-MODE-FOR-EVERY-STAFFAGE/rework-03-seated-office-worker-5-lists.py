"""🌡️ Stage 5 of the seated office worker: the thermal and structure lists of every scale follow the restated bodies: inner shells restated from their body solid with the real layer thickness, new parts referenced with their region and layer or member, the neck skin and the 1:500 backrest carrier restated.

Run after stages 2 and 4: `python3 rework-03-seated-office-worker-5-lists.py` (repeatable).
"""
import copy

from rework_03_lib import *

A = "humans/seated-office-worker"
src = read(source_path(A))
V = src["volumes"]
THICK = {"vest": 0.0008, "briefs": 0.0014, "foam": 0.001, "socks": 0.004}
REGION = {
    "shirt-waist": ("shirt", "shirt"), "elbow-l": ("shirt", "shirt"), "elbow-r": ("shirt", "shirt"),
    "chair-backrest-slider": ("chair-plastic", "pa-shell"), "chair-knob": ("chair-plastic", "pa-shell"), "chair-lever": ("chair-steel", "tube"),
    "chair-armrest-bracket-l": ("chair-steel", "tube"), "chair-armrest-bracket-r": ("chair-steel", "tube"),
    "chair-armrest-sleeve-l": ("chair-plastic", "pa-shell"), "chair-armrest-sleeve-r": ("chair-plastic", "pa-shell"),
    "chair-backrest-strut": ("chair-steel", "tube"), "chair-backrest-pivot": ("chair-steel", "tube"), "chair-mechanism-arm": ("chair-steel", "tube"), "chair-mechanism": ("chair-steel", "tube"),
    **{f"chair-stem-{k}": ("chair-steel", "tube") for k in ("front", "side-a", "back-a", "back-b", "side-b")}, "chair-backrest-shell": ("chair-plastic", "pa-shell"),
    "desk-plate-fl": ("desk", None), "desk-plate-fr": ("desk", None), "desk-plate-bl": ("desk", None), "desk-plate-br": ("desk", None),
}
MEMBER = {"chair-mechanism": "chair-seat", "chair-mechanism-arm": "chair-seat", "chair-backrest-pivot": "chair-backrest", "chair-armrest-sleeve-l": "chair-seat", "chair-armrest-sleeve-r": "chair-seat", "laptop-hinge": "laptop",
          **{f"desk-glide-{k}": "desk-leg" for k in ("fl", "fr", "bl", "br")},
          **{f"chair-{part}-{k}{n}": (f"chair-arm-{k}" if part == "socket" else f"chair-castor-{k}") for k in ("front", "side-a", "back-a", "back-b", "side-b") for part in ("socket", "hood", "stem", "wheel", "tyre") for n in ("", "-a", "-b")},
          "chair-armrest-bracket-l": "chair-seat", "chair-armrest-bracket-r": "chair-seat", "chair-backrest-slider": "chair-backrest", "chair-backrest-strut": "chair-backrest"}


def shrink(solid, t, id, region, layer):
    s = copy.deepcopy(solid)
    s["id"] = id
    for sec in s["sections"]:
        sec["rx"], sec["ry"] = round(sec["rx"] - t, 5), round(sec["ry"] - t, 5)
    s.update({"region": region, "layer": layer})
    return s


for scale, vol in V.items():
    body = by_id(vol["body"])
    thermal = vol["thermal"]
    layered = any("layer" in s for s in thermal)
    for i, s in enumerate(thermal):
        if "of" in s or s.get("layer") not in THICK:
            continue
        target = s["id"][3 : -len(s["layer"]) - 1]
        if target in body:
            thermal[i] = shrink(body[target], THICK[s["layer"]], s["id"], s["region"], s["layer"])
    T = by_id(thermal)
    if "th-neck" in T and "of" not in T["th-neck"]:
        T["th-neck"]["sections"] = [{"at": [0.02, -0.01, 0], "rx": 0.058, "ry": 0.06, "rotation": ACROSS}, {"at": [0.07, -0.012, 0], "rx": 0.055, "ry": 0.058, "rotation": ACROSS}, {"at": [0.125, -0.008, 0], "rx": 0.056, "ry": 0.06, "rotation": ACROSS}]
    covered = {s.get("of") for s in thermal}
    for id, (region, layer) in REGION.items():
        if id not in body or id in covered:
            continue
        ref = {"id": f"th-{id}" + (f"-{layer}" if layer and layered else ""), "of": id, "region": region}
        if layer and layered:
            ref["layer"] = layer
        after = max((i for i, s in enumerate(thermal) if s.get("region") == region), default=len(thermal) - 1)
        thermal.insert(after + 1, ref)
        if id == "shirt-waist" and layered:
            thermal.insert(after + 1, shrink(body[id], THICK["vest"], "th-shirt-waist-vest", "shirt", "vest"))
    for s in thermal:
        if s.get("of", "").startswith("chair-armrest-post") and "layer" in s:
            steel = body[s["of"]]["material"] == "steel-painted"
            s.update({"region": "chair-steel" if steel else "chair-plastic", "layer": "tube" if steel else "pa-shell"})
    vol["thermal"] = [s for s in thermal if "of" not in s or s["of"] in body]
    structure = [s for s in vol["structure"] if s["id"] != "backrest-frame"]
    done = {s.get("of") for s in structure}
    for id, member in MEMBER.items():
        if id in body and id not in done:
            after = max((i for i, s in enumerate(structure) if s.get("member") == member and s.get("part") == "member"), default=len(structure) - 1)
            structure.insert(after + 1, {"id": f"st-{id}", "of": id, "member": member, "part": "member"})
    vol["structure"] = [s for s in structure if "of" not in s or s["of"] in body]
    print(scale, len(vol["thermal"]), len(vol["structure"]))
write(source_path(A), src)
