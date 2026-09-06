#!/usr/bin/env python3
"""🧬️ W8 authoring aid for the remodeling artifact's normative schema-description leaves.

The repo has NO generator for these files: `🔣️.json` is the hand-authored normative leaf of a
schema facet (`schemaFacetKinds.🧬️data.normativeFormat` in `🔣️taxonomy.json`) and rust/ts/graphql/
protobuf mirror it (`policyArtifactSchemaFieldParityBreaches` in the root `📜️script.ts`). This
script exists only so the 35-payload / 40-record remodeling model can be transcribed into the
JSON Schema, GraphQL and protobuf leaves without hand-typing errors; it is a ticket-local
authoring tool, not repo infrastructure, and it is never wired into any target.

It reads the Rust records (currently the only complete statement of the model) and writes the
JSON Schema / GraphQL / protobuf leaves. TypeScript leaves are NOT written (W3 owns them).
"""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
SUBSET = ROOT / "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any"
SCHEMA = SUBSET / "🧬️schema"
ARTIFACT_RS = ROOT / "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🦀️.rs"

STRUCT_RE = re.compile(r"^((?:#\[[^\n]*\]\n)*)pub struct (\w+)\s*\{(.*?)^\}", re.S | re.M)
ENUM_RE = re.compile(r"^((?:#\[[^\n]*\]\n)*)pub enum (\w+)\s*\{(.*?)^\}", re.S | re.M)


def camel(name: str) -> str:
    head, *rest = name.split("_")
    return head + "".join(p[:1].upper() + p[1:] for p in rest)


def kebab(variant: str) -> str:
    out = []
    for i, ch in enumerate(variant):
        if ch.isupper() and i:
            out.append("-")
        out.append(ch.lower())
    return "".join(out)


def lower_camel(pascal: str) -> str:
    """serde `rename_all = "camelCase"` on the RemodelingMutation variant names — the JSON wire tag."""
    return pascal[:1].lower() + pascal[1:]


def screaming(name: str) -> str:
    out = []
    for i, ch in enumerate(name):
        if ch.isupper() and i and not name[i - 1].isupper():
            out.append("_")
        out.append(ch.upper())
    return "".join(out)


class Registry:
    def __init__(self) -> None:
        self.structs: dict[str, list[tuple[str, str]]] = {}
        self.enums: dict[str, list[str]] = {}

    def load(self, path: Path) -> None:
        text = path.read_text(encoding="utf-8")
        for attrs, name, body in STRUCT_RE.findall(text):
            fields: list[tuple[str, str]] = []
            for line in body.split("\n"):
                s = line.strip()
                if not s or s.startswith("//") or s.startswith("#["):
                    continue
                m = re.match(r"pub (\w+):\s*(.+?),\s*$", s)
                if m:
                    fields.append((m.group(1), m.group(2)))
            if fields:
                self.structs.setdefault(name, fields)
        for attrs, name, body in ENUM_RE.findall(text):
            if "DslScalar" not in attrs:
                continue
            variants = []
            for line in body.split("\n"):
                s = line.strip()
                if not s or s.startswith("//") or s.startswith("#["):
                    continue
                m = re.match(r"(\w+),\s*$", s)
                if m:
                    variants.append(m.group(1))
            if variants:
                self.enums.setdefault(name, variants)


REG = Registry()
REG.load(ARTIFACT_RS)
REG.load(SCHEMA / "🦀️.rs")
REG.load(SCHEMA / "📸️snapshot/🦀️.rs")
REG.load(SCHEMA / "🔺️diff/🦀️.rs")
REG.load(SCHEMA / "💡️inferences/🦀️.rs")
REG.load(SCHEMA / "💡️inferences/📦bounds/🦀️.rs")
REG.load(SCHEMA / "💡️inferences/🔄relative-pose/🦀️.rs")
for d in sorted((SCHEMA / "🧬️mutations").iterdir()):
    if (d / "🦀️.rs").exists():
        REG.load(d / "🦀️.rs")

