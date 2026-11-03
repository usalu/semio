//! 🧰️ EN 1992 quantity and path helpers, the national annex and the clause modules of parts 1-1, 1-2, 2, 3 and 4.

use super::*;

//#region 🔖️Helpers
pub(super) fn moment_nm(nm: f64) -> Quantity {
    Quantity::new(QuantityKind::Moment, nm)
}
pub(super) fn force_n(n: f64) -> Quantity {
    Quantity::new(QuantityKind::Force, n)
}
pub(super) fn stress_pa(pa: f64) -> Quantity {
    Quantity::new(QuantityKind::Stress, pa)
}
pub(super) fn length_m(m: f64) -> Quantity {
    Quantity::length_m(m)
}
pub(super) fn area_m2(a: f64) -> Quantity {
    Quantity::area_m2(a)
}
pub(super) fn member_path(member_id: &str, field: &str) -> String {
    format!("members[id={member_id}].{field}")
}
pub(super) fn layer_path(member_id: &str, layer_id: &str, field: &str) -> String {
    format!("members[id={member_id}].longitudinal[id={layer_id}].{field}")
}
fn anchor_path(anchor_id: &str, field: &str) -> String {
    format!("anchors[id={anchor_id}].{field}")
}
pub(super) fn member_ref(doc: &En1992Snapshot, member: &RcMember, path: &str) -> SubjectRef {
    let base = member.subject_label();
    let label = l(
        &format!("{} — {}", doc.title, base.en),
        &format!("{} — {}", doc.title, base.de),
    );
    SubjectRef::new(member.id.clone(), path.to_string(), label)
}
/// 🛡 Cites the shared NDP registry rows a check rests on: the listed spec ids plus every `ndp.en1992.*` value read since the previous check was built; the kernel turns a pass into `UnverifiedBasis` while any row is unverified.
pub(super) fn ndps(builder: CheckBuilder, ids: &[&str]) -> CheckBuilder {
    builder.ndps(ids).ndp_reads()
}
pub(super) fn l(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}
pub(super) fn clause_11(section: &str) -> ClauseId {
    ClauseId::new("EN 1992-1-1", section, section.trim_start_matches('§'))
}
pub(super) fn part_11() -> &'static str {
    "DIN EN 1992-1-1"
}
pub(super) fn part_12() -> &'static str {
    "DIN EN 1992-1-2"
}
pub(super) fn part_2() -> &'static str {
    "DIN EN 1992-2"
}
pub(super) fn part_3() -> &'static str {
    "DIN EN 1992-3"
}
pub(super) fn part_4() -> &'static str {
    "DIN EN 1992-4"
}
//#endregion 🔖️Helpers

/// 🇩🇪 National annex parameters for EN 1992.
pub mod na_de {
    use crate::ndp;
    use semio_s_plugin_norm_compliance::AnnexChoice;

    #[derive(Clone, Copy, Debug)]
    pub struct AnnexParams {
        pub alpha_cc: f64,
        pub alpha_ct: f64,
        pub gamma_c: f64,
        pub gamma_s: f64,
    }

    impl AnnexParams {
        /// 🇪🇺 EN recommended values read from the shared registry.
        pub fn en() -> Self {
            Self::for_choice(AnnexChoice::En)
        }
        /// 🇩🇪 DIN EN 1992-1-1/NA values read from the shared registry.
        pub fn de() -> Self {
            Self::for_choice(AnnexChoice::De)
        }
        pub fn for_choice(choice: AnnexChoice) -> Self {
            Self { alpha_cc: ndp(choice, "ndp.en1992.m-01.alpha-cc-persistent"), alpha_ct: ndp(choice, "ndp.en1992.m-01.alpha-ct"), gamma_c: ndp(choice, "ndp.en1992.m-01.gamma-c-persistent"), gamma_s: ndp(choice, "ndp.en1992.gamma-s") }
        }
        /// 🚨 Accidental situation values of the registry (DE γ_c = 1.3, EN 1.2).
        pub fn accidental(choice: AnnexChoice) -> Self {
            Self { alpha_cc: ndp(choice, "ndp.en1992.m-01.alpha-cc-accidental"), alpha_ct: ndp(choice, "ndp.en1992.m-01.alpha-ct"), gamma_c: ndp(choice, "ndp.en1992.m-01.gamma-c-accidental"), gamma_s: ndp(choice, "ndp.en1992.gamma-s-acc") }
        }
        /// 🚨 Accidental / fire situations use the reduced γ of the registry.
        pub fn for_situation(choice: AnnexChoice, situation: &str) -> Self {
            match situation {
                "accidental" | "fire" => Self::accidental(choice),
                _ => Self::for_choice(choice),
            }
        }
        pub fn f_cd_pa(&self, f_ck_pa: f64) -> f64 {
            self.alpha_cc * f_ck_pa / self.gamma_c
        }
        pub fn f_yd_pa(&self, f_yk_pa: f64) -> f64 {
            f_yk_pa / self.gamma_s
        }
    }
}

