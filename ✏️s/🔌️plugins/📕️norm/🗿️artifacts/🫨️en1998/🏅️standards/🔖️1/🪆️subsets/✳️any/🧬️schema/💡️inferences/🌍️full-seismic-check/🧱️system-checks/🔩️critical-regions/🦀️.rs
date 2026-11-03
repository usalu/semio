//! 🔩 Critical region checks of the ductile RC beams and columns of one building (EN 1998-1 5.4.3 DCM, 5.5.3 DCH): critical region length, hoop diameter and spacing, restrained bar spacing, the confinement α·ω_wd and the total longitudinal ratio ρ_tot. See [`super`] and [`critical_region`].

use super::*;
use semio_s_artifact_norm_en1998_model::{DuctilityClass, En1998CriticalRegion, StructuralMaterial};
use semio_s_artifact_norm_en1998_schema::critical_region::{self, Detailing};
use semio_s_plugin_norm_compliance::{drain_ndp_reads, CheckBuilder, CheckStatus, QuantityKind};

/// 🔩 Largest hoop spacing s ≤ `spacing` for which `ok` holds, found by bisection in [1 mm, `spacing`]; `None` when even 1 mm does not reach it. `ok` must be monotone: true for small spacings.
fn largest_spacing(spacing: f64, ok: impl Fn(f64) -> bool) -> Option<f64> {
    if !ok(0.001) {
        return None;
    }
    let (mut low, mut high) = (0.001, spacing);
    for _ in 0..48 {
        let mid = 0.5 * (low + high);
        if ok(mid) { low = mid } else { high = mid }
    }
    Some(low)
}

fn area(m2: f64) -> Quantity {
    Quantity::new(QuantityKind::Area, m2)
}

fn clause_of(d: &Detailing, class: DuctilityClass) -> &'static str {
    match (d.column, class) {
        (true, DuctilityClass::Dch) => "5.5.3.2.2",
        (true, _) => "5.4.3.2.2",
        (false, DuctilityClass::Dch) => "5.5.3.1.3",
        (false, _) => "5.4.3.1.2",
    }
}

