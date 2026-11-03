//! 🦴️ Web panel diagram of a plate girder: the panels of the web between adjacent transverse stiffeners and the supports as filled rectangles (height h_w), coloured by the shear buckling utilisation V_Ed/V_b,Rd of each panel (EN 1999-1-1 §6.7.4), with aspect ratio a/h_w and ρ_v per panel and one draggable marker per stiffener bound to `transverseStiffeners[id=…].position`. A pure function of the document and its report, so a drag preview redraws the panels at once.

use super::*;
use crate::checks::web_stiffener;
use crate::scene::projection_kit::handle_kit::{identity_binding, round};

fn rectangle(x0: f64, x1: f64, height: f64) -> Vec<Vec2> {
    vec![Vec2::new(x0, 0.0), Vec2::new(x1, 0.0), Vec2::new(x1, height), Vec2::new(x0, height)]
}

/// 🦴️ The panel diagram of `member`, `None` unless a transverse stiffener stands inside it and its section has a flanged web with a usable alloy.
pub(super) fn web_panels(d: &En1999Snapshot, report: &CheckReport, member: &AluminiumMember) -> Option<Diagram2d> {
    let rows: Vec<_> = d.transverse_stiffeners.iter().filter(|s| s.girder_id == member.id).collect();
    let positions: Vec<f64> = rows.iter().map(|s| s.position).collect();
    let edges = web_stiffener::web_edges(member.length, &positions);
    if edges.is_empty() {
        return None;
    }
    let (section, alloy) = facts::member_parts(d, member).filter(|(s, _)| !checks::is_free(s) && !part_1_1::is_tube(s))?;
    let (h_w, t_w) = (section.height - 2.0 * section.flange_thickness, section.web_thickness);
    if h_w <= 1e-6 || t_w <= 1e-6 {
        return None;
    }
    let params = na_de::AnnexParams::for_choice(d.annex);
    let v_ed = part_en1990::uls_combinations(member, &params, d.reliability_class).iter().map(|g| g.v_z_ed.abs()).fold(0.0, f64::max);
    let v_pl = h_w * t_w * alloy.f_o_pa / (3.0f64.sqrt() * params.gamma_m1);
    let panels: Vec<(f64, f64, f64, f64, f64, f64)> = edges
        .windows(2)
        .map(|w| {
            let a = w[1] - w[0];
            let rho = part_1_1::shear_buckling_rho(h_w / t_w, alloy.f_o_pa, h_w, a);
            let resistance = rho * v_pl;
            (w[0], w[1], a, a / h_w, rho, if resistance > 0.0 { v_ed / resistance } else { f64::INFINITY })
        })
        .collect();
    let mut series = Vec::new();
    let mut notes = Vec::new();
    for (k, (x0, x1, a, aspect, rho, u)) in panels.iter().enumerate() {
        let tone = utilisation_tone(*u);
        let label = loc(&format!("Panel {}: a = {:.0} mm, a/h_w = {:.2}, ρ_v = {:.3}, V_Ed/V_b,Rd = {:.3}", k + 1, a * 1e3, aspect, rho, u), &format!("Feld {}: a = {:.0} mm, a/h_w = {:.2}, ρ_v = {:.3}, V_Ed/V_b,Rd = {:.3}", k + 1, a * 1e3, aspect, rho, u));
        series.push(Series::Polygon { id: format!("panel-{}", k + 1), label: label.clone(), ring: rectangle(*x0, *x1, h_w), fill: Fill { tone, opacity: 0.35 }, hatch: None });
        let mut outline = rectangle(*x0, *x1, h_w);
        let first = outline[0].clone();
        outline.push(first);
        series.push(Series::Polyline { id: format!("panel-{}-outline", k + 1), label: label.clone(), points: outline, style: stroke(tone, 1.5, if !super::scene::norm::within_limit(*u) { Dash::Dashed } else { Dash::Solid }) });
        notes.push(DiagramNote { id: format!("panel-{}-note", k + 1), at: Vec2::new(0.5 * (x0 + x1), 0.5 * h_w), text: loc(&format!("a/h_w = {:.2}, u = {:.2}", aspect, u), &format!("a/h_w = {:.2}, u = {:.2}", aspect, u)), tone });
    }
    let markers: Vec<Marker> = rows
        .iter()
        .filter(|s| s.position > 0.0 && s.position < member.length)
        .map(|s| {
            let check = worst(report, &s.id, "en1999.6.7.6");
            let u = check.map(|c| c.utilization);
            Marker {
                id: s.id.clone(),
                at: Vec2::new(s.position, 0.5 * h_w),
                shape: MarkerShape::Diamond,
                tone: u.map_or(Tone::Neutral, utilisation_tone),
                label: loc(&format!("Stiffener {} at x = {:.3} m, {:.0} × {:.0} mm, {:?}", s.id, s.position, s.width * 1e3, s.thickness * 1e3, s.sides), &format!("Steife {} bei x = {:.3} m, {:.0} × {:.0} mm, {:?}", s.id, s.position, s.width * 1e3, s.thickness * 1e3, s.sides)),
                utilisation: u,
                bound_check: check.map(|c| c.id.clone()),
                binding: Some(identity_binding(format!("transverseStiffeners[id={}].position", s.id), "m", round(0.01))),
            }
        })
        .collect();
    let column_of = |pick: &dyn Fn(&(f64, f64, f64, f64, f64, f64)) -> f64| panels.iter().map(pick).collect::<Vec<f64>>();
    Some(Diagram2d {
        id: format!("web-panels:{}", member.id),
        title: loc(&format!("Web panels between the transverse stiffeners – {}", member.id), &format!("Stegfelder zwischen den Quersteifen – {}", member.id)),
        description: loc(
            "Panels of the web between adjacent transverse stiffeners and the supports with their aspect ratio a/h_w and the shear buckling utilisation V_Ed/V_b,Rd (EN 1999-1-1, 6.7.4; gross web h_w·t_w, governing ULS V_z); drag a stiffener marker to move it, the panels follow.",
            "Stegfelder zwischen benachbarten Quersteifen und den Auflagern mit Seitenverhältnis a/h_w und Schubbeulausnutzung V_Ed/V_b,Rd (EN 1999-1-1, 6.7.4; Bruttosteg h_w·t_w, maßgebendes ULS V_z); eine Steifenmarke ziehen verschiebt sie, die Felder folgen.",
        ),
        axes: vec![
            axis("Position x along the girder", "Lage x entlang des Trägers", QuantityKind::Length, "m", AxisScale::Linear, Some((0.0, member.length))),
            axis("Web height h_w", "Steghöhe h_w", QuantityKind::Length, "m", AxisScale::Linear, Some((0.0, h_w))),
        ],
        series,
        markers,
        annotations: notes,
        focus: member_focus(&member.id),
        legend: true,
        table_twin: DataTable {
            columns: vec![
                column("Panel", "Feld", "1", (1..=panels.len()).map(|k| k as f64).collect()),
                column("From x", "Von x", "m", column_of(&|p| p.0)),
                column("To x", "Bis x", "m", column_of(&|p| p.1)),
                column("Panel width a", "Feldbreite a", "m", column_of(&|p| p.2)),
                column("Aspect ratio a/h_w", "Seitenverhältnis a/h_w", "1", column_of(&|p| p.3)),
                column("Shear buckling factor ρ_v", "Schubbeulbeiwert ρ_v", "1", column_of(&|p| p.4)),
                column("Utilisation V_Ed/V_b,Rd", "Ausnutzung V_Ed/V_b,Rd", "1", column_of(&|p| p.5)),
            ],
        },
    })
}
