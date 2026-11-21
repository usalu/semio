use array::{JsonNumberArrayCursor, PagedJsonNumbers};
use semio_framework_dsl_record::receiving::OwnedSlot;
use semio_framework_value::{AuthoringWallet, Number, RetainedCloneGrant, RetainedCloneProgress, ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::owned::tree::NativeValueSource;

const TURN: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 64, maximum_copy_bytes: 32768, maximum_capacity_bytes: 16777216,
    maximum_release_bytes: 1048576, maximum_depth: 4096,
};
const STOCK: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 1 << 26, maximum_copy_bytes: 1 << 34, maximum_capacity_bytes: 1 << 28,
    maximum_release_bytes: 1 << 28, ..TURN
};

fn drive(
    cursor: &mut JsonNumberArrayCursor<'_>,
    wallet: &mut AuthoringWallet,
) -> Result<(RetainedCloneProgress, usize), semio_framework_value::ValueError> {
    let mut physical = RetainedCloneProgress::default();
    let mut largest = 0;
    while !cursor.is_ready() {
        let step = wallet.run(TURN, 65536, |effect| {
            let mut progress = RetainedCloneProgress::default();
            let mut largest = 0;
            while !cursor.is_ready() {
                let grant = RetainedCloneGrant {
                    maximum_items: TURN.maximum_items - progress.copied_items,
                    maximum_copy_bytes: TURN.maximum_copy_bytes - progress.copied_bytes,
                    maximum_capacity_bytes: TURN.maximum_capacity_bytes - progress.retained_capacity_bytes,
                    ..TURN
                };
                let work = effect.remaining_work_units()?;
                let turn = cursor.advance(&mut effect.reborrow(grant, work)?, || true);
                let receipt = turn.result?;
                progress = progress.checked_add(receipt)?;
                largest = largest.max(receipt.retained_capacity_bytes);
                if turn.work == 0 || effect.actual().is_some() { break; }
            }
            Ok::<_, semio_framework_value::ValueError>((progress, largest))
        })?.unwrap();
        let (progress, attempted) = step.output?;
        assert!(step.performed > 0);
        assert_eq!(step.actual.unwrap_or(0), progress.retained_capacity_bytes);
        wallet.post(progress)?;
        physical = physical.checked_add(progress)?;
        largest = largest.max(attempted);
    }
    Ok((physical, largest))
}

fn take(cursor: &mut JsonNumberArrayCursor<'_>, wallet: &mut AuthoringWallet) -> PagedJsonNumbers {
    let mut output = None;
    let turn = wallet.run(TURN, 1, |effect| cursor.take(&mut output, effect)).unwrap().unwrap();
    wallet.post(turn.output.unwrap()).unwrap();
    assert!(cursor.terminal_is_empty());
    output.unwrap()
}

fn close(cursor: &mut JsonNumberArrayCursor<'_>, wallet: &mut AuthoringWallet) -> usize {
    let mut released = 0;
    let mut born = 0;
    while !cursor.terminal_is_empty() {
        let (turn, _) = wallet.run_settled(TURN, 4096, |effect| cursor.close(effect)).unwrap().unwrap();
        let progress = turn.result.unwrap().progress();
        assert!(turn.work > 0);
        wallet.post(progress).unwrap();
        born += progress.retained_capacity_bytes;
        released += progress.released_bytes;
    }
    released - born
}

fn retire(value: PagedJsonNumbers, wallet: &mut AuthoringWallet) -> usize {
    let mut slot = OwnedSlot::<PagedJsonNumbers>::default();
    *slot.value = Some(value);
    let mut released = 0;
    let mut born = 0;
    while !slot.terminal_is_empty() {
        let (turn, _) = wallet.run_settled(TURN, 4096, |effect| slot.close_step(effect)).unwrap().unwrap();
        let progress = turn.result.unwrap();
        assert!(turn.work > 0);
        wallet.post(progress).unwrap();
        born += progress.retained_capacity_bytes;
        released += progress.released_bytes;
    }
    released - born
}

fn wallet() -> AuthoringWallet { AuthoringWallet::new(STOCK, 1 << 32) }