/// 🔩 Checks every critical region of the building against the strictest ductility class of its systems; a region of a missing, non-RC or wall member is reported as a failed reference, a region of a DCL system is not checked.
pub fn check_critical_regions(document: &En1998Snapshot, annex: AnnexChoice, report: &mut CheckReport, building: &semio_s_artifact_norm_en1998_model::En1998Building, bid: &str, systems: &[semio_s_artifact_norm_en1998_model::En1998System], t1: f64, tc: f64) {
    let Some(system) = critical_region::governing_system(systems) else { return };
    let class = system.ductility_class;
    for r in document.critical_regions.iter().filter(|r| r.building_id == bid) {
        let path = id_sel("criticalRegions", &r.id);
        let subject = |field: &str| SubjectRef::new(bid, format!("{path}.{field}"), lc(&r.id, &r.id));
        let id = |name: &str| format!("en1998.1.{bid}.region.{}.{name}", r.id);
        let Some(member) = building.members.iter().find(|m| m.id == r.member_id && m.material == StructuralMaterial::Rc) else {
            report.push(
                CheckResult::assess(id("member"), "DIN EN 1998-1", ClauseId::new("EN 1998-1", "1", "5.4.3"), subject("memberId"), lc("Critical region refers to an RC member (not checked otherwise)", "Kritischer Bereich verweist auf ein Stahlbetonbauteil (sonst nicht geprüft)"))
                    .informative(q_dim(0.0))
                    .annex(annex)
                    .explanation(lc(&format!("Member {} is not an RC member of building {bid}", r.member_id), &format!("Bauteil {} ist kein Stahlbetonbauteil des Gebäudes {bid}", r.member_id)))
                    .build(),
            );
            continue;
        };
        let _ = drain_ndp_reads();
        let Some(d) = critical_region::detail(r, member.role, class, system.q0, t1, tc, annex) else { continue };
        let ndps = drain_ndp_reads();
        let clause = clause_of(&d, class);
        let start = |name: &str, title: LocalizedCopy, field: &str| -> CheckBuilder {
            CheckResult::assess(id(name), "DIN EN 1998-1", ClauseId::new("EN 1998-1", "1", clause), subject(field), title).annex(annex).ndps(&ndps)
        };
        let mut length = start("length", lc("Critical region length l_cr", "Länge des kritischen Bereichs l_cr"), "hoopedLengthM")
            .minimum(q_len(r.hooped_length_m), q_len(d.critical_length_m))
            .explanation(lc(
                &format!("hoops over {:.3} m, l_cr = {:.3} m{}", r.hooped_length_m, d.critical_length_m, if d.whole_height { " (whole clear length, l_cl/h_c below the limit)" } else { "" }),
                &format!("Bügel auf {:.3} m, l_cr = {:.3} m{}", r.hooped_length_m, d.critical_length_m, if d.whole_height { " (ganze lichte Länge, l_cl/h_c unter der Grenze)" } else { "" }),
            ));
        if r.hooped_length_m + 1e-12 < d.critical_length_m {
            length = length.remedy(Remedy::at_least(subject("hoopedLengthM"), q_len(r.hooped_length_m), q_len(d.critical_length_m), lc(&format!("Provide the hoops over at least l_cr = {:.3} m.", d.critical_length_m), &format!("Bügel auf mindestens l_cr = {:.3} m anordnen.", d.critical_length_m))));
        }
        report.push(length.build());
        let mut diameter = start("hoopDiameter", lc("Hoop diameter d_bw", "Bügeldurchmesser d_bw"), "hoopDiameterM")
            .minimum(q_len(r.hoop_diameter_m), q_len(d.hoop_diameter_min_m))
            .explanation(lc(&format!("d_bw = {:.1} mm, required at least {:.1} mm", r.hoop_diameter_m * 1000.0, d.hoop_diameter_min_m * 1000.0), &format!("d_bw = {:.1} mm, erforderlich mindestens {:.1} mm", r.hoop_diameter_m * 1000.0, d.hoop_diameter_min_m * 1000.0)));
        if r.hoop_diameter_m + 1e-12 < d.hoop_diameter_min_m {
            diameter = diameter.remedy(Remedy::at_least(subject("hoopDiameterM"), q_len(r.hoop_diameter_m), q_len(d.hoop_diameter_min_m), lc(&format!("Use hoops of at least {:.1} mm.", d.hoop_diameter_min_m * 1000.0), &format!("Bügel von mindestens {:.1} mm verwenden.", d.hoop_diameter_min_m * 1000.0))));
        }
        report.push(diameter.build());
        if r.hoop_spacing_m > 0.0 {
            let mut spacing = start("hoopSpacing", lc("Hoop spacing s", "Bügelabstand s"), "hoopSpacingM")
                .utilization(q_len(r.hoop_spacing_m), floor_limit(q_len(d.hoop_spacing_max_m)))
                .explanation(lc(&format!("s = {:.0} mm, allowed at most {:.0} mm", r.hoop_spacing_m * 1000.0, d.hoop_spacing_max_m * 1000.0), &format!("s = {:.0} mm, zulässig höchstens {:.0} mm", r.hoop_spacing_m * 1000.0, d.hoop_spacing_max_m * 1000.0)));
            if r.hoop_spacing_m > d.hoop_spacing_max_m + 1e-12 {
                spacing = spacing.remedy(Remedy::at_most(subject("hoopSpacingM"), q_len(r.hoop_spacing_m), q_len(d.hoop_spacing_max_m), lc(&format!("Reduce the hoop spacing to at most {:.0} mm.", d.hoop_spacing_max_m * 1000.0), &format!("Bügelabstand auf höchstens {:.0} mm verringern.", d.hoop_spacing_max_m * 1000.0))));
            }
            report.push(spacing.build());
        }
        if r.restrained_bar_count > 0.0 {
            let mut bars = start("barSpacing", lc("Spacing of restrained longitudinal bars", "Abstand der gehaltenen Längsstäbe"), "restrainedBarSpacingM")
                .utilization(q_len(r.restrained_bar_spacing_m), floor_limit(q_len(d.bar_spacing_max_m)))
                .explanation(lc(&format!("b_i = {:.0} mm, allowed at most {:.0} mm", r.restrained_bar_spacing_m * 1000.0, d.bar_spacing_max_m * 1000.0), &format!("b_i = {:.0} mm, zulässig höchstens {:.0} mm", r.restrained_bar_spacing_m * 1000.0, d.bar_spacing_max_m * 1000.0)));
            if r.restrained_bar_spacing_m > d.bar_spacing_max_m + 1e-12 {
                bars = bars.remedy(Remedy::at_most(subject("restrainedBarSpacingM"), q_len(r.restrained_bar_spacing_m), q_len(d.bar_spacing_max_m), lc(&format!("Engage the longitudinal bars at most {:.0} mm apart with hoops or cross-ties.", d.bar_spacing_max_m * 1000.0), &format!("Längsstäbe im Abstand von höchstens {:.0} mm durch Bügel oder Zwischenbügel halten.", d.bar_spacing_max_m * 1000.0))));
            }
            report.push(bars.build());
        }
        if d.nu_d.is_finite() && d.nu_d_max.is_finite() && r.axial_force_n != 0.0 {
            let mut axial = start("nuD", lc("Normalised axial force ν_d", "Bezogene Normalkraft ν_d"), "axialForceN")
                .utilization(q_dim(d.nu_d), floor_limit(q_dim(d.nu_d_max)))
                .explanation(lc(&format!("ν_d = N_Ed/(A_c f_cd) = {:.3}, allowed at most {:.2}", d.nu_d, d.nu_d_max), &format!("ν_d = N_Ed/(A_c f_cd) = {:.3}, zulässig höchstens {:.2}", d.nu_d, d.nu_d_max)));
            if d.nu_d > d.nu_d_max + 1e-12 {
                let n = d.nu_d_max * r.section_width_m * r.section_depth_m * r.concrete_design_strength_mpa * 1.0e6;
                axial = axial.remedy(Remedy::at_most(subject("axialForceN"), q_force(r.axial_force_n), q_force(n), lc(&format!("Reduce the design axial force to at most {:.0} kN or enlarge the section.", n / 1000.0), &format!("Bemessungs-Normalkraft auf höchstens {:.0} kN senken oder den Querschnitt vergrößern.", n / 1000.0))));
            }
            report.push(axial.build());
        }
        if d.omega_wd.is_finite() && d.confinement_required.is_finite() {
            let provided = d.alpha * d.omega_wd;
            let reach = |s: f64| {
                let probe = En1998CriticalRegion { hoop_spacing_m: s, ..r.clone() };
                critical_region::alpha_n(probe.restrained_bar_count, probe.restrained_bar_spacing_m, probe.core_width_m, probe.core_depth_m) * critical_region::alpha_s(s, probe.core_width_m, probe.core_depth_m) * critical_region::omega_wd(&probe)
            };
            let mut confinement = start("confinement", lc("Confinement α·ω_wd of the critical region", "Umschnürung α·ω_wd des kritischen Bereichs"), "hoopSpacingM")
                .minimum(q_dim(provided), q_dim(d.confinement_required))
                .explanation(lc(
                    &format!("α·ω_wd = {:.3}·{:.3} = {provided:.3}, required ≥ 30·μ_φ·ν_d·ε_sy,d·b_c/b_0 − 0.035 = {:.3} (μ_φ {:.2}, ν_d {:.3}, ε_sy,d {:.5}; α_n {:.3}, α_s {:.3})", d.alpha, d.omega_wd, d.confinement_required, d.mu_phi, d.nu_d, d.eps_sy_d, d.alpha_n, d.alpha_s),
                    &format!("α·ω_wd = {:.3}·{:.3} = {provided:.3}, erforderlich ≥ 30·μ_φ·ν_d·ε_sy,d·b_c/b_0 − 0,035 = {:.3} (μ_φ {:.2}, ν_d {:.3}, ε_sy,d {:.5}; α_n {:.3}, α_s {:.3})", d.alpha, d.omega_wd, d.confinement_required, d.mu_phi, d.nu_d, d.eps_sy_d, d.alpha_n, d.alpha_s),
                ));
            if provided + 1e-12 < d.confinement_required {
                confinement = match largest_spacing(r.hoop_spacing_m, |s| reach(s) >= d.confinement_required) {
                    Some(s) => confinement.remedy(Remedy::at_most(subject("hoopSpacingM"), q_len(r.hoop_spacing_m), q_len(s), lc(&format!("Reduce the hoop spacing to at most {:.0} mm to reach the required confinement.", s * 1000.0), &format!("Bügelabstand auf höchstens {:.0} mm verringern, um die erforderliche Umschnürung zu erreichen.", s * 1000.0)))),
                    None => {
                        let bar = r.hoop_diameter_m * (d.confinement_required / provided).sqrt();
                        confinement.remedy(Remedy::at_least(subject("hoopDiameterM"), q_len(r.hoop_diameter_m), q_len(bar), lc(&format!("Use hoops of at least {:.1} mm to reach the required confinement (ω_wd grows with d_bw²).", bar * 1000.0), &format!("Bügel von mindestens {:.1} mm verwenden, um die erforderliche Umschnürung zu erreichen (ω_wd wächst mit d_bw²).", bar * 1000.0))))
                    }
                };
            }
            report.push(confinement.build());
            if d.omega_wd_min > 0.0 {
                let mut minimum = start("omegaWdMin", lc("Mechanical volumetric ratio ω_wd of the hoops", "Mechanischer Bügelbewehrungsgrad ω_wd"), "hoopSpacingM")
                    .minimum(q_dim(d.omega_wd), q_dim(d.omega_wd_min))
                    .explanation(lc(&format!("ω_wd = {:.3}, required ≥ {:.3}", d.omega_wd, d.omega_wd_min), &format!("ω_wd = {:.3}, erforderlich ≥ {:.3}", d.omega_wd, d.omega_wd_min)));
                if d.omega_wd + 1e-12 < d.omega_wd_min {
                    let s = r.hoop_spacing_m * d.omega_wd / d.omega_wd_min;
                    minimum = minimum.remedy(Remedy::at_most(subject("hoopSpacingM"), q_len(r.hoop_spacing_m), q_len(s), lc(&format!("Reduce the hoop spacing to at most {:.0} mm.", s * 1000.0), &format!("Bügelabstand auf höchstens {:.0} mm verringern.", s * 1000.0))));
                }
                report.push(minimum.build());
            }
        }
        if d.rho_tot.is_finite() && r.longitudinal_steel_area_m2 > 0.0 {
            let gross = r.section_width_m * r.section_depth_m;
            let mut rho = start("rhoTot", lc("Total longitudinal reinforcement ratio ρ_tot", "Gesamter Längsbewehrungsgrad ρ_tot"), "longitudinalSteelAreaM2")
                .utilization(q_dim(d.rho_tot), q_dim(if d.rho_tot < d.rho_min { d.rho_min } else { d.rho_max }))
                .explanation(lc(&format!("ρ_tot = A_sl/(b_c h_c) = {:.4} over all longitudinal bars, limits [{:.4}, {:.4}]", d.rho_tot, d.rho_min, d.rho_max), &format!("ρ_tot = A_sl/(b_c h_c) = {:.4} über alle Längsstäbe, Grenzen [{:.4}, {:.4}]", d.rho_tot, d.rho_min, d.rho_max)));
            if d.rho_tot + 1e-12 < d.rho_min {
                rho = rho.status(CheckStatus::Fail).remedy(Remedy::at_least(subject("longitudinalSteelAreaM2"), area(r.longitudinal_steel_area_m2), area(d.rho_min * gross), lc(&format!("Increase the longitudinal steel area to at least {:.5} m².", d.rho_min * gross), &format!("Längsbewehrungsfläche auf mindestens {:.5} m² erhöhen.", d.rho_min * gross))));
            } else if d.rho_tot > d.rho_max + 1e-12 {
                rho = rho.status(CheckStatus::Fail).remedy(Remedy::at_most(subject("longitudinalSteelAreaM2"), area(r.longitudinal_steel_area_m2), area(d.rho_max * gross), lc(&format!("Reduce the longitudinal steel area to at most {:.5} m².", d.rho_max * gross), &format!("Längsbewehrungsfläche auf höchstens {:.5} m² senken.", d.rho_max * gross))));
            }
            report.push(rho.build());
        }
    }
}
