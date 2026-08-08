#!/usr/bin/env python3
"""Generate artifact/snapshot/diff schema leaves and refactor norm en1990–en1994 artifacts."""

from __future__ import annotations

import json
import re
import shutil
from dataclasses import dataclass
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
NORM = ROOT / "✏️s/🔌️plugins/📕️norm/🗿️artifacts"

ARTIFACTS = [
    ("📘️en1990", "en1990", "En1990", "norm.en1990"),
    ("📘️en1991", "en1991", "En1991", "norm.en1991"),
    ("📘️en1992", "en1992", "En1992", "norm.en1992"),
    ("📘️en1993", "en1993", "En1993", "norm.en1993"),
    ("📘️en1994", "en1994", "En1994", "norm.en1994"),
]

SHARED_UI = ("selected_check_index", "selectedCheckIndex", "shared-ui", "Option<u32>", "u32_optional")


@dataclass
class Field:
    rust: str
    camel: str
    ty: str
    state: str = "persistent"


def snake_to_camel(s: str) -> str:
    parts = s.split("_")
    return parts[0] + "".join(p.capitalize() for p in parts[1:])


def parse_document_struct(path: Path) -> tuple[list[Field], str, list[str]]:
    text = path.read_text(encoding="utf-8")
    nested: list[str] = []
    for m in re.finditer(r"pub mod (\w+_\d+_\d+|\w+) \{", text):
        nested.append(m.group(1))
    m = re.search(r"pub struct Document \{([^}]+)\}", text, re.S)
    if not m:
        raise SystemExit(f"no Document struct in {path}")
    body = m.group(1)
    fields: list[Field] = []
    for line in body.splitlines():
        line = line.strip()
        if not line or line.startswith("#") or line.startswith("///"):
            continue
        line = re.sub(r"#\[.*?\]", "", line).strip()
        if not line or line.startswith("//"):
            continue
        mm = re.match(r"pub (\w+): (.+),?$", line)
        if not mm:
            continue
        rust, ty = mm.group(1), mm.group(2).rstrip(",")
        fields.append(Field(rust, snake_to_camel(rust), ty))
    return fields, text, nested


def json_type(field: Field, prefix: str) -> dict:
    ty = field.ty
    if ty == "f64":
        return {"type": "number", "format": "double"}
    if ty in ("u8", "u32", "i32", "u64"):
        return {"type": "integer", "format": "uint32" if "u" in ty else "int32"}
    if ty == "bool":
        return {"type": "boolean"}
    if ty == "String":
        return {"type": "string"}
    if "AnnexChoice" in ty:
        return {"type": "string"}
    if "ImposedCategory" in ty:
        return {"type": "string"}
    if "FireCurve" in ty:
        return {"type": "string"}
    if "FireRating" in ty:
        return {"type": "string"}
    if "TightnessClass" in ty:
        return {"type": "string"}
    if ty.startswith("Vec<QkEntry>"):
        return {"type": "array", "items": {"$ref": f"#/$defs/{prefix}QkEntry"}}
    if ty.startswith("Vec<"):
        return {"type": "array", "items": {"type": "string"}}
    return {"type": "string"}


def rust_schema_type(field: Field, prefix: str, snapshot: bool) -> str:
    ty = field.ty
    if ty == "f64":
        return "f64"
    if ty == "u8":
        return "u8"
    if ty == "u32":
        return "u32"
    if ty == "bool":
        return "bool"
    if ty == "String":
        return "String"
    if "AnnexChoice" in ty:
        return "crate::document::AnnexChoice"
    if "ImposedCategory" in ty:
        return "crate::document::ImposedCategory"
    if "part_1_2::FireCurve" in ty:
        return "crate::artifacts::en1991::part_1_2::FireCurve"
    if "part_1_2::FireRating" in ty:
        return "crate::artifacts::en1992::part_1_2::FireRating"
    if "part_3::TightnessClass" in ty:
        return "crate::artifacts::en1992::part_3::TightnessClass"
    if ty.startswith("Vec<QkEntry>"):
        return f"Vec<{prefix}QkEntry>"
    if ty.startswith("Vec<"):
        inner = ty[4:-1]
        return f"Vec<{inner}>"
    return "String"


