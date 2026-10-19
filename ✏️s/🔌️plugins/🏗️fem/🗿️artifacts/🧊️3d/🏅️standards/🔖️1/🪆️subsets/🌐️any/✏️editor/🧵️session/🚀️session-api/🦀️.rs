//! 🚀️ Session API of the FEM 3D session: the initialization, snapshot read, reconcile and the live visual and solver publication. See [`super`].

use super::*;

/// 🧮️ Source and mode indices belong to the admitted generation original document.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Fem3dResultRequest {
    pub source: Fem3dResultSource,
    pub mode: Option<usize>,
    pub buckling: Option<usize>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Fem3dResultSource {
    #[default]
    FirstCase,
    Case(usize),
    Combination(usize),
}

pub fn result_request(doc: &Fem3dSnapshot, source: Option<&str>, mode: Option<usize>) -> Result<Fem3dResultRequest, &'static str> {
    if mode.is_some_and(|index| index >= doc.analysis.modal_count) { return Err("fem3d.results-mode-missing"); }
    let source = match source {
        None => Fem3dResultSource::FirstCase,
        Some(identity) => {
            let source = semio_s_fem_analyses_model::borrow_result_source_identity(identity).map_err(|_| "fem3d.results-source-identity")?;
            match source.kind {
                "loadCase" => Fem3dResultSource::Case(doc.load_cases.iter().position(|case| case.id == source.id).ok_or("fem3d.results-source-missing")?),
                "combination" => Fem3dResultSource::Combination(doc.combinations.iter().position(|combination| combination.id == source.id).ok_or("fem3d.results-source-missing")?),
                _ => return Err("fem3d.results-source-kind"),
            }
        }
    };
    Ok(Fem3dResultRequest { source, mode, buckling: None })
}


//#region 💼️MountedSession
pub fn initialize() {
    MOUNTED.with(|registry| {
        let _ = registry.borrow().free_len;
    });
    semio_framework_plugin::reactor::jobs::register_bounded_job_kind(FEM3D_MOUNTED_VISUAL_JOB_KIND, BoundedJobFactory { admit: job_admit, demands: job_demands });
}

#[cfg(test)]
pub(super) fn current_identity(app_instance_id: u32) -> Option<Identity> {
    MOUNTED.with(|registry| {
        let registry = registry.borrow();
        registry.current.iter().flatten().find(|current| current.app_instance_id == app_instance_id).map(|current| current.identity)
    })
}

pub(super) fn identity_live(identity: Identity) -> bool { MOUNTED.with(|registry| registry.borrow().current.iter().flatten().any(|current| current.identity == identity)) }

pub(super) fn source_version(render: AppRenderOperationContext) -> semio_framework_value::RequestSourceVersion { semio_framework_value::RequestSourceVersion { base_revision: render.base_revision.0, generation: render.generation.0, canonical_revision: render.canonical_base_revision } }
fn identity_version(identity: Identity) -> semio_framework_value::RequestSourceVersion { semio_framework_value::RequestSourceVersion { base_revision: identity.base_revision.0, generation: identity.generation.0, canonical_revision: identity.canonical_base_revision } }
pub(super) fn record_terminal(identity: Identity, kind: semio_framework_value::RequestTerminalKind) {
    MOUNTED.with(|registry| { let mut registry = registry.borrow_mut(); if let Some(slot) = registry.current.iter().position(|current| current.is_some_and(|current| current.identity == identity)) { if let Some((owner, _, _)) = registry.requests.at(slot) { registry.requests.record_terminal(owner, identity_version(identity), identity.job, kind); } } });
}

