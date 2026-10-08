# 🧬️ Generates the schema facets (json, ts, proto, graphql) of the BIM editor window configs and the window transient
# from the field tables below. The Rust declarations (`window_config!` invocations, `BimWindowTransient`) are the source of
# truth: keep the tables in sync with them and re-run `python r4-u-editor-gen-facets.py <repo root>`.
import json
import os
import sys

ROOT = sys.argv[1]
S = "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
W = S + "/🎭️modes/✏️edit/🪟️windows"
FW = "🧰️framework/🔨️modules/🖱️ui/🪟️viewport"
DOMAIN = "https://json.schemas.assets.semio-tech.com"

PROJECTION = [
    ("kind", "string"), ("orthographicView", "string"), ("axonometricVariant", "string"), ("axonometricAngleA", "double"), ("axonometricAngleB", "double"),
    ("axonometricQuadrant", "string"), ("obliqueVariant", "string"), ("obliqueAngle", "double"), ("obliqueDepth", "double"), ("onePointAxis", "string"),
    ("fov", "double"), ("twoPointShift", "double"), ("curvilinearFov", "double"), ("curvilinearStrength", "double"), ("curvilinearMapping", "string"),
]

LANES = [
    dict(dir=W + "/🗺️plan/🎚️config/🧬️schema", name="BimPlanWindowConfig", slug="plan-window-config", package="plan_window_config", doc="Persisted navigation and storey of one exact BIM plan window.",
         fields=[("storey", "string"), ("cutHeight", "double"), ("framed", "bool"), ("viewport", "Viewport2d")], defs={}),
    dict(dir=W + "/🧊️world/🎚️config/🧬️schema", name="BimWorldWindowConfig", slug="world-window-config", package="world_window_config", doc="Persisted camera, projection, storey visibility and section plane of one exact BIM world window.",
         fields=[("camera", "Viewport3dOrbit"), ("projection", "def:BimWorldProjection"), ("isolatedStorey", "string"), ("hiddenStoreys", "[string]"), ("sectionEnabled", "bool"), ("sectionAxis", "string"), ("sectionOffset", "double"), ("framed", "bool")],
         defs={"BimWorldProjection": PROJECTION}),
    dict(dir=W + "/📐️section/🎚️config/🧬️schema", name="BimSectionWindowConfig", slug="section-window-config", package="section_window_config", doc="Persisted section line, depth and navigation of one exact BIM section window.",
         fields=[("startX", "double"), ("startY", "double"), ("endX", "double"), ("endY", "double"), ("depth", "double"), ("framed", "bool"), ("viewport", "Viewport2d")], defs={}),
    dict(dir=S + "/🫧️transient/🧬️schema", name="BimWindowTransient", slug="window-transient", package="window_transient", doc="Ephemeral local interaction state of one exact BIM window.",
         fields=[("engagementInput", "string"), ("pointerGeneration", "uint")], defs={}),
]

JSON = {"string": {"type": "string"}, "double": {"type": "number"}, "bool": {"type": "boolean"}, "uint": {"type": "integer", "minimum": 0, "maximum": 9007199254740991}}
TS = {"string": "string", "double": "number", "bool": "boolean", "uint": "number"}
PROTO = {"string": "string", "double": "double", "bool": "bool", "uint": "uint64"}
GQL = {"string": "String", "double": "Float", "bool": "Boolean", "uint": "Long"}
REFS = {"Viewport2d": ("viewport/2d", "◻️2d", "parseViewport2d", "Viewport2d"), "Viewport3dOrbit": ("viewport/3d", "🧊️3d", "parseViewport3dOrbit", "Viewport3dOrbit")}


def snake(name):
    return "".join("_" + c.lower() if c.isupper() else c for c in name)


def json_type(kind):
    if kind.startswith("def:"):
        return {"$ref": "#/$defs/" + kind[4:]}
    if kind.startswith("["):
        return {"type": "array", "items": JSON[kind[1:-1]]}
    if kind in REFS:
        return {"$ref": f"{DOMAIN}/framework/ui/{REFS[kind][0]}"}
    return dict(JSON[kind])


