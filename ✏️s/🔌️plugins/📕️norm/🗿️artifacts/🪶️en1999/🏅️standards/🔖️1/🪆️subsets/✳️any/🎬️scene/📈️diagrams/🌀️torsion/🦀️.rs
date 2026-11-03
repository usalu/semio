//! 🌀️ Torsional and welded-member buckling diagrams (EN 1999-1-1, 6.3.1): `torsional:<member>` plots χ_T over the relative slenderness λ̄_T with the HAZ-reduced κ·χ_T and the design point, `torsional-length:<member>` plots N_b,T,Rd over the torsional buckling length with the demand line and a marker that drags L_cr,T, and `haz-buckling:<member>` plots χ, κ and κ·χ over λ̄ with the governing axis of the welded member (κ_0 from the HAZ-reduced effective area, ω_x at a transverse weld).

use super::*;

const LENGTH_SAMPLES: usize = 48;

fn kappa_0(c: &Ctx) -> f64 {
    let a_loc = part_1_1::props(c.section, &c.base()).a;
    if c.section.elements.iter().any(|e| e.welded) && a_loc > 0.0 { part_1_1::props(c.section, &EffCtx { haz_rho: Some(c.alloy.rho_o_haz), ..c.base() }).a / a_loc } else { 1.0 }
}

fn l_cr_t(c: &Ctx) -> f64 {
    if c.member.buckling_length_t > 0.0 { c.member.buckling_length_t } else { k_support(c.member) * c.member.length }
}

fn lambda_t(c: &Ctx, a_loc: f64, l: f64) -> f64 {
    let (_, i_y, i_z, _, _, i_t, i_w) = part_1_1::section_geometry(c.section);
    let gross = part_1_1::props(c.section, &EffCtx::gross(c.alloy.f_o_pa, c.alloy.buckling_class_a));
    let n_cr = part_1_1::n_cr_torsional(gross.a, i_t, i_w, i_y, i_z, part_1_1::shear_centre_offset(c.section), l);
    if n_cr.is_finite() && n_cr > 0.0 { (a_loc * c.alloy.f_o_pa / n_cr).sqrt() } else { 0.0 }
}

fn factor_t(c: &Ctx, k0: f64, lambda: f64) -> f64 {
    let (alpha, lambda_0) = part_1_1::torsional_curve(c.alloy.buckling_class_a);
    part_1_1::kappa(k0, lambda) * part_1_1::chi_from_lambda(lambda, alpha, lambda_0)
}

