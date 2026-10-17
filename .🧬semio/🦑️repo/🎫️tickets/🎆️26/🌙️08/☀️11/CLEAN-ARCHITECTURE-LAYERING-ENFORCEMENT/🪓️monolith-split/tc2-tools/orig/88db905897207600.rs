//! 🧬️ Drawing diff schema — sparse field delta over the artifact + its `apply`/`absorb` pure
//! transform (design.md rule 3: `🧬️schema` keeps types + pure transforms; the facet's grammar spec
//! asset moved to `🚪️io/📝️text/🔺️diff/🦀️.rs`, but `apply`/`absorb` are not a byte-boundary
//! codec — they transform already-decoded `DrawingDiff`/`DrawingSnapshot` values — so they stayed here).

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

use semio_s_artifact_draw_drawing_model::{layer_base, layer_base_mut, layer_id, remove_layer_from_tree, update_layer_in_tree};
use semio_s_artifact_draw_drawing_model::{DrawingArtboard, DrawingImageAsset, DrawingLayerNode, FillStyle, StrokeStyle};
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;
use std::collections::{BTreeMap, BTreeSet};
use semio_framework_value::{list::PagedList, paged::{PagedMap, PagedUtf8, Utf8Text}};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the drawing artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[artifact_schema(id = "s.draw.drawing")]
pub struct DrawingDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub id: Option<String>,
    #[state(artifact)]
    pub title: Option<Option<String>>,
    #[state(artifact)]
    pub layers: Option<DrawingLayersDelta>,
    #[state(artifact)]
    pub assets: Option<DrawingAssetsDelta>,
    #[state(artifact)]
    pub artboard: Option<Option<DrawingArtboard>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 🗂️ Asset-map wrapper so optional map diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct DrawingAssetsDelta {
    pub entries: BTreeMap<String, Option<DrawingImageAsset>>,
}

/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct DrawingStringList {
    pub values: Vec<String>,
}

/// 🧩 Positional delta for the layer TREE (the shape of the framework's `protocol::list_delta`, addressed per container): `removed`
/// rows carry their BASE address, `inserted` rows their AFTER address, `moved` rows both (a move may cross containers); no order
/// list and no anchor is ever carried. Every index is a coordinate of the base or of the after child list of its parent.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct DrawingLayersDelta {
    pub removed: Vec<DrawingLayerRemoval>,
    pub inserted: Vec<DrawingLayerInsertion>,
    pub moved: Vec<DrawingLayerRelocation>,
    pub modified: Vec<DrawingLayerModification>,
}

/// 📍️ A position in the layer tree: the child list of `parent_id` (the root list when absent) at `index`.
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DrawingLayerAddress {
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub parent_id: Option<String>,
    pub index: usize,
}

/// ➖️ One removed layer subtree and the BASE address the inverse reinserts it at.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DrawingLayerRemoval {
    pub id: String,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub parent_id: Option<String>,
    pub index: usize,
}

/// ➕️ One inserted layer subtree and its AFTER address.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DrawingLayerInsertion {
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub parent_id: Option<String>,
    pub index: usize,
    pub layer: DrawingLayerNode,
}

/// ↕️ One repositioned layer subtree: its BASE address and its AFTER address — the layer itself is never carried.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DrawingLayerRelocation {
    pub id: String,
    pub from: DrawingLayerAddress,
    pub to: DrawingLayerAddress,
}

/// 🩹 One modified layer entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DrawingLayerModification {
    pub id: String,
    pub patch: DrawingLayerPatch,
}

/// 🩹 Explicit replacement preserving an absent patch and a cleared value.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(default)]
pub struct DrawingFillPatch { pub value: Option<FillStyle> }

/// 🩹 Explicit replacement preserving an absent patch and a cleared value.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(default)]
pub struct DrawingStrokePatch { pub value: Option<StrokeStyle> }

/// 🔷️ One sparse authored coordinate edit, independent of transform and appearance.
#[derive(Clone,Debug,PartialEq,semio_framework_value::ToValue,semio_framework_value::FromValue)]
#[cfg_attr(test,derive(serde::Serialize,serde::Deserialize))]
#[value(rename_all="camelCase")]
#[cfg_attr(test,serde(rename_all="camelCase"))]
pub struct DrawingShapeCoordinatePatch {
    pub field:crate::schema::shape_geometry::ShapeCoordinateField,
    pub index:Option<usize>,
    pub value:f64,
}