def ts_type(field: Field) -> str:
    ty = field.ty
    if ty in ("f64",):
        return "number"
    if ty in ("u8", "u32", "i32"):
        return "number"
    if ty == "bool":
        return "boolean"
    if ty == "String" or "Choice" in ty or "Category" in ty or "Curve" in ty or "Rating" in ty or "Class" in ty:
        return "string"
    if ty.startswith("Vec<"):
        if "QkEntry" in ty:
            return f"{field.camel.replace('Q', 'En1990Q') if 'qK' in field.camel else 'En1990QkEntry'}[]"
        return "string[]"
    return "string"


def proto_scalar(field: Field) -> str:
    ty = field.ty
    if ty == "f64":
        return "double"
    if ty in ("u8", "u32"):
        return "uint32"
    if ty == "bool":
        return "bool"
    return "string"


def graphql_scalar(field: Field) -> str:
    ty = field.ty
    if ty == "f64":
        return "Float"
    if ty in ("u8", "u32"):
        return "Int"
    if ty == "bool":
        return "Boolean"
    if ty.startswith("Vec<"):
        inner = "String"
        if "QkEntry" in ty:
            inner = f"{field.camel}Item"  # placeholder
        return f"[{inner}!]!"
    return "String"


def write_json_artifact(key: str, prefix: str, fields: list[Field], path: Path) -> None:
    props = {}
    required = []
    defs = {}
    if any(f.rust == "q_k" for f in fields):
        defs[f"{prefix}QkEntry"] = {
            "title": f"{prefix}QkEntry",
            "type": "object",
            "additionalProperties": False,
            "required": ["category", "value"],
            "properties": {
                "category": {"type": "string"},
                "value": {"type": "number", "format": "double"},
            },
        }
    for f in fields:
        props[f.camel] = {**json_type(f, prefix), "x-semio-state": f.state}
        required.append(f.camel)
    props["selectedCheckIndex"] = {
        "oneOf": [{"type": "null"}, {"type": "integer", "format": "uint32", "minimum": 0}],
        "x-semio-state": "shared-ui",
    }
    doc = {
        "$id": f"https://semio.tech/schema/s/norm/{key}/artifact.json",
        "title": f"{prefix}Artifact",
        "type": "object",
        "additionalProperties": False,
        "required": required,
        "properties": props,
    }
    if defs:
        doc["$defs"] = defs
    path.write_text(json.dumps(doc, indent=2) + "\n", encoding="utf-8")


def write_json_snapshot(key: str, prefix: str, fields: list[Field], path: Path) -> None:
    props = {}
    required = []
    defs = {}
    if any(f.rust == "q_k" for f in fields):
        defs[f"{prefix}QkEntry"] = {
            "title": f"{prefix}QkEntry",
            "type": "object",
            "additionalProperties": False,
            "required": ["category", "value"],
            "properties": {
                "category": {"type": "string"},
                "value": {"type": "number", "format": "double"},
            },
        }
    for f in fields:
        props[f.camel] = {**json_type(f, prefix), "x-semio-state": "persistent"}
        required.append(f.camel)
    doc = {
        "$id": f"https://semio.tech/schema/s/norm/{key}/snapshot.json",
        "title": f"{prefix}Snapshot",
        "type": "object",
        "additionalProperties": False,
        "required": required,
        "properties": props,
    }
    if defs:
        doc["$defs"] = defs
    path.write_text(json.dumps(doc, indent=2) + "\n", encoding="utf-8")


def write_json_diff(key: str, prefix: str, fields: list[Field], path: Path) -> None:
    props = {
        "artifact": {
            "title": f"{prefix}Artifact",
            "type": "object",
            "x-semio-state": "persistent",
        }
    }
    defs = {}
    if any(f.rust == "q_k" for f in fields):
        defs[f"{prefix}QkList"] = {
            "title": f"{prefix}QkList",
            "type": "object",
            "additionalProperties": False,
            "required": ["values"],
            "properties": {
                "values": {
                    "type": "array",
                    "items": {"$ref": f"#/$defs/{prefix}QkEntry"},
                }
            },
        }
        defs[f"{prefix}QkEntry"] = {
            "title": f"{prefix}QkEntry",
            "type": "object",
            "additionalProperties": False,
            "required": ["category", "value"],
            "properties": {
                "category": {"type": "string"},
                "value": {"type": "number", "format": "double"},
            },
        }
    for f in fields:
        jt = json_type(f, prefix)
        if f.ty.startswith("Vec<"):
            wrap = f"{prefix}{'QkList' if 'QkEntry' in f.ty else 'StringList'}"
            if wrap not in defs and wrap.endswith("StringList"):
                defs[wrap] = {
                    "title": wrap,
                    "type": "object",
                    "additionalProperties": False,
                    "required": ["values"],
                    "properties": {"values": {"type": "array", "items": {"type": "string"}}},
                }
            props[f.camel] = {"$ref": f"#/$defs/{wrap}", "x-semio-state": "persistent"}
        else:
            props[f.camel] = {**jt, "x-semio-state": "persistent"}
    props["selectedCheckIndex"] = {
        "oneOf": [{"type": "null"}, {"type": "integer", "format": "uint32", "minimum": 0}],
        "x-semio-state": "shared-ui",
    }
    doc = {
        "$id": f"https://semio.tech/schema/s/norm/{key}/diff.json",
        "title": f"{prefix}Diff",
        "type": "object",
        "additionalProperties": False,
        "required": [],
        "properties": props,
    }
    if defs:
        doc["$defs"] = defs
    path.write_text(json.dumps(doc, indent=2) + "\n", encoding="utf-8")


