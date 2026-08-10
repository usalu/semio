#!/usr/bin/env python3
# -*- coding: utf-8 -*-
from pathlib import Path
import json

TICKET = list(Path(".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))[0]
TOKENS = json.loads((TICKET / "🧪tokens.json").read_text())
ROSTER = json.loads((TICKET / "🧪owner-table.json").read_text())["stdio_roster"]
PLUGIN = Path("✏️s/🔌️plugins") / TOKENS["stdio_plugin"]

for mid, name, field, fty in [
    ("binary", "Binary", "bytes", "Vec<u8>"),
    ("txt", "Txt", "text", "String"),
    ("json", "Json", "value", "serde_json::Value"),
]:
    schema_dir = PLUGIN / "🗿️artifacts" / ROSTER[mid]["dir"] / "🧬️schema"
    files = [p.name for p in schema_dir.iterdir() if p.is_file()]
    rs = next(n for n in files if n.endswith("component.rs"))
    ts = next(n for n in files if n.endswith("component.ts"))
    gql = next(n for n in files if n.endswith("component.graphql"))
    jsch = next(n for n in files if n.endswith("component.json"))
    proto = next(n for n in files if n.endswith("component.proto"))
    art = name + "Artifact"
    snap = name + "Snapshot"
    sid = "s.stdio." + mid
    kind = "stdio." + mid
    L = []
    a = L.append
    a("//! 🧬️ " + art + " schema — full artifact state.")
    a("")
    a("use crate::artifacts::" + mid + "::{" + snap + "};")
    a("use schema::ArtifactSchema;")
    a("use serde::{Deserialize, Serialize};")
    a("")
    a("//#region 🔖️Artifact")
    a("/// 🧬️ Full `" + kind + "` artifact state.")
    a("#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]")
    a('#[serde(rename_all = "camelCase")]')
    a('#[artifact_schema(id = "' + sid + '")]')
    a("pub struct " + art + " {")
    a("    #[state(persistent)]")
    a("    pub schema: String,")
    a("    #[state(persistent)]")
    a("    #[serde(default)]")
    a("    pub " + field + ": " + fty + ",")
    a("}")
    a("//#endregion 🔖️Artifact")
    a("")
    a("//#region 🔖️Conversions")
    a("impl Default for " + art + " {")
    a("    fn default() -> Self {")
    a("        Self::from_snapshot(" + snap + "::default())")
    a("    }")
    a("}")
    a("")
    a("impl " + art + " {")
    a("    /// 📸️ Persisted subset.")
    a("    pub fn to_snapshot(&self) -> " + snap + " {")
    a("        " + snap + " {")
    a("            schema: self.schema.clone(),")
    a("            " + field + ": self." + field + ".clone(),")
    a("        }")
    a("    }")
    a("")
    a("    /// 🧬️ Builds a full artifact from a snapshot.")
    a("    pub fn from_snapshot(snapshot: " + snap + ") -> Self {")
    a("        Self {")
    a("            schema: snapshot.schema,")
    a("            " + field + ": snapshot." + field + ",")
    a("        }")
    a("    }")
    a("")
    a("    /// 🔄 Writes persistent fields from a snapshot into this artifact.")
    a("    pub fn set_snapshot(&mut self, snapshot: " + snap + ") {")
    a("        self.schema = snapshot.schema;")
    a("        self." + field + " = snapshot." + field + ";")
    a("    }")
    a("}")
    a("//#endregion 🔖️Conversions")
    a("")
    a("//#region 🔖️Descriptor")
    a("/// 🧬️ Descriptor for `" + sid + "`.")
    a("pub fn " + mid + "_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {")
    a("    schema::ArtifactSchemaDescriptor {")
    a('        id: "' + sid + '",')
    a("        artifact: schema::FacetLeaves {")
    a('            rust: include_str!("' + rs + '"),')
    a('            typescript: include_str!("' + ts + '"),')
    a('            graphql: include_str!("' + gql + '"),')
    a('            json_schema: include_str!("' + jsch + '"),')
    a('            proto: include_str!("' + proto + '"),')
    a("        },")
    a("        snapshot: schema::FacetLeaves {")
    a('            rust: include_str!("📸️snapshot/' + rs + '"),')
    a('            rust: include_str!("📸️snapshot/' + rs + '"),')
    a('            typescript: include_str!("📸️snapshot/' + ts + '"),')
    a('            graphql: include_str!("📸️snapshot/' + gql + '"),')
    a('            json_schema: include_str!("📸️snapshot/' + jsch + '"),')
    a('            proto: include_str!("📸️snapshot/' + proto + '"),')
    a("        },")
    a("        diff: schema::FacetLeaves {")
    a('            rust: include_str!("🔺️diff/' + rs + '"),')
    a('            typescript: include_str!("🔺️diff/' + ts + '"),')
    a('            graphql: include_str!("🔺️diff/' + gql + '"),')
    a('            json_schema: include_str!("🔺️diff/' + jsch + '"),')
    a('            proto: include_str!("🔺️diff/' + proto + '"),')
    a("        },")
    a("    }")
    a("}")
    a("//#endregion 🔖️Descriptor")
    out = "\n".join(L) + "\n"
    assert "\ufffd" not in out
    (schema_dir / rs).write_text(out)
    print("rewrote", mid)
