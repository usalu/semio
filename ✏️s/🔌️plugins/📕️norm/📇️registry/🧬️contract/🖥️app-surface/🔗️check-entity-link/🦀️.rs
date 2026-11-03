//! 🔗️ Check and entity link: the bidirectional selection between the checks window and the 3D viewport, driven by the owner every check is filed under (`CheckResult.subject.entity_id`, see [`Owners`](semio_s_plugin_norm_compliance::Owners)). Selecting a check selects and frames its owner entity, selecting an entity narrows the checks list to its checks and scrolls to the governing one; both are ephemeral local view events, never document edits. A check without an owner is document-level and selects the global badge.
//!
//! 🔗️ Selection: [`focus_effect`]; framing: [`fit_selection`](crate::viewport3d::camera::fit_selection); list: [`plan_checks`](crate::plan_checks).

use crate::*;
use crate::viewport3d::camera::{fit_selection, CameraPose};
use crate::viewport3d::config::CameraPreset;
use crate::viewport3d::picking::{entity_address, granularity};
use semio_framework::ShellSelectTarget;
use semio_framework_value::DslValue;
use semio_s_plugin_norm_structural_scene::projection_kit::{base_id, GLOBAL_BADGE};
use semio_s_plugin_norm_structural_scene::StructuralScene;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

/// 🧭️ What a check belongs to: the entity it is filed under, or the document as a whole.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CheckOwner {
    Entity(String),
    Global,
}

/// 🎯️ The view effects of selecting one check: the interaction selection to write and the camera that frames the owner (`None` keeps the camera, as for a global check).
#[derive(Clone, Debug, PartialEq)]
pub struct CheckFocus {
    pub owner: CheckOwner,
    pub targets: Vec<ShellSelectTarget>,
    pub camera: Option<CameraPose>,
}

/// 📋️ The checks list for a selected entity: the rows that belong to it in list order, and the row to scroll to (the governing one, the highest utilisation).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityChecks {
    pub rows: Vec<usize>,
    pub scroll_to: Option<usize>,
}

/// 🧭️ The owner of check `index`; `None` when the report has no such check.
pub fn check_owner(report: &CheckReport, index: usize) -> Option<CheckOwner> {
    let id = report.checks.get(index)?.subject.entity_id.as_str();
    Some(if id.is_empty() { CheckOwner::Global } else { CheckOwner::Entity(base_id(id).to_owned()) })
}

/// 🎯️ Selecting check `index`: the scene entity of its owner is selected (the entity whose id is the owner, else the first of that owner) and all entities of the owner are framed; a global check selects the global badge and keeps the camera.
pub fn focus_of_check(report: &CheckReport, scene: &StructuralScene, index: usize, preset: CameraPreset) -> Option<CheckFocus> {
    let owner = check_owner(report, index)?;
    let target = |kind: &str, id: &str| crate::protocol::wire::encode_scoped_identity(&[kind, id]).ok().map(|id| ShellSelectTarget { granularity: kind.to_owned(), id });
    let (targets, camera) = match &owner {
        CheckOwner::Global => (scene.annotations.iter().any(|a| a.id() == GLOBAL_BADGE).then(|| target("annotation", GLOBAL_BADGE)).flatten().into_iter().collect(), None),
        CheckOwner::Entity(id) => {
            let owned: Vec<&semio_s_plugin_norm_structural_scene::SceneEntity> = scene.entities.iter().filter(|e| base_id(&e.entity.id) == id).collect();
            let main = owned.iter().find(|e| e.entity.id == *id).or(owned.first());
            let targets = main.and_then(|e| entity_address(e.entity.kind, &e.entity.id).map(|address| ShellSelectTarget { granularity: granularity(e.entity.kind).to_owned(), id: address })).into_iter().collect();
            let ids: Vec<String> = owned.iter().map(|e| e.entity.id.clone()).collect();
            (targets, (!ids.is_empty()).then(|| fit_selection(scene, &ids, preset)))
        }
    };
    Some(CheckFocus { owner, targets, camera })
}

/// 🫧️ The selection write of a [`CheckFocus`] for the interaction `domain`, raised by the click on a check row as the framework's own `interactionSelect` (replace, pick): view state only; `None` for a document-level check or an owner without a drawn entity.
pub fn focus_effect(domain: &str, focus: &CheckFocus) -> Option<semio_framework_plugin::Effect> {
    if focus.targets.is_empty() || focus.owner == CheckOwner::Global {
        return None;
    }
    let targets = semio_framework_pack_json::to_json_string(&DslValue::Array(focus.targets.iter().map(|target| DslValue::object(vec![("granularity".to_string(), DslValue::String(target.granularity.clone())), ("id".to_string(), DslValue::String(target.id.clone()))])).collect()));
    Some(semio_framework_plugin::Effect::ReplayShellCommand {
        action_id: semio_framework::INTERACTION_SELECT_ACTION_ID.into(),
        args: Some(DslValue::object(vec![("domainId".to_string(), DslValue::String(domain.to_owned())), ("targets".to_string(), DslValue::String(targets)), ("merge".to_string(), DslValue::String("replace".to_owned())), ("method".to_string(), DslValue::String("pick".to_owned()))])),
    })
}

/// 📋️ Selecting the owner `owner` in the viewport: the checks filed under it that the list state admits, in the list order of `state`; the global badge (or no entity) selects the document-level checks.
pub fn checks_of_owner(report: &CheckReport, owner: &CheckOwner, state: &ChecksViewState, locale: Locale) -> EntityChecks {
    let owns = |index: usize| {
        let id = report.checks[index].subject.entity_id.as_str();
        match owner {
            CheckOwner::Global => id.is_empty(),
            CheckOwner::Entity(owner) => !id.is_empty() && base_id(id) == owner,
        }
    };
    let rows: Vec<usize> = plan_checks(report, state, locale).into_iter().flat_map(|group| group.rows).filter(|index| owns(*index)).collect();
    let scroll_to = rows.iter().copied().filter(|index| !matches!(report.checks[*index].status, CheckStatus::NotApplicable | CheckStatus::Info)).max_by(|a, b| report.checks[*a].utilization.total_cmp(&report.checks[*b].utilization)).or(rows.first().copied());
    EntityChecks { scroll_to, rows }
}

/// 🧭️ The owner a picked scene id selects: the global badge is the document, every other id is the entity of its base id.
pub fn owner_of_scene_id(id: &str) -> CheckOwner {
    if id == GLOBAL_BADGE { CheckOwner::Global } else { CheckOwner::Entity(base_id(id).to_owned()) }
}

/// 🫧️ The ephemeral view event of an entity selection: the check selected in the list follows the scroll target (`None` leaves the selection).
pub fn entity_selected_event(checks: &EntityChecks) -> Option<LaneEvent> {
    checks.scroll_to.map(|index| LaneEvent::ViewCheck(Some(index as u32)))
}
