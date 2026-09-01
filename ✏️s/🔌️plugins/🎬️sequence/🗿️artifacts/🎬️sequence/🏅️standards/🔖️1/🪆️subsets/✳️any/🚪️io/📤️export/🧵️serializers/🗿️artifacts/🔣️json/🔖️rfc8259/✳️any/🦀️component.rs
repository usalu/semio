//! 🚪️ sequence -> json — foreign `Serializer<SequenceSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Serializes the MATERIALIZED
//! fixture — `SequenceFixture` is this artifact's existing JSON wire contract (`SequenceHost::to_json`
//! /`load_json` speak it), so the export now speaks the shape the plugin already agreed on.
//!
//! ⚠️ `IoFidelity::Exact` IS STILL NOT UNCONDITIONALLY TRUE, and the remaining hole is not this
//! function's to close. `to_fixture()` reads the scene through
//! `sequence_working_scene_for_handle`, which is
//! `handle.local_owner::<SequenceWorkingScene>().map(..).unwrap_or_default()`. `local_owner` is
//! `ArtifactChild`'s `#[serde(skip)]` field: populated for a snapshot materialized in-process, ABSENT
//! for one decoded from bytes — and the `unwrap_or_default()` turns that absence into an EMPTY scene
//! rather than an error. So a decoded snapshot exports as `{schema, steps: [], edges: []}` while still
//! claiming `Exact`.
//!
//! That is the SAME root cause as `MathematicalIntoJson`'s (there the accessor returns `Option` and
//! the handle is exported verbatim; here it defaults to empty), and it is one defect in
//! `ArtifactChild`-backed export, not two. Fixing it means deciding where an exporter sources the
//! scene when the owner is absent — rebuild from the child, refuse with an `IoError`, or declare
//! `Lossy` honestly. See ticket 26/08/27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING,
//! `📓️one-defect-behind-all-thirteen.md`.
//!
//! 🐛️It previously serialized the snapshot directly, and the `Exact` claim was false: a
//! `SequenceSnapshot` is `{schema, content: ArtifactChild<..>}`, and `ArtifactChild` keeps its
//! materialized scene in a `#[serde(skip)]` `local_owner` (`🏪️store/🦀️component.rs:2567`) — so the
//! export emitted `{schema, content: {childId, target}}` and dropped every step and edge. The sibling
//! csv serializer in this same tree already went through `to_fixture()`; this one now does too.
//!
//! Consequence for testing: for a LIVE snapshot `edges`, `x`/`y` and `collapsed` now reach the carrier,
//! which is what `connect-steps`, `disconnect-steps`, `move-step` and `change-step-collapsed` need in
//! order to be witnessable by a third-party JSON reader at all. Registering that oracle is deliberately
//! NOT done here: the fix is unverified while `semio-s-plugin-stdio` does not build, and the decoded
//! path above still exports an empty scene, so an oracle bound to it today would be claiming a carrier
//! that is only conditionally real.

use crate::artifacts::sequence::SequenceSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::json::STDIO_JSON_DOCUMENT_SCHEMA;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct SequenceIntoJson;

impl Serializer<SequenceSnapshot> for SequenceIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    fn serialize(from: &SequenceSnapshot) -> IoResult<IoPayload> {
        let _ = STDIO_JSON_DOCUMENT_SCHEMA;
        let value = serde_json::to_value(from.to_fixture()).map_err(|error| IoError { message: format!("SequenceIntoJson: {error}"), diagnostics: Vec::new() })?;
        let bytes = serde_json::to_vec_pretty(&value).map_err(|error| IoError { message: format!("SequenceIntoJson: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Binary(bytes)))
    }
}
