//! 🎨️ GIS 2D play app command — loading a bundled example map.

use crate::op::GisMapMutation;
use crate::schema::{positions_operations, regions_operations, routes_operations};
use crate::{gis_map_snapshot_with_derived_children, GisMapSnapshot};
use semio_framework_plugin::{ActionArgOption, ArtifactView, ConfigView, Emit, ExampleSource, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Catalogue
/// 🎬️ The map's real example catalogue — the subset's own `📚️examples/*` facets, the single source
/// the manifest's `exampleId` options, the navbar picker and [`set_active_example`] all read.
pub fn example_catalogue() -> Vec<ExampleSource> {
    vec![crate::examples::demo::source()]
}

/// 🏷️ The catalogue id the manifest stages by default — the first declared example.
pub const DEFAULT_EXAMPLE_ID: &str = crate::examples::demo::ID;

/// 📝️ The catalogue as the palette's `exampleId` select options, so a new `📚️examples` facet shows
/// up in the UI without a second, hand-maintained list.
pub fn example_arg_options() -> Vec<ActionArgOption> {
    example_catalogue().into_iter().map(|source| ActionArgOption::new(source.id(), source.label().clone())).collect()
}

/// 🔎️ Resolves one catalogue id into its document. An empty id is the catalogue's "none" arm (an
/// empty map); every other id must name a declared example, so a typo faults instead of silently
/// loading whichever example happens to be bundled.
pub fn example_document(example_id: &str) -> Result<GisMapSnapshot, Fault> {
    if example_id.is_empty() {
        return Ok(GisMapSnapshot::default());
    }
    let source = example_catalogue().into_iter().find(|source| source.id() == example_id).ok_or_else(|| Fault::from(format!("gis map example '{example_id}' is not in the catalogue")))?;
    let parsed = <GisMapSnapshot as store::ArtifactDsl>::parse_dsl(source.document_json()).map_err(|error| Fault::from(format!("gis map example '{example_id}' does not parse: {error:?}")))?;
    Ok(gis_map_snapshot_with_derived_children(parsed))
}
//#endregion 🔖️Catalogue

//#region 🔖️SetActiveExample
/// ✏️ Replaces document content by diffing every collection (positions/routes/regions) into batched
/// create/delete/replace-data operations, so this is an Operation action (not a View one) — an empty
/// `example_id` clears the map, every other id is resolved against [`example_catalogue`] and framed.
/// Never a whole-document snapshot swap (that vocabulary is retired by the taxonomy): each batched
/// operation still has a real per-mutation inverse, so undo restores the prior document exactly.
pub mod set_active_example {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<GisMapMutation, NoConfigMutation>, Fault> {
        let next = example_document(&payload.example_id)?;
        // 🕹️ The pre-migration layer/feature selection clear that used to live here (`SetSelection {
        // ids: Vec::new() }`) is gone — selection is framework-owned config now, and `Emit` has no
        // channel to touch the framework's `interaction_store` (ticket
        // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM). The `"features"` domain is
        // `HierarchyProvider::Flat`, so `validate_state` does not auto-prune it either — a stale
        // selection surviving a document swap is a known, accepted gap of this wave.
        let document = doc.snapshot;
        let mut artifact_mutations = positions_operations(&document.positions, &next.positions);
        artifact_mutations.extend(routes_operations(&document.routes, &next.routes));
        artifact_mutations.extend(regions_operations(&document.regions, &next.regions));
        Ok(Emit { artifact_mutations, ..Default::default() })
    }
}
//#endregion 🔖️SetActiveExample

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
