//! 📈️ Design-point diagrams: the sheet pile M–V interaction, the bridge S–N curve with the damage-equivalent range, the crane runway wheel position envelope, the plated panel ρ(λ̄_p) curve, one stacked load-case diagram per load case, the N, V and M courses along every frame bar and the utilisation ladder of every collection that has no dedicated curve.

use super::checks::clauses::{bridge_fatigue, crane_runway, effective_width, sheet_piling, toughness};
use super::checks::{frame_envelope, frame_eigen, joint_frame, part_1_1, part_3, toughness_point, tower_design_point, AnnexParams};
use super::model::{SteelMember, BridgeFatigue, TowerLeg, CraneRunway, En1993Snapshot, LoadCase, PlatedPanel, SheetPileEnvironment, SteelPile};
use super::norm::{CheckReport, CheckResult};
use super::scene::diagram_kit::{sn_curves, Xy};
use super::scene::projection_kit::diagram_parts::{axis, column, stroke};
use super::scene::*;

const SAMPLES: usize = 40;

fn loc(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

fn tone(ratio: f64) -> Tone {
    super::scene::utilisation_tone(ratio)
}

fn worst(report: &CheckReport, entity: &str, prefix: &str) -> f64 {
    report.checks.iter().filter(|c| c.subject.entity_id == entity && c.id.starts_with(prefix) && c.utilization.is_finite()).map(|c| c.utilization).fold(0.0, f64::max)
}

fn bar(id: String, label: LocalizedCopy, (x0, x1): (f64, f64), (y0, y1): (f64, f64), tone: Tone) -> Series {
    Series::Polygon { id, label, ring: vec![Vec2::new(x0, y0), Vec2::new(x1, y0), Vec2::new(x1, y1), Vec2::new(x0, y1)], fill: Fill { tone, opacity: 0.55 }, hatch: None }
}

fn pile_resistance(d: &En1993Snapshot, pile: &SteelPile) -> Option<(Vec<Xy>, f64, f64)> {
    let env = match pile.environment {
        SheetPileEnvironment::UndisturbedSoil => sheet_piling::Environment::UndisturbedSoil,
        SheetPileEnvironment::PollutedSoil => sheet_piling::Environment::PollutedSoil,
        SheetPileEnvironment::CompactedFill => sheet_piling::Environment::CompactedFill,
        SheetPileEnvironment::NonCompactedFill => sheet_piling::Environment::NonCompactedFill,
        _ => return None,
    };
    let material = d.materials.iter().find(|m| m.id == pile.material_id)?;
    if !(pile.section_modulus > 0.0 && pile.wall_thickness > 0.0 && pile.shear_area > 0.0) {
        return None;
    }
    let (fy, gm0) = (material.fy / 1e6, AnnexParams::for_choice(d.annex).gamma_m0);
    let t = pile.wall_thickness * 1e3;
    let loss = sheet_piling::corrosion_loss(env, pile.design_life);
    let ratio = sheet_piling::net_thickness(t, loss, loss) / t;
    let v_pl = sheet_piling::v_pl_rd_pile(pile.shear_area * 1e6 * ratio, fy, gm0);
    let m_at = |v: f64| sheet_piling::m_c_rd_pile(pile.interlock_bending_factor, pile.section_modulus * 1e9 * ratio, sheet_piling::fy_reduced_by_shear(v, v_pl, fy), gm0);
    let curve = (0..=SAMPLES).map(|i| v_pl * i as f64 / SAMPLES as f64).map(|v| [v / 1e3, m_at(v) / 1e6]).collect();
    Some((curve, v_pl / 1e3, m_at(0.0) / 1e6))
}

fn pile_interaction(d: &En1993Snapshot, report: &CheckReport, pile: &SteelPile) -> Option<Diagram2d> {
    let (curve, v_rd, m_rd) = pile_resistance(d, pile)?;
    let (v_ed, m_ed) = (pile.design_shear / 1e3, pile.design_moment / 1e3);
    let utilisation = worst(report, &pile.id, "en1993.5.5.2.");
    let ring: Vec<Vec2> = std::iter::once(Vec2::ZERO).chain(curve.iter().map(|q| Vec2::new(q[0], q[1]))).chain(std::iter::once(Vec2::new(v_rd, 0.0))).collect();
    Some(Diagram2d {
        id: format!("pile-mv:{}", pile.id),
        title: loc(&format!("Sheet pile M–V interaction – {}", pile.id), &format!("Spundwand M-V-Interaktion – {}", pile.id)),
        description: loc("Moment resistance M_c,Rd over the shear force V (EN 1993-5, 5.2.2) after corrosion; above V = 0.5 V_pl,Rd the yield strength is reduced. The marker is the design effect (V_Ed, M_Ed).", "Biegetragfähigkeit M_c,Rd über der Querkraft V (EN 1993-5, 5.2.2) nach Korrosion; oberhalb V = 0,5 V_pl,Rd wird die Streckgrenze abgemindert. Die Marke ist die Beanspruchung (V_Ed, M_Ed)."),
        axes: vec![axis("Shear force V", "Querkraft V", QuantityKind::Force, "kN", AxisScale::Linear, Some((0.0, v_rd.max(v_ed) * 1.15))), axis("Bending moment M", "Biegemoment M", QuantityKind::Moment, "kN·m", AxisScale::Linear, Some((0.0, m_rd.max(m_ed) * 1.15)))],
        series: vec![Series::Polygon { id: "capacity".into(), label: loc("M–V capacity", "M-V-Tragfähigkeit"), ring, fill: Fill { tone: Tone::Info, opacity: 0.15 }, hatch: None }],
        markers: vec![Marker { id: "demand".into(), at: Vec2::new(v_ed, m_ed), shape: MarkerShape::Diamond, tone: tone(utilisation), label: loc(&format!("V_Ed = {v_ed:.0} kN, M_Ed = {m_ed:.1} kN·m"), &format!("V_Ed = {v_ed:.0} kN, M_Ed = {m_ed:.1} kN·m")), utilisation: Some(utilisation), bound_check: Some(format!("en1993.5.5.2.m.{}", pile.id)), binding: None }],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Group, &pile.id, format!("piles[id={}]", pile.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Shear force V", "Querkraft V", "kN", curve.iter().map(|q| q[0]).collect()), column("Moment resistance M_c,Rd", "Momententragfähigkeit M_c,Rd", "kN·m", curve.iter().map(|q| q[1]).collect())] },
    })
}

fn bridge_sn(d: &En1993Snapshot, bf: &BridgeFatigue) -> Option<Diagram2d> {
    if !(bf.heavy_vehicle_weight > 0.0 && bf.reference_weight > 0.0 && bf.delta_sigma_p > 0.0) {
        return None;
    }
    let params = AnnexParams::for_choice(d.annex);
    let l2 = bridge_fatigue::lambda_2(bf.heavy_vehicle_weight, bf.reference_weight, bf.observed_traffic, 0.5e6);
    let l3 = bridge_fatigue::lambda_3(bf.design_life);
    let l4 = bridge_fatigue::lambda_4(&[(bf.second_lane_traffic, bf.second_lane_weight)], bf.observed_traffic, bf.heavy_vehicle_weight);
    let delta_e2 = bridge_fatigue::delta_sigma_e2_bridge(bf.lambda_1, l2, l3, l4, if bf.lambda_max > 0.0 { bf.lambda_max } else { f64::INFINITY }, bf.phi2, bf.delta_sigma_p / 1e6);
    let delta_c = bridge_fatigue::orthotropic_detail(&bf.detail_id).map_or(f64::from(bf.category), |c| c.min(f64::from(bf.category))) * bridge_fatigue::size_effect(bf.plate_thickness * 1e3);
    let utilisation = bridge_fatigue::fatigue_utilisation(delta_e2, delta_c, 1.0, params.gamma_mf);
    if !(delta_e2 > 0.0 && delta_c > 0.0 && utilisation.is_finite()) {
        return None;
    }
    let curve = sn_curves::sn_curve(delta_c, 1.0e4, 1.0e8);
    Some(Diagram2d {
        id: format!("bridge-sn:{}", bf.id),
        title: loc(&format!("S–N curve and damage-equivalent range – {}", bf.id), &format!("Wöhlerlinie und schadensäquivalente Schwingbreite – {}", bf.id)),
        description: loc(&format!("Fatigue strength Δσ_R of the detail category Δσ_C = {delta_c:.1} MPa (EN 1993-1-9) in log–log scale with the damage-equivalent range γ_Mf Δσ_E,2 = {:.1} MPa at 2·10⁶ cycles (EN 1993-2, 9.5.2).", delta_e2 * params.gamma_mf), &format!("Ermüdungsfestigkeit Δσ_R der Kerbfallklasse Δσ_C = {delta_c:.1} MPa (EN 1993-1-9) doppeltlogarithmisch mit der schadensäquivalenten Schwingbreite γ_Mf Δσ_E,2 = {:.1} MPa bei 2·10⁶ Lastspielen (EN 1993-2, 9.5.2).", delta_e2 * params.gamma_mf)),
        axes: vec![axis("Cycles N", "Lastspiele N", QuantityKind::Dimensionless, "1", AxisScale::Log10, Some((1.0e4, 1.0e8))), axis("Stress range Δσ", "Spannungsschwingbreite Δσ", QuantityKind::Stress, "MPa", AxisScale::Log10, Some((10.0, 1000.0)))],
        series: vec![Series::Polyline { id: "wohler".into(), label: loc(&format!("Δσ_C = {delta_c:.1} MPa"), &format!("Δσ_C = {delta_c:.1} MPa")), points: curve.iter().map(|q| Vec2::new(q[0], q[1])).collect(), style: stroke(Tone::Info, 2.0, Dash::Solid) }],
        markers: vec![Marker { id: "design".into(), at: Vec2::new(2.0e6, delta_e2 * params.gamma_mf), shape: MarkerShape::Diamond, tone: tone(utilisation), label: loc(&format!("γ_Mf Δσ_E,2 = {:.1} MPa", delta_e2 * params.gamma_mf), &format!("γ_Mf Δσ_E,2 = {:.1} MPa", delta_e2 * params.gamma_mf)), utilisation: Some(utilisation), bound_check: Some(format!("en1993.2.9.5.{}", bf.id)), binding: None }],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Group, &bf.id, format!("bridgeFatigue[id={}]", bf.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Cycles N", "Lastspiele N", "1", curve.iter().map(|q| q[0]).collect()), column("Strength Δσ_R", "Festigkeit Δσ_R", "MPa", curve.iter().map(|q| q[1]).collect())] },
    })
}

fn crane_wheel(d: &En1993Snapshot, report: &CheckReport, c: &CraneRunway) -> Option<Diagram2d> {
    let member = d.members.iter().find(|m| m.id == c.member_id)?;
    if !(member.length > 0.0 && c.max_wheel_load > 0.0) {
        return None;
    }
    let (l, f) = (member.length, crane_runway::wheel_load_design(c.max_wheel_load, c.phi, c.load_factor) / 1e3);
    let stations: Vec<f64> = (0..=SAMPLES).map(|i| l * i as f64 / SAMPLES as f64).collect();
    let moments: Vec<f64> = stations.iter().map(|x| f * x * (l - x) / l).collect();
    let utilisation = worst(report, &c.id, "en1993.6.");
    Some(Diagram2d {
        id: format!("crane-wheel:{}", c.id),
        title: loc(&format!("Wheel load positions on the runway – {}", c.id), &format!("Radlaststellungen auf der Kranbahn – {}", c.id)),
        description: loc(&format!("Envelope of the bending moment of a simply supported girder (span {l:.2} m) while one wheel F = {f:.0} kN crosses it: M(x) = F x (L − x)/L; the marker is the governing wheel position at midspan."), &format!("Einhüllende des Biegemoments eines Einfeldträgers (Spannweite {l:.2} m) beim Überfahren eines Rades F = {f:.0} kN: M(x) = F x (L − x)/L; die Marke ist die maßgebende Radstellung in Feldmitte.")),
        axes: vec![axis("Wheel position x", "Radposition x", QuantityKind::Length, "m", AxisScale::Linear, Some((0.0, l))), axis("Moment M", "Moment M", QuantityKind::Moment, "kN·m", AxisScale::Linear, Some((0.0, f * l / 4.0 * 1.2)))],
        series: vec![Series::Polyline { id: "envelope".into(), label: loc("M(x) envelope", "M(x)-Einhüllende"), points: stations.iter().zip(&moments).map(|(x, m)| Vec2::new(*x, *m)).collect(), style: stroke(Tone::Info, 2.5, Dash::Solid) }],
        markers: vec![Marker { id: "wheel".into(), at: Vec2::new(l / 2.0, f * l / 4.0), shape: MarkerShape::Circle, tone: tone(utilisation), label: loc(&format!("F = {f:.0} kN, M = {:.0} kN·m", f * l / 4.0), &format!("F = {f:.0} kN, M = {:.0} kN·m", f * l / 4.0)), utilisation: Some(utilisation), bound_check: Some(format!("en1993.6.5.7.vm.{}", c.id)), binding: None }],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Group, &c.id, format!("craneRunways[id={}]", c.id))),
        legend: false,
        table_twin: DataTable { columns: vec![column("Wheel position x", "Radposition x", "m", stations), column("Moment M", "Moment M", "kN·m", moments)] },
    })
}

fn panel_rho(report: &CheckReport, p: &PlatedPanel) -> Option<Diagram2d> {
    if !(p.b > 0.0 && p.thickness > 0.0 && p.fy > 0.0 && p.k_sigma > 0.0) {
        return None;
    }
    let eps = (235.0e6 / p.fy).sqrt();
    let lambda = effective_width::lambda_p(p.b, p.thickness, eps, p.k_sigma);
    let rho = effective_width::rho_internal(lambda, 1.0);
    let curve: Vec<Vec2> = (0..=SAMPLES).map(|i| 0.2 + 2.8 * i as f64 / SAMPLES as f64).map(|x| Vec2::new(x, effective_width::rho_internal(x, 1.0))).collect();
    let utilisation = worst(report, &p.id, "en1993.1-5.");
    Some(Diagram2d {
        id: format!("panel-rho:{}", p.id),
        title: loc(&format!("Plate buckling reduction ρ(λ̄_p) – {}", p.id), &format!("Beulabminderung ρ(λ̄_p) – {}", p.id)),
        description: loc(&format!("Reduction factor ρ of an internal plate element under uniform compression (EN 1993-1-5, 4.4) over the plate slenderness; this panel has λ̄_p = {lambda:.3} and ρ = {rho:.3} (k_σ = {:.2}).", p.k_sigma), &format!("Abminderungsfaktor ρ eines innenliegenden Plattenstreifens unter gleichmäßigem Druck (EN 1993-1-5, 4.4) über der Plattenschlankheit; dieses Feld hat λ̄_p = {lambda:.3} und ρ = {rho:.3} (k_σ = {:.2}).", p.k_sigma)),
        axes: vec![axis("Slenderness λ̄_p", "Schlankheit λ̄_p", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.2, 3.0))), axis("Reduction factor ρ", "Abminderungsfaktor ρ", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, 1.1)))],
        series: vec![Series::Polyline { id: "rho".into(), label: loc("ρ(λ̄_p)", "ρ(λ̄_p)"), points: curve.clone(), style: stroke(Tone::Info, 2.5, Dash::Solid) }],
        markers: vec![Marker { id: "panel".into(), at: Vec2::new(lambda.clamp(0.2, 3.0), rho), shape: MarkerShape::Circle, tone: tone(utilisation), label: loc(&format!("λ̄_p = {lambda:.2}, ρ = {rho:.2}"), &format!("λ̄_p = {lambda:.2}, ρ = {rho:.2}")), utilisation: Some(utilisation), bound_check: report.checks.iter().filter(|c| c.subject.entity_id == p.id && c.id.starts_with("en1993.1-5.") && c.utilization.is_finite()).max_by(|a, b| a.utilization.total_cmp(&b.utilization)).map(|c| c.id.clone()), binding: None }],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Group, &p.id, format!("platedPanels[id={}]", p.id))),
        legend: false,
        table_twin: DataTable { columns: vec![column("Slenderness λ̄_p", "Schlankheit λ̄_p", "1", curve.iter().map(|q| q.x).collect()), column("Reduction factor ρ", "Abminderungsfaktor ρ", "1", curve.iter().map(|q| q.y).collect())] },
    })
}

