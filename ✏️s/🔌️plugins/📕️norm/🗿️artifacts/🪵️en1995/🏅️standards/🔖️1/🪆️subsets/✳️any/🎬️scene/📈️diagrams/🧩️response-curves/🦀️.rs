//! 🧩️ Response curves of the catalogue cases that have no typed entity: the buckling factor k_c of the columns T-20, T-21 and T-23 over the relative slenderness, k_crit of the beam T-22 over the effective length, the final deflection of T-61 over k_def, the embedment strength of T-70 over the angle to the grain and the char depth behind the cladding of T-103 over the fire duration. Each is a pure projection of the case row with its table twin; the markers drag the case leaves. See [`super`].

use super::components::{limit, tone_of};
use super::special::{diagram, marker, val};
use super::*;
use crate::norm::design_case::DesignCase;

fn leaf(c: &CaseRow, key: &str, unit: &str, k: f64, step: f64) -> Option<FieldBinding> {
    val(c, key).map(|_| binding(case_binding(c, key), unit, Affine { base: 0.0, k }, step))
}

fn buckling(report: &CheckReport, c: &CaseRow, input: &t::Case) -> Option<Diagram2d> {
    let p = t::column_probe(input).ok()?;
    let hi = LAMBDA_MAX.max(1.1 * p.y.lambda_rel.max(p.z.lambda_rel));
    let kc = curves::sample_with_breaks(|x| t::kc_factor(x, p.beta_c), 0.0, hi, SAMPLES, &[0.3]);
    let euler: Vec<Xy> = kc.iter().map(|q| [q[0], if q[0] > 0.3 { (1.0 / (q[0] * q[0])).min(1.0) } else { 1.0 }]).collect();
    let prefix = format!("en1995.{}.buckling_", c.entry_id.to_lowercase());
    let markers = [("y", p.y), ("z", p.z)].map(|(name, axis)| {
        let check = worst(report, &c.id, &format!("{prefix}{name}"));
        let k = if axis.lambda_rel > 1e-9 { axis.lef / axis.lambda_rel } else { 0.0 };
        marker(&format!("marker-lef-{name}"), Vec2::new(axis.lambda_rel, axis.kc), tone_of(check), format!("l_ef,{name} = {:.0} mm, λ_rel,{name} = {:.2}, k_c,{name} = {:.3}", axis.lef, axis.lambda_rel, axis.kc), check.map(|k| k.utilization), check.map(|k| k.id.clone()), (k > 0.0).then(|| binding(case_binding(c, &format!("lef_{name}")), "mm", Affine { base: 0.0, k }, 10.0)))
    });
    let series = vec![polyline("k-c", "k_c", "k_c", kc.clone(), stroke(Tone::Info, 2.0, Dash::Solid)), polyline("euler", "1/λ_rel² (Euler)", "1/λ_rel² (Euler)", euler.clone(), stroke(Tone::Neutral, 1.5, Dash::Dotted))];
    let twin = vec![column("Relative slenderness λ_rel", "Bezogener Schlankheitsgrad λ_rel", "1", kc.iter().map(|q| q[0]).collect()), column("k_c", "k_c", "1", kc.iter().map(|q| q[1]).collect()), column("Euler 1/λ_rel²", "Euler 1/λ_rel²", "1", euler.iter().map(|q| q[1]).collect())];
    Some(diagram(c, "case-buckling", ("Column: k_c over the relative slenderness", "Druckstab: k_c über dem bezogenen Schlankheitsgrad"), (&format!("k_c of EN 1995-1-1 (6.25 – 6.28, β_c = {:.1}) against the Euler curve; the markers are the two buckling axes and drag their effective lengths.", p.beta_c), &format!("k_c nach EN 1995-1-1 (6.25 – 6.28, β_c = {:.1}) gegen die Euler-Kurve; die Marker sind die beiden Knickachsen und ändern ihre Knicklängen.", p.beta_c)), vec![axis("Relative slenderness λ_rel", "Bezogener Schlankheitsgrad λ_rel", QuantityKind::Dimensionless, "1", Some((0.0, hi)), TickRule::Auto), axis("k_c", "k_c", QuantityKind::Dimensionless, "1", Some((0.0, 1.05)), TickRule::Auto)], series, markers.into_iter().collect(), twin))
}

