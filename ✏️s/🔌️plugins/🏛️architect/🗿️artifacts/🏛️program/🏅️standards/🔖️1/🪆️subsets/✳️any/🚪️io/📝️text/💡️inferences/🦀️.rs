//! 📖️ ProgramSnapshot inference — the normative handcrafted text grammar for this facet.
//! Inference values are never authored via DSL text (they are always computed from a snapshot,
//! never a source of truth), so — unlike `📸️snapshot/📝️text`'s live `parse_dsl`/`print_dsl` pair —
//! this leaf declares the wire grammar only, matching the generic header/payload scaffold shape
//! every other representation leaf in this tree already uses for its own facet.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type ProgramInferenceText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod inferences_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::inferences::*;
use crate::ProgramSnapshot;
use framework_schema::ArtifactSchema;
use protocol::Inference;
use crate::standards::v1::subsets::any::schema::inferences::topology::compute_topology;
/// 🧭️ Dissolved out of the former `⚙️engine` topic files (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — every `fn(&ProgramSnapshot, ...) ->
/// Value`-style read-only projection that used to live on the artifact-tree engine hub. Mutating /
/// constructing counterparts (marked in each region below) moved to `crate::apps::architect`'s own
/// `//#region 🔧️Behavior` instead, since they take `&mut ProgramSnapshot`.
use crate::kernel::{DiagnosticSeverity, EntityHeader, EntityId, LifecycleStatus, PluginError, Priority, ProgramDiagnostic};
use crate::registers::{AdjacencyKind, AnalysisKind, AuditEvent, RelationshipKind, ReportKind, RiskLevel, SearchFilter, SeparationKind, ValidationStatus};
use crate::ARCHITECT_PROGRAM_SCHEMA;
use semio_s_artifact_stdio_csv as stdio_csv;
use semio_s_artifact_stdio_tsv as stdio_tsv;
use semio_s_artifact_stdio_tsv::standards::iana::subsets::any::schema::snapshot as stdio_tsv_engine;
use semio_s_artifact_stdio_tsv::standards::iana::subsets::any::schema::snapshot as stdio_tsv_line_ending;
use std::collections::{HashMap, HashSet};
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::standards::v1::subsets::any::schema::inferences::topology::ProgramTopology;

/// 📥️ Deserializes a plugin from JSON with schema validation.
pub fn import_json(json: &str) -> Result<ProgramSnapshot, PluginError> {
    let program: ProgramSnapshot = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| PluginError::Deserialize(e.to_string()))?;
    if program.schema != ARCHITECT_PROGRAM_SCHEMA {
        return Err(PluginError::InvalidSchema { expected: ARCHITECT_PROGRAM_SCHEMA.into(), actual: program.schema });
    }
    Ok(program)
}
}
pub use inferences_codec::*;

#[allow(unused_imports)]
mod inferences_wire_codec {
use crate::standards::v1::subsets::any::schema::inferences::*;
use crate::ProgramSnapshot;
use framework_schema::ArtifactSchema;
use protocol::Inference;
use crate::standards::v1::subsets::any::schema::inferences::topology::compute_topology;
/// 🧭️ Dissolved out of the former `⚙️engine` topic files (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — every `fn(&ProgramSnapshot, ...) ->
/// Value`-style read-only projection that used to live on the artifact-tree engine hub. Mutating /
/// constructing counterparts (marked in each region below) moved to `crate::apps::architect`'s own
/// `//#region 🔧️Behavior` instead, since they take `&mut ProgramSnapshot`.
use crate::kernel::{DiagnosticSeverity, EntityHeader, EntityId, LifecycleStatus, PluginError, Priority, ProgramDiagnostic};
use crate::registers::{AdjacencyKind, AnalysisKind, AuditEvent, RelationshipKind, ReportKind, RiskLevel, SearchFilter, SeparationKind, ValidationStatus};
use crate::ARCHITECT_PROGRAM_SCHEMA;
use semio_s_artifact_stdio_csv as stdio_csv;
use semio_s_artifact_stdio_tsv as stdio_tsv;
use semio_s_artifact_stdio_tsv::standards::iana::subsets::any::schema::snapshot as stdio_tsv_engine;
use semio_s_artifact_stdio_tsv::standards::iana::subsets::any::schema::snapshot as stdio_tsv_line_ending;
use std::collections::{HashMap, HashSet};
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::standards::v1::subsets::any::schema::inferences::topology::ProgramTopology;

/// 📤️ Serializes a plugin to pretty JSON.
pub fn export_json(program: &ProgramSnapshot) -> Result<String, PluginError> {
    Ok(semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(program))))
}
}
pub use inferences_wire_codec::*;