def write_ts_artifact(prefix: str, fields: list[Field], path: Path) -> None:
    lines = [f"/** 🧬️ {prefix} artifact schema — every field with its state class. */", "", f"export interface {prefix}Artifact {{"]
    for f in fields:
        lines.append(f"  /** @state {f.state.replace('_', '-')} */")
        lines.append(f"  {f.camel}: {ts_type(f)};")
    lines.append("  /** @state shared-ui */")
    lines.append("  selectedCheckIndex?: number | null;")
    lines.append("}")
    lines.append("")
    path.write_text("\n".join(lines), encoding="utf-8")


def write_ts_snapshot(prefix: str, fields: list[Field], path: Path) -> None:
    lines = [f"/** 🧬️ {prefix} snapshot schema — persistent fields only. */", "", f"export interface {prefix}Snapshot {{"]
    for f in fields:
        lines.append("  /** @state persistent */")
        lines.append(f"  {f.camel}: {ts_type(f)};")
    lines.append("}")
    lines.append("")
    path.write_text("\n".join(lines), encoding="utf-8")


def write_ts_diff(prefix: str, fields: list[Field], path: Path) -> None:
    lines = [f"/** 🧬️ {prefix} diff schema — sparse field delta. */", "", f"export interface {prefix}Diff {{"]
    lines.append("  /** @state persistent */")
    lines.append(f"  artifact?: {prefix}Artifact;")
    for f in fields:
        lines.append("  /** @state persistent */")
        if f.ty.startswith("Vec<"):
            wrap = f"{prefix}QkList" if "QkEntry" in f.ty else f"{prefix}StringList"
            lines.append(f"  {f.camel}?: {wrap};")
        else:
            lines.append(f"  {f.camel}?: {ts_type(f)};")
    lines.append("  /** @state shared-ui */")
    lines.append(f"  selectedCheckIndex?: number | null;")
    lines.append("}")
    lines.append("")
    lines.append(f"export interface {prefix}Artifact {{")
    for f in fields:
        lines.append(f"  {f.camel}: {ts_type(f)};")
    lines.append("  selectedCheckIndex?: number | null;")
    lines.append("}")
    if any(f.ty.startswith("Vec<QkEntry>") for f in fields):
        lines.append("")
        lines.append(f"export interface {prefix}QkEntry {{ category: string; value: number; }}")
        lines.append(f"export interface {prefix}QkList {{ values: {prefix}QkEntry[]; }}")
    path.write_text("\n".join(lines), encoding="utf-8")


def write_graphql_artifact(prefix: str, fields: list[Field], path: Path) -> None:
    lines = [f"# 🧬️ {prefix} artifact schema — every field with its state class.", "", f"type {prefix}Artifact {{"]
    for f in fields:
        g = graphql_scalar(f)
        lines.append(f"  {f.camel}: {g} @state(class: PERSISTENT)")
    lines.append("  selectedCheckIndex: Int @state(class: SHARED_UI)")
    lines.append("}")
    lines.append("")
    path.write_text("\n".join(lines), encoding="utf-8")


def write_graphql_snapshot(prefix: str, fields: list[Field], path: Path) -> None:
    lines = [f"# 🧬️ {prefix} snapshot schema — persistent fields only.", "", f"type {prefix}Snapshot {{"]
    for f in fields:
        g = graphql_scalar(f)
        lines.append(f"  {f.camel}: {g} @state(class: PERSISTENT)")
    lines.append("}")
    lines.append("")
    path.write_text("\n".join(lines), encoding="utf-8")


