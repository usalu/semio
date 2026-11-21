use semio_framework_pack_json::{JsonMemberPolicy, JsonU32ArrayCursor, parse_bytes, to_dsl_value};
use semio_framework_value::{
    AuthoringWallet, DslValue, FromValue, NativeDecodeControl, RetainedCloneGrant,
    RetainedCloneProgress, ValueError, ValueRefusalKind,
};
use standards::v1_7::subsets::base::schema::snapshot::PdfImageBody;

const TURN: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 64, maximum_copy_bytes: 32768, maximum_capacity_bytes: 16777216,
    maximum_release_bytes: 1048576, maximum_depth: 4096,
};
const STOCK: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 16777216, maximum_copy_bytes: 134217728, maximum_capacity_bytes: 268435456,
    maximum_release_bytes: 536870912, ..TURN
};
const WORK: u128 = 1 << 30;

fn field<'a>(value: &'a DslValue, name: &str) -> &'a DslValue {
    let DslValue::Object(entries) = value else { panic!("expected a fixture record"); };
    &entries.iter().find(|(key, _)| key == name).expect("fixture field").1
}

fn body(value: &DslValue) -> &DslValue { field(field(value, "image"), "body") }

fn payload(frame: &[u8]) -> (u8, std::ops::Range<usize>) {
    let mut reader = semio_framework_replication_codec::ByteReader::new(frame);
    assert_eq!(reader.read_u8().unwrap(), 1);
    let tag = reader.read_u8().unwrap();
    let size = reader.read_varint_u64().unwrap() as usize;
    assert_eq!(reader.remaining(), size);
    (tag, frame.len() - size..frame.len())
}

fn projected(
    bytes: &[u8], range: std::ops::Range<usize>, tag: u8,
) -> (Result<projection::AnchoredSetImageSource<'_>, ValueError>, RetainedCloneProgress, u128, usize) {
    let mut cursor = match projection::SetImageProjection::admit(bytes, range, tag, 26, TURN) {
        Ok(Some(cursor)) => cursor,
        Ok(None) => panic!("fixture must meet projection admission"),
        Err(error) => return (Err(error), Default::default(), 0, 0),
    };
    assert!(std::ptr::eq(bytes, cursor.source()));
    let mut wallet = AuthoringWallet::new(STOCK, WORK);
    let mut checkpoint = |_| true;
    let mut native = NativeDecodeControl::new(STOCK.maximum_capacity_bytes, &mut checkpoint);
    let mut failure = None;
    let mut peak = 0;
    let mut receipt = RetainedCloneProgress::default();
    while !cursor.ready() {
        let (demand, units) = match cursor.demand() {
            Ok((demand, units)) => (demand, units),
            Err(error) => { failure = Some(error); break; }
        };
        assert!(demand.copy_bytes <= TURN.maximum_copy_bytes);
        peak = peak.max(demand.capacity_bytes);
        let turn = wallet.run(TURN, units, |effect| cursor.advance(&mut native, effect)).unwrap().unwrap();
        match turn.output.result {
            Ok(progress) => {
                assert!(turn.performed > 0 && progress.copied_items > 0);
                assert_eq!(turn.actual.unwrap_or(0), progress.retained_capacity_bytes);
                wallet.post(progress).unwrap();
                receipt = receipt.checked_add(progress).unwrap();
            }
            Err(error) => {
                let progress = error.retained_progress();
                wallet.post(progress).unwrap();
                receipt = receipt.checked_add(progress).unwrap();
                failure = Some(error);
                break;
            }
        }
    }
    let mut output = None;
    if failure.is_none() {
        let take = wallet.run(TURN, 1, |effect| cursor.take(&mut output, effect)).unwrap().unwrap().output.unwrap();
        wallet.post(take).unwrap();
        receipt = receipt.checked_add(take).unwrap();
    }
    let work = WORK - wallet.remaining_work();
    while !cursor.terminal_is_empty() {
        let (turn, _) = wallet.run_settled(TURN, 4096, |effect| cursor.close(effect)).unwrap().unwrap();
        let progress = turn.result.unwrap().progress();
        wallet.post(progress).unwrap();
        receipt = receipt.checked_add(progress).unwrap();
    }
    (failure.map_or_else(|| Ok(output.unwrap()), Err), receipt, work, peak)
}

