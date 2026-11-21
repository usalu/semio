//! 🌱️ Drives the actual Generator example through the mounted publication and result-page owners.

use semio_framework_actor::{OriginalActorTurnWork, OriginalJobBudget, RetainedTurnInput, typed_operation_page::TypedOperationResultLane};
use semio_framework_async::poll::resolve_ready;
use semio_framework_job::{OriginalRetirementPump, OriginalTurnFailure, StepBudget, StepContext};
use semio_framework_value::{DslValue, RetainedCloneGrant};
use semio_os_kernel::{ActorId, os_vcs::io::binary::entity_identity::control::{EntityIdentityAuthority, Observer}};
use semio_plugin::{ActionMeta, Effect, MountedOwnerIdentityV1, MountedOwnerPolicyV1, MountedOwnerTurnV1, PluginApp};
#[cfg(generator_production)]
use semio_plugin::{AppActionRegistry, EditorApp, VcsArtifactApp};
use semio_plugin_reactor_app::InstanceBirth;
use semio_plugin_reactor_context::OriginalActorGuestEntry;
use semio_plugin_retained_command::typed_operation::MountedIdentityWork;
#[cfg(generator_production)]
use semio_s_artifact_procedural_3d_editor::Generation3dPlayApp;
use std::{sync::OnceLock, time::Instant};

const POLICY: MountedOwnerPolicyV1 = MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
const LIMIT: usize = 16_384;
static START: OnceLock<Instant> = OnceLock::new();

fn clock() -> Option<u64> {
    u64::try_from(START.get_or_init(Instant::now).elapsed().as_micros()).ok()
}

fn budget(input: RetainedTurnInput, cancelled: bool) -> OriginalJobBudget {
    OriginalJobBudget {
        input,
        retained_policy: input.grant,
        fuel: input.work.fuel.min(2_000_000),
        deadline_us: input.work.absolute_deadline_us,
        cancelled,
    }
}

fn turn<T>(entry: &mut OriginalActorGuestEntry, input: &mut RetainedTurnInput, run: impl FnOnce(&mut StepContext<'_>) -> Result<T, OriginalTurnFailure>) -> T {
    let mut spent = Default::default();
    let call = entry.dispatch_value_step(*input, StepBudget::new(budget(*input, false).fuel, input.work.absolute_deadline_us, input.grant), &mut spent, clock, |cx| {
        cx.acknowledge_return_rows_locally();
        run(cx)
    });
    let returned = call.receipt().expect("original turn receipt");
    input.epoch += 1;
    input.grant = returned.remaining;
    input.work.fuel = returned.remaining_fuel;
    call.result.unwrap_or_else(|failure| panic!("mounted turn failed: {failure:?}; {:?}", entry.original_job_refusal().map(|fault| fault.primary.message())))
}

fn result_effect<R>(grant: RetainedCloneGrant, run: impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>) -> R) -> R {
    OriginalRetirementPump::new().turn(1 << 16, u64::MAX, clock, grant)
        .and_then(|mut turn| turn.with_effect(run))
        .expect("funded result page turn").expect("result effect issued").0
}

fn collect_receipts(app: &mut impl PluginApp) {
    if let Some((_, _, grant, receipt)) = app.take_constructor_receipt() { assert!(receipt.fits(grant)); }
    if let Some((_, grant, receipt)) = app.take_typed_operation_preparation_receipt() { assert!(receipt.fits(grant)); }
    if let Some((_, grant, receipt)) = app.take_typed_operation_retirement_receipt() { assert!(receipt.fits(grant)); }
}