# 🧩️ Handles onto other artifacts: `store::ArtifactChild<S>` serialises exactly childId + target.
CHILD_ALIASES = {"RemodelingAssetChild": "SemioImageSnapshot", "RemodelingMeshChild": "SemioMeshSnapshot"}
HANDCRAFTED = {
    "ArtifactDialect": {
        "type": "object",
        "title": "ArtifactDialect",
        "additionalProperties": False,
        "required": ["artifactKind", "standard", "subset"],
        "properties": {
            "artifactKind": {"type": "string"},
            "standard": {"type": "string"},
            "subset": {"type": "string"},
        },
    },
    "ArtifactRef": {
        "type": "object",
        "title": "ArtifactRef",
        "additionalProperties": False,
        "required": ["artifactId", "dialect"],
        "properties": {"artifactId": {"type": "string"}, "dialect": {"$ref": "#/$defs/ArtifactDialect"}},
    },
}
for alias, snapshot in CHILD_ALIASES.items():
    HANDCRAFTED[alias] = {
        "type": "object",
        "title": alias,
        "description": f"store::ArtifactChild<{snapshot}> — an owned child handle; only childId and target cross the wire.",
        "additionalProperties": False,
        "required": ["childId", "target"],
        "properties": {"childId": {"type": "string"}, "target": {"$ref": "#/$defs/ArtifactRef"}},
    }

SCALARS = {
    "String": {"type": "string"},
    "bool": {"type": "boolean"},
    "u8": {"type": "integer", "format": "uint8", "minimum": 0},
    "u32": {"type": "integer", "format": "uint32", "minimum": 0},
    "u64": {"type": "integer", "format": "uint64", "minimum": 0},
    "usize": {"type": "integer", "format": "uint64", "minimum": 0},
    "i32": {"type": "integer", "format": "int32"},
    "i64": {"type": "integer", "format": "int64"},
    "f32": {"type": "number", "format": "float"},
    "f64": {"type": "number", "format": "double"},
    "PackedF32": {"type": "string", "format": "base64", "description": "base64 of little-endian f32 lanes (semio_s_plugin_remodel PackedF32)"},
    "PackedU8": {"type": "string", "format": "base64", "description": "base64 of raw u8 lanes (semio_s_plugin_remodel PackedU8)"},
}


def strip_generic(t: str, wrapper: str) -> str | None:
    if t.startswith(wrapper + "<") and t.endswith(">"):
        return t[len(wrapper) + 1 : -1].strip()
    return None


def nullable(schema: dict) -> dict:
    if "$ref" in schema:
        return {"anyOf": [schema, {"type": "null"}]}
    out = dict(schema)
    t = out.get("type")
    if isinstance(t, str):
        out["type"] = [t, "null"]
        return out
    return {"anyOf": [schema, {"type": "null"}]}


def jtype(rust: str, used: set[str]) -> dict:
    rust = rust.strip()
    inner = strip_generic(rust, "Box")
    if inner:
        return jtype(inner, used)
    inner = strip_generic(rust, "Option")
    if inner:
        return nullable(jtype(inner, used))
    inner = strip_generic(rust, "Vec")
    if inner:
        return {"type": "array", "items": jtype(inner, used)}
    m = re.match(r"BTreeMap<\s*String\s*,\s*(.+)>$", rust)
    if m:
        return {"type": "object", "additionalProperties": jtype(m.group(1), used)}
    m = re.match(r"\[(\w+);\s*(\d+)\]$", rust)
    if m:
        return {"type": "array", "items": jtype(m.group(1), used), "minItems": int(m.group(2)), "maxItems": int(m.group(2))}
    if rust == "RemodelingDurableArtifactStore":
        used.add("RemodelingDurableArtifact")
        return {"type": "object", "additionalProperties": {"$ref": "#/$defs/RemodelingDurableArtifact"}}
    if rust in SCALARS:
        return dict(SCALARS[rust])
    if rust in HANDCRAFTED or rust in REG.structs or rust in REG.enums:
        used.add(rust)
        return {"$ref": f"#/$defs/{rust}"}
    raise SystemExit(f"unmapped rust type {rust!r}")


