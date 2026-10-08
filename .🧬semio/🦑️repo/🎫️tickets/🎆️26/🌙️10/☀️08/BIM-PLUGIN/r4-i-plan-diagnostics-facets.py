#!/usr/bin/env python3
"""Idempotent facet splice for `🗺️plan-linework` and `⚠️diagnostics` (label i-plan-diagnostics).

Writes the TypeScript facet of each leaf and splices the json-schema / ts / graphql / proto facets of the shared inference aggregate by text
insertion (read, insert, write in one go, so concurrent splices of other inferences are not rewritten). The enum variant lists are read from
the Rust sources so the facets cannot drift from them.

Usage: PYTHONUTF8=1 python r4-i-plan-diagnostics-facets.py <S>/🧬️schema/💡️inferences
"""
import json
import pathlib
import re
import sys

root = pathlib.Path(sys.argv[1])
plan_rs = (root / "🗺️plan-linework" / "🦀️.rs").read_text(encoding="utf8")
diag_rs = (root / "⚠️diagnostics" / "🦀️.rs").read_text(encoding="utf8")


def variants(source, name):
    body = re.search(r"pub enum %s \{(.*?)\n\}" % name, source, re.S).group(1)
    return re.findall(r"^\s+([A-Z][A-Za-z0-9]*),?$", body, re.M)


ENUMS = {"PlanStyle": variants(plan_rs, "PlanStyle"), "PlanKind": variants(plan_rs, "PlanKind"), "Severity": variants(diag_rs, "Severity"), "DiagnosticCode": variants(diag_rs, "DiagnosticCode")}

# name -> [(field, type, optional)] ; types: number, string, boolean, ("array", t), ("map", t), or a name
STRUCTS = {
    "PlanVertex": [("x", "number", False), ("y", "number", False), ("bulge", "number", False)],
    "PlanRegion": [("id", "string", False), ("element", "string", False), ("kind", "PlanKind", False), ("style", "PlanStyle", False), ("outer", ("array", "PlanVertex"), False), ("holes", ("array", ("array", "PlanVertex")), False)],
    "PlanPolyline": [("id", "string", False), ("element", "string", False), ("kind", "PlanKind", False), ("style", "PlanStyle", False), ("closed", "boolean", False), ("vertices", ("array", "PlanVertex"), False)],
    "PlanText": [("id", "string", False), ("element", "string", False), ("kind", "PlanKind", False), ("style", "PlanStyle", False), ("x", "number", False), ("y", "number", False), ("rotation", "number", False), ("label", "string", False), ("detail", "string", False), ("measure", "number", True)],
    "PlanBounds": [("min_x", "number", False), ("min_y", "number", False), ("max_x", "number", False), ("max_y", "number", False)],
    "PlanLinework": [("storey", "string", False), ("cut_height", "number", False), ("cut_elevation", "number", False), ("regions", ("array", "PlanRegion"), False), ("polylines", ("array", "PlanPolyline"), False), ("texts", ("array", "PlanText"), False), ("bounds", "PlanBounds", False)],
    "Diagnostic": [("code", "DiagnosticCode", False), ("severity", "Severity", False), ("message_key", "string", False), ("elements", ("array", "string"), False), ("missing", ("array", "string"), False), ("storey", "string", True), ("values", ("map", "number"), False)],
}
PLAN_TYPES = ["PlanStyle", "PlanKind", "PlanVertex", "PlanRegion", "PlanPolyline", "PlanText", "PlanBounds", "PlanLinework"]
DIAGNOSTIC_TYPES = ["Severity", "DiagnosticCode", "Diagnostic"]
ALL_TYPES = PLAN_TYPES + DIAGNOSTIC_TYPES
PLAN_DOC = "🗺️ `plan-linework`: the architectural plan of a storey cut 1.2 m above its elevation as filled regions, stroked polylines (vertices carry a bulge) and text anchors, each with a style class and the id of its element."
DIAGNOSTIC_DOC = "⚠️ `diagnostics`: every finding of the model with a severity, a stable code, the element ids involved, the English and German message key and the numbers of the message."


