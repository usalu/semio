//! 📈️ Diagrams: the monthly heating and cooling balance of each thermal zone (DIN V 18599-2), the useful-final-primary energy flow of the building (DIN V 18599-1, -5, -8, -10) and the U-value of each envelope element against the GEG reference.

use super::checks::{annex3_envelope_groups, derive_balance, Annex3EnvelopeGroup, primary_energy_factor, reference_u, zone_heating_months, BalanceDerived, HeatingMonth, RenewableElectricityMonth};
use super::kit::{check_with, entity_ref, loc};
use super::model::{Adjacency, BuildingCategory, Din18599Snapshot, ElementKind, EnvelopeElement, UsageProfile};
use super::norm::CheckReport;
use super::scene::diagram_kit::Xy;
use super::scene::projection_kit::diagram_parts::{axis, checked_marker_id, column, stroke};
use super::scene::*;

pub const KWH: &str = "kWh";
pub const U_UNIT: &str = "W/(m²·K)";
const BAR: f64 = 0.4;
const ENERGY_CHECK: &str = "din18599.2.heating-demand";
const PRIMARY_CHECK: &str = "din18599.geg.qp";
const RENEWABLE_CHECK: &str = "din18599.geg.renewable-electricity";
const HT_CHECK: &str = "din18599.geg.ht-prime";
const RESIDENTIAL_GAINS_CHECK: &str = "din18599.10.residential-internal-gains";

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
    let internal = at(|m| m.q_i_kwh);
    let gain_source = zone
        .filter(|z| z.usage_profile == UsageProfile::WFH)
        .and_then(|_| check_with(report, RESIDENTIAL_GAINS_CHECK))
        .map_or_else(|| loc("Includes the entered internal gains.", "Enthält die eingegebenen internen Wärmegewinne."), |check| loc(&check.explanation.en, &check.explanation.de));
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
            &format!("Monthly heat balance of zone {en} (DIN V 18599-2): losses, gains and the net heating and cooling demand per month; annual heating {annual_h:.0} kWh, cooling {annual_c:.0} kWh. {}", gain_source.en),
            &format!("Monatliche Wärmebilanz der Zone {de} (DIN V 18599-2): Senken, Quellen sowie Nutzheiz- und Nutzkältebedarf je Monat; jährlich Heizen {annual_h:.0} kWh, Kühlen {annual_c:.0} kWh. {}", gain_source.de),
        ),
        axes: vec![axis("Month", "Monat", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.5, 12.5))), axis("Energy", "Energie", QuantityKind::Energy, KWH, AxisScale::Linear, Some((0.0, top)))],
        series,
        markers: vec![marker("peak-month", p(peak.0 as f64 + 1.0, peak.1), report, ENERGY_CHECK, loc(&format!("Peak heating demand {:.0} kWh in month {}", peak.1, peak.0 + 1), &format!("Spitzen-Heizwärmebedarf {:.0} kWh im Monat {}", peak.1, peak.0 + 1)))],
        annotations: Vec::new(),
        focus: Some(entity_ref(EntityKind::Solid, id, "zones")),
        legend: true,
        table_twin: DataTable { columns: vec![column("Month", "Monat", "1", loss.iter().map(|q| q[0]).collect()), column("Losses Q_T + Q_V", "Senken Q_T + Q_V", KWH, loss.iter().map(|q| q[1]).collect()), column("Gains Q_i + Q_s", "Quellen Q_i + Q_s", KWH, gain.iter().map(|q| q[1]).collect()), column("Net heating demand", "Nutzheizwärmebedarf", KWH, months.iter().map(|m| m.q_h_kwh).collect()), column("Net cooling demand", "Nutzkältebedarf", KWH, months.iter().map(|m| m.q_c_kwh).collect()), column("Internal heat gains Q_i", "Interne Wärmegewinne Q_i", KWH, internal.iter().map(|q| q[1]).collect())] },
    }
}

/// 🔀️ The five services of the energy flow with their useful, final and primary energy in kWh: heating, domestic hot water, cooling, lighting and auxiliary (fans).
pub fn flows(document: &Din18599Snapshot) -> [(&'static str, &'static str, [f64; 3]); 5] {
    service_flows(document, &derive_balance(document))
}

