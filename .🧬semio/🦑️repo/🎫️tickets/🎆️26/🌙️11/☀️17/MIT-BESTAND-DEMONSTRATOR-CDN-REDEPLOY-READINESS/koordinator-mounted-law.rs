//! 📬️ Drives one real, untagged CAD contribution through the mounted Demonstrator app.

use semio_framework::manifest::{CommandAddress, CommandOwnerAddress};
use semio_framework_actor::{OriginalActorTurnWork, OriginalJobBudget, RetainedTurnInput};
use semio_framework_async::poll::resolve_ready;
use semio_framework_job::{OriginalTurnFailure, StepBudget, StepContext};
use semio_framework_value::{DslValue, RetainedCloneGrant};
use semio_os_kernel::{ActorId, os_vcs::io::binary::entity_identity::control::{EntityIdentityAuthority, Observer}};
use semio_plugin::{ActionMeta, MountedOwnerPolicyV1, PluginApp};
use semio_plugin_reactor_app::InstanceBirth;
use semio_plugin_reactor_context::OriginalActorGuestEntry;
use semio_plugin_retained_command::typed_operation::MountedIdentityWork;
use std::{sync::OnceLock, time::Instant};

const POLICY: MountedOwnerPolicyV1 = MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
const LIMIT: usize = 16_384;
const APP_ID: &str = "s.cad.cad@1/*#editor";
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

fn main() {
    let mut input = RetainedTurnInput {
        operation: 719,
        generation: 1,
        epoch: 1,
        work: OriginalActorTurnWork { clock_binding: 719, absolute_deadline_us: 30_000_000, fuel: 50_000_000 },
        grant: RetainedCloneGrant { maximum_items: 8 << 20, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 29, maximum_release_bytes: 1 << 29, maximum_depth: 64 },
    };
    let mut entry = OriginalActorGuestEntry::new();
    let mut plugin = None;
    let admitted = (0..256).any(|_| {
        let call = entry.admit_plugin_step(budget(input, false), clock, |_| {
            plugin = Some(semio_hub_demonstrator::manifest::plugin().expect("real Demonstrator registration"));
            Ok(())
        });
        let returned = call.receipt().expect("original admission receipt");
        input.epoch += 1;
        input.grant = returned.remaining;
        input.work.fuel = returned.remaining_fuel;
        call.result.expect("admit original actor").ready
    });
    assert!(admitted, "original actor admission bound");
    let mut plugin = plugin.expect("admitted Demonstrator manifest");
    let mut app = turn(&mut entry, &mut input, |cx| {
        let mut observer = |_| true;
        let mut work = cx.native_encoder_work();
        let mut identity = EntityIdentityAuthority::new(1 << 24, &mut observer as &mut Observer<'_>)
            .expect("authoring authority").with_work_lender(Box::new(MountedIdentityWork(&mut work)));
        let app = plugin.create_app(APP_ID, ActorId("local".into()), POLICY, &mut identity)?;
        drop(identity);
        cx.consume_fuel(work.performed());
        Ok(app.expect("registered CAD editor"))
    });
    let mut birth = InstanceBirth::default();
    let born = (0..LIMIT).any(|_| turn(&mut entry, &mut input, |cx| {
        if !app.advance_instance_birth(&mut birth, 1, "local", cx)? { return Ok(false); }
        birth.construction.close_step(cx)
    }));
    assert!(born, "mounted editor admission bound");
    let stores_ready = (0..LIMIT).any(|_| turn(&mut entry, &mut input, |cx| {
        let mut observer = |_| true;
        let mut work = cx.native_encoder_work();
        let mut identity = EntityIdentityAuthority::new(1 << 24, &mut observer as &mut Observer<'_>)
            .expect("authoring authority").with_work_lender(Box::new(MountedIdentityWork(&mut work)));
        resolve_ready(app.advance_typed_operation_publication(&mut identity, cx))?;
        drop(identity);
        cx.consume_fuel(work.performed());
        if let Some((_, _, grant, receipt)) = app.take_constructor_receipt() {
            assert!(receipt.fits(grant), "mounted store construction receipt");
        }
        Ok(!app.store_constructor_is_pending())
    }));
    assert!(stores_ready, "mounted store construction bound");
    let mut args = Some(DslValue::object([
        ("json".into(), DslValue::String("[]".into())),
        ("page".into(), DslValue::Number(0_u64.into())),
        ("pageCount".into(), DslValue::Number(1_u64.into())),
    ]));
    let mut meta = Some(ActionMeta { actor: "local".into(), instance_id: 1, view_state: None, expected_document_revision: None });
    let address = CommandAddress {
        owner: CommandOwnerAddress::App { plugin_id: "demonstrator".into(), app_id: APP_ID.into() },
        command_id: "setContributions".into(),
    };
    let mut original_fault = None;
    let completed = (0..LIMIT).position(|_| turn(&mut entry, &mut input, |cx| {
        let mut observer = |_| true;
        let mut work = cx.native_encoder_work();
        let mut identity = EntityIdentityAuthority::new(1 << 24, &mut observer as &mut Observer<'_>)
            .expect("authoring authority").with_work_lender(Box::new(MountedIdentityWork(&mut work)));
        let result = resolve_ready(app.handle_command(&address, &mut args, &mut original_fault, None, &mut meta, &mut identity, cx))?;
        drop(identity);
        cx.consume_fuel(work.performed());
        Ok(result.is_some())
    }));
    let completed = completed.expect("untagged mounted command must settle within its turn bound") + 1;
    assert!(args.is_none() && meta.is_none() && original_fault.is_none(), "the real command takes its original input owners");
    println!("[DEBUG] Mounted CAD setContributions settled in {completed} turns");
    let closed = (0..LIMIT).any(|_| turn(&mut entry, &mut input, |cx| {
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
    let closed = (0..LIMIT).any(|_| turn(&mut entry, &mut input, |cx| plugin.close_original_definitions(cx)));
    assert!(closed && plugin.original_definitions_terminal_is_empty(), "registered definitions close bound");
    let closed = (0..LIMIT).any(|_| {
        let call = entry.close_plugin_step(budget(input, true));
        let returned = call.receipt().expect("original close receipt");
        input.epoch += 1;
        input.grant = returned.remaining;
        input.work.fuel = returned.remaining_fuel;
        call.result.expect("close original actor");
        entry.terminal_is_empty()
    });
    assert!(closed, "original actor close bound");
}
