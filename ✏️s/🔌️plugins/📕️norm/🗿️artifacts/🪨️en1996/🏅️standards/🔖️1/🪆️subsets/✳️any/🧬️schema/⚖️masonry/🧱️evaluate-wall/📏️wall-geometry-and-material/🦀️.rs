//! 📏️ Wall geometry and material phases: the wall paths, the slenderness and the material conformance of the wall evaluation. See [`super`].

use super::*;

/// 🗺 The wall field paths and the derived geometry the wall evaluation phases share.
pub(super) struct WallBasics {
    pub(super) path_t: String,
    pub(super) path_support: String,
    pub(super) path_fb: String,
    pub(super) path_mortar: String,
    pub(super) path_mortar_f: String,
    pub(super) path_mu: String,
    pub(super) path_bearing: String,
    pub(super) path_as_h: String,
    pub(super) path_unit_h: String,
    pub(super) path_density: String,
    pub(super) path_phi_inf: String,
    pub(super) path_basement: String,
    pub(super) lambda: f64,
    pub(super) a: f64,
    pub(super) f_k: f64,
    pub(super) t: f64,
}

/// 📏 Resolves the wall field paths, the slenderness, the area, the characteristic strength and the thickness.
pub(super) fn wall_paths_and_basics(annex: AnnexChoice, wall: &MasonryWall) -> WallBasics {
    let path_t = wall_path(wall, "thicknessM");
        let path_support = wall_path(wall, "supportSides");
    let path_fb = wall_path(wall, "fBPa");
    let path_mortar = wall_path(wall, "mortarClass");
    let path_mortar_f = wall_path(wall, "mortarStrengthPa");
    let path_mu = wall_path(wall, "mu");
    let path_bearing = wall_path(wall, "slabBearingDepthM");
    let path_as_h = wall_path(wall, "asHorizontalM2");
    let path_unit_h = wall_path(wall, "unitHeightM");
    let path_density = wall_path(wall, "densityKgM3");
    let path_phi_inf = wall_path(wall, "phiInfinity");
    let path_basement = wall_path(wall, "isBasement");

    let lambda = slenderness(wall);
    let a = area_m2(wall);
    let f_k = f_k_pa(annex, wall);
    let t = wall.thickness_m;
    WallBasics { path_t, path_support, path_fb, path_mortar, path_mortar_f, path_mu, path_bearing, path_as_h, path_unit_h, path_density, path_phi_inf, path_basement, lambda, a, f_k, t }
}

/// 📐 Checks the slenderness of the wall.
pub(super) fn check_slenderness(
    annex: AnnexChoice,
    wall: &MasonryWall,
    out: &mut Vec<CheckResult>,
    path_t: &str,
    path_support: &str,
    lambda: f64,
    t: f64,
) {
    {
        let mut b = CheckResult::assess(
            format!("en1996.5.5.1.slenderness.{}", wall.id),
            "EN 1996-1-1",
            clause("EN 1996-1-1", "§5.5.1", "5.5.1"),
            wall_subject(wall, path_t),
            loc("Slenderness λ ≤ 27", "Schlankheit λ ≤ 27"),
        )
        .utilization(Quantity::new(QuantityKind::Dimensionless, lambda), Quantity::new(QuantityKind::Dimensionless, 27.0))
        .annex(annex)
        .explanation(loc(
            &format!("λ=h_ef/t_ef={lambda:.2} with L_eff={:.3} m (openings).", effective_length_m(wall)),
            &format!("λ=h_ef/t_ef={lambda:.2} mit L_eff={:.3} m (Öffnungen).", effective_length_m(wall)),
        ));
        let t_req = h_ef_m(wall) / 27.0;
        b = b
            .remedy(Remedy::at_least(
                wall_subject(wall, path_t),
                Quantity::length_m(t),
                Quantity::length_m(t_req),
                loc("Increase thickness so λ ≤ 27.", "Dicke erhöhen, damit λ ≤ 27."),
            ))
            .remedy(Remedy::exactly(
                wall_subject(wall, path_support),
                Quantity::new(QuantityKind::Dimensionless, wall.support_sides as f64),
                Quantity::new(QuantityKind::Dimensionless, 4.0),
                loc("Increase to 4-sided support.", "Auf 4-seitige Halterung erhöhen."),
            ));
        out.push(b.build());
    }
}

