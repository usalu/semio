#!/usr/bin/env python3
"""🪞️ Projects a scope's JSON Schema exports into the other formats its module provides.

Execution contract §B: every export must exist in every format the scope provides. The normative
JSON Schema already states the contract, so the GraphQL type, the protobuf message, the TypeScript
type and its `parse<Export>()` entry point are all *derivable* from it — and a derived projection is
the only kind that cannot drift from the contract it projects.

What it does NOT do: invent a Rust type. A Rust entity that exists elsewhere in the plugin crate is
re-exported (`pub use`, execution contract §A / ledger row 111); one that exists nowhere is reported,
never fabricated, because the Rust file is the crate's real code and not a projection.

Usage:
  python3 wp4c-project-exports.py report|apply [--under ✏️s/🔌️plugins/✒️writer] [--formats a,b]
"""
from __future__ import annotations
import json, os, re, sys, collections

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), *[".."] * 7))
LEAF = {"🦀️rust": "🦀️.rs", "🟦️typescript": "🟦️.ts", "🔗️graphql": "🔗️.graphql", "🔣️jsonschema": "🔣️.json", "🛰️protobuf": "🛰️.proto"}
FACET_DIRS = ("🔺️diff", "📸️snapshot", "💡️inferences", "🧬️mutations", "📝️text", "💾️binary")
EXPORT_RE = re.compile(r"^[A-Z][A-Za-z0-9]*$")
GRAPHQL_KEYWORDS = "type|input|enum|interface|union|scalar"
IDENTIFIER_RE = re.compile(r"^[A-Za-z_][A-Za-z0-9_]*$")
PARSER_REGION = "//#region 🚪️Parsers"


def snake(name: str) -> str:
    return re.sub(r"(?<!^)(?=[A-Z])", "_", name).lower()


def declares(fmt: str, source: str, name: str) -> bool:
    escaped = re.escape(name)
    if fmt == "🛰️protobuf":
        return re.search(rf"^\s*message\s+{escaped}\b", source, re.M) is not None
    if fmt == "🔗️graphql":
        return re.search(rf"^\s*(?:{GRAPHQL_KEYWORDS})\s+{escaped}\b", source, re.M) is not None
    if fmt == "🦀️rust":
        return re.search(rf"^\s*pub\s+(?:struct|enum|type)\s+{escaped}\b", source, re.M) is not None
    return re.search(rf"^\s*export\s+(?:interface|type|const|class)\s+{escaped}\b", source, re.M) is not None


def declares_parser(source: str, name: str) -> bool:
    return re.search(rf"^\s*export\s+(?:(?:async\s+)?function|const|let|declare\s+function)\s+parse{re.escape(name)}\b", source, re.M) is not None


#region 🔗️GraphQL
def graphql_type(node: dict, root_defs: dict) -> str | None:
    if "$ref" in node:
        target = node["$ref"].rsplit("/", 1)[-1]
        return target if EXPORT_RE.match(target) else None
    kind = node.get("type")
    if kind == "string":
        return "String"
    if kind == "boolean":
        return "Boolean"
    if kind == "integer":
        return "Int"
    if kind == "number":
        return "Float"
    if kind == "array":
        inner = graphql_type(node.get("items") or {}, root_defs)
        return f"[{inner}!]" if inner else None
    return None


def render_graphql(name: str, definition: dict, root_defs: dict) -> str | None:
    if isinstance(definition.get("enum"), list) and all(isinstance(entry, str) for entry in definition["enum"]):
        if all(IDENTIFIER_RE.match(entry) for entry in definition["enum"]):
            values = "\n".join(f"  {entry}" for entry in definition["enum"])
            return f"enum {name} {{\n{values}\n}}\n"
        return f"# 🔤️ {name} is a string domain whose members are not GraphQL names; the JSON Schema enumerates them.\nscalar {name}\n"
    if definition.get("type") == "object" and isinstance(definition.get("properties"), dict) and definition["properties"]:
        required = set(definition.get("required") or [])
        lines = []
        for field, shape in definition["properties"].items():
            rendered = graphql_type(shape if isinstance(shape, dict) else {}, root_defs)
            if rendered is None:
                return None
            lines.append(f"  {field}: {rendered}{'!' if field in required else ''}")
        body = "\n".join(lines)
        return f"type {name} {{\n{body}\n}}\n"
    if definition.get("type") in ("string", "integer", "number", "boolean") or "oneOf" in definition or "anyOf" in definition:
        return f"scalar {name}\n"
    return None