def def_for(name: str, used: set[str]) -> dict:
    if name in HANDCRAFTED:
        out = json.loads(json.dumps(HANDCRAFTED[name]))
        for prop in out["properties"].values():
            if "$ref" in prop:
                used.add(prop["$ref"].split("/")[-1])
        return out
    if name in REG.enums:
        return {"type": "string", "title": name, "enum": [kebab(v) for v in REG.enums[name]]}
    fields = REG.structs[name]
    props = {camel(f): jtype(t, used) for f, t in fields}
    return {
        "type": "object",
        "title": name,
        "additionalProperties": False,
        "required": [camel(f) for f, _ in fields],
        "properties": props,
    }


def closure(seed: set[str]) -> dict:
    defs: dict[str, dict] = {}
    pending = set(seed)
    while pending:
        name = pending.pop()
        if name in defs:
            continue
        used: set[str] = set()
        defs[name] = def_for(name, used)
        pending |= used - set(defs)
    return {k: defs[k] for k in sorted(defs)}


STATE_RE = re.compile(r"#\[state\((\w+)\)\]")


def state_fields(path: Path, struct: str) -> list[tuple[str, str, str]]:
    text = path.read_text(encoding="utf-8")
    m = re.search(rf"pub struct {struct}\s*\{{(.*?)^\}}", text, re.S | re.M)
    body = m.group(1)
    out: list[tuple[str, str, str]] = []
    state = ""
    for line in body.split("\n"):
        s = line.strip()
        sm = STATE_RE.match(s)
        if sm:
            state = sm.group(1)
            continue
        fm = re.match(r"pub (\w+):\s*(.+?),\s*$", s)
        if fm:
            out.append((fm.group(1), fm.group(2), state))
            state = ""
    return out


# ─────────────────────────────── JSON Schema leaves ───────────────────────────────
def record_schema(id_: str, title: str, fields: list[tuple[str, str, str]], extra_defs: set[str]) -> dict:
    used: set[str] = set(extra_defs)
    props = {}
    for name, ty, state in fields:
        prop = jtype(ty, used)
        if state == "derived":
            prop["x-semio-derived"] = True
        elif state:
            prop["x-semio-state"] = state
        props[camel(name)] = prop
    return {
        "$id": id_,
        "title": title,
        "type": "object",
        "additionalProperties": False,
        "required": [camel(n) for n, _, _ in fields],
        "properties": props,
        "$defs": closure(used),
    }


BASE = "https://semio.tech/schema/s/remodeling/remodeling"

artifact_fields = state_fields(SCHEMA / "🦀️.rs", "RemodelingArtifact")
snapshot_fields = state_fields(SCHEMA / "📸️snapshot/🦀️.rs", "RemodelingSnapshot")
diff_fields = state_fields(SCHEMA / "🔺️diff/🦀️.rs", "RemodelingDiff")

artifact_json = record_schema(f"{BASE}/artifact.json", "RemodelingArtifact", artifact_fields, set())
snapshot_json = record_schema(f"{BASE}/snapshot.json", "RemodelingSnapshot", snapshot_fields, set())
diff_json = record_schema(f"{BASE}/diff.json", "RemodelingDiff", diff_fields, {"RemodelingArtifact"})
diff_json["required"] = []
# 🔺️ Every diff lane is sparse: `#[serde(default)]` on the record, `Option<_>` on every field.
for prop in diff_json["properties"].values():
    pass
diff_json["$defs"]["RemodelingArtifact"] = {
    "type": "object",
    "title": "RemodelingArtifact",
    "additionalProperties": False,
    "required": artifact_json["required"],
    "properties": {k: {kk: vv for kk, vv in v.items() if kk != "x-semio-state"} for k, v in artifact_json["properties"].items()},
}
extra: set[str] = set()
for _, ty, _ in artifact_fields:
    jtype(ty, extra)
diff_json["$defs"].update({k: v for k, v in closure(extra).items() if k not in diff_json["$defs"]})
diff_json["$defs"] = {k: diff_json["$defs"][k] for k in sorted(diff_json["$defs"])}

