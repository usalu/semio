use crate::Vdi3805Family;
use semio_s_artifact_norm_vdi3805_inferences::evaluate;
use semio_s_artifact_norm_vdi3805_model::*;

pub(super) fn documents() -> Vec<Vdi3805Snapshot> {
    vec![conforming_valve_dataset(), nonconforming_valve_dataset(), conforming_blatt_3_dataset(), nonconforming_blatt_3_dataset(), conforming_blatt_5_dataset(), conforming_blatt_6_dataset(), Vdi3805Snapshot::default()]
}

semio_s_plugin_norm_compliance::incremental_laws!(source = Vdi3805Family, full = evaluate, live = semio_s_artifact_norm_vdi3805_inferences::live::report, documents = documents());

#[test]
fn a_geometry_edit_leaves_the_head_and_tail_units_clean() {
    use semio_s_plugin_norm_compliance::incremental_evaluation::UnitSource;
    let units = <Vdi3805Family as UnitSource>::units(&documents().remove(0));
    assert_eq!(units.first().map(|u| u.id.0.as_str()), Some("global:document"));
    assert_eq!(units.len(), 2 + 2, "head, two products, tail");
}

#[test]
fn a_product_edit_re_runs_only_that_products_unit() {
    use semio_s_plugin_norm_compliance::incremental_evaluation::UnitSource;
    let units = <Vdi3805Family as UnitSource>::units(&documents().remove(0));
    let reads = |id: &str| units.iter().find(|unit| unit.id.0 == id).map(|unit| unit.reads.clone()).unwrap_or_else(|| panic!("the unit {id}"));
    let first = reads("products:VLV-50-001");
    let second = reads("products:ACT-01");
    assert!(first.covers("catalog.products.0.configuration.attributes.kvsM3S") && first.covers("curves") && first.covers("editionProfile"));
    assert!(!first.overlaps("catalog.products.1.configuration.attributes.kvsM3S"), "an edit of the second product leaves the first product's unit clean");
    assert!(!second.overlaps("catalog.products.0.configuration.attributes.kvsM3S"), "an edit of the first product leaves the second product's unit clean");
    assert!(!first.overlaps("geometry.geom-valve-50.bbox.maxX") && !first.overlaps("strictMode"), "geometry and the strict flag are not read by a product row");
}

#[test]
fn the_head_and_tail_units_read_only_what_they_depend_on() {
    use semio_s_plugin_norm_compliance::incremental_evaluation::UnitSource;
    let units = <Vdi3805Family as UnitSource>::units(&documents().remove(0));
    let reads = |id: &str| units.iter().find(|unit| unit.id.0 == id).map(|unit| unit.reads.clone()).unwrap_or_else(|| panic!("the unit {id}"));
    let (head, tail) = (reads("global:document"), reads("part:tail"));
    assert!(head.covers("catalog.products.0.records") && head.covers("geometry.geom-valve-50.bbox.maxX") && head.covers("limits.maxRecords") && head.covers("strictMode"));
    assert!(!head.overlaps("editionProfile"), "the edition profile is read by the tail and the product rows only");
    assert!(tail.covers("editionProfile.2") && tail.covers("catalog.products.0.sheet") && tail.covers("strictMode"));
    for path in ["geometry", "curves", "limits", "index", "catalog.products.0.title", "catalog.products.0.accessories", "catalog.products.0.components"] {
        assert!(!tail.overlaps(path), "the tail does not read {path}");
    }
}
