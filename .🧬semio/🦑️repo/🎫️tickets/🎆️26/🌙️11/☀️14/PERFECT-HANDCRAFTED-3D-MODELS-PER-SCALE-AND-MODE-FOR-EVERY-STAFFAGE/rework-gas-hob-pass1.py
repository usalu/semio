"""🔥 Pass 1 chisel for appliances/gas-hob: a 50 mm fascia that really holds a 40 mm knob, gas rail, igniter, all scales and drawings.

Hand-chosen numbers only. Reads the pass-0 base kept under 🗑️generated/rework-gas-hob/base and writes the volumes
source and a candidate asset document (🗑️generated/r3-gas-hob/🔣️.json); `--publish` writes the catalog root too.
"""
import copy
import json
import os
import shutil
import sys

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/gas-hob/🔣️.json"
SOURCE = os.path.join(T, "volumes/appliances/gas-hob.json")
BASE = os.path.join(T, "🗑️generated/rework-gas-hob/base")
CANDIDATE = os.path.join(T, "🗑️generated/r3-gas-hob/🔣️.json")

RISE = 0.026
PLATE_TOP = 0.066
PLATE_UNDER = 0.056
HOUSING_TOP = 0.057
KNOB_Y = 0.031
KNOB_X = 0.105
FASCIA_Z = -0.234
KNOB_TIP_Z = -0.259
SLOT_Y0, SLOT_Y1 = 0.029, 0.043
BEAD_Y = 0.0495
INLET = [KNOB_X, 0.03, 0.25]
TAP = [KNOB_X, KNOB_Y, -0.176]
INJECTOR = [0.0, 0.046, 0.03]

if not os.path.exists(os.path.join(BASE, "catalog.json")):
    if json.load(open(CATALOG))["asset"]["dimensions"]["heightM"] != 0.088:
        sys.exit("pass 1 is already in the catalog: this chisel only cuts the 0.088 m pass-0 state")
    os.makedirs(BASE, exist_ok=True)
    shutil.copyfile(CATALOG, os.path.join(BASE, "catalog.json"))
    shutil.copyfile(SOURCE, os.path.join(BASE, "source.json"))
document = json.load(open(os.path.join(BASE, "catalog.json")))
volumes = json.load(open(os.path.join(BASE, "source.json")))["volumes"]
asset = document["asset"]


def solid(scale, group, ident):
    return next(s for s in volumes[scale][group] if s["id"] == ident)


def after(scale, group, ident, new):
    items = volumes[scale][group]
    items.insert(next(i for i, s in enumerate(items) if s["id"] == ident) + 1, new)


def shift_path(d, dy):
    out, tokens, i = [], d.split(), 0
    while i < len(tokens):
        if tokens[i] in ("M", "L"):
            out += [tokens[i], tokens[i + 1], repr(round(float(tokens[i + 2]) + dy, 4))]
            i += 3
        elif tokens[i] == "Z":
            out.append("Z")
            i += 1
        else:
            raise ValueError(tokens[i])
    return " ".join(out)


def shift_shape(shape, dy):
    if shape["kind"] == "rect":
        shape["y"] = round(shape["y"] + dy, 4)
    elif shape["kind"] == "ellipse":
        shape["cy"] = round(shape["cy"] + dy, 4)
    elif shape["kind"] == "poly":
        shape["points"] = [[p[0], round(p[1] + dy, 4)] for p in shape["points"]]
    elif shape["kind"] == "path":
        shape["d"] = shift_path(shape["d"], dy)
    else:
        raise ValueError(shape["kind"])


def shape(scale, view, ident):
    return next(s for s in asset["representations"][scale]["geometry"][view] if s["id"] == ident)


def slots(positions):
    return " ".join(f"M {p} {SLOT_Y0} L {p} {SLOT_Y1}" for p in positions)


# ── 1:50 body ────────────────────────────────────────────────────────────────
solid("50", "body", "housing").update({"from": 0.0075, "to": HOUSING_TOP})
solid("50", "body", "plate").update({"from": PLATE_UNDER, "to": PLATE_TOP})
well = solid("50", "body", "burner-well")
well["profile"] = [[round(p[0] + RISE, 4), p[1]] for p in well["profile"]]
for side in ("front", "back", "right", "left"):
    bead = solid("50", "body", f"bead-{side}")
    bead["p0"][1] = bead["p1"][1] = BEAD_Y