# ─────────────────────────────── mutation union ───────────────────────────────
MUT_ORDER: list[tuple[str, str]] = []
enum_text = (SCHEMA / "🧬️mutations/🦀️.rs").read_text(encoding="utf-8")
enum_body = re.search(r"pub enum RemodelingMutation\s*\{(.*?)^\}", enum_text, re.S | re.M).group(1)
for line in enum_body.split("\n"):
    m = re.match(r"\s*(\w+)\((\w+)\),\s*$", line)
    if m:
        MUT_ORDER.append((m.group(1), m.group(2)))

KEYWORDS: dict[str, str] = {}
for d in sorted((SCHEMA / "🧬️mutations").iterdir()):
    rs = d / "🦀️.rs"
    if not rs.exists():
        continue
    text = rs.read_text(encoding="utf-8")
    for attrs, name, _ in STRUCT_RE.findall(text):
        kw = re.search(r'dsl\(keyword = "([a-z0-9-]+)"\)', attrs)
        if kw:
            KEYWORDS[name] = kw.group(1)

mut_used: set[str] = set()
mut_defs: dict[str, dict] = {}
for variant, payload in MUT_ORDER:
    fields = REG.structs[payload]
    props = {"mutation": {"const": lower_camel(variant)}}
    for f, t in fields:
        props[camel(f)] = jtype(t, mut_used)
    mut_defs[variant] = {
        "type": "object",
        "title": payload,
        "additionalProperties": False,
        "required": ["mutation"] + [camel(f) for f, _ in fields],
        "properties": props,
    }
mutation_json = {
    "$id": f"{BASE}/mutation.json",
    "title": "RemodelingMutation",
    "description": "One handcrafted semantic mutation kind per $defs entry - the tagged union RemodelingMutation dispatches on.",
    "oneOf": [{"$ref": f"#/$defs/{variant}"} for variant, _ in MUT_ORDER],
    "$defs": {**mut_defs, **closure(mut_used)},
}

inference_fields = [(n, t, "derived") for n, t, _ in state_fields(SCHEMA / "💡️inferences/🦀️.rs", "RemodelingInference")]
inference_json = record_schema(f"{BASE}/inference.json", "RemodelingInference", inference_fields, set())

# ─────────────────────────────── GraphQL ───────────────────────────────
GQL_SCALAR = {
    "String": "String",
    "bool": "Boolean",
    "u8": "Int",
    "u32": "Int",
    "u64": "Int",
    "usize": "Int",
    "i32": "Int",
    "i64": "Int",
    "f32": "Float",
    "f64": "Float",
    "PackedF32": "String",
    "PackedU8": "String",
}
MAP_ENTRIES: dict[str, str] = {}


def gql(rust: str, used: set[str], required: bool = True) -> str:
    rust = rust.strip()
    inner = strip_generic(rust, "Box")
    if inner:
        return gql(inner, used, required)
    inner = strip_generic(rust, "Option")
    if inner:
        return gql(inner, used, False)
    bang = "!" if required else ""
    inner = strip_generic(rust, "Vec")
    if inner:
        return f"[{gql(inner, used)}]{bang}"
    m = re.match(r"BTreeMap<\s*String\s*,\s*(.+)>$", rust)
    if m:
        value = m.group(1).strip()
        entry = f"{value}Entry"
        MAP_ENTRIES[entry] = value
        used.add(value)
        return f"[{entry}!]{bang}"
    m = re.match(r"\[(\w+);\s*(\d+)\]$", rust)
    if m:
        return f"[{gql(m.group(1), used)}]{bang}"
    if rust == "RemodelingDurableArtifactStore":
        MAP_ENTRIES["RemodelingDurableArtifactEntry"] = "RemodelingDurableArtifact"
        used.add("RemodelingDurableArtifact")
        return f"[RemodelingDurableArtifactEntry!]{bang}"
    if rust in GQL_SCALAR:
        return GQL_SCALAR[rust] + bang
    used.add(rust)
    return rust + bang


