#!/usr/bin/env python3
"""🎚️ Completes the tree-wide window-config migration that added
`store::mounted_pack_rt::DslField` to `WindowConfigOwner::State`.

For one `*WindowConfig` it

1. adds `dsl::DslArtifact` to the struct's derive list plus an explicit
   `#[dsl(id = ..., extension = ..., layout = "lines")]` reproducing the envelope identity the
   hand-written `ArtifactDsl` already carried,
2. marks the named nested-record fields `#[dsl(block)]`, and
3. replaces the hand-written `impl store::ArtifactDsl` / `impl store::ArtifactPack` bodies with the
   record-backed pair every already-migrated sibling uses, `record_spec` included — the retained
   window-config pack loader reads that spec and fails the kind with `TypedState` without it.

Run from the repo root: `python3 <this file> <target-key>...` (no argument = every target).
"""

from __future__ import annotations

import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[7]

DSL_IMPL = '''/// 📜️ Record-backed text form — the derived `__dsl_spec` grammar inside this window kind's semio
/// text envelope, the same shape every sibling window config prints.
impl store::ArtifactDsl for {struct} {{
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {{
        Self::__DSL_ENVELOPE_ID
    }}
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {{
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions {{ limits: dsl::Limits::default(), mode: dsl::SourceMode::Document }})?;
        Self::__dsl_from_record(&record)
    }}
    fn print_dsl(&self) -> String {{
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("{label}");
        store::semio_format::wrap_text(&envelope, &body)
    }}
}}'''

PACK_IMPL = '''/// 🎒️ Record-backed pack form. `record_spec` is what the retained window-config loader reads to
/// decode a mounted pack field-by-field; returning `None` here would fail every retained load of
/// this window kind with `WindowConfigPackLoadDiagnostic::TypedState`.
impl store::ArtifactPack for {struct} {{
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {{
        let body = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }}
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {{
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {{
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {{}}.pack v1, got {{}}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }}
        let (record, _) = store::pack_rt::decode_document(&body, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }}
    fn record_spec() -> Option<dsl::RecordSpec> {{
        Some(Self::__dsl_spec())
    }}
}}'''


def block_end(text: str, start: int) -> int:
    depth = 0
    index = text.index("{", start)
    while index < len(text):
        if text[index] == "{":
            depth += 1
        elif text[index] == "}":
            depth -= 1
            if depth == 0:
                return index + 1
        index += 1
    raise SystemExit(f"unbalanced block from offset {start}")


def replace_impl(text: str, trait_name: str, struct: str, body: str) -> str:
    marker = f"impl store::{trait_name} for {struct} {{"
    start = text.index(marker)
    return text[:start] + body + text[block_end(text, start):]


def replace_json_store(text: str, struct: str, body: str) -> str:
    """🧹 Owners whose codecs came from a local `json_store!` shorthand lose that invocation instead."""
    start = text.index(f"json_store!({struct},")
    return text[:start] + body + text[text.index("\n", start):]


def derive_artifact(text: str, struct: str, envelope_id: str, extension: str) -> str:
    marker = f"pub struct {struct} {{"
    struct_at = text.index(marker)
    derive_at = text.rindex("\n#[derive(", 0, struct_at) + 1
    derive_end = text.index(")]", derive_at) + 2
    derive = text[derive_at:derive_end]
    if "DslArtifact" not in derive:
        derive = derive[: derive.rindex(")]")] + ", dsl::DslArtifact)]"
    attribute = f'#[dsl(id = "{envelope_id}", extension = "{extension}", layout = "lines")]\n'
    return text[:derive_at] + derive + text[derive_end:struct_at] + attribute + text[struct_at:]


def block_fields(text: str, struct: str, fields: list[str]) -> str:
    for field in fields:
        marker = f"    pub {field}:"
        start = text.index(marker, text.index(f"pub struct {struct} {{"))
        text = text[:start] + "    #[dsl(block)]\n" + text[start:]
    return text