fn service_flows(document: &Din18599Snapshot, d: &BalanceDerived) -> [(&'static str, &'static str, [f64; 3]); 5] {
    let el = primary_energy_factor("electricity");
    [
        ("Heating", "Heizung", [d.q_h_nd_kwh, d.q_f_heat_kwh, d.q_f_heat_kwh * primary_energy_factor(&document.heating.energy_carrier)]),
        ("Hot water", "Warmwasser", [d.q_w_useful_kwh, d.q_f_dhw_kwh, d.q_f_dhw_kwh * primary_energy_factor(&document.dhw.energy_carrier)]),
        ("Cooling", "Kühlung", [d.q_c_nd_kwh, d.q_f_cool_kwh, d.q_f_cool_kwh * primary_energy_factor(document.cooling.energy_carrier())]),
        ("Lighting", "Beleuchtung", [d.q_l_kwh, d.q_f_light_kwh, d.q_f_light_kwh * el]),
        ("Auxiliary", "Hilfsenergie", [d.q_f_aux_kwh, d.q_f_aux_kwh, d.q_f_aux_kwh * el]),
    ]
}

fn energy_flow(document: &Din18599Snapshot, report: &CheckReport, balance: &BalanceDerived) -> Diagram2d {
    let flows = service_flows(document, balance);
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
    let pv = balance.q_pv_kwh * primary_energy_factor("electricity");
    Diagram2d {
        id: "energy-flow:building".into(),
        title: loc("Energy flow: useful, final, primary", "Energiefluss: Nutz-, End-, Primärenergie"),
        description: loc(
            &format!("Annual energy flow per service from useful through final to primary energy (DIN V 18599-1, -5, -8, -10): total useful {:.0} kWh, final {:.0} kWh, primary {:.0} kWh before the modelled photovoltaic credit of {pv:.0} kWh.", totals[0], totals[1], totals[2]),
            &format!("Jährlicher Energiefluss je Dienst von der Nutz- über die End- zur Primärenergie (DIN V 18599-1, -5, -8, -10): Nutzenergie gesamt {:.0} kWh, Endenergie {:.0} kWh, Primärenergie {:.0} kWh vor der modellierten Photovoltaik-Gutschrift von {pv:.0} kWh.", totals[0], totals[1], totals[2]),
        ),
        axes: vec![axis("Stage (useful, final, primary)", "Stufe (Nutz-, End-, Primärenergie)", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, 2.0))), axis("Energy", "Energie", QuantityKind::Energy, KWH, AxisScale::Linear, Some((0.0, top)))],
        series,
        markers: vec![marker("primary-energy", p(2.0, balance.q_p_kwh), report, PRIMARY_CHECK, loc(
            &format!("Primary energy after modelled credit {:.0} kWh", balance.q_p_kwh),
            &format!("Primärenergie nach modellierter Anrechnung {:.0} kWh", balance.q_p_kwh),
        ))],
        annotations: Vec::new(),
        focus: None,
        legend: true,
        table_twin: DataTable { columns: vec![column("Useful energy", "Nutzenergie", KWH, flows.iter().map(|f| f.2[0]).collect()), column("Final energy", "Endenergie", KWH, flows.iter().map(|f| f.2[1]).collect()), column("Primary energy", "Primärenergie", KWH, flows.iter().map(|f| f.2[2]).collect())] },
    }
}

fn renewable_electricity(balance: &BalanceDerived, report: &CheckReport) -> Diagram2d {
    let months = &balance.renewable_electricity_months;
    let at = |value: fn(&RenewableElectricityMonth) -> f64| -> Vec<Xy> {
        months.iter().enumerate().map(|(m, row)| [m as f64 + 1.0, value(row)]).collect()
    };
    let generation = at(|m| m.generation_kwh);
    let demand = at(|m| m.eligible_electricity_kwh);
    let credit = at(|m| m.credit_kwh);
    let peak = credit.iter().max_by(|a, b| a[1].total_cmp(&b[1])).copied().unwrap_or([1.0, 0.0]);
    Diagram2d {
        id: "renewable-electricity:building".into(),
        title: loc("Monthly renewable electricity", "Monatlicher erneuerbarer Strom"),
        description: loc(
            "Compare estimated generation, eligible electricity demand and modelled credit for each month.",
            "Vergleich von geschätztem Ertrag, anrechenbarem Strombedarf und modellierter Anrechnung je Monat.",
        ),
        axes: vec![
            axis("Month", "Monat", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.5, 12.5))),
            axis("Electricity", "Strom", QuantityKind::Energy, KWH, AxisScale::Linear, None),
        ],
        series: vec![
            line("generation", "Estimated generation", "Geschätzter Ertrag", &generation, Tone::Neutral, Dash::Dashed),
            line("demand", "Eligible demand", "Anrechenbarer Bedarf", &demand, Tone::Warning, Dash::Solid),
            line("credit", "Modelled credit", "Modellierte Anrechnung", &credit, Tone::Info, Dash::Solid),
        ],
        markers: vec![marker("peak-credit", p(peak[0], peak[1]), report, RENEWABLE_CHECK, loc(
            &format!("Largest monthly credit {:.0} kWh", peak[1]),
            &format!("Höchste monatliche Anrechnung {:.0} kWh", peak[1]),
        ))],
        annotations: Vec::new(),
        focus: None,
        legend: true,
        table_twin: DataTable { columns: vec![
            column("Month", "Monat", "1", generation.iter().map(|row| row[0]).collect()),
            column("Estimated generation", "Geschätzter Ertrag", KWH, generation.iter().map(|row| row[1]).collect()),
            column("Eligible demand", "Anrechenbarer Bedarf", KWH, demand.iter().map(|row| row[1]).collect()),
            column("Modelled credit", "Modellierte Anrechnung", KWH, credit.iter().map(|row| row[1]).collect()),
        ] },
    }
}