fn load_case_bars(d: &En1993Snapshot, case: &LoadCase) -> Vec<Diagram2d> {
    let actions: Vec<_> = d.member_actions.iter().filter(|a| a.load_case_id == case.id).collect();
    if actions.is_empty() {
        return Vec::new();
    }
    let build = |key: &str, en: &str, de: &str, unit: &str, kind: QuantityKind, value: &dyn Fn(&super::model::MemberAction) -> f64| -> Option<Diagram2d> {
        let values: Vec<f64> = actions.iter().map(|a| value(a)).collect();
        let peak = values.iter().fold(0.0_f64, |m, v| m.max(v.abs()));
        if peak <= 0.0 {
            return None;
        }
        let series = actions.iter().zip(&values).enumerate().map(|(i, (a, v))| bar(format!("bar-{}", a.id), loc(&a.member_id, &a.member_id), (i as f64 + 0.1, i as f64 + 0.9), if *v >= 0.0 { (0.0, *v) } else { (*v, 0.0) }, if *v >= 0.0 { Tone::Info } else { Tone::Warning })).collect();
        Some(Diagram2d {
            id: format!("load-case-{key}:{}", case.id),
            title: loc(&format!("{en} per member – load case {}", case.id), &format!("{de} je Stab – Lastfall {}", case.id)),
            description: loc(&format!("Characteristic {en} of every member action of load case {} ({}); positive upwards.", case.id, case.name), &format!("Charakteristische {de} jeder Stabeinwirkung des Lastfalls {} ({}); positiv nach oben.", case.id, case.name)),
            axes: vec![axis("Member action", "Stabeinwirkung", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, actions.len() as f64))), axis(en, de, kind, unit, AxisScale::Linear, Some((-peak * 1.2, peak * 1.2)))],
            series,
            markers: Vec::new(),
            annotations: Vec::new(),
            focus: Some(EntityRef::new(EntityKind::Group, &case.id, format!("loadCases[id={}]", case.id))),
            legend: true,
            table_twin: DataTable { columns: vec![column("Member action", "Stabeinwirkung", "1", (0..actions.len()).map(|i| i as f64 + 1.0).collect()), column(en, de, unit, values.clone())] },
        })
    };
    [build("n", "Axial force N", "Normalkraft N", "kN", QuantityKind::Force, &|a| a.action.n / 1e3), build("my", "Bending moment M_y", "Biegemoment M_y", "kN·m", QuantityKind::Moment, &|a| a.action.my / 1e3)].into_iter().flatten().collect()
}

