//! 🛞 Seismic isolation checks of EN 1998-2 chapter 7 (bridges) and EN 1998-1 chapter 10 (buildings) by the equivalent linear method: the design displacement d_cd of the bilinear isolation system, the isolator displacement capacity, the effective period against the fixed-base period and the period limit, the equivalent damping and the isolation shear. The limits are unverified registry rows. See [`schema::isolation`].

use super::*;
use compliance::{ndp_number, CheckStatus};
use schema::isolation::{Bilinear, Spectrum, State};

type System = model::En1998IsolationSystem;

/// 🛞 The structure that rests on an isolation system: its seismic mass, its fixed-base period (0 while unknown) and, for a bridge, the shear resistance of the substructure (0 for a building, whose isolation shear is reported only).
#[derive(Clone, Debug, PartialEq)]
pub struct Supported {
    pub is_bridge: bool,
    pub id: String,
    pub mass_kg: f64,
    pub fixed_period_s: f64,
    pub shear_rd_n: f64,
}

fn q_stiff(n_per_m: f64) -> Quantity { Quantity::new(QuantityKind::Dimensionless, n_per_m) }

fn q_time(s: f64) -> Quantity { Quantity::new(QuantityKind::Time, s) }

fn smallest(low: f64, high: f64, ok: &dyn Fn(f64) -> bool) -> Option<f64> {
    if !ok(high) {
        return None;
    }
    let (mut low, mut high) = (low, high);
    for _ in 0..60 {
        let mid = 0.5 * (low + high);
        if ok(mid) { high = mid } else { low = mid }
    }
    Some(high)
}

fn largest(low: f64, high: f64, ok: &dyn Fn(f64) -> bool) -> Option<f64> {
    if !ok(low) {
        return None;
    }
    let (mut low, mut high) = (low, high);
    for _ in 0..60 {
        let mid = 0.5 * (low + high);
        if ok(mid) { low = mid } else { high = mid }
    }
    Some(low)
}