/// 🧱 EN 1992-1-1 structural checks in the SI unit system (N, m, Pa); every formula delegates to the catalogue functions in mm, N, MPa.
pub mod part_1_1 {
    use crate::*;

    fn mpa(pa: f64) -> f64 {
        pa / 1.0e6
    }

    /// 📈 Mean tensile strength f_ctm [Pa] from f_ck (Table 3.1).
    pub fn f_ctm_pa(f_ck_pa: f64) -> f64 {
        conc(mpa(f_ck_pa)).fctm * 1.0e6
    }

    /// 💪 Rectangular-section flexural resistance M_Rd [N·m] by strain compatibility (parabola-rectangle, S-04); N_Ed tension positive.
    pub fn flexural_resistance_nm(f_ck_pa: f64, b: f64, h: f64, d: f64, a_s: f64, f_yk_pa: f64, n_ed: f64, annex: AnnexChoice) -> f64 {
        let fck = mpa(f_ck_pa);
        let mut s = Sec::rect(b * 1000.0, h * 1000.0, conc(fck), f_cd(annex, fck, false), f_yd(annex, mpa(f_yk_pa), false));
        s.bars = vec![Bar { y: 0.0, z: (h / 2.0 - d) * 1000.0, a: a_s * 1.0e6 }];
        member_m_rd_nm(&s, n_ed)
    }

    /// ✂️ Shear resistance without stirrups V_Rd,c [N] — Eq. 6.2a/b with σ_cp = N_Ed / A_c of the gross concrete area `a_c` [m²] (6.2.2(1)); N_Ed tension positive.
    pub fn shear_v_rd_c_n(b: f64, d: f64, a_c: f64, f_ck_pa: f64, rho_l: f64, n_ed: f64, annex: AnnexChoice) -> f64 {
        let (bm, dm) = (b * 1000.0, d * 1000.0);
        let sigma_cp = if a_c > 0.0 { -n_ed / (a_c * 1.0e6) } else { 0.0 };
        shear_torsion::v_rd_c(annex, mpa(f_ck_pa), bm, dm, rho_l * bm * dm, sigma_cp).0
    }

    /// ➰ Shear with stirrups V_Rd,s [N] — Eq. 6.8.
    pub fn shear_v_rd_s_n(asw_per_s: f64, z: f64, f_ywd_pa: f64, cot_theta: f64) -> f64 {
        asw_per_s * z * f_ywd_pa * cot_theta
    }

    /// 🧱 Crushing limit V_Rd,max [N] — Eq. 6.9 with ν1 and α_cw of the annex.
    pub fn shear_v_rd_max_n(b: f64, z: f64, f_ck_pa: f64, cot_theta: f64, annex: AnnexChoice) -> f64 {
        let fck = mpa(f_ck_pa);
        let fcd = f_cd(annex, fck, false);
        shear_torsion::v_rd_max(shear_torsion::alpha_cw(annex, 0.0, fcd), b * 1000.0, z * 1000.0, shear_torsion::nu1(annex, fck), fcd, cot_theta, 0.0)
    }

