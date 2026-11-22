#[cfg(not(history_only))]
use semio_plugin::artifact_app_laws as laws;
#[cfg(not(history_only))]
use semio_plugin::{EditorApp, MountedOwnerPolicyV1, PluginApp};
#[cfg(not(history_only))]
use semio_s_artifact_gis_gisterrain_editor::editor::gis3d::{Gis3dPlayApp, create_gis3d_app};
use std::future::Future;

#[cfg(history_only)]
use semio_framework_value::{RetainedCloneGrant, ValueError};
#[cfg(history_only)]
use semio_framework_value_native_owned::native_decoding::{
    OriginalSourceTurnIdentity, OriginalWorkAuthority, OriginalWorkPartialCheck, OriginalWorkPartialCheckComplete,
    OriginalWorkQuote, OriginalWorkQuoteComplete, OriginalWorkRecipient,
};
#[cfg(history_only)]
use semio_framework_value_native_owned::OriginalSourceTurnIssuer;

#[cfg(history_only)]
struct Work;
#[cfg(history_only)]
impl OriginalWorkRecipient for Work {
    type Context<'turn, 'source> = (OriginalSourceTurnIdentity, u64) where 'source: 'turn;
    fn validate_context<'turn, 'source>(&self, _: &Self::Context<'turn, 'source>) -> Result<(), ValueError> where 'source: 'turn { Ok(()) }
    fn source_turn<'turn, 'source>(&self, context: &Self::Context<'turn, 'source>) -> Option<OriginalSourceTurnIdentity> where 'source: 'turn { Some(context.0) }
    fn admits<'reply, 'turn, 'source>(&mut self, context: &mut Self::Context<'turn, 'source>, quote: OriginalWorkQuote<'reply>, observed: u64, additional: u64) -> OriginalWorkQuoteComplete<'reply> where 'source: 'turn {
        quote.complete(Ok(observed.checked_add(additional).is_some_and(|work| work <= context.1)))
    }
    fn settle_partial<'reply, 'turn, 'source>(&mut self, _: &mut Self::Context<'turn, 'source>, check: OriginalWorkPartialCheck<'reply>) -> OriginalWorkPartialCheckComplete<'reply> where 'source: 'turn { check.complete(Ok(())) }
    fn recover_partial<'reply, 'turn, 'source>(&mut self, _: &mut Self::Context<'turn, 'source>, check: OriginalWorkPartialCheck<'reply>) -> OriginalWorkPartialCheckComplete<'reply> where 'source: 'turn { check.complete(Ok(())) }
}

fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let mut context = std::task::Context::from_waker(std::task::Waker::noop());
    loop {
        match future.as_mut().poll(&mut context) {
            std::task::Poll::Ready(output) => return output,
            std::task::Poll::Pending => std::thread::yield_now(),
        }
    }
}

