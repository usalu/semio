use semio_framework_dsl_record::{
    BorrowedDslRecord, BorrowedFieldSpec, BorrowedRecordSpec, BorrowedShape, FieldSpec, FieldValue,
    RecordLayout, RecordSpec, RecordValue, Shape,
};
use semio_framework_pack::snapshot::{ArtifactDsl, ArtifactPack};
use semio_s_artifact_norm_din18599_io::binary::snapshot::decode_din18599_pack;
use semio_s_artifact_norm_din18599_model::{
    DIN18599_CLIMATE_TABLE_SLOT, din18599_climate_data_from_table, genesis_din18599_child_pack,
};
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;
use semio_framework_pack_record_retained::RetainedRecordReceiving;
use semio_framework_replication_codec::PackLimits;
use semio_framework_value::RetainedCloneGrant;
use semio_framework_value_control_contract::AuthoringWallet;
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use serde_json::Value;

static FIELDS: [BorrowedFieldSpec; 1] = [BorrowedFieldSpec::new(0, "text", BorrowedShape::Text)];

fn main() {
    let input: Value = serde_json::from_str(include_str!(env!("RECORD_RECEIVING_FIXTURE"))).unwrap();
    let text = input["text"].as_str().unwrap();
    let ceiling = input["documentReceiving"]["totalAllocationCeiling"].as_str().unwrap().parse().unwrap();
    let mut record = RecordValue::default();
    record.fields.insert(0, FieldValue::Text(text.to_owned()));
    let owned_spec = RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(0, "text", Shape::Text)]);
    let borrowed_spec = BorrowedRecordSpec { keyword: None, layout: RecordLayout::Inline, fields: &FIELDS };
    let bytes = semio_framework_pack_record::encode_document(&owned_spec, &record, &Default::default()).unwrap();
    let result = read_document(&bytes, borrowed_spec, ceiling);
    assert_eq!(result.get(0), Some(&FieldValue::Text(text.into())));
    println!("[DEBUG] wasm{} verified original Pack document with policy maximum {ceiling}", usize::BITS);

    let parent = decode_din18599_pack(include_bytes!(env!("DIN18599_CAPTURED_PACK"))).unwrap();
    let child = genesis_din18599_child_pack(&parent, DIN18599_CLIMATE_TABLE_SLOT, &parent.climate_table.child_id).unwrap();
    let expected = SemioTableSnapshot::decode_pack(&child).unwrap();
    assert_eq!(din18599_climate_data_from_table(&expected), parent.climate);
    let prefix = semio_os_semio::declared_envelope_prefix_len(
        SemioTableSnapshot::envelope_id(), semio_os_semio::Component::Pack, 1,
    ).unwrap();
    let record = read_document(&child[prefix..], SemioTableSnapshot::RECORD, PackLimits::default().max_total_alloc);
    let received = SemioTableSnapshot::__dsl_from_record(&record).unwrap();
    assert_eq!(received, expected);
    println!("[DEBUG] wasm32 verified captured DIN V 18599 climate child Pack with {} rows", expected.rows.len());
}

fn read_document(bytes: &[u8], spec: BorrowedRecordSpec, ceiling: u64) -> RecordValue {
    let grant = RetainedCloneGrant {
        maximum_items: 8, maximum_copy_bytes: 65536, maximum_capacity_bytes: 65536,
        maximum_release_bytes: 65536, maximum_depth: 128,
    };
    let limits = PackLimits { max_total_alloc: ceiling, ..PackLimits::default() };
    let (mut receiving, born) = RetainedRecordReceiving::admit_document(
        bytes, (0, bytes.len()), spec, limits, grant,
    ).result.unwrap().unwrap();
    let stock = RetainedCloneGrant {
        maximum_items: 1 << 20, maximum_copy_bytes: 1 << 24, maximum_capacity_bytes: 1 << 24,
        maximum_release_bytes: 1 << 24, maximum_depth: 128,
    };
    let mut wallet = AuthoringWallet::new(stock, 1 << 24);
    wallet.post(born).unwrap();
    let mut observe = |_| true;
    let mut native = NativeDecodeControl::new(1 << 24, &mut observe);
    for _ in 0..100000 {
        if receiving.is_ready() {
            break;
        }
        let receipt = wallet.run_invoked(grant, 4096, |effect| receiving.step(bytes, &mut native, effect))
            .unwrap().unwrap().output;
        wallet.post(receipt.result.unwrap()).unwrap();
    }
    assert!(receiving.is_ready(), "document did not become ready");
    let mut result = None;
    let taken = receiving.take_into(&mut result, grant).result.unwrap();
    wallet.post(taken).unwrap();
    for _ in 0..100000 {
        if receiving.terminal_is_empty() {
            break;
        }
        let turn = wallet.run_invoked(grant, 4096, |effect| receiving.close_step(grant.maximum_copy_bytes, effect))
            .unwrap().unwrap().output;
        match turn {
            semio_framework_value::RetainedWorkPoll::Complete(outcome) => wallet.post(outcome.result.unwrap().progress()).unwrap(),
            semio_framework_value::RetainedWorkPoll::Pending { .. } => (),
        }
    }
    assert!(receiving.terminal_is_empty());
    result.unwrap()
}