# region json schema
def json_type(t):
    if t == "number":
        return {"type": "number"}
    if t == "string":
        return {"type": "string"}
    if t == "boolean":
        return {"type": "boolean"}
    if isinstance(t, tuple) and t[0] == "array":
        return {"type": "array", "items": json_type(t[1])}
    if isinstance(t, tuple) and t[0] == "map":
        return {"type": "object", "additionalProperties": json_type(t[1])}
    return {"$ref": "#/$defs/%s" % t}


def json_def(name):
    if name in ENUMS:
        return {"type": "string", "enum": ENUMS[name]}
    fields = STRUCTS[name]
    return {"type": "object", "additionalProperties": False, "required": [f for f, _, optional in fields if not optional], "properties": {f: json_type(t) for f, t, _ in fields}}


def indent(text, spaces):
    return "\n".join((" " * spaces + line) if line else line for line in text.splitlines())


def splice_json(path):
    text = path.read_text(encoding="utf8")
    if '"plan_linework"' in text:
        return
    properties = {"plan_linework": {"type": "object", "additionalProperties": {"$ref": "#/$defs/PlanLinework"}, "x-semio-derived": True}, "diagnostics": {"type": "array", "items": {"$ref": "#/$defs/Diagnostic"}, "x-semio-derived": True}}
    block = ",\n".join('    "%s": %s' % (name, indent(json.dumps(value, indent=2, ensure_ascii=False), 4).lstrip()) for name, value in properties.items())
    text = text.replace('\n  ],\n  "properties": {', ',\n    "plan_linework",\n    "diagnostics"\n  ],\n  "properties": {', 1)
    marker = '\n  },\n  "$defs": {'
    assert marker in text
    text = text.replace(marker, ",\n" + block + marker, 1)
    defs = ",\n".join('    "%s": %s' % (name, indent(json.dumps(json_def(name), indent=2, ensure_ascii=False), 4).lstrip()) for name in ALL_TYPES)
    end = "\n  }\n}\n"
    assert text.endswith(end)
    text = text[: -len(end)] + ",\n" + defs + end
    path.write_text(text, encoding="utf8")


# endregion


# region typescript
def ts_type(t):
    if t in ("number", "string", "boolean"):
        return t
    if isinstance(t, tuple) and t[0] == "array":
        return ts_type(t[1]) + "[]"
    if isinstance(t, tuple) and t[0] == "map":
        return "Record<string, %s>" % ts_type(t[1])
    return t


def ts_def(name):
    if name in ENUMS:
        return "export type %s = %s;\n" % (name, " | ".join('"%s"' % v for v in ENUMS[name]))
    rows = "".join("  %s%s: %s;\n" % (f, "?" if optional else "", ts_type(t)) for f, t, optional in STRUCTS[name])
    return "export interface %s {\n%s}\n" % (name, rows)


def splice_ts(path):
    text = path.read_text(encoding="utf8")
    if "plan_linework" in text:
        return
    types = "\n".join(ts_def(name) for name in ALL_TYPES)
    text = text.replace("export interface ModelInference {", types + "\nexport interface ModelInference {", 1)
    start = text.index("export interface ModelInference {")
    end = text.index("\n}\n", start)
    text = text[:end] + "\n  /** @derived */\n  plan_linework: Record<string, PlanLinework>;\n  /** @derived */\n  diagnostics: Diagnostic[];" + text[end:]
    path.write_text(text, encoding="utf8")


def write_leaf_ts(directory, doc, names):
    (directory / "🟦️.ts").write_text("/** %s */\n\n%s" % (doc, "\n".join(ts_def(name) for name in names)), encoding="utf8")


# endregion


# region graphql
def gql_type(t):
    if t == "number":
        return "Float!"
    if t == "string":
        return "String!"
    if t == "boolean":
        return "Boolean!"
    if isinstance(t, tuple) and t[0] == "array":
        inner = gql_type(t[1])
        return "[%s]!" % inner
    if isinstance(t, tuple) and t[0] == "map":
        return "[DiagnosticValue!]!"
    return t + "!"


