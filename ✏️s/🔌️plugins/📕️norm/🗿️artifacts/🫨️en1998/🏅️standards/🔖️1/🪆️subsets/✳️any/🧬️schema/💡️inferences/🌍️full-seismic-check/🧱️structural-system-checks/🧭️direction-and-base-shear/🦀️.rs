//! 🧭️ Direction and base shear checks: the analysis direction and the seismic base shear of one structural system. See [`super`].

use super::*;

/// 🧭 Checks that the analysis direction of the system is known and unique.
pub(super) fn check_system_direction(
    annex: AnnexChoice,
    report: &mut CheckReport,
    building: &semio_s_artifact_norm_en1998_model::En1998Building,
    bid: &str,
    bpath: &str,
    system: &semio_s_artifact_norm_en1998_model::En1998System,
) {
    {
        let dup = building
            .systems
            .iter()
            .filter(|s| s.direction.to_ascii_lowercase() == system.direction.to_ascii_lowercase())
            .count()
            > 1;
        let known = matches!(system.direction.to_ascii_lowercase().as_str(), "x" | "y");
        let mut dch = CheckResult::assess(
            format!("en1998.1.{bid}.{}.direction", system.id),
            "DIN EN 1998-1",
            ClauseId::new("EN 1998-1", "1", "4.3.3.2.1"),
            SubjectRef::new(bid, format!("{}.systems[id={}].direction", bpath, system.id), lc(&system.id, &system.id)),
            lc("Horizontal analysis direction", "Horizontale Analyserichtung"),
        )
        .utilization(q_dim(if !known || dup { 1.5 } else { 0.5 }), q_dim(1.0))
        .annex(annex)
        .explanation(lc(
            &format!("direction={}, known={}, duplicate={}", system.direction, known, dup),
            &format!("Richtung={}, gültig={}, doppelt={}", system.direction, known, dup),
        ));
        if !known || dup {
            dch = dch.remedy(Remedy::exactly(
                SubjectRef::new(bid, format!("{}.systems[id={}].direction", bpath, system.id), lc("Direction", "Richtung")),
                q_dim(0.0),
                q_dim(1.0),
                lc(
                    "Assign unique orthogonal directions x and y to resisting systems.",
                    "Den Aussteifungssystemen eindeutige orthogonale Richtungen x und y zuweisen.",
                ),
            ));
        }
        report.push(dch.build());
    }
}

