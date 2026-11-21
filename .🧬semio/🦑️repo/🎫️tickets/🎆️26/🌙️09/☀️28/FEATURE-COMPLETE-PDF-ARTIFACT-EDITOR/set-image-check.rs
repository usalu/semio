use mixed::{AnchoredSetImageSource, SetImageProjection, SetImageSource};
use semio_framework_dsl_record::{FieldValue, receiving::owned::{
    NativeSourceOrigin, NativeSourceReceiving, OwnedAdvanceSelection, OwnedFieldReceiving,
}};
use semio_framework_value::{
    AuthoringWallet, FromValue, NativeDecodeControl, NativeReceive, RetainedCloneGrant,
    RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome, RetainedWorkPoll, RetireOwned,
    ValueError, ValueRefusalKind,
};
use semio_framework_value_native_owned::OriginalSourceTurnIssuer;
use semio_framework_value_native_owned::native_decoding::owned::tree::NativeValueSource;

const TURN: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 64, maximum_copy_bytes: 32768, maximum_capacity_bytes: 16777216,
    maximum_release_bytes: 1048576, maximum_depth: 4096,
};
const STOCK: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 16777216, maximum_copy_bytes: 134217728, maximum_capacity_bytes: 268435456,
    maximum_release_bytes: 536870912, ..TURN
};
const DIAGNOSTIC_TURN: RetainedCloneGrant = RetainedCloneGrant {
    maximum_copy_bytes: 1 << 20, ..TURN
};
const DIAGNOSTIC_STOCK: RetainedCloneGrant = RetainedCloneGrant {
    maximum_copy_bytes: 1 << 35, ..STOCK
};
type ResultOf<T> = Result<T, ValueError>;

#[derive(Debug, PartialEq, RetireOwned, FromValue, NativeReceive)]
#[value(tag = "mutation", rename_all = "camelCase")]
enum PdfMutation { SetImage(SetImage) }

#[derive(Debug, PartialEq, RetireOwned, FromValue, NativeReceive)]
#[value(rename_all = "camelCase")]
struct SetImage {
    image: PdfImage,
    #[value(default)] index: Option<usize>,
}

#[derive(Debug, PartialEq, RetireOwned, FromValue, NativeReceive)]
#[value(rename_all = "camelCase")]
struct PdfImage {
    id: String,
    width: u32,
    height: u32,
    #[value(default)] color_space: Option<PdfColorSpace>,
    #[value(default)] bits_per_component: u32,
    #[value(default)] image_mask: bool,
    #[value(default)] decode: Vec<f64>,
    #[value(default)] interpolate: bool,
    body: PdfImageBody,
    #[value(default)] soft_mask: Option<String>,
    #[value(default)] soft_mask_in_data: Option<u32>,
    #[value(default)] mask: Option<PdfImageMask>,
    #[value(default)] matte: Option<Vec<f64>>,
    #[value(default)] intent: Option<String>,
    #[value(default)] optional_content: Option<String>,
    #[value(default)] struct_parent: Option<u32>,
    #[value(default)] extra: Vec<PdfDictEntry>,
}

#[derive(Debug, PartialEq, RetireOwned, FromValue, NativeReceive)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
enum PdfColorSpace { DeviceGray, DeviceRgb }

#[derive(Debug, PartialEq, RetireOwned, FromValue, NativeReceive)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
enum PdfImageBody { Samples { values: Vec<u32> } }

#[derive(Debug, PartialEq, RetireOwned, FromValue, NativeReceive)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
enum PdfImageMask { Stencil { image: String }, ColorKey { ranges: Vec<u32> } }

#[derive(Debug, PartialEq, RetireOwned, FromValue, NativeReceive)]
struct PdfDictEntry { key: String, value: PdfObject }

#[derive(Debug, PartialEq, RetireOwned, FromValue, NativeReceive)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
enum PdfObject { Int { value: i64 } }

struct OriginalSource;
impl NativeSourceOrigin<SetImageSource> for OriginalSource {
    const RECORD_FIELD: Option<u16> = None;
    fn check(_: &Option<FieldValue>) -> ResultOf<()> { Err(ValueError::invariant("already owned image source requires no field extraction")) }
    fn extract(_: &mut Option<FieldValue>) -> ResultOf<SetImageSource> { Err(ValueError::invariant("already owned image source requires no field extraction")) }
}

