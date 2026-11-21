"""🪙 Exercise the captured history fold through the actual actor, Source and lifecycle receipt."""
import os
from pathlib import Path
import subprocess
import sys

ticket = Path(__file__).resolve().parent
workspace = ticket.parents[6]
work = ticket / "🗑️generated"
lane = workspace / ".cache/b/r/m/u/l" / sys.argv[1]
family = os.environ.get("NORM_ARCHIVE_FAMILY", "din4108")
names = {"din4108": "DIN 4108", "din18599": "DIN V 18599", "din16798": "DIN 16798", "iso16757": "ISO 16757", "vdi3805": "VDI 3805", **{f"en199{i}": f"EN 199{i}" for i in range(10)}}
if family not in names:
    raise ValueError(f"Unsupported archive model {family}")
display = names[family]
stem = "archive-context" if family == "din4108" else family + "-archive-context"
if "diagnose-initializer" in sys.argv[2:]:
    stem += "-initializer"
if "runtime-cleanup" in sys.argv[2:]:
    stem += "-runtime"
if "member-maintenance" in sys.argv[2:]:
    stem += "-member"
if "runtime-clock" in sys.argv[2:]:
    stem += "-clock"
if "shell-note" in sys.argv[2:]:
    stem += "-note"
if "browser-commands" in sys.argv[2:]:
    stem += "-commands"
if "picker-archive" in sys.argv[2:]:
    stem += "-picker"
if "retained-store" in sys.argv[2:]:
    stem += "-store"
if "replacement-refusal" in sys.argv[2:]:
    stem += "-refusal"
if "repeat-archive" in sys.argv[2:]:
    if family != "din4108":
        raise ValueError("The captured repeated archive fixture is DIN 4108")
    stem += "-repeat"
