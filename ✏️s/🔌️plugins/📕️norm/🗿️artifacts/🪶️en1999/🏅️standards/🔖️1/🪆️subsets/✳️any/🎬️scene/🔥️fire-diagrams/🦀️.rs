//! 🔥️ Fire diagrams of EN 1999-1-2: the reduced 0.2 % proof strength over the member temperature and the member resistance over the fire time with the required duration. The retention factors are the unverified seed of `part_1_2` (EN 1999-1-2 Table 3.1).

use super::*;

/// 🔥️ Minimum of the time samples of the resistance curve.
const TIME_SAMPLES: usize = 60;

/// 🔥️ The proof strength diagram `fire-proof:<id>` (f_0.2,θ = k_0.2proof,θ·f_o over θ_a with the unreduced f_o, the critical temperature and the end temperature) and the resistance diagram `fire-resistance:<id>` (cross-section utilisation and retained resistance over the time with the draggable required duration marker and the time to failure); the second is omitted without fire-situation effects or for an unknown alloy.
pub(super) fn proof_strength_and_resistance(d: &En1999Snapshot, fire: &FireScenario, member: &AluminiumMember, section: &AluminiumSection, theta_end: f64, theta_cr: f64, bound_check: Option<String>) -> Vec<Diagram2d> {
    let focus = Some(EntityRef::new(EntityKind::Load, &fire.id, format!("fireScenarios[id={}]", fire.id)));
    let tone = if theta_end > theta_cr { Tone::Error } else { Tone::Info };
    let mut out = Vec::new();
    let Some((_, alloy)) = facts::member_parts(d, member) else { return out };
    let f_o = alloy.f_o_pa / 1.0e6;
    let thetas: Vec<f64> = (0..=SAMPLES).map(|i| 20.0 + 530.0 * i as f64 / SAMPLES as f64).collect();
    let f_theta = |t: f64| part_1_2::k_theta(t) * f_o;
    out.push(Diagram2d {
        id: format!("fire-proof:{}", fire.id),
        title: loc(&format!("Reduced 0.2 % proof strength – {}", member.id), &format!("Abgeminderte 0,2-%-Dehngrenze – {}", member.id)),
        description: loc("Proof strength f_0.2,θ = k_0.2proof,θ·f_o of the alloy over the member temperature with the unreduced f_o, the critical temperature and the temperature at the required duration (EN 1999-1-2, Table 3.1; retention factors unverified).", "Dehngrenze f_0,2,θ = k_0,2proof,θ·f_o der Legierung über der Bauteiltemperatur mit dem unabgeminderten f_o, der kritischen Temperatur und der Temperatur bei der geforderten Dauer (EN 1999-1-2, Tabelle 3.1; Abminderungsfaktoren ungeprüft)."),
        axes: vec![axis("Member temperature θ_a", "Bauteiltemperatur θ_a", QuantityKind::Temperature, "°C", AxisScale::Linear, Some((20.0, 550.0))), axis("Proof strength f_0.2,θ", "Dehngrenze f_0,2,θ", QuantityKind::Stress, "MPa", AxisScale::Linear, Some((0.0, f_o * 1.05)))],
        series: vec![
            Series::Polyline { id: "f-theta".into(), label: loc("f_0.2,θ", "f_0,2,θ"), points: thetas.iter().map(|t| Vec2::new(*t, f_theta(*t))).collect(), style: stroke(Tone::Info, 2.0, Dash::Solid) },
            Series::Limit { id: "f-o".into(), label: loc("f_o at 20 °C", "f_o bei 20 °C"), axis: AxisRef::Y, value: f_o, style: stroke(Tone::Neutral, 1.0, Dash::Dashed) },
            Series::Limit { id: "theta-cr".into(), label: loc("Critical θ_cr", "Kritisch θ_cr"), axis: AxisRef::X, value: theta_cr.clamp(20.0, 550.0), style: stroke(Tone::Error, 1.5, Dash::Dashed) },
        ],
        markers: vec![Marker { id: ::semio_s_plugin_norm_structural_scene::projection_kit::diagram_parts::checked_marker_id("f-at-end", &bound_check), at: Vec2::new(theta_end.clamp(20.0, 550.0), f_theta(theta_end)), shape: MarkerShape::Circle, tone, label: loc(&format!("f_0.2,θ = {:.0} MPa at {theta_end:.0} °C", f_theta(theta_end)), &format!("f_0,2,θ = {:.0} MPa bei {theta_end:.0} °C", f_theta(theta_end))), utilisation: None, bound_check: bound_check.clone(), binding: None }],
        annotations: Vec::new(),
        focus: focus.clone(),
        legend: true,
        table_twin: DataTable { columns: vec![column("Member temperature θ_a", "Bauteiltemperatur θ_a", "°C", thetas.clone()), column("f_0.2,θ", "f_0,2,θ", "MPa", thetas.iter().map(|t| f_theta(*t)).collect())] },
    });
    let Some(material) = d.materials.iter().find(|m| m.id == member.material_id) else { return out };
    let a_m_v = part_1_2::section_factor(section);
    let cover = d.fire_insulations.iter().find(|i| i.fire_id == fire.id).map(part_1_2::Insulation::from);
    let theta_at = |minutes: f64| part_1_2::member_temperature_c(fire.theta_a, a_m_v, minutes * 60.0, cover.as_ref());
    let utilisation = |minutes: f64| checks::fire_utilisation_at(member, section, material, d.annex, theta_at(minutes));
    let (Some(u0), Some(u_req)) = (utilisation(0.0), utilisation(fire.duration_s.max(0.0) / 60.0)) else { return out };
    if u0 <= 0.0 { return out }
    let end_min = (fire.duration_s / 60.0 * 1.25).max(5.0);
    let times: Vec<f64> = (0..=TIME_SAMPLES).map(|i| end_min * i as f64 / TIME_SAMPLES as f64).collect();
    let us: Vec<f64> = times.iter().map(|t| utilisation(*t).unwrap_or(f64::NAN)).collect();
    let retained: Vec<f64> = us.iter().map(|u| if *u > 0.0 { (u0 / u).min(1.0) } else { 1.0 }).collect();
    let failure = times.iter().zip(&us).find(|(_, u)| **u > 1.0).map(|(t, _)| *t);
    let u_cap = us.iter().cloned().fold(1.2, f64::max).min(3.0);
    let required_tone = if super::scene::norm::within_limit(u_req) { Tone::Info } else { Tone::Error };
    let mut binding = FieldBinding::new(format!("fireScenarios[id={}].durationS", fire.id), Affine { base: 0.0, k: 60.0 }, "s");
    binding.quantise = Some(Quantise::Round { step: 60.0 });
    let required_min = (fire.duration_s / 60.0).clamp(0.0, end_min);
    let mut markers = vec![Marker { id: "required".into(), at: Vec2::new(required_min, u_req.min(u_cap)), shape: MarkerShape::Circle, tone: required_tone, label: loc(&format!("Required {:.0} min: utilisation {u_req:.2}", fire.duration_s / 60.0), &format!("Gefordert {:.0} min: Ausnutzung {u_req:.2}", fire.duration_s / 60.0)), utilisation: Some(u_req), bound_check: bound_check.clone(), binding: Some(binding) }];
    if let Some(t) = failure {
        markers.push(Marker { id: ::semio_s_plugin_norm_structural_scene::projection_kit::diagram_parts::checked_marker_id("failure", &bound_check), at: Vec2::new(t, 1.0), shape: MarkerShape::Circle, tone: Tone::Error, label: loc(&format!("Resistance exhausted after {t:.0} min"), &format!("Tragfähigkeit erschöpft nach {t:.0} min")), utilisation: None, bound_check: bound_check.clone(), binding: None });
    }
    out.push(Diagram2d {
        id: format!("fire-resistance:{}", fire.id),
        title: loc(&format!("Member resistance over fire time – {}", member.id), &format!("Bauteilwiderstand über der Branddauer – {}", member.id)),
        description: loc("Cross-section utilisation Σ(N, M_y, M_z) of the fire situation and the retained fraction of the cold resistance over the ISO 834 exposure time (EN 1999-1-2, 4.2); the marker is the required fire duration, drag it to change it (retention factors unverified).", "Querschnittsausnutzung Σ(N, M_y, M_z) der Brandsituation und verbleibender Anteil des Kaltwiderstands über der ISO-834-Branddauer (EN 1999-1-2, 4.2); der Marker ist die geforderte Branddauer und kann verschoben werden (Abminderungsfaktoren ungeprüft)."),
        axes: vec![axis("Time t", "Zeit t", QuantityKind::Time, "min", AxisScale::Linear, Some((0.0, end_min))), axis("Utilisation / retained resistance", "Ausnutzung / verbleibender Widerstand", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, u_cap * 1.05)))],
        series: vec![
            Series::Polyline { id: "utilisation".into(), label: loc("Utilisation Σ", "Ausnutzung Σ"), points: times.iter().zip(&us).map(|(t, u)| Vec2::new(*t, u.min(u_cap))).collect(), style: stroke(Tone::Warning, 2.5, Dash::Solid) },
            Series::Polyline { id: "retained".into(), label: loc("Retained resistance R(t)/R(20 °C)", "Verbleibender Widerstand R(t)/R(20 °C)"), points: times.iter().zip(&retained).map(|(t, r)| Vec2::new(*t, *r)).collect(), style: stroke(Tone::Info, 2.0, Dash::Dashed) },
            Series::Limit { id: "limit".into(), label: loc("Limit 1.0", "Grenze 1,0"), axis: AxisRef::Y, value: 1.0, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
        ],
        markers,
        annotations: Vec::new(),
        focus,
        legend: true,
        table_twin: DataTable { columns: vec![column("Time t", "Zeit t", "min", times.clone()), column("Utilisation Σ", "Ausnutzung Σ", "1", us.clone()), column("Retained resistance", "Verbleibender Widerstand", "1", retained.clone())] },
    });
    out
}
