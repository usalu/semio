//! 📈️ Norm diagrams of the instantaneous centre bolt group, the fatigue stress range, the buckling length nomogram, the cold forming gain, the screw and rivet bearing and the material ductility. See [EN 1993](https://eurocodes.jrc.ec.europa.eu/).

use crate::checks::clauses::{effective_length as el, fasteners as fs};
use crate::checks::{bolt_group_ic_checks, effective_length_checks, fastener_checks, forming_gain_checks, stress_range_checks, AnnexParams};
use crate::model::{BucklingLength, En1993Snapshot, FastenedJoint, FatigueStress, FormingGain, JointPlate, MaterialDuctility};
use crate::norm::CheckReport;
use crate::scene::diagram_kit::sn_curves;
use crate::scene::projection_kit::diagram_parts::{axis, column, stroke};
use crate::scene::*;

const SAMPLES: usize = 40;

fn loc(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

fn tone(ratio: f64) -> Tone {
    crate::scene::utilisation_tone(ratio)
}

fn worst_of(report: &CheckReport, entity: &str) -> f64 {
    report.checks.iter().filter(|c| c.subject.entity_id == entity && c.utilization.is_finite()).map(|c| c.utilization).fold(0.0, f64::max)
}

fn scatter(points: &[(f64, f64)], tone: Tone) -> Series {
    Series::Scatter { id: "points".into(), label: loc("Bolts", "Schrauben"), points: points.iter().map(|q| ScatterPoint { at: Vec2::new(q.0, q.1), tone, label: None }).collect() }
}

fn bolt_group(d: &En1993Snapshot, report: &CheckReport, p: &JointPlate) -> Option<Diagram2d> {
    let view = bolt_group_ic_checks::view(d, p, AnnexParams::for_choice(d.annex).gamma_m2)?;
    let f_max = view.elastic.iter().map(|q| q.0.hypot(q.1)).fold(1e-9, f64::max);
    let extent = view.points.iter().map(|q| q.0.hypot(q.1)).fold(1.0, f64::max);
    let scale = 0.5 * extent / f_max;
    let utilisation = worst_of(report, &p.id);
    let ray = |id: String, label: LocalizedCopy, at: (f64, f64), v: (f64, f64), style: Stroke| Series::Ray { id, label, origin: Vec2::new(at.0, at.1), through: Vec2::new(at.0 + v.0 * scale, at.1 + v.1 * scale), to_boundary: false, style };
    let mut series = vec![scatter(&view.points, Tone::Neutral)];
    for (i, (at, v)) in view.points.iter().zip(&view.elastic).enumerate() {
        series.push(ray(format!("elastic-{i}"), loc("Elastic bolt force", "Elastische Schraubenkraft"), *at, *v, stroke(Tone::Info, 1.5, Dash::Dashed)));
    }
    if let Some(plastic) = &view.plastic {
        for (i, (at, v)) in view.points.iter().zip(plastic).enumerate() {
            series.push(ray(format!("plastic-{i}"), loc("Bolt force at the instantaneous centre", "Schraubenkraft am Momentanpol"), *at, *v, stroke(tone(utilisation), 2.5, Dash::Solid)));
        }
    }
    let markers = view.centre.map(|c| Marker { id: "ic".into(), at: Vec2::new(c.0, c.1), shape: MarkerShape::Circle, tone: tone(utilisation), label: loc("Instantaneous centre", "Momentanpol"), utilisation: Some(utilisation), bound_check: Some(format!("en1993.1-8.3.12.ic.{}", p.id)), binding: None }).into_iter().collect();
    let force = |v: &(f64, f64)| v.0.hypot(v.1);
    Some(Diagram2d {
        id: format!("bolt-group:{}", p.id),
        title: loc(&format!("Bolt group forces – {}", p.id), &format!("Schraubenkräfte der Gruppe – {}", p.id)),
        description: loc(
            &format!("Bolt forces of the group (EN 1993-1-8 3.12): dashed the elastic vector method, solid the plastic distribution about the instantaneous centre; R_ult = {:.1} kN, largest elastic force {:.1} kN{}.", view.r_ult / 1e3, f_max / 1e3, view.factor.map_or(String::new(), |k| format!(", the group carries {k:.2} times the load"))),
            &format!("Schraubenkräfte der Gruppe (EN 1993-1-8 3.12): gestrichelt das elastische Vektorverfahren, durchgezogen die plastische Verteilung um den Momentanpol; R_ult = {:.1} kN, größte elastische Kraft {:.1} kN{}.", view.r_ult / 1e3, f_max / 1e3, view.factor.map_or(String::new(), |k| format!(", die Gruppe trägt das {k:.2}-fache der Last"))),
        ),
        axes: vec![axis("Across x", "Quer x", QuantityKind::Length, "mm", AxisScale::Linear, Some((-1.8 * extent, 1.8 * extent))), axis("Along y", "Längs y", QuantityKind::Length, "mm", AxisScale::Linear, Some((-1.8 * extent, 1.8 * extent)))],
        series,
        markers,
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Group, &p.id, format!("jointPlates[id={}]", p.id))),
        legend: false,
        table_twin: DataTable { columns: vec![
            column("Bolt x", "Schraube x", "mm", view.points.iter().map(|q| q.0).collect()),
            column("Bolt y", "Schraube y", "mm", view.points.iter().map(|q| q.1).collect()),
            column("Elastic force", "Elastische Kraft", "N", view.elastic.iter().map(force).collect()),
            column("Plastic force", "Plastische Kraft", "N", view.plastic.as_ref().map_or_else(|| vec![f64::NAN; view.points.len()], |v| v.iter().map(force).collect())),
        ] },
    })
}

