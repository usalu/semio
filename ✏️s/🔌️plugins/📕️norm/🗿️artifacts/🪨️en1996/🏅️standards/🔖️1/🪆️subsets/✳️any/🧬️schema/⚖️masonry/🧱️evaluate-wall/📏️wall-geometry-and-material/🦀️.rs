//! 📏️ Wall geometry and material phases: the wall paths, the slenderness and the material conformance of the wall evaluation. See [`super`].

use super::*;

/// 🔖️ Registry ids (`ndp.en1996.*`) the checks of this evaluator rest on, by data flow: the slenderness cites the restraint factor and the limit, the stiffening wall rule only for walls held on three or four sides and the 0.3 floor of ρ₃ only on three sides; the material conformance cites its own rules and the bed-joint cells it read; the bond overlap cites the execution rule; the geometric fit of openings and members rests on no parameter.
const OVERLAP: &[&str] = &["ndp.en1996.m70.execution"];
const CONFORMANCE: &[&str] = &["ndp.en1996.m01.conformance"];

/// 🔖️ The registry ids behind the slenderness of one wall.
fn slenderness_ndps(wall: &MasonryWall) -> Vec<&'static str> {
    let mut ids = vec!["ndp.en1996.m10.rho2", "ndp.en1996.m11.limit"];
    if wall.support_sides >= 3 {
        ids.push("ndp.en1996.m10.stiffener");
    }
    if wall.support_sides == 3 {
        ids.push("ndp.en1996.m11.beta-floor");
    }
    ids
}

/// 🗺 The wall field paths and the derived geometry the wall evaluation phases share.
pub(super) struct WallBasics {
    pub(super) path_t: String,
    pub(super) path_support: String,
    pub(super) path_fb: String,
    pub(super) path_mortar: String,
    pub(super) path_mortar_f: String,
    pub(super) path_unit_h: String,
    pub(super) lambda: f64,
    pub(super) a: f64,
    pub(super) t: f64,
}

/// 📏 Resolves the wall field paths, the slenderness, the area and the thickness.
pub(super) fn wall_paths_and_basics(wall: &MasonryWall) -> WallBasics {
    WallBasics {
        path_t: wall_path(wall, "thicknessM"),
        path_support: wall_path(wall, "supportSides"),
        path_fb: wall_path(wall, "fBPa"),
        path_mortar: wall_path(wall, "mortarClass"),
        path_mortar_f: wall_path(wall, "mortarStrengthPa"),
        path_unit_h: wall_path(wall, "unitHeightM"),
        lambda: slenderness(wall),
        a: area_m2(wall),
        t: wall.thickness_m,
    }
}

/// 📐 Slenderness limit h_ef/t_ef of load-bearing walls (registry row `ndp.en1996.limit.slenderness`, unverified).
pub(super) fn slenderness_limit() -> f64 {
    normative_limit(&["slenderness"])
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
    let limit = slenderness_limit();
    let mut b = CheckResult::assess(
        format!("en1996.5.5.1.slenderness.{}", wall.id),
        "EN 1996-1-1",
        clause("EN 1996-1-1", "§5.5.1", "5.5.1"),
        wall_subject(wall, path_t),
        loc(&format!("Slenderness λ ≤ {limit:.0}"), &format!("Schlankheit λ ≤ {limit:.0}")),
    )
    .utilization(Quantity::new(QuantityKind::Dimensionless, lambda), Quantity::new(QuantityKind::Dimensionless, limit))
    .annex(annex)
    .explanation(loc(
        &format!("λ=h_ef/t_ef={lambda:.2} with ρ_n={:.3}, ρ₂={:.2}, ℓ_ef={:.3} m.", h_ef_m(wall) / wall.height_m.max(1e-6), rho_2(wall), effective_length_m(wall)),
        &format!("λ=h_ef/t_ef={lambda:.2} mit ρ_n={:.3}, ρ₂={:.2}, ℓ_ef={:.3} m.", h_ef_m(wall) / wall.height_m.max(1e-6), rho_2(wall), effective_length_m(wall)),
    ));
    if lambda > limit {
        b = b.remedy(Remedy::at_least(
            wall_subject(wall, path_t),
            Quantity::length_m(t),
            Quantity::length_m(ceil_to(h_ef_m(wall) / limit, 1e-3)),
            loc(&format!("Increase thickness so λ ≤ {limit:.0}."), &format!("Dicke erhöhen, damit λ ≤ {limit:.0}.")),
        ));
        if wall.support_sides < 4 {
            b = b.remedy(Remedy::exactly(
                wall_subject(wall, path_support),
                Quantity::new(QuantityKind::Dimensionless, wall.support_sides as f64),
                Quantity::new(QuantityKind::Dimensionless, 4.0),
                loc("Increase to 4-sided support.", "Auf 4-seitige Halterung erhöhen."),
            ));
        }
    }
    out.push(b.ndps(&slenderness_ndps(wall)).build());
}

