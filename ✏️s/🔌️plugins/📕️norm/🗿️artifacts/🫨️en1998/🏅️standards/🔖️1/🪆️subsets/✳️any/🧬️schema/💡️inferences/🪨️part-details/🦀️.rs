//! 🪨️ Part details: the liquefaction, pier, chord rotation, tower period and timber and masonry member checks whose constants are unverified NDPs of the registry. See [`super`].

use super::*;
use semio_s_plugin_norm_compliance::ndp_number;

type Member = semio_s_artifact_norm_en1998_model::En1998Member;

fn status_of(ok: bool) -> semio_s_plugin_norm_compliance::CheckStatus {
    if ok { semio_s_plugin_norm_compliance::CheckStatus::Pass } else { semio_s_plugin_norm_compliance::CheckStatus::Fail }
}

/// 🗼 Reports the fundamental period of a tower with a given stiffness EI and returns the period the design spectrum is read at.
pub fn tower_period(report: &mut CheckReport, annex: AnnexChoice, t: &semio_s_artifact_norm_en1998_model::En1998Tower, mass_kg: f64, plateau_period: f64) -> f64 {
    let fixed = part_6::uniform_cantilever_period_s(mass_kg, t.height_m, t.ei_nm2);
    let period = if t.base_rotational_stiffness_nm > 0.0 && fixed.is_finite() {
        semio_s_artifact_norm_en1998_schema::tower::modes(mass_kg, t.height_m, t.ei_nm2, t.base_rotational_stiffness_nm).and_then(|m| m.modes.first().map(|mode| mode.period_s)).unwrap_or(fixed)
    } else {
        fixed
    };
    if !period.is_finite() {
        return plateau_period;
    }
    report.push(
        CheckResult::assess(
            format!("en1998.6.{}.period", t.id),
            "DIN EN 1998-6",
            ClauseId::new("EN 1998-6", "6", "4.3"),
            SubjectRef::new(&t.id, format!("{}.eiNm2", id_sel("towers", &t.id)), lc(&t.id, &t.id)),
            lc("Fundamental period of the cantilever tower", "Grundperiode des Kragturms"),
        )
        .informative(Quantity::new(QuantityKind::Time, period))
        .annex(annex)
        .explanation(lc(
            &format!("T1 = 2π/(3.516·√(EI/(μ·H⁴))) = {period:.3} s with EI = {:.3e} N·m², M = {mass_kg:.0} kg, H = {:.1} m; the design spectrum is read at T1", t.ei_nm2, t.height_m),
            &format!("T1 = 2π/(3,516·√(EI/(μ·H⁴))) = {period:.3} s mit EI = {:.3e} N·m², M = {mass_kg:.0} kg, H = {:.1} m; das Bemessungsspektrum wird bei T1 abgelesen", t.ei_nm2, t.height_m),
        ))
        .build(),
    );
    period
}

/// 🔧 Design chord rotation capacities of an assessment: the direct inputs divided by CF and γ_el, or, while a direct input is 0 and the section is given, the Annex A expressions of the member with the strengths over CF.
pub struct ChordCapacity {
    pub yield_rad: f64,
    pub ultimate_rad: f64,
    pub member: Option<part_3::RcMember>,
    pub coefficients: part_3::AnnexA,
    pub cf: f64,
}

pub fn chord_capacity(annex: AnnexChoice, a: &semio_s_artifact_norm_en1998_model::En1998Assessment) -> ChordCapacity {
    let ls = a.limit_state.token();
    let cf = if part_3::limit_state_applies_cf(&ls) { part_3::confidence_factor(a.knowledge_level) } else { 1.0 };
    let coefficients = part_3::AnnexA::from_registry(annex);
    let part_3::DesignCapacities { yield_rad, ultimate_rad, member } = part_3::design_capacities(a, &coefficients, cf);
    ChordCapacity { yield_rad, ultimate_rad, member, coefficients, cf }
}