fn lateral(report: &CheckReport, c: &CaseRow, input: &t::Case) -> Option<Diagram2d> {
    let p = t::ltb_probe(input).ok()?;
    let (lo, hi) = (0.25 * p.lef, 3.0 * p.lef);
    let curve = curves::sample_with_breaks(|x| p.at(x).1, lo, hi, SAMPLES, &[]);
    let relative: Vec<Xy> = curve.iter().map(|q| [q[0], p.at(q[0]).0]).collect();
    let (rel, kcrit) = p.at(p.lef);
    let check = worst(report, &c.id, "en1995.t-22.ltb");
    let m = marker("marker-lef", Vec2::new(p.lef, kcrit), tone_of(check), format!("l_ef = {:.0} mm, λ_rel,m = {rel:.2}, k_crit = {kcrit:.3}", p.lef), check.map(|k| k.utilization), check.map(|k| k.id.clone()), leaf(c, "lef", "mm", 1.0, 10.0));
    let twin = vec![column("Effective length l_ef", "Ersatzstablänge l_ef", "mm", curve.iter().map(|q| q[0]).collect()), column("λ_rel,m", "λ_rel,m", "1", relative.iter().map(|q| q[1]).collect()), column("k_crit", "k_crit", "1", curve.iter().map(|q| q[1]).collect())];
    Some(diagram(c, "case-ltb", ("Beam: k_crit over the effective length", "Biegeträger: k_crit über der Ersatzstablänge"), ("k_crit of EN 1995-1-1 (6.34) with σ_m,crit = 0.78 b² E₀,₀₅ / (h l_ef) (6.32) over the effective length; the marker drags l_ef when the case gives it.", "k_crit nach EN 1995-1-1 (6.34) mit σ_m,crit = 0,78 b² E₀,₀₅ / (h l_ef) (6.32) über der Ersatzstablänge; der Marker ändert l_ef, wenn der Fall sie vorgibt."), vec![axis("Effective length l_ef", "Ersatzstablänge l_ef", QuantityKind::Length, "mm", Some((lo, hi)), TickRule::Auto), axis("k_crit", "k_crit", QuantityKind::Dimensionless, "1", Some((0.0, 1.05)), TickRule::Auto)], vec![polyline("k-crit", "k_crit", "k_crit", curve, stroke(Tone::Info, 2.0, Dash::Solid))], vec![m], twin))
}

fn deflection(report: &CheckReport, c: &CaseRow, input: &t::Case) -> Option<Diagram2d> {
    let p = t::deflection_probe(input).ok()?;
    let hi = 2.5_f64.max(1.2 * p.k_def);
    let fin = curves::sample(|k| p.at(k).0, 0.0, hi, SAMPLES);
    let net = curves::sample(|k| p.at(k).1, 0.0, hi, SAMPLES);
    let (now, _) = p.at(p.k_def);
    let check = worst(report, &c.id, "en1995.t-61.w_fin");
    let m = marker("marker-k-def", Vec2::new(p.k_def, now), tone_of(check), format!("k_def = {:.2}, w_fin = {now:.1} mm", p.k_def), check.map(|k| k.utilization), check.map(|k| k.id.clone()), leaf(c, "k_def", "1", 1.0, 0.05));
    let series = vec![polyline("w-fin", "w_fin", "w_fin", fin.clone(), stroke(Tone::Info, 2.0, Dash::Solid)), polyline("w-net-fin", "w_net,fin", "w_net,fin", net.clone(), stroke(Tone::Warning, 2.0, Dash::Solid)), limit("limit-fin", "Limit w_fin", "Grenze w_fin", (0.0, hi), p.limit_fin), limit("limit-net", "Limit w_net,fin", "Grenze w_net,fin", (0.0, hi), p.limit_net)];
    let twin = vec![column("Deformation factor k_def", "Verformungsbeiwert k_def", "1", fin.iter().map(|q| q[0]).collect()), column("w_fin", "w_fin", "mm", fin.iter().map(|q| q[1]).collect()), column("w_net,fin", "w_net,fin", "mm", net.iter().map(|q| q[1]).collect())];
    Some(diagram(c, "case-deflection", ("Final deflection over the deformation factor", "Enddurchbiegung über dem Verformungsbeiwert"), ("w_fin = w_G(1 + k_def) + w_Q(1 + ψ₂ k_def) and w_net,fin = w_fin − w_c (EN 1995-1-1 2.2.3, 7.2) over k_def against both limits; the marker drags k_def when the case gives it.", "w_fin = w_G(1 + k_def) + w_Q(1 + ψ₂ k_def) und w_net,fin = w_fin − w_c (EN 1995-1-1 2.2.3, 7.2) über k_def gegen beide Grenzen; der Marker ändert k_def, wenn der Fall ihn vorgibt."), vec![axis("Deformation factor k_def", "Verformungsbeiwert k_def", QuantityKind::Dimensionless, "1", Some((0.0, hi)), TickRule::Auto), axis("Deflection", "Durchbiegung", QuantityKind::Length, "mm", None, TickRule::Auto)], series, vec![m], twin))
}

