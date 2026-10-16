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

`--closing` limits the TypeScript pass to the parsers that existing code already calls but never declared (and the
parsers and types those need), so a call can never dangle; without it every undeclared export is projected.
A parser is emitted only when every parser it calls is declared on disk or emitted in the same pass.
"""
from __future__ import annotations
import json, os, re, sys, collections
from urllib.parse import urlsplit

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
def item_schema(node: dict) -> dict:
    """The one element schema of an array; a positional (tuple) `items` list has none, so the caller refuses it."""
    inner = node.get("items")
    return inner if isinstance(inner, dict) else {}


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
        inner = graphql_type(item_schema(node), root_defs)
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
        inner = proto_type(item_schema(node))
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
def is_null(node: object) -> bool:
    return isinstance(node, dict) and node.get("type") == "null"


def nullable_of(node: dict) -> dict | None:
    """The single non-null alternative of `type: [T, "null"]` or `anyOf|oneOf: [T, {"type": "null"}]`, else None."""
    kinds = node.get("type")
    if isinstance(kinds, list) and "null" in kinds:
        rest = [kind for kind in kinds if kind != "null"]
        return {**{key: value for key, value in node.items() if key != "type"}, "type": rest[0]} if len(rest) == 1 else None
    for key in ("anyOf", "oneOf"):
        alternatives = node.get(key)
        if isinstance(alternatives, list) and len(alternatives) == 2 and sum(1 for alternative in alternatives if is_null(alternative)) == 1:
            other = next(alternative for alternative in alternatives if not is_null(alternative))
            return other if isinstance(other, dict) else None
    return None


def ts_type(node: dict) -> str | None:
    if "$ref" in node:
        target = node["$ref"].rsplit("/", 1)[-1]
        return target if EXPORT_RE.match(target) else None
    nullable = nullable_of(node)
    if nullable is not None:
        inner = ts_type(nullable)
        return f"{inner} | null" if inner else None
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
        inner = ts_type(item_schema(node))
        return (f"readonly ({inner})[]" if " | " in inner else f"readonly {inner}[]") if inner else None
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


class Owner:
    """One schema document's TypeScript leaf and the exports it states, addressed by the document's `$id`."""

    def __init__(self, ts: str, exports: set[str]) -> None:
        self.ts = ts
        self.exports = exports


class Context:
    """What one parser rendering needs from its surroundings and what it learned about its callees."""

    def __init__(self, helper: str, document_id: str | None, index: dict[str, Owner]) -> None:
        self.helper = helper
        self.document_id = document_id
        self.index = index
        self.local: set[str] = set()
        self.foreign: set[tuple[str, str]] = set()


def key_text(field: str) -> str:
    return field if IDENTIFIER_RE.match(field) else json.dumps(field, ensure_ascii=False)


def reference_call(reference: str, access: str, at: str, context: Context) -> str | None:
    """`parse<Export>(…)` for a `$ref`; a reference into another document records that document's parser as a dependency."""
    document, separator, fragment = reference.partition("#")
    target = fragment.rsplit("/", 1)[-1] if separator else ""
    if not EXPORT_RE.match(target):
        return None
    if document and document != context.document_id:
        owner = context.index.get(document)
        if owner is None or target not in owner.exports:
            return None
        context.foreign.add((owner.ts, target))
    else:
        context.local.add(target)
    return f"parse{target}({access}, {at})"


def object_fields(node: dict, row: str, path: str, context: Context, depth: int, declared: dict[str, bool] | None = None) -> list[str] | None:
    required = set(node.get("required") or [])
    if declared:
        required |= {field for field, optional in declared.items() if not optional}
    lines = []
    for field, shape in node["properties"].items():
        access = f"{row}[{json.dumps(field)}]"
        expression = parser_expression(shape if isinstance(shape, dict) else {}, access, f"{path}.{field}", context, depth)
        if expression is None:
            return None
        lines.append(f"{key_text(field)}: {expression}" if field in required else f"{key_text(field)}: {access} === undefined ? undefined : {expression}")
    return lines


