#[cfg(not(raw_picker_product_module))]
#[path = "📜️original-media-command-bridge.rs"]
mod bridge;
#[cfg(not(raw_picker_product_module))]
#[path = "📜️original-media-command-owner.rs"]
mod command_owner;
#[cfg(not(raw_picker_product_module))]
#[path = "📜️original-media-guest-reloan.rs"]
mod guest;
#[cfg(all(original_media_source_fixture, not(raw_picker_product_module)))]
#[path = "raw-picker-receipts.rs"]
mod raw_picker;
#[cfg(all(original_media_source_fixture, not(raw_picker_product_module)))]
#[path = "raw-picker-pack.rs"]
mod raw_picker_pack;
#[cfg(raw_picker_product_module)]
#[path = "🗑️generated/rm-sdk/🦀️.rs"]
pub mod original_media;
#[cfg(raw_reactor_product_module)]
extern crate self as semio_framework;
#[cfg(raw_reactor_product_module)]
extern crate self as semio_plugin_decl_media;
#[cfg(raw_reactor_product_module)]
pub mod kernel {
    pub use crate::raw_reactor_prologue_test::Event;
    #[cfg(raw_pdf_issuer_module)]
    pub use crate::raw_reactor_prologue_test::Effect;
    pub use semio_framework_replication_command_ingress::{
        CommandIngressStatus, CommandPageCursor, FixedCommandPage, OriginalMediaControlEvent, OriginalMediaWireAck,
        OriginalMediaWireControl, OriginalMediaWireLoan, OriginalMediaWirePhase, OriginalMediaWireResult,
        COMMAND_MAXIMUM_BYTES, ORIGINAL_MEDIA_COMMAND_KIND,
    };
}
#[cfg(raw_reactor_product_module)]
pub use original_media::runtime as raw_reactor_owner;
#[cfg(raw_reactor_product_module)]
#[path = "raw-reactor-test.rs"]
mod raw_reactor_test;
#[cfg(raw_reactor_product_module)]
extern crate self as semio_plugin_reactor_app;
#[cfg(raw_reactor_product_module)]
extern crate self as semio_plugin_reactor_lifecycle;
#[cfg(raw_reactor_product_module)]
extern crate self as semio_os_kernel;
#[cfg(raw_reactor_product_module)]
#[path = "raw-reactor-prologue-test.rs"]
mod raw_reactor_prologue_test;
#[cfg(raw_reactor_product_module)]
pub use raw_reactor_prologue_test::{PluginApp, instance_lifetime, life_instance_registry, os_vcs};
#[cfg(raw_pdf_issuer_module)]
#[path = "🗑️generated/rm-reactor/picker.rs"]
mod raw_publisher;
#[cfg(raw_pdf_issuer_module)]
#[path = "raw-pdf-issuer-test.rs"]
mod raw_pdf_issuer_test;
#[cfg(raw_picker_product_module)]
use original_media::{bridge, command_owner, guest, ingress, raw_picker, raw_picker_pack, receiver};
#[cfg(original_media_source_fixture)]
#[path = "🗑️generated/paged-media-receiver/command-origin.rs"]
mod command_origin;
#[cfg(original_media_source_fixture)]
#[path = "raw-picker-receipts-test.rs"]
mod raw_picker_test;

use bridge::OriginalMediaCommandBinding;
use semio_framework_replication_command_ingress::{CommandPageCursor, FixedCommandPage};

#[cfg(not(raw_picker_product_module))]
#[path = "📜️original-media-ingress.rs"]
mod ingress;
#[cfg(not(raw_picker_product_module))]
#[path = "📜️original-media-paged-receiver.rs"]
mod receiver;

use ingress::{OriginalMediaLoan, ORIGINAL_MEDIA_PAGE_BYTES};
use receiver::{OriginalMediaPagedReceiver, OriginalMediaPage};
use semio_framework_value::{AuthoringWallet, RetainedCloneGrant, RetainedCloneProgress};

