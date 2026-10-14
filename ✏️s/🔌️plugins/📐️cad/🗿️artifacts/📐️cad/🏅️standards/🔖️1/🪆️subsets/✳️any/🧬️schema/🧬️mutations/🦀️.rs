//! 🧬️ CAD artifact — document mutation dispatch enum + shared internal patch/helper types.
//! Every variant wraps exactly one `🧬️mutations/<kind>/🦠️mutation` payload struct implementing
//! `protocol::MutationKind<CadSnapshot, CadMutation>`; `#[derive(dsl::Mutations)]` below
//! generates `impl protocol::Mutation`/`impl protocol::SemanticMutation` by delegating to each
//! payload's own `diff`/`inverse` — see `🧪️MutationsDeriveLaws` in
//! `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs` for the reference shape.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: the fourteen triads that used to
//! mutate `CadObject` fields inline on the parent document (`create-object`, `delete-object`,
//! `move-object`, `rotate-object`, `scale-object`, `rename-object`, `change-object-visible`,
//! `change-object-locked`, `change-object-typology`, `replace-object-geometry`,
//! `replace-pane-objects`, `drag-objects`, `rotate-objects`, `scale-objects`) are RETIRED — that
//! data now lives inside the four composed `s.stdio.semio.model` CHILD documents (`shape_model`/
//! `building_model`/`energy_model`/`structure_classic_model`), each its own document with its own
//! independent mutation history; a per-element move/rotate/rename now targets the CHILD document
//! directly, never this parent enum (`🔖️Composition`'s "a parent's diff never embeds a child
//! diff" rule). What replaces them here is real CHILD-SLOT LIFECYCLE — `create`/`delete` for each
//! of the four fixed model slots plus the `drawings` collection — approved verbs only, per
//! `📌️important.md`. Every object edit (create, delete, field, drag, turn, scale) is a child-lane leaf of the pane's
//! composed model child (`insert-element`, `remove-element`, `set-element`, `drag-elements`, `rotate-elements`,
//! `scale-elements`; ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING design §20.15): no parent leaf reads a child.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

//#region 🧩️Modules
#[cfg(test)]
#[path = "➕create-node/🧪️tests/🌱️appends-node-3/🦀️.rs"]
mod tests_appends_node_3;
#[cfg(test)]
#[path = "🗑️delete-node/🧪️tests/🚫️removes-node-2/🦀️.rs"]
mod tests_removes_node_2;
#[cfg(test)]
#[path = "🏷️rename-node/🧪️tests/🔤️relabels-the-root-node/🦀️.rs"]
mod tests_relabels_the_root_node;
#[cfg(test)]
#[path = "👁️change-reference-hidden/🧪️tests/🙈️hides/🦀️.rs"]
mod tests_hides_the_shape_reference;
#[cfg(test)]
#[path = "🔒change-reference-locked/🧪️tests/🔓️unlocks/🦀️.rs"]
mod tests_unlocks_the_shape_reference;
#[cfg(test)]
#[path = "📏change-reference-width/🧪️tests/↔️widens/🦀️.rs"]
mod tests_widens_the_shape_reference_plane;
#[cfg(test)]
#[path = "📍move-reference/🧪️tests/📍️moves/🦀️.rs"]
mod tests_moves_the_shape_reference_off_origin;
#[cfg(test)]
#[path = "🖇️replace-reference-media/🧪️tests/🖼️reattaches/🦀️.rs"]
mod tests_reattaches_the_shape_reference_to_a_new_plan;
#[cfg(test)]
#[path = "📎replace-references/🧪️tests/🔄️swaps/🦀️.rs"]
mod tests_swaps_the_shape_reference_list;
#[cfg(test)]
#[path = "📐️create-drawing/🧪️tests/📐️appends-drawing-2/🦀️.rs"]
mod tests_appends_drawing_2;
#[cfg(test)]
#[path = "🧹delete-drawing/🧪️tests/🚫️removes-drawing-1/🦀️.rs"]
mod tests_removes_drawing_1;
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod component {
                        #[path = "."]
                        pub mod binary {
                            #[path = "../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                            pub mod mutations;
                        }
                        #[path = "."]
                        pub mod text {
                            #[path = "../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                            pub mod mutations;
                        }
                    }
                    pub use component::*;
                }
            }
        }
    }
}
//#endregion 🧩️Modules

use semio_s_artifact_cad_cad_model_diff::CadDiff;
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};