fn frame_course(bar: &super::model::FrameBar, key: &str, en: &str, de: &str, unit: &str, kind: QuantityKind, first: &[frame_eigen::Station], second: &[frame_eigen::Station], amplified: Option<&[frame_eigen::Station]>, pick: &dyn Fn(&frame_eigen::Station) -> f64) -> Option<Diagram2d> {
    let curve = |stations: &[frame_eigen::Station]| -> Vec<Vec2> { stations.iter().map(|s| Vec2::new(s.x, pick(s) * 1e-3)).collect() };
    let (a, b) = (curve(first), curve(second));
    let c = amplified.map(curve);
    let peak = a.iter().chain(&b).chain(c.iter().flatten()).fold(0.0_f64, |m, p| m.max(p.y.abs()));
    let length = b.last().map_or(0.0, |p| p.x);
    if a.len() < 2 || peak <= 0.0 || length <= 0.0 {
        return None;
    }
    let line = |name: &str, name_de: &str, points: Vec<Vec2>, tone: Tone, dash: Dash, tag: &str| Series::Polyline { id: format!("{tag}:{}", bar.id), label: loc(name, name_de), points, style: stroke(tone, 2.0, dash) };
    let mut series = vec![
        Series::Limit { id: "zero".into(), label: loc("Bar axis", "Stabachse"), axis: AxisRef::Y, value: 0.0, style: stroke(Tone::Neutral, 1.0, Dash::Solid) },
        line("First order", "Theorie I. Ordnung", a.clone(), Tone::Neutral, Dash::Dashed, "first"),
        line("Second order with imperfections", "Theorie II. Ordnung mit Imperfektionen", b.clone(), Tone::Info, Dash::Solid, "second"),
    ];
    if let Some(points) = c.clone() {
        series.push(line("First order amplified 1/(1-1/α_cr)", "Theorie I. Ordnung vergrößert 1/(1-1/α_cr)", points, Tone::Warning, Dash::Dotted, "amplified"));
    }
    let mut columns = vec![column("Position x", "Lage x", "m", b.iter().map(|p| p.x).collect()), column("First order", "Theorie I. Ordnung", unit, a.iter().map(|p| p.y).collect()), column("Second order", "Theorie II. Ordnung", unit, b.iter().map(|p| p.y).collect())];
    if let Some(points) = &c {
        columns.push(column("Amplified first order", "Vergrößert Theorie I. Ordnung", unit, points.iter().map(|p| p.y).collect()));
    }
    Some(Diagram2d {
        id: format!("frame-{key}:{}", bar.id),
        title: loc(&format!("{en} along bar {} ({})", bar.id, bar.label), &format!("{de} entlang Stab {} ({})", bar.id, bar.label)),
        description: loc(
            &format!("{en} of frame bar {} from the shared mechanics kernel over the design loads of the frame (one combination), in the local axes of the bar from the start node; first order, second order with the imperfections of 5.3.2 and, for 3 ≤ α_cr < 10, the first-order course amplified by 1/(1-1/α_cr) (5.2.2(5)B).", bar.id),
            &format!("{de} des Rahmenstabs {} aus dem gemeinsamen Mechanik-Kern unter den Bemessungslasten des Rahmens (eine Kombination), in den lokalen Achsen des Stabs ab dem Anfangsknoten; Theorie I. Ordnung, Theorie II. Ordnung mit den Imperfektionen nach 5.3.2 und für 3 ≤ α_cr < 10 der um 1/(1-1/α_cr) vergrößerte Verlauf nach Theorie I. Ordnung (5.2.2(5)B).", bar.id),
        ),
        axes: vec![axis("Position x", "Lage x", QuantityKind::Length, "m", AxisScale::Linear, Some((0.0, length))), axis(en, de, kind, unit, AxisScale::Linear, Some((-peak * 1.15, peak * 1.15)))],
        series,
        markers: Vec::new(),
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Member, &bar.id, format!("frameBars[id={}]", bar.id))),
        legend: true,
        table_twin: DataTable { columns },
    })
}