/// 🛞 Isolation checks of one system on the structure it carries; `law` is invalid or the design equilibrium is missing the checks say so with the stiffness that restores it.
pub fn check_system(report: &mut CheckReport, annex: AnnexChoice, system: &System, supported: &Supported, spectrum: &Spectrum) {
    let path = id_sel("isolationSystems", &system.id);
    let (standard, clause, family) = if supported.is_bridge { ("DIN EN 1998-2", ClauseId::new("EN 1998-2", "2", "7"), "en2") } else { ("DIN EN 1998-1", ClauseId::new("EN 1998-1", "1", "10"), "en1") };
    let subject = |field: &str, label: LocalizedCopy| SubjectRef::new(&system.id, format!("{path}.{field}"), label);
    let id = |tail: &str| format!("en1998.isolation.{}.{tail}", system.id);
    let law = Bilinear { strength_n: system.strength_n, post_yield_n_per_m: system.stiffness_n_per_m, yield_displacement_m: system.yield_displacement_m };
    if !law.is_valid() {
        let mut check = CheckResult::assess(id("law"), standard, clause.clone(), subject("stiffnessNPerM", lc(&system.id, &system.id)), lc("Bilinear hysteresis of the isolation system", "Bilineare Hysterese des Isolationssystems"))
            .status(CheckStatus::Fail)
            .annex(annex)
            .explanation(lc(
                &format!("Q_d = {:.0} N, K_d = {:.3e} N/m and d_y = {:.4} m do not describe a hysteresis loop: Q_d ≥ 0, K_d > 0 and d_y > 0 are required", system.strength_n, system.stiffness_n_per_m, system.yield_displacement_m),
                &format!("Q_d = {:.0} N, K_d = {:.3e} N/m und d_y = {:.4} m beschreiben keine Hysterese: Q_d ≥ 0, K_d > 0 und d_y > 0 sind erforderlich", system.strength_n, system.stiffness_n_per_m, system.yield_displacement_m),
            ));
        if !(system.stiffness_n_per_m > 0.0) {
            check = check.remedy(Remedy::at_least(subject("stiffnessNPerM", lc("Post-yield stiffness K_d", "Steifigkeit nach dem Fließen K_d")), q_stiff(system.stiffness_n_per_m), q_stiff(f64::EPSILON), lc("Enter the positive post-yield stiffness K_d of the isolator units.", "Positive Steifigkeit K_d der Isolatoren nach dem Fließen eingeben.")));
        }
        if !(system.yield_displacement_m > 0.0) {
            check = check.remedy(Remedy::at_least(subject("yieldDisplacementM", lc("Yield displacement d_y", "Fließverschiebung d_y")), q_len(system.yield_displacement_m), q_len(f64::EPSILON), lc("Enter the positive yield displacement d_y of the isolator units.", "Positive Fließverschiebung d_y der Isolatoren eingeben.")));
        }
        if !(system.strength_n >= 0.0) {
            check = check.remedy(Remedy::at_least(subject("strengthN", lc("Characteristic strength Q_d", "Charakteristische Festigkeit Q_d")), q_force(system.strength_n), q_force(0.0), lc("Enter a characteristic strength Q_d of at least zero.", "Charakteristische Festigkeit Q_d von mindestens null eingeben.")));
        }
        report.push(check.build());
        return;
    }
    if !(supported.mass_kg > 0.0) {
        report.push(CheckResult::assess(id("design"), standard, clause, subject("stiffnessNPerM", lc(&system.id, &system.id)), lc("Design displacement of the isolation system", "Bemessungsverschiebung des Isolationssystems")).not_applicable(lc(&format!("{} carries no seismic mass, the equivalent linear analysis needs one.", supported.id), &format!("{} hat keine seismische Masse, die äquivalent lineare Berechnung braucht eine.", supported.id))).annex(annex).build());
        return;
    }
    let with = |strength: f64, stiffness: f64| Bilinear { strength_n: strength, post_yield_n_per_m: stiffness, ..law }.design(supported.mass_kg, spectrum);
    let Some(state) = law.design(supported.mass_kg, spectrum) else {
        let mut check = CheckResult::assess(id("design"), standard, clause, subject("stiffnessNPerM", lc(&system.id, &system.id)), lc("Design displacement of the isolation system", "Bemessungsverschiebung des Isolationssystems"))
            .status(CheckStatus::Fail)
            .annex(annex)
            .explanation(lc(
                &format!("The force K_eff d of the system (Q_d = {:.0} N, K_d = {:.3e} N/m) does not balance the spectral force M S_e(T_eff, ξ_eff) with M = {:.0} kg at any displacement up to {} m: the system is too soft", system.strength_n, system.stiffness_n_per_m, supported.mass_kg, schema::isolation::SEARCH_LIMIT_M),
                &format!("Die Kraft K_eff d des Systems (Q_d = {:.0} N, K_d = {:.3e} N/m) hält der Spektralkraft M S_e(T_eff, ξ_eff) mit M = {:.0} kg bei keiner Verschiebung bis {} m das Gleichgewicht: das System ist zu weich", system.strength_n, system.stiffness_n_per_m, supported.mass_kg, schema::isolation::SEARCH_LIMIT_M),
            ));
        if let Some(required) = smallest(system.stiffness_n_per_m, system.stiffness_n_per_m * 1.0e6, &|k| with(system.strength_n, k).is_some()) {
            check = check.remedy(Remedy::at_least(subject("stiffnessNPerM", lc("Post-yield stiffness K_d", "Steifigkeit nach dem Fließen K_d")), q_stiff(system.stiffness_n_per_m), q_stiff(required), lc("Stiffen the isolator units until the system balances the spectral force.", "Isolatoren versteifen, bis das System der Spektralkraft das Gleichgewicht hält.")));
        }
        report.push(check.build());
        return;
    };
    let State { displacement_m: d_cd, force_n, effective_stiffness_n_per_m: k_eff, period_s: t_eff, damping: xi_eff, eta, .. } = state;
    let mut displacement = CheckResult::assess(id("displacement"), standard, clause.clone(), subject("capacityM", lc(&system.id, &system.id)), lc("Design displacement d_cd against the isolator displacement capacity", "Bemessungsverschiebung d_cd gegen die Verschiebungskapazität der Isolatoren"))
        .utilization(q_len(d_cd), floor_limit(q_len(system.capacity_m)))
        .annex(annex)
        .explanation(lc(
            &format!("d_cd = {d_cd:.4} m from K_eff d = M S_e(T_eff, ξ_eff) with M = {:.0} kg: K_eff = {k_eff:.3e} N/m, T_eff = {t_eff:.3} s, ξ_eff = {:.1} %, η = {eta:.3}, force {force_n:.0} N; capacity d_max = {:.4} m", supported.mass_kg, 100.0 * xi_eff, system.capacity_m),
            &format!("d_cd = {d_cd:.4} m aus K_eff d = M S_e(T_eff, ξ_eff) mit M = {:.0} kg: K_eff = {k_eff:.3e} N/m, T_eff = {t_eff:.3} s, ξ_eff = {:.1} %, η = {eta:.3}, Kraft {force_n:.0} N; Kapazität d_max = {:.4} m", supported.mass_kg, 100.0 * xi_eff, system.capacity_m),
        ));
    if d_cd > system.capacity_m {
        displacement = displacement.remedy(Remedy::at_least(subject("capacityM", lc("Displacement capacity d_max", "Verschiebungskapazität d_max")), q_len(system.capacity_m), q_len(d_cd), lc("Choose isolator units with a displacement capacity of at least d_cd.", "Isolatoren mit einer Verschiebungskapazität von mindestens d_cd wählen.")));
    }
    report.push(displacement.build());
    if supported.fixed_period_s > 0.0 {
        let min_ratio = ndp_number(&format!("ndp.en1998.{family}.isolation.min-period-ratio"), annex);
        let ratio = t_eff / supported.fixed_period_s;
        let mut period = CheckResult::assess(id("period"), standard, clause.clone(), subject("stiffnessNPerM", lc(&system.id, &system.id)), lc("Effective period T_eff against the fixed-base period", "Effektive Periode T_eff gegen die Periode bei fester Einspannung"))
            .minimum(q_dim(ratio), q_dim(min_ratio))
            .annex(annex)
            .explanation(lc(
                &format!("T_eff = {t_eff:.3} s against T_fixed = {:.3} s gives {ratio:.2}, at least {min_ratio} is required", supported.fixed_period_s),
                &format!("T_eff = {t_eff:.3} s gegen T_fest = {:.3} s ergibt {ratio:.2}, mindestens {min_ratio} ist erforderlich", supported.fixed_period_s),
            ));
        if ratio < min_ratio {
            if let Some(allowed) = largest(system.stiffness_n_per_m * 1.0e-3, system.stiffness_n_per_m, &|k| with(system.strength_n, k).is_some_and(|s| s.period_s / supported.fixed_period_s >= min_ratio)) {
                period = period.remedy(Remedy::at_most(subject("stiffnessNPerM", lc("Post-yield stiffness K_d", "Steifigkeit nach dem Fließen K_d")), q_stiff(system.stiffness_n_per_m), q_stiff(allowed), lc("Soften the isolator units until T_eff reaches the required multiple of the fixed-base period.", "Isolatoren weicher machen, bis T_eff das geforderte Vielfache der Periode bei fester Einspannung erreicht.")));
            }
        }
        report.push(period.ndp_reads().build());
    }
    let max_period = ndp_number(&format!("ndp.en1998.{family}.isolation.max-period-s"), annex);
    let mut longest = CheckResult::assess(id("maxPeriod"), standard, clause.clone(), subject("stiffnessNPerM", lc(&system.id, &system.id)), lc("Effective period T_eff against the period limit", "Effektive Periode T_eff gegen die Periodengrenze"))
        .utilization(q_time(t_eff), floor_limit(q_time(max_period)))
        .annex(annex)
        .explanation(lc(&format!("T_eff = {t_eff:.3} s against the limit {max_period} s"), &format!("T_eff = {t_eff:.3} s gegen die Grenze {max_period} s")));
    if t_eff > max_period {
        if let Some(required) = smallest(system.stiffness_n_per_m, system.stiffness_n_per_m * 1.0e3, &|k| with(system.strength_n, k).is_some_and(|s| s.period_s <= max_period)) {
            longest = longest.remedy(Remedy::at_least(subject("stiffnessNPerM", lc("Post-yield stiffness K_d", "Steifigkeit nach dem Fließen K_d")), q_stiff(system.stiffness_n_per_m), q_stiff(required), lc("Stiffen the isolator units until T_eff stays within the period limit.", "Isolatoren versteifen, bis T_eff die Periodengrenze einhält.")));
        }
    }
    report.push(longest.ndp_reads().build());
    let max_damping = ndp_number(&format!("ndp.en1998.{family}.isolation.max-damping"), annex);
    let mut damping = CheckResult::assess(id("damping"), standard, clause.clone(), subject("strengthN", lc(&system.id, &system.id)), lc("Equivalent damping ξ_eff against the damping limit", "Äquivalente Dämpfung ξ_eff gegen die Dämpfungsgrenze"))
        .utilization(q_dim(xi_eff), floor_limit(q_dim(max_damping)))
        .annex(annex)
        .explanation(lc(
            &format!("ξ_eff = E_D/(2π K_eff d_cd²) = {:.0}/(2π {k_eff:.3e} {d_cd:.4}²) = {:.1} % against the limit {:.0} %", state.dissipated_energy_nm, 100.0 * xi_eff, 100.0 * max_damping),
            &format!("ξ_eff = E_D/(2π K_eff d_cd²) = {:.0}/(2π {k_eff:.3e} {d_cd:.4}²) = {:.1} % gegen die Grenze {:.0} %", state.dissipated_energy_nm, 100.0 * xi_eff, 100.0 * max_damping),
        ));
    if xi_eff > max_damping {
        if let Some(allowed) = largest(0.0, system.strength_n, &|q| with(q, system.stiffness_n_per_m).is_some_and(|s| s.damping <= max_damping)) {
            damping = damping.remedy(Remedy::at_most(subject("strengthN", lc("Characteristic strength Q_d", "Charakteristische Festigkeit Q_d")), q_force(system.strength_n), q_force(allowed), lc("Reduce the characteristic strength of the isolator units until ξ_eff stays within the limit.", "Charakteristische Festigkeit der Isolatoren verringern, bis ξ_eff die Grenze einhält.")));
        }
    }
    report.push(damping.ndp_reads().build());
    let shear_title = lc("Isolation shear K_eff d_cd", "Isolationsquerkraft K_eff d_cd");
    let shear_text = lc(&format!("V = K_eff d_cd = {k_eff:.3e}·{d_cd:.4} = {force_n:.0} N"), &format!("V = K_eff d_cd = {k_eff:.3e}·{d_cd:.4} = {force_n:.0} N"));
    if supported.is_bridge && supported.shear_rd_n > 0.0 {
        let target = SubjectRef::new(&supported.id, format!("{}.vRdN", id_sel("bridges", &supported.id)), lc("Bridge shear resistance V_Rd", "Brückenquerkrafttragfähigkeit V_Rd"));
        let mut shear = CheckResult::assess(id("shear"), standard, clause, subject("stiffnessNPerM", lc(&system.id, &system.id)), shear_title).utilization(q_force(force_n), floor_limit(q_force(supported.shear_rd_n))).annex(annex).explanation(shear_text);
        if force_n > supported.shear_rd_n {
            shear = shear.remedy(Remedy::at_least(target, q_force(supported.shear_rd_n), q_force(force_n), lc("Increase the shear resistance of the substructure to the isolation shear.", "Querkrafttragfähigkeit des Unterbaus auf die Isolationsquerkraft erhöhen.")));
        }
        report.push(shear.build());
    } else {
        report.push(CheckResult::assess(id("shear"), standard, clause, subject("stiffnessNPerM", lc(&system.id, &system.id)), shear_title).informative(q_force(force_n)).annex(annex).explanation(shear_text).build());
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
