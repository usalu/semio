//! 🔩️ Connection diagrams (EN 1999-1-1 §8): the bolt group in plan with its hole circles, the permitted edge-distance band (1.2·d0 ≤ e ≤ 4·t + 40 mm), the per-fastener shear check and draggable pitch and edge markers, and the fillet weld resistance (weld metal and HAZ fusion boundary) over the throat with the design force per length. Every number is read from the check report or the same clause functions the checks call.

use super::*;
use crate::checks::joints;
use crate::model::{AluminiumConnection, ConnectionKind};
use crate::scene::projection_kit::handle_kit::{field_binding, round};

const HOLE_SEGMENTS: usize = 24;
const THROAT_MIN: f64 = 0.002;
const THROAT_MAX: f64 = 0.012;

fn circle(cx: f64, cy: f64, r: f64) -> Vec<Vec2> {
    (0..=HOLE_SEGMENTS).map(|k| std::f64::consts::TAU * k as f64 / HOLE_SEGMENTS as f64).map(|a| Vec2::new(cx + r * a.cos(), cy + r * a.sin())).collect()
}

fn rectangle(x0: f64, y0: f64, x1: f64, y1: f64) -> Vec<Vec2> {
    vec![Vec2::new(x0, y0), Vec2::new(x1, y0), Vec2::new(x1, y1), Vec2::new(x0, y1), Vec2::new(x0, y0)]
}

fn connection_focus(conn: &AluminiumConnection) -> Option<EntityRef> {
    Some(EntityRef::new(EntityKind::Group, &conn.id, format!("connections[id={}]", conn.id)))
}

/// 🔩️ The bolt coordinates `(x, y)` in plan: `x` along the member axis carries the rows (pitch `p1`, every second bolt line shifted by the hole stagger `s`), `y` the bolts per row (gauge `p2`), both from the plate corner.
pub fn bolt_positions(conn: &AluminiumConnection) -> Vec<(f64, f64)> {
    let (b, stagger) = (&conn.bolts, conn.hole_stagger);
    (0..b.rows).flat_map(|i| (0..b.bolts_per_row).map(move |j| (b.edge_distance + f64::from(i) * b.pitch + if j % 2 == 1 { stagger } else { 0.0 }, b.edge_distance + f64::from(j) * b.gauge))).collect()
}

