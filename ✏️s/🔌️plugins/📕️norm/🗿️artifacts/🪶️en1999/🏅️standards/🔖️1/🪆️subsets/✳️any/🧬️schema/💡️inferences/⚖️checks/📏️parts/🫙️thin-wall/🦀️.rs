//! 🫙️ EN 1999-1-4 cold-formed sheeting and EN 1999-1-5 shells: pure clause functions. See [`super`].

use crate::*;

/// 🫙 Length parameter ω = L/√(r·t).
pub fn omega(length: f64, radius: f64, thickness: f64) -> f64 { length / (radius * thickness).sqrt().max(1e-12) }

/// 🫙 Value of the registry row `ndp.en1999.shell-<key>` (EN and DE agree; unverified rows, see the row evidence).
pub fn shell_constant(key: &str) -> f64 { semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1999.shell-{key}")) }

/// 🫙 Axial elastic critical stress σ_x,Rcr = 0.605·E·C_x·t/r (C_x: short, medium, long cylinders; clamped-edge assumption C_x,b = 1).
pub fn sigma_x_rcr(radius: f64, thickness: f64, length: f64) -> f64 {
    let w = omega(length, radius, thickness);
    let r_t = radius / thickness;
    let c_x = if w < 1.7 { (1.36 - 1.83 / w + 2.07 / (w * w)).max(0.1) } else if w <= 0.5 * r_t { 1.0 } else { (1.0 + 0.2 * (1.0 - 2.0 * w / r_t)).max(0.6) };
    shell_constant("sigma-x-factor") * na_de::E_PA * c_x * thickness / radius
}

/// 🫙 Circumferential elastic critical stress σ_θ,Rcr = 0.92·E·(t/r)/ω (medium cylinders, C_θ = 1).
pub fn sigma_theta_rcr(radius: f64, thickness: f64, length: f64) -> f64 {
    shell_constant("sigma-theta-factor") * na_de::E_PA * (thickness / radius) / omega(length, radius, thickness).max(1e-6)
}

/// 🫙 Shear elastic critical stress τ_Rcr = 0.75·E·(t/r)/√ω (medium cylinders, C_τ = 1).
pub fn tau_rcr(radius: f64, thickness: f64, length: f64) -> f64 {
    shell_constant("tau-factor") * na_de::E_PA * (thickness / radius) / omega(length, radius, thickness).max(1e-6).sqrt()
}

/// 🫙 Imperfection factor α_x = 0.62/(1 + 1.91·(Δw_k/t)^1.44) with Δw_k = √(r/t)·t/Q and fabrication quality parameter Q.
pub fn alpha_x(radius: f64, thickness: f64, q: f64) -> f64 {
    let dw = (radius / thickness).sqrt() * thickness / q;
    0.62 / (1.0 + 1.91 * (dw / thickness).powf(1.44))
}

/// 🫙 Axial reduction χ_x of the relative slenderness `lambda_bar` with the imperfection factor `alpha` and the registry constants β_x, η_x, λ̄_x0.
pub fn shell_chi_axial(lambda_bar: f64, alpha: f64) -> f64 { shell_chi(lambda_bar, alpha, shell_constant("chi-axial-beta"), shell_constant("chi-axial-eta"), shell_constant("chi-axial-lambda-0")) }

/// 🫙 Circumferential and shear reduction χ_θ, χ_τ with the registry imperfection factor α (`ndp.en1999.shell-alpha-hoop-shear`) and constants β, η, λ̄0.
pub fn shell_chi_hoop_shear(lambda_bar: f64) -> f64 { shell_chi(lambda_bar, shell_constant("alpha-hoop-shear"), shell_constant("chi-hoop-shear-beta"), shell_constant("chi-hoop-shear-eta"), shell_constant("chi-hoop-shear-lambda-0")) }

/// 🫙 Shell buckling reduction χ = 1 (λ̄ ≤ λ̄0), 1 − β·((λ̄−λ̄0)/(λ̄p−λ̄0))^η (plastic range), α/λ̄² (elastic range).
pub fn shell_chi(lambda_bar: f64, alpha: f64, beta: f64, eta: f64, lambda_0: f64) -> f64 {
    if lambda_bar <= lambda_0 { return 1.0; }
    let lambda_p = (alpha / (1.0 - beta)).sqrt();
    if lambda_bar >= lambda_p { return alpha / (lambda_bar * lambda_bar); }
    1.0 - beta * ((lambda_bar - lambda_0) / (lambda_p - lambda_0)).powf(eta)
}

/// 🫙 Quality parameter Q of a fabrication class A, B or C read from the registry (`ndp.en1999.shell-quality-q*`, the same for every annex); 
pub fn shell_quality_q(fabrication_class: semio_s_artifact_norm_en1999_model::FabricationClass) -> f64 {
    semio_s_plugin_norm_compliance::ndp_common(match fabrication_class { semio_s_artifact_norm_en1999_model::FabricationClass::A => "ndp.en1999.shell-quality-q-class-a", semio_s_artifact_norm_en1999_model::FabricationClass::C => "ndp.en1999.shell-quality-q-class-c", semio_s_artifact_norm_en1999_model::FabricationClass::B => "ndp.en1999.shell-quality-q" })
}

/// 🫙 Interaction of the membrane stress components (exponents k_x, k_θ, k_τ and coupling k_i of the shell buckling rules).
pub fn shell_interaction(sx: f64, st: f64, tau: f64, rx: f64, rt: f64, rtau: f64, chi_x: f64, chi_t: f64, chi_tau: f64) -> f64 {
    let (base, slope) = (shell_constant("interaction-k-base"), shell_constant("interaction-k-slope"));
    let kx = base + slope * chi_x;
    let kt = base + slope * chi_t;
    let ktau = shell_constant("interaction-ktau-base") + shell_constant("interaction-ktau-slope") * chi_tau;
    let ki = (chi_x * chi_t).powi(2);
    let ax = (sx / rx.max(1e-9)).abs();
    let at = (st / rt.max(1e-9)).abs();
    (ax.powf(kx) - ki * ax * at + at.powf(kt) + (tau / rtau.max(1e-9)).abs().powf(ktau)).max(0.0)
}

/// 🪗 One trapezoidal rib of a profiled sheet per pitch: two webs of the height `height` between the flange mid-lines, the flanges `top` and `bottom` wide, all dimensions [m] on the mid-line.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rib { pub pitch: f64, pub top: f64, pub bottom: f64, pub height: f64 }

impl Rib {
    /// 🪗 Horizontal projection of one web, (p − b_t − b_b)/2.
    pub fn projection(&self) -> f64 { 0.5 * (self.pitch - self.top - self.bottom) }
    /// 🪗 Positive flanges and height and a pitch that holds both flanges.
    pub fn is_valid(&self) -> bool { self.top > 0.0 && self.bottom > 0.0 && self.height > 0.0 && self.projection() >= 0.0 }
    /// 🪗 Slant length s_w = √(h² + projection²) of one web.
    pub fn web_length(&self) -> f64 { self.height.hypot(self.projection()) }
    /// 🪗 Web inclination φ [rad] against the flanges.
    pub fn web_angle(&self) -> f64 { self.height.atan2(self.projection()) }
    /// 🪗 The same rib turned upside down (hogging moment: the bottom flange is compressed).
    pub fn mirrored(&self) -> Rib { Rib { top: self.bottom, bottom: self.top, ..*self } }
}

/// 🪗 Thin-walled properties of a rib per pitch for the thickness factors `k = [top, web, bottom]` (effective thickness k·t): area `a`, centroid `z_c` above the bottom mid-line, `inertia` about it (Steiner parts, b·t³/12 of the flanges and t·s_w·h²/12 of the webs), and the moduli to the outer fibres `w_top`, `w_bottom`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RibProps { pub a: f64, pub z_c: f64, pub inertia: f64, pub w_top: f64, pub w_bottom: f64 }

/// 🪗 [`RibProps`] of `rib` with the sheet thickness `t` and the effective thickness factors `k = [top, web, bottom]`.
pub fn rib_props(rib: &Rib, t: f64, k: [f64; 3]) -> RibProps {
    let (h, s) = (rib.height, rib.web_length());
    let (a_t, a_w, a_b) = (k[0] * t * rib.top, k[1] * t * s, k[2] * t * rib.bottom);
    let a = a_t + 2.0 * a_w + a_b;
    let z_c = h * (a_t + a_w) / a.max(1e-30);
    let own = |b: f64, ki: f64| b * (ki * t).powi(3) / 12.0;
    let inertia = a_t * (h - z_c).powi(2) + a_b * z_c * z_c + 2.0 * a_w * ((0.5 * h - z_c).powi(2) + h * h / 12.0) + own(rib.top, k[0]) + own(rib.bottom, k[2]) + 2.0 * (k[1] * t).powi(3) * rib.projection().powi(2) / (12.0 * s);
    RibProps { a, z_c, inertia, w_top: inertia / (h - z_c + 0.5 * t).max(1e-12), w_bottom: inertia / (z_c + 0.5 * t).max(1e-12) }
}

/// 🪗 Effective section of a rib under a sagging moment (the top flange is compressed) found by fixed-point iteration: the compressed flange is the internal element b_t/t, the web has the stress ratio ψ = −z_c/(h − z_c) of its mid-line ends and β = η·s_w/t with η = 0.70 + 0.30·ψ (EN 1999-1-1 6.1.4.2), ρ_c of §6.1.5 gives the effective thickness ρ_c·t of each; the tension flange stays gross. Returns the thickness factors and the properties of the effective rib.
pub fn rib_effective(rib: &Rib, t: f64, f_o: f64, class_a: bool, welded: bool) -> ([f64; 3], RibProps) {
    let eps = part_1_1::epsilon(f_o);
    let s = rib.web_length();
    let mut k = [1.0; 3];
    let mut props = rib_props(rib, t, k);
    for _ in 0..24 {
        let above = rib.height - props.z_c;
        let psi = if above > 1e-12 { -props.z_c / above } else { -1.0 };
        let eta = part_1_1::stress_gradient_eta(psi);
        let next = [part_1_1::rho_c(rib.top / t / eps, class_a, false, welded), part_1_1::rho_c(eta * s / t / eps, class_a, false, welded), 1.0];
        let done = next.iter().zip(&k).all(|(a, b)| (a - b).abs() < 1e-12);
        k = next;
        props = rib_props(rib, t, k);
        if done { break; }
    }
    (k, props)
}

/// 🪗 Bending resistance of a rib per pitch for a sagging (`hogging = false`) or hogging moment: M_Rd = min(W_eff,top, W_eff,bottom)·f_o/γ_M1 of the effective rib ([`rib_effective`]); also the effective rib properties.
pub fn rib_bending_resistance(rib: &Rib, t: f64, f_o: f64, class_a: bool, welded: bool, hogging: bool, gamma_m1: f64) -> (f64, RibProps) {
    let (_, props) = rib_effective(&if hogging { rib.mirrored() } else { *rib }, t, f_o, class_a, welded);
    (props.w_top.min(props.w_bottom) * f_o / gamma_m1, props)
}

/// 🪗 Axial resistance of a rib per pitch under uniform compression: A_eff·f_o/γ_M1 with the effective thickness ρ_c·t of the flanges (b/t) and the webs (s_w/t, uniform stress), and the effective area.
pub fn rib_axial_resistance(rib: &Rib, t: f64, f_o: f64, class_a: bool, welded: bool, gamma_m1: f64) -> (f64, f64) {
    let eps = part_1_1::epsilon(f_o);
    let rho = |b: f64| part_1_1::rho_c(b / t / eps, class_a, false, welded);
    let a_eff = t * (rho(rib.top) * rib.top + rho(rib.bottom) * rib.bottom + 2.0 * rho(rib.web_length()) * rib.web_length());
    (a_eff * f_o / gamma_m1, a_eff)
}

/// 🪗 Flexure of a profiled sheet of the width `width`: effective area `a_eff`, N_Rd, the bending resistances M_Rd for a sagging and a hogging moment (both with ρ_o,haz), the effective moduli `w_sag`, `w_hog` per rib and the effective second moment `inertia` of the width at the stress f_o.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SheetFlexure { pub a_eff: f64, pub n_rd: f64, pub m_sag: f64, pub m_hog: f64, pub w_sag: f64, pub w_hog: f64, pub inertia: f64 }

/// 🪗 [`SheetFlexure`] of `width` of the ribs `rib` of thickness `t`: the resistances per pitch ([`rib_axial_resistance`], [`rib_bending_resistance`]) scaled by width/pitch and by `rho_haz` (1 for an unwelded sheet).
pub fn rib_sheet_flexure(rib: &Rib, width: f64, t: f64, f_o: f64, class_a: bool, welded: bool, rho_haz: f64, gamma_m1: f64) -> SheetFlexure {
    let scale = width / rib.pitch;
    let (n, a) = rib_axial_resistance(rib, t, f_o, class_a, welded, gamma_m1);
    let ((m_sag, p_sag), (m_hog, p_hog)) = (rib_bending_resistance(rib, t, f_o, class_a, welded, false, gamma_m1), rib_bending_resistance(rib, t, f_o, class_a, welded, true, gamma_m1));
    SheetFlexure { a_eff: a * scale * rho_haz, n_rd: n * scale * rho_haz, m_sag: m_sag * scale * rho_haz, m_hog: m_hog * scale * rho_haz, w_sag: p_sag.w_top.min(p_sag.w_bottom), w_hog: p_hog.w_top.min(p_hog.w_bottom), inertia: p_sag.inertia.min(p_hog.inertia) * scale }
}

/// 🪗 Web shear resistance of `width` of the ribs `rib`: [`rib_shear_resistance`] per pitch scaled by width/pitch, with ρ_v from `constants = (factor, limit)` ([`sheet_web_shear_constants`]).
pub fn rib_sheet_shear(rib: &Rib, width: f64, t: f64, f_o: f64, constants: (f64, f64), gamma_m1: f64) -> f64 {
    rib_shear_resistance(rib, t, f_o, sheet_web_rho_v(rib.web_length() / t, f_o, constants.0, constants.1), gamma_m1) * width / rib.pitch
}

/// 🪗 `(factor, limit)` of the unstiffened sheeting web shear rule from the unverified registry rows `ndp.en1999.sheet-web-shear-factor` and `-limit`.
pub fn sheet_web_shear_constants() -> (f64, f64) { (semio_s_plugin_norm_compliance::ndp_common("ndp.en1999.sheet-web-shear-factor"), semio_s_plugin_norm_compliance::ndp_common("ndp.en1999.sheet-web-shear-limit")) }

/// 🪗 Shear buckling reduction ρ_v = 1 for λ̄_w ≤ `limit`, `limit`/λ̄_w above, with λ̄_w = `factor`·(s_w/t)·√(f_o/E) of an unstiffened sheet web; `factor` and `limit` are the registry rows `ndp.en1999.sheet-web-shear.*`.
pub fn sheet_web_rho_v(s_w_over_t: f64, f_o: f64, factor: f64, limit: f64) -> f64 {
    let lambda = factor * s_w_over_t * (f_o / na_de::E_PA).sqrt();
    if lambda <= limit { 1.0 } else { limit / lambda }
}

/// 🪗 Shear resistance of a rib per pitch, V_Rd = 2·h·t·ρ_v·f_o/(√3·γ_M1): the shear flow of the two inclined webs carries the vertical force over the web height h (the vertical component of the slant resultant).
pub fn rib_shear_resistance(rib: &Rib, t: f64, f_o: f64, rho_v: f64, gamma_m1: f64) -> f64 { 2.0 * rib.height * t * rho_v * f_o / (3.0f64.sqrt() * gamma_m1) }

/// ⬇️ Registry ids of the five coefficients (α, c_r, c_a, c_l, c_φ) of the web crippling resistance of one load position; unverified and null until transcribed.
pub const WEB_CRIPPLING_KEYS: [&str; 5] = ["alpha", "c-r", "c-a", "c-l", "c-phi"];

/// ⬇️ The coefficient set of a load position (`end` | `interior`), `None` while any of them is not registered as a number.
pub fn web_crippling_coefficients(position: &str, annex: AnnexChoice) -> Option<[f64; 5]> {
    let pos = if position == "end" { "end" } else { "interior" };
    let mut out = [0.0; 5];
    for (slot, key) in out.iter_mut().zip(WEB_CRIPPLING_KEYS) {
        *slot = semio_s_plugin_norm_compliance::ndp_cell_opt("ndp.en1999.web-crippling", &[pos, key], annex)?;
    }
    Some(out)
}

/// ⬇️ Web crippling resistance R_w,Rd = α·t²·√(f_o·E)·(1 − c_r·√(r/t))·(c_a + √(c_l·s_s/t))·(c_φ + (φ/90°)²)/γ_M1 of a sheet web (EN 1999-1-4 §6.1.7); the coefficients are the unverified registry rows.
pub fn web_crippling_resistance(t: f64, f_o: f64, r: f64, s_s: f64, phi_deg: f64, c: [f64; 5], gamma_m1: f64) -> f64 {
    let [alpha, c_r, c_a, c_l, c_phi] = c;
    alpha * t * t * (f_o * na_de::E_PA).sqrt() * (1.0 - c_r * (r / t).sqrt()) * (c_a + (c_l * s_s / t).sqrt()) * (c_phi + (phi_deg / 90.0).powi(2)) / gamma_m1
}

/// ⬇️ Intermediate quantities of the transverse web load rule (EN 1999-1-1 §6.7.5).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TransverseLoad {
    pub k_f: f64,
    pub l_y: f64,
    pub f_cr: f64,
    pub lambda_f: f64,
    pub chi_f: f64,
    pub l_eff: f64,
    pub f_rd: f64,
}

