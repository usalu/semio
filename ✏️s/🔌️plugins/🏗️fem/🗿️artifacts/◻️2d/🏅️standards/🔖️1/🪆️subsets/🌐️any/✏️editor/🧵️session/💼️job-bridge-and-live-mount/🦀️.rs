//! 💼️ Job bridge and live mount: the bounded job bridge with the input codec and the live mount entries that initialize, prepare, reconcile, retire and close the session. See [`super`].

use super::*;

//#region 💼️JobBridge
struct MountedBoundedJob {
    pub(super) shell: Rc<RefCell<Option<MountedState>>>,
    pub(super) identity: MountedIdentity,
    input: Option<Vec<u8>>,
    restored: Option<Vec<u8>>,
}

impl BoundedJob for MountedBoundedJob {
    /// ⏱️ Drives exact session units back to back until the host's step ceiling elapses, a unit
    /// publishes output, or the session leaves `Running` — the host owes the job one round trip per
    /// ceiling, never one per unit.
    fn step(&mut self, _original: &mut IoRunControl<'_, '_>, _snapshot: &mut SqliteSnapshotControl<'_>, cx: &mut StepContext<'_>) -> Result<JobStep, semio_framework_job::OriginalTurnFailure> {
        let step = (|| {
            let Ok(mut shell) = self.shell.try_borrow_mut() else { return Ok(JobStep::Running(None)); };
            let Some(state) = shell.as_mut() else { return Ok(JobStep::Failed(b"fem2d.session-owner-missing".to_vec())); };
            if state.identity != self.identity { return Ok(JobStep::Failed(b"fem2d.session-aba".to_vec())); }
            let Some(started) = semio_framework_job::default_now_us() else { return state.step(cx); };
            let deadline = started.saturating_add(SESSION_STEP_CEILING_MS.saturating_mul(1_000)).min(cx.deadline_us());
            let mut step = JobStep::Running(None);
            for _ in 0..SESSION_UNITS_PER_STEP {
                if cx.should_yield() || semio_framework_job::default_now_us().unwrap_or(deadline) >= deadline { break; }
                step = state.step(cx)?;
                if !matches!(step, JobStep::Running(None)) { break; }
            }
            Ok::<_, semio_framework_job::OriginalTurnFailure>(step)
        })()?;
        match &step { JobStep::Done(_) => record_terminal(self.identity, semio_framework_value_registry::fixed_request_owners::RequestTerminalKind::Validated), JobStep::Failed(_) => record_terminal(self.identity, semio_framework_value_registry::fixed_request_owners::RequestTerminalKind::Faulted), _ => {} }
        Ok(step)
    }

    /// 🧹️ The heavy session owner is retired by the registry's own close ladder; this handle releases its two original input buffers and itself.
    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        let grant = cx.retained_grant();
        let demand = self.retirement_demands(0)?;
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(false);
        }
        if let Some(input) = self.input.take() {
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, released_bytes: input.capacity(), ..Default::default() })?;
            return Ok(false);
        }
        if let Some(restored) = self.restored.take() {
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, released_bytes: restored.capacity(), ..Default::default() })?;
            return Ok(false);
        }
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
        Ok(true)
    }

    fn retirement_demands(&self, _copy: usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand { release_bytes: self.input.as_ref().map_or_else(|| self.restored.as_ref().map_or(0, Vec::capacity), Vec::capacity), depth: 1, ..Default::default() })
    }

    fn cancel(&mut self) {
        if let Ok(shell) = self.shell.try_borrow() {
            if let Some(state) = shell.as_ref() {
                state.cancel.cancel_now();
            }
        }
    }

    fn checkpoint(&self) -> Option<Vec<u8>> {
        let shell = self.shell.try_borrow().ok()?;
        let state = shell.as_ref()?;
        let mut bytes = Vec::with_capacity(24);
        bytes.extend_from_slice(&state.identity.operation.0.to_le_bytes());
        bytes.extend_from_slice(&state.identity.base_revision.0.to_le_bytes());
        bytes.extend_from_slice(&state.identity.generation.0.to_le_bytes());
        Some(bytes)
    }

    fn terminal_drop_is_shallow(&self) -> bool {
        self.input.is_none() && self.restored.is_none()
    }
}

pub(super) fn encode_input(shell: u16, identity: MountedIdentity) -> Vec<u8> {
    let mut input = Vec::with_capacity(INPUT_BYTES);
    input.push(1);
    input.extend_from_slice(&shell.to_le_bytes());
    input.extend_from_slice(&identity.app_instance_id.to_le_bytes());
    input.extend_from_slice(&identity.base_revision.0.to_le_bytes());
    input.extend_from_slice(&identity.generation.0.to_le_bytes());
    input.extend_from_slice(&identity.canonical_base_revision);
    input.extend_from_slice(&identity.operation.0.to_le_bytes());
    input
}

pub(super) fn decode_input(job: u64, input: &[u8]) -> Option<(u16, MountedIdentity)> {
    if input.len() != INPUT_BYTES || input[0] != 1 {
        return None;
    }
    let shell = u16::from_le_bytes(input[1..3].try_into().ok()?);
    let app_instance_id = u32::from_le_bytes(input[3..7].try_into().ok()?);
    let base_revision = semio_framework_job::RevisionId(u64::from_le_bytes(input[7..15].try_into().ok()?));
    let generation = semio_framework_job::Generation(u64::from_le_bytes(input[15..23].try_into().ok()?));
    let canonical_base_revision = input[23..55].try_into().ok()?;
    let operation = OperationId(u64::from_le_bytes(input[55..63].try_into().ok()?));
    if operation.0 != job || job & !FEM2D_JOB_COUNTER_MAXIMUM != FEM2D_JOB_TAG {
        return None;
    }
    Some((shell, MountedIdentity { app_instance_id, base_revision, generation, canonical_base_revision, operation, job }))
}

