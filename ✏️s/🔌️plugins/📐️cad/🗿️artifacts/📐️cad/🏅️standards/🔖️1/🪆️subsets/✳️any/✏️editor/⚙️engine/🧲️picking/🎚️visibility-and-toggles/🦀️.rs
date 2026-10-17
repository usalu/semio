//! 🎚️ Visibility and toggles of the CAD pick engine: the entity flags, kind, typology and primitive toggles and the scene visibility. See [`super`].

use super::*;

//#region 🔖️Toggles
/// 👁️ All geometry pick kinds enabled.
pub fn default_spatial_pick_kind_toggles() -> SpatialPickKindToggles {
    SpatialPickKindToggles { object: Some(true), face: Some(true), edge: Some(true), vertex: Some(true) }
}

/// 👁️ Filters pick targets by visibility (show/hide highlights); never affects ray pick or selection.
pub fn filter_spatial_pick_targets_for_visibility(targets: Vec<SpatialPickTarget>, filter_kind_toggles: SpatialPickKindToggles) -> Vec<SpatialPickTarget> {
    targets.into_iter().filter(|target| filter_kind_toggles.enabled(target.kind)).collect()
}

/// 👁️ Hide/lock flags authored on a pane object row.
pub fn spatial_entity_flags_for_object(objects: &[CadObject], entity_id: &str) -> SpatialEntityFlags {
    objects.iter().find(|object| object.id == entity_id).map_or(SpatialEntityFlags::default(), |object| SpatialEntityFlags { hidden: !object.visible, locked: object.locked })
}

/// 👁️ A kernel member inherits the hide/lock state of the object row that owns it — the owner is
/// looked up through the same fixed alias order React uses (`object`, `solid`, `face`, `edge`,
/// `vertex`, `anchor`), and only a `true` flag on the owner is inherited.
pub fn resolve_spatial_entity_flags(objects: &[CadObject], geometry: &CadGeometry, model_definition_id: &str, entity_id: &str) -> SpatialEntityFlags {
    let direct = spatial_entity_flags_for_object(objects, entity_id);
    if direct.hidden || direct.locked {
        return direct;
    }
    let index = build_geometry_object_index(objects, geometry, model_definition_id);
    let Some(owner) = ["object", "solid", "face", "edge", "vertex", "anchor"].into_iter().find_map(|kind| index.get(&format!("{kind}:{entity_id}"))) else {
        return direct;
    };
    let owner_flags = spatial_entity_flags_for_object(objects, owner);
    if !owner_flags.hidden && !owner_flags.locked {
        return direct;
    }
    SpatialEntityFlags { hidden: direct.hidden || owner_flags.hidden, locked: direct.locked || owner_flags.locked }
}

/// 👁️ Excludes hidden/locked entities from canvas pick and selection.
pub fn filter_spatial_pick_targets_for_entity_flags(targets: Vec<SpatialPickTarget>, flags_for_id: &impl Fn(&str) -> SpatialEntityFlags) -> Vec<SpatialPickTarget> {
    targets.into_iter().filter(|target| flags_for_id(&target.id).selectable()).collect()
}

/// 👁️ Drops locked/hidden entities from committed selection targets.
pub fn prune_selection_targets_for_entity_flags(targets: Vec<SelectionTarget>, flags_for_id: &impl Fn(&str) -> SpatialEntityFlags) -> Vec<SelectionTarget> {
    targets.into_iter().filter(|target| flags_for_id(&target.id).selectable()).collect()
}

/// 👁️ Effective pick kinds must be both visible AND enabled for selection/hover.
pub fn intersect_spatial_pick_kind_toggles(visible: SpatialPickKindToggles, selection: SpatialPickKindToggles) -> SpatialPickKindToggles {
    let mut merged = SpatialPickKindToggles::default();
    for kind in SPATIAL_PICK_TARGET_KINDS {
        if visible.get(*kind) == Some(false) || selection.get(*kind) == Some(false) {
            merged.set(*kind, Some(false));
        }
    }
    merged
}

