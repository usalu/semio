//! 📚️ Catalog model of the VDI 3805 snapshot: the configuration, the catalogue product, the manufacturer catalog, the geometry and characteristic curve records and the catalog index. See [`super`].

use super::localized_text_and_units::VdiUnit;
use super::product_identity_and_links::{AccessoryLink, CompositionLink, ExtensionBag, ManufacturerFile, ProductIdentity};
use super::sheet_attributes::{NativeRecord, SheetAttributes};
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
