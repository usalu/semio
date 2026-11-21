fn archive_stock() -> dsl::original_receiving::ReceivingStock {
    use semio_framework_value::RetainedCloneGrant;
    let quantum = RetainedCloneGrant {
        maximum_items: 64, maximum_copy_bytes: 1 << 19, maximum_capacity_bytes: 1 << 20,
        maximum_release_bytes: 1 << 20, maximum_depth: 128,
    };
    let stock = RetainedCloneGrant {
        maximum_items: 1 << 20, maximum_copy_bytes: 1 << 26, maximum_capacity_bytes: 1 << 26,
        maximum_release_bytes: 1 << 26, ..quantum
    };
    dsl::original_receiving::ReceivingStock { stock, quantum, fuel: 1 << 20 }
}

fn archive_specimen() -> (serde_json::Value, Vec<u8>) {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("archive-cases.json")).unwrap();
    let payload = fixture["json"].as_str().unwrap();
    let header = [fixture["format"].as_u64().unwrap() as u8, fixture["tag"].as_u64().unwrap() as u8];
    let length: Vec<u8> = serde_json::from_value(fixture["length"].clone()).unwrap();
    let bytes = header.into_iter().chain(length).chain(payload.bytes()).collect();
    (fixture, bytes)
}

#[test]
fn original_pdf_mutation_receives_the_exact_archived_range() {
    use dsl::original_receiving::receive_to_completion;
    let (_, bytes) = archive_specimen();
    let expected = PdfMutation::decode_op(&bytes).unwrap();
    let source: Vec<u8> = [0xff, 0x00].into_iter().chain(bytes.iter().copied()).chain([0xff]).collect();
    let actual = receive_to_completion::<PdfMutation>(&source, (2, source.len() - 1), archive_stock()).unwrap();
    assert_eq!(actual, expected);
    assert_eq!(actual.encode_op().unwrap(), bytes);
    for expected in samples() {
        let bytes = expected.encode_op().unwrap();
        let actual = receive_to_completion::<PdfMutation>(&bytes, (0, bytes.len()), archive_stock()).unwrap();
        assert_eq!(actual, expected);
        assert_eq!(actual.encode_op().unwrap(), bytes);
    }
}

#[test]
fn original_pdf_mutation_refuses_bad_frames_and_payloads() {
    use dsl::original_receiving::receive_to_completion;
    use semio_framework_value::ValueRefusalKind;
    let (fixture, bytes) = archive_specimen();
    for case in fixture["refusals"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let malformed: Vec<u8> = if let Some(json) = case["json"].as_str() {
            assert!(json.len() < 128);
            [1, case["tag"].as_u64().unwrap() as u8, json.len() as u8].into_iter().chain(json.bytes()).collect()
        } else {
            let header: Vec<u8> = serde_json::from_value(case["header"].clone()).unwrap();
            header.into_iter().chain(bytes[4..].iter().copied()).collect()
        };
        assert_eq!(PdfMutation::decode_op(&malformed).is_ok(), case["coldAccepts"].as_bool().unwrap_or(false), "{name}");
        let error = receive_to_completion::<PdfMutation>(&malformed, (0, malformed.len()), archive_stock()).unwrap_err();
        let expected = match case["kind"].as_str() {
            Some("work-limit") => ValueRefusalKind::WorkLimit,
            _ => ValueRefusalKind::InvalidValue,
        };
        assert_eq!(error.refusal_kind(), Some(expected), "{name}: {error:?}");
    }
    for length in 0..4 {
        assert!(receive_to_completion::<PdfMutation>(&bytes, (0, length), archive_stock()).is_err());
    }
}

#[test]
fn original_pdf_mutation_closes_every_cancelled_prefix_under_grant() {
    use dsl::original_receiving::{OpBinaryReceiving, OpBinaryReceivingCursor};
    use semio_framework_value::{AuthoringWallet, NativeDecodeControl, RetainedCloneGrant, RetainedCloneProgress, ValueRefusalKind};
    use std::cell::Cell;
    let (fixture, bytes) = archive_specimen();
    let grants = archive_stock();
    let grant = grants.quantum;
    let body = grant.maximum_copy_bytes;
    let denied = RetainedCloneGrant { maximum_items: 0, ..grant };
    assert!(PdfMutation::admit_receiving(&bytes, (0, bytes.len()), denied).result.unwrap().is_none());
    let cuts: Vec<usize> = serde_json::from_value(fixture["cancelAfterPaidTurns"].clone()).unwrap();
    let mut observed_ready = false;
    for prefix in cuts {
        let (mut cursor, _) = PdfMutation::admit_receiving(&bytes, (0, bytes.len()), grant).result.unwrap().unwrap();
        let active = Cell::new(true);
        let mut checkpoint = |_| active.get();
        let mut native = NativeDecodeControl::new(grants.stock.maximum_capacity_bytes, &mut checkpoint);
        let mut wallet = AuthoringWallet::new(grants.stock, u128::from(grants.fuel));
        let mut total = RetainedCloneProgress::default();
        for _ in 0..prefix {
            if cursor.is_ready() { break; }
            let quote = cursor.demands(&bytes, body).unwrap();
            let turn = wallet.run(grant, 1, |effect| cursor.step_in(&bytes, &mut native, body, effect)).unwrap().unwrap().output;
            let progress = turn.result.unwrap();
            assert_eq!(turn.work, 1, "{prefix}: {quote:?}");
            assert!(progress.fits(grant));
            wallet.post(progress).unwrap();
            total = total.checked_add(progress).unwrap();
        }
        let ready = cursor.is_ready();
        observed_ready |= ready;
        if !ready {
            active.set(false);
            let turn = wallet.run(grant, 1, |effect| cursor.step_in(&bytes, &mut native, body, effect)).unwrap().unwrap().output;
            assert_eq!(turn.result.unwrap_err().refusal_kind(), Some(ValueRefusalKind::Canceled));
            assert_eq!(turn.work, 0);
        }
        let mut output = None;
        assert_eq!(cursor.take_into(&mut output, denied).work, 0);
        assert!(output.is_none());
        for _ in 0..4096 {
            if cursor.terminal_is_empty() { break; }
            let (turn, _) = wallet.run_settled(grant, 64, |effect| cursor.close_step(effect)).unwrap().unwrap();
            let progress = turn.result.unwrap().progress();
            assert!(turn.work > 0);
            wallet.post(progress).unwrap();
            total = total.checked_add(progress).unwrap();
        }
        assert!(cursor.terminal_is_empty(), "{prefix}");
        if !ready {
            assert_eq!(total.released_bytes, total.retained_capacity_bytes, "{prefix}: {total:?}");
        }
    }
    assert!(observed_ready);
}
