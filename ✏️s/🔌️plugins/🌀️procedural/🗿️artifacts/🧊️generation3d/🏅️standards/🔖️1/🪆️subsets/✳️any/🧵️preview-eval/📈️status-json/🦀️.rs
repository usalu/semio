//! 📈️ Status of the preview evaluation: the status, scene and progress JSON. See [`super`].

use super::*;

//#region 📈️Status
/// 📈️ The ONE status object EVERY generation3d World3d preview window publishes — edit mode's
/// `procedural-preview`, generate mode's `generation3d-generate-preview` and the viewer's
/// `procedural-view-preview`. One projection, one schema, one place.
///
/// 🐛️ It used to live on `✏️editor`, so the viewer's preview published a `World3dScene` with
/// `status_json: None`: meshes rendered, but `data-status-json` carried no `phase`, no `progress`
/// and no `cancellable`, i.e. a read-only surface could watch nothing and stop nothing
/// (`🗑️generated/journey-7/results.json`, rows `view:*`, ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
/// A viewer file may not reach through `::editor::` (`policyViewerPurityBreaches`), so the fix is
/// the same one the tick chain got: the projection moves HERE and both surfaces bind it.
///
/// 🔒️ `session` is OPTIONAL because a surface's marks-free `render` entry point is handed no
/// retained session at all. A window with no session still publishes the full contract — idle
/// phase, zero progress, nothing to abort — rather than the empty status that was the whole defect.

/// 📈️ The per-widget half: the evaluation's own `error`, or the `widgetErrors` map of every widget
/// whose evaluation carries one. Surface-neutral — it reads the evaluation text and the fixture the
/// surface is looking at, nothing else.
pub fn preview_status_json(eval_json: &str, host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot) -> Option<String> {
    let eval = semio_framework_pack_json::parse(eval_json, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()?;
    if eval.get("error").and_then(semio_framework_pack_json::Value::as_str).is_some() {
        let mut error_object = semio_framework_pack_json::Object::new();
        error_object.insert("error", eval.get("error").cloned().unwrap_or(semio_framework_pack_json::Value::Null));
        return Some(semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Object(error_object)));
    }
    let mut errors = semio_framework_pack_json::Object::new();
    for widget in &host_snapshot.widgets {
        let id = semio_s_artifact_procedural_3d_model::widget_id(widget).to_string();
        let Some(entry) = eval.get(&id) else { continue };
        if let Some(error) = entry.get("error").and_then(semio_framework_pack_json::Value::as_str) {
            errors.insert(id, semio_framework_pack_json::Value::String(error.to_string()));
        }
    }
    if errors.is_empty() {
        None
    } else {
        let mut wrapper = semio_framework_pack_json::Object::new();
        wrapper.insert("widgetErrors", semio_framework_pack_json::Value::Object(errors));
        Some(semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Object(wrapper)))
    }
}

/// 🧩️ Folds one status object's keys onto another's, right-hand side winning.
fn merge_status_json(computing: Option<String>, preview_status: Option<String>) -> Option<String> {
    match (computing, preview_status) {
        (Some(c), Some(p)) => {
            let mut computing_object = semio_framework_pack_json::parse(&c, semio_framework_pack_json::JsonMemberPolicy::Reject).ok().and_then(|value| value.as_object().cloned()).unwrap_or_else(|| {
                let mut fallback = semio_framework_pack_json::Object::new();
                fallback.insert("computing", semio_framework_pack_json::Value::Bool(true));
                fallback
            });
            let preview_object = semio_framework_pack_json::parse(&p, semio_framework_pack_json::JsonMemberPolicy::Reject).ok().and_then(|value| value.as_object().cloned()).unwrap_or_default();
            for (key, value) in preview_object.iter() {
                computing_object.insert(key, value.clone());
            }
            Some(semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Object(computing_object)))
        }
        (Some(c), None) => Some(c),
        (None, Some(p)) => Some(p),
        (None, None) => None,
    }
}

