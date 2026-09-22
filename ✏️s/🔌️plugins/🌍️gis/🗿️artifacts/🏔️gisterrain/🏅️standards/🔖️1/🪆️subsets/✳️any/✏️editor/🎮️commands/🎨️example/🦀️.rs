//! 🎬️ GIS 3D play app command — loading a bundled example terrain.

use crate::mutations::change_exaggeration::ChangeExaggeration;
use crate::mutations::change_imported_features::ChangeImportedFeatures;
use crate::op::GisTerrainMutation;
use crate::GisTerrainSnapshot;
use semio_framework_plugin::{ActionArgOption, ArtifactView, ConfigView, Emit, ExampleSource, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Catalogue
/// 🎬️ The terrain's real example catalogue — the subset's own `📚️examples/*` facets, the single
/// source the manifest's `exampleId` options, the navbar picker and [`set_active_example`] all read.
/// The gismap twin is `✏️editor/🎮️commands/🎨️example/🦀️.rs::example_catalogue`.
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

/// 🔎️ Resolves one catalogue id into its document. An empty id is the catalogue's "none" arm (the
/// flat unimported terrain); every other id must name a declared example, so a typo faults instead
/// of silently loading whichever example happens to be bundled.
pub fn example_document(example_id: &str) -> Result<GisTerrainSnapshot, Fault> {
    if example_id.is_empty() {
        return Ok(crate::schema::empty_gis_terrain_snapshot());
    }
    let source = example_catalogue().into_iter().find(|source| source.id() == example_id).ok_or_else(|| Fault::from(format!("gis terrain example '{example_id}' is not in the catalogue")))?;
    <GisTerrainSnapshot as store::ArtifactDsl>::parse_dsl(&source.document_json()).map_err(|error| Fault::from(format!("gis terrain example '{example_id}' does not parse: {error:?}")))
}
//#endregion 🔖️Catalogue

//#region 🔖️SetActiveExample
/// ✏️ Replaces document content by diffing this artifact's two editable fields — `exaggeration` and
/// the `map:in` overlay's `importedFeaturesJson` — into the authored `change-exaggeration` /
/// `change-imported-features` leaves, so this is a Mutation action, not a View one. Never a
/// whole-document snapshot swap (that vocabulary is retired by the taxonomy): each emitted leaf
/// still has its own real inverse, so one undo restores the prior document exactly. A field that
/// already holds the example's value emits nothing, for the same reason `change-exaggeration`
/// itself warns `mutation.no-op` rather than publishing an empty edit.
///
/// 🕸️ `mesh` is NOT emitted: it is the composed child derived from exactly these two fields
/// (`gis_terrain_mesh_content_key`), so it follows the document the same way it follows a plain
/// `setExaggeration` — this command does not own a second, divergent rule for it.
pub mod set_active_example {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, GisTerrainSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<GisTerrainMutation, NoConfigMutation>, Fault> {
        let next = example_document(&payload.example_id)?;
        let document = doc.snapshot;
        let mut artifact_mutations = Vec::new();
        if document.exaggeration != next.exaggeration {
            artifact_mutations.push(GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: next.exaggeration }));
        }
        if document.imported_features_json != next.imported_features_json {
            artifact_mutations.push(GisTerrainMutation::ChangeImportedFeatures(ChangeImportedFeatures { new_imported_features_json: next.imported_features_json.clone() }));
        }
        Ok(Emit { artifact_mutations, ..Default::default() })
    }
}
//#endregion 🔖️SetActiveExample

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