/// 👁️ Pick kinds the active model definition's entity kinds map onto.
pub fn model_definition_pick_target_kinds(model_definition_id: Option<&str>) -> Vec<SpatialPickTargetKind> {
    let fallback: &str = default_model_definition_id();
    let md_id = model_definition_id.unwrap_or(fallback);
    let mut out: Vec<SpatialPickTargetKind> = Vec::new();
    for kind in model_definition_selection_entity_kinds(md_id) {
        let mapped = match kind {
            ModelEntityKind::Vertex | ModelEntityKind::Anchor => Some(SpatialPickTargetKind::Vertex),
            ModelEntityKind::Edge | ModelEntityKind::Wire => Some(SpatialPickTargetKind::Edge),
            ModelEntityKind::Face | ModelEntityKind::Shell => Some(SpatialPickTargetKind::Face),
            ModelEntityKind::Solid | ModelEntityKind::Geometry | ModelEntityKind::Object => Some(SpatialPickTargetKind::Object),
            _ => None,
        };
        if let Some(mapped) = mapped {
            if !out.contains(&mapped) {
                out.push(mapped);
            }
        }
    }
    if !out.is_empty() {
        return out;
    }
    if is_shape_model_definition(model_definition_id) {
        SPATIAL_PICK_TARGET_KINDS.to_vec()
    } else {
        vec![SpatialPickTargetKind::Object]
    }
}

/// 👁️ Default visibility/selection toggles for the kinds the active model definition allows.
pub fn default_spatial_pick_kind_toggles_for_model_definition(model_definition_id: Option<&str>) -> SpatialPickKindToggles {
    let allowed = model_definition_pick_target_kinds(model_definition_id);
    let mut toggles = SpatialPickKindToggles::default();
    for kind in SPATIAL_PICK_TARGET_KINDS {
        toggles.set(*kind, Some(allowed.contains(kind)));
    }
    toggles
}

/// 👁️ Every typology on the active model definition enabled for show/selection.
pub fn default_spatial_typology_toggles_for_model_definition(model_definition_id: Option<&str>) -> SpatialTypologyToggles {
    SpatialTypologyToggles(model_definition_typology_ids(model_definition_id).into_iter().map(|id| (id, true)).collect())
}

fn typology_toggle_allows_target(target: &SpatialPickTarget, toggles: &SpatialTypologyToggles, typology_ids: &[String]) -> bool {
    match target.typology_id.as_deref() {
        Some(typology_id) => toggles.enabled(typology_id),
        None => typology_ids.iter().any(|id| toggles.enabled(id)),
    }
}

/// 👁️ Filters pick targets by typology show/selection toggles.
pub fn filter_spatial_pick_targets_for_typology_toggles(targets: Vec<SpatialPickTarget>, toggles: &SpatialTypologyToggles, typology_ids: &[String]) -> Vec<SpatialPickTarget> {
    targets.into_iter().filter(|target| typology_toggle_allows_target(target, toggles, typology_ids)).collect()
}

/// 👁️ Derives per-kind toggles from typology-filtered targets (scene layers + legacy gates).
pub fn spatial_pick_kind_toggles_from_typology_filtered_targets(model_definition_id: Option<&str>, visible_targets: &[SpatialPickTarget]) -> SpatialPickKindToggles {
    let allowed = model_definition_pick_target_kinds(model_definition_id);
    let mut merged = SpatialPickKindToggles::default();
    for kind in SPATIAL_PICK_TARGET_KINDS {
        merged.set(*kind, Some(allowed.contains(kind) && visible_targets.iter().any(|target| target.kind == *kind)));
    }
    merged
}

/// 🧱️ Kernel primitive kinds toggled in play chrome.
pub const SPATIAL_PRIMITIVE_KINDS: &[ModelEntityKind] = PRIMITIVE_MODEL_ENTITY_KINDS;

/// 👁️ All kernel primitive kinds enabled for show/filter.
pub fn default_spatial_primitive_toggles() -> SpatialPrimitiveToggles {
    SpatialPrimitiveToggles(SPATIAL_PRIMITIVE_KINDS.iter().map(|kind| (*kind, true)).collect())
}

/// 👁️ Scene-layer pick-kind toggles from model definition + primitive show toggles.
pub fn spatial_scene_kind_toggles_for_model_definition(model_definition_id: Option<&str>, primitive_toggles: &SpatialPrimitiveToggles) -> SpatialPickKindToggles {
    let mut toggles = default_spatial_pick_kind_toggles_for_model_definition(model_definition_id);
    let off = |kind: ModelEntityKind| primitive_toggles.0.get(&kind) == Some(&false);
    if off(ModelEntityKind::Vertex) && off(ModelEntityKind::Anchor) {
        toggles.vertex = Some(false);
    }
    if off(ModelEntityKind::Edge) && off(ModelEntityKind::Wire) {
        toggles.edge = Some(false);
    }
    if off(ModelEntityKind::Face) && off(ModelEntityKind::Shell) {
        toggles.face = Some(false);
    }
    if off(ModelEntityKind::Solid) {
        toggles.object = Some(false);
    }
    toggles
}