/// 🧱 Checks the material conformance of unit, mortar and bed joint.
pub(super) fn check_material_conformance(
    annex: AnnexChoice,
    wall: &MasonryWall,
    out: &mut Vec<CheckResult>,
    path_fb: &str,
    path_mortar_f: &str,
    _path_unit_h: &str,
) {
    let (c, reads) = semio_s_plugin_norm_compliance::record_ndp_reads(|| material_conformance(wall));
    let u = c.utilization();
    let class_fm = wall.mortar_class.f_m_mpa();
    let text = |mortar: &str, dims: &str, joint: &str, fb: &str| {
        format!(
            "{mortar} {:.2}; {dims} {:.2}; {joint} {:.2}; {fb} {:.2}.",
            c.mortar, c.dimension.max(c.aspect), c.joint, c.strength
        )
    };
    let mut b = CheckResult::assess(
        format!("en1996.3.1.material.{}", wall.id),
        "EN 1996-1-1",
        clause("EN 1996-1-1", "§3.1", "3.1"),
        wall_subject(wall, path_mortar_f),
        loc("Unit/mortar conformance (DE NA)", "Stein/Mörtel-Konformität (DE NA)"),
    )
    .utilization(Quantity::new(QuantityKind::Dimensionless, u), Quantity::new(QuantityKind::Dimensionless, 1.0))
    .annex(annex)
    .explanation(loc(
        &text("Mortar f_m deviation/15 % of class", "unit dimension/aspect", "bed joint", "unit strength"),
        &text("Mörtel f_m Abweichung/15 % der Klasse", "Steinabmessung/Schlankheit", "Lagerfuge", "Steindruckfestigkeit"),
    ));
    if u > 1.0 {
        let path_joint = wall_path(wall, "bedJointThicknessM");
        let path_unit_w = wall_path(wall, "unitWidthM");
        let path_unit_h = wall_path(wall, "unitHeightM");
        let fb = wall.f_b_pa;
        if c.mortar > 1.0 {
            b = b.remedy(Remedy::exactly(
                wall_subject(wall, path_mortar_f),
                Quantity::new(QuantityKind::Stress, wall.mortar_strength_pa),
                Quantity::new(QuantityKind::Stress, class_fm * 1e6),
                loc("Align mortarStrengthPa with mortarClass.", "mortarStrengthPa an mortarClass anpassen."),
            ));
        }
        if c.strength > 1.0 {
            b = b.remedy(if fb < c.fb_range.0 {
                Remedy::at_least(wall_subject(wall, path_fb), Quantity::new(QuantityKind::Stress, fb), Quantity::new(QuantityKind::Stress, c.fb_range.0), loc("Raise unit strength f_b into the admissible range.", "f_b in den zulässigen Bereich anheben."))
            } else {
                Remedy::at_most(wall_subject(wall, path_fb), Quantity::new(QuantityKind::Stress, fb), Quantity::new(QuantityKind::Stress, c.fb_range.1), loc("Lower unit strength f_b into the admissible range.", "f_b in den zulässigen Bereich senken."))
            });
        }
        if c.dimension > 1.0 {
            b = b.remedy(Remedy::at_least(
                wall_subject(wall, &path_unit_w),
                Quantity::length_m(wall.unit_width_m),
                Quantity::length_m(0.09),
                loc("Widen the unit to at least 90 mm.", "Stein auf mindestens 90 mm verbreitern."),
            ));
        }
        if c.aspect > 1.0 {
            b = b.remedy(Remedy::at_most(
                wall_subject(wall, &path_unit_h),
                Quantity::length_m(wall.unit_height_m),
                Quantity::length_m(floor_to(2.5 * wall.unit_width_m - 1e-3, 1e-3)),
                loc("Lower the unit so that h/w < 2.5 for group 2.", "Stein so niedriger wählen, dass h/b < 2,5 für Gruppe 2."),
            ));
        }
        if c.joint > 1.0 {
            let t_joint = wall.bed_joint_thickness_m;
            b = b.remedy(if t_joint > c.joint_range.1 {
                Remedy::at_most(wall_subject(wall, &path_joint), Quantity::length_m(t_joint), Quantity::length_m(c.joint_range.1), loc("Reduce the bed joint into the admissible range.", "Lagerfuge in den zulässigen Bereich verringern."))
            } else {
                Remedy::at_least(wall_subject(wall, &path_joint), Quantity::length_m(t_joint), Quantity::length_m(c.joint_range.0), loc("Increase the bed joint into the admissible range.", "Lagerfuge in den zulässigen Bereich vergrößern."))
            });
        }
    }
    out.push(b.ndps(CONFORMANCE).ndps(&reads).build());
}

