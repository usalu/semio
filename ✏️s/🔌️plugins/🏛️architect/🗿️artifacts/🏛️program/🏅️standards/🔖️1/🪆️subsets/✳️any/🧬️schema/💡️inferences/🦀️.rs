//! 💡️ ProgramSnapshot inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).
//!
//! Architectural-programming elements are non-spatial (no x/y/z — `area`/`🟫️volume`/`height` are
//! target BANDS, not measured geometry), so `flat-position`/`bounds`-style derivations don't
//! apply; `elements[].parentId` is the one real structural relationship on the snapshot, so a
//! topology summary over it is the honest whole-snapshot derivation. Whole-snapshot scalar, not
//! per-entity, so this uses the plain `protocol::Inference<P>` shape (no `InferredField`/caching
//! machinery — see `🧭topology/🦀️.rs` for the derivation).

use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;
use framework_schema::ArtifactSchema;
use protocol::Inference;

use super::topology::compute_topology;
//#region 🔖️DerivedComputeImports
/// 🧭️ Dissolved out of the former `⚙️engine` topic files (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — every `fn(&ProgramSnapshot, ...) ->
/// Value`-style read-only projection that used to live on the artifact-tree engine hub. Mutating /
/// constructing counterparts (marked in each region below) moved to `crate::apps::architect`'s own
/// `//#region 🔧️Behavior` instead, since they take `&mut ProgramSnapshot`.
use crate::standards::v1::subsets::any::schema::kernel::{DiagnosticSeverity, EntityHeader, EntityId, LifecycleStatus, Priority, ProgramDiagnostic};
use crate::standards::v1::subsets::any::schema::registers::{AdjacencyKind, AnalysisKind, AuditEvent, RelationshipKind, ReportKind, RiskLevel, SearchFilter, SeparationKind, ValidationStatus};
use crate::standards::v1::subsets::any::schema::snapshot::ARCHITECT_PROGRAM_SCHEMA;
use std::collections::{HashMap, HashSet};
//#endregion 🔖️DerivedComputeImports

//#region 🔖️Inference
/// 💡️ Everything inferable from an architect program snapshot. One field per named inference
/// under `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[artifact_schema(id = "s.architect.program.inference")]
pub struct ProgramInference {
    #[derived]
    pub topology: ProgramTopology,
}

impl Inference<ProgramSnapshot> for ProgramInference {
    fn infer(snapshot: &ProgramSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { topology: compute_topology(&snapshot.elements) }
    
        })
    }
}

/// 🌉️ Hand impl (not derived): `ProgramTopology` has no meaningful `#[derive(Default)]` shape of
/// its own beyond the zero-element case it already matches (see its own `Default` impl) — this
/// exists only so `ProgramInference` itself has a `Default` without requiring `ProgramTopology` to
/// derive one, and to make the "default == infer(default snapshot)" law explicit at this level too.
impl Default for ProgramInference {
    fn default() -> Self {
        let snapshot = &ProgramSnapshot::default();

        Self { topology: compute_topology(&snapshot.elements) }
    }
}

impl protocol::InferenceSpec<ProgramSnapshot> for ProgramInference {
    fn inference_schema_id() -> &'static str {
        "s.architect.program.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[
            protocol::InferenceFieldSpec { id: "s.architect.program.inference.topology.nodeCount", reads: &["elements"] },
            protocol::InferenceFieldSpec { id: "s.architect.program.inference.topology.rootCount", reads: &["elements"] },
            protocol::InferenceFieldSpec { id: "s.architect.program.inference.topology.maxDepth", reads: &["elements"] },
            protocol::InferenceFieldSpec { id: "s.architect.program.inference.topology.cycleFree", reads: &["elements"] },
            protocol::InferenceFieldSpec { id: "s.architect.program.inference.topology.topoOrder", reads: &["elements"] },
        ]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.architect.program.inference`'s facet leaves into the OS-wide inference catalog
/// — call once at plugin init, alongside `program_artifact_schema_descriptor`'s registration.
pub fn program_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.architect.program.inference",
        inference: semio_framework_schema_registry::FacetLeaves { rust: concat!(include_str!("🦀️.rs"), include_str!("🔀️adjacency-and-validation/🦀️.rs"), include_str!("🎁️outputs-and-reports/🦀️.rs"), include_str!("📊️status-search-and-analysis/🦀️.rs")), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
    }
}
//#endregion 🔖️Descriptor

#[path = "🔀️adjacency-and-validation/🦀️.rs"]
mod adjacency_and_validation;
pub use adjacency_and_validation::*;


//#endregion 🧪️AdjacencyTests


//#endregion 🧪️ValidateTests

#[path = "🎁️outputs-and-reports/🦀️.rs"]
mod outputs_and_reports;
pub use outputs_and_reports::*;


//#endregion 🧪️OutputsTests


//#endregion 🧪️ReportTests

#[path = "📊️status-search-and-analysis/🦀️.rs"]
mod status_search_and_analysis;
pub use status_search_and_analysis::*;


//#endregion 🧪️StatusSummaryTests


//#endregion 🧪️SearchTests


//#endregion 🧪️AnalyzeTests

//#region 📤️ExchangeReads

//#endregion 📤️ExchangeReads


//#endregion 🧪️ExchangeReadsTests

//#region 🧭️TraceReads
/// 📜️ Filtered audit trail slice.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct AuditTrail {
    pub subject_id: Option<EntityId>,
    pub events: Vec<AuditEvent>,
}

/// 📋️ Returns audit events for an optional subject, newest first.
pub fn audit_trail(program: &ProgramSnapshot, subject_id: Option<&EntityId>) -> AuditTrail {
    let mut events: Vec<AuditEvent> = program.audit_events.iter().filter(|event| subject_id.is_none_or(|id| &event.subject_id == id)).cloned().collect();
    events.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    AuditTrail { subject_id: subject_id.cloned(), events }
}

/// 🔁️ Resolves superseded requirements to their terminal replacement.
pub fn resolve_supersedes(program: &ProgramSnapshot, requirement_id: &EntityId) -> EntityId {
    let mut current = requirement_id.clone();
    let mut visited = HashSet::new();
    loop {
        if !visited.insert(current.clone()) {
            break;
        }
        let Some(next) = program.requirements.iter().find(|r| r.header.id == current).and_then(|r| r.superseded_by.clone()) else {
            break;
        };
        current = next;
    }
    current
}
//#endregion 🧭️TraceReads


//#endregion 🧪️TraceReadsTests


//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use super::topology::ProgramTopology;
//#endregion 🔁️Re-exports
