//! 📍️ Layout: where every entity stands. The documented default rule (golden-tested): catalogue products stand side by side along `+X` with a 0.4 m gap on the front line `y = 0`, their size taken from the referenced parametric geometry, else from the sheet attributes; parametric geometries stand as ghost bodies in the row `y = GEOMETRY_ROW` in the same way.

use super::kit::v3;
use super::model::{CatalogueProduct, ParametricGeometry, SheetAttributes, Vdi3805Snapshot};
use super::scene::Vec3;
use std::collections::BTreeMap;

pub const GAP: f64 = 0.4;
pub const GEOMETRY_ROW: f64 = 3.0;
pub const MIN_EDGE: f64 = 0.02;
pub const MAX_EDGE: f64 = 5.0;

fn edge(value: f64) -> f64 {
    if value.is_finite() { value.clamp(MIN_EDGE, MAX_EDGE) } else { MIN_EDGE }
}

/// 📏️ The drawn size of a parametric geometry: its scaled bounding box.
pub fn geometry_size(geometry: &ParametricGeometry) -> Vec3 {
    let b = geometry.evaluate_bbox();
    v3(edge(b.max_x - b.min_x), edge(b.max_y - b.min_y), edge(b.max_z - b.min_z))
}

/// 📏️ The drawn size of a product: the referenced geometry, else a body derived from its sheet attributes (radiator extent, valve and pump nominal diameter, generator output).
pub fn product_size(document: &Vdi3805Snapshot, product: &CatalogueProduct) -> Vec3 {
    if let Some(geometry) = product.configuration.geometry_ref.as_ref().and_then(|key| document.geometry.get(key)) {
        return geometry_size(geometry);
    }
    match &product.configuration.attributes {
        SheetAttributes::Radiator(a) => v3(edge(a.length_m), edge(a.depth_m), edge(a.height_m)),
        SheetAttributes::ValveHeating(a) => {
            let dn = f64::from(a.dn) / 1000.0;
            v3(edge(dn * 3.0), edge(dn * 1.6), edge(dn * 2.4))
        }
        SheetAttributes::PumpHeating(a) => {
            let dn = f64::from(a.dn_discharge) / 1000.0;
            v3(edge(dn * 4.0), edge(dn * 3.0), edge(dn * 3.0))
        }
        SheetAttributes::HeatGenerator(a) => {
            let s = (a.nominal_heat_output_w / 1000.0).max(1.0).cbrt() * 0.4;
            v3(edge(s * 1.2), edge(s), edge(s * 1.6))
        }
        SheetAttributes::Generic(_) => v3(0.3, 0.2, 0.2),
    }
}

/// 📍️ One resolved pose: the front-left-bottom corner and the size of the body.
#[derive(Clone, Debug, PartialEq)]
pub struct Pose {
    pub min: Vec3,
    pub size: Vec3,
}

impl Pose {
    /// 🎯️ The centre of the base: the anchor of boxes and cylinders.
    pub fn base_centre(&self) -> Vec3 {
        v3(self.min.x + self.size.x / 2.0, self.min.y + self.size.y / 2.0, self.min.z)
    }
}

/// 📍️ All resolved poses of one projection.
#[derive(Clone, Debug, Default)]
pub struct Layout {
    poses: BTreeMap<String, Pose>,
}

impl Layout {
    pub fn of(document: &Vdi3805Snapshot) -> Self {
        let mut layout = Self::default();
        let mut cursor = 0.0;
        for product in &document.catalog.products {
            let size = product_size(document, product);
            layout.poses.insert(product.id.clone(), Pose { min: v3(cursor, 0.0, 0.0), size: size.clone() });
            cursor += size.x + GAP;
        }
        cursor = 0.0;
        for (key, geometry) in &document.geometry {
            let size = geometry_size(geometry);
            layout.poses.insert(key.clone(), Pose { min: v3(cursor, GEOMETRY_ROW, 0.0), size: size.clone() });
            cursor += size.x + GAP;
        }
        layout
    }

    pub fn pose(&self, id: &str) -> Option<&Pose> {
        self.poses.get(id)
    }

    pub fn poses(&self) -> impl Iterator<Item = (&String, &Pose)> {
        self.poses.iter()
    }
}