def object_schema(fields):
    return {"type": "object", "additionalProperties": False, "required": [name for name, _ in fields], "properties": {name: json_type(kind) for name, kind in fields}}


def write(lane, filename, text):
    folder = os.path.join(ROOT, lane["dir"])
    os.makedirs(folder, exist_ok=True)
    with open(os.path.join(folder, filename), "w", encoding="utf-8", newline="\n") as handle:
        handle.write(text)


def emit_json(lane):
    document = {"$schema": "http://json-schema.org/draft-07/schema#", "$id": f"{DOMAIN}/app/bim/model/{lane['slug']}/schema.json", **object_schema(lane["fields"])}
    if lane["defs"]:
        document["$defs"] = {name: object_schema(fields) for name, fields in lane["defs"].items()}
    write(lane, "🔣️.json", json.dumps(document, indent=2, ensure_ascii=False) + "\n")


def ts_check(kind, expr, at):
    if kind.startswith("def:"):
        return f"parse{kind[4:]}({expr}, {at})"
    if kind.startswith("["):
        return f"list({expr}, {at}, text)"
    if kind in REFS:
        return f"{REFS[kind][2]}({expr})"
    return {"string": f"text({expr}, {at})", "double": f"num({expr}, {at})", "bool": f"flag({expr}, {at})", "uint": f"count({expr}, {at})"}[kind]


def ts_type(kind):
    if kind.startswith("def:"):
        return kind[4:]
    if kind.startswith("["):
        return TS[kind[1:-1]] + "[]"
    if kind in REFS:
        return REFS[kind][3]
    return TS[kind]


def emit_ts(lane):
    imports = sorted({kind for _, kind in lane["fields"] if kind in REFS})
    lines = []
    for kind in imports:
        _, folder, parser, typename = REFS[kind]
        rel = os.path.relpath(os.path.join(ROOT, FW, folder, "🧬️schema", "🟦️.ts"), os.path.join(ROOT, lane["dir"])).replace(os.sep, "/")
        lines.append(f'import {{ {parser}, type {typename} }} from "{rel}";')
    lines += [
        "const exact = (value: unknown, at: string, keys: readonly string[]): Record<string, unknown> => {",
        '  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError(`${at} must be an object`);',
        "  const row = value as Record<string, unknown>;",
        "  for (const key of Object.keys(row)) if (!keys.includes(key)) throw new TypeError(`${at}.${key} is unknown`);",
        "  for (const key of keys) if (!(key in row)) throw new TypeError(`${at}.${key} is required`);",
        "  return row;",
        "};",
        'const text = (value: unknown, at: string): string => { if (typeof value !== "string") throw new TypeError(`${at} must be a string`); return value; };',
        'const num = (value: unknown, at: string): number => { if (typeof value !== "number" || !Number.isFinite(value)) throw new TypeError(`${at} must be a finite number`); return value; };',
        'const count = (value: unknown, at: string): number => { if (typeof value !== "number" || !Number.isSafeInteger(value) || value < 0) throw new TypeError(`${at} must be a non-negative safe integer`); return value; };',
        'const flag = (value: unknown, at: string): boolean => { if (typeof value !== "boolean") throw new TypeError(`${at} must be a boolean`); return value; };',
        "const list = <T,>(value: unknown, at: string, item: (entry: unknown, at: string) => T): T[] => { if (!Array.isArray(value)) throw new TypeError(`${at} must be an array`); return value.map((entry, index) => item(entry, `${at}[${index}]`)); };",
    ]
    for name, fields in lane["defs"].items():
        lines.append(f"/** 🧬️ {name}. */")
        lines.append(f"export interface {name} {{")
        lines += [f"  {field}: {ts_type(kind)};" for field, kind in fields]
        lines.append("}")
        keys = ", ".join(f'"{field}"' for field, _ in fields)
        lines.append(f"export function parse{name}(value: unknown, at = \"$\"): {name} {{")
        lines.append(f"  const row = exact(value, at, [{keys}]);")
        lines.append("  return {")
        lines += [f"    {field}: {ts_check(kind, f'row.{field}', f'`${{at}}.{field}`')}," for field, kind in fields]
        lines.append("  };")
        lines.append("}")
    name = lane["name"]
    lines.append(f"/** 🎚️ {lane['doc']} */")
    lines.append(f"export interface {name} {{")
    lines += [f"  {field}: {ts_type(kind)};" for field, kind in lane["fields"]]
    lines.append("}")
    if lane["name"] == "BimWindowTransient":
        lines.append(f"/** 🧬️ The one `set` mutation of {name}: every field. */")
        lines.append(f'export type {name}Mutation = {{ kind: "set" }} & {name};')
    else:
        lines.append(f"/** 🧬️ The one `snapshot` mutation of {name}: the whole configuration replaces the previous one. */")
        lines.append(f'export type {name}Mutation = {{ kind: "snapshot"; config: {name} }};')
    keys = ", ".join(f'"{field}"' for field, _ in lane["fields"])
    lines.append(f"/** 🚪️ Parses one exact {name}. */")
    lines.append(f"export function parse{name}(value: unknown): {name} {{")
    lines.append(f'  const row = exact(value, "$", [{keys}]);')
    lines.append("  return {")
    lines += [f"    {field}: {ts_check(kind, f'row.{field}', json.dumps(f'$.{field}'))}," for field, kind in lane["fields"]]
    lines.append("  };")
    lines.append("}")
    lines.append(f"/** 🔁️ Applies one exact {name} mutation. */")
    lines.append(f"export function apply{name}Mutation(_base: {name}, mutation: {name}Mutation): {name} {{")
    if lane["name"] == "BimWindowTransient":
        lines.append("  const { kind: _kind, ...fields } = mutation;")
        lines.append(f"  return parse{name}(fields);")
    else:
        lines.append(f"  return parse{name}(mutation.config);")
    lines.append("}")
    write(lane, "🟦️.ts", "\n".join(lines) + "\n")


