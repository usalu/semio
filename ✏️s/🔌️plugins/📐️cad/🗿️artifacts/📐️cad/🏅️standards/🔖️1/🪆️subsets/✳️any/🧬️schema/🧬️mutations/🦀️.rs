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

//#region 🔖️ObjectRecords
/// 🧱️ Public wire twin of the crate-private, EPHEMERAL `io::geometry_import::CadObject` — the shape
/// `create-object` carries so an object edit is expressible as one bounded parent op while the object
/// data itself keeps living inside the composed `s.stdio.semio.model` child (the op names the child's
/// next CONTENT, never a child diff; the child handle it re-mints is content-addressed over exactly
/// this data — see `crate::cad_pane_rematerialized_child`). `primitives` is absent on purpose: it is
/// derived from `solid_handle`, the same way `cad_object_from_model_element` derives it on the READ
/// side, so the record stays one flat row of scalars.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct CadObjectSpec {
    pub id: String,
    pub label: String,
    pub typology: String,
    pub visible: bool,
    pub locked: bool,
    pub origin: [f64; 3],
    pub orientation: Option<[f64; 4]>,
    pub scale: Option<[f64; 3]>,
    pub mesh_url: Option<String>,
    pub extent: Option<[f64; 3]>,
    pub solid_handle: Option<String>,
}

/// 🧱️ One object's exact next origin — `move-objects` carries one row per touched object, ABSOLUTE
/// rather than a delta so the inverse restores the recorded pose bit-for-bit instead of relying on
/// `x + d - d` round-tripping through f64.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct CadObjectOrigin {
    pub object_id: String,
    pub new_origin: [f64; 3],
}

/// 🧱️ One object's exact next orientation quaternion — see [`CadObjectOrigin`] on why it is absolute.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct CadObjectOrientation {
    pub object_id: String,
    pub new_orientation: [f64; 4],
}

/// 🧱️ One object's exact next per-axis scale — see [`CadObjectOrigin`] on why it is absolute.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct CadObjectScale {
    pub object_id: String,
    pub new_scale: [f64; 3],
}

/// 🧱️ One authored primitive slot of an object — carried beside [`CadObjectSpec`] rather than inside
/// it because a `#[dsl(table)]` field cannot nest inside a `#[dsl(block)]` one. The importer authors
/// slot ids of its own (`…-solid-313`), which a `solid_handle`-derived list would silently rewrite,
/// so `delete-object`'s inverse has to carry them verbatim.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct CadObjectPrimitive {
    pub slot: String,
    pub primitive_id: String,
    pub kind: String,
}

/// 🌉️ `CadObjectSpec` plus its authored primitive slots → the ephemeral working object. An empty
/// `primitives` falls back to the one slot `solid_handle` implies, which is what a hand-authored
/// `create-object` (the catalogue's "add a box") wants.
pub(crate) fn cad_object_from_spec(spec: &CadObjectSpec, primitives: &[CadObjectPrimitive]) -> crate::standards::v1::subsets::any::io::geometry_import::CadObject {
    use crate::standards::v1::subsets::any::io::geometry_import::{CadObject, CadPrimitiveSlot};
    let primitives = if primitives.is_empty() {
        spec.solid_handle.clone().map(|primitive_id| vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id, kind: "solid".into() }]).unwrap_or_default()
    } else {
        primitives.iter().map(|primitive| CadPrimitiveSlot { slot: primitive.slot.clone(), primitive_id: primitive.primitive_id.clone(), kind: primitive.kind.clone() }).collect()
    };
    CadObject {
        id: spec.id.clone(),
        label: spec.label.clone(),
        typology: spec.typology.clone(),
        visible: spec.visible,
        locked: spec.locked,
        origin: spec.origin,
        orientation: spec.orientation,
        scale: spec.scale,
        mesh_url: spec.mesh_url.clone(),
        extent: spec.extent,
        solid_handle: spec.solid_handle.clone(),
        primitives,
    }
}

/// 🌉️ The authored primitive slots of a working object, in `create-object`'s wire shape.
pub(crate) fn cad_object_primitives_of(object: &crate::standards::v1::subsets::any::io::geometry_import::CadObject) -> Vec<CadObjectPrimitive> {
    object.primitives.iter().map(|primitive| CadObjectPrimitive { slot: primitive.slot.clone(), primitive_id: primitive.primitive_id.clone(), kind: primitive.kind.clone() }).collect()
}

/// 🌉️ The `cad_object_from_spec` inverse — what `delete-object`'s inverse captures.
pub(crate) fn cad_object_spec_of(object: &crate::standards::v1::subsets::any::io::geometry_import::CadObject) -> CadObjectSpec {
    CadObjectSpec {
        id: object.id.clone(),
        label: object.label.clone(),
        typology: object.typology.clone(),
        visible: object.visible,
        locked: object.locked,
        origin: object.origin,
        orientation: object.orientation,
        scale: object.scale,
        mesh_url: object.mesh_url.clone(),
        extent: object.extent,
        solid_handle: object.solid_handle.clone(),
    }
}
//#endregion 🔖️ObjectRecords

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
    CreateObject(create_object::CreateObject),
    DeleteObject(delete_object::DeleteObject),
    MoveObjects(move_objects::MoveObjects),
    RotateObjects(rotate_objects::RotateObjects),
    ScaleObjects(scale_objects::ScaleObjects),
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
    "create-object",
    "delete-object",
    "move-objects",
    "rotate-objects",
    "scale-objects",
];
//#endregion 🔖️Mutations

//#region 🔖️Leaves
use super::change_reference_hidden;
use super::change_reference_locked;
use super::change_reference_width;
use super::create_building_model;
use super::create_drawing;
use super::create_energy_model;
use super::create_node;
use super::create_object;
use super::create_shape_model;
use super::delete_object;
use super::move_objects;
use super::rotate_objects;
use super::scale_objects;
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