fn quote(bytes: &[u8]) -> (u8, std::ops::Range<usize>) {
    use semio_framework_replication_codec::ByteReader;
    let mut reader = ByteReader::new(bytes);
    assert_eq!(reader.read_u8().unwrap(), 1);
    let tag = reader.read_u8().unwrap();
    let size = reader.read_varint_u64().unwrap() as usize;
    assert_eq!(reader.remaining(), size);
    (tag, bytes.len() - size..bytes.len())
}

fn projected(
    bytes: &[u8], range: std::ops::Range<usize>, tag: u8,
) -> (ResultOf<AnchoredSetImageSource<'_>>, RetainedCloneProgress, u128) {
    let mut cursor = match SetImageProjection::admit(bytes, range, tag, 26, TURN) {
        Ok(Some(cursor)) => cursor,
        Ok(None) => panic!("fixture should fit SetImage cursor admission"),
        Err(error) => return (Err(error), Default::default(), 0),
    };
    assert_eq!(bytes.as_ptr(), cursor.source().as_ptr());
    let mut wallet = AuthoringWallet::new(STOCK, 1 << 30);
    let mut checkpoint = |_| true;
    let mut native = NativeDecodeControl::new(STOCK.maximum_capacity_bytes, &mut checkpoint);
    let mut result = Ok(());
    let mut received = RetainedCloneProgress::default();
    for _ in 0..800000 {
        if cursor.ready() { break; }
        let work = match cursor.demand() {
            Ok((demand, work)) => {
                assert!(demand.capacity_bytes <= TURN.maximum_capacity_bytes);
                assert!(demand.copy_bytes <= TURN.maximum_copy_bytes);
                work
            }
            Err(error) => { result = Err(error); break; }
        };
        let turn = wallet.run(TURN, work, |effect| cursor.advance(&mut native, effect)).unwrap().unwrap();
        let progress = match turn.output.result {
            Ok(progress) => progress,
            Err(error) => { result = Err(error); break; }
        };
        assert!(turn.performed > 0);
        assert_eq!(turn.actual.unwrap_or(0), progress.retained_capacity_bytes);
        received = received.checked_add(progress).unwrap();
        wallet.post(progress).unwrap();
    }
    let mut output = None;
    if result.is_ok() {
        assert!(cursor.ready());
        let taken = wallet.run(TURN, 1, |effect| cursor.take(&mut output, effect)).unwrap().unwrap();
        let progress = taken.output.unwrap();
        wallet.post(progress).unwrap();
        received = received.checked_add(progress).unwrap();
    }
    let work = (1 << 30) - wallet.remaining_work();
    while !cursor.terminal_is_empty() {
        let (turn, _) = wallet.run_settled(TURN, 4096, |effect| cursor.close(effect)).unwrap().unwrap();
        wallet.post(turn.result.unwrap().progress()).unwrap();
    }
    (result.map(|()| output.unwrap()), received, work)
}

fn bind(source: AnchoredSetImageSource<'_>) -> (ResultOf<PdfMutation>, RetainedCloneGrant, usize, usize) {
    source.with_original(|original, payload, source| {
        assert!(payload.as_ptr_range().start >= original.as_ptr_range().start);
        assert!(payload.as_ptr_range().end <= original.as_ptr_range().end);
        bind_original(source)
    })
}

