"""🛴️ HW-09 generator of `e-scooter-rider`: an adult standing on an electric kick scooter (250 W hub motor nameplate).

Body: the rider of `cyclist` (helmet, softshell, gloves closed round a bar), re-rooted on the scooter deck through the same
hip-lift / hip-shift prismatic pair and scaled to a 1.80 m, 76 kg man (k = 1.80 / 1.74 of the drawn stature without helmet).
Scooter: `electric-kick-scooter` (vehicles) unchanged in its own frame as the root of the rig; its 1:500 drawing is new here.
Poses and clips are solved afterwards by `poses_e_scooter_rider.ts`. Run: python3 build_e_scooter_rider.py
"""
import copy
import json
from hw09lib import load, write, subtree, take, shift_rest, scale_part, rename, merge_refs, uniq, T_, SCALES, VIEWS, LISTS

SLUG = "e-scooter-rider"
K = 1.035
MASS = 76.0
HIP = (-0.10, 1.10)

cyc = load("👤️humans", "cyclist")
sco = load("🚜️vehicles", "electric-kick-scooter")

rider_bones = subtree(cyc, ["hip-lift"])
rider = take(cyc, rider_bones)
src_hip = next(b for b in cyc["rig"]["bones"] if b["id"] == "hip-lift")
rider_mass = sum(b["mass"]["kg"] for b in rider["bones"] if "mass" in b)
shift_rest(rider, dx=-src_hip["x"], dy=-src_hip["y"])
scale_part(rider, K, MASS / rider_mass)
shift_rest(rider, dx=HIP[0], dy=HIP[1])
for b in rider["bones"]:
    if b["id"] == "hip-lift":
        b.update(parent="deck", x=HIP[0], y=HIP[1], lengthM=0.01)
        b["title"] = T_("Pelvis vertical travel above the deck", "Vertikale Beckenverschiebung über dem Trittbrett")
        b["joint"] = {"type": "prismatic", "axisDeg": 90, "minM": -0.25, "maxM": 0.1}
    if b["id"] == "hip-shift":
        b["lengthM"] = 0.01
        b["title"] = T_("Pelvis travel along the deck", "Beckenverschiebung entlang dem Trittbrett")
        b["joint"] = {"type": "prismatic", "axisDeg": 0, "minM": -0.3, "maxM": 0.3}
    if b["id"] == "pelvis":
        b["title"] = T_("Pelvis", "Becken")

scooter = take(sco, [b["id"] for b in sco["rig"]["bones"]], scales=["50", "100", "200"])
scooter["solids"]["500"] = copy.deepcopy({k: sco["volumes"]["500"][k] for k in LISTS})
TYPIFY_500 = {
    "deck": lambda x: x.update({"from": 0.075}),
    "neck": lambda x: [sec.update(rx=max(sec["rx"], 0.038), ry=max(sec["ry"], 0.038)) for sec in x["sections"]],
    "fork": lambda x: x.update({"from": -0.03, "to": 0.03}),
    "bars": lambda x: x.update({"profile": [[0, 0.02], [0.43, 0.02]]}),
    "tyre-r": lambda x: x.update({"profile": [[-0.038, 0.07], [-0.038, 0.095], [-0.025, 0.108], [0.025, 0.108], [0.038, 0.095], [0.038, 0.07], [-0.038, 0.07]]}),
    "tyre-f": lambda x: x.update({"profile": [[-0.038, 0.07], [-0.038, 0.095], [-0.025, 0.108], [0.025, 0.108], [0.038, 0.095], [0.038, 0.07], [-0.038, 0.07]]}),
    "th-bms": lambda x: x.update({"size": [0.09, 0.075, 0.1], "transform": {"translation": [0.17, 0.1125, 0]}}),
    "th-fender-r": lambda x: x.update({"from": -0.0375, "to": 0.0375}),
    "th-fender-f": lambda x: x.update({"from": -0.0375, "to": 0.0375, "profile": [[round(0.18 + (p[0] - 0.18) * 1.25, 5), p[1]] for p in x["profile"]]}),
    "motor-f": lambda x: x.update({"profile": [[-0.038, 0], [-0.038, 0.06], [-0.0225, 0.077], [0.0225, 0.077], [0.038, 0.06], [0.038, 0]]}),
    "mass-battery": lambda x: x.update({"size": [0.41, 0.075, 0.095], "transform": {"translation": [-0.04, 0.1125, 0]}}),
    "hinge-pin": lambda x: x.update({"profile": [[0, 0.03], [0.06, 0.03]], "origin": [0, -0.024, -0.03]}),
    "mass-stem": lambda x: x.update({"radii": [0.09, 0.02, 0.02]}),
    "mass-motor": lambda x: x.update({"profile": [[0, 0.05], [0.076, 0.05]], "origin": [0, 0, -0.038]}),
}
for lst in LISTS:
    for x in scooter["solids"]["500"][lst]:
        if x["id"] in TYPIFY_500 and "of" not in x:
            TYPIFY_500[x["id"]](x)