fn frame_courses(d: &En1993Snapshot, wanted: &dyn Fn(&str) -> bool) -> Vec<Diagram2d> {
    if !d.frame_bars.iter().any(|b| wanted(&b.id)) {
        return Vec::new();
    }
    let Ok(result) = joint_frame::solve_second_order(d) else { return Vec::new() };
    let alpha = joint_frame::alpha_cr(d).ok();
    let mut out = Vec::new();
    for bar in d.frame_bars.iter().filter(|b| wanted(&b.id)) {
        let Some(forces) = result.bar(&bar.id) else { continue };
        let amplified = alpha.and_then(|a| frame_eigen::amplified_profile(&forces.first_profile, a));
        let (first, second, amp) = (forces.first_profile.as_slice(), forces.second_profile.as_slice(), amplified.as_deref());
        out.extend(frame_course(bar, "n", "Axial force N", "Normalkraft N", "kN", QuantityKind::Force, first, second, None, &|s| s.n));
        out.extend(frame_course(bar, "v", "Shear force V", "Querkraft V", "kN", QuantityKind::Force, first, second, amp, &|s| s.v));
        out.extend(frame_course(bar, "m", "Bending moment M", "Biegemoment M", "kN·m", QuantityKind::Moment, first, second, amp, &|s| s.m));
    }
    out
}

type Pick = fn(&frame_eigen::Station) -> f64;

fn axial(s: &frame_eigen::Station) -> f64 {
    s.n
}

fn shear(s: &frame_eigen::Station) -> f64 {
    s.v
}

fn moment(s: &frame_eigen::Station) -> f64 {
    s.m
}

const FRAME_COMPONENTS: [(&str, &str, &str, &str, QuantityKind, Pick); 3] = [
    ("n", "Axial force N", "Normalkraft N", "kN", QuantityKind::Force, axial),
    ("v", "Shear force V", "Querkraft V", "kN", QuantityKind::Force, shear),
    ("m", "Bending moment M", "Biegemoment M", "kN·m", QuantityKind::Moment, moment),
];

fn polyline_of(id: String, label: LocalizedCopy, stations: &[frame_eigen::Station], pick: Pick, tone: Tone, dash: Dash) -> Series {
    Series::Polyline { id, label, points: stations.iter().map(|s| Vec2::new(s.x, pick(s) * 1e-3)).collect(), style: stroke(tone, 2.0, dash) }
}

