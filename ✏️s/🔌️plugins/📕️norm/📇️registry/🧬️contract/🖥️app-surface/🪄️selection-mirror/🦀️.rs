//! 🪄️ Selection mirror: one per-artifact selection (entities picked in the interaction domain plus the selected check) resolved against the settled report, so outliner, viewport highlight, properties and checks always show the same thing.

use crate::*;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

/// 🪄️ The resolved selection of one artifact: entity ids, the base ids of the entities picked in the interaction domain (the checks window narrows to them), the selected check and the field paths it points at.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NormSelection {
    pub entities: Vec<String>,
    pub picked: Vec<String>,
    pub check: Option<usize>,
    pub paths: Vec<String>,
    pub units: Option<UnitSystem>,
}

fn picked_target(address: &str) -> (bool, String) {
    let base = address.split('#').next().unwrap_or(address);
    match crate::protocol::wire::decode_scoped_identity(base) {
        Ok(segments) => match segments.as_slice() {
            [kind, id] => (*kind == viewport3d::picking::GRANULARITY_CHECK, (*id).to_string()),
            _ => (false, base.to_string()),
        },
        Err(_) => (false, base.to_string()),
    }
}

/// 🎯️ The applicable check of an entity with the highest utilisation.
pub(crate) fn governing(report: &CheckReport, entity: &str) -> Option<usize> {
    report.checks.iter().enumerate().filter(|(_, check)| check.status != CheckStatus::NotApplicable && !check.subject.entity_id.is_empty() && semio_s_plugin_norm_structural_scene::projection_kit::base_id(&check.subject.entity_id) == semio_s_plugin_norm_structural_scene::projection_kit::base_id(entity)).max_by(|a, b| a.1.utilization.total_cmp(&b.1.utilization)).map(|(index, _)| index)
}

impl NormSelection {
    /// 🪄️ Resolves `picked` interaction addresses (scoped `kind/id`, `check/id` or raw entity ids, `#part` suffixes allowed) and the stored `check_index`; picks come first, the checked entity is added, an unselected check falls back to the governing check of the first picked entity.
    pub fn resolve(report: &CheckReport, check_index: Option<u32>, picked: &[String]) -> Self {
        let mut entities: Vec<String> = Vec::new();
        let mut owners: Vec<String> = Vec::new();
        let mut check = check_index.map(|index| index as usize).filter(|index| *index < report.checks.len());
        for address in picked {
            match picked_target(address) {
                (true, id) => check = report.checks.iter().position(|candidate| candidate.id == id).or(check),
                (false, id) if !id.is_empty() && !entities.contains(&id) => {
                    let owner = semio_s_plugin_norm_structural_scene::projection_kit::base_id(&id).to_owned();
                    if !owners.contains(&owner) && owner != semio_s_plugin_norm_structural_scene::projection_kit::GLOBAL_BADGE {
                        owners.push(owner);
                    }
                    entities.push(id);
                }
                _ => {}
            }
        }
        if check.is_none() {
            check = entities.first().and_then(|entity| governing(report, entity));
        }
        let subject = check.map(|index| &report.checks[index].subject);
        if let Some(entity) = subject.map(|subject| subject.entity_id.clone()).filter(|id| !id.is_empty()) {
            if !entities.contains(&entity) {
                entities.push(entity);
            }
        }
        let paths = subject.map(|subject| subject.path.clone()).filter(|path| !path.is_empty()).into_iter().collect();
        Self { entities, picked: owners, check, paths, units: None }
    }

    /// 🎯️ The entity every window focuses on: the selected check's entity, else the first picked one.
    pub fn primary(&self, report: &CheckReport) -> Option<String> {
        self.check.and_then(|index| report.checks.get(index)).map(|check| check.subject.entity_id.clone()).filter(|id| !id.is_empty()).or_else(|| self.entities.first().cloned())
    }

    pub fn contains(&self, entity: &str) -> bool {
        self.entities.iter().any(|selected| selected == entity)
    }

    /// 🌲️ Outliner and inspector state: selected entity ids and the selected check's field paths.
    pub fn outliner_state(&self) -> OutlinerState {
        OutlinerState { selected: self.entities.iter().chain(&self.paths).cloned().collect(), ..OutlinerState::default() }
    }

    /// 🧾️ Inspector options of this selection: its unit system (engineering until the unit picker chooses SI), the outliner state and the unit picker.
    pub fn inspector_options(&self) -> InspectorOptions {
        InspectorOptions { preset: self.units.unwrap_or(UnitSystem::Engineering), unit_picker: true, outliner: self.outliner_state(), check: self.check, ..InspectorOptions::new(UnitSystem::Engineering, Tier::Expert) }
    }

    /// 🌲️ [`Self::outliner_state`] for an outliner window whose row activations write the interaction `domain` (outliner, viewport and checks stay in sync).
    pub fn outliner_state_in(&self, domain: &'static str) -> OutlinerState {
        OutlinerState { interaction_domain: Some(domain), ..self.outliner_state() }
    }

