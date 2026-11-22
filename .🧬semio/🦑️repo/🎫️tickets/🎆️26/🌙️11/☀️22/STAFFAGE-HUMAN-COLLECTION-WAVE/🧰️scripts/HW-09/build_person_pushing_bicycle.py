"""🚲️ HW-09 generator of `person-pushing-bicycle`: an adult walking on the left side of a city bicycle, both hands on the handlebar
(left hand on the left grip, right hand on the left half of the bar beside the stem).

Body: the parent of `parent-with-stroller` (padded jacket, jeans, sneakers, hands that close round a bar; its planted-foot gait keys)
without the stroller and the toddler, scaled by 1.03 to a 1.80 m, 75 kg adult and hung from the bicycle frame through a walk-lift /
walk-shift prismatic pair 0.34 m to the left of the bicycle's centre plane. Bicycle: `city-bicycle` (vehicles) in its own frame as the
root of the rig, with a new 1:500 drawing. Poses and clips: `poses_person_pushing_bicycle.ts`. Run: python3 build_person_pushing_bicycle.py
"""
import copy
from hw09lib import load, write, subtree, take, shift_rest, scale_part, rename, merge_refs, uniq, T_, SCALES, VIEWS, LISTS

SLUG = "person-pushing-bicycle"
K = 1.03
MASS = 75.0
CENTRE = 0.135
HIP = (-0.25, 0.95, 0.34 - CENTRE)

par = load("👤️humans", "parent-with-stroller")
bic = load("🚜️vehicles", "city-bicycle")

person_bones = [b for b in subtree(par, ["pelvis"]) if b not in set(subtree(par, ["stroller"]))]
person = take(par, person_bones)
person_mass = sum(b["mass"]["kg"] for b in person["bones"] if "mass" in b)
scale_part(person, K, MASS / person_mass)
shift_rest(person, dx=HIP[0], dy=HIP[1], dz=HIP[2])
person["bones"] = [
    {"id": "walk-lift", "parent": "frame", "title": T_("Pelvis height beside the bicycle", "Beckenhöhe neben dem Fahrrad"), "x": HIP[0], "y": HIP[1], "z": round(HIP[2] + CENTRE, 5), "angleDeg": 0, "lengthM": 0.01, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.2, "maxM": 0.1}},
    {"id": "walk-shift", "parent": "walk-lift", "title": T_("Pelvis travel along the bicycle", "Beckenverschiebung entlang dem Fahrrad"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.01, "joint": {"type": "prismatic", "axisDeg": 0, "minM": -0.3, "maxM": 0.3}},
] + person["bones"]
for b in person["bones"]:
    if b["id"] == "pelvis":
        b.update(parent="walk-shift", x=0, y=0)
        b["joint"] = {"type": "revolute", "minDeg": -30, "maxDeg": 30}

bike = take(bic, [b["id"] for b in bic["rig"]["bones"]], scales=["50", "100", "200"])
bike["solids"]["500"] = copy.deepcopy({k: bic["volumes"]["500"][k] for k in LISTS})
shape500 = {v: [copy.deepcopy(x) for x in bic["representations"]["200"]["geometry"][v] if x["role"] == "body"] for v in VIEWS}
bike["shapes"]["500"] = shape500
import bicycle_fixes
bicycle_fixes.fix_bike(bike, bic, CENTRE)
bike["hatches"]["500"] = []
pid = {s: {v: {x["id"] for x in person["shapes"][s].get(v, [])} for v in VIEWS} for s in SCALES}
clash = {x["id"]: "bike-" + x["id"] for s in SCALES for v in VIEWS for x in bike["shapes"][s][v] if x["id"] in pid[s][v]}
psol = {x["id"] for s in SCALES for l in LISTS for x in person["solids"][s].get(l, [])}
sclash = {x["id"]: "bike-" + x["id"] for s in SCALES for l in LISTS for x in bike["solids"][s].get(l, []) if x["id"] in psol}
mclash = {m["id"]: "bike-" + m["id"] for m in bike["materials"] if m["id"] in {n["id"] for n in person["materials"]}}
rclash = {r["id"]: "bike-" + r["id"] for r in bike["regions"] if r["id"] in {n["id"] for n in person["regions"]}}
aclash = {"contact-r": "patch-rear", "contact-f": "patch-front"}
rename(bike, {**clash, **sclash, **mclash, **rclash, **aclash}, {"shape", "solid", "material", "region", "anchor"})
print("renamed", len(clash), len(sclash), mclash, rclash, aclash)

bones = [copy.deepcopy(b) for b in bic["rig"]["bones"]] + person["bones"]
bones[0]["z"] = -CENTRE
TITLE_FIX = [("Right hand on the push bar", "Right hand"), ("Left hand on the push bar", "Left hand")]
for b in bones:
    if b["id"] == "frame":
        b["title"] = T_("Bicycle frame, the root of the pushing person", "Fahrradrahmen, Wurzel der schiebenden Person")

asset = {
    "id": SLUG, "category": "humans",
    "title": T_("Person Pushing a Bicycle", "Person schiebt Fahrrad"),
    "summary": T_("", ""),
    "tags": ["human", "adult", "bicycle", "pushing", "walking", "gait", "wheels"],
    "provenance": {}, "dimensions": {},
    "materials": uniq(person["materials"] + bike["materials"]),
    "rig": {"kind": "anatomical", "root": "frame", "bones": bones},
    "volumes": {}, "anchors": bike["anchors"] + person["anchors"],
    "representations": {}, "symbols": [], "poses": [], "clips": [], "thermal": {}, "structure": {},
    "presentation": {"defaultPose": "pushing-bicycle", "defaultClip": "push-walk", "defaultScale": 100},
    "exports": par["exports"], "completion": "authored",
}
for s in SCALES:
    rep = copy.deepcopy(par["representations"][s])
    rep["geometry"] = {v: person["shapes"][s][v] + bike["shapes"][s][v] for v in VIEWS}
    rep["hatches"] = uniq(person["hatches"].get(s, []) + bike["hatches"].get(s, []))
    asset["representations"][s] = rep
    asset["volumes"][s] = {"status": "authored", "denominator": int(s), **{l: person["solids"][s][l] + bike["solids"][s][l] for l in LISTS}, "decisions": []}

for side in ("l", "r"):
    toe = next(b for b in asset["rig"]["bones"] if b["id"] == f"toe-{side}")
    asset["volumes"]["500"]["structure"].append({"id": f"toe-bones-{side}", "kind": "capsule", "bone": f"toe-{side}", "material": "bone", "role": "body", "tone": 0.93, "p0": [0, -0.022, 0], "p1": [round(toe["lengthM"] * 0.6, 4), -0.022, 0], "r0": 0.038, "r1": 0.038, "member": f"toe-{side}", "part": "member"})
import bicycle_texts
bicycle_texts.finish(asset, par, bic, person, bike, K, MASS)
write(SLUG, asset)
print("written", SLUG, len(bones), "bones")
