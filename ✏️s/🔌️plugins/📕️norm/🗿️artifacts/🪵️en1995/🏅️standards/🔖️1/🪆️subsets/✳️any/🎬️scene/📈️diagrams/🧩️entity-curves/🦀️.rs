//! 🧩️ Curves of the typed tapers, cutouts and bridge members, each a pure projection of the case row (or member) with its table twin and a draggable marker on a numeric geometric field: the utilisation of the tapered and the parallel edge along the length of the single tapered beam (T-30), the apex factors k_l, k_r and k_p over the inner radius (T-31, T-32), the apex reinforcement over the rod count (T-33), k_v and the shear utilisation over the notch depth (T-45), the reinforced notch over the notch depth (T-46), the net-section utilisation over the hole diameter (T-18) and the Miner damage of a bridge member over its design life (EN 1995-2 6.2). Utilisations are the check utilisation scaled by the exact ratio of the quantity that changes, so they follow the checks of the report. See [`super`].

use super::components::{limit, tone_of};
use super::special::{diagram, marker, val};
use super::*;
use crate::norm::design_case::DesignCase;

fn leaf(c: &CaseRow, key: &str, unit: &str, step: f64) -> Option<FieldBinding> {
    val(c, key).map(|_| binding(case_binding(c, key), unit, Affine::IDENTITY, step))
}

fn length_axis(en: &str, de: &str, unit: &str, domain: (f64, f64)) -> Axis {
    axis(en, de, QuantityKind::Length, unit, Some(domain), TickRule::Auto)
}

fn ratio_axis(en: &str, de: &str, domain: Option<(f64, f64)>) -> Axis {
    axis(en, de, QuantityKind::Dimensionless, "1", domain, TickRule::Auto)
}

fn xs(curve: &[Xy]) -> Vec<f64> {
    curve.iter().map(|q| q[0]).collect()
}

fn ys(curve: &[Xy]) -> Vec<f64> {
    curve.iter().map(|q| q[1]).collect()
}

fn vertical(id: &str, en: &str, de: &str, x: f64, top: f64) -> Series {
    polyline(id, en, de, vec![[x, 0.0], [x, top]], stroke(Tone::Error, 1.5, Dash::Dashed))
}