solid("50", "body", "vent-slots").update({"size": [0.003, 0.014, 0.0008], "transform": {"translation": [-0.11, 0.036, -0.2343]}})
solid("50", "body", "vent-slots-right").update({"size": [0.0008, 0.014, 0.003], "transform": {"translation": [0.1403, 0.036, -0.2]}})
solid("50", "body", "vent-slots-left").update({"size": [0.0008, 0.014, 0.003], "transform": {"translation": [-0.1403, 0.036, -0.2]}})
after("50", "body", "vent-slots-left", {
    "id": "gas-inlet", "kind": "revolve", "bone": "hob", "material": "brass", "role": "body", "tone": 0.5,
    "profile": [[0.0, 0.0], [0.0, 0.0135], [0.004, 0.0135], [0.004, 0.0105], [0.0098, 0.0105], [0.0098, 0.0]],
    "axis": [0, 0, 1], "origin": INLET})
solid("50", "body", "knob")["profile"] = [
    [0.0, 0.0], [0.0, 0.021], [0.002, 0.021], [0.003, 0.0125], [0.006, 0.0125], [0.006, 0.0185],
    [0.0165, 0.0185], [0.018, 0.017], [0.018, 0.0]]
solid("50", "body", "knob-ridge").update({"size": [0.009, 0.036, 0.009], "transform": {"translation": [0.0, 0.0, -0.2545]}})
solid("50", "body", "knob-pointer").update({"size": [0.0012, 0.008, 0.0004], "transform": {"translation": [0.0, 0.012, -0.2591]}})
after("50", "body", "burner-ports", {
    "id": "igniter", "kind": "revolve", "bone": "burner", "material": "steel-enamel", "role": "detail", "tone": 0.9,
    "profile": [[0.0, 0.0], [0.0, 0.0035], [0.011, 0.0035], [0.012, 0.0015], [0.0155, 0.0015], [0.0155, 0.0]],
    "axis": [0, 1, 0], "origin": [0.0368, -0.004, -0.0068], "texture": True})
after("50", "body", "igniter", {
    "id": "thermocouple", "kind": "revolve", "bone": "burner", "material": "brass", "role": "detail", "tone": 0.5,
    "profile": [[0.0, 0.0], [0.0, 0.003], [0.013, 0.003], [0.0145, 0.0015], [0.0145, 0.0]],
    "axis": [0, 1, 0], "origin": [-0.0368, -0.004, -0.0068], "texture": True})

# ── 1:50 structure ───────────────────────────────────────────────────────────
solid("50", "structure", "member-housing").update({"from": 0.0075, "to": HOUSING_TOP})
pipe = solid("50", "structure", "member-gas-pipe")
pipe.update({"p0": TAP, "p1": INJECTOR})
rail = copy.deepcopy(pipe)
rail.update({"id": "member-gas-rail", "p0": [INLET[0], INLET[1], 0.248], "p1": TAP, "r0": 0.006, "r1": 0.006})
after("50", "structure", "member-gas-pipe", rail)
after("50", "structure", "member-gas-rail", {"id": "member-gas-inlet", "of": "gas-inlet", "member": "hob", "part": "member"})
solid("50", "structure", "mass-hob")["transform"] = {"translation": [0.0, 0.033, 0.007]}

# ── 1:100, feature 15 mm ─────────────────────────────────────────────────────
solid("100", "body", "plate").update({"from": 0.0, "to": PLATE_TOP})
solid("100", "body", "knob")["profile"] = [[0.0, 0.0], [0.0, 0.02], [0.022, 0.02], [0.025, 0.017], [0.025, 0.0]]
solid("100", "thermal", "th-plate").update({"from": 0.051, "to": PLATE_TOP})
solid("100", "structure", "member-housing").update({"from": 0.0, "to": 0.051})
solid("100", "structure", "member-plate").update({"from": 0.051, "to": PLATE_TOP})
solid("100", "structure", "mass-hob")["transform"] = {"translation": [0.0, 0.028, 0.007]}

# ── 1:200, feature 30 mm ─────────────────────────────────────────────────────
solid("200", "body", "plate").update({"from": 0.0, "to": PLATE_TOP})
solid("200", "body", "knob")["profile"] = [[-0.004, 0.0], [-0.004, 0.02], [0.026, 0.02], [0.026, 0.0]]
solid("200", "thermal", "th-plate").update({"from": 0.036, "to": PLATE_TOP})
solid("200", "structure", "member-housing").update({"from": 0.0, "to": PLATE_TOP})
solid("200", "structure", "member-plate").update({"from": 0.036, "to": PLATE_TOP})
solid("200", "structure", "mass-hob")["transform"] = {"translation": [0.0, 0.033, 0.007]}