def tagged_discriminator(alternatives: list) -> str | None:
    """The property every alternative pins to a distinct string `const`, which selects the alternative."""
    if not all(isinstance(alternative, dict) and isinstance(alternative.get("properties"), dict) for alternative in alternatives):
        return None
    for key in alternatives[0]["properties"]:
        tags = [alternative["properties"].get(key, {}).get("const") for alternative in alternatives]
        if all(isinstance(tag, str) for tag in tags) and len(set(tags)) == len(tags):
            return key
    return None


def union_expression(alternatives: list, access: str, path: str, context: Context, depth: int) -> str | None:
    helper = context.helper
    at = f"`{path}`"
    row = f"row{depth}"
    key = tagged_discriminator(alternatives)
    if key is not None:
        tags = [alternative["properties"][key]["const"] for alternative in alternatives]
        cases = []
        for alternative, tag in zip(alternatives, tags):
            parts = []
            for member in alternative.get("allOf") or []:
                call = parser_expression(member, row, path, context, depth + 1) if isinstance(member, dict) else None
                if call is None:
                    return None
                parts.append(f"...{call}")
            rest = {**alternative, "properties": {field: shape for field, shape in alternative["properties"].items() if field != key}}
            fields = object_fields(rest, row, path, context, depth + 1) if rest["properties"] else []
            if fields is None:
                return None
            cases.append(f"case {json.dumps(tag)}: return {{ {', '.join([f'{key_text(key)}: {json.dumps(tag)}', *parts, *fields])} }};")
        members = ", ".join(json.dumps(tag) for tag in tags)
        return f"(({row}) => {{ switch ({helper}Member({row}[{json.dumps(key)}], `{path}.{key}`, [{members}] as const)) {{ {' '.join(cases)} }} }})({helper}Object({access}, {at}))"
    if all(isinstance(alternative, dict) and alternative.get("type") == "object" and isinstance(alternative.get("properties"), dict) and len(alternative["properties"]) == 1 for alternative in alternatives):
        branches = []
        for alternative in alternatives:
            field, shape = next(iter(alternative["properties"].items()))
            expression = parser_expression(shape if isinstance(shape, dict) else {}, f"{row}[{json.dumps(field)}]", f"{path}.{field}", context, depth + 1)
            if expression is None:
                return None
            branches.append(f"if ({row}[{json.dumps(field)}] !== undefined) return {{ {key_text(field)}: {expression} }};")
        return f"(({row}) => {{ {' '.join(branches)} return {helper}Reject({at}, \"value matches no variant\"); }})({helper}Object({access}, {at}))"
    return None


def parser_expression(node: dict, access: str, path: str, context: Context, depth: int = 0) -> str | None:
    """`path` is the INNER text of a template literal, e.g. `${at}.edits`, so an array element can
    extend it positionally without nesting one template literal inside another. `depth` names the
    row, item and index bindings of nested levels so an inner level never shadows the path of an outer one."""
    helper = context.helper
    at = f"`{path}`"
    if "$ref" in node:
        return reference_call(node["$ref"], access, at, context)
    nullable = nullable_of(node)
    if nullable is not None:
        expression = parser_expression(nullable, access, path, context, depth)
        return None if expression is None else f"({access} === null ? null : {expression})"
    if isinstance(node.get("enum"), list) and all(isinstance(entry, str) for entry in node["enum"]):
        members = ", ".join(json.dumps(entry, ensure_ascii=False) for entry in node["enum"])
        return f"{helper}Member({access}, {at}, [{members}] as const)"
    if "const" in node and isinstance(node["const"], (str, int, float, bool)):
        return f"{helper}Constant({access}, {at}, {json.dumps(node['const'], ensure_ascii=False)})"
    for key in ("oneOf", "anyOf"):
        if isinstance(node.get(key), list):
            return union_expression(node[key], access, path, context, depth)
    if isinstance(node.get("allOf"), list) and len(node["allOf"]) == 1 and isinstance(node["allOf"][0], dict) and "properties" not in node:
        return parser_expression(node["allOf"][0], access, path, context, depth)
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
        items = node.get("items")
        limits = bounds(node, ("minItems", "maxItems"))
        if isinstance(items, list):
            names = f"items{depth}"
            elements = [parser_expression(member if isinstance(member, dict) else {}, f"{names}[{at_index}]", f"{path}[{at_index}]", context, depth + 1) for at_index, member in enumerate(items)]
            if any(element is None for element in elements):
                return None
            exact = f", {{\"minItems\": {len(items)}, \"maxItems\": {len(items)}}}"
            return f"(({names}) => [{', '.join(elements)}])({helper}Array({access}, {at}{exact}))"
        if isinstance(items, dict) and items:
            item, index = ("item", "index") if depth == 0 else (f"item{depth}", f"index{depth}")
            element = parser_expression(items, item, f"{path}[${{{index}}}]", context, depth + 1)
            if element is None:
                return None
            return f"{helper}Array({access}, {at}{limits}).map(({item}, {index}) => {element})"
        return f"[...{helper}Array({access}, {at}{limits})]"
    if kind == "object":
        if isinstance(node.get("properties"), dict) and node["properties"]:
            row = f"row{depth}"
            fields = object_fields(node, row, path, context, depth + 1)
            return None if fields is None else f"(({row}) => ({{ {', '.join(fields)} }}))({helper}Object({access}, {at}))"
        extra = node.get("additionalProperties")
        if isinstance(extra, dict) and extra:
            key, value = f"key{depth}", f"value{depth}"
            element = parser_expression(extra, value, f"{path}.${{{key}}}", context, depth + 1)
            return None if element is None else f"Object.fromEntries(Object.entries({helper}Object({access}, {at})).map(([{key}, {value}]) => [{key}, {element}]))"
        return f"{helper}Object({access}, {at})"
    return None