def write_graphql_diff(prefix: str, fields: list[Field], path: Path) -> None:
    lines = [f"# 🧬️ {prefix} diff schema — sparse field delta.", "", f"type {prefix}Diff {{"]
    lines.append(f"  artifact: {prefix}Artifact @state(class: PERSISTENT)")
    for f in fields:
        if f.ty.startswith("Vec<QkEntry>"):
            lines.append(f"  {f.camel}: {prefix}QkList @state(class: PERSISTENT)")
        elif f.ty.startswith("Vec<"):
            lines.append(f"  {f.camel}: {prefix}StringList @state(class: PERSISTENT)")
        else:
            g = graphql_scalar(f).replace("!", "")
            lines.append(f"  {f.camel}: {g} @state(class: PERSISTENT)")
    lines.append("  selectedCheckIndex: Int @state(class: SHARED_UI)")
    lines.append("}")
    lines.append("")
    lines.append(f"type {prefix}Artifact {{")
    for f in fields:
        lines.append(f"  {f.camel}: {graphql_scalar(f)}")
    lines.append("  selectedCheckIndex: Int")
    lines.append("}")
    if any(f.ty.startswith("Vec<QkEntry>") for f in fields):
        lines.extend(["", f"type {prefix}QkEntry {{", "  category: String!", "  value: Float!", "}", "", f"type {prefix}QkList {{", f"  values: [{prefix}QkEntry!]!", "}"])
    path.write_text("\n".join(lines), encoding="utf-8")


def write_proto_artifact(key: str, prefix: str, fields: list[Field], path: Path) -> None:
    lines = [
        "syntax = \"proto3\";",
        f"package semio.s.norm.{key}.artifact;",
        "",
        f"// 🧬️ {prefix} artifact schema — every field with its state class.",
        "",
        f"message {prefix}Artifact {{",
    ]
    for i, f in enumerate(fields, 1):
        lines.append(f"  // @state persistent")
        rep = "repeated " if f.ty.startswith("Vec<") else ""
        sc = proto_scalar(f)
        if f.ty.startswith("Vec<QkEntry>"):
            lines.append(f"  repeated {prefix}QkEntry {f.rust} = {i};")
        elif f.ty.startswith("Vec<"):
            lines.append(f"  repeated string {f.rust} = {i};")
        else:
            lines.append(f"  {sc} {f.rust} = {i};")
    lines.append(f"  // @state shared-ui")
    lines.append(f"  optional uint32 selected_check_index = {len(fields) + 1};")
    lines.append("}")
    if any(f.rust == "q_k" for f in fields):
        lines.extend(["", f"message {prefix}QkEntry {{", "  string category = 1;", "  double value = 2;", "}"])
    lines.append("")
    path.write_text("\n".join(lines), encoding="utf-8")


def write_proto_snapshot(key: str, prefix: str, fields: list[Field], path: Path) -> None:
    lines = [
        "syntax = \"proto3\";",
        f"package semio.s.norm.{key}.snapshot;",
        "",
        f"message {prefix}Snapshot {{",
    ]
    for i, f in enumerate(fields, 1):
        lines.append("  // @state persistent")
        if f.ty.startswith("Vec<QkEntry>"):
            lines.append(f"  repeated {prefix}QkEntry {f.rust} = {i};")
        elif f.ty.startswith("Vec<"):
            lines.append(f"  repeated string {f.rust} = {i};")
        else:
            lines.append(f"  {proto_scalar(f)} {f.rust} = {i};")
    lines.append("}")
    if any(f.rust == "q_k" for f in fields):
        lines.extend(["", f"message {prefix}QkEntry {{", "  string category = 1;", "  double value = 2;", "}"])
    lines.append("")
    path.write_text("\n".join(lines), encoding="utf-8")