# ── 1:500, feature 75 mm ─────────────────────────────────────────────────────
solid("500", "body", "hob").update({"from": 0.0, "to": PLATE_TOP})
solid("500", "body", "knob")["profile"] = [[-0.004, 0.0], [-0.004, 0.02], [0.026, 0.02], [0.026, 0.0]]
solid("500", "thermal", "th-plate").update({"from": 0.046, "to": PLATE_TOP})
solid("500", "structure", "member-housing").update({"from": 0.0, "to": PLATE_TOP})
solid("500", "structure", "member-plate").update({"from": 0.046, "to": PLATE_TOP})
solid("500", "structure", "mass-hob")["transform"] = {"translation": [0.0, 0.033, 0.007]}

# ── rig, dimensions ──────────────────────────────────────────────────────────
bones = {b["id"]: b for b in asset["rig"]["bones"]}
bones["burner"]["y"] = PLATE_TOP
bones["trivet"]["y"] = PLATE_TOP
bones["knob"]["y"] = KNOB_Y
bones["knob"]["lengthM"] = 0.018
bones["hob"]["mass"]["comY"] = 0.033
asset["dimensions"]["heightM"] = 0.114
regions = {r["id"]: r for r in asset["thermal"]["regions"]}
regions["knob"].update({"areaM2": 0.0046, "characteristicLengthM": 0.04})

# ── 2D: front view carries absolute rest heights, so everything above the housing rises with the plate ──
for scale in ("50", "100", "200"):
    for s in asset["representations"][scale]["geometry"]["front"]:
        if s["bone"] not in ("hob", "knob"):
            shift_shape(s, RISE)

KNOB_SOLID = {"kind": "extruded", "sweep": "w", "z0": KNOB_TIP_Z, "z1": FASCIA_Z}
for view, across in (("elevation", [-0.11, -0.09, -0.07, -0.05, -0.03, -0.01, 0.01, 0.03, 0.05]),
                     ("front", [-0.2, -0.16, -0.12, -0.08, -0.04, 0.0, 0.04, 0.08, 0.12, 0.16, 0.2])):
    shape("50", view, "plate").update({"y": PLATE_UNDER, "h": 0.01})
    shape("50", view, "housing").update({"y": 0.0075, "h": 0.0485})
    shape("50", view, "vent-slots")["d"] = slots(across)
    shape("50", view, "knob").update({"rx": 0.021, "ry": 0.021, "solid": dict(KNOB_SOLID)})
shape("50", "elevation", "front-panel-seam")["d"] = f"M -0.136 {BEAD_Y} L 0.136 {BEAD_Y}"
shape("50", "front", "front-panel-seam")["d"] = f"M -0.23 {BEAD_Y} L 0.246 {BEAD_Y}"
shape("50", "elevation", "knob-pointer")["d"] = (
    "M -0.0045 -0.018 L 0.0045 -0.018 L 0.0045 0.018 L -0.0045 0.018 L -0.0045 -0.018 "
    "M 0.0 0.008 L 0.0 0.016 M -0.0185 0.0 A 0.0185 0.0185 0 1 0 0.0185 0.0 A 0.0185 0.0185 0 1 0 -0.0185 0.0")
shape("50", "plan", "plan-knob").update({"x": 0.0865, "y": KNOB_TIP_Z, "w": 0.037, "h": 0.025})
shape("50", "plan", "plan-knob-pointer")["points"] = [[0.0865, -0.25], [0.1235, -0.25]]

for view in ("elevation", "front"):
    shape("100", view, "plate").update({"y": 0.0, "h": PLATE_TOP})
    shape("100", view, "knob").update({"rx": 0.02, "ry": 0.02, "solid": dict(KNOB_SOLID)})
shape("100", "plan", "plan-knob").update({"x": 0.085, "y": KNOB_TIP_Z, "w": 0.04, "h": 0.025})

shape("200", "elevation", "plate").update({"y": 0.0, "h": PLATE_TOP})
shape("200", "front", "plate")["points"] = [
    [-0.236, 0.0], [0.26, 0.0], [0.26, PLATE_TOP], [-0.236, PLATE_TOP],
    [-0.236, 0.051], [-0.26, 0.051], [-0.26, 0.011], [-0.236, 0.011]]
plan200 = shape("200", "plan", "plan-plate")["points"]
plan200[3:7] = [[0.085, -0.236], [0.085, -0.26], [0.125, -0.26], [0.125, -0.236]]