    /// 🎯 Shear capacity V_Rd [N] with the best strut inclination within the annex limits and the governing cot θ, for links A_sw/s [m²/m].
    pub fn shear_capacity_n(b: f64, z: f64, f_ck_pa: f64, f_yk_pa: f64, asw_per_s: f64, sigma_cp_pa: f64, v_ed: f64, annex: AnnexChoice) -> (f64, f64) {
        let (fck, bm, zm) = (mpa(f_ck_pa), b * 1000.0, z * 1000.0);
        let fcd = f_cd(annex, fck, false);
        let acw = shear_torsion::alpha_cw(annex, mpa(sigma_cp_pa), fcd);
        let nu = shear_torsion::nu1(annex, fck);
        let cmax = shear_torsion::cot_theta_max(annex, fck, bm, zm, mpa(sigma_cp_pa), fcd, v_ed.max(1.0));
        let fywd = f_yd(annex, mpa(f_yk_pa), false);
        let asw = asw_per_s * 1.0e3;
        let vs = |cot: f64| asw * zm * fywd * cot;
        let vm = |cot: f64| shear_torsion::v_rd_max(acw, bm, zm, nu, fcd, cot, 0.0);
        if vs(1.0) >= vm(1.0) { return (vm(1.0), 1.0); }
        if vs(cmax) <= vm(cmax) { return (vs(cmax), cmax); }
        let (mut lo, mut hi) = (1.0, cmax);
        for _ in 0..80 { let mid = 0.5 * (lo + hi); if vs(mid) < vm(mid) { lo = mid } else { hi = mid } }
        (vs(lo), lo)
    }

    /// 🕳️ Punching v_Rd,c [Pa] — Eq. 6.47 with the NA-DE C_Rd,c (reduced for interior columns with small u_0/d), k1 and v_min.
    pub fn punching_v_rd_c_pa(f_ck_pa: f64, d: f64, rho_l: f64, annex: AnnexChoice, pos: punching_and_struts::Position, u0_over_d: f64) -> f64 {
        punching_and_struts::v_rd_c_punch(annex, mpa(f_ck_pa), d * 1000.0, rho_l, rho_l, 0.0, pos, u0_over_d).0 * 1.0e6
    }

    /// 🕳️ Punching v_Rd,max [Pa]: DE 1.4 v_Rd,c, EC 0.5 ν f_cd.
    pub fn punching_v_rd_max_pa(v_rd_c_pa: f64, f_ck_pa: f64, annex: AnnexChoice) -> f64 {
        match annex {
            AnnexChoice::De => ndp(annex, "ndp.en1992.punch-vrdmax-factor") * v_rd_c_pa,
            AnnexChoice::En => 0.5 * 0.6 * (1.0 - mpa(f_ck_pa) / 250.0) * f_cd(annex, mpa(f_ck_pa), false) * 1.0e6,
        }
    }

    /// 🕳️ Load increase factor β of the NDP by position text.
    pub fn punching_beta(position: semio_s_artifact_norm_en1992_model::ColumnPosition, annex: AnnexChoice) -> f64 {
        ndp(annex, match position { semio_s_artifact_norm_en1992_model::ColumnPosition::Edge => "ndp.en1992.beta-edge", semio_s_artifact_norm_en1992_model::ColumnPosition::Corner => "ndp.en1992.beta-corner", semio_s_artifact_norm_en1992_model::ColumnPosition::WallEnd => "ndp.en1992.beta-wall-end", semio_s_artifact_norm_en1992_model::ColumnPosition::Interior => "ndp.en1992.beta-interior" })
    }

    /// 🎯 Load increase factor β of a punching column [m, N, N·m]: the fixed NDP value of 6.4.3(6) for the simplified method, the unbalanced-moment rules of 6.4.3(3) to (5) otherwise.
    pub fn punching_beta_of(p: &semio_s_artifact_norm_en1992_model::PunchingSpec, d: f64, v_ed: f64, annex: AnnexChoice) -> Result<punching_and_struts::BetaOutcome, punching_and_struts::BetaRefusal> {
        use semio_s_artifact_norm_en1992_model::{ColumnPosition, PunchingBetaMethod};
        match p.beta_method {
            PunchingBetaMethod::Simplified => Ok(punching_and_struts::BetaOutcome { beta: punching_beta(p.column_position, annex), rule: punching_and_struts::BetaRule::Simplified }),
            PunchingBetaMethod::UnbalancedMoments => {
                let pos = match p.column_position { ColumnPosition::Interior => punching_and_struts::Position::Interior, ColumnPosition::Edge => punching_and_struts::Position::Edge, ColumnPosition::Corner => punching_and_struts::Position::Corner, ColumnPosition::WallEnd => punching_and_struts::Position::WallEnd };
                punching_and_struts::beta_unbalanced(pos, p.column_width, p.column_depth, d, v_ed, p.moment_c1, p.moment_c2)
            }
        }
    }