def render_parser(name: str, definition: dict, context: Context, declared: dict[str, bool] | None = None) -> str | None:
    if definition.get("type") == "object" and isinstance(definition.get("properties"), dict) and definition["properties"]:
        lines = object_fields(definition, "row", "${at}", context, 0, declared)
        if lines is None:
            return None
        body = "\n".join(f"    {line}," for line in lines)
        return f'export function parse{name}(value: unknown, at = "$"): {name} {{\n  const row = {context.helper}Object(value, at);\n  return {{\n{body}\n  }};\n}}\n'
    expression = parser_expression(definition, "value", "${at}", context, 0)
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
    """`https://json.schemas.assets.semio-tech.com/s/stdio/csv/rfc4180/any/diff.json` → `stdioCsvRfc4180AnyDiffGuard`.

    The document's own `$id` path names the helpers, so two schema modules compiled into one TypeScript program
    never collide and the name says which contract the guard belongs to.
    """
    if not isinstance(document_id, str):
        return "schemaGuard"
    path = urlsplit(document_id).path.removesuffix(".json")
    parts = [re.sub(r"[^A-Za-z0-9]", "", part) for part in path.split("/") if part]
    if len(parts) > 2 and parts[0] in ("s", "app"):
        parts = parts[1:]
    if len(parts) > 1 and parts[-1] == "schema":
        parts = parts[:-1]
    if not parts or not parts[0]:
        return "schemaGuard"
    head = parts[0][:1].lower() + parts[0][1:]
    return head + "".join(part[:1].upper() + part[1:] for part in parts[1:]) + "Guard"


def existing_helper(source: str) -> str | None:
    """The guard prefix a file already states (`export class <prefix>Refusal`), which new parsers must call."""
    for found in re.finditer(r"^export class (\w+)Refusal extends Error", source, re.M):
        if re.search(rf"^export const {re.escape(found.group(1))}Object\b", source, re.M):
            return found.group(1)
    return None


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


def defines_parser(source: str, name: str) -> bool:
    """A `parse<Name>` this file already has in scope: exported or private, written by hand or projected earlier."""
    return re.search(rf"^\s*(?:export\s+)?(?:(?:async\s+)?function|const|let)\s+parse{re.escape(name)}\b", source, re.M) is not None


def imports_parser(source: str, name: str) -> bool:
    return re.search(rf"import\s*(?:type\s*)?\{{[^}}]*\bparse{re.escape(name)}\b[^}}]*\}}\s*from", source) is not None