fn fatigue_range(d: &En1993Snapshot, report: &CheckReport, s: &FatigueStress) -> Option<Diagram2d> {
    let r = stress_range_checks::derive(d, s, d.annex)?;
    let detail = d.fatigue_details.iter().find(|x| x.id == s.detail_id)?;
    let strength = r.strength / r.gamma_mf / 1e6;
    let curve = sn_curves::sn_curve(strength, 1.0e4, 1.0e8);
    let utilisation = worst_of(report, &s.id).max(r.damage);
    let point = Vec2::new(s.cycles.clamp(1.0e4, 1.0e8), (r.design / 1e6).clamp(10.0, 1000.0));
    Some(Diagram2d {
        id: format!("fatigue-range:{}", s.id),
        title: loc(&format!("Stress range of the load model – {}", s.id), &format!("Spannungsschwingbreite des Lastmodells – {}", s.id)),
        description: loc(
            &format!("S–N curve of category {} with k_s={:.3} and γ_Mf={} (EN 1993-1-9 7.1) and the design range γ_Ff k_f Δσ={:.1} MPa over {:.2e} cycles; damage D={:.3}.", detail.category, r.size_effect, r.gamma_mf, r.design / 1e6, s.cycles, r.damage),
            &format!("Wöhlerlinie des Kerbfalls {} mit k_s={:.3} und γ_Mf={} (EN 1993-1-9 7.1) und die Bemessungsschwingbreite γ_Ff k_f Δσ={:.1} MPa bei {:.2e} Lastspielen; Schädigung D={:.3}.", detail.category, r.size_effect, r.gamma_mf, r.design / 1e6, s.cycles, r.damage),
        ),
        axes: vec![axis("Cycles N", "Lastspiele N", QuantityKind::Dimensionless, "1", AxisScale::Log10, Some((1.0e4, 1.0e8))), axis("Stress range Δσ", "Spannungsschwingbreite Δσ", QuantityKind::Stress, "MPa", AxisScale::Log10, Some((10.0, 1000.0)))],
        series: vec![Series::Polyline { id: "sn".into(), label: loc("k_s Δσ_C / γ_Mf", "k_s Δσ_C / γ_Mf"), points: curve.iter().map(|q| Vec2::new(q[0], q[1])).collect(), style: stroke(Tone::Info, 2.0, Dash::Solid) }],
        markers: vec![Marker { id: "design".into(), at: point, shape: MarkerShape::Circle, tone: tone(utilisation), label: loc(&format!("Δσ = {:.1} MPa, n = {:.2e}", r.design / 1e6, s.cycles), &format!("Δσ = {:.1} MPa, n = {:.2e}", r.design / 1e6, s.cycles)), utilisation: Some(utilisation), bound_check: Some(format!("en1993.1-9.5.range.{}", s.id)), binding: None }],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Group, &s.id, format!("fatigueStresses[id={}]", s.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Cycles N", "Lastspiele N", "1", curve.iter().map(|q| q[0]).collect()), column("Strength", "Festigkeit", "MPa", curve.iter().map(|q| q[1]).collect())] },
    })
}

fn nomogram(d: &En1993Snapshot, report: &CheckReport, b: &BucklingLength) -> Option<Diagram2d> {
    let (ga, gb, k) = effective_length_checks::factors(b)?;
    let factor = |a: f64, c: f64| match b.sway {
        crate::model::FrameSway::NonSway => Some(el::k_non_sway(a, c)),
        crate::model::FrameSway::Sway => el::k_sway(a, c),
    };
    let curve = |c: f64| -> Vec<Vec2> { (0..=SAMPLES).filter_map(|i| { let a = 10.0 * i as f64 / SAMPLES as f64; factor(a, c).map(|k| Vec2::new(a, k)) }).collect() };
    let at_b = curve(gb.min(10.0));
    let (low, high) = (curve(0.0), curve(10.0));
    let utilisation = worst_of(report, &b.id);
    let (lo, hi) = match b.sway {
        crate::model::FrameSway::NonSway => (0.5, 1.0),
        crate::model::FrameSway::Sway => (1.0, 6.0),
    };
    let _ = d;
    Some(Diagram2d {
        id: format!("nomogram:{}", b.id),
        title: loc(&format!("Effective length factor – {}", b.id), &format!("Knicklängenbeiwert – {}", b.id)),
        description: loc(
            &format!("Nomogram of the effective length factor K over G_A for G_B = {:.2} (and the limits G_B = 0 and 10); the marker is the column with G_A = {:.2}, K = {k:.3}.", gb.min(10.0), ga.min(10.0)),
            &format!("Nomogramm des Knicklängenbeiwerts K über G_A für G_B = {:.2} (und die Grenzen G_B = 0 und 10); die Marke ist die Stütze mit G_A = {:.2}, K = {k:.3}.", gb.min(10.0), ga.min(10.0)),
        ),
        axes: vec![axis("Distribution factor G_A", "Verteilungsfaktor G_A", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, 10.0))), axis("Effective length factor K", "Knicklängenbeiwert K", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((lo, hi)))],
        series: vec![
            Series::Polyline { id: "gb-low".into(), label: loc("G_B = 0", "G_B = 0"), points: low, style: stroke(Tone::Neutral, 1.0, Dash::Dotted) },
            Series::Polyline { id: "gb".into(), label: loc("G_B of the column", "G_B der Stütze"), points: at_b.clone(), style: stroke(Tone::Info, 2.5, Dash::Solid) },
            Series::Polyline { id: "gb-high".into(), label: loc("G_B = 10", "G_B = 10"), points: high, style: stroke(Tone::Neutral, 1.0, Dash::Dashed) },
        ],
        markers: vec![Marker { id: "column".into(), at: Vec2::new(ga.min(10.0), k), shape: MarkerShape::Circle, tone: tone(utilisation), label: loc(&format!("K = {k:.3}"), &format!("K = {k:.3}")), utilisation: Some(utilisation), bound_check: Some(format!("en1993.1-1.6.3.1.3.lcr.{}", b.id)), binding: None }],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Group, &b.id, format!("bucklingLengths[id={}]", b.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("G_A", "G_A", "1", at_b.iter().map(|q| q.x).collect()), column("K", "K", "1", at_b.iter().map(|q| q.y).collect())] },
    })
}