pub(super) type RequestKey = semio_framework_value::ContentRequestKey;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConsumerKey { Model, Window(semio_framework_plugin::WindowConfigAuthorityIdentity) }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct PhysicalRequestSpec { pub(super) family: u8, pub(super) namespace: u8, pub(super) index: usize }
fn physical_parts(snapshot: &Fem3dSnapshot, request: Fem3dResultRequest) -> Option<(PhysicalRequestSpec, &str)> {
    let family = if request.mode.is_some() { 1 } else if request.buckling.is_some() { 2 } else { 0 };
    let index = request.mode.or(request.buckling).unwrap_or(0);
    let (namespace, source) = if family == 1 { (2, "none") } else { match request.source { Fem3dResultSource::FirstCase => snapshot.load_cases.first().map_or((2, "none"), |case| (0, case.id.as_str())), Fem3dResultSource::Case(index) => (0, snapshot.load_cases.get(index)?.id.as_str()), Fem3dResultSource::Combination(index) => (1, snapshot.combinations.get(index)?.id.as_str()) } };
    let spec = PhysicalRequestSpec { family, namespace, index };
    Some((spec, source))
}
#[cfg(test)]
fn request_key(snapshot: &Fem3dSnapshot, request: Fem3dResultRequest) -> Option<RequestKey> { let (spec, source) = physical_parts(snapshot, request)?; RequestKey::from_segments(&[&[spec.family], &[spec.namespace], &(spec.index as u64).to_le_bytes(), source.as_bytes()]) }
#[cfg(test)]
fn resolve_physical_request(snapshot: &Fem3dSnapshot, spec: PhysicalRequestSpec, id: &str) -> Option<Fem3dResultRequest> {
    let source = if spec.namespace == 2 { Fem3dResultSource::FirstCase } else if spec.namespace == 0 { Fem3dResultSource::Case(snapshot.load_cases.iter().position(|case| case.id == id)?) } else { Fem3dResultSource::Combination(snapshot.combinations.iter().position(|case| case.id == id)?) };
    Some(Fem3dResultRequest { source, mode: (spec.family == 1).then_some(spec.index), buckling: (spec.family == 2).then_some(spec.index) })
}
#[cfg(test)]
fn resolve_request(registry: &Registry, snapshot: &Fem3dSnapshot, owner: semio_framework_value::FixedKeySlot) -> Option<Fem3dResultRequest> { resolve_physical_request(snapshot, registry.request_specs[owner.index()]?, registry.requests.request_name(owner)?) }
fn prepare_result_demand(registry: &mut Registry, render: AppRenderOperationContext, consumer: ConsumerKey, window: &str, spec: PhysicalRequestSpec, source: &str, version: semio_framework_value::RequestCandidateVersion, grant: RetainedCloneGrant) -> RetainedCloneProgress {
    if registry.app_slots.lookup(render.app_instance_id).is_none() { let bytes = size_of::<u32>() + size_of::<semio_framework_value::FixedKeySlot>(); if grant.maximum_items == 0 || grant.maximum_copy_bytes < bytes || grant.maximum_depth == 0 || registry.app_slots.claim(render.app_instance_id).is_none() { return Default::default(); } return RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() }; }
    let consumer_index = registry.requests.consumer_index(render.app_instance_id, consumer);
    if consumer == ConsumerKey::Model && consumer_index.is_none() { return registry.requests.claim_consumer_controlled(render.app_instance_id, consumer, grant).map_or(Default::default(), |step| step.progress); }
    if consumer != ConsumerKey::Model && consumer_index.is_none_or(|index| registry.requests.consumer_name(index).is_none()) { return registry.requests.prepare_consumer(render.app_instance_id, consumer, window, grant).map_or(Default::default(), |step| step.progress); }
    let mode = (spec.index as u64).to_le_bytes(); let family = [spec.family]; let namespace = [spec.namespace]; let segments = [family.as_slice(), namespace.as_slice(), mode.as_slice(), source.as_bytes()];
    let Some(key) = registry.requests.selector(render.app_instance_id, consumer, version) else { return registry.requests.prepare_selector(render.app_instance_id, consumer, version, &segments, grant).map_or(Default::default(), |step| step.progress); };
    let candidate = registry.requests.binding(render.app_instance_id, consumer);
    if candidate.is_none_or(|owner| registry.requests.key(owner).is_none_or(|(_, current)| current != key)) { let Some(step) = registry.requests.stage_controlled(render.app_instance_id, consumer, key, grant) else { return Default::default(); }; return step.progress; }
    let owner = candidate.unwrap();
    if registry.request_specs[owner.index()].is_none() { let bytes = size_of::<PhysicalRequestSpec>(); if grant.maximum_items == 0 || grant.maximum_copy_bytes < bytes || grant.maximum_depth == 0 { return Default::default(); } registry.request_specs[owner.index()] = Some(spec); return RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() }; }
    if registry.request_specs[owner.index()] != Some(spec) { return Default::default(); }
    let consumer_index = consumer_index.unwrap();
    if registry.verified_sources[consumer_index] != Some((version, key)) {
        let bytes = size_of::<Option<(semio_framework_value::RequestCandidateVersion, RequestKey)>>(); if grant.maximum_items == 0 || grant.maximum_copy_bytes < bytes || grant.maximum_depth == 0 { return Default::default(); }
        if spec.namespace == 2 { registry.verified_sources[consumer_index] = Some((version, key)); return RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() }; }
        let child = RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - bytes, ..grant };
        return match registry.requests.prepare_request(render.app_instance_id, key, source, child) { Ok(mut step) => { if step.ready && step.fault.is_none() { registry.verified_sources[consumer_index] = Some((version, key)); step.progress.copied_items = step.progress.copied_items.max(1); step.progress.copied_bytes += bytes; } step.progress }, Err(_) => Default::default() };
    }
    if registry.requests.live_binding(render.app_instance_id, consumer) != Some(owner) { return registry.requests.admit_controlled(render.app_instance_id, consumer, key, grant).map_or(Default::default(), |step| step.progress); }
    Default::default()
}

