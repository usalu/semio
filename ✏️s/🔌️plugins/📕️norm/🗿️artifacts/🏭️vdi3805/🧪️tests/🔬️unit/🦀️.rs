use super::*;

#[semio_framework_async_macros::async_test]
async fn building_system_number_parse_render() {
    let bsn = BuildingSystemNumber::parse("420.10.1").expect("parse");
    assert_eq!(bsn.render(), "420.10.1");
}

#[semio_framework_async_macros::async_test]
async fn building_system_number_parse_rejects_wrong_part_count() {
    let err = BuildingSystemNumber::parse("420.10").unwrap_err();
    assert!(matches!(err, NormError::InvalidValue { field, .. } if field == "building_system_number"));
}

#[semio_framework_async_macros::async_test]
async fn building_system_number_parse_rejects_non_numeric_sequence() {
    let err = BuildingSystemNumber::parse("420.10.abc").unwrap_err();
    assert!(matches!(err, NormError::InvalidValue { field, .. } if field == "building_system_number.sequence"));
}

#[semio_framework_async_macros::async_test]
async fn security_limits_validate_text_rejects_oversized_input() {
    let limits = SecurityLimits { max_file_bytes: 8, ..SecurityLimits::default() };
    let err = limits.validate_text("this text is way longer than eight bytes").unwrap_err();
    assert!(matches!(err, NormError::InvalidValue { field, .. } if field == "file"));
}

#[semio_framework_async_macros::async_test]
async fn security_limits_validate_text_accepts_within_bound() {
    let limits = SecurityLimits::default();
    assert!(limits.validate_text("short").is_ok());
}

#[semio_framework_async_macros::async_test]
async fn characteristic_curve_interpolates() {
    let doc = Vdi3805Snapshot::default();
    let curve = doc.curves.get("curve.kvs").expect("curve");
    let y = curve.interpolate(50.0);
    assert!((y - 2.25).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn characteristic_curve_interpolate_handles_edges() {
    let empty = CharacteristicCurve { id: "empty".into(), x_unit: VdiUnit::delta("%", VdiQuantityKind::Dimensionless, 0.01), y_unit: VdiUnit::absolute("m3/h", VdiQuantityKind::Volume, 1.0), points: Vec::new() };
    assert_eq!(empty.interpolate(10.0), 0.0);

    let doc = Vdi3805Snapshot::default();
    let curve = doc.curves.get("curve.kvs").expect("curve");
    assert_eq!(curve.interpolate(-10.0), curve.points[0].y);
    assert_eq!(curve.interpolate(1000.0), curve.points[curve.points.len() - 1].y);
}

#[semio_framework_async_macros::async_test]
async fn bounding_box_overlaps_detects_intersection_and_gap() {
    let a = BoundingBox::from_size(1.0, 1.0, 1.0);
    let b = BoundingBox { min_x: 0.5, min_y: 0.5, min_z: 0.5, max_x: 1.5, max_y: 1.5, max_z: 1.5 };
    assert!(a.overlaps(b, 0.0));
    let c = BoundingBox { min_x: 5.0, min_y: 5.0, min_z: 5.0, max_x: 6.0, max_y: 6.0, max_z: 6.0 };
    assert!(!a.overlaps(c, 0.0));
}

#[semio_framework_async_macros::async_test]
async fn geometry_bbox_volume() {
    let doc = Vdi3805Snapshot::default();
    let geom = doc.geometry.get("geom.valve.50").expect("geom");
    let bbox = geom.evaluate_bbox();
    assert!((bbox.volume_m3() - 0.003).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn catalog_index_filters_by_dn() {
    let doc = Vdi3805Snapshot::default();
    let matches = doc.index.filter_by_dn(50);
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].product_id, "VLV-50-001");
}

#[semio_framework_async_macros::async_test]
async fn catalog_index_filter_by_sheet_and_tag() {
    let doc = Vdi3805Snapshot::default();
    let by_sheet = doc.index.filter_by_sheet(SheetId(2));
    assert_eq!(by_sheet.len(), 1);
    let by_tag = doc.index.filter_by_tag("control valve");
    assert_eq!(by_tag.len(), 1);
    assert!(doc.index.filter_by_tag("nonexistent-tag").is_empty());
}

#[semio_framework_async_macros::async_test]
async fn correction_overlay_applicability() {
    let registry = SchemaCatalog::current();
    let corrections = registry.corrections_for_sheet(SheetId(2));
    let corr = corrections.first().expect("part 2 correction");
    assert!(corr.id.starts_with("part-02-corr-"));
    assert!(corr.applies_as_of(EditionId::new(2024, 1)));
    assert!(!corr.applies_as_of(EditionId::new(2010, 1)));
}

#[semio_framework_async_macros::async_test]
async fn schema_registry_with_status_and_sheet_lookup() {
    let registry = SchemaCatalog::with_status(SchemaStatus::Reserved);
    assert!(registry.sheets().iter().all(|s| s.status == SchemaStatus::Reserved));
    let full = SchemaCatalog::current();
    let sheet = full.sheet(SheetId(2)).expect("sheet 2");
    assert_eq!(sheet.title_en, "Control valves heating");
    assert!(full.sheet(SheetId(9999)).is_none());
}

#[semio_framework_async_macros::async_test]
async fn schema_registry_sheets_in_domain_and_reserved_numbers() {
    let registry = SchemaCatalog::current();
    let heating = registry.sheets_in_domain(Domain::Heating);
    assert!(heating.iter().any(|s| s.id == SheetId(2)));
    let reserved = registry.reserved_numbers();
    assert!(reserved.contains(&15));
    assert!(!reserved.contains(&2));
}

#[semio_framework_async_macros::async_test]
async fn sheet_id_part_str_and_edition_id_key() {
    assert_eq!(SheetId(42).part_str(), "42");
    assert!(EditionId::new(2023, 3).key() > EditionId::new(2022, 6).key());
}

#[semio_framework_async_macros::async_test]
async fn schema_status_is_operative() {
    assert!(SchemaStatus::Published.is_operative());
    assert!(SchemaStatus::Checked.is_operative());
    assert!(!SchemaStatus::Draft.is_operative());
    assert!(!SchemaStatus::Reserved.is_operative());
}

#[semio_framework_async_macros::async_test]
async fn record_family_id_all_known_contains_expected() {
    let known = RecordFamilyId::all_known();
    assert!(known.contains(&RecordFamilyId::R010));
    assert!(known.contains(&RecordFamilyId::R970_41));
}

#[semio_framework_async_macros::async_test]
async fn manufacturer_catalog_product_for_sheet() {
    let doc = Vdi3805Snapshot::default();
    assert!(doc.catalog.product_for_sheet(SheetId(2)).is_some());
    assert!(doc.catalog.product_for_sheet(SheetId(3)).is_none());
}
