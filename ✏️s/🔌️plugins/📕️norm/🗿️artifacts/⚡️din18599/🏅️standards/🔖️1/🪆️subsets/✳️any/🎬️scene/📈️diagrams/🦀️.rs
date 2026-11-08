//! 📈️ Diagrams: the monthly heating and cooling balance of each thermal zone (DIN V 18599-2), the useful-final-primary energy flow of the building (DIN V 18599-1, -5, -8, -10) and the U-value of each envelope element against the GEG reference.

use super::checks::{derive_balance, primary_energy_factor, reference_u, zone_heating_months, HeatingMonth};
use super::kit::{check_with, entity_ref, loc};
use super::model::{Din18599Snapshot, ElementKind, EnvelopeElement};
use super::norm::CheckReport;
use super::scene::diagram_kit::Xy;
use super::scene::projection_kit::diagram_parts::{axis, checked_marker_id, column, stroke};
use super::scene::*;

pub const KWH: &str = "kWh";
pub const U_UNIT: &str = "W/(m²·K)";
const BAR: f64 = 0.4;
const ENERGY_CHECK: &str = "din18599.2.heating-demand";
const PRIMARY_CHECK: &str = "din18599.geg.qp";
const HT_CHECK: &str = "din18599.geg.ht-prime";

fn p(x: f64, y: f64) -> Vec2 {
    Vec2::new(x, y)
}

fn line(id: &str, en: &str, de: &str, points: &[Xy], tone: Tone, dash: Dash) -> Series {
    Series::Polyline { id: id.into(), label: loc(en, de), points: points.iter().map(|q| p(q[0], q[1])).collect(), style: stroke(tone, 2.0, dash) }
}

fn tone_of(report: &CheckReport, id: &str) -> Tone {
    check_with(report, id).map_or(Tone::Neutral, |c| utilisation_tone(c.utilization))
}

fn marker(id: &str, at: Vec2, report: &CheckReport, check: &str, label: LocalizedCopy) -> Marker {
    let found = check_with(report, check);
    let bound = found.map(|c| c.id.clone());
    Marker { id: checked_marker_id(id, &bound), at, shape: MarkerShape::Circle, tone: tone_of(report, check), label, utilisation: found.map(|c| c.utilization), bound_check: bound, binding: None }
}

fn bar(id: String, label: LocalizedCopy, month: usize, lane: f64, value: f64, tone: Tone, hatch: Option<Hatch>) -> Option<Series> {
    let x0 = month as f64 + 1.0 + lane;
    (value > 1e-9).then(|| Series::Polygon { id, label, ring: vec![p(x0, 0.0), p(x0 + BAR, 0.0), p(x0 + BAR, value), p(x0, value)], fill: Fill { tone, opacity: 0.7 }, hatch })
}