/// 🔧 Checks the chord rotation demand of an assessment against the capacity of the limit state, derived from the member section by EN 1998-3 Annex A while no direct capacity is given.
pub fn check_chord_rotation(report: &mut CheckReport, annex: AnnexChoice, a: &semio_s_artifact_norm_en1998_model::En1998Assessment) {
    let capacity = chord_capacity(annex, a);
    if a.chord_rotation_demand_rad <= 0.0 && capacity.member.is_none() && a.chord_rotation_yield_rad <= 0.0 && a.chord_rotation_ultimate_rad <= 0.0 {
        return;
    }
    let ls = a.limit_state.token();
    let sd_factor = ndp_number("ndp.en1998.en3.chord-rotation.sd-factor", annex);
    let Some(design) = part_3::chord_rotation_capacity_rad(&ls, capacity.yield_rad, capacity.ultimate_rad, sd_factor) else { return };
    let path = id_sel("assessments", &a.id);
    let dl = ls.starts_with("dl") || ls.starts_with("damage");
    let direct = if dl { a.chord_rotation_yield_rad > 0.0 } else { a.chord_rotation_ultimate_rad > 0.0 };
    let field = if direct { if dl { "chordRotationYieldRad" } else { "chordRotationUltimateRad" } } else { "chordRotationDemandRad" };
    let mut check = CheckResult::assess(
        format!("en1998.3.{}.chordRotation", a.id),
        "DIN EN 1998-3",
        ClauseId::new("EN 1998-3", "3", "2.3"),
        SubjectRef::new(&a.id, format!("{path}.{field}"), lc(&a.id, &a.id)),
        lc("Chord rotation demand against the capacity of the limit state", "Sehnendrehwinkel-Einwirkung gegen die Kapazität des Grenzzustands"),
    )
    .utilization(q_dim(a.chord_rotation_demand_rad), floor_limit(q_dim(design)))
    .annex(annex)
    .explanation(lc(
        &format!("θ_Ed = {:.5} rad against the design capacity {design:.5} rad (limit state {}, θ_y {:.5} rad, θ_um {:.5} rad, CF {:.2}, γ_el {:.2}, {})", a.chord_rotation_demand_rad, a.limit_state.token(), capacity.yield_rad, capacity.ultimate_rad, capacity.cf, a.gamma_el, if capacity.member.is_some() && !direct { "derived from the section by Annex A" } else { "direct input" }),
        &format!("θ_Ed = {:.5} rad gegen die Bemessungskapazität {design:.5} rad (Grenzzustand {}, θ_y {:.5} rad, θ_um {:.5} rad, CF {:.2}, γ_el {:.2}, {})", a.chord_rotation_demand_rad, a.limit_state.token(), capacity.yield_rad, capacity.ultimate_rad, capacity.cf, a.gamma_el, if capacity.member.is_some() && !direct { "aus dem Querschnitt nach Anhang A abgeleitet" } else { "direkte Eingabe" }),
    ));
    if a.chord_rotation_demand_rad > design {
        check = chord_rotation_remedies(check, a, &capacity, &path, &ls, sd_factor);
    }
    report.push(check.ndp("ndp.en1998.en3.chord-rotation.sd-factor").build());
    if let Some(member) = capacity.member {
        chord_rotation_derivation(report, annex, a, &capacity, &member, &path);
    }
}