fn taper_length(document: &En1995Snapshot, report: &CheckReport, c: &CaseRow) -> Option<Diagram2d> {
    let mut p = t::taper_probe(&t::case_input(document.annex, &DesignCase::from(c))).ok()?;
    let check = worst(report, &c.id, "en1995.t-30.tapered_edge");
    let unit_moment = p.m_d <= 0.0;
    if unit_moment {
        p.m_d = 1.0;
    }
    let governing = p.governing();
    let scale = match check {
        Some(k0) if governing.tapered > 0.0 => k0.utilization / governing.tapered,
        _ if !unit_moment => 1.0,
        _ => return None,
    };
    let l = p.line.span;
    let tapered = |x: f64| scale * p.section(x).tapered;
    let parallel = |x: f64| scale * p.section(x).parallel;
    let edge = curves::sample(tapered, 0.0, l, SAMPLES);
    let straight = curves::sample(parallel, 0.0, l, SAMPLES);
    let depth = curves::sample(|x| p.section(x).h, 0.0, l, SAMPLES);
    let moment = curves::sample(|x| p.moment_at(x) / p.m_d, 0.0, l, SAMPLES);
    let u_max = scale * governing.tapered;
    let mut markers = vec![marker("marker-l", Vec2::new(l, tapered(l)), tone_of(check), format!("x = {l:.0} mm, σ_m,α,d / (k_m,α f_m,d) = {:.3}", tapered(l)), check.map(|k0| k0.utilization), check.map(|k0| k0.id.clone()), val(c, "l").map(|_| binding(case_binding(c, "l"), "mm", Affine { base: 0.0, k: val(c, "l").map_or(1.0, |span| span / l) }, 10.0)))];
    let mut series = vec![polyline("tapered-edge", "Tapered edge σ_m,α,d / (k_m,α f_m,d)", "Angeschnittener Rand σ_m,α,d / (k_m,α f_m,d)", edge.clone(), stroke(Tone::Warning, 2.0, Dash::Solid)), polyline("parallel-edge", "Parallel edge σ_m,0,d / f_m,d", "Paralleler Rand σ_m,0,d / f_m,d", straight.clone(), stroke(Tone::Info, 2.0, Dash::Solid)), limit("limit", "Utilisation 1", "Ausnutzung 1", (0.0, l), 1.0)];
    series.push(vertical("governing", &format!("Governing section x = {:.0} mm", governing.x), &format!("Maßgebender Schnitt x = {:.0} mm", governing.x), governing.x, u_max.max(1.0)));
    markers.push(marker("marker-governing", Vec2::new(governing.x, u_max), tone_of(check), format!("x = {:.0} mm, h = {:.0} mm, σ_m,α,d / (k_m,α f_m,d) = {u_max:.3}", governing.x, governing.h), check.map(|k0| k0.utilization), check.map(|k0| k0.id.clone()), None));
    let state = curves::sample(|x| if p.section(x).tension { 1.0 } else { 0.0 }, 0.0, l, SAMPLES);
    let twin = vec![column("Position x", "Lage x", "mm", xs(&edge)), column("Depth h(x)", "Höhe h(x)", "mm", ys(&depth)), column("Moment |M(x)| / M_max", "Moment |M(x)| / M_max", "1", ys(&moment)), column("Tapered edge in tension (1) or compression (0)", "Angeschnittener Rand auf Zug (1) oder Druck (0)", "1", ys(&state)), column("Tapered edge utilisation", "Ausnutzung angeschnittener Rand", "1", ys(&edge)), column("Parallel edge utilisation", "Ausnutzung paralleler Rand", "1", ys(&straight))];
    let (en, de) = (
        format!("Bending stress of the tapered and of the parallel edge along the member for the moment line of the load case, σ(x) = 6 |M(x)| / (b h(x)²) with h(x) = h + x tan α measured from the shallow end, as utilisation of k_m,α f_m,d (EN 1995-1-1 6.38, 6.39, k_h and f_m,d taken at the depth h(x)) and of f_m,d (6.37); where the moment turns against its peak (the fields of a continuous beam) the tapered edge changes between tension and compression and k_m,α follows; the dashed line marks the governing section x = {:.0} mm (6.4.2), the marker at the end drags the span.", governing.x),
        format!("Biegespannung des angeschnittenen und des parallelen Rands entlang des Bauteils für die Momentenlinie des Lastfalls, σ(x) = 6 |M(x)| / (b h(x)²) mit h(x) = h + x tan α ab dem flachen Ende, als Ausnutzung von k_m,α f_m,d (EN 1995-1-1 6.38, 6.39, k_h und f_m,d bei der Höhe h(x)) und von f_m,d (6.37); wo das Moment gegen seinen Größtwert wechselt (die Felder eines Durchlaufträgers), wechselt der angeschnittene Rand zwischen Zug und Druck und k_m,α folgt; die gestrichelte Linie markiert den maßgebenden Schnitt x = {:.0} mm (6.4.2), der Marker am Ende ändert die Spannweite.", governing.x),
    );
    Some(diagram(c, "case-taper-length", ("Tapered beam: edge stress along the length", "Satteldachträger: Randspannung entlang der Länge"), (&en, &de), vec![length_axis("Position x", "Lage x", "mm", (0.0, l)), ratio_axis("Utilisation", "Ausnutzung", None)], series, markers, twin))
}

struct Apex {
    alpha: f64,
    h_ap: f64,
    r_in: f64,
}