def write_proto_diff(key: str, prefix: str, fields: list[Field], path: Path) -> None:
    lines = [
        "syntax = \"proto3\";",
        f"package semio.s.norm.{key}.diff;",
        "",
        f"message {prefix}Diff {{",
        "  // @state persistent",
        f"  optional {prefix}Artifact artifact = 1;",
    ]
    idx = 2
    for f in fields:
        lines.append("  // @state persistent")
        if f.ty.startswith("Vec<QkEntry>"):
            lines.append(f"  optional {prefix}QkList {f.rust} = {idx};")
        elif f.ty.startswith("Vec<"):
            lines.append(f"  optional {prefix}StringList {f.rust} = {idx};")
        else:
            lines.append(f"  optional {proto_scalar(f)} {f.rust} = {idx};")
        idx += 1
    lines.append("  // @state shared-ui")
    lines.append(f"  optional uint32 selected_check_index = {idx};")
    lines.append("}")
    lines.extend(["", f"message {prefix}Artifact {{"])
    for i, f in enumerate(fields, 1):
        if f.ty.startswith("Vec<QkEntry>"):
            lines.append(f"  repeated {prefix}QkEntry {f.rust} = {i};")
        elif f.ty.startswith("Vec<"):
            lines.append(f"  repeated string {f.rust} = {i};")
        else:
            lines.append(f"  {proto_scalar(f)} {f.rust} = {i};")
    lines.append(f"  optional uint32 selected_check_index = {len(fields) + 1};")
    lines.append("}")
    if any(f.rust == "q_k" for f in fields):
        lines.extend([
            "",
            f"message {prefix}QkEntry {{ string category = 1; double value = 2; }}",
            f"message {prefix}QkList {{ repeated {prefix}QkEntry values = 1; }}",
        ])
    else:
        lines.extend(["", f"message {prefix}StringList {{ repeated string values = 1; }}"])
    lines.append("")
    path.write_text("\n".join(lines), encoding="utf-8")


def write_rust_artifact_schema(key: str, prefix: str, mod_key: str, fields: list[Field], path: Path) -> None:
    qk = any(f.rust == "q_k" for f in fields)
    lines = [
        f"//! 🧬️ {prefix} artifact schema — every field of the artifact with its state class.",
        "",
        "use schema::ArtifactSchema;",
        "use serde::{{Deserialize, Serialize}};",
        "",
        "//#region 🔖️Artifact",
        f"/// 🧬️ Full {prefix} artifact state across persistent and shared-ui classes.",
        "#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]",
        '#[serde(rename_all = "camelCase")]',
        f'#[artifact_schema(id = "s.norm.{key}")]',
        f"pub struct {prefix}Artifact {{",
    ]
    for f in fields:
        rt = rust_schema_type(f, prefix, False)
        lines.append(f"    #[state(persistent)] pub {f.rust}: {rt},")
    lines.append("    #[state(shared_ui)] pub selected_check_index: Option<u32>,")
    lines.append("}")
    lines.append("//#endregion 🔖️Artifact")
    lines.append("")
    if qk:
        lines.extend([
            "//#region 🔖️Helpers",
            f"/// 📊️ One variable action category/value pair for `{prefix}Snapshot.q_k`.",
            '#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]',
            '#[serde(rename_all = "camelCase")]',
            f"pub struct {prefix}QkEntry {{",
            "    pub category: String,",
            "    pub value: f64,",
            "}",
            "//#endregion 🔖️Helpers",
            "",
        ])
    lines.extend([
        "//#region 🔖️Conversions",
        f"impl {prefix}Artifact {{",
        f"    /// 📸️ Persisted subset.",
        f"    pub fn to_snapshot(&self) -> crate::artifacts::{mod_key}::{prefix}Snapshot {{",
        f"        crate::artifacts::{mod_key}::{prefix}Snapshot {{",
    ])
    for f in fields:
        if f.ty == "String" or f.ty.startswith("Vec<"):
            lines.append(f"            {f.rust}: self.{f.rust}.clone(),")
        else:
            lines.append(f"            {f.rust}: self.{f.rust},")
    lines.extend([
        "        }",
        "    }",
        "",
        f"    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.",
        f"    pub fn from_snapshot(snapshot: crate::artifacts::{mod_key}::{prefix}Snapshot) -> Self {{",
        "        Self {",
    ])
    for f in fields:
        if f.ty == "String" or f.ty.startswith("Vec<"):
            lines.append(f"            {f.rust}: snapshot.{f.rust}.clone(),")
        else:
            lines.append(f"            {f.rust}: snapshot.{f.rust},")
    lines.append("            selected_check_index: None,")
    lines.append("        }")
    lines.append("    }")
    lines.append("}")
    lines.append("//#endregion 🔖️Conversions")
    lines.append("")
    lines.extend(descriptor_block(key, prefix, mod_key))
    path.write_text("\n".join(lines), encoding="utf-8")


