"""Hand-fixes every committed EN 1996 mutation fixture for the wave 2 entities and writes the apply vectors of the new leaves. Idempotent."""
import copy, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
exec(compile(open(os.path.join(HERE, "en1996_spec.py"), encoding="utf-8").read(), "en1996_spec.py", "exec"))

OPEN_POS = {"win-1": 0.8, "door-1": 3.4, "win-2": 2.0}
EMPTY_DELTA = {"removed": [], "inserted": [], "moved": [], "modified": []}
SCALAR_DEFAULT = {s["camel"]: s["default"] for s in SCALARS}


def upgrade(node):
    if isinstance(node, list):
        for x in node:
            upgrade(x)
        return
    if not isinstance(node, dict):
        return
    for v in list(node.values()):
        upgrade(v)
    if "phiInfinity" in node and "isBasement" in node:
        if isinstance(node.get("loadCases"), dict):
            for s in SCALARS:
                node.setdefault(s["camel"], None)
            for r in ROWS:
                node.setdefault(r["coll"], copy.deepcopy(EMPTY_DELTA))
        elif isinstance(node.get("loadCases"), list):
            for r in ROWS:
                node.setdefault(r["coll"], [])
            for s in SCALARS:
                node.setdefault(s["camel"], s["default"])
    if "sillHeightM" in node and "widthM" in node and "positionM" not in node:
        node["positionM"] = OPEN_POS.get(node.get("id"), 2.0) if "id" in node else None
    if "bearingLengthM" in node and "forceN" in node and "distanceA1M" not in node:
        node["distanceA1M"] = 0.0 if "id" in node else None
        node["positionM"] = 1.0 if "id" in node else None


def base_snapshot():
    d = jload(f"{FIX}/↔️change-wall-length/✅apply/🔣️.before.json")
    w = d["walls"][0]
    w["openings"] = [{"id": "win-1", "widthM": 1.2, "heightM": 1.4, "sillHeightM": 0.9, "positionM": 0.8}]
    w["loadCases"][0]["concentrated"] = [{"id": "beam-A", "forceN": 30000.0, "bearingAreaM2": 0.04, "bearingLengthM": 0.2, "distanceA1M": 0.0, "positionM": 1.0}]
    w["piers"] = [{"id": "pier-1", "positionM": 2.0, "widthM": 1.4}]
    w["lintels"] = [{"id": "lintel-1", "positionM": 0.6, "spanM": 1.4, "depthM": 0.2, "bearingLengthM": 0.2}]
    w["ties"] = [{"id": "tie-1", "kind": "WallTie", "positionM": 1.0, "levelM": 1.0, "capacityN": 1500.0}]
    w["prestress"] = [{"id": "tendon-1", "forceN": 80000.0, "areaM2": 0.00015, "fPkPa": 1.5e9, "positionM": 2.0, "eccentricityM": 0.02, "lossRatio": 0.1}]
    w["confined"] = [{"id": "column-1", "positionM": 4.9, "widthM": 0.24, "depthM": 0.24, "asM2": 0.0005}]
    w["flanges"] = [{"id": "flange-1", "lengthM": 1.0, "thicknessM": 0.2, "positionM": 0.0}]
    w["bars"] = [{"id": "bar-1", "kind": "Anchorage", "diameterM": 0.01, "lengthM": 0.5, "spacingM": 0.3}]
    return d


def empty_wall_patch():
    p = {}
    for k in ("labelEn", "labelDe", "wallType", "thicknessM", "heightM", "lengthM", "supportSides", "slabBearingDepthM", "eccentricityTopM", "eccentricityBottomM", "unitGroup", "unitMaterial", "fBPa", "unitLengthM", "unitWidthM", "unitHeightM", "mortarType", "mortarClass", "mortarStrengthPa", "bedJointThicknessM", "reinforced", "asVerticalM2", "asHorizontalM2", "fYdPa", "fireReiMin", "exposure", "mu", "densityKgM3", "phiInfinity", "isBasement"):
        p[k] = None
    for s in SCALARS:
        p[s["camel"]] = None
    p["openings"] = copy.deepcopy(EMPTY_DELTA)
    p["loadCases"] = copy.deepcopy(EMPTY_DELTA)
    for r in ROWS:
        p[r["coll"]] = copy.deepcopy(EMPTY_DELTA)
    return p