/// 🌀️ The diagrams `torsional:<id>` and `torsional-length:<id>` of the check `en1999.6.3.1.tb.<id>`; none without the check.
pub(super) fn torsional(c: &Ctx) -> Vec<Diagram2d> {
    let Some(check) = exact(c.report, &format!("en1999.6.3.1.tb.{}", c.member.id)) else { return Vec::new() };
    let (alpha, lambda_0) = part_1_1::torsional_curve(c.alloy.buckling_class_a);
    let a_loc = part_1_1::props(c.section, &c.base()).a;
    let k0 = kappa_0(c);
    let l_cr = l_cr_t(c);
    let lambda = lambda_t(c, a_loc, l_cr);
    let factor = factor_t(c, k0, lambda);
    let utilisation = check.utilization;
    let tone = utilisation_tone(utilisation);
    let id = &c.member.id;
    let lambdas: Vec<f64> = (0..=SAMPLES).map(|i| LAMBDA_MAX * i as f64 / SAMPLES as f64).collect();
    let chi: Vec<f64> = lambdas.iter().map(|l| part_1_1::chi_from_lambda(*l, alpha, lambda_0)).collect();
    let reduced: Vec<f64> = lambdas.iter().map(|l| factor_t(c, k0, *l)).collect();
    let line = |values: &[f64]| lambdas.iter().zip(values).map(|(x, y)| Vec2::new(*x, *y)).collect::<Vec<_>>();
    let mut series = vec![Series::Polyline { id: "chi-t".into(), label: loc(&format!("χ_T: α = {alpha:.2}, λ̄₀ = {lambda_0:.2}"), &format!("χ_T: α = {alpha:.2}, λ̄₀ = {lambda_0:.2}")), points: line(&chi), style: stroke(Tone::Info, 2.5, Dash::Solid) }];
    if k0 < 1.0 - 1e-9 {
        series.push(Series::Polyline { id: "kappa-chi-t".into(), label: loc(&format!("κ·χ_T with κ₀ = {k0:.3} (HAZ)"), &format!("κ·χ_T mit κ₀ = {k0:.3} (WEZ)")), points: line(&reduced), style: stroke(Tone::Warning, 2.0, Dash::Solid) });
    }
    series.push(Series::Polyline { id: "euler".into(), label: loc("Euler hyperbola 1/λ̄²", "Euler-Hyperbel 1/λ̄²"), points: curves::sample(|l| 1.0 / (l * l), 1.0, LAMBDA_MAX, SAMPLES).into_iter().map(p).collect(), style: stroke(Tone::Neutral, 1.0, Dash::Dashed) });
    let point = Marker {
        id: "marker-t".into(),
        at: Vec2::new(lambda.min(LAMBDA_MAX), factor),
        shape: MarkerShape::Circle,
        tone,
        label: loc(&format!("λ̄_T = {lambda:.2}, κ·χ_T = {factor:.3}"), &format!("λ̄_T = {lambda:.2}, κ·χ_T = {factor:.3}")),
        utilisation: Some(utilisation),
        bound_check: Some(check.id.clone()),
        binding: None,
    };
    let slenderness = Diagram2d {
        id: format!("torsional:{id}"),
        title: loc(&format!("Torsional buckling curve – {id}"), &format!("Drillknicklinie – {id}")),
        description: loc("Reduction factor χ_T over the relative torsional slenderness λ̄_T = √(A_eff·f_o/N_cr,T) with the curve of EN 1999-1-1, 6.3.1.4 and, for a welded member, the HAZ factor κ; the marker is the design point of the member.", "Abminderungsfaktor χ_T über dem bezogenen Drillschlankheitsgrad λ̄_T = √(A_eff·f_o/N_cr,T) mit der Kurve nach EN 1999-1-1, 6.3.1.4 und, beim geschweißten Bauteil, dem WEZ-Faktor κ; der Marker ist der Bemessungspunkt des Bauteils."),
        axes: vec![axis("Relative slenderness λ̄_T", "Bezogener Schlankheitsgrad λ̄_T", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, LAMBDA_MAX))), axis("Reduction factor χ_T", "Abminderungsfaktor χ_T", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, 1.1)))],
        series,
        markers: vec![point],
        annotations: Vec::new(),
        focus: member_focus(id),
        legend: true,
        table_twin: DataTable { columns: vec![lambda_column("Relative slenderness λ̄_T", "Bezogener Schlankheitsgrad λ̄_T"), column("χ_T", "χ_T", "1", chi), column("κ·χ_T", "κ·χ_T", "1", reduced)] },
    };
    let n_b = |l: f64| factor_t(c, k0, lambda_t(c, a_loc, l)) * a_loc * c.alloy.f_o_pa / c.params.gamma_m1 / 1.0e3;
    let top = (2.0 * c.member.length).max(1.5 * l_cr).max(1.0);
    let lengths: Vec<f64> = (0..=LENGTH_SAMPLES).map(|i| 0.1 + (top - 0.1) * i as f64 / LENGTH_SAMPLES as f64).collect();
    let resistance: Vec<f64> = lengths.iter().map(|l| n_b(*l)).collect();
    let demand = check.computed.value.abs() / 1.0e3;
    let mut binding = FieldBinding::new(format!("members[id={id}].bucklingLengthT"), Affine::IDENTITY, "m");
    binding.quantise = Some(Quantise::Round { step: 0.05 });
    let by_length = Diagram2d {
        id: format!("torsional-length:{id}"),
        title: loc(&format!("Torsional buckling resistance over the length – {id}"), &format!("Drillknickwiderstand über der Länge – {id}")),
        description: loc("Design resistance N_b,T,Rd over the torsional buckling length L_cr,T (N_cr,T = (G·I_t + π²·E·I_w/L²)/i₀², EN 1999-1-1, 6.3.1.4) against the compression demand; the marker drags L_cr,T.", "Bemessungswiderstand N_b,T,Rd über der Drillknicklänge L_cr,T (N_cr,T = (G·I_t + π²·E·I_w/L²)/i₀², EN 1999-1-1, 6.3.1.4) gegen die Druckbeanspruchung; der Marker verschiebt L_cr,T."),
        axes: vec![axis("Torsional buckling length L_cr,T", "Drillknicklänge L_cr,T", QuantityKind::Length, "m", AxisScale::Linear, Some((0.0, top))), axis("Resistance N_b,T,Rd", "Widerstand N_b,T,Rd", QuantityKind::Force, "kN", AxisScale::Linear, None)],
        series: vec![
            Series::Polyline { id: "n-b-t".into(), label: loc("N_b,T,Rd", "N_b,T,Rd"), points: lengths.iter().zip(&resistance).map(|(x, y)| Vec2::new(*x, *y)).collect(), style: stroke(Tone::Info, 2.5, Dash::Solid) },
            Series::Limit { id: "n-ed".into(), label: loc("Compression demand N_Ed", "Druckbeanspruchung N_Ed"), axis: AxisRef::Y, value: demand, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
        ],
        markers: vec![Marker { id: "marker-length".into(), at: Vec2::new(l_cr.min(top), n_b(l_cr)), shape: MarkerShape::Circle, tone, label: loc(&format!("L_cr,T = {l_cr:.2} m, N_b,T,Rd = {:.1} kN", n_b(l_cr)), &format!("L_cr,T = {l_cr:.2} m, N_b,T,Rd = {:.1} kN", n_b(l_cr))), utilisation: Some(utilisation), bound_check: Some(check.id.clone()), binding: Some(binding) }],
        annotations: Vec::new(),
        focus: member_focus(id),
        legend: true,
        table_twin: DataTable { columns: vec![column("Torsional buckling length L_cr,T", "Drillknicklänge L_cr,T", "m", lengths), column("Resistance N_b,T,Rd", "Widerstand N_b,T,Rd", "kN", resistance)] },
    };
    vec![slenderness, by_length]
}