rider_ids = {s: {v: {x["id"] for x in rider["shapes"][s].get(v, [])} for v in VIEWS} for s in SCALES}
clash = {}
for s in ["50", "100", "200"]:
    for v in VIEWS:
        for x in scooter["shapes"][s][v]:
            if x["id"] in rider_ids[s][v]:
                clash[x["id"]] = "deck-" + x["id"]
solid_ids = {x["id"] for s in SCALES for l in LISTS for x in rider["solids"][s].get(l, [])}
sclash = {x["id"]: "scooter-" + x["id"] for s in SCALES for l in LISTS for x in scooter["solids"][s].get(l, []) if x["id"] in solid_ids}
rename(scooter, {**clash, **sclash}, {"shape", "solid"})
assert not ({r["id"] for r in rider["regions"]} & {r["id"] for r in scooter["regions"]})
assert not ({m["id"] for m in rider["materials"]} & {m["id"] for m in scooter["materials"]})

S500 = {
    "elevation": [
        {"kind": "poly", "id": "deck", "bone": "deck", "role": "body", "z": 40, "filled": True, "tone": 0.3, "points": [[-0.452, 0.075], [0.245, 0.075], [0.4216, 0.2915], [0.3744, 0.3285], [0.222, 0.15], [-0.452, 0.15]], "closed": True, "regions": ["deck-battery", "deck-grip", "fenders"], "material": "anodised"},
        {"kind": "poly", "id": "fork", "bone": "fork", "role": "body", "z": 41, "filled": True, "tone": 0.4, "points": [[0, -0.03], [0.292, -0.03], [0.292, 0.03], [0, 0.03]], "closed": True, "region": "stem-bars", "material": "anodised", "solid": {"kind": "extruded", "sweep": "w", "z0": -0.047, "z1": 0.047}},
        {"kind": "poly", "id": "stem", "bone": "stem", "role": "body", "z": 42, "filled": True, "tone": 0.4, "points": [[0, 0.003], [0.737, -0.004], [0.737, -0.044], [0, -0.051]], "closed": True, "region": "stem-bars", "material": "anodised", "solid": {"kind": "extruded", "sweep": "w", "z0": -0.027, "z1": 0.027}},
        {"kind": "ellipse", "id": "bars", "bone": "stem", "role": "body", "z": 43, "filled": True, "tone": 0.2, "region": "stem-bars", "material": "rubber", "cx": 0.719, "cy": -0.024, "rx": 0.02, "ry": 0.02, "solid": {"kind": "extruded", "sweep": "w", "z0": -0.215, "z1": 0.215}},
        {"kind": "ellipse", "id": "wheel-f", "bone": "wheel-f", "role": "body", "z": 44, "filled": True, "tone": 0.1, "material": "rubber", "cx": 0, "cy": 0, "rx": 0.108, "ry": 0.108, "regions": ["tyres", "hub-motor"], "solid": {"kind": "extruded", "sweep": "w", "z0": -0.038, "z1": 0.038}},
        {"kind": "ellipse", "id": "wheel-r", "bone": "wheel-r", "role": "body", "z": 45, "filled": True, "tone": 0.1, "material": "rubber", "cx": 0, "cy": 0, "rx": 0.108, "ry": 0.108, "regions": ["tyres", "rear-rim", "brake-disc"], "solid": {"kind": "extruded", "sweep": "w", "z0": -0.04, "z1": 0.04}},
    ],
}
S500["front"] = [
    {"kind": "poly", "id": "deck", "bone": "deck", "role": "body", "z": 40, "filled": True, "tone": 0.3, "points": [[-0.075, 0.075], [0.075, 0.075], [0.075, 0.15], [0.03, 0.17], [0.027, 0.34], [-0.027, 0.34], [-0.03, 0.17], [-0.075, 0.15]], "closed": True, "regions": ["deck-battery", "deck-grip", "fenders"], "material": "anodised"},
] + [copy.deepcopy(x) for x in S500["elevation"][1:]]
S500["plan"] = [
    {"kind": "poly", "id": "deck", "bone": "deck", "role": "body", "z": 40, "filled": True, "tone": 0.3, "points": [[-0.452, -0.075], [0.266, -0.075], [0.3, -0.032], [0.398, -0.027], [0.398, 0.027], [0.3, 0.032], [0.266, 0.075], [-0.452, 0.075]], "closed": True, "regions": ["deck-battery", "deck-grip", "fenders"], "material": "anodised"},
] + [copy.deepcopy(x) for x in S500["elevation"][1:]]
scooter["shapes"]["500"] = S500
scooter["hatches"]["500"] = []