def build_index() -> dict[str, Owner]:
    """Every schema document under `✏️s`, by `$id`, so a `$ref` into another document finds that document's TypeScript leaf."""
    index: dict[str, Owner] = {}
    for module in modules("✏️s"):
        for path in documents(module):
            try:
                with open(os.path.join(REPO, path), encoding="utf-8") as handle:
                    document = json.load(handle)
            except Exception:  # noqa: BLE001
                continue
            identifier = document.get("$id")
            if not isinstance(identifier, str):
                continue
            facet = os.path.dirname(path[len(module) + 1 :])
            ts = os.path.join(REPO, module, facet, LEAF["🟦️typescript"]) if facet else os.path.join(REPO, module, LEAF["🟦️typescript"])
            index[identifier] = Owner(ts, set(exports_of(document)))
    return index


def read_text(path: str, cache: dict[str, str]) -> str:
    if path not in cache:
        try:
            with open(path, encoding="utf-8") as handle:
                cache[path] = handle.read()
        except OSError:
            cache[path] = ""
    return cache[path]


def import_lines(target: str, wanted: dict[str, set[str]]) -> str:
    lines = []
    for owner in sorted(wanted):
        relative = os.path.relpath(owner, os.path.dirname(target)).replace(os.sep, "/")
        relative = relative if relative.startswith(".") else f"./{relative}"
        lines.append(f"import {{ {', '.join(f'parse{name}' for name in sorted(wanted[owner]))} }} from {json.dumps(relative, ensure_ascii=False)};")
    return "\n".join(lines)


def with_imports(source: str, lines: str) -> str:
    if not lines:
        return source
    if source.startswith("/**"):
        end = source.find("*/")
        if end != -1:
            cut = source.find("\n", end) + 1
            return f"{source[:cut]}{lines}\n{source[cut:]}"
    return f"{lines}\n{source}"


def settle(plans: dict[str, dict], closing: bool, texts: dict[str, str], refused: list[str]) -> set[tuple[str, str]]:
    """Decide which parsers are emitted. A parser is emitted only when every parser it calls is declared on disk or emitted in the
    same pass, so a call can never dangle; `closing` narrows the set to the parsers that existing code already calls but never declared."""
    candidates = {(target, name): entry for target, plan in plans.items() for name, entry in plan["entries"].items() if entry["parser"] is not None and entry["typed"]}
    emit = set(candidates)
    if closing:
        needed: set[tuple[str, str]] = set()
        stack = []
        for target, plan in plans.items():
            source = read_text(target, texts)
            for name in set(re.findall(r"\bparse([A-Z][A-Za-z0-9]*)\(", source)):
                if name in plan["exports"] and not defines_parser(source, name) and not imports_parser(source, name):
                    stack.append((target, name))
        while stack:
            key = stack.pop()
            if key in needed:
                continue
            if key not in candidates:
                refused.append(f"dangling parse{key[1]} {os.path.relpath(key[0], REPO)} has no renderable parser")
                continue
            needed.add(key)
            entry = candidates[key]
            for name in entry["local"]:
                if not defines_parser(read_text(key[0], texts), name) and not imports_parser(read_text(key[0], texts), name):
                    stack.append((key[0], name))
            for owner, name in entry["foreign"]:
                if not defines_parser(read_text(owner, texts), name):
                    stack.append((owner, name))
        emit = needed

    def available(owner: str, name: str) -> bool:
        return (owner, name) in emit or defines_parser(read_text(owner, texts), name) or imports_parser(read_text(owner, texts), name)

    changed = True
    while changed:
        changed = False
        for key in sorted(emit):
            entry = candidates[key]
            missing = [f"parse{name}" for name in sorted(entry["local"]) if not available(key[0], name)] + [f"parse{name}" for owner, name in sorted(entry["foreign"]) if not available(owner, name)]
            if missing:
                refused.append(f"parser {key[1]} {os.path.relpath(key[0], REPO)} needs {', '.join(missing)}")
                emit.discard(key)
                changed = True
    return emit


