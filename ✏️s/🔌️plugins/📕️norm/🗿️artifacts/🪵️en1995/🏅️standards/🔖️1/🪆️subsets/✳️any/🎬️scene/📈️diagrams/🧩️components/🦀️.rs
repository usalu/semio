//! 🧩️ Diagrams of the tapered beam T-30, the notch T-45, the diaphragm T-94, the truss T-95, the bracing T-96 and the stress-laminated deck T-122, each a pure projection of the case row with its table twin: the factor k_m,α and the edge utilisation over the taper angle, k_v and the notch shear utilisation over the remaining depth, the racking resistance over the panel width and the shear flow over the diaphragm length, the stabilising load over the restraint length, the bracing deflection over its stiffness and the wheel spread stress over the deck depth.

use super::special::{diagram, marker, text, val};
use super::*;
use crate::norm::design_case::DesignCase;

pub(super) fn tone_of(check: Option<&crate::norm::CheckResult>) -> Tone {
    check.map_or(Tone::Neutral, |k| utilisation_tone(k.utilization))
}

pub(super) fn limit(id: &str, en: &str, de: &str, x: (f64, f64), y: f64) -> Series {
    polyline(id, en, de, vec![[x.0, y], [x.1, y]], stroke(Tone::Error, 1.5, Dash::Dashed))
}

fn taper(document: &En1995Snapshot, report: &CheckReport, c: &CaseRow) -> Option<Diagram2d> {
    let p = t::taper_probe(&t::case_input(document.annex, &DesignCase::from(c))).ok()?;
    let k = |deg: f64| t::km_alpha(p.tension, deg.to_radians(), p.fmd, p.fvd, p.f_perp_d);
    let factor = curves::sample(k, 0.0, 10.0, SAMPLES);
    let check = worst(report, &c.id, "en1995.t-30.tapered_edge");
    let at = p.alpha_deg.clamp(0.0, 10.0);
    let scale = check.map_or(0.0, |k0| k0.utilization * k(p.alpha_deg));
    let edge = curves::sample(|x| scale / k(x), 0.0, 10.0, SAMPLES);
    let m = marker("marker-alpha", Vec2::new(at, k(at)), tone_of(check), format!("α = {at:.1}°, k_m,α = {:.3}", k(at)), check.map(|k| k.utilization), check.map(|k| k.id.clone()), Some(binding(case_binding(c, "alpha_deg"), "°", Affine::IDENTITY, 0.1)));
    let mut series = vec![polyline("km-alpha", "k_m,α", "k_m,α", factor.clone(), stroke(Tone::Info, 2.0, Dash::Solid))];
    let mut twin = vec![column("Taper angle α", "Neigungswinkel α", "°", factor.iter().map(|q| q[0]).collect()), column("k_m,α", "k_m,α", "1", factor.iter().map(|q| q[1]).collect())];
    if check.is_some() {
        series.push(polyline("edge-utilisation", "σ_m,α,d / (k_m,α f_m,d)", "σ_m,α,d / (k_m,α f_m,d)", edge.clone(), stroke(Tone::Warning, 2.0, Dash::Solid)));
        series.push(limit("limit", "Utilisation 1", "Ausnutzung 1", (0.0, 10.0), 1.0));
        twin.push(column("Edge utilisation", "Randausnutzung", "1", edge.iter().map(|q| q[1]).collect()));
    }
    Some(diagram(c, "case-taper", ("Tapered beam: k_m,α over the taper angle", "Satteldachträger: k_m,α über dem Neigungswinkel"), ("k_m,α of EN 1995-1-1 (6.38, 6.39) and the utilisation of the tapered edge over the taper angle α up to the 10° limit; the marker drags α.", "k_m,α nach EN 1995-1-1 (6.38, 6.39) und Ausnutzung des angeschnittenen Rands über dem Neigungswinkel α bis zur Grenze 10°; der Marker ändert α."), vec![axis("Taper angle α", "Neigungswinkel α", QuantityKind::Angle, "°", Some((0.0, 10.0)), TickRule::Auto), axis("Factor and utilisation", "Beiwert und Ausnutzung", QuantityKind::Dimensionless, "1", Some((0.0, 1.5)), TickRule::Auto)], series, vec![m], twin))
}