fn frame_set_diagram(bar: &super::model::FrameBar, key: &str, (en, de, unit, kind): (&str, &str, &str, QuantityKind), suffix: &str, scope: (LocalizedCopy, LocalizedCopy), lines: Vec<Series>, columns: Vec<DataColumn>, peak: f64, length: f64) -> Option<Diagram2d> {
    if peak <= 0.0 || length <= 0.0 {
        return None;
    }
    let zero = Series::Limit { id: "zero".into(), label: loc("Bar axis", "Stabachse"), axis: AxisRef::Y, value: 0.0, style: stroke(Tone::Neutral, 1.0, Dash::Solid) };
    Some(Diagram2d {
        id: format!("frame-{key}-{}:{}", suffix.replace(':', "-"), bar.id),
        title: loc(&format!("{en} along bar {} – {}", bar.id, scope.0.en), &format!("{de} entlang Stab {} – {}", bar.id, scope.0.de)),
        description: loc(
            &format!("{en} of frame bar {} for {}: second-order analysis of the shared mechanics kernel without imperfections, in the local axes of the bar from the start node. The load cases are combined by the EN 1990 rules (6.10a/b) and every combination is solved on its own; the envelope is taken over the ultimate combinations.", bar.id, scope.1.en),
            &format!("{de} des Rahmenstabs {} für {}: Berechnung nach Theorie II. Ordnung mit dem gemeinsamen Mechanik-Kern ohne Imperfektionen, in den lokalen Achsen des Stabs ab dem Anfangsknoten. Die Lastfälle werden nach EN 1990 (6.10a/b) kombiniert und jede Kombination für sich berechnet; die Einhüllende wird über die Grenzzustände der Tragfähigkeit gebildet.", bar.id, scope.1.de),
        ),
        axes: vec![axis("Position x", "Lage x", QuantityKind::Length, "m", AxisScale::Linear, Some((0.0, length))), axis(en, de, kind, unit, AxisScale::Linear, Some((-peak * 1.15, peak * 1.15)))],
        series: std::iter::once(zero).chain(lines).collect(),
        markers: Vec::new(),
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Member, &bar.id, format!("frameBars[id={}]", bar.id))),
        legend: true,
        table_twin: DataTable { columns },
    })
}

fn peak_of(stations: &[&[frame_eigen::Station]], pick: Pick) -> f64 {
    stations.iter().flat_map(|s| s.iter()).fold(0.0_f64, |m, s| m.max(pick(s).abs() * 1e-3))
}

/// 📈️ The internal force courses per load case (characteristic loads) and the envelope over the ultimate combinations with the governing combination, one diagram each per bar and component; the load-case track of the scene shows the case of the bending moment surface that belongs to the same course.
fn frame_set_courses(d: &En1993Snapshot, wanted: &dyn Fn(&str) -> bool) -> Vec<Diagram2d> {
    if d.frame_actions.is_empty() || !d.frame_bars.iter().any(|b| wanted(&b.id)) {
        return Vec::new();
    }
    let (Ok(cases), Ok(combinations)) = (frame_envelope::case_results(d), frame_envelope::combination_results_cached(d)) else { return Vec::new() };
    let envelopes = frame_envelope::envelope(d, &combinations);
    let plains: Vec<(String, frame_eigen::SecondOrder)> = frame_envelope::load_sets(d).unwrap_or_default().into_iter().filter(|s| combinations.iter().any(|c| c.id == s.id && c.sign != 0.0)).filter_map(|s| frame_envelope::second_order(d, &s.loads, &frame_eigen::Imperfections::default()).ok().map(|r| (s.id, r))).collect();
    let mut out = Vec::new();
    for bar in d.frame_bars.iter().filter(|b| wanted(&b.id)) {
        for (key, en, de, unit, kind, pick) in FRAME_COMPONENTS {
            let names = (en, de, unit, kind);
            for case in &cases {
                let Some(forces) = case.result.bar(&bar.id) else { continue };
                let name = d.load_cases.iter().find(|c| c.id == case.case_id).map_or(case.case_id.clone(), |c| c.name.clone());
                let lines = vec![polyline_of("first".into(), loc("First order", "Theorie I. Ordnung"), &forces.first_profile, pick, Tone::Neutral, Dash::Dashed), polyline_of("second".into(), loc("Second order", "Theorie II. Ordnung"), &forces.second_profile, pick, Tone::Info, Dash::Solid)];
                let columns = vec![column("Position x", "Lage x", "m", forces.second_profile.iter().map(|s| s.x).collect()), column("First order", "Theorie I. Ordnung", unit, forces.first_profile.iter().map(|s| pick(s) * 1e-3).collect()), column("Second order", "Theorie II. Ordnung", unit, forces.second_profile.iter().map(|s| pick(s) * 1e-3).collect())];
                let length = forces.second_profile.last().map_or(0.0, |s| s.x);
                out.extend(frame_set_diagram(bar, key, names, &format!("case:{}", case.case_id), (loc(&format!("load case {}", case.case_id), &format!("Lastfall {}", case.case_id)), loc(&format!("the characteristic load case {} ({name})", case.case_id), &format!("den charakteristischen Lastfall {} ({name})", case.case_id))), lines, columns, peak_of(&[&forces.first_profile, &forces.second_profile], pick), length));
            }
            let Some(envelope) = envelopes.iter().find(|e| e.bar_id == bar.id) else { continue };
            let governing = combinations.iter().find(|c| c.id == envelope.governing.combination).and_then(|c| c.result.bar(&bar.id));
            let mut lines = vec![polyline_of("max".into(), loc("Envelope maximum", "Einhüllende Maximum"), &envelope.max, pick, Tone::Error, Dash::Solid), polyline_of("min".into(), loc("Envelope minimum", "Einhüllende Minimum"), &envelope.min, pick, Tone::Info, Dash::Solid)];
            let mut columns = vec![column("Position x", "Lage x", "m", envelope.max.iter().map(|s| s.x).collect()), column("Envelope maximum", "Einhüllende Maximum", unit, envelope.max.iter().map(|s| pick(s) * 1e-3).collect()), column("Envelope minimum", "Einhüllende Minimum", unit, envelope.min.iter().map(|s| pick(s) * 1e-3).collect())];
            if let Some(own) = governing {
                lines.push(polyline_of("governing".into(), loc(&format!("Governing combination {}", envelope.governing.combination), &format!("Maßgebende Kombination {}", envelope.governing.combination)), &own.second_profile, pick, Tone::Warning, Dash::Dashed));
                columns.push(column("Governing combination", "Maßgebende Kombination", unit, own.second_profile.iter().map(|s| pick(s) * 1e-3).collect()));
            }
            if let (Some(own), Some(plain)) = (governing, plains.iter().find(|p| p.0 == envelope.governing.combination).and_then(|p| p.1.bar(&bar.id))) {
                let share: Vec<frame_eigen::Station> = own.second_profile.iter().zip(&plain.second_profile).map(|(a, b)| frame_eigen::Station { x: a.x, n: a.n - b.n, v: a.v - b.v, m: a.m - b.m }).collect();
                lines.push(polyline_of("imperfection".into(), loc("Sway imperfection share (5.3.2)", "Anteil der Schiefstellung (5.3.2)"), &share, pick, Tone::Neutral, Dash::Dotted));
                columns.push(column("Sway imperfection share", "Anteil der Schiefstellung", unit, share.iter().map(|s| pick(s) * 1e-3).collect()));
            }
            let length = envelope.max.last().map_or(0.0, |s| s.x);
            out.extend(frame_set_diagram(bar, key, names, "envelope", (loc("envelope", "Einhüllende"), loc(&format!("the envelope of the {} ultimate combinations (governing: {})", combinations.len(), envelope.governing.combination), &format!("die Einhüllende der {} Grenzzustandskombinationen (maßgebend: {})", combinations.len(), envelope.governing.combination))), lines, columns, peak_of(&[&envelope.max, &envelope.min], pick), length));
        }
    }
    out
}