/// 📐️ The original request transfers its two handles into the boxed job under the framework's own original-job admission quote.
fn mounted_job_demands(_job: u64, input: &Option<Vec<u8>>, restored: &Option<Vec<u8>>, _cx: &StepContext<'_>) -> Result<RetainedCloneGrant, ValueError> {
    semio_framework_plugin::reactor::jobs::original_job_admission_demands::<MountedBoundedJob>(input, restored)
}

fn mounted_job_admit(job: u64, input: &mut Option<Vec<u8>>, restored: &mut Option<Vec<u8>>, cx: &mut StepContext<'_>) -> Result<Option<Box<dyn BoundedJob>>, ValueError> {
    let bytes = input.as_deref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "fem2d.session-input"))?;
    let (shell, identity) = decode_input(job, bytes).ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "fem2d.session-input"))?;
    let shell_owner = MOUNTED.with(|registry| {
        let registry = registry.borrow();
        let shell_owner = registry.shells.get(shell as usize).ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "fem2d.session-shell"))?.clone();
        let matches = shell_owner.try_borrow().is_ok_and(|owner| owner.as_ref().is_some_and(|state| state.identity == identity));
        if !matches {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "fem2d.session-stale-factory"));
        }
        Ok(shell_owner)
    })?;
    semio_framework_plugin::reactor::jobs::admit_original_job(input, restored, cx, move |input, restored| MountedBoundedJob { shell: shell_owner, identity, input: Some(input), restored })
}

/// 🏭️ Preallocates the fixed arena at plugin installation and registers the explicit job factory.
pub fn initialize() {
    MOUNTED.with(|registry| {
        let _ = registry.borrow().free_len;
    });
    semio_framework_plugin::reactor::jobs::register_bounded_job_kind(FEM2D_MOUNTED_JOB_KIND, BoundedJobFactory { admit: mounted_job_admit, demands: mounted_job_demands });
}
//#endregion 💼️JobBridge

//#region 🎛️LiveMount
pub(super) fn source_version(render:AppRenderOperationContext)->semio_framework_value_registry::fixed_request_owners::RequestSourceVersion{semio_framework_value_registry::fixed_request_owners::RequestSourceVersion{base_revision:render.base_revision.0,generation:render.generation.0,canonical_revision:render.canonical_base_revision}}
fn identity_version(identity:MountedIdentity)->semio_framework_value_registry::fixed_request_owners::RequestSourceVersion{semio_framework_value_registry::fixed_request_owners::RequestSourceVersion{base_revision:identity.base_revision.0,generation:identity.generation.0,canonical_revision:identity.canonical_base_revision}}
fn record_terminal(identity:MountedIdentity,kind:semio_framework_value_registry::fixed_request_owners::RequestTerminalKind){MOUNTED.with(|registry|{let mut registry=registry.borrow_mut();if let Some(index)=registry.current.iter().position(|current|current.is_some_and(|current|current.identity==identity)){if let Some((owner,_,_))=registry.requests.at(index){registry.requests.record_terminal(owner,identity_version(identity),identity.job,kind);}}});}
pub(super) fn current_identity(identity: MountedIdentity) -> Option<MountedIdentity> {
    MOUNTED.with(|registry| {
        let registry = registry.borrow();
        registry.current.iter().enumerate().find_map(|(index, current)| { let current = current.filter(|current| current.identity == identity)?; let (owner, _, _) = registry.requests.at(index)?; (registry.requests.references(owner) != 0).then_some(current.identity) })
    })
}