fn notch(document: &En1995Snapshot, report: &CheckReport, c: &CaseRow) -> Option<Diagram2d> {
    let p = t::notch_probe(&t::case_input(document.annex, &DesignCase::from(c))).ok()?;
    let (lo, hi) = (0.3, 1.0);
    let kv = curves::sample(|a| p.kv(a), lo, hi, SAMPLES);
    let alpha = (p.hef / p.h).clamp(lo, hi);
    let check = worst(report, &c.id, "en1995.t-45.notch_shear");
    let scale = check.map_or(0.0, |k0| k0.utilization * alpha * p.kv(alpha));
    let utilisation = curves::sample(|a| (scale / (a * p.kv(a))).min(3.0), lo, hi, SAMPLES);
    let m = marker("marker-h-ef", Vec2::new(alpha, p.kv(alpha)), tone_of(check), format!("h_ef/h = {alpha:.2}, k_v = {:.3}", p.kv(alpha)), check.map(|k| k.utilization), check.map(|k| k.id.clone()), Some(binding(case_binding(c, "h_ef"), "mm", Affine { base: 0.0, k: p.h }, 1.0)));
    let mut series = vec![polyline("k-v", "k_v", "k_v", kv.clone(), stroke(Tone::Info, 2.0, Dash::Solid))];
    let mut twin = vec![column("Remaining depth h_ef/h", "Restquerschnitt h_ef/h", "1", kv.iter().map(|q| q[0]).collect()), column("k_v", "k_v", "1", kv.iter().map(|q| q[1]).collect())];
    if check.is_some() {
        series.push(polyline("shear-utilisation", "τ_d / (k_v f_v,d)", "τ_d / (k_v f_v,d)", utilisation.clone(), stroke(Tone::Warning, 2.0, Dash::Solid)));
        series.push(limit("limit", "Utilisation 1", "Ausnutzung 1", (lo, hi), 1.0));
        twin.push(column("Shear utilisation", "Schubausnutzung", "1", utilisation.iter().map(|q| q[1]).collect()));
    }
    Some(diagram(c, "case-notch", ("Notch: k_v over the remaining depth", "Ausklinkung: k_v über der Restträgerhöhe"), ("k_v of EN 1995-1-1 (6.62) and the shear utilisation at the notch over the ratio h_ef/h; the marker drags the remaining depth h_ef.", "k_v nach EN 1995-1-1 (6.62) und Schubausnutzung an der Ausklinkung über dem Verhältnis h_ef/h; der Marker ändert die Resthöhe h_ef."), vec![axis("Remaining depth h_ef/h", "Restträgerhöhe h_ef/h", QuantityKind::Dimensionless, "1", Some((lo, hi)), TickRule::Auto), axis("Factor and utilisation", "Beiwert und Ausnutzung", QuantityKind::Dimensionless, "1", Some((0.0, 3.0)), TickRule::Auto)], series, vec![m], twin))
}

