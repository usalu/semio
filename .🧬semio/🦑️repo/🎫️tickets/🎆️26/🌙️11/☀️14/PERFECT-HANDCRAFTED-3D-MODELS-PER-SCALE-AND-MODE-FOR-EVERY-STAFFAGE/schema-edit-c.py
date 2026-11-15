import json, sys
path = sys.argv[1]
d = json.load(open(path, encoding="utf8"))
D = d["$defs"]
ref = lambda n: {"$ref": f"#/$defs/{n}"}
SECTION = {"description": "Faces of this solid that are section cuts through a continuing construction (a wall, slab or beam sample) or the hidden back of a one-sided patch: every face whose outward normal lies within 45 degrees of one of these bone axes (`-u`, `+u`, `-v`, `+v`, `-w`, `+w` of the solid's bone frame) counts for no thermal area and is drawn as a cut face.", "type": "array", "items": {"enum": ["-u", "+u", "-v", "+v", "-w", "+w"]}, "uniqueItems": True, "minItems": 1}
SLENDER = {"description": "Declares a slender part (cord, wire, capillary, rod, hose, antenna): its length is at least 20 times its largest cross extent (validator `volumes-slender`). A slender solid is exempt from `volumes-feature` and from the cross-scale comparison; at each scale it is as thick as that scale's 2D draws it, and a coarser scale may keep it in `thermal` or `structure` without having it in `body`.", "type": "boolean"}
ANCHOR = {"description": "For `part: \"support\"`: the id of the support anchor of `structure.supports` this solid is the support of; every support anchor has exactly one support solid per authored scale and the solid is checked against that anchor, not against another anchor of its bone (validators `volume-anchor`, `volumes-structure-coverage`). Absent for the other parts.", **ref("Slug")}
def patch_branch(b, slender=True, anchor=False):
    p = b["properties"]
    p["sectionFaces"] = SECTION
    if slender:
        p["slender"] = SLENDER
    if anchor:
        p["anchor"] = ANCHOR
for name in ("VolumeSolid", "ThermalSolid", "StructuralSolid"):
    for b in D[name]["oneOf"]:
        is_ref = "of" in b["properties"] and "kind" not in b["properties"]
        patch_branch(b, slender=not is_ref, anchor=(name == "StructuralSolid"))
        if name == "StructuralSolid":
            b["if"] = {"properties": {"part": {"const": "support"}}, "required": ["part"]}
            b["then"] = {"required": ["anchor"]}
json.dump(d, open(path, "w", encoding="utf8"), ensure_ascii=False, indent=2)
open(path, "a").write("\n")