/// 🔍️ Advances exactly one schema owner/scalar census before the shared application may
/// request an opaque snapshot lease. The fixed shell and exact process credit are reserved only
/// after the census reaches its complete witness.
fn prepare_snapshot_admission(render: AppRenderOperationContext, snapshot: &Fem2dSnapshot,grant:RetainedCloneGrant) -> semio_framework_plugin::MountedSnapshotReadPreparation {
    if render.app_instance_id == 0 {
        return Default::default();
    }
    let mut progress=RetainedCloneProgress::default();
    let ready = MOUNTED.with(|registry| {
        let mut registry = registry.borrow_mut();
        let Some(app_slot) = registry.app_slots.lookup(render.app_instance_id).map(|slot| slot.index()) else { return false };
        let candidate = (0..SESSION_ACTIVE_CAPACITY).find_map(|offset| {
            let index = (registry.request_cursor + offset) % SESSION_ACTIVE_CAPACITY;
            let (owner, scope, _) = registry.requests.at(index)?;
            if scope != render.app_instance_id || !registry.requests.ready(owner) || registry.requests.terminal(owner,source_version(render)).is_some() { return None; }
            if registry.current[index].is_some_and(|current| render.matches_revision(current.app_instance_id, current.identity.base_revision, current.identity.generation, &current.identity.canonical_base_revision)) { return None; }
            Some(owner)
        });
        let Some(owner) = candidate else { return false };
        let current_slot = owner.index();
        if registry.source_kinds[current_slot]!=2 {
            let version=source_version(render);let lookup=registry.requests.source_lookup(owner,version);
            if let Some((position,semio_framework_value_registry::fixed_request_owners::RequestLookupOutcome::Matched))=lookup{registry.sources[current_slot]=Some(if registry.source_kinds[current_slot]==0{ResolvedResultSource::LoadCase(position)}else{ResolvedResultSource::Combination(position)});}
            else if lookup.is_some_and(|(_,outcome)|outcome==semio_framework_value_registry::fixed_request_owners::RequestLookupOutcome::Missing){return false;}
            else{if grant.maximum_copy_bytes<8{return false;}let position=lookup.map_or(0,|(position,_)|position);let candidate=if registry.source_kinds[current_slot]==0{snapshot.load_cases.get(position).map(|case|case.id.as_str())}else{snapshot.combinations.get(position).map(|combination|combination.id.as_str())};let lookup_grant=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-8,..grant};if let Ok(step)=registry.requests.source_lookup_step(owner,version,candidate,lookup_grant){progress=step.progress;if progress!=Default::default(){registry.request_cursor=(current_slot+1)%SESSION_ACTIVE_CAPACITY;progress.copied_bytes+=8;}}return false;}
        }else{registry.sources[current_slot]=None;}
        registry.request_cursor = (current_slot + 1) % SESSION_ACTIVE_CAPACITY;
        registry.snapshot_owners[app_slot] = Some(owner);
        if registry.source_apps[current_slot].is_some_and(|app| app != render.app_instance_id) { return false; }
        if registry.families[current_slot] != semio_s_fem_app_surface::ResultMode::Static {
            let detail = if registry.families[current_slot] == semio_s_fem_app_surface::ResultMode::Modal { "fem2d.modal-job-owner-missing" } else { "fem2d.buckling-job-owner-missing" };
            if registry.preflight_fault[current_slot].is_none() { registry.preflight_fault[current_slot] = Some(PendingSnapshotFault { app_instance_id: render.app_instance_id, render, detail, emitted: false }); }
            return false;
        }
        if let Some(current) = registry.current[current_slot].filter(|current| render.matches_revision(current.app_instance_id, current.identity.base_revision, current.identity.generation, &current.identity.canonical_base_revision)) {
            let Ok(owner) = registry.shells[current.shell as usize].try_borrow() else { return false };
            if owner.as_ref().is_some_and(|state| state.source == registry.sources[current_slot]) { return false }
        }
        let previous = match registry.current[current_slot] {
            Some(current) if current.app_instance_id != render.app_instance_id => return false,
            Some(current) => Some(current),
            None => None,
        };
        if let Some(pending) = registry.pending[current_slot] {
            if pending.app_instance_id != render.app_instance_id {
                return false;
            }
            if pending.identity.base_revision == render.base_revision && pending.identity.generation == render.generation && pending.identity.canonical_base_revision == render.canonical_base_revision {
                return true;
            }
            registry.pending[current_slot] = None;
            registry.release_credit(pending.shell);
            registry.release(pending.shell);
        }
        if previous.is_some() && !registry.retiring.iter().any(Option::is_none) {
            return false;
        }
        let matches_render = |pending: PendingSnapshotAdmission| {
            pending.app_instance_id == render.app_instance_id && pending.render.base_revision == render.base_revision && pending.render.generation == render.generation && pending.render.canonical_base_revision == render.canonical_base_revision
        };
        if let Some(fault) = registry.preflight_fault[current_slot] {
            if fault.app_instance_id == render.app_instance_id && fault.render.base_revision == render.base_revision && fault.render.generation == render.generation && fault.render.canonical_base_revision == render.canonical_base_revision {
                return false;
            }
            registry.preflight_fault[current_slot] = None;
        }
        if !registry.preflight[current_slot].is_some_and(matches_render) {
            registry.preflight[current_slot] = Some(PendingSnapshotAdmission { app_instance_id: render.app_instance_id, render, cursor: SnapshotAdmissionCursor::new() });
        }
        let completed = {
            let preflight = registry.preflight[current_slot].as_mut().expect("matching FEM preflight retained");
            let mut outcome = Ok(false);
            for _ in 0..SESSION_PREFLIGHT_UNITS_PER_OPPORTUNITY.min(grant.maximum_items).min(grant.maximum_copy_bytes/64) {
                progress.copied_items+=1;progress.copied_bytes+=64;
                outcome = preflight.cursor.step_one(snapshot);
                if outcome != Ok(false) {
                    break;
                }
            }
            match outcome {
                Ok(completed) => completed,
                Err(detail) => {
                    registry.preflight[current_slot] = None;
                    registry.preflight_fault[current_slot] = Some(PendingSnapshotFault { app_instance_id: render.app_instance_id, render, detail: std::str::from_utf8(detail).unwrap_or("fem2d.session-preflight-fault"), emitted: false });
                    return false;
                }
            }
        };
        if !completed {
            return false;
        }
        let preflight = registry.preflight[current_slot].take().expect("completed FEM preflight retained");
        let (process_items, process_bytes) = match preflight.cursor.process_credit() {
            Ok(credit) => credit,
            Err(detail) => {
                registry.preflight_fault[current_slot] = Some(PendingSnapshotFault { app_instance_id: render.app_instance_id, render, detail: std::str::from_utf8(detail).unwrap_or("fem2d.session-process-credit-fault"), emitted: false });
                return false;
            }
        };
        let Some(shell) = registry.allocate() else { return false };
        if !registry.reserve_credit(shell, process_items, process_bytes) {
            registry.release(shell);
            return false;
        }
        let Some(counter) = registry.next_job.checked_add(1).filter(|counter| *counter <= FEM2D_JOB_COUNTER_MAXIMUM) else {
            registry.release_credit(shell);
            registry.release(shell);
            return false;
        };
        registry.next_job = counter;
        let job = FEM2D_JOB_TAG | counter;
        let identity = MountedIdentity { app_instance_id: render.app_instance_id, base_revision: render.base_revision, generation: render.generation, canonical_base_revision: render.canonical_base_revision, operation: OperationId(job), job };
        registry.pending[current_slot] = Some(PendingAdmission { app_instance_id: render.app_instance_id, shell, identity, admitted_items: process_items });
        registry.shell_requests[shell as usize] = Some(owner);
        true
    });
    semio_framework_plugin::MountedSnapshotReadPreparation{ready,progress}
}

