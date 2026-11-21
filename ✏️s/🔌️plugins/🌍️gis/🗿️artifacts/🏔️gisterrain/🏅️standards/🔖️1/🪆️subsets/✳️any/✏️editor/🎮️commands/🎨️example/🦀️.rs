//! 🎬️ GIS 3D play app command — loading a bundled example terrain.

use crate::mutations::change_exaggeration::ChangeExaggeration;
use crate::mutations::change_imported_features::ChangeImportedFeatures;
use crate::mutations::ReplaceElevationSource;
use crate::standards::v1::subsets::any::schema::mutations::GisTerrainMutation;
use crate::GisTerrainSnapshot;
use semio_plugin::{ActionArgOption, ArtifactView, ConfigView, Emit, ExampleSource, Fault, NoConfig, NoConfigMutation};
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

/// 🔎️ Resolves a catalogue id or the empty terrain document.
pub fn example_document(example_id: &str) -> Result<GisTerrainSnapshot, Fault> {
    if example_id.is_empty() {
        return Ok(crate::standards::v1::subsets::any::io::text::snapshot::empty_gis_terrain_snapshot());
    }
    let source = example_catalogue().into_iter().find(|source| source.id() == example_id).ok_or_else(|| Fault::from(format!("gis terrain example '{example_id}' is not in the catalogue")))?;
    <GisTerrainSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(&source.document_json()).map_err(|error| Fault::from(format!("gis terrain example '{example_id}' does not parse: {error:?}")))
}
//#endregion 🔖️Catalogue

//#region 🔖️SetActiveExample
/// ✏️ Emits one invertible mutation for each change to the example's exaggeration,
/// imported map, and elevation source.
pub mod set_active_example {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
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
        if document.imported_map != next.imported_map {
            artifact_mutations.push(GisTerrainMutation::ChangeImportedFeatures(ChangeImportedFeatures { new_imported_map: next.imported_map }));
        }
        if document.elevation_source != next.elevation_source {
            artifact_mutations.push(GisTerrainMutation::ReplaceElevationSource(ReplaceElevationSource { new_elevation_source: next.elevation_source }));
        }
        Ok(Emit { artifact_mutations, ..Default::default() })
    }
}
//#endregion 🔖️SetActiveExample

//#region 🧪️Tests
//#endregion 🧪️Tests