#endregion 🔗️GraphQL


#region 🛰️Protobuf
def proto_type(node: dict) -> tuple[str, bool] | None:
    """(proto type, repeated)"""
    if "$ref" in node:
        target = node["$ref"].rsplit("/", 1)[-1]
        return (target, False) if EXPORT_RE.match(target) else None
    kind = node.get("type")
    declared = node.get("format")
    if kind == "string":
        return ("string", False)
    if kind == "boolean":
        return ("bool", False)
    if kind == "integer":
        return (declared if declared in ("uint32", "uint64", "int32", "int64") else "int64", False)
    if kind == "number":
        return (declared if declared in ("double", "float") else "double", False)
    if kind == "array":
        inner = proto_type(node.get("items") or {})
        return (inner[0], True) if inner and not inner[1] else None
    return None


def render_proto(name: str, definition: dict) -> str | None:
    if definition.get("type") == "object" and isinstance(definition.get("properties"), dict) and definition["properties"]:
        required = set(definition.get("required") or [])
        lines = []
        for index, (field, shape) in enumerate(definition["properties"].items(), start=1):
            rendered = proto_type(shape if isinstance(shape, dict) else {})
            if rendered is None:
                return None
            kind, repeated = rendered
            prefix = "repeated " if repeated else ("" if field in required or kind not in ("string", "bool", "double", "float", "int32", "int64", "uint32", "uint64") and False else "")
            if not repeated and field not in required:
                prefix = "optional "
            lines.append(f"  {prefix}{kind} {snake(field)} = {index};")
        body = "\n".join(lines)
        return f"message {name} {{\n{body}\n}}\n"
    scalar = proto_type(definition)
    if scalar is not None and not scalar[1]:
        return f"message {name} {{\n  {scalar[0]} value = 1;\n}}\n"
    if isinstance(definition.get("enum"), list) and all(isinstance(entry, str) for entry in definition["enum"]):
        return f"message {name} {{\n  string value = 1;\n}}\n"
    return None
#endregion 🛰️Protobuf


#region 🟦️TypeScript
def ts_type(node: dict) -> str | None:
    if "$ref" in node:
        target = node["$ref"].rsplit("/", 1)[-1]
        return target if EXPORT_RE.match(target) else None
    if isinstance(node.get("enum"), list) and all(isinstance(entry, str) for entry in node["enum"]):
        return " | ".join(json.dumps(entry, ensure_ascii=False) for entry in node["enum"])
    if "const" in node and isinstance(node["const"], (str, int, float, bool)):
        return json.dumps(node["const"], ensure_ascii=False)
    kind = node.get("type")
    if kind == "string":
        return "string"
    if kind == "boolean":
        return "boolean"
    if kind in ("integer", "number"):
        return "number"
    if kind == "array":
        inner = ts_type(node.get("items") or {})
        return f"readonly {inner}[]" if inner else None
    if kind == "object" and not node.get("properties"):
        return "Readonly<Record<string, unknown>>"
    return None


def render_ts_type(name: str, definition: dict) -> str | None:
    if definition.get("type") == "object" and isinstance(definition.get("properties"), dict) and definition["properties"]:
        required = set(definition.get("required") or [])
        lines = []
        for field, shape in definition["properties"].items():
            rendered = ts_type(shape if isinstance(shape, dict) else {})
            if rendered is None:
                return None
            lines.append(f"  readonly {field}{'' if field in required else '?'}: {rendered};")
        body = "\n".join(lines)
        return f"export interface {name} {{\n{body}\n}}\n"
    inline = ts_type(definition)
    return f"export type {name} = {inline};\n" if inline else None


def bounds(node: dict, keys: tuple[str, ...]) -> str:
    """The schema's own bounds, passed to the guard so the parser refuses what the schema refuses."""
    stated = {key: node[key] for key in keys if key in node and isinstance(node[key], (int, float, str)) and not isinstance(node[key], bool)}
    return f", {json.dumps(stated, ensure_ascii=False)}" if stated else ""