# ── thermal region "housing": the 50 mm tray is the largest visible surface and may not stay a ghost ──
HOUSING_UNDER = {"100": 0.051, "200": 0.036, "500": 0.046}
housing_region = copy.deepcopy(regions["knob"])
housing_region.update({
    "id": "housing", "title": {"en": "Pressed housing", "de": "Gepresstes Gehäuse"}, "material": "steel-enamel",
    "areaM2": 0.211, "characteristicLengthM": 0.05, "geometry": "vertical-plate",
    "layers": [dict(copy.deepcopy(regions["plate"]["layers"][0]), id="housing-skin", title={"en": "Pressed housing", "de": "Gepresstes Gehäuse"})],
    "heatCapacityJM2K": regions["plate"]["heatCapacityJM2K"]})
asset["thermal"]["regions"].insert(1, housing_region)
for pose in asset["poses"]:
    pose["thermal"]["regions"]["housing"] = {"sunlitFraction": 0, "exposedFraction": 1}
after("50", "thermal", "th-plate-well", {"id": "th-housing", "of": "housing", "region": "housing", "sectionFaces": ["+v"]})
for scale, under in HOUSING_UNDER.items():
    tray = copy.deepcopy(solid(scale, "thermal", "th-plate"))
    tray.update({"id": "th-housing", "from": 0.0, "to": under, "region": "housing", "sectionFaces": ["+v"], "tone": 0.4})
    after(scale, "thermal", "th-plate", tray)
for view in ("elevation", "front"):
    shape("50", view, "housing")["region"] = "housing"
    block100 = shape("100", view, "plate")
    del block100["region"]
    block100["regions"] = ["plate", "housing"]
    shape("200", view, "plate")["regions"] = ["plate", "housing", "knob"]
solid("100", "structure", "member-housing").update({"from": 0.008, "to": 0.051})

# ── flicker: a rigid 8 degree lean drove the tongues into the pan-support fingers and the well; a draught leans them 5 ──
flicker = next(c for c in asset["clips"] if c["id"] == "flicker")
for key, lean, stretch in zip(flicker["keys"][1:4], (-5, 4, -3), (0.004, -0.003, 0.003)):
    key["bones"] = {"flame-body": {"angleDeg": lean}, "flame-tip": {"dy": stretch}}

# ── 1:200 and 1:500 thermal: the grate bars floated; they stand on the body posts of the scale ──
for scale in ("200", "500"):
    for side in ("front", "back"):
        volumes[scale]["thermal"].append({"id": f"th-trivet-posts-{side}", "of": f"trivet-posts-{side}", "region": "trivet"})

# ── 1:500 thermal: the real bars lie on the 28.5 mm posts, inside the 20 mm body frame ──
solid("500", "thermal", "th-trivet-bars-x")["transform"] = {"translation": [0.0, 0.0345, -0.1]}
solid("500", "thermal", "th-trivet-bars-z")["transform"] = {"translation": [-0.13, 0.0345, 0.03]}

if os.path.exists(os.path.join(T, "rework-gas-hob-pass1-texts.json")):
    texts = json.load(open(os.path.join(T, "rework-gas-hob-pass1-texts.json")))
    blob = json.dumps(document, ensure_ascii=False)
    vblob = json.dumps(volumes, ensure_ascii=False)
    for old, new in texts["replace"]:
        if old not in blob and old not in vblob:
            print("[DEBUG] text not found:", old[:70])
        blob, vblob = blob.replace(old, new), vblob.replace(old, new)
    document, volumes = json.loads(blob), json.loads(vblob)
    asset = document["asset"]

os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
json.dump({"volumes": volumes}, open(SOURCE, "w"), ensure_ascii=False, indent=1)
candidate = copy.deepcopy(document)
candidate["asset"]["volumes"] = volumes
json.dump(candidate, open(CANDIDATE, "w"), ensure_ascii=False, indent=1)
print("candidate", CANDIDATE)

if "--publish" in sys.argv:
    live = json.load(open(CATALOG))
    for key in ("rig", "dimensions", "representations", "thermal", "summary", "provenance", "structure", "anchors", "poses", "clips"):
        live["asset"][key] = asset[key]
    live["asset"]["volumes"] = volumes
    scope = {}
    exec(open(os.path.join(T, "volumes-insert.py")).read().split("def member_end")[0], scope)
    with open(CATALOG, "w") as out:
        out.write(scope["dump"](live, 0) + "\n")
    print("published", CATALOG)