/// 🧭️ Admits physical demands from original committed window partitions under one caller turn.
pub fn prepare_snapshot_read(render: AppRenderOperationContext, snapshot: &Fem3dSnapshot, windows: &semio_framework_plugin::WindowConfigOwnerRegistry, view: Option<&semio_framework_plugin::ViewModel>, grant: RetainedCloneGrant) -> semio_framework_plugin::MountedSnapshotReadPreparation {
    if render.app_instance_id == 0 || grant.maximum_items == 0 || grant.maximum_depth == 0 { return Default::default(); }
    let progress = MOUNTED.with(|registry| {
        let mut registry = registry.borrow_mut();
        if let Some(view) = view { for index in 0..64 { let Some((scope, consumer)) = registry.requests.consumer_at(index) else { continue }; let ConsumerKey::Window(authority) = consumer else { continue }; if scope != render.app_instance_id || registry.requests.consumer_retiring(index) { continue; } if !view.window_instances.iter().any(|window| windows.borrow(&window.window_kind_id, &window.id).is_some_and(|window| window.authority == authority)) { if let Some(step) = registry.requests.remove_consumer_controlled(scope, consumer, grant) { registry.verified_sources[index] = None; return step.progress; } return Default::default(); } } }
        let count = view.map_or(1, |view| view.window_instances.len().saturating_add(1));
        for _ in 0..count { let index = registry.consumer_cursor % count; registry.consumer_cursor = (index + 1) % count;
            let (consumer, window, spec, source, version) = if index == 0 { let Some((spec, source)) = physical_parts(snapshot, Fem3dResultRequest::default()) else { continue }; (ConsumerKey::Model, "model", spec, source, semio_framework_value::RequestCandidateVersion { source: source_version(render), consumer_generation: 0, consumer_revision: [0;32] }) } else {
                let Some(window) = view.and_then(|view| view.window_instances.get(index - 1)) else { continue }; let Some(authority) = windows.borrow(&window.window_kind_id, &window.id) else { continue }; let Some(config) = authority.get::<semio_s_artifact_fem_3d_editor_results_window::editor::fem3d::modes::edit::windows::results::config::Fem3dResultsWindowConfigOwner>() else { continue };
                let family = match config.result_mode { semio_s_fem_app_surface::ResultMode::Static => 0, semio_s_fem_app_surface::ResultMode::Modal => 1, semio_s_fem_app_surface::ResultMode::Buckling => 2 }; let index = if family == 0 { 0 } else { config.result_mode_index as usize };
                let (namespace, source) = if family == 1 { (2, "none") } else if let Some(id) = config.result_source_id.as_deref() { let Ok(source) = semio_s_fem_analyses_model::borrow_result_source_identity(id) else { continue }; (if source.kind == "loadCase" { 0 } else { 1 }, source.id) } else { snapshot.load_cases.first().map_or((2, "none"), |case| (0, case.id.as_str())) };
                (ConsumerKey::Window(authority.authority), window.id.as_str(), PhysicalRequestSpec { family, namespace, index }, source, semio_framework_value::RequestCandidateVersion { source: source_version(render), consumer_generation: authority.generation, consumer_revision: authority.revision })
            };
            let progress = prepare_result_demand(&mut registry, render, consumer, window, spec, source, version, grant); if progress != Default::default() { return progress; }
        }
        Default::default()
    });
    if progress != Default::default() { return semio_framework_plugin::MountedSnapshotReadPreparation { ready: false, progress }; }
    prepare_snapshot_admission(render, snapshot, grant)
}