def gql_def(name):
    if name in ENUMS:
        return "enum %s {\n%s}\n" % (name, "".join("  %s\n" % v for v in ENUMS[name]))
    rows = "".join("  %s: %s\n" % (f, gql_type(t).rstrip("!") if optional else gql_type(t)) for f, t, optional in STRUCTS[name])
    return "type %s {\n%s}\n" % (name, rows)


def splice_graphql(path):
    text = path.read_text(encoding="utf8")
    if "plan_linework" in text:
        return
    start = text.index("type ModelInference {")
    end = text.index("\n}\n", start)
    text = text[:end] + "\n  plan_linework: [PlanLineworkRow!]! @derived\n  diagnostics: [Diagnostic!]! @derived" + text[end:]
    extra = "type PlanLineworkRow {\n  id: String!\n  value: PlanLinework!\n}\n\ntype DiagnosticValue {\n  name: String!\n  value: Float!\n}\n\n"
    text = text.rstrip("\n") + "\n\n" + extra + "\n".join(gql_def(name) for name in ALL_TYPES)
    path.write_text(text, encoding="utf8")


# endregion


# region proto
def snake(name):
    return re.sub(r"(?<!^)(?=[A-Z])", "_", name).upper()


def proto_type(t):
    if t == "number":
        return "double"
    if t == "string":
        return "string"
    if t == "boolean":
        return "bool"
    if isinstance(t, tuple) and t[0] == "array":
        return "repeated " + proto_type(t[1])
    if isinstance(t, tuple) and t[0] == "map":
        return "map<string, %s>" % proto_type(t[1])
    return t


def proto_def(name):
    if name == "PlanRegion":
        return "message PlanRegion {\n  string id = 1;\n  string element = 2;\n  PlanKind kind = 3;\n  PlanStyle style = 4;\n  repeated PlanVertex outer = 5;\n  repeated PlanVertexRing holes = 6;\n}\n"
    if name in ENUMS:
        rows = "".join("  %s_%s = %d;\n" % (snake(name), snake(v), i) for i, v in enumerate(ENUMS[name]))
        return "enum %s {\n%s}\n" % (name, rows)
    rows = ""
    for index, (f, t, optional) in enumerate(STRUCTS[name], 1):
        if isinstance(t, tuple) and t[0] == "array" and isinstance(t[1], tuple):
            raise SystemExit("nested arrays need a wrapper message: %s.%s" % (name, f))
        rows += "  %s%s %s = %d;\n" % ("optional " if optional else "", proto_type(t), f, index)
    return "message %s {\n%s}\n" % (name, rows)


def splice_proto(path):
    text = path.read_text(encoding="utf8")
    if "plan_linework" in text:
        return
    start = text.index("message ModelInference {")
    end = text.index("\n}\n", start)
    tags = [int(tag) for tag in re.findall(r"= (\d+);", text[start:end])]
    first = max(tags) + 1
    text = text[:end] + "\n  // @derived\n  map<string, PlanLinework> plan_linework = %d;\n  // @derived\n  repeated Diagnostic diagnostics = %d;" % (first, first + 1) + text[end:]
    holes = "message PlanVertexRing {\n  repeated PlanVertex vertices = 1;\n}\n"
    text = text.rstrip("\n") + "\n\n" + holes + "\n" + "\n".join(proto_def(name) for name in ALL_TYPES)
    path.write_text(text, encoding="utf8")


# endregion


write_leaf_ts(root / "🗺️plan-linework", PLAN_DOC, PLAN_TYPES)
write_leaf_ts(root / "⚠️diagnostics", DIAGNOSTIC_DOC, DIAGNOSTIC_TYPES)
splice_json(root / "🔣️.json")
splice_ts(root / "🟦️.ts")
splice_graphql(root / "🔗️.graphql")
splice_proto(root / "🛰️.proto")
print("facets spliced")