def gql_defs(names: set[str]) -> list[str]:
    blocks: list[str] = []
    emitted: set[str] = set()
    pending = set(names)
    while pending:
        name = min(pending)
        pending.discard(name)
        if name in emitted:
            continue
        emitted.add(name)
        if name in REG.enums:
            blocks.append(f"enum {name} {{\n" + "".join(f"  {v}\n" for v in REG.enums[name]) + "}")
            continue
        if name in HANDCRAFTED:
            lines = []
            for prop, spec in HANDCRAFTED[name]["properties"].items():
                if "$ref" in spec:
                    ref = spec["$ref"].split("/")[-1]
                    pending.add(ref)
                    lines.append(f"  {prop}: {ref}!")
                else:
                    lines.append(f"  {prop}: String!")
            blocks.append(f"type {name} {{\n" + "\n".join(lines) + "\n}")
            continue
        used: set[str] = set()
        lines = [f"  {camel(f)}: {gql(t, used)}" for f, t in REG.structs[name]]
        pending |= used - emitted
        blocks.append(f"type {name} {{\n" + "\n".join(lines) + "\n}")
    for entry in sorted(MAP_ENTRIES):
        if entry in emitted:
            continue
        emitted.add(entry)
        value = MAP_ENTRIES[entry]
        vt = GQL_SCALAR.get(value, value)
        blocks.append(f"type {entry} {{\n  key: String!\n  value: {vt}!\n}}")
    return sorted(blocks, key=lambda b: b.split("\n")[0].split(" ")[1])


def gql_record(header: str, title: str, fields: list[tuple[str, str, str]]) -> str:
    MAP_ENTRIES.clear()
    used: set[str] = set()
    lines = []
    for name, ty, state in fields:
        suffix = " @derived" if state == "derived" else (f" @state(class: {state.upper()})" if state else "")
        lines.append(f"  {camel(name)}: {gql(ty, used)}{suffix}")
    body = f"type {title} {{\n" + "\n".join(lines) + "\n}"
    return header + "\n\n" + body + "\n\n" + "\n\n".join(gql_defs(used)) + "\n"


# ─────────────────────────────── protobuf ───────────────────────────────
PROTO_SCALAR = {
    "String": "string",
    "bool": "bool",
    "u8": "uint32",
    "u32": "uint32",
    "u64": "uint64",
    "usize": "uint64",
    "i32": "int32",
    "i64": "int64",
    "f32": "float",
    "f64": "double",
    "PackedF32": "string",
    "PackedU8": "string",
}


def proto(rust: str, used: set[str]) -> tuple[str, str]:
    """Returns (label, type) where label is "", "optional " or "repeated "."""
    rust = rust.strip()
    inner = strip_generic(rust, "Box")
    if inner:
        return proto(inner, used)
    inner = strip_generic(rust, "Option")
    if inner:
        label, ty = proto(inner, used)
        return ("repeated " if label == "repeated " else "optional ", ty)
    inner = strip_generic(rust, "Vec")
    if inner:
        _, ty = proto(inner, used)
        return "repeated ", ty
    m = re.match(r"BTreeMap<\s*String\s*,\s*(.+)>$", rust)
    if m:
        _, ty = proto(m.group(1), used)
        return "", f"map<string, {ty}>"
    m = re.match(r"\[(\w+);\s*(\d+)\]$", rust)
    if m:
        _, ty = proto(m.group(1), used)
        return "repeated ", ty
    if rust == "RemodelingDurableArtifactStore":
        used.add("RemodelingDurableArtifact")
        return "", "map<string, RemodelingDurableArtifact>"
    if rust in PROTO_SCALAR:
        return "", PROTO_SCALAR[rust]
    used.add(rust)
    return "", rust