def proto_type(kind, name):
    if kind.startswith("def:"):
        return kind[4:]
    if kind.startswith("["):
        return "repeated " + PROTO[kind[1:-1]]
    if kind == "Viewport2d":
        return "semio.framework.ui.viewport.Viewport2d"
    if kind == "Viewport3dOrbit":
        return "semio.framework.ui.viewport.Viewport3dOrbit"
    return PROTO[kind]


def emit_proto(lane):
    lines = ['syntax = "proto3";', f"package semio.app.bim.model.{lane['package']};"]
    for kind, folder in (("Viewport2d", "◻️2d"), ("Viewport3dOrbit", "🧊️3d")):
        if any(k == kind for _, k in lane["fields"]):
            lines.append(f'import "{FW}/{folder}/🧬️schema/🛰️.proto";')
    for name, fields in lane["defs"].items():
        lines.append(f"message {name} {{")
        lines += [f"  {proto_type(kind, field)} {snake(field)} = {index + 1};" for index, (field, kind) in enumerate(fields)]
        lines.append("}")
    lines.append(f"message {lane['name']} {{")
    lines += [f"  {proto_type(kind, field)} {snake(field)} = {index + 1};" for index, (field, kind) in enumerate(lane["fields"])]
    lines.append("}")
    write(lane, "🛰️.proto", "\n".join(lines) + "\n")


def gql_type(kind):
    if kind.startswith("def:"):
        return kind[4:] + "!"
    if kind.startswith("["):
        return "[" + GQL[kind[1:-1]] + "!]!"
    if kind in REFS:
        return REFS[kind][3] + "!"
    return GQL[kind] + "!"


def emit_graphql(lane):
    blocks = []
    for name, fields in lane["defs"].items():
        blocks.append("\n".join([f"type {name} {{"] + [f"  {field}: {gql_type(kind)}" for field, kind in fields] + ["}"]))
    blocks.append("\n".join([f"type {lane['name']} {{"] + [f"  {field}: {gql_type(kind)}" for field, kind in lane["fields"]] + ["}"]))
    write(lane, "🔗️.graphql", "\n\n".join(blocks) + "\n")


for lane in LANES:
    emit_json(lane)
    emit_ts(lane)
    emit_proto(lane)
    emit_graphql(lane)
    print("facets", lane["name"])