fn prepare_snapshot_admission(render: AppRenderOperationContext, snapshot: &Fem3dSnapshot, grant: RetainedCloneGrant) -> semio_framework_plugin::MountedSnapshotReadPreparation {
    let mut progress = RetainedCloneProgress::default();
    if render.app_instance_id == 0 || snapshot.solids.len() > MAXIMUM_REGIONS || snapshot.nodes.len() > MAXIMUM_NODES || snapshot.elements.len() > MAXIMUM_ELEMENTS || snapshot.supports.len() > MAXIMUM_SUPPORTS {
        return Default::default();
    }
    let ready = MOUNTED.with(|registry| {
        let mut registry = registry.borrow_mut();
        let mut selected = None;
        for _ in 0..ACTIVE_CAPACITY {
            let index = registry.request_cursor; registry.request_cursor = (index + 1) % ACTIVE_CAPACITY;
            let Some((owner, scope, key)) = registry.requests.at(index) else { continue };
            if scope != render.app_instance_id || registry.requests.references(owner) == 0 { continue; }
            if !registry.requests.ready(owner) { continue; }
            if registry.requests.terminal(owner, source_version(render)).is_some() { continue; }
            let Some(spec) = registry.request_specs[index] else { continue };
            let source = if spec.namespace == 2 { Fem3dResultSource::FirstCase } else {
                let (position, outcome) = registry.requests.source_lookup(owner, source_version(render)).unwrap_or((0, semio_framework_value::RequestLookupOutcome::Searching));
                if outcome == semio_framework_value::RequestLookupOutcome::Missing { continue; }
                if outcome == semio_framework_value::RequestLookupOutcome::Searching {
                    let candidate = if spec.namespace == 0 { snapshot.load_cases.get(position).map(|case| case.id.as_str()) } else { snapshot.combinations.get(position).map(|case| case.id.as_str()) };
                    progress = registry.requests.source_lookup_step(owner, source_version(render), candidate, grant).map_or(Default::default(), |step| step.progress); return false;
                }
                if spec.namespace == 0 { Fem3dResultSource::Case(position) } else { Fem3dResultSource::Combination(position) }
            };
            let request = Fem3dResultRequest { source, mode: (spec.family == 1).then_some(spec.index), buckling: (spec.family == 2).then_some(spec.index) };
            if registry.requested[index] != Some((scope, request)) { let bytes = size_of::<Option<(u32, Fem3dResultRequest)>>(); if grant.maximum_items == 0 || grant.maximum_copy_bytes < bytes || grant.maximum_depth == 0 { return false; } registry.requested[index] = Some((scope, request)); progress = RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() }; return false; }
            let settled = registry.current[index].is_some_and(|current| render.matches_revision(current.app_instance_id, current.identity.base_revision, current.identity.generation, &current.identity.canonical_base_revision));
            if !settled { selected = Some(index); break; }
        }
        let Some(slot) = selected else { return false };
        if registry.current[slot].is_some_and(|current| current.app_instance_id != render.app_instance_id) || registry.retained_results[slot].is_some_and(|retained| retained.app_instance_id != render.app_instance_id) {
            return false;
        }
        let request = registry.requested[slot].filter(|(instance, _)| *instance == render.app_instance_id).map_or(Fem3dResultRequest::default(), |(_, request)| request);
        let source_current = registry.current[slot].and_then(|current| registry.shells[current.shell as usize].try_borrow().ok().map(|state| state.as_ref().and_then(|state| state.numerical.as_ref()).is_some_and(|child| child.request == request))).unwrap_or(false);
        if source_current && registry.current[slot].is_some_and(|current| render.matches_revision(current.app_instance_id, current.identity.base_revision, current.identity.generation, &current.identity.canonical_base_revision)) {
            return false;
        }
        let matches = |pending: PendingSnapshot| {
            pending.render.app_instance_id == render.app_instance_id && pending.render.base_revision == render.base_revision && pending.render.generation == render.generation && pending.render.canonical_base_revision == render.canonical_base_revision
        };
        let bytes = size_of::<PendingSnapshot>() * 2;
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < bytes || grant.maximum_depth == 0 { return false; }
        if !registry.pending[slot].is_some_and(matches) {
            registry.pending[slot] = Some(PendingSnapshot { render, preflight: SnapshotPreflight::new() });
            progress = RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() }; return false;
        }
        let Some(pending) = registry.pending[slot].as_mut() else { return false };
        let mut outcome = Ok(false);
        for _ in 0..PREFLIGHT_UNITS_PER_OPPORTUNITY.min(grant.maximum_items).min(grant.maximum_copy_bytes / bytes) {
            progress.copied_items += 1; progress.copied_bytes += bytes;
            let mut next = *pending;
            outcome = next.preflight.step_one(snapshot);
            *pending = next;
            if outcome != Ok(false) {
                break;
            }
        }
        match outcome {
            Ok(complete) => complete,
            Err(()) => {
                registry.pending[slot] = None;
                false
            }
        }
    });
    semio_framework_plugin::MountedSnapshotReadPreparation { ready, progress }
}