def proto_defs(names: set[str]) -> list[str]:
    blocks: list[str] = []
    emitted: set[str] = set()
    pending = set(names)
    while pending:
        name = min(pending)
        pending.discard(name)
        if name in emitted:
            continue
        emitted.add(name)
        if name in REG.enums:
            prefix = screaming(name)
            lines = [f"  {prefix}_{screaming(v)} = {i};" for i, v in enumerate(REG.enums[name])]
            blocks.append(f"enum {name} {{\n" + "\n".join(lines) + "\n}")
            continue
        if name in HANDCRAFTED:
            lines = []
            for i, (prop, spec) in enumerate(HANDCRAFTED[name]["properties"].items(), start=1):
                if "$ref" in spec:
                    ref = spec["$ref"].split("/")[-1]
                    pending.add(ref)
                    lines.append(f"  {ref} {snake(prop)} = {i};")
                else:
                    lines.append(f"  string {snake(prop)} = {i};")
            blocks.append(f"message {name} {{\n" + "\n".join(lines) + "\n}")
            continue
        used: set[str] = set()
        lines = []
        for i, (f, t) in enumerate(REG.structs[name], start=1):
            label, ty = proto(t, used)
            if ty.startswith("map<"):
                label = ""
            lines.append(f"  {label}{ty} {f} = {i};")
        pending |= used - emitted
        blocks.append(f"message {name} {{\n" + "\n".join(lines) + "\n}")
    return sorted(blocks, key=lambda b: b.split("\n")[0].split(" ")[1])


def snake(name: str) -> str:
    return re.sub(r"(?<!^)(?=[A-Z])", "_", name).lower()


def proto_record(header: str, package: str, title: str, fields: list[tuple[str, str, str]]) -> str:
    used: set[str] = set()
    lines = []
    for i, (name, ty, state) in enumerate(fields, start=1):
        label, pt = proto(ty, used)
        if pt.startswith("map<"):
            label = ""
        if state:
            lines.append("  // @derived" if state == "derived" else f"  // @state {state}")
        lines.append(f"  {label}{pt} {name} = {i};")
    body = f"message {title} {{\n" + "\n".join(lines) + "\n}"
    return (
        f"syntax = \"proto3\";\npackage {package};\n\n{header}\n\n" + body + "\n\n" + "\n\n".join(proto_defs(used)) + "\n"
    )


