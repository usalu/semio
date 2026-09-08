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
//! `📌️important.md`.

use crate::diff::CadDiff;
use crate::CadSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️InternalPatches
/// 🩹 Option-bag field delta for [`crate::CadNode`] — INTERNAL diff-construction glue only.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct CadNodePatch {
    pub label: Option<String>,
}

/// 🩹 Option-bag field delta for [`crate::CadReference`] — INTERNAL diff-construction glue only.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct CadReferencePatch {
    pub source_url: Option<String>,
    pub media_kind: Option<String>,
    pub origin: Option<[f64; 3]>,
    pub orientation: Option<[f64; 4]>,
    pub scale: Option<f64>,
    pub width_world: Option<f64>,
    pub hidden: Option<bool>,
    pub locked: Option<bool>,
    pub opacity: Option<f64>,
}
//#endregion 🔖️InternalPatches

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the cad document, derived per
/// `📓️derivation-rules.md` from `CadSnapshot`'s shape. `SetSnapshot`/`SetPaneObjects`-as-whole-doc-
/// replace and every generic `Patch*`/`CollectionMutation` variant this facet used to carry are
/// gone — whole-document replace is not an in-history mutation at all (routed through
/// `ArtifactStore::reset`, see `CadPlayApp::whole_document_operation` returning `None` now).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslEnum, dsl::Mutations)]
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
    ChangeActiveModelDefinition(change_active_model_definition::ChangeActiveModelDefinition),
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
    "change-active-model-definition",
];
//#endregion 🔖️Mutations

//#region 🔖️Leaves
use super::change_active_model_definition;
use super::change_reference_hidden;
use super::change_reference_locked;
use super::change_reference_width;
use super::create_building_model;
use super::create_drawing;
use super::create_energy_model;
use super::create_node;
use super::create_shape_model;
use super::create_structure_classic_model;
use super::delete_building_model;
use super::delete_drawing;
use super::delete_energy_model;
use super::delete_node;
use super::delete_shape_model;
use super::delete_structure_classic_model;
use super::move_reference;
use super::rename_node;
use super::replace_reference_media;
use super::replace_references;
//#endregion 🔖️Leaves

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub mod tests;
//#endregion 🧪️Tests