/// ⬇️ The five structure constants of the transverse web load rule from the registry (`k_F,0`, `k_F,1`, F_cr factor, χ_F constant, m_2 factor); recalled and unverified.
pub fn transverse_load_constants(annex: AnnexChoice) -> Option<[f64; 5]> {
    let mut out = [0.0; 5];
    for (slot, key) in out.iter_mut().zip(["kf-base", "kf-slope", "fcr-factor", "chi-const", "m2-factor"]) {
        *slot = semio_s_plugin_norm_compliance::ndp_number_opt(&format!("ndp.en1999.transverse-load.{key}"), annex)?;
    }
    Some(out)
}

/// ⬇️ Transverse load resistance of a web loaded through one flange over `s_s`: k_F = k0 + k1·(h_w/a)², m_1 = f_of·b_f/(f_ow·t_w), m_2 = m2·(h_w/t_f)² for λ̄_F > 0.5, l_y = s_s + 2·t_f·(1 + √(m_1 + m_2)) ≤ a, F_cr = c·k_F·E·t_w³/h_w, λ̄_F = √(l_y·t_w·f_ow/F_cr), χ_F = min(χ0/λ̄_F, 1), F_Rd = f_ow·χ_F·l_y·t_w/γ_M1 (`a` ≤ 0: no transverse stiffeners).
pub fn transverse_load_resistance(h_w: f64, t_w: f64, t_f: f64, b_f: f64, f_o: f64, s_s: f64, a: f64, k: [f64; 5], gamma_m1: f64) -> TransverseLoad {
    let [k0, k1, c_cr, chi0, m2_factor] = k;
    let k_f = if a > 0.0 { k0 + k1 * (h_w / a).powi(2) } else { k0 };
    let m1 = f_o * b_f / (f_o * t_w).max(1e-12);
    let f_cr = c_cr * k_f * na_de::E_PA * t_w.powi(3) / h_w.max(1e-12);
    let reach = |m2: f64| { let l = s_s + 2.0 * t_f * (1.0 + (m1 + m2).sqrt()); if a > 0.0 { l.min(a) } else { l } };
    let lambda = |l: f64| (l * t_w * f_o / f_cr.max(1e-12)).sqrt();
    let l0 = reach(0.0);
    let (l_y, lambda_f) = if lambda(l0) > 0.5 { let l = reach(m2_factor * (h_w / t_f.max(1e-12)).powi(2)); (l, lambda(l)) } else { (l0, lambda(l0)) };
    let chi_f = (chi0 / lambda_f.max(1e-12)).min(1.0);
    let l_eff = chi_f * l_y;
    TransverseLoad { k_f, l_y, f_cr, lambda_f, chi_f, l_eff, f_rd: f_o * l_eff * t_w / gamma_m1 }
}