const TURN: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 64, maximum_copy_bytes: 32768, maximum_capacity_bytes: 16777216,
    maximum_release_bytes: 1048576, maximum_depth: 4096,
};
const STOCK: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 1 << 22, maximum_copy_bytes: 1 << 27,
    maximum_capacity_bytes: 1 << 25, maximum_release_bytes: 1 << 25, ..TURN
};
const MAX: usize = 4 * 1024 * 1024;

type Command = command_owner::OriginalMediaCommand<[u32; 2], MAX>;

fn issued() -> (OriginalMediaCommandBinding<'static>, CommandPageCursor) {
    let binding = OriginalMediaCommandBinding { loan: loan(), owner: 43, seq: 17, kind: 91, declared: 4097 };
    let cursor = CommandPageCursor {
        owner: binding.owner, generation: binding.loan.generation, command_index: 0, command_count: 1,
        instance: binding.loan.instance_id, seq: binding.seq, kind: binding.kind, page_index: 0,
        page_count: 2, item_count: 1, metadata: binding.declared,
    };
    (binding, cursor)
}

fn ready_command(wallet: &mut AuthoringWallet, binding: &OriginalMediaCommandBinding<'_>) -> Command {
    let mut owner = None;
    for live in [
        OriginalMediaLoan { operation_id: 7, ..loan() },
        OriginalMediaLoan { generation: 8, ..loan() },
        OriginalMediaLoan { instance_id: 43, ..loan() },
        OriginalMediaLoan { document_id: "pdf/document-0008", ..loan() },
        OriginalMediaLoan { base_revision: &[19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19,
            19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 20], ..loan() },
    ] {
        let turn = wallet.run(TURN, Command::BIRTH_WORK, |effect| Command::admit(binding, live, [2, 13], MAX, &mut owner, effect)).unwrap().unwrap();
        assert!(turn.output.is_err());
        assert_eq!(turn.performed, 0);
        assert!(owner.is_none());
    }
    let demand = Command::birth_demand();
    let short = RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes - 1, ..TURN };
    for (grant, work) in [(short, Command::BIRTH_WORK), (TURN, Command::BIRTH_WORK - 1)] {
        let turn = wallet.run(grant, work, |effect| Command::admit(binding, loan(), [2, 13], MAX, &mut owner, effect)).unwrap().unwrap();
        assert_eq!(turn.output.unwrap(), RetainedCloneProgress::default());
        assert_eq!(turn.performed, 0);
        assert!(owner.is_none());
    }
    let turn = wallet.run(TURN, Command::BIRTH_WORK, |effect| Command::admit(binding, loan(), [2, 13], MAX, &mut owner, effect)).unwrap().unwrap();
    assert_eq!(turn.performed, u128::from(Command::BIRTH_WORK));
    let receipt = turn.output.unwrap();
    assert_eq!(receipt.copied_bytes, demand.copy_bytes);
    assert_eq!(receipt.retained_capacity_bytes, 0);
    wallet.post(receipt).unwrap();
    println!("[DEBUG] paid command birth: {} byte placement, {} work", demand.copy_bytes, turn.performed);
    owner.unwrap()
}

