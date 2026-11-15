import copy, json, sys
path = sys.argv[1]
d = json.load(open(path, encoding="utf8"))
D = d["$defs"]
ref = lambda n: {"$ref": f"#/$defs/{n}"}

def clean(branch):
    p = branch["properties"]
    for k in ("region", "regions", "maxDenominator"):
        p.pop(k, None)
    p["id"]["description"] = "Unique among the solids of its list at this scale (validator `volume-duplicate-id`)."
    return branch

solid = D["VolumeSolid"]
solid["description"] = solid["description"].replace("One declarative solid of a volume", "One declarative solid of one list of a scale volume (body, thermal or structure)").replace("(validators `volume-size`", "A solid of the body list is exactly this; thermal and structural solids add the fields of ThermalSolid and StructuralSolid (validators `volume-size`")
solid["oneOf"] = [clean(b) for b in solid["oneOf"]]
base = copy.deepcopy(solid["oneOf"])

thermal_extra = {
    "region": {"description": "The id of the `thermal.regions` entry whose surface this solid models (validator `volume-region`).", **ref("Slug")},
    "layer": {"description": "The id of a layer of that region this solid is the shell of, innermost layer first; absent for a solid that is the whole region or an emitter (validator `volume-layer`).", **ref("Slug")},
}
structural_extra = {
    "member": {"description": "The id of the bone whose load-bearing member, joint, support or concentrated mass this solid is (validator `volume-member`).", **ref("Slug")},
    "part": {"description": "What the solid is: `member` (the load-bearing cross-section of the bone), `joint` (a rig joint), `support` (a support at an anchor of `structure.supports`) or `mass` (a compact solid at a concentrated centre of mass).", "enum": ["member", "joint", "support", "mass"]},
}

def derived(extra, required, what):
    out = []
    for b in base:
        n = copy.deepcopy(b)
        n["description"] = b["description"] + f" In the {what} list the solid additionally names what it models."
        n["properties"].update(copy.deepcopy(extra))
        n["required"] = b["required"] + required
        out.append(n)
    return out

def reference(extra, required, what, own):
    props = {"id": {"description": "Unique among the solids of its list at this scale (validator `volume-duplicate-id`).", **ref("Slug")},
             "of": {"description": f"The id of a solid of the `body` list of the same scale whose geometry this {what} solid reuses: same shape, bone, material and placement, shared by construction (validator `volume-of`).", **ref("Slug")}}
    props.update(copy.deepcopy(extra))
    return {"description": f"A {what} solid that reuses the geometry of a body solid of the same scale where the {own} is that body surface.", "type": "object", "additionalProperties": False, "required": ["id", "of"] + required, "properties": props}

D["ThermalSolid"] = {"description": "A solid of the `thermal` list: the thermal model made spatial. Either its own VolumeSolid with a `region` (and a `layer`), or a reference `{ id, of, region, layer? }` that reuses a body solid where the thermal surface is that body surface. At 1:50 a region with several layers is modelled as nested shells, one per layer (innermost first, real thickness); hidden regions are real solids inside the body; emitters are solids.", "oneOf": derived(thermal_extra, ["region"], "thermal") + [reference(thermal_extra, ["region"], "thermal", "thermal surface")]}
D["StructuralSolid"] = {"description": "A solid of the `structure` list: the structural idealisation made spatial. Either its own VolumeSolid with a `member` and a `part`, or a reference `{ id, of, member, part }` that reuses a body solid. Every load-bearing bone is a member solid with its real cross-section, joints are joint solids at the rig joints, supports sit at the `structure.supports` anchors, concentrated masses are compact solids at their centre of mass; non-bearing cladding is absent.", "oneOf": derived(structural_extra, ["member", "part"], "structure") + [reference(structural_extra, ["member", "part"], "structure", "load-bearing surface")]}