fn chord_rotation_remedies(check: semio_s_plugin_norm_compliance::CheckBuilder, a: &semio_s_artifact_norm_en1998_model::En1998Assessment, capacity: &ChordCapacity, path: &str, ls: &str, sd_factor: f64) -> semio_s_plugin_norm_compliance::CheckBuilder {
    let dl = ls.starts_with("dl") || ls.starts_with("damage");
    let direct_given = if dl { a.chord_rotation_yield_rad > 0.0 } else { a.chord_rotation_ultimate_rad > 0.0 };
    let demand = a.chord_rotation_demand_rad;
    if let (Some(member), false, false) = (capacity.member, direct_given, dl) {
        let reach = |m: &part_3::RcMember| part_3::chord_rotation_capacity_rad(ls, capacity.yield_rad, part_3::chord_rotation_ultimate_rad(m, a.gamma_el, &capacity.coefficients), sd_factor).unwrap_or(0.0);
        let tighter = |spacing: f64| part_3::RcMember { s_h: spacing, ..member };
        if member.a_sx > 0.0 && member.s_h > 0.0 && reach(&tighter(0.02)) >= demand {
            let (mut high, mut low) = (member.s_h, 0.02);
            for _ in 0..48 {
                let mid = 0.5 * (high + low);
                if reach(&tighter(mid)) >= demand { low = mid } else { high = mid }
            }
            return check.remedy(Remedy::at_most(
                SubjectRef::new(&a.id, format!("{path}.stirrupSpacingM"), lc("Stirrup spacing s_h", "Bügelabstand s_h")),
                q_len(member.s_h),
                q_len(low),
                lc("Reduce the stirrup spacing in the plastic hinge region to raise the confinement and the ultimate chord rotation.", "Bügelabstand im Gelenkbereich verringern, um Umschnürung und Bruch-Sehnendrehwinkel zu erhöhen."),
            ));
        }
        let doubled = part_3::RcMember { a_s2: 2.0 * member.a_s1, ..member };
        if reach(&doubled) >= demand {
            let (mut low, mut high) = (member.a_s2, doubled.a_s2);
            for _ in 0..48 {
                let mid = 0.5 * (low + high);
                if reach(&part_3::RcMember { a_s2: mid, ..member }) >= demand { high = mid } else { low = mid }
            }
            return check.remedy(Remedy::at_least(
                SubjectRef::new(&a.id, format!("{path}.compressionSteelAreaM2"), lc("Compression reinforcement A_s2", "Druckbewehrung A_s2")),
                Quantity::new(QuantityKind::Area, member.a_s2),
                Quantity::new(QuantityKind::Area, high),
                lc("Raise the compression reinforcement: the ratio ω'/ω enters the ultimate chord rotation.", "Druckbewehrung erhöhen: das Verhältnis ω'/ω geht in den Bruch-Sehnendrehwinkel ein."),
            ));
        }
    }
    let factor = if ls.starts_with("sd") || ls.starts_with("significant") { sd_factor } else { 1.0 };
    let required = demand * capacity.cf * a.gamma_el / factor;
    let (field, current) = if dl { ("chordRotationYieldRad", a.chord_rotation_yield_rad) } else { ("chordRotationUltimateRad", a.chord_rotation_ultimate_rad) };
    check.remedy(Remedy::at_least(
        SubjectRef::new(&a.id, format!("{path}.{field}"), lc("Chord rotation capacity", "Sehnendrehwinkel-Kapazität")),
        q_dim(current),
        q_dim(required),
        lc("Retrofit the member (jacket, FRP) until the chord rotation capacity reaches the demand, or reduce the demand.", "Bauteil ertüchtigen (Mantel, FRP), bis die Sehnendrehwinkel-Kapazität die Einwirkung erreicht, oder die Einwirkung verringern."),
    ))
}