    /// 🕳️ Perimeters (u0, u1) [m] of an interior rectangular column.
    pub fn punching_perimeters(c1: f64, c2: f64, d: f64) -> (f64, f64) {
        (2.0 * (c1 + c2), punching_and_struts::u1_rect(punching_and_struts::Position::Interior, c1, c2, d, 0.0, 0.0))
    }

    /// 🌀 Torsion T_Rd,max [N·m] of the thin-walled section with the enclosed area `a_k` [m²] and the wall `t_ef` [m] (cot θ = 1).
    pub fn torsion_t_rd_nm(f_ck_pa: f64, a_k: f64, t_ef: f64, annex: AnnexChoice) -> f64 {
        let fck = mpa(f_ck_pa);
        2.0 * shear_torsion::nu1(annex, fck) * f_cd(annex, fck, false) * (a_k * 1.0e6) * (t_ef * 1.0e3) * 0.5 / 1000.0
    }

    /// 📉 Basic l/d limit of Eq. 7.16 with K of the support and the NA-DE cap.
    pub fn basic_ld_limit(support: SupportCondition, f_ck_pa: f64, rho: f64, rho_prime: f64, annex: AnnexChoice) -> f64 {
        serviceability::ld_limit(annex, mpa(f_ck_pa), rho.max(1e-6), rho_prime.max(0.0), support_k(support))
    }

    /// 📏 c_min,dur [m] of the S4 column (Table 4.4N / NA-DE).
    pub fn c_min_dur_m(annex: AnnexChoice, exposure: ExposureClass) -> f64 {
        materials_and_time::c_min_dur_mm(annex, exposure_row(exposure), 4) / 1000.0
    }

    /// 🌦 Row of Table 4.4N for the exposure class; XF and XA classes do not govern the cover and map to the X0 row.
    pub fn exposure_row(e: ExposureClass) -> usize {
        match e {
            ExposureClass::X0 | ExposureClass::Xf1 | ExposureClass::Xf2 | ExposureClass::Xf3 | ExposureClass::Xf4 | ExposureClass::Xa1 | ExposureClass::Xa2 | ExposureClass::Xa3 => 0,
            ExposureClass::Xc1 => 1,
            ExposureClass::Xc2 | ExposureClass::Xc3 => 2,
            ExposureClass::Xc4 => 3,
            ExposureClass::Xd1 | ExposureClass::Xs1 => 4,
            ExposureClass::Xd2 | ExposureClass::Xs2 => 5,
            ExposureClass::Xd3 | ExposureClass::Xs3 => 6,
        }
    }

    /// 🪟 w_max [m] from exposure (quasi-permanent) — Table 7.1N / NA-DE.
    pub fn w_max_m(annex: AnnexChoice, exposure: ExposureClass) -> f64 {
        match exposure {
            ExposureClass::X0 | ExposureClass::Xc1 => ndp(annex, "ndp.en1992.wk-lim-xc1") / 1000.0,
            _ => ndp(annex, "ndp.en1992.wk-lim") / 1000.0,
        }
    }

    /// 📉 Min reinforcement A_s,min [m²] — §9.2.1.1 with the mean width `b` of the tension zone.
    pub fn a_s_min_m2(f_ck_pa: f64, f_yk_pa: f64, b: f64, d: f64) -> f64 {
        (0.26 * f_ctm_pa(f_ck_pa) / f_yk_pa * b * d).max(0.0013 * b * d)
    }

    /// 📉 Maximum reinforcement A_s,max = 0.04 A_c [m²] — §9.2.1.1(3).
    pub fn a_s_max_m2(a_c: f64) -> f64 {
        0.04 * a_c
    }