/// ⭕ Bay lengths of a shell of `length` divided by ring stiffeners at the axial positions `rings` (positions outside the open interval are ignored); one bay without rings.
pub fn ring_bays(length: f64, rings: &[f64]) -> Vec<f64> {
    let mut at: Vec<f64> = rings.iter().copied().filter(|x| *x > 0.0 && *x < length).collect();
    at.sort_by(f64::total_cmp);
    at.dedup();
    let mut bays = Vec::new();
    let mut prev = 0.0;
    for x in at.into_iter().chain([length]) {
        bays.push(x - prev);
        prev = x;
    }
    bays
}

/// ⭕ Effective shell width b_e = factor·√(r·t) beside a ring stiffener.
pub fn ring_effective_width(radius: f64, thickness: f64, factor: f64) -> f64 { factor * (radius * thickness).sqrt() }

/// ⭕ `(A, I, z_c)` of a flat-bar ring (depth × thickness) with the effective shell strip `b_e × t`: the shell mid-surface is the reference line, the bar stands on one side of it; `I` about the combined centroid.
pub fn ring_section(shell_thickness: f64, b_e: f64, depth: f64, bar_thickness: f64) -> (f64, f64, f64) {
    let (a_s, a_b) = (b_e * shell_thickness, depth * bar_thickness);
    let z_b = 0.5 * shell_thickness + 0.5 * depth;
    let z_c = a_b * z_b / (a_s + a_b).max(1e-30);
    let inertia = b_e * shell_thickness.powi(3) / 12.0 + a_s * z_c * z_c + bar_thickness * depth.powi(3) / 12.0 + a_b * (z_b - z_c).powi(2);
    (a_s + a_b, inertia, z_c)
}