fn tower_nm(d: &En1993Snapshot, report: &CheckReport, tower: &TowerLeg) -> Option<Diagram2d> {
    let member = d.members.iter().find(|m| m.id == tower.member_id)?;
    let section = d.sections.iter().find(|s| s.id == member.section_id)?;
    let material = d.materials.iter().find(|m| m.id == member.material_id)?;
    let (fy, params) = (material.fy, AnnexParams::for_choice(d.annex));
    let class = part_1_1::section_class_rolled_i(section, fy);
    let area = if class == 4 { part_1_1::effective_area_class4(section, fy) } else { section.area };
    let n_pl = part_1_1::axial_resistance_n(area, fy, params);
    let m_y = part_1_1::bending_resistance_nm(part_1_1::bending_modulus_y(section, class, fy), fy, class, params);
    let m_z = part_1_1::bending_resistance_nm(part_1_1::bending_modulus_z(section, class, fy), fy, class, params);
    if !(n_pl > 0.0 && m_y > 0.0) {
        return None;
    }
    let point = tower_design_point(d, tower);
    let eta = |n: f64, m: f64| part_1_1::section_mn_eta(section, n, m, point.mz, n_pl, m_y, m_z, class);
    let boundary = |n: f64| {
        let (mut lo, mut hi) = (0.0, m_y);
        if eta(n, hi) <= 1.0 {
            return hi;
        }
        for _ in 0..40 {
            let mid = 0.5 * (lo + hi);
            if eta(n, mid) <= 1.0 { lo = mid } else { hi = mid }
        }
        lo
    };
    let curve: Vec<Vec2> = (0..=SAMPLES).map(|i| n_pl * i as f64 / SAMPLES as f64).map(|n| Vec2::new(n / 1e3, boundary(n) / 1e3)).collect();
    let (n_ed, m_ed) = (point.n, point.my);
    let n_b = part_3::tower_buckling_n(section.area, fy, part_1_1::tower_flexural_chi(section, member.buckling_length_y, member.buckling_length_z, material.e_modulus, fy), tower.force_coefficient, tower.dynamic_factor, params);
    let utilisation = worst(report, &tower.id, "en1993.3-1.tower.");
    let ring: Vec<Vec2> = std::iter::once(Vec2::ZERO).chain(curve.iter().cloned()).chain(std::iter::once(Vec2::new(n_pl / 1e3, 0.0))).collect();
    let (x_max, y_max) = (n_pl.max(n_ed).max(n_b) / 1e3 * 1.15, m_y.max(m_ed) / 1e3 * 1.15);
    Some(Diagram2d {
        id: format!("tower-nm:{}", tower.id),
        title: loc(&format!("Tower leg N–M interaction – {}", tower.id), &format!("Turmstiel N-M-Interaktion – {}", tower.id)),
        description: loc(
            &format!("Section resistance M_N,y,Rd over the axial force N (EN 1993-1-1 6.2.9, class {class}, section kind {:?}: I and H sections by Eq. 6.36 to 6.41, rectangular hollow sections by Eq. 6.39 and 6.40 with the exponents of 6.2.9.1(6), circular hollow sections by M_pl,Rd (1 − n^1.7) with α = β = 2, angles on the principal axes u-u (y) and v-v (z) by the linear elastic criterion of 6.2.9.2 with the class 3 or 4 resistances; EN 1993-3-1 Annex G and H are not applied) at the concurrent M_z,Ed = {:.2} kN·m, and the buckling resistance N_b,Rd with wind amplification (EN 1993-3-1). The design point is N_Ed of the tower rows (governing {}) with the governing M_y,Ed of the member rows of the same member (governing {}); without member rows M_Ed = 0.", section.kind, point.mz / 1e3, point.axial_combination, point.moment_combination),
            &format!("Querschnittstragfähigkeit M_N,y,Rd über der Normalkraft N (EN 1993-1-1 6.2.9, Klasse {class}, Querschnittsart {:?}: I- und H-Querschnitte nach Gl. 6.36 bis 6.41, Rechteck-Hohlprofile nach Gl. 6.39 und 6.40 mit den Exponenten aus 6.2.9.1(6), Kreis-Hohlprofile mit M_pl,Rd (1 − n^1,7) und α = β = 2, Winkelprofile auf den Hauptachsen u-u (y) und v-v (z) nach dem linear-elastischen Kriterium von 6.2.9.2 mit den Widerständen der Klasse 3 oder 4; EN 1993-3-1 Anhang G und H werden nicht angewendet) bei gleichzeitigem M_z,Ed = {:.2} kN·m und Knickwiderstand N_b,Rd mit Windverstärkung (EN 1993-3-1). Der Bemessungspunkt ist N_Ed der Turmzeilen (maßgebend {}) mit dem maßgebenden M_y,Ed der Bauteilzeilen desselben Bauteils (maßgebend {}); ohne Bauteilzeilen ist M_Ed = 0.", section.kind, point.mz / 1e3, point.axial_combination, point.moment_combination),
        ),
        axes: vec![axis("Axial force N", "Normalkraft N", QuantityKind::Force, "kN", AxisScale::Linear, Some((0.0, x_max))), axis("Bending moment M", "Biegemoment M", QuantityKind::Moment, "kN·m", AxisScale::Linear, Some((0.0, y_max)))],
        series: vec![
            Series::Polygon { id: "resistance".into(), label: loc("Section resistance 6.2.9", "Querschnittstragfähigkeit 6.2.9"), ring, fill: Fill { tone: Tone::Info, opacity: 0.25 }, hatch: None },
            Series::Limit { id: "buckling".into(), label: loc("N_b,Rd (tower buckling)", "N_b,Rd (Turmknicken)"), axis: AxisRef::X, value: n_b / 1e3, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
        ],
        markers: vec![Marker { id: "demand".into(), at: Vec2::new(n_ed / 1e3, m_ed / 1e3), shape: MarkerShape::Diamond, tone: tone(utilisation), label: loc(&format!("N_Ed = {:.0} kN, M_y,Ed = {:.1} kN·m", n_ed / 1e3, m_ed / 1e3), &format!("N_Ed = {:.0} kN, M_y,Ed = {:.1} kN·m", n_ed / 1e3, m_ed / 1e3)), utilisation: Some(utilisation), bound_check: Some(format!("en1993.3-1.tower.{}", tower.id)), binding: None }],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Group, &tower.id, format!("towerLegs[id={}]", tower.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Axial force N", "Normalkraft N", "kN", curve.iter().map(|q| q.x).collect()), column("Resistance M_N,y,Rd", "Widerstand M_N,y,Rd", "kN·m", curve.iter().map(|q| q.y).collect())] },
    })
}

/// 🌡️ Allowed thickness t_max over the reference temperature T_Ed (EN 1993-1-10 Table 2.1, stress column interpolated at σ_Ed/f_y(t)) with the design point (T_Ed, t) of the member; the dashed line is the member thickness.
fn toughness_diagram(d: &En1993Snapshot, report: &CheckReport, member: &SteelMember) -> Option<Diagram2d> {
    let section = d.sections.iter().find(|s| s.id == member.section_id)?;
    let material = d.materials.iter().find(|m| m.id == member.material_id)?;
    let points = toughness::curve(&material.grade, &material.subgrade, member.toughness_stress_ratio);
    if points.is_empty() {
        return None;
    }
    let (t, t_ed) = toughness_point(member, section);
    let check_id = format!("en1993.1-10.t.{}", member.id);
    let utilisation = report.checks.iter().find(|c| c.id == check_id).map_or(0.0, |c| if c.utilization.is_finite() { c.utilization } else { 0.0 });
    let (x_min, x_max) = ((t_ed.min(-50.0) - 5.0).floor(), (t_ed.max(10.0) + 5.0).ceil());
    let y_max = points.iter().map(|p| p.1).fold(t, f64::max) * 1.15;
    let label = format!("T_Ed = {t_ed:.1} °C, t = {t:.1} mm");
    Some(Diagram2d {
        id: format!("toughness:{}", member.id),
        title: loc(&format!("Brittle fracture – allowed thickness – {}", member.id), &format!("Sprödbruch – zulässige Dicke – {}", member.id)),
        description: loc(
            &format!("Maximum thickness t_max of {} {} over the reference temperature T_Ed (EN 1993-1-10 Table 2.1, linear interpolation in T_Ed and in σ_Ed/f_y(t) = {:.2}); the design point is the larger flange or web thickness at the member T_Ed = T_md + ΔT_r + ΔT_σ + ΔT_R + ΔT_ε̇ + ΔT_εcf. Cells not transcribed from the print fall back on the next-higher-stress column.", material.grade, material.subgrade, member.toughness_stress_ratio),
            &format!("Größte Dicke t_max von {} {} über der Bezugstemperatur T_Ed (EN 1993-1-10 Tabelle 2.1, lineare Interpolation in T_Ed und in σ_Ed/f_y(t) = {:.2}); der Bemessungspunkt ist die größere Flansch- oder Stegdicke bei T_Ed des Bauteils = T_md + ΔT_r + ΔT_σ + ΔT_R + ΔT_ε̇ + ΔT_εcf. Nicht aus dem Druck übertragene Zellen fallen auf die nächsthöhere Spannungsspalte zurück.", material.grade, material.subgrade, member.toughness_stress_ratio),
        ),
        axes: vec![axis("Reference temperature T_Ed", "Bezugstemperatur T_Ed", QuantityKind::Temperature, "°C", AxisScale::Linear, Some((x_min, x_max))), axis("Allowed thickness t_max", "Zulässige Dicke t_max", QuantityKind::Length, "mm", AxisScale::Linear, Some((0.0, y_max)))],
        series: vec![
            Series::Polyline { id: "t-max".into(), label: loc(&format!("t_max {} {}", material.grade, material.subgrade), &format!("t_max {} {}", material.grade, material.subgrade)), points: points.iter().map(|p| Vec2::new(p.0, p.1)).collect(), style: stroke(Tone::Info, 2.5, Dash::Solid) },
            Series::Limit { id: "thickness".into(), label: loc("Member thickness t", "Bauteildicke t"), axis: AxisRef::Y, value: t, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
        ],
        markers: vec![Marker { id: "design".into(), at: Vec2::new(t_ed.clamp(x_min, x_max), t), shape: MarkerShape::Diamond, tone: tone(utilisation), label: loc(&label, &label), utilisation: Some(utilisation), bound_check: Some(check_id), binding: None }],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Member, &member.id, format!("members[id={}]", member.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Reference temperature T_Ed", "Bezugstemperatur T_Ed", "°C", points.iter().map(|p| p.0).collect()), column("Allowed thickness t_max", "Zulässige Dicke t_max", "mm", points.iter().map(|p| p.1).collect())] },
    })
}

fn ladder_of(rows: Vec<&CheckResult>, diagram_id: String, collection: &str, id: &str, (scope_en, scope_de): (&str, &str)) -> Option<Diagram2d> {
    if rows.is_empty() {
        return None;
    }
    let peak = rows.iter().map(|c| c.utilization).fold(1.0_f64, f64::max);
    let worst_u = rows.iter().map(|c| c.utilization).fold(0.0_f64, f64::max);
    let series: Vec<Series> = rows.iter().enumerate().map(|(i, c)| bar(format!("check-{i}"), c.title.clone(), (0.0, c.utilization), (i as f64 + 0.1, i as f64 + 0.9), tone(c.utilization))).collect();
    let notes = rows.iter().enumerate().map(|(i, c)| DiagramNote { id: format!("note-{i}"), at: Vec2::new(c.utilization.min(peak * 1.05), i as f64 + 0.5), text: loc(&format!("{} ({:.2})", c.title.en, c.utilization), &format!("{} ({:.2})", c.title.de, c.utilization)), tone: tone(c.utilization) }).collect();
    Some(Diagram2d {
        id: diagram_id,
        title: loc(&format!("{scope_en} – {id}"), &format!("{scope_de} – {id}")),
        description: loc(&format!("Utilisation (demand over resistance) of every check of {collection} {id} listed here; the dashed line is 1.0. Worst check: {worst_u:.2}."), &format!("Ausnutzung (Beanspruchung zu Widerstand) jedes hier gelisteten Nachweises von {collection} {id}; die gestrichelte Linie ist 1,0. Maßgebend: {worst_u:.2}.")),
        axes: vec![axis("Utilisation", "Ausnutzung", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, peak * 1.6))), axis("Check", "Nachweis", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, rows.len() as f64)))],
        series: std::iter::once(Series::Limit { id: "limit".into(), label: loc("Utilisation 1.0", "Ausnutzung 1,0"), axis: AxisRef::X, value: 1.0, style: stroke(Tone::Error, 1.5, Dash::Dashed) }).chain(series).collect(),
        markers: Vec::new(),
        annotations: notes,
        focus: Some(EntityRef::new(EntityKind::Group, id, format!("{collection}[id={id}]"))),
        legend: false,
        table_twin: DataTable { columns: vec![column("Check", "Nachweis", "1", (0..rows.len()).map(|i| i as f64 + 1.0).collect()), column("Utilisation", "Ausnutzung", "1", rows.iter().map(|c| c.utilization).collect())] },
    })
}