/// 🪪️ Physical namespace, complete id, family and mode index own the scheduler identity.
#[cfg(test)]
pub fn result_request_key(source: Option<semio_s_fem_analyses_model::ResultSourceIdentity<'_>>, family: semio_s_fem_app_surface::ResultMode, index: usize) -> Option<RequestKey> {
    let source = if family == semio_s_fem_app_surface::ResultMode::Modal { None } else { source };
    let family = match family { semio_s_fem_app_surface::ResultMode::Static => "static", semio_s_fem_app_surface::ResultMode::Modal => "modal", semio_s_fem_app_surface::ResultMode::Buckling => "buckling" };
    let index = if family == "static" { 0 } else { index };
    if let Some(source)=source { semio_s_fem_analyses_model::validate_result_source_parts(source.kind,source.id).ok()?; }
    RequestKey::from_segments(&[family.as_bytes(), &(index as u64).to_le_bytes(), source.map_or("none", |source|source.kind).as_bytes(), source.map_or("none", |source|source.id).as_bytes()])
}

#[cfg(test)]
fn resolved_source(snapshot:&Fem2dSnapshot,source:Option<semio_s_fem_analyses_model::ResultSourceIdentity<'_>>)->Option<Option<ResolvedResultSource>>{match source{None=>Some(None),Some(source)if source.kind=="loadCase"=>snapshot.load_cases.iter().position(|case|case.id==source.id).map(|index|Some(ResolvedResultSource::LoadCase(index))),Some(source)if source.kind=="combination"=>snapshot.combinations.iter().position(|case|case.id==source.id).map(|index|Some(ResolvedResultSource::Combination(index))),Some(_)=>None}}

fn prepare_result_demand(registry:&mut MountedRegistry,render:AppRenderOperationContext,consumer:ConsumerKey,window:&str,source:Option<semio_s_fem_analyses_model::ResultSourceIdentity<'_>>,family:semio_s_fem_app_surface::ResultMode,index:usize,version:semio_framework_value_registry::fixed_request_owners::RequestCandidateVersion,grant:RetainedCloneGrant)->RetainedCloneProgress{
 let source=if family==semio_s_fem_app_surface::ResultMode::Modal{None}else{source};let index=if family==semio_s_fem_app_surface::ResultMode::Static{0}else{index};
 let Some(app)=registry.app_slots.lookup(render.app_instance_id)else{let bytes=size_of::<u32>()+size_of::<semio_framework_value_registry::fixed_key_slots::FixedKeySlot>();if grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Default::default();}if registry.app_slots.claim(render.app_instance_id).is_none(){return Default::default();}return RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};};
 let consumer_index=registry.requests.consumer_index(render.app_instance_id,consumer);
 if consumer==ConsumerKey::Model&&consumer_index.is_none(){return registry.requests.claim_consumer_controlled(render.app_instance_id,consumer,grant).map_or(Default::default(),|step|step.progress);}
 if consumer!=ConsumerKey::Model&&consumer_index.is_none_or(|index|registry.requests.consumer_name(index).is_none()){return registry.requests.prepare_consumer(render.app_instance_id,consumer,window,grant).map_or(Default::default(),|step|step.progress);}
 let family_name=match family{semio_s_fem_app_surface::ResultMode::Static=>"static",semio_s_fem_app_surface::ResultMode::Modal=>"modal",semio_s_fem_app_surface::ResultMode::Buckling=>"buckling"};let index_bytes=(index as u64).to_le_bytes();let segments=[family_name.as_bytes(),index_bytes.as_slice(),source.map_or("none",|source|source.kind).as_bytes(),source.map_or("none",|source|source.id).as_bytes()];
 let Some(key)=registry.requests.selector(render.app_instance_id,consumer,version)else{return registry.requests.prepare_selector(render.app_instance_id,consumer,version,&segments,grant).map_or(Default::default(),|step|step.progress);};
 let candidate=registry.requests.binding(render.app_instance_id,consumer);if candidate.is_none_or(|owner|registry.requests.key(owner).is_none_or(|(_,current)|current!=key)){
  let Some(step)=registry.requests.stage_controlled(render.app_instance_id,consumer,key,grant)else{return Default::default();};if step.admission.fresh{let slot=step.admission.owner.index();registry.source_apps[slot]=Some(render.app_instance_id);registry.source_kinds[slot]=source.map_or(2,|source|if source.kind=="loadCase"{0}else{1});registry.families[slot]=family;registry.mode_indices[slot]=index;registry.request_cancelled[slot]=false;}if consumer==ConsumerKey::Model{registry.model_requests[app.index()]=Some(key);}return step.progress;
 }
 let consumer_index=consumer_index.unwrap();if registry.verified_sources[consumer_index]!=Some((version,key)){if source.is_none(){registry.verified_sources[consumer_index]=Some((version,key));}else{match registry.requests.prepare_request(render.app_instance_id,key,source.map_or("none",|source|source.id),grant){Ok(step)=>{if step.ready{registry.verified_sources[consumer_index]=Some((version,key));}return step.progress;},Err(_)=>return Default::default()}}}
 let owner=candidate.unwrap();if registry.requests.live_binding(render.app_instance_id,consumer)!=Some(owner){return registry.requests.admit_controlled(render.app_instance_id,consumer,key,grant).map_or(Default::default(),|step|step.progress);}Default::default()
}