    /// 📏 Slenderness λ = l0 / i with the radius of gyration `i` about the weak axis.
    pub fn slenderness(l0: f64, i: f64) -> f64 {
        if i <= 0.0 { 0.0 } else { l0 / i }
    }

    /// 🎯 Limit slenderness λ_lim = 20 A B C/√n (5.8.3.1) with A = 0.7, B = √(1+2ω), C = 0.7 − r_m·0 (default 0.7); DE raises isolated members with n ≥ 0.41 to at least 25.
    pub fn lambda_limit(n: f64, omega: f64, annex: AnnexChoice) -> f64 {
        let lim = 20.0 * 0.7 * (1.0 + 2.0 * omega).sqrt() * 0.7 / n.max(1e-6).sqrt();
        if annex == AnnexChoice::De && n >= ndp(annex, "ndp.en1992.lambda-lim-de-n") { lim.max(25.0) } else { lim }
    }

    /// 🔍 Required A_s [m²] for M_Ed by bisection on the strain-compatibility resistance.
    pub fn required_a_s_m2(m_ed: f64, f_ck_pa: f64, b: f64, h: f64, d: f64, f_yk_pa: f64, n_ed: f64, annex: AnnexChoice) -> f64 {
        let (mut lo, mut hi) = (0.0, 0.04 * b * h);
        for _ in 0..40 {
            let mid = 0.5 * (lo + hi);
            if flexural_resistance_nm(f_ck_pa, b, h, d, mid, f_yk_pa, n_ed, annex) >= m_ed { hi = mid } else { lo = mid }
        }
        hi
    }

    /// 🏗️ Structural class delta vs S4 (Table 4.3N) from design life, strength, exposure and member kind.
    pub fn structural_class_delta(design_life_years: f64, f_ck_pa: f64, exposure: ExposureClass, member_kind: MemberKind) -> i32 {
        materials_and_time::structural_class(design_life_years, mpa(f_ck_pa), exposure_row(exposure), matches!(member_kind, MemberKind::Slab | MemberKind::FlatSlab), false, false) as i32 - 4
    }

    /// 📏 c_min,dur [m] of the structural class derived from the member.
    pub fn c_min_dur_adjusted_m(annex: AnnexChoice, exposure: ExposureClass, design_life_years: f64, f_ck_pa: f64, member_kind: MemberKind) -> f64 {
        let sc = (4 + structural_class_delta(design_life_years, f_ck_pa, exposure, member_kind)).clamp(1, 6) as u32;
        materials_and_time::c_min_dur_mm(annex, exposure_row(exposure), sc) / 1000.0
    }

    /// 🔗 c_min,b §4.4.1.2 — φ or φ_n = φ√n_b ≤ 55 mm; DE: d_g + 5 mm for aggregate above 16 mm.
    pub fn c_min_b_m(annex: AnnexChoice, bar_diameter: f64, aggregate_size: f64, bundled_count: u32) -> f64 {
        let phi_eq = (bar_diameter * (bundled_count.max(1) as f64).sqrt()).min(0.055);
        if aggregate_size > if annex == AnnexChoice::De { 0.016 } else { 0.032 } { phi_eq.max(aggregate_size + 0.005) } else { phi_eq }
    }

    /// 🧱 c_min = max(c_min,b, c_min,dur, 10 mm); c_nom = c_min + Δc_dev.
    pub fn c_nom_m(c_min_b: f64, c_min_dur: f64, delta_c_dev: f64) -> (f64, f64) {
        let c_min = c_min_b.max(c_min_dur).max(0.010);
        (c_min, c_min + delta_c_dev)
    }

    /// 🪢 l_b,rqd [m] with f_bd from the annex: f_bd = 2.25 η1 η2 f_ctd.
    pub fn l_b_rqd_m(phi: f64, sigma_sd: f64, f_bd: f64) -> f64 {
        if f_bd <= 0.0 { return 50.0 * phi; }
        detailing::l_b_rqd(phi, sigma_sd, f_bd)
    }