/// 🔥️ The diagram `haz-buckling:<id>` of the check `en1999.6.3.1.haz.<id>`: χ, κ and κ·χ over λ̄ and the design point of the governing axis, `None` without the check.
pub(super) fn welded(c: &Ctx) -> Option<Diagram2d> {
    let check = exact(c.report, &format!("en1999.6.3.1.haz.{}", c.member.id))?;
    let (alpha, lambda_0) = part_1_1::buckling_curve(c.alloy.buckling_class_a);
    let a_loc = part_1_1::props(c.section, &c.base()).a;
    let (_, i_y, i_z, ..) = part_1_1::section_geometry(c.section);
    let k0 = kappa_0(c);
    let fo = c.alloy.f_o_pa;
    let pick = |l: f64| if l > 0.0 { l } else { k_support(c.member) * c.member.length };
    let omega = |kc: f64, l: f64| if c.member.transverse_weld { part_1_1::omega_x(kc, c.alloy.rho_o_haz, c.member.transverse_weld_offset, l, 1.0) } else { 1.0 };
    let axis_of = |i: f64, l: f64| {
        let lambda = part_1_1::lambda_bar(l, i, a_loc, fo);
        let kc = part_1_1::kappa(k0, lambda) * part_1_1::chi_from_lambda(lambda, alpha, lambda_0);
        (lambda, kc, omega(kc, l))
    };
    let (y, z) = (axis_of(i_y, pick(c.member.buckling_length_y)), axis_of(i_z, pick(c.member.buckling_length_z)));
    let (lambda, kc, w) = if y.1 * y.2 <= z.1 * z.2 { y } else { z };
    let kappa_w = part_1_1::kappa(k0, lambda);
    let lambdas: Vec<f64> = (0..=SAMPLES).map(|i| LAMBDA_MAX * i as f64 / SAMPLES as f64).collect();
    let chi: Vec<f64> = lambdas.iter().map(|l| part_1_1::chi_from_lambda(*l, alpha, lambda_0)).collect();
    let kappa: Vec<f64> = lambdas.iter().map(|l| part_1_1::kappa(k0, *l)).collect();
    let both: Vec<f64> = chi.iter().zip(&kappa).map(|(x, k)| x * k).collect();
    let line = |values: &[f64]| lambdas.iter().zip(values).map(|(x, y)| Vec2::new(*x, *y)).collect::<Vec<_>>();
    let id = &c.member.id;
    Some(Diagram2d {
        id: format!("haz-buckling:{id}"),
        title: loc(&format!("Welded member buckling – {id}"), &format!("Knicken geschweißter Bauteile – {id}")),
        description: loc("Reduction χ, the HAZ factor κ = κ₀ + (1 − κ₀)·(λ̄ − 0.2) and their product over the relative slenderness λ̄ (EN 1999-1-1, 6.3.1.1); κ₀ is the ratio of the HAZ-reduced to the local effective area. The marker is the governing axis, including ω_x of a transverse weld.", "Abminderung χ, der WEZ-Faktor κ = κ₀ + (1 − κ₀)·(λ̄ − 0,2) und ihr Produkt über dem bezogenen Schlankheitsgrad λ̄ (EN 1999-1-1, 6.3.1.1); κ₀ ist das Verhältnis der WEZ-abgeminderten zur lokal wirksamen Fläche. Der Marker ist die maßgebende Achse, mit ω_x einer Quernaht."),
        axes: vec![axis("Relative slenderness λ̄", "Bezogener Schlankheitsgrad λ̄", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, LAMBDA_MAX))), axis("Reduction factor", "Abminderungsfaktor", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, 1.1)))],
        series: vec![
            Series::Polyline { id: "chi".into(), label: loc("χ", "χ"), points: line(&chi), style: stroke(Tone::Info, 2.0, Dash::Solid) },
            Series::Polyline { id: "kappa".into(), label: loc(&format!("κ with κ₀ = {k0:.3}"), &format!("κ mit κ₀ = {k0:.3}")), points: line(&kappa), style: stroke(Tone::Neutral, 1.5, Dash::Dashed) },
            Series::Polyline { id: "kappa-chi".into(), label: loc("κ·χ", "κ·χ"), points: line(&both), style: stroke(Tone::Warning, 2.5, Dash::Solid) },
        ],
        markers: vec![Marker {
            id: "marker-haz".into(),
            at: Vec2::new(lambda.min(LAMBDA_MAX), kc * w),
            shape: MarkerShape::Circle,
            tone: utilisation_tone(check.utilization),
            label: loc(&format!("λ̄ = {lambda:.2}, κ = {kappa_w:.3}, ω_x = {w:.3}"), &format!("λ̄ = {lambda:.2}, κ = {kappa_w:.3}, ω_x = {w:.3}")),
            utilisation: Some(check.utilization),
            bound_check: Some(check.id.clone()),
            binding: None,
        }],
        annotations: Vec::new(),
        focus: member_focus(id),
        legend: true,
        table_twin: DataTable { columns: vec![lambda_column("Relative slenderness λ̄", "Bezogener Schlankheitsgrad λ̄"), column("χ", "χ", "1", chi), column("κ", "κ", "1", kappa), column("κ·χ", "κ·χ", "1", both)] },
    })
}