/// 🪨 Checks the behaviour factor, the design spectrum and the base shear of the system against its resistance.
pub(super) fn check_system_base_shear(
    annex: AnnexChoice,
    report: &mut CheckReport,
    gamma_i: f64,
    a_g: f64,
    s: f64,
    tb: f64,
    tc: f64,
    td: f64,
    building: &semio_s_artifact_norm_en1998_model::En1998Building,
    bid: &str,
    bpath: &str,
    total_mass: f64,
    t1: f64,
    system: &semio_s_artifact_norm_en1998_model::En1998System,
) -> (f64, f64) {
    let q = part_1::behaviour_factor(system.q0, system.alpha_u_over_alpha_1, system.k_w);
    let q_max = part_1::q_max_for_system(&system.system_type, &system.material, &system.ductility_class);
    let s_e = part_1::elastic_response_spectrum(a_g, s, tb, tc, td, t1);
    let s_d = part_1::design_spectrum_sd(s_e, gamma_i, q.min(q_max), a_g);
    let lambda = part_1::lambda_factor(t1, tc, building.storeys.len());
    let f_b = part_1::base_shear_n(s_d, total_mass, lambda);
    let spath = format!("{}.systems[id={}]", bpath, system.id);

    {
        let ok = q <= q_max + 1e-9;
        let mut c = CheckResult::assess(
            format!("en1998.1.{bid}.{}.qLimit", system.id),
            "DIN EN 1998-1",
            ClauseId::new("EN 1998-1", "1", "5.2.2.2"),
            SubjectRef::new(bid, format!("{spath}.q0"), lc(
        &format!("System {}", system.id),
        &format!("Nachweis: System {}", system.id),
    )),
            lc("Behaviour factor q within tabulated limit", "Verhaltensbeiwert q innerhalb Tabellengrenze"),
        )
        .utilization(q_dim(if ok { q.min(q_max) } else { q }), q_dim(q_max))
        .annex(annex)
        .explanation(lc(
            &format!("q={q:.2} from q0={:.2}·αu/α1·kw; q_max={q_max:.2} for {}/{}/{}", system.q0, system.system_type, system.material, system.ductility_class),
            &format!("q={q:.2} aus q0={:.2}·αu/α1·kw; q_max={q_max:.2} für {}/{}/{}", system.q0, system.system_type, system.material, system.ductility_class),
        ));
        if !ok {
            let req_q0 = (q_max / (system.alpha_u_over_alpha_1 * system.k_w).max(1e-9)).max(1.0);
            c = c.remedy(Remedy::at_most(
                SubjectRef::new(bid, format!("{spath}.q0"), lc("q₀", "q₀")),
                q_dim(system.q0),
                q_dim(req_q0),
                lc(&format!("Reduce q0 to at most {req_q0:.2} so q ≤ {q_max:.2}.",), &format!("q0 auf höchstens {req_q0:.2} senken, damit q ≤ {q_max:.2}.")),
            ));
        }
        report.push(c.build());
    }

    let mut shear = CheckResult::assess(
        format!("en1998.1.{bid}.{}.baseShear", system.id),
        "DIN EN 1998-1",
        ClauseId::new("EN 1998-1", "1", "4.3.3.2.2"),
        SubjectRef::new(bid, format!("{spath}.baseShearResistanceN"), lc(
        &format!("System {}", system.id),
        &format!("Nachweis: System {}", system.id),
    )),
        lc("Seismic base shear F_b ≤ V_Rd", "Erdbeben-Querkraft F_b ≤ V_Rd"),
    )
    .utilization(q_force(f_b), q_force(system.base_shear_resistance_n))
    .annex(annex)
    .explanation(lc(
        &format!("dir={}, Sd(T1={t1:.3}s)={s_d:.3} m/s², λ={lambda:.2}, Fb={:.0} N, VRd={:.0} N, q={q:.2}", system.direction, f_b, system.base_shear_resistance_n),
        &format!("Richtung={}, Sd(T1={t1:.3}s)={s_d:.3} m/s², λ={lambda:.2}, F_b={:.0} N, V_Rd={:.0} N, q={q:.2}", system.direction, f_b, system.base_shear_resistance_n),
    ));
    if f_b > system.base_shear_resistance_n {
        let req_v = f_b;
        let req_mass = (system.base_shear_resistance_n / (s_d * lambda)).max(0.0);
        shear = shear
            .remedy(Remedy::at_least(
                SubjectRef::new(bid, format!("{spath}.baseShearResistanceN"), lc("Base shear resistance", "Querkrafttragfähigkeit")),
                q_force(system.base_shear_resistance_n),
                q_force(req_v),
                lc(&format!("Increase V_Rd from {:.0} N to at least {:.0} N (or add walls).", system.base_shear_resistance_n, req_v), &format!("V_Rd von {:.0} N auf mindestens {:.0} N erhöhen (oder Wände ergänzen).", system.base_shear_resistance_n, req_v)),
            ))
            .remedy(Remedy::at_most(
                SubjectRef::new(bid, format!("{bpath}.storeys[id={}].permanentGkN", building.storeys.first().map(|s| s.id.as_str()).unwrap_or("s1")), lc("Permanent G_k", "Ständige Einwirkung G_k")),
                q_force(building.storeys.first().map(|s| s.permanent_gk_n).unwrap_or(0.0)),
                q_force((req_mass * 9.81 * 0.85).max(0.0)),
                lc(&format!("Reduce permanent G_k so seismic mass approaches {req_mass:.0} kg.",), &format!("Ständige G_k senken, damit seismische Masse Richtung {req_mass:.0} kg geht.")),
            ));
    }
    report.push(shear.build());
    (q, f_b)
}
