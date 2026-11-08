use semio_os_kernel::{DiffBinary, DiffText};
use semio_s_artifact_norm_vdi3805_diff::{Vdi3805Diff, Vdi3805ProductsPatch, Vdi3805ProductsRows};
use semio_s_artifact_norm_vdi3805_model::{conforming_valve_dataset, sync_typed_attributes_into_records, SheetAttributes};

fn synced_diff() -> Vdi3805Diff {
    let base = conforming_valve_dataset();
    let mut next = base.catalog.products[0].clone();
    if let SheetAttributes::ValveHeating(a) = &mut next.configuration.attributes {
        a.dn = 80;
    }
    sync_typed_attributes_into_records(&mut next);
    Vdi3805Diff {
        products: Some(Vdi3805ProductsRows::modification(&next.id, Vdi3805ProductsPatch { title: None, configuration: Some(next.configuration.clone()), records: Some(next.records.clone()) })),
        ..Default::default()
    }
}

#[test]
fn a_patch_with_native_records_survives_the_binary_diff_codec() {
    let diff = synced_diff();
    let bytes = diff.encode_diff().expect("encodes");
    assert_eq!(Vdi3805Diff::decode_diff(&bytes).expect("decodes"), diff);
}

#[test]
fn a_patch_with_native_records_survives_the_text_diff_codec() {
    let diff = synced_diff();
    let text = diff.print_diff();
    assert!(text.contains("210"), "the rewritten native record is part of the text: {text}");
    assert_eq!(Vdi3805Diff::parse_diff(&text).expect("parses"), diff);
}

#[test]
fn the_records_of_a_patch_are_not_dropped_by_either_codec() {
    let diff = synced_diff();
    let Some(rows) = &diff.products else { panic!("products") };
    let again = Vdi3805Diff::decode_diff(&diff.encode_diff().expect("encodes")).expect("decodes");
    assert_eq!(format!("{:?}", again.products.as_ref().expect("products")), format!("{rows:?}"));
}
