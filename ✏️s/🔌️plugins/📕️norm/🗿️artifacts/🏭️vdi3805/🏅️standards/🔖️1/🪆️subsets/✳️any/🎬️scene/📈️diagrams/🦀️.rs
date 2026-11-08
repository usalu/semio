//! 📈️ Diagrams: the characteristic curves of a product (pump Q–H, valve kv, any catalogue curve) with draggable points and the nominal operating point, the heat output of a radiator over the excess temperature (EN 442, `Φ = Φ_N·(ΔT/ΔT_N)^n`), the pressure drop of a control valve over the flow (`Δp = (Q/kvs)²`) and the utilisation of every catalogue product. Every diagram has a table twin; markers bind to checks and to document leaves.

use super::kit::{binding, tone_of};
use super::model::{CatalogueProduct, CharacteristicCurve, SheetAttributes, Vdi3805Snapshot};
use super::norm::{CheckReport, UnitFamily, UnitId};
use super::scene::projection_kit::diagram_parts::{checked_marker_id, column, stroke};
use super::scene::projection_kit::diagram_parts;
use super::scene::*;

/// 🌡️ The logarithmic mean excess temperature of the EN 442 standard condition 75/65/20 °C in kelvin.
pub const DELTA_T_NOMINAL_K: f64 = 49.83;
/// 📈️ The first and last excess temperature drawn on a radiator output curve in kelvin.
pub const DELTA_T_RANGE_K: (f64, f64) = (10.0, 80.0);
/// 📈️ The number of samples of a computed curve.
pub const SAMPLES: usize = 29;