pub fn reconcile(doc: &semio_framework_plugin::ArtifactPendingView<'_, Fem3dSnapshot>) -> Vec<Effect> {
    let render = doc.render_operation();
    MOUNTED.with(|registry| {
        let mut registry = registry.borrow_mut();
        let Some(app_slot) = registry.app_slots.lookup(render.app_instance_id).map(|slot| slot.index()) else { return Vec::new() };
        let Some(slot) = registry.pending.iter().position(|pending| pending.is_some_and(|pending| pending.render.app_instance_id == render.app_instance_id && pending.render.canonical_base_revision == render.canonical_base_revision && pending.preflight.stage == SnapshotPreflightStage::Complete)) else { return Vec::new() };
        let Some((request_owner, _, _)) = registry.requests.at(slot) else { return Vec::new() };
        if !registry.requests.ready(request_owner) || registry.requested[slot].is_none_or(|(scope, _)| scope != render.app_instance_id) { return Vec::new(); }
        if registry.requests.terminal(request_owner, source_version(render)).is_some() { return Vec::new(); }
        let Some(pending) = registry.pending[slot].filter(|pending| {
            pending.render.app_instance_id == render.app_instance_id
                && pending.render.base_revision == render.base_revision
                && pending.render.generation == render.generation
                && pending.render.canonical_base_revision == render.canonical_base_revision
                && pending.preflight.stage == SnapshotPreflightStage::Complete
        }) else {
            return Vec::new();
        };
        let Ok(credit) = pending.preflight.credit() else { return Vec::new() };
        let previous = registry.current[slot];
        if let Some(previous) = previous {
            if registry.retiring[previous.shell as usize] || (registry.retiring_count[app_slot] != 0 && registry.retiring_owner[app_slot] != render.app_instance_id) {
                return Vec::new();
            }
        }
        let Some(shell) = registry.allocate() else { return Vec::new() };
        if !registry.reserve_credit(shell) {
            registry.release(shell);
            return Vec::new();
        }
        let Some(counter) = registry.next_job.checked_add(1).filter(|counter| *counter <= JOB_COUNTER_MAXIMUM) else {
            registry.release_credit(shell);
            registry.release(shell);
            return Vec::new();
        };
        let snapshot = match doc.take_snapshot_read() {
            Ok(snapshot) => snapshot,
            Err(_) => {
                registry.release_credit(shell);
                registry.release(shell);
                return Vec::new();
            }
        };
        registry.next_job = counter;
        registry.pending[slot] = None;
        let job = JOB_TAG | counter;
        let identity = Identity { app_instance_id: render.app_instance_id, base_revision: render.base_revision, generation: render.generation, canonical_base_revision: render.canonical_base_revision, operation: OperationId(job), job };
        if let Some(previous) = previous { if identity_version(previous.identity) != source_version(render) { let cancelled = registry.shells[previous.shell as usize].try_borrow().is_ok_and(|child| child.as_ref().is_some_and(|state| { state.cancel.cancel_now(); true })); if cancelled { registry.requests.record_terminal(request_owner, identity_version(previous.identity), previous.identity.job, semio_framework_value::RequestTerminalKind::Cancelled); } } }
        if !registry.requests.begin_execution(request_owner, source_version(render), job) {
            registry.release_credit(shell); registry.release(shell); return Vec::new();
        }
        let recovery = registry.recoveries[shell as usize].clone();
        if !recovery.reserve(identity) {
            registry.release_credit(shell);
            registry.release(shell);
            return Vec::new();
        }
        let retain_results = previous.is_some_and(|previous| registry.shells[previous.shell as usize].try_borrow().is_ok_and(|owner| owner.as_ref().is_some_and(|state| state.done && state.solver.as_ref().is_some_and(Fem3dSolverView::ready))));
        if let Some(previous) = previous {
            let retained = if retain_results {
                registry.retire_retained_results(render.app_instance_id, slot)
            } else {
                registry.retain_retiring(render.app_instance_id, previous.shell)
            };
            if !retained {
                assert!(recovery.release(identity));
                registry.release_credit(shell);
                registry.release(shell);
                return Vec::new();
            }
            if retain_results { registry.retained_results[slot] = Some(previous); }
        }
        let former = previous.and_then(|previous| registry.shells[previous.shell as usize].try_borrow_mut().ok()?.as_mut()?.current.take());
        if let Some(previous) = previous {
            if let Ok(owner) = registry.shells[previous.shell as usize].try_borrow() {
                if let Some(state) = owner.as_ref() {
                    state.cancel.cancel_now();
                }
            }
        }
        let request = registry.requested[slot].filter(|(instance, _)| *instance == render.app_instance_id).map_or(Fem3dResultRequest::default(), |(_, request)| request);
        let mut state = MountedState::new(identity, snapshot, former, credit, recovery);
        if let Some(child) = state.numerical.as_mut() { child.request = request; }
        *registry.shells[shell as usize].borrow_mut() = Some(state);
        registry.shell_request[shell as usize] = Some(request_owner);
        registry.current[slot] = Some(Current { app_instance_id: render.app_instance_id, shell, identity });
        let mut effects = Vec::with_capacity(2);
        if let Some(previous) = previous {
            effects.push(Effect::CancelJob { job: previous.identity.job });
        }
        effects.push(Effect::SpawnJob { job, kind: FEM3D_MOUNTED_VISUAL_JOB_KIND.to_string(), input: encode_input(shell, identity), placement: JobPlacement::Isolated });
        eprintln!(
            "[TRACE] fem3d live_visual reconcile spawn: instance={} revision={} generation={} shell={shell} job={job:#x} meshItems={} drawInstances={} cancelled={}",
            render.app_instance_id,
            render.base_revision.0,
            render.generation.0,
            credit.item_count,
            credit.draw_count,
            effects.len() - 1
        );
        effects
    })
}

