//! 💡️ ProgramSnapshot inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).
//!
//! Architectural-programming elements are non-spatial (no x/y/z — `area`/`volume`/`height` are
//! target BANDS, not measured geometry), so `flat-position`/`bounds`-style derivations don't
//! apply; `elements[].parentId` is the one real structural relationship on the snapshot, so a
//! topology summary over it is the honest whole-snapshot derivation. Whole-snapshot scalar, not
//! per-entity, so this uses the plain `protocol::Inference<P>` shape (no `InferredField`/caching
//! machinery — see `🧭topology/🦀️component.rs` for the derivation).

use crate::artifacts::program::ProgramSnapshot;
use protocol::Inference;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::topology::{compute_topology, ProgramTopology};

//#region 🔖️Inference
/// 💡️ Everything inferable from an architect program snapshot. One field per named inference
/// under `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.architect.program.inference")]
pub struct ProgramInference {
    #[state(inferred)]
    pub topology: ProgramTopology,
}

impl protocol::Inference<ProgramSnapshot> for ProgramInference {
    fn infer(snapshot: &ProgramSnapshot) -> Self {
        Self { topology: compute_topology(&snapshot.elements) }
    }
}

/// 🌉️ Hand impl (not derived): `ProgramTopology` has no meaningful `#[derive(Default)]` shape of
/// its own beyond the zero-element case it already matches (see its own `Default` impl) — this
/// exists only so `ProgramInference` itself has a `Default` without requiring `ProgramTopology` to
/// derive one, and to make the "default == infer(default snapshot)" law explicit at this level too.
impl Default for ProgramInference {
    fn default() -> Self {
        Self::infer(&ProgramSnapshot::default())
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

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::program::standards::v1::subsets::any::schema::ProgramBuilder {
    type Snapshot = ProgramSnapshot;
    type Inference = ProgramInference;

    /// 🎯️ Whole-snapshot scalar — nothing here is per-entity, so the cache/session are unused
    /// (same "plain `Inference`" shape the family doc calls out as correct for `dimensions`/
    /// `outline`/`bounds`-style facets).
    fn infer_cached(snapshot: &Self::Snapshot, cache: &mut store::InferenceCache, session: &mut store::InferenceSession) -> Self::Inference {
        let _ = (cache, session);
        <ProgramInference as protocol::Inference<ProgramSnapshot>>::infer(snapshot)
    }
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.architect.program.inference`'s facet leaves into the OS-wide inference catalog
/// — call once at plugin init, alongside `program_artifact_schema_descriptor`'s registration.
pub fn program_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.architect.program.inference",
        inference: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use protocol::Inference;

    //#region 🧪️InferenceLaws
    #[test]
    fn inference_determinism_law() {
        let snapshot = ProgramSnapshot::default();
        assert_eq!(ProgramInference::infer(&snapshot), ProgramInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(ProgramInference::infer(&ProgramSnapshot::default()), ProgramInference::default());
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
