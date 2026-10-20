//! 🧪️ Reference fixture of the VDI 3805 snapshot: the reference snapshot and the conforming valve dataset. See [`super`].

use super::catalog_model::{sync_identity_into_records, sync_titles_into_records, sync_typed_attributes_into_records, BoundingBox, CatalogIndex, CatalogueProduct, CharacteristicCurve, Configuration, ConnectionPoint, CurvePoint, ManufacturerCatalog, ParametricGeometry};
use super::limits_and_code_tables::SecurityLimits;
use super::localized_text_and_units::{bilingual, VdiQuantityKind, VdiUnit};
use super::product_identity_and_links::{AccessoryLink, CompositionLink, ExtensionBag, ManufacturerFile, ProductIdentity};
use super::record_families::{BuildingSystemNumber, RecordFamilyId};
use super::sheet_attributes::{GenericAttribute, GenericAttributes, HeatGeneratorAttributes, NativeRecord, PumpHeatingAttributes, RadiatorAttributes, SheetAttributes, ValveHeatingAttributes};
use super::sheet_registry::{EditionId, SheetId};
use super::Vdi3805Snapshot;
use std::collections::BTreeMap;

pub fn reference_fixture() -> Vdi3805Snapshot {
    conforming_valve_dataset()
}

/// ✅️ Fully conforming sheet-2 heating control valve dataset (Part 1 + Blatt 2).
pub fn conforming_valve_dataset() -> Vdi3805Snapshot {
    let bsn = BuildingSystemNumber { system_code: "420".into(), subsystem: "10".into(), sequence: 1 };
    let file = ManufacturerFile {
        header_version: "3805".into(),
        manufacturer: "DEMO".into(),
        building_system_number: bsn,
        created: "2023-06-01".into(),
        charset: "UTF-8".into(),
        record_count: 11,
        extensions: ExtensionBag::default(),
    };
    let attributes = SheetAttributes::ValveHeating(ValveHeatingAttributes::from_kvs_m3_h(50, 4.5, "PN16", "flange", 0.3, 0.7));
    let product = CatalogueProduct {
        id: "VLV-50-001".into(),
        identity: ProductIdentity { manufacturer_code: "DEMO".into(), product_group: "HV".into(), article_number: "VLV-50-001".into() },
        title: bilingual("Stellventil DN50", "Control valve DN50"),
        sheet: SheetId(2),
        records: vec![
            NativeRecord { family: RecordFamilyId(RecordFamilyId::R100.to_string()), fields: vec!["100".into(), "DEMO".into(), "HV".into(), "VLV-50-001".into(), "2".into()], extensions: ExtensionBag::default() },
            NativeRecord {
                family: RecordFamilyId(RecordFamilyId::R110.to_string()),
                fields: vec!["110".into(), "cfg.VLV-50-001".into(), "geom-valve-50".into(), "curve-kvs".into()],
                extensions: ExtensionBag::default(),
            },
            NativeRecord {
                family: RecordFamilyId(RecordFamilyId::R200.to_string()),
                fields: vec!["200".into(), "geom-valve-50".into(), "0.15".into(), "0.20".into(), "0.10".into()],
                extensions: ExtensionBag::default(),
            },
            NativeRecord {
                family: RecordFamilyId(RecordFamilyId::R210.to_string()),
                fields: vec![
                    "210".into(),
                    "dn".into(),
                    "50".into(),
                    "kvs".into(),
                    "4.5".into(),
                    "pressure_class".into(),
                    "PN16".into(),
                    "connection_type".into(),
                    "flange".into(),
                    "authority_min".into(),
                    "0.3".into(),
                    "authority_max".into(),
                    "0.7".into(),
                ],
                extensions: ExtensionBag::default(),
            },
            NativeRecord {
                family: RecordFamilyId(RecordFamilyId::R400.to_string()),
                fields: vec!["400".into(), "curve-kvs".into(), "0".into(), "0".into(), "100".into(), "4.5".into()],
                extensions: ExtensionBag::default(),
            },
            NativeRecord {
                family: RecordFamilyId(RecordFamilyId::R700.to_string()),
                fields: vec!["700".into(), "de".into(), "Stellventil DN50".into(), "en".into(), "Control valve DN50".into()],
                extensions: ExtensionBag::default(),
            },
        ],
        configuration: Configuration { id: "cfg.VLV-50-001".into(), attributes, geometry_ref: Some("geom-valve-50".into()), function_refs: vec!["curve-kvs".into()] },
        accessories: Vec::new(),
        components: Vec::new(),
        extensions: ExtensionBag::default(),
    };
    
    let accessory = CatalogueProduct {
        id: "ACT-01".into(),
        identity: ProductIdentity { manufacturer_code: "DEMO".into(), product_group: "ACT".into(), article_number: "ACT-01".into() },
        title: bilingual("Stellantrieb", "Actuator"),
        sheet: SheetId(2),
        records: vec![
            NativeRecord {
                family: RecordFamilyId(RecordFamilyId::R100.to_string()),
                fields: vec!["100".into(), "DEMO".into(), "ACT".into(), "ACT-01".into(), "2".into()],
                extensions: ExtensionBag::default(),
            },
            NativeRecord {
                family: RecordFamilyId(RecordFamilyId::R210.to_string()),
                fields: vec![
                    "210".into(),
                    "dn".into(),
                    "50".into(),
                    "kvs".into(),
                    "4.5".into(),
                    "pressure_class".into(),
                    "PN16".into(),
                    "connection_type".into(),
                    "flange".into(),
                    "authority_min".into(),
                    "0.3".into(),
                    "authority_max".into(),
                    "0.7".into(),
                ],
                extensions: ExtensionBag::default(),
            },
            NativeRecord {
                family: RecordFamilyId(RecordFamilyId::R700.to_string()),
                fields: vec!["700".into(), "de".into(), "Stellantrieb".into(), "en".into(), "Actuator".into()],
                extensions: ExtensionBag::default(),
            },
        ],
        configuration: Configuration {
            id: "cfg.ACT-01".into(),
            attributes: SheetAttributes::ValveHeating(ValveHeatingAttributes::from_kvs_m3_h(50, 4.5, "PN16", "flange", 0.3, 0.7)),
            geometry_ref: None,
            function_refs: vec![],
        },
        accessories: Vec::new(),
        components: Vec::new(),
        extensions: ExtensionBag::default(),
    };
    let mut product = product;
    product.accessories = vec![AccessoryLink { accessory_id: "ACT-01".into(), required: true, quantity: 1 }];
    product.components = vec![CompositionLink { component_id: "ACT-01".into(), quantity: 1 }];
    let mut catalog = ManufacturerCatalog { file: file.clone(), products: vec![product, accessory], extensions: ExtensionBag::default() };

    let geometry = BTreeMap::from([(
        "geom-valve-50".into(),
        ParametricGeometry {
            id: "geom-valve-50".into(),
            bbox: BoundingBox::from_size(0.15, 0.20, 0.10),
            connections: vec![ConnectionPoint {
                id: "in".into(),
                medium: "water".into(),
                position: [0.0, 0.1, 0.05],
                direction: [-1.0, 0.0, 0.0],
                diameter_mm: Some(50.0),
            }],
            parameters: BTreeMap::from([("scale".into(), 1.0), ("clearance".into(), 0.01)]),
        },
    )]);
    let curves = BTreeMap::from([(
        "curve-kvs".into(),
        CharacteristicCurve {
            id: "curve-kvs".into(),
            x_unit: VdiUnit::delta("%", VdiQuantityKind::Dimensionless, 0.01),
            y_unit: VdiUnit::absolute("m3/h", VdiQuantityKind::Volume, 1.0 / 3600.0),
            points: vec![CurvePoint { x: 0.0, y: 0.0 }, CurvePoint { x: 100.0, y: 4.5 }],
        },
    )]);
    catalog.file.record_count = catalog.products.iter().map(|p| p.records.len() as u32).sum();
    let index = CatalogIndex::from_catalog(&catalog);
    Vdi3805Snapshot {
        catalog,
        edition_profile: BTreeMap::new(),
        correction_as_of: EditionId::new(2024, 1),
        strict_mode: false,
        index,
        geometry,
        curves,
        limits: SecurityLimits::default(),
    }
}

