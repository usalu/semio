use super::*;
use protocol::{Mutation, MutationDiff, SemanticMutation};

fn round_trip(base: &Vdi3805Snapshot, operation: &Vdi3805Mutation) -> Vdi3805Snapshot {
    let forward = operation.diff(base).diff().apply(base).expect("valid mutation diff");
    let backwards = operation.inverse(base);
    let mut restored = forward.clone();
    for back in &backwards {
        restored = back.diff(base).diff().apply(&restored).expect("valid mutation diff");
    }
    assert_eq!(&restored, base, "inverse must exactly restore the pre-operation fixture");
    forward
}

#[semio_framework_async_macros::async_test]
async fn update_manufacturer_file_round_trips() {
    let base = Vdi3805Snapshot::default();
    let mut new_file = base.manufacturer_file.clone();
    new_file.manufacturer = "ACME".into();
    let mutation = Vdi3805Mutation::UpdateManufacturerFile(update_manufacturer_file::UpdateManufacturerFile { new_manufacturer_file: new_file.clone() });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.manufacturer_file.manufacturer, "ACME");
}

#[semio_framework_async_macros::async_test]
async fn change_correction_as_of_and_strict_mode_round_trip() {
    let base = Vdi3805Snapshot::default();
    let correction = Vdi3805Mutation::ChangeCorrectionAsOf(change_correction_as_of::ChangeCorrectionAsOf { new_correction_as_of: crate::EditionId::new(2025, 3) });
    let after = round_trip(&base, &correction);
    assert_eq!(after.correction_as_of, crate::EditionId::new(2025, 3));

    let strict = Vdi3805Mutation::ChangeStrictMode(change_strict_mode::ChangeStrictMode { new_strict_mode: true });
    let after = round_trip(&base, &strict);
    assert!(after.strict_mode);
}

#[semio_framework_async_macros::async_test]
async fn update_limits_round_trips() {
    let base = Vdi3805Snapshot::default();
    let new_limits = crate::SecurityLimits { max_file_bytes: 1, max_records: 2, max_field_length: 3, max_nesting_depth: 4 };
    let mutation = Vdi3805Mutation::UpdateLimits(update_limits::UpdateLimits { new_limits });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.limits, new_limits);
}

#[semio_framework_async_macros::async_test]
async fn change_and_remove_edition_profile_round_trip() {
    let base = Vdi3805Snapshot::default();
    let change = Vdi3805Mutation::ChangeEditionProfile(change_edition_profile::ChangeEditionProfile { sheet: "8".into(), new_choice: crate::EditionProfileChoice::Legacy });
    let after_change = round_trip(&base, &change);
    assert_eq!(after_change.edition_profile.get("8"), Some(&crate::EditionProfileChoice::Legacy));

    let remove = Vdi3805Mutation::RemoveEditionProfile(remove_edition_profile::RemoveEditionProfile { sheet: "8".into() });
    let after_remove = round_trip(&after_change, &remove);
    assert!(!after_remove.edition_profile.contains_key("8"));
}

#[semio_framework_async_macros::async_test]
async fn change_edition_profile_undo_of_a_fresh_sheet_is_remove() {
    let base = Vdi3805Snapshot::default();
    let change = Vdi3805Mutation::ChangeEditionProfile(change_edition_profile::ChangeEditionProfile { sheet: "fresh".into(), new_choice: crate::EditionProfileChoice::Current });
    let undo = change.inverse(&base);
    assert_eq!(undo, vec![Vdi3805Mutation::RemoveEditionProfile(remove_edition_profile::RemoveEditionProfile { sheet: "fresh".into() })]);
}

