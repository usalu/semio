//! 🌉️ Timber bridges of EN 1995-2 as catalogue entries T-66, T-120 … T-124: footbridge vibration, fatigue, stress-laminated decks, connections and the combined bridge assessment. Units: N, mm, N/mm², SI for dynamics. See [`super`].

use super::*;
use std::f64::consts::PI;

/// 🌊 First vertical bending frequency f₁ = π/(2ℓ²)·√(EI/m) of a simply supported span (ℓ in m, EI in N·m², m in kg/m).
pub fn bridge_f1(span: f64, ei: f64, mass: f64) -> f64 {
    PI / (2.0 * span * span) * (ei / mass).sqrt()
}

/// 🚶 Acceleration of one pedestrian a₁ = 200/(M·ζ) (1.0–2.5 Hz), 100/(M·ζ) (2.5–5 Hz) vertical and 50/(M·ζ) horizontal (EN 1995-2 7.3).
pub fn single_pedestrian_acceleration(f1: f64, modal_mass: f64, zeta: f64, vertical: bool) -> f64 {
    let k = if vertical { if f1 <= 2.5 { 200.0 } else { 100.0 } } else { 50.0 };
    k / (modal_mass * zeta)
}

/// 🚶 Harmonic-crowd frequency weighting ψ of the first vertical harmonic: 0 below 0.5 Hz, linear to 1 at 1 Hz, 1 up to 1.7 Hz, linear to 0 at 2.1 Hz; the second harmonic is not modelled (Sétra/HIVOSS style model, to be confirmed against EN 1995-2 7.3 and the national annex).
pub fn crowd_psi(f1: f64) -> f64 {
    let (start, from, to, end) = (ndp_common("ndp.en1995.bridge.crowd-psi-start"), ndp_common("ndp.en1995.bridge.crowd-psi-full-from"), ndp_common("ndp.en1995.bridge.crowd-psi-full-to"), ndp_common("ndp.en1995.bridge.crowd-psi-end"));
    if f1 < start || f1 > end { 0.0 } else if f1 < from { (f1 - start) / (from - start) } else if f1 <= to { 1.0 } else { (end - f1) / (end - to) }
}

/// 🚶 Resonant vertical acceleration in m/s² of a simply supported footbridge under a harmonic crowd of `density` persons/m² on a deck `width` wide: p = d·280 N·10.8·√(ξ/n)·ψ per m² with n = d·ℓ·b persons, a = 2·p·b/(π·ξ·m).
pub fn crowd_acceleration(span: f64, width: f64, density: f64, f1: f64, mass: f64, zeta: f64) -> f64 {
    let n = density * span * width;
    if n <= 0.0 {
        return 0.0;
    }
    let p = density * ndp_common("ndp.en1995.bridge.crowd-force") * ndp_common("ndp.en1995.bridge.crowd-factor") * (zeta / n).sqrt() * crowd_psi(f1);
    2.0 * p * width / (PI * zeta * mass)
}

fn t120(c: &Case) -> R {
    let (l, ei, m, z) = (c.n("span_m")?, c.n("ei_nm2")?, c.n("mass_kg_m")?, c.o("zeta", 0.015));
    let f1 = bridge_f1(l, ei, m);
    let big_m = m * l / 2.0;
    let npeds = c.opt("n_peds").unwrap_or_else(|| c.o("crowd_density", 0.0) * l * c.o("deck_width_m", 0.0));
    let a1v = single_pedestrian_acceleration(f1, big_m, z, true);
    let a1h = single_pedestrian_acceleration(f1, big_m, z, false);
    let (av, ah) = if npeds > 1.0 { (0.23 * a1v * npeds, 0.18 * a1h * npeds) } else { (a1v, a1h) };
    let lim_h = if c.so("comfort", "normal") == "crowd" { 0.4 } else { 0.2 };
    let skip_v = f1 > 5.0;
    let skip_h = c.o("f_lat_hz", 3.0) > 2.5;
    let mut out = vec![
        Outcome::info("f1", "First vertical frequency f₁", "Erste vertikale Eigenfrequenz f₁", QuantityKind::Dimensionless, f1, format!("f₁ = {} Hz", f(f1))),
        Outcome::accel("a_vert", "Vertical acceleration (7.3)", "Vertikale Beschleunigung (7.3)", if skip_v { 0.0 } else { av }, 0.7, format!("a_vert = {} m/s²{}", f(av), if skip_v { " (f₁ > 5 Hz, no check)" } else { "" })),
        Outcome::accel("a_hor", "Horizontal acceleration (7.3)", "Horizontale Beschleunigung (7.3)", if skip_h { 0.0 } else { ah }, lim_h, format!("a_hor = {} m/s²", f(ah))),
    ];
    if c.so("crowd_model", "en1995-2") == "setra" {
        let (d, b) = (c.n("crowd_density")?, c.n("deck_width_m")?);
        let a = crowd_acceleration(l, b, d, f1, m, z);
        out.push(Outcome::accel("a_crowd", "Vertical acceleration under crowd loading", "Vertikale Beschleunigung unter Menschenmenge", a, c.o("a_limit", 0.7), format!("n = {} persons, ψ = {}, a = 2 p b/(π ξ m) = {} m/s²", f(d * l * b), f(crowd_psi(f1)), f(a))));
    }
    Ok(out)
}