/// 🧱 Checks the material conformance of unit, mortar and density.
pub(super) fn check_material_conformance(
    annex: AnnexChoice,
    wall: &MasonryWall,
    out: &mut Vec<CheckResult>,
    path_fb: &str,
    path_mortar_f: &str,
    path_unit_h: &str,
) {
    {
        let (ok, en, de) = material_conformance_ok(wall);
        let mut b = CheckResult::assess(
            format!("en1996.3.1.material.{}", wall.id),
            "EN 1996-1-1",
            clause("EN 1996-1-1", "§3.1", "3.1"),
            wall_subject(wall, path_mortar_f),
            loc("Unit/mortar conformance (DE NA)", "Stein/Mörtel-Konformität (DE NA)"),
        )
        .annex(annex)
        .explanation(loc(&en, &de));
        if ok {
            b = b.utilization(Quantity::new(QuantityKind::Dimensionless, 0.5), Quantity::new(QuantityKind::Dimensionless, 1.0));
        } else {
            let path_joint = wall_path(wall, "bedJointThicknessM");
            let path_unit_w = wall_path(wall, "unitWidthM");
            let joint_target = if matches!(wall.mortar_type, MortarType::ThinLayer) {
                0.002
            } else {
                0.010
            };
            let fb_target_mpa = match wall.unit_material {
                UnitMaterial::Aerated => wall.f_b_pa.max(2.0e6).min(10.0e6) / 1e6,
                UnitMaterial::Clay => wall.f_b_pa.max(6.0e6) / 1e6,
                _ => wall.f_b_pa.max(6.0e6) / 1e6,
            };
            b = b
                .utilization(Quantity::new(QuantityKind::Dimensionless, 1.5), Quantity::new(QuantityKind::Dimensionless, 1.0))
                .remedy(Remedy::exactly(
                    wall_subject(wall, path_mortar_f),
                    Quantity::new(QuantityKind::Stress, wall.mortar_strength_pa),
                    Quantity::new(QuantityKind::Stress, wall.mortar_class.f_m_mpa() * 1e6),
                    loc("Align mortarStrengthPa with mortarClass.", "mortarStrengthPa an mortarClass anpassen."),
                ))
                .remedy(Remedy::at_least(
                    wall_subject(wall, path_fb),
                    Quantity::new(QuantityKind::Stress, wall.f_b_pa),
                    Quantity::stress_mpa(fb_target_mpa.max(6.0)),
                    loc("Raise unit strength f_b into admissible range.", "f_b in zulässigen Bereich anheben."),
                ))
                .remedy(Remedy::exactly(
                    wall_subject(wall, path_unit_h),
                    Quantity::length_m(wall.unit_height_m),
                    Quantity::length_m(0.113),
                    loc("Use admissible unit height for group/shape factor δ.", "Zulässige Steinhöhe für Formfaktor δ wählen."),
                ))
                .remedy(Remedy::at_least(
                    wall_subject(wall, &path_unit_w),
                    Quantity::length_m(wall.unit_width_m),
                    Quantity::length_m((wall.unit_height_m / 2.4).max(0.100)),
                    loc("Widen unit so group aspect and dimensions conform.", "Steinsicht verbreitern für Gruppen-/Abmessungskonformität."),
                ))
                .remedy(Remedy::exactly(
                    wall_subject(wall, &path_joint),
                    Quantity::length_m(wall.bed_joint_thickness_m),
                    Quantity::length_m(joint_target),
                    loc(
                        "Set bed-joint thickness to admissible GP (10 mm) or thin-layer (2 mm) value.",
                        "Lagerfugendicke auf zulässigen NM- (10 mm) bzw. DBM-Wert (2 mm) setzen.",
                    ),
                ));
        }
        out.push(b.build());
    }
}