fn ladder(report: &CheckReport, collection: &str, id: &str) -> Option<Diagram2d> {
    let rows = report.checks.iter().filter(|c| c.subject.entity_id == id && c.utilization.is_finite()).collect();
    ladder_of(rows, format!("ladder:{collection}:{id}"), collection, id, ("Check utilisation", "Nachweisausnutzung"))
}

/// 📊️ The outcome diagram `check-outcomes:<row>` of every row outside [`LADDER_COLLECTIONS`](super::kit::LADDER_COLLECTIONS) that owns checks of a family no dedicated diagram draws: one bar per such check against the limit at 1, routed through [`owner_entity`](super::kit::owner_entity).
fn family_outcomes(d: &En1993Snapshot, report: &CheckReport, wanted: &dyn Fn(&str) -> bool) -> Vec<Diagram2d> {
    let mut rows: std::collections::BTreeMap<String, Vec<&CheckResult>> = std::collections::BTreeMap::new();
    for check in report.checks.iter().filter(|c| c.utilization.is_finite() && c.status != super::norm::CheckStatus::NotApplicable && super::kit::is_outcome_check(&c.id)) {
        if let Some(id) = super::kit::owner_entity(d, check).filter(|id| wanted(id)) {
            rows.entry(id).or_default().push(check);
        }
    }
    rows.into_iter().filter_map(|(id, checks)| {
        let spec = super::kit::entity_kind_of(d, &id).filter(|k| !super::kit::LADDER_COLLECTIONS.contains(&k.collection))?;
        ladder_of(checks, format!("check-outcomes:{id}"), spec.collection, &id, ("Check outcomes without a dedicated diagram", "Nachweisergebnisse ohne eigenes Diagramm"))
    }).collect()
}