/// ❌️ Non-conforming dataset with multiple Part 1 + Blatt 2 violations for remediation tests.
pub fn nonconforming_valve_dataset() -> Vdi3805Snapshot {
    let mut doc = conforming_valve_dataset();
    doc.catalog.file.record_count = 3;
    if let SheetAttributes::ValveHeating(ref mut attrs) = doc.catalog.products[0].configuration.attributes {
        attrs.dn = 47;
        attrs.kvs_m3_s = 0.5 / 3600.0;
        attrs.authority_min = 0.8;
        attrs.authority_max = 0.2;
        attrs.pressure_class.clear();
    }
    if let Some(r210) = doc.catalog.products[0].records.iter_mut().find(|r| r.family.0 == RecordFamilyId::R210) {
        r210.fields = vec![
            "210".into(),
            "dn".into(),
            "47".into(),
            "kvs".into(),
            "0.5".into(),
            "pressure_class".into(),
            "".into(),
            "connection_type".into(),
            "flange".into(),
            "authority_min".into(),
            "0.8".into(),
            "authority_max".into(),
            "0.2".into(),
        ];
    }
    doc.catalog.products[0].configuration.geometry_ref = Some("geom.missing".into());
    doc.catalog.products[0].records.retain(|r| r.family.0 != RecordFamilyId::R700);
    doc.catalog.products.push(CatalogueProduct {
        id: "HIST-12".into(),
        identity: ProductIdentity { manufacturer_code: "DEMO".into(), product_group: "HX".into(), article_number: "HIST-12".into() },
        title: bilingual("Historischer Vorschlag", "Historical proposal"),
        sheet: SheetId(12),
        records: vec![NativeRecord {
            family: RecordFamilyId(RecordFamilyId::R100.to_string()),
            fields: vec!["100".into(), "DEMO".into(), "HX".into(), "HIST-12".into(), "12".into()],
            extensions: ExtensionBag::default(),
        }],
        configuration: Configuration { id: "cfg.HIST-12".into(), attributes: SheetAttributes::Generic(GenericAttributes::default()), geometry_ref: None, function_refs: Vec::new() },
        accessories: Vec::new(),
        components: Vec::new(),
        extensions: ExtensionBag::default(),
    });
    doc.strict_mode = true;
    doc.index = CatalogIndex::from_catalog(&doc.catalog);
    doc
}
/// 📋 Assessed operative Blätter that require committed examples (CORRECTION 13:43 + Wave D Round 3).
pub const ASSESSED_BLATT_SHEETS: &[u16] = &[2, 3, 4, 5, 6, 7, 8, 16, 19, 53, 60];