fn chord_rotation_derivation(report: &mut CheckReport, annex: AnnexChoice, a: &semio_s_artifact_norm_en1998_model::En1998Assessment, capacity: &ChordCapacity, member: &part_3::RcMember, path: &str) {
    let k = &capacity.coefficients;
    let xi = part_3::yield_neutral_axis_ratio(member, k.steel_modulus_mpa);
    let response = member.section_response(k);
    let phi = response.map_or_else(|| part_3::yield_curvature_per_m(member, k.steel_modulus_mpa), |r| r.kappa_y);
    let (m_y, phi_u, m_u) = response.map_or((f64::NAN, f64::NAN, f64::NAN), |r| (r.m_y / 1000.0, r.kappa_u, r.m_u / 1000.0));
    let stress = part_3::concrete_stress_ratio_at_yield(member, k.steel_modulus_mpa);
    let alpha = part_3::confinement_effectiveness(member);
    let rho_sx = part_3::transverse_ratio(member);
    let subject = || SubjectRef::new(&a.id, format!("{path}.sectionDepthM"), lc(&a.id, &a.id));
    report.push(
        CheckResult::assess(format!("en1998.3.{}.chordRotation.derivation", a.id), "DIN EN 1998-3", ClauseId::new("EN 1998-3", "3", "A.3.2"), subject(), lc("Chord rotation capacity derived from the member section", "Aus dem Bauteilquerschnitt abgeleitete Sehnendrehwinkel-Kapazität"))
            .informative(q_dim(capacity.ultimate_rad))
            .annex(annex)
            .explanation(lc(
                &format!("ξ_y {xi:.3}, φ_y {phi:.5} 1/m, M_y {:.1} kNm, φ_u {:.5} 1/m, M_u {:.1} kNm, θ_y {:.5} rad, α {alpha:.3}, ρ_sx {rho_sx:.5}, θ_um {:.5} rad (strengths over CF {:.2}, γ_el {:.2})", m_y, phi_u, m_u, capacity.yield_rad, capacity.ultimate_rad, capacity.cf, a.gamma_el),
                &format!("ξ_y {xi:.3}, φ_y {phi:.5} 1/m, M_y {:.1} kNm, φ_u {:.5} 1/m, M_u {:.1} kNm, θ_y {:.5} rad, α {alpha:.3}, ρ_sx {rho_sx:.5}, θ_um {:.5} rad (Festigkeiten über CF {:.2}, γ_el {:.2})", m_y, phi_u, m_u, capacity.yield_rad, capacity.ultimate_rad, capacity.cf, a.gamma_el),
            ))
            .ndps(&["ndp.en1998.en3.annex-a.theta-y.base", "ndp.en1998.en3.annex-a.theta-y.slenderness", "ndp.en1998.en3.annex-a.theta-y.bar-slip", "ndp.en1998.en3.annex-a.theta-um.base", "ndp.en1998.en3.annex-a.theta-um.axial", "ndp.en1998.en3.annex-a.theta-um.reinforcement", "ndp.en1998.en3.annex-a.theta-um.shear-span", "ndp.en1998.en3.annex-a.theta-um.confinement", "ndp.en1998.en3.annex-a.theta-um.diagonal", "ndp.en1998.en3.annex-a.steel-modulus-mpa", "ndp.en1998.en3.annex-a.section.eps-c2-permille", "ndp.en1998.en3.annex-a.section.eps-cu-permille", "ndp.en1998.en3.annex-a.section.parabola-exponent", "ndp.en1998.en3.annex-a.section.eps-su-permille", "ndp.en1998.en3.annex-a.section.max-concrete-mpa"])
            .build(),
    );
    let mut linear = CheckResult::assess(format!("en1998.3.{}.chordRotation.linearity", a.id), "DIN EN 1998-3", ClauseId::new("EN 1998-3", "3", "A.3.2"), subject(), lc("Concrete stress at yield within the linear range of the cracked section", "Betonspannung beim Fließen im linearen Bereich des gerissenen Querschnitts"))
        .utilization(q_dim(if stress.is_finite() { stress } else { f64::INFINITY }), q_dim(1.0))
        .annex(annex)
        .explanation(lc(&format!("σ_c/f_c at yield = {stress:.2}; above 1 the cracked linear section underestimates φ_y"), &format!("σ_c/f_c beim Fließen = {stress:.2}; über 1 unterschätzt der lineare gerissene Querschnitt φ_y")));
    let ratio = |n: f64| part_3::concrete_stress_ratio_at_yield(&part_3::RcMember { n, ..*member }, k.steel_modulus_mpa);
    if !(stress <= 1.0) && ratio(0.0) <= 1.0 {
        let (mut ok, mut bad) = (0.0_f64, member.n);
        for _ in 0..48 {
            let mid = 0.5 * (ok + bad);
            if ratio(mid) <= 1.0 { ok = mid } else { bad = mid }
        }
        linear = linear.remedy(Remedy::at_most(SubjectRef::new(&a.id, format!("{path}.axialForceN"), lc("Axial force N", "Normalkraft N")), Quantity::new(QuantityKind::Force, member.n), Quantity::new(QuantityKind::Force, ok), lc("The closed form is invalid here: lower the axial force or give θ_y and θ_um from a nonlinear moment-curvature analysis.", "Die geschlossene Form gilt hier nicht: Normalkraft verringern oder θ_y und θ_um aus einer nichtlinearen Momenten-Krümmungs-Analyse angeben.")));
    }
    report.push(linear.build());
    if member.a_sx > 0.0 && (member.b_0 <= 0.0 || member.h_0 <= 0.0) {
        let core = |leaf: &str, en: &str, de: &str, current: f64, required: f64| Remedy::at_least(SubjectRef::new(&a.id, format!("{path}.{leaf}"), lc(en, de)), Quantity::new(QuantityKind::Length, current), Quantity::new(QuantityKind::Length, required), lc(&format!("Enter the confined core {en} of at least {:.3} m (section {en} minus the cover) so that the confinement term counts.", required), &format!("Den umschnürten Kern {de} von mindestens {:.3} m (Querschnitt abzüglich Überdeckung) eingeben, damit der Umschnürungsterm angesetzt wird.", required)));
        let mut core_check =
            CheckResult::assess(format!("en1998.3.{}.chordRotation.core", a.id), "DIN EN 1998-3", ClauseId::new("EN 1998-3", "3", "A.3.2"), SubjectRef::new(&a.id, format!("{path}.coreWidthM"), lc(&a.id, &a.id)), lc("Confined core dimensions", "Abmessungen des umschnürten Kerns"))
                .status(semio_s_plugin_norm_compliance::CheckStatus::Warning)
                .annex(annex)
                .explanation(lc("Stirrups are given but the core b_0 and h_0 are missing: the confinement term is not counted.", "Bügel sind angegeben, aber der Kern b_0 und h_0 fehlt: der Umschnürungsterm wird nicht berücksichtigt."));
        if member.b_0 <= 0.0 {
            core_check = core_check.remedy(core("coreWidthM", "width b_0", "Breite b_0", member.b_0, (member.b - 0.06).max(0.1)));
        }
        if member.h_0 <= 0.0 {
            core_check = core_check.remedy(core("coreDepthM", "depth h_0", "Höhe h_0", member.h_0, (member.h - 0.06).max(0.1)));
        }
        report.push(core_check.build());
    }
}