/// 📐 Elastic foundation stiffness K = E·t³/(4·(1 − ν²)·b³) per unit length of the outstand plate strip (cantilever of width `b`, tip spring 3·E·I_p/b³ with I_p = t³/(12·(1 − ν²)) per metre) that supports an edge stiffener.
pub fn edge_stiffener_spring(thickness: f64, outstand: f64) -> f64 {
    na_de::E_PA * thickness.powi(3) / (4.0 * (1.0 - na_de::NU * na_de::NU) * outstand.powi(3).max(1e-18))
}

/// 📐 Flat-bar lip `depth × thickness`: `(A_s, I_s)` with I_s = t_s·d³/12 for the deflection along the lip depth.
pub fn edge_stiffener_section(depth: f64, thickness: f64) -> (f64, f64) { (depth * thickness, thickness * depth.powi(3) / 12.0) }

/// 📐 Elastic flexural buckling stress σ_cr,s = 2·√(K·E·I_s)/A_s of a stiffener strut on the elastic foundation `k` (Winkler strut of minimum buckling load 2·√(K·E·I)).
pub fn edge_stiffener_sigma_cr(k: f64, inertia: f64, area: f64) -> f64 { 2.0 * (k * na_de::E_PA * inertia).sqrt() / area.max(1e-18) }

