//! 🔥️ Fire design of EN 1992-1-2 as catalogue entries R-01 … R-08: tabulated data, ISO 834 temperature field by finite differences, 500 °C isotherm and zone method. See [`super`].

use super::*;

/// 📚 One tabulated row: R class in minutes, minimum dimension and axis distance in mm.
pub type FireRow = (f64, f64, f64);

const BEAM_SS: [[FireRow; 6]; 4] = [
    [(30.0, 80.0, 25.0), (60.0, 120.0, 40.0), (90.0, 150.0, 55.0), (120.0, 200.0, 65.0), (180.0, 240.0, 80.0), (240.0, 280.0, 90.0)],
    [(30.0, 120.0, 20.0), (60.0, 160.0, 35.0), (90.0, 200.0, 45.0), (120.0, 240.0, 60.0), (180.0, 300.0, 70.0), (240.0, 350.0, 80.0)],
    [(30.0, 160.0, 15.0), (60.0, 200.0, 30.0), (90.0, 300.0, 40.0), (120.0, 300.0, 55.0), (180.0, 400.0, 65.0), (240.0, 500.0, 75.0)],
    [(30.0, 200.0, 15.0), (60.0, 300.0, 25.0), (90.0, 400.0, 35.0), (120.0, 500.0, 50.0), (180.0, 600.0, 60.0), (240.0, 700.0, 70.0)],
];
const BEAM_CONT: [[FireRow; 6]; 2] = [
    [(30.0, 80.0, 15.0), (60.0, 120.0, 25.0), (90.0, 150.0, 35.0), (120.0, 200.0, 45.0), (180.0, 240.0, 60.0), (240.0, 280.0, 75.0)],
    [(30.0, 160.0, 12.0), (60.0, 200.0, 12.0), (90.0, 250.0, 25.0), (120.0, 300.0, 35.0), (180.0, 400.0, 50.0), (240.0, 500.0, 60.0)],
];
const SLAB_ONE_WAY: [FireRow; 6] = [(30.0, 60.0, 10.0), (60.0, 80.0, 20.0), (90.0, 100.0, 30.0), (120.0, 120.0, 40.0), (180.0, 150.0, 55.0), (240.0, 175.0, 65.0)];
const SLAB_TWO_WAY: [FireRow; 6] = [(30.0, 60.0, 10.0), (60.0, 80.0, 10.0), (90.0, 100.0, 15.0), (120.0, 120.0, 20.0), (180.0, 150.0, 30.0), (240.0, 175.0, 40.0)];
const SLAB_FLAT: [FireRow; 6] = [(30.0, 150.0, 10.0), (60.0, 180.0, 15.0), (90.0, 200.0, 25.0), (120.0, 200.0, 35.0), (180.0, 200.0, 45.0), (240.0, 200.0, 50.0)];
const COLUMN_A: [[FireRow; 6]; 3] = [
    [(30.0, 200.0, 25.0), (60.0, 200.0, 25.0), (90.0, 200.0, 31.0), (120.0, 250.0, 40.0), (180.0, 350.0, 45.0), (240.0, 350.0, 61.0)],
    [(30.0, 200.0, 25.0), (60.0, 200.0, 36.0), (90.0, 300.0, 45.0), (120.0, 350.0, 45.0), (180.0, 350.0, 63.0), (240.0, 450.0, 75.0)],
    [(30.0, 200.0, 32.0), (60.0, 250.0, 46.0), (90.0, 350.0, 53.0), (120.0, 350.0, 57.0), (180.0, 450.0, 70.0), (240.0, 450.0, 80.0)],
];
const WALL: [FireRow; 6] = [(30.0, 120.0, 10.0), (60.0, 130.0, 10.0), (90.0, 140.0, 25.0), (120.0, 160.0, 35.0), (180.0, 210.0, 50.0), (240.0, 270.0, 60.0)];
const TENSION: [FireRow; 6] = [(30.0, 80.0, 25.0), (60.0, 120.0, 40.0), (90.0, 150.0, 55.0), (120.0, 200.0, 65.0), (180.0, 240.0, 80.0), (240.0, 280.0, 90.0)];

/// 📈 Interpolate (dimension, axis distance) of a row set at the fire resistance `r` in minutes.
pub fn interp_rows(rows: &[FireRow], r: f64) -> (f64, f64) {
    let dims: Vec<(f64, f64)> = rows.iter().map(|x| (x.0, x.1)).collect();
    let axes: Vec<(f64, f64)> = rows.iter().map(|x| (x.0, x.2)).collect();
    (interp(&dims, r), interp(&axes, r))
}