fn racking(report: &CheckReport, c: &CaseRow, input: &t::Case) -> Option<Diagram2d> {
    let (ff, bi, h, s, fed) = (val(c, "f_f_rd")?, val(c, "b_i")?, val(c, "h")?, val(c, "s")?, val(c, "f_v_ed")?);
    if bi <= 0.0 || h <= 0.0 || s <= 0.0 {
        return None;
    }
    let factor = t::racking_factor(input) * val(c, "n_panels").unwrap_or(1.0);
    let resistance = |b: f64| factor * t::wall_panel_racking(ff, b, h, s);
    let hi = (2.0 * h).max(1.5 * bi);
    let curve = curves::sample(resistance, 0.05 * hi, hi, SAMPLES);
    let check = worst(report, &c.id, "en1995.t-94.racking");
    let m = marker("marker-b-i", Vec2::new(bi, resistance(bi)), tone_of(check), format!("b_i = {bi:.0} mm, ΣF_i,v,Rd = {:.0} N", resistance(bi)), check.map(|k| k.utilization), check.map(|k| k.id.clone()), Some(binding(case_binding(c, "b_i"), "mm", Affine::IDENTITY, 10.0)));
    let series = vec![polyline("racking", "ΣF_i,v,Rd", "ΣF_i,v,Rd", curve.clone(), stroke(Tone::Info, 2.0, Dash::Solid)), limit("demand", "F_v,Ed", "F_v,Ed", (0.05 * hi, hi), fed)];
    let twin = vec![column("Panel width b_i", "Plattenbreite b_i", "mm", curve.iter().map(|q| q[0]).collect()), column("Racking resistance", "Wandscheibentragfähigkeit", "N", curve.iter().map(|q| q[1]).collect())];
    Some(diagram(c, "case-racking", ("Wall racking over the panel width", "Wandscheibe über der Plattenbreite"), ("Racking resistance of the wall panels (EN 1995-1-1 9.2.4, c_i = b_i/b₀ below b₀ = h/2) against the design shear; the marker drags the panel width.", "Wandscheibentragfähigkeit (EN 1995-1-1 9.2.4, c_i = b_i/b₀ unter b₀ = h/2) gegen die Bemessungsquerkraft; der Marker ändert die Plattenbreite."), vec![axis("Panel width b_i", "Plattenbreite b_i", QuantityKind::Length, "mm", Some((0.05 * hi, hi)), TickRule::Auto), axis("Racking resistance", "Tragfähigkeit", QuantityKind::Force, "N", None, TickRule::Auto)], series, vec![m], twin))
}

fn shear_flow(report: &CheckReport, c: &CaseRow) -> Option<Diagram2d> {
    let (fd, l, q_rd) = (val(c, "f_d")?, val(c, "l_dia")?, val(c, "q_v_rd")?);
    if l <= 0.0 {
        return None;
    }
    let curve = curves::sample(|x| fd / x, 0.25 * l, 2.0 * l, SAMPLES);
    let check = worst(report, &c.id, "en1995.t-94.diaphragm_shear");
    let m = marker("marker-l-dia", Vec2::new(l, fd / l), tone_of(check), format!("l = {l:.0} mm, q_v,Ed = {:.2} N/mm", fd / l), check.map(|k| k.utilization), check.map(|k| k.id.clone()), Some(binding(case_binding(c, "l_dia"), "mm", Affine::IDENTITY, 10.0)));
    let series = vec![polyline("shear-flow", "q_v,Ed = F_d / l", "q_v,Ed = F_d / l", curve.clone(), stroke(Tone::Info, 2.0, Dash::Solid)), limit("resistance", "q_v,Rd", "q_v,Rd", (0.25 * l, 2.0 * l), q_rd)];
    let twin = vec![column("Diaphragm length l", "Scheibenlänge l", "mm", curve.iter().map(|q| q[0]).collect()), column("Shear flow q_v,Ed", "Schubfluss q_v,Ed", "N/mm", curve.iter().map(|q| q[1]).collect())];
    Some(diagram(c, "case-shear-flow", ("Diaphragm shear flow over the length", "Schubfluss der Scheibe über der Länge"), ("Shear flow F_d/l of the roof or floor diaphragm (EN 1995-1-1 9.2.3) against its shear resistance; the marker drags the diaphragm length.", "Schubfluss F_d/l der Dach- oder Deckenscheibe (EN 1995-1-1 9.2.3) gegen ihre Schubtragfähigkeit; der Marker ändert die Scheibenlänge."), vec![axis("Diaphragm length l", "Scheibenlänge l", QuantityKind::Length, "mm", Some((0.25 * l, 2.0 * l)), TickRule::Auto), axis("Shear flow", "Schubfluss", QuantityKind::ForcePerLength, "N/mm", None, TickRule::Auto)], series, vec![m], twin))
}