fn forming(d: &En1993Snapshot, report: &CheckReport, g: &FormingGain) -> Option<Diagram2d> {
    let m = d.cold_formed_members.iter().find(|m| m.id == g.member_id)?;
    if m.fy <= 0.0 || m.thickness <= 0.0 || g.gross_area <= 0.0 || g.ultimate_strength <= m.fy {
        return None;
    }
    let counted = forming_gain_checks::bends_counted(g, m.thickness, m.corner_radius);
    let top = (2 * g.bends.max(4)) as usize;
    let curve: Vec<Vec2> = (0..=top).map(|n| Vec2::new(n as f64, forming_gain_checks::average_yield(g, n as f64, m.fy, m.thickness) / 1e6)).collect();
    let fya = forming_gain_checks::average_yield(g, counted, m.fy, m.thickness);
    let used = if g.used_yield > 0.0 { g.used_yield } else { m.fy };
    let utilisation = worst_of(report, &g.id);
    Some(Diagram2d {
        id: format!("forming-gain:{}", g.id),
        title: loc(&format!("Enhanced yield strength – {}", g.id), &format!("Erhöhte Streckgrenze – {}", g.id)),
        description: loc(
            &format!("Average yield strength f_ya over the number of 90° bends (EN 1993-1-3 3.2.2), capped at (f_u+f_yb)/2 = {:.1} MPa; f_ya = {:.1} MPa for {counted:.0} counted bends, the design uses {:.1} MPa.", (g.ultimate_strength + m.fy) / 2e6, fya / 1e6, used / 1e6),
            &format!("Mittlere Streckgrenze f_ya über die Anzahl der 90°-Biegungen (EN 1993-1-3 3.2.2), begrenzt auf (f_u+f_yb)/2 = {:.1} MPa; f_ya = {:.1} MPa bei {counted:.0} angesetzten Biegungen, angesetzt sind {:.1} MPa.", (g.ultimate_strength + m.fy) / 2e6, fya / 1e6, used / 1e6),
        ),
        axes: vec![axis("Bends n", "Biegungen n", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, top as f64))), axis("Yield strength", "Streckgrenze", QuantityKind::Stress, "MPa", AxisScale::Linear, None)],
        series: vec![
            Series::Polyline { id: "fya".into(), label: loc("f_ya", "f_ya"), points: curve.clone(), style: stroke(Tone::Info, 2.5, Dash::Solid) },
            Series::Limit { id: "cap".into(), label: loc("(f_u + f_yb)/2", "(f_u + f_yb)/2"), axis: AxisRef::Y, value: (g.ultimate_strength + m.fy) / 2e6, style: stroke(Tone::Neutral, 1.0, Dash::Dashed) },
            Series::Limit { id: "used".into(), label: loc("Yield strength used", "Angesetzte Streckgrenze"), axis: AxisRef::Y, value: used / 1e6, style: stroke(tone(utilisation), 1.5, Dash::Dotted) },
        ],
        markers: vec![Marker { id: "bends".into(), at: Vec2::new(counted, fya / 1e6), shape: MarkerShape::Circle, tone: tone(utilisation), label: loc(&format!("f_ya = {:.1} MPa", fya / 1e6), &format!("f_ya = {:.1} MPa", fya / 1e6)), utilisation: Some(utilisation), bound_check: Some(format!("en1993.1-3.3.2.2.fya.{}", g.id)), binding: None }],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Group, &g.id, format!("formingGains[id={}]", g.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Bends n", "Biegungen n", "1", curve.iter().map(|q| q.x).collect()), column("f_ya", "f_ya", "MPa", curve.iter().map(|q| q.y).collect())] },
    })
}