/// 🩹 Typed layer changes.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct DrawingLayerPatch {
    pub visible: Option<bool>,
    pub locked: Option<bool>,
    pub name: Option<String>,
    pub opacity: Option<f64>,
    pub blend_mode: Option<String>,
    pub fill_rule: Option<semio_s_artifact_draw_drawing_model::fill_rule::FillRule>,
    #[value(skip_serializing_if="Option::is_none")]
    #[cfg_attr(test,serde(skip_serializing_if="Option::is_none"))]
    pub isolation:Option<bool>,
    pub transform: Option<semio_s_artifact_draw_drawing_model::DrawingTransform>,
    pub fill: Option<DrawingFillPatch>,
    pub stroke: Option<DrawingStrokePatch>,
    pub boolean_operation: Option<String>,
    pub trace_params: Option<semio_s_artifact_draw_drawing_model::DrawingTraceParams>,
    pub layer: Option<DrawingLayerNode>,
    pub path_segments: Option<PagedList<semio_s_artifact_draw_drawing_model::PathSegment, {usize::MAX}>>,
    pub text_content: Option<String>,
    pub text_size: Option<f64>,
    pub font_family: Option<semio_s_artifact_draw_drawing_model::font_family::DrawingFontFamily>,
    pub image_key: Option<String>,
    pub image_width: Option<f64>,
    pub image_height: Option<f64>,
    #[value(default,skip_serializing_if="Vec::is_empty")]
    #[cfg_attr(test,serde(default,skip_serializing_if="Vec::is_empty"))]
    pub shape_coordinates:Vec<DrawingShapeCoordinatePatch>,
}
//#endregion 🔖️DeltaHelpers

//#region 🔖️Apply

#[path = "🩹️apply-delta/🦀️.rs"]
mod apply_delta;
pub use apply_delta::*;

//#endregion 🔖️Apply

//#region 🔖️Builders
/// 🩹 Layer visibility patch.
pub fn diff_set_layer_visible(layer_id: &(impl std::fmt::Display + ?Sized), visible: bool) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { visible: Some(visible), ..Default::default() })
}

/// 🔒️ Layer locked patch.
pub fn diff_set_layer_locked(layer_id: &(impl std::fmt::Display + ?Sized), locked: bool) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { locked: Some(locked), ..Default::default() })
}

/// 🏷️ Layer name patch.
pub fn diff_set_layer_name(layer_id: &(impl std::fmt::Display + ?Sized), name: &(impl std::fmt::Display + ?Sized)) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { name: Some(name.to_string()), ..Default::default() })
}

/// 🌫️ Layer opacity patch.
pub fn diff_set_layer_opacity(layer_id: &(impl std::fmt::Display + ?Sized), opacity: f64) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { opacity: Some(opacity), ..Default::default() })
}

/// 🖌️ Layer blend-mode patch.
pub fn diff_set_layer_blend_mode(layer_id: &(impl std::fmt::Display + ?Sized), blend_mode: &(impl std::fmt::Display + ?Sized)) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { blend_mode: Some(blend_mode.to_string()), ..Default::default() })
}

/// ✏️ Several paths' geometry patches in one sparse delta, in the given order.
pub fn diff_set_path_geometries<S: IntoIterator<Item = semio_s_artifact_draw_drawing_model::PathSegment>>(entries: impl IntoIterator<Item = (String, S)>) -> DrawingDiff {
    let modified = entries.into_iter().map(|(id, segments)| DrawingLayerModification { id, patch: DrawingLayerPatch { path_segments: Some(segments.into_iter().collect()), ..Default::default() } }).collect();
    DrawingDiff { layers: Some(DrawingLayersDelta { modified, ..Default::default() }), ..Default::default() }
}

/// 🔀 Boolean operation patch.
pub fn diff_set_boolean_operation(layer_id: &(impl std::fmt::Display + ?Sized), boolean_operation: &(impl std::fmt::Display + ?Sized)) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { boolean_operation: Some(boolean_operation.to_string()), ..Default::default() })
}

/// 🌱️ Layer insertion into `parent_id` (root when `None`) at the after index `index` (clamped to the end of the child list).
pub fn diff_create_layer(base: &PagedList<DrawingLayerNode, {usize::MAX}>, parent_id: Option<&PagedUtf8<{usize::MAX}>>, index: usize, layer: DrawingLayerNode) -> DrawingDiff {
    let parent = parent_id.map(PagedUtf8::to_string_owner);
    let length = container_of(base, parent.as_deref()).map_or(0, |children| children.len());
    DrawingDiff { layers: Some(DrawingLayersDelta { inserted: vec![DrawingLayerInsertion { parent_id: parent, index: index.min(length), layer }], ..Default::default() }), ..Default::default() }
}

