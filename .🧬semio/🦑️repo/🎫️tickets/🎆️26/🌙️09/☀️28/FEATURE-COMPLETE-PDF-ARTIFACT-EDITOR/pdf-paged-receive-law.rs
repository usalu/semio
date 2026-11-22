//! 🧪️ Exercises the staged PDF envelope and Record against the released Pack codec.
extern crate self as store;
pub use semio_os_semio as semio_format;
#[path = "pdf-paged-receive.rs"]
mod staged;

use protocol::{OriginalWireSourceIdentity, PackLimits, RetainedWireByteSource};
use semio_framework_dsl_record::{BorrowedFieldSpec, BorrowedRecordSpec, BorrowedShape, FieldSpec, FieldValue, RecordLayout, RecordSpec, RecordValue, Shape};
use semio_framework_value::{DslValue, RetainedCloneGrant, RetainedWorkPoll};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_framework_value_testing_effect_bank::{EffectBank, Wallet, invoke_original, invocation_extent};
use staged::PagedPdfPackRecord;
use semio_framework_value_native_owned::native_decoding::owned::tree::NativeValueSource;

const ENVELOPE: &str = "s.stdio.pdf.1.7";
const STOCK: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 1, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20,
    maximum_release_bytes: 1 << 20, maximum_depth: 128,
};

struct Pages {
    pages: Vec<Box<[u8]>>,
    extent: usize,
}
impl Pages {
    fn new(bytes: &[u8]) -> Self {
        Self { pages: bytes.chunks(PAGE).map(Box::from).collect(), extent: bytes.len() }
    }
}
impl RetainedWireByteSource for Pages {
    fn original_source_identity(&self) -> OriginalWireSourceIdentity {
        OriginalWireSourceIdentity { range_start: 0, address: self.pages.as_ptr() as usize, extent: self.extent, authority: None }
    }
    fn byte_at(&self, index: usize) -> Option<u8> {
        self.pages.get(index / PAGE)?.get(index % PAGE).copied()
    }
}
include!("🗑️generated/pdf-reopen-record/fields.rs");

#[test]
fn original_archive_pages_decode_and_close_with_exact_record() {
    let (pages, expected) = fixture();
    assert!(pages.pages.len() > 1);
    let alternative = fixture().0;
    let mut accept = |_| true;
    let mut native = NativeDecodeControl::new(1 << 25, &mut accept);
    let deny = RetainedCloneGrant { maximum_items: 0, ..STOCK };
    let spec = BorrowedRecordSpec { keyword: None, layout: RecordLayout::Lines, fields: &FIELDS };
    let limits = PackLimits::default();
    assert!(PagedPdfPackRecord::admit(&pages, ENVELOPE, spec, limits.clone(), &native, deny).unwrap().is_none());
    let (mut reader, _) = PagedPdfPackRecord::admit(&pages, ENVELOPE, spec, limits.clone(), &native, STOCK).unwrap().unwrap();
    assert!(reader.demands(&alternative).is_err());
    let mut other = |_| true;
    let mut foreign = NativeDecodeControl::new(1 << 25, &mut other);
    let (wide, work) = invocation_extent(STOCK, 4096).unwrap();
    let mut bank = EffectBank::new(Wallet::new(wide, 100_000_000));
    let foreign_attempt = bank.run_outcome(STOCK, 4096, |effect| reader.step(&pages, &mut foreign, effect));
    assert!(foreign_attempt.result.is_err());
    assert_eq!(foreign_attempt.work, 0);
    assert_eq!(bank.take_admitted(), None);
    let mut output = None;
    for _ in 0..300_000 {
        if reader.record_is_ready() { break; }
        let (poll, _) = bank.run(STOCK, 4096, |effect| RetainedWorkPoll::Complete(reader.step(&pages, &mut native, effect)));
        assert_ne!(bank.take_admitted(), Some(false));
        if let RetainedWorkPoll::Complete(turn) = poll { assert!(turn.result.unwrap().fits(STOCK)); }
    }
    assert!(reader.record_is_ready());
    let _ = bank.run(STOCK, 4096, |effect| {
        assert_eq!(reader.take_record_into(&mut output, effect).unwrap().copied_items, 1);
        RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome::<(), ()> { result: Ok(()), work: effect.performed_work() })
    });
    assert_eq!(bank.take_admitted(), Some(true));
    let output = output.take().unwrap();
    assert_eq!(output.record(), &expected);
    assert_eq!(output.record_name(0).unwrap(), "schema");
    assert_eq!(output.record_child(0).unwrap().text().unwrap(), "stdio.pdf.1.7");
    assert_eq!(output.record_name(29).unwrap(), "objects");
    assert!(!reader.record_is_ready());
    close(&mut reader, &mut bank, wide, work);
    let mut output_close = semio_framework_value::retirement::controlled::ControlledRetirement::retain(output);
    for _ in 0..300_000 {
        if output_close.terminal_is_empty() { break; }
        let (poll, _) = bank.run(wide, work, |effect| {
            let result = invoke_original(effect, Some((STOCK, 4096)), |effect| output_close.step(STOCK.maximum_copy_bytes, effect)).unwrap();
            let RetainedWorkPoll::Complete(turn) = result else { panic!("fixture selected a pending owner close") };
            RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome { result: turn.result, work: effect.performed_work() })
        });
        assert_eq!(bank.take_admitted(), Some(true));
        if let RetainedWorkPoll::Complete(turn) = poll { assert!(turn.result.unwrap().progress().fits(STOCK)); }
    }
    assert!(output_close.terminal_is_empty());
}

