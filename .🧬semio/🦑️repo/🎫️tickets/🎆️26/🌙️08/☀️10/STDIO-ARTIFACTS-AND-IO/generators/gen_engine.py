#!/usr/bin/env python3
# -*- coding: utf-8 -*-
from pathlib import Path
import json

TICKET = list(Path(".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))[0]
TOKENS = json.loads((TICKET / "🧪tokens.json").read_text())
ROSTER = json.loads((TICKET / "🧪owner-table.json").read_text())["stdio_roster"]
PLUGIN = Path("✏️s/🔌️plugins") / TOKENS["stdio_plugin"]
RS = next(
    p.name
    for p in (PLUGIN / "🗿️artifacts" / ROSTER["binary"]["dir"] / "🧬️schema").iterdir()
    if p.name.endswith("component.rs")
)

for mid, name, ext in [
    ("binary", "Binary", "bin"),
    ("txt", "Txt", "txt"),
    ("json", "Json", "json"),
]:
    art = name + "Artifact"
    snap = name + "Snapshot"
    diff = name + "Diff"
    mut = name + "Mutation"
    eng = name + "Engine"
    empty = "empty_" + mid + "_snapshot"
    doc = "STDIO_" + mid.upper() + "_DOCUMENT_SCHEMA"
    kind = "stdio." + mid
    sid = "s.stdio." + mid
    base = PLUGIN / "🗿️artifacts" / ROSTER[mid]["dir"] / "⚙️engine"
    L = []
    a = L.append
    a("//! ⚙️ " + eng + " — owns a real `" + art + "`.")
    a("")
    a(
        "use crate::artifacts::"
        + mid
        + "::{"
        + art
        + ", "
        + diff
        + ", "
        + mut
        + ", "
        + snap
        + ", "
        + doc
        + "};"
    )
    a("")
    a("//#region 🔖️DocumentHelpers")
    a("/// 🌱 Empty persisted snapshot.")
    a("pub fn " + empty + "() -> " + snap + " {")
    a("    " + snap + "::default()")
    a("}")
    a("//#endregion 🔖️DocumentHelpers")
    a("")
    a("//#region 🔖️Register")
    a("/// 🗂️ Registers codecs and the artifact schema descriptor.")
    a("pub fn register() {")
    a("    crate::artifacts::" + mid + "::io::register();")
    a("    register_artifact_schema();")
    a("    register_pilot_languages();")
    a(
        "    store::register_document_codec(store::DocumentCodec::of::<"
        + snap
        + ", "
        + mut
        + ">("
        + doc
        + "));"
    )
    a("}")
    a("")
    a("/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary).")
    a("pub fn register_pilot_languages() {")
    a("    dsl::register_language(dsl::LanguageSpec {")
    a('        id: "' + kind + '",')
    a('        extension: Some("' + ext + '"),')
    a("        role: dsl::LanguageRole::Document,")
    a(
        "        grammar: Some(crate::artifacts::"
        + mid
        + "::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),"
    )
    a(
        "        grammar_path: Some(crate::artifacts::"
        + mid
        + "::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),"
    )
    a(
        "        protocol: Some(crate::artifacts::"
        + mid
        + "::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),"
    )
    a(
        "        protocol_path: Some(crate::artifacts::"
        + mid
        + "::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),"
    )
    a('        hooks: dsl::passthrough_hooks("' + kind + '"),')
    a("    });")
    a("}")
    a("")
    a("/// 📌️ Registers schema leaves for `" + sid + "`.")
    a("pub fn register_artifact_schema() {")
    a(
        "    ::schema::register_artifact_schema_descriptor(crate::artifacts::"
        + mid
        + "::schema::"
        + mid
        + "_artifact_schema_descriptor());"
    )
    a("}")
    a("//#endregion 🔖️Register")
    a("")
    a("//#region 🔖️ArtifactEngine")
    a("/// ⚙️ `" + kind + "` artifact engine.")
    a("pub struct " + eng + " {")
    a("    artifact_state: " + art + ",")
    a("    snapshot_state: " + snap + ",")
    a("}")
    a("")
    a("impl " + eng + " {")
    a("    /// 🏗️ Builds an engine from a persisted snapshot.")
    a("    pub fn new(snapshot: " + snap + ") -> Self {")
    a("        let artifact_state = " + art + "::from_snapshot(snapshot.clone());")
    a("        Self { artifact_state, snapshot_state: snapshot }")
    a("    }")
    a("}")
    a("")
    a("impl protocol::ArtifactEngine for " + eng + " {")
    a("    type Artifact = " + art + ";")
    a("    type Snapshot = " + snap + ";")
    a("    type Mutation = " + mut + ";")
    a("    type Diff = " + diff + ";")
    a("")
    a("    fn artifact(&self) -> &Self::Artifact {")
    a("        &self.artifact_state")
    a("    }")
    a("")
    a("    fn snapshot(&self) -> &Self::Snapshot {")
    a("        &self.snapshot_state")
    a("    }")
    a("")
    a(
        "    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {"
    )
    a(
        "        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(mutation, &self.snapshot_state);"
    )
    a(
        "        self.snapshot_state = <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(&diff, &self.snapshot_state);"
    )
    a("        self.artifact_state.set_snapshot(self.snapshot_state.clone());")
    a("        Ok(diff)")
    a("    }")
    a("")
    a("    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {")
    a(
        "        <Self::Mutation as protocol::Mutation<Self::Snapshot>>::inverse(mutation, &self.snapshot_state)"
    )
    a("    }")
    a("}")
    a("//#endregion 🔖️ArtifactEngine")
    a("")
    a("//#region 🧪️Tests")
    a("#[cfg(test)]")
    a("mod tests {")
    a("    use super::*;")
    a("")
    a("    #[test]")
    a("    fn empty_snapshot_matches_schema() {")
    a("        let snapshot = " + empty + "();")
    a("        assert_eq!(snapshot.schema, " + doc + ");")
    a("    }")
    a("")
    a("    #[test]")
    a("    fn codec_round_trip() {")
    a("        let snap = " + empty + "();")
    a("        let text = store::DocumentDsl::print_dsl(&snap);")
    a(
        "        let parsed = <"
        + snap
        + " as store::DocumentDsl>::parse_dsl(&text).expect(\"parse\");"
    )
    a("        assert_eq!(parsed.schema, snap.schema);")
    a("        let bytes = store::DocumentPack::encode_pack(&snap);")
    a(
        "        let decoded = <"
        + snap
        + " as store::DocumentPack>::decode_pack(&bytes).expect(\"decode\");"
    )
    a("        assert_eq!(decoded, snap);")
    a("    }")
    a("}")
    a("//#endregion 🧪️Tests")
    out = "\n".join(L) + "\n"
    assert "\ufffd" not in out
    base.mkdir(parents=True, exist_ok=True)
    (base / RS).write_text(out)
    print("engine", mid)
