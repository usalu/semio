//! 🚪️ fem3d ← md — foreign `Deserializer<Fem3dSnapshot>` on the framework's `io_mechanism`
//! channel, the exact inverse of the sibling `📤️export` leaf's fenced-code-block envelope
//! (`IoFidelity::Exact`). The `fem3d`-tagged fence wins; an untagged fence is accepted as a
//! fallback (documents written before the info string existed, and hand-authored ones). A markdown
//! document with no code block at all is a typed `Err` naming the reason.

use crate::artifacts::fem3d::Fem3dSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Confidence, Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_artifact_stdio_md::schema::snapshot::MdBlock;
use semio_s_artifact_stdio_md::MdSnapshot;

/// 🎯️ The foreign dialect this leaf reads.
pub const MD_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId::ANY };

/// 🏷️ The fenced block's info string the sibling exporter writes.
pub const FENCE_INFO: &str = "fem3d";

fn fenced_literal(snapshot: &MdSnapshot) -> Option<&str> {
    let tagged = snapshot.blocks.iter().find_map(|block| match block {
        MdBlock::CodeBlock { info: Some(info), literal } if info.trim() == FENCE_INFO => Some(literal.as_str()),
        _ => None,
    });
    tagged.or_else(|| {
        snapshot.blocks.iter().find_map(|block| match block {
            MdBlock::CodeBlock { info: None, literal } => Some(literal.as_str()),
            _ => None,
        })
    })
}

/// 📝️ Reads the envelope's fenced code block back into this subset's snapshot.
pub fn from_md_text(text: &str) -> Result<Fem3dSnapshot, IoError> {
    let snapshot = MdSnapshot::from_text(text);
    let literal = fenced_literal(&snapshot).ok_or_else(|| IoError { message: format!("md→fem3d: not a fem3d envelope — no `{FENCE_INFO}` fenced code block and no untagged one to fall back on"), diagnostics: Vec::new() })?;
    <Fem3dSnapshot as store::ArtifactDsl>::parse_dsl(literal).map_err(|error| IoError { message: format!("md→fem3d: {error}"), diagnostics: Vec::new() })
}

/// 🧩️ `s.stdio.md@commonmark/*` → `s.fem.fem3d@1/*`.
pub struct MdIntoFem3d;

impl Deserializer<Fem3dSnapshot> for MdIntoFem3d {
    const FROM: Dialect = MD_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn sniff(payload: &IoPayload) -> Confidence {
        match payload {
            IoPayload::Text(text) if text.contains("```fem3d") => Confidence::High,
            IoPayload::Text(text) if text.contains("```") => Confidence::Low,
            _ => Confidence::None,
        }
    }
    async fn deserialize(payload: &IoPayload) -> IoResult<Fem3dSnapshot> {
        let IoPayload::Text(text) = payload else {
            return Err(IoError { message: "md→fem3d: expected a text commonmark payload".to_string(), diagnostics: Vec::new() });
        };
        Ok(IoOutcome::clean(from_md_text(text)?))
    }
}