fn apex_of(c: &CaseRow) -> Option<Apex> {
    Some(Apex { alpha: val(c, "alpha_ap_deg").unwrap_or(0.0).to_radians(), h_ap: val(c, "h_ap")?, r_in: val(c, "r_in")? })
}

fn radius_axis(a: &Apex) -> (f64, f64) {
    (0.25 * a.r_in, 2.5 * a.r_in)
}

fn apex_bending(report: &CheckReport, c: &CaseRow) -> Option<Diagram2d> {
    let (a, t_lam) = (apex_of(c)?, val(c, "t_lam")?);
    let check = worst(report, &c.id, "en1995.t-31.apex_bending");
    let hr = |r: f64| a.h_ap / (r + 0.5 * a.h_ap);
    let kl = |r: f64| t::apex_kl(a.alpha, hr(r));
    let kr = |r: f64| t::apex_kr(r, t_lam);
    let u0 = check.map(|k| k.utilization)?;
    let u = |r: f64| u0 * kl(r) / kl(a.r_in) * kr(a.r_in) / kr(r);
    let range = radius_axis(&a);
    let (util, factor_l, factor_r) = (curves::sample(u, range.0, range.1, SAMPLES), curves::sample(kl, range.0, range.1, SAMPLES), curves::sample(kr, range.0, range.1, SAMPLES));
    let m = marker("marker-r-in", Vec2::new(a.r_in, u0), tone_of(check), format!("r_in = {:.0} mm, k_l = {:.3}, k_r = {:.3}", a.r_in, kl(a.r_in), kr(a.r_in)), Some(u0), check.map(|k| k.id.clone()), leaf(c, "r_in", "mm", 100.0));
    let series = vec![polyline("apex-utilisation", "σ_m,d / (k_r f_m,d)", "σ_m,d / (k_r f_m,d)", util.clone(), stroke(Tone::Warning, 2.0, Dash::Solid)), polyline("k-l", "k_l", "k_l", factor_l.clone(), stroke(Tone::Info, 1.5, Dash::Solid)), polyline("k-r", "k_r", "k_r", factor_r.clone(), stroke(Tone::Neutral, 1.5, Dash::Dotted)), limit("limit", "Utilisation 1", "Ausnutzung 1", range, 1.0)];
    let twin = vec![column("Inner radius r_in", "Innenradius r_in", "mm", xs(&util)), column("Apex utilisation", "Ausnutzung im Firstbereich", "1", ys(&util)), column("k_l", "k_l", "1", ys(&factor_l)), column("k_r", "k_r", "1", ys(&factor_r))];
    Some(diagram(c, "case-apex", ("Apex: bending over the inner radius", "Firstbereich: Biegung über dem Innenradius"), ("Apex bending utilisation σ_m,d / (k_r f_m,d) with k_l (6.43 – 6.47) and k_r (6.49) over the inner radius r_in at the pitch angle and apex depth of the member; the marker drags r_in.", "Ausnutzung der Biegung im Firstbereich σ_m,d / (k_r f_m,d) mit k_l (6.43 – 6.47) und k_r (6.49) über dem Innenradius r_in bei Neigung und Firsthöhe des Bauteils; der Marker ändert r_in."), vec![length_axis("Inner radius r_in", "Innenradius r_in", "mm", range), ratio_axis("Factor and utilisation", "Beiwert und Ausnutzung", None)], series, vec![m], twin))
}