#[test]
fn exact_production_image_body_and_unselected_fields_on_mounted_stock() {
    for frame in [include_bytes!("fixture-0.bin").as_slice(), include_bytes!("fixture-1.bin").as_slice()] {
        let (tag, range) = payload(frame);
        let wire: serde_json::Value = serde_json::from_slice(&frame[range.clone()]).unwrap();
        let mut cold = to_dsl_value(&parse_bytes(&frame[range.clone()], JsonMemberPolicy::Reject).unwrap());
        let expected = PdfImageBody::from_value(body(&cold).clone()).unwrap();
        let (source, receipt, work, peak) = projected(frame, range.clone(), tag);
        let source = source.unwrap();
        assert_eq!(source.projected().body, expected);
        source.with_original(|original, json, source| {
            assert!(std::ptr::eq(frame, original));
            assert!(std::ptr::eq(&frame[range], json));
            assert_eq!(source.body, expected);
            let PdfImageBody::Samples { values } = source.body else { panic!("typed samples expected"); };
            let oracle = wire["image"]["body"]["values"].as_array().unwrap();
            assert!(values.iter().map(|value| u64::from(*value)).eq(oracle.iter().map(|n| n.as_u64().unwrap())));
            assert_eq!(values.len(), values.capacity());
            let mut record = &mut cold;
            for member in ["image", "body", "values"] {
                let DslValue::Object(entries) = record else { panic!("fixture record"); };
                record = &mut entries.iter_mut().find(|(key, _)| key == member).unwrap().1;
            }
            *record = DslValue::Array(Vec::new());
            assert_eq!(source.metadata, cold);
            assert_eq!(values.capacity() * size_of::<u32>(), peak);
            assert!(receipt.copied_bytes < STOCK.maximum_copy_bytes);
            assert!(receipt.copied_items < STOCK.maximum_items);
            println!("[DEBUG] frame {} samples {}; aggregate receipt {receipt:?}; work {work}; peak {peak}", frame.len(), values.len());
        });
    }
}