def parser_expression(node: dict, access: str, path: str, helper: str) -> str | None:
    """`path` is the INNER text of a template literal, e.g. `${at}.edits`, so an array element can
    extend it positionally without nesting one template literal inside another."""
    at = f"`{path}`"
    if "$ref" in node:
        target = node["$ref"].rsplit("/", 1)[-1]
        return f"parse{target}({access}, {at})" if EXPORT_RE.match(target) else None
    if isinstance(node.get("enum"), list) and all(isinstance(entry, str) for entry in node["enum"]):
        members = ", ".join(json.dumps(entry, ensure_ascii=False) for entry in node["enum"])
        return f"{helper}Member({access}, {at}, [{members}] as const)"
    if "const" in node and isinstance(node["const"], (str, int, float, bool)):
        return f"{helper}Constant({access}, {at}, {json.dumps(node['const'], ensure_ascii=False)})"
    kind = node.get("type")
    if kind == "string":
        return f"{helper}String({access}, {at}{bounds(node, ('minLength', 'maxLength', 'pattern'))})"
    if kind == "boolean":
        return f"{helper}Boolean({access}, {at})"
    if kind == "integer":
        return f"{helper}Integer({access}, {at}{bounds(node, ('minimum', 'maximum'))})"
    if kind == "number":
        return f"{helper}Number({access}, {at}{bounds(node, ('minimum', 'maximum'))})"
    if kind == "array":
        inner = node.get("items") or {}
        element = parser_expression(inner if isinstance(inner, dict) else {}, "item", f"{path}[${{index}}]", helper)
        if element is None:
            return None
        return f"{helper}Array({access}, {at}{bounds(node, ('minItems', 'maxItems'))}).map((item, index) => {element})"
    if kind == "object" and not node.get("properties"):
        return f"{helper}Object({access}, {at})"
    return None


def render_parser(name: str, definition: dict, helper: str, declared: dict[str, bool] | None = None) -> str | None:
    if definition.get("type") == "object" and isinstance(definition.get("properties"), dict) and definition["properties"]:
        required = set(definition.get("required") or [])
        if declared:
            required |= {field for field, optional in declared.items() if not optional}
        lines = []
        for field, shape in definition["properties"].items():
            access = f"row[{json.dumps(field)}]"
            expression = parser_expression(shape if isinstance(shape, dict) else {}, access, f"${{at}}.{field}", helper)
            if expression is None:
                return None
            if field in required:
                lines.append(f"    {field}: {expression},")
            else:
                lines.append(f"    {field}: {access} === undefined ? undefined : {expression},")
        body = "\n".join(lines)
        return f'export function parse{name}(value: unknown, at = "$"): {name} {{\n  const row = {helper}Object(value, at);\n  return {{\n{body}\n  }};\n}}\n'
    expression = parser_expression(definition, "value", "${at}", helper)
    return f'export function parse{name}(value: unknown, at = "$"): {name} {{\n  return {expression};\n}}\n' if expression else None


def helpers_block(helper: str) -> str:
    return f"""{PARSER_REGION}
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class {helper}Refusal extends Error {{
  constructor(readonly at: string, readonly why: string) {{
    super(`${{at}}: ${{why}}`);
  }}
}}

const {helper}Reject = (at: string, why: string): never => {{
  throw new {helper}Refusal(at, why);
}};

type {helper}TextBounds = {{ readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string }};
type {helper}RangeBounds = {{ readonly minimum?: number; readonly maximum?: number }};
type {helper}SizeBounds = {{ readonly minItems?: number; readonly maxItems?: number }};

export const {helper}Object = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : {helper}Reject(at, "value is not an object");
export const {helper}Array = (value: unknown, at: string, bounds: {helper}SizeBounds = {{}}): readonly unknown[] => {{
  if (!Array.isArray(value)) return {helper}Reject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) {helper}Reject(at, `array has fewer than ${{bounds.minItems}} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) {helper}Reject(at, `array has more than ${{bounds.maxItems}} items`);
  return value;
}};
export const {helper}String = (value: unknown, at: string, bounds: {helper}TextBounds = {{}}): string => {{
  if (typeof value !== "string") return {helper}Reject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) {helper}Reject(at, `string is shorter than ${{bounds.minLength}}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) {helper}Reject(at, `string is longer than ${{bounds.maxLength}}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) {helper}Reject(at, `string does not match ${{bounds.pattern}}`);
  return value;
}};
export const {helper}Boolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : {helper}Reject(at, "value is not a boolean"));
export const {helper}Number = (value: unknown, at: string, bounds: {helper}RangeBounds = {{}}): number => {{
  if (typeof value !== "number" || !Number.isFinite(value)) return {helper}Reject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) {helper}Reject(at, `number is below ${{bounds.minimum}}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) {helper}Reject(at, `number is above ${{bounds.maximum}}`);
  return value;
}};
export const {helper}Integer = (value: unknown, at: string, bounds: {helper}RangeBounds = {{}}): number =>
  Number.isSafeInteger(value) ? {helper}Number(value, at, bounds) : {helper}Reject(at, "value is not an integer");
export const {helper}Member = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : {helper}Reject(at, `value is not one of ${{members.join(", ")}}`);
export const {helper}Constant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : {helper}Reject(at, `value is not ${{String(expected)}}`);
//#endregion 🚪️Parsers
"""
#endregion 🟦️TypeScript


