//! 🧬️ Vdi3805 artifact schema — every field of the artifact with its state class.

use std::collections::BTreeMap;

use crate::{GenericAttributes, CatalogIndex, CharacteristicCurve, EditionId, EditionProfileChoice, ManufacturerCatalog, ManufacturerFile, ParametricGeometry, SecurityLimits};
use ::framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full Vdi3805 artifact state across the artifact and presence lanes.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.vdi3805")]
pub struct Vdi3805Artifact {
    #[state(artifact)]
    pub catalog: ManufacturerCatalog,
    #[state(artifact)]
    pub edition_profile: BTreeMap<String, EditionProfileChoice>,
    #[state(artifact)]
    pub correction_as_of: EditionId,
    #[state(artifact)]
    pub strict_mode: bool,
    #[state(artifact)]
    pub index: CatalogIndex,
    #[state(artifact)]
    pub geometry: BTreeMap<String, ParametricGeometry>,
    #[state(artifact)]
    pub curves: BTreeMap<String, CharacteristicCurve>,
    #[state(artifact)]
    pub limits: SecurityLimits,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Vdi3805Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> Vdi3805Snapshot {
        Vdi3805Snapshot {
            catalog: self.catalog.clone(),
            edition_profile: self.edition_profile.clone(),
            correction_as_of: self.correction_as_of,
            strict_mode: self.strict_mode,
            index: self.index.clone(),
            geometry: self.geometry.clone(),
            curves: self.curves.clone(),
            limits: self.limits,
        }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: Vdi3805Snapshot) -> Self {
        Self {
            catalog: snapshot.catalog,
            edition_profile: snapshot.edition_profile,
            correction_as_of: snapshot.correction_as_of,
            strict_mode: snapshot.strict_mode,
            index: snapshot.index,
            geometry: snapshot.geometry,
            curves: snapshot.curves,
            limits: snapshot.limits,
        }
    }
}

//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.vdi3805` — twenty handcrafted schema leaves.
pub fn vdi3805_artifact_schema_descriptor() -> ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
    ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.norm.vdi3805",
        artifact: ::semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️ComplianceHelpers
use crate::document::{AnnexChoice, ClauseId, LocalizedCopy, NormError, SubjectRef};
/// 📐️ Pure VDI 3805 compliance helpers — native-text parsing with typed record families,
/// Part 1 structural validation, and sheet attribute catalogues.
use crate::*;
use ::dsl;

const FAMILY: &str = "VDI 3805";
pub const ANNEX: AnnexChoice = AnnexChoice::De;

pub fn clause(part: &str, section: &str) -> ClauseId {
    ClauseId::new(FAMILY, part, section)
}

pub fn subject_product(product: &CatalogueProduct) -> SubjectRef {
    SubjectRef::new(
        product.id.clone(),
        format!("catalog.products[id={}]", product.id),
        LocalizedCopy::new(format!("Product {}", product.identity.article_number), format!("Produkt {}", product.identity.article_number)),
    )
}

pub fn subject_dataset() -> SubjectRef {
    SubjectRef::whole(LocalizedCopy::new("Manufacturer dataset", "Herstellerdatensatz"))
}

/// 🎛 Nominal diameters admitted by VDI 3805 Blatt 2 / ISO 6708 series.
pub const VALVE_DN_SERIES: &[u16] = &[10, 15, 20, 25, 32, 40, 50, 65, 80, 100, 125, 150, 200, 250, 300];

/// 📊️ Minimum kvs [m³/h] per DN for Blatt 2 control valves (catalogue floor).
pub fn valve_min_kvs_m3_h(dn: u16) -> f64 {
    match dn {
        10 => 0.16,
        15 => 0.25,
        20 => 0.40,
        25 => 0.63,
        32 => 1.00,
        40 => 1.60,
        50 => 2.50,
        65 => 4.00,
        80 => 6.30,
        100 => 10.0,
        125 => 16.0,
        150 => 25.0,
        200 => 40.0,
        250 => 63.0,
        300 => 100.0,
        _ => 0.63,
    }
}

/// 📐️ Minimum radiator heat exponent n (EN 442 / Blatt 3).
pub const RADIATOR_N_MIN: f64 = 1.1;
/// 📐️ Maximum radiator heat exponent n (EN 442 / Blatt 3).
pub const RADIATOR_N_MAX: f64 = 1.5;

/// 📔️ Minimum field arity (including family code) per record family.
pub fn record_min_fields(family: &str) -> Option<usize> {
    match family {
        "010" => Some(6),
        "100" => Some(5),
        "110" => Some(3),
        "200" => Some(5),
        "400" => Some(4),
        "700" => Some(3),
        "900" => Some(2),
        _ => None,
    }
}