# ─────────────────────────────── write ───────────────────────────────
def write_json(path: Path, doc: dict) -> None:
    path.write_text(json.dumps(doc, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print("wrote", path.relative_to(ROOT))


def write_text(path: Path, text: str) -> None:
    path.write_text(text, encoding="utf-8")
    print("wrote", path.relative_to(ROOT))


write_json(SCHEMA / "🔣️.json", artifact_json)
write_json(SCHEMA / "📸️snapshot/🔣️.json", snapshot_json)
write_json(SCHEMA / "🔺️diff/🔣️.json", diff_json)
write_json(SCHEMA / "🧬️mutations/🔣️.json", mutation_json)
write_json(SCHEMA / "💡️inferences/🔣️.json", inference_json)

write_text(
    SCHEMA / "🔗️.graphql",
    gql_record("# 🧬️ Remodeling artifact schema — every field with its state class.", "RemodelingArtifact", artifact_fields),
)
write_text(
    SCHEMA / "📸️snapshot/🔗️.graphql",
    gql_record("# 📸️ Remodeling snapshot schema — exactly the artifact-lane fields.", "RemodelingSnapshot", snapshot_fields),
)
write_text(
    SCHEMA / "🔺️diff/🔗️.graphql",
    gql_record("# 🔺️ Remodeling diff schema — one sparse lane per non-transient artifact field.", "RemodelingDiff", diff_fields),
)
write_text(
    SCHEMA / "💡️inferences/🔗️.graphql",
    gql_record("# 💡️ Remodeling inference schema — derived, never persisted.", "RemodelingInference", inference_fields),
)

write_text(
    SCHEMA / "🛰️.proto",
    proto_record("// 🧬️ Remodeling artifact schema.", "semio.s.remodeling.remodeling.artifact", "RemodelingArtifact", artifact_fields),
)
write_text(
    SCHEMA / "📸️snapshot/🛰️.proto",
    proto_record("// 📸️ Remodeling snapshot schema.", "semio.s.remodeling.remodeling.snapshot", "RemodelingSnapshot", snapshot_fields),
)
write_text(
    SCHEMA / "🔺️diff/🛰️.proto",
    proto_record("// 🔺️ Remodeling diff schema.", "semio.s.remodeling.remodeling.diff", "RemodelingDiff", diff_fields),
)
write_text(
    SCHEMA / "💡️inferences/🛰️.proto",
    proto_record("// 💡️ Remodeling inference schema.", "semio.s.remodeling.remodeling.inference", "RemodelingInference", inference_fields),
)

# 🧬️ Mutation union GraphQL/protobuf: one type/message per payload plus the shared record closure.
MAP_ENTRIES.clear()
mut_gql_used: set[str] = set()
mut_blocks = []
for variant, payload in MUT_ORDER:
    lines = [f"  mutation: String!"]
    for f, t in REG.structs[payload]:
        lines.append(f"  {camel(f)}: {gql(t, mut_gql_used)}")
    mut_blocks.append(f"type {payload} {{\n" + "\n".join(lines) + "\n}")
union = "union RemodelingMutation =\n" + "\n".join(
    ("    " if i == 0 else "  | ") + payload for i, (_, payload) in enumerate(MUT_ORDER)
)
write_text(
    SCHEMA / "🧬️mutations/🔗️.graphql",
    "# 🧬️ Remodeling mutation vocabulary — one type per semantic kind, tagged by `mutation`.\n\n"
    + union
    + "\n\n"
    + "\n\n".join(mut_blocks)
    + "\n\n"
    + "\n\n".join(gql_defs(mut_gql_used))
    + "\n",
)

mut_proto_used: set[str] = set()
mut_msgs = []
for variant, payload in MUT_ORDER:
    lines = [f"  string mutation = 1;"]
    for i, (f, t) in enumerate(REG.structs[payload], start=2):
        label, pt = proto(t, mut_proto_used)
        if pt.startswith("map<"):
            label = ""
        lines.append(f"  {label}{pt} {f} = {i};")
    mut_msgs.append(f"message {payload} {{\n" + "\n".join(lines) + "\n}")
oneof = "\n".join(f"    {payload} {snake(payload)} = {i};" for i, (_, payload) in enumerate(MUT_ORDER, start=1))
write_text(
    SCHEMA / "🧬️mutations/🛰️.proto",
    "syntax = \"proto3\";\npackage semio.s.remodeling.remodeling.mutation;\n\n"
    "// 🧬️ Remodeling mutation vocabulary — one message per semantic kind.\n\n"
    "message RemodelingMutation {\n  oneof kind {\n" + oneof + "\n  }\n}\n\n"
    + "\n\n".join(mut_msgs)
    + "\n\n"
    + "\n\n".join(proto_defs(mut_proto_used))
    + "\n",
)

# 🧬️ Per-mutation payload schemas (`🧬️.schema.json`), Draft-07, self-contained (no $defs).
def inline(rust: str) -> dict:
    used: set[str] = set()
    spec = jtype(rust, used)
    return expand(spec)


def expand(spec: dict) -> dict:
    if "$ref" in spec:
        name = spec["$ref"].split("/")[-1]
        inner: set[str] = set()
        return expand(def_for(name, inner))
    out = {}
    for key, value in spec.items():
        if key in ("items", "additionalProperties") and isinstance(value, dict):
            out[key] = expand(value)
        elif key == "properties":
            out[key] = {k: expand(v) for k, v in value.items()}
        elif key == "anyOf":
            out[key] = [expand(v) for v in value]
        else:
            out[key] = value
    return out


for d in sorted((SCHEMA / "🧬️mutations").iterdir()):
    rs = d / "🦀️.rs"
    if not rs.exists():
        continue
    text = rs.read_text(encoding="utf-8")
    payload = None
    for attrs, name, _ in STRUCT_RE.findall(text):
        if re.search(r'dsl\(keyword = "', attrs):
            payload = name
    if payload is None:
        continue
    fields = REG.structs[payload]
    doc = {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": payload,
        "type": "object",
        "additionalProperties": False,
        "required": sorted(camel(f) for f, _ in fields),
        "properties": {camel(f): inline(t) for f, t in fields},
    }
    write_json(d / "🧬️.schema.json", doc)

print("done")

# ─────────────────────── 📖️ mutation op grammar (the dangling root leaf) ───────────────────────
# `🧬️mutations/📖️.grammar.semio` is NOT include_str!'d anywhere (only `📝️text/📖️.grammar.semio` is,
# as the registered LanguageSpec.grammar), and it still carried a foreign mesh-editing vocabulary
# (add-vertex / set-face / transform-mesh / merge-solid). Rewritten here from the real 35 payloads,
# in the same `dialect grammar` shape 🧱️block's own mutations root leaf uses.
DSL_ATTR = re.compile(r"#\[dsl\(([^)]*)\)\]")


def dsl_markers(path: Path, struct: str) -> dict[str, str]:
    text = path.read_text(encoding="utf-8")
    m = re.search(rf"pub struct {struct}\s*\{{(.*?)^\}}", text, re.S | re.M)
    if not m:
        return {}
    markers: dict[str, str] = {}
    pending = ""
    for line in m.group(1).split("\n"):
        s = line.strip()
        am = DSL_ATTR.match(s)
        if am:
            pending = am.group(1).split("=")[0].strip()
            continue
        fm = re.match(r"pub (\w+):", s)
        if fm:
            if pending:
                markers[fm.group(1)] = pending
            pending = ""
    return markers


def dsl_shape(rust: str, marker: str) -> str:
    rust = rust.strip()
    optional = False
    inner = strip_generic(rust, "Option")
    if inner:
        optional, rust = True, inner
    inner = strip_generic(rust, "Box")
    if inner:
        rust = inner
    if marker == "coord":
        shape = "CRD"
    elif marker == "unit":
        shape = "QTY"
    elif rust.startswith("Vec<"):
        shape = "TABLE"
    elif re.match(r"\[\w+;\s*\d+\]$", rust):
        shape = "TUPLE"
    elif rust in ("String", "PackedF32", "PackedU8"):
        shape = "TEXT"
    elif rust == "bool":
        shape = "BOOL"
    elif rust in ("f32", "f64"):
        shape = "NUM"
    elif rust in ("u8", "u32", "u64", "usize"):
        shape = "UINT"
    elif rust in ("i32", "i64"):
        shape = "INT"
    elif rust in REG.enums:
        shape = "ENUM"
    else:
        shape = "BLOCK"
    return shape + ("?" if optional else "")


PAYLOAD_DIR: dict[str, Path] = {}
for d in sorted((SCHEMA / "🧬️mutations").iterdir()):
    rs = d / "🦀️.rs"
    if not rs.exists():
        continue
    for attrs, name, _ in STRUCT_RE.findall(rs.read_text(encoding="utf-8")):
        if re.search(r'dsl\(keyword = "', attrs):
            PAYLOAD_DIR[name] = rs

op_lines: list[str] = []
op_rules: list[str] = []
for _variant, payload in MUT_ORDER:
    rs = PAYLOAD_DIR[payload]
    keyword = KEYWORDS[payload]
    markers = dsl_markers(rs, payload)
    rule = keyword + "-op"
    op_lines.append(rule)
    cells = " ".join(f'"{kebab(camel(f))}" "=" {dsl_shape(t, markers.get(f, ""))}' for f, t in REG.structs[payload])
    op_rules.append(f'{rule} = "{keyword}"' + (f" {cells}" if cells else ""))

grammar = [
    "dialect grammar",
    "grammar remodeling.remodeling.op",
    "extension remodeling",
    "use family-scene",
    "start mutation",
    "",
    "# 📖️ The op text form one `RemodelingMutation` prints as (protocol::OpText::print_op): the",
    "# variant keyword followed by that payload printed as one inline DSL record. One production per",
    "# semantic kind, in RemodelingMutation variant order; the shapes are the DSL cell classes",
    "# (TEXT/UINT/INT/NUM/QTY/BOOL/ENUM/CRD/TUPLE/BLOCK/TABLE), a trailing `?` marking an Option lane.",
    "",
    'artifact-mark = "remodeling.remodeling-op"',
    "mutation = artifact-mark op+",
    "op = " + "\n   | ".join(op_lines),
    "",
]
grammar += op_rules
write_text(SCHEMA / "🧬️mutations/📖️.grammar.semio", "\n".join(grammar) + "\n")