    /// 🪢 l_bd [m] = max(α l_b,rqd, 0.3 l_b,rqd, 10φ, 100 mm).
    pub fn l_bd_m(l_b_rqd: f64, phi: f64, alpha: f64) -> f64 {
        (alpha * l_b_rqd).max(l_b_min_m(l_b_rqd, phi))
    }

    /// 🪢 Minimum anchorage length l_b,min [m] = max(0.3 l_b,rqd, 10φ, 100 mm) (Eq. 8.6).
    pub fn l_b_min_m(l_b_rqd: f64, phi: f64) -> f64 {
        (0.3 * l_b_rqd).max(10.0 * phi).max(0.100)
    }

    /// 🪢 Product α_2 α_3 α_5 limited to at least 0.7 (8.4.4(1)).
    pub fn alpha_235(alpha2: f64, alpha3: f64, alpha5: f64) -> f64 {
        (alpha2 * alpha3 * alpha5).max(0.7)
    }

    /// 🪢 l_0 [m] = max(α l_bd-based, 0.3 α6 l_b,rqd-based, 15φ, 200 mm).
    pub fn l_0_m(l_bd: f64, phi: f64, alpha6: f64) -> f64 {
        (alpha6 * l_bd).max((0.3 * alpha6 * l_bd).max(15.0 * phi).max(0.200))
    }

    pub fn de_ld_caps(k: f64, span: f64, deflection_sensitive: bool) -> f64 {
        let cap1 = k * ndp(AnnexChoice::De, "ndp.en1992.ld-de-cap");
        if deflection_sensitive && span > 0.0 { cap1.min((k * k) * 150.0 / span) } else { cap1 }
    }

    /// 🧮 K of Table 7.4N: simply supported 1.0, continuous (end span) 1.3, fixed 1.5, cantilever 0.4.
    pub fn support_k(support: SupportCondition) -> f64 {
        match support {
            SupportCondition::SimplySupported => 1.0,
            SupportCondition::Continuous => 1.3,
            SupportCondition::Fixed => 1.5,
            SupportCondition::Cantilever => 0.4,
        }
    }
}

/// 🔥 EN 1992-1-2 tabulated fire resistance: member-level facade over the single table set of the fire catalogue (`fire_design`); columns use Table 5.2a at μ_fi = 0.5 for both methods, ribbed slabs use the solid one-way slab table.
pub mod part_1_2_fire {
    use crate::*;

    fn minutes(rating: FireRating) -> f64 {
        match rating {
            FireRating::R30 => 30.0,
            FireRating::R60 => 60.0,
            FireRating::R90 => 90.0,
            FireRating::R120 => 120.0,
        }
    }

    /// 🎯 Required (min_dimension, a) [m] for member kind, support and slab system.
    pub fn required_for(kind: MemberKind, support: SupportCondition, rating: FireRating, slab_system: semio_s_artifact_norm_en1992_model::SlabFireSystem) -> (f64, f64) {
        let (table, sub, key) = match kind {
            MemberKind::Column => ("column", "", 0.5),
            MemberKind::Wall | MemberKind::LiquidRetaining => ("wall", "", 0.0),
            MemberKind::TensionMember => ("tension", "", 0.0),
            MemberKind::FlatSlab => ("flat_slab", "", 0.0),
            MemberKind::RibbedSlab => ("slab", "one_way", 0.0),
            MemberKind::Slab => ("slab", if slab_system == semio_s_artifact_norm_en1992_model::SlabFireSystem::TwoWay { "two_way" } else { "one_way" }, 0.0),
            MemberKind::Beam | MemberKind::Bridge => (if matches!(support, SupportCondition::Continuous | SupportCondition::Fixed) { "beam_cont" } else { "beam_ss" }, "", 0.0),
        };
        let rows = fire_design::pick(table, sub, key).unwrap_or_default();
        let (dim, axis) = fire_design::interp_rows(&rows, minutes(rating));
        (dim / 1000.0, axis / 1000.0)
    }

    /// 📏 Smallest axis distance [m] of Table 5.5 for a simply supported beam of the given width [m].
    pub fn required_axis_distance_beam_m(width: f64, rating: FireRating) -> f64 {
        fire_design::beam_axis_for_width(width * 1000.0, minutes(rating)) / 1000.0
    }

