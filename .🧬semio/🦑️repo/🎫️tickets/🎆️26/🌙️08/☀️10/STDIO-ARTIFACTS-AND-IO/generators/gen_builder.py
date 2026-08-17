#!/usr/bin/env python3
# -*- coding: utf-8 -*-
from pathlib import Path
import json

TICKET = list(Path(".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))[0]
TOKENS = json.loads((TICKET / "🧪tokens.json").read_text())
ROSTER = json.loads((TICKET / "🧪owner-table.json").read_text())["stdio_roster"]
PLUGIN = Path("✏️s/🔌️plugins") / TOKENS["stdio_plugin"]
BUILDER = TOKENS["builder"]
DECOMPOSER = TOKENS["decomposer"]
schema_dir = PLUGIN / "🗿️artifacts" / ROSTER["binary"]["dir"] / "🧬️schema"
RS = next(p.name for p in schema_dir.iterdir() if p.name.endswith("component.rs"))
TS = next(p.name for p in schema_dir.iterdir() if p.name.endswith("component.ts"))

SPECS = [("binary", "Binary"), ("txt", "Txt"), ("json", "Json")]

for mid, name in SPECS:
    snap = name + "Snapshot"
    diff = name + "Diff"
    mut = name + "Mutation"
    bld = name + "Builder"
    kind = "stdio." + mid
    base = PLUGIN / "🗿️artifacts" / ROSTER[mid]["dir"] / BUILDER
    base.mkdir(parents=True, exist_ok=True)
    text = "\n".join([
        "//! 🏗️ " + bld + " — local ArtifactBuilder until SDK Wave 3.",
        "",
        "use crate::artifacts::" + mid + "::{" + diff + ", " + mut + ", " + snap + "};",
        "",
        "//#region 🔖️LocalContracts",
        "/// 🏗️ Local builder contract (W3 swaps to SDK `ArtifactBuilder`).",
        "pub trait ArtifactBuilder: Sized {",
        "    type Snapshot;",
        "    type Mutation;",
        "    type Diff;",
        "    fn empty() -> Self;",
        "    fn from_snapshot(snapshot: Self::Snapshot) -> Self;",
        "    fn from_text(text: &str) -> Result<Self, store::TextError>;",
        "    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError>;",
        "    fn mutate(self, mutation: Self::Mutation) -> Self;",
        "    fn absorb(self, diff: Self::Diff) -> Self;",
        "    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>>;",
        "}",
        "//#endregion 🔖️LocalContracts",
        "",
        "//#region 🔖️Builder",
        "/// 🏗️ Builds a `" + kind + "` snapshot.",
        "#[derive(Clone, Debug, Default)]",
        "pub struct " + bld + " {",
        "    snapshot: " + snap + ",",
        "    diagnostics: Vec<dsl::Diagnostic>,",
        "}",
        "",
        "impl ArtifactBuilder for " + bld + " {",
        "    type Snapshot = " + snap + ";",
        "    type Mutation = " + mut + ";",
        "    type Diff = " + diff + ";",
        "    fn empty() -> Self {",
        "        Self { snapshot: " + snap + "::default(), diagnostics: Vec::new() }",
        "    }",
        "    fn from_snapshot(snapshot: Self::Snapshot) -> Self {",
        "        Self { snapshot, diagnostics: Vec::new() }",
        "    }",
        "    fn from_text(text: &str) -> Result<Self, store::TextError> {",
        "        Ok(Self::from_snapshot(<" + snap + " as store::DocumentDsl>::parse_dsl(text)?))",
        "    }",
        "    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {",
        "        Ok(Self::from_snapshot(<" + snap + " as store::DocumentPack>::decode_pack(bytes)?))",
        "    }",
        "    fn mutate(mut self, mutation: Self::Mutation) -> Self {",
        "        crate::artifacts::" + mid + "::schema::mutations::apply_" + mid + "_mutation(&mut self.snapshot, &mutation);",
        "        self",
        "    }",
        "    fn absorb(mut self, diff: Self::Diff) -> Self {",
        "        self.snapshot = <" + diff + " as protocol::MutationDiff<" + snap + ">>::apply(&diff, &self.snapshot);",
        "        self",
        "    }",
        "    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {",
        "        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }",
        "    }",
        "}",
        "//#endregion 🔖️Builder",
        "",
    ])
    assert "\ufffd" not in text
    (base / RS).write_text(text)
    (base / TS).write_text(
        "/** 🏗️ " + bld + " facade (local until SDK W3). */\n"
        "export interface " + bld + " { build(): { schema: string }; }\n"
    )
    print("builder", mid)

for mid, name in SPECS:
    snap = name + "Snapshot"
    dec = name + "Decomposer"
    parts = name + "Parts"
    kind = "stdio." + mid
    base = PLUGIN / "🗿️artifacts" / ROSTER[mid]["dir"] / DECOMPOSER
    base.mkdir(parents=True, exist_ok=True)
    text = "\n".join([
        "//! 📑️ " + dec + " — local ArtifactDecomposer until SDK Wave 3.",
        "",
        "use crate::artifacts::" + mid + "::{" + snap + "};",
        "",
        "//#region 🔖️LocalContracts",
        "/// 🎚 Soft confidence for partial decomposition success.",
        "#[derive(Clone, Copy, Debug, PartialEq, Eq)]",
        "pub enum Confidence { High, Medium, Low }",
        "",
        "/// 📥 One decomposition source blob.",
        "#[derive(Clone, Debug)]",
        "pub enum DecomposeSource<'a> { Text(&'a str), Binary(&'a [u8]) }",
        "",
        "/// 📦 Decomposition result carrying soft diagnostics.",
        "#[derive(Clone, Debug)]",
        "pub struct Decomposition<T> {",
        "    pub parts: T,",
        "    pub confidence: Confidence,",
        "    pub diagnostics: Vec<dsl::Diagnostic>,",
        "}",
        "",
        "/// 📑️ Local decomposer contract (W3 swaps to SDK `ArtifactDecomposer`).",
        "pub trait ArtifactDecomposer: Sized {",
        "    type Snapshot;",
        "    type Parts;",
        "    fn decompose(sources: &[DecomposeSource]) -> Decomposition<Self::Parts>;",
        "}",
        "//#endregion 🔖️LocalContracts",
        "",
        "//#region 🔖️Parts",
        "/// 🧩 Decomposed `" + kind + "` parts.",
        "#[derive(Clone, Debug, Default)]",
        "pub struct " + parts + " { pub snapshot: Option<" + snap + ">, }",
        "//#endregion 🔖️Parts",
        "",
        "//#region 🔖️Decomposer",
        "/// 📑️ Decomposes `" + kind + "` sources.",
        "pub struct " + dec + ";",
        "",
        "impl ArtifactDecomposer for " + dec + " {",
        "    type Snapshot = " + snap + ";",
        "    type Parts = " + parts + ";",
        "    fn decompose(sources: &[DecomposeSource]) -> Decomposition<Self::Parts> {",
        "        let mut parts = " + parts + "::default();",
        "        let mut diagnostics = Vec::new();",
        "        let mut confidence = Confidence::High;",
        "        for source in sources {",
        "            match source {",
        "                DecomposeSource::Text(text) => match <" + snap + " as store::DocumentDsl>::parse_dsl(text) {",
        "                    Ok(snapshot) => parts.snapshot = Some(snapshot),",
        "                    Err(err) => {",
        "                        confidence = Confidence::Low;",
        "                        diagnostics.push(dsl::Diagnostic::error(",
        '                            "stdio.decompose.text",',
        "                            dsl::TextSpan::at(1, 1),",
        "                            err.to_string(),",
        "                        ));",
        "                    }",
        "                },",
        "                DecomposeSource::Binary(bytes) => match <" + snap + " as store::DocumentPack>::decode_pack(bytes) {",
        "                    Ok(snapshot) => parts.snapshot = Some(snapshot),",
        "                    Err(err) => {",
        "                        confidence = Confidence::Low;",
        "                        diagnostics.push(dsl::Diagnostic::error(",
        '                            "stdio.decompose.binary",',
        "                            dsl::TextSpan::at(1, 1),",
        "                            err.to_string(),",
        "                        ));",
        "                    }",
        "                },",
        "            }",
        "        }",
        "        Decomposition { parts, confidence, diagnostics }",
        "    }",
        "}",
        "//#endregion 🔖️Decomposer",
        "",
    ])
    assert "\ufffd" not in text
    (base / RS).write_text(text)
    (base / TS).write_text(
        "/** 📑️ " + dec + " facade (local until SDK W3). */\n"
        "export interface Decomposition<T> {\n"
        "  parts: T;\n"
        "  confidence: 'high' | 'medium' | 'low';\n"
        "  diagnostics: unknown[];\n"
        "}\n"
    )
    print("decomposer", mid)