pub fn pick(kind: &str, sub: &str, width_or_mu: f64) -> Result<Vec<FireRow>, String> {
    Ok(match kind {
        "beam" | "beam_ss" => {
            let mut best: Option<Vec<FireRow>> = None;
            for col in BEAM_SS.iter() {
                let ok = col.iter().all(|r| r.1 <= width_or_mu || r.0 > 240.0) || width_or_mu >= col[0].1;
                if ok { best = Some(col.to_vec()); }
            }
            best.unwrap_or_else(|| BEAM_SS[0].to_vec())
        }
        "beam_cont" => BEAM_CONT[0].to_vec(),
        "slab" => if sub == "two_way" { SLAB_TWO_WAY.to_vec() } else { SLAB_ONE_WAY.to_vec() },
        "flat_slab" => SLAB_FLAT.to_vec(),
        "column" => {
            let m = width_or_mu;
            let j = if m <= 0.2 { 0 } else if m <= 0.5 { 1 } else { 2 };
            COLUMN_A[j].to_vec()
        }
        "wall" => WALL.to_vec(),
        "tension" => TENSION.to_vec(),
        _ => return Err("unknown member kind".into()),
    })
}

/// 📏 Smallest axis distance (mm) of Table 5.5 over the columns whose minimum width does not exceed `width_mm`, at `r` minutes.
pub fn beam_axis_for_width(width_mm: f64, r: f64) -> f64 {
    let best = BEAM_SS.iter().map(|col| interp_rows(col, r)).filter(|(d, _)| *d <= width_mm).map(|(_, a)| a).fold(f64::MAX, f64::min);
    if best == f64::MAX { interp_rows(&BEAM_SS[0], r).1 } else { best }
}

/// 🏅 Largest standard resistance R (min) reached by dimension and axis distance for the member kind.
pub fn r_achieved(kind: &str, sub: &str, key: f64, dim: f64, a: f64) -> Result<f64, String> {
    if kind == "beam" || kind == "beam_ss" {
        let mut best = 0.0f64;
        for col in BEAM_SS.iter() {
            for r in col.iter() { if dim >= r.1 && a >= r.2 { best = best.max(r.0); } }
        }
        return Ok(best);
    }
    let rows = pick(kind, sub, key)?;
    Ok(rows.iter().filter(|r| dim >= r.1 && a >= r.2).map(|r| r.0).fold(0.0, f64::max))
}

pub fn r01(c: &Case) -> R {
    let kind = c.s("kind")?;
    let r = c.pos("r_class")?;
    let sub = c.so("sub", "one_way");
    let key = c.o("mu_fi", c.o("width", 0.0));
    let dim = c.pos("dim")?;
    let a = c.pos("a")?;
    let (mut dmin, mut amin) = if kind == "beam" || kind == "beam_ss" {
        let mut best = (f64::MAX, f64::MAX);
        for col in BEAM_SS.iter() {
            let (d, ax) = interp_rows(col, r);
            if dim >= d && ax < best.1 { best = (d, ax); }
        }
        if best.1 == f64::MAX { let (d, ax) = interp_rows(&BEAM_SS[3], r); best = (d, ax); }
        best
    } else {
        interp_rows(&pick(kind, sub, key)?, r)
    };
    if c.o("fck", 30.0) >= 70.0 { amin += c.ndp("ndp.en1992.spalling-add-mm"); dmin = dmin.max(0.0); }
    let achieved = r_achieved(kind, sub, key, dim, a)?;
    Ok(vec![
        Outcome::length_min("dim", "Minimum dimension", "Mindestabmessung", dim, dmin, format!("b_min or h_s = {} mm", f(dmin))),
        Outcome::length_min("axis", "Axis distance", "Achsabstand", a, amin, format!("a_min = {} mm", f(amin))),
        Outcome::num("r_achieved", "Achieved fire resistance R (min)", "Erreichte Feuerwiderstandsdauer R (min)", achieved, "largest tabulated class satisfied".into()).steps_de("größte tabellierte Klasse erfüllt"),
    ])
}

/// 🌡 ISO 834 gas temperature in °C at t minutes.
pub fn iso834(t_min: f64) -> f64 {
    20.0 + 345.0 * (8.0 * t_min + 1.0).log10()
}

/// 🔥 Exposed faces `(left, right, bottom, top)` of a case spelled with the letters `l`, `r`, `b`, `t`.
fn faces_of(spec: &str) -> fire_field::Faces {
    (spec.contains('l'), spec.contains('r'), spec.contains('b'), spec.contains('t'))
}