fn loc(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

fn p(x: f64, y: f64) -> Vec2 {
    Vec2::new(x, y)
}

fn axis(en: &str, de: &str, quantity: QuantityKind, unit: &str) -> Axis {
    diagram_parts::axis(en, de, quantity, unit, AxisScale::Linear, None)
}

/// 📐️ The canonical symbol of `symbol` in the shared unit vocabulary, `1` for an unknown or empty one.
pub fn canonical_unit(symbol: &str) -> String {
    UnitId::lookup(symbol).map_or_else(|| "1".to_owned(), |id| id.symbol().to_owned())
}

fn kind_of_unit(unit: &str) -> QuantityKind {
    match UnitId::lookup(unit).map(|id| id.family()) {
        Some(UnitFamily::Length) => QuantityKind::Length,
        Some(UnitFamily::Pressure) => QuantityKind::Pressure,
        Some(UnitFamily::Temperature) => QuantityKind::Temperature,
        Some(UnitFamily::Area) => QuantityKind::Area,
        Some(UnitFamily::Volume) => QuantityKind::Volume,
        Some(UnitFamily::Time) => QuantityKind::Time,
        Some(UnitFamily::Mass) => QuantityKind::Mass,
        Some(UnitFamily::Force) => QuantityKind::Force,
        Some(UnitFamily::Energy) => QuantityKind::Energy,
        _ => QuantityKind::Dimensionless,
    }
}

fn focus_of(product: &CatalogueProduct) -> EntityRef {
    EntityRef::new(EntityKind::Solid, product.id.clone(), format!("catalog.products[id={}]", product.id))
}

/// 🔗️ The characteristic curves of a product that the document holds: its function references, then the head curve of a pump, without repeats.
pub fn curve_ids(document: &Vdi3805Snapshot, product: &CatalogueProduct) -> Vec<String> {
    let mut ids: Vec<String> = product.configuration.function_refs.clone();
    if let SheetAttributes::PumpHeating(a) = &product.configuration.attributes {
        ids.extend(a.qh_curve_ref.clone());
    }
    let mut seen = std::collections::BTreeSet::new();
    ids.retain(|id| document.curves.contains_key(id) && seen.insert(id.clone()));
    ids
}

fn curve_diagram(document: &Vdi3805Snapshot, report: &CheckReport, product: &CatalogueProduct, id: &str, curve: &CharacteristicCurve) -> Diagram2d {
    let tone = tone_of(&binding(document, report, &[&product.id]));
    let (xu, yu) = (canonical_unit(&curve.x_unit.symbol), canonical_unit(&curve.y_unit.symbol));
    let points: Vec<Vec2> = curve.points.iter().map(|q| p(q.x, q.y)).collect();
    let editable = xu != yu;
    let mut markers: Vec<Marker> = curve
        .points
        .iter()
        .enumerate()
        .map(|(i, q)| Marker {
            id: checked_marker_id(&format!("{id}:{i}"), &None),
            at: p(q.x, q.y),
            shape: MarkerShape::Circle,
            tone,
            label: loc(&format!("Point {} of curve {id}", i + 1), &format!("Punkt {} der Kennlinie {id}", i + 1)),
            utilisation: None,
            bound_check: None,
            binding: editable.then(|| FieldBinding { path: format!("curves[id={id}].points[{i}].y"), map: Affine { base: 0.0, k: 1.0 }, unit: yu.clone(), quantise: None }),
        })
        .collect();
    if let SheetAttributes::PumpHeating(a) = &product.configuration.attributes {
        if a.qh_curve_ref.as_deref() == Some(id) {
            let (fx, fy) = (curve.x_unit.si_factor, curve.y_unit.si_factor);
            let q = a.nominal_flow_m3_s / if fx > 0.0 { fx } else { 1.0 };
            let h = a.nominal_head_m / if fy > 0.0 { fy } else { 1.0 };
            markers.push(Marker {
                id: format!("operating:{}", product.id),
                at: p(q, h),
                shape: MarkerShape::Diamond,
                tone,
                label: loc("Nominal operating point", "Nennbetriebspunkt"),
                utilisation: None,
                bound_check: Some(format!("vdi3805.5.qh.{}", product.id)),
                binding: None,
            });
        }
    }
    Diagram2d {
        id: format!("curve:{}:{id}", product.id),
        title: loc(&format!("Characteristic curve {id} of {}", product.id), &format!("Kennlinie {id} von {}", product.id)),
        description: loc("The catalogue characteristic curve with linear interpolation between its points; dragging a point changes its value.", "Die Katalog-Kennlinie mit linearer Interpolation zwischen ihren Punkten; das Ziehen eines Punkts ändert seinen Wert."),
        axes: vec![axis("Curve argument", "Kennlinienargument", kind_of_unit(&xu), &xu), axis("Curve value", "Kennlinienwert", kind_of_unit(&yu), &yu)],
        series: vec![Series::Polyline { id: "curve".into(), label: loc("Characteristic curve", "Kennlinie"), points, style: stroke(tone, 2.0, Dash::Solid) }],
        markers,
        annotations: Vec::new(),
        focus: Some(focus_of(product)),
        legend: false,
        table_twin: DataTable { columns: vec![column("Curve argument", "Kennlinienargument", &xu, curve.points.iter().map(|q| q.x).collect()), column("Curve value", "Kennlinienwert", &yu, curve.points.iter().map(|q| q.y).collect())] },
    }
}

/// 📐️ The radiator output `Φ = Φ_N·(ΔT/ΔT_N)^n` in watt at the excess temperature `delta_t` in kelvin (EN 442).
pub fn radiator_output_w(standard_output_w: f64, exponent: f64, delta_t_k: f64) -> f64 {
    standard_output_w * (delta_t_k / DELTA_T_NOMINAL_K).powf(exponent)
}

fn radiator_diagram(document: &Vdi3805Snapshot, report: &CheckReport, product: &CatalogueProduct) -> Option<Diagram2d> {
    let SheetAttributes::Radiator(a) = &product.configuration.attributes else { return None };
    let tone = tone_of(&binding(document, report, &[&product.id]));
    let (lo, hi) = DELTA_T_RANGE_K;
    let dt: Vec<f64> = (0..SAMPLES).map(|i| lo + (hi - lo) * i as f64 / (SAMPLES - 1) as f64).collect();
    let phi: Vec<f64> = dt.iter().map(|t| radiator_output_w(a.standard_output_w, a.heat_exponent_n, *t)).collect();
    let check = format!("vdi3805.3.phi.{}", product.id);
    Some(Diagram2d {
        id: format!("output:{}", product.id),
        title: loc(&format!("Heat output of {} over the excess temperature", product.id), &format!("Wärmeleistung von {} über der Übertemperatur", product.id)),
        description: loc("Heat output Φ = Φ_N·(ΔT/ΔT_N)^n of the radiator (EN 442); the marker is the standard output at 75/65/20 °C.", "Wärmeleistung Φ = Φ_N·(ΔT/ΔT_N)^n des Heizkörpers (EN 442); die Markierung ist die Normleistung bei 75/65/20 °C."),
        axes: vec![axis("Excess temperature ΔT", "Übertemperatur ΔT", QuantityKind::Temperature, "K"), axis("Heat output Φ", "Wärmeleistung Φ", QuantityKind::Dimensionless, "W")],
        series: vec![Series::Polyline { id: "output".into(), label: loc("Heat output", "Wärmeleistung"), points: dt.iter().zip(&phi).map(|(x, y)| p(*x, *y)).collect(), style: stroke(tone, 2.0, Dash::Solid) }],
        markers: vec![Marker { id: format!("standard:{}", product.id), at: p(DELTA_T_NOMINAL_K, a.standard_output_w), shape: MarkerShape::Diamond, tone, label: loc("Standard output at 75/65/20 °C", "Normleistung bei 75/65/20 °C"), utilisation: None, bound_check: Some(check), binding: None }],
        annotations: Vec::new(),
        focus: Some(focus_of(product)),
        legend: false,
        table_twin: DataTable { columns: vec![column("Excess temperature", "Übertemperatur", "K", dt), column("Heat output", "Wärmeleistung", "W", phi)] },
    })
}

/// 📐️ The pressure drop `Δp = (Q/kvs)²` in bar of a valve at the flow `flow_m3_h` (VDI 2173, DIN EN 60534).
pub fn valve_pressure_drop_bar(kvs_m3_h: f64, flow_m3_h: f64) -> f64 {
    if kvs_m3_h > 0.0 { (flow_m3_h / kvs_m3_h).powi(2) } else { 0.0 }
}

fn valve_diagram(document: &Vdi3805Snapshot, report: &CheckReport, product: &CatalogueProduct) -> Option<Diagram2d> {
    let SheetAttributes::ValveHeating(a) = &product.configuration.attributes else { return None };
    let kvs = a.kvs_m3_h();
    if kvs <= 0.0 {
        return None;
    }
    let tone = tone_of(&binding(document, report, &[&product.id]));
    let q: Vec<f64> = (0..SAMPLES).map(|i| 1.2 * kvs * i as f64 / (SAMPLES - 1) as f64).collect();
    let dp: Vec<f64> = q.iter().map(|x| valve_pressure_drop_bar(kvs, *x)).collect();
    Some(Diagram2d {
        id: format!("drop:{}", product.id),
        title: loc(&format!("Pressure drop of {} over the flow", product.id), &format!("Druckverlust von {} über dem Volumenstrom", product.id)),
        description: loc("Pressure drop Δp = (Q/kvs)² of the fully open valve; the marker is the flow at 1 bar, the kvs value.", "Druckverlust Δp = (Q/kvs)² des voll geöffneten Ventils; die Markierung ist der Volumenstrom bei 1 bar, der kvs-Wert."),
        axes: vec![axis("Flow Q", "Volumenstrom Q", QuantityKind::Dimensionless, "m³/h"), axis("Pressure drop Δp", "Druckverlust Δp", QuantityKind::Pressure, "bar")],
        series: vec![Series::Polyline { id: "drop".into(), label: loc("Pressure drop", "Druckverlust"), points: q.iter().zip(&dp).map(|(x, y)| p(*x, *y)).collect(), style: stroke(tone, 2.0, Dash::Solid) }],
        markers: vec![Marker { id: format!("kvs:{}", product.id), at: p(kvs, 1.0), shape: MarkerShape::Diamond, tone, label: loc("kvs value at 1 bar", "kvs-Wert bei 1 bar"), utilisation: None, bound_check: Some(format!("vdi3805.2.kvs.{}", product.id)), binding: None }],
        annotations: Vec::new(),
        focus: Some(focus_of(product)),
        legend: false,
        table_twin: DataTable { columns: vec![column("Flow", "Volumenstrom", "m³/h", q), column("Pressure drop", "Druckverlust", "bar", dp)] },
    })
}

fn catalogue_diagram(document: &Vdi3805Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let rows: Vec<(&CatalogueProduct, Option<UtilisationBinding>)> = document.catalog.products.iter().map(|product| (product, binding(document, report, &[&product.id]))).collect();
    if rows.is_empty() {
        return None;
    }
    let mut series = Vec::new();
    let mut markers = Vec::new();
    let mut ratios = Vec::new();
    for (index, (product, found)) in rows.iter().enumerate() {
        let x = index as f64;
        let ratio = found.as_ref().map_or(0.0, |b| b.ratio).max(0.0);
        let ratio = if ratio.is_finite() { ratio } else { 0.0 };
        ratios.push(ratio);
        series.push(Series::Polygon { id: format!("bar:{}", product.id), label: loc(&format!("Utilisation of {}: {ratio:.2}", product.id), &format!("Ausnutzung von {}: {ratio:.2}", product.id)), ring: vec![p(x + 0.1, 0.0), p(x + 0.9, 0.0), p(x + 0.9, ratio), p(x + 0.1, ratio)], fill: Fill { tone: tone_of(found), opacity: 0.6 }, hatch: None });
        let check = found.as_ref().map(|b| b.check_id.clone());
        markers.push(Marker { id: checked_marker_id(&format!("pick:{}", product.id), &check), at: p(x + 0.5, ratio), shape: MarkerShape::Circle, tone: tone_of(found), label: loc(&format!("Select {}", product.id), &format!("{} wählen", product.id)), utilisation: Some(ratio), bound_check: check, binding: None });
    }
    let index: Vec<f64> = (0..rows.len()).map(|i| i as f64).collect();
    Some(Diagram2d {
        id: "utilisation:catalogue".into(),
        title: loc("Utilisation of the catalogue products", "Ausnutzung der Katalogprodukte"),
        description: loc("Governing utilisation of every catalogue product in the order of the outliner; each bar is coloured by its governing check.", "Maßgebende Ausnutzung jedes Katalogprodukts in der Reihenfolge der Gliederung; jeder Balken ist nach seinem maßgebenden Nachweis gefärbt."),
        axes: vec![axis("Product", "Produkt", QuantityKind::Dimensionless, "1"), axis("Utilisation", "Ausnutzung", QuantityKind::Dimensionless, "1")],
        series,
        markers,
        annotations: Vec::new(),
        focus: None,
        legend: false,
        table_twin: DataTable { columns: vec![column("Product index", "Produktnummer", "1", index), column("Utilisation", "Ausnutzung", "1", ratios)] },
    })
}

/// 📈️ Every diagram of the document; `focus` set to an unknown entity yields none, to a product its own curves and the catalogue bars.
pub fn diagrams(document: &Vdi3805Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    if focus.is_some_and(|f| super::kit::entity_kind_of(document, f).is_none()) {
        return Vec::new();
    }
    let mut out: Vec<Diagram2d> = Vec::new();
    for product in document.catalog.products.iter().filter(|p| focus.is_none_or(|f| f == p.id)) {
        for id in curve_ids(document, product) {
            if let Some(curve) = document.curves.get(&id) {
                out.push(curve_diagram(document, report, product, &id, curve));
            }
        }
        out.extend(radiator_diagram(document, report, product));
        out.extend(valve_diagram(document, report, product));
    }
    out.extend(catalogue_diagram(document, report));
    super::scene::projection::with_basis(out, &super::scene::projection::Basis::new(super::norm::NormFamilyId::Vdi3805, super::checks::ANNEX))
}