/// 🌉 Checks the capacity-design shear of a ductile pier against its shear resistance.
pub fn check_pier(report: &mut CheckReport, annex: AnnexChoice, b: &semio_s_artifact_norm_en1998_model::En1998Bridge) {
    if b.pier_height_m <= 0.0 || b.pier_moment_rd_nm <= 0.0 {
        return;
    }
    let gamma_0 = ndp_number("ndp.en1998.en2.pier.gamma-0", annex);
    let demand = part_2::pier_capacity_design_shear_n(gamma_0, b.pier_moment_rd_nm, b.pier_height_m);
    let path = id_sel("bridges", &b.id);
    let mut check = CheckResult::assess(
        format!("en1998.2.{}.pier.shear", b.id),
        "DIN EN 1998-2",
        ClauseId::new("EN 1998-2", "2", "5.6.3.3"),
        SubjectRef::new(&b.id, format!("{path}.pierShearRdN"), lc(&b.id, &b.id)),
        lc("Pier capacity-design shear V_CD = γ₀·M_Rd/L_s", "Pfeiler-Kapazitätsquerkraft V_CD = γ₀·M_Rd/L_s"),
    )
    .utilization(q_force(demand), floor_limit(q_force(b.pier_shear_rd_n)))
    .annex(annex)
    .explanation(lc(&format!("V_CD = {gamma_0}·{:.0} Nm / {:.2} m = {demand:.0} N, V_Rd = {:.0} N", b.pier_moment_rd_nm, b.pier_height_m, b.pier_shear_rd_n), &format!("V_CD = {gamma_0}·{:.0} Nm / {:.2} m = {demand:.0} N, V_Rd = {:.0} N", b.pier_moment_rd_nm, b.pier_height_m, b.pier_shear_rd_n)));
    if demand > b.pier_shear_rd_n {
        check = check.remedy(Remedy::at_least(SubjectRef::new(&b.id, format!("{path}.pierShearRdN"), lc("Pier shear resistance", "Pfeilerquerkrafttragfähigkeit")), q_force(b.pier_shear_rd_n), q_force(demand), lc("Increase the shear reinforcement of the pier.", "Querkraftbewehrung des Pfeilers erhöhen.")));
    }
    report.push(check.ndp("ndp.en1998.en2.pier.gamma-0").build());
}