def main() -> None:
    mode = sys.argv[1] if len(sys.argv) > 1 else "report"
    under = sys.argv[sys.argv.index("--under") + 1] if "--under" in sys.argv else "✏️s"
    wanted = set((sys.argv[sys.argv.index("--formats") + 1]).split(",")) if "--formats" in sys.argv else {"🔗️graphql", "🛰️protobuf", "🟦️typescript"}
    closing = "--closing" in sys.argv
    index = build_index()
    added = collections.Counter()
    refused: list[str] = []
    rust_absent: list[str] = []
    touched: set[str] = set()
    texts: dict[str, str] = {}
    plans: dict[str, dict] = {}
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
                source = read_text(target, texts)
                appended: list[str] = []
                helper = existing_helper(source) or helper_prefix(document.get("$id"))
                plan = plans.setdefault(target, {"entries": {}, "exports": set(exports), "helper": helper}) if fmt == "🟦️typescript" else None
                for name, definition in exports.items():
                    restricted = definition.get("x-semio-formats")
                    if isinstance(restricted, list) and fmt not in restricted:
                        continue
                    present = declares(fmt, source, name)
                    type_text = None
                    if not present and fmt in wanted and not (closing and fmt == "🟦️typescript"):
                        rendered = {"🔗️graphql": lambda: render_graphql(name, definition, document.get("$defs") or {}), "🛰️protobuf": lambda: render_proto(name, definition), "🟦️typescript": lambda: render_ts_type(name, definition)}[fmt]()
                        if rendered is None:
                            refused.append(f"{fmt} {name} {relative}")
                            continue
                        type_text = rendered
                        if fmt != "🟦️typescript":
                            appended.append(rendered)
                            added[fmt] += 1
                        present = True
                    elif not present and fmt == "🟦️typescript" and closing:
                        type_text = render_ts_type(name, definition)
                        present = type_text is not None
                    elif not present and fmt == "🦀️rust":
                        rust_absent.append(f"{name} {relative}")
                        continue
                    if fmt == "🟦️typescript" and not declares_parser(source, name):
                        context = Context(helper, document.get("$id"), index)
                        rendered = render_parser(name, definition, context, declared_optional_fields(source, name)) if present else None
                        if rendered is None and present:
                            refused.append(f"parser {name} {relative}")
                        plan["entries"][name] = {"type": type_text, "typed": present, "parser": rendered, "local": context.local - {name}, "foreign": set(context.foreign)}
                if fmt == "🟦️typescript" or not appended or mode != "apply":
                    continue
                block = "\n".join(appended)
                with open(target, "w", encoding="utf-8") as handle:
                    handle.write(f"{source.rstrip()}\n\n{block.rstrip()}\n")
                touched.add(relative)
    emit = settle(plans, closing, texts, refused)
    for target, plan in plans.items():
        keep = sorted(name for owner, name in emit if owner == target)
        if not keep:
            continue
        source = read_text(target, texts)
        blocks = []
        foreign: dict[str, set[str]] = collections.defaultdict(set)
        for name in keep:
            entry = plan["entries"][name]
            if entry["type"] is not None and not declares("🟦️typescript", source, name):
                blocks.append(entry["type"])
                added["🟦️typescript"] += 1
            blocks.append(entry["parser"])
            added["parser"] += 1
            for owner, callee in entry["foreign"]:
                if owner != target and not defines_parser(source, callee) and not imports_parser(source, callee):
                    foreign[owner].add(callee)
        relative = os.path.relpath(target, REPO)
        touched.add(relative)
        if mode != "apply":
            continue
        block = "\n".join(blocks)
        if PARSER_REGION not in source:
            block = f"{helpers_block(plan['helper'])}\n{block}"
        updated = with_imports(f"{source.rstrip()}\n\n{block.rstrip()}\n", import_lines(target, foreign))
        with open(target, "w", encoding="utf-8") as handle:
            handle.write(updated)
    print(json.dumps({"mode": mode, "under": under, "closing": closing, "added": dict(added), "filesTouched": len(touched), "refused": len(refused), "rustAbsent": len(rust_absent)}, ensure_ascii=False))
    for row in refused[:60]:
        print("REFUSED", row)
    for row in rust_absent[:30]:
        print("RUST-ABSENT", row)
    for relative in sorted(touched):
        print("TOUCHED", relative)


if __name__ == "__main__":
    main()