#[semio_framework_async_macros::async_test]
async fn create_rename_replace_configuration_delete_product_round_trip() {
    let base = Vdi3805Snapshot::default();
    let product = CatalogueProduct {
        identity: crate::ProductIdentity { manufacturer_code: "DEMO".into(), product_group: "HV".into(), article_number: "VLV-NEW".into() },
        title: crate::bilingual("Neu", "New"),
        sheet: crate::SheetId(3),
        records: Vec::new(),
        configuration: crate::Configuration { id: "cfg.new".into(), parameters: BTreeMap::new(), geometry_ref: None, function_refs: Vec::new() },
        accessories: Vec::new(),
        components: Vec::new(),
        extensions: crate::ExtensionBag::default(),
    };
    let create = Vdi3805Mutation::CreateProduct(create_product::CreateProduct { product: product.clone(), index: None });
    let after_create = round_trip(&base, &create);
    assert!(after_create.catalog.products.iter().any(|p| p.identity.article_number == "VLV-NEW"));
    assert!(after_create.index.entries.iter().any(|e| e.product_id == "VLV-NEW"));

    let undo = create.inverse(&base);
    assert_eq!(undo, vec![Vdi3805Mutation::DeleteProduct(delete_product::DeleteProduct { id: "VLV-NEW".into() })]);

    let rename = Vdi3805Mutation::RenameProduct(rename_product::RenameProduct { id: "VLV-NEW".into(), new_title: crate::bilingual("Umbenannt", "Renamed") });
    let after_rename = round_trip(&after_create, &rename);
    assert_eq!(crate::text_in(&after_rename.catalog.products.iter().find(|p| p.identity.article_number == "VLV-NEW").unwrap().title, "en"), "Renamed");
    assert_eq!(after_rename.index.entries.iter().find(|e| e.product_id == "VLV-NEW").unwrap().tags, vec!["Umbenannt".to_string(), "Renamed".to_string()]);

    let mut new_parameters = BTreeMap::new();
    new_parameters.insert("dn".into(), VdiValue::Integer { value: 80 });
    let replace = Vdi3805Mutation::ReplaceProductConfiguration(replace_product_configuration::ReplaceProductConfiguration {
        id: "VLV-NEW".into(),
        new_configuration: crate::Configuration { id: "cfg.new".into(), parameters: new_parameters, geometry_ref: None, function_refs: Vec::new() },
    });
    let after_replace = round_trip(&after_create, &replace);
    assert_eq!(after_replace.index.entries.iter().find(|e| e.product_id == "VLV-NEW").unwrap().dn, Some(80));

    let delete = Vdi3805Mutation::DeleteProduct(delete_product::DeleteProduct { id: "VLV-50-001".into() });
    let after_delete = round_trip(&base, &delete);
    assert!(!after_delete.catalog.products.iter().any(|p| p.identity.article_number == "VLV-50-001"));
    assert!(!after_delete.index.entries.iter().any(|e| e.product_id == "VLV-50-001"));
}

#[semio_framework_async_macros::async_test]
async fn delete_product_of_a_missing_id_has_an_empty_inverse() {
    let base = Vdi3805Snapshot::default();
    let delete = Vdi3805Mutation::DeleteProduct(delete_product::DeleteProduct { id: "nope".into() });
    assert!(delete.inverse(&base).is_empty(), "deleting an absent id has nothing to undo");
}