#[cfg(history_only)]
async fn witness(bytes: &[u8]) {
    use semio_os_spr::{DOCUMENT_ARCHIVE_MAXIMUM_BYTES, DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS, RetainedHistoryDecode, RetainedHistorySourceStep, RetainedHistoryUninterrupted};
    let archive = semio_os_spr::decode_document_archive_bytes(bytes).await.unwrap();
    let history = semio_os_spr::decode_history(&archive.parent_spr, &semio_os_spr::DecodeOptions::default()).await.unwrap();
    eprintln!("[DEBUG] archive bytes={} pack={} spr={} members={} document={} schema={} edits={} transitions={} conflicts={}",
        bytes.len(), archive.parent_pack.len(), archive.parent_spr.len(), archive.members.len(),
        history.doc_id, history.schema, history.edits.len(), history.transitions.len(), history.conflicts.len());
    let limits = protocol::codec::HistoryReadLimits {
        file_bytes: DOCUMENT_ARCHIVE_MAXIMUM_BYTES as u32, frame_body_bytes: DOCUMENT_ARCHIVE_MAXIMUM_BYTES as u32,
        records: (DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS * 8) as u32, maximum_fields: 128,
        maximum_text_bytes: 1_048_576, maximum_binary_bytes: DOCUMENT_ARCHIVE_MAXIMUM_BYTES as u32,
        maximum_json_bytes: 1_048_576, maximum_json_allocation_bytes: 8_388_608, maximum_json_items: 65_536,
        maximum_json_depth: 64,
    };
    for (copy, fuel) in [(32_768, 4096), (32_768, 65_536), (32_768, 1_048_576), (65_536, 1_048_576)] {
        let grant = RetainedCloneGrant {
            maximum_items: 64, maximum_copy_bytes: copy, maximum_capacity_bytes: 16_777_216,
            maximum_release_bytes: 1_048_576, maximum_depth: 4096,
        };
        let mut reader = RetainedHistoryDecode::new((0, archive.parent_spr.len()), limits, true, 8_388_608).unwrap();
        let mut ownership = OriginalWorkAuthority::new(Work);
        let issuer = OriginalSourceTurnIssuer::new();
        let mut output = None;
        let mut copied_items = 0;
        let mut blocked = 0;
        for turn in 1usize..=4096 {
            let demand = reader.demands(copy).unwrap();
            let token = issuer.issue().unwrap();
            let outcome = ownership.with_loan(&mut (token.identity(), fuel), |loan| reader.step(
                &archive.parent_spr, &RetainedHistoryUninterrupted, grant, &mut output, loan,
            )).unwrap();
            let step = outcome.result.unwrap();
            let progress = reader.take_receipt().map(|(_, progress)| progress).unwrap_or_default();
            if step != RetainedHistorySourceStep::Ready {
                copied_items += progress.copied_items;
            }
            blocked += usize::from(step == RetainedHistorySourceStep::Blocked);
            if turn.is_power_of_two() || (475..=490).contains(&turn) || matches!(step, RetainedHistorySourceStep::Ready | RetainedHistorySourceStep::Fault) {
                eprintln!("[DEBUG] copy={copy} fuel={fuel} turn={turn} {step:?} items={copied_items} blocked={blocked} consumed={} work={} demand={demand:?} progress={progress:?} transferred={} empty={} fault={:?}", reader.consumed_end(), outcome.work, output.is_some(), reader.terminal_is_empty(), reader.fault());
            }
            if matches!(step, RetainedHistorySourceStep::Ready | RetainedHistorySourceStep::Fault) {
                break;
            }
        }
        if let Some(retained) = output {
            assert_eq!(retained, history);
            assert!(reader.terminal_is_empty());
        } else {
            std::mem::forget(reader);
        }
    }
    let grant = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 16_777_216, maximum_release_bytes: 1_048_576, maximum_depth: 4096 };
    let stock = RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 29, maximum_release_bytes: 1 << 29, maximum_depth: 4096 };
    let mut caller = semio_framework_job_testing_caller::OriginalJobFixtureCaller::admit(
        semio_framework_job::OperationId(111), semio_framework_job::Generation(1), stock, || Some(0), u64::MAX, 1 << 20,
    );
    let mut reader = RetainedHistoryDecode::new((0, archive.parent_spr.len()), limits, true, 8_388_608).unwrap();
    let mut ownership = OriginalWorkAuthority::new(Work);
    let mut output = None;
    let mut copied_items = 0;
    for turn in 1usize..=4096 {
        let result = caller.bare_turn(grant, |cx| {
            use semio_framework_job_native_decode::{OriginalNativeDecodeIssued, OriginalNativeDecodeSource};
            let granted = cx.retained_grant();
            let fuel = cx.fuel_remaining();
            let source = cx.original_source_turn_identity().unwrap();
            let outcome = ownership.with_loan(&mut (source, fuel), |loan| reader.step(
                &archive.parent_spr, &RetainedHistoryUninterrupted, granted, &mut output, loan,
            )).unwrap();
            let work = outcome.work as u64;
            let observed = reader.last_turn_native_work() as u64;
            let progress = reader.take_receipt().map(|(_, progress)| progress).unwrap_or_default();
            let result = OriginalNativeDecodeSource::receive_observed_work(cx, granted, fuel, OriginalNativeDecodeIssued {
                value: outcome.result, grant: granted, progress, work,
            }, observed);
            if let Some(refusal) = result.receiving_refusal {
                return Err(refusal);
            }
            let step = result.value.unwrap();
            Ok((step, progress, work, observed))
        });
        let (step, progress, work, observed) = result.unwrap();
        if step != RetainedHistorySourceStep::Ready {
            copied_items += progress.copied_items;
        }
        if turn.is_power_of_two() || (475..=490).contains(&turn) || matches!(step, RetainedHistorySourceStep::Ready | RetainedHistorySourceStep::Fault) {
            eprintln!("[DEBUG] original receiver turn={turn} {step:?} items={copied_items} work={work} observed={observed} progress={progress:?} transferred={} empty={}", output.is_some(), reader.terminal_is_empty());
        }
        if let Some(expected) = match turn {
            481 => Some((RetainedHistorySourceStep::Progress, 1, 0, false)),
            482 => Some((RetainedHistorySourceStep::Progress, 0, 0, false)),
            483 => Some((RetainedHistorySourceStep::Progress, 0, 336, false)),
            484 => Some((RetainedHistorySourceStep::Ready, 1, 0, true)),
            _ => None,
        } {
            assert_eq!((step, progress.copied_items, progress.released_bytes, reader.terminal_is_empty()), expected);
            assert_eq!((copied_items, work, observed), (481, 2, 2));
            assert!(output.is_some());
        }
        if matches!(step, RetainedHistorySourceStep::Ready | RetainedHistorySourceStep::Fault) {
            break;
        }
    }
    assert!(reader.terminal_is_empty());
    assert_eq!(copied_items, 481);
    assert_eq!(output, Some(history));
    caller.retire(grant);
}

