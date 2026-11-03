//! 🌊️ Geotechnical details of EN 1998-5: the Annex B liquefaction check with the Youd et al. (2001) comparison and the screening note, the Annex F seismic bearing surface and the inertial and kinematic pile moments. Every constant is an unverified registry row. See [`super`].

use super::*;
use semio_s_plugin_norm_compliance::{ndp_number, CheckStatus};

type Foundation = semio_s_artifact_norm_en1998_model::En1998Foundation;

const ANNEX_F_IDS: [&str; 16] = [
    "ndp.en1998.en5.annex-f.gamma-rd", "ndp.en1998.en5.annex-f.sand.a", "ndp.en1998.en5.annex-f.sand.b", "ndp.en1998.en5.annex-f.sand.c", "ndp.en1998.en5.annex-f.sand.d", "ndp.en1998.en5.annex-f.sand.e",
    "ndp.en1998.en5.annex-f.sand.f", "ndp.en1998.en5.annex-f.sand.m", "ndp.en1998.en5.annex-f.sand.k", "ndp.en1998.en5.annex-f.sand.k-prime", "ndp.en1998.en5.annex-f.sand.c-t", "ndp.en1998.en5.annex-f.sand.c-t-prime",
    "ndp.en1998.en5.annex-f.sand.c-m", "ndp.en1998.en5.annex-f.sand.c-m-prime", "ndp.en1998.en5.annex-f.sand.beta", "ndp.en1998.en5.annex-f.sand.gamma",
];

fn q_moment(v: f64) -> Quantity { Quantity::new(QuantityKind::Moment, v) }

/// 🌊️ The liquefaction layer of a foundation with the design acceleration a_g S, `None` while the layer is not described.
pub fn liquefaction_layer(annex: AnnexChoice, f: &Foundation, a_g_s: f64) -> Option<part_5::LiquefactionLayer> {
    if f.liq_n1_60 <= 0.0 || f.liq_unit_weight_n_m3 <= 0.0 || f.liq_depth_m <= 0.0 || f.liq_magnitude <= 0.0 {
        return None;
    }
    Some(part_5::LiquefactionLayer { n1_60: f.liq_n1_60, fines_percent: f.liq_fines_percent, unit_weight_n_m3: f.liq_unit_weight_n_m3, water_depth_m: f.liq_water_depth_m, magnitude: f.liq_magnitude, a_g_s_ms2: a_g_s, csr_factor: ndp_number("ndp.en1998.en5.liquefaction.annex-b.csr-factor", annex) })
}

/// 🌊️ Depth range of the layer: the point value, or the thickness around the layer depth clamped to the surface.
pub fn liquefaction_band(f: &Foundation) -> (f64, f64) {
    let half = 0.5 * f.liq_layer_thickness_m.max(0.0);
    ((f.liq_depth_m - half).max(0.1), f.liq_depth_m + half)
}

fn governing_depth(layer: &part_5::LiquefactionLayer, band: (f64, f64), factor: impl Fn(&part_5::LiquefactionLayer, f64) -> f64) -> (f64, f64) {
    (0..=8)
        .map(|i| band.0 + (band.1 - band.0) * f64::from(i) / 8.0)
        .map(|z| (z, factor(layer, z)))
        .fold((band.0, f64::INFINITY), |best, point| if point.1 < best.1 { point } else { best })
}