fn bind_original(source: SetImageSource) -> (ResultOf<PdfMutation>, RetainedCloneGrant, usize, usize) {
    let mut actor = NativeSourceReceiving::<SetImageSource, PdfMutation, OriginalSource>::from_original(source);
    let issuer = OriginalSourceTurnIssuer::new();
    let dispatch = issuer.issue().unwrap();
    let mut bank = bank::OperationBank::new(DIAGNOSTIC_TURN, DIAGNOSTIC_STOCK, 1 << 30, dispatch.identity());
    let mut checkpoint = |_| true;
    let mut native = NativeDecodeControl::new(STOCK.maximum_capacity_bytes, &mut checkpoint);
    let mut progress = 0;
    let mut idle = 0;
    let mut outcome = Ok(());
    let mut result = None;
    let mut peak_copy = 0;
    native.with_source_witness(Some(dispatch.witness()), |native| {
        let body = 4096;
        for _ in 0..4_000_000 {
            if actor.is_ready() { break; }
            let work = actor.selected_work_demand(&None, body).unwrap();
            let selection = actor.select_advance(&None, body, work).unwrap();
            let work = if matches!(selection, OwnedAdvanceSelection::Nested) { work.max(8192) } else { work };
            let demand = actor.demands(&None, body).unwrap();
            peak_copy = peak_copy.max(demand.copy_bytes);
            assert!(demand.copy_bytes <= DIAGNOSTIC_TURN.maximum_copy_bytes);
            if demand.copy_bytes > TURN.maximum_copy_bytes && matches!(selection, OwnedAdvanceSelection::Nested) {
                let denied = bank.native(native, TURN, work + 1, |native, grant| {
                    match actor.advance_nested(&mut None, native, dispatch.witness(), body, work, grant) {
                        RetainedWorkPoll::Complete(turn) => turn,
                        RetainedWorkPoll::Pending { work } => RetainedWorkOutcome { result: Ok(Default::default()), work },
                    }
                });
                assert_eq!(denied.work, 0);
                assert_eq!(denied.result.unwrap(), RetainedCloneProgress::default());
                assert_eq!(actor.demands(&None, body).unwrap(), demand);
            }
            let turn = bank.normal(native, work, |native, grant| {
                let poll = match selection {
                    OwnedAdvanceSelection::Nested => actor.advance_nested(&mut None, native, dispatch.witness(), body, work, grant),
                    OwnedAdvanceSelection::Local { work } => native.with_effect(grant, work, |_, effect| actor.advance_local(&mut None, body, effect)).unwrap().unwrap(),
                };
                match poll {
                    RetainedWorkPoll::Complete(turn) => turn,
                    RetainedWorkPoll::Pending { work } => RetainedWorkOutcome { result: Ok(Default::default()), work },
                }
            });
            if turn.work == 0 {
                idle += 1;
                assert!(idle < 1000, "SetImage bind stalled after {progress}: selected {selection:?}, work {work}, demand {:?}, grant {:?}", actor.demands(&None, body), bank.grant());
            } else { idle = 0; }
            if let Err(primary) = turn.result {
                outcome = Err(primary.primary.or(primary.quote_refusal).or(primary.allocation_refusal)
                    .unwrap_or_else(|| ValueError::invariant("typed SetImage rejected without a cause")));
                break;
            }
            progress += 1;
        }
        if outcome.is_ok() {
            assert!(actor.is_ready());
            let mut wallet = AuthoringWallet::new(STOCK, 20);
            let taken = wallet.run(TURN, 1, |effect| actor.take_into(&mut result, effect)).unwrap().unwrap();
            let RetainedWorkPoll::Complete(turn) = taken.output else { panic!("SetImage output should transfer immediately"); };
            turn.result.unwrap();
        }
        while !actor.terminal_is_empty() {
            bank.close_native(native, |effect| actor.close_step(effect).map(RetainedCloneStep::Progress)).result.unwrap();
        }
    });
    (outcome.map(|()| result.unwrap()), bank.left, progress, peak_copy)
}

fn verify_projection(source: &dyn NativeValueSource, expected: &semio_framework_value::DslValue) {
    use semio_framework_value::DslValue;
    match expected {
        DslValue::Null => assert!(source.is_null()),
        DslValue::Bool(value) => assert_eq!(source.boolean().unwrap(), *value),
        DslValue::Number(value) => assert_eq!(source.number().unwrap(), *value),
        DslValue::String(value) => assert_eq!(source.text().unwrap(), value),
        DslValue::Array(values) => {
            assert_eq!(source.sequence_len().unwrap(), values.len());
            for (index, child) in values.iter().enumerate() {
                verify_projection(source.sequence_child(index).unwrap(), child);
            }
        }
        DslValue::Object(entries) => {
            assert_eq!(source.record_len().unwrap(), entries.len());
            for (index, (name, field)) in entries.iter().enumerate() {
                assert_eq!(source.record_name(index).unwrap(), name);
                verify_projection(source.record_child(index).unwrap(), field);
            }
        }
        DslValue::Bytes(_) => unreachable!("JSON source carries no binary intrinsics"),
    }
}