pub fn with_live_visual<R>(render: Option<AppRenderOperationContext>, build: impl FnOnce(Option<&Fem3dPageVisualLease>) -> R) -> R {
    let Some(render) = render else { return build(None) };
    let shell = MOUNTED.with(|registry| {
        let registry = registry.borrow();
        let current = registry.current.iter().flatten().find(|current| render.matches_revision(current.app_instance_id, current.identity.base_revision, current.identity.generation, &current.identity.canonical_base_revision))?;
        if !render.matches_revision(current.app_instance_id, current.identity.base_revision, current.identity.generation, &current.identity.canonical_base_revision) {
            return None;
        }
        Some(registry.shells[current.shell as usize].clone())
    });
    let Some(shell) = shell else { return build(None) };
    let Ok(owner) = shell.try_borrow() else { return build(None) };
    build(owner.as_ref().and_then(|state| state.current.as_ref()))
}

/// 🎯️ Configuration changes request a bounded source generation from the snapshot owner.

/// 📊️ Lends validated fields and their original admitted node identities.
pub struct Fem3dResultBorrow<'a> {
    solver: &'a Fem3dSolverView,
    child: &'a Fem3dNumericalChild,
    snapshot: &'a Fem3dSnapshot,
    current_revision: bool,
}

impl Fem3dResultBorrow<'_> {
    pub fn snapshot(&self) -> &Fem3dSnapshot { self.snapshot }
    pub fn len(&self) -> usize { self.solver.len }
    pub fn node_id(&self, index: usize) -> Option<&str> { self.child.analysis_node_ids.get(index).map(String::as_str) }
    pub fn scalar(&self, index: usize) -> Option<Fem3dSolverScalar> { self.solver.scalar(index) }
    pub fn current_revision(&self) -> bool { self.current_revision }
    pub fn static_values(&self, index: usize) -> Option<[f64; 6]> {
        let equations = self.child.equations.get(index)?;
        Some(std::array::from_fn(|axis| equations[axis].and_then(|equation| self.child.pcg.as_ref()?.visual_scalar(equation)).map_or(0.0, |scalar| scalar.displacement)))
    }
    pub fn mode_values(&self, index: usize) -> Option<([f64; 6], f64)> {
        let equations = self.child.equations.get(index)?;
        let mode = self.child.request.mode?;
        let mut eigenvalue = 0.0;
        let values = std::array::from_fn(|axis| equations[axis].and_then(|equation| self.child.subspace.as_ref()?.visual_mode_scalar(mode, equation)).map_or(0.0, |(value, eigen)| { eigenvalue = eigen; value }));
        Some((values, eigenvalue))
    }
    pub fn stress(&self, index: usize) -> Option<f64> {
        let count = *self.child.stress_counts.get(index)?;
        (count != 0).then(|| self.child.stress_sums[index] / count as f64)
    }
    pub fn reaction_values(&self, index: usize) -> Option<[f64; 6]> { self.child.full_reactions.get(index).copied() }
}