fn apex_tension(report: &CheckReport, c: &CaseRow) -> Option<Diagram2d> {
    let a = apex_of(c)?;
    let check = worst(report, &c.id, "en1995.t-32.apex_tension90")?;
    let kp = |r: f64| t::apex_kp(a.alpha, a.h_ap / (r + 0.5 * a.h_ap));
    let range = radius_axis(&a);
    let curve = curves::sample(kp, range.0, range.1, SAMPLES);
    let m = marker("marker-r-in", Vec2::new(a.r_in, kp(a.r_in)), tone_of(Some(check)), format!("r_in = {:.0} mm, k_p = {:.3}", a.r_in, kp(a.r_in)), Some(check.utilization), Some(check.id.clone()), leaf(c, "r_in", "mm", 100.0));
    let twin = vec![column("Inner radius r_in", "Innenradius r_in", "mm", xs(&curve)), column("k_p", "k_p", "1", ys(&curve))];
    Some(diagram(c, "case-apex-tension", ("Apex: k_p over the inner radius", "Firstbereich: k_p über dem Innenradius"), ("The factor k_p of the tension perpendicular to the grain at the apex (EN 1995-1-1 6.48 – 6.51) over the inner radius r_in; σ_t,90,d grows with k_p, the marker drags r_in.", "Der Beiwert k_p der Querzugspannung im Firstbereich (EN 1995-1-1 6.48 – 6.51) über dem Innenradius r_in; σ_t,90,d wächst mit k_p, der Marker ändert r_in."), vec![length_axis("Inner radius r_in", "Innenradius r_in", "mm", range), ratio_axis("Factor k_p", "Beiwert k_p", None)], vec![polyline("k-p", "k_p", "k_p", curve.clone(), stroke(Tone::Info, 2.0, Dash::Solid))], vec![m], twin))
}

fn rods(report: &CheckReport, c: &CaseRow, prefix: &str, key: &str, title: (&str, &str), description: (&str, &str)) -> Option<Diagram2d> {
    let check = worst(report, &c.id, prefix)?;
    let n0 = val(c, "n_rods").filter(|n| *n >= 1.0)?;
    let top = (2.0 * n0).max(6.0);
    let curve = curves::sample(|n| check.utilization * n0 / n, 1.0, top, SAMPLES);
    let m = marker("marker-n-rods", Vec2::new(n0, check.utilization), tone_of(Some(check)), format!("n = {n0:.0}, F_t,90,d / ΣR_ax,d = {:.3}", check.utilization), Some(check.utilization), Some(check.id.clone()), leaf(c, "n_rods", "1", 1.0));
    let series = vec![polyline("reinforcement", "F_t,90,d / ΣR_ax,d", "F_t,90,d / ΣR_ax,d", curve.clone(), stroke(Tone::Warning, 2.0, Dash::Solid)), limit("limit", "Utilisation 1", "Ausnutzung 1", (1.0, top), 1.0)];
    let twin = vec![column("Number of rods", "Anzahl Stäbe", "1", xs(&curve)), column("Utilisation", "Ausnutzung", "1", ys(&curve))];
    Some(diagram(c, key, title, description, vec![axis("Number of rods n", "Anzahl Stäbe n", QuantityKind::Dimensionless, "1", Some((1.0, top)), TickRule::Auto), ratio_axis("Utilisation", "Ausnutzung", None)], series, vec![m], twin))
}

