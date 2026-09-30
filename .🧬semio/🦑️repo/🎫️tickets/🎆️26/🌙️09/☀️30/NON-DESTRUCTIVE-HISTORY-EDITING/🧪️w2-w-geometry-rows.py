#!/usr/bin/env python3
"""🧪️ W2-W geometry: rewrites the `{"kind", "params"}` Examples rows of the dxf, dwg, ply, obj, las, stl, bcf and epw
features so `params` is exactly the leaf wire payload (`payload_value()`, design §11, recipe F10).

Idempotent: every transform recognises the wire form and leaves it alone. Prints every changed row.

    python3 🧪️w2-w-geometry-rows.py [--dry-run] <artifact>...
"""
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
ARTIFACTS = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"

EPW_RECORD_COLUMNS = [
    "year", "month", "day", "hour", "minute", "dataSourceUncertainty", "dryBulbTemp", "dewPointTemp", "relativeHumidity",
    "atmosphericPressure", "extraterrestrialHorizontalRadiation", "extraterrestrialDirectNormalRadiation",
    "horizontalInfraredRadiation", "globalHorizontalRadiation", "directNormalRadiation", "diffuseHorizontalRadiation",
    "globalHorizontalIlluminance", "directNormalIlluminance", "diffuseHorizontalIlluminance", "zenithLuminance",
    "windDirection", "windSpeed", "totalSkyCover", "opaqueSkyCover", "visibility", "ceilingHeight",
    "presentWeatherObservation", "presentWeatherCodes", "precipitableWater", "aerosolOpticalDepth", "snowDepth",
    "daysSinceLastSnowfall", "albedo", "liquidPrecipDepth", "liquidPrecipQuantity",
]


def epw_record(cells):
    return cells if isinstance(cells, dict) else {column: cells[index] if index < len(cells) else "" for index, column in enumerate(EPW_RECORD_COLUMNS)}


def epw(kind, params):
    if kind == "insert-record" and "fields" in params:
        return {"index": params["index"], "record": epw_record(params["fields"])}
    if kind == "set-snapshot" and "schema" not in params["snapshot"]:
        snapshot = params["snapshot"]
        return {"snapshot": {"schema": "stdio.epw", **{key: value for key, value in snapshot.items() if key != "records"}, "records": [epw_record(row) for row in snapshot["records"]]}}
    return params


def stl(kind, params):
    if kind == "set-snapshot" and "snapshot" not in params:
        return {"snapshot": {"schema": "stdio.stl", "solidName": "replacement-triangle", "triangles": params["triangles"]}}
    return params


def las_bytes(data):
    return list(data.encode("utf-8")) if isinstance(data, str) else data


def las_vlr(vlr):
    return {**vlr, "data": las_bytes(vlr["data"])}


def las_header(header):
    if "creationYear" in header:
        return header
    (xs, ys, zs), (xo, yo, zo), (xM, yM, zM), (xm, ym, zm) = header["scale"], header["offset"], header["max"], header["min"]
    return {
        "versionMajor": header["versionMajor"], "versionMinor": header["versionMinor"], "systemIdentifier": header["systemIdentifier"],
        "generatingSoftware": header["generatingSoftware"], "creationDayOfYear": header["dayOfYear"], "creationYear": header["year"],
        "headerSize": 227, "offsetToPointData": 227, "numberOfVlrs": 0, "pointDataFormatId": 0, "pointDataRecordLength": 20,
        "numberOfPointRecords": 0, "pointsByReturn": header["counts"], "xScale": xs, "yScale": ys, "zScale": zs, "xOffset": xo,
        "yOffset": yo, "zOffset": zo, "maxX": xM, "minX": xm, "maxY": yM, "minY": ym, "maxZ": zM, "minZ": zm,
    }