/// 🔩️ The diagram `bolt-group:<id>` of a bolted, combined or pinned connection, `None` without a fastener grid.
pub(super) fn bolt_group(report: &CheckReport, conn: &AluminiumConnection) -> Option<Diagram2d> {
    let b = &conn.bolts;
    if conn.kind == ConnectionKind::Welded || b.diameter <= 1e-6 || b.rows == 0 || b.bolts_per_row == 0 {
        return None;
    }
    let d0 = joints::hole_diameter(b.diameter);
    let length = 2.0 * b.edge_distance + f64::from(b.rows - 1) * b.pitch + if b.bolts_per_row > 1 { conn.hole_stagger } else { 0.0 };
    let width = 2.0 * b.edge_distance + f64::from(b.bolts_per_row - 1) * b.gauge;
    let shear = exact(report, &format!("en1999.8.5.bolt.{}", conn.id)).or_else(|| exact(report, &format!("en1999.8.5.pin.{}", conn.id)));
    let tension = exact(report, &format!("en1999.8.5.tension.{}", conn.id));
    let u = shear.map(|c| c.utilization);
    let tone = u.map_or(Tone::Neutral, utilisation_tone);
    let min_edge = joints::MIN_END_EDGE_D0 * d0;
    let max_edge = 4.0 * b.plate_thickness + 0.040;
    let mut series = vec![Series::Polyline { id: "plate".into(), label: loc("Connected plate", "Anschlussblech"), points: rectangle(0.0, 0.0, length, width), style: stroke(Tone::Info, 2.0, Dash::Solid) }];
    if 2.0 * min_edge < length.min(width) {
        series.push(Series::Polyline { id: "edge-min".into(), label: loc(&format!("Minimum edge distance 1.2·d0 = {:.1} mm", min_edge * 1e3), &format!("Mindestrandabstand 1,2·d0 = {:.1} mm", min_edge * 1e3)), points: rectangle(min_edge, min_edge, length - min_edge, width - min_edge), style: stroke(Tone::Warning, 1.0, Dash::Dashed) });
    }
    if 2.0 * max_edge < length.min(width) {
        series.push(Series::Polyline { id: "edge-max".into(), label: loc(&format!("Maximum edge distance 4·t + 40 mm = {:.1} mm", max_edge * 1e3), &format!("Größter Randabstand 4·t + 40 mm = {:.1} mm", max_edge * 1e3)), points: rectangle(max_edge, max_edge, length - max_edge, width - max_edge), style: stroke(Tone::Neutral, 1.0, Dash::Dotted) });
    }
    let positions = bolt_positions(conn);
    let net = exact(report, &format!("en1999.8.5.net.{}", conn.id));
    let across = joints::plate_width(b.bolts_per_row, b.gauge, b.edge_distance);
    let path = joints::net_path(across, d0, b.bolts_per_row, b.gauge, conn.hole_stagger);
    if let (Some(first), Some(last)) = (path.first(), path.last()) {
        let at = |line: usize| positions[line];
        let points: Vec<Vec2> = std::iter::once(Vec2::new(at(*first).0, 0.0)).chain(path.iter().map(|line| Vec2::new(at(*line).0, at(*line).1))).chain(std::iter::once(Vec2::new(at(*last).0, width))).collect();
        let b_net = joints::net_width(across, d0, b.bolts_per_row, b.gauge, conn.hole_stagger);
        series.push(Series::Polyline { id: "net-path".into(), label: loc(&format!("Critical net-section path: b_net = {:.1} mm of b = {:.1} mm{}", b_net * 1e3, across * 1e3, net.map_or(String::new(), |c| format!(", utilisation {:.2}", c.utilization))), &format!("Maßgebender Nettoquerschnittspfad: b_net = {:.1} mm von b = {:.1} mm{}", b_net * 1e3, across * 1e3, net.map_or(String::new(), |c| format!(", Ausnutzung {:.2}", c.utilization)))), points, style: stroke(utilisation_tone(net.map_or(0.0, |c| c.utilization)), 1.5, Dash::Dashed) });
    }
    let forces = crate::connections::fastener_forces(report, conn);
    let capacity = shear.map(|c| c.limit.value).filter(|v| *v > 0.0);
    let peak = forces.as_ref().map_or(0.0, |(f, g)| f[*g].resultant());
    let arrow = 0.35 * b.pitch.max(b.gauge);
    let mut markers = Vec::new();
    let mut loads = Vec::new();
    let mut utilisations = Vec::new();
    for (k, (x, y)) in positions.iter().enumerate() {
        let (row, line) = (k / b.bolts_per_row as usize, k % b.bolts_per_row as usize);
        let mine = forces.as_ref().map(|(f, g)| (f[k], k == *g));
        let share = match (mine, capacity) {
            (Some((f, _)), Some(c)) => Some(f.resultant() / c),
            _ => u,
        };
        let tone = share.map_or(tone, utilisation_tone);
        utilisations.push(share.unwrap_or(0.0));
        loads.push(mine.map_or(shear.map_or(0.0, |c| c.computed.value), |(f, _)| f.resultant()) / 1e3);
        series.push(Series::Polyline { id: format!("hole-{row}-{line}"), label: loc(&format!("Hole d0 = {:.1} mm", d0 * 1e3), &format!("Loch d0 = {:.1} mm", d0 * 1e3)), points: circle(*x, *y, d0 / 2.0), style: stroke(tone, 1.5, Dash::Solid) });
        if let Some((f, _)) = mine.filter(|_| peak > 0.0) {
            let (ux, uy) = (arrow * f.fx / peak, arrow * f.fy / peak);
            let tip = Vec2::new(x + ux, y + uy);
            let back = (ux.hypot(uy)).max(1e-12);
            let (hx, hy) = (-ux / back * 0.25 * arrow, -uy / back * 0.25 * arrow);
            series.push(Series::Polyline {
                id: format!("force-{row}-{line}"),
                label: loc(&format!("Force on bolt {row}/{line}: {:.2} kN", f.resultant() / 1e3), &format!("Kraft auf Schraube {row}/{line}: {:.2} kN", f.resultant() / 1e3)),
                points: vec![Vec2::new(*x, *y), tip.clone(), Vec2::new(tip.x + hx - hy * 0.5, tip.y + hy + hx * 0.5), tip.clone(), Vec2::new(tip.x + hx + hy * 0.5, tip.y + hy - hx * 0.5)],
                style: stroke(tone, 2.0, Dash::Solid),
            });
        }
        let spread = forces.as_ref().is_some_and(|(all, _)| all.iter().any(|g| (g.resultant() - peak).abs() > 1e-9 * peak));
        let describe = |governing: &str, label: &str| match (mine, shear) {
            (Some((f, top)), Some(c)) => format!(": F_v,Ed = {:.2} kN{}, F_Rd = {:.2} kN, {label} {:.2}", f.resultant() / 1e3, if top && peak > 0.0 && spread { governing } else { "" }, c.limit.value / 1e3, share.unwrap_or(c.utilization)),
            (None, Some(c)) => format!(": F_v,Ed = {:.2} kN, F_Rd = {:.2} kN, {label} {:.2}", c.computed.value / 1e3, c.limit.value / 1e3, c.utilization),
            _ => String::new(),
        };
        let (force, force_de) = (describe(" (governing)", "utilisation"), describe(" (maßgebend)", "Ausnutzung"));
        let pull = tension.map_or(String::new(), |c| format!(", F_t,Ed = {:.2} kN of {:.2} kN", c.computed.value / 1e3, c.limit.value / 1e3));
        let binding = match (row, line) {
            (0, 0) => Some(field_binding(format!("connections[id={}].bolts.edgeDistance", conn.id), Affine::IDENTITY, "m", round(0.001))),
            (1, 0) => Some(field_binding(format!("connections[id={}].bolts.pitch", conn.id), Affine { base: -b.edge_distance, k: 1.0 }, "m", round(0.001))),
            (0, 1) => Some(field_binding(format!("connections[id={}].holeStagger", conn.id), Affine { base: -b.edge_distance, k: 1.0 }, "m", round(0.001))),
            _ => None,
        };
        let name = match (row, line) {
            (0, 0) => " (drag: edge distance e)",
            (1, 0) => " (drag: pitch p1)",
            (0, 1) => " (drag: hole stagger s)",
            _ => "",
        };
        let name_de = match (row, line) {
            (0, 0) => " (ziehen: Randabstand e)",
            (1, 0) => " (ziehen: Teilung p1)",
            (0, 1) => " (ziehen: Lochversatz s)",
            _ => "",
        };
        markers.push(Marker {
            id: format!("bolt-{row}-{line}"),
            at: Vec2::new(*x, *y),
            shape: if binding.is_some() { MarkerShape::Diamond } else { MarkerShape::Circle },
            tone,
            label: loc(&format!("Bolt {row}/{line}{force}{pull}{name}"), &format!("Schraube {row}/{line}{force_de}{pull}{name_de}")),
            utilisation: share,
            bound_check: shear.map(|c| c.id.clone()),
            binding,
        });
    }
    let pad = 0.1 * length.max(width);
    let notes = vec![DiagramNote { id: "group".into(), at: Vec2::new(0.5 * length, width + 0.5 * pad), text: loc(&format!("{} × {} {} Ø{:.0}, p1 = {:.0} mm, p2 = {:.0} mm, e = {:.0} mm", b.rows, b.bolts_per_row, b.material, b.diameter * 1e3, b.pitch * 1e3, b.gauge * 1e3, b.edge_distance * 1e3), &format!("{} × {} {} Ø{:.0}, p1 = {:.0} mm, p2 = {:.0} mm, e = {:.0} mm", b.rows, b.bolts_per_row, b.material, b.diameter * 1e3, b.pitch * 1e3, b.gauge * 1e3, b.edge_distance * 1e3)), tone }];
    Some(Diagram2d {
        id: format!("bolt-group:{}", conn.id),
        title: loc(&format!("Bolt group in plan – {}", conn.id), &format!("Schraubenbild im Grundriss – {}", conn.id)),
        description: loc(
            "Plate outline with the hole circles of the fastener grid (EN 1999-1-1, 8.5), the permitted edge band between 1.2·d0 and 4·t + 40 mm (dashed and dotted) and the elastic shear force per fastener of the governing combination (arrows; the load eccentricity e of the connection adds the torque V·e); the first diamond drags the edge distance, the second the pitch.",
            "Blechumriss mit den Lochkreisen des Verbindungsmittelbilds (EN 1999-1-1, 8.5), dem zulässigen Randabstandsband zwischen 1,2·d0 und 4·t + 40 mm (gestrichelt und gepunktet) und der elastisch verteilten Abscherkraft je Verbindungsmittel der maßgebenden Kombination (Pfeile; die Lastexzentrizität e des Anschlusses ergibt das Torsionsmoment V·e); die erste Raute zieht den Randabstand, die zweite die Teilung.",
        ),
        axes: vec![axis("Along the member x", "Längs des Bauteils x", QuantityKind::Length, "m", AxisScale::Linear, Some((-pad, length + pad))), axis("Across the member y", "Quer zum Bauteil y", QuantityKind::Length, "m", AxisScale::Linear, Some((-pad, width + 2.0 * pad)))],
        series,
        markers,
        annotations: notes,
        focus: connection_focus(conn),
        legend: true,
        table_twin: DataTable { columns: vec![column("Bolt x", "Schraube x", "m", positions.iter().map(|p| p.0).collect()), column("Bolt y", "Schraube y", "m", positions.iter().map(|p| p.1).collect()), column("Shear force F_v,Ed", "Abscherkraft F_v,Ed", "kN", loads), column("Utilisation", "Ausnutzung", "1", utilisations)] },
    })
}