/// 👁️ Merges the session's live "still computing" flag, the resumable tessellation's progress and a
/// fresh [`preview_status_json`] result into the one status object a preview window publishes.
pub fn preview_scene_status_json(session: Option<&FlowEvalSession>, run: Option<&ToolRunView>, preview_status: Option<String>) -> Option<String> {
    let chain = session.map(FlowEvalSession::preview_chain_status).unwrap_or_default();
    let computing = (chain.working || (run.is_some_and(preview_eval_run_is_abortable) && !chain.settled())).then(|| r#"{"computing":true}"#.to_string());
    merge_status_json(merge_status_json(computing, Some(preview_progress_status_json(session, run))), preview_status)
}

/// ⏯️ Whether `run` is this surface's preview evaluation and still has work its abort would stop.
pub fn preview_eval_run_is_abortable(run: &ToolRunView) -> bool {
    run.tool_id == PREVIEW_EVAL_TOOL_ID && matches!(run.state, ToolRunState::Starting | ToolRunState::Running | ToolRunState::Paused)
}

/// 📈 The schema-first tessellation progress object: `phase` (wire tag) and its `phaseLabel`
/// English/German pair, the monotone `progress` counters and ratio, the abort affordance
/// (`cancellable`, `cancelAction` = the framework's `toolRunAbort`, `cancelArgs` = the run's
/// `{runId, generation}`) and any typed validate-gate `diagnostics`.
///
/// 🪪️ An unaddressable geometry kernel outranks whatever the tessellation ledger last recorded:
/// nothing was ever invoked, so the ledger's `idle` is a lie the surface cannot act on. The miss is
/// resolved through the SAME [`geometry_extension_address`] the invocation producer uses — read
/// live, never cached — and published as `phase: "faulted"` plus a `fault` object naming the flow
/// extension id and every `<manifest id> → <owning plugin id>` translation the session does carry.
pub fn preview_progress_status_json(session: Option<&FlowEvalSession>, run: Option<&ToolRunView>) -> String {
    preview_progress_status_json_for(session, run, geometry_extension_address())
}

/// 📈 The projection itself, over an ALREADY RESOLVED geometry address — pure in both its inputs, so
/// the unaddressable branch is provable without mutating the process-global contribution table (a
/// test that uninstalls and reinstalls it poisons the flow catalogue's cache lock and leaves the
/// neural registry unretired for every later test in the binary).
pub fn preview_progress_status_json_for(session: Option<&FlowEvalSession>, run: Option<&ToolRunView>, address: Result<String, semio_framework_os_flow::FlowExtensionAddressMiss>) -> String {
    let status = session.map(FlowEvalSession::preview_tessellate_status).unwrap_or_default();
    let eval_status = session.map(FlowEvalSession::preview_eval_status).unwrap_or_default();
    let chain = session.map(FlowEvalSession::preview_chain_status).unwrap_or_default();
    let phase = if address.is_err() {
        semio_framework_os_flow::PreviewTessellatePhase::Faulted
    } else if session.is_some_and(FlowEvalSession::preview_cancelled) {
        semio_framework_os_flow::PreviewTessellatePhase::Cancelled
    } else {
        status.phase
    };
    let abortable = run.filter(|run| preview_eval_run_is_abortable(run) && address.is_ok() && !chain.settled());
    let show_eval_phase = eval_status.in_flight > 0 && status.in_flight == 0 && matches!(phase, semio_framework_os_flow::PreviewTessellatePhase::Idle);
    let show_chain_phase = !show_eval_phase && chain.working && status.in_flight == 0 && matches!(phase, semio_framework_os_flow::PreviewTessellatePhase::Idle);
    let (phase_tag, english, german) = if show_eval_phase {
        let (english, german) = eval_status.phase.labels();
        (eval_status.phase.tag(), english, german)
    } else if show_chain_phase {
        let (english, german) = semio_framework_os_flow::PreviewEvalPhase::Computing.labels();
        (semio_framework_os_flow::PreviewEvalPhase::Computing.tag(), english, german)
    } else {
        let (english, german) = phase.labels();
        (phase.tag(), english, german)
    };
    let mut label = semio_framework_pack_json::Object::new();
    label.insert("en", semio_framework_pack_json::Value::String(english.to_string()));
    label.insert("de", semio_framework_pack_json::Value::String(german.to_string()));
    let mut progress = semio_framework_pack_json::Object::new();
    let chain_progress = chain.working && status.in_flight == 0 && matches!(phase, semio_framework_os_flow::PreviewTessellatePhase::Idle) && (chain.nodes_total > 0 || eval_status.in_flight == 0);
    let (units_done, units_total) = if chain_progress {let(done,total)=chain.units();(u64::from(done),u64::from(total))} else { (status.units_done, status.units_total) };
    progress.insert("unitsDone", semio_framework_pack_json::Value::from(u64::from(units_done)));
    progress.insert("unitsTotal", semio_framework_pack_json::Value::from(u64::from(units_total)));
    progress.insert("facesDone", semio_framework_pack_json::Value::from(u64::from(status.faces_done)));
    progress.insert("facesTotal", semio_framework_pack_json::Value::from(u64::from(status.faces_total)));
    progress.insert("inFlight", semio_framework_pack_json::Value::from(u64::from(chain.in_flight.max(status.in_flight.saturating_add(eval_status.in_flight)))));
    progress.insert("evalUnitsDone", semio_framework_pack_json::Value::from(u64::from(eval_status.units_done)));
    progress.insert("evalUnitsTotal", semio_framework_pack_json::Value::from(u64::from(eval_status.units_total)));
    progress.insert("nodesDone", semio_framework_pack_json::Value::from(u64::from(chain.nodes_done)));
    progress.insert("nodesTotal", semio_framework_pack_json::Value::from(u64::from(chain.nodes_total)));
    let ratio = if chain_progress {
        chain.ratio()
    } else if show_eval_phase {
        eval_status.ratio()
    } else {
        status.ratio()
    };
    progress.insert("ratio", semio_framework_pack_json::Value::from(ratio));
    let mut object = semio_framework_pack_json::Object::new();
    object.insert("phase", semio_framework_pack_json::Value::String(phase_tag.to_string()));
    object.insert("phaseLabel", semio_framework_pack_json::Value::Object(label));
    object.insert("progress", semio_framework_pack_json::Value::Object(progress));
    object.insert("cancellable", semio_framework_pack_json::Value::Bool(abortable.is_some()));
    object.insert("cancelAction", semio_framework_pack_json::Value::String(TOOL_RUN_ABORT_ACTION_ID.to_string()));
    if let Some(run) = abortable {
        let mut arguments = semio_framework_pack_json::Object::new();
        arguments.insert(TOOL_RUN_ARG_RUN_ID, semio_framework_pack_json::Value::String(run.identity.id.run.to_string()));
        arguments.insert(TOOL_RUN_ARG_GENERATION, semio_framework_pack_json::Value::from(u64::from(run.identity.generation)));
        object.insert("cancelArgs", semio_framework_pack_json::Value::Object(arguments));
    }
    if let Some(evaluate_fault) = session.and_then(FlowEvalSession::extension_evaluate_fault) {
        let (english, german) = evaluate_fault.labels();
        let mut message = semio_framework_pack_json::Object::new();
        message.insert("en", semio_framework_pack_json::Value::String(english));
        message.insert("de", semio_framework_pack_json::Value::String(german));
        let mut fault = semio_framework_pack_json::Object::new();
        fault.insert("code", semio_framework_pack_json::Value::String(semio_framework_os_flow::ExtensionEvaluateFault::CODE.to_string()));
        fault.insert("extensionId", semio_framework_pack_json::Value::String(evaluate_fault.extension_id.clone()));
        fault.insert("capability", semio_framework_pack_json::Value::String(evaluate_fault.capability.clone()));
        fault.insert("faultCode", semio_framework_pack_json::Value::String(evaluate_fault.code.clone()));
        fault.insert("faultMessage", semio_framework_pack_json::Value::String(evaluate_fault.message.clone()));
        fault.insert("message", semio_framework_pack_json::Value::Object(message));
        let (faulted_en, faulted_de) = semio_framework_os_flow::PreviewTessellatePhase::Faulted.labels();
        let mut faulted_label = semio_framework_pack_json::Object::new();
        faulted_label.insert("en", semio_framework_pack_json::Value::String(faulted_en.to_string()));
        faulted_label.insert("de", semio_framework_pack_json::Value::String(faulted_de.to_string()));
        object.insert("phase", semio_framework_pack_json::Value::String(semio_framework_os_flow::PreviewTessellatePhase::Faulted.tag().to_string()));
        object.insert("phaseLabel", semio_framework_pack_json::Value::Object(faulted_label));
        object.insert("fault", semio_framework_pack_json::Value::Object(fault));
    } else if let Err(miss) = address {
        let (english, german) = miss.labels();
        let mut message = semio_framework_pack_json::Object::new();
        message.insert("en", semio_framework_pack_json::Value::String(english));
        message.insert("de", semio_framework_pack_json::Value::String(german));
        let mut fault = semio_framework_pack_json::Object::new();
        fault.insert("code", semio_framework_pack_json::Value::String(semio_framework_os_flow::FlowExtensionAddressMiss::CODE.to_string()));
        fault.insert("extensionId", semio_framework_pack_json::Value::String(miss.extension_id));
        fault.insert("message", semio_framework_pack_json::Value::Object(message));
        fault.insert(
            "contributed",
            semio_framework_pack_json::Value::Array(
                miss.contributed
                    .into_iter()
                    .map(|(extension_id, plugin_id)| {
                        let mut entry = semio_framework_pack_json::Object::new();
                        entry.insert("extensionId", semio_framework_pack_json::Value::String(extension_id));
                        entry.insert("pluginId", semio_framework_pack_json::Value::String(plugin_id));
                        semio_framework_pack_json::Value::Object(entry)
                    })
                    .collect(),
            ),
        );
        object.insert("fault", semio_framework_pack_json::Value::Object(fault));
    }
    if status.diagnostics > 0 {
        object.insert("diagnosticCount", semio_framework_pack_json::Value::from(u64::from(status.diagnostics)));
        let entries: Vec<semio_framework_pack_json::Value> = session
            .map(FlowEvalSession::preview_diagnostic_entries)
            .unwrap_or_default()
            .into_iter()
            .map(|(handle, issues)| {
                let mut entry = semio_framework_pack_json::Object::new();
                entry.insert("handle", semio_framework_pack_json::Value::String(handle.to_string()));
                entry.insert("issues", semio_framework_pack_json::parse(issues, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or(semio_framework_pack_json::Value::Array(Vec::new())));
                semio_framework_pack_json::Value::Object(entry)
            })
            .collect();
        object.insert("diagnostics", semio_framework_pack_json::Value::Array(entries));
    }
    semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Object(object))
}

/// 🐞️ The observable counters a preview window stamps onto its status so the browser probe can read
/// back off `data-status-json` what actually reached the SCENE, without a screenshot.
///
/// 🧹️ `eval_json` is deliberately absent. It used to ride along as `evalLen` plus an `evalHead` of
/// the first 240 characters of the raw evaluation text — on EVERY status frame of every preview
/// window, in the payload `World3dHost` renders progress chrome from. Neither is a fact about the
/// scene: the evaluation's own failures are already projected as `error`/`widgetErrors` by
/// [`preview_status_json`], and its progress as `phase`/`progress` by
/// [`preview_progress_status_json`], so the pair was pure debug spill that also leaked document text
/// into view state (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️audit-user-journey-gaps-2026-09-13.md` §9 item 17).
pub struct PreviewStatusDebug<'a> {
    pub meshes_json: &'a str,
    pub instances_json: &'a str,
}

/// 📈️ THE preview-window status projection: progress + widget errors + debug counters + the
/// optional authored hint a window shows while it has nothing to paint. All three World3d preview
/// windows publish exactly this, so a probe, a shell pane and a law can name one shape.
pub fn preview_window_status_json(session: Option<&FlowEvalSession>, run: Option<&ToolRunView>, widget_status: Option<String>, debug: &PreviewStatusDebug<'_>, hint: Option<&str>) -> Option<String> {
    let base = preview_scene_status_json(session, run, widget_status);
    let mut object = base.as_deref().and_then(|text| semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()).and_then(|value| value.as_object().cloned()).unwrap_or_else(semio_framework_pack_json::Object::new);
    let mut debug_object = semio_framework_pack_json::Object::new();
    debug_object.insert("meshesLen", semio_framework_pack_json::Value::from(debug.meshes_json.len()));
    debug_object.insert("instancesLen", semio_framework_pack_json::Value::from(debug.instances_json.len()));
    object.insert("debug", semio_framework_pack_json::Value::Object(debug_object));
    if let Some(hint) = hint {
        object.insert("hint", semio_framework_pack_json::Value::String(hint.to_string()));
    }
    Some(semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Object(object)))
}
//#endregion 📈️Status