#[test]
fn malformed_and_numeric_kinds_match_existing_grammar() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../../paged-number-array.json")).unwrap();
    for case in cases["cases"].as_array().unwrap() {
        let bytes = case["source"].as_str().unwrap().as_bytes();
        let oracle = serde_json::from_slice::<Vec<serde_json::Number>>(bytes);
        assert_eq!(oracle.is_ok(), case["numbers"].is_array(), "{case}");
        let mut wallet = wallet();
        let mut cursor = JsonNumberArrayCursor::admit(bytes, 0..bytes.len(), bytes.len(), 32, TURN).unwrap().unwrap();
        if let (Ok((physical, _)), Some(expected)) = (drive(&mut cursor, &mut wallet), case["numbers"].as_array()) {
            let array = take(&mut cursor, &mut wallet);
            assert_eq!(array.sequence_len().unwrap(), expected.len());
            for (index, expected) in expected.iter().enumerate() {
                let number = array.sequence_child(index).unwrap().number().unwrap();
                let float = match number { Number::UInt(n) => n as f64, Number::Int(n) => n as f64, Number::Float(n) => n };
                assert_eq!(float, oracle.as_ref().unwrap()[index].as_f64().unwrap());
                let actual = match number {
                    Number::UInt(n) => format!("u:{n}"),
                    Number::Int(n) => format!("i:{n}"),
                    Number::Float(n) => format!("f:{n}"),
                };
                assert_eq!(actual, expected.as_str().unwrap());
            }
            assert!(array.sequence_child(expected.len()).is_err());
            assert_eq!(retire(array, &mut wallet), physical.retained_capacity_bytes);
        } else {
            assert!(case["numbers"].is_null(), "{case}");
            assert!(!cursor.is_ready());
            close(&mut cursor, &mut wallet);
        }
    }
}

#[test]
fn exact_original_span_and_denied_grants_survive_cancellation() {
    let bytes = b"unchanged[1, 2, 3]suffix";
    assert!(JsonNumberArrayCursor::admit(bytes, 9..25, 20, 3, TURN).is_err());
    assert!(JsonNumberArrayCursor::admit(bytes, 9..18, 8, 3, TURN).is_err());
    let mut wallet = wallet();
    let mut cursor = JsonNumberArrayCursor::admit(bytes, 9..18, 9, 3, TURN).unwrap().unwrap();
    assert_eq!(cursor.source().as_ptr(), bytes.as_ptr());
    let mut capacity = 0;
    while capacity == 0 {
        let (demand, work) = cursor.demand().unwrap();
        let denied = RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes - 1, ..TURN };
        let turn = wallet.run(denied, work, |effect| cursor.advance(effect, || panic!("denied turn cannot enter the checkpoint"))).unwrap().unwrap();
        assert_eq!((turn.performed, turn.actual, turn.output.result.unwrap()), (0, None, RetainedCloneProgress::default()));
        if demand.capacity_bytes > 0 {
            let denied = RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..TURN };
            let denied = wallet.run(denied, work, |effect| cursor.advance(effect, || panic!("denied page cannot enter the checkpoint"))).unwrap().unwrap();
            assert_eq!((denied.performed, denied.actual, denied.output.result.unwrap()), (0, None, RetainedCloneProgress::default()));
        }
        assert_eq!(cursor.demand().unwrap(), (demand, work));
        let full = wallet.run(TURN, work, |effect| cursor.advance(effect, || true)).unwrap().unwrap();
        let progress = full.output.result.unwrap();
        capacity += progress.retained_capacity_bytes;
        wallet.post(progress).unwrap();
    }
    let (_, work) = cursor.demand().unwrap();
    let turn = wallet.run(TURN, work, |effect| cursor.advance(effect, || false)).unwrap().unwrap();
    assert_eq!(turn.performed, 0);
    assert_eq!(turn.output.result.unwrap_err().kind, ValueRefusalKind::Canceled);
    assert_eq!(cursor.demand().unwrap().1, 0);
    assert_eq!(close(&mut cursor, &mut wallet), capacity);
}

