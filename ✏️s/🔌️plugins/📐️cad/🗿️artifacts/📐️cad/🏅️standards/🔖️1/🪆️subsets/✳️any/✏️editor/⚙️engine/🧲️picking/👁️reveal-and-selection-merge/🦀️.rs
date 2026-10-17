//! 👁️ Reveal and selection merging of the CAD pick engine: the revealed objects, the render targets and the modifier-mode selection merges. See [`super`].

use super::*;

//#region 🔖️Reveal
/// 👁️ Object ids whose kernel primitives should draw (hover/selection on the object or its topology).
pub fn revealed_object_ids_from_pick_keys(object_index: &BTreeMap<String, String>, hovered_target_key: Option<&str>, selected_target_keys: &BTreeSet<String>) -> BTreeSet<String> {
    let mut revealed = BTreeSet::new();
    let mut consider = |key: Option<&str>| {
        for alias in spatial_hover_key_aliases(key) {
            if let Some(owner) = object_index.get(&alias) {
                revealed.insert(owner.clone());
            }
        }
    };
    consider(hovered_target_key);
    for key in selected_target_keys {
        consider(Some(key));
    }
    revealed
}

fn spatial_pick_target_object_revealed(target: &SpatialPickTarget, object_index: &BTreeMap<String, String>, revealed_object_ids: &BTreeSet<String>) -> bool {
    if target.kind == SpatialPickTargetKind::Object && target.geometry_kind.is_none() {
        return true;
    }
    object_index.get(&spatial_pick_target_member_key(target)).is_some_and(|owner| revealed_object_ids.contains(owner))
}

/// 📌️ Visibility-enabled pick highlights plus pinned hover/selection targets (pinned win over hidden).
pub fn resolve_spatial_pick_targets_to_render(view_targets: &[SpatialPickTarget], filter_kind_toggles: SpatialPickKindToggles, pinned_target_keys: &BTreeSet<String>, flags_for_id: &impl Fn(&str) -> SpatialEntityFlags, object_reveal: Option<(&BTreeMap<String, String>, &BTreeSet<String>)>) -> Vec<SpatialPickTarget> {
    let pinned_keys = pinned_pick_target_keys(pinned_target_keys.iter().map(String::as_str));
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut out: Vec<SpatialPickTarget> = Vec::new();
    for target in view_targets.iter().filter(|target| filter_kind_toggles.enabled(target.kind)) {
        let key = spatial_pick_target_key(target);
        if flags_for_id(&target.id).hidden && !pinned_keys.contains(&key) {
            continue;
        }
        if let Some((object_index, revealed)) = object_reveal {
            if !pinned_keys.contains(&key) && !spatial_pick_target_object_revealed(target, object_index, revealed) {
                continue;
            }
        }
        if !seen.insert(key) {
            continue;
        }
        out.push(target.clone());
    }
    for target in view_targets {
        let key = spatial_pick_target_key(target);
        if !pinned_keys.contains(&key) || seen.contains(&key) {
            continue;
        }
        seen.insert(key);
        out.push(target.clone());
    }
    out
}
//#endregion 🔖️Reveal

//#region 🔖️Selection
/// 🖱️ The selection merge a pointer's modifiers ask for — the framework's one wire vocabulary.
pub fn spatial_merge_mode_from_modifiers(shift: bool, ctrl: bool, meta: bool) -> MergeMode {
    let ctrl = ctrl || meta;
    match (shift, ctrl) {
        (true, true) => MergeMode::Invertive,
        (true, false) => MergeMode::Additive,
        (false, true) => MergeMode::Subtractive,
        (false, false) => MergeMode::Replace,
    }
}

/// 🖱️ Deduplicates selection rows by `kind:id`, first occurrence wins.
pub fn unique_selection_targets(targets: Vec<SelectionTarget>) -> Vec<SelectionTarget> {
    let mut seen: BTreeSet<String> = BTreeSet::new();
    targets.into_iter().filter(|target| seen.insert(selection_target_hover_key(target))).collect()
}

/// 🖱️ Applies a merge mode to the current selection. `Range` has no ordered topology on a spatial
/// canvas, so it behaves as `Replace` — the same choice `marqueeModeFromModifiers` makes by never
/// producing it.
pub fn merge_selection_targets(current: &[SelectionTarget], next: Vec<SelectionTarget>, mode: MergeMode) -> Vec<SelectionTarget> {
    let unique_next = unique_selection_targets(next);
    let next_keys: BTreeSet<String> = unique_next.iter().map(selection_target_hover_key).collect();
    match mode {
        MergeMode::Replace | MergeMode::Range => unique_next,
        MergeMode::Additive => {
            let mut seen: BTreeSet<String> = current.iter().map(selection_target_hover_key).collect();
            let mut merged = current.to_vec();
            for target in unique_next {
                if seen.insert(selection_target_hover_key(&target)) {
                    merged.push(target);
                }
            }
            merged
        }
        MergeMode::Subtractive => current.iter().filter(|target| !next_keys.contains(&selection_target_hover_key(target))).cloned().collect(),
        MergeMode::Invertive => {
            let current_keys: BTreeSet<String> = current.iter().map(selection_target_hover_key).collect();
            let mut merged: Vec<SelectionTarget> = current.iter().filter(|target| !next_keys.contains(&selection_target_hover_key(target))).cloned().collect();
            merged.extend(unique_next.into_iter().filter(|target| !current_keys.contains(&selection_target_hover_key(target))));
            merged
        }
    }
}

/// 🖱️ The committed selection row a pick target resolves to.
pub fn spatial_selection_target(target: &SpatialPickTarget) -> SelectionTarget {
    if target.kind == SpatialPickTargetKind::Object && target.geometry_kind.is_none() {
        return SelectionTarget { kind: ModelEntityKind::Object, id: target.id.clone(), editable: false };
    }
    SelectionTarget { kind: kernel_geometry_kind_for_object_pick(target.kind, target.geometry_kind), id: target.id.clone(), editable: true }
}

/// 🖱️ Maps a committed selection row to the canvas hover key (the typology object when one owns it).
pub fn canvas_hover_key_for_selection_target(objects: &[CadObject], geometry: &CadGeometry, model_definition_id: &str, target: &SelectionTarget) -> String {
    if target.kind == ModelEntityKind::Object {
        return selection_target_hover_key(target);
    }
    let buckets = geometry_buckets(geometry);
    let member_key = format!("{}:{}", target.kind.as_str(), target.id);
    for object in list_model_objects_for_model_definition(objects, model_definition_id) {
        for (kind, primitive_ref) in object_primitive_entries(object) {
            if target.kind == kind && target.id == primitive_ref {
                return selection_target_hover_key(&SelectionTarget { kind: ModelEntityKind::Object, id: object.id.clone(), editable: false });
            }
            if kind == ModelEntityKind::Solid && buckets.solid_primitive_member_ids(primitive_ref).contains(&member_key) {
                return selection_target_hover_key(&SelectionTarget { kind: ModelEntityKind::Object, id: object.id.clone(), editable: false });
            }
        }
    }
    selection_target_hover_key(target)
}
//#endregion 🔖️Selection