#[test]
fn editor_size_rgb_and_alpha_match_the_cold_full_shape() {
    for frame in [include_bytes!("fixture-0.bin").as_slice(), include_bytes!("fixture-1.bin").as_slice()] {
        let (tag, range) = quote(frame);
        let wire: serde_json::Value = serde_json::from_slice(&frame[range.clone()]).unwrap();
        let json = semio_framework_pack_json::parse_bytes(&frame[range.clone()], semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let expected_shape = semio_framework_pack_json::to_dsl_value(&json);
        let (source, physical, work) = projected(frame, range, tag);
        let source = source.unwrap();
        verify_projection(source.projected(), &expected_shape);
        let expected = PdfMutation::from_value(expected_shape).unwrap();
        println!("[DEBUG] editor-size source projected: {physical:?}; work {work}");
        let (actual, left, advances, peak_copy) = bind(source);
        assert_eq!(actual.unwrap(), expected);
        assert!(physical.copied_bytes < STOCK.maximum_copy_bytes);
        let PdfMutation::SetImage(expected) = expected;
        let PdfImageBody::Samples { values } = expected.image.body;
        assert_eq!(wire["image"]["id"].as_str().unwrap(), expected.image.id);
        let wire_samples = wire["image"]["body"]["values"].as_array().unwrap();
        assert_eq!(wire_samples.len(), values.len());
        for (wire, actual) in wire_samples.iter().zip(&values) {
            assert_eq!(wire.as_u64().unwrap(), u64::from(*actual));
        }
        println!("[DEBUG] {} ({} samples): projection {:?}, work {}; typed copy {}, capacity {}, items {}, advances {}, peak copy quote {}",
            expected.image.id, values.len(), physical, work,
            DIAGNOSTIC_STOCK.maximum_copy_bytes-left.maximum_copy_bytes, STOCK.maximum_capacity_bytes-left.maximum_capacity_bytes,
            STOCK.maximum_items-left.maximum_items, advances, peak_copy);
    }
}

#[test]
fn malformed_tags_duplicate_members_and_typed_fields_refuse() {
    let data = include_bytes!("fixture-0.bin");
    let (tag, range) = quote(data);
    assert!(projected(data, range.clone(), tag + 1).0.is_err());
    let text = std::str::from_utf8(&data[range]).unwrap();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../set-image-shape.json")).unwrap();
    for case in fixture["malformed"].as_array().unwrap() {
        let parts = case["replace"].as_array().unwrap();
        let body = text.replacen(parts[0].as_str().unwrap(), parts[1].as_str().unwrap(), 1);
        assert_ne!(body, text);
        let result = projected(body.as_bytes(), 0..body.len(), tag).0;
        if case["typed"].as_bool().unwrap_or(false) {
            let expected = semio_framework_pack_json::parse_bytes(body.as_bytes(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
            let expected = PdfMutation::from_value(semio_framework_pack_json::to_dsl_value(&expected)).unwrap_err();
            assert_eq!(expected.kind, ValueRefusalKind::InvalidValue, "{case}");
            let error = bind(result.unwrap()).0.unwrap_err();
            assert_eq!(error.kind, ValueRefusalKind::InvariantViolated);
            println!("[DEBUG] typed native refusal {case}: {error}");
        } else {
            let Err(error) = result else { panic!("malformed source accepted: {case}"); };
            assert_eq!(error.kind, ValueRefusalKind::InvalidValue, "{case}");
        }
    }
}

#[test]
fn closure_after_unpublished_array_allocation() {
    let frame = include_bytes!("fixture-0.bin");
    let (tag, range) = quote(frame);
    let mut cursor = SetImageProjection::admit(frame, range, tag, 26, TURN).unwrap().unwrap();
    let mut wallet = AuthoringWallet::new(STOCK, 1 << 30);
    let mut checkpoint = |_| true;
    let mut native = NativeDecodeControl::new(STOCK.maximum_capacity_bytes, &mut checkpoint);
    let mut placed = 0;
    let mut released = 0;
    for _ in 0..100000 {
        let (_, work) = cursor.demand().unwrap();
        let turn = wallet.run(TURN, work, |effect| cursor.advance(&mut native, effect)).unwrap().unwrap();
        let progress = turn.output.result.unwrap();
        wallet.post(progress).unwrap();
        placed += progress.retained_capacity_bytes;
        released += progress.released_bytes;
        if placed - released > 12_000 { break; }
    }
    assert!(placed - released > 12_000);
    assert!(!cursor.ready());
    while !cursor.terminal_is_empty() {
        let (turn, _) = wallet.run_settled(TURN, 4096, |effect| cursor.close(effect)).unwrap().unwrap();
        let progress = turn.result.unwrap().progress();
        wallet.post(progress).unwrap();
        placed += progress.retained_capacity_bytes;
        released += progress.released_bytes;
    }
    assert_eq!(placed, released);
    println!("[DEBUG] unpublished source cancellation retired {released} bytes");
}