/// 🧱 Checks that the unit fits the wall and bonds: unit width within the thickness, running-bond overlap l_u/2 ≥ max(0.4 h_u, 45 mm), and that every opening fits the wall height.
pub(super) fn check_unit_and_opening_fit(annex: AnnexChoice, wall: &MasonryWall, out: &mut Vec<CheckResult>) {
    let t = wall.thickness_m;
    let overlap_need = (normative_limit(&["overlap", "ratio"]) * wall.unit_height_m).max(normative_limit(&["overlap", "min"]));
    let width_u = wall.unit_width_m / t.max(1e-9);
    let overlap_u = overlap_need / (wall.unit_length_m / 2.0).max(1e-9);
    let path_w = wall_path(wall, "unitWidthM");
    let path_l = wall_path(wall, "unitLengthM");
    let mut b = CheckResult::assess(
        format!("en1996.8.1.5.unit-fit.{}", wall.id),
        "EN 1996-2",
        clause("EN 1996-2", "§8.1.5", "8.1.5"),
        wall_subject(wall, &path_w),
        loc("Unit fits the wall and bonds", "Stein passt in die Wand und bindet"),
    )
    .utilization(Quantity::new(QuantityKind::Dimensionless, width_u.max(overlap_u)), Quantity::new(QuantityKind::Dimensionless, 1.0))
    .annex(annex)
    .explanation(loc(
        &format!("unit width/t={width_u:.3}; overlap need {overlap_need:.3} m over l_u/2={:.3} m ⇒ {overlap_u:.3}.", wall.unit_length_m / 2.0),
        &format!("Steinbreite/t={width_u:.3}; Überbindung erforderlich {overlap_need:.3} m bei l_u/2={:.3} m ⇒ {overlap_u:.3}.", wall.unit_length_m / 2.0),
    ));
    if width_u > 1.0 {
        b = b.remedy(Remedy::at_most(wall_subject(wall, &path_w), Quantity::length_m(wall.unit_width_m), Quantity::length_m(t), loc("Use a unit no wider than the wall.", "Stein höchstens so breit wie die Wand wählen.")));
    }
    if overlap_u > 1.0 {
        b = b.remedy(Remedy::at_least(wall_subject(wall, &path_l), Quantity::length_m(wall.unit_length_m), Quantity::length_m(ceil_to(2.0 * overlap_need, 1e-3)), loc("Use a longer unit for the running-bond overlap.", "Längeren Stein für die Überbindung wählen.")));
    }
    out.push(b.ndps(OVERLAP).build());
    for o in &wall.openings {
        let path_sill = format!("walls[id={}].openings[id={}].sillHeightM", wall.id, o.id);
        let path_h = format!("walls[id={}].openings[id={}].heightM", wall.id, o.id);
        let u = (o.sill_height_m + o.height_m) / wall.height_m.max(1e-9);
        let mut bo = CheckResult::assess(
            format!("en1996.5.4.opening.{}.{}", wall.id, o.id),
            "EN 1996-1-1",
            clause("EN 1996-1-1", "§5.4", "5.4"),
            wall_subject(wall, &path_sill),
            loc("Opening fits the wall height", "Öffnung passt in die Wandhöhe"),
        )
        .utilization(Quantity::length_m(o.sill_height_m + o.height_m), Quantity::length_m(wall.height_m))
        .annex(annex)
        .explanation(loc(&format!("sill + height = {:.3} m of {:.3} m (ratio {u:.3}).", o.sill_height_m + o.height_m, wall.height_m), &format!("Brüstung + Höhe = {:.3} m von {:.3} m (Verhältnis {u:.3}).", o.sill_height_m + o.height_m, wall.height_m)));
        if u > 1.0 {
            bo = bo
                .remedy(Remedy::at_most(wall_subject(wall, &path_sill), Quantity::length_m(o.sill_height_m), Quantity::length_m(floor_to((wall.height_m - o.height_m).max(0.0), 1e-3)), loc("Lower the sill.", "Brüstung absenken.")))
                .remedy(Remedy::at_most(wall_subject(wall, &path_h), Quantity::length_m(o.height_m), Quantity::length_m(floor_to((wall.height_m - o.sill_height_m).max(0.0), 1e-3)), loc("Reduce the opening height.", "Öffnungshöhe verringern.")));
        }
        out.push(bo.build());
    }
    let l = wall.length_m.max(1e-9);
    let extents = wall
        .openings
        .iter()
        .map(|o| ("openings", o.id.as_str(), o.position_m, o.position_m, o.width_m))
        .chain(wall.piers.iter().map(|p| ("piers", p.id.as_str(), p.position_m, p.position_m, p.width_m)))
        .chain(wall.lintels.iter().map(|x| ("lintels", x.id.as_str(), x.position_m, x.position_m - x.bearing_length_m, x.span_m + 2.0 * x.bearing_length_m)))
        .chain(wall.confined.iter().map(|c| ("confined", c.id.as_str(), c.position_m, c.position_m - 0.5 * c.width_m, c.width_m)));
    for (coll, id, pos, start, width) in extents {
        let path = format!("walls[id={}].{coll}[id={id}].positionM", wall.id);
        let end = start + width;
        let u = if start < 0.0 { 1.0 - start / l } else { end / l };
        let mut be = CheckResult::assess(
            format!("en1996.5.4.extent.{}.{coll}.{id}", wall.id),
            "EN 1996-1-1",
            clause("EN 1996-1-1", "§5.4", "5.4"),
            wall_subject(wall, &path),
            loc("Member lies within the wall length", "Bauteil liegt innerhalb der Wandlänge"),
        )
        .utilization(Quantity::new(QuantityKind::Dimensionless, u), Quantity::new(QuantityKind::Dimensionless, 1.0))
        .annex(annex)
        .explanation(loc(&format!("x from {start:.3} m to {end:.3} m of l = {:.3} m.", wall.length_m), &format!("x von {start:.3} m bis {end:.3} m bei l = {:.3} m.", wall.length_m)));
        if start < 0.0 {
            be = be.remedy(Remedy::at_least(wall_subject(wall, &path), Quantity::length_m(pos), Quantity::length_m(pos - start), loc("Move the member into the wall.", "Bauteil in die Wand verschieben.")));
        } else if end > wall.length_m {
            be = be.remedy(Remedy::at_most(wall_subject(wall, &path), Quantity::length_m(pos), Quantity::length_m(floor_to((pos - (end - wall.length_m)).max(0.0), 1e-3)), loc("Move the member into the wall.", "Bauteil in die Wand verschieben.")));
        }
        out.push(be.build());
    }
}