def modules(under: str) -> list[str]:
    found = []
    for base, dirs, _ in os.walk(os.path.join(REPO, under)):
        dirs[:] = [name for name in dirs if name not in ("node_modules", ".venv")]
        if os.path.basename(base) != "🧬️schema":
            continue
        module = os.path.relpath(base, REPO)
        if "/🧬️mutations/" in f"{module}/" or not os.path.exists(os.path.join(base, LEAF["🔣️jsonschema"])):
            continue
        found.append(module)
    return sorted(found)


def documents(module: str) -> list[str]:
    found = [f"{module}/{LEAF['🔣️jsonschema']}"]
    for base, dirs, files in os.walk(os.path.join(REPO, module)):
        dirs[:] = [name for name in dirs if name in FACET_DIRS]
        relative = os.path.relpath(base, os.path.join(REPO, module))
        if relative != "." and LEAF["🔣️jsonschema"] in files:
            found.append(f"{module}/{relative}/{LEAF['🔣️jsonschema']}")
    return found


def exports_of(document: dict) -> dict[str, dict]:
    found: dict[str, dict] = {}
    root = document.get("title")
    root_object = document.get("type") == "object" or ("type" not in document and isinstance(document.get("properties"), dict))
    if isinstance(root, str) and root_object and EXPORT_RE.match(root):
        found[root] = document
    for name, definition in (document.get("$defs") or {}).items():
        if EXPORT_RE.match(name) and name not in found:
            found[name] = definition if isinstance(definition, dict) else {}
    return found


def helper_prefix(document_id: str | None) -> str:
    """`https://semio.tech/schema/s/writer/writer/diff.json` → `sWriterWriterDiffGuard`.

    The scope's own `$id` names the helpers, so two schema modules compiled into one TypeScript program
    never collide and the name says which contract the guard belongs to.
    """
    if not isinstance(document_id, str) or "/schema/" not in document_id:
        return "schemaGuard"
    tail = document_id.split("/schema/", 1)[1].removesuffix(".json")
    parts = [re.sub(r"[^A-Za-z0-9]", "", part) for part in tail.split("/") if part]
    if len(parts) > 2 and parts[0] in ("s", "app"):
        parts = parts[1:]
    if len(parts) > 1 and parts[-1] == "schema":
        parts = parts[:-1]
    if not parts:
        return "schemaGuard"
    head = parts[0][:1].lower() + parts[0][1:]
    return head + "".join(part[:1].upper() + part[1:] for part in parts[1:]) + "Guard"


def declared_optional_fields(source: str, name: str) -> dict[str, bool] | None:
    """Which fields an already-written `export interface <name>` marks optional.

    A generated parser must satisfy the type that is already on disk: where the hand-written interface
    requires a field the JSON Schema forgot to list in `required`, the parser follows the interface and
    the mismatch is reported rather than compiled into a type error.
    """
    match = re.search(rf"^export interface {re.escape(name)}\s*(?:extends [^{{]+)?{{\n(.*?)^}}", source, re.M | re.S)
    if match is None:
        return None
    found: dict[str, bool] = {}
    for line in match.group(1).split("\n"):
        field = re.match(r"\s*(?:readonly\s+)?([A-Za-z_$][A-Za-z0-9_$]*)(\??):", line)
        if field:
            found[field.group(1)] = field.group(2) == "?"
    return found


