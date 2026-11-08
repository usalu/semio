#!/usr/bin/env python3
"""EN 1998: writes the collections isolationSystems, pierBases and criticalRegions (rows En1998IsolationSystem, En1998PierBase, En1998CriticalRegion) into the snapshot, combined and diff JSON schemas (idempotent); the twins are generated afterwards by `schema twins generate`."""
import copy, json, sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent / "field-adder"))
import schema_twins as st

SCHEMA = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/"
SNAP, COMB, DIFF = SCHEMA + "📸️snapshot/🔣️.json", SCHEMA + "🔣️.json", SCHEMA + "🔺️diff/🔣️.json"
S = "String"
COLLS = [
    ("isolationSystems", "En1998IsolationSystem", "isolation_systems", "IsolationSystem", [("bridgeId", S), ("buildingId", S), ("strengthN", "f64"), ("stiffnessNPerM", "f64"), ("yieldDisplacementM", "f64"), ("capacityM", "f64")]),
    ("pierBases", "En1998PierBase", "pier_bases", "PierBase", [("bridgeId", S), ("pierId", S), ("radiusM", "f64"), ("soilVsMS", "f64"), ("soilUnitWeightNM3", "f64"), ("poissonRatio", "f64")]),
    ("criticalRegions", "En1998CriticalRegion", "critical_regions", "CriticalRegion", [("buildingId", S), ("memberId", S)] + [(n, "f64") for n in "sectionWidthM sectionDepthM clearLengthM hoopedLengthM coreWidthM coreDepthM axialForceN concreteDesignStrengthMpa steelDesignYieldMpa hoopDesignYieldMpa longitudinalSteelAreaM2 longitudinalBarDiameterM longitudinalBarMaxDiameterM hoopDiameterM hoopSpacingM hoopLegsX hoopLegsY restrainedBarCount restrainedBarSpacingM".split()]),
]
PRIMARY = "https://json.schemas.assets.semio-tech.com/s/norm/en1998/snapshot.json#/$defs/"
JSN = lambda t: {"type": "number"} if t == "f64" else {"type": "string"}


def row_def(data, title, fields):
    base = copy.deepcopy(data["$defs"]["En1998Slope"])
    base["title"] = title
    base["required"] = ["id"] + [n for n, _ in fields]
    base["properties"] = {"id": {"type": "string"}, **{n: JSN(t) for n, t in fields}}
    return base


def snapshot_like(path, with_dsl):
    def mutate(data):
        if "isolationSystems" in data["properties"]:
            return False
        slopes = data["properties"]["slopes"]
        for coll, title, _, _, fields in COLLS:
            node = copy.deepcopy(slopes)
            node["items"] = {"$ref": "#/$defs/" + title}
            data["$defs"][title] = row_def(data, title, fields)
            data["properties"][coll] = node
            data["required"].append(coll)
    return st.edit_schema(path, mutate)


def diff(path):
    def mutate(data):
        if "isolationSystems" in data["properties"]:
            return False
        d = data["$defs"]
        for coll, title, _, _, fields in COLLS:
            sub = lambda k: json.loads(json.dumps(d["En1998Slope" + k]).replace("En1998Slope", title))
            for k in ("Delta", "Removal", "Insertion", "Relocation", "Modification"):
                d[title + k] = sub(k)
            patch = sub("Patch")
            patch["properties"] = {n: {"anyOf": [{"$ref": PRIMARY + title + "/properties/" + n}, {"type": "null"}]} for n, _ in fields}
            d[title + "Patch"] = patch
            data["properties"][coll] = {"anyOf": [{"$ref": "#/$defs/" + title + "Delta"}], "x-semio-state": "artifact"}
            if coll in data.get("required", []):
                continue
            data["required"].append(coll)
    return st.edit_schema(path, mutate)


print(snapshot_like(SNAP, True), snapshot_like(COMB, False), diff(DIFF))