fn truss(report: &CheckReport, c: &CaseRow) -> Option<Diagram2d> {
    let (n, nd, l, q_rd) = (val(c, "n_braced")?, val(c, "n_d")?, val(c, "l_m")?, val(c, "q_rd")?);
    if l <= 0.0 {
        return None;
    }
    let q = |x: f64| t::bracing_qd(n, nd, x);
    let curve = curves::sample(q, 0.25 * l, 2.0 * l, SAMPLES);
    let check = worst(report, &c.id, "en1995.t-95");
    let m = marker("marker-l-m", Vec2::new(l, q(l)), tone_of(check), format!("l = {l:.2} m, q_d = {:.1} N/m", q(l)), check.map(|k| k.utilization), check.map(|k| k.id.clone()), Some(binding(case_binding(c, "l_m"), "m", Affine::IDENTITY, 0.05)));
    let series = vec![polyline("stabilising-load", "q_d", "q_d", curve.clone(), stroke(Tone::Info, 2.0, Dash::Solid)), limit("capacity", "q_Rd", "q_Rd", (0.25 * l, 2.0 * l), q_rd)];
    let twin = vec![column("Restraint length l", "Haltelänge l", "m", curve.iter().map(|q| q[0]).collect()), column("Stabilising load q_d", "Stabilisierungslast q_d", "N/m", curve.iter().map(|q| q[1]).collect())];
    Some(diagram(c, "case-truss-chord", ("Truss chord: stabilising load over the restraint length", "Fachwerkgurt: Stabilisierungslast über der Haltelänge"), ("Stabilising load q_d = k_l n N_d/(k_f,3 l) of the compressed chord (EN 1995-1-1 9.2.5.3, 9.37, 9.38) against the capacity of the bracing system; the marker drags the restraint length.", "Stabilisierungslast q_d = k_l n N_d/(k_f,3 l) des gedrückten Gurts (EN 1995-1-1 9.2.5.3, 9.37, 9.38) gegen die Tragfähigkeit des Aussteifungssystems; der Marker ändert die Haltelänge."), vec![axis("Restraint length l", "Haltelänge l", QuantityKind::Length, "m", Some((0.25 * l, 2.0 * l)), TickRule::Auto), axis("Stabilising load", "Stabilisierungslast", QuantityKind::ForcePerLength, "N/m", None, TickRule::Auto)], series, vec![m], twin))
}

fn bracing(report: &CheckReport, c: &CaseRow) -> Option<Diagram2d> {
    let (n, nd, l, k) = (val(c, "n_braced")?, val(c, "n_d")?, val(c, "l_m")?, val(c, "k_br")?);
    if l <= 0.0 || k <= 0.0 {
        return None;
    }
    let force = t::bracing_qd(n, nd, l) * l;
    let allowed = l * 1e3 / crate::norm::ndp_common("ndp.en1995.bracing.deflection-limit-divisor");
    let required = force / allowed;
    let deflection = |x: f64| force / x;
    let curve = curves::sample(deflection, 0.2 * k.min(required), 3.0 * k.max(required), SAMPLES);
    let check = worst(report, &c.id, "en1995.t-96.bracing_deflection");
    let m = marker("marker-k-br", Vec2::new(k, deflection(k)), tone_of(check), format!("k_br = {k:.1} N/mm, δ = {:.2} mm, k_req = {required:.1} N/mm", deflection(k)), check.map(|k| k.utilization), check.map(|k| k.id.clone()), Some(binding(case_binding(c, "k_br"), "N/mm", Affine::IDENTITY, 1.0)));
    let range = (curve[0][0], curve[curve.len() - 1][0]);
    let series = vec![polyline("deflection", "δ = ΣF_d / k_br", "δ = ΣF_d / k_br", curve.clone(), stroke(Tone::Info, 2.0, Dash::Solid)), limit("limit", "Limit L/500", "Grenze L/500", range, allowed)];
    let twin = vec![column("Stiffness k_br", "Steifigkeit k_br", "N/mm", curve.iter().map(|q| q[0]).collect()), column("Deflection δ", "Verformung δ", "mm", curve.iter().map(|q| q[1]).collect())];
    Some(diagram(c, "case-bracing", ("Bracing: deflection over the stiffness", "Aussteifung: Verformung über der Steifigkeit"), ("Deflection of the bracing system under ΣF_d against the limit L/500 (EN 1995-1-1 9.2.5.3); the curve crosses the limit at the required stiffness, the marker drags k_br.", "Verformung des Aussteifungssystems unter ΣF_d gegen die Grenze L/500 (EN 1995-1-1 9.2.5.3); die Kurve schneidet die Grenze bei der erforderlichen Steifigkeit, der Marker ändert k_br."), vec![axis("Stiffness k_br", "Steifigkeit k_br", QuantityKind::ForcePerLength, "N/mm", Some(range), TickRule::Auto), axis("Deflection", "Verformung", QuantityKind::Length, "mm", None, TickRule::Auto)], series, vec![m], twin))
}