fn select_result_shell(current: Option<u16>, retained: Option<u16>) -> Option<u16> { current.or(retained) }

fn same_result_source(old: &Fem3dSnapshot, old_request: Fem3dResultRequest, new: &Fem3dSnapshot, new_request: Fem3dResultRequest) -> bool {
    fn identity(doc: &Fem3dSnapshot, source: Fem3dResultSource) -> Option<(&str, &str)> {
        match source {
            Fem3dResultSource::FirstCase => doc.load_cases.first().map(|case| ("loadCase", case.id.as_str())),
            Fem3dResultSource::Case(index) => doc.load_cases.get(index).map(|case| ("loadCase", case.id.as_str())),
            Fem3dResultSource::Combination(index) => doc.combinations.get(index).map(|combination| ("combination", combination.id.as_str())),
        }
    }
    old_request.mode == new_request.mode && old_request.buckling == new_request.buckling && (old_request.mode.is_some() || identity(old, old_request.source).is_some_and(|source| Some(source) == identity(new, new_request.source)))
}

/// 🧵️ Rendering borrows completed pages; pending or stale generations never trigger a solver.
pub fn with_result_fields<R>(render: Option<AppRenderOperationContext>, _snapshot: &Fem3dSnapshot, authority: Option<semio_framework_plugin::WindowConfigAuthorityIdentity>, _request: Fem3dResultRequest, read: impl FnOnce(Option<Fem3dResultBorrow<'_>>) -> R) -> R {
    let Some(render) = render else { return read(None) };
    let shell = MOUNTED.with(|registry| {
        let registry = registry.borrow();
        let request_owner = registry.requests.live_binding(render.app_instance_id, authority.map_or(ConsumerKey::Model, ConsumerKey::Window))?;
        let slot = request_owner.index();
        let ready = |current: Current, require_revision: bool| {
            if current.app_instance_id != render.app_instance_id || (require_revision && !render.matches_revision(current.app_instance_id, current.identity.base_revision, current.identity.generation, &current.identity.canonical_base_revision)) { return None; }
            let owner = registry.shells[current.shell as usize].try_borrow().ok()?;
            let state = owner.as_ref()?;
            let _child = state.numerical.as_ref()?;
            let _original = state.snapshot.as_ref()?;
            (state.numerical_done && (require_revision || state.done) && state.solver.as_ref().is_some_and(Fem3dSolverView::ready) && registry.shell_request[current.shell as usize] == Some(request_owner)).then_some(current.shell)
        };
        let current = registry.current[slot].and_then(|current| ready(current, true));
        let retained = registry.current[slot].and_then(|current| ready(current, false)).or_else(|| registry.retained_results[slot].and_then(|retained| ready(retained, false)));
        select_result_shell(current, retained).map(|shell| registry.shells[shell as usize].clone())
    });
    let Some(shell) = shell else { return read(None) };
    let Ok(owner) = shell.try_borrow() else { return read(None) };
    let Some(state) = owner.as_ref() else { return read(None) };
    let (Some(solver), Some(child), Some(snapshot)) = (state.solver.as_ref(), state.numerical.as_ref(), state.snapshot.as_ref()) else { return read(None) };
    read(Some(Fem3dResultBorrow { solver, child, snapshot, current_revision: render.matches_revision(state.identity.app_instance_id, state.identity.base_revision, state.identity.generation, &state.identity.canonical_base_revision) }))
}

/// 🏷️ Lends the addressed original numerical stage also published through StepContext's shared trace port.
pub fn result_stage(render: Option<AppRenderOperationContext>, authority: Option<semio_framework_plugin::WindowConfigAuthorityIdentity>) -> Option<&'static str> {
    let render = render?;
    MOUNTED.with(|registry| {
        let registry = registry.borrow();
        let request_owner = registry.requests.live_binding(render.app_instance_id, authority.map_or(ConsumerKey::Model, ConsumerKey::Window))?;
        let slot = request_owner.index();
        if let Some((owner, _, _)) = registry.requests.at(slot) { if let Some(terminal) = registry.requests.terminal(owner, source_version(render)) { match terminal.kind { semio_framework_value::RequestTerminalKind::Faulted => return Some("fem3d.failed"), semio_framework_value::RequestTerminalKind::Cancelled => return Some("fem3d.cancelled"), _ => {} } } }
        let current = registry.current[slot]?;
        if !render.matches_revision(current.app_instance_id, current.identity.base_revision, current.identity.generation, &current.identity.canonical_base_revision) { return None; }
        let owner = registry.shells[current.shell as usize].try_borrow().ok()?;
        let state = owner.as_ref()?;
        Some(if state.cancel.is_cancelled_now() { "fem3d.cancelled" } else if state.done { "fem3d.validated" } else if state.numerical_done { "fem3d.visual" } else { state.numerical.as_ref()?.stage.progress_stage() })
    })
}