def main() -> None:
    mode = sys.argv[1] if len(sys.argv) > 1 else "report"
    under = sys.argv[sys.argv.index("--under") + 1] if "--under" in sys.argv else "✏️s"
    wanted = set((sys.argv[sys.argv.index("--formats") + 1]).split(",")) if "--formats" in sys.argv else {"🔗️graphql", "🛰️protobuf", "🟦️typescript"}
    added = collections.Counter()
    refused: list[str] = []
    rust_absent: list[str] = []
    touched: set[str] = set()
    for module in modules(under):
        provided = {fmt: leaf for fmt, leaf in LEAF.items() if os.path.exists(os.path.join(REPO, module, leaf))}
        for path in documents(module):
            try:
                with open(os.path.join(REPO, path), encoding="utf-8") as handle:
                    document = json.load(handle)
            except Exception:  # noqa: BLE001
                continue
            facet = os.path.dirname(path[len(module) + 1 :])
            exports = exports_of(document)
            if not exports:
                continue
            for fmt in provided:
                if fmt == "🔣️jsonschema":
                    continue
                target = os.path.join(REPO, module, facet, LEAF[fmt]) if facet else os.path.join(REPO, module, LEAF[fmt])
                if not os.path.exists(target):
                    continue
                relative = os.path.relpath(target, REPO)
                with open(target, encoding="utf-8") as handle:
                    source = handle.read()
                appended: list[str] = []
                helper = helper_prefix(document.get("$id"))
                for name, definition in exports.items():
                    restricted = definition.get("x-semio-formats")
                    if isinstance(restricted, list) and fmt not in restricted:
                        continue
                    present = declares(fmt, source, name)
                    if not present and fmt in wanted:
                        rendered = {"🔗️graphql": lambda: render_graphql(name, definition, document.get("$defs") or {}), "🛰️protobuf": lambda: render_proto(name, definition), "🟦️typescript": lambda: render_ts_type(name, definition)}[fmt]()
                        if rendered is None:
                            refused.append(f"{fmt} {name} {relative}")
                            continue
                        appended.append(rendered)
                        added[fmt] += 1
                        present = True
                    elif not present and fmt == "🦀️rust":
                        rust_absent.append(f"{name} {relative}")
                        continue
                    if fmt == "🟦️typescript" and present and not declares_parser(source + "\n".join(appended), name):
                        rendered = render_parser(name, definition, helper, declared_optional_fields(source, name))
                        if rendered is None:
                            refused.append(f"parser {name} {relative}")
                            continue
                        # 🚪️A parser may only call entry points this file will actually have: a `$ref` that
                        # crosses into another document needs that document's `parse<Export>` imported,
                        # which is the TypeScript half of ledger row 111 and a per-owner decision.
                        wanted_parsers = set(re.findall(r"\bparse([A-Z][A-Za-z0-9]*)\(", rendered)) - {name}
                        unresolved = [target for target in wanted_parsers if not declares_parser(source, target) and target not in exports]
                        if unresolved:
                            refused.append(f"parser {name} {relative} needs parse{', parse'.join(sorted(unresolved))}")
                            continue
                        appended.append(rendered)
                        added["parser"] += 1
                if not appended or mode != "apply":
                    continue
                block = "\n".join(appended)
                if fmt == "🟦️typescript" and PARSER_REGION not in source:
                    block = f"{helpers_block(helper)}\n{block}"
                with open(target, "w", encoding="utf-8") as handle:
                    handle.write(f"{source.rstrip()}\n\n{block.rstrip()}\n")
                touched.add(relative)
    print(json.dumps({"mode": mode, "under": under, "added": dict(added), "filesTouched": len(touched), "refused": len(refused), "rustAbsent": len(rust_absent)}, ensure_ascii=False))
    for row in refused[:30]:
        print("REFUSED", row)
    for row in rust_absent[:30]:
        print("RUST-ABSENT", row)


if __name__ == "__main__":
    main()