fn embedment(c: &CaseRow, input: &t::Case) -> Option<Diagram2d> {
    let p = t::embedment_probe(input).ok().filter(t::EmbedmentProbe::angle_dependent)?;
    let curve = curves::sample(|a| p.at(a), 0.0, 90.0, SAMPLES);
    let now = p.at(p.alpha_deg.clamp(0.0, 90.0));
    let m = marker("marker-alpha", Vec2::new(p.alpha_deg.clamp(0.0, 90.0), now), Tone::Neutral, format!("α = {:.0}°, f_h,α,k = {now:.1} N/mm²", p.alpha_deg), None, None, leaf(c, "alpha_deg", "°", 1.0, 1.0));
    let twin = vec![column("Angle to the grain α", "Winkel zur Faser α", "°", curve.iter().map(|q| q[0]).collect()), column("f_h,α,k", "f_h,α,k", "N/mm²", curve.iter().map(|q| q[1]).collect())];
    Some(diagram(c, "case-embedment", ("Embedment strength over the angle to the grain", "Lochleibungsfestigkeit über dem Winkel zur Faser"), ("f_h,α,k = f_h,0,k / (k₉₀ sin²α + cos²α) (EN 1995-1-1 8.31) of the fastener over the angle between load and grain; the marker drags α when the case gives it.", "f_h,α,k = f_h,0,k / (k₉₀ sin²α + cos²α) (EN 1995-1-1 8.31) des Verbindungsmittels über dem Winkel zwischen Kraft und Faser; der Marker ändert α, wenn der Fall ihn vorgibt."), vec![axis("Angle to the grain α", "Winkel zur Faser α", QuantityKind::Angle, "°", Some((0.0, 90.0)), TickRule::Auto), axis("Embedment strength f_h,α,k", "Lochleibungsfestigkeit f_h,α,k", QuantityKind::Stress, "N/mm²", None, TickRule::Auto)], vec![polyline("f-h", "f_h,α,k", "f_h,α,k", curve, stroke(Tone::Info, 2.0, Dash::Solid))], vec![m], twin))
}

fn cladding(report: &CheckReport, c: &CaseRow, input: &t::Case) -> Option<Diagram2d> {
    let p = t::protected_probe(input).ok()?;
    let end = (1.5 * p.t_min).max(p.t_f + 30.0).max(60.0);
    let protected = curves::sample_with_breaks(|x| p.depth_at(x), 0.0, end, SAMPLES, &[p.t_ch, p.t_f]);
    let bare: Vec<Xy> = protected.iter().map(|q| [q[0], p.b0 * q[0]]).collect();
    let top = bare.last().map_or(1.0, |q| q[1]);
    let now = p.depth_at(p.t_min);
    let check = worst(report, &c.id, "en1995.t-103");
    let m = marker("marker-t-min", Vec2::new(p.t_min, now), tone_of(check), format!("{:.0} min: d_char = {now:.1} mm", p.t_min), check.map(|k| k.utilization), check.map(|k| k.id.clone()), leaf(c, "t_min", "min", 1.0, 1.0));
    let series = vec![polyline("protected", "Behind the cladding", "Hinter der Bekleidung", protected.clone(), stroke(Tone::Error, 2.0, Dash::Solid)), polyline("bare", "Unprotected β₀ t", "Ungeschützt β₀ t", bare.clone(), stroke(Tone::Neutral, 1.5, Dash::Dashed)), polyline("start-of-charring", "t_ch", "t_ch", vec![[p.t_ch, 0.0], [p.t_ch, top]], stroke(Tone::Info, 1.0, Dash::Dotted)), polyline("cladding-failure", "t_f", "t_f", vec![[p.t_f, 0.0], [p.t_f, top]], stroke(Tone::Warning, 1.0, Dash::Dotted))];
    let twin = vec![column("Fire duration", "Branddauer", "min", protected.iter().map(|q| q[0]).collect()), column("Char depth behind the cladding", "Abbrandtiefe hinter der Bekleidung", "mm", protected.iter().map(|q| q[1]).collect()), column("Char depth unprotected", "Abbrandtiefe ungeschützt", "mm", bare.iter().map(|q| q[1]).collect())];
    Some(diagram(c, "case-cladding", ("Protected member: char depth over the fire duration", "Geschütztes Bauteil: Abbrandtiefe über der Branddauer"), (&format!("Charring starts at t_ch = {:.1} min behind the {:.0} mm board, runs at k₂β₀ until the board fails at t_f = {:.1} min and at 2β₀ afterwards (EN 1995-1-2 3.4.3.3); the marker drags the required time.", p.t_ch, p.hp, p.t_f), &format!("Der Abbrand beginnt bei t_ch = {:.1} min hinter der {:.0} mm dicken Platte, läuft mit k₂β₀ bis die Platte bei t_f = {:.1} min versagt und danach mit 2β₀ (EN 1995-1-2 3.4.3.3); der Marker ändert die geforderte Zeit.", p.t_ch, p.hp, p.t_f)), vec![axis("Fire duration", "Branddauer", QuantityKind::Time, "min", Some((0.0, end)), TickRule::Auto), axis("Char depth", "Abbrandtiefe", QuantityKind::Length, "mm", Some((0.0, top)), TickRule::Auto)], series, vec![m], twin))
}

/// 🧩️ The response curves of one case, chosen by its catalogue entry.
pub(super) fn case_diagrams(document: &En1995Snapshot, report: &CheckReport, c: &CaseRow) -> Vec<Diagram2d> {
    let input = t::case_input(document.annex, &DesignCase::from(c));
    match c.entry_id.as_str() {
        "T-20" | "T-21" | "T-23" => buckling(report, c, &input).into_iter().collect(),
        "T-22" => lateral(report, c, &input).into_iter().collect(),
        "T-61" => deflection(report, c, &input).into_iter().collect(),
        "T-70" => embedment(c, &input).into_iter().collect(),
        "T-103" => cladding(report, c, &input).into_iter().collect(),
        _ => Vec::new(),
    }
}