fn notch_depth(document: &En1995Snapshot, report: &CheckReport, c: &CaseRow) -> Option<Diagram2d> {
    let p = t::notch_probe(&t::case_input(document.annex, &DesignCase::from(c))).ok()?;
    let top = 0.7 * p.h;
    let depth0 = (p.h - p.hef).clamp(0.0, top);
    let kv = |d: f64| p.kv(1.0 - d / p.h);
    let curve = curves::sample(kv, 0.0, top, SAMPLES);
    let check = worst(report, &c.id, "en1995.t-45.notch_shear");
    let scale = check.map_or(0.0, |k0| k0.utilization * (1.0 - depth0 / p.h) * kv(depth0));
    let utilisation = curves::sample(|d| (scale / ((1.0 - d / p.h) * kv(d))).min(3.0), 0.0, top, SAMPLES);
    let m = marker("marker-depth", Vec2::new(depth0, kv(depth0)), tone_of(check), format!("a = {depth0:.0} mm, k_v = {:.3}", kv(depth0)), check.map(|k0| k0.utilization), check.map(|k0| k0.id.clone()), val(c, "h_ef").map(|_| binding(case_binding(c, "h_ef"), "mm", Affine { base: p.h, k: -1.0 }, 1.0)));
    let mut series = vec![polyline("k-v", "k_v", "k_v", curve.clone(), stroke(Tone::Info, 2.0, Dash::Solid)), limit("limit", "Limit 1 (k_v ≤ 1, utilisation 1)", "Grenze 1 (k_v ≤ 1, Ausnutzung 1)", (0.0, top), 1.0)];
    let mut twin = vec![column("Notch depth a", "Ausklinkungstiefe a", "mm", xs(&curve)), column("k_v", "k_v", "1", ys(&curve))];
    if check.is_some() {
        series.push(polyline("shear-utilisation", "τ_d / (k_v f_v,d)", "τ_d / (k_v f_v,d)", utilisation.clone(), stroke(Tone::Warning, 2.0, Dash::Solid)));
        twin.push(column("Shear utilisation", "Schubausnutzung", "1", ys(&utilisation)));
    }
    if document.annex == crate::norm::AnnexChoice::De {
        series.push(vertical("na-depth", &format!("NA-DE: h_ef ≥ {} h", t::NOTCH_NA_MIN_RATIO), &format!("NA-DE: h_ef ≥ {} h", format!("{}", t::NOTCH_NA_MIN_RATIO).replace('.', ",")), (1.0 - t::NOTCH_NA_MIN_RATIO) * p.h, 3.0));
    }
    Some(diagram(c, "case-notch-depth", ("Notch: k_v over the notch depth", "Ausklinkung: k_v über der Ausklinkungstiefe"), ("k_v of EN 1995-1-1 (6.62) and the shear utilisation at the notch over the notch depth a = h − h_ef with the limit 1 (k_v never exceeds 1); the marker drags the depth.", "k_v nach EN 1995-1-1 (6.62) und Schubausnutzung an der Ausklinkung über der Ausklinkungstiefe a = h − h_ef mit der Grenze 1 (k_v überschreitet 1 nie); der Marker ändert die Tiefe."), vec![length_axis("Notch depth a", "Ausklinkungstiefe a", "mm", (0.0, top)), ratio_axis("Factor and utilisation", "Beiwert und Ausnutzung", Some((0.0, 3.0)))], series, vec![m], twin))
}

fn notch_reinforcement(report: &CheckReport, c: &CaseRow) -> Option<Diagram2d> {
    let check = worst(report, &c.id, "en1995.t-46.")?;
    let (h, hef) = (val(c, "h")?, val(c, "h_ef")?);
    let ft0 = t::notch_ft90(1.0, hef / h);
    if ft0 <= 1e-12 {
        return None;
    }
    let top = 0.7 * h;
    let depth0 = (h - hef).clamp(0.0, top);
    let curve = curves::sample(|d| check.utilization * t::notch_ft90(1.0, 1.0 - d / h) / ft0, 0.0, top, SAMPLES);
    let m = marker("marker-depth", Vec2::new(depth0, check.utilization), tone_of(Some(check)), format!("a = {depth0:.0} mm, F_t,90,d / ΣR_ax,d = {:.3}", check.utilization), Some(check.utilization), Some(check.id.clone()), Some(binding(case_binding(c, "h_ef"), "mm", Affine { base: h, k: -1.0 }, 1.0)));
    let series = vec![polyline("reinforcement", "F_t,90,d / ΣR_ax,d", "F_t,90,d / ΣR_ax,d", curve.clone(), stroke(Tone::Warning, 2.0, Dash::Solid)), limit("limit", "Utilisation 1", "Ausnutzung 1", (0.0, top), 1.0)];
    let twin = vec![column("Notch depth a", "Ausklinkungstiefe a", "mm", xs(&curve)), column("Utilisation", "Ausnutzung", "1", ys(&curve))];
    Some(diagram(c, "case-notch-reinforcement", ("Reinforced notch over the notch depth", "Verstärkte Ausklinkung über der Ausklinkungstiefe"), ("Tension force F_t,90,d = 1.3 V_d [3(1 − α)² − 2(1 − α)³] against the withdrawal resistance of the rods (NCI to EN 1995-1-1 6.5.2) over the notch depth a; the marker drags the depth.", "Zugkraft F_t,90,d = 1,3 V_d [3(1 − α)² − 2(1 − α)³] gegen den Ausziehwiderstand der Stäbe (NCI zu EN 1995-1-1 6.5.2) über der Ausklinkungstiefe a; der Marker ändert die Tiefe."), vec![length_axis("Notch depth a", "Ausklinkungstiefe a", "mm", (0.0, top)), ratio_axis("Utilisation", "Ausnutzung", None)], series, vec![m], twin))
}