fn wheel_spread(report: &CheckReport, c: &CaseRow) -> Option<Diagram2d> {
    let (q, a, h, f_c90) = (val(c, "wheel_n")?, val(c, "contact_mm")?, val(c, "h")?, val(c, "f_c_90_d").unwrap_or(1.0));
    if a <= 0.0 || h <= 0.0 {
        return None;
    }
    let stress = |x: f64| q / ((a + 2.0 * x) * a);
    let curve = curves::sample(stress, 0.25 * h, 2.0 * h, SAMPLES);
    let spread = curves::sample(|x| a + 2.0 * x, 0.25 * h, 2.0 * h, SAMPLES);
    let check = worst(report, &c.id, "en1995.t-122.wheel_spread");
    let m = marker("marker-h", Vec2::new(h, stress(h)), tone_of(check), format!("h = {h:.0} mm, b_ef = {:.0} mm, σ = {:.2} N/mm²", a + 2.0 * h, stress(h)), check.map(|k| k.utilization), check.map(|k| k.id.clone()), Some(binding(case_binding(c, "h"), "mm", Affine::IDENTITY, 5.0)));
    let series = vec![polyline("wheel-stress", "σ = Q / (b_ef a)", "σ = Q / (b_ef a)", curve.clone(), stroke(Tone::Info, 2.0, Dash::Solid)), limit("strength", "f_c,90,d", "f_c,90,d", (0.25 * h, 2.0 * h), f_c90)];
    let twin = vec![column("Deck depth h", "Deckenhöhe h", "mm", curve.iter().map(|q| q[0]).collect()), column("Effective width b_ef (unverified)", "Wirksame Breite b_ef (ungeprüft)", "mm", spread.iter().map(|q| q[1]).collect()), column("Wheel stress", "Radlastspannung", "N/mm²", curve.iter().map(|q| q[1]).collect())];
    Some(diagram(c, "case-wheel-spread", ("Deck: wheel load spread over the depth", "Deck: Radlastverteilung über der Höhe"), ("Wheel load spread b_ef = a + 2h and the bearing stress under the contact area over the deck depth (b_ef unverified, the Annex A.2 formula is not in the specs); the marker drags the depth.", "Radlastverteilung b_ef = a + 2h und Pressung unter der Aufstandsfläche über der Deckenhöhe (b_ef ungeprüft, die Formel aus Anhang A.2 liegt nicht in den Spezifikationen vor); der Marker ändert die Höhe."), vec![axis("Deck depth h", "Deckenhöhe h", QuantityKind::Length, "mm", Some((0.25 * h, 2.0 * h)), TickRule::Auto), axis("Wheel stress", "Radlastspannung", QuantityKind::Stress, "N/mm²", None, TickRule::Auto)], series, vec![m], twin))
}

/// 🧩️ The component diagrams of one case, chosen by its catalogue entry.
pub(super) fn case_diagrams(document: &En1995Snapshot, report: &CheckReport, c: &CaseRow) -> Vec<Diagram2d> {
    let input = || t::case_input(document.annex, &DesignCase::from(c));
    match c.entry_id.as_str() {
        "T-30" => taper(document, report, c).into_iter().collect(),
        "T-45" => notch(document, report, c).into_iter().collect(),
        "T-94" if text(c, "kind") == Some("wall") => racking(report, c, &input()).into_iter().collect(),
        "T-94" => shear_flow(report, c).into_iter().collect(),
        "T-95" => truss(report, c).into_iter().collect(),
        "T-96" => bracing(report, c).into_iter().collect(),
        "T-122" => wheel_spread(report, c).into_iter().collect(),
        _ => Vec::new(),
    }
}