def las(kind, params):
    if kind == "set-snapshot" and "snapshot" not in params:
        return {"snapshot": {"schema": "stdio.las", "header": las_header(params["header"]), "vlrs": [las_vlr(vlr) for vlr in params["vlrs"]], "points": params["points"]}}
    if kind == "insert-vlr":
        return {**params, "vlr": las_vlr(params["vlr"])}
    if kind == "set-vlr-data":
        return {**params, "data": las_bytes(params["data"])}
    return params


def obj(kind, params):
    if kind == "set-snapshot" and "schema" not in params["snapshot"]:
        snapshot = {key: value for key, value in params["snapshot"].items() if not (key == "mtllib" and value is None)}
        return {"snapshot": {"schema": "stdio.obj", **{("usemtl" if key == "usemtlRanges" else key): value for key, value in snapshot.items()}}}
    return params


PLY_FIXTURE_PROPERTIES = {"vertex": [{"name": name, "form": "scalar", "kind": "float"} for name in ["x", "y", "z", "nx", "ny", "nz", "s", "t"]]}


def ply_cell(value, prop):
    if isinstance(value, dict):
        return value
    if prop["form"] == "list":
        return {"kind": "list", "value": [{"kind": prop["valueKind"], "value": item} for item in value]}
    return {"kind": prop["kind"], "value": value}


def ply_row(row, properties):
    return {"values": [ply_cell(value, prop) for value, prop in zip(row["values"], properties)]}


def ply_element(element):
    return {**element, "rows": [ply_row(row, element["properties"]) for row in element["rows"]]}


def ply(kind, params):
    if kind == "set-snapshot" and "schema" not in params["snapshot"]:
        snapshot = params["snapshot"]
        return {"snapshot": {"schema": "stdio.ply", **snapshot, "elements": [ply_element(element) for element in snapshot["elements"]]}}
    if kind == "add-element":
        return {**params, "element": ply_element(params["element"])}
    if kind == "insert-row" and not isinstance(params["row"]["values"][0], dict):
        return {"elementName": params["elementName"], "index": params["index"], "row": ply_row(params["row"], PLY_FIXTURE_PROPERTIES[params["elementName"]])}
    if kind == "set-row-property" and not isinstance(params["value"], dict):
        prop = next(prop for prop in PLY_FIXTURE_PROPERTIES[params["elementName"]] if prop["name"] == params["propertyName"])
        return {"elementName": params["elementName"], "rowIndex": params["rowIndex"], "propertyName": params["propertyName"], "value": ply_cell(params["value"], prop)}
    return params


def dwg(kind, params):
    if kind == "set-snapshot" and "snapshot" not in params:
        return {"snapshot": {"schema": "stdio.dwg", **params}}
    return params


def dxf_entity(entity):
    if "entityKind" not in entity:
        return entity
    body = {key: value for key, value in entity.items() if key != "entityKind"}
    layer = body.pop("layer")
    if entity["entityKind"] == "insert":
        body = {"blockName": body["blockName"], "position": body["position"], "scale": [1.0, 1.0, 1.0], "rotation": 0.0}
    return {entity["entityKind"]: {**body, "layer": layer}}


def dxf_block(params):
    return {"name": params["name"], "basePoint": params["basePoint"], "entities": [dxf_entity(entity) for entity in params["entities"]]}