#[test]
fn original_command_keeps_one_staged_backing_across_paid_reloans() {
    let (binding, header) = issued();
    let mut wallet = AuthoringWallet::new(STOCK, 1 << 24);
    let mut owner = ready_command(&mut wallet, &binding);
    let mut output = None;
    let bytes: Vec<u8> = (0..4097).map(|index| (index % 251) as u8).collect();
    let mut total = RetainedCloneProgress::default();
    let mut copies = 0;
    for (index, body) in bytes.chunks(ORIGINAL_MEDIA_PAGE_BYTES).enumerate() {
        let header = CommandPageCursor { page_index: index as u32, ..header };
        let page = FixedCommandPage::try_copy_from(body).unwrap();
        let addr = page.as_slice().as_ptr();
        let mut incoming = Some((header.clone(), page));
        let demand = Command::stage_demand();
        let short = RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes - 1, ..TURN };
        let turn = wallet.run(short, 1, |effect| owner.stage_page(loan(), &mut incoming, effect)).unwrap().unwrap();
        assert_eq!(turn.output.unwrap(), RetainedCloneProgress::default());
        assert_eq!(incoming.as_ref().unwrap().1.as_slice().as_ptr(), addr);
        let turn = wallet.run(TURN, 1, |effect| owner.stage_page(loan(), &mut incoming, effect)).unwrap().unwrap();
        assert_eq!(turn.performed, 1);
        assert!(incoming.is_none());
        wallet.post(turn.output.unwrap()).unwrap();
        let mut duplicate = Some((header, FixedCommandPage::try_copy_from(body).unwrap()));
        let turn = wallet.run(TURN, 1, |effect| owner.stage_page(loan(), &mut duplicate, effect)).unwrap().unwrap();
        assert_eq!(turn.output.unwrap(), RetainedCloneProgress::default());
        assert_eq!(turn.performed, 0);
        assert!(duplicate.is_some());
        let partial = RetainedCloneGrant { maximum_copy_bytes: 512, ..TURN };
        let frontier = index * ORIGINAL_MEDIA_PAGE_BYTES + body.len();
        while owner.accepted_bytes() != frontier {
            let prior = owner.accepted_bytes();
            let turn = wallet.run(partial, 512, |effect| owner.copy_page(loan(), effect)).unwrap().unwrap();
            let receipt = turn.output.unwrap();
            let advanced = owner.accepted_bytes() - prior;
            assert!(advanced <= 512);
            if advanced != 0 { assert_eq!(receipt.copied_bytes, advanced); }
            assert_ne!(turn.performed, 0);
            copies += usize::from(advanced != 0);
            wallet.post(receipt).unwrap();
            total = total.checked_add(receipt).unwrap();
        }
        let turn = wallet.run(TURN, 1, |effect| owner.acknowledge_page(loan(), effect)).unwrap().unwrap();
        assert!(turn.output.unwrap().is_none());
        assert_eq!(turn.performed, 0);
        let turn = wallet.run(TURN, 1, |effect| owner.complete_into(loan(), &mut output, effect)).unwrap().unwrap();
        assert_eq!(turn.output.unwrap(), RetainedCloneProgress::default());
        while owner.page_retirement_demand(TURN.maximum_copy_bytes).unwrap().is_some() {
            let (turn, _) = wallet.run_settled(TURN, 65536,
                |effect| owner.retire_page(TURN.maximum_copy_bytes, effect).unwrap()).unwrap().unwrap();
            let step = turn.result.unwrap();
            let receipt = step.progress();
            wallet.post(receipt).unwrap();
            total = total.checked_add(receipt).unwrap();
        }
        let turn = wallet.run(TURN, 1, |effect| owner.acknowledge_page(loan(), effect)).unwrap().unwrap();
        let (accepted, receipt) = turn.output.unwrap().unwrap();
        assert_eq!(accepted, index as u32);
        wallet.post(receipt).unwrap();
    }
    let turn = wallet.run(RetainedCloneGrant { maximum_copy_bytes: 0, ..TURN }, 1,
        |effect| owner.complete_into(loan(), &mut output, effect)).unwrap().unwrap();
    assert_eq!(turn.output.unwrap(), RetainedCloneProgress::default());
    assert!(output.is_none());
    let turn = wallet.run(TURN, 1, |effect| owner.complete_into(loan(), &mut output, effect)).unwrap().unwrap();
    wallet.post(turn.output.unwrap()).unwrap();
    assert!(owner.terminal_is_empty());
    let output = output.unwrap();
    assert!(output.admits_origin(loan()));
    assert!(!output.admits_origin(OriginalMediaLoan { base_revision: &[20; 32], ..loan() }));
    assert!(!output.admits_origin(OriginalMediaLoan { document_id: "pdf/wrong", ..loan() }));
    let target = output.target;
    let mut source = output.bytes;
    assert_eq!(target, [2, 13]);
    assert!(source.original().unwrap().iter().eq(bytes));
    while !source.terminal_is_empty() {
        let (turn, _) = wallet.run_settled(TURN, 65536, |effect| source.step(TURN.maximum_copy_bytes, effect)).unwrap().unwrap();
        let receipt = turn.result.unwrap().progress();
        wallet.post(receipt).unwrap();
        total = total.checked_add(receipt).unwrap();
    }
    assert_eq!(total.released_bytes, total.retained_capacity_bytes + 2 * ORIGINAL_MEDIA_PAGE_BYTES);
    println!("[DEBUG] {} bounded copy turns, {} retained bytes plus 8192 incoming page bytes balanced", copies, total.retained_capacity_bytes);
}

