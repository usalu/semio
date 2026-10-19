//! 🪟️ `setView`: sets one named view parameter of the addressed window, persisted in that window's own config: the plan and section windows' authored view, the world's projection, storey
//! isolation and visibility and section plane. It never touches the document. Choosing a plan view shares its storey as the working storey in presence.

use semio_s_artifact_bim_model_kit::editor::bim::kit::fault;
use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::{plan, section, sheet, world};
use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context::BimDispatchCtx;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::inferences::phase_visibility::ViewPhase;
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone)]
#[dsl(keyword = "set-view")]
pub struct SetView {
    pub field: String,
    pub value: String,
    pub pressed: Option<bool>,
}

fn number(payload: &SetView) -> Result<f64, Fault> {
    payload.value.trim().parse::<f64>().ok().filter(|value| value.is_finite()).ok_or_else(|| fault("bim.view.value-invalid", format!("'{}' is not a number", payload.value)))
}

fn flag(payload: &SetView) -> Result<bool, Fault> {
    payload.pressed.map_or_else(|| payload.value.trim().parse::<bool>().map_err(|_| fault("bim.view.value-invalid", format!("'{}' is not a flag", payload.value))), Ok)
}

fn unknown(window: &str, field: &str) -> Fault {
    fault("bim.view.field-unknown", format!("the window '{window}' has no view parameter '{field}'"))
}

fn membership_view(payload: &SetView, snapshot: &ModelSnapshot, options: &mut std::collections::BTreeMap<String, String>, worksets: &mut std::collections::BTreeMap<String, bool>) -> Result<bool, Fault> {
    if let Some(group) = payload.field.strip_prefix("option:") {
        if !snapshot.option_groups.contains_key(group) || !payload.value.is_empty() && !snapshot.design_options.get(&payload.value).is_some_and(|option| option.group == group) { return Err(fault("bim.view.option-missing", "Option must belong to the addressed group")); }
        options.remove(group);
        if !payload.value.is_empty() { options.insert(group.to_owned(), payload.value.clone()); }
        return Ok(true);
    }
    if let Some(workset) = payload.field.strip_prefix("workset:") {
        if !snapshot.worksets.contains_key(workset) { return Err(fault("bim.view.workset-missing", "Workset is missing")); }
        worksets.insert(workset.to_owned(), flag(payload)?);
        return Ok(true);
    }
    Ok(false)
}

fn plan_view(payload: &SetView, snapshot: &ModelSnapshot, ctx: &mut BimDispatchCtx) -> Result<semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan::config::BimPlanWindowConfig, Fault> {
    let mut config = ctx.plan.clone();
    if membership_view(payload, snapshot, &mut config.selected_options, &mut config.workset_visibility)? { return Ok(config); }
    match payload.field.as_str() {
        "view" => {
            let storey = snapshot.views.get(&payload.value).filter(|view| view.kind.is_plan()).and_then(|view| view.storey.clone()).ok_or_else(|| fault("bim.view.view-missing", format!("no plan view '{}'", payload.value)))?;
            config.view = payload.value.clone();
            config.framed = false;
            ctx.presence_out.push(ctx.presence.on_storey(&storey));
        }
        other => return Err(unknown(semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan::WINDOW_KIND_ID, other)),
    }
    Ok(config)
}

fn world_view(payload: &SetView, snapshot: &ModelSnapshot, ctx: &BimDispatchCtx) -> Result<semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world::config::BimWorldWindowConfig, Fault> {
    let mut config = ctx.world.clone();
    if membership_view(payload, snapshot, &mut config.selected_options, &mut config.workset_visibility)? { return Ok(config); }
    match payload.field.as_str() {
        "projection" => {
            if !semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world::PROJECTION_KINDS.contains(&payload.value.as_str()) {
                return Err(fault("bim.view.value-invalid", format!("'{}' is not a projection of the 3D window", payload.value)));
            }
            config.projection.kind = payload.value.clone();
            config.framed = false;
        }
        "isolated_storey" => {
            if !payload.value.is_empty() && !snapshot.storeys.contains_key(&payload.value) {
                return Err(fault("bim.view.storey-missing", format!("no storey '{}'", payload.value)));
            }
            config.isolated_storey = payload.value.clone();
        }
        "hidden_storey" => {
            let hide = payload.pressed.unwrap_or_else(|| !config.hidden_storeys.contains(&payload.value));
            config.hidden_storeys.retain(|storey| storey != &payload.value);
            if hide {
                config.hidden_storeys.push(payload.value.clone());
            }
        }
        "view_phase" => {
            let phase = ViewPhase::parse(&payload.value).ok_or_else(|| fault("bim.view.value-invalid", format!("'{}' is not a phase filter", payload.value)))?;
            config.view_phase = phase.key().to_string();
        }
        "section_enabled" => config.section_enabled = flag(payload)?,
        "section_axis" if matches!(payload.value.as_str(), "x" | "y" | "z") => config.section_axis = payload.value.clone(),
        "section_axis" => return Err(fault("bim.view.value-invalid", format!("'{}' is not an axis", payload.value))),
        "section_offset" => config.section_offset = number(payload)?,
        "energy_overlay" => config.energy_overlay = flag(payload)?,
        "structural_overlay" => config.structural_overlay = flag(payload)?,
        "energy_mode" => {
            let mode = semio_s_artifact_bim_model_modify::render::envelope::Mode::parse(&payload.value).ok_or_else(|| fault("bim.view.value-invalid", format!("'{}' is not an envelope overlay mode", payload.value)))?;
            config.energy_mode = mode.key().to_string();
        }
        other => return Err(unknown(semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world::WINDOW_KIND_ID, other)),
    }
    Ok(config)
}

