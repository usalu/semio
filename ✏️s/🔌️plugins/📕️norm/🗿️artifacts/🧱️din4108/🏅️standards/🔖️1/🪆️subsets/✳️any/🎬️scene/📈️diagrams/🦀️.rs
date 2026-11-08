//! 📈️ Diagrams: the layer-section temperature profile and the Glaser vapour-pressure profile of every envelope element (DIN 4108-3, EN ISO 6946, EN ISO 13788) and the U-value bars of the building (DIN 4108-2 and -6). Every diagram has a table twin; markers bind to checks and to document leaves.

use super::checks::{self, part_3};
use super::model::{Din4108Snapshot, EnvelopeElement};
use super::norm::{CheckReport, CheckStatus};
use super::scene::projection_kit::diagram_parts::{column, stroke};
use super::scene::projection_kit::diagram_parts;
use super::scene::*;

const FREEZING_C: f64 = 0.0;

fn loc(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

fn axis(en: &str, de: &str, quantity: QuantityKind, unit: &str) -> Axis {
    diagram_parts::axis(en, de, quantity, unit, AxisScale::Linear, None)
}

fn p(x: f64, y: f64) -> Vec2 {
    Vec2::new(x, y)
}

fn check_tone(report: &CheckReport, id: &str) -> Tone {
    report.checks.iter().find(|c| c.id == id).map_or(Tone::Neutral, |c| match c.status {
        CheckStatus::Fail => Tone::Error,
        CheckStatus::Warning | CheckStatus::UnverifiedBasis => Tone::Warning,
        CheckStatus::Pass | CheckStatus::Info => Tone::Info,
        CheckStatus::NotApplicable => Tone::Neutral,
    })
}

fn focus_of(element: &EnvelopeElement) -> EntityRef {
    EntityRef::new(EntityKind::Solid, element.id.clone(), format!("elements[id={}]", element.id))
}

/// 📍️ The cumulative depth of the layer interfaces from the interior surface in metres, `layers + 1` values starting at zero.
pub fn depths(element: &EnvelopeElement) -> Vec<f64> {
    let mut depth = vec![0.0];
    for layer in &element.layers {
        depth.push(depth.last().copied().unwrap_or(0.0) + super::elements::layer_thickness(layer));
    }
    depth
}

/// 🌡️ The winter boundary and the profile of an element: the single source shared with the Glaser check.
pub fn winter_profile(document: &Din4108Snapshot, element: &EnvelopeElement) -> part_3::VapourProfile {
    let (r_si, r_se) = checks::surface_resistances_inclined(&element.kind, &element.adjacent, element.inclination_deg);
    let boundary = part_3::glaser_boundary(element, document.climate_zone);
    let t_int = if document.t_int_c > 0.0 { document.t_int_c } else { checks::GLASER_INTERIOR_T_C };
    let rh_int = if document.rh_int > 0.0 { document.rh_int } else { checks::GLASER_INTERIOR_RH };
    part_3::vapour_profile(&element.layers, r_si, r_se, t_int, rh_int, boundary.t_ext_w, boundary.rh_ext_w)
}

fn interface_marker(element: &EnvelopeElement, depths: &[f64], at: usize, y: f64, check: &str, report: &CheckReport) -> Option<Marker> {
    let layer = element.layers.get(at.checked_sub(1)?)?;
    Some(Marker {
        id: format!("interface:{}:{}", element.id, layer.id),
        at: p(depths[at], y),
        shape: MarkerShape::Diamond,
        tone: check_tone(report, check),
        label: loc(&format!("Outer face of layer {}", layer.id), &format!("Außenseite der Schicht {}", layer.id)),
        utilisation: None,
        bound_check: Some(check.to_owned()),
        binding: Some(projection_kit_binding(element, &layer.id, depths[at - 1])),
    })
}

fn projection_kit_binding(element: &EnvelopeElement, layer: &str, before: f64) -> FieldBinding {
    FieldBinding { path: format!("elements[id={}].layers[id={layer}].thicknessM", element.id), map: Affine { base: -before, k: 1.0 }, unit: "m".into(), quantise: Some(Quantise::Round { step: 0.001 }) }
}

fn temperature(document: &Din4108Snapshot, report: &CheckReport, element: &EnvelopeElement) -> Option<Diagram2d> {
    if element.layers.is_empty() {
        return None;
    }
    let depth = depths(element);
    let profile = winter_profile(document, element);
    let check = format!("din4108-6.u.{}", element.id);
    let points: Vec<Vec2> = depth.iter().zip(&profile.t_c).map(|(x, t)| p(*x, *t)).collect();
    let markers = (1..depth.len()).filter_map(|i| interface_marker(element, &depth, i, profile.t_c[i], &check, report)).collect();
    Some(Diagram2d {
        id: format!("temperature:{}", element.id),
        title: loc(&format!("Temperature profile of {}", element.id), &format!("Temperaturverlauf von {}", element.id)),
        description: loc("Winter temperature at the layer interfaces from the interior to the exterior surface; dragging an interface changes the layer thickness.", "Wintertemperatur an den Schichtgrenzen von der Innen- zur Außenoberfläche; das Ziehen einer Schichtgrenze ändert die Schichtdicke."),
        axes: vec![axis("Depth from the interior surface", "Tiefe ab der Rauminnenoberfläche", QuantityKind::Length, "m"), axis("Temperature", "Temperatur", QuantityKind::Temperature, "°C")],
        series: vec![
            Series::Polyline { id: "temperature".into(), label: loc("Temperature", "Temperatur"), points, style: stroke(check_tone(report, &check), 2.0, Dash::Solid) },
            Series::Limit { id: "freezing".into(), label: loc("Freezing point", "Gefrierpunkt"), axis: AxisRef::Y, value: FREEZING_C, style: stroke(Tone::Info, 1.0, Dash::Dashed) },
        ],
        markers,
        annotations: Vec::new(),
        focus: Some(focus_of(element)),
        legend: true,
        table_twin: DataTable { columns: vec![column("Depth", "Tiefe", "m", depth), column("Temperature", "Temperatur", "°C", profile.t_c)] },
    })
}

fn glaser(document: &Din4108Snapshot, report: &CheckReport, element: &EnvelopeElement) -> Option<Diagram2d> {
    if element.layers.is_empty() || matches!(element.kind.as_str(), "window" | "door") {
        return None;
    }
    let profile = winter_profile(document, element);
    let check = format!("din4108-3.glaser.{}", element.id);
    let vapour: Vec<Vec2> = profile.sd_m.iter().zip(&profile.p_pa).map(|(x, y)| p(*x, *y)).collect();
    let saturation: Vec<Vec2> = profile.sd_m.iter().zip(&profile.p_sat_pa).map(|(x, y)| p(*x, *y)).collect();
    let markers = (0..profile.sd_m.len())
        .filter(|i| profile.p_pa[*i] > profile.p_sat_pa[*i])
        .map(|i| Marker {
            id: format!("condensation:{}:{i}", element.id),
            at: p(profile.sd_m[i], profile.p_pa[i]),
            shape: MarkerShape::Cross,
            tone: Tone::Error,
            label: loc(&format!("Condensation at interface {i}: {:.0} Pa above saturation", profile.p_pa[i] - profile.p_sat_pa[i]), &format!("Tauwasser an Schichtgrenze {i}: {:.0} Pa über Sättigung", profile.p_pa[i] - profile.p_sat_pa[i])),
            utilisation: None,
            bound_check: Some(check.clone()),
            binding: None,
        })
        .collect();
    Some(Diagram2d {
        id: format!("glaser:{}", element.id),
        title: loc(&format!("Glaser vapour-pressure profile of {}", element.id), &format!("Glaser-Dampfdruckverlauf von {}", element.id)),
        description: loc("Vapour pressure against the saturation pressure over the water-vapour diffusion-equivalent air layer thickness; condensation occurs where the vapour pressure exceeds saturation.", "Dampfdruck gegen Sättigungsdampfdruck über der wasserdampfdiffusionsäquivalenten Luftschichtdicke; Tauwasser fällt aus, wo der Dampfdruck die Sättigung übersteigt."),
        axes: vec![axis("Equivalent air layer thickness s_d", "Wasserdampfdiffusionsäquivalente Luftschichtdicke s_d", QuantityKind::Length, "m"), axis("Vapour pressure", "Dampfdruck", QuantityKind::Pressure, "Pa")],
        series: vec![
            Series::Polyline { id: "vapour".into(), label: loc("Vapour pressure", "Dampfdruck"), points: vapour, style: stroke(check_tone(report, &check), 2.0, Dash::Solid) },
            Series::Polyline { id: "saturation".into(), label: loc("Saturation pressure", "Sättigungsdampfdruck"), points: saturation, style: stroke(Tone::Info, 2.0, Dash::Dashed) },
        ],
        markers,
        annotations: Vec::new(),
        focus: Some(focus_of(element)),
        legend: true,
        table_twin: DataTable { columns: vec![column("Equivalent air layer thickness", "Äquivalente Luftschichtdicke", "m", profile.sd_m), column("Vapour pressure", "Dampfdruck", "Pa", profile.p_pa), column("Saturation pressure", "Sättigungsdampfdruck", "Pa", profile.p_sat_pa)] },
    })
}

fn u_values(document: &Din4108Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let rows: Vec<(&EnvelopeElement, f64)> = document.elements.iter().filter_map(|e| super::elements::u_value_of(e).map(|u| (e, u))).collect();
    if rows.is_empty() {
        return None;
    }
    let mut series = Vec::new();
    let mut markers = Vec::new();
    for (index, (element, u)) in rows.iter().enumerate() {
        let check = format!("din4108-6.u.{}", element.id);
        let x = index as f64;
        series.push(Series::Polygon { id: format!("bar:{}", element.id), label: loc(&format!("U-value of {}: {u:.3} W/(m²·K)", element.id), &format!("U-Wert von {}: {u:.3} W/(m²·K)", element.id)), ring: vec![p(x + 0.1, 0.0), p(x + 0.9, 0.0), p(x + 0.9, *u), p(x + 0.1, *u)], fill: Fill { tone: check_tone(report, &check), opacity: 0.6 }, hatch: None });
        let found = report.checks.iter().find(|c| c.id == check);
        markers.push(Marker { id: format!("pick:{}", element.id), at: p(x + 0.5, *u), shape: MarkerShape::Circle, tone: check_tone(report, &check), label: loc(&format!("Select {}", element.id), &format!("{} wählen", element.id)), utilisation: found.map(|c| c.utilization.max(0.0)).filter(|v| v.is_finite()), bound_check: Some(check), binding: None });
    }
    let (index, values): (Vec<f64>, Vec<f64>) = rows.iter().enumerate().map(|(i, (_, u))| (i as f64, *u)).unzip();
    Some(Diagram2d {
        id: "u-values:building".into(),
        title: loc("U-values of the envelope elements", "U-Werte der Hüllflächenbauteile"),
        description: loc("Heat transfer coefficient of every envelope element in the order of the outliner; each bar is coloured by its DIN 4108-6 check.", "Wärmedurchgangskoeffizient jedes Hüllflächenbauteils in der Reihenfolge der Gliederung; jeder Balken ist nach seinem Nachweis nach DIN 4108-6 gefärbt."),
        axes: vec![axis("Element", "Bauteil", QuantityKind::Dimensionless, "1"), axis("U-value", "U-Wert", QuantityKind::HeatTransferCoefficient, "W/(m²·K)")],
        series,
        markers,
        annotations: Vec::new(),
        focus: None,
        legend: false,
        table_twin: DataTable { columns: vec![column("Element index", "Bauteilnummer", "1", index), column("U-value", "U-Wert", "W/(m²·K)", values)] },
    })
}

/// 📈️ Every diagram of the document; `focus` set to an unknown entity yields none, to an element only its own profiles and the building bars.
pub fn diagrams(document: &Din4108Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    if focus.is_some_and(|f| super::kit::entity_kind_of(document, f).is_none()) {
        return Vec::new();
    }
    let mut out: Vec<Diagram2d> = Vec::new();
    for element in document.elements.iter().filter(|e| focus.is_none_or(|f| f == e.id)) {
        out.extend(temperature(document, report, element));
        out.extend(glaser(document, report, element));
    }
    out.extend(u_values(document, report));
    super::scene::projection::with_basis(out, &super::scene::projection::Basis::new(super::norm::NormFamilyId::Din4108, super::norm::AnnexChoice::De))
}