#[test]
fn original_command_stale_revision_retires_page_and_partial_destination() {
    let (binding, header) = issued();
    let mut wallet = AuthoringWallet::new(STOCK, 1 << 24);
    let mut owner = ready_command(&mut wallet, &binding);
    let page = FixedCommandPage::try_copy_from(&[0xA5; ORIGINAL_MEDIA_PAGE_BYTES]).unwrap();
    let mut incoming = Some((header, page));
    let turn = wallet.run(TURN, 1, |effect| owner.stage_page(loan(), &mut incoming, effect)).unwrap().unwrap();
    wallet.post(turn.output.unwrap()).unwrap();
    let partial = RetainedCloneGrant { maximum_copy_bytes: 512, ..TURN };
    let mut total = RetainedCloneProgress::default();
    while owner.accepted_bytes() == 0 {
        let turn = wallet.run(partial, 512, |effect| owner.copy_page(loan(), effect)).unwrap().unwrap();
        let receipt = turn.output.unwrap();
        wallet.post(receipt).unwrap();
        total = total.checked_add(receipt).unwrap();
    }
    assert!(owner.cancel(OriginalMediaLoan { instance_id: 12, ..loan() }).is_err());
    let mut stale = [19; 32];
    stale[31] = 20;
    let turn = wallet.run(TURN, 512, |effect|
        owner.copy_page(OriginalMediaLoan { base_revision: &stale, ..loan() }, effect)).unwrap().unwrap();
    assert!(turn.output.unwrap_err().to_string().contains("live PDF revision"));
    assert_eq!(turn.performed, 0);
    let mut output = None;
    let turn = wallet.run(TURN, 1, |effect| owner.complete_into(loan(), &mut output, effect)).unwrap().unwrap();
    assert!(turn.output.is_err());
    assert!(output.is_none() && turn.performed == 0);
    while !owner.terminal_is_empty() {
        assert!(owner.close_demands(TURN.maximum_copy_bytes).is_ok());
        let (turn, _) = wallet.run_settled(TURN, 65536, |effect| owner.close_step(TURN.maximum_copy_bytes, effect).unwrap()).unwrap().unwrap();
        let receipt = turn.result.unwrap().progress();
        wallet.post(receipt).unwrap();
        total = total.checked_add(receipt).unwrap();
    }
    assert_eq!(total.released_bytes, total.retained_capacity_bytes + ORIGINAL_MEDIA_PAGE_BYTES);
    println!("[DEBUG] stale command canceled after 512 bytes; {} original destination capacity balanced", total.retained_capacity_bytes);
}