/// 🪵️🪨️ Checks the timber connection and masonry wall data of one member.
pub fn check_member_materials(report: &mut CheckReport, annex: AnnexChoice, bid: &str, bpath: &str, m: &Member) {
    let mpath = format!("{bpath}.members[id={}]", m.id);
    let (d_max, t_factor, slenderness, t_min) = (ndp_number("ndp.en1998.en1.timber.max-diameter-mm", annex), ndp_number("ndp.en1998.en1.timber.min-thickness-factor", annex), ndp_number("ndp.en1998.en1.masonry.slenderness-max", annex), ndp_number("ndp.en1998.en1.masonry.min-thickness-m", annex));
    if m.fastener_diameter_mm > 0.0 {
        let util = part_1::timber_connection_utilization(m.fastener_diameter_mm, m.timber_thickness_mm);
        let mut check = CheckResult::assess(
            format!("en1998.1.{bid}.member.{}.timberConnection", m.id),
            "DIN EN 1998-1",
            ClauseId::new("EN 1998-1", "1", "8.3"),
            SubjectRef::new(bid, format!("{mpath}.fastenerDiameterMm"), lc(&m.id, &m.id)),
            lc("Ductile timber connection: fastener diameter and member thickness", "Duktiler Holzanschluss: Verbindungsmitteldurchmesser und Holzdicke"),
        )
        .utilization(q_dim(util.min(1e9)), q_dim(1.0))
        .annex(annex)
        .explanation(lc(&format!("d = {:.1} mm (limit {d_max}), thickness {:.1} mm (at least {t_factor}·d = {:.1} mm)", m.fastener_diameter_mm, m.timber_thickness_mm, t_factor * m.fastener_diameter_mm), &format!("d = {:.1} mm (Grenze {d_max}), Dicke {:.1} mm (mindestens {t_factor}·d = {:.1} mm)", m.fastener_diameter_mm, m.timber_thickness_mm, t_factor * m.fastener_diameter_mm)));
        if util > 1.0 {
            check = check.remedy(Remedy::at_most(SubjectRef::new(bid, format!("{mpath}.fastenerDiameterMm"), lc("Fastener diameter", "Verbindungsmitteldurchmesser")), q_dim(m.fastener_diameter_mm), q_dim((m.timber_thickness_mm / t_factor).min(d_max)), lc("Use a smaller fastener diameter or a thicker timber member.", "Kleineren Verbindungsmitteldurchmesser oder dickeres Holz verwenden.")));
        }
        report.push(check.ndps(&["ndp.en1998.en1.timber.max-diameter-mm", "ndp.en1998.en1.timber.min-thickness-factor"]).build());
    }
    if m.wall_thickness_m > 0.0 {
        let util = part_1::masonry_wall_utilization(m.wall_effective_height_m, m.wall_thickness_m);
        let mut check = CheckResult::assess(
            format!("en1998.1.{bid}.member.{}.masonryWall", m.id),
            "DIN EN 1998-1",
            ClauseId::new("EN 1998-1", "1", "9.5.1"),
            SubjectRef::new(bid, format!("{mpath}.wallThicknessM"), lc(&m.id, &m.id)),
            lc("Masonry wall: slenderness h_ef/t_ef and thickness", "Mauerwerkswand: Schlankheit h_ef/t_ef und Dicke"),
        )
        .utilization(q_dim(util.min(1e9)), q_dim(1.0))
        .annex(annex)
        .explanation(lc(&format!("h_ef/t_ef = {:.2} (limit {slenderness}), t_ef = {:.3} m (at least {t_min} m)", m.wall_effective_height_m / m.wall_thickness_m, m.wall_thickness_m), &format!("h_ef/t_ef = {:.2} (Grenze {slenderness}), t_ef = {:.3} m (mindestens {t_min} m)", m.wall_effective_height_m / m.wall_thickness_m, m.wall_thickness_m)));
        if util > 1.0 {
            check = check.remedy(Remedy::at_least(SubjectRef::new(bid, format!("{mpath}.wallThicknessM"), lc("Wall thickness", "Wanddicke")), q_len(m.wall_thickness_m), q_len((m.wall_effective_height_m / slenderness).max(t_min)), lc("Increase the wall thickness or reduce the effective height.", "Wanddicke erhöhen oder wirksame Höhe verringern.")));
        }
        report.push(check.ndps(&["ndp.en1998.en1.masonry.slenderness-max", "ndp.en1998.en1.masonry.min-thickness-m"]).build());
    }
}