bones = [copy.deepcopy(b) for b in sco["rig"]["bones"]] + rider["bones"]
TITLE_FIX = [(" on the pedal", ""), (" on the saddle", ""), ("Fahrerin", "Person"), (" auf dem Pedal", ""), (" auf dem Sattel", "")]
for b in bones:
    for a_, b_ in TITLE_FIX:
        b["title"] = {k: v.replace(a_, b_) for k, v in b["title"].items()}
    if b["id"] == "deck":
        b["title"] = T_("Scooter deck with battery, the root of the rider", "Trittbrett mit Akku, Wurzel der fahrenden Person")

asset = {
    "id": SLUG,
    "category": "humans",
    "title": T_("E-Scooter Rider", "Person auf E-Scooter"),
    "summary": T_("", ""),
    "tags": ["human", "adult", "e-scooter", "electric", "helmet", "micromobility", "wheels"],
    "provenance": {},
    "dimensions": {},
    "materials": uniq(rider["materials"] + scooter["materials"]),
    "rig": {"kind": "anatomical", "root": "deck", "bones": bones},
    "volumes": {},
    "anchors": scooter["anchors"] + rider["anchors"],
    "representations": {},
    "symbols": [],
    "poses": [],
    "clips": [],
    "thermal": {},
    "structure": {},
    "presentation": {"defaultPose": "cruising", "defaultClip": "ride-cruise", "defaultScale": 100},
    "exports": cyc["exports"],
    "completion": "authored",
}
for s in SCALES:
    geo = {v: rider["shapes"][s][v] + scooter["shapes"][s][v] for v in VIEWS}
    hat = uniq(rider["hatches"].get(s, []) + scooter["hatches"].get(s, []))
    rep = copy.deepcopy(cyc["representations"][s])
    rep["geometry"] = geo
    rep["hatches"] = hat
    asset["representations"][s] = rep
    vol = {"status": "authored", "denominator": int(s)}
    for l in LISTS:
        vol[l] = rider["solids"][s][l] + scooter["solids"][s][l]
    vol["decisions"] = []
    asset["volumes"][s] = vol

json.dump({"rider_regions": rider["regions"], "scooter_regions": scooter["regions"]}, open("/dev/null", "w"))
asset["_parts"] = {"rider_regions": [r["id"] for r in rider["regions"]], "scooter_regions": [r["id"] for r in scooter["regions"]]}
import e_scooter_texts as texts
texts.finish(asset, cyc, sco, rider, scooter, K, MASS)
del asset["_parts"]
write(SLUG, asset)
print("written", SLUG, len(bones), "bones")