source = work / (stem.replace("-", "_") + ".rs")
source.write_text('''
use semio_framework_actor_retained_context::OriginalActorContext;
use semio_framework_actor_retained_turn::{OriginalActorTurnWork, RetainedTurnInput};
use semio_framework_job::{OriginalTurnFailure, OriginalTurnPrimary, StepBudget, StepContext};
use semio_framework_value::{NativeDecodeControl, RetainedCloneGrant, RetainedCloneProgress, RetirementDemand, ValueError};
use semio_os_kernel::{ActorId, PersistedDocumentHydrationIdentity, PersistedDocumentHydrationStep,
                     PersistedDocumentHydrationTarget, RetainedPersistedDocumentHydration};
use semio_os_spr::{HistoryFoldJob, HistoryShape, history::RetainedHistoryFold};
use semio_plugin_mounted_owner::{MountedOwnerIdentityV1, MountedOwnerPhaseV1, MountedOwnerPolicyV1, MountedOwnerTurnV1};
use semio_plugin_ownership::bounded_ownership::{PluginLifecycleStep, original_lifecycle_step, original_lifecycle_delta};
use semio_s_artifact_norm_FAMILY_model::{FAMILY_DIALECT as DIALECT, FamilySnapshot as Snapshot};
use semio_s_artifact_norm_FAMILY_mutations::FamilyMutation as Mutation;

fn clock() -> Option<u64> {
    #[cfg(archive_runtime_clock)]
    {
        static START: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
        return u64::try_from(START.get_or_init(std::time::Instant::now).elapsed().as_micros()).ok();
    }
    #[cfg(not(archive_runtime_clock))]
    Some(1)
}

fn member_maintenance() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| std::env::args().any(|value| value == "member-maintenance"))
}

fn mounted_policy() -> MountedOwnerPolicyV1 {
    let mut policy = MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
    if member_maintenance() {
        policy.maintenance.maximum_copy_bytes = 256 * 1024;
    }
    policy
}


#[cfg(archive_replacement)]
mod replacement {
    use super::*;
    use semio_framework_async::poll::resolve_ready;
    use semio_framework_job::{NativeEncoderWork, StepContext};
    use semio_framework_value::{NativeEncodeControl, ValueError};
    use semio_os_kernel::os_store::serialization::ArtifactPackFiles;
    use semio_os_kernel::os_vcs::io::binary::entity_identity::control::{EntityIdentityAuthority, IdentityWorkLender, Observer};
    use semio_os_kernel::{DocumentArchiveLoadState, DocumentArchivePack};
    use semio_plugin::{AppActionRegistry, EditorApp, PluginApp, VcsArtifactApp};
    use semio_plugin_document_media::{ActiveArtifactStoreReplacementState, ActiveDocumentArchiveLoadPhase, OriginalDocumentArchiveAdmission};
    use semio_s_artifact_norm_FAMILY_editor::{FamilyPlayApp as PlayApp, create_FAMILY_app as create_app};

    const OPERATION: u64 = 1 << 61;

    struct IdentityWork<'a>(&'a mut NativeEncoderWork);
    impl IdentityWorkLender for IdentityWork<'_> {
        fn lend(&mut self, control: &mut NativeEncodeControl<'_>,
                operation: &mut dyn FnMut(&mut NativeEncodeControl<'_>) -> Result<(), ValueError>) -> Result<(), ValueError> {
            self.0.lend(control, operation)
        }
    }
    fn identity<R>(cx: &mut StepContext<'_>, run: impl FnOnce(&mut EntityIdentityAuthority<'_>, &mut StepContext<'_>) -> R) -> R {
        let mut work = cx.native_encoder_work();
        let observer: &mut Observer<'_> = &mut |_| true;
        let mut authority = EntityIdentityAuthority::new(1 << 24, observer).unwrap()
            .with_work_lender(Box::new(IdentityWork(&mut work)));
        let result = run(&mut authority, cx);
        drop(authority);
        cx.consume_fuel(work.performed());
        result
    }

    pub struct Archive {
        app: Box<VcsArtifactApp<EditorApp<PlayApp>, <PlayApp as semio_plugin::ArtifactEditor>::Members>>,
        pack: DocumentArchivePack,
        admission: OriginalDocumentArchiveAdmission,
        cancellation: bool,
        admitted: bool,
        status: Option<DocumentArchiveLoadState>,
        frontier: Option<String>,
        narrow: bool,
        isolate: bool,
        replay_fixture: bool,
        direct_initializer: bool,
        old_ack: bool,
        last_initializer: Option<String>,
        initial_generation: u64,
        #[cfg(archive_shell_note)]
        note: Option<ShellNote>,
    }

    #[cfg(archive_shell_note)]
    struct ShellNote {
        address: semio_framework::manifest::ActionAddress,
        args: Option<semio_framework_value::DslValue>,
        meta: Option<semio_plugin_decl_emission::ActionMeta>,
        fault: Option<semio_framework::Fault>,
    }

    #[cfg(archive_shell_note)]
    impl ShellNote {
        fn new(app_id: String) -> Self {
            use semio_framework_value::DslValue::{Object, String as Text};
            let window = "norm-FAMILY-outliner";
            Self {
                address: semio_framework::manifest::ActionAddress { plugin_id: "norm".into(), app_id, mode_id: "edit".into(), window_kind_id: window.into(), window_instance_id: window.into(), action_id: "noteShellCommand".into() },
                args: Some(Object(vec![("commandId".into(), Text("shell.windowResize".into())), ("label".into(), Object(vec![("en".into(), Text("Resize Window".into())), ("de".into(), Text("Fenster skalieren".into()))])), ("windowId".into(), Text(window.into()))])),
                meta: Some(semio_plugin_decl_emission::ActionMeta { expected_document_revision: None, actor: semio_os_kernel::LOCAL_ACTOR_ID.into(), instance_id: 1, view_state: None }),
                fault: None,
            }
        }
    }

    impl Archive {
        pub fn new(cx: &mut StepContext<'_>) -> Self {
            semio_plugin_decl_registry::artifact_registration::PluginRuntimeRegistry::publish_framework_schema_documents().unwrap();
            let registry = AppActionRegistry::from_definition(&create_app());
            let policy = mounted_policy();
            let actor = ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into());
            let mut app: Box<VcsArtifactApp<EditorApp<PlayApp>, <PlayApp as semio_plugin::ArtifactEditor>::Members>> = identity(cx, |identity, _| resolve_ready(
                VcsArtifactApp::with_registry_boxed(EditorApp::<PlayApp>::default(), registry, actor, policy, identity)))
                .expect("DISPLAY app");
            resolve_ready(app.bind_instance_id(1));
            for _ in 0..20_000 {
                if let Some((_, _, grant, progress)) = app.take_constructor_receipt() {
                    assert!(progress.fits(grant));
                }
                if !app.store_constructor_is_pending() { break; }
                let app = &mut *app;
                app.stores.drive_store_constructor_unit_cold(&mut app.constructor, policy.preparation)
                    .expect("DISPLAY store constructor turn");
            }
            assert!(!app.store_constructor_is_pending());
            let pack = app.document_load_archive(&ArtifactPackFiles {
                pack: include_bytes!("FAMILY-observed.pack").to_vec(),
                spr: include_bytes!("FAMILY-observed.spr").to_vec(),
                ops: String::new(),
            }).unwrap();
            let initial_generation = app.stores.store.generation_now();
            Self { app, pack, admission: OriginalDocumentArchiveAdmission::empty(), cancellation: false,
                   admitted: false, status: None, frontier: None, narrow: std::env::args().any(|value| value == "narrow"),
                   isolate: std::env::args().any(|value| value == "isolate"),
                   replay_fixture: std::env::args().any(|value| value == "old-empty-replay"),
                   direct_initializer: std::env::args().any(|value| value == "direct-initializer"),
                   old_ack: std::env::args().any(|value| value == "old-ack"), last_initializer: None, initial_generation,
                   #[cfg(archive_shell_note)]
                   note: None }
        }

        pub fn attempt(&mut self, cx: &mut StepContext<'_>, turn: usize) {
            #[cfg(archive_shell_note)]
            if let Some(note) = self.note.as_mut() {
                let before = cx.retained_progress();
                let result = identity(cx, |identity, cx| resolve_ready(self.app.handle_action_invocation(
                    &note.address, &mut note.args, &mut note.fault, Some("edit"), &mut note.meta, identity, cx,
                ))).unwrap_or_else(|failure| panic!("shell note invocation {turn}: {failure:?}"));
                if turn < 340 || turn % 500 == 0 {
                    println!("[DEBUG] note {turn}: action source {}, lease {}, effect {:?}, grant {:?}",
                        self.app.ingress.action_source_pending(), self.app.tools.pending_tool_lease_admission.is_some(),
                        original_lifecycle_delta(before, cx.retained_progress()).unwrap(), cx.retained_grant());
                }
                if let Some(result) = result {
                    assert!(result.requested_effects.iter().any(|effect| matches!(effect, semio_framework::Effect::SpawnJob { .. })), "native note admission publishes a host job");
                    println!("DISPLAY native shell note published its reserved job on turn {turn}");
                    std::process::exit(0);
                }
                return;
            }
            if !self.cancellation {
                self.cancellation = self.app.admit_cancellation_issuer(cx).unwrap();
                return;
            }
            if !self.admitted {
                self.admitted = self.app.begin_document_archive_load(OPERATION, &mut self.pack, &mut self.admission, cx)
                    .unwrap_or_else(|failure| panic!("admission {turn}: {failure:?}"));
                return;
            }
            let phase = self.app.mounted_owner_maintenance_phase().unwrap();
            let identity = MountedOwnerIdentityV1 { instance_id: 1, operation: cx.operation(), generation: cx.generation() };
            let policy = mounted_policy();
            let isolated = self.isolate && self.app.ingest.document_archive_loads.get(OPERATION).is_some_and(|archive|
                matches!(archive.phase, ActiveDocumentArchiveLoadPhase::DecodeParent | ActiveDocumentArchiveLoadPhase::HydrateParent));
            let direct_initializer = self.direct_initializer && self.app.ingest.document_archive_loads.get(OPERATION)
                .and_then(|archive| archive.replacement).and_then(|handle| self.app.ingest.store_replacement_jobs.get(handle.operation.0))
                .is_some_and(|job| job.state == ActiveArtifactStoreReplacementState::Initializing);
            let replacement = self.app.ingest.document_archive_loads.get(OPERATION).and_then(|archive| archive.replacement);
            if isolated && self.replay_fixture {
                if let Some(hydration) = self.app.ingest.document_archive_loads.get_mut(OPERATION)
                    .and_then(|archive| archive.hydration.as_mut())
                    .filter(|hydration| format!("{:?}", hydration.phase) == "CloseEnvelopeOwners")
                {
                    assert!(hydration.replay_ids.as_ref().is_none_or(|ids| ids.capacity() == 0));
                    assert!(hydration.replay_order.as_ref().is_none_or(|order| order.is_empty()));
                    hydration.replay_ids.take();
                    hydration.replay_order.take();
                }
            }
            let mut drive = |cx: &mut StepContext<'_>| -> Result<(), OriginalTurnFailure> {
                if isolated && self.replay_fixture {
                    let grant = cx.retained_grant();
                    self.app.ladder_ctx().drive_document_archive_load_retirements(cx, grant, false)?;
                    return Ok(());
                }
                if direct_initializer {
                    if self.old_ack {
                        let job = self.app.ingest.store_replacement_jobs.get_mut(replacement.unwrap().operation.0).unwrap();
                        let copy = std::env::args().find_map(|value| value.strip_prefix("initializer-bytes=")?.parse().ok()).unwrap_or(24_576);
                        let grant = RetainedCloneGrant { maximum_copy_bytes: copy, ..policy.maintenance };
                        let before = cx.retained_progress();
                        let fuel = cx.fuel_remaining();
                        let step = cx.with_retained_loan(grant, |owner| {
                            if std::env::args().any(|value| value == "no-current") {
                                job.drive_initializer(owner, grant).map_err(Into::into)
                            } else {
                                owner.with_replacement_current(job.replacement_identity(), |owner| job.drive_initializer(owner, grant).map_err(Into::into))
                            }
                        })?;
                        let actual = original_lifecycle_delta(before, cx.retained_progress())?;
                        let reported = step.progress().unwrap_or_default();
                        cx.consume_retained(RetainedCloneProgress {
                            copied_items: reported.copied_items.saturating_sub(actual.copied_items),
                            copied_bytes: reported.copied_bytes.saturating_sub(actual.copied_bytes),
                            retained_capacity_bytes: reported.retained_capacity_bytes.saturating_sub(actual.retained_capacity_bytes),
                            released_bytes: reported.released_bytes.saturating_sub(actual.released_bytes),
                        })?;
                        if fuel == cx.fuel_remaining() { cx.consume_fuel(1); }
                        return Ok(());
                    }
                    let grant = cx.retained_grant();
                    self.app.ladder_ctx().drive_store_replacement_jobs(cx, grant, false)?;
                    return Ok(());
                }
                let mut mounted = MountedOwnerTurnV1::admit(1, identity, phase, policy, cx)?.unwrap();
                if isolated {
                    mounted.advance_lifecycle(|cx, grant| self.app.ladder_ctx().drive_document_archive_load_retirements(cx, grant, false))?;
                } else {
                    let before = mounted.context().retained_progress();
                    if let Err(failure) = self.app.mounted_owner_maintenance_step(&mut mounted) {
                        let failure: OriginalTurnFailure = failure.into();
                        let old_window_admission = self.old_ack && replacement
                            .and_then(|handle| self.app.ingest.store_replacement_jobs.get(handle.operation.0))
                            .is_some_and(|job| job.state == ActiveArtifactStoreReplacementState::CandidateReady);
                        if !old_window_admission || before == mounted.context().retained_progress()
                            || failure.primary.message() != "original lifecycle child receipt differs from its whole producer receipt"
                        {
                            return Err(failure);
                        }
                        println!("turn {turn}: older window admission posted {:?}",
                                 original_lifecycle_delta(before, mounted.context().retained_progress())?);
                    }
                }
                Ok(())
            };
            let progress = if isolated {
                let copy = if self.replay_fixture { 24_576 } else { 12_288 };
                cx.with_retained_loan(RetainedCloneGrant { maximum_copy_bytes: copy, ..policy.maintenance }, drive)
            } else if self.narrow {
                let peak = if std::env::args().any(|value| value == "funded-candidate-turn") { 32_768 } else { 24_576 };
                let copy = [3509, 8192, 16_384, peak][turn % 4];
                cx.with_retained_loan(RetainedCloneGrant { maximum_copy_bytes: copy, ..policy.maintenance }, drive)
            } else {
                drive(cx)
            };
            if let Some(archive) = self.app.ingest.document_archive_loads.get(OPERATION) {
                let hydration = archive.hydration.as_ref().map(|owner| owner.phase);
                let replacement = archive.replacement.and_then(|handle| self.app.ingest.store_replacement_jobs.get(handle.operation.0))
                    .map(|job| (job.state, job.initializer_demands(32_768), job.session.as_ref().map(|session| session.poll()),
                        job.session_rejected.as_ref().map(|source| (source.admission_pending(), source.job().is_some())),
                        job.outcome_pending, job.resume_pending, job.fold_progress));
                let frontier = format!("{:?}, hydration={hydration:?}, replacement={replacement:?}", archive.phase);
                if direct_initializer {
                    let details = archive.replacement.and_then(|handle| self.app.ingest.store_replacement_jobs.get(handle.operation.0))
                        .and_then(|job| job.session.as_ref()).and_then(|session| {
                            let job = session.checked_out_job()?;
                            Some(format!("progress={:?}; demand={:?}; receipt={:?}", job.progress(), job.retirement_demands(32_768), session.checked_out_retained_step_progress()))
                        });
                    if details.is_some() && self.last_initializer != details {
                        println!("turn {turn}: initializer {details:?}");
                        self.last_initializer = details;
                    }
                }
                if self.frontier.as_ref() != Some(&frontier) && (!direct_initializer || turn < 1400 || turn % 1000 == 0) {
                    println!("turn {turn}: {frontier}");
                    self.frontier = Some(frontier);
                }
            }
            progress.unwrap_or_else(|failure| panic!("archive maintenance {turn}: {failure:?}"));
            let status = replacement::identity(cx, |identity, cx| {
                resolve_ready(self.app.poll_document_archive_load(OPERATION, identity, cx))
            }).expect("archive load status");
            if self.status != Some(status.state) {
                println!("turn {turn}: archive {:?} {}/{}", status.state, status.completed, status.total);
                self.status = Some(status.state);
            }
            match status.state {
                DocumentArchiveLoadState::Pending | DocumentArchiveLoadState::Running => {}
                DocumentArchiveLoadState::Ready => {
                    assert!(self.app.stores.store.generation_now() > self.initial_generation);
                    self.app.acknowledge_document_archive_load(OPERATION).expect("archive ready acknowledgement");
                    assert!(self.app.ingest.document_archive_loads.get(OPERATION).is_none());
                    println!("DISPLAY archived document installed and acknowledged on turn {turn}");
                    #[cfg(archive_shell_note)]
                    {
                        self.note = Some(ShellNote::new("s.norm.FAMILY@1/*#editor".to_string()));
                        return;
                    }
                    #[cfg(not(archive_shell_note))]
                    std::process::exit(0);
                }
                _ => panic!("archive refused: {status:?}"),
            }
        }
    }

    #[cfg(archive_browser_commands)]
    fn browser_command(bytes: &[u8]) -> semio_os_kernel::AppCommand {
        fn number(bytes: &mut &[u8]) -> u64 {
            let mut value = 0;
            for shift in (0..64).step_by(7) {
                let (byte, suffix) = bytes.split_first().unwrap();
                *bytes = suffix;
                value |= u64::from(byte & 127) << shift;
                if byte & 128 == 0 { return value; }
            }
            panic!("wire length exceeds u64")
        }
        assert_eq!(bytes[0], 1);
        let mut rest = &bytes[1..];
        let seq = number(&mut rest);
        let length = number(&mut rest) as usize;
        let (command, mut bytes) = rest.split_at(length);
        assert_eq!(number(&mut bytes) as usize, bytes.len());
        semio_os_kernel::AppCommand::Command { seq, command: command.to_vec(), view_state: bytes.to_vec() }
    }

    #[cfg(archive_browser_commands)]
    const BROWSER_COMMANDS: [(&str, &[u8]); 2] = [
        ("noteShellCommand", include_bytes!("din4108-note.bin")),
        ("setActiveExample", include_bytes!("din4108-example.bin")),
    ];

    #[cfg(archive_runtime_cleanup)]
    pub struct RuntimeArchive {
        runtime: semio_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<VcsArtifactApp<EditorApp<PlayApp>, <PlayApp as semio_plugin::ArtifactEditor>::Members>>,
        pack: DocumentArchivePack,
        admission: OriginalDocumentArchiveAdmission,
        cancellation: bool,
        admitted: bool,
        initial_generation: u64,
        last_state: Option<(DocumentArchiveLoadState, u64)>,
        last_phase: Option<String>,
        operation: u64,
        archive_index: usize,
        #[cfg(archive_energy_member)]
        distinct_replacement_seen: bool,
        #[cfg(archive_browser_commands)]
        commands: Option<(usize, semio_plugin_runtime_ops::ops_ingress::OriginalDecodedPluginCommand)>,
    }

    #[cfg(archive_runtime_cleanup)]
    impl RuntimeArchive {
        pub fn new(cx: &mut StepContext<'_>) -> Self {
            let Archive { app, pack, admission, initial_generation, .. } = Archive::new(cx);
            let policy = mounted_policy();
            let runtime = semio_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::new(policy).unwrap();
            let surfaces = Default::default();
            #[cfg(archive_browser_commands)]
            let surfaces = {
                use semio_plugin_plugin_program::surface_context::SurfaceContexts;
                let mut surfaces: SurfaceContexts = surfaces;
                let semio_os_kernel::AppCommand::Command { view_state, .. } = browser_command(BROWSER_COMMANDS[0].1) else { unreachable!() };
                let view: semio_framework::ViewModel = resolve_ready(semio_plugin_plugin_program::decode_wire_serialized(&view_state)).unwrap();
                for window in &view.window_instances {
                    let mut projected = view.clone();
                    projected.window_id = Some(window.id.clone());
                    surfaces.insert(window.id.clone(), "window-kind".into(), projected).unwrap();
                }
                runtime.instance_actors.borrow_mut().insert_admitted(1,
                    semio_plugin_reactor_lifecycle::life_instance_registry::RuntimeActorAuthority::new(semio_os_kernel::LOCAL_ACTOR_ID).unwrap());
                runtime.plugin.replace(Some(semio_plugin_plugin_program::Plugin::new("norm", "Norm", "1")));
                surfaces
            };
            let cell = semio_plugin_reactor_lifecycle::life_app_cell::RuntimeAppCell::new(
                semio_plugin::AppInstance { id: 1, app: *app, surface_contexts: surfaces }, policy,
            );
            cell.maintenance_generation.store(cx.generation().0, std::sync::atomic::Ordering::Relaxed);
            runtime.instances.borrow_mut().insert_admitted(1, std::sync::Arc::new(cell));
            Self { runtime, pack, admission, cancellation: false, admitted: false, initial_generation, last_state: None, last_phase: None, operation: OPERATION, archive_index: 0,
                #[cfg(archive_energy_member)]
                distinct_replacement_seen: false,
                #[cfg(archive_browser_commands)]
                commands: None }
        }

        pub fn attempt(&mut self, cx: &mut StepContext<'_>, turn: usize) {
            #[cfg(archive_browser_commands)]
            let isolate_command = self.commands.is_some() && std::env::args().any(|arg| arg == "command-only");
            #[cfg(not(archive_browser_commands))]
            let isolate_command = false;
            let direct_member_turn = turn >= 10_500 && std::env::args().any(|arg| arg == "direct-member-turn");
            if self.admitted && !isolate_command && !direct_member_turn {
                semio_plugin_runtime_instances::inst_cleanup_turns::plugin_step_live_cleanup(&self.runtime, cx)
                    .unwrap_or_else(|failure| panic!("runtime archive cleanup {turn}: {failure:?}"));
            }
            #[cfg(archive_browser_commands)]
            if let Some((index, owner)) = self.commands.as_mut() {
                let available = cx.retained_grant();
                let copy = std::env::args().find_map(|arg| arg.strip_prefix("command-copy=")?.parse::<usize>().ok());
                let grant = RetainedCloneGrant { maximum_copy_bytes: copy.map_or(available.maximum_copy_bytes, |copy| available.maximum_copy_bytes.min(copy)), ..available };
                let before = cx.retained_progress();
                let mut result = None;
                cx.with_retained_loan(grant, |cx| {
                    result = identity(cx, |identity, cx| resolve_ready(owner.advance_manifest_step(&self.runtime, 1, identity, cx)))?;
                    Ok(())
                }).unwrap_or_else(|error| panic!("browser {} on turn {turn}: {error:?}", BROWSER_COMMANDS[*index].0));
                if std::env::args().any(|arg| arg == "reactor-phases") {
                    use semio_plugin_runtime_ops::ops_typed_operations::{
                        plugin_continue_typed_operations, plugin_drive_pending_effects, TypedOperationGrant,
                        TYPED_OPERATION_UNITS_PER_TURN,
                    };
                    let grant = TypedOperationGrant {
                        units: TYPED_OPERATION_UNITS_PER_TURN.min(u32::try_from(cx.fuel_remaining()).unwrap_or(u32::MAX)),
                        max_frames: 64, max_effects: 64, deadline_us: Some(cx.deadline_us()), clock: cx.clock_source(),
                    };
                    let (continued, _) = identity(cx, |identity, cx| {
                        resolve_ready(plugin_continue_typed_operations(&self.runtime, grant, identity, cx))
                    }).unwrap_or_else(|error| panic!("reactor typed continuation on turn {turn}: {error:?}"));
                    if let Some(output) = continued { std::mem::forget(output); }
                    let mut effects = Vec::new();
                    plugin_drive_pending_effects(&self.runtime, &mut effects, cx).unwrap();
                    semio_plugin_runtime_ops::ops_surface_render::plugin_drive_displaced_surfaces(&self.runtime, cx).unwrap();
                    if !effects.is_empty() { std::mem::forget(effects); }
                }

                if turn % 100 == 0 {
                    println!("[DEBUG] browser {} {turn}: dispatch applied {}, copy {}", BROWSER_COMMANDS[*index].0,
                        owner.receiving.as_ref().is_some_and(|owner| owner.finished()),
                        cx.retained_progress().copied_bytes - before.copied_bytes);
                }
                if let Some((seq, result)) = result {
                    if *index == 0 {
                        assert!(result.requested_effects.iter().any(|effect| matches!(effect, semio_framework::Effect::SpawnJob { .. })));
                    }
                    println!("mounted browser {} returned sequence {seq} on turn {turn}: effects {:?}, output {:?}",
                        BROWSER_COMMANDS[*index].0, result.requested_effects, result.output);
                    let next = *index + 1;
                    if next == BROWSER_COMMANDS.len() { std::process::exit(0); }
                    if std::env::args().any(|arg| arg == "settle-note") {
                        let instances = self.runtime.instances.borrow();
                        let cell = instances.get(1).unwrap();
                        let mut instance = cell.background_instance().unwrap();
                        let committed = resolve_ready(semio_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission(
                            &mut instance.app, result, cx,
                        )).unwrap_or_else(|error| panic!("native reserved note job did not settle: {error:?}"));
                        println!("mounted editor settled actual browser note job on turn {turn}: history committed {}", committed.history_patch.is_some());
                        std::mem::forget(committed);
                    } else {
                        std::mem::forget(result);
                    }
                    let next_owner = semio_plugin_runtime_ops::ops_ingress::OriginalDecodedPluginCommand::new(browser_command(BROWSER_COMMANDS[next].1));
                    std::mem::forget(std::mem::replace(owner, next_owner));
                    *index = next;
                }
                return
            }
            let instances = self.runtime.instances.borrow();
            let cell = instances.get(1).unwrap();
            let mut guard = cell.background_instance().unwrap_or_else(|_| panic!("runtime app not available on turn {turn}"));
            let app = &mut guard.app;
            if direct_member_turn {
                let handle = app.ingest.document_archive_loads.get(self.operation).and_then(|archive| archive.replacement);
                if let Some(job) = handle.and_then(|handle| app.ingest.store_replacement_jobs.get_mut(handle.operation.0))
                    .filter(|job| job.state == ActiveArtifactStoreReplacementState::OpeningMembers)
                {
                    let before = cx.retained_progress();
                    let work = cx.fuel_remaining();
                    let grant = mounted_policy().maintenance;
                    let step = cx.with_original_retirement(|cx| cx.with_replacement_current(job.replacement_identity(), |cx| {
                        job.drive_member_open(cx, grant).map_err(Into::into)
                    })).unwrap_or_else(|failure| panic!("direct member turn {turn}: {failure:?}"));
                    let progress = original_lifecycle_delta(before, cx.retained_progress()).unwrap();
                    if turn % 500 == 0 || job.state != ActiveArtifactStoreReplacementState::OpeningMembers {
                        println!("[DEBUG] direct member {turn}: {step:?}, actual {progress:?}, work {}, stage {}", work - cx.fuel_remaining(), cx.stage());
                    }
                    if job.state != ActiveArtifactStoreReplacementState::OpeningMembers {
                        std::process::exit(0);
                    }
                }
            }
            if !self.cancellation {
                self.cancellation = app.admit_cancellation_issuer(cx).unwrap();
                return;
            }
            if !self.admitted {
                self.admitted = app.begin_document_archive_load(self.operation, &mut self.pack, &mut self.admission, cx).unwrap();
                return;
            }
            if let Some(archive) = app.ingest.document_archive_loads.get(self.operation) {
                #[cfg(archive_energy_member)]
                if let Some(handle) = archive.replacement {
                    self.distinct_replacement_seen |= (handle.operation, handle.generation) != (cx.operation(), cx.generation());
                }
                let replacement = archive.replacement.and_then(|handle| app.ingest.store_replacement_jobs.get(handle.operation.0))
                    .map(|job| {
                        let details = std::env::args().any(|value| value == "retained-store").then(|| {
                            let copy = mounted_policy().maintenance.maximum_copy_bytes;
                            let store = job.retained_store.as_ref().map(|store| (store.close_owned_demands(copy), store.close_owned_store_terminal_is_empty(), store.factory_aliases_present()));
                            (job.retirement_demands(copy), store)
                        });
                        (job.state, job.terminal_target, job.committed, details, &job.refusal)
                    });
                let phase = format!("{:?}, {replacement:?}", archive.phase);
                if turn % 500 == 0 && std::env::args().any(|arg| arg == "replacement-stock") {
                    if let Some(job) = archive.replacement.and_then(|handle| app.ingest.store_replacement_jobs.get(handle.operation.0)) {
                        let maintenance = mounted_policy().maintenance;
                        let quote = job.member_open_demands(maintenance.maximum_copy_bytes);
                        println!("[DEBUG] replacement stock {turn}: {:?}, open {}, quote {quote:?}; stock {:?}; spent {:?}; work {}", job.state,
                            job.active_member_open.is_some(),
                            cx.retained_grant(), cx.retained_progress(), cx.fuel_remaining());
                        #[cfg(archive_energy_member)]
                        if let Some(semio_s_artifact_stdio_semio::SemioMembersOpen::Table(open)) = job.active_member_open.as_ref() {
                            use semio_os_kernel::os_store::MemberOpenOperation;
                            println!("[DEBUG] climate table {turn}: {}/{} owners, snapshot {}, history {}, selected {}, dictionary {}, witness {}, retired {}, demand {:?}, last stage {}",
                                open.owners.is_some(), open.owners.as_ref().is_some_and(|owners| owners.constructor_is_complete()),
                                open.snapshot.is_some(), open.history.is_some(), open.selected.is_some(), open.dictionary.is_some(),
                                open.witness.is_some(), open.terminal_is_empty(), open.retirement_demands(maintenance.maximum_copy_bytes),
                                semio_plugin_reactor_app::LAST_MAINTENANCE_STAGE.load(std::sync::atomic::Ordering::Relaxed));
                        }
                    }
                }
                if self.last_phase.as_ref() != Some(&phase) || turn % 500 == 0 {
                    println!("[DEBUG] runtime archive {turn}: {phase}; claim {}", cx.last_claim_reason());
                    self.last_phase = Some(phase);
                }
            }
            let status = identity(cx, |identity, cx| resolve_ready(app.poll_document_archive_load(self.operation, identity, cx))).unwrap();
            let next = (status.state, status.completed);
            if self.last_state != Some(next) {
                println!("runtime archive {turn}: {:?} {}/{}", status.state, status.completed, status.total);
                self.last_state = Some(next);
            }
            if status.state == DocumentArchiveLoadState::Ready {
                assert!(app.stores.store.generation_now() > self.initial_generation);
                #[cfg(archive_energy_member)]
                {
                    use semio_s_artifact_norm_din18599_model::{DIN18599_CLIMATE_TABLE_SLOT, din18599_climate_data_from_table};
                    use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;
                    assert!(self.distinct_replacement_seen, "member replay exercised distinct source and replacement identities");
                    let snapshot = app.stores.store.snapshot_ref();
                    let table = app.child_composition.child_content_root
                        .typed_read::<SemioTableSnapshot>(DIN18599_CLIMATE_TABLE_SLOT, &snapshot.climate_table.child_id)
                        .expect("mounted archive installed its climate table");
                    assert_eq!((table.columns.len(), table.rows.len()), (2, 12));
                    assert_eq!(din18599_climate_data_from_table(&table), snapshot.climate);
                }
                app.acknowledge_document_archive_load(self.operation).unwrap();
                assert!(app.ingest.document_archive_loads.get(self.operation).is_none());
                println!("DISPLAY runtime archive installed and acknowledged on turn {turn}");
                #[cfg(archive_repeat)]
                {
                    let remaining = [
                        (2564, &include_bytes!("din4108-round-trip-2.pack")[..], &include_bytes!("din4108-round-trip-2.spr")[..]),
                        (5486, &include_bytes!("din4108-round-trip-3.pack")[..], &include_bytes!("din4108-round-trip-3.spr")[..]),
                    ];
                    let Some((operation, pack, spr)) = remaining.get(self.archive_index).copied() else { std::process::exit(0); };
                    self.archive_index += 1;
                    self.operation = operation;
                    self.pack = app.document_load_archive(&ArtifactPackFiles { pack: pack.to_vec(), spr: spr.to_vec(), ops: String::new() }).unwrap();
                    self.admission = OriginalDocumentArchiveAdmission::empty();
                    self.initial_generation = app.stores.store.generation_now();
                    self.admitted = false;
                    self.last_state = None;
                    self.last_phase = None;
                    println!("[DEBUG] runtime next archive {operation}, copy {}", cx.retained_grant().maximum_copy_bytes);
                    return;
                }
                #[cfg(archive_browser_commands)]
                {
                    self.commands = Some((0, semio_plugin_runtime_ops::ops_ingress::OriginalDecodedPluginCommand::new(browser_command(BROWSER_COMMANDS[0].1))));
                }
                #[cfg(not(archive_browser_commands))]
                std::process::exit(0);
            }
        }
    }
}

#[cfg(archive_initializer)]
mod initializer {
    use super::*;
    use semio_os_kernel::os_store::document_store_owners::BoundedDocumentStoreOwnerProducer;
    use semio_os_store_initialization_replay::{ArtifactStoreReplayInitializer, Phase, initialization_authority::ArtifactStoreInitializationAuthority};
    #[cfg(archive_initializer_worker)]
    use semio_plugin_document_media::{ActiveArtifactStoreReplacement, ActiveArtifactStoreReplacementState};
    #[cfg(archive_initializer_worker)]
    use semio_plugin_ownership::store_lifecycle::ArtifactStoreInitializationJob;

    type Replay = ArtifactStoreReplayInitializer<Snapshot, Mutation, BoundedDocumentStoreOwnerProducer>;
    type Envelope = semio_os_kernel::store_schema::ArtifactEnvelope<Snapshot, Mutation>;
    #[cfg(archive_initializer_worker)]
    type Store = semio_os_kernel::os_store::ArtifactStore<Snapshot, Mutation>;
    #[cfg(archive_initializer_worker)]
    type Worker = ActiveArtifactStoreReplacement<Snapshot, Mutation, semio_os_kernel::NoMembers>;

    pub enum Probe {
        Direct(Replay),
        #[cfg(archive_initializer_worker)]
        Worker(Worker),
    }

    impl Probe {
        pub fn new(envelope: Envelope, cx: &StepContext<'_>, _worker: bool) -> Self {
            #[cfg(archive_initializer_worker)]
            if _worker { return Self::Worker(worker(envelope, cx)); }
            Self::Direct(replay(envelope, cx))
        }

        pub fn step(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant, turn: usize) {
            match self {
                Self::Direct(replay) => step_replay(replay, cx, grant, turn),
                #[cfg(archive_initializer_worker)]
                Self::Worker(worker) => step_worker(worker, cx, grant, turn),
            }
        }
    }

    fn replay(envelope: Envelope, cx: &StepContext<'_>) -> Replay {
        Replay::new(envelope, "semio.norm.FAMILY/v1", cx.operation(), cx.generation(), ActorId("local".into()))
    }

    #[cfg(archive_initializer_worker)]
    struct TracedInitializer {
        replay: Replay,
        last_phase: Option<Phase>,
        calls: usize,
    }

    #[cfg(archive_initializer_worker)]
    impl ArtifactStoreInitializationAuthority<Snapshot, Mutation> for TracedInitializer {
        fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, OriginalTurnFailure> {
            let phase = self.replay.phase;
            self.calls += 1;
            if self.last_phase != Some(phase) || self.calls % 100 == 0 {
                println!("[DEBUG] initializer callback #{} {phase:?}, grant {:?}, fuel {}", self.calls, cx.retained_grant(), cx.fuel_remaining());
                self.last_phase = Some(phase);
            }
            #[cfg(archive_registry_quote)]
            if phase == Phase::BindGenesis && !self.replay.registry_birth.ready()? {
                advance_registry_birth(&mut self.replay.registry_birth, cx)?;
                return Ok(semio_framework_job::JobOutcomeBorrow::admit_yield(cx)?);
            }
            self.replay.step(cx)
        }

        fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, ValueError> {
            self.replay.borrow_outcome(descriptor)
        }

        fn progress(&self) -> (u64, u64) { self.replay.progress() }
        fn next_step_demands(&self) -> Result<RetirementDemand, ValueError> {
            #[cfg(archive_candidate_quote)]
            if self.replay.phase == Phase::BuildCandidate && self.replay.candidate.is_none() {
                let owner = Store::initialized_runtime_adoption_demands(self.replay.envelope.as_ref().unwrap(), self.replay.runtime.as_ref().unwrap())?;
                let demand = semio_framework_job::OriginalConstructionDemand {
                    items: 1, copy_bytes: owner.copy_bytes + std::mem::size_of::<Box<Store>>(),
                    capacity_bytes: owner.capacity_bytes + std::mem::size_of::<Store>(), release_bytes: owner.release_bytes, depth: owner.depth,
                };
                let (retained, _) = semio_framework_job::original_construction_reserve::<Box<Store>>(demand, 1)?;
                let quote = retained.physical();
                static REPORTED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
                if !REPORTED.swap(true, std::sync::atomic::Ordering::Relaxed) {
                    println!("[DEBUG] candidate body {demand:?}, attempt quote {quote:?}");
                }
                return Ok(quote);
            }
            self.replay.next_step_demands()
        }
        fn bind_replacement(&mut self, replacement: semio_framework_job::ReplacementIdentity) { self.replay.bind_replacement(replacement); }
        fn request_cancel(&mut self) { self.replay.request_cancel(); }
        fn take_candidate(&mut self) -> Option<Store> { self.replay.take_candidate() }
        fn take_boxed_candidate(&mut self) -> Option<Box<Store>> { self.replay.take_boxed_candidate() }
        fn retirement_demands(&self, maximum_copy_bytes: usize) -> Result<RetirementDemand, ValueError> { self.replay.retirement_demands(maximum_copy_bytes) }
        fn begin_close(&mut self) { self.replay.begin_close(); }
        fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<semio_framework_value::RetainedCloneStep, OriginalTurnFailure> { self.replay.close_step(cx) }
        fn terminal_is_empty(&self) -> bool { self.replay.terminal_is_empty() }
    }

    #[cfg(archive_initializer_worker)]
    fn worker(envelope: Envelope, cx: &StepContext<'_>) -> Worker {
        let replay = replay(envelope, cx);
        let job = ArtifactStoreInitializationJob::new(Box::new(TracedInitializer { replay, last_phase: None, calls: 0 }));
        Worker::new(cx.operation(), cx.generation(), 0, job, (cx.operation(), cx.generation()))
    }

    #[cfg(archive_initializer_worker)]
    fn step_worker(worker: &mut Worker, cx: &mut StepContext<'_>, policy: RetainedCloneGrant, turn: usize) {
        let before = cx.retained_progress();
        let step = cx.with_retained_loan(policy, |owner| {
            let grant = owner.retained_grant();
            worker.drive_initializer(owner, grant).map_err(Into::into)
        })
            .unwrap_or_else(|failure| panic!("initializer worker {turn}: {failure:?}"));
        let observed = original_lifecycle_delta(before, cx.retained_progress()).unwrap();
        let report = step.progress().unwrap_or_default();
        cx.consume_retained(original_lifecycle_delta(observed, report).unwrap()).unwrap();
        if worker.state != ActiveArtifactStoreReplacementState::Initializing {
            assert_eq!(worker.state, ActiveArtifactStoreReplacementState::AwaitingMembers);
            assert!(worker.retained_store.is_some());
            println!("DISPLAY original worker produced a candidate and retired on turn {turn}");
            std::process::exit(0);
        }
    }

    fn step_replay(replay: &mut Replay, cx: &mut StepContext<'_>, policy: RetainedCloneGrant, turn: usize) {
        let phase = replay.phase;
        let before = cx.retained_progress();
        cx.with_retained_loan(policy, |owner| replay.step(owner).map(|_| ())).unwrap_or_else(|failure| panic!("replay {turn} {phase:?}: {failure:?}"));
        let receipt = original_lifecycle_delta(before, cx.retained_progress()).unwrap();
        if replay.phase != phase || turn % 100 == 0 {
            println!("turn {turn}: initializer phase {phase:?} to {:?}; receipt {receipt:?}", replay.phase);
        }
        if replay.phase == Phase::Complete {
            println!("DISPLAY initializer produced the exact candidate on turn {turn}");
            std::process::exit(0);
        }
    }
}

fn main() {
    if std::env::args().any(|value| value == "runtime-diagnostics") {
        semio_framework_job::set_runtime_diagnostics(true);
    }
    let wire = std::env::args().any(|value| value == "wire");
    let reconcile = std::env::args().any(|value| value == "reconcile");
    let wrap = std::env::args().any(|value| value == "lifecycle");
    let hydrate = std::env::args().any(|value| value == "hydrate");
    let quote_fold = std::env::args().any(|value| value == "quote");
    #[cfg(archive_replacement)]
    let mut replace = None::<replacement::Archive>;
    #[cfg(archive_runtime_cleanup)]
    let mut runtime = None::<replacement::RuntimeArchive>;
    #[cfg(archive_initializer)]
    let (mut replay, mut envelope) = (None, None);
    let diagnosis = std::env::args().any(|value| value == "diagnose-initializer");
    let worker_diagnosis = std::env::args().any(|value| value == "worker-claim");
    let narrow_diagnosis = std::env::args().any(|value| value == "narrow");
    let funded_candidate_turn = std::env::args().any(|value| value == "funded-candidate-turn");
    let runtime_clock = cfg!(archive_runtime_clock);
    let spr = include_bytes!("FAMILY-observed.spr");
    let source = std::sync::Arc::new(semio_framework_async::poll::resolve_ready(
        semio_os_spr::decode_history(spr, &Default::default())).unwrap());
    let mut runner = Some(move |control| async move { source.fold_controlled(HistoryShape::Document, &control).await });
    let mut input = RetainedTurnInput {
        operation: 1, generation: 1, epoch: 1,
        work: OriginalActorTurnWork { clock_binding: 1, absolute_deadline_us: 3_600_000_000, fuel: if runtime_clock { 50_000_000 } else { 900_000_000 } },
        grant: RetainedCloneGrant {
            maximum_items: 8_388_608, maximum_copy_bytes: std::env::args().find_map(|value| value.strip_prefix("original-copy=")?.parse::<usize>().ok()).unwrap_or(536_870_912),
            maximum_capacity_bytes: 536_870_912, maximum_release_bytes: 536_870_912, maximum_depth: 64,
        },
    };
    let policy = RetainedCloneGrant {
        maximum_items: 64, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 16_777_216,
        maximum_release_bytes: 1_048_576, maximum_depth: 64,
    };
    let turn_fuel = if runtime_clock { 50_000_000 } else if member_maintenance() { 1 << 20 } else { 65_536 };
    let mut actor = OriginalActorContext::new((), input).unwrap();
    for _ in 0..100 {
        let budget = StepBudget::new(turn_fuel.min(input.work.fuel), input.work.absolute_deadline_us, input.grant);
        let ready = actor.admit_step(input, budget, clock).unwrap().ready;
        input.epoch += 1;
        input.work = actor.remaining_work().unwrap();
        input.grant = actor.remaining_grant().unwrap();
        if ready { break; }
    }
    actor.take_input().unwrap();
    let mut wire_ready = !wire;
    let mut hydration = hydrate.then(|| {
        let pack = include_bytes!("FAMILY-observed.pack").to_vec();
        let history = semio_framework_async::poll::resolve_ready(
            semio_os_spr::decode_history(spr, &Default::default())).unwrap();
        let identity = PersistedDocumentHydrationIdentity {
            id: history.doc_id.clone(), dialect: Some(DIALECT.into()),
        };
        let (owners, _) = semio_os_kernel::os_store::document_store_owners::
            bounded_artifact_store_owners::<Snapshot, Mutation>(policy)
            .unwrap_or_else(|failure| panic!("owners: {}", failure.error));
        RetainedPersistedDocumentHydration::from_pack(
            pack, history, identity, None, "semio.norm.FAMILY/v1".into(), owners,
            semio_framework_job::OperationId(input.operation), semio_framework_job::Generation(input.generation),
            input.work.absolute_deadline_us, PersistedDocumentHydrationTarget::Envelope, ActorId("local".into()))
    });
    let mut job = None::<HistoryFoldJob<'static, RetainedHistoryFold>>;
    let mut output = None;
    let mut protocol = None;
    let mut reported_disagreement = false;
    let mut deferred = 0;
    let mut small_progress = 0;
    let turn_limit = std::env::args().find_map(|arg| arg.strip_prefix("turn-limit=")?.parse::<usize>().ok())
        .unwrap_or(if member_maintenance() { 90_000 } else { 30_000 });
    for turn in 0..turn_limit {
        if runtime_clock && turn % 500 == 0 {
            println!("[DEBUG] actor stock {turn}: {} copy bytes, {} work units, time {} us", input.grant.maximum_copy_bytes, input.work.fuel, clock().unwrap());
        }
        actor.begin_dispatch(input).unwrap();
        let mut observed = RetainedCloneProgress::default();
        let budget = StepBudget::new(turn_fuel.min(input.work.fuel), input.work.absolute_deadline_us, input.grant);
        let mut cx = actor.borrow_context(input, budget, &mut observed, clock)
            .unwrap_or_else(|error| panic!("borrow {turn}: {error}"));
        if !wire_ready {
            cx.acknowledge_return_rows_locally();
            wire_ready = cx.admit_original_core_wire_delivery().unwrap() && cx.admit_original_escrow_publisher().unwrap();
        } else if cfg!(archive_replacement) && !diagnosis {
            #[cfg(archive_runtime_cleanup)]
            {
                match &mut runtime {
                    Some(archive) => archive.attempt(&mut cx, turn),
                    None => runtime = Some(replacement::RuntimeArchive::new(&mut cx)),
                }
            }
            #[cfg(not(archive_runtime_cleanup))]
            #[cfg(archive_replacement)]
            match &mut replace {
                Some(archive) => archive.attempt(&mut cx, turn),
                None => replace = Some(replacement::Archive::new(&mut cx)),
            }
        } else if diagnosis && hydration.as_ref().is_some_and(|owner| semio_framework_value::ErasedSnapshotRetirement::terminal_is_empty(owner)) {
            #[cfg(archive_initializer)]
            {
                let owner = replay.get_or_insert_with(|| initializer::Probe::new(envelope.take().unwrap(), &cx, worker_diagnosis));
                let grant = if narrow_diagnosis {
                    let peak = if funded_candidate_turn { 32_768 } else { 24_576 };
                    RetainedCloneGrant { maximum_copy_bytes: [3_509, 8_192, 16_384, peak][turn % 4], ..policy }
                } else {
                    policy
                };
                owner.step(&mut cx, grant, turn);
            }
        } else if let Some(hydration) = &mut hydration {
            let phase = hydration.phase;
            let mut complete = false;
            let mut original_step = |cx: &mut semio_framework_job::StepContext<'_>, grant| {
                if diagnosis && format!("{:?}", hydration.phase) == "CloseEnvelopeOwners" {
                    assert!(hydration.replay_ids.as_ref().is_none_or(|ids| ids.capacity() == 0));
                    assert!(hydration.replay_order.as_ref().is_none_or(|order| order.is_empty()));
                    hydration.replay_ids.take();
                    hydration.replay_order.take();
                }
                let before = cx.retained_progress();
                let reserved = cx.reserved_return_progress();
                match hydration.step(cx, grant) {
                    PersistedDocumentHydrationStep::Rejected(diagnostic) => panic!(
                        "hydration turn {turn} {phase:?}: {diagnostic:?}, direct {:?}, context {:?}",
                        *hydration.original_failure, *hydration.original_context_failure),
                    PersistedDocumentHydrationStep::Ready(output) => {
                        assert!(semio_framework_value::ErasedSnapshotRetirement::terminal_is_empty(hydration),
                            "Ready retained replay_ids={}, replay_order={}, actor={}",
                            hydration.replay_ids.is_some(), hydration.replay_order.is_some(), hydration.actor.0.has_owner());
                        #[cfg(archive_initializer)]
                        if diagnosis {
                            let semio_os_kernel::PersistedDocumentHydrationOutput::Envelope(output) = output else { panic!("expected replay envelope") };
                            envelope = Some(output);
                        } else { std::mem::forget(output); }
                        #[cfg(not(archive_initializer))]
                        std::mem::forget(output);
                        complete = true;
                    }
                    PersistedDocumentHydrationStep::Pending(_) => {}
                }
                let mut progress = original_lifecycle_delta(before, cx.retained_progress())?;
                progress.copied_items = progress.copied_items.max(1);
                if wrap && reconcile {
                    progress = original_lifecycle_delta(
                        original_lifecycle_delta(reserved, cx.reserved_return_progress())?, progress)?;
                }
                Ok((PluginLifecycleStep::Progress(progress), false))
            };
            if wrap {
                let identity = MountedOwnerIdentityV1 { instance_id: 1, operation: cx.operation(), generation: cx.generation() };
                let mut mounted = MountedOwnerTurnV1::admit(
                    1, identity, MountedOwnerPhaseV1::Maintenance, MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut cx)
                    .unwrap().unwrap();
                mounted.advance_lifecycle(|cx, loan| {
                    let before = cx.reserved_return_progress();
                    let step = original_lifecycle_step(cx, loan, Default::default(), |cx, _| original_step(cx, loan))?;
                    let mut progress = step.progress().unwrap();
                    if reconcile {
                        progress = progress.checked_add(original_lifecycle_delta(before, cx.reserved_return_progress())?)?;
                    }
                    Ok::<_, semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure>(PluginLifecycleStep::Progress(progress))
                }).unwrap_or_else(|error| panic!("mounted hydration turn {turn}: {error:?}"));
            } else {
                original_step(&mut cx, policy).unwrap();
            }
            if complete {
                println!("actual actor hydration completed on turn {turn}; mounted: {wrap}");
                if !diagnosis { std::process::exit(0); }
            }
            if phase != hydration.phase { println!("turn {turn}: {phase:?} to {:?}", hydration.phase); }
        } else {
            let before = cx.retained_progress();
            let reserved_before = cx.reserved_return_progress();
            let mut step = |cx: &mut semio_framework_job::StepContext<'_>, _| {
                let mut scope = None;
                let mut inner = None;
                let (body, quote) = if quote_fold {
                    let result = match job.as_ref() {
                        None => {
                            let demand = FoldJob::original_birth_demand(runner.as_ref().unwrap()).unwrap();
                            fold_receiving_quote(cx, demand).map(|(grant, work)| (4096, grant, work))
                        }
                        Some(job) => fold_step_quote(cx, job).unwrap(),
                    };
                    let Some((body, grant, work)) = result else {
                        deferred += 1;
                        return Ok((PluginLifecycleStep::Progress(Default::default()), false));
                    };
                    (body, Some((grant, work)))
                } else {
                    (4096, None)
                };
                let run = |native: &mut NativeDecodeControl<'_>, _: RetainedCloneGrant| {
                    let (result, receipt) = match job.as_mut() {
                        None => {
                            let outcome = HistoryFoldJob::admit_original(&mut runner, &mut job, native);
                            (outcome.result.map(|result| result.map(|_| ())), outcome.receipt_refusal)
                        }
                        Some(job) if job.original_rejection_ready() => {
                            let outcome = job.take_original_rejection(&mut protocol, native);
                            (outcome.result.map(|result| result.map(|_| ())), outcome.receipt_refusal)
                        }
                        Some(job) if job.original_output_ready() => {
                            let outcome = job.take_original_output(&mut output, native);
                            (outcome.result.map(|result| result.map(|_| ())), outcome.receipt_refusal)
                        }
                        Some(job) => {
                            let outcome = job.step_original(native, body);
                            (outcome.result.map(|result| result.map(|_| ())), outcome.receipt_refusal)
                        }
                    };
                    inner = Some((result, receipt));
                    Ok(())
                };
                let result = match quote {
                    Some(quote) => cx.with_native_receiving_quoted(&mut scope, quote, run),
                    None => cx.with_native_receiving(&mut scope, run),
                };
                if let Some(scope) = scope {
                    if scope.source_refusal.is_some() || scope.call.is_err() {
                        let failure = OriginalTurnFailure::from(OriginalTurnPrimary::NativeSource(scope));
                        panic!("scope {turn}: {failure:?}");
                    }
                    cx.settle_receiving_scope().unwrap();
                }
                result.unwrap_or_else(|failure| panic!("receiving turn {turn}: {failure:?}"));
                if let Some((result, receipt)) = inner {
                    assert!(receipt.is_none(), "fold receipt {turn}: {receipt:?}");
                    if let Some(Err(error)) = result { panic!("fold construction {turn}: {error:?}"); }
                }
                let actual = original_lifecycle_delta(before, cx.retained_progress())?;
                let progress = if reconcile {
                    let net = semio_plugin_ownership::bounded_ownership::original_lifecycle_net_after(
                        cx.retained_progress(), reserved_before, cx.reserved_return_progress());
                    original_lifecycle_delta(before, net)?
                } else {
                    actual
                };
                Ok((PluginLifecycleStep::Progress(progress), false))
            };
            let slice_bytes = [512, 3509, 8192][turn % 3];
            let reported = if wrap {
                original_lifecycle_step(&mut cx, policy, Default::default(), step)
            } else if quote_fold {
                let loan = RetainedCloneGrant { maximum_copy_bytes: slice_bytes, ..policy };
                cx.with_retained_loan(loan, |cx| step(cx, loan).map(|(progress, _)| progress).map_err(Into::into))
                    .map_err(Into::into)
            } else {
                step(&mut cx, policy).map(|(progress, _)| progress)
            }.unwrap_or_else(|error| panic!("lifecycle turn {turn}: {error:?}")).progress().unwrap();
            let actual = original_lifecycle_delta(before, cx.retained_progress()).unwrap();
            let reserved = original_lifecycle_delta(reserved_before, cx.reserved_return_progress()).unwrap();
            if quote_fold && slice_bytes == 3509 && actual != RetainedCloneProgress::default() {
                small_progress += 1;
            }
            let reported = if wrap && reconcile { reported.checked_add(reserved).unwrap() } else { reported };
            if actual != reported && !reported_disagreement {
                println!("turn {turn} reported {reported:?} actual {actual:?} reserved {reserved:?}");
                reported_disagreement = true;
            }
            if let Some(protocol) = protocol.as_ref() { panic!("fold rejected {turn}: {protocol}"); }
            if output.is_some() {
                assert!(job.as_ref().is_some_and(|job| job.terminal_is_empty()));
                assert_eq!(reported_disagreement, wrap && !reconcile);
                assert!(!quote_fold || deferred > 0 && small_progress > 0);
                println!("actual actor fold completed on turn {turn}; receipts match: {}; deferred: {deferred}; progress under 3509 bytes: {small_progress}", !reported_disagreement);
                std::process::exit(0);
            }
        }
        drop(cx);
        input.epoch += 1;
        input.work = actor.remaining_work().unwrap();
        input.grant = actor.remaining_grant().unwrap();
    }
    panic!("actual actor did not reach the requested terminal state");
}
'''.replace("FAMILY-observed", "FAMILY-example-observed" if "picker-archive" in sys.argv[2:] else "FAMILY-observed").replace("DISPLAY", display).replace("FAMILY_DIALECT", family.upper() + "_DIALECT").replace("FAMILY", family).replace("Family", family.capitalize()))
fold_source = next(workspace.glob("🧰️framework/*products/*os/*modules/*store/*document/*history/*hydration/*fold/🦀️.rs")).read_text()
start = fold_source.index("fn fold_receiving_quote")
end = fold_source.index("\nimpl<P,M>", start)
with source.open("a") as file:
    file.write(fold_source[start:end])