    /// ✅️ The check row to mark selected in the checks window.
    pub fn checks_selected(&self) -> Option<u32> {
        self.check.map(|index| index as u32)
    }

    /// 🧊️ Viewport highlight addresses for the scene entities whose document entity is selected (`id` or `id:part`).
    pub fn highlights<'a>(&self, entities: impl IntoIterator<Item = (semio_s_plugin_norm_structural_scene::EntityKind, &'a str)>) -> Vec<String> {
        entities.into_iter().filter(|(_, id)| self.entities.iter().any(|selected| *id == selected || id.strip_prefix(selected.as_str()).is_some_and(|part| part.starts_with(':')))).filter_map(|(kind, id)| viewport3d::picking::entity_address(kind, id)).collect()
    }
}

/// 📏️ The unit system chosen in the view lane; `None` until the picker was used.
pub fn unit_system_of(view: &crate::ViewLane) -> Option<UnitSystem> {
    match view.units.as_deref() {
        Some("si") => Some(UnitSystem::Si),
        Some("engineering") => Some(UnitSystem::Engineering),
        _ => None,
    }
}

/// ⏱️ Longest an editor paint waits for slow (T2) units before it shows the partial report and the progress.
pub const PAINT_BUDGET: std::time::Duration = std::time::Duration::from_millis(16);

/// ⏱️ Same for the frames of an open drag: the geometry follows every frame, slow units finish on the worker.
pub const DRAG_BUDGET: std::time::Duration = std::time::Duration::from_millis(8);

/// ⏳️ The checks window progress of a run: `None` once settled, otherwise the finished and total unit counts (never `completed > total`).
pub fn pending_of(settled: bool, progress: &semio_s_plugin_norm_compliance::incremental_evaluation::EvalProgress) -> Option<ChecksPending> {
    (!settled).then(|| ChecksPending { completed: progress.done, total: progress.total.max(progress.done) })
}

struct OverlayMemo {
    family: &'static str,
    base: Box<dyn std::any::Any + Send>,
    base_value: semio_framework_value::DslValue,
    applied: Vec<crate::FieldWrite>,
    preview_value: semio_framework_value::DslValue,
    preview: Box<dyn std::any::Any + Send>,
}

/// 🫧️ The committed `document` with the provisional writes of `transient` laid over it, `None` without writes or when they do not apply. The value tree of the committed document is built once per drag (until the document changes) and the overlay is patched in place with only the writes that changed since the previous tick; an unchanged write set returns the previous typed preview. See [`crate::overlay_document`].
fn overlay_preview<F: NormFamily>(document: &F::Document, transient: &crate::NormTransient) -> Option<F::Document> {
    static MEMO: std::sync::Mutex<Option<OverlayMemo>> = std::sync::Mutex::new(None);
    let writes: Vec<crate::FieldWrite> = transient.overlay().cloned().collect();
    if writes.is_empty() {
        return None;
    }
    let family = std::any::type_name::<F>();
    let mut guard = MEMO.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let current = guard.take().filter(|memo| memo.family == family && memo.base.downcast_ref::<F::Document>() == Some(document));
    let mut memo = match current {
        Some(memo) => memo,
        None => {
            let base_value = semio_framework_value::ToValue::to_value(document);
            OverlayMemo { family, base: Box::new(document.clone()), preview_value: base_value.clone(), base_value, applied: Vec::new(), preview: Box::new(()) }
        }
    };
    if memo.applied == writes {
        if let Some(preview) = memo.preview.downcast_ref::<F::Document>().cloned() {
            *guard = Some(memo);
            return Some(preview);
        }
    }
    let same_paths = !memo.applied.is_empty() && memo.applied.len() == writes.len() && memo.applied.iter().zip(&writes).all(|(a, b)| a.path == b.path);
    let patch: Vec<crate::FieldWrite> = if same_paths {
        memo.applied.iter().zip(&writes).filter(|(a, b)| a.value != b.value).map(|(_, b)| b.clone()).collect()
    } else {
        memo.preview_value = memo.base_value.clone();
        writes.clone()
    };
    crate::direct_manipulation::overlay_writes(&mut memo.preview_value, &patch).ok()?;
    let typed = <F::Document as semio_framework_value::FromValue>::from_value(memo.preview_value.clone()).ok()?;
    memo.applied = writes;
    memo.preview = Box::new(typed.clone());
    *guard = Some(memo);
    Some(typed)
}

/// 🛰️ Everything one editor paint reads, derived once: the host with its live report, the resolved selection, the focus entity and the worst entity; editor and viewer windows read the same values. `settled` is false while slow units still run on the session worker; `pending` then feeds the checks window progress.
pub struct LiveView<F: NormFamily> {
    pub host: NormHost<F>,
    pub selection: NormSelection,
    pub focus: Option<String>,
    pub worst: Option<String>,
    pub progress: semio_s_plugin_norm_compliance::incremental_evaluation::EvalProgress,
    pub settled: bool,
}