pub fn r02(c: &Case) -> R {
    let (b, h, t) = (c.pos("b")?, c.pos("h")?, c.pos("t_min")?);
    let field = fire_field::solve(b / 1000.0, h / 1000.0, faces_of(c.so("faces", "blr")), t, c.o("moisture", 1.5));
    let at = |y: f64, z: f64| field.at((y - b / 2.0) / 1000.0, (z - h / 2.0) / 1000.0);
    let mut v = vec![Outcome::num("gas", "Gas temperature θ_g (°C)", "Gastemperatur θ_g (°C)", iso834(t), "20 + 345 log10(8t + 1)".into())];
    let (py, pz) = (c.l("point_y").map(|v| v.to_vec()).unwrap_or_default(), c.l("point_z").map(|v| v.to_vec()).unwrap_or_default());
    for (i, (y, z)) in py.iter().zip(pz.iter()).enumerate() {
        v.push(Outcome::num(&format!("theta_{i}"), &format!("Temperature at point {i} (°C)"), &format!("Temperatur an Punkt {i} (°C)"), at(*y, *z), format!("y = {}, z = {}", f(*y), f(*z))));
    }
    v.push(Outcome::mm("a500_bottom", "500 °C isotherm depth from the bottom face", "Tiefe der 500-°C-Isotherme von der Unterseite", field.bottom_depth(500.0) * 1000.0, "mid-width section".into()).steps_de("Querschnitt in Breitenmitte"));
    v.push(Outcome::num("theta_max", "Maximum section temperature (°C)", "Höchste Querschnittstemperatur (°C)", field.peak(), "".into()));
    Ok(v)
}

/// 🧱 k_c(θ) of siliceous concrete (Table 3.1).
pub fn kc_theta(theta: f64) -> f64 {
    interp(&[(20.0, 1.0), (100.0, 1.0), (200.0, 0.95), (300.0, 0.85), (400.0, 0.75), (500.0, 0.60), (600.0, 0.45), (700.0, 0.30), (800.0, 0.15), (900.0, 0.08), (1000.0, 0.04), (1100.0, 0.01), (1200.0, 0.0)], theta)
}

/// 🔩 k_s(θ) of hot-rolled reinforcing steel (Table 3.2a).
pub fn ks_theta(theta: f64) -> f64 {
    interp(&[(20.0, 1.0), (400.0, 1.0), (500.0, 0.78), (600.0, 0.47), (700.0, 0.23), (800.0, 0.11), (900.0, 0.06), (1000.0, 0.04), (1100.0, 0.02), (1200.0, 0.0)], theta)
}

pub fn r07(c: &Case) -> R {
    let th = c.n("theta")?;
    Ok(vec![Outcome::num("kc", "k_c(θ)", "k_c(θ)", kc_theta(th), "Table 3.1".into()), Outcome::num("ks", "k_s(θ)", "k_s(θ)", ks_theta(th), "Table 3.2a".into()), Outcome::num("eps_th", "Thermal strain of concrete (permille)", "Thermische Dehnung Beton (Promille)", (if th <= 700.0 { -1.8e-4 + 9e-6 * th + 2.3e-11 * th.powi(3) } else { 14.0e-3 }) * 1000.0, "3.3.1".into())])
}