#[cfg(test)]
pub fn publish_solver_scalar(render: AppRenderOperationContext, index: usize, scalar: Fem3dSolverScalar) -> Result<(), Fem3dSolverScalar> {
    let shell = MOUNTED.with(|registry| {
        let registry = registry.borrow();
        let current = registry.current.iter().flatten().find(|current| render.matches_revision(current.app_instance_id, current.identity.base_revision, current.identity.generation, &current.identity.canonical_base_revision))?;
        render.matches_revision(current.app_instance_id, current.identity.base_revision, current.identity.generation, &current.identity.canonical_base_revision).then(|| registry.shells[current.shell as usize].clone())
    });
    let Some(shell) = shell else { return Err(scalar) };
    let Ok(mut owner) = shell.try_borrow_mut() else { return Err(scalar) };
    let Some(state) = owner.as_mut() else { return Err(scalar) };
    let freshness = state.identity.freshness(state.preview_sequence);
    state.solver.as_mut().ok_or(scalar)?.publish_scalar(freshness, index, scalar)
}

#[cfg(test)]
pub fn publish_solver_progress(render: AppRenderOperationContext, state_value: Fem3dVisualState, residual_norm: f64, tolerance: f64, completed: usize, total: usize) -> bool {
    let shell = MOUNTED.with(|registry| {
        let registry = registry.borrow();
        let current = registry.current.iter().flatten().find(|current| render.matches_revision(current.app_instance_id, current.identity.base_revision, current.identity.generation, &current.identity.canonical_base_revision))?;
        render.matches_revision(current.app_instance_id, current.identity.base_revision, current.identity.generation, &current.identity.canonical_base_revision).then(|| registry.shells[current.shell as usize].clone())
    });
    let Some(shell) = shell else { return false };
    let Ok(mut owner) = shell.try_borrow_mut() else { return false };
    let Some(state) = owner.as_mut() else { return false };
    let freshness = state.identity.freshness(state.preview_sequence);
    state.solver.as_mut().is_some_and(|solver| solver.publish_progress(freshness, state_value, residual_norm, tolerance, completed, total))
}
//#endregion 💼️MountedSession