fn hole_net(report: &CheckReport, c: &CaseRow) -> Option<Diagram2d> {
    let check = worst(report, &c.id, "en1995.t-18.net_section")?;
    let (b, h, d0) = (val(c, "b")?, val(c, "h")?, val(c, "hole_d")?);
    let count = val(c, "hole_count").unwrap_or(1.0);
    let net = |d: f64| b * (h - d * count);
    let top = 0.3 * h;
    let u = |d: f64| (check.utilization * net(d0) / net(d)).min(3.0);
    let curve = curves::sample(u, 0.0, top, SAMPLES);
    let m = marker("marker-hole-d", Vec2::new(d0.min(top), u(d0.min(top))), tone_of(Some(check)), format!("d = {d0:.0} mm, N_t,d / (A_net f_t,0,d) = {:.3}", check.utilization), Some(check.utilization), Some(check.id.clone()), leaf(c, "hole_d", "mm", 1.0));
    let series = vec![polyline("net-section", "N_t,d / (A_net f_t,0,d)", "N_t,d / (A_net f_t,0,d)", curve.clone(), stroke(Tone::Warning, 2.0, Dash::Solid)), limit("limit", "Utilisation 1", "Ausnutzung 1", (0.0, top), 1.0), vertical("hole-limit", &format!("d ≤ {} h (NCI)", t::HOLE_LIMIT_RATIO), &format!("d ≤ {} h (NCI)", format!("{}", t::HOLE_LIMIT_RATIO).replace('.', ",")), t::HOLE_LIMIT_RATIO * h, 3.0)];
    let twin = vec![column("Hole diameter d", "Lochdurchmesser d", "mm", xs(&curve)), column("Net-section utilisation", "Ausnutzung des Nettoquerschnitts", "1", ys(&curve))];
    Some(diagram(c, "case-hole-net", ("Hole: net-section utilisation over the diameter", "Loch: Ausnutzung des Nettoquerschnitts über dem Durchmesser"), ("Tension utilisation of the net section A_net = b (h − n d) (EN 1995-1-1 5.2) over the hole diameter d with the hole limit of the NCI; the marker drags the diameter.", "Zugausnutzung des Nettoquerschnitts A_net = b (h − n d) (EN 1995-1-1 5.2) über dem Lochdurchmesser d mit der Lochgrenze des NCI; der Marker ändert den Durchmesser."), vec![length_axis("Hole diameter d", "Lochdurchmesser d", "mm", (0.0, top)), ratio_axis("Utilisation", "Ausnutzung", Some((0.0, 3.0)))], series, vec![m], twin))
}

