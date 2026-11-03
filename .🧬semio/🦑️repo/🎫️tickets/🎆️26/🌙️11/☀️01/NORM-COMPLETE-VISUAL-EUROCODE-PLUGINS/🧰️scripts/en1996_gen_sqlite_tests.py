"""Applies the EN 1996 wave 2 entities to the SQLite tests and the shared sqlite fixture. Idempotent."""
import os, re
HERE = os.path.dirname(os.path.abspath(__file__))
exec(compile(open(os.path.join(HERE, "en1996_spec.py"), encoding="utf-8").read(), "en1996_spec.py", "exec"))
T = f"{IO}/🪶️sqlite/📸️snapshot/🧪️tests"
FL = [x for x in SCALARS if x["kind"] == "f64"]
# rust
p = f"{T}/🦀️.rs"; s = rd(p)
if "wall.cover_m=value" not in s:
    s = s.replace("wall.phi_infinity=value;", "wall.phi_infinity=value;" + "".join(f"wall.{x['name']}=value;" for x in FL) + "".join(
        f"for row in &mut wall.{r['coll']}{{" + "".join(f"row.{f['name']}=value;" for f in r["fields"] if f["kind"] == "f64") + "}" for r in ROWS), 1)
    s = s.replace("opening.sill_height_m=value;}", "opening.sill_height_m=value;opening.position_m=value;}", 1)
    s = s.replace("load.bearing_length_m=value;}", "load.bearing_length_m=value;load.distance_a1_m=value;load.position_m=value;}", 1)
    s = s.replace('("exposure",false)]', '("exposure",false),("frostClass",false)]', 1) if False else s
    wr(p, s)
# ts
p = f"{T}/🟦️.ts"; s = rd(p)
if "coverM=v" not in s:
    s = s.replace("sillHeightM:binary64(o.sillHeightM)}))", "sillHeightM:binary64(o.sillHeightM),positionM:binary64(o.positionM)}))", 1)
    s = s.replace("bearingLengthM:binary64(l.bearingLengthM)}))", "bearingLengthM:binary64(l.bearingLengthM),distanceA1M:binary64(l.distanceA1M),positionM:binary64(l.positionM)}))", 1)
    conv = ""
    for r in ROWS:
        conv += f",{r['coll']}:w.{r['coll']}.map(x=>({{...x," + ",".join(f"{f['camel']}:binary64(x.{f['camel']})" if f["kind"] == "f64" else f"{f['camel']}:x.{f['camel']} as En1996Sqlite{r['struct']}[\"{f['camel']}\"]" for f in r["fields"]) + "}))"
    conv += ",frostClass:w.frostClass as En1996SqliteWall[\"frostClass\"]" + "".join(f",{x['camel']}:binary64(w.{x['camel']})" for x in FL)
    s = s.replace("phiInfinity:binary64(w.phiInfinity),", "phiInfinity:binary64(w.phiInfinity)" + conv + ",", 1)
    s = s.replace("type En1996SqliteWall }", "type En1996SqliteWall, " + ", ".join(f"type En1996Sqlite{r['struct']}" for r in ROWS) + " }", 1)
    s = s.replace("w.phiInfinity=v;", "w.phiInfinity=v;" + "".join(f"w.{x['camel']}=v;" for x in FL) + "".join(f"for(const x of w.{r['coll']}){{" + "".join(f"x.{f['camel']}=v;" for f in r["fields"] if f["kind"] == "f64") + "}" for r in ROWS), 1)
    s = s.replace("o.sillHeightM=v}", "o.sillHeightM=v;o.positionM=v}", 1)
    s = s.replace("l.bearingLengthM=v}", "l.bearingLengthM=v;l.distanceA1M=v;l.positionM=v}", 1)
    wr(p, s)
# choices fixture
p = f"{T}/🧫️fixtures/🛂️choices.json"; d = jload(p)
for n, e in ENUMS.items():
    d[camel(n)] = e["variants"]
jdump(p, d)
# neutral sqlite fixture
p = f"{T}/🧫️fixtures/🔣️.json"; d = jload(p)
if "piers" not in d["walls"][0]:
    for w in d["walls"]:
        for o in w["openings"]:
            o["positionM"] = 0.5 if o is w["openings"][0] else -0.25
        for lc in w["loadCases"]:
            for c in lc["concentrated"]:
                c["distanceA1M"] = 0.25
                c["positionM"] = 1.5
        w["piers"] = [{"id": "pier 世界", "positionM": 3.0, "widthM": 0.9}]
        w["lintels"] = [{"id": "lintel 1", "positionM": 0.5, "spanM": 1.5, "depthM": 0.25, "bearingLengthM": 0.25}]
        w["ties"] = [{"id": "tie 1", "kind": "Anchor", "positionM": 1.0, "levelM": 2.5, "capacityN": 12000.5}, {"id": "tie 2", "kind": "RingTie", "positionM": -1.0, "levelM": 0.0, "capacityN": 0.0}]
        w["prestress"] = [{"id": "tendon 1", "forceN": 90000.25, "areaM2": 0.0002, "fPkPa": 1770000000.5, "positionM": 2.5, "eccentricityM": -0.05, "lossRatio": 0.2}]
        w["confined"] = [{"id": "column 1", "positionM": 2.5, "widthM": 0.25, "depthM": 0.24, "asM2": 0.0006}]
        w["flanges"] = [{"id": "flange 1", "lengthM": 1.2, "thicknessM": 0.24, "positionM": 0.0}]
        w["bars"] = [{"id": "bar 1", "kind": "Shear", "diameterM": 0.012, "lengthM": 0.6, "spacingM": 0.4}, {"id": "bar 2", "kind": "DeepBeam", "diameterM": 0.016, "lengthM": 1.0, "spacingM": 0.2}]
        w.update(frostClass="Severe", coverM=0.035, deflectionLimit=300.0, soilHeightM=1.8, soilDensityKgM3=1900.25, isothermDepthM=0.04, planXM=2.0, planYM=-3.0, planAngleDeg=90.0, inPlaneShearN=15000.5, inPlaneMomentNm=-20000.25)
    jdump(p, d)
print("sqlite tests done")