fn base_header() -> ManufacturerFile {
    ManufacturerFile {
        header_version: "3805".into(),
        manufacturer: "DEMO".into(),
        building_system_number: BuildingSystemNumber { system_code: "420".into(), subsystem: "10".into(), sequence: 1 },
        created: "2023-06-01".into(),
        charset: "UTF-8".into(),
        record_count: 0,
        extensions: ExtensionBag::default(),
    }
}

fn generic_product(article: &str, sheet: u16, group: &str, title_de: &str, title_en: &str, attrs: Vec<(&str, &str)>) -> CatalogueProduct {
    let mut fields = vec!["210".into()];
    let mut entries = Vec::new();
    if !attrs.iter().any(|(k, _)| k.eq_ignore_ascii_case("product_group")) {
        fields.push("product_group".into());
        fields.push(group.into());
        entries.push(GenericAttribute { key: "product_group".into(), value: group.into(), unit: None });
    }
    for (k, v) in &attrs {
        fields.push((*k).into());
        fields.push((*v).into());
        entries.push(GenericAttribute { key: (*k).into(), value: (*v).into(), unit: None });
    }
    let records = vec![
        NativeRecord {
            family: RecordFamilyId(RecordFamilyId::R100.to_string()),
            fields: vec!["100".into(), "DEMO".into(), group.into(), article.into(), sheet.to_string()],
            extensions: ExtensionBag::default(),
        },
        NativeRecord { family: RecordFamilyId(RecordFamilyId::R210.to_string()), fields, extensions: ExtensionBag::default() },
        NativeRecord {
            family: RecordFamilyId(RecordFamilyId::R700.to_string()),
            fields: vec!["700".into(), "de".into(), title_de.into(), "en".into(), title_en.into()],
            extensions: ExtensionBag::default(),
        },
    ];
    CatalogueProduct {
        id: article.into(),
        identity: ProductIdentity { manufacturer_code: "DEMO".into(), product_group: group.into(), article_number: article.into() },
        title: bilingual(title_de, title_en),
        sheet: SheetId(sheet),
        records,
        configuration: Configuration {
            id: format!("cfg.{article}"),
            attributes: SheetAttributes::Generic(GenericAttributes { entries }),
            geometry_ref: None,
            function_refs: vec![],
        },
        accessories: Vec::new(),
        components: Vec::new(),
        extensions: ExtensionBag::default(),
    }
}

