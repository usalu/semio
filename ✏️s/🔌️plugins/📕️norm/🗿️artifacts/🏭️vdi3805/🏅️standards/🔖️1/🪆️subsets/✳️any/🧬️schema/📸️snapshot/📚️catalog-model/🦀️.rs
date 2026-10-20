//! 📚️ Catalog model of the VDI 3805 snapshot: the configuration, the catalogue product, the manufacturer catalog, the geometry and characteristic curve records and the catalog index. See [`super`].

use super::localized_text_and_units::{text_in, VdiUnit};
use super::product_identity_and_links::{AccessoryLink, CompositionLink, ExtensionBag, ManufacturerFile, ProductIdentity};
use super::sheet_attributes::{NativeRecord, SheetAttributes};
use super::record_families::RecordFamilyId;
use super::sheet_registry::SheetId;
use semio_s_plugin_norm_compliance::LocalizedText;
use std::collections::BTreeMap;

/// ⚙️ Product configuration block.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Configuration {
    pub id: String,
    pub attributes: SheetAttributes,
    pub geometry_ref: Option<String>,
    pub function_refs: Vec<String>,
}

/// 📦️ Catalogue product in Part 1 hierarchy.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CatalogueProduct {
    pub id: String,
    pub identity: ProductIdentity,
    #[dsl(table)]
    pub title: Vec<LocalizedText>,
    pub sheet: SheetId,
    #[dsl(table)]
    pub records: Vec<NativeRecord>,
    pub configuration: Configuration,
    #[dsl(table)]
    pub accessories: Vec<AccessoryLink>,
    #[dsl(table)]
    pub components: Vec<CompositionLink>,
    pub extensions: ExtensionBag,
}

/// 📚️ Manufacturer catalogue document (Part 1).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ManufacturerCatalog {
    pub file: ManufacturerFile,
    #[dsl(table)]
    pub products: Vec<CatalogueProduct>,
    pub extensions: ExtensionBag,
}

impl ManufacturerCatalog {
    pub fn product_for_sheet(&self, sheet: SheetId) -> Option<&CatalogueProduct> {
        self.products.iter().find(|p| p.sheet == sheet)
    }
}

/// 📦️ Axis-aligned bounding box [m].
#[derive(Clone, Copy, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct BoundingBox {
    pub min_x: f64,
    pub min_y: f64,
    pub min_z: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub max_z: f64,
}

impl BoundingBox {
    pub fn from_size(w: f64, h: f64, d: f64) -> Self {
        Self { min_x: 0.0, min_y: 0.0, min_z: 0.0, max_x: w, max_y: h, max_z: d }
    }

    pub fn volume_m3(self) -> f64 {
        (self.max_x - self.min_x) * (self.max_y - self.min_y) * (self.max_z - self.min_z)
    }

    pub fn overlaps(self, other: Self, clearance: f64) -> bool {
        self.min_x - clearance < other.max_x && self.max_x + clearance > other.min_x && self.min_y - clearance < other.max_y && self.max_y + clearance > other.min_y && self.min_z - clearance < other.max_z && self.max_z + clearance > other.min_z
    }
}

/// 🔌️ Connection point on product geometry.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ConnectionPoint {
    pub id: String,
    pub medium: String,
    pub position: [f64; 3],
    pub direction: [f64; 3],
    #[dsl(unit = "mm")]
    pub diameter_mm: Option<f64>,
}

/// 🧊️ Parametric geometry definition.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ParametricGeometry {
    pub id: String,
    pub bbox: BoundingBox,
    #[dsl(table)]
    pub connections: Vec<ConnectionPoint>,
    pub parameters: BTreeMap<String, f64>,
}

impl ParametricGeometry {
    pub fn evaluate_bbox(&self) -> BoundingBox {
        let scale = self.parameters.get("scale").copied().unwrap_or(1.0);
        BoundingBox { min_x: self.bbox.min_x * scale, min_y: self.bbox.min_y * scale, min_z: self.bbox.min_z * scale, max_x: self.bbox.max_x * scale, max_y: self.bbox.max_y * scale, max_z: self.bbox.max_z * scale }
    }

    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }
}

/// 📈️ Characteristic curve point.
#[derive(Clone, Copy, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CurvePoint {
    pub x: f64,
    pub y: f64,
}

/// 📉️ Characteristic curve with linear interpolation.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CharacteristicCurve {
    pub id: String,
    pub x_unit: VdiUnit,
    pub y_unit: VdiUnit,
    #[dsl(table)]
    pub points: Vec<CurvePoint>,
}

impl CharacteristicCurve {
    pub fn interpolate(&self, x: f64) -> f64 {
        if self.points.is_empty() {
            return 0.0;
        }
        if x <= self.points[0].x {
            return self.points[0].y;
        }
        let last = self.points.len() - 1;
        if x >= self.points[last].x {
            return self.points[last].y;
        }
        for w in self.points.windows(2) {
            if x >= w[0].x && x <= w[1].x {
                let t = if (w[1].x - w[0].x).abs() < f64::EPSILON { 0.0 } else { (x - w[0].x) / (w[1].x - w[0].x) };
                return w[0].y + t * (w[1].y - w[0].y);
            }
        }
        self.points[last].y
    }
}

/// 🔍️ Product index entry.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CatalogIndexEntry {
    pub product_id: String,
    pub sheet: SheetId,
    pub tags: Vec<String>,
    pub dn: Option<u16>,
}

/// 📚️ Searchable catalogue index.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CatalogIndex {
    #[dsl(table)]
    pub entries: Vec<CatalogIndexEntry>,
}

impl CatalogIndex {
    pub fn from_catalog(catalog: &ManufacturerCatalog) -> Self {
        let mut entries: Vec<CatalogIndexEntry> = catalog
            .products
            .iter()
            .map(|p| CatalogIndexEntry {
                product_id: p.identity.article_number.clone(),
                sheet: p.sheet,
                tags: p.title.iter().map(|t| t.text.clone()).collect(),
                dn: p.configuration.attributes.dn(),
            })
            .collect();
        entries.sort_by(|a, b| a.product_id.cmp(&b.product_id));
        Self { entries }
    }

    pub fn filter_by_sheet(&self, sheet: SheetId) -> Vec<&CatalogIndexEntry> {
        self.entries.iter().filter(|e| e.sheet == sheet).collect()
    }

    pub fn filter_by_dn(&self, dn: u16) -> Vec<&CatalogIndexEntry> {
        self.entries.iter().filter(|e| e.dn == Some(dn)).collect()
    }

    pub fn filter_by_tag(&self, tag: &str) -> Vec<&CatalogIndexEntry> {
        let lower = tag.to_lowercase();
        self.entries.iter().filter(|e| e.tags.iter().any(|t| t.to_lowercase().contains(&lower))).collect()
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
