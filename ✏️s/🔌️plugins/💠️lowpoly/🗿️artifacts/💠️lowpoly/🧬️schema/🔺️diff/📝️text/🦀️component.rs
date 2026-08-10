//! 🔺️ Lowpoly artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::lowpoly::schema::diff::{
    LowpolyDiff, LowpolyObjectPatchEntry, LowpolyObjectsDelta, LowpolyPaintLayersDelta, LowpolyPaintStrokeAt,
    PixelRun as SchemaPixelRun,
};
use crate::artifacts::lowpoly::schema::LowpolyArtifact;
use crate::artifacts::lowpoly::{apply_paint_layers_delta, LowpolySnapshot};
use protocol::MutationDiff;


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::lowpoly::schema::diff::*;


//#region 🔖️Apply
impl LowpolyDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &LowpolyArtifact) -> LowpolyArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(delta) = &self.objects {
            next.objects = apply_objects_delta(&next.objects, delta);
        }
        if let Some(value) = &self.active_object_id {
            next.active_object_id = value.clone();
        }
        if let Some(value) = &self.selection {
            next.selection = value.clone();
        }
        if let Some(list) = &self.selected_object_ids {
            next.selected_object_ids = list.values.clone();
        }
        if let Some(value) = &self.paint_utility {
            next.paint_utility = value.clone();
        }
        if let Some(value) = self.active_paint_layer {
            next.active_paint_layer = value;
        }
        if let Some(value) = &self.active_utility_id {
            next.active_utility_id = value.clone();
        }
        if let Some(value) = self.show_edges {
            next.show_edges = value;
        }
        if let Some(value) = self.sun_enabled {
            next.sun_enabled = value;
        }
        if let Some(value) = self.sun_azimuth {
            next.sun_azimuth = value;
        }
        if let Some(value) = self.sun_elevation {
            next.sun_elevation = value;
        }
        if let Some(value) = self.sun_intensity {
            next.sun_intensity = value;
        }
        if let Some(value) = &self.sun_color {
            next.sun_color = value.clone();
        }
        if let Some(value) = self.world_camera_position_x {
            next.world_camera_position_x = value;
        }
        if let Some(value) = self.world_camera_position_y {
            next.world_camera_position_y = value;
        }
        if let Some(value) = self.world_camera_position_z {
            next.world_camera_position_z = value;
        }
        if let Some(value) = self.world_camera_target_x {
            next.world_camera_target_x = value;
        }
        if let Some(value) = self.world_camera_target_y {
            next.world_camera_target_y = value;
        }
        if let Some(value) = self.world_camera_target_z {
            next.world_camera_target_z = value;
        }
        if let Some(value) = self.world_camera_fov {
            next.world_camera_fov = value;
        }
        if let Some(value) = &self.utility_params_json {
            next.utility_params_json = value.clone();
        }
        if let Some(value) = self.paint_color_r {
            next.paint_color_r = value;
        }
        if let Some(value) = self.paint_color_g {
            next.paint_color_g = value;
        }
        if let Some(value) = self.paint_color_b {
            next.paint_color_b = value;
        }
        if let Some(value) = self.paint_color_a {
            next.paint_color_a = value;
        }
        if let Some(value) = &self.selection_method {
            next.selection_method = value.clone();
        }
        if let Some(value) = &self.selection_mode_default {
            next.selection_mode_default = value.clone();
        }
        if let Some(value) = &self.engagement_input {
            next.engagement_input = value.clone();
        }
        if let Some(value) = &self.locale {
            next.locale = value.clone();
        }
        if let Some(value) = &self.hovered_object_id {
            next.hovered_object_id = value.clone();
        }
        if let Some(value) = &self.hovered_target_object_id {
            next.hovered_target_object_id = value.clone();
        }
        if let Some(value) = &self.hovered_target_mode {
            next.hovered_target_mode = value.clone();
        }
        if let Some(value) = &self.hovered_target_id {
            next.hovered_target_id = *value;
        }
        if let Some(value) = self.stroke_drag_active {
            next.stroke_drag_active = value;
        }
        if let Some(value) = self.transform_drag_active {
            next.transform_drag_active = value;
        }
        if let Some(value) = self.preview_seq {
            next.preview_seq = value;
        }
        next
    }
}