impl<F: NormFamily> LiveView<F> {
    /// 🛰️ Settles the live report of `document` (no evaluate command) and resolves the selection from the stored check index and the picked interaction addresses.
    pub fn new(document: F::Document, check_index: Option<u32>, picked: &[String]) -> Self {
        Self::from_host(NormHost::<F>::live(document), true, check_index, picked)
    }

    /// 🖼️ Like [`Self::new`] but never waits longer than `budget` for slow units: the partial report is shown with `settled == false` and the progress of the worker, and the next paint picks up what finished meanwhile.
    pub fn within(document: F::Document, budget: std::time::Duration, check_index: Option<u32>, picked: &[String]) -> Self {
        let (host, settled) = NormHost::<F>::live_within(document, budget);
        Self::from_host(host, settled, check_index, picked)
    }

    /// 🫧️ Like [`Self::within`] over the committed `document` with the provisional writes of `transient` laid over it (drag first, then ghost): every window paints the preview document and the incremental session re-evaluates only the affected units; no overlay is the committed document. The check the view lane holds (a click on a check row or a stepping shortcut) wins over the stored `check_index`, so the selected check, its entity in the viewport and the camera framing follow the shortcut. While a drag is open the geometry follows every frame but the report is only re-evaluated when the T1 debounce (60 ms after the last tick, or a commit) is due; in between the last preview report is shown.
    pub fn with_lanes(document: F::Document, check_index: Option<u32>, picked: &[String], transient: &crate::NormTransient) -> Self {
        let mut view = Self::over_lanes(document, transient.view.check.or(check_index), picked, transient);
        view.selection.units = unit_system_of(&transient.view);
        view
    }

    fn over_lanes(document: F::Document, check_index: Option<u32>, picked: &[String], transient: &crate::NormTransient) -> Self {
        let shown = overlay_preview::<F>(&document, transient).unwrap_or(document);
        if transient.drag.is_none() {
            return Self::within(shown, PAINT_BUDGET, check_index, picked);
        }
        let family = std::any::type_name::<F>();
        let report = crate::with_clock(|clock| (!clock.report_due(crate::now_ms())).then(|| clock.cached_report(family)).flatten());
        let (host, settled) = match report {
            Some(report) => (NormHost::<F> { document: shown, report }, true),
            None => {
                let (host, settled) = NormHost::<F>::live_within(shown, DRAG_BUDGET);
                if settled {
                    crate::with_clock(|clock| clock.store_report(family, host.shared_report()));
                }
                (host, settled)
            }
        };
        Self::from_host(host, settled, check_index, picked)
    }

    fn from_host(host: NormHost<F>, settled: bool, check_index: Option<u32>, picked: &[String]) -> Self {
        let selection = NormSelection::resolve(host.report(), check_index, picked);
        let focus = selection.primary(host.report());
        let worst = host.report().checks.iter().filter(|check| check.status != CheckStatus::NotApplicable && !check.subject.entity_id.is_empty()).max_by(|a, b| a.utilization.total_cmp(&b.utilization)).map(|check| check.subject.entity_id.clone());
        Self { host, selection, focus, worst, progress: F::live_progress(), settled }
    }

    /// 📈️ The entity diagrams show: the focus, else the worst entity.
    pub fn diagram_focus(&self) -> Option<&str> {
        self.focus.as_deref().or(self.worst.as_deref())
    }

    /// ⏳️ Evaluation progress for the checks window while slow units run; `None` once settled.
    pub fn pending(&self) -> Option<ChecksPending> {
        pending_of(self.settled, &self.progress)
    }

    /// ✅️ The checks view state over the window's own grouping, sort and filter: this selection, the entities picked in the viewport (the list narrows to their checks) and the evaluation progress.
    pub fn checks_view(&self, base: ChecksViewState) -> ChecksViewState {
        ChecksViewState { selected: self.selection.check, pending: self.pending(), owners: self.selection.picked.clone(), ..base }
    }

    /// ✅️ The checks window of this live view: its pending progress, its selected check and the list narrowed to the entities picked in the viewport of the interaction `domain`.
    pub fn render_checks(&self, windows: &TreeWindows<'_>, locale: Locale, controller_id: &'static str, domain: &'static str) -> UiAssemblyResult<BuiltNode> {
        render_checks_with(self.host.report(), &self.checks_view(ChecksViewState::default()), &ChecksWindowSpec { controller_id: Some(controller_id), interaction_domain: Some(domain), ..ChecksWindowSpec::default() }, windows, locale)
    }

    /// 🛑️ Supersedes the evaluation in flight (the checks window cancel action); finished units are kept, the rest re-run on the next paint.
    pub fn cancel() {
        F::cancel_live();
    }
}