/// 🧭️ Committed window partitions and source revisions admit one fair original preparation frontier.
pub fn prepare_snapshot_read(render:AppRenderOperationContext,snapshot:&Fem2dSnapshot,windows:&semio_framework_plugin::WindowConfigOwnerRegistry,view:Option<&semio_framework_plugin::ViewModel>,grant:RetainedCloneGrant)->semio_framework_plugin::MountedSnapshotReadPreparation{
 if render.app_instance_id==0||grant.maximum_items==0||grant.maximum_depth==0{return Default::default();}
 let departure=retain_result_windows(render,windows,view,grant);if departure!=Default::default(){return semio_framework_plugin::MountedSnapshotReadPreparation{ready:false,progress:departure};}
 let progress=MOUNTED.with(|registry|{let mut registry=registry.borrow_mut();let count=view.map_or(1,|view|view.window_instances.len()+1);for _ in 0..count{let index=registry.consumer_cursor%count;registry.consumer_cursor=(index+1)%count;let (consumer,window,source,family,mode,version)=if index==0{(ConsumerKey::Model,"model",snapshot.load_cases.first().map(|case|semio_s_fem_analyses_model::ResultSourceIdentity{kind:"loadCase",id:&case.id}),semio_s_fem_app_surface::ResultMode::Static,0,semio_framework_value_registry::fixed_request_owners::RequestCandidateVersion{source:source_version(render),consumer_generation:0,consumer_revision:[0;32]})}else{let Some(window)=view.and_then(|view|view.window_instances.get(index-1))else{continue;};let Some(authority)=windows.borrow(&window.window_kind_id,&window.id)else{continue;};let Some(config)=authority.get::<semio_s_artifact_fem_2d_results_state::config::Fem2dResultsWindowConfigOwner>()else{continue;};let source=match config.result_source_id.as_deref(){Some(id)=>match semio_s_fem_analyses_model::borrow_result_source_identity(id){Ok(source)=>Some(source),Err(_)=>continue},None=>snapshot.load_cases.first().map(|case|semio_s_fem_analyses_model::ResultSourceIdentity{kind:"loadCase",id:&case.id})};(ConsumerKey::Window(authority.authority),window.id.as_str(),source,config.result_mode,config.result_mode_index,semio_framework_value_registry::fixed_request_owners::RequestCandidateVersion{source:source_version(render),consumer_generation:authority.generation,consumer_revision:authority.revision})};let progress=prepare_result_demand(&mut registry,render,consumer,window,source,family,mode,version,grant);if progress!=Default::default(){return progress;}}Default::default()});
 if progress!=Default::default(){return semio_framework_plugin::MountedSnapshotReadPreparation{ready:false,progress};}prepare_snapshot_admission(render,snapshot,grant)
}

/// 🪟️ Original committed partition identities prune departed consumers before request retirement.
pub fn retain_result_windows(render:AppRenderOperationContext,windows:&semio_framework_plugin::WindowConfigOwnerRegistry,view:Option<&semio_framework_plugin::ViewModel>,grant:RetainedCloneGrant)->RetainedCloneProgress{MOUNTED.with(|registry|{let mut registry=registry.borrow_mut();for index in 0..SESSION_CONSUMER_CAPACITY{let Some((scope,consumer))=registry.requests.consumer_at(index)else{continue;};let ConsumerKey::Window(identity)=consumer else{continue;};if scope!=render.app_instance_id{continue;}let live=view.is_some_and(|view|view.window_instances.iter().any(|window|windows.borrow(&window.window_kind_id,&window.id).is_some_and(|authority|authority.authority==identity)));if !live{if let Some(step)=registry.requests.remove_consumer_controlled(scope,consumer,grant){registry.verified_sources[index]=None;return step.progress;}}}Default::default()})}