/// 🔃 Move the layer `layer_id` to the after index `index` of `parent_id`'s child list — one tree-aware move row from its base
/// address, never the layer itself. Empty when the layer is not in `base`.
pub fn diff_reorder_layer(base: &PagedList<DrawingLayerNode, {usize::MAX}>, layer_id: &(impl std::fmt::Display + ?Sized), parent_id: Option<&PagedUtf8<{usize::MAX}>>, index: usize) -> DrawingDiff {
    let id = layer_id.to_string();
    let Some((from_parent, from_index)) = address_of(base, &id) else { return DrawingDiff::default() };
    let parent = parent_id.map(PagedUtf8::to_string_owner);
    let length = container_of(base, parent.as_deref()).map_or(0, |children| children.len());
    let room = if parent == from_parent { length.saturating_sub(1) } else { length };
    DrawingDiff {
        layers: Some(DrawingLayersDelta {
            moved: vec![DrawingLayerRelocation { id, from: DrawingLayerAddress { parent_id: from_parent, index: from_index }, to: DrawingLayerAddress { parent_id: parent, index: index.min(room) } }],
            ..Default::default()
        }),
        ..Default::default()
    }
}

/// ➖️ Layer remove: one removal row at the layer's base address. Empty when the layer is not in `base`.
pub fn diff_remove_layer(base: &PagedList<DrawingLayerNode, {usize::MAX}>, layer_id: &(impl std::fmt::Display + ?Sized)) -> DrawingDiff {
    let id = layer_id.to_string();
    let Some((parent_id, index)) = address_of(base, &id) else { return DrawingDiff::default() };
    DrawingDiff { layers: Some(DrawingLayersDelta { removed: vec![DrawingLayerRemoval { id, parent_id, index }], ..Default::default() }), ..Default::default() }
}

/// 🗂️ Assets delta helper.
pub fn diff_assets(entries: DrawingAssetsDelta) -> DrawingDiff {
    DrawingDiff { assets: Some(entries), ..Default::default() }
}
//#endregion 🔖️Builders

/// ✏️ Replaces only the path geometry facet.
pub fn diff_set_path_geometry(layer_id: &(impl std::fmt::Display + ?Sized), segments: &(impl semio_s_artifact_draw_drawing_geometry::editing::PathGeometrySource + ?Sized)) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { path_segments: Some(segments.path_segments().cloned().collect()), ..Default::default() })
}

/// 🌀️ Sparse authored fill-rule delta.
pub fn diff_set_layer_fill_rule(layer_id:&(impl std::fmt::Display + ?Sized),fill_rule:crate::FillRule)->DrawingDiff {layer_base_patch(layer_id,DrawingLayerPatch {fill_rule:Some(fill_rule),..Default::default()})}

pub fn diff_set_group_isolation(layer_id:&(impl std::fmt::Display + ?Sized),isolation:bool)->DrawingDiff {layer_base_patch(layer_id,DrawingLayerPatch {isolation:Some(isolation),..Default::default()})}

/// ↔️ Layer transform patch.
pub fn diff_set_layer_transform(layer_id: &(impl std::fmt::Display + ?Sized), transform: &semio_s_artifact_draw_drawing_model::DrawingTransform) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { transform: Some(transform.clone()), ..Default::default() })
}

/// ↔️ Several layers' transform patches in one sparse delta, in the given order.
pub fn diff_set_layer_transforms(entries: impl IntoIterator<Item = (String, semio_s_artifact_draw_drawing_model::DrawingTransform)>) -> DrawingDiff {
    let modified = entries.into_iter().map(|(id, transform)| DrawingLayerModification { id, patch: DrawingLayerPatch { transform: Some(transform), ..Default::default() } }).collect();
    DrawingDiff { layers: Some(DrawingLayersDelta { modified, ..Default::default() }), ..Default::default() }
}

/// 🎨 Layer fill patch.
pub fn diff_set_fill(layer_id: &(impl std::fmt::Display + ?Sized), fill: &Option<FillStyle>) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { fill: Some(DrawingFillPatch { value: fill.clone() }), ..Default::default() })
}

/// ✏️ Layer stroke patch.
pub fn diff_set_stroke(layer_id: &(impl std::fmt::Display + ?Sized), stroke: &Option<StrokeStyle>) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { stroke: Some(DrawingStrokePatch { value: stroke.clone() }), ..Default::default() })
}

/// 🖼️ Trace params patch.
pub fn diff_set_trace_params(layer_id: &(impl std::fmt::Display + ?Sized), params: &semio_s_artifact_draw_drawing_model::DrawingTraceParams) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { trace_params: Some(params.clone()), ..Default::default() })
}

pub fn layer_base_patch(layer_id: &(impl std::fmt::Display + ?Sized), patch: DrawingLayerPatch) -> DrawingDiff {
    DrawingDiff { layers: Some(DrawingLayersDelta { modified: vec![DrawingLayerModification { id: layer_id.to_string(), patch }], ..Default::default() }), ..Default::default() }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#region 🪢️TaxonomyMounts
#[path = "."]
pub mod io {
    #[path = "../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
    pub mod binary;
    #[path = "../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
    pub mod text;
}
//#endregion 🪢️TaxonomyMounts