struct N2Governing {
    along_y: bool,
    assessment: part_1::N2Assessment,
    storey_height_m: f64,
    capacity_m: f64,
    utilisation: f64,
}

fn n2_direction(building: &semio_s_artifact_norm_en1998_model::En1998Building, along_y: bool, resistance: &[f64], theta_rad: f64, spectrum: &dyn Fn(f64) -> f64, t_c: f64) -> Option<N2Governing> {
    let n = building.storeys.len();
    let masses: Vec<f64> = building.storeys.iter().enumerate().map(|(i, st)| part_1::storey_seismic_mass_kg(st, i + 1 == n)).collect();
    let stiffness: Vec<f64> = building.storeys.iter().map(|st| if along_y { cracked_stiffness::storey_y(building, st) } else { cracked_stiffness::storey_x(building, st) }).collect();
    if masses.iter().chain(&stiffness).any(|v| *v <= 0.0) {
        return None;
    }
    let (_, shapes) = part_1::shear_building_modes(&masses, &stiffness);
    let first = shapes.first()?;
    let roof = *first.last()?;
    if roof.abs() < 1e-12 {
        return None;
    }
    let shape: Vec<f64> = first.iter().map(|v| v / roof).collect();
    let assessment = part_1::n2_assess(&masses, &stiffness, resistance, &shape, spectrum, t_c)?;
    let storey_height_m = building.storeys[assessment.weakest].height_m;
    let capacity_m = part_1::n2_roof_capacity_m(&assessment, storey_height_m, theta_rad);
    let utilisation = if capacity_m > 0.0 { assessment.target_roof_m / capacity_m } else { f64::INFINITY };
    Some(N2Governing { along_y, assessment, storey_height_m, capacity_m, utilisation })
}