fn snapshot_with_products(products: Vec<CatalogueProduct>) -> Vdi3805Snapshot {
    let mut file = base_header();
    file.record_count = products.iter().map(|p| p.records.len() as u32).sum();
    let catalog = ManufacturerCatalog { file, products, extensions: ExtensionBag::default() };
    let index = CatalogIndex::from_catalog(&catalog);
    Vdi3805Snapshot {
        catalog,
        edition_profile: BTreeMap::new(),
        correction_as_of: EditionId::new(2024, 1),
        strict_mode: false,
        index,
        geometry: BTreeMap::new(),
        curves: BTreeMap::new(),
        limits: SecurityLimits::default(),
    }
}

/// ✅️ Conforming Blatt-3 radiator example.
pub fn conforming_blatt_3_dataset() -> Vdi3805Snapshot {
    let mut doc = conforming_valve_dataset();
    doc.catalog.products[0].id = "RAD-600".into();
    doc.catalog.products[0].identity.article_number = "RAD-600".into();
    doc.catalog.products[0].configuration.id = format!("cfg.RAD-600");
    doc.catalog.products[0].sheet = SheetId(3);
    doc.catalog.products[0].title = bilingual("Heizkörper 600", "Radiator 600");
    doc.catalog.products[0].configuration.attributes = SheetAttributes::Radiator(RadiatorAttributes {
        standard_output_w: 1200.0,
        heat_exponent_n: 1.3,
        length_m: 0.6,
        height_m: 0.6,
        depth_m: 0.1,
        connection_type: "flange".into(),
    });
    sync_identity_into_records(&mut doc.catalog.products[0]);
    sync_typed_attributes_into_records(&mut doc.catalog.products[0]);
    sync_titles_into_records(&mut doc.catalog.products[0]);
    sync_titles_into_records(&mut doc.catalog.products[0]);
    sync_titles_into_records(&mut doc.catalog.products[0]);
    sync_titles_into_records(&mut doc.catalog.products[0]);
    doc.catalog.products[0].configuration.geometry_ref = None;
    doc.catalog.products[0].configuration.function_refs.clear();
    doc.geometry.clear();
    doc.curves.clear();
    doc.catalog.file.record_count = doc.catalog.products.iter().map(|p| p.records.len() as u32).sum();
    doc.index = CatalogIndex::from_catalog(&doc.catalog);
    doc
}
/// ❌️ Non-conforming Blatt-3 radiator (≥2 fails).
pub fn nonconforming_blatt_3_dataset() -> Vdi3805Snapshot {
    let mut doc = conforming_blatt_3_dataset();
    if let SheetAttributes::Radiator(ref mut a) = doc.catalog.products[0].configuration.attributes {
        a.heat_exponent_n = 0.2;
        a.standard_output_w = 0.0;
        a.connection_type.clear();
    }
    sync_identity_into_records(&mut doc.catalog.products[0]);
    sync_typed_attributes_into_records(&mut doc.catalog.products[0]);
    sync_titles_into_records(&mut doc.catalog.products[0]);
    doc
}
/// ✅️ Conforming Blatt-5 pump example.
pub fn conforming_blatt_5_dataset() -> Vdi3805Snapshot {
    let mut doc = conforming_valve_dataset();
    doc.catalog.products[0].id = "PMP-50".into();
    doc.catalog.products[0].identity.article_number = "PMP-50".into();
    doc.catalog.products[0].configuration.id = format!("cfg.PMP-50");
    doc.catalog.products[0].sheet = SheetId(5);
    doc.catalog.products[0].title = bilingual("Heizungspumpe DN50", "Heating pump DN50");
    doc.catalog.products[0].configuration.attributes = SheetAttributes::PumpHeating(PumpHeatingAttributes {
        dn_suction: 50,
        dn_discharge: 50,
        nominal_flow_m3_s: 0.0025,
        nominal_head_m: 6.0,
        motor_power_w: 750.0,
        hydraulic_efficiency: 0.45,
        qh_curve_ref: Some("curve-qh".into()),
    });
    doc.catalog.products[0].configuration.function_refs = vec!["curve-qh".into()];
    if let Some(old) = doc.curves.remove("curve-kvs") {
        let mut qh = old;
        qh.id = "curve-qh".into();
        qh.x_unit = VdiUnit::delta("%", VdiQuantityKind::Dimensionless, 0.01);
        qh.y_unit = VdiUnit::absolute("m", VdiQuantityKind::Length, 1.0);
        qh.points = vec![CurvePoint { x: 0.0, y: 8.0 }, CurvePoint { x: 50.0, y: 6.0 }, CurvePoint { x: 100.0, y: 2.0 }];
        doc.curves.insert("curve-qh".into(), qh);
    }
    sync_identity_into_records(&mut doc.catalog.products[0]);
    sync_typed_attributes_into_records(&mut doc.catalog.products[0]);
    sync_titles_into_records(&mut doc.catalog.products[0]);
    doc.catalog.file.record_count = doc.catalog.products.iter().map(|p| p.records.len() as u32).sum();
    doc.index = CatalogIndex::from_catalog(&doc.catalog);
    doc
}
/// ❌️ Non-conforming Blatt-5 pump.
pub fn nonconforming_blatt_5_dataset() -> Vdi3805Snapshot {
    let mut doc = conforming_blatt_5_dataset();
    if let SheetAttributes::PumpHeating(ref mut a) = doc.catalog.products[0].configuration.attributes {
        a.nominal_flow_m3_s = 0.0;
        a.hydraulic_efficiency = 1.5;
    }
    sync_identity_into_records(&mut doc.catalog.products[0]);
    sync_typed_attributes_into_records(&mut doc.catalog.products[0]);
    sync_titles_into_records(&mut doc.catalog.products[0]);
    doc
}
/// ✅️ Conforming Blatt-6 heat generator.
pub fn conforming_blatt_6_dataset() -> Vdi3805Snapshot {
    let mut doc = conforming_valve_dataset();
    doc.catalog.products[0].id = "HG-24".into();
    doc.catalog.products[0].identity.article_number = "HG-24".into();
    doc.catalog.products[0].configuration.id = format!("cfg.HG-24");
    doc.catalog.products[0].sheet = SheetId(6);
    doc.catalog.products[0].title = bilingual("Wärmeerzeuger 24 kW", "Heat generator 24 kW");
    doc.catalog.products[0].configuration.attributes = SheetAttributes::HeatGenerator(HeatGeneratorAttributes {
        nominal_heat_output_w: 24000.0,
        fuel_type: "gas".into(),
        flow_temp_max_c: 80.0,
        return_temp_min_c: 40.0,
    });
    sync_identity_into_records(&mut doc.catalog.products[0]);
    sync_typed_attributes_into_records(&mut doc.catalog.products[0]);
    sync_titles_into_records(&mut doc.catalog.products[0]);
    doc.catalog.products[0].configuration.geometry_ref = None;
    doc.catalog.products[0].configuration.function_refs.clear();
    doc.geometry.clear();
    doc.curves.clear();
    doc.catalog.file.record_count = doc.catalog.products.iter().map(|p| p.records.len() as u32).sum();
    doc.index = CatalogIndex::from_catalog(&doc.catalog);
    doc
}
/// ❌️ Non-conforming Blatt-6 heat generator.
pub fn nonconforming_blatt_6_dataset() -> Vdi3805Snapshot {
    let mut doc = conforming_blatt_6_dataset();
    if let SheetAttributes::HeatGenerator(ref mut a) = doc.catalog.products[0].configuration.attributes {
        a.nominal_heat_output_w = 0.0;
        a.fuel_type.clear();
        a.return_temp_min_c = 95.0;
    }
    sync_identity_into_records(&mut doc.catalog.products[0]);
    sync_typed_attributes_into_records(&mut doc.catalog.products[0]);
    sync_titles_into_records(&mut doc.catalog.products[0]);
    doc
}
/// ✅️ Conforming operative Blatt snapshot for sheets that use GenericAttributes + Blatt code lists.
pub fn conforming_blatt_dataset(sheet: u16) -> Vdi3805Snapshot {
    match sheet {
        2 => conforming_valve_dataset(),
        3 => conforming_blatt_3_dataset(),
        5 => conforming_blatt_5_dataset(),
        6 => conforming_blatt_6_dataset(),
        4 => snapshot_with_products(vec![generic_product(
            "PIPE-DN50",
            4,
            "PIPE",
            "Rohr DN50",
            "Pipe DN50",
            vec![("dn", "50"), ("pressure_class", "PN16"), ("connection_type", "weld"), ("outer_diameter_m", "0.0603"), ("wall_thickness_m", "0.0029")],
        )]),
        7 => snapshot_with_products(vec![generic_product(
            "TANK-200",
            7,
            "TANK",
            "Speicher 200 L",
            "Storage 200 L",
            vec![("dn", "50"), ("pressure_class", "PN10"), ("connection_type", "flange"), ("volume_m3", "0.2")],
        )]),
        8 => snapshot_with_products(vec![generic_product(
            "FIT-80",
            8,
            "FIT",
            "Armatur DN80",
            "Fitting DN80",
            vec![("product_group", "FIT"), ("type_code", "STD"), ("dn", "80"), ("pressure_class", "PN16")],
        )]),
        16 => snapshot_with_products(vec![generic_product(
            "AT-100",
            16,
            "AT",
            "Luftdurchlass",
            "Air terminal",
            vec![("product_group", "AT"), ("type_code", "TYPE-A"), ("airflow_m3_s", "0.15"), ("pressure_drop_pa", "25")],
        )]),
        19 => snapshot_with_products(vec![generic_product(
            "FIL-F7",
            19,
            "FIL",
            "Filter F7",
            "Filter F7",
            vec![("product_group", "FIL"), ("type_code", "STD"), ("airflow_m3_s", "0.5"), ("filter_class", "F7")],
        )]),
        53 => snapshot_with_products(vec![generic_product(
            "HP-12",
            53,
            "HP",
            "Wärmepumpe 12 kW",
            "Heat pump 12 kW",
            vec![("product_group", "HP"), ("type_code", "STD"), ("dn", "32"), ("nominal_heat_output_w", "12000"), ("cop", "3.5")],
        )]),
        60 => snapshot_with_products(vec![generic_product(
            "COMP-50",
            60,
            "COMP",
            "Kompensator DN50",
            "Compensator DN50",
            vec![("dn", "50"), ("pressure_class", "PN16"), ("connection_type", "flange"), ("axial_force_n", "5000")],
        )]),
        other => snapshot_with_products(vec![generic_product(
            &format!("GEN-{other}"),
            other,
            "GEN",
            &format!("Produkt Blatt {other}"),
            &format!("Product sheet {other}"),
            vec![("product_group", "GEN"), ("type_code", "DEFAULT"), ("dn", "50")],
        )]),
    }
    }