def descriptor_block(key: str, prefix: str, mod_key: str) -> list[str]:
    return [
        "//#region 🔖️Descriptor",
        f"/// 🧬️ Descriptor for `s.norm.{key}` — fifteen handcrafted schema leaves.",
        f"pub fn {key}_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {{",
        "    schema::ArtifactSchemaDescriptor {",
        f'        id: "s.norm.{key}",',
        "        artifact: schema::FacetLeaves {",
        '            rust: include_str!("🦀️component.rs"),',
        '            typescript: include_str!("🟦️component.ts"),',
        '            graphql: include_str!("🔗️component.graphql"),',
        '            json_schema: include_str!("🔣️component.json"),',
        '            proto: include_str!("🛰️component.proto"),',
        "        },",
        "        snapshot: schema::FacetLeaves {",
        '            rust: include_str!("../📸️snapshot/🧬️schema/🦀️component.rs"),',
        '            typescript: include_str!("../📸️snapshot/🧬️schema/🟦️component.ts"),',
        '            graphql: include_str!("../📸️snapshot/🧬️schema/🔗️component.graphql"),',
        '            json_schema: include_str!("../📸️snapshot/🧬️schema/🔣️component.json"),',
        '            proto: include_str!("../📸️snapshot/🧬️schema/🛰️component.proto"),',
        "        },",
        "        diff: schema::FacetLeaves {",
        '            rust: include_str!("../🔺️diff/🧬️schema/🦀️component.rs"),',
        '            typescript: include_str!("../🔺️diff/🧬️schema/🟦️component.ts"),',
        '            graphql: include_str!("../🔺️diff/🧬️schema/🔗️component.graphql"),',
        '            json_schema: include_str!("../🔺️diff/🧬️schema/🔣️component.json"),',
        '            proto: include_str!("../🔺️diff/🧬️schema/🛰️component.proto"),',
        "        },",
        "    }",
        "}",
        "//#endregion 🔖️Descriptor",
    ]


def refactor_root_component(folder: Path, key: str, prefix: str, mod_key: str, orig: str, nested: list[str]) -> None:
    qk_block = ""
    if "QkEntry" in orig:
        qk_block = f"""
/// 📊️ One variable action category/value pair for `{prefix}Snapshot.q_k`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct {prefix}QkEntry {{
    #[dsl(positional)]
    pub category: String,
    #[dsl(positional)]
    pub value: f64,
}}
"""
    nested_blocks = ""
    for n in nested:
        nested_blocks += re.search(rf"pub mod {n} \{{[\s\S]*?^}}", orig, re.M) and "" or ""
    nested_matches = list(re.finditer(r"(pub mod \w+ \{[\s\S]*?\n\})", orig))
    nested_blocks = "\n".join(m.group(1) for m in nested_matches)

    new = f"""//! {prefix} — document entities (constitutional: general).

use crate::document::AnnexChoice;
use serde::{{Deserialize, Serialize}};

//#region 🔖️Types
{nested_blocks}
{qk_block}
/// 📸️ Persisted snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.
pub use crate::artifacts::{mod_key}::snapshot::schema::{prefix}Snapshot;
//#endregion 🔖️Types

//#region 🔖️ArtifactKind
/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {{
    crate::app_surface::artifact_kind_spec("{key}", "{prefix.replace('En199', 'EN 199')}")
}}
//#endregion 🔖️ArtifactKind
"""
    (folder / "🦀️component.rs").write_text(new, encoding="utf-8")


def write_snapshot_rust(folder: Path, key: str, prefix: str, dsl_id: str, orig: str, fields: list[Field], nested: list[str]) -> None:
    nested_blocks = "\n".join(m.group(1) for m in re.finditer(r"(pub mod \w+ \{[\s\S]*?\n\})", orig))
    struct_body = re.search(r"pub struct Document \{([^}]+)\}", orig, re.S).group(1)
    struct_body = struct_body.replace("Document", f"{prefix}Snapshot").replace("QkEntry", f"{prefix}QkEntry")
    codecs = re.search(r"//#region 🔖️HandcraftedDocumentCodecs[\s\S]*?//#endregion 🔖️HandcraftedDocumentCodecs", orig)
    default_impl = re.search(r"impl Default for Document \{[\s\S]*?\n\}", orig)
    default_text = default_impl.group(0).replace("Document", f"{prefix}Snapshot").replace("QkEntry", f"{prefix}QkEntry") if default_impl else ""
    qk = ""
    if "QkEntry" in orig and f"{prefix}QkEntry" not in nested_blocks:
        qk = f"""
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct {prefix}QkEntry {{
    #[dsl(positional)]
    pub category: String,
    #[dsl(positional)]
    pub value: f64,
}}
"""
    text = f"""//! 🧬️ {prefix} snapshot schema — persistent fields only.

use schema::ArtifactSchema;
use serde::{{Deserialize, Serialize}};

//#region 🔖️Snapshot
{nested_blocks}
{qk}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "{dsl_id}", layout = "lines")]
#[artifact_schema(id = "s.norm.{key}")]
pub struct {prefix}Snapshot {{{struct_body}}}
{codecs.group(0).replace('Document', prefix + 'Snapshot').replace('QkEntry', prefix + 'QkEntry') if codecs else ''}

{default_text}
//#endregion 🔖️Snapshot
"""
    out = folder / "📸️snapshot/🧬️schema/🦀️component.rs"
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(text, encoding="utf-8")