/// 📊️ The utilisation of every check of a member as bars (bending, shear, bearing, tension, buckling, notch, fire, bridge, material), one bar per check with the limit 1, with a marker bound to the check it shows.
pub(super) fn member_results(report: &CheckReport, m: &TimberMember) -> Option<Diagram2d> {
    let rows: Vec<(&str, f64, String, String)> = report.checks.iter().filter(|k| k.subject.entity_id == m.id && k.utilization.is_finite() && k.utilization > 0.0).map(|k| (k.id.as_str(), k.utilization, k.title.en.clone(), k.title.de.clone())).collect();
    if rows.is_empty() {
        return None;
    }
    let n = rows.len() as f64;
    let top = rows.iter().map(|r| r.1).fold(1.2_f64, f64::max);
    let mut series: Vec<Series> = rows.iter().enumerate().map(|(i, (_, ratio, en, de))| bar(&format!("result-{i}"), en, de, i as f64 + 0.6, i as f64 + 1.4, *ratio, utilisation_tone(*ratio))).collect();
    series.push(limit("limit", "Utilisation 1", "Ausnutzung 1", (0.4, n + 0.6), 1.0));
    let markers = rows.iter().enumerate().map(|(i, (id, ratio, en, de))| Marker { id: format!("marker-result-{i}"), at: Vec2::new(i as f64 + 1.0, *ratio), shape: MarkerShape::Diamond, tone: utilisation_tone(*ratio), label: loc(&format!("{en}: {:.0} %", ratio * 100.0), &format!("{de}: {:.0} %", ratio * 100.0)), utilisation: Some(*ratio), bound_check: Some((*id).to_string()), binding: None }).collect();
    Some(Diagram2d {
        id: format!("member-results:{}", m.id),
        title: loc(&format!("Utilisation of every check – {}", m.label_en), &format!("Ausnutzung jedes Nachweises – {}", m.label_de)),
        description: loc("Demand over resistance of every check of the member (material, bending, shear, bearing, tension, buckling, notch, fire, bridge); a bar above 1 fails.", "Beanspruchung zu Beanspruchbarkeit jedes Nachweises des Bauteils (Material, Biegung, Schub, Querdruck, Zug, Knicken, Ausklinkung, Brand, Brücke); ein Balken über 1 versagt."),
        axes: vec![axis("Check", "Nachweis", QuantityKind::Dimensionless, "1", Some((0.4, n + 0.6)), TickRule::Auto), ratio_axis("Utilisation", "Ausnutzung", Some((0.0, top)))],
        series,
        markers,
        annotations: Vec::new(),
        focus: focus_member(m),
        legend: false,
        table_twin: DataTable { columns: vec![column("Check", "Nachweis", "1", (1..=rows.len()).map(|i| i as f64).collect()), column("Utilisation", "Ausnutzung", "1", rows.iter().map(|r| r.1).collect())] },
    })
}