/// ❌️ Non-conforming counterpart for an assessed Blatt (≥2 fails with remedies).
pub fn nonconforming_blatt_dataset(sheet: u16) -> Vdi3805Snapshot {
    match sheet {
        2 => nonconforming_valve_dataset(),
        3 => nonconforming_blatt_3_dataset(),
        5 => nonconforming_blatt_5_dataset(),
        6 => nonconforming_blatt_6_dataset(),
        _ => {
            let mut doc = conforming_blatt_dataset(sheet);
            if let Some(product) = doc.catalog.products.get_mut(0) {
                if let SheetAttributes::Generic(ref mut g) = product.configuration.attributes {
                    for e in &mut g.entries {
                        if e.key == "connection_type" || e.key == "filter_class" || e.key == "type_code" || e.key == "pressure_class" {
                            e.value = "INVALID".into();
                        }
                        if e.key == "dn" || e.key == "cop" || e.key == "airflow_m3_s" || e.key == "axial_force_n" || e.key == "volume_m3" {
                            e.value = "-1".into();
                        }
                    }
                }
                let kv: BTreeMap<String, String> = match &product.configuration.attributes {
                    SheetAttributes::Generic(g) => g.entries.iter().map(|e| (e.key.clone(), e.value.clone())).collect(),
                    _ => Default::default(),
                };
                if let Some(r210) = product.records.iter_mut().find(|r| r.family.0 == RecordFamilyId::R210) {
                    let mut fields = vec!["210".into()];
                    for (k, v) in &kv {
                        fields.push(k.clone());
                        fields.push(v.clone());
                    }
                    r210.fields = fields;
                }
                doc.catalog.file.record_count = 0;
            }
            doc
        }
    }
    }

/// 📚 All conforming assessed-Blatt examples for oracle + perturbation subjects.
pub fn all_conforming_blatt_examples() -> Vec<(u16, Vdi3805Snapshot)> {
    ASSESSED_BLATT_SHEETS.iter().copied().map(|s| (s, conforming_blatt_dataset(s))).collect()
}