//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the cad document, derived per
/// `📓️derivation-rules.md` from `CadSnapshot`'s shape. `SetSnapshot`/`SetPaneObjects`-as-whole-doc-
/// replace and every generic `Patch*`/`CollectionMutation` variant this facet used to carry are
/// gone — whole-document replace is not an in-history mutation at all (routed through
/// `ArtifactStore::reset` / `Effect::LoadDocument`).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value_derive::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = CadSnapshot, diff = CadDiff, schema = "cad.cad")]
pub enum CadMutation {
    CreateShapeModel(create_shape_model::CreateShapeModel),
    DeleteShapeModel(delete_shape_model::DeleteShapeModel),
    CreateBuildingModel(create_building_model::CreateBuildingModel),
    DeleteBuildingModel(delete_building_model::DeleteBuildingModel),
    CreateEnergyModel(create_energy_model::CreateEnergyModel),
    DeleteEnergyModel(delete_energy_model::DeleteEnergyModel),
    CreateStructureClassicModel(create_structure_classic_model::CreateStructureClassicModel),
    DeleteStructureClassicModel(delete_structure_classic_model::DeleteStructureClassicModel),
    CreateDrawing(create_drawing::CreateDrawing),
    DeleteDrawing(delete_drawing::DeleteDrawing),
    CreateNode(create_node::CreateNode),
    DeleteNode(delete_node::DeleteNode),
    RenameNode(rename_node::RenameNode),
    ChangeReferenceHidden(change_reference_hidden::ChangeReferenceHidden),
    ChangeReferenceLocked(change_reference_locked::ChangeReferenceLocked),
    ChangeReferenceWidth(change_reference_width::ChangeReferenceWidth),
    MoveReference(move_reference::MoveReference),
    ReplaceReferenceMedia(replace_reference_media::ReplaceReferenceMedia),
    ReplaceReferences(replace_references::ReplaceReferences),
    CreateBrep(create_brep::CreateBrep),
    DeleteBrep(delete_brep::DeleteBrep),
}

/// 🏷️ The kebab-case spelling of every [`CadMutation`] variant, in declaration order — the exact
/// vocabulary the `cad-1-any` mutation catalog (`../../🔣️oracle.json`) declares and the
/// `📐️mutate-cad-1` exhaustive case measures itself against. The framework never parses Rust, so
/// `kinds_match_the_enum_and_the_catalog` below is what keeps this list honest against both.
pub const KINDS: &[&str] = &[
    "create-shape-model",
    "delete-shape-model",
    "create-building-model",
    "delete-building-model",
    "create-energy-model",
    "delete-energy-model",
    "create-structure-classic-model",
    "delete-structure-classic-model",
    "create-drawing",
    "delete-drawing",
    "create-node",
    "delete-node",
    "rename-node",
    "change-reference-hidden",
    "change-reference-locked",
    "change-reference-width",
    "move-reference",
    "replace-reference-media",
    "replace-references",
    "create-brep",
    "delete-brep",
];
//#endregion 🔖️Mutations

//#region 🔖️Leaves
use semio_s_artifact_cad_cad_mut_references::change_reference_hidden;
use semio_s_artifact_cad_cad_mut_references::change_reference_locked;
use semio_s_artifact_cad_cad_mut_references::change_reference_width;
use semio_s_artifact_cad_cad_mut_slots::create_building_model;
use semio_s_artifact_cad_cad_mut_documents::create_drawing;
use semio_s_artifact_cad_cad_mut_documents::{create_brep, delete_brep};
use semio_s_artifact_cad_cad_mut_slots::create_energy_model;
use semio_s_artifact_cad_cad_mut_nodes::create_node;
use semio_s_artifact_cad_cad_mut_slots::create_shape_model;
use semio_s_artifact_cad_cad_mut_slots::create_structure_classic_model;
use semio_s_artifact_cad_cad_mut_slots::delete_building_model;
use semio_s_artifact_cad_cad_mut_documents::delete_drawing;
use semio_s_artifact_cad_cad_mut_slots::delete_energy_model;
use semio_s_artifact_cad_cad_mut_nodes::delete_node;
use semio_s_artifact_cad_cad_mut_slots::delete_shape_model;
use semio_s_artifact_cad_cad_mut_slots::delete_structure_classic_model;
use semio_s_artifact_cad_cad_mut_references::move_reference;
use semio_s_artifact_cad_cad_mut_nodes::rename_node;
use semio_s_artifact_cad_cad_mut_references::replace_reference_media;
use semio_s_artifact_cad_cad_mut_references::replace_references;
//#endregion 🔖️Leaves

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub mod tests;
//#endregion 🧪️Tests

/// 🧬️ The mutation facet's schema leaves (rust, typescript, graphql, json schema, proto).
pub const MUTATION_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto"), };