/// 🔁️ Reconciles one live store revision and returns the exact retained job effects.
pub fn reconcile(doc: &semio_framework_plugin::ArtifactPendingView<'_, Fem2dSnapshot>) -> Vec<Effect> {
    let render = doc.render_operation();
    if render.app_instance_id == 0 {
        return Vec::new();
    }
    MOUNTED.with(|registry| {
        let mut registry = registry.borrow_mut();
        let mut cancellations=Vec::new();
        for index in 0..SESSION_ACTIVE_CAPACITY{let Some((owner,scope,_))=registry.requests.at(index)else{continue;};if scope!=render.app_instance_id||registry.requests.references(owner)!=0||registry.request_cancelled[index]{continue;}if let Some(current)=registry.current[index]{registry.requests.record_terminal(owner,identity_version(current.identity),current.identity.job,semio_framework_value_registry::fixed_request_owners::RequestTerminalKind::Cancelled);registry.request_cancelled[index]=true;cancellations.push(Effect::CancelJob{job:current.identity.job});}}
        if !cancellations.is_empty(){return cancellations;}
        let Some(app_slot) = registry.app_slots.lookup(render.app_instance_id).map(|slot| slot.index()) else { return Vec::new() };
        let Some(owner) = registry.snapshot_owners[app_slot].filter(|owner| registry.requests.key(*owner).is_some_and(|(scope, _)| scope == render.app_instance_id) && registry.requests.ready(*owner)) else { return Vec::new() };
        let current_slot = owner.index();
        if let Some(fault) = registry.preflight_fault[current_slot].as_mut().filter(|fault| {
            fault.app_instance_id == render.app_instance_id && fault.render.base_revision == render.base_revision && fault.render.generation == render.generation && fault.render.canonical_base_revision == render.canonical_base_revision
        }) {
            if !fault.emitted {
                fault.emitted = true;
                return vec![Effect::Notify { message: fault.detail.to_string() }];
            }
            return Vec::new();
        }
        let previous = if let Some(current) = registry.current[current_slot] {
            if current.app_instance_id != render.app_instance_id {
                return Vec::new();
            }
            if render.matches_revision(current.app_instance_id, current.identity.base_revision, current.identity.generation, &current.identity.canonical_base_revision) {
                let Ok(owner) = registry.shells[current.shell as usize].try_borrow() else { return Vec::new() };
                if owner.as_ref().is_some_and(|state| state.source == registry.sources[current_slot]) { return Vec::new() }
            }
            if !registry.retiring.iter().any(Option::is_none) {
                return Vec::new();
            }
            Some(current)
        } else {
            None
        };
        let Some(pending) = registry.pending[current_slot].filter(|pending| {
            pending.app_instance_id == render.app_instance_id && pending.identity.base_revision == render.base_revision && pending.identity.generation == render.generation && pending.identity.canonical_base_revision == render.canonical_base_revision
        }) else {
            return Vec::new();
        };
        let shell = pending.shell;
        let snapshot = match doc.take_snapshot_read() {
            Ok(snapshot) => snapshot,
            Err(_) => return Vec::new(),
        };
        registry.pending[current_slot] = None;
        let previous_valid = previous.and_then(|previous| {
            let owner = registry.shells[previous.shell as usize].try_borrow().ok()?;
            if owner.as_ref().is_some_and(|state| state.visual.validated_final) { Some(previous.shell) } else { previous.previous_valid }
        });
        if let Some(previous) = previous {
            assert!(registry.retain_retirement(previous.shell, previous.app_instance_id), "retirement slot was preflighted under the registry owner");
            if let Ok(shell) = registry.shells[previous.shell as usize].try_borrow() {
                if let Some(state) = shell.as_ref() {
                    state.cancel.cancel_now();
                }
            }
            registry.requests.record_terminal(owner,identity_version(previous.identity),previous.identity.job,semio_framework_value_registry::fixed_request_owners::RequestTerminalKind::Cancelled);
            registry.current[current_slot] = None;
        }
        let identity = pending.identity;
        let job = identity.job;
        assert!(registry.requests.begin_execution(owner,identity_version(identity),job), "only the original verified request admits one current job");
        let mut state = MountedState::new(identity, snapshot, pending.admitted_items);
        state.source = registry.sources[current_slot];
        *registry.shells[shell as usize].borrow_mut() = Some(state);
        registry.current[current_slot] = Some(CurrentSession { app_instance_id: render.app_instance_id, shell, identity, previous_valid });
        let mut effects = Vec::with_capacity(2);
        if let Some(previous) = previous {
            effects.push(Effect::CancelJob { job: previous.identity.job });
        }
        effects.push(Effect::SpawnJob { job, kind: FEM2D_MOUNTED_JOB_KIND.to_string(), input: encode_input(shell, identity), placement: JobPlacement::Isolated });
        effects
    })
}

/// 📊️ Borrows physical fields from the addressed source and generation without running a solver in render.
pub struct MountedFieldsView<'a> { pub progress: &'a Fem2dLiveVisual, pub fields: &'a [semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::NodeLiveField], pub retained: bool }

