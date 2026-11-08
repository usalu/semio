//! 🧱️ Products: every catalogue product as the body its sheet describes (radiator panel, valve body, pump cylinder, generator cabinet) at the size of its parametric geometry, and every parametric geometry as a ghost body with its labelled connection points.

use super::kit::*;
use super::layout::{Layout, Pose};
use super::scene::*;
use super::{handles, model, SceneInput};

const LABEL_LIFT: f64 = 0.15;
const GHOST_OPACITY: f64 = 0.35;
const PORT_LABEL_LIFT: f64 = 0.05;

fn frame_at(origin: Vec3) -> Frame {
    Frame { origin, x: Vec3::X, y: Vec3::Y, z: Vec3::Z }
}

/// 🎨️ The material role of a product: its sheet decides the colour family.
pub fn role_of(product: &model::CatalogueProduct) -> MaterialRole {
    match &product.configuration.attributes {
        model::SheetAttributes::ValveHeating(_) | model::SheetAttributes::PumpHeating(_) => MaterialRole::Steel,
        model::SheetAttributes::HeatGenerator(_) => MaterialRole::Masonry,
        model::SheetAttributes::Radiator(_) | model::SheetAttributes::Generic(_) => MaterialRole::Neutral,
    }
}

fn name_of(product: &model::CatalogueProduct) -> LocalizedCopy {
    let en = model::text_in(&product.title, "en");
    let de = model::text_in(&product.title, "de");
    let article = &product.identity.article_number;
    loc(&format!("{} ({article})", if en.is_empty() { product.id.as_str() } else { en.as_str() }), &format!("{} ({article})", if de.is_empty() { product.id.as_str() } else { de.as_str() }))
}

/// 📝️ The one-line technical summary of a product, with the sheet figures that matter for its trade.
pub fn detail_of(product: &model::CatalogueProduct) -> LocalizedCopy {
    let sheet = product.sheet.0;
    match &product.configuration.attributes {
        model::SheetAttributes::ValveHeating(a) => loc(&format!("Sheet {sheet}: control valve DN {}, kvs {:.2} m³/h, {}", a.dn, a.kvs_m3_h(), a.pressure_class), &format!("Blatt {sheet}: Stellventil DN {}, kvs {:.2} m³/h, {}", a.dn, a.kvs_m3_h(), a.pressure_class)),
        model::SheetAttributes::Radiator(a) => loc(&format!("Sheet {sheet}: radiator {:.0} W at 75/65/20 °C, exponent n = {:.2}", a.standard_output_w, a.heat_exponent_n), &format!("Blatt {sheet}: Heizkörper {:.0} W bei 75/65/20 °C, Exponent n = {:.2}", a.standard_output_w, a.heat_exponent_n)),
        model::SheetAttributes::PumpHeating(a) => loc(&format!("Sheet {sheet}: pump DN {}, {:.2} m³/h at {:.1} m head, {:.0} W motor", a.dn_discharge, a.nominal_flow_m3_s * 3600.0, a.nominal_head_m, a.motor_power_w), &format!("Blatt {sheet}: Pumpe DN {}, {:.2} m³/h bei {:.1} m Förderhöhe, {:.0} W Motor", a.dn_discharge, a.nominal_flow_m3_s * 3600.0, a.nominal_head_m, a.motor_power_w)),
        model::SheetAttributes::HeatGenerator(a) => loc(&format!("Sheet {sheet}: heat generator {:.1} kW, {}, flow up to {:.0} °C", a.nominal_heat_output_w / 1000.0, a.fuel_type, a.flow_temp_max_c), &format!("Blatt {sheet}: Wärmeerzeuger {:.1} kW, {}, Vorlauf bis {:.0} °C", a.nominal_heat_output_w / 1000.0, a.fuel_type, a.flow_temp_max_c)),
        model::SheetAttributes::Generic(_) => loc(&format!("Sheet {sheet}: catalogue product"), &format!("Blatt {sheet}: Katalogprodukt")),
    }
}

