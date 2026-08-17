#!/usr/bin/env python3
"""Generate fifteen schema leaves + diff runtime stubs for en1995–en1999."""
from __future__ import annotations

import json
import re
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
TICKET = Path(
    "/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️08/ARTIFACT-SCHEMA-FACETS"
)

ARTIFACTS = [
    ("en1995", "En1995", "🪵️"),
    ("en1996", "En1996", "🧱️"),
    ("en1997", "En1997", "🌍️"),
    ("en1998", "En1998", "🌋️"),
    ("en1999", "En1999", "✨️"),
]

ENUM_FIELDS: dict[str, set[str]] = {
    "en1995": {"annex"},
    "en1996": {"annex", "masonry_class", "design_situation", "exposure", "mortar"},
    "en1997": {"annex", "design_approach"},
    "en1998": set(),
    "en1999": {"annex"},
}

INT_FIELDS: dict[str, set[str]] = {
    "en1995": set(),
    "en1996": {"fire_resistance_min", "storeys"},
    "en1997": {"pile_n_profiles"},
    "en1998": {"seismic_zone"},
    "en1999": set(),
}

BOOL_FIELDS: dict[str, set[str]] = {
    "en1998": {"multiple_resisting_systems", "tower_is_chimney"},
    "en1995": set(),
    "en1996": set(),
    "en1997": set(),
    "en1999": set(),
}


def snake_to_camel(s: str) -> str:
    parts = s.split("_")
    return parts[0] + "".join(p.capitalize() for p in parts[1:])


def parse_document_fields(component_rs: str) -> list[tuple[str, str]]:
    m = re.search(r"pub struct Document\s*\{([^}]+)\}", component_rs, re.S)
    if not m:
        raise ValueError("Document struct not found")
    fields = []
    for line in m.group(1).splitlines():
        line = line.strip()
        if not line or line.startswith("#") or line.startswith("//"):
            continue
        if line.startswith("pub "):
            fm = re.match(r"pub\s+(\w+):\s+([^,]+)", line)
            if fm:
                fields.append((fm.group(1), fm.group(2).strip()))
    return fields


def rust_type(field: str, rust_ty: str, key: str) -> str:
    if field in BOOL_FIELDS.get(key, set()):
        return "bool"
    if field in INT_FIELDS.get(key, set()):
        return "u32" if "profiles" in field or "storeys" in field or "resistance" in field else "u8"
    if "AnnexChoice" in rust_ty or field in ENUM_FIELDS.get(key, set()):
        if "String" in rust_ty:
            return "String"
        return f"crate::document::{rust_ty}" if "::" not in rust_ty and rust_ty not in (
            "String",
            "bool",
            "f64",
            "u8",
            "u32",
        ) else rust_ty.replace("part_2::", "crate::artifacts::en1996::part_2::" if key == "en1996" else rust_ty)
    if rust_ty.startswith("part_2::"):
        return f"crate::artifacts::en1996::{rust_ty}"
    return rust_ty


def json_type(field: str, key: str) -> dict:
    if field in BOOL_FIELDS.get(key, set()):
        return {"type": "boolean"}
    if field in INT_FIELDS.get(key, set()):
        return {"type": "integer"}
    return {"type": "number", "format": "double"}


def gql_type(field: str, key: str) -> str:
    if field in BOOL_FIELDS.get(key, set()):
        return "Boolean!"
    if field in INT_FIELDS.get(key, set()):
        return "Int!"
    return "Float!" if json_type(field, key).get("format") == "double" else "String!"


def ts_type(field: str, key: str) -> str:
    if field in BOOL_FIELDS.get(key, set()):
        return "boolean"
    if field in INT_FIELDS.get(key, set()):
        return "number"
    return "number"


def proto_type(field: str, key: str) -> str:
    if field in BOOL_FIELDS.get(key, set()):
        return "bool"
    if field in INT_FIELDS.get(key, set()):
        return "uint32"
    return "double"