#[test]
fn projects_the_original_large_image_field_without_a_contiguous_array_backing() {
    let source = include_bytes!("fixture.json");
    let span: serde_json::Value = serde_json::from_str(include_str!("span.json")).unwrap();
    let start = span["start"].as_u64().unwrap() as usize;
    let end = span["end"].as_u64().unwrap() as usize;
    let count = span["samples"].as_u64().unwrap() as usize;
    let mut cursor = JsonNumberArrayCursor::admit(source, start..end, source.len(), count, TURN).unwrap().unwrap();
    let mut wallet = wallet();
    let (physical, largest) = drive(&mut cursor, &mut wallet).unwrap();
    let work = (1u128 << 32) - wallet.remaining_work();
    let array = take(&mut cursor, &mut wallet);
    let source: &dyn NativeValueSource = &array;
    assert_eq!(source.sequence_len().unwrap(), count);
    for index in [0, 1, 511, 512, count / 2, count - 1] {
        assert_eq!(source.sequence_child(index).unwrap().number().unwrap(), Number::UInt(0));
    }
    assert!(largest <= 16 * 1024);
    assert_eq!(retire(array, &mut wallet), physical.retained_capacity_bytes);
    println!("[DEBUG] {count} numeric children; largest allocation {largest}; pages paid and released {}; source work {work}; items {}; copies {}",
        physical.retained_capacity_bytes, physical.copied_items, physical.copied_bytes);
}

#[test]
fn paged_source_drives_the_original_typed_sequence_receiver() {
    use semio_framework_dsl_record::{FieldValue, receiving::owned::{
        NativeSourceOrigin, NativeSourceReceiving, OwnedAdvanceSelection, OwnedFieldReceiving,
    }};
    use semio_framework_value::{NativeDecodeControl, RetainedWorkOutcome, RetainedWorkPoll, ValueError};
    use semio_framework_value_native_owned::OriginalSourceTurnIssuer;

    struct AlreadyDecoded;
    impl NativeSourceOrigin<PagedJsonNumbers> for AlreadyDecoded {
        const RECORD_FIELD: Option<u16> = None;
        fn check(_: &Option<FieldValue>) -> Result<(), ValueError> { Err(ValueError::invariant("test supplies an owned numeric source")) }
        fn extract(_: &mut Option<FieldValue>) -> Result<PagedJsonNumbers, ValueError> { Err(ValueError::invariant("test supplies an owned numeric source")) }
    }

    let bytes = b"[0,17,4294967295]";
    let mut wallet = wallet();
    let mut cursor = JsonNumberArrayCursor::admit(bytes, 0..bytes.len(), bytes.len(), 3, TURN).unwrap().unwrap();
    drive(&mut cursor, &mut wallet).unwrap();
    let source = take(&mut cursor, &mut wallet);
    let mut typed = NativeSourceReceiving::<PagedJsonNumbers, Vec<u32>, AlreadyDecoded>::from_original(source);
    let issuer = OriginalSourceTurnIssuer::new();
    let dispatch = issuer.issue().unwrap();
    let mut bank = bank::OperationBank::new(TURN, STOCK, 1 << 22, dispatch.identity());
    let mut checkpoint = |_| true;
    let mut native = NativeDecodeControl::new(STOCK.maximum_capacity_bytes, &mut checkpoint);
    native.with_source_witness(Some(dispatch.witness()), |native| {
        let body = TURN.maximum_copy_bytes;
        for _ in 0..4096 {
            if typed.is_ready() { break; }
            let work = typed.selected_work_demand(&None, body).unwrap();
            let selection = typed.select_advance(&None, body, work).unwrap();
            let turn = bank.normal(native, work, |native, grant| match selection {
                OwnedAdvanceSelection::Nested => match typed.advance_nested(&mut None, native, dispatch.witness(), body, work, grant) {
                    RetainedWorkPoll::Complete(turn) => turn,
                    RetainedWorkPoll::Pending { work } => RetainedWorkOutcome { result: Ok(Default::default()), work },
                },
                OwnedAdvanceSelection::Local { work } => match native.with_effect(grant, work, |_, effect| typed.advance_local(&mut None, body, effect)).unwrap().unwrap() {
                    RetainedWorkPoll::Complete(turn) => turn,
                    RetainedWorkPoll::Pending { work } => RetainedWorkOutcome { result: Ok(Default::default()), work },
                },
            });
            turn.result.unwrap();
        }
        assert!(typed.is_ready());
        let mut result = None;
        let moved = wallet.run(TURN, 1, |effect| typed.take_into(&mut result, effect)).unwrap().unwrap();
        let RetainedWorkPoll::Complete(turn) = moved.output else { panic!("numeric output should transfer immediately"); };
        turn.result.unwrap();
        assert_eq!(result, Some(vec![0, 17, u32::MAX]));
        while !typed.terminal_is_empty() {
            bank.close_native(native, |effect| typed.close_step(effect).map(semio_framework_value::RetainedCloneStep::Progress)).result.unwrap();
        }
    });
}