/// 🌊️ Checks the liquefaction safety factor of the layer of a foundation by EN 1998-5 Annex B, with the Youd et al. (2001) factor and the screening criteria reported next to it.
pub fn check_liquefaction(report: &mut CheckReport, annex: AnnexChoice, f: &Foundation, gamma_i: f64, a_g: f64, s: f64) {
    let a_g_s = gamma_i * a_g * s;
    let Some(layer) = liquefaction_layer(annex, f, a_g_s) else { return };
    let band = liquefaction_band(f);
    let (z, f_l) = governing_depth(&layer, band, |l, z| l.factor_annex_b(z));
    let (z_youd, f_youd) = governing_depth(&layer, band, |l, z| l.factor_youd(z));
    let required_f = ndp_number("ndp.en1998.en5.liquefaction.min-safety-factor", annex);
    let (total, effective) = layer.stresses(z);
    let csr = part_5::liquefaction_csr_annex_b(layer.csr_factor, a_g_s, total, effective);
    let path = id_sel("foundations", &f.id);
    let mut check = CheckResult::assess(
        format!("en1998.5.{}.liquefaction", f.id),
        "DIN EN 1998-5",
        ClauseId::new("EN 1998-5", "5", "4.1.4"),
        SubjectRef::new(&f.id, format!("{path}.liqN160"), lc(&f.id, &f.id)),
        lc("Liquefaction safety factor F_L = CRR·MSF/CSR (Annex B)", "Verflüssigungssicherheit F_L = CRR·MSF/CSR (Anhang B)"),
    )
    .utilization(q_dim(required_f), floor_limit(q_dim(f_l.min(1e9))))
    .annex(annex)
    .explanation(lc(
        &format!("F_L = {f_l:.3} at {z:.1} m (required {required_f}); CSR {csr:.4} without depth reduction, CRR·MSF {:.4}, a_g S {a_g_s:.3} m/s², σ_v {total:.0} Pa, σ'_v {effective:.0} Pa; Youd et al. 2001 with r_d gives {f_youd:.3} at {z_youd:.1} m", layer.crr()),
        &format!("F_L = {f_l:.3} in {z:.1} m (erforderlich {required_f}); CSR {csr:.4} ohne Tiefenabminderung, CRR·MSF {:.4}, a_g S {a_g_s:.3} m/s², σ_v {total:.0} Pa, σ'_v {effective:.0} Pa; Youd et al. 2001 mit r_d ergibt {f_youd:.3} in {z_youd:.1} m", layer.crr()),
    ));
    if f_l < required_f {
        let improved = |n: f64| governing_depth(&part_5::LiquefactionLayer { n1_60: n, ..layer }, band, |l, z| l.factor_annex_b(z)).1;
        let (mut low, mut high) = (f.liq_n1_60, 60.0);
        if improved(high) >= required_f {
            for _ in 0..48 {
                let mid = 0.5 * (low + high);
                if improved(mid) >= required_f { high = mid } else { low = mid }
            }
            check = check.remedy(Remedy::at_least(
                SubjectRef::new(&f.id, format!("{path}.liqN160"), lc("Blow count (N1)60", "Schlagzahl (N1)60")),
                q_dim(f.liq_n1_60),
                q_dim(high),
                lc("Densify the layer (ground improvement) until the blow count reaches the required value.", "Schicht verdichten (Baugrundverbesserung), bis die Schlagzahl den erforderlichen Wert erreicht."),
            ));
        }
        let drained = |w: f64| governing_depth(&part_5::LiquefactionLayer { water_depth_m: w, ..layer }, band, |l, z| l.factor_annex_b(z)).1;
        if drained(band.1) >= required_f {
            let (mut low, mut high) = (f.liq_water_depth_m, band.1);
            for _ in 0..48 {
                let mid = 0.5 * (low + high);
                if drained(mid) >= required_f { high = mid } else { low = mid }
            }
            check = check.remedy(Remedy::at_least(
                SubjectRef::new(&f.id, format!("{path}.liqWaterDepthM"), lc("Groundwater depth", "Grundwassertiefe")),
                q_len(f.liq_water_depth_m),
                q_len(high),
                lc("Lower the ground water table (drainage) below the layer.", "Grundwasserspiegel unter die Schicht absenken (Dränage)."),
            ));
        }
    }
    report.push(check.ndps(&["ndp.en1998.en5.liquefaction.min-safety-factor", "ndp.en1998.ref.liquefaction.youd-2001", "ndp.en1998.en5.liquefaction.annex-b.csr-factor"]).build());
    report.push(
        CheckResult::assess(format!("en1998.5.{}.liquefaction.youd", f.id), "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "4.1.4"), SubjectRef::new(&f.id, format!("{path}.liqN160"), lc(&f.id, &f.id)), lc("Liquefaction safety factor by Youd et al. (2001) for comparison", "Verflüssigungssicherheit nach Youd et al. (2001) zum Vergleich"))
            .informative(q_dim(f_youd.min(1e9)))
            .annex(annex)
            .explanation(lc(
                &format!("F_L = {f_youd:.3} with the depth reduction r_d = {:.3} at {z_youd:.1} m; Annex B uses no r_d, so its factor is lower by r_d (here {:.0} %); CRR and MSF are identical, the fines correction follows Youd et al.", part_5::liquefaction_rd(z_youd), 100.0 * (1.0 - part_5::liquefaction_rd(z_youd))),
                &format!("F_L = {f_youd:.3} mit der Tiefenabminderung r_d = {:.3} in {z_youd:.1} m; Anhang B verwendet kein r_d, sein Faktor ist daher um r_d niedriger (hier {:.0} %); CRR und MSF sind gleich, die Feinkornkorrektur folgt Youd et al.", part_5::liquefaction_rd(z_youd), 100.0 * (1.0 - part_5::liquefaction_rd(z_youd))),
            ))
            .ndp("ndp.en1998.ref.liquefaction.youd-2001")
            .build(),
    );
    let (limit_g, depth_limit) = (ndp_number("ndp.en1998.en5.liquefaction.screen-ag-s-g", annex), ndp_number("ndp.en1998.en5.liquefaction.screen-depth-m", annex));
    let neglect = a_g_s / 9.81 <= limit_g || f.liq_depth_m > depth_limit;
    report.push(
        CheckResult::assess(format!("en1998.5.{}.liquefaction.screen", f.id), "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "4.1.4"), SubjectRef::new(&f.id, format!("{path}.liqDepthM"), lc(&f.id, &f.id)), lc("Screening criteria for liquefaction", "Ausschlusskriterien für Bodenverflüssigung"))
            .informative(q_dim(a_g_s / 9.81))
            .annex(annex)
            .explanation(lc(
                &format!("a_g S/g = {:.3} (screening limit {limit_g}), layer depth {:.1} m (limit {depth_limit} m): {}", a_g_s / 9.81, f.liq_depth_m, if neglect { "liquefaction may be neglected by the screening criteria; the factor above is still reported" } else { "the screening criteria do not exclude liquefaction" }),
                &format!("a_g S/g = {:.3} (Grenze {limit_g}), Schichttiefe {:.1} m (Grenze {depth_limit} m): {}", a_g_s / 9.81, f.liq_depth_m, if neglect { "Bodenverflüssigung darf nach den Ausschlusskriterien vernachlässigt werden; der Faktor oben wird dennoch angegeben" } else { "die Ausschlusskriterien schließen Bodenverflüssigung nicht aus" }),
            ))
            .ndps(&["ndp.en1998.en5.liquefaction.screen-ag-s-g", "ndp.en1998.en5.liquefaction.screen-depth-m"])
            .build(),
    );
}

/// 🏗️ Checks the seismic bearing capacity of a footing with the generalised failure surface of EN 1998-5 Annex F.
pub fn check_bearing_annex_f(report: &mut CheckReport, annex: AnnexChoice, f: &Foundation, gamma_i: f64, a_g: f64, s: f64) {
    if f.footing_width_m <= 0.0 || f.bearing_n_max_n <= 0.0 || f.soil_friction_angle_deg <= 0.0 {
        return;
    }
    let k = part_5::AnnexF::from_registry(annex);
    let a_g_s = gamma_i * a_g * s;
    let f_bar = part_5::AnnexF::soil_inertia(a_g_s, f.soil_friction_angle_deg);
    let Some((n, v, m)) = k.normalise(f.bearing_axial_n, f.bearing_shear_n, f.bearing_moment_nm, f.footing_width_m, f.bearing_n_max_n) else { return };
    let util = k.utilisation(n, v, m, f_bar);
    let path = id_sel("foundations", &f.id);
    let mut check = CheckResult::assess(
        format!("en1998.5.{}.annexF", f.id),
        "DIN EN 1998-5",
        ClauseId::new("EN 1998-5", "5", "F"),
        SubjectRef::new(&f.id, format!("{path}.bearingNMaxN"), lc(&f.id, &f.id)),
        lc("Seismic bearing capacity: generalised failure surface (Annex F)", "Erdbebengrundbruch: verallgemeinerte Bruchfläche (Anhang F)"),
    )
    .utilization(q_dim(util.min(1e6)), q_dim(1.0))
    .annex(annex)
    .explanation(lc(
        &format!("N̄ {n:.3}, V̄ {v:.3}, M̄ {m:.3}, soil inertia F̄ {f_bar:.3}; vertical bound {:.3}; surface value {:.3}; the load path scales by 1/{util:.3} to the surface", k.vertical_bound(f_bar), k.surface(n, v, m, f_bar)),
        &format!("N̄ {n:.3}, V̄ {v:.3}, M̄ {m:.3}, Bodenträgheit F̄ {f_bar:.3}; vertikale Grenze {:.3}; Flächenwert {:.3}; der Lastpfad erreicht die Fläche beim Faktor 1/{util:.3}", k.vertical_bound(f_bar), k.surface(n, v, m, f_bar)),
    ));
    if util > 1.0 {
        let at = |n_max: f64| k.normalise(f.bearing_axial_n, f.bearing_shear_n, f.bearing_moment_nm, f.footing_width_m, n_max).map_or(f64::INFINITY, |(n, v, m)| k.utilisation(n, v, m, f_bar));
        let (mut low, mut high) = (f.bearing_n_max_n, f.bearing_n_max_n * 1000.0);
        if at(high) <= 1.0 {
            for _ in 0..60 {
                let mid = 0.5 * (low + high);
                if at(mid) <= 1.0 { high = mid } else { low = mid }
            }
            check = check.remedy(Remedy::at_least(
                SubjectRef::new(&f.id, format!("{path}.bearingNMaxN"), lc("Ultimate vertical force N_max", "Vertikale Grenztragkraft N_max")),
                q_force(f.bearing_n_max_n),
                q_force(high),
                lc("Enlarge or deepen the footing or improve the ground until N_max reaches the required value.", "Fundament vergrößern oder tiefer gründen bzw. Baugrund verbessern, bis N_max den erforderlichen Wert erreicht."),
            ));
        }
    }
    report.push(check.ndps(&ANNEX_F_IDS).build());
}

/// 🏗️ Checks the inertial and kinematic bending moments of a pile against its bending resistance.
pub fn check_pile(report: &mut CheckReport, annex: AnnexChoice, f: &Foundation, gamma_i: f64, a_g: f64, s: f64) {
    if f.pile_diameter_m <= 0.0 || f.pile_length_m <= 0.0 || f.pile_modulus_pa <= 0.0 || f.pile_subgrade_modulus_n_m3 <= 0.0 {
        return;
    }
    let a_g_s = gamma_i * a_g * s;
    let path = id_sel("foundations", &f.id);
    let fixed = f.pile_head_fixed >= 0.5;
    let inertial = part_5::pile_inertial(f.pile_modulus_pa, f.pile_diameter_m, f.pile_length_m, f.pile_subgrade_modulus_n_m3, f.pile_head_shear_n, fixed);
    let subject = |field: &str| SubjectRef::new(&f.id, format!("{path}.{field}"), lc(&f.id, &f.id));
    let mut length = CheckResult::assess(format!("en1998.5.{}.pile.length", f.id), "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "5.4.2"), subject("pileLengthM"), lc("Pile length for the long pile Winkler solution (β L ≥ π)", "Pfahllänge für die Winkler-Lösung des langen Pfahls (β L ≥ π)"))
        .minimum(q_dim(inertial.beta * f.pile_length_m), q_dim(std::f64::consts::PI))
        .annex(annex)
        .explanation(lc(&format!("β = {:.3} 1/m, β L = {:.2}", inertial.beta, inertial.beta * f.pile_length_m), &format!("β = {:.3} 1/m, β L = {:.2}", inertial.beta, inertial.beta * f.pile_length_m)));
    if !inertial.long {
        length = length.remedy(Remedy::at_least(SubjectRef::new(&f.id, format!("{path}.pileLengthM"), lc("Pile length", "Pfahllänge")), q_len(f.pile_length_m), q_len(std::f64::consts::PI / inertial.beta), lc("Lengthen the pile so that the closed form of the long pile holds, or analyse it with its base conditions.", "Pfahl verlängern, damit die geschlossene Form des langen Pfahls gilt, oder mit Fußbedingung berechnen.")));
    }
    report.push(length.build());
    report.push(
        CheckResult::assess(format!("en1998.5.{}.pile.inertial", f.id), "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "5.4.2"), subject("pileHeadShearN"), lc("Inertial pile moment from the superstructure", "Trägheitsmoment des Pfahls aus dem Tragwerk"))
            .informative(q_moment(inertial.max_moment_nm))
            .annex(annex)
            .explanation(lc(
                &format!("V_head {:.0} N, head {}: M_max {:.0} N·m, head moment {:.0} N·m, head deflection {:.2} mm (Winkler, k_h {:.2e} N/m³)", f.pile_head_shear_n, if fixed { "fixed" } else { "free" }, inertial.max_moment_nm, inertial.head_moment_nm, inertial.head_deflection_m * 1000.0, f.pile_subgrade_modulus_n_m3),
                &format!("V_Kopf {:.0} N, Kopf {}: M_max {:.0} N·m, Kopfmoment {:.0} N·m, Kopfverschiebung {:.2} mm (Winkler, k_h {:.2e} N/m³)", f.pile_head_shear_n, if fixed { "eingespannt" } else { "frei" }, inertial.max_moment_nm, inertial.head_moment_nm, inertial.head_deflection_m * 1000.0, f.pile_subgrade_modulus_n_m3),
            ))
            .build(),
    );
    let contrast = part_5::stiffness_contrast(f.pile_upper_vs_m_s, f.pile_lower_vs_m_s);
    let limit = ndp_number("ndp.en1998.en5.pile.stiffness-contrast", annex);
    let kinematic_applies = f.pile_interface_depth_m > 0.0 && f.pile_upper_vs_m_s > 0.0 && f.pile_lower_vs_m_s > 0.0 && f.liq_unit_weight_n_m3 > 0.0 && contrast >= limit;
    let mut kinematic = 0.0;
    if kinematic_applies {
        let coefficients = part_5::Nikolaou::from_registry(annex);
        let tau = part_5::interface_shear_stress_pa(a_g_s, f.liq_unit_weight_n_m3, f.pile_interface_depth_m);
        let e_1 = part_5::soil_young_modulus_pa(f.liq_unit_weight_n_m3, f.pile_upper_vs_m_s);
        kinematic = part_5::pile_kinematic_moment_nm(&coefficients, tau, f.pile_diameter_m, f.pile_length_m, f.pile_modulus_pa, e_1, f.pile_upper_vs_m_s, f.pile_lower_vs_m_s);
        report.push(
            CheckResult::assess(format!("en1998.5.{}.pile.kinematic", f.id), "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "5.4.2"), subject("pileInterfaceDepthM"), lc("Kinematic pile moment at the layer interface", "Kinematisches Pfahlmoment an der Schichtgrenze"))
                .informative(q_moment(kinematic))
                .annex(annex)
                .explanation(lc(
                    &format!("G₂/G₁ = {contrast:.1} (limit {limit}), τ_c {tau:.0} Pa, E₁ {e_1:.3e} Pa: M_k = {kinematic:.0} N·m at {:.1} m (Nikolaou et al. 2001, a reference basis outside EN 1998-5)", f.pile_interface_depth_m),
                    &format!("G₂/G₁ = {contrast:.1} (Grenze {limit}), τ_c {tau:.0} Pa, E₁ {e_1:.3e} Pa: M_k = {kinematic:.0} N·m in {:.1} m (Nikolaou et al. 2001, eine Bezugsbasis außerhalb von EN 1998-5)", f.pile_interface_depth_m),
                ))
                .ndps(&["ndp.en1998.en5.pile.stiffness-contrast", "ndp.en1998.ref.pile.nikolaou-2001.coefficient", "ndp.en1998.ref.pile.nikolaou-2001.length-exponent", "ndp.en1998.ref.pile.nikolaou-2001.modulus-exponent", "ndp.en1998.ref.pile.nikolaou-2001.velocity-exponent"])
                .build(),
        );
    }
    let demand = inertial.max_moment_nm + kinematic;
    let mut moment = CheckResult::assess(format!("en1998.5.{}.pile.moment", f.id), "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "5.4.2"), subject("pileMomentRdNm"), lc("Pile bending moment (inertial + kinematic) against the bending resistance", "Pfahlbiegemoment (Trägheit + Kinematik) gegen die Biegetragfähigkeit"))
        .utilization(q_moment(demand), floor_limit(q_moment(f.pile_moment_rd_nm)))
        .annex(annex)
        .explanation(lc(&format!("M_Ed = {:.0} + {kinematic:.0} = {demand:.0} N·m against M_Rd {:.0} N·m (the maxima add, which is conservative)", inertial.max_moment_nm, f.pile_moment_rd_nm), &format!("M_Ed = {:.0} + {kinematic:.0} = {demand:.0} N·m gegen M_Rd {:.0} N·m (die Maxima werden addiert, was auf der sicheren Seite liegt)", inertial.max_moment_nm, f.pile_moment_rd_nm)));
    if demand > f.pile_moment_rd_nm {
        moment = moment.remedy(Remedy::at_least(SubjectRef::new(&f.id, format!("{path}.pileMomentRdNm"), lc("Pile bending resistance", "Pfahlbiegetragfähigkeit")), q_moment(f.pile_moment_rd_nm), q_moment(demand), lc("Increase the reinforcement or the diameter of the pile.", "Bewehrung oder Durchmesser des Pfahls erhöhen.")));
    }
    report.push(moment.build());
    if f.pile_interface_depth_m > 0.0 && !kinematic_applies && f.pile_upper_vs_m_s > 0.0 && f.pile_lower_vs_m_s > 0.0 {
        report.push(
            CheckResult::assess(format!("en1998.5.{}.pile.kinematic", f.id), "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "5.4.2"), subject("pileInterfaceDepthM"), lc("Kinematic pile moment at the layer interface", "Kinematisches Pfahlmoment an der Schichtgrenze"))
                .status(CheckStatus::Info)
                .annex(annex)
                .explanation(lc(&format!("G₂/G₁ = {contrast:.1} is below {limit} (or the unit weight is missing): kinematic bending is not considered"), &format!("G₂/G₁ = {contrast:.1} liegt unter {limit} (oder die Wichte fehlt): kinematische Biegung wird nicht berücksichtigt")))
                .ndp("ndp.en1998.en5.pile.stiffness-contrast")
                .build(),
        );
    }
}

type SoilLayer = semio_s_artifact_norm_en1998_model::En1998SoilLayer;

/// 🪨️ The layered soil profile of a foundation (rows of `soilLayers` with its id, top to bottom) with the ground water, magnitude and a_g S of the foundation, `None` without layers.
pub fn soil_profile(annex: AnnexChoice, document: &En1998Snapshot, f: &Foundation, a_g_s: f64) -> Option<(part_5::SoilProfile, Vec<&SoilLayer>)> {
    let rows: Vec<&SoilLayer> = document.soil_layers.iter().filter(|l| l.foundation_id == f.id).collect();
    if rows.is_empty() {
        return None;
    }
    let layers = rows.iter().map(|l| part_5::ProfileLayer { thickness_m: l.thickness_m, unit_weight_n_m3: l.unit_weight_n_m3, shear_wave_velocity_m_s: l.shear_wave_velocity_m_s, n1_60: l.n1_60, fines_percent: l.fines_percent }).collect();
    Some((part_5::SoilProfile { layers, water_depth_m: f.liq_water_depth_m, magnitude: f.liq_magnitude, a_g_s_ms2: a_g_s, csr_factor: ndp_number("ndp.en1998.en5.liquefaction.annex-b.csr-factor", annex) }, rows))
}

const PROFILE_IDS: [&str; 2] = ["ndp.en1998.en5.liquefaction.min-safety-factor", "ndp.en1998.en5.liquefaction.annex-b.csr-factor"];

/// 🪨️ Checks a multi-layer soil profile of a foundation: the Annex B liquefaction factor of every layer with its own (N1)60 and fines content over the integrated stresses (EN 1998-5 4.1.4 and Annex B), the governing layer of the profile, and the site response (v_s,30, ground type by EN 1998-1 Table 3.1, site period).
pub fn check_soil_profile(report: &mut CheckReport, annex: AnnexChoice, document: &En1998Snapshot, f: &Foundation, gamma_i: f64, a_g: f64, s: f64) {
    let a_g_s = gamma_i * a_g * s;
    let Some((profile, rows)) = soil_profile(annex, document, f, a_g_s) else { return };
    let required_f = ndp_number("ndp.en1998.en5.liquefaction.min-safety-factor", annex);
    let path = id_sel("foundations", &f.id);
    let assessable = f.liq_magnitude > 0.0 && profile.layers.iter().all(|l| l.unit_weight_n_m3 > 0.0 && l.thickness_m > 0.0);
    if assessable {
        for (index, row) in rows.iter().enumerate().filter(|(i, _)| profile.layers[*i].n1_60 > 0.0) {
            let (z, f_l) = profile.layer_governing(index);
            let (total, effective) = profile.stresses(z);
            let layer_path = format!("soilLayers[id={}]", row.id);
            let mut check = CheckResult::assess(
                format!("en1998.5.{}.layer.{}.liquefaction", f.id, row.id),
                "DIN EN 1998-5",
                ClauseId::new("EN 1998-5", "5", "4.1.4"),
                SubjectRef::new(&row.id, format!("{layer_path}.n160"), lc(&row.id, &row.id)),
                lc(&format!("Liquefaction safety factor of layer {} (Annex B)", row.id), &format!("Verflüssigungssicherheit der Schicht {} (Anhang B)", row.id)),
            )
            .utilization(q_dim(required_f), floor_limit(q_dim(f_l.min(1e9))))
            .annex(annex)
            .explanation(lc(
                &format!("F_L = {f_l:.3} at {z:.1} m (required {required_f}); (N1)60 {:.1}, FC {:.0} %, CRR·MSF {:.4}, σ_v {total:.0} Pa, σ'_v {effective:.0} Pa, a_g S {a_g_s:.3} m/s²", row.n1_60, row.fines_percent, profile.resistance(index)),
                &format!("F_L = {f_l:.3} in {z:.1} m (erforderlich {required_f}); (N1)60 {:.1}, FC {:.0} %, CRR·MSF {:.4}, σ_v {total:.0} Pa, σ'_v {effective:.0} Pa, a_g S {a_g_s:.3} m/s²", row.n1_60, row.fines_percent, profile.resistance(index)),
            ));
            if f_l < required_f {
                let improved = |n: f64| {
                    let mut layers = profile.layers.clone();
                    layers[index].n1_60 = n;
                    part_5::SoilProfile { layers, ..profile.clone() }.layer_governing(index).1
                };
                let (mut low, mut high) = (row.n1_60, 60.0);
                if improved(high) >= required_f {
                    for _ in 0..48 {
                        let mid = 0.5 * (low + high);
                        if improved(mid) >= required_f { high = mid } else { low = mid }
                    }
                }
                check = check.remedy(Remedy::at_least(
                    SubjectRef::new(&row.id, format!("{layer_path}.n160"), lc("Blow count (N1)60 of the layer", "Schlagzahl (N1)60 der Schicht")),
                    q_dim(row.n1_60),
                    q_dim(high),
                    lc("Densify the layer (ground improvement) until the blow count reaches the required value; where 60 does not suffice, replace the layer or lower the ground water.", "Schicht verdichten (Baugrundverbesserung), bis die Schlagzahl den erforderlichen Wert erreicht; reicht 60 nicht, Schicht ersetzen oder Grundwasser absenken."),
                ));
                let drained = |w: f64| part_5::SoilProfile { water_depth_m: w, ..profile.clone() }.layer_governing(index).1;
                let bottom = profile.bottoms()[index];
                if drained(bottom) >= required_f {
                    let (mut low, mut high) = (f.liq_water_depth_m, bottom);
                    for _ in 0..48 {
                        let mid = 0.5 * (low + high);
                        if drained(mid) >= required_f { high = mid } else { low = mid }
                    }
                    check = check.remedy(Remedy::at_least(
                        SubjectRef::new(&f.id, format!("{path}.liqWaterDepthM"), lc("Groundwater depth", "Grundwassertiefe")),
                        q_len(f.liq_water_depth_m),
                        q_len(high),
                        lc("Lower the ground water table (drainage) below the layer.", "Grundwasserspiegel unter die Schicht absenken (Dränage)."),
                    ));
                }
            }
            report.push(check.ndps(&PROFILE_IDS).build());
        }
        if let Some((index, z, f_l)) = profile.governing().filter(|g| g.2.is_finite()) {
            report.push(
                CheckResult::assess(format!("en1998.5.{}.profile.liquefaction", f.id), "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "4.1.4"), SubjectRef::new(&f.id, format!("{path}.liqWaterDepthM"), lc(&f.id, &f.id)), lc("Governing liquefaction factor of the soil profile", "Maßgebender Verflüssigungsfaktor des Bodenprofils"))
                    .informative(q_dim(f_l.min(1e9)))
                    .annex(annex)
                    .explanation(lc(
                        &format!("F_L = {f_l:.3} at {z:.1} m in layer {} of {} layers over {:.1} m; required {required_f}", rows[index].id, rows.len(), profile.depth_m()),
                        &format!("F_L = {f_l:.3} in {z:.1} m in Schicht {} von {} Schichten über {:.1} m; erforderlich {required_f}", rows[index].id, rows.len(), profile.depth_m()),
                    ))
                    .ndps(&PROFILE_IDS)
                    .build(),
            );
        }
    }
    let v_s30 = profile.equivalent_velocity_m_s(30.0);
    if v_s30 <= 0.0 {
        return;
    }
    let derived = part_1::ground_type_table_3_1(&part_1::ground_inputs_of_site(&document.site, v_s30));
    let class = derived.and_then(part_1::GroundType::spectrum_letter);
    let special = matches!(derived, Some(part_1::GroundType::S1 | part_1::GroundType::S2));
    let period = profile.site_period_s();
    let declared = Some(document.site.en_ground_type.letter());
    let mismatch = matches!(annex, AnnexChoice::En) && class.is_some() && declared != class;
    let name = derived.map_or("-", part_1::GroundType::name);
    let mut check = CheckResult::assess(format!("en1998.5.{}.profile.siteResponse", f.id), "DIN EN 1998-1", ClauseId::new("EN 1998-1", "1", "3.1.2"), SubjectRef::new("", "site.enGroundType", lc("Ground type", "Baugrundklasse")), lc("Site response of the soil profile: v_s,30, ground type and site period", "Standortantwort des Bodenprofils: v_s,30, Baugrundklasse und Standortperiode"))
        .informative(Quantity::new(QuantityKind::Time, period))
        .annex(annex)
        .ndps(&part_1::GROUND_NDP_IDS)
        .explanation(lc(
            &format!("v_s,30 = {v_s30:.0} m/s (profile of {:.1} m{}), ground type {name} by Table 3.1 with the N_SPT, c_u, soft clay, liquefiable and alluvium inputs of the site, site period T_0 = 4 Σ h/v_s = {period:.3} s over {:.1} m{}{}", profile.depth_m(), if profile.depth_m() < 30.0 { ", continued with the last layer" } else { "" }, profile.depth_m(), if mismatch { format!("; the site declares ground type {}", declared.unwrap_or('-')) } else { String::new() }, if special { "; ground type S1 or S2 needs a special study (EN 1998-1 3.1.2(4)), no spectrum letter" } else { "" }),
            &format!("v_s,30 = {v_s30:.0} m/s (Profil von {:.1} m{}), Baugrundklasse {name} nach Tabelle 3.1 mit den Eingaben N_SPT, c_u, weiche Tonschicht, verflüssigungsfähige Böden und Alluvion des Standorts, Standortperiode T_0 = 4 Σ h/v_s = {period:.3} s über {:.1} m{}{}", profile.depth_m(), if profile.depth_m() < 30.0 { ", mit der letzten Schicht fortgesetzt" } else { "" }, profile.depth_m(), if mismatch { format!("; der Standort deklariert die Klasse {}", declared.unwrap_or('-')) } else { String::new() }, if special { "; Baugrundklasse S1 oder S2 erfordert eine Sonderuntersuchung (EN 1998-1 3.1.2(4)), kein Spektrumbuchstabe" } else { "" }),
        ));
    if special {
        check = check.status(CheckStatus::Warning);
    }
    if mismatch {
        check = check.status(CheckStatus::Fail).remedy(Remedy::one_of(
            SubjectRef::new("", "site.enGroundType", lc("Ground type", "Baugrundklasse")),
            vec![class.map_or_else(String::new, |c| c.to_string())],
            lc("Set the site ground type to the class of the soil profile, or correct the layer shear wave velocities.", "Baugrundklasse des Standorts auf die Klasse des Bodenprofils setzen oder die Scherwellengeschwindigkeiten der Schichten korrigieren."),
        ));
    }
    report.push(check.build());
}