/// 🔥️ The diagram `weld-resistance:<id>` of a welded or combined connection: the weld metal and the HAZ fusion boundary resistance per unit length over the throat (the parent band follows `max(t_p, a/0.7)` exactly as the check), the design force per length as a limit and the throat marker that drags `welds.throat`. `None` without a weld, its check or a resolvable alloy.
pub(super) fn weld_resistance(d: &En1999Snapshot, report: &CheckReport, conn: &AluminiumConnection) -> Option<Diagram2d> {
    let w = &conn.welds;
    if !matches!(conn.kind, ConnectionKind::Welded | ConnectionKind::Combined) || w.throat <= 1e-6 || w.length <= 1e-6 {
        return None;
    }
    let check = exact(report, &format!("en1999.8.6.weld.{}", conn.id))?;
    let material = d.materials.iter().find(|m| m.id == conn.material_id)?;
    let params = na_de::AnnexParams::for_choice(d.annex);
    let fallback = part_1_1::resolve_alloy(&material.designation);
    let resistance = |a: f64| -> Option<(f64, f64)> {
        let parent_t = conn.bolts.plate_thickness.max(a / 0.7);
        let band = part_1_1::resolve_alloy_for_thickness(&material.designation, parent_t).or(fallback)?;
        Some(joints::weld_resistance_per_length(a, w.beta_w, joints::filler_fw_pa(&w.filler_alloy, band.f_u_pa), band.f_u_pa, band.rho_u_haz, params.gamma_mw, params.gamma_m2))
    };
    let throats: Vec<f64> = (0..=SAMPLES).map(|i| THROAT_MIN + (THROAT_MAX - THROAT_MIN) * i as f64 / SAMPLES as f64).collect();
    let rows: Option<Vec<(f64, f64)>> = throats.iter().map(|a| resistance(*a)).collect();
    let rows = rows?;
    let metal: Vec<f64> = rows.iter().map(|r| r.0 / 1e3).collect();
    let haz: Vec<f64> = rows.iter().map(|r| r.1 / 1e3).collect();
    let f_ed = check.computed.value / 1e3;
    let top = metal.iter().chain(&haz).cloned().fold(f_ed, f64::max) * 1.1;
    let line = |values: &[f64]| throats.iter().zip(values).map(|(a, v)| Vec2::new(*a, *v)).collect::<Vec<_>>();
    let series = vec![
        Series::Polyline { id: "weld-metal".into(), label: loc("Weld metal f_w·a/(β_w·γ_Mw)", "Schweißgut f_w·a/(β_w·γ_Mw)"), points: line(&metal), style: stroke(Tone::Info, 2.5, Dash::Solid) },
        Series::Polyline { id: "haz-boundary".into(), label: loc("HAZ fusion boundary ρ_u,haz·f_u·(a/0.7)/(√3·γ_M2)", "WEZ-Schmelzgrenze ρ_u,haz·f_u·(a/0,7)/(√3·γ_M2)"), points: line(&haz), style: stroke(Tone::Warning, 2.5, Dash::Dashed) },
        Series::Limit { id: "f-ed".into(), label: loc("Design force per length f_Ed", "Bemessungskraft je Länge f_Ed"), axis: AxisRef::Y, value: f_ed, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
    ];
    let governing = check.utilization;
    let marker = Marker {
        id: "throat".into(),
        at: Vec2::new(w.throat.clamp(THROAT_MIN, THROAT_MAX), (check.limit.value / 1e3).min(top)),
        shape: MarkerShape::Diamond,
        tone: utilisation_tone(governing),
        label: loc(&format!("a = {:.1} mm: f_Rd = {:.1} kN/m, f_Ed = {f_ed:.1} kN/m, utilisation {governing:.2}", w.throat * 1e3, check.limit.value / 1e3), &format!("a = {:.1} mm: f_Rd = {:.1} kN/m, f_Ed = {f_ed:.1} kN/m, Ausnutzung {governing:.2}", w.throat * 1e3, check.limit.value / 1e3)),
        utilisation: Some(governing),
        bound_check: Some(check.id.clone()),
        binding: Some(field_binding(format!("connections[id={}].welds.throat", conn.id), Affine::IDENTITY, "m", round(0.0005))),
    };
    Some(Diagram2d {
        id: format!("weld-resistance:{}", conn.id),
        title: loc(&format!("Fillet weld resistance over the throat – {}", conn.id), &format!("Kehlnahtwiderstand über der Nahtdicke – {}", conn.id)),
        description: loc(
            "Resistance per unit length of the fillet weld (EN 1999-1-1, 8.6): the smaller of the weld metal and the HAZ fusion boundary, against the design force per length; the diamond is the current throat, drag it to change welds.throat.",
            "Widerstand der Kehlnaht je Längeneinheit (EN 1999-1-1, 8.6): der kleinere Wert aus Schweißgut und WEZ-Schmelzgrenze gegenüber der Bemessungskraft je Länge; die Raute ist die aktuelle Nahtdicke, ziehen ändert welds.throat.",
        ),
        axes: vec![axis("Throat a", "Nahtdicke a", QuantityKind::Length, "m", AxisScale::Linear, Some((THROAT_MIN, THROAT_MAX))), axis("Resistance per length", "Widerstand je Länge", QuantityKind::ForcePerLength, "kN/m", AxisScale::Linear, Some((0.0, top)))],
        series,
        markers: vec![marker],
        annotations: Vec::new(),
        focus: connection_focus(conn),
        legend: true,
        table_twin: DataTable { columns: vec![column("Throat a", "Nahtdicke a", "m", throats.clone()), column("Weld metal", "Schweißgut", "kN/m", metal), column("HAZ boundary", "WEZ-Schmelzgrenze", "kN/m", haz)] },
    })
}