def dxf(kind, params):
    if kind == "set-snapshot" and "snapshot" not in params:
        header_vars = [{"name": "$ACADVER", "groupCode": 1, "value": {"kind": "str", "value": "AC1009"}}, {"name": "$INSBASE", "groupCode": 10, "value": {"kind": "point", "value": params["insertionBase"]}}]
        layers = [{**layer, "flags": 0} for layer in params["layers"]]
        return {"snapshot": {"schema": "stdio.dxf", "headerVars": header_vars, "tables": {"layers": layers}, "otherTables": [], "blocks": [], "entities": [dxf_entity(entity) for entity in params["entities"]]}}
    if kind == "set-header-var" and "value" in params:
        return {"name": params["name"], "headerVar": {"name": params["name"], "groupCode": 10, "value": {"kind": "point", "value": params["value"]}}}
    if kind in ("insert-layer", "set-layer") and "layer" not in params:
        layer = {"name": params["name"], "color": params["color"], "linetype": params["linetype"], "flags": 0}
        return {"index": params["index"], "layer": layer} if kind == "insert-layer" else {"name": params["name"], "layer": layer}
    if kind in ("insert-style", "set-style") and "style" not in params:
        style = {"name": params["name"], "flags": 0, "fontName": params["font"]}
        return {"index": params["index"], "style": style} if kind == "insert-style" else {"name": params["name"], "style": style}
    if kind in ("insert-linetype", "set-linetype") and "linetype" not in params:
        linetype = {"name": params["name"], "flags": 0, "description": params["description"]}
        return {"index": params["index"], "linetype": linetype} if kind == "insert-linetype" else {"name": params["name"], "linetype": linetype}
    if kind in ("insert-entity", "set-entity") and "entity" not in params:
        return {"index": params["index"], "entity": dxf_entity({key: value for key, value in params.items() if key != "index"})}
    if kind in ("insert-block", "set-block") and "block" not in params:
        return {"index": params["index"], "block": dxf_block(params)}
    return params


def bcf(kind, params):
    if kind == "set-snapshot" and "snapshot" not in params:
        return {"snapshot": {"schema": "stdio.bcf", **params}}
    if kind == "set-viewpoint-snapshot" and isinstance(params.get("snapshot"), str):
        return {**params, "snapshot": list(bytes.fromhex(params["snapshot"])) if params["snapshot"] else None}
    return params


TRANSFORMS = {"🌦️epw": epw, "🔺️stl": stl, "☁️las": las, "🗽️obj": obj, "🧱️ply": ply, "🖊️dwg": dwg, "🖋️dxf": dxf, "💬️bcf": bcf}


def drop_sentinel_scenarios(lines):
    """🧹️ Removes every `@id-no-mutation-baseline-*` scenario block (tag line to the next blank line, inclusive)."""
    out, skipping = [], False
    for line in lines:
        if line.strip().startswith("@id-no-mutation-baseline-"):
            skipping = True
        if skipping:
            if line.strip() == "":
                skipping = False
            continue
        out.append(line)
    return out


def rewrite_row(line, transform):
    cells = line.split("|")
    if len(cells) < 4:
        return line
    raw = cells[-2].strip()
    kind = cells[1].strip()
    if not raw.startswith("{"):
        return line
    params = json.loads(raw)
    wire = transform(kind, params)
    if wire == params:
        return line
    cells[-2] = " " + json.dumps(wire, ensure_ascii=False, separators=(", ", ": ")) + " "
    rewritten = "|".join(cells)
    assert rewritten.count("|") == line.count("|"), f"a `|` inside a cell: {rewritten}"
    return rewritten


def main():
    dry = "--dry-run" in sys.argv
    for artifact in [argument for argument in sys.argv[1:] if not argument.startswith("--")]:
        transform = TRANSFORMS[artifact]
        for feature in sorted((ARTIFACTS / artifact).rglob("🥒️.feature")):
            lines = feature.read_text().split("\n")
            out = []
            changed = 0
            for line in lines:
                rewritten = rewrite_row(line, transform) if line.lstrip().startswith("|") else line
                if rewritten != line:
                    changed += 1
                    print(f"{feature.relative_to(ARTIFACTS)}\n  - {line.strip()[:160]}\n  + {rewritten.strip()[:160]}")
                out.append(rewritten)
            dropped = drop_sentinel_scenarios(out)
            if len(dropped) != len(out):
                changed += 1
                print(f"{feature.relative_to(ARTIFACTS)}: dropped {len(out) - len(dropped)} no-mutation scenario line(s)")
            if changed and not dry:
                feature.write_text("\n".join(dropped))
            print(f"{feature.relative_to(ARTIFACTS)}: {changed} row(s) {'would change' if dry else 'changed'}")


if __name__ == "__main__":
    main()