    pub fn b_min_beam_m(rating: FireRating) -> f64 {
        required_for(MemberKind::Beam, SupportCondition::SimplySupported, rating, semio_s_artifact_norm_en1992_model::SlabFireSystem::OneWay).0
    }

    pub fn required_axis_distance_column_m(rating: FireRating) -> f64 {
        required_for(MemberKind::Column, SupportCondition::Fixed, rating, semio_s_artifact_norm_en1992_model::SlabFireSystem::OneWay).1
    }
}

/// 🌉 EN 1992-2/// 🌉 EN 1992-2 bridges.
pub mod part_2 {
    #[cfg(test)] use crate::*;

    pub fn concrete_stress_limit_pa(f_ck_pa: f64) -> f64 {
        0.6 * f_ck_pa
    }

    pub fn fatigue_limit_pa() -> f64 {
        ndp(AnnexChoice::De, "ndp.en1992.fatigue-straight") * 1.0e6 / ndp(AnnexChoice::De, "ndp.en1992.gamma-s-fat")
    }
}

/// 💧 EN 1992-3 liquid retaining.
pub mod part_3_liquid {
    use crate::*;

    pub fn tightness_crack_limit_m(class: TightnessClass, hd_over_h: f64) -> Option<f64> {
        let lim = || interp(&[(5.0, ndp(AnnexChoice::De, "ndp.en1992.tightness-wk-high")), (35.0, ndp(AnnexChoice::De, "ndp.en1992.tightness-wk-low"))], hd_over_h) / 1000.0;
        match class {
            TightnessClass::Tc0 => None,
            TightnessClass::Tc1 | TightnessClass::Tc2 => Some(lim()),
        }
    }
}

/// ⚓️ EN 1992-4 fastenings of a single headed anchor in the SI unit system (N, m, Pa) on top of the domain-neutral [`semio_s_plugin_norm_mechanics::fastening`] kernel; every annex row is read from the given annex, the head diameter is 2d unless given, the member thickness 2 h_ef, the anchor is cast in.
pub mod part_4 {
    use crate::*;
    use semio_s_plugin_norm_mechanics::fastening as kernel;

    fn gamma_mc(annex: AnnexChoice) -> f64 {
        ndp(annex, "ndp.en1992.m-01.gamma-c-persistent") * ndp(annex, "ndp.en1992.gamma-inst")
    }

    fn k1(annex: AnnexChoice, cracked: bool) -> f64 {
        ndp(annex, if cracked { "ndp.en1992.cone-k-cast-cracked" } else { "ndp.en1992.cone-k-cast-uncracked" })
    }

    pub fn gamma_ms(f_uk_pa: f64, f_yk_pa: f64) -> f64 {
        crate::gamma_ms(f_uk_pa / 1.0e6, f_yk_pa / 1.0e6, false)
    }

    pub fn gamma_ms_shear(f_uk_pa: f64, f_yk_pa: f64) -> f64 {
        crate::gamma_ms(f_uk_pa / 1.0e6, f_yk_pa / 1.0e6, true)
    }

    /// 🔩 N_Rd,s = A_s f_uk / γ_Ms,N (7.2.1.2).
    pub fn steel_resistance_n(a_s: f64, f_uk_pa: f64, f_yk_pa: f64) -> f64 {
        kernel::steel_n_rk_s(a_s, f_uk_pa) / gamma_ms(f_uk_pa, f_yk_pa)
    }

    /// 🔩 V_Rd,s = 0.5 A_s f_uk / γ_Ms,V without lever arm (7.2.2.3).
    pub fn steel_shear_resistance_n(a_s: f64, f_uk_pa: f64, f_yk_pa: f64) -> f64 {
        kernel::steel_v_rk_s(a_s, f_uk_pa, 1.0) / gamma_ms_shear(f_uk_pa, f_yk_pa)
    }

    fn n_rk_c_mm(annex: AnnexChoice, f_ck_mpa: f64, h_ef_mm: f64, cracked: bool, c1_mm: f64) -> f64 {
        kernel::cone_breakout(&kernel::ConeInput::single(f_ck_mpa, h_ef_mm, k1(annex, cracked), c1_mm)).n_rk_c
    }

