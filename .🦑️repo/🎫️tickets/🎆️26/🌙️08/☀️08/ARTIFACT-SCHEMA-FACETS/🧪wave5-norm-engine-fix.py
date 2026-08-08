#!/usr/bin/env python3
from pathlib import Path
import re

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
KEYS = ["en1990", "en1991", "en1992", "en1993", "en1994"]

for key in KEYS:
    pre = f"En{key[2:]}"
    snap = f"{pre}Snapshot"
    path = ROOT / f"📘️{key}" / "⚙️engine/🦀️component.rs"
    text = path.read_text(encoding="utf-8")
    text = text.replace("diff::Diff", f"diff::{pre}Diff")
    engine_impl = f"""//#region 🔖️ArtifactEngine
/// @emoji ⚙️ UI-independent {pre} artifact engine — owns the artifact; every transition is a mutation.
pub struct {pre}Engine {{
    artifact: crate::artifacts::{key}::schema::{pre}Artifact,
    snapshot: {snap},
}}

impl {pre}Engine {{
    pub fn new(snapshot: {snap}) -> Self {{
        let artifact = crate::artifacts::{key}::schema::{pre}Artifact::from_snapshot(snapshot.clone());
        Self {{ artifact, snapshot }}
    }}

    pub fn into_snapshot(self) -> {snap} {{
        self.snapshot
    }}
}}

impl protocol::ArtifactEngine for {pre}Engine {{
    type Artifact = crate::artifacts::{key}::schema::{pre}Artifact;
    type Snapshot = {snap};
    type Mutation = crate::artifacts::{key}::mutations::{pre}Mutation;
    type Diff = crate::artifacts::{key}::diff::{pre}Diff;

    fn artifact(&self) -> &Self::Artifact {{
        &self.artifact
    }}

    fn snapshot(&self) -> &Self::Snapshot {{
        &self.snapshot
    }}

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {{
        let diff = protocol::Mutation::diff(mutation, &self.snapshot);
        self.snapshot = vcs::apply_mutation(&self.snapshot, mutation);
        self.artifact = crate::artifacts::{key}::schema::{pre}Artifact::from_snapshot(self.snapshot.clone());
        Ok(diff)
    }}

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {{
        protocol::Mutation::inverse(mutation, &self.snapshot)
    }}
}}
//#endregion 🔖️ArtifactEngine"""
    text = re.sub(
        r"//#region 🔖️ArtifactEngine[\s\S]*?//#endregion 🔖️ArtifactEngine",
        engine_impl,
        text,
        count=1,
    )
    text = re.sub(
        rf"type Document = {snap};",
        f"type Document = {snap};",
        text,
    )
  # fix NormFamily evaluate signature if broken
    text = text.replace(f"fn evaluate(document: &Document)", f"fn evaluate(document: &{snap})")
    path.write_text(text, encoding="utf-8")
    print("engine", key)