fn close(reader: &mut PagedPdfPackRecord, bank: &mut EffectBank, wide: RetainedCloneGrant, work: u64) {
    for _ in 0..300_000 {
        if reader.terminal_is_empty() { break; }
        let demand = reader.close_demands(STOCK.maximum_copy_bytes).unwrap();
        assert!(demand.depth <= STOCK.maximum_depth);
        let (poll, _) = bank.run(wide, work, |effect| {
            let result = invoke_original(effect, Some((STOCK, 4096)), |effect| reader.close_step(effect)).unwrap();
            let RetainedWorkPoll::Complete(turn) = result else { panic!("fixture selected a pending close") };
            RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome { result: turn.result, work: effect.performed_work() })
        });
        assert_ne!(bank.take_admitted(), Some(false));
        if let RetainedWorkPoll::Complete(turn) = poll { assert!(turn.result.unwrap().progress().fits(STOCK)); }
    }
    assert!(reader.terminal_is_empty());
}

#[test]
fn every_cancellation_cut_retires_without_publishing_and_bad_prefix_refuses() {
    let (pages, _) = fixture();
    let spec = BorrowedRecordSpec { keyword: None, layout: RecordLayout::Lines, fields: &FIELDS };
    let (wide, work) = invocation_extent(STOCK, 4096).unwrap();
    let mut bank = EffectBank::new(Wallet::new(wide, 100_000_000));
    for &cut in CUTS {
        let mut accept = |_| true;
        let mut native = NativeDecodeControl::new(1 << 25, &mut accept);
        let (mut reader, _) = PagedPdfPackRecord::admit(&pages, ENVELOPE, spec, PackLimits::default(), &native, STOCK).unwrap().unwrap();
        for index in 0..cut {
            let grant = if index == 0 { RetainedCloneGrant { maximum_copy_bytes: 0, ..STOCK } } else { STOCK };
            let turn = bank.run_outcome(grant, 4096, |effect| reader.step(&pages, &mut native, effect));
            assert!(turn.result.unwrap().fits(grant));
            assert_ne!(bank.take_admitted(), Some(false));
            if index == 0 { assert_eq!(turn.work, 0); }
        }
        assert!(!reader.record_is_ready());
        close(&mut reader, &mut bank, wide, work);
    }
    let mut bad = fixture().0;
    bad.pages[0][0] ^= 1;
    let mut accept = |_| true;
    let mut native = NativeDecodeControl::new(1 << 25, &mut accept);
    let (mut reader, _) = PagedPdfPackRecord::admit(&bad, ENVELOPE, spec, PackLimits::default(), &native, STOCK).unwrap().unwrap();
    let turn = bank.run_outcome(STOCK, 4096, |effect| reader.step(&bad, &mut native, effect));
    assert!(turn.result.is_err());
    assert_eq!(bank.take_admitted(), Some(true));
    close(&mut reader, &mut bank, wide, work);
}