#[test]
fn numeric_law_and_enclosing_duplicate_and_discriminator_refusals() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../set-image-u32-shape.json")).unwrap();
    for case in corpus["accepted"].as_array().unwrap() {
        let array = case["array"].as_str().unwrap();
        let json = format!(r#"{{"mutation":"setImage","image":{{"body":{{"kind":"samples","values":{array}}}}}}}"#);
        let (native, _, _, _) = projected(json.as_bytes(), 0..json.len(), 26);
        let expected = PdfImageBody::from_value(body(&to_dsl_value(&parse_bytes(json.as_bytes(), JsonMemberPolicy::Reject).unwrap())).clone()).unwrap();
        let oracle = case["samples"].as_array().unwrap();
        native.unwrap().with_original(|_, _, source| {
            assert_eq!(source.body, expected);
            let PdfImageBody::Samples { values } = source.body else { panic!("typed samples expected"); };
            assert!(values.iter().map(|n| u64::from(*n)).eq(oracle.iter().map(|n| n.as_u64().unwrap())));
        });
    }
    for array in corpus["invalid"].as_array().unwrap().iter().map(|entry| entry.as_str().unwrap()) {
        let json = format!(r#"{{"mutation":"setImage","image":{{"body":{{"kind":"samples","values":{array}}}}}}}"#);
        let Err(error) = projected(json.as_bytes(), 0..json.len(), 26).0 else { panic!("accepted invalid samples {array}"); };
        assert_eq!(error.kind, ValueRefusalKind::InvalidValue, "{array}");
        let cold = parse_bytes(json.as_bytes(), JsonMemberPolicy::Reject)
            .map_err(|error| error.kind())
            .and_then(|wire| PdfImageBody::from_value(body(&to_dsl_value(&wire)).clone()).map_err(|error| error.kind));
        assert_eq!(cold.unwrap_err(), error.kind);
    }
    let fixture = include_bytes!("fixture-0.bin");
    let (tag, range) = payload(fixture);
    assert!(projected(fixture, range.clone(), tag + 1).0.is_err());
    let template = std::str::from_utf8(&fixture[range]).unwrap();
    let laws: serde_json::Value = serde_json::from_str(include_str!("../../set-image-shape.json")).unwrap();
    for case in laws["malformed"].as_array().unwrap() {
        let pair = case["replace"].as_array().unwrap();
        if pair[0] == "\"height\":64" { continue; }
        let input = template.replacen(pair[0].as_str().unwrap(), pair[1].as_str().unwrap(), 1);
        assert_ne!(input, template);
        let Err(error) = projected(input.as_bytes(), 0..input.len(), tag).0 else { panic!("accepted malformed projection {case}"); };
        assert_eq!(error.kind, ValueRefusalKind::InvalidValue, "{case}");
    }
}

#[test]
fn exact_reservation_short_grants_and_cancel_balance() {
    let source = b" [0,1,2,3] rest";
    let mut cursor = JsonU32ArrayCursor::admit(source, 1, 16, 16, TURN).unwrap().unwrap();
    let mut wallet = AuthoringWallet::new(STOCK, WORK);
    loop {
        let (demand, work) = cursor.demand().unwrap();
        if demand.capacity_bytes > 0 {
            assert_eq!(demand.capacity_bytes, 16);
            let short = RetainedCloneGrant { maximum_capacity_bytes: 15, ..TURN };
            let denied = wallet.run(short, work, |effect| cursor.advance(effect, || true)).unwrap().unwrap();
            assert_eq!(denied.performed, 0);
            assert_eq!(denied.output.result.unwrap(), RetainedCloneProgress::default());
            assert_eq!(cursor.demand().unwrap(), (demand, work));
        }
        let turn = wallet.run(TURN, work, |effect| cursor.advance(effect, || true)).unwrap().unwrap();
        let progress = turn.output.result.unwrap();
        wallet.post(progress).unwrap();
        if let Some(16) = turn.actual { break; }
    }
    let before = cursor.demand().unwrap();
    let canceled = wallet.run(TURN, before.1, |effect| cursor.advance(effect, || false)).unwrap().unwrap();
    assert_eq!(canceled.output.result.unwrap_err().kind, ValueRefusalKind::Canceled);
    assert_eq!(canceled.performed, 0);
    assert_eq!(cursor.demand().unwrap(), before);
    let denied = RetainedCloneGrant { maximum_release_bytes: 15, ..TURN };
    let (turn, _) = wallet.run_settled(denied, 1, |effect| cursor.close(effect)).unwrap().unwrap();
    assert_eq!(turn.result.unwrap().progress(), RetainedCloneProgress::default());
    let (turn, _) = wallet.run_settled(TURN, 1, |effect| cursor.close(effect)).unwrap().unwrap();
    let release = turn.result.unwrap().progress();
    assert_eq!(release.released_bytes, 16);
    wallet.post(release).unwrap();
    assert!(cursor.terminal_is_empty());

    let mut cursor = JsonU32ArrayCursor::admit(source, 1, 16, 15, TURN).unwrap().unwrap();
    loop {
        let (demand, work) = cursor.demand().unwrap();
        assert_eq!(demand.capacity_bytes, 0);
        let turn = wallet.run(TURN, work, |effect| cursor.advance(effect, || true)).unwrap().unwrap();
        match turn.output.result {
            Ok(progress) => wallet.post(progress).unwrap(),
            Err(error) => { assert_eq!(error.kind, ValueRefusalKind::OwnershipLimit); break; }
        }
    }
    let (turn, _) = wallet.run_settled(TURN, 1, |effect| cursor.close(effect)).unwrap().unwrap();
    assert_eq!(turn.result.unwrap().progress().released_bytes, 0);
    assert!(cursor.terminal_is_empty());
}

#[test]
fn cancel_editor_sized_projection_after_reservation() {
    let source = include_bytes!("fixture-0.bin");
    let (tag, range) = payload(source);
    let mut cursor = projection::SetImageProjection::admit(source, range, tag, 26, TURN).unwrap().unwrap();
    let mut wallet = AuthoringWallet::new(STOCK, WORK);
    let permitted = std::cell::Cell::new(true);
    let mut checkpoint = |_| permitted.get();
    let mut native = NativeDecodeControl::new(STOCK.maximum_capacity_bytes, &mut checkpoint);
    let mut accounted = RetainedCloneProgress::default();
    loop {
        let (_, work) = cursor.demand().unwrap();
        let turn = wallet.run(TURN, work, |effect| cursor.advance(&mut native, effect)).unwrap().unwrap();
        let progress = turn.output.result.unwrap();
        wallet.post(progress).unwrap();
        accounted = accounted.checked_add(progress).unwrap();
        if turn.actual == Some(49_152) { break; }
    }
    permitted.set(false);
    let (demand, work) = cursor.demand().unwrap();
    let turn = wallet.run(TURN, work, |effect| cursor.advance(&mut native, effect)).unwrap().unwrap();
    assert_eq!(turn.performed, 0);
    assert_eq!(turn.output.result.unwrap_err().kind, ValueRefusalKind::Canceled);
    assert!(!cursor.ready());
    while !cursor.terminal_is_empty() {
        let (turn, _) = wallet.run_settled(TURN, 4096, |effect| cursor.close(effect)).unwrap().unwrap();
        let progress = turn.result.unwrap().progress();
        wallet.post(progress).unwrap();
        accounted = accounted.checked_add(progress).unwrap();
    }
    assert_eq!(accounted.retained_capacity_bytes, accounted.released_bytes);
    println!("[DEBUG] canceled RGB reservation {demand:?}; balanced {:?}", accounted);
}