/// 🔁 Writes identity + sheet into the mandatory 100 record fields.
pub fn sync_titles_into_records(product: &mut CatalogueProduct) {
    let de = text_in(&product.title, "de");
    let en = text_in(&product.title, "en");
    let fields = vec!["700".into(), "de".into(), de, "en".into(), en];
    if let Some(r700) = product.records.iter_mut().find(|r| r.family.0 == "700" || r.fields.first().is_some_and(|f| f == "700")) {
        r700.family = RecordFamilyId("700".into());
        r700.fields = fields;
    } else {
        product.records.push(NativeRecord {
            family: RecordFamilyId("700".into()),
            fields,
            extensions: ExtensionBag::default(),
        });
    }
}

pub fn sync_identity_into_records(product: &mut CatalogueProduct) {
    let sheet = product.sheet.0.to_string();
    let fields = vec![
        "100".into(),
        product.identity.manufacturer_code.clone(),
        product.identity.product_group.clone(),
        product.identity.article_number.clone(),
        sheet,
    ];
    if let Some(r100) = product.records.iter_mut().find(|r| r.family.0 == "100" || r.fields.first().is_some_and(|f| f == "100")) {
        r100.family = RecordFamilyId("100".into());
        r100.fields = fields;
    } else {
        product.records.insert(0, NativeRecord {
            family: RecordFamilyId("100".into()),
            fields,
            extensions: ExtensionBag::default(),
        });
    }
}

/// 🔄 Writes typed configuration.attributes into a native 210 record (Part 1 authoritative mirror).
pub fn sync_typed_attributes_into_records(product: &mut CatalogueProduct) {
    let fields = match &product.configuration.attributes {
        SheetAttributes::ValveHeating(a) => vec![
            "210".into(),
            "dn".into(),
            a.dn.to_string(),
            "kvs".into(),
            format!("{}", a.kvs_m3_h()),
            "pressure_class".into(),
            a.pressure_class.clone(),
            "connection_type".into(),
            a.connection_type.clone(),
            "authority_min".into(),
            a.authority_min.to_string(),
            "authority_max".into(),
            a.authority_max.to_string(),
        ],
        SheetAttributes::Radiator(a) => vec![
            "210".into(),
            "standard_output_w".into(),
            a.standard_output_w.to_string(),
            "n".into(),
            a.heat_exponent_n.to_string(),
            "length_m".into(),
            a.length_m.to_string(),
            "height_m".into(),
            a.height_m.to_string(),
            "depth_m".into(),
            a.depth_m.to_string(),
            "connection_type".into(),
            a.connection_type.clone(),
        ],
        SheetAttributes::PumpHeating(a) => {
            let mut fields = vec![
                "210".into(),
                "dn_suction".into(),
                a.dn_suction.to_string(),
                "dn_discharge".into(),
                a.dn_discharge.to_string(),
                "nominal_flow_m3_s".into(),
                a.nominal_flow_m3_s.to_string(),
                "nominal_head_m".into(),
                a.nominal_head_m.to_string(),
                "motor_power_w".into(),
                a.motor_power_w.to_string(),
                "hydraulic_efficiency".into(),
                a.hydraulic_efficiency.to_string(),
            ];
            if let Some(curve) = &a.qh_curve_ref {
                fields.push("qh_curve_ref".into());
                fields.push(curve.clone());
            }
            fields
        },
        SheetAttributes::HeatGenerator(a) => vec![
            "210".into(),
            "nominal_heat_output_w".into(),
            a.nominal_heat_output_w.to_string(),
            "fuel_type".into(),
            a.fuel_type.clone(),
            "flow_temp_max_c".into(),
            a.flow_temp_max_c.to_string(),
            "return_temp_min_c".into(),
            a.return_temp_min_c.to_string(),
        ],
        SheetAttributes::Generic(g) => {
            let mut fields = vec!["210".into()];
            for e in &g.entries {
                fields.push(e.key.clone());
                fields.push(e.value.clone());
            }
            fields
        },
    };
    if let Some(r) = product.records.iter_mut().find(|r| r.family.0 == RecordFamilyId::R210) {
        r.fields = fields;
    } else {
        product.records.push(NativeRecord {
            family: RecordFamilyId(RecordFamilyId::R210.to_string()),
            fields,
            extensions: ExtensionBag::default(),
        });
    }
    if product.id.is_empty() {
        product.id = product.identity.article_number.clone();
    }
}










fn count_native_records(catalog: &ManufacturerCatalog) -> u32 {
    catalog.products.iter().map(|p| p.records.len() as u32).sum()
}