fn zone_balance(document: &Din18599Snapshot, report: &CheckReport, id: &str, months: &[HeatingMonth]) -> Diagram2d {
    let zone = document.zones.iter().find(|z| z.id == id);
    let (en, de) = zone.map_or((id.to_string(), id.to_string()), |z| (z.label_en.clone(), z.label_de.clone()));
    let at = |f: fn(&HeatingMonth) -> f64| -> Vec<Xy> { months.iter().enumerate().map(|(m, v)| [m as f64 + 1.0, f(v)]).collect() };
    let loss = at(|m| m.q_t_kwh + m.q_v_kwh);
    let gain = at(|m| m.q_i_kwh + m.q_s_kwh);
    let top = months.iter().map(|m| (m.q_t_kwh + m.q_v_kwh).max(m.q_i_kwh + m.q_s_kwh).max(m.q_h_kwh).max(m.q_c_kwh)).fold(1.0, f64::max) * 1.1;
    let mut series = vec![line("loss", "Heat losses Q_T + Q_V", "Wärmesenken Q_T + Q_V", &loss, Tone::Neutral, Dash::Solid), line("gain", "Heat gains Q_i + Q_s", "Wärmequellen Q_i + Q_s", &gain, Tone::Neutral, Dash::Dashed)];
    for (m, v) in months.iter().enumerate() {
        series.extend(bar(format!("qh-m{}", m + 1), loc("Net heating demand Q_h,b", "Nutzheizwärmebedarf Q_h,b"), m, -BAR, v.q_h_kwh, Tone::Warning, None));
        series.extend(bar(format!("qc-m{}", m + 1), loc("Net cooling demand Q_c,b", "Nutzkältebedarf Q_c,b"), m, 0.0, v.q_c_kwh, Tone::Info, Some(Hatch::Diagonal)));
    }
    let peak = months.iter().enumerate().max_by(|a, b| a.1.q_h_kwh.total_cmp(&b.1.q_h_kwh)).map_or((0, 0.0), |(m, v)| (m, v.q_h_kwh));
    let annual_h: f64 = months.iter().map(|m| m.q_h_kwh).sum();
    let annual_c: f64 = months.iter().map(|m| m.q_c_kwh).sum();
    Diagram2d {
        id: format!("monthly-balance:{id}"),
        title: loc(&format!("Monthly balance – {en}"), &format!("Monatsbilanz – {de}")),
        description: loc(
            &format!("Monthly heat balance of zone {en} (DIN V 18599-2): losses, gains and the net heating and cooling demand per month; annual heating {annual_h:.0} kWh, cooling {annual_c:.0} kWh."),
            &format!("Monatliche Wärmebilanz der Zone {de} (DIN V 18599-2): Senken, Quellen sowie Nutzheiz- und Nutzkältebedarf je Monat; jährlich Heizen {annual_h:.0} kWh, Kühlen {annual_c:.0} kWh."),
        ),
        axes: vec![axis("Month", "Monat", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.5, 12.5))), axis("Energy", "Energie", QuantityKind::Energy, KWH, AxisScale::Linear, Some((0.0, top)))],
        series,
        markers: vec![marker("peak-month", p(peak.0 as f64 + 1.0, peak.1), report, ENERGY_CHECK, loc(&format!("Peak heating demand {:.0} kWh in month {}", peak.1, peak.0 + 1), &format!("Spitzen-Heizwärmebedarf {:.0} kWh im Monat {}", peak.1, peak.0 + 1)))],
        annotations: Vec::new(),
        focus: Some(entity_ref(EntityKind::Solid, id, "zones")),
        legend: true,
        table_twin: DataTable { columns: vec![column("Month", "Monat", "1", loss.iter().map(|q| q[0]).collect()), column("Losses Q_T + Q_V", "Senken Q_T + Q_V", KWH, loss.iter().map(|q| q[1]).collect()), column("Gains Q_i + Q_s", "Quellen Q_i + Q_s", KWH, gain.iter().map(|q| q[1]).collect()), column("Net heating demand", "Nutzheizwärmebedarf", KWH, months.iter().map(|m| m.q_h_kwh).collect()), column("Net cooling demand", "Nutzkältebedarf", KWH, months.iter().map(|m| m.q_c_kwh).collect())] },
    }
}

/// 🔀️ The five services of the energy flow with their useful, final and primary energy in kWh: heating, domestic hot water, cooling, lighting and auxiliary (fans).
pub fn flows(document: &Din18599Snapshot) -> [(&'static str, &'static str, [f64; 3]); 5] {
    let d = derive_balance(document);
    let el = primary_energy_factor("electricity");
    [
        ("Heating", "Heizung", [d.q_h_nd_kwh, d.q_f_heat_kwh, d.q_f_heat_kwh * primary_energy_factor(&document.heating.energy_carrier)]),
        ("Hot water", "Warmwasser", [d.q_w_kwh, d.q_f_dhw_kwh, d.q_f_dhw_kwh * primary_energy_factor(&document.dhw.energy_carrier)]),
        ("Cooling", "Kühlung", [d.q_c_nd_kwh, d.q_f_cool_kwh, d.q_f_cool_kwh * primary_energy_factor(document.cooling.energy_carrier())]),
        ("Lighting", "Beleuchtung", [d.q_l_kwh, d.q_f_light_kwh, d.q_f_light_kwh * el]),
        ("Auxiliary", "Hilfsenergie", [d.q_f_aux_kwh, d.q_f_aux_kwh, d.q_f_aux_kwh * el]),
    ]
}