fn element_u(document: &Din18599Snapshot, report: &CheckReport, element: &EnvelopeElement, groups: &[Annex3EnvelopeGroup<'_>]) -> Diagram2d {
    let (same, reference, check, description, limit_label) = match document.building_category {
        BuildingCategory::Residential => {
            let reference = reference_u(element);
            let same: Vec<_> = document.elements.iter()
                .filter(|e| e.adjacency != Adjacency::Heated && e.kind == element.kind && reference_u(e) == reference)
                .collect();
            let description = loc(
                &format!("U-values of envelope elements sharing the Annex 1 reference of {reference:.2} W/(m²·K)."),
                &format!("U-Werte von Hüllbauteilen mit demselben Referenzwert von {reference:.2} W/(m²·K) nach Anlage 1."),
            );
            (same, Some(reference), HT_CHECK.to_string(), description, loc("GEG reference U", "GEG-Referenz-U"))
        }
        BuildingCategory::NonResidential => {
            let group = groups.iter().find(|group| group.elements.iter().any(|e| e.id == element.id));
            let (same, reference, id) = match group {
                Some(group) => (group.elements.clone(), Some(group.limit()), group.check_id()),
                None => (vec![element], None, "din18599.geg.mean-u.basis".to_string()),
            };
            let description = check_with(report, &id).map_or_else(|| loc(
                "Entered transmittance of this component.",
                "Erfasster Wärmedurchgangskoeffizient des Bauteils.",
            ), |check| loc(&check.explanation.en, &check.explanation.de));
            (same, reference, id, description, loc("Annex 3 mean U limit", "Grenzwert des mittleren U-Werts nach Anlage 3"))
        }
    };
    let max_area = same.iter().map(|e| e.area_m2).fold(1.0, f64::max) * 1.15;
    let max_u = same.iter().map(|e| e.u_value_w_m2k).fold(reference.unwrap_or(1.0), f64::max) * 1.25;
    let points = same.iter().map(|e| ScatterPoint {
        at: p(e.area_m2, e.u_value_w_m2k),
        tone: if e.id == element.id { Tone::Warning } else { Tone::Neutral },
        label: Some(loc(&e.label_en, &e.label_de)),
    }).collect();
    let mut mark = marker("element-u", p(element.area_m2, element.u_value_w_m2k), report, &check, loc(
        &format!("Entered U = {:.2} W/(m²·K)", element.u_value_w_m2k),
        &format!("Erfasster U-Wert = {:.2} W/(m²·K)", element.u_value_w_m2k),
    ));
    mark.binding = Some(FieldBinding { path: format!("elements[id={}].uValueWM2k", element.id), map: Affine::IDENTITY, unit: U_UNIT.into(), quantise: None });
    let mut series = vec![Series::Scatter { id: "elements".into(), label: loc("Envelope elements", "Hüllbauteile"), points }];
    if let Some(value) = reference {
        series.push(Series::Limit { id: "reference".into(), label: limit_label, axis: AxisRef::Y, value, style: stroke(Tone::Neutral, 1.5, Dash::Dashed) });
    }
    Diagram2d {
        id: format!("element-u:{}", element.id),
        title: loc(&format!("U-value – {}", element.label_en), &format!("U-Wert – {}", element.label_de)),
        description,
        axes: vec![axis("Area", "Fläche", QuantityKind::Area, "m²", AxisScale::Linear, Some((0.0, max_area))), axis("U-value", "U-Wert", QuantityKind::HeatTransferCoefficient, U_UNIT, AxisScale::Linear, Some((0.0, max_u)))],
        series,
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
    let balance = derive_balance(document);
    out.push(energy_flow(document, report, &balance));
    if document.renewables.pv_area_m2 > 0.0 {
        out.push(renewable_electricity(&balance, report));
    }
    let groups = if document.building_category == BuildingCategory::NonResidential { annex3_envelope_groups(document).0 } else { Vec::new() };
    out.extend(document.elements.iter().filter(|e| e.adjacency != Adjacency::Heated && wanted(&e.id)).map(|e| element_u(document, report, e, &groups)));
    out
}