    /// 🧱 N_Rd,c = N⁰_Rk,c (A_c,N/A⁰) ψ_s ψ_re / γ_Mc (7.2.1.4) for an anchor at distance c1 from one free edge.
    pub fn cone_resistance_n(annex: AnnexChoice, f_ck_pa: f64, h_ef: f64, cracked: bool, c1: f64) -> f64 {
        n_rk_c_mm(annex, f_ck_pa / 1.0e6, h_ef * 1000.0, cracked, c1 * 1000.0) / gamma_mc(annex)
    }

    /// 🧲 N_Rd,p = k2 A_h f_ck / γ_Mp (7.2.1.5), A_h = π/4 (d_h² − d²); `head_d` is d_h in m, zero derives d_h = 2d.
    pub fn pullout_resistance_n(annex: AnnexChoice, d: f64, head_d: f64, f_ck_pa: f64, cracked: bool) -> f64 {
        let dh = if head_d > d { head_d } else { 2.0 * d };
        let k2 = ndp(annex, if cracked { "ndp.en1992.pullout-k2-cracked" } else { "ndp.en1992.pullout-k2-uncracked" });
        kernel::pullout_n_rk_p(k2, f_ck_pa / 1.0e6, d * 1000.0, dh * 1000.0) / gamma_mc(annex)
    }

    /// 🧱 N_Rd,sp = min(N⁰_Rk,c, N_Rk,p) ψ_s ψ_h,sp / γ_Mc (7.2.1.7) with ψ_h,sp = 1.
    pub fn splitting_resistance_n(annex: AnnexChoice, f_ck_pa: f64, h_ef: f64, c1: f64, d: f64, head_d: f64, cracked: bool) -> f64 {
        let n_c = n_rk_c_mm(annex, f_ck_pa / 1.0e6, h_ef * 1000.0, cracked, c1 * 1000.0);
        let n_p = pullout_resistance_n(annex, d, head_d, f_ck_pa, cracked) * gamma_mc(annex);
        let ccr = 1.5 * h_ef * 1000.0;
        kernel::splitting_n_rk_sp(n_c, n_p, (c1 * 1000.0).min(ccr) / ccr * 0.5 + 0.5) / gamma_mc(annex)
    }

    /// 🪝 V_Rd,cp = k8 N_Rk,c / γ_Mc (7.2.2.4), k8 = 1 below h_ef 60 mm, else 2.
    pub fn pryout_resistance_n(annex: AnnexChoice, f_ck_pa: f64, h_ef: f64, cracked: bool, c1: f64) -> f64 {
        kernel::pryout_v_rk_cp(cone_resistance_n(annex, f_ck_pa, h_ef, cracked, c1), h_ef * 1000.0)
    }

    /// 🧱 V_Rd,c edge failure (7.2.2.5) with d_nom = d, l_f = min(h_ef, 8 d) and the member thickness 2 h_ef.
    pub fn edge_resistance_n(annex: AnnexChoice, d: f64, h_ef: f64, f_ck_pa: f64, c1: f64, cracked: bool) -> f64 {
        let k9 = if cracked { 1.7 } else { 2.4 };
        kernel::edge_breakout(&kernel::EdgeInput::single(f_ck_pa / 1.0e6, k9, d * 1000.0, h_ef * 1000.0, c1 * 1000.0, 2.0 * h_ef * 1000.0)).v_rk_c / gamma_mc(annex)
    }

/// 🔗 Tension–shear interaction (N/N_Rd)^1.5 + (V/V_Rd)^1.5 ≤ 1 (7.2.3).
    pub fn interaction_utilization(n_ed: f64, n_rd: f64, v_ed: f64, v_rd: f64) -> f64 {
        kernel::interaction(n_ed, n_rd, v_ed, v_rd, 1.5)
    }

    /// 🔩 Steel interaction (N/N_Rd,s)² + (V/V_Rd,s)² ≤ 1.
    pub fn steel_interaction_utilization(n_ed: f64, n_rd_s: f64, v_ed: f64, v_rd_s: f64) -> f64 {
        kernel::interaction(n_ed, n_rd_s, v_ed, v_rd_s, 2.0)
    }
}