/// ✅️ Part 1 structural validation over the full manufacturer dataset.
pub fn validate_structure(document: &Vdi3805Snapshot) -> Vec<Diagnostic> {
    let catalog = &document.catalog;
    let mut issues = Vec::new();
    if catalog.file.manufacturer.is_empty() {
        issues.push(Diagnostic::error("catalog.file.manufacturer", "missing manufacturer code"));
    }
    if catalog.file.charset.is_empty() {
        issues.push(Diagnostic::error("catalog.file.charset", "missing charset"));
    }
    if catalog.products.is_empty() {
        issues.push(Diagnostic::error("catalog.products", "empty product list"));
    }
    let actual = count_native_records(catalog);
    if catalog.file.record_count != actual {
        issues.push(Diagnostic::error("catalog.file.recordCount", format!("record_count {} != actual {}", catalog.file.record_count, actual)));
    }
    let product_ids: BTreeSet<String> = catalog.products.iter().map(|p| p.id.clone()).collect();
    for (pi, product) in catalog.products.iter().enumerate() {
        let base = format!("catalog.products[{pi}]");
        if product.identity.article_number.is_empty() {
            issues.push(Diagnostic::error(format!("{base}.identity.articleNumber"), "missing article number"));
        }
        if product.identity.manufacturer_code.is_empty() {
            issues.push(Diagnostic::error(format!("{base}.identity.manufacturerCode"), "missing manufacturer code"));
        }
        if product.identity.product_group.is_empty() {
            issues.push(Diagnostic::error(format!("{base}.identity.productGroup"), "missing product group"));
        }
        if product.configuration.id.is_empty() {
            issues.push(Diagnostic::error(format!("{base}.configuration.id"), "missing configuration id"));
        }
        let has_100 = product.records.iter().any(|r| r.family.0 == "100");
        if !has_100 {
            issues.push(Diagnostic::error(format!("{base}.records"), "missing mandatory record 100"));
        }
        for (ri, record) in product.records.iter().enumerate() {
            if let Some(min) = record_min_fields(record.family.0.as_str()) {
                if record.fields.len() < min {
                    issues.push(Diagnostic::error(format!("{base}.records[{ri}]"), format!("record {} needs ≥{min} fields, has {}", record.family.0, record.fields.len())));
                }
            } else {
                let known: BTreeSet<&str> = RecordFamilyId::all_known().iter().copied().collect();
                if !known.contains(record.family.0.as_str()) && !record.family.0.starts_with('9') {
                    issues.push(Diagnostic::error(format!("{base}.records[{ri}].family"), format!("unknown record family {}", record.family.0)));
                }
            }
        }
        if let Some(geom_ref) = &product.configuration.geometry_ref {
            if !document.geometry.contains_key(geom_ref) {
                issues.push(Diagnostic::error(format!("{base}.configuration.geometryRef"), format!("dangling geometry ref {geom_ref}")));
            }
            let has_200 = product.records.iter().any(|r| r.family.0 == "200");
            if !has_200 {
                issues.push(Diagnostic::error(format!("{base}.records"), "geometry_ref set but record 200 missing"));
            }
        }
        for (fi, fun_ref) in product.configuration.function_refs.iter().enumerate() {
            if !document.curves.contains_key(fun_ref) {
                issues.push(Diagnostic::error(format!("{base}.configuration.functionRefs[{fi}]"), format!("dangling curve ref {fun_ref}")));
            }
        }
        for (ai, link) in product.accessories.iter().enumerate() {
            if !product_ids.contains(&link.accessory_id) {
                issues.push(Diagnostic::error(format!("{base}.accessories[{ai}].accessoryId"), format!("dangling accessory {}", link.accessory_id)));
            }
        }
        for (ci, link) in product.components.iter().enumerate() {
            if !product_ids.contains(&link.component_id) {
                issues.push(Diagnostic::error(format!("{base}.components[{ci}].componentId"), format!("dangling component {}", link.component_id)));
            }
        }
        let title_locales: BTreeSet<&str> = product.title.iter().map(|t| t.locale.as_str()).collect();
        if !title_locales.contains("de") || !title_locales.contains("en") {
            issues.push(Diagnostic::error(format!("{base}.title"), "multilingual texts require de and en"));
        }
    }
    issues
}

/// 🔢️ Linear map between two scalar domains.
pub fn linear_map(x: f64, x0: f64, x1: f64, y0: f64, y1: f64) -> f64 {
    if (x1 - x0).abs() < f64::EPSILON {
        return y0;
    }
    y0 + (x - x0) * (y1 - y0) / (x1 - x0)
}




//#endregion 🔖️ComplianceHelpers

//#region 🧪️ComplianceTests
#[cfg(test)]
#[path = "🧪️tests/⚖️compliance/🦀️.rs"]
mod compliance_tests;
//#endregion 🧪️ComplianceTests