#[test]
#[cfg(original_media_source_fixture)]
fn actual_original_source_reloans_are_confined_to_one_picker() {
    use guest::{OriginalMediaGuest, OriginalMediaPickerIntent};
    use semio_framework_job::{Generation, InteractiveJobCloseStep, OperationId};
    use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
    type Guest = OriginalMediaGuest<[u32; 2], MAX>;
    let bytes = include_bytes!("🗑️generated/paged-media-receiver/raw-picker.png");
    let (issued, cursor) = issued();
    let binding = OriginalMediaCommandBinding { declared: bytes.len() as u32, ..issued };
    let header = CommandPageCursor { metadata: binding.declared, ..cursor };
    let clock = || Some(1);
    let mut source = OriginalJobFixtureCaller::admit(OperationId(3), Generation(1), STOCK, clock, 1_000_000, 1 << 24);
    let mut foreign = OriginalJobFixtureCaller::admit(OperationId(3), Generation(1), STOCK, clock, 1_000_000, 1 << 24);
    source.retain_dispatch().unwrap();
    foreign.retain_dispatch().unwrap();
    let mut intent = None;
    let mut owner = None;
    assert!(source.turn(TURN, |cx| OriginalMediaPickerIntent::issue(loan(), [4, 3], binding.kind, MAX, &mut intent, cx)).unwrap());
    let mut intent = intent.unwrap();
    let denied = RetainedCloneGrant { maximum_copy_bytes: 0, ..TURN };
    assert!(!source.turn(denied, |cx| Guest::begin(&mut intent, &binding, loan(), &mut owner, cx)).unwrap());
    assert!(source.turn(TURN, |cx| Guest::begin(&mut intent, &binding, loan(), &mut owner, cx)).unwrap());
    let mut repeated = None;
    assert!(source.turn(TURN, |cx| Guest::begin(&mut intent, &binding, loan(), &mut repeated, cx)).is_err());
    let mut owner = owner.unwrap();
    for (index, chunk) in bytes.chunks(ORIGINAL_MEDIA_PAGE_BYTES).enumerate() {
        let mut offered = Some((CommandPageCursor { page_index: index as u32, ..header }, FixedCommandPage::try_copy_from(chunk).unwrap()));
        assert!(foreign.turn(TURN, |cx| owner.stage_page(loan(), &mut offered, cx)).is_err());
        assert!(offered.is_some());
        assert!(source.turn(TURN, |cx| owner.stage_page(loan(), &mut offered, cx)).unwrap());
        let mut ack = None;
        for turn in 0..64 {
            let frontier = index * ORIGINAL_MEDIA_PAGE_BYTES + chunk.len();
            let copy = RetainedCloneGrant { maximum_copy_bytes: if owner.accepted_bytes() < frontier { 512 } else { TURN.maximum_copy_bytes }, ..TURN };
            ack = source.turn(copy, |cx| owner.page_progress(loan(), cx)).unwrap();
            if matches!(turn, 0 | 15 | 63) { println!("[DEBUG] Original guest page {index} turn {turn}, accepted {}, receipt {:?}", owner.accepted_bytes(), source.receipt()); }
            if ack.is_some() { break; }
        }
        assert_eq!(ack, Some(index as u32));
    }
    let mut output = None;
    assert!(foreign.turn(TURN, |cx| owner.complete(loan(), &mut output, cx)).is_err());
    assert!(source.turn(TURN, |cx| owner.complete(loan(), &mut output, cx)).unwrap());
    assert!(matches!(source.turn(TURN, |cx| owner.close(cx)).unwrap(), InteractiveJobCloseStep::Complete { .. }));
    let mut output = output.unwrap();
    source.turn(TURN, |cx| {
        let received = output.current_input(loan(), cx)?.unwrap();
        assert_eq!(received.target, [4, 3]);
        assert!(received.bytes.original().unwrap().iter().eq(bytes.iter().copied()));
        Ok(())
    }).unwrap();
    let mut revision = [19; 32];
    revision[31] = 20;
    let changed = OriginalMediaLoan { base_revision: &revision, ..loan() };
    assert!(source.turn(TURN, |cx| output.current_input(changed, cx).map(|_| ())).is_err());
    assert!(source.turn(TURN, |cx| output.current_input(loan(), cx).map(|_| ())).is_err());
    let mut closed = false;
    for _ in 0..1024 {
        if matches!(source.turn(TURN, |cx| cx.with_original_retirement(|cx| output.close(cx))).unwrap(), InteractiveJobCloseStep::Complete { .. }) {
            closed = true;
            break;
        }
    }
    assert!(closed);
    drop(output);
    source.retire(TURN);
    foreign.retire(TURN);
}

fn loan() -> OriginalMediaLoan<'static> {
    OriginalMediaLoan {
        operation_id: 61714, generation: 7, instance_id: 42, document_id: "pdf/document-0009",
        base_revision: &[19; 32],
    }
}