/// 🌋️ Checks the N2 target displacement of the roof at the limit states SD and NC against the roof displacement capacity of the pushover mechanism (EN 1998-1 Annex B, EN 1998-3 2.3): the weakest storey deforms up to the chord rotation capacity of the limit state, the other storeys stay at their plateau drift. Remedies: a larger chord rotation capacity or a stronger weakest storey.
pub fn check_n2_performance(report: &mut CheckReport, annex: AnnexChoice, document: &En1998Snapshot, a: &semio_s_artifact_norm_en1998_model::En1998Assessment, gamma_i: f64, a_g: f64, s: f64, tb: f64, tc: f64, td: f64) {
    let Some(building) = document.buildings.iter().find(|b| b.id == a.supported_building_id) else { return };
    let resistance: Vec<f64> = building.storeys.iter().map(|st| st.shear_resistance_n).collect();
    if resistance.is_empty() || resistance.iter().any(|r| *r <= 0.0) {
        return;
    }
    let sd_factor = ndp_number("ndp.en1998.en3.chord-rotation.sd-factor", annex);
    let path = id_sel("assessments", &a.id);
    for (state, label_en, label_de) in [("sd", "SD", "SD"), ("nc", "NC", "NC")] {
        let probe = semio_s_artifact_norm_en1998_model::En1998Assessment { limit_state: state.into(), ..a.clone() };
        let capacity = chord_capacity(annex, &probe);
        let Some(theta) = part_3::chord_rotation_capacity_rad(state, capacity.yield_rad, capacity.ultimate_rad, sd_factor).filter(|t| *t > 0.0) else { continue };
        let a_g_state = gamma_i * a_g * part_3::limit_state_ag_factor(state, annex);
        let spectrum = move |t: f64| part_1::elastic_response_spectrum(a_g_state, s, tb, tc, td, t);
        let Some(worst) = [false, true].into_iter().filter_map(|along_y| n2_direction(building, along_y, &resistance, theta, &spectrum, tc)).fold(None, |best: Option<N2Governing>, next| match best { Some(b) if b.utilisation >= next.utilisation => Some(b), _ => Some(next) }) else { continue };
        let weakest = &building.storeys[worst.assessment.weakest];
        let direction = if worst.along_y { "y" } else { "x" };
        let mut check = CheckResult::assess(
            format!("en1998.3.{}.n2.{state}", a.id),
            "DIN EN 1998-3",
            ClauseId::new("EN 1998-3", "3", "2.3"),
            SubjectRef::new(&a.id, format!("{path}.chordRotationUltimateRad"), lc(&a.id, &a.id)),
            lc(&format!("N2 target displacement against the displacement capacity at {label_en}"), &format!("N2-Zielverschiebung gegen die Verschiebungskapazität bei {label_de}")),
        )
        .utilization(q_len(worst.assessment.target_roof_m), floor_limit(q_len(worst.capacity_m)))
        .annex(annex)
        .explanation(lc(
            &format!("d_t = {:.1} mm against d_cap = {:.1} mm in direction {direction} (T* {:.3} s, q_u {:.2}, d_y {:.1} mm, weakest storey {} with h {:.2} m and capacity θ = {theta:.5} rad; the other storeys stay at their plateau drift)", worst.assessment.target_roof_m * 1000.0, worst.capacity_m * 1000.0, worst.assessment.target.t_star, worst.assessment.target.q_u, worst.assessment.yield_displacement * 1000.0, weakest.id, worst.storey_height_m),
            &format!("d_t = {:.1} mm gegen d_cap = {:.1} mm in Richtung {direction} (T* {:.3} s, q_u {:.2}, d_y {:.1} mm, schwächstes Geschoss {} mit h {:.2} m und Kapazität θ = {theta:.5} rad; die übrigen Geschosse bleiben bei ihrer Plateau-Verschiebung)", worst.assessment.target_roof_m * 1000.0, worst.capacity_m * 1000.0, worst.assessment.target.t_star, worst.assessment.target.q_u, worst.assessment.yield_displacement * 1000.0, weakest.id, worst.storey_height_m),
        ));
        if worst.utilisation > 1.0 {
            let required_theta = part_1::n2_required_drift_capacity_rad(&worst.assessment, worst.storey_height_m);
            let factor = if state == "sd" { sd_factor } else { 1.0 };
            if required_theta.is_finite() && factor > 0.0 {
                check = check.remedy(Remedy::at_least(
                    SubjectRef::new(&a.id, format!("{path}.chordRotationUltimateRad"), lc("Chord rotation capacity θ_um", "Sehnendrehwinkel-Kapazität θ_um")),
                    q_dim(a.chord_rotation_ultimate_rad),
                    q_dim(required_theta * capacity.cf * a.gamma_el / factor),
                    lc("Retrofit the critical members (jacket, FRP) until the chord rotation capacity lets the weakest storey follow the target displacement.", "Maßgebende Bauteile ertüchtigen (Mantel, FRP), bis die Sehnendrehwinkel-Kapazität dem schwächsten Geschoss die Zielverschiebung erlaubt."),
                ));
            }
            let stronger = |factor: f64| {
                let scaled: Vec<f64> = resistance.iter().enumerate().map(|(i, r)| if i == worst.assessment.weakest { r * factor } else { *r }).collect();
                n2_direction(building, worst.along_y, &scaled, theta, &spectrum, tc).map_or(f64::INFINITY, |g| g.utilisation)
            };
            if stronger(8.0) <= 1.0 {
                let (mut low, mut high) = (1.0, 8.0);
                for _ in 0..48 {
                    let mid = 0.5 * (low + high);
                    if stronger(mid) <= 1.0 { high = mid } else { low = mid }
                }
                check = check.remedy(Remedy::at_least(
                    SubjectRef::new(&building.id, format!("{}.storeys[id={}].shearResistanceN", id_sel("buildings", &building.id), weakest.id), lc("Shear resistance of the weakest storey", "Querkraftwiderstand des schwächsten Geschosses")),
                    q_force(weakest.shear_resistance_n),
                    q_force(weakest.shear_resistance_n * high),
                    lc("Strengthen the weakest storey: a higher plateau lowers the ductility demand q_u and with it the target displacement.", "Schwächstes Geschoss verstärken: ein höheres Plateau senkt die Duktilitätsanforderung q_u und damit die Zielverschiebung."),
                ));
            }
        }
        report.push(check.ndp("ndp.en1998.en3.chord-rotation.sd-factor").build());
    }
}