if "quoted-registry" in sys.argv[2:]:
    replay_source = next(workspace.glob("🧰️framework/*products/*os/*modules/*store/*initialization/*replay/🦀️.rs"))
    replay = replay_source.read_text()
    start = replay.index("fn advance_registry_birth(")
    end = replay.index("\nuse semio_os_spr", start)
    native = next(replay_source.parent.glob("*native/🦀️.rs")).read_text()
    native_start = native.index("fn invalid(message:")
    native_end = native.index("\nimpl<P,M>", native_start)
    with source.open("a") as file:
        file.write("\nuse semio_os_store_initialization_replay::permits;\n" + replay[start:end])
        file.write("\nmod native {\nuse super::*;\nuse semio_framework_value::{RetainedCloneStep, ValueRefusalKind};\n" + native[native_start:native_end] + "\n}\n")
component = work / (stem + ".wasm")
args = ["--edition=2021", "--target=wasm32-wasip2", "-Copt-level=1", "-Cpanic=abort",
        "-Cstrip=symbols" if family == "din18599" else "-Cstrip=debuginfo", str(source), "-o", str(component)]
args += ["-Ldependency=" + str(path) for path in lane.glob("**/out") if "/build/" in str(path)]
crates = ["semio-framework-actor-retained-context", "semio-framework-actor-retained-turn",
              "semio-framework-async", "semio-framework-job", "semio-framework-value",
              "semio-os-kernel", "semio-os-spr", "semio-plugin-ownership", "semio-plugin-mounted-owner",
              f"semio-s-artifact-norm-{family}-model", f"semio-s-artifact-norm-{family}-mutations"]
