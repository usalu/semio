//! 🚪️ fem3d ← txt — foreign `Deserializer<Fem3dSnapshot>` on the framework's `io_mechanism`
//! channel: `store::ArtifactDsl::parse_dsl` on this subset's own `.semio` DSL snapshot text, the
//! exact inverse of the sibling `📤️export` leaf (`IoFidelity::Exact`).
//!
//! 🐛️ Repaired here (ticket 26/09/06/FEM-PLUGIN-END-TO-END, W4): this file used to be an
//! `Err("txt import not yet implemented")` stub left by a copy-paste of stdio's own json↔txt bridge.

use crate::artifacts::fem3d::Fem3dSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Confidence, Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

/// 🎯️ The foreign dialect this leaf reads.
pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

/// 📄️ The `.semio` text preamble every `fem.fem3d` document opens with — the sniff anchor, and the
/// exact first line of `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`.
pub const DSL_PREAMBLE: &str = "semio fem.fem3d.dsl ";

/// 🔤️ Parses `.semio` DSL text into this subset's snapshot — also used by the `📊️csv`/`📝️md`
/// envelope leaves, which carry this exact text as their payload.
pub fn from_dsl_text(text: &str) -> Result<Fem3dSnapshot, IoError> {
    <Fem3dSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| IoError { message: format!("txt→fem3d: {error}"), diagnostics: Vec::new() })
}

/// 🧩️ `s.stdio.txt@utf-8/*` → `s.fem.fem3d@1/*`.
pub struct TxtIntoFem3d;

impl Deserializer<Fem3dSnapshot> for TxtIntoFem3d {
    const FROM: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn sniff(payload: &IoPayload) -> Confidence {
        match payload {
            IoPayload::Text(text) if text.starts_with(DSL_PREAMBLE) => Confidence::High,
            _ => Confidence::None,
        }
    }
    async fn deserialize(payload: &IoPayload) -> IoResult<Fem3dSnapshot> {
        let IoPayload::Text(text) = payload else {
            return Err(IoError { message: "txt→fem3d: expected a text utf-8 payload".to_string(), diagnostics: Vec::new() });
        };
        Ok(IoOutcome::clean(from_dsl_text(text)?))
    }
}