#[cfg(not(history_only))]
async fn witness(bytes: &[u8]) {
    let archive = semio_os_spr::decode_document_archive_bytes(bytes).await.unwrap();
    eprintln!("[DEBUG] archive bytes={} pack={} spr={} members={}",
        bytes.len(), archive.parent_pack.len(), archive.parent_spr.len(), archive.members.len());
    let mut scope = laws::OriginalFixtureScope::admit();
    let policy = MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
    let mut identity = laws::fixture_identity();
    let mut app = laws::new_app_with_registry_boxed::<EditorApp<Gis3dPlayApp>>(
        || semio_plugin::App { definition: create_gis3d_app(), examples: Vec::new() },
        semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), policy, &mut identity,
    ).await;
    app.bind_instance_id(laws::meta("local").instance_id).await;
    let operation = laws::LAW_DOCUMENT_LOAD_OPERATION;
    let cx = scope.cx();
    eprintln!("[DEBUG] constructed: {}", app.close_diagnostic());
    laws::admit_document_archive_load(&mut *app, operation, archive, cx).unwrap();
    laws::ensure_fixture_cancellation_issuer(&mut *app, cx).unwrap();
    let (grant, _) = cx.original_turn_ceiling();
    let mut previous = None;
    for poll in 1usize..=4096 {
        laws::fixture_renew_turn(cx, laws::OriginalFixtureCaller::turn_fuel(), grant).unwrap();
        cx.consume_fuel(1);
        let maintenance = app.maintenance_step(cx, cx.retained_grant()).unwrap();
        let status = app.poll_document_archive_load(operation, &mut identity, cx).await.unwrap();
        let record = (status.completed, status.total);
        if previous != Some(record) || poll.is_power_of_two() {
            eprintln!("[DEBUG] poll={poll} {status:?} maintenance={maintenance:?} owner={}",
                app.close_diagnostic());
        }
        previous = Some(record);
        if !matches!(status.state, semio_os_spr::DocumentArchiveLoadState::Pending | semio_os_spr::DocumentArchiveLoadState::Running) {
            eprintln!("[DEBUG] terminal poll={poll} {status:?}");
            app.acknowledge_document_archive_load(operation).unwrap();
            if !status.fault.is_empty() {
                eprintln!("[DEBUG] terminal fault={}", String::from_utf8_lossy(&status.fault));
            }
            let _ = laws::close_registered_fixture_app(&mut *app, policy, cx);
            return;
        }
    }
    eprintln!("[DEBUG] poll bound exhausted: {}", app.close_diagnostic());
    std::mem::forget(app);
    std::process::exit(2);
}

fn main() {
    let bytes = std::fs::read(std::env::args_os().nth(1).unwrap()).unwrap();
    std::thread::Builder::new().name("terrain-archive-witness".into()).stack_size(64 * 1024 * 1024)
        .spawn(move || block_on(witness(&bytes))).unwrap().join().unwrap();
}