if "replacement" in sys.argv[2:]:
    args += ["--cfg=archive_replacement", "-Clink-arg=-zstack-size=8388608", "-Ccodegen-units=16", "-Copt-level=0"]
    crates += ["semio-plugin", "semio-plugin-decl-registry", "semio-plugin-document-media",
               f"semio-s-artifact-norm-{family}-editor"]
if "diagnose-initializer" in sys.argv[2:]:
    args += ["--cfg=archive_initializer", "-Clink-arg=-zstack-size=8388608"]
    crates += ["semio-os-store-initialization-replay"]
if "worker-claim" in sys.argv[2:]:
    args += ["--cfg=archive_initializer_worker"]
    crates += ["semio-plugin-document-media"]
if "quoted-candidate" in sys.argv[2:]:
    args += ["--cfg=archive_candidate_quote"]
if "runtime-cleanup" in sys.argv[2:]:
    args += ["--cfg=archive_runtime_cleanup", "-Clink-arg=-zstack-size=16777216"]
    crates += ["semio-plugin-reactor-lifecycle", "semio-plugin-runtime-instances"]
if family == "din18599" and "replacement-stock" in sys.argv[2:]:
    args += ["--cfg=archive_energy_member"]
    crates += ["semio-plugin-reactor-app", "semio-s-artifact-stdio-semio", "semio-s-artifact-stdio-semio-table-model"]
