"""🏘️ Wave W07: the inference facets (json schema, ts, graphql, proto) of the finish rows and the zone and scheme totals.

The aggregate facets of `s.bim.model.inference` are hand maintained, so this one-shot adds exactly the new members to the current text of each file and is idempotent: a member that is
already there is left alone. Run from the repo root: `python r10-w07-zones-facets.py`.
Added: `FinishSurface`, `FinishQuantity`, `ElementQuantity.finishes`, `QuantityTotals.finishes`, `ZoneTotals`, `SchemeTotals`, `ModelInference.zone_totals` and `ModelInference.scheme_totals`,
and the field slug files `🎨️finishes/🟦️.ts` and `🏘️zones/🟦️.ts`.
"""
import json
import os
import re
import sys

sys.stdout.reconfigure(encoding="utf-8")
I = "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/"


def put(path, text):
    try:
        with open(path, "w", encoding="utf8", newline="") as handle:
            handle.write(text)
    except OSError:
        tmp = path + ".w07tmp"
        with open(tmp, "w", encoding="utf8", newline="") as handle:
            handle.write(text)
        os.replace(tmp, path)


def edit(name, change):
    path = I + name
    before = open(path, encoding="utf8", newline="").read()
    after = change(before)
    if after != before:
        put(path, after)
    print(name, "updated" if after != before else "unchanged")


ZONE = ["spaces:u32", "resolved:u32", "area:f64", "net_area:f64", "volume:f64", "occupancy:f64", "floor_finish_area:f64", "wall_finish_area:f64", "ceiling_finish_area:f64"]
SCHEME = ["spaces:u32", "resolved:u32", "area:f64", "volume:f64", "occupancy:f64"]


def parsed(rows):
    return [tuple(row.split(":")) for row in rows]


# region 🔖️Json
def json_facet(text):
    doc = json.loads(text)
    defs = doc["$defs"]
    number = {"type": "number"}
    integer = {"type": "integer"}
    kind = lambda t: integer if t == "u32" else number
    defs.setdefault("FinishSurface", {"enum": ["Floor", "Wall", "Ceiling"]})
    defs.setdefault("FinishQuantity", {"type": "object", "additionalProperties": False, "required": ["surface", "material", "area"], "properties": {"surface": {"$ref": "#/$defs/FinishSurface"}, "material": {"type": "string"}, "area": number}})
    quantity = defs["ElementQuantity"]
    if "finishes" not in quantity["properties"]:
        quantity["properties"]["finishes"] = {"type": "array", "items": {"$ref": "#/$defs/FinishQuantity"}}
        quantity["required"].append("finishes")
    totals = defs["QuantityTotals"]
    if "finishes" not in totals["properties"]:
        totals["properties"]["finishes"] = {"type": "object", "additionalProperties": {"$ref": "#/$defs/Totals"}}
        totals["required"].append("finishes")
    for name, rows in (("ZoneTotals", ZONE), ("SchemeTotals", SCHEME)):
        defs.setdefault(name, {"type": "object", "additionalProperties": False, "required": [row[0] for row in parsed(rows)], "properties": {row[0]: kind(row[1]) for row in parsed(rows)}})
    for field, name in (("zone_totals", "ZoneTotals"), ("scheme_totals", "SchemeTotals")):
        if field not in doc["properties"]:
            doc["properties"][field] = {"type": "object", "additionalProperties": {"$ref": "#/$defs/" + name}, "x-semio-derived": True}
            doc["required"].append(field)
    return json.dumps(doc, indent=2, ensure_ascii=False) + "\n"


# endregion 🔖️Json


# region 🔖️TypeScript
def ts_interface(name, rows):
    fields = "\n".join("  %s: number;" % row[0] for row in parsed(rows))
    return "export interface %s {\n%s\n}\n\n" % (name, fields)


def ts_facet(text):
    if "export type FinishSurface" not in text:
        text = text.replace("export interface LayerQuantity {", 'export type FinishSurface = "Floor" | "Wall" | "Ceiling";\n\nexport interface FinishQuantity {\n  surface: FinishSurface;\n  material: string;\n  area: number;\n}\n\nexport interface LayerQuantity {', 1)
    if "  finishes: FinishQuantity[];" not in text:
        text = text.replace("  layers: LayerQuantity[];\n}", "  layers: LayerQuantity[];\n  finishes: FinishQuantity[];\n}", 1)
    if "  finishes: Record<string, Totals>;" not in text:
        text = text.replace("  materials: Record<string, Totals>;\n}", "  materials: Record<string, Totals>;\n  finishes: Record<string, Totals>;\n}", 1)
    if "export interface ZoneTotals" not in text:
        text = text.replace("export interface Totals {", ts_interface("ZoneTotals", ZONE) + ts_interface("SchemeTotals", SCHEME) + "export interface Totals {", 1)
    if "zone_totals" not in text:
        text = re.sub(r"(export interface ModelInference \{.*?)(\n\})", lambda m: m.group(1) + "\n  /** @derived */\n  zone_totals: Record<string, ZoneTotals>;\n  /** @derived */\n  scheme_totals: Record<string, SchemeTotals>;" + m.group(2), text, count=1, flags=re.S)
    return text


# endregion 🔖️TypeScript