fn append(receiver: &mut OriginalMediaPagedReceiver<MAX>, page: &mut OriginalMediaPage<'_>, wallet: &mut AuthoringWallet) -> RetainedCloneProgress {
    let mut total = RetainedCloneProgress::default();
    while !page.is_complete() {
        let turn = wallet.run(TURN, 16384, |effect| receiver.append(page, effect)).unwrap().unwrap();
        let progress = turn.output.unwrap();
        assert!(turn.performed > 0);
        assert_eq!(turn.actual.unwrap_or(0), progress.retained_capacity_bytes);
        assert!(progress.copied_bytes <= ORIGINAL_MEDIA_PAGE_BYTES);
        assert!(progress.retained_capacity_bytes <= 8192);
        wallet.post(progress).unwrap();
        total = total.checked_add(progress).unwrap();
    }
    total
}

#[test]
fn owns_and_donates_a_complete_image_source() {
    let mut state = 0x3167ab52u32;
    let original: Vec<u8> = (0..1_572_881).map(|_| {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        state as u8
    }).collect();
    let mut receiver = OriginalMediaPagedReceiver::<MAX>::new(loan(), original.len(), MAX).unwrap();
    let mut wallet = AuthoringWallet::new(STOCK, 1 << 26);
    let mut held = 0;
    for (index, bytes) in original.chunks(ORIGINAL_MEDIA_PAGE_BYTES).enumerate() {
        let mut page = OriginalMediaPage::new(loan(), index as u32, bytes);
        held += append(&mut receiver, &mut page, &mut wallet).retained_capacity_bytes;
    }
    assert!(held > original.len());
    let wrong = OriginalMediaLoan { base_revision: &[7; 32], ..loan() };
    let turn = wallet.run(TURN, 10, |effect| receiver.take(wrong, effect)).unwrap().unwrap();
    assert!(turn.output.is_err() && turn.performed == 0);
    let turn = wallet.run(RetainedCloneGrant { maximum_copy_bytes: 0, ..TURN }, 10, |effect| receiver.take(loan(), effect)).unwrap().unwrap();
    assert!(turn.output.unwrap().is_none() && turn.performed == 0);
    let turn = wallet.run(TURN, 1, |effect| receiver.take(loan(), effect)).unwrap().unwrap();
    let (mut source, receipt) = turn.output.unwrap().unwrap();
    wallet.post(receipt).unwrap();
    assert!(receiver.terminal_is_empty());
    assert!(source.original().unwrap().iter().eq(original));
    let mut released = 0;
    let mut retirement_capacity = 0;
    while !source.terminal_is_empty() {
        let (turn, _) = wallet.run_settled(TURN, 65536, |effect| source.step(TURN.maximum_copy_bytes, effect)).unwrap().unwrap();
        let progress = turn.result.unwrap().progress();
        wallet.post(progress).unwrap();
        released += progress.released_bytes;
        retirement_capacity += progress.retained_capacity_bytes;
    }
    assert_eq!(released, held + retirement_capacity);
    println!("[DEBUG] Retained and retired {} input bytes across 385 issued source pages", released - retirement_capacity);
}

#[test]
fn refuses_bad_pages_and_retires_only_paid_capacity_after_cancel() {
    let bytes = [41; ORIGINAL_MEDIA_PAGE_BYTES];
    let mut receiver = OriginalMediaPagedReceiver::<MAX>::new(loan(), 8193, MAX).unwrap();
    let mut wallet = AuthoringWallet::new(STOCK, 1 << 24);
    for mut invalid in [
        OriginalMediaPage::new(OriginalMediaLoan { instance_id: 12, ..loan() }, 0, &bytes),
        OriginalMediaPage::new(loan(), 1, &bytes),
        OriginalMediaPage::new(loan(), 0, &bytes[..4095]),
    ] {
        let turn = wallet.run(TURN, 16384, |effect| receiver.append(&mut invalid, effect)).unwrap().unwrap();
        assert!(turn.output.is_err());
        assert_eq!(turn.performed, 0);
    }
    let mut first = OriginalMediaPage::new(loan(), 0, &bytes);
    let turn = wallet.run(RetainedCloneGrant { maximum_capacity_bytes: 0, ..TURN }, 16384,
        |effect| receiver.append(&mut first, effect)).unwrap().unwrap();
    assert_eq!(turn.output.unwrap(), RetainedCloneProgress::default());
    assert_eq!(turn.performed, 0);
    let held = append(&mut receiver, &mut first, &mut wallet).retained_capacity_bytes;
    let duplicate = OriginalMediaPage::new(loan(), 0, &bytes);
    assert!(receiver.demand(&duplicate).is_err());
    receiver.cancel();
    let mut following = OriginalMediaPage::new(loan(), 1, &bytes);
    let turn = wallet.run(TURN, 16384, |effect| receiver.append(&mut following, effect)).unwrap().unwrap();
    assert!(turn.output.is_err() && turn.performed == 0);
    let mut released = 0;
    let mut retirement_capacity = 0;
    while !receiver.terminal_is_empty() {
        assert!(receiver.close_demand(TURN.maximum_copy_bytes).is_ok());
        let (turn, _) = wallet.run_settled(TURN, 65536, |effect| receiver.close_step(TURN.maximum_copy_bytes, effect).unwrap()).unwrap().unwrap();
        let progress = turn.result.unwrap().progress();
        wallet.post(progress).unwrap();
        released += progress.released_bytes;
        retirement_capacity += progress.retained_capacity_bytes;
    }
    assert_eq!(released, held + retirement_capacity);
}