def write_diff_schema_rust(key: str, prefix: str, mod_key: str, fields: list[Field], path: Path) -> None:
    lines = [
        f"//! 🧬️ {prefix} diff schema — sparse field delta over the artifact.",
        "",
        "use schema::ArtifactSchema;",
        "use serde::{Deserialize, Serialize};",
        "",
        "//#region 🔖️Diff",
        f"/// 🔺️ Sparse field delta for the {prefix} artifact.",
        "#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]",
        '#[serde(rename_all = "camelCase", default)]',
        f'#[artifact_schema(id = "s.norm.{key}")]',
        f"pub struct {prefix}Diff {{",
        f"    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::{mod_key}::schema::{prefix}Artifact>>,",
    ]
    for f in fields:
        rt = rust_schema_type(f, prefix, False)
        if f.ty.startswith("Vec<QkEntry>"):
            lines.append(f"    #[state(persistent)] pub {f.rust}: Option<{prefix}QkList>,")
        elif f.ty.startswith("Vec<"):
            lines.append(f"    #[state(persistent)] pub {f.rust}: Option<{prefix}StringList>,")
        else:
            lines.append(f"    #[state(persistent)] pub {f.rust}: Option<{rt}>,")
    lines.append("    #[state(shared_ui)] pub selected_check_index: Option<Option<u32>>,")
    lines.append("}")
    lines.append("//#endregion 🔖️Diff")
    lines.append("")
    lines.extend([
        "//#region 🔖️DeltaHelpers",
        f"/// 📋 List wrapper for optional vector diffs.",
        "#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]",
        '#[serde(rename_all = "camelCase", default)]',
        f"pub struct {prefix}StringList {{ pub values: Vec<String> }}",
    ])
    if any(f.rust == "q_k" for f in fields):
        lines.extend([
            "",
            f"/// 📋 Qk table wrapper for optional list diffs.",
            "#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]",
            '#[serde(rename_all = "camelCase", default)]',
            f"pub struct {prefix}QkList {{ pub values: Vec<crate::artifacts::{mod_key}::schema::{prefix}QkEntry> }}",
        ])
    lines.append("//#endregion 🔖️DeltaHelpers")
    lines.append("")
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("\n".join(lines), encoding="utf-8")