fn energy_flow(document: &Din18599Snapshot, report: &CheckReport) -> Diagram2d {
    let flows = flows(document);
    let mut offsets = [[0.0; 5]; 3];
    let mut totals = [0.0; 3];
    for stage in 0..3 {
        for (i, f) in flows.iter().enumerate() {
            offsets[stage][i] = totals[stage];
            totals[stage] += f.2[stage];
        }
    }
    let top = totals.iter().copied().fold(1.0, f64::max) * 1.1;
    let tones = [Tone::Warning, Tone::Info, Tone::Info, Tone::Neutral, Tone::Neutral];
    let hatches = [None, Some(Hatch::Dots), Some(Hatch::Diagonal), None, Some(Hatch::Dots)];
    let mut series = Vec::new();
    for (i, f) in flows.iter().enumerate() {
        for stage in 0..2 {
            let (h0, h1) = (f.2[stage], f.2[stage + 1]);
            if h0 + h1 > 1e-9 {
                let x = stage as f64;
                let ring = vec![p(x, offsets[stage][i]), p(x + 1.0, offsets[stage + 1][i]), p(x + 1.0, offsets[stage + 1][i] + h1), p(x, offsets[stage][i] + h0)];
                series.push(Series::Polygon { id: format!("flow-{}-{}", f.0.to_lowercase().replace(' ', "-"), stage + 1), label: loc(f.0, f.1), ring, fill: Fill { tone: tones[i], opacity: 0.6 }, hatch: hatches[i].clone() });
            }
        }
    }
    let pv = derive_balance(document).q_pv_kwh * primary_energy_factor("electricity");
    Diagram2d {
        id: "energy-flow:building".into(),
        title: loc("Energy flow: useful, final, primary", "Energiefluss: Nutz-, End-, Primärenergie"),
        description: loc(
            &format!("Annual energy flow per service from useful through final to primary energy (DIN V 18599-1, -5, -8, -10): total useful {:.0} kWh, final {:.0} kWh, primary {:.0} kWh before the photovoltaic credit of {pv:.0} kWh.", totals[0], totals[1], totals[2]),
            &format!("Jährlicher Energiefluss je Dienst von der Nutz- über die End- zur Primärenergie (DIN V 18599-1, -5, -8, -10): Nutzenergie gesamt {:.0} kWh, Endenergie {:.0} kWh, Primärenergie {:.0} kWh vor der Photovoltaik-Gutschrift von {pv:.0} kWh.", totals[0], totals[1], totals[2]),
        ),
        axes: vec![axis("Stage (useful, final, primary)", "Stufe (Nutz-, End-, Primärenergie)", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, 2.0))), axis("Energy", "Energie", QuantityKind::Energy, KWH, AxisScale::Linear, Some((0.0, top)))],
        series,
        markers: vec![marker("primary-energy", p(2.0, totals[2]), report, PRIMARY_CHECK, loc(&format!("Primary energy {:.0} kWh", totals[2]), &format!("Primärenergie {:.0} kWh", totals[2])))],
        annotations: Vec::new(),
        focus: None,
        legend: true,
        table_twin: DataTable { columns: vec![column("Useful energy", "Nutzenergie", KWH, flows.iter().map(|f| f.2[0]).collect()), column("Final energy", "Endenergie", KWH, flows.iter().map(|f| f.2[1]).collect()), column("Primary energy", "Primärenergie", KWH, flows.iter().map(|f| f.2[2]).collect())] },
    }
}

