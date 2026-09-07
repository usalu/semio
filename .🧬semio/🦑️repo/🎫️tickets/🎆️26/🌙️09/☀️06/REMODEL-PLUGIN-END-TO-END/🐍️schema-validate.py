#!/usr/bin/env python3
"""🧪️ Validates every committed remodeling fixture against the regenerated normative JSON Schema
leaves. Stdlib only (no `jsonschema` in this environment) — implements exactly the Draft 2020-12
subset the leaves use: type (string or list), enum, const, required, properties,
additionalProperties (false | schema), items, minItems/maxItems, minimum, anyOf, oneOf, $ref into
`#/$defs/`. `format`/`contentEncoding`/`description`/`title` are annotations and are not asserted.
"""
from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
SUBSET = ROOT / "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any"
SCHEMA = SUBSET / "🧬️schema"


def load(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def type_ok(t: str, value) -> bool:
    if t == "object":
        return isinstance(value, dict)
    if t == "array":
        return isinstance(value, list)
    if t == "string":
        return isinstance(value, str)
    if t == "integer":
        return isinstance(value, int) and not isinstance(value, bool)
    if t == "number":
        return isinstance(value, (int, float)) and not isinstance(value, bool)
    if t == "boolean":
        return isinstance(value, bool)
    if t == "null":
        return value is None
    return True


def validate(schema: dict, value, root: dict, path: str, errors: list[str]) -> None:
    if "$ref" in schema:
        name = schema["$ref"].split("/")[-1]
        validate(root["$defs"][name], value, root, path, errors)
        return
    if "const" in schema and value != schema["const"]:
        errors.append(f"{path}: expected const {schema['const']!r}, got {value!r}")
        return
    if "enum" in schema and value not in schema["enum"]:
        errors.append(f"{path}: {value!r} is not one of {schema['enum']}")
        return
    if "anyOf" in schema:
        if not any(not sub_errors(branch, value, root, path) for branch in schema["anyOf"]):
            errors.append(f"{path}: {shorten(value)} matches no anyOf branch")
        return
    if "oneOf" in schema:
        matches = [i for i, branch in enumerate(schema["oneOf"]) if not sub_errors(branch, value, root, path)]
        if len(matches) != 1:
            detail = "no branch" if not matches else f"{len(matches)} branches"
            errors.append(f"{path}: {shorten(value)} matches {detail} of oneOf")
        return
    declared = schema.get("type")
    if declared is not None:
        options = declared if isinstance(declared, list) else [declared]
        if not any(type_ok(t, value) for t in options):
            errors.append(f"{path}: expected {declared}, got {json_type(value)}")
            return
    if isinstance(value, dict) and "properties" in schema or (isinstance(value, dict) and "required" in schema):
        properties = schema.get("properties", {})
        for key in schema.get("required", []):
            if key not in value:
                errors.append(f"{path}/{key}: required but absent")
        for key, item in value.items():
            if key in properties:
                validate(properties[key], item, root, f"{path}/{key}", errors)
            elif schema.get("additionalProperties") is False:
                errors.append(f"{path}/{key}: not allowed (additionalProperties false)")
    elif isinstance(value, dict) and isinstance(schema.get("additionalProperties"), dict):
        for key, item in value.items():
            validate(schema["additionalProperties"], item, root, f"{path}/{key}", errors)
    if isinstance(value, list):
        if "minItems" in schema and len(value) < schema["minItems"]:
            errors.append(f"{path}: {len(value)} items < minItems {schema['minItems']}")
        if "maxItems" in schema and len(value) > schema["maxItems"]:
            errors.append(f"{path}: {len(value)} items > maxItems {schema['maxItems']}")
        if "items" in schema:
            for i, item in enumerate(value):
                validate(schema["items"], item, root, f"{path}/{i}", errors)
    if isinstance(value, (int, float)) and not isinstance(value, bool) and "minimum" in schema and value < schema["minimum"]:
        errors.append(f"{path}: {value} < minimum {schema['minimum']}")


def sub_errors(schema: dict, value, root: dict, path: str) -> list[str]:
    out: list[str] = []
    validate(schema, value, root, path, out)
    return out


def json_type(value) -> str:
    if value is None:
        return "null"
    if isinstance(value, bool):
        return "boolean"
    if isinstance(value, int):
        return "integer"
    if isinstance(value, float):
        return "number"
    if isinstance(value, str):
        return "string"
    if isinstance(value, list):
        return "array"
    return "object"


def shorten(value) -> str:
    text = json.dumps(value, ensure_ascii=False)
    return text if len(text) <= 60 else text[:57] + "..."


SNAPSHOT = load(SCHEMA / "📸️snapshot/🔣️.json")
DIFF = load(SCHEMA / "🔺️diff/🔣️.json")
MUTATION = load(SCHEMA / "🧬️mutations/🔣️.json")

targets: list[tuple[Path, dict, str]] = []
for case in sorted((SCHEMA / "🧬️mutations").glob("*/🧪️tests/*")):
    if not case.is_dir():
        continue
    targets.append((case / "📸️snapshot/⬅️before/🔣️.json", SNAPSHOT, "snapshot"))
    targets.append((case / "📸️snapshot/➡️after/🔣️.json", SNAPSHOT, "snapshot"))
    targets.append((case / "🦠️mutation/🔣️.json", MUTATION, "mutation"))
    # 🚫️ A refused vector commits `🔺️diff/🚫️.absent` instead of a delta — the repository-wide marker
    # for "this file is deliberately not here" (93 uses under ✏️s/), so its absence is not a gap.
    if not (case / "🔺️diff/🚫️.absent").exists():
        targets.append((case / "🔺️diff/🔣️.json", DIFF, "diff"))
for extra in sorted((SUBSET / "🧫️fixtures").rglob("*.json")):
    targets.append((extra, MUTATION if extra.name.startswith("🦠️") else SNAPSHOT, "mutation" if extra.name.startswith("🦠️") else "snapshot"))

total = 0
bad: dict[str, list[str]] = {}
missing: list[str] = []
for path, schema, label in targets:
    if not path.exists():
        missing.append(str(path.relative_to(ROOT)))
        continue
    total += 1
    errors: list[str] = []
    validate(schema, load(path), schema, "", errors)
    if errors:
        bad[str(path.relative_to(SUBSET))] = errors


# 📚️ The two example documents are DSL text, not JSON: assert their key vocabulary against the
# kebab-cased field names the regenerated snapshot schema declares, and their enum lexemes.
import re as _re


def kebab_of(camel_name: str) -> str:
    return _re.sub(r"(?<!^)(?=[A-Z0-9])", "-", camel_name).lower().replace("--", "-")


def schema_vocabulary(schema: dict) -> tuple[set[str], set[str]]:
    keys: set[str] = set(schema.get("properties", {}))
    lexemes: set[str] = set()
    for spec in schema.get("$defs", {}).values():
        keys |= set(spec.get("properties", {}))
        lexemes |= set(spec.get("enum", []))
    return {kebab_of(k) for k in keys}, lexemes


VOCAB, LEXEMES = schema_vocabulary(SNAPSHOT)
DSL_KEY = _re.compile(r"([a-z][a-z0-9_-]*)\s*(?:=|:[A-Z]+)")
example_report: list[str] = []
for example in sorted((SUBSET / "📚️examples").glob("*/🖼️assets/🗣️.dsl.semio")):
    text = example.read_text(encoding="utf-8")
    keys = set(DSL_KEY.findall(text))
    unknown = sorted(k for k in keys if k not in VOCAB and k not in {"target", "child-id", "schema", "id"})
    absent = sorted(k for k in ("durable-artifacts",) if k not in keys)
    stages = set(_re.findall(r"stage=([a-z-]+)", text))
    bad_stage = sorted(s for s in stages if s not in LEXEMES)
    example_report.append(
        f"  {example.relative_to(SUBSET)}: unknown keys {unknown or 'none'}; absent {absent or 'none'}; "
        f"invalid stage lexemes {bad_stage or 'none'}"
    )


# 🦀️ Runtime gate: the framework compiles every normative leaf with `OwnedJsonSchemaValidator`
# (`🧰️framework/🔨️modules/🧬️schema/✅️validator.rs::validate_schema_node`), which HARD-ERRORS on any
# keyword outside this allowlist and on a `$ref` that does not resolve locally.
RUST_KEYWORDS = {
    "$id", "$schema", "$anchor", "$comment", "title", "description", "format", "$ref", "$defs",
    "definitions", "properties", "type", "required", "additionalProperties", "items", "not", "enum",
    "const", "default", "examples", "allOf", "anyOf", "oneOf", "minItems", "maxItems", "minLength",
    "maxLength", "uniqueItems", "readOnly", "writeOnly", "deprecated", "minimum", "maximum",
    "exclusiveMinimum", "exclusiveMaximum", "multipleOf",
}


def runtime_problems(node, root: dict, path: str = "$") -> list[str]:
    if not isinstance(node, dict):
        return []
    out: list[str] = []
    for key, child in node.items():
        if key not in RUST_KEYWORDS and not key.startswith("x-"):
            out.append(f"{path}: unsupported JSON Schema keyword `{key}`")
        if key == "$ref" and child.split("/")[-1] not in root.get("$defs", {}):
            out.append(f"{path}: $ref {child} does not resolve")
        if key in ("$defs", "definitions", "properties") and isinstance(child, dict):
            for name, sub in child.items():
                out += runtime_problems(sub, root, f"{path}.{key}.{name}")
        elif key in ("additionalProperties", "items", "not") and isinstance(child, dict):
            out += runtime_problems(child, root, f"{path}.{key}")
        elif key in ("allOf", "anyOf", "oneOf") and isinstance(child, list):
            for i, sub in enumerate(child):
                out += runtime_problems(sub, root, f"{path}.{key}[{i}]")
    return out


runtime_report: list[str] = []
for leaf in [SCHEMA / "🔣️.json", SCHEMA / "📸️snapshot/🔣️.json", SCHEMA / "🔺️diff/🔣️.json",
             SCHEMA / "🧬️mutations/🔣️.json", SCHEMA / "💡️inferences/🔣️.json"]:
    doc = load(leaf)
    for problem in runtime_problems(doc, doc):
        runtime_report.append(f"{leaf.relative_to(SUBSET)}: {problem}")

# 🧷️ Per-mutation payload leaves: each committed 🦠️mutation/🔣️.json minus its `mutation` tag must
# validate against its own owner-relative 🧬️.schema.json, and that leaf must satisfy the repo's
# draft-07 payload contract (`mutationPayloadSchemaDocumentProblems`, ported for the used keywords).
DRAFT07 = "http://json-schema.org/draft-07/schema#"
JSON_TYPES = {"array", "boolean", "integer", "null", "number", "object", "string"}


def payload_contract_problems(doc, path: str = "") -> list[str]:
    out: list[str] = []
    if path == "" and doc.get("$schema") != DRAFT07:
        out.append("must declare the registered draft-07 dialect")
    if not isinstance(doc, dict):
        return out
    for key, child in doc.items():
        at = f"{path}/{key}"
        if key in ("$id", "$schema", "$ref", "title", "description", "format", "contentEncoding") and not isinstance(child, str):
            out.append(f"{at} must be a string")
        if key in ("minItems", "maxItems", "minLength", "maxLength") and not (isinstance(child, int) and child >= 0):
            out.append(f"{at} must be a nonnegative integer")
        if key == "required" and (not isinstance(child, list) or len(set(child)) != len(child)):
            out.append(f"{at} must contain unique strings")
        if key == "enum" and (not isinstance(child, list) or not child or len(set(map(json.dumps, child))) != len(child)):
            out.append(f"{at} must contain unique values")
        if key == "type":
            types = child if isinstance(child, list) else [child]
            if not types or set(types) - JSON_TYPES:
                out.append(f"{at} must name unique JSON types")
        if key in ("anyOf", "allOf", "oneOf"):
            if not isinstance(child, list) or not child:
                out.append(f"{at} must contain schemas")
            else:
                for i, sub in enumerate(child):
                    out += payload_contract_problems(sub, f"{at}/{i}")
        elif key in ("items", "additionalProperties", "contains") and isinstance(child, dict):
            out += payload_contract_problems(child, at)
        elif key == "properties" and isinstance(child, dict):
            for name, sub in child.items():
                out += payload_contract_problems(sub, f"{at}/{name}")
    return out


payload_problems: list[str] = []
payload_checked = 0
for case in sorted((SCHEMA / "🧬️mutations").glob("*/🧪️tests/*")):
    leaf = case.parent.parent / "🧬️.schema.json"
    fixture = case / "🦠️mutation/🔣️.json"
    if not leaf.exists() or not fixture.exists():
        continue
    payload_schema = load(leaf)
    payload = {k: v for k, v in load(fixture).items() if k != "mutation"}
    errs: list[str] = []
    validate(payload_schema, payload, payload_schema, "", errs)
    payload_checked += 1
    for e in errs:
        payload_problems.append(f"{leaf.relative_to(SUBSET)} vs {case.name}: {e}")
contract_problems: list[str] = []
for leaf in sorted((SCHEMA / "🧬️mutations").glob("*/🧬️.schema.json")):
    for problem in payload_contract_problems(load(leaf)):
        contract_problems.append(f"{leaf.relative_to(SUBSET)}: {problem}")

print(f"normative leaves vs the Rust OwnedJsonSchemaValidator keyword allowlist: {len(runtime_report) or 'no'} problems")
for line in runtime_report[:10]:
    print("   ", line)
print(f"validated {total} committed fixture documents against the regenerated leaves")
print(f"validated {payload_checked} committed payloads against their per-mutation 🧬️.schema.json: "
      f"{len(payload_problems) or 'no'} violations")
for line in payload_problems[:10]:
    print("   ", line)
print(f"per-mutation payload leaves vs the repo draft-07 payload contract: {len(contract_problems) or 'no'} problems")
for line in contract_problems[:10]:
    print("   ", line)
print("\nexample DSL documents:")
for line in example_report:
    print(line)
if missing:
    print(f"\nabsent ({len(missing)}):")
    for m in missing:
        print("  -", m)
buckets: dict[str, list[str]] = {}
for file, errors in bad.items():
    for e in errors:
        buckets.setdefault(e.split(": ", 1)[1] if ": " in e else e, []).append(f"{file}{e.split(':')[0]}")
print(f"\n{len(bad)} of {total} documents violate the schema; {sum(len(v) for v in bad.values())} violations")
for message, files in sorted(buckets.items(), key=lambda kv: -len(kv[1])):
    print(f"\n  [{len(files)}x] {message}")
    for f in files[:3]:
        print("      e.g.", f)
sys.exit(0)