/// 📐 Edge stiffener flexural utilisation λ̄_s² = f_o/σ_cr,s against the slenderness limit λ̄_s,max squared; `limit` is the registry row `ndp.en1999.stiffener.edge-slenderness-limit`.
pub fn edge_stiffener_utilization(f_o: f64, sigma_cr: f64, limit: f64) -> f64 { f_o / sigma_cr.max(1e-9) / (limit * limit).max(1e-12) }

/// ⭕ Critical radial line load w_cr = 3·E·I/r³ of a ring (flexural n = 2 mode of a ring of in-plane inertia `inertia` under uniform external pressure) per unit of circumference.
pub fn ring_critical_line_load(inertia: f64, radius: f64) -> f64 { 3.0 * na_de::E_PA * inertia / radius.powi(3).max(1e-18) }

/// ⭕ Tributary length of the ring at `x`: half of the bay on each side.
pub fn ring_tributary(shell_length: f64, rings: &[f64], x: f64) -> f64 {
    let mut at: Vec<f64> = rings.iter().copied().filter(|v| *v > 0.0 && *v < shell_length).collect();
    at.sort_by(f64::total_cmp);
    at.dedup();
    let before = at.iter().copied().filter(|v| *v < x).fold(0.0, f64::max);
    let after = at.iter().copied().filter(|v| *v > x).fold(shell_length, f64::min);
    0.5 * (after - before)
}