fn element_u(document: &Din18599Snapshot, report: &CheckReport, element: &EnvelopeElement) -> Diagram2d {
    let same: Vec<&EnvelopeElement> = document.elements.iter().filter(|e| e.kind == element.kind).collect();
    let reference = reference_u(element.kind);
    let max_area = same.iter().map(|e| e.area_m2).fold(1.0, f64::max) * 1.15;
    let max_u = same.iter().map(|e| e.u_value_w_m2k).fold(reference, f64::max) * 1.25;
    let own = |e: &&EnvelopeElement| e.id == element.id;
    let points = same.iter().map(|e| ScatterPoint { at: p(e.area_m2, e.u_value_w_m2k), tone: if own(e) { Tone::Warning } else { Tone::Neutral }, label: Some(loc(&e.label_en, &e.label_de)) }).collect();
    let mean_id = format!("din18599.geg.mean-u.{:?}", element.kind).to_lowercase();
    let check = if check_with(report, &mean_id).is_some() { mean_id } else { HT_CHECK.to_string() };
    let mut mark = marker("element-u", p(element.area_m2, element.u_value_w_m2k), report, &check, loc(&format!("U = {:.2} W/(m²·K) against reference {reference:.2}", element.u_value_w_m2k), &format!("U = {:.2} W/(m²·K) gegen Referenz {reference:.2}", element.u_value_w_m2k)));
    mark.binding = Some(FieldBinding { path: format!("elements[id={}].uValueWM2k", element.id), map: Affine::IDENTITY, unit: U_UNIT.into(), quantise: None });
    Diagram2d {
        id: format!("element-u:{}", element.id),
        title: loc(&format!("U-value – {}", element.label_en), &format!("U-Wert – {}", element.label_de)),
        description: loc(
            &format!("U-values of all envelope elements of the same kind against the GEG reference U = {reference:.2} W/(m²·K); this element has U = {:.2} W/(m²·K) on {:.1} m².", element.u_value_w_m2k, element.area_m2),
            &format!("U-Werte aller Hüllbauteile gleicher Art gegen den GEG-Referenzwert U = {reference:.2} W/(m²·K); dieses Bauteil hat U = {:.2} W/(m²·K) auf {:.1} m².", element.u_value_w_m2k, element.area_m2),
        ),
        axes: vec![axis("Area", "Fläche", QuantityKind::Area, "m²", AxisScale::Linear, Some((0.0, max_area))), axis("U-value", "U-Wert", QuantityKind::HeatTransferCoefficient, U_UNIT, AxisScale::Linear, Some((0.0, max_u)))],
        series: vec![Series::Scatter { id: "elements".into(), label: loc("Envelope elements of this kind", "Hüllbauteile dieser Art"), points }, Series::Limit { id: "reference".into(), label: loc("GEG reference U", "GEG-Referenz-U"), axis: AxisRef::Y, value: reference, style: stroke(Tone::Neutral, 1.5, Dash::Dashed) }],
        markers: vec![mark],
        annotations: Vec::new(),
        focus: Some(entity_ref(EntityKind::Solid, &element.id, "elements")),
        legend: true,
        table_twin: DataTable { columns: vec![column("Area", "Fläche", "m²", same.iter().map(|e| e.area_m2).collect()), column("U-value", "U-Wert", U_UNIT, same.iter().map(|e| e.u_value_w_m2k).collect())] },
    }
}

/// 📈️ The diagrams of `focus` (a zone, an element or none for all): the zone's monthly balance, the building energy flow and the element's U-value.
pub fn diagrams(document: &Din18599Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    let zone_of_focus = focus.and_then(|f| document.elements.iter().find(|e| e.id == f).map(|e| e.zone_id.as_str()));
    let wanted = |id: &str| focus.is_none_or(|f| f == id || zone_of_focus == Some(id));
    let mut out: Vec<Diagram2d> = zone_heating_months(document).iter().filter(|(id, _)| wanted(id)).map(|(id, months)| zone_balance(document, report, id, months)).collect();
    out.push(energy_flow(document, report));
    out.extend(document.elements.iter().filter(|e| wanted(&e.id)).map(|e| element_u(document, report, e)));
    out
}