def diff_doc(wall_id, patch):
    return {"annex": None, "masonryClass": None, "designSituation": None, "storeys": None, "walls": {"removed": [], "inserted": [], "moved": [], "modified": [{"id": wall_id, "patch": patch}]}}


def row_by_coll(coll):
    return next(r for r in ROWS if r["coll"] == coll)


def modification(delta, id_, patch):
    delta["modified"].append({"id": id_, "patch": patch})


def write_vector(l, before, mutation, after, diff):
    d = f"{FIX}/{dirname(l)}/✅apply"
    jdump(f"{d}/🔣️.before.json", before)
    jdump(f"{d}/🔣️.after.json", after)
    jdump(f"{d}/🔣️.mutation.json", {pascal(l["kind"]): mutation})
    jdump(f"{d}/🔣️.outcome.json", {"status": "applied"})
    jdump(f"{d}/🔣️.diff.json", diff)


def new_vectors():
    for l in leaves():
        if os.path.exists(f"{FIX}/{dirname(l)}/✅apply/🔣️.mutation.json"):
            continue
        before = base_snapshot()
        after = copy.deepcopy(before)
        wb, wa = before["walls"][0], after["walls"][0]
        wid = wb["id"]
        patch = empty_wall_patch()
        op = l["op"]
        if op in ("insert", "remove"):
            r = l["row"]
            coll = r["coll"]
            if op == "insert":
                row = {"id": r["new_id"]}
                for f in r["fields"]:
                    row[f["camel"]] = f["sample"] if f["kind"] != "f64" else float(f["sample"])
                wa[coll].append(row)
                patch[coll]["inserted"].append({"index": 1, "row": row})
                write_vector(l, before, {"wallIndex": 0, "index": 1, "row": row}, after, diff_doc(wid, patch))
            else:
                removed = wa[coll].pop(0)
                patch[coll]["removed"].append({"id": removed["id"], "index": 0})
                write_vector(l, before, {"wallIndex": 0, "index": 0}, after, diff_doc(wid, patch))
            continue
        f = l["field"]
        val = f["sample"] if f["kind"] != "f64" else float(f["sample"])
        cam = f["camel"]
        newk = "new" + cam[:1].upper() + cam[1:]
        if op == "scalar":
            wa[cam] = val
            patch[cam] = val
            write_vector(l, before, {"index": 0, newk: val}, after, diff_doc(wid, patch))
        elif op == "field":
            coll = l["row"]["coll"]
            wa[coll][0][cam] = val
            modification(patch[coll], wb[coll][0]["id"], {k["camel"]: None for k in l["row"]["fields"]} | {cam: val})
            write_vector(l, before, {"wallIndex": 0, "index": 0, newk: val}, after, diff_doc(wid, patch))
        elif op == "opening":
            wa["openings"][0][cam] = val
            modification(patch["openings"], "win-1", {"widthM": None, "heightM": None, "sillHeightM": None, "positionM": None} | {cam: val})
            write_vector(l, before, {"wallIndex": 0, "index": 0, newk: val}, after, diff_doc(wid, patch))
        else:
            wa["loadCases"][0]["concentrated"][0][cam] = val
            lc = {k: None for k in ("designSituation", "imposedCategory", "gKSlabN", "qKImposedPa", "tributaryAreaM2", "slabSpanM", "qKSnowPa", "qPWindPa", "cPe", "hKEarthN")}
            lc["concentrated"] = copy.deepcopy(EMPTY_DELTA)
            modification(lc["concentrated"], "beam-A", {"forceN": None, "bearingAreaM2": None, "bearingLengthM": None, "distanceA1M": None, "positionM": None} | {cam: val})
            modification(patch["loadCases"], wb["loadCases"][0]["id"], lc)
            write_vector(l, before, {"wallIndex": 0, "loadCaseIndex": 0, "index": 0, newk: val}, after, diff_doc(wid, patch))


def upgrade_existing():
    for root, _, files in os.walk(FIX):
        for fn in files:
            if not fn.endswith(".json"):
                continue
            p = os.path.join(root, fn)
            txt = rd(p)
            if "phiInfinity" not in txt and "sillHeightM" not in txt and "bearingLengthM" not in txt:
                continue
            d = json.loads(txt)
            before = json.dumps(d, sort_keys=True)
            upgrade(d)
            if json.dumps(d, sort_keys=True) != before:
                jdump(p, d)


upgrade_existing()
new_vectors()
print("fixtures done")