# region 🔖️GraphQL
def gql_type(name, rows):
    fields = "\n".join("  %s: %s!" % (row[0], "Int" if row[1] == "u32" else "Float") for row in parsed(rows))
    return "type %s {\n%s\n}\n\n" % (name, fields)


def graphql_facet(text):
    if "enum FinishSurface" not in text:
        text = text.replace("type LayerQuantity {", "enum FinishSurface {\n  Floor\n  Wall\n  Ceiling\n}\n\ntype FinishQuantity {\n  surface: FinishSurface!\n  material: String!\n  area: Float!\n}\n\ntype LayerQuantity {", 1)
    if "  finishes: [FinishQuantity!]!" not in text:
        text = text.replace("  layers: [LayerQuantity!]!\n}", "  layers: [LayerQuantity!]!\n  finishes: [FinishQuantity!]!\n}", 1)
    if "  finishes: [TotalsRow!]!" not in text:
        text = text.replace("  materials: [TotalsRow!]!\n}", "  materials: [TotalsRow!]!\n  finishes: [TotalsRow!]!\n}", 1)
    if "type ZoneTotals" not in text:
        text = text.replace("type Totals {", gql_type("ZoneTotals", ZONE) + gql_type("SchemeTotals", SCHEME) + "type Totals {", 1)
    if "type ZoneTotalsRow" not in text:
        text = text.replace("type StoreyLevelRow {", "type ZoneTotalsRow {\n  id: String!\n  value: ZoneTotals!\n}\n\ntype SchemeTotalsRow {\n  id: String!\n  value: SchemeTotals!\n}\n\ntype StoreyLevelRow {", 1)
    if "zone_totals" not in text:
        text = re.sub(r"(type ModelInference \{.*?)(\n\})", lambda m: m.group(1) + "\n  zone_totals: [ZoneTotalsRow!]! @derived\n  scheme_totals: [SchemeTotalsRow!]! @derived" + m.group(2), text, count=1, flags=re.S)
    return text


# endregion 🔖️GraphQL


# region 🔖️Proto
def proto_message(name, rows):
    fields = "\n".join("  %s %s = %d;" % ("uint32" if row[1] == "u32" else "double", row[0], index + 1) for index, row in enumerate(parsed(rows)))
    return "message %s {\n%s\n}\n\n" % (name, fields)


def proto_facet(text):
    if "enum FinishSurface" not in text:
        text = text.replace("message LayerQuantity {", "enum FinishSurface {\n  FINISH_SURFACE_FLOOR = 0;\n  FINISH_SURFACE_WALL = 1;\n  FINISH_SURFACE_CEILING = 2;\n}\n\nmessage FinishQuantity {\n  FinishSurface surface = 1;\n  string material = 2;\n  double area = 3;\n}\n\nmessage LayerQuantity {", 1)
    if "repeated FinishQuantity finishes" not in text:
        text = re.sub(r"(message ElementQuantity \{.*?)(\n\})", lambda m: m.group(1) + "\n  repeated FinishQuantity finishes = %d;" % (len(re.findall(r"= \d+;", m.group(1))) + 1) + m.group(2), text, count=1, flags=re.S)
    if "map<string, Totals> finishes" not in text:
        text = re.sub(r"(message QuantityTotals \{.*?)(\n\})", lambda m: m.group(1) + "\n  map<string, Totals> finishes = %d;" % (len(re.findall(r"= \d+;", m.group(1))) + 1) + m.group(2), text, count=1, flags=re.S)
    if "message ZoneTotals" not in text:
        text = text.replace("message StoreyLevel {", proto_message("ZoneTotals", ZONE) + proto_message("SchemeTotals", SCHEME) + "message StoreyLevel {", 1)
    if "zone_totals" not in text:
        def add(match):
            body = match.group(1)
            top = max(int(number) for number in re.findall(r"= (\d+);", body))
            return body + "\n  // @derived\n  map<string, ZoneTotals> zone_totals = %d;\n  // @derived\n  map<string, SchemeTotals> scheme_totals = %d;" % (top + 1, top + 2) + match.group(2)

        text = re.sub(r"(message ModelInference \{.*?)(\n\})", add, text, count=1, flags=re.S)
    return text


# endregion 🔖️Proto

edit("🔣️.json", json_facet)
edit("🟦️.ts", ts_facet)
edit("🔗️.graphql", graphql_facet)
edit("🛰️.proto", proto_facet)

finishes = "/** 🎨️ `finishes`: the floor, wall and ceiling finish areas of a room: the material the space names (empty when unfinished) and the area it covers, per surface. */\n\nexport type FinishSurface = \"Floor\" | \"Wall\" | \"Ceiling\";\n\nexport interface FinishQuantity {\n  surface: FinishSurface;\n  material: string;\n  area: number;\n}\n"
zones = "/** 🏘️ `zones`: what the zones and the area schemes add up over the rooms of their spaces. */\n\n" + ts_interface("ZoneTotals", ZONE) + ts_interface("SchemeTotals", SCHEME).rstrip("\n") + "\n"
for directory, body in (("🎨️finishes", finishes), ("🏘️zones", zones)):
    path = I + directory + "/🟦️.ts"
    if not os.path.exists(path) or open(path, encoding="utf8").read() != body:
        put(path, body)
        print(directory + "/🟦️.ts written")