def write_diff_runtime(prefix: str, mod_key: str, fields: list[Field], path: Path) -> None:
    apply_snap = []
    apply_art = []
    take_fields = []
    for f in fields:
        take_fields.append(f"        take!({f.rust});")
        if f.ty.startswith("Vec<QkEntry>"):
            apply_snap.append(f"        if let Some(list) = &self.{f.rust} {{ next.{f.rust} = list.values.clone(); }}")
            apply_art.append(f"        if let Some(list) = &self.{f.rust} {{ next.{f.rust} = list.values.clone(); }}")
        elif f.ty.startswith("Vec<"):
            apply_snap.append(f"        if let Some(list) = &self.{f.rust} {{ next.{f.rust} = list.values.clone(); }}")
            apply_art.append(f"        if let Some(list) = &self.{f.rust} {{ next.{f.rust} = list.values.clone(); }}")
        elif f.ty == "String":
            apply_snap.append(f"        if let Some(value) = &self.{f.rust} {{ next.{f.rust} = value.clone(); }}")
            apply_art.append(f"        if let Some(value) = &self.{f.rust} {{ next.{f.rust} = value.clone(); }}")
        else:
            apply_snap.append(f"        if let Some(value) = self.{f.rust} {{ next.{f.rust} = value; }}")
            apply_art.append(f"        if let Some(value) = self.{f.rust} {{ next.{f.rust} = value; }}")
    text = f"""//! 🔺️ {prefix} artifact — sparse field diff runtime.

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use super::schema::*;

use crate::artifacts::{mod_key}::schema::{prefix}Artifact;
use crate::artifacts::{mod_key}::{prefix}Snapshot;
use protocol::MutationDiff;

//#region 🔖️Apply
impl {prefix}Diff {{
    pub fn apply_to_artifact(&self, artifact: &{prefix}Artifact) -> {prefix}Artifact {{
        if let Some(replacement) = &self.artifact {{
            return (**replacement).clone();
        }}
        let mut next = artifact.clone();
{chr(10).join(apply_art)}
        if let Some(value) = &self.selected_check_index {{
            next.selected_check_index = *value;
        }}
        next
    }}
}}

impl MutationDiff<{prefix}Snapshot> for {prefix}Diff {{
    fn apply(&self, snapshot: &{prefix}Snapshot) -> {prefix}Snapshot {{
        if let Some(replacement) = &self.artifact {{
            return replacement.to_snapshot();
        }}
        let mut next = snapshot.clone();
{chr(10).join(apply_snap)}
        next
    }}

    fn absorb(&mut self, other: Self) {{
        if other.artifact.is_some() {{
            *self = other;
            return;
        }}
        macro_rules! take {{
            ($field:ident) => {{
                if other.$field.is_some() {{
                    self.$field = other.$field;
                }}
            }};
        }}
{chr(10).join(take_fields)}
        take!(selected_check_index);
    }}
}}
//#endregion 🔖️Apply

//#region 🔖️Helpers
pub fn diff_set_snapshot(snapshot: &{prefix}Snapshot) -> {prefix}Diff {{
    {prefix}Diff {{
        artifact: Some(Box::new({prefix}Artifact::from_snapshot(snapshot.clone()))),
        ..Default::default()
    }}
}}
//#endregion 🔖️Helpers
"""
    path.write_text(text, encoding="utf-8")


def move_pack(folder: Path) -> None:
    src = folder / "🎒️pack"
    dst = folder / "📸️snapshot/🎒️pack"
    if not src.exists():
        return
    dst.parent.mkdir(parents=True, exist_ok=True)
    if dst.exists():
        shutil.rmtree(dst)
    shutil.move(str(src), str(dst))
    proto = dst / "📡️component.protocol.semio"
    if proto.exists():
        text = proto.read_text(encoding="utf-8")
        text = re.sub(r"schema en199\d", lambda m: m.group(0).replace("en", "norm.en") if "norm" not in m.group(0) else m.group(0), text)
        proto.write_text(text, encoding="utf-8")


def generate_one(folder_name: str, key: str, prefix: str, dsl_id: str) -> None:
    folder = NORM / folder_name
    fields, orig, nested = parse_document_struct(folder / "🦀️component.rs")
    for f in fields:
        f.state = "persistent"
    art = folder / "🧬️schema"
    snap = folder / "📸️snapshot/🧬️schema"
    diff = folder / "🔺️diff/🧬️schema"
    for d in (art, snap, diff):
        d.mkdir(parents=True, exist_ok=True)
    write_json_artifact(key, prefix, fields, art / "🔣️component.json")
    write_json_snapshot(key, prefix, fields, snap / "🔣️component.json")
    write_json_diff(key, prefix, fields, diff / "🔣️component.json")
    write_ts_artifact(prefix, fields, art / "🟦️component.ts")
    write_ts_snapshot(prefix, fields, snap / "🟦️component.ts")
    write_ts_diff(prefix, fields, diff / "🟦️component.ts")
    write_graphql_artifact(prefix, fields, art / "🔗️component.graphql")
    write_graphql_snapshot(prefix, fields, snap / "🔗️component.graphql")
    write_graphql_diff(prefix, fields, diff / "🔗️component.graphql")
    write_proto_artifact(key, prefix, fields, art / "🛰️component.proto")
    write_proto_snapshot(key, prefix, fields, snap / "🛰️component.proto")
    write_proto_diff(key, prefix, fields, diff / "🛰️component.proto")
    write_rust_artifact_schema(key, prefix, key, fields, art / "🦀️component.rs")
    write_snapshot_rust(folder, key, prefix, dsl_id, orig, fields, nested)
    write_diff_schema_rust(key, prefix, key, fields, diff / "🦀️component.rs")
    write_diff_runtime(prefix, key, fields, folder / "🔺️diff/🦀️component.rs")
    move_pack(folder)
    refactor_root_component(folder, key, prefix, key, orig, nested)


def main() -> None:
    for folder_name, key, prefix, dsl_id in ARTIFACTS:
        generate_one(folder_name, key, prefix, dsl_id)
        print("generated", key)


if __name__ == "__main__":
    main()