/// 🌉️ The Miner damage of a bridge member over its design life: D = n_obs t_L / N_R is linear in t_L, so the curve is the check value scaled by t / t_L, and the marker drags the design life.
pub(super) fn bridge_fatigue(report: &CheckReport, m: &TimberMember) -> Option<Diagram2d> {
    let check = worst(report, &m.id, "en1995.2.a.fatigue.")?;
    let life = m.bridge_t_l_years;
    if life <= 0.0 || m.bridge_n_obs * life <= 1.0 {
        return None;
    }
    let u0 = check.utilization;
    let hi = (1.2 * life / u0.clamp(0.3, 1.0)).max(1.5 * life);
    let curve = curves::sample(|years| u0 * years / life, 0.0, hi, SAMPLES);
    let marker = Marker { id: "marker-life".into(), at: Vec2::new(life, u0), shape: MarkerShape::Diamond, tone: tone_of(Some(check)), label: loc(&format!("t_L = {life:.0} a, D = {u0:.3}"), &format!("t_L = {life:.0} a, D = {u0:.3}")), utilisation: Some(u0), bound_check: Some(check.id.clone()), binding: Some(binding(format!("members[id={}].bridgeTLYears", m.id), "a", Affine::IDENTITY, 1.0)) };
    let series = vec![polyline("miner", "Miner sum D", "Miner-Summe D", curve.clone(), stroke(Tone::Warning, 2.0, Dash::Solid)), limit("limit", "D = 1", "D = 1", (0.0, hi), 1.0)];
    Some(Diagram2d {
        id: format!("fatigue:{}", m.id),
        title: loc(&format!("Bridge fatigue over the design life – {}", m.label_en), &format!("Brückenermüdung über der Nutzungsdauer – {}", m.label_de)),
        description: loc("Miner sum D = n_obs t_L / N_R of the fatigue check (EN 1995-2 6.2) over the design life t_L at the stress range of the crowd load model; D reaches 1 at the fatigue life, the marker drags t_L.", "Miner-Summe D = n_obs t_L / N_R des Ermüdungsnachweises (EN 1995-2 6.2) über der Nutzungsdauer t_L bei der Spannungsschwingbreite des Personenlastmodells; D erreicht 1 bei der Ermüdungslebensdauer, der Marker ändert t_L."),
        axes: vec![axis("Design life t_L", "Nutzungsdauer t_L", QuantityKind::Time, "a", Some((0.0, hi)), TickRule::Auto), ratio_axis("Miner sum D", "Miner-Summe D", None)],
        series,
        markers: vec![marker],
        annotations: Vec::new(),
        focus: focus_member(m),
        legend: true,
        table_twin: DataTable { columns: vec![column("Design life t_L", "Nutzungsdauer t_L", "a", xs(&curve)), column("Miner sum D", "Miner-Summe D", "1", ys(&curve))] },
    })
}

/// 🧩️ The entity curves of one case row, chosen by its catalogue entry; the row of a typed taper or cutout carries the entry of its first check, the reinforcement curves follow the checks of the report.
pub(super) fn case_diagrams(document: &En1995Snapshot, report: &CheckReport, c: &CaseRow) -> Vec<Diagram2d> {
    let apex_rods = || rods(report, c, "en1995.t-33.", "case-apex-reinforcement", ("Apex reinforcement over the rod count", "Firstverstärkung über der Stabanzahl"), ("Reinforcement of the apex zone with screws or glued-in rods (NCI to EN 1995-1-1 6.4.3), the total tension force against the sum of the withdrawal resistances over the number of rods; the marker drags the rod count.", "Verstärkung des Firstbereichs mit Schrauben oder eingeklebten Stäben (NCI zu EN 1995-1-1 6.4.3), die gesamte Zugkraft gegen die Summe der Ausziehwiderstände über der Stabanzahl; der Marker ändert die Stabanzahl."));
    let notch_rods = || rods(report, c, "en1995.t-46.", "case-notch-rods", ("Notch reinforcement over the rod count", "Ausklinkungsverstärkung über der Stabanzahl"), ("Reinforcement of the notch with screws or rods (NCI to EN 1995-1-1 6.5.2), the tension force against the sum of the withdrawal resistances over the number of rods; the marker drags the rod count.", "Verstärkung der Ausklinkung mit Schrauben oder Stäben (NCI zu EN 1995-1-1 6.5.2), die Zugkraft gegen die Summe der Ausziehwiderstände über der Stabanzahl; der Marker ändert die Stabanzahl."));
    match c.entry_id.as_str() {
        "T-30" => taper_length(document, report, c).into_iter().collect(),
        "T-31" => [apex_bending(report, c), apex_tension(report, c), apex_rods()].into_iter().flatten().collect(),
        "T-32" => apex_tension(report, c).into_iter().collect(),
        "T-33" => apex_rods().into_iter().collect(),
        "T-45" => [notch_depth(document, report, c), notch_reinforcement(report, c), notch_rods()].into_iter().flatten().collect(),
        "T-46" => [notch_reinforcement(report, c), notch_rods()].into_iter().flatten().collect(),
        "T-18" => hole_net(report, c).into_iter().collect(),
        _ => Vec::new(),
    }
}