/// 🧩 Applies an identified-collection delta to a snapshot object list.
pub fn apply_objects_delta(
    objects: &[crate::artifacts::lowpoly::LowpolyObject],
    delta: &LowpolyObjectsDelta,
) -> Vec<crate::artifacts::lowpoly::LowpolyObject> {
    let mut next = objects.to_vec();
    for id in &delta.removed {
        next.retain(|object| &object.id != id);
    }
    for item in &delta.added {
        next.push(item.clone());
    }
    for entry in &delta.patched {
        if let Some(object) = next.iter_mut().find(|object| object.id == entry.id) {
            use protocol::Patchable;
            object.apply_patch(&entry.patch);
            if let Some(paint) = &entry.paint_layers {
                apply_paint_layers_delta(object, paint);
            }
        }
    }
    if let Some(order) = &delta.reordered {
        let mut by_id: std::collections::BTreeMap<_, _> =
            next.into_iter().map(|object| (object.id.clone(), object)).collect();
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            if let Some(object) = by_id.remove(id) {
                ordered.push(object);
            }
        }
        ordered.extend(by_id.into_values());
        next = ordered;
    }
    next
}

impl MutationDiff<LowpolySnapshot> for LowpolyDiff {
    fn apply(&self, snapshot: &LowpolySnapshot) -> LowpolySnapshot {
        if let Some(replacement) = &self.artifact {
            return LowpolySnapshot { schema: replacement.schema.clone(), objects: replacement.objects.clone() };
        }
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(delta) = &self.objects {
            next.objects = apply_objects_delta(&next.objects, delta);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(active_object_id);
        take!(selection);
        take!(selected_object_ids);
        take!(paint_utility);
        take!(active_paint_layer);
        take!(active_utility_id);
        take!(show_edges);
        take!(sun_enabled);
        take!(sun_azimuth);
        take!(sun_elevation);
        take!(sun_intensity);
        take!(sun_color);
        take!(world_camera_position_x);
        take!(world_camera_position_y);
        take!(world_camera_position_z);
        take!(world_camera_target_x);
        take!(world_camera_target_y);
        take!(world_camera_target_z);
        take!(world_camera_fov);
        take!(utility_params_json);
        take!(paint_color_r);
        take!(paint_color_g);
        take!(paint_color_b);
        take!(paint_color_a);
        take!(selection_method);
        take!(selection_mode_default);
        take!(engagement_input);
        take!(locale);
        take!(hovered_object_id);
        take!(hovered_target_object_id);
        take!(hovered_target_mode);
        take!(hovered_target_id);
        take!(stroke_drag_active);
        take!(transform_drag_active);
        take!(preview_seq);
        match (&mut self.objects, other.objects) {
            (Some(dst), Some(src)) => {
                dst.added.extend(src.added);
                dst.removed.extend(src.removed);
                dst.patched.extend(src.patched);
                if src.reordered.is_some() {
                    dst.reordered = src.reordered;
                }
            }
            (dst, Some(src)) => *dst = Some(src),
            _ => {}
        }
    }
}
//#endregion 🔖️Apply

//#region 🔖️Constructors
/// 🏗️ Objects-add field delta.
pub fn diff_objects_add(index: usize, item: crate::artifacts::lowpoly::LowpolyObject, base: &LowpolySnapshot) -> LowpolyDiff {
    let mut order: Vec<String> = base.objects.iter().map(|object| object.id.clone()).collect();
    let id = item.id.clone();
    let at = index.min(order.len());
    order.insert(at, id);
    LowpolyDiff {
        objects: Some(LowpolyObjectsDelta {
            added: vec![item],
            removed: Vec::new(),
            patched: Vec::new(),
            reordered: Some(order),
        }),
        ..LowpolyDiff::default()
    }
}

/// 🏗️ Objects-remove field delta.
pub fn diff_objects_remove(id: String) -> LowpolyDiff {
    LowpolyDiff {
        objects: Some(LowpolyObjectsDelta {
            added: Vec::new(),
            removed: vec![id],
            patched: Vec::new(),
            reordered: None,
        }),
        ..LowpolyDiff::default()
    }
}

/// 🏗️ Objects-move field delta.
pub fn diff_objects_move(id: &str, to_index: usize, base: &LowpolySnapshot) -> LowpolyDiff {
    let mut order: Vec<String> = base.objects.iter().map(|object| object.id.clone()).collect();
    if let Some(from) = order.iter().position(|existing| existing == id) {
        let moved = order.remove(from);
        let at = to_index.min(order.len());
        order.insert(at, moved);
    }
    LowpolyDiff {
        objects: Some(LowpolyObjectsDelta {
            added: Vec::new(),
            removed: Vec::new(),
            patched: Vec::new(),
            reordered: Some(order),
        }),
        ..LowpolyDiff::default()
    }
}

/// 🏗️ Objects-patch field delta.
pub fn diff_objects_patch(id: String, patch: crate::artifacts::lowpoly::LowpolyObjectPatch) -> LowpolyDiff {
    LowpolyDiff {
        objects: Some(LowpolyObjectsDelta {
            added: Vec::new(),
            removed: Vec::new(),
            patched: vec![LowpolyObjectPatchEntry { id, patch, paint_layers: None }],
            reordered: None,
        }),
        ..LowpolyDiff::default()
    }
}

/// 🏗️ Add-paint-layer field delta.
pub fn diff_add_paint_layer(object_id: String, index: usize, layer: crate::artifacts::lowpoly::LowpolyPaintLayer) -> LowpolyDiff {
    LowpolyDiff {
        objects: Some(LowpolyObjectsDelta {
            patched: vec![LowpolyObjectPatchEntry {
                id: object_id,
                patch: crate::artifacts::lowpoly::LowpolyObjectPatch::default(),
                paint_layers: Some(LowpolyPaintLayersDelta {
                    added: vec![crate::artifacts::lowpoly::schema::diff::LowpolyIndexedPaintLayer {
                        index: index as u32,
                        layer,
                    }],
                    ..LowpolyPaintLayersDelta::default()
                }),
            }],
            ..LowpolyObjectsDelta::default()
        }),
        ..LowpolyDiff::default()
    }
}

/// 🏗️ Remove-paint-layer field delta.
pub fn diff_remove_paint_layer(object_id: String, index: usize) -> LowpolyDiff {
    LowpolyDiff {
        objects: Some(LowpolyObjectsDelta {
            patched: vec![LowpolyObjectPatchEntry {
                id: object_id,
                patch: crate::artifacts::lowpoly::LowpolyObjectPatch::default(),
                paint_layers: Some(LowpolyPaintLayersDelta {
                    removed: vec![index as u32],
                    ..LowpolyPaintLayersDelta::default()
                }),
            }],
            ..LowpolyObjectsDelta::default()
        }),
        ..LowpolyDiff::default()
    }
}

/// 🏗️ Patch-paint-layer field delta.
pub fn diff_patch_paint_layer(
    object_id: String,
    index: usize,
    patch: crate::artifacts::lowpoly::schema::diff::LowpolyPaintLayerPatch,
) -> LowpolyDiff {
    LowpolyDiff {
        objects: Some(LowpolyObjectsDelta {
            patched: vec![LowpolyObjectPatchEntry {
                id: object_id,
                patch: crate::artifacts::lowpoly::LowpolyObjectPatch::default(),
                paint_layers: Some(LowpolyPaintLayersDelta {
                    patched: vec![crate::artifacts::lowpoly::schema::diff::LowpolyIndexedPaintLayerPatch {
                        index: index as u32,
                        patch,
                    }],
                    ..LowpolyPaintLayersDelta::default()
                }),
            }],
            ..LowpolyObjectsDelta::default()
        }),
        ..LowpolyDiff::default()
    }
}

/// 🏗️ Paint-stroke field delta.
pub fn diff_paint_stroke(object_id: String, layer_index: usize, runs: Vec<SchemaPixelRun>) -> LowpolyDiff {
    LowpolyDiff {
        objects: Some(LowpolyObjectsDelta {
            patched: vec![LowpolyObjectPatchEntry {
                id: object_id,
                patch: crate::artifacts::lowpoly::LowpolyObjectPatch::default(),
                paint_layers: Some(LowpolyPaintLayersDelta {
                    strokes: vec![LowpolyPaintStrokeAt { layer_index: layer_index as u32, runs }],
                    ..LowpolyPaintLayersDelta::default()
                }),
            }],
            ..LowpolyObjectsDelta::default()
        }),
        ..LowpolyDiff::default()
    }
}

/// 🏗️ Whole snapshot replacement via schema+objects (clears then adds).
pub fn diff_replace_snapshot(before: &LowpolySnapshot, after: &LowpolySnapshot) -> LowpolyDiff {
    LowpolyDiff {
        schema: (before.schema != after.schema).then(|| after.schema.clone()),
        objects: Some(LowpolyObjectsDelta {
            added: after.objects.clone(),
            removed: before.objects.iter().map(|object| object.id.clone()).collect(),
            patched: Vec::new(),
            reordered: Some(after.objects.iter().map(|object| object.id.clone()).collect()),
        }),
        ..LowpolyDiff::default()
    }
}
//#endregion 🔖️Constructors