fn product(input: &SceneInput, layout: &Layout, out: &mut Out, product: &model::CatalogueProduct) {
    let Some(Pose { min, size }) = layout.pose(&product.id).cloned() else { return };
    let pose = Pose { min, size };
    let centre = pose.base_centre();
    let binding = binding(input.document, input.report, &[&product.id]);
    let name = name_of(product);
    let frame = frame_at(centre.clone());
    let shape = match &product.configuration.attributes {
        model::SheetAttributes::PumpHeating(_) => SolidShape::Cylinder { frame, radius: pose.size.x.min(pose.size.y) / 2.0, height: pose.size.z },
        _ => SolidShape::Box { frame, size: pose.size.clone() },
    };
    let top = v3(centre.x, centre.y, pose.size.z);
    out.push(
        Spec {
            kind: EntityKind::Solid,
            id: product.id.clone(),
            collection: super::kit::PRODUCTS,
            geometry: Geometry::Solid(SolidGeometry { shape }),
            role: role_of(product),
            binding: binding.clone(),
            name: name.clone(),
            detail: detail_of(product),
            layer: LAYER_PRODUCTS,
            handles: handles::ids(input.document, layout, &product.id),
            opacity: 1.0,
        },
        &[pose.min.clone(), v3(pose.min.x + pose.size.x, pose.min.y + pose.size.y, pose.size.z)],
    );
    let entity = entity_ref(EntityKind::Solid, &product.id, super::kit::PRODUCTS);
    let label_at = along(&top, &Vec3::Z, LABEL_LIFT);
    out.annotate(Annotation::Label { id: format!("{}:label", product.id), anchor: label_at.clone(), text: name, tone: tone_of(&binding) }, &label_at);
    let from = pose.min.clone();
    let to = v3(pose.min.x + pose.size.x, pose.min.y, 0.0);
    out.annotate(Annotation::Dimension { id: format!("{}:width", product.id), from: from.clone(), to, offset: v3(0.0, -LABEL_LIFT, 0.0), value: Quantity::new(QuantityKind::Length, pose.size.x), binding: None }, &from);
    if let Some(clause) = binding.as_ref().and_then(|b| clause_of(input.report, &b.check_id)) {
        out.annotate(Annotation::Clause { id: format!("{}:clause", product.id), anchor: entity, clause }, &label_at);
    }
}

fn geometry(input: &SceneInput, layout: &Layout, out: &mut Out, key: &str, row: &model::ParametricGeometry) {
    let Some(pose) = layout.pose(key).cloned() else { return };
    let binding = binding(input.document, input.report, &[key]);
    let name = loc(&format!("Geometry {key}"), &format!("Geometrie {key}"));
    let centre = pose.base_centre();
    let scale = row.parameters.get("scale").copied().unwrap_or(1.0);
    out.push(
        Spec {
            kind: EntityKind::Solid,
            id: key.to_owned(),
            collection: super::kit::GEOMETRY,
            geometry: Geometry::Solid(SolidGeometry { shape: SolidShape::Box { frame: frame_at(centre.clone()), size: pose.size.clone() } }),
            role: MaterialRole::Glass,
            binding: binding.clone(),
            name: name.clone(),
            detail: loc(&format!("{:.3} × {:.3} × {:.3} m, {} connections, scale {scale}", pose.size.x, pose.size.y, pose.size.z, row.connections.len()), &format!("{:.3} × {:.3} × {:.3} m, {} Anschlüsse, Maßstab {scale}", pose.size.x, pose.size.y, pose.size.z, row.connections.len())),
            layer: LAYER_GEOMETRY,
            handles: handles::ids(input.document, layout, key),
            opacity: GHOST_OPACITY,
        },
        &[pose.min.clone(), v3(pose.min.x + pose.size.x, pose.min.y + pose.size.y, pose.size.z)],
    );
    let top = v3(centre.x, centre.y, pose.size.z);
    let label_at = along(&top, &Vec3::Z, LABEL_LIFT);
    out.annotate(Annotation::Label { id: format!("{key}:label"), anchor: label_at.clone(), text: name, tone: tone_of(&binding) }, &label_at);
    for connection in &row.connections {
        let at = v3(pose.min.x + (connection.position[0] - row.bbox.min_x) * scale, pose.min.y + (connection.position[1] - row.bbox.min_y) * scale, pose.min.z + (connection.position[2] - row.bbox.min_z) * scale + PORT_LABEL_LIFT);
        let diameter = connection.diameter_mm.map_or_else(String::new, |d| format!(" DN {d:.0}"));
        out.annotate(Annotation::Label { id: format!("{key}:port:{}", connection.id), anchor: at.clone(), text: loc(&format!("{} {}{diameter}", connection.id, connection.medium), &format!("{} {}{diameter}", connection.id, connection.medium)), tone: Tone::Neutral }, &at);
    }
}

pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    for row in &d.catalog.products {
        if out.wants(&row.id) {
            product(input, layout, out, row);
        }
    }
    for (key, row) in &d.geometry {
        if out.wants(key) {
            geometry(input, layout, out, key, row);
        }
    }
}