pub fn r03(c: &Case) -> R {
    let (b, h, t) = (c.pos("b")?, c.pos("h")?, c.pos("t_min")?);
    let (fck, fyk, as_) = (c.pos("fck")?, c.o("fyk", 500.0), c.pos("as")?);
    let (ybar, zbar) = (c.o("bar_y", b / 2.0) - b / 2.0, c.pos("a")? - h / 2.0);
    let field = fire_field::solve(b / 1000.0, h / 1000.0, faces_of(c.so("faces", "blr")), t, c.o("moisture", 1.5));
    let (side, bot) = (field.side_depth(500.0) * 1000.0, field.bottom_depth(500.0) * 1000.0);
    let theta_s = field.at(ybar / 1000.0, zbar / 1000.0);
    let ks = ks_theta(theta_s);
    let mut section = Sec::rect(b, h, conc(fck), fck, fyk);
    section.bars = vec![Bar { y: ybar, z: zbar, a: as_ }];
    let ring: Vec<(f64, f64)> = field.isotherm_ring(500.0, 72).iter().map(|p| (p.0 * 1000.0, p.1 * 1000.0)).collect();
    let mrd = if ring.len() >= 3 { fire_field::reduced_section(&section, ring, &field, fyk).m_rd_y(0.0).map_or(0.0, |r| r.0) } else { 0.0 };
    let mut v = vec![
        Outcome::mm("az", "Damaged zone a_z at the sides", "Geschädigte Zone a_z seitlich", side, "500 °C isotherm".into()),
        Outcome::mm("a500", "500 °C isotherm depth at the bottom", "Tiefe 500-°C-Isotherme unten", bot, "".into()),
        Outcome::num("theta_s", "Bar temperature (°C)", "Stabtemperatur (°C)", theta_s, "bilinear in the field".into()).steps_de("bilinear im Feld"),
        Outcome::num("ks", "k_s(θ_s)", "k_s(θ_s)", ks, "Table 3.2a".into()),
        Outcome::nmm("m_rd_fi", "M_Rd,fi", "M_Rd,fi", mrd, "strain compatibility on the section inside the 500 °C isotherm, bar area k_s A_s".into()).steps_de("Dehnungsverträglichkeit am Querschnitt innerhalb der 500-°C-Isotherme, Stabfläche k_s A_s"),
    ];
    if let Some(m) = c.opt("m_ed_fi") { v.push(Outcome::moment("util", "Fire bending utilisation", "Biegeausnutzung im Brandfall", m, mrd, "M_Ed,fi ≤ M_Rd,fi".into())); }
    Ok(v)
}

pub fn r04(c: &Case) -> R {
    let (w, tm) = (c.pos("w")?, c.pos("theta_m")?);
    let th = c.l("zone_theta")?;
    let n = th.len() as f64;
    let kcm = (1.0 - 0.2 / n) / n * th.iter().map(|t| kc_theta(*t)).sum::<f64>();
    let az = w * (1.0 - (kcm / kc_theta(tm)).powf(c.ndp("ndp.en1992.fire-zone-exponent")));
    let mut v = vec![Outcome::num("kc_m", "k_c,m", "k_c,m", kcm, format!("{} zones", n)), Outcome::mm("az", "Damage thickness a_z", "Schädigungstiefe a_z", az, "w (1 − (k_c,m/k_c(θ_M))^e)".into())];
    if let Some(b) = c.opt("b") { v.push(Outcome::mm("b_red", "Reduced width b'", "Reduzierte Breite b'", b - 2.0 * az, "b − 2 a_z".into())); }
    Ok(v)
}

pub fn r05(c: &Case) -> R {
    let (b, h, fck, as_) = (c.pos("b")?, c.pos("h")?, c.pos("fck")?, c.pos("as")?);
    let (ned, l0, e0) = (c.pos("n_ed_fi")?, c.pos("l0_fi")?, c.o("e0", 0.0));
    let az = c.o("az", 0.0);
    let fyk = c.o("fyk", 500.0);
    let ks = c.o("ks", 1.0);
    let hr = h - 2.0 * az;
    let br = b - 2.0 * az;
    let lam_fi = l0 / (hr / 12.0f64.sqrt());
    let mut s = Sec::rect(br, hr, conc(fck), fck, fyk * ks);
    let ad = c.o("a", 40.0) - az;
    s.bars = vec![Bar { y: 0.0, z: -(hr / 2.0 - ad), a: as_ / 2.0 }, Bar { y: 0.0, z: hr / 2.0 - ad, a: as_ / 2.0 }];
    let eps_yd = fyk * ks / 200000.0;
    let curv = eps_yd / (0.45 * (hr / 2.0 + hr / 4.0));
    let e2 = curv * l0 * l0 / std::f64::consts::PI.powi(2);
    let (m, _) = s.m_rd_y(ned).ok_or("N_Ed,fi outside the fire resistance")?;
    let med = ned * (e0 + e2);
    Ok(vec![
        Outcome::num("lambda_fi", "Slenderness at fire λ_fi", "Schlankheit im Brandfall λ_fi", lam_fi, "l_0,fi/i".into()),
        Outcome::length("lambda_limit", "Slenderness (limit 80)", "Schlankheit (Grenze 80)", lam_fi, 80.0, "λ_fi ≤ 80".into()),
        Outcome::nmm("m_rd_fi", "M_Rd,fi(N_Ed,fi)", "M_Rd,fi(N_Ed,fi)", m, "strain compatibility on the reduced section".into()).steps_de("Dehnungsverträglichkeit am reduzierten Querschnitt"),
        Outcome::moment("util", "Column fire utilisation", "Stützen-Brandausnutzung", med, m, format!("e2 = {}", f(e2))),
    ])
}