fn section_view(payload: &SetView, snapshot: &ModelSnapshot, ctx: &BimDispatchCtx) -> Result<semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::section::config::BimSectionWindowConfig, Fault> {
    let mut config = ctx.section.clone();
    if membership_view(payload, snapshot, &mut config.selected_options, &mut config.workset_visibility)? { return Ok(config); }
    match payload.field.as_str() {
        "view" => {
            if !snapshot.views.get(&payload.value).is_some_and(|view| view.kind.is_vertical()) {
                return Err(fault("bim.view.view-missing", format!("no section or elevation view '{}'", payload.value)));
            }
            config.view = payload.value.clone();
            config.framed = false;
        }
        other => return Err(unknown(semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::section::WINDOW_KIND_ID, other)),
    }
    Ok(config)
}

fn schedule_view(payload: &SetView, snapshot: &ModelSnapshot, ctx: &BimDispatchCtx) -> Result<semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::schedule::config::BimScheduleWindowConfig, Fault> {
    let mut config = ctx.schedule.clone();
    match payload.field.as_str() {
        "schedule" => {
            if !payload.value.is_empty() && !snapshot.schedules.contains_key(&payload.value) {
                return Err(fault("bim.view.schedule-missing", format!("no schedule '{}'", payload.value)));
            }
            config.schedule = payload.value.clone();
            config.editing = false;
        }
        "editing" => config.editing = flag(payload)? && snapshot.schedules.contains_key(&config.schedule),
        other => return Err(unknown(semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::schedule::WINDOW_KIND_ID, other)),
    }
    Ok(config)
}

fn sheet_view(payload: &SetView, snapshot: &ModelSnapshot, ctx: &BimDispatchCtx) -> Result<semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::sheet::config::BimSheetWindowConfig, Fault> {
    let mut config = ctx.sheet.clone();
    match payload.field.as_str() {
        "sheet" => {
            if !snapshot.sheets.contains_key(&payload.value) {
                return Err(fault("bim.view.view-missing", format!("no sheet '{}'", payload.value)));
            }
            config.sheet = payload.value.clone();
            config.framed = false;
        }
        other => return Err(unknown(semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::sheet::WINDOW_KIND_ID, other)),
    }
    Ok(config)
}

pub fn handle(payload: &SetView, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    if payload.field == "workset_owner" {
        if !doc.snapshot.worksets.contains_key(&payload.value) { return Err(fault("bim.view.workset-missing", "Workset is missing")); }
        let claim = payload.pressed.unwrap_or_else(|| !ctx.presence.owned_worksets.contains(&payload.value));
        ctx.presence_out.push(ctx.presence.claim_workset(&payload.value, claim));
        return Ok(Emit::default());
    }
    let view = ctx.view.clone().ok_or_else(|| fault("bim.view.window-required", "a view parameter belongs to one open window"))?;
    let mut emit = Emit::default();
    let mutation = match ctx.window_kind.as_str() {
        semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan::WINDOW_KIND_ID => semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan::config::addressed(&view, plan_view(payload, doc.snapshot, ctx)?)?,
        semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world::WINDOW_KIND_ID => semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world::config::addressed(&view, world_view(payload, doc.snapshot, ctx)?)?,
        semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::section::WINDOW_KIND_ID => semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::section::config::addressed(&view, section_view(payload, doc.snapshot, ctx)?)?,
        semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::schedule::WINDOW_KIND_ID => semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::schedule::config::addressed(&view, schedule_view(payload, doc.snapshot, ctx)?)?,
        semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::sheet::WINDOW_KIND_ID => semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::sheet::config::addressed(&view, sheet_view(payload, doc.snapshot, ctx)?)?,
        other => return Err(fault("bim.view.window-unsupported", format!("the window '{other}' has no view parameters"))),
    };
    emit.window_config_mutations.push(mutation);
    Ok(emit)
}