/// 📉 Miner damage n/N of `cycles` at the stress range `ds` in N/mm² on the S-N curve N = 10^(a − b·log₁₀ Δσ).
pub fn miner_damage(sn_a: f64, sn_b: f64, ds: f64, cycles: f64) -> f64 {
    cycles / 10f64.powf(sn_a - sn_b * ds.max(1e-9).log10())
}

fn t121(c: &Case) -> R {
    let (a, b, years, cut) = (c.o("sn_a", 15.0), c.o("sn_b", 4.0), c.o("years", 1.0), c.o("ds_cut", 0.0));
    let mut blocks = Vec::new();
    for k in 1..=6 {
        if let (Some(ds), Some(n)) = (c.opt(&format!("ds{k}")), c.opt(&format!("n{k}"))) {
            blocks.push((k, if ds < cut { 0.0 } else { miner_damage(a, b, ds, n * years) }));
        }
    }
    if blocks.is_empty() {
        return Err("missing input ds1 and n1".into());
    }
    let damage: f64 = blocks.iter().map(|b| b.1).sum();
    let mut out = vec![Outcome::ratio("miner", "Fatigue damage sum (Miner)", "Schädigungssumme (Miner)", damage, 1.0, format!("D = Σ n_i/N_i = {} over {} blocks and {} years, N = 10^({} − {}·log Δσ)", f(damage), blocks.len(), f(years), f(a), f(b)))];
    out.extend(blocks.iter().map(|(k, d)| Outcome::info(&format!("damage_{k}"), &format!("Damage of block {k}"), &format!("Schädigung Block {k}"), QuantityKind::Dimensionless, *d, format!("D_{k} = {}", f(*d)))));
    if c.opt("years").is_some() && damage > 0.0 {
        out.push(Outcome::info("life_years", "Fatigue life until D = 1", "Ermüdungslebensdauer bis D = 1", QuantityKind::Time, years / damage * 31_557_600.0, format!("{} years", f(years / damage))));
    }
    Ok(out)
}