fn fastener(d: &En1993Snapshot, report: &CheckReport, f: &FastenedJoint) -> Option<Diagram2d> {
    let gm2 = AnnexParams::for_choice(d.annex).gamma_m2;
    let r = fastener_checks::resistance(f, gm2)?;
    let (t1, fu1, t2) = if f.head_thickness <= f.base_thickness { (f.head_thickness, f.head_strength, f.base_thickness) } else { (f.base_thickness, f.base_strength, f.head_thickness) };
    let kind = if f.kind == crate::model::FastenerKind::SelfTappingScrew { fs::Kind::Screw } else { fs::Kind::Rivet };
    let curve: Vec<Vec2> = (0..=SAMPLES).map(|i| { let ratio = 0.5 + 3.0 * i as f64 / SAMPLES as f64; Vec2::new(ratio, fs::bearing(kind, t1, t1 * ratio, f.diameter, fu1, gm2) / 1e3) }).collect();
    let ed = f.shear_force.abs() / f64::from(f.count.max(1)) / 1e3;
    let utilisation = worst_of(report, &f.id);
    Some(Diagram2d {
        id: format!("fastener:{}", f.id),
        title: loc(&format!("Fastener bearing resistance – {}", f.id), &format!("Lochleibungswiderstand des Verbindungsmittels – {}", f.id)),
        description: loc(
            &format!("Bearing resistance F_b,Rd of one fastener over the thickness ratio t_2/t_1 (EN 1993-1-3 8.3, linear between the equal-thickness and the thick-support formula); the dashed line is F_v,Ed = {ed:.2} kN per fastener, the marker the joint at t_2/t_1 = {:.2}.", t2 / t1),
            &format!("Lochleibungswiderstand F_b,Rd eines Verbindungsmittels über dem Dickenverhältnis t_2/t_1 (EN 1993-1-3 8.3, linear zwischen der Formel für gleiche Dicken und der für dicke Unterlage); gestrichelt F_v,Ed = {ed:.2} kN je Verbindungsmittel, die Marke zeigt die Verbindung bei t_2/t_1 = {:.2}.", t2 / t1),
        ),
        axes: vec![axis("Thickness ratio t_2/t_1", "Dickenverhältnis t_2/t_1", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.5, 3.5))), axis("Resistance F_b,Rd", "Widerstand F_b,Rd", QuantityKind::Force, "kN", AxisScale::Linear, None)],
        series: vec![
            Series::Polyline { id: "bearing".into(), label: loc("F_b,Rd", "F_b,Rd"), points: curve.clone(), style: stroke(Tone::Info, 2.5, Dash::Solid) },
            Series::Limit { id: "ed".into(), label: loc("F_v,Ed per fastener", "F_v,Ed je Verbindungsmittel"), axis: AxisRef::Y, value: ed, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
        ],
        markers: vec![Marker { id: "joint".into(), at: Vec2::new(t2 / t1, r.bearing / 1e3), shape: MarkerShape::Circle, tone: tone(utilisation), label: loc(&format!("F_b,Rd = {:.2} kN", r.bearing / 1e3), &format!("F_b,Rd = {:.2} kN", r.bearing / 1e3)), utilisation: Some(utilisation), bound_check: Some(format!("en1993.1-3.8.3.shear.{}", f.id)), binding: None }],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Group, &f.id, format!("fastenedJoints[id={}]", f.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("t_2/t_1", "t_2/t_1", "1", curve.iter().map(|q| q.x).collect()), column("F_b,Rd", "F_b,Rd", "kN", curve.iter().map(|q| q.y).collect())] },
    })
}

