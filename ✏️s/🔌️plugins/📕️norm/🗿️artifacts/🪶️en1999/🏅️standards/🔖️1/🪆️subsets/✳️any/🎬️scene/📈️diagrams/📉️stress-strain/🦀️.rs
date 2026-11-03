//! 📉️ Stress–strain diagram of an alloy: the elastic, perfectly plastic idealisation up to the 0.2 % proof strength f_0.2 with the ultimate strength f_u as a limit, and, for a heat-affected zone, the same curve reduced by ρ_o,haz and ρ_u,haz (EN 1999-1-1, 3.2.2, 3.2.5, 6.1.6, Table 3.2b; the alloy rows are unverified).

use super::*;

const STRAIN_MAX_PERCENT: f64 = 2.0;
const PROOF_OFFSET: f64 = 0.002;

/// 📉️ The idealised curve `(strain in %, stress in MPa)` of the strength `f_o_pa`: the elastic line to the yield strain f_o / E, then the plateau to the end of the strain axis.
pub(super) fn curve(f_o_pa: f64) -> Vec<Xy> {
    let yield_strain = f_o_pa / na_de::E_PA * 100.0;
    vec![[0.0, 0.0], [yield_strain, f_o_pa / 1.0e6], [STRAIN_MAX_PERCENT, f_o_pa / 1.0e6]]
}

/// 📍️ The 0.2 % proof point `(strain in %, stress in MPa)` of the strength `f_o_pa`: the yield strain plus the 0.2 % offset.
pub(super) fn proof_point(f_o_pa: f64) -> Xy {
    [(f_o_pa / na_de::E_PA + PROOF_OFFSET) * 100.0, f_o_pa / 1.0e6]
}

fn marker(id: &str, at: Xy, tone: Tone, en: String, de: String) -> Marker {
    Marker { id: format!("reference:{id}"), at: Vec2::new(at[0], at[1]), shape: MarkerShape::Diamond, tone, label: loc(&en, &de), utilisation: None, bound_check: None, binding: None }
}

/// 📉️ The diagram `stress-strain:<id>` of a material, `None` for an unknown designation.
pub(super) fn stress_strain(material: &AluminiumMaterial) -> Option<Diagram2d> {
    let alloy = part_1_1::resolve_alloy(&material.designation)?;
    let (f_o, f_u) = (alloy.f_o_pa, alloy.f_u_pa);
    let reduced = alloy.rho_o_haz < 1.0 || alloy.rho_u_haz < 1.0;
    let (h_o, h_u) = (f_o * alloy.rho_o_haz, f_u * alloy.rho_u_haz);
    let mut series = vec![
        Series::Polyline { id: "parent".into(), label: loc("Parent material", "Grundwerkstoff"), points: curve(f_o).into_iter().map(p).collect(), style: stroke(Tone::Info, 2.5, Dash::Solid) },
        Series::Limit { id: "f-u".into(), label: loc("Ultimate strength f_u", "Zugfestigkeit f_u"), axis: AxisRef::Y, value: f_u / 1.0e6, style: stroke(Tone::Neutral, 1.5, Dash::Dashed) },
    ];
    let mut markers = vec![marker("f-0-2", proof_point(f_o), Tone::Info, format!("f_0.2 = {:.0} MPa", f_o / 1.0e6), format!("f_0,2 = {:.0} MPa", f_o / 1.0e6)), marker("f-u", [STRAIN_MAX_PERCENT, f_u / 1.0e6], Tone::Neutral, format!("f_u = {:.0} MPa", f_u / 1.0e6), format!("f_u = {:.0} MPa", f_u / 1.0e6))];
    let mut columns = vec![column("Strain ε (parent)", "Dehnung ε (Grundwerkstoff)", "%", curve(f_o).iter().map(|q| q[0]).collect()), column("Stress σ (parent)", "Spannung σ (Grundwerkstoff)", "MPa", curve(f_o).iter().map(|q| q[1]).collect())];
    if reduced {
        series.push(Series::Polyline { id: "haz".into(), label: loc("Heat-affected zone", "Wärmeeinflusszone"), points: curve(h_o).into_iter().map(p).collect(), style: stroke(Tone::Warning, 2.5, Dash::Solid) });
        series.push(Series::Limit { id: "f-u-haz".into(), label: loc("HAZ ultimate strength ρ_u,haz·f_u", "WEZ-Zugfestigkeit ρ_u,haz·f_u"), axis: AxisRef::Y, value: h_u / 1.0e6, style: stroke(Tone::Warning, 1.5, Dash::Dashed) });
        markers.push(marker("f-0-2-haz", proof_point(h_o), Tone::Warning, format!("ρ_o,haz·f_0.2 = {:.0} MPa", h_o / 1.0e6), format!("ρ_o,haz·f_0,2 = {:.0} MPa", h_o / 1.0e6)));
        columns.push(column("Strain ε (HAZ)", "Dehnung ε (WEZ)", "%", curve(h_o).iter().map(|q| q[0]).collect()));
        columns.push(column("Stress σ (HAZ)", "Spannung σ (WEZ)", "MPa", curve(h_o).iter().map(|q| q[1]).collect()));
    }
    Some(Diagram2d {
        id: format!("stress-strain:{}", material.id),
        title: loc(&format!("Stress–strain – {} ({})", material.id, material.designation), &format!("Spannungs-Dehnungs-Linie – {} ({})", material.id, material.designation)),
        description: loc("Elastic, perfectly plastic idealisation up to the 0.2 % proof strength f_0.2 with the ultimate strength f_u; where the alloy loses strength in the heat-affected zone, the reduced curve is drawn too (alloy rows unverified).", "Elastisch-ideal plastische Idealisierung bis zur 0,2-%-Dehngrenze f_0,2 mit der Zugfestigkeit f_u; verliert die Legierung in der Wärmeeinflusszone an Festigkeit, ist auch die abgeminderte Linie gezeichnet (Legierungswerte ungeprüft)."),
        axes: vec![axis("Strain ε", "Dehnung ε", QuantityKind::Dimensionless, "%", AxisScale::Linear, Some((0.0, STRAIN_MAX_PERCENT))), axis("Stress σ", "Spannung σ", QuantityKind::Stress, "MPa", AxisScale::Linear, Some((0.0, f_u / 1.0e6 * 1.1)))],
        series,
        markers,
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Group, &material.id, format!("materials[id={}]", material.id))),
        legend: true,
        table_twin: DataTable { columns },
    })
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