def write_snapshot_rust(key: str, prefix: str, emoji: str, fields: list[tuple[str, str]], out: Path) -> None:
    use_lines = ["use crate::document::AnnexChoice;", "use schema::ArtifactSchema;", "use serde::{Deserialize, Serialize};"]
    if key == "en1996":
        use_lines = [
            "use crate::artifacts::en1996::{MasonryClass, part_2};",
            "use crate::document::{AnnexChoice, DesignSituation};",
            "use schema::ArtifactSchema;",
            "use serde::{Deserialize, Serialize};",
        ]
    elif key == "en1997":
        use_lines = ["use crate::document::AnnexChoice;", "use schema::ArtifactSchema;", "use serde::{Deserialize, Serialize};"]
    elif key == "en1998":
        use_lines = ["use schema::ArtifactSchema;", "use serde::{Deserialize, Serialize};"]
    body_fields = []
    for name, rty in fields:
        rt = rust_type(name, rty, key)
        body_fields.append(f"    #[state(persistent)]\n    pub {name}: {rt},")
    dsl_attrs = f'#[dsl(id = "norm.{key}", layout = "lines")]\n#[artifact_schema(id = "s.norm.{key}")]'
    content = f"""//! {emoji} EN {key[2:]} snapshot schema — persistent fields only.

{chr(10).join(use_lines)}

//#region 🔖️Snapshot
/// 📸️ Persisted EN {key[2:]} document snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
{dsl_attrs}
pub struct {prefix}Snapshot {{
{chr(10).join(body_fields)}
}}
//#endregion 🔖️Snapshot
"""
    out.write_text(content)


def write_artifact_rust(key: str, prefix: str, emoji: str, fields: list[tuple[str, str]], out: Path) -> None:
    use_lines = ["use crate::document::AnnexChoice;", "use schema::ArtifactSchema;", "use serde::{Deserialize, Serialize};"]
    if key == "en1996":
        use_lines = [
            "use crate::artifacts::en1996::{MasonryClass, part_2};",
            "use crate::document::{AnnexChoice, DesignSituation};",
            "use schema::ArtifactSchema;",
            "use serde::{Deserialize, Serialize};",
        ]
    elif key == "en1998":
        use_lines = ["use schema::ArtifactSchema;", "use serde::{Deserialize, Serialize};"]
    snap_type = f"crate::artifacts::{key}::{prefix}Snapshot"
    body_fields = []
    for name, rty in fields:
        rt = rust_type(name, rty, key)
        body_fields.append(f"    #[state(persistent)] pub {name}: {rt},")
    body_fields.append("    #[state(shared_ui)] pub selected_check_index: Option<u32>,")
    set_lines = [f"        self.{n} = snapshot.{n};" for n, _ in fields]
    from_snap = [f"            {n}: snapshot.{n}," for n, _ in fields]
    to_snap = [f"            {n}: self.{n}.clone()," for n, _ in fields]
    content = f"""//! {emoji} EN {key[2:]} artifact schema — every field with its state class.

{chr(10).join(use_lines)}

//#region 🔖️Artifact
/// 🧬️ Full EN {key[2:]} artifact state (persisted document + shared UI).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.{key}")]
pub struct {prefix}Artifact {{
{chr(10).join(body_fields)}
}}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for {prefix}Artifact {{
    fn default() -> Self {{
        Self {{
            ..{snap_type}::default().into()
        }}
    }}
}}

impl From<{snap_type}> for {prefix}Artifact {{
    fn from(snapshot: {snap_type}) -> Self {{
        Self::from_snapshot(snapshot)
    }}
}}

impl {prefix}Artifact {{
    pub fn to_snapshot(&self) -> {snap_type} {{
        {snap_type} {{
{chr(10).join(to_snap)}
        }}
    }}

    pub fn from_snapshot(snapshot: {snap_type}) -> Self {{
        Self {{
{chr(10).join(from_snap)}
            selected_check_index: None,
        }}
    }}

    pub fn set_snapshot(&mut self, snapshot: {snap_type}) {{
{chr(10).join(set_lines)}
    }}
}}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub fn {key}_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {{
    schema::ArtifactSchemaDescriptor {{
        id: "s.norm.{key}",
        artifact: schema::FacetLeaves {{
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        }},
        snapshot: schema::FacetLeaves {{
            rust: include_str!("../📸️snapshot/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../📸️snapshot/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../📸️snapshot/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../📸️snapshot/🧬️schema/🔣️component.json"),
            proto: include_str!("../📸️snapshot/🧬️schema/🛰️component.proto"),
        }},
        diff: schema::FacetLeaves {{
            rust: include_str!("../🔺️diff/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../🔺️diff/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../🔺️diff/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../🔺️diff/🧬️schema/🔣️component.json"),
            proto: include_str!("../🔺️diff/🧬️schema/🛰️component.proto"),
        }},
    }}
}}
//#endregion 🔖️Descriptor
"""
    out.write_text(content)