/// 👁️ Filters pick targets by primitive show/filter toggles (typology object rows pass through).
pub fn filter_spatial_pick_targets_for_primitive_toggles(targets: Vec<SpatialPickTarget>, toggles: &SpatialPrimitiveToggles) -> Vec<SpatialPickTarget> {
    targets
        .into_iter()
        .filter(|target| match pick_target_primitive_kind(target) {
            None => true,
            Some(primitive) => toggles.enabled(primitive),
        })
        .collect()
}

/// ☑️ Whether every key in a toggle group is on, every key is off, or the group is mixed.
pub fn spatial_toggle_group_state(keys: &[String], toggles: &BTreeMap<String, bool>) -> SpatialToggleGroupState {
    if keys.is_empty() {
        return SpatialToggleGroupState::None;
    }
    let on = keys.iter().filter(|key| toggles.get(*key) != Some(&false)).count();
    if on == 0 {
        SpatialToggleGroupState::None
    } else if on == keys.len() {
        SpatialToggleGroupState::All
    } else {
        SpatialToggleGroupState::Partial
    }
}

/// ☑️ Sets every key in a chrome toggle group on or off.
pub fn spatial_toggle_group_fill(keys: &[String], enabled: bool) -> BTreeMap<String, bool> {
    keys.iter().map(|key| (key.clone(), enabled)).collect()
}

/// ☑️ Maps a chrome group aggregate onto the owned tri-state checkbox value.
pub fn spatial_toggle_checkbox_state(state: SpatialToggleGroupState) -> SpatialToggleCheckboxState {
    match state {
        SpatialToggleGroupState::Partial => SpatialToggleCheckboxState::Indeterminate,
        SpatialToggleGroupState::All => SpatialToggleCheckboxState::Checked,
        SpatialToggleGroupState::None => SpatialToggleCheckboxState::Unchecked,
    }
}

/// 👁️ Resolves which scene layers stay visible for geometry edit vs typology object picking.
pub fn resolve_spatial_scene_visibility(active_model_definition_id: Option<&str>, filter_kind_toggles: SpatialPickKindToggles) -> SpatialSceneVisibility {
    let fallback: &str = default_model_definition_id();
    let md_id = active_model_definition_id.unwrap_or(fallback);
    if model_definition_uses_geometry_picking(md_id) {
        return SpatialSceneVisibility {
            show_factory_wireframe: filter_kind_toggles.enabled(SpatialPickTargetKind::Edge),
            show_committed_faces: filter_kind_toggles.enabled(SpatialPickTargetKind::Face) || filter_kind_toggles.enabled(SpatialPickTargetKind::Object),
            show_committed_edges: filter_kind_toggles.enabled(SpatialPickTargetKind::Edge),
        };
    }
    SpatialSceneVisibility { show_factory_wireframe: false, show_committed_faces: false, show_committed_edges: false }
}

/// 👁️ Keeps pick targets the active model definition allows (primitives + typology objects).
pub fn filter_spatial_pick_targets_for_active_view(targets: Vec<SpatialPickTarget>, active_model_definition_id: Option<&str>) -> Vec<SpatialPickTarget> {
    let fallback: &str = default_model_definition_id();
    let md_id = active_model_definition_id.unwrap_or(fallback);
    let allowed_pick_kinds = model_definition_pick_target_kinds(Some(md_id));
    let entity_kinds = model_definition_selection_entity_kinds(md_id);
    let is_shape = is_shape_model_definition(Some(md_id));
    targets
        .into_iter()
        .filter(|target| {
            if !allowed_pick_kinds.contains(&target.kind) {
                return false;
            }
            if target.kind == SpatialPickTargetKind::Object && target.geometry_kind.is_none() {
                return entity_kinds.contains(&ModelEntityKind::Object) && !is_shape;
            }
            entity_kinds.contains(&kernel_geometry_kind_for_object_pick(target.kind, target.geometry_kind))
        })
        .collect()
}
//#endregion 🔖️Toggles