fn t122(c: &Case) -> R {
    let staged = ["loss_relax", "loss_creep", "loss_mc"].iter().any(|k| c.opt(k).is_some());
    let loss = if staged { 1.0 - ["loss_relax", "loss_creep", "loss_mc"].iter().map(|k| 1.0 - c.o(k, 0.0)).product::<f64>() } else { c.o("loss", ndp_common("ndp.en1995.deck.default-loss")) };
    let sp = c.n("sigma_p_0")? * (1.0 - loss);
    let mut out = vec![Outcome::stress("prestress_min", "Minimum prestress after losses", "Mindestvorspannung nach Verlusten", ndp_common("ndp.en1995.deck.min-prestress"), sp, format!("σ_p,min = {} N/mm² after a loss of {}", f(sp), f(loss)))];
    if let Some(tau) = c.opt("tau_h") {
        out.push(Outcome::stress("friction", "Friction against slip", "Reibung gegen Gleiten", tau, c.o("mu", ndp_common("ndp.en1995.deck.friction-coefficient")) * sp, format!("μ σ_p = {}", f(c.o("mu", ndp_common("ndp.en1995.deck.friction-coefficient")) * sp))));
    }
    if let (Some(q), Some(a), Some(h)) = (c.opt("wheel_n"), c.opt("contact_mm"), c.opt("h")) {
        let bef = a + 2.0 * h;
        out.push(Outcome::stress("wheel_spread", "Wheel load spread (b_ef unverified)", "Radlastverteilung (b_ef ungeprüft)", q / (bef * a), c.o("f_c_90_d", 1.0), format!("b_ef = a + 2h = {} mm; unverified / ungeprüft: the Annex A.2 formula is not in the specs (T-122 reads 'bef = …'), the plate action is not modelled", f(bef))));
    }
    if let (Some(spacing), Some(depth)) = (c.opt("bar_spacing"), c.opt("deck_depth")) {
        let force = c.n("sigma_p_0")? * spacing * depth;
        if let (Some(area), Some(fbar)) = (c.opt("bar_area"), c.opt("f_bar_d")) {
            out.push(Outcome::force("bar_force", "Bar force against bar resistance", "Stabkraft gegen Stabwiderstand", force, area * fbar, format!("P = σ_p,0 s h = {} N", f(force))));
        }
        if let Some(plate) = c.opt("plate_a") {
            out.push(Outcome::stress("plate_bearing", "Anchor plate bearing on the laminations", "Pressung der Ankerplatte auf den Lamellen", force / plate, c.o("k_c90", ndp_common("ndp.en1995.deck.k-c90")) * c.o("f_c_90_d", 1.0), format!("σ_c,90 = P/A = {} N/mm²", f(force / plate))));
        }
    }
    Ok(out)
}

fn t123(c: &Case) -> R {
    let rd = c.n("kmod")? * c.n("f_v_rk")? * c.o("n", 1.0) / gamma_m_for(c.annex, "connection", false).unwrap_or(f64::NAN);
    Ok(vec![Outcome::force("bridge_connection", "Bridge connection resistance", "Verbindungstragfähigkeit Brücke", c.n("f_ed")?, rd, format!("F_Rd = {} N", f(rd)))])
}

fn t124(c: &Case) -> R {
    let mut out = Vec::new();
    for e in [t120 as fn(&Case) -> R, t121, t122, t123] {
        if let Ok(o) = e(c) {
            out.extend(o);
        }
    }
    if out.is_empty() { Err("missing input for any bridge check".into()) } else { Ok(out) }
}

/// 🗂 Catalogue rows of the bridge group.
pub const ENTRIES: &[Entry] = &[
    Entry { id: "T-66", part: "EN 1995-2", clause: "7.4", en: "Walking excitation of footbridges", de: "Fußgängeranregung von Fußgängerbrücken", inputs: &["span_m", "ei_nm2", "mass_kg_m", "zeta", "n_peds"], eval: t120 },
    Entry { id: "T-120", part: "EN 1995-2", clause: "7.2", en: "Pedestrian dynamics", de: "Fußgängerdynamik", inputs: &["span_m", "ei_nm2", "mass_kg_m", "zeta", "n_peds", "comfort", "deck_width_m", "crowd_density", "crowd_model", "a_limit", "f_lat_hz"], eval: t120 },
    Entry { id: "T-121", part: "EN 1995-2", clause: "6.2", en: "Fatigue", de: "Ermüdung", inputs: &["sn_a", "sn_b", "ds1", "n1", "ds2", "n2", "ds3", "n3", "years", "ds_cut"], eval: t121 },
    Entry { id: "T-122", part: "EN 1995-2", clause: "Annex A", en: "Stress-laminated decks", de: "Vorgespannte Brettstapeldecks", inputs: &["sigma_p_0", "loss", "loss_relax", "loss_creep", "loss_mc", "tau_h", "mu", "bar_spacing", "deck_depth", "bar_area", "f_bar_d", "plate_a", "span_m", "deck_width_m"], eval: t122 },
    Entry { id: "T-123", part: "EN 1995-2", clause: "6.1", en: "Connections in bridges", de: "Verbindungen in Brücken", inputs: &["kmod", "f_v_rk", "n", "f_ed"], eval: t123 },
    Entry { id: "T-124", part: "EN 1995-2", clause: "7", en: "Combined bridge assessment", de: "Gesamtnachweis Brücke", inputs: &["span_m", "ei_nm2", "mass_kg_m", "ds1", "n1"], eval: t124 },
];
