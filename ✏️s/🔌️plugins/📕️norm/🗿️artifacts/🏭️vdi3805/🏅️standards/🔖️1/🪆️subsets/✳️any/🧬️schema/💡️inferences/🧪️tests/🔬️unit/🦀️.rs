use crate::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = Vdi3805Snapshot::default();
    assert_eq!(Vdi3805Inference::infer(&snapshot).expect("valid materialized inference fixture"), Vdi3805Inference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(Vdi3805Inference::infer(&Vdi3805Snapshot::default()).expect("valid materialized inference fixture"), Vdi3805Inference::default());
}

#[semio_framework_async_macros::async_test]
async fn part_100_component_references_follow_the_published_scope() {
    use crate::CheckStatus;
    use semio_s_artifact_norm_vdi3805_model::{conforming_valve_dataset, CompositionLink, Domain, EditionId, GenericAttributes, SheetAttributes, SheetId};

    assert_eq!(crate::part_66::metadata().domains, &[Domain::Electrical]);
    assert_eq!(crate::part_66::metadata().current_edition, EditionId::new(2026, 4));
    assert_eq!(crate::part_100::metadata().title_en, "Systems");
    assert_eq!(crate::part_100::metadata().current_edition, EditionId::new(2018, 6));
    assert_eq!(crate::part_100::metadata().part1_edition, EditionId::new(2011, 10));

    for row in include_str!("🧫️fixtures/part-100-components.tsv").lines().skip(1) {
        let (component_ids, status) = row.split_once('\t').expect("fixture columns");
        let expected = match status {
            "pass" => CheckStatus::Pass,
            "fail" => CheckStatus::Fail,
            other => panic!("unknown expected verdict {other}"),
        };
        let mut document = conforming_valve_dataset();
        let mut component = document.catalog.products[0].clone();
        component.id = "component-a".into();
        document.catalog.products.push(component.clone());
        component.id = "component-b".into();
        document.catalog.products.push(component);
        let package = &mut document.catalog.products[0];
        package.id = "package".into();
        package.sheet = SheetId(100);
        package.configuration.attributes = SheetAttributes::Generic(GenericAttributes::default());
        package.components = component_ids.split(',').filter(|id| !id.is_empty()).map(|id| CompositionLink { component_id: id.into(), quantity: 1 }).collect();

        let report = crate::evaluate(&document);
        let result = report.checks.iter().find(|check| check.id == "vdi3805.100.componentReferences.package").expect("Part 100 §1 component check");
        assert_eq!(result.status, expected, "{row}");
        assert!(report.unverified().any(|check| check.id == "vdi3805.100.recordSchema.package"));
        let part_expected = if expected == CheckStatus::Fail { CheckStatus::Fail } else { CheckStatus::UnverifiedBasis };
        assert_eq!(crate::part_100::check(&document).status, part_expected, "{row}");
        if expected == CheckStatus::Fail {
            assert!(!result.remedies.is_empty(), "{row}");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn part_66_requires_an_electrical_box_record_schema() {
    use semio_s_artifact_norm_vdi3805_model::{conforming_valve_dataset, EditionProfileChoice, GenericAttributes, SheetAttributes, SheetId};

    let mut document = conforming_valve_dataset();
    let product = &mut document.catalog.products[0];
    product.id = "box".into();
    product.sheet = SheetId(66);
    product.configuration.attributes = SheetAttributes::Generic(GenericAttributes::default());
    document.edition_profile.cold_insert("66".into(), EditionProfileChoice::Current);

    let report = crate::evaluate(&document);
    assert!(report.unverified().any(|check| check.id == "vdi3805.66.recordSchema.box"));
    assert!(!report.checks.iter().any(|check| check.id.starts_with("vdi3805.66.mandatory")
        || check.id == "vdi3805.66.edition_profile"));
    assert!(!report.complies());
    assert_eq!(crate::part_66::check(&document).status, CheckStatus::UnverifiedBasis);
}

#[semio_framework_async_macros::async_test]
async fn published_sheets_keep_distinct_product_scopes() {
    use semio_s_artifact_norm_vdi3805_model::{conforming_blatt_dataset, EditionId, SHEET_ENTRIES};

    for row in include_str!("🧫️fixtures/published-sheet-scopes.tsv").lines().skip(1) {
        let f: Vec<_> = row.split('\t').collect();
        let sheet: u16 = f[0].parse().unwrap();
        let metadata = SHEET_ENTRIES.iter().find(|entry| entry.id.0 == sheet).unwrap();
        assert_eq!(metadata.current_edition, EditionId::new(f[1].parse().unwrap(), f[2].parse().unwrap()));
        assert_eq!((metadata.title_de, metadata.title_en), (f[3], f[4]));
        if sheet == 1 { continue; }
        let document = conforming_blatt_dataset(sheet);
        let prefix = format!("vdi3805.{sheet}.");
        let report = crate::evaluate(&document);
        let product = &document.catalog.products[0];
        match sheet {
            10 => assert_eq!(crate::part_10::check(&document).status, CheckStatus::UnverifiedBasis),
            19 => assert_eq!(crate::part_19::check(&document).status, CheckStatus::UnverifiedBasis),
            _ => {}
        }
        assert!(report.unverified().any(|check| check.id == format!("{prefix}recordSchema.{}", product.id)));
        assert!(report.checks.iter().all(|check| !check.id.starts_with(&format!("{prefix}mandatory"))));
    }
}

#[test]
fn typed_attributes_follow_the_published_sheet_numbers() {
    use semio_s_artifact_norm_vdi3805_model::{NativeRecord, RecordFamilyId, SheetAttributes, SheetId, attributes_from_records};

    for line in include_str!("🧫️fixtures/typed-sheet-routing.tsv").lines().skip(1) {
        let row: Vec<_> = line.split('\t').collect();
        let sheet = SheetId(row[0].parse().unwrap());
        let record = NativeRecord { family: RecordFamilyId("210".into()), fields: vec!["210".into(), row[2].into(), row[3].into()], extensions: Default::default() };
        let kind = match attributes_from_records(sheet, &[record]) {
            SheetAttributes::ValveHeating(_) => "valve",
            SheetAttributes::HeatGenerator(_) => "generator",
            SheetAttributes::PumpHeating(attrs) => {
                assert_eq!(attrs.nominal_flow_m3_s, row[4].parse::<f64>().unwrap());
                "pump"
            }
            SheetAttributes::Radiator(_) => "radiator",
            SheetAttributes::Generic(_) => "generic",
        };
        assert_eq!(kind, row[1], "sheet {}", sheet.0);
    }
}

#[test]
fn part22_air_to_air_record_checks_follow_correction_date() {
    use semio_s_artifact_norm_vdi3805_model::{conforming_blatt_dataset, EditionId, NativeRecord, RecordFamilyId, SheetId};
    let candidates = include_str!("🧫️fixtures/air-to-air-heat-pump-records.tsv");
    for line in candidates.lines().skip(1) {
        let mut row = line.split('\t');
        let family = row.next().unwrap();
        let name = row.next().unwrap();
        let invalid = row.next().unwrap();
        let mut document = conforming_blatt_dataset(22);
        let product = &mut document.catalog.products[0];
        product.records.push(NativeRecord {
            family: RecordFamilyId(family.into()), fields: row.map(str::to_owned).collect(), extensions: Default::default(),
        });
        document.catalog.file.record_count += 1;
        let prefix = format!("vdi3805.22.record{}.", family.replace('.', ""));
        let check = |document: &_| crate::evaluate(document).checks.into_iter().find(|check| check.id.starts_with(&prefix));
        let result = check(&document).unwrap();
        assert_eq!(result.status, if invalid.is_empty() { CheckStatus::Pass } else { CheckStatus::Fail }, "{name}");
        assert!(crate::evaluate(&document).unverified().any(|check| check.id.starts_with("vdi3805.22.recordSchema.")));
        document.correction_as_of = EditionId::new(2021, 12);
        assert_eq!(check(&document).unwrap().status, CheckStatus::Fail, "{name} before the correction");
        document.catalog.products[0].sheet = SheetId(23);
        assert!(check(&document).is_none(), "{name} outside Part 22");
    }
}

#[test]
fn part22_published_correction_fields_are_checked_in_the_report() {
    use semio_s_artifact_norm_vdi3805_model::{conforming_blatt_dataset, EditionId, NativeRecord, RecordFamilyId, SheetId};

    let candidates = include_str!("🧫️fixtures/part22-published-correction-fields.tsv");
    for line in candidates.lines().skip(1) {
        let mut row = line.split('\t');
        let date: u32 = row.next().unwrap().parse().unwrap();
        let family = row.next().unwrap();
        let name = row.next().unwrap();
        let invalid = row.next().unwrap();
        let mut document = conforming_blatt_dataset(22);
        document.correction_as_of = EditionId::new((date / 100) as u16, (date % 100) as u8);
        let product = &mut document.catalog.products[0];
        let index = product.records.len();
        let id = format!("vdi3805.22.record{}.{}.{index}", family.replace('.', ""), product.id);
        product.records.push(NativeRecord {
            family: RecordFamilyId(family.into()), fields: row.map(str::to_owned).collect(), extensions: Default::default(),
        });
        document.catalog.file.record_count += 1;
        let result = crate::evaluate(&document).checks.into_iter().find(|check| check.id == id).expect("published positions check");
        assert_eq!(result.status, if invalid.is_empty() { CheckStatus::Pass } else { CheckStatus::Fail }, "{name}");
        if !invalid.is_empty() {
            assert!(result.remedies.iter().any(|remedy| !remedy.applicable), "{name}");
        }
        document.catalog.products[0].sheet = SheetId(23);
        assert!(crate::evaluate(&document).checks.iter().all(|check| check.id != id), "{name}");
    }
}
