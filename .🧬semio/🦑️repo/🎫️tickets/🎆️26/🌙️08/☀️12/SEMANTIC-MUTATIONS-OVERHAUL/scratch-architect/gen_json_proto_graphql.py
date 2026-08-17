#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import json, re

plan = json.load(open("/tmp/architect_plan_full.json", encoding="utf-8"))
DISPATCH = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"
BASE = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️架构师"
BASE = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"

dispatch_src = open(DISPATCH, encoding="utf-8").read()
enum_body_m = re.search(r"pub enum ProgramMutation \{(.*?)\n\}", dispatch_src, re.DOTALL)
variant_order = re.findall(r"^\s{4}([A-Za-z0-9]+)\(", enum_body_m.group(1), re.MULTILINE)
assert len(variant_order) == 266

by_struct = {p["struct_name"]: p for p in plan}

def fields_of(struct_body):
    return re.findall(r"pub\s+(\w+):\s*([^,\n]+),", struct_body)

def camel(name):
    return re.sub(r"_([a-z])", lambda m: m.group(1).upper(), name)

RUST_TO_JSON = {"String": "string", "bool": "boolean", "EntityId": "string"}
def json_type(ty):
    ty = ty.strip()
    if ty.startswith("Option<") and ty.endswith(">"):
        return json_type(ty[7:-1])
    if ty in RUST_TO_JSON:
        return RUST_TO_JSON[ty]
    if ty in ("f32", "f64", "u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64", "usize"):
        return "number"
    return "object"  # structured noun payload — opaque, mirrors the grammar's opaque block convention

# ---- JSON Schema ----
defs = {}
for name in variant_order:
    p = by_struct[name]
    fields = fields_of(p["struct_body"])
    props = {camel(fn): {"type": json_type(ft)} for fn, ft in fields}
    defs[name] = {
        "type": "object",
        "additionalProperties": False,
        "required": [camel(fn) for fn, _ in fields],
        "properties": props,
    }
json_schema = {
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "$id": "https://semio.tech/schema/s/architect/program/mutation.json",
    "title": "ProgramMutation",
    "description": "One real per-mutation record per SEMANTIC-MUTATIONS-OVERHAUL — 266 semantic kinds, matching the Rust dispatch enum 1:1 (Wave C rewrite; supersedes the pre-migration whole-snapshot-shaped generic schema).",
    "oneOf": [{"$ref": f"#/$defs/{name}"} for name in variant_order],
    "$defs": defs,
}
open(f"{BASE}/🔣️component.json", "w", encoding="utf-8").write(json.dumps(json_schema, indent=2, ensure_ascii=False) + "\n")
print("json written")

# ---- proto ----
RUST_TO_PROTO = {"String": "string", "bool": "bool", "EntityId": "string"}
def proto_type(ty):
    ty = ty.strip()
    if ty.startswith("Option<") and ty.endswith(">"):
        return proto_type(ty[7:-1])
    if ty in RUST_TO_PROTO:
        return RUST_TO_PROTO[ty]
    if ty in ("f32", "f64"):
        return "double"
    if ty in ("u8", "u16", "u32", "i8", "i16", "i32"):
        return "int32"
    if ty in ("u64", "i64", "usize"):
        return "int64"
    return "bytes"  # structured noun payload — opaque, mirrors the grammar's opaque block convention

lines = []
lines.append("syntax = \"proto3\";")
lines.append("package semio.s.architect.program.mutation;")
lines.append("")
lines.append("// 🧬️ Program mutation schema — one real message per semantic mutation kind (Wave C rewrite;")
lines.append("// supersedes the pre-migration whole-snapshot-shaped generic message).")
lines.append("message ProgramMutation {")
lines.append("  oneof mutation {")
for i, name in enumerate(variant_order, start=1):
    lines.append(f"    {name} {camel(name)[0].lower()}{camel(name)[1:]} = {i};")
lines.append("  }")
lines.append("}")
lines.append("")
for name in variant_order:
    p = by_struct[name]
    fields = fields_of(p["struct_body"])
    lines.append(f"message {name} {{")
    for i, (fn, ft) in enumerate(fields, start=1):
        lines.append(f"  {proto_type(ft)} {fn} = {i};")
    lines.append("}")
    lines.append("")
open(f"{BASE}/🛰️component.proto", "w", encoding="utf-8").write("\n".join(lines).rstrip() + "\n")
print("proto written")

# ---- graphql ----
RUST_TO_GQL = {"String": "String", "bool": "Boolean", "EntityId": "ID"}
def gql_type(ty, required=True):
    ty = ty.strip()
    if ty.startswith("Option<") and ty.endswith(">"):
        return gql_type(ty[7:-1], required=False)
    base = RUST_TO_GQL.get(ty)
    if base is None:
        base = "JSON"  # structured noun payload — opaque, mirrors the grammar's opaque block convention
    return f"{base}!" if required else base

glines = []
glines.append("# 🧬️ Program mutation schema — one real type per semantic mutation kind (Wave C rewrite;")
glines.append("# supersedes the pre-migration whole-snapshot-shaped generic type).")
glines.append("scalar JSON")
glines.append("")
glines.append("union ProgramMutation = " + " | ".join(variant_order))
glines.append("")
for name in variant_order:
    p = by_struct[name]
    fields = fields_of(p["struct_body"])
    glines.append(f"type {name} {{")
    for fn, ft in fields:
        glines.append(f"  {camel(fn)}: {gql_type(ft)}")
    glines.append("}")
    glines.append("")
open(f"{BASE}/🔗️component.graphql", "w", encoding="utf-8").write("\n".join(glines).rstrip() + "\n")
print("graphql written")