#[cfg(generator_production)]
fn main() {
    let mut input = RetainedTurnInput {
        operation: 719,
        generation: 1,
        epoch: 1,
        work: OriginalActorTurnWork { clock_binding: 719, absolute_deadline_us: 30_000_000, fuel: 50_000_000 },
        grant: RetainedCloneGrant { maximum_items: 8 << 20, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 29, maximum_release_bytes: 1 << 29, maximum_depth: 64 },
    };
    let mut entry = OriginalActorGuestEntry::new();
    assert!((0..256).any(|_| {
        let call = entry.admit_plugin_step(budget(input, false), clock, |_| Ok(()));
        let returned = call.receipt().expect("original admission receipt");
        input.epoch += 1;
        input.grant = returned.remaining;
        input.work.fuel = returned.remaining_fuel;
        call.result.expect("admit original actor").ready
    }), "original actor admission bound");
    let mut app = turn(&mut entry, &mut input, |cx| {
        let mut observer = |_| true;
        let mut work = cx.native_encoder_work();
        let mut identity = EntityIdentityAuthority::new(1 << 24, &mut observer as &mut Observer<'_>)
            .expect("authoring authority").with_work_lender(Box::new(MountedIdentityWork(&mut work)));
        let definition = semio_s_artifact_procedural_generation3d::manifest::create_generation3d_app();
        let registry = AppActionRegistry::from_definition(&definition);
        let result = resolve_ready(VcsArtifactApp::<EditorApp<Generation3dPlayApp>>::with_registry_boxed(
            EditorApp::<Generation3dPlayApp>::default(), registry, ActorId("local".into()), POLICY, &mut identity,
        ));
        drop(identity);
        cx.consume_fuel(work.performed());
        Ok(result.expect("registered real Generation3d editor"))
    });
    drive(&mut *app, &mut entry, &mut input);
}

#[cfg(not(generator_production))]
fn main() {}

fn drive(app: &mut impl PluginApp, entry: &mut OriginalActorGuestEntry, input: &mut RetainedTurnInput) {
    let mut birth = InstanceBirth::default();
    let born = (0..LIMIT).position(|_| turn(entry, input, |cx| {
        let ready = app.advance_instance_birth(&mut birth, 1, "local", cx)?;
        collect_receipts(app);
        if !ready { return Ok(false); }
        birth.construction.close_step(cx)
    }));
    eprintln!("[DEBUG] Real Generator editor ready on turn {}",born.expect("mounted editor admission bound")+1);
    assert!((0..LIMIT).any(|_| turn(entry, input, |cx| {
        let mut observer = |_| true;
        let mut work = cx.native_encoder_work();
        let mut identity = EntityIdentityAuthority::new(1 << 24, &mut observer as &mut Observer<'_>)
            .expect("authoring authority").with_work_lender(Box::new(MountedIdentityWork(&mut work)));
        let result=resolve_ready(app.advance_typed_operation_publication(&mut identity,cx));
        drop(identity);
        cx.consume_fuel(work.performed());
        collect_receipts(app);
        result?;
        Ok(!app.store_constructor_is_pending())
    })), "mounted Generator store construction bound");
    let mut args = Some(DslValue::object([("exampleId".into(), DslValue::String("hexagonal-mushroom-column".into()))]));
    let mut meta = Some(ActionMeta { actor: "local".into(), instance_id: 1, view_state: None, expected_document_revision: None });
    let mut load_count=0;
    let dispatch = (0..LIMIT).position(|_| turn(entry, input, |cx| {
        let mut observer = |_| true;
        let mut work = cx.native_encoder_work();
        let mut identity = EntityIdentityAuthority::new(1 << 24, &mut observer as &mut Observer<'_>)
            .expect("authoring authority").with_work_lender(Box::new(MountedIdentityWork(&mut work)));
        let result = resolve_ready(app.handle_action("setActiveExample", &mut args, &mut meta, &mut identity, cx));
        drop(identity);
        cx.consume_fuel(work.performed());
        collect_receipts(app);
        if let Some(result)=result? {
            for effect in &result.requested_effects { assert!(!matches!(effect,Effect::LoadDocument {..}),"document load must follow Config publication"); }
            Ok(true)
        } else {
            app.maintenance_step(cx,POLICY.maintenance)?;
            Ok(false)
        }
    }));
    eprintln!("[DEBUG] setActiveExample produced its initiating response on turn {}",dispatch.expect("example dispatch bound")+1);
    let mut lanes = Vec::new();
    let mut completions = Vec::new();
    let mut operation = None;
    let settled = (0..LIMIT).position(|index| {
        turn(entry, input, |cx| {
            if !app.maintenance_terminal_is_empty() {
                let phase = app.mounted_owner_maintenance_phase()?;
                let caller = MountedOwnerIdentityV1 { instance_id: 1, operation: cx.operation(), generation: cx.generation() };
                if let Some(mut maintenance) = MountedOwnerTurnV1::admit(1, caller, phase, POLICY, cx)? {
                    let steps = app.mounted_owner_maintenance_step(&mut maintenance)?;
                    assert_eq!(steps.progress().unwrap_or_default(), maintenance.progress());
                }
            }
            let mut observer = |_| true;
            let mut work = cx.native_encoder_work();
            let mut identity = EntityIdentityAuthority::new(1 << 24, &mut observer as &mut Observer<'_>)
                .expect("authoring authority").with_work_lender(Box::new(MountedIdentityWork(&mut work)));
            let result = resolve_ready(app.advance_typed_operation_publication(&mut identity,cx));
            drop(identity);
            cx.consume_fuel(work.performed());
            collect_receipts(app);
            result?;
            Ok(())
        });
        for _ in 0..64 {
            let (page,spent) = result_effect(POLICY.preparation,|effect|app.take_typed_operation_result_page(1,effect)).expect("present result page");
            assert!(spent.fits(POLICY.preparation));
            let Some(mut page)=page else { if spent==Default::default(){break;} continue; };
            let token=page.token;
            let lane=page.lane;
            let length=page.bytes().len();
            let fault=(lane==TypedOperationResultLane::Fault).then(||String::from_utf8_lossy(page.bytes()).into_owned());
            for _ in 0..128 {
                if page.terminal_is_empty() { break; }
                let step=result_effect(POLICY.close,|effect|page.close_step(effect)).expect("close page projection");
                assert!(step.progress().fits(POLICY.close));
            }
            assert!(page.terminal_is_empty(),"receiver must release the exact projected page before ACK");
            assert!(app.acknowledge_typed_operation_result(token).expect("accept result token"));
            if let Some(fault)=fault { panic!("Generator result fault on turn {index}: {fault}"); }
            assert_eq!(*operation.get_or_insert(token.operation),token.operation,"all pages belong to one operation");
            eprintln!("[DEBUG] setActiveExample turn {index} ACK lane {lane:?}, operation {}, {} bytes",token.operation,length);
            lanes.push(lane);
        }
        while let Some(effect)=app.take_typed_operation_effect() {
            if let Effect::LoadDocument { pack, spr }=effect {
                assert!(lanes.contains(&TypedOperationResultLane::Config),"document load before Config ACK");
                assert!(!pack.is_empty()&&!spr.is_empty(),"document effect must be usable");
                load_count+=1;
                eprintln!("[DEBUG] setActiveExample turn {index} received LoadDocument, pack {} bytes, spr {} bytes",pack.len(),spr.len());
            }
        }
        while app.take_typed_operation_event().is_some() {}
        while app.take_task_resume_outcome().is_some() {}
        while app.take_typed_operation_composed_result().is_some() {}
        while app.take_typed_operation_ui_progress().is_some() {}
        while let Some(completion)=resolve_ready(app.take_typed_operation_completion()).expect("receive terminal result") {
            completions.push(completion.operation);
        }
        !app.has_pending_typed_operations() && app.maintenance_terminal_is_empty()
    });
    let settled = settled.unwrap_or_else(||panic!("Generator settlement bound: {}",app.typed_operation_diagnostic()));
    let required = [TypedOperationResultLane::Config,TypedOperationResultLane::Effect,TypedOperationResultLane::Terminal];
    let mut observed=lanes.iter();
    for lane in required { assert!(observed.any(|candidate| *candidate==lane),"missing ordered {lane:?} in {lanes:?}"); }
    assert_eq!(load_count,1,"one real document load effect");
    assert_eq!(completions.len(),1,"one terminal completion");
    assert_eq!(app.live_typed_operation_slots(),0);
    eprintln!("[DEBUG] Mounted Generator Config, LoadDocument and Terminal settled on turn {settled} with no retained typed operations");
    let closed = (0..LIMIT).any(|_| turn(entry, input, |cx| {
        cx.with_original_retirement(|cx| {
            let demand = app.close_retirement_demands(POLICY.close.maximum_copy_bytes)?;
            semio_plugin_ownership::bounded_ownership::original_lifecycle_step(cx, POLICY.close, demand, |same, grant| {
                let step = app.close_step(same, grant)?;
                Ok((step, app.close_terminal_is_empty()))
            })?;
            Ok(app.close_terminal_is_empty())
        })
    }));
    assert!(closed, "mounted application close bound");
    let closed = (0..LIMIT).any(|_| {
        let call = entry.close_plugin_step(budget(*input, true));
        let returned = call.receipt().expect("original close receipt");
        input.epoch += 1;
        input.grant = returned.remaining;
        input.work.fuel = returned.remaining_fuel;
        call.result.expect("close original actor");
        entry.terminal_is_empty()
    });
    assert!(closed, "original actor close bound");
}