TARGETS = {
    "sequence": dict(
        path="✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📽️main/🎚️config/🦀️.rs",
        struct="SequenceMainWindowConfig",
        envelope_id="s.sequence.sequence.mainwindowconfig",
        extension="sequencemainwindowcfg",
        label="valid Sequence window envelope",
        blocks=["camera"],
    ),
    "forms": dict(
        path="✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🎚️config/🦀️.rs",
        struct="FormsTryWindowConfig",
        envelope_id="s.forms.forms.try-window-config",
        extension="formstrywindowcfg",
        label="valid Forms Try window config envelope",
        blocks=[],
    ),
    "layout": dict(
        path="✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📐️blueprint/🎚️config/🦀️.rs",
        struct="LayoutWindowConfig",
        envelope_id="s.layout.layout.windowconfig",
        extension="layoutwindowcfg",
        label="valid Layout window envelope",
        blocks=["camera"],
    ),
    "note": dict(
        path="✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs",
        struct="NoteCompositeWindowConfig",
        envelope_id="s.note.note.compositewindowconfig",
        extension="notecompositewindowcfg",
        label="valid Note composite window envelope",
        blocks=["camera"],
    ),
    "draw": dict(
        path="✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🎚️config/🦀️.rs",
        struct="DrawingCanvasWindowConfig",
        envelope_id="s.draw.drawing.canvas-window.config",
        extension="drawingcanvaswindowcfg",
        label="valid Drawing canvas window envelope",
        blocks=["viewport"],
    ),
    "remodel-frames": dict(
        path="✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📷️capture/🪟️windows/🖼️frames/🎚️config/🦀️.rs",
        struct="RemodelingFramesWindowConfig",
        envelope_id="s.remodel.remodeling.frameswindowconfig",
        extension="remodelingframeswindowcfg",
        label="valid Remodeling frames window envelope",
        blocks=["frame_cursor"],
    ),
    "remodel-report": dict(
        path="✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🔍️analyze/🪟️windows/📊️report/🎚️config/🦀️.rs",
        struct="RemodelingReportWindowConfig",
        envelope_id="s.remodel.remodeling.reportwindowconfig",
        extension="remodelingreportwindowcfg",
        label="valid Remodeling report window envelope",
        blocks=[],
    ),
    "remodel-model": dict(
        path="✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧊️model/🪟️windows/🧊️model/🎚️config/🦀️.rs",
        struct="RemodelingModelWindowConfig",
        envelope_id="s.remodel.remodeling.modelwindowconfig",
        extension="remodelingmodelwindowcfg",
        label="valid Remodeling model window envelope",
        blocks=["camera", "layers"],
    ),
    "architect-adjacency": dict(
        path="✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/↔️adjacency/🎚️config/🦀️.rs",
        struct="ArchitectAdjacencyWindowConfig",
        envelope_id="s.architect.program.adjacency-window.config",
        extension="architectadjacencywindowcfg",
        label="valid Architect adjacency window envelope",
        blocks=[],
    ),
    "architect-register": dict(
        path="✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📋️register/🎚️config/🦀️.rs",
        struct="ArchitectRegisterWindowConfig",
        envelope_id="s.architect.program.register-window.config",
        extension="architectregisterwindowcfg",
        label="valid Architect register window envelope",
        blocks=[],
    ),
    "architect-report": dict(
        path="✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📓️report/🎚️config/🦀️.rs",
        struct="ArchitectReportWindowConfig",
        envelope_id="s.architect.program.report-window.config",
        extension="architectreportwindowcfg",
        label="valid Architect report window envelope",
        blocks=[],
    ),
    "architect-graph": dict(
        path="✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🎚️config/🦀️.rs",
        struct="ArchitectGraphWindowConfig",
        envelope_id="s.architect.program.graph-window.config",
        extension="architectgraphwindowcfg",
        label="valid Architect graph window envelope",
        blocks=["viewport"],
    ),
}


def apply(key: str) -> None:
    target = TARGETS[key]
    path = ROOT / target["path"]
    text = path.read_text(encoding="utf-8")
    struct = target["struct"]
    text = derive_artifact(text, struct, target["envelope_id"], target["extension"])
    text = block_fields(text, struct, target["blocks"])
    dsl_impl = DSL_IMPL.format(struct=struct, label=target["label"])
    pack_impl = PACK_IMPL.format(struct=struct)
    if f"impl store::ArtifactDsl for {struct} {{" in text:
        text = replace_impl(text, "ArtifactDsl", struct, dsl_impl)
        text = replace_impl(text, "ArtifactPack", struct, pack_impl)
    else:
        text = replace_json_store(text, struct, f"{dsl_impl}\n\n{pack_impl}")
    path.write_text(text, encoding="utf-8")
    print(f"{key}: {struct} forwarded")


if __name__ == "__main__":
    for key in sys.argv[1:] or TARGETS:
        apply(key)