pub fn with_live_fields<R>(render: AppRenderOperationContext, authority: Option<semio_framework_plugin::WindowConfigAuthorityIdentity>, read: impl FnOnce(Option<MountedFieldsView<'_>>) -> R) -> R {
    let shells = MOUNTED.with(|registry| {
        let registry = registry.borrow();
        let owner = registry.requests.live_binding(render.app_instance_id, ConsumerKey::Window(authority?))?;
        let current = registry.current[owner.index()]?;
        if !render.matches_revision(current.app_instance_id, current.identity.base_revision, current.identity.generation, &current.identity.canonical_base_revision) { return None; }
        Some((registry.shells[current.shell as usize].clone(), current.previous_valid.map(|shell| registry.shells[shell as usize].clone())))
    });
    let Some((shell, previous)) = shells else { return read(None); };
    let Ok(owner) = shell.try_borrow() else { return read(None); };
    let Some(current) = owner.as_ref() else { return read(None); };
    let previous = previous.as_ref().and_then(|owner| owner.try_borrow().ok());
    let previous = previous.as_ref().and_then(|owner| owner.as_ref()).filter(|state| state.visual.validated_final && !current.visual.validated_final);
    read(Some(MountedFieldsView { progress: &current.visual, fields: previous.map_or(current.visual.fields.as_slice(), |state| state.visual.fields.as_slice()), retained: previous.is_some() }))
}

/// 👁️ Borrows the immutable generation-exact mounted visual lease only while the renderer adopts it.
pub fn with_live_visual<R>(render: Option<AppRenderOperationContext>, build: impl FnOnce(Option<&Fem2dMountedVisualLease>) -> R) -> R {
    let Some(render) = render else { return build(None) };
    let shell = MOUNTED.with(|registry| {
        let registry = registry.borrow();
        let app = registry.app_slots.lookup(render.app_instance_id)?.index();
        let key = registry.model_requests[app]?;
        let current = registry.current[registry.requests.lookup(render.app_instance_id, key)?.index()]?;
        if !render.matches_revision(current.app_instance_id, current.identity.base_revision, current.identity.generation, &current.identity.canonical_base_revision) {
            return None;
        }
        Some(registry.shells[current.shell as usize].clone())
    });
    let Some(shell) = shell else { return build(None) };
    let Ok(owner) = shell.try_borrow() else { return build(None) };
    build(owner.as_ref().and_then(|state| state.visual_current.as_ref()).filter(|lease| lease.matches(render.app_instance_id, render.base_revision.0, render.generation.0)))
}

fn retire_one(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, PluginLifecycleFailure> {
    MOUNTED.with(|registry| {
        let mut registry = registry.borrow_mut();
        let Some(index) = registry.retiring.iter().position(|entry| entry.is_some_and(|entry| entry.app_instance_id == app_instance_id && !registry.current.iter().flatten().any(|current| current.previous_valid == Some(entry.shell))))
        else {
            return Ok(PluginLifecycleStep::Complete(RetainedCloneProgress::default()));
        };
        let shell = registry.retiring[index].expect("matched retirement shell").shell;
        let step = {
            let mut owner = match registry.shells[shell as usize].try_borrow_mut() {
                Ok(owner) => owner,
                Err(_) => return Ok(PluginLifecycleStep::Blocked { reason: "mounted FEM job owner is checked out by its worker turn" }),
            };
            let Some(state) = owner.as_mut() else { return Ok(PluginLifecycleStep::Blocked { reason: "mounted FEM retirement lost its original child owner" })};
            if state.identity.app_instance_id != app_instance_id { return Ok(PluginLifecycleStep::Blocked { reason: "mounted FEM retirement child app affinity changed" })}
            state.close_step(cx, grant)?
        };
        if matches!(step, PluginLifecycleStep::Complete(_)) {
            let terminal = registry.shells[shell as usize].try_borrow().is_ok_and(|owner| owner.as_ref().is_some_and(MountedState::terminal_is_empty));
            if !terminal {
                return Ok(PluginLifecycleStep::Blocked { reason: "mounted FEM job reported a false terminal shell" });
            }
            *registry.shells[shell as usize].borrow_mut() = None;
            registry.retiring[index] = None;
            registry.release_credit(shell);
            registry.release(shell);
            return Ok(session_pending(1, 0));
        }
        Ok(step)
    })
}

pub fn maintenance_step(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, PluginLifecycleFailure> {
    semio_framework_plugin::app::original_lifecycle_step(cx, grant, close_demands(app_instance_id)?, |cx, grant| {
        let step = if let Some(progress) = MOUNTED.with(|registry| registry.borrow_mut().requests.retire_consumer_step(app_instance_id, grant)) {
            PluginLifecycleStep::Progress(progress)
        } else {
            let owner = MOUNTED.with(|registry| { let registry = registry.borrow(); (0..SESSION_ACTIVE_CAPACITY).find_map(|index| { let (owner, scope, _) = registry.requests.at(index)?; (scope == app_instance_id && registry.requests.references(owner) == 0).then_some(owner) }) });
            if let Some(owner) = owner { close_request_step(app_instance_id, owner, cx, grant)? } else { retire_one(app_instance_id, cx, grant)? }
        };
        Ok((step, terminal_is_empty(app_instance_id)))
    })
}

/// 📏️ Quotes the next close or maintenance turn: one owner release bounded by the largest session owner.
pub fn close_demands(app_instance_id: u32) -> Result<RetirementDemand, ValueError> {
    MOUNTED.with(|registry| {
        let registry = registry.borrow();
        let mut demand = RetirementDemand { copy_bytes: SESSION_CLOSE_COPY_BYTES, release_bytes: SESSION_CLOSE_RELEASE_BYTES, depth: SESSION_CLOSE_DEPTH, ..Default::default() };
        for shell in &registry.shells {
            if let Ok(owner) = shell.try_borrow() {
                if let Some(state) = owner.as_ref().filter(|state| state.identity.app_instance_id == app_instance_id) { state.child_close_demands(&mut demand)?; }
            }
        }
        Ok(demand)
    })
}

pub(super) fn merge_close_demand(total: &mut RetirementDemand, child: RetirementDemand) {
    total.copy_bytes = total.copy_bytes.max(child.copy_bytes);
    total.capacity_bytes = total.capacity_bytes.max(child.capacity_bytes);
    total.release_bytes = total.release_bytes.max(child.release_bytes);
    total.depth = total.depth.max(child.depth);
}

fn close_request_step(app_instance_id: u32, lease: semio_framework_value_registry::fixed_key_slots::FixedKeySlot, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, PluginLifecycleFailure> {
    if grant.maximum_items == 0 { return Ok(session_pending(0, 0)); }
    MOUNTED.with(|registry| {
        let mut registry = registry.borrow_mut();
        if registry.requests.key(lease).is_none_or(|(scope, _)| scope != app_instance_id) || registry.requests.references(lease) != 0 { return Ok(PluginLifecycleStep::Blocked { reason: "mounted FEM request is still referenced or its original lease changed" }); }
        let slot = lease.index();
        if registry.source_apps[slot] == Some(app_instance_id) {
            let bytes = size_of::<Option<ResolvedResultSource>>();
            if bytes > grant.maximum_release_bytes { return Ok(session_pending(0, 0)); }
            registry.sources[slot] = None;
            registry.source_apps[slot] = None;
            return Ok(session_pending(1, bytes));
        }
        if registry.preflight_fault[slot].is_some_and(|fault| fault.app_instance_id == app_instance_id) {
            registry.preflight_fault[slot] = None;
            return Ok(session_pending(1, size_of::<PendingSnapshotFault>()));
        }
        if registry.preflight[slot].is_some_and(|preflight| preflight.app_instance_id == app_instance_id) {
            registry.preflight[slot] = None;
            return Ok(session_pending(1, size_of::<PendingSnapshotAdmission>()));
        }
        if let Some(pending) = registry.pending[slot].filter(|pending| pending.app_instance_id == app_instance_id) {
            registry.pending[slot] = None;
            registry.release_credit(pending.shell);
            registry.release(pending.shell);
            return Ok(session_pending(1, INPUT_BYTES));
        }
        if let Some(current) = registry.current[slot].filter(|current| current.app_instance_id == app_instance_id) {
            if !registry.retain_retirement(current.shell, current.app_instance_id) {
                return Ok(PluginLifecycleStep::Blocked { reason: "mounted FEM retirement arena is saturated" });
            }
            if let Ok(owner) = registry.shells[current.shell as usize].try_borrow() {
                if let Some(state) = owner.as_ref() {
                    state.cancel.cancel_now();
                }
            }
            registry.current[slot] = None;
            return Ok(session_pending(1, 0));
        }
        drop(registry);
        let step = retire_one(app_instance_id, cx, grant)?;
        if !matches!(step, PluginLifecycleStep::Complete(_)) { return Ok(step)}
        let children_remain = MOUNTED.with(|registry| registry.borrow().shell_requests.iter().any(|owner| *owner == Some(lease)));
        if children_remain { return Ok(PluginLifecycleStep::Blocked { reason: "mounted FEM request still owns its original child shell" }); }
        let copy_bytes = size_of::<RequestKey>() * 2 + size_of::<semio_framework_value_registry::fixed_key_slots::FixedKeySlot>() * 2 + size_of::<Option<u32>>() + size_of::<usize>() * 2;
        if grant.maximum_copy_bytes < copy_bytes || grant.maximum_depth == 0 { return Ok(session_pending(0, 0))}
        if let Some(progress)=MOUNTED.with(|registry|registry.borrow_mut().requests.retire_request_step(lease,grant)){return Ok(PluginLifecycleStep::Progress(progress));}
        MOUNTED.with(|registry| {
            let mut registry = registry.borrow_mut();
            assert!(registry.requests.release(lease), "the original request claim remains live through child retirement");
            for selected in &mut registry.snapshot_owners { if *selected == Some(lease) { *selected = None; } }
        });
        Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: copy_bytes, ..Default::default() }))
    })
}

/// 🪦️ App close removes consumers before independently retiring their request children.
pub fn close_step(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, PluginLifecycleFailure> {
    semio_framework_plugin::app::original_lifecycle_step(cx, grant, close_demands(app_instance_id)?, |cx, grant| {
        let step = close_turn(app_instance_id, cx, grant)?;
        Ok((step, terminal_is_empty(app_instance_id)))
    })
}

fn close_turn(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, PluginLifecycleFailure> {
    if grant.maximum_items == 0 { return Ok(session_pending(0, 0)); }
    if let Some(step)=MOUNTED.with(|registry|registry.borrow_mut().requests.remove_one_scoped_consumer_controlled(app_instance_id,grant)){return Ok(PluginLifecycleStep::Progress(step.progress));}
    if let Some(progress)=MOUNTED.with(|registry|registry.borrow_mut().requests.retire_consumer_step(app_instance_id,grant)){return Ok(PluginLifecycleStep::Progress(progress));}
    let owner = MOUNTED.with(|registry| { let registry = registry.borrow(); (0..SESSION_ACTIVE_CAPACITY).find_map(|index| { let (owner, scope, _) = registry.requests.at(index)?; (scope == app_instance_id).then_some(owner) }) });
    if let Some(owner) = owner { return Ok(close_request_step(app_instance_id, owner, cx, grant)?); }
    let step = retire_one(app_instance_id, cx, grant)?;
    if !matches!(step, PluginLifecycleStep::Complete(_)) { return Ok(step); }
    let copy_bytes = size_of::<RequestKey>() + size_of::<semio_framework_value_registry::fixed_key_slots::FixedKeySlot>() * 2 + size_of::<Option<u32>>() + size_of::<usize>() * 2;
    if grant.maximum_copy_bytes < copy_bytes || grant.maximum_depth == 0 { return Ok(session_pending(0, 0)); }
    MOUNTED.with(|registry| { let mut registry = registry.borrow_mut(); let Some(app) = registry.app_slots.lookup(app_instance_id) else { return Ok(PluginLifecycleStep::Complete(Default::default())); }; registry.model_requests[app.index()] = None; registry.snapshot_owners[app.index()] = None; assert!(registry.app_slots.release(app_instance_id, app)); Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: copy_bytes, ..Default::default() })) })
}

pub fn terminal_is_empty(app_instance_id: u32) -> bool {
    MOUNTED.with(|registry| {
        let registry = registry.borrow();
        registry.app_slots.lookup(app_instance_id).is_none()
            && !registry.requests.has_consumer(app_instance_id)
            && (0..SESSION_ACTIVE_CAPACITY).all(|index|registry.requests.at(index).is_none_or(|(_,scope,_)|scope!=app_instance_id))
            && registry.retiring.iter().all(|entry| entry.is_none_or(|entry| entry.app_instance_id != app_instance_id))
    })
}
//#endregion 🎛️LiveMount