fn ductility(d: &En1993Snapshot, report: &CheckReport, c: &MaterialDuctility) -> Option<Diagram2d> {
    let m = d.materials.iter().find(|m| m.id == c.material_id)?;
    if m.fy <= 0.0 || m.fu <= 0.0 || m.e_modulus <= 0.0 || c.ultimate_strain <= 0.0 {
        return None;
    }
    let eps_y = m.fy / m.e_modulus;
    let required = semio_s_plugin_norm_compliance::ndp_common("ndp.en1993.1-1.ductility.eps-u-factor") * eps_y;
    let curve = vec![Vec2::new(0.0, 0.0), Vec2::new(eps_y, m.fy / 1e6), Vec2::new(c.ultimate_strain, m.fu / 1e6)];
    let utilisation = worst_of(report, &c.id).max(required / c.ultimate_strain);
    Some(Diagram2d {
        id: format!("ductility:{}", c.id),
        title: loc(&format!("Ductility of the material – {}", c.material_id), &format!("Duktilität des Werkstoffs – {}", c.material_id)),
        description: loc(
            &format!("Idealised stress–strain curve through (ε_y, f_y) and (ε_u, f_u); the dashed line marks the required ε_u = 15 ε_y = {required:.4}, the material reaches ε_u = {:.4}.", c.ultimate_strain),
            &format!("Idealisierte Spannungs-Dehnungs-Linie durch (ε_y, f_y) und (ε_u, f_u); gestrichelt die geforderte Dehnung ε_u = 15 ε_y = {required:.4}, der Werkstoff erreicht ε_u = {:.4}.", c.ultimate_strain),
        ),
        axes: vec![axis("Strain ε", "Dehnung ε", QuantityKind::Dimensionless, "1", AxisScale::Linear, None), axis("Stress σ", "Spannung σ", QuantityKind::Stress, "MPa", AxisScale::Linear, None)],
        series: vec![
            Series::Polyline { id: "curve".into(), label: loc("σ(ε)", "σ(ε)"), points: curve.clone(), style: stroke(Tone::Info, 2.5, Dash::Solid) },
            Series::Limit { id: "required".into(), label: loc("ε_u ≥ 15 ε_y", "ε_u ≥ 15 ε_y"), axis: AxisRef::X, value: required, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
        ],
        markers: vec![Marker { id: "ultimate".into(), at: Vec2::new(c.ultimate_strain, m.fu / 1e6), shape: MarkerShape::Circle, tone: tone(utilisation), label: loc(&format!("ε_u = {:.4}", c.ultimate_strain), &format!("ε_u = {:.4}", c.ultimate_strain)), utilisation: Some(utilisation), bound_check: Some(format!("en1993.3.2.2.strain.{}", c.id)), binding: None }],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Group, &c.id, format!("materialDuctility[id={}]", c.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Strain ε", "Dehnung ε", "1", curve.iter().map(|q| q.x).collect()), column("Stress σ", "Spannung σ", "MPa", curve.iter().map(|q| q.y).collect())] },
    })
}

/// 📈️ The diagrams of the norm entities of wave 3; `focus` limits them to one entity id.
pub(crate) fn norm_diagrams(d: &En1993Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    let wanted = |id: &str, related: &str| focus.is_none_or(|f| f == id || f == related);
    let mut out = Vec::new();
    for p in d.joint_plates.iter().filter(|p| wanted(&p.id, &p.joint_id)) {
        out.extend(bolt_group(d, report, p));
    }
    for s in d.fatigue_stresses.iter().filter(|s| wanted(&s.id, &s.detail_id)) {
        out.extend(fatigue_range(d, report, s));
    }
    for b in d.buckling_lengths.iter().filter(|b| wanted(&b.id, &b.member_id)) {
        out.extend(nomogram(d, report, b));
    }
    for g in d.forming_gains.iter().filter(|g| wanted(&g.id, &g.member_id)) {
        out.extend(forming(d, report, g));
    }
    for f in d.fastened_joints.iter().filter(|f| wanted(&f.id, &f.member_id)) {
        out.extend(fastener(d, report, f));
    }
    for c in d.material_ductility.iter().filter(|c| wanted(&c.id, &c.material_id)) {
        out.extend(ductility(d, report, c));
    }
    out
}