#[semio_framework_async_macros::async_test]
async fn geometry_lifecycle_round_trips() {
    let base = Vdi3805Snapshot::default();
    let geometry = crate::ParametricGeometry { id: "geom.new".into(), bbox: crate::BoundingBox::from_size(1.0, 1.0, 1.0), connections: Vec::new(), parameters: BTreeMap::new() };
    let create = Vdi3805Mutation::CreateGeometry(create_geometry::CreateGeometry { geometry: geometry.clone() });
    let after_create = round_trip(&base, &create);
    assert!(after_create.geometry.contains_key("geom.new"));

    let resize = Vdi3805Mutation::ResizeGeometry(resize_geometry::ResizeGeometry { id: "geom.new".into(), new_bbox: crate::BoundingBox::from_size(2.0, 2.0, 2.0) });
    let after_resize = round_trip(&after_create, &resize);
    assert_eq!(after_resize.geometry.get("geom.new").unwrap().bbox.max_x, 2.0);

    let connection = crate::ConnectionPoint { id: "c1".into(), medium: "water".into(), position: [0.0, 0.0, 0.0], direction: [1.0, 0.0, 0.0], diameter_mm: None };
    let add_conn = Vdi3805Mutation::AddGeometryConnection(add_geometry_connection::AddGeometryConnection { id: "geom.new".into(), connection: connection.clone() });
    let after_add = round_trip(&after_create, &add_conn);
    assert_eq!(after_add.geometry.get("geom.new").unwrap().connections.len(), 1);

    let remove_conn = Vdi3805Mutation::RemoveGeometryConnection(remove_geometry_connection::RemoveGeometryConnection { id: "geom.new".into(), connection_id: "c1".into() });
    let after_remove = round_trip(&after_add, &remove_conn);
    assert!(after_remove.geometry.get("geom.new").unwrap().connections.is_empty());

    let mut params = BTreeMap::new();
    params.insert("scale".into(), 2.0);
    let replace_params = Vdi3805Mutation::ReplaceGeometryParameters(replace_geometry_parameters::ReplaceGeometryParameters { id: "geom.new".into(), new_parameters: params.clone() });
    let after_params = round_trip(&after_create, &replace_params);
    assert_eq!(after_params.geometry.get("geom.new").unwrap().parameters, params);

    let delete = Vdi3805Mutation::DeleteGeometry(delete_geometry::DeleteGeometry { id: "geom.valve.50".into() });
    let after_delete = round_trip(&base, &delete);
    assert!(!after_delete.geometry.contains_key("geom.valve.50"));
    let undo = delete.inverse(&base);
    assert_eq!(undo.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn curve_lifecycle_round_trips() {
    let base = Vdi3805Snapshot::default();
    let curve = crate::CharacteristicCurve {
        id: "curve.new".into(),
        x_unit: crate::VdiUnit::delta("%", crate::VdiQuantityKind::Dimensionless, 0.01),
        y_unit: crate::VdiUnit::absolute("m3/h", crate::VdiQuantityKind::Volume, 1.0),
        points: vec![crate::CurvePoint { x: 0.0, y: 0.0 }],
    };
    let create = Vdi3805Mutation::CreateCurve(create_curve::CreateCurve { curve: curve.clone() });
    let after_create = round_trip(&base, &create);
    assert!(after_create.curves.contains_key("curve.new"));

    let new_points = vec![crate::CurvePoint { x: 0.0, y: 0.0 }, crate::CurvePoint { x: 100.0, y: 9.0 }];
    let replace = Vdi3805Mutation::ReplaceCurvePoints(replace_curve_points::ReplaceCurvePoints { id: "curve.new".into(), new_points: new_points.clone() });
    let after_replace = round_trip(&after_create, &replace);
    assert_eq!(after_replace.curves.get("curve.new").unwrap().points, new_points);

    let delete = Vdi3805Mutation::DeleteCurve(delete_curve::DeleteCurve { id: "curve.kvs".into() });
    let after_delete = round_trip(&base, &delete);
    assert!(!after_delete.curves.contains_key("curve.kvs"));
}

#[semio_framework_async_macros::async_test]
async fn semantic_kinds_cover_every_variant() {
    assert_eq!(Vdi3805Mutation::kinds().len(), 19);
    let mutation = Vdi3805Mutation::ChangeStrictMode(change_strict_mode::ChangeStrictMode { new_strict_mode: true });
    assert_eq!(mutation.semantics().kind, "change-strict-mode");
    assert_eq!(mutation.semantics().record, "ChangedStrictMode");
}

// 🧪️ NOTE: `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️test/🦀️kit.rs`'s shared
// `assert_mutation_inverse_law`/`assert_mutation_diff_absorb_law` law helpers were deliberately
// NOT wired into this facet's tests — `semio-s-plugin-norm`'s `Cargo.toml` has no existing
// dependency on that test context crate (grepped: no `test context` import anywhere in this crate, and
// sibling facet `iso16757`'s migration made the same call), and step (e) of the fan-out recipe
// says to skip rather than add a new Cargo dependency. The `round_trip` helper above and its
// per-variant assertions already exercise the same diff/inverse laws directly.