pub fn r06(c: &Case) -> R {
    let (gk, qk) = (c.pos("gk")?, c.o("qk", 0.0));
    let psi = c.o("psi_fi", 0.5);
    let (gg, gq) = (c.o("gamma_g", c.ndp("ndp.en1990.gG_sup.str.A1")), c.o("gamma_q", c.ndp("ndp.en1990.gQ.str.A1")));
    let eta = (gk + psi * qk) / (gg * gk + gq * qk);
    Ok(vec![Outcome::num("eta_fi", "Reduction factor η_fi", "Abminderungsfaktor η_fi", eta, "(G + ψ Q)/(γ_G G + γ_Q Q)".into()), Outcome::n("e_d_fi", "E_d,fi", "E_d,fi", gk + psi * qk, "γ_GA = 1.0".into())])
}

pub fn r08(c: &Case) -> R {
    let fck = c.pos("fck")?;
    let spall = fck > 80.0;
    Ok(vec![Outcome::num("spalling_risk", "Explosive spalling risk (1 = yes)", "Gefahr abplatzender Betondeckung (1 = ja)", if spall { 1.0 } else { 0.0 }, "f_ck > C80/95: check or provide polypropylene fibres ≥ 2 kg/m³".into()).steps_de("f_ck > C80/95: nachweisen oder Polypropylenfasern ≥ 2 kg/m³ vorsehen"), Outcome::mm("axis_add", "Additional axis distance", "Zusätzlicher Achsabstand", if fck >= 70.0 { c.ndp("ndp.en1992.spalling-add-mm") } else { 0.0 }, "f_ck ≥ 70 MPa".into())])
}

/// 🗂 Entries of this domain.
pub const ENTRIES: &[Entry] = &[
    entry_of("R-01", "DIN EN 1992-1-2", "5", "Tabulated fire resistance", "Tabellenverfahren", &["kind", "sub", "r_class", "mu_fi", "width", "dim", "a", "fck"], &["ndp.en1992.fire-tables", "ndp.en1992.spalling-add-mm"], r01),
    entry_of("D-06", "DIN EN 1992-1-2", "5", "Cover for fire (axis distance)", "Betondeckung für Brand (Achsabstand)", &["kind", "sub", "r_class", "mu_fi", "width", "dim", "a", "fck"], &["ndp.en1992.fire-tables", "ndp.en1992.spalling-add-mm"], r01),
    entry_of("M-13", "DIN EN 1992-1-2", "3", "Elevated temperature factors", "Faktoren bei erhöhter Temperatur", &["theta"], &["ndp.en1992.fire-ks-kc"], r07),
    entry_of("R-02", "DIN EN 1992-1-2", "A", "Temperature field", "Temperaturfeld", &["b", "h", "t_min", "faces", "moisture", "point_y", "point_z"], &["ndp.en1992.fire-ks-kc"], r02),
    entry_of("R-03", "DIN EN 1992-1-2", "B.1", "500 °C isotherm method", "Isothermenmethode", &["b", "h", "t_min", "fck", "fyk", "as", "a", "bar_y", "bar_z", "faces", "m_ed_fi"], &["ndp.en1992.fire-ks-kc"], r03),
    entry_of("R-04", "DIN EN 1992-1-2", "B.2", "Zone method", "Zonenmethode", &["w", "theta_m", "zone_theta", "b"], &["ndp.en1992.fire-ks-kc", "ndp.en1992.fire-zone-exponent"], r04),
    entry_of("R-05", "DIN EN 1992-1-2", "C.3", "Column in fire", "Stütze im Brandfall", &["b", "h", "fck", "as", "n_ed_fi", "l0_fi", "e0", "az", "a", "ks", "fyk"], &["ndp.en1992.fire-ks-kc"], r05),
    entry_of("R-06", "DIN EN 1992-1-2", "2.4", "Fire action and load combination", "Brandeinwirkung und Kombination", &["gk", "qk", "psi_fi", "gamma_g", "gamma_q"], &[], r06),
    entry_of("R-07", "DIN EN 1992-1-2", "3", "Material properties at elevated temperature", "Baustoffeigenschaften bei erhöhter Temperatur", &["theta"], &["ndp.en1992.fire-ks-kc"], r07),
    entry_of("R-08", "DIN EN 1992-1-2", "4.5", "Spalling, shear, torsion and anchorage in fire", "Abplatzen, Querkraft, Torsion und Verankerung im Brandfall", &["fck"], &["ndp.en1992.spalling-add-mm"], r08),
];