/// 📈️ The design-point diagrams of the collections without a dedicated curve elsewhere; `focus` limits them to one entity id.
pub(crate) fn design_diagrams(d: &En1993Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    let wanted = |id: &str| focus.is_none_or(|f| f == id);
    let mut out = Vec::new();
    for pile in d.piles.iter().filter(|x| wanted(&x.id)) {
        out.extend(pile_interaction(d, report, pile));
    }
    for bf in d.bridge_fatigue.iter().filter(|x| wanted(&x.id)) {
        out.extend(bridge_sn(d, bf));
    }
    for c in d.crane_runways.iter().filter(|x| wanted(&x.id)) {
        out.extend(crane_wheel(d, report, c));
    }
    for p in d.plated_panels.iter().filter(|x| wanted(&x.id)) {
        out.extend(panel_rho(report, p));
    }
    for case in d.load_cases.iter().filter(|x| wanted(&x.id)) {
        out.extend(load_case_bars(d, case));
    }
    for member in d.members.iter().filter(|x| wanted(&x.id)) {
        out.extend(toughness_diagram(d, report, member));
    }
    out.extend(frame_courses(d, &wanted));
    out.extend(frame_set_courses(d, &wanted));
    for tower in d.tower_legs.iter().filter(|x| wanted(&x.id)) {
        out.extend(tower_nm(d, report, tower));
    }
    let ids: Vec<(&str, &str)> = d.tower_legs.iter().map(|x| ("towerLegs", x.id.as_str()))
        .chain(d.tension_components.iter().map(|x| ("tensionComponents", x.id.as_str())))
        .chain(d.storeys.iter().map(|x| ("storeys", x.id.as_str())))
        .chain(d.built_up_members.iter().map(|x| ("builtUpMembers", x.id.as_str())))
        .chain(d.crane_runways.iter().map(|x| ("craneRunways", x.id.as_str())))
        .chain(d.joint_rows.iter().map(|x| ("jointRows", x.id.as_str())))
        .chain(d.frame_bars.iter().map(|x| ("frameBars", x.id.as_str())))
        .chain(d.frame_nodes.iter().map(|x| ("frameNodes", x.id.as_str())))
        .chain(d.joint_plates.iter().map(|x| ("jointPlates", x.id.as_str())))
        .chain(d.foundation_blocks.iter().map(|x| ("foundationBlocks", x.id.as_str())))
        .chain(d.piles.iter().map(|x| ("piles", x.id.as_str())))
        .collect();
    for (collection, id) in ids.into_iter().filter(|(_, id)| wanted(id)) {
        out.extend(ladder(report, collection, id));
    }
    out.extend(family_outcomes(d, report, &wanted));
    out
}