#[test]
fn binds_real_command_pages_and_refuses_changed_envelope_fields() {
    let bytes = [0x6d; 4097];
    let binding = OriginalMediaCommandBinding { loan: loan(), owner: 43, seq: 17, kind: 91, declared: 4097 };
    let header = CommandPageCursor {
        owner: binding.owner, generation: binding.loan.generation, command_index: 0, command_count: 1,
        instance: binding.loan.instance_id, seq: binding.seq, kind: binding.kind, page_index: 0,
        page_count: 2, item_count: 1, metadata: binding.declared,
    };
    let first = FixedCommandPage::try_copy_from(&bytes[..4096]).unwrap();
    let final_byte = FixedCommandPage::try_copy_from(&bytes[4096..]).unwrap();
    for changed in [
        CommandPageCursor { owner: 99, ..header },
        CommandPageCursor { generation: 0, ..header },
        CommandPageCursor { command_index: 1, ..header },
        CommandPageCursor { command_count: 2, ..header },
        CommandPageCursor { instance: 25, ..header },
        CommandPageCursor { seq: 0, ..header },
        CommandPageCursor { kind: 0, ..header },
        CommandPageCursor { page_index: 2, ..header },
        CommandPageCursor { page_count: 1, ..header },
        CommandPageCursor { item_count: 0, ..header },
        CommandPageCursor { metadata: 4098, ..header },
    ] {
        assert!(binding.body(&changed, &first).is_err());
    }
    assert!(binding.body(&header, &final_byte).is_err());
    assert!(binding.body(&CommandPageCursor { page_index: 1, ..header }, &first).is_err());
    let mut receiver = OriginalMediaPagedReceiver::<MAX>::new(loan(), bytes.len(), MAX).unwrap();
    let mut wallet = AuthoringWallet::new(STOCK, 1 << 24);
    let mut first = binding.body(&header, &first).unwrap();
    let partial = RetainedCloneGrant { maximum_copy_bytes: 512, ..TURN };
    while !first.is_complete() {
        let turn = wallet.run(partial, 512, |effect| receiver.append(&mut first, effect)).unwrap().unwrap();
        assert_ne!(turn.performed, 0);
        wallet.post(turn.output.unwrap()).unwrap();
    }
    let mut last = binding.body(&CommandPageCursor { page_index: 1, ..header }, &final_byte).unwrap();
    append(&mut receiver, &mut last, &mut wallet);
    let turn = wallet.run(TURN, 1, |effect| receiver.take(loan(), effect)).unwrap().unwrap();
    let (mut source, receipt) = turn.output.unwrap().unwrap();
    wallet.post(receipt).unwrap();
    assert!(source.original().unwrap().iter().eq(bytes));
    while !source.terminal_is_empty() {
        let (turn, _) = wallet.run_settled(TURN, 65536, |effect| source.step(TURN.maximum_copy_bytes, effect)).unwrap().unwrap();
        wallet.post(turn.result.unwrap().progress()).unwrap();
    }
}
