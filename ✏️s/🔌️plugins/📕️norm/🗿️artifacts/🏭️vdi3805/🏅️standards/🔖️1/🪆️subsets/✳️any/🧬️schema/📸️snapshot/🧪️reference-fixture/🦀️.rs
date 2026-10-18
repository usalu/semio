//! 🧪️ Reference fixture of the VDI 3805 snapshot: the reference snapshot and the conforming valve dataset. See [`super`].

use super::catalog_model::{BoundingBox, CatalogIndex, CatalogueProduct, CharacteristicCurve, Configuration, ConnectionPoint, CurvePoint, ManufacturerCatalog, ParametricGeometry};
use super::limits_and_code_tables::SecurityLimits;
use super::localized_text_and_units::{bilingual, VdiQuantityKind, VdiUnit};
use super::product_identity_and_links::{AccessoryLink, CompositionLink, ExtensionBag, ManufacturerFile, ProductIdentity};
use super::record_families::{BuildingSystemNumber, RecordFamilyId};
use super::sheet_attributes::{NativeRecord, SheetAttributes, ValveHeatingAttributes};
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