def write_diff_rust(key: str, prefix: str, fields: list[tuple[str, str]], out: Path) -> None:
    art = f"crate::artifacts::{key}::schema::{prefix}Artifact"
    snap = f"crate::artifacts::{key}::{prefix}Snapshot"
    body = [
        f"    #[state(persistent)] pub artifact: Option<Box<{art}>>,",
    ]
    for name, _ in fields:
        rt = rust_type(name, _, key)
        opt = f"Option<{rt}>"
        body.append(f"    #[state(persistent)] pub {name}: {opt},")
    body.append("    #[state(shared_ui)] pub selected_check_index: Option<Option<u32>>,")
    take_macro = "\n        ".join(f"take!({n});" for n, _ in fields)
    apply_fields = "\n        ".join(
        f"if let Some(value) = self.{n} {{ next.{n} = value; }}" for n, _ in fields
    )
    apply_art = "\n        ".join(
        f"if let Some(value) = &self.{n} {{ next.{n} = value.clone(); }}" for n, _ in fields
    )
    content = f"""//! 🧬️ EN {key[2:]} diff schema — sparse field delta.

use {art.replace('schema::', 'schema::')} as EnArtifact;
use {snap};
use schema::ArtifactSchema;
use serde::{{Deserialize, Serialize}};

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.{key}")]
pub struct {prefix}Diff {{
{chr(10).join(body)}
}}
//#endregion 🔖️Diff
"""
    out.write_text(content)
    runtime = Path(out).parents[2] / "🦀️component.rs"
    runtime_content = f"""//! 🔺️ EN {key[2:]} artifact — sparse field diff runtime.

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use super::schema::*;

use crate::artifacts::{key}::schema::{prefix}Artifact;
use crate::artifacts::{key}::{prefix}Snapshot;
use protocol::MutationDiff;

//#region 🔖️Apply
impl {prefix}Diff {{
    pub fn apply_to_artifact(&self, artifact: &{prefix}Artifact) -> {prefix}Artifact {{
        if let Some(replacement) = &self.artifact {{
            return (**replacement).clone();
        }}
        let mut next = artifact.clone();
{apply_art}
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
{apply_fields}
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
        {take_macro}
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
    runtime.write_text(runtime_content)


def write_json_facet(key: str, prefix: str, facet: str, fields: list[tuple[str, str]], out: Path, shared_ui: bool = False) -> None:
    props = {}
    required = []
    for name, _ in fields:
        camel = snake_to_camel(name)
        props[camel] = {**json_type(name, key), "x-semio-state": "persistent"}
        required.append(camel)
    if facet == "artifact":
        props["selectedCheckIndex"] = {
            "oneOf": [{"type": "null"}, {"type": "integer"}],
            "x-semio-state": "shared-ui",
        }
    if facet == "diff":
        props = {
            "artifact": {"title": f"{prefix}Artifact", "type": "object", "x-semio-state": "persistent"},
        }
        for name, _ in fields:
            camel = snake_to_camel(name)
            props[camel] = {**json_type(name, key), "x-semio-state": "persistent"}
        props["selectedCheckIndex"] = {
            "oneOf": [{"type": "null"}, {"type": "integer"}],
            "x-semio-state": "shared-ui",
        }
        required = []
    doc = {
        "$id": f"https://semio.tech/schema/s/norm/{key}/{facet}.json",
        "title": f"{prefix}{'Artifact' if facet == 'artifact' else 'Snapshot' if facet == 'snapshot' else 'Diff'}",
        "type": "object",
        "additionalProperties": False,
        "required": required,
        "properties": props,
    }
    out.write_text(json.dumps(doc, indent=2) + "\n")


def write_ts_facet(key: str, prefix: str, facet: str, fields: list[tuple[str, str]], out: Path) -> None:
    lines = [f"/** 🧬️ EN {key[2:]} {facet} schema. */", ""]
    title = f"{prefix}{'Artifact' if facet == 'artifact' else 'Snapshot' if facet == 'snapshot' else 'Diff'}"
    lines.append(f"export interface {title} {{")
    for name, _ in fields:
        camel = snake_to_camel(name)
        opt = ""
        lines.append(f"  /** @state persistent */")
        lines.append(f"  {camel}: {ts_type(name, key)};")
    if facet == "artifact":
        lines.append("  /** @state shared-ui */")
        lines.append("  selectedCheckIndex?: number | null;")
    if facet == "diff":
        lines = [f"/** 🧬️ EN {key[2:]} diff schema. */", "", f"export interface {title} {{"]
        lines.append("  /** @state persistent */")
        lines.append(f"  artifact?: {prefix}Artifact;")
        for name, _ in fields:
            camel = snake_to_camel(name)
            lines.append("  /** @state persistent */")
            lines.append(f"  {camel}?: {ts_type(name, key)};")
        lines.append("  /** @state shared-ui */")
        lines.append("  selectedCheckIndex?: number | null | null;")
    lines.append("}")
    lines.append("")
    out.write_text("\n".join(lines))


def write_gql_facet(key: str, prefix: str, facet: str, fields: list[tuple[str, str]], out: Path) -> None:
    title = f"{prefix}{'Artifact' if facet == 'artifact' else 'Snapshot' if facet == 'snapshot' else 'Diff'}"
    lines = [f"# 🧬️ EN {key[2:]} {facet} schema.", "", f"type {title} {{"]
    for name, _ in fields:
        camel = snake_to_camel(name)
        gt = gql_type(name, key)
        if facet == "diff":
            gt = gt.replace("!", "")
        lines.append(f"  {camel}: {gt} @state(class: PERSISTENT)")
    if facet == "artifact":
        lines.append("  selectedCheckIndex: Int @state(class: SHARED_UI)")
    if facet == "diff":
        lines = [f"# 🧬️ EN {key[2:]} diff schema.", "", f"type {title} {{"]
        lines.append(f"  artifact: {prefix}Artifact @state(class: PERSISTENT)")
        for name, _ in fields:
            camel = snake_to_camel(name)
            gt = gql_type(name, key).replace("!", "")
            lines.append(f"  {camel}: {gt} @state(class: PERSISTENT)")
        lines.append("  selectedCheckIndex: Int @state(class: SHARED_UI)")
    lines.append("}")
    lines.append("")
    out.write_text("\n".join(lines))


def write_proto_facet(key: str, prefix: str, facet: str, fields: list[tuple[str, str]], out: Path) -> None:
    title = f"{prefix}{'Artifact' if facet == 'artifact' else 'Snapshot' if facet == 'snapshot' else 'Diff'}"
    pkg = f"semio.s.norm.{key}.{facet}"
    lines = [
        f"// 🧬️ EN {key[2:]} {facet} schema.",
        f"syntax = \"proto3\";",
        f"package {pkg};",
        "",
        f"message {title} {{",
    ]
    idx = 1
    for name, _ in fields:
        lines.append(f"  // @state persistent")
        lines.append(f"  {proto_type(name, key)} {name} = {idx};")
        idx += 1
    if facet == "artifact":
        lines.append("  // @state shared-ui")
        lines.append(f"  optional uint32 selected_check_index = {idx};")
    if facet == "diff":
        lines = [
            f"// 🧬️ EN {key[2:]} diff schema.",
            f"syntax = \"proto3\";",
            f"package {pkg};",
            "",
            f"message {title} {{",
            "  // @state persistent",
            f"  {prefix}Artifact artifact = 1;",
        ]
        idx = 2
        for name, _ in fields:
            pt = proto_type(name, key)
            lines.append(f"  // @state persistent")
            lines.append(f"  optional {pt} {name} = {idx};")
            idx += 1
        lines.append("  // @state shared-ui")
        lines.append(f"  optional uint32 selected_check_index = {idx};")
    lines.append("}")
    lines.append("")
    out.write_text("\n".join(lines))


def main() -> None:
    for key, prefix, emoji in ARTIFACTS:
        art_dir = ROOT / f"📘️{key}"
        comp = (art_dir / "🦀️component.rs").read_text()
        fields = parse_document_fields(comp)
        for facet in ("artifact", "snapshot", "diff"):
            base = art_dir / ("🧬️schema" if facet == "artifact" else f"📸️snapshot/🧬️schema" if facet == "snapshot" else "🔺️diff/🧬️schema")
            base.mkdir(parents=True, exist_ok=True)
            if facet == "snapshot":
                write_snapshot_rust(key, prefix, emoji, fields, base / "🦀️component.rs")
            elif facet == "artifact":
                write_artifact_rust(key, prefix, emoji, fields, base / "🦀️component.rs")
            else:
                write_diff_rust(key, prefix, fields, base / "🦀️component.rs")
            write_json_facet(key, prefix, facet, fields, base / "🔣️component.json")
            write_ts_facet(key, prefix, facet, fields, base / "🟦️component.ts")
            write_gql_facet(key, prefix, facet, fields, base / "🔗️component.graphql")
            write_proto_facet(key, prefix, facet, fields, base / "🛰️component.proto")
        print(key, len(fields), "fields")


if __name__ == "__main__":
    main()