if "runtime-clock" in sys.argv[2:]:
    args += ["--cfg=archive_runtime_clock"]
if "repeat-archive" in sys.argv[2:]:
    args += ["--cfg=archive_repeat"]
if "shell-note" in sys.argv[2:]:
    args += ["--cfg=archive_shell_note"]
    crates += ["semio-framework", "semio-plugin-decl-emission"]
if "browser-commands" in sys.argv[2:]:
    args += ["--cfg=archive_browser_commands"]
    crates += ["semio-framework", "semio-plugin-plugin-program", "semio-plugin-runtime-ops", "semio-plugin-instance-app"]
if "quoted-registry" in sys.argv[2:]:
    args += ["--cfg=archive_registry_quote"]
    crates += ["semio-framework-value-snapshot-read"]
for crate in crates:
    library = max((lane / "wasm32-wasip2/prototype/build" / crate).glob("*/out/*.rlib"),
                  key=lambda path: path.stat().st_mtime)
    for suffix in [".rlib", ".rmeta"]:
        args += ["--extern", crate.replace("-", "_") + "=" + str(library.with_suffix(suffix))]
flags = work / (stem + ".args")
flags.write_text("\n".join(args) + "\n")
subprocess.run([os.environ.get("RUSTC", "rustc"), "@" + str(flags)], check=True)
if "vary" in sys.argv[2:]:
    cases = [["wire", "replacement"], ["wire", "replacement", "narrow"]]
else:
    cases = [sys.argv[2:]] if len(sys.argv) > 2 else [[], ["wire"], ["wire", "lifecycle"], ["wire", "lifecycle", "reconcile"]]
errors = []
for options in cases:
    print(f"[DEBUG] original actor options {options}", flush=True)
    result = subprocess.run(["node_modules/.bin/jco", "run", "--jco-dir", work / (stem + "-jco"),
                             component, *options], cwd=workspace, check=False)
    if result.returncode:
        errors.append((options, result.returncode))
if errors:
    raise SystemExit(f"failed archive cases: {errors}")