text = ref("Text")
D["AuthoredVolume"] = {
    "description": "The handcrafted 3D model of an asset at one preset scale, authored individually against that scale's own 2D shapes (never a filtered copy of another scale): the smallest modelled feature is the print profile's smallest printable feature at the denominator, a coarser scale is a new abstraction with merged masses, simplified sections and a generalised outline. `body` carries the tone views, `thermal` the thermal model made spatial and `structure` the structural idealisation made spatial. The silhouette of the body in `presentation.defaultPose` agrees with the filled shapes of the representation of the same scale and the bounding box with `dimensions` (validators `volumes-silhouette`, `volumes-bounds`).",
    "type": "object", "additionalProperties": False,
    "required": ["status", "denominator", "body", "thermal", "structure", "decisions"],
    "properties": {
        "status": {"const": "authored"},
        "denominator": {"description": "The preset scale of this volume; equals its key in `volumes` (validator `volumes-availability`).", **ref("ScaleDenominator")},
        "body": {"description": "The solids of the body: what the tone views (dark-line, dark-solid, light-line, light-solid) show.", "type": "array", "items": ref("VolumeSolid"), "minItems": 1},
        "thermal": {"description": "The thermal solids: every thermal region has at least one solid at every authored scale (validator `volumes-thermal-coverage`).", "type": "array", "items": ref("ThermalSolid"), "minItems": 1},
        "structure": {"description": "The structural solids: every load-bearing bone has a member solid (validator `volumes-structure-coverage`).", "type": "array", "items": ref("StructuralSolid"), "minItems": 1},
        "decisions": {"description": "The authoring decisions of this scale: what each group of solids stands for, what was merged or simplified against the finer scale and why.", "type": "array", "items": text, "minItems": 1},
    },
}
D["UnavailableVolume"] = {
    "description": "The volume of a scale whose representation is unavailable: present exactly when `representations[denominator]` is unavailable; the reason lives in that representation (validator `volumes-availability`).",
    "type": "object", "additionalProperties": False, "required": ["status", "denominator"],
    "properties": {"status": {"const": "unavailable"}, "denominator": ref("ScaleDenominator")},
}
D["ScaleVolume"] = {"description": "The volume decision for one preset scale: authored or unavailable, mirroring the representation of the same scale.", "oneOf": [ref("AuthoredVolume"), ref("UnavailableVolume")]}
old = D.pop("Volume")
D["Volumes"] = {"description": "The 3D volumes of an asset, one per preset scale and keyed like `representations`: every scale is authored individually, never derived from another.", "type": "object", "additionalProperties": False, "required": ["50", "100", "200", "500"], "properties": {k: ref("ScaleVolume") for k in ("50", "100", "200", "500")}}

# keep definition order: place the new defs where Volume was
order = [k for k in D if k not in ("ThermalSolid", "StructuralSolid", "AuthoredVolume", "UnavailableVolume", "ScaleVolume", "Volumes")]
i = order.index("VolumeSolid") + 1
order[i:i] = ["ThermalSolid", "StructuralSolid", "AuthoredVolume", "UnavailableVolume", "ScaleVolume", "Volumes"]
d["$defs"] = {k: D[k] for k in order}
D = d["$defs"]

A = D["Asset"]
props = A["properties"]
newprops = {}
for k, v in props.items():
    if k == "volume":
        newprops["volumes"] = {"description": "The 3D volumes bound to the rig, one individually handcrafted model per preset scale and keyed like `representations`; an authored volume exists exactly where the representation is authored.", **ref("Volumes")}
    else:
        newprops[k] = v
A["properties"] = newprops
A["required"] = ["volumes" if k == "volume" else k for k in A["required"]]
D["Viewpoint"]["description"] = D["Viewpoint"]["description"].replace("derived from `volume`", "derived from `volumes`").replace("the 3D volume", "the 3D volumes")
for key in ("VolumeTransform", "VolumeRepeat"):
    D[key]["description"] = D[key]["description"].replace("a volume solid", "a volume solid")
json.dump(d, open(path, "w", encoding="utf8"), ensure_ascii=False, indent=2)
open(path, "a").write("\n")
