//! 🧰️ Member cases of the EN 1996 document evaluation: piers, lintels, ties, anchors, tendons, confining columns, flange walls, bars, frost, soil and fire isotherm of one wall, bound to their rows so that every result and remedy resolves in the document. See [`super`].

use super::*;
use std::f64::consts::PI;

const GRAVITY: f64 = 9.81;
const E_P_PA: f64 = 195e9;
const D_OVER_L: f64 = 0.8;
const K_THETA: f64 = 1.0;

fn row_path(wall: &MasonryWall, coll: &str, id: &str, field: &str) -> String {
    format!("walls[id={}].{coll}[id={id}].{field}", wall.id)
}

fn row_case(annex: AnnexChoice, wall: &MasonryWall, coll: &str, id: &str, field: &str, suffix: &str) -> Case {
    Case::new(&format!("{}.{coll}.{id}{suffix}", wall.id), annex).bound(&wall.id, &row_path(wall, coll, id, field), loc(&wall.label_en, &wall.label_de))
}

fn wall_case(annex: AnnexChoice, wall: &MasonryWall, field: &str, suffix: &str) -> Case {
    Case::new(&format!("{}{suffix}", wall.id), annex).bound(&wall.id, &wall_path(wall, field), loc(&wall.label_en, &wall.label_de))
}

struct Governing {
    n_min: f64,
    n_max: f64,
    w_ed: f64,
    n_ed_fi: f64,
}

fn governing(annex: AnnexChoice, wall: &MasonryWall, default_situation: DesignSituation, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass) -> Governing {
    let ns: Vec<f64> = wall.load_cases.iter().flat_map(|lc| design_effects(annex, wall, lc, default_situation, reliability_class)).map(|e| e.n_mid).collect();
    let n_min = ns.iter().copied().fold(f64::INFINITY, f64::min);
    let w_ed = wall
        .load_cases
        .iter()
        .map(|lc| {
            let exceptional = matches!(DesignSituation::from(lc.design_situation), DesignSituation::Accidental | DesignSituation::Seismic) || matches!(default_situation, DesignSituation::Accidental);
            gamma_q(annex, reliability_class, exceptional) * lc.q_p_wind_pa * lc.c_pe.abs()
        })
        .fold(0.0, f64::max);
    let n_ed_fi = wall
        .load_cases
        .iter()
        .map(|lc| {
            let (g, q_imp, _) = vertical_actions(lc);
            g + self_weight_n(wall) + psi1_imposed(annex, lc.imposed_category.as_str()) * q_imp
        })
        .fold(0.0, f64::max);
    Governing { n_min: if n_min.is_finite() { n_min.max(0.0) } else { 0.0 }, n_max: ns.iter().copied().fold(0.0, f64::max), w_ed, n_ed_fi }
}

/// 🧱 Adds the second leaf thickness and the cavity width of the wall to a cavity tie case when they are given (0 = not given, nothing is assumed).
fn second_leaf(wall: &MasonryWall, c: Case) -> Case {
    let c = if wall.second_leaf_thickness_m > 0.0 { c.with_n("t2_m", wall.second_leaf_thickness_m) } else { c };
    if wall.cavity_width_m > 0.0 { c.with_n("cavity_m", wall.cavity_width_m) } else { c }
}

fn tie_kind_is(t: &WallTie, kind: TieKind) -> bool {
    t.kind == kind
}

/// 🧰️ The catalogue entries evaluated for the member rows and the site fields of one wall.
pub(super) fn member_cases(annex: AnnexChoice, default_situation: DesignSituation, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass, wall: &MasonryWall, gamma_m: f64, f_k: f64) -> Vec<(&'static str, Case)> {
    let g = governing(annex, wall, default_situation, reliability_class);
    let f_d = f_k / gamma_m;
    let (t, l, h) = (wall.thickness_m, wall.length_m, wall.height_m);
    let fvk = f_vk0_mpa(annex, wall.unit_material, wall.mortar_type, wall.mortar_class) * 1e6;
    let e_masonry = 1000.0 * f_k;
    let mut out: Vec<(&'static str, Case)> = Vec::new();
    for p in &wall.piers {
        out.push(("M-15", row_case(annex, wall, "piers", &p.id, "widthM", "").with_path("pier_l_m", &row_path(wall, "piers", &p.id, "widthM")).with_n("pier_l_m", p.width_m).with_n("t_m", t).with_n("fd_pa", f_d).with_n("n_pier_n", g.n_max * p.width_m / l.max(1e-9))));
    }
    for lt in &wall.lintels {
        out.push(("M-15", row_case(annex, wall, "lintels", &lt.id, "spanM", "").with_path("opening_w_m", &row_path(wall, "lintels", &lt.id, "spanM")).with_n("pier_l_m", l).with_n("t_m", t).with_n("fd_pa", f_d).with_n("n_pier_n", 0.0).with_n("opening_w_m", lt.span_m).with_n("density", wall.density_kg_m3)));
        let load = wall.density_kg_m3 * GRAVITY * t * lt.span_m.powi(2) / 4.0;
        let deflection = load * lt.span_m.powi(3) / (60.0 * e_masonry * (t * lt.depth_m.powi(3) / 12.0).max(1e-12));
        out.push((
            "M-61",
            row_case(annex, wall, "lintels", &lt.id, "depthM", ".serviceability")
                .with_path("d_m", &row_path(wall, "lintels", &lt.id, "depthM"))
                .with_n("span_m", lt.span_m)
                .with_n("d_m", lt.depth_m)
                .with_n("deflection_m", deflection)
                .with_n("limit_ratio", wall.deflection_limit),
        ));
    }
    let cavity: Vec<&WallTie> = wall.ties.iter().filter(|x| tie_kind_is(x, TieKind::WallTie)).collect();
    if let Some(weakest) = cavity.iter().min_by(|a, b| a.capacity_n.total_cmp(&b.capacity_n)) {
        let rd = weakest.capacity_n;
        out.push((
            "M-50",
            second_leaf(wall, row_case(annex, wall, "ties", &weakest.id, "capacityN", ".cavity")
                .with_path("tie_rd_n", &row_path(wall, "ties", &weakest.id, "capacityN"))
                .with_n("face_m2", l * h)
                .with_n("n_per_m2", cavity.len() as f64 / (l * h).max(1e-9))
                .with_n("h_wall_m", h)
                .with_n("w_ed_pa", g.w_ed)
                .with_n("t1_m", t)
                .with_n("tie_rd_n", rd)),
        ));
    }
    let anchors: Vec<&WallTie> = wall.ties.iter().filter(|x| tie_kind_is(x, TieKind::Anchor)).collect();
    let rings: Vec<&WallTie> = wall.ties.iter().filter(|x| tie_kind_is(x, TieKind::RingTie)).collect();
    if let Some(first) = anchors.first().or(rings.first()) {
        let ring_path = rings.first().map(|r| row_path(wall, "ties", &r.id, "capacityN"));
        let rk = anchors.iter().map(|x| x.capacity_n).fold(f64::INFINITY, f64::min);
        let mut ordered = anchors.clone();
        ordered.sort_by(|a, b| a.position_m.total_cmp(&b.position_m));
        let positions: Vec<f64> = ordered.iter().map(|a| a.position_m).collect();
        let spacing = positions.windows(2).map(|w| w[1] - w[0]).chain(positions.first().map(|x| 2.0 * x)).chain(positions.last().map(|x| 2.0 * (l - x))).fold(0.0, f64::max);
        out.push((
            "M-51",
            row_case(annex, wall, "ties", &first.id, "capacityN", ".anchor")
                .with_v("pos_m", &positions)
                .with_n("l_m", l)
                .with_paths(ordered.iter().take(ROW_KEYS.len()).enumerate().map(|(i, a)| (ROW_KEYS[i], row_path(wall, "ties", &a.id, "positionM"))))
                .with_path("anchor_rk_n", &row_path(wall, "ties", &first.id, "capacityN"))
                .with_paths(ring_path.map(|path| ("ring_tie_n", path)))
                .with_n("w_ed_pa", g.w_ed)
                .with_n("h_trib_m", h / 2.0)
                .with_n("s_anchor_m", if anchors.is_empty() { l } else { spacing })
                .with_n("anchor_rk_n", if rk.is_finite() { rk } else { 0.0 })
                .with_n("ring_tie_n", rings.iter().map(|x| x.capacity_n).sum::<f64>())
                .with_n("gamma_m", gamma_m),
        ));
    }
    for p in &wall.prestress {
        out.push((
            "M-44",
            row_case(annex, wall, "prestress", &p.id, "forceN", "")
                .with_path("p0_n", &row_path(wall, "prestress", &p.id, "forceN"))
                .with_path("t_m", &wall_path(wall, "thicknessM"))
                .with_n("t_m", t)
                .with_n("p0_n", p.force_n)
                .with_n("ap_m2", p.area_m2)
                .with_n("fpk_pa", p.f_pk_pa)
                .with_n("ep_pa", E_P_PA)
                .with_n("e_pa", e_masonry)
                .with_n("phi_inf", wall.phi_infinity)
                .with_n("rho_relax", p.loss_ratio)
                .with_n("a_m2", area_m2(wall))
                .with_n("n_ed_n", g.n_min)
                .with_n("fd_pa", f_d)
                .with_n("fxk1_pa", f_xk_pa(1, wall))
                .with_n("z_m3", l * t * t / 6.0),
        ));
    }
    if let Some(first) = wall.confined.first() {
        let mut xs: Vec<f64> = std::iter::once(0.0).chain(wall.confined.iter().map(|c| c.position_m)).chain(std::iter::once(l)).collect();
        xs.sort_by(f64::total_cmp);
        let spacing = xs.windows(2).map(|w| w[1] - w[0]).fold(0.0, f64::max);
        let least = wall.confined.iter().min_by(|a, b| a.width_m.min(a.depth_m).total_cmp(&b.width_m.min(b.depth_m))).unwrap_or(first);
        let smallest = wall.confined.iter().min_by(|a, b| (a.width_m * a.depth_m).total_cmp(&(b.width_m * b.depth_m))).unwrap_or(first);
        let mut ordered: Vec<_> = wall.confined.iter().collect();
        ordered.sort_by(|a, b| a.position_m.total_cmp(&b.position_m));
        out.push((
            "M-43",
            row_case(annex, wall, "confined", &first.id, "asM2", "")
                .with_path("col_as_m2", &row_path(wall, "confined", &first.id, "asM2"))
                .with_path("t_m", &wall_path(wall, "thicknessM"))
                .with_path("col_min_m", &row_path(wall, "confined", &least.id, if least.width_m <= least.depth_m { "widthM" } else { "depthM" }))
                .with_v("pos_m", &ordered.iter().map(|c| c.position_m).collect::<Vec<_>>())
                .with_paths(ordered.iter().take(ROW_KEYS.len()).enumerate().map(|(i, c)| (ROW_KEYS[i], row_path(wall, "confined", &c.id, "positionM"))))
                .with_path("col_a_side_m", &row_path(wall, "confined", &smallest.id, if smallest.width_m <= smallest.depth_m { "widthM" } else { "depthM" }))
                .with_n("col_a_side_m", smallest.width_m.min(smallest.depth_m))
                .with_n("col_a_other_m", smallest.width_m.max(smallest.depth_m))
                .with_n("col_a_m2", wall.confined.iter().map(|c| c.width_m * c.depth_m).fold(f64::INFINITY, f64::min))
                .with_n("col_min_m", wall.confined.iter().map(|c| c.width_m.min(c.depth_m)).fold(f64::INFINITY, f64::min))
                .with_n("col_as_m2", wall.confined.iter().map(|c| c.as_m2).fold(f64::INFINITY, f64::min))
                .with_n("col_spacing_m", spacing)
                .with_n("v_ed_n", wall.in_plane_shear_n)
                .with_n("l_m", l)
                .with_n("t_m", t)
                .with_n("fvk_pa", fvk)
                .with_n("fd_pa", f_d)
                .with_n("n_ed_n", g.n_min),
        ));
    }
    if wall.in_plane_shear_n > 0.0 {
        let mut c = wall_case(annex, wall, "inPlaneShearN", ".inplane")
            .with_path("v_ed_n", &wall_path(wall, "inPlaneShearN"))
            .with_path("t_m", &wall_path(wall, "thicknessM"))
            .with_path("l_m", &wall_path(wall, "lengthM"))
            .with_n("l_m", l)
            .with_n("t_m", t)
            .with_n("n_ed_n", g.n_min)
            .with_n("m_ed_nm", wall.in_plane_moment_nm)
            .with_n("v_ed_n", wall.in_plane_shear_n)
            .with_n("fb_pa", f_b_normalised_pa(wall))
            .with_n("fm_pa", f_m_mpa(wall) * 1e6)
            .with_n("fd_pa", f_d)
            .with_s("material", material_key(wall.unit_material))
            .with_s("mortar", mortar_key(wall.mortar_type))
            .with_n("group", group_number(wall.unit_group))
            .with_n("gamma_m", gamma_m);
        if !wall.flanges.is_empty() {
            let n = wall.flanges.len() as f64;
            c = c.with_n("flange_t_m", wall.flanges.iter().map(|f| f.thickness_m).sum::<f64>() / n).with_v("flange_len_m", &wall.flanges.iter().map(|f| f.length_m).collect::<Vec<_>>()).with_n("height_total_m", h).with_n("flanges", n);
        }
        if wall.web_spacing_m > 0.0 {
            c = c.with_n("web_spacing_m", wall.web_spacing_m);
        }
        out.push(("M-20", c));
        if let Some(bar) = wall.bars.iter().find(|b| b.kind == BarKind::Shear) {
            out.push((
                "M-41",
                row_case(annex, wall, "bars", &bar.id, "spacingM", "")
                    .with_path("s_m", &row_path(wall, "bars", &bar.id, "spacingM"))
                    .with_n("b_m", t)
                    .with_n("d_m", D_OVER_L * l)
                    .with_n("fvk_pa", fvk)
                    .with_n("fd_pa", f_d)
                    .with_n("v_ed_n", wall.in_plane_shear_n)
                    .with_n("asw_m2", PI * bar.diameter_m.powi(2) / 4.0)
                    .with_n("s_m", bar.spacing_m)
                    .with_n("av_m", h / 2.0)
                    .with_n("fyd_pa", wall.f_yd_pa.max(1.0)),
            ));
        }
    }
    for bar in wall.bars.iter().filter(|b| b.kind == BarKind::Anchorage) {
        out.push((
            "M-45",
            row_case(annex, wall, "bars", &bar.id, "lengthM", "")
                .with_path("provided_m", &row_path(wall, "bars", &bar.id, "lengthM"))
                .with_n("diameter_m", bar.diameter_m)
                .with_n("provided_m", bar.length_m)
                .with_n("fyd_pa", wall.f_yd_pa.max(1.0))
                .with_n("gamma_m", gamma_m),
        ));
        out.push((
            "M-05",
            row_case(annex, wall, "bars", &bar.id, "lengthM", ".bond")
                .advisory()
                .with_path("lb_avail_m", &row_path(wall, "bars", &bar.id, "lengthM"))
                .with_n("diameter_m", bar.diameter_m)
                .with_n("lb_avail_m", bar.length_m)
                .with_n("fyd_pa", wall.f_yd_pa.max(1.0))
                .with_n("gamma_m", gamma_m),
        ));
    }
    let deep: Vec<&WallBar> = wall.bars.iter().filter(|b| b.kind == BarKind::DeepBeam).collect();
    if !deep.is_empty() && wall.in_plane_moment_nm > 0.0 {
        out.push((
            "M-42",
            row_case(annex, wall, "bars", &deep[0].id, "diameterM", ".deep")
                .with_n("b_m", t)
                .with_n("l_m", l)
                .with_n("h_m", h)
                .with_n("m_ed_nm", wall.in_plane_moment_nm)
                .with_n("n_ed_n", g.n_min)
                .with_n("fd_pa", f_d)
                .with_n("fyd_pa", wall.f_yd_pa.max(1.0))
                .with_v("bar_a_m2", &deep.iter().map(|b| PI * b.diameter_m.powi(2) / 4.0).collect::<Vec<_>>())
                .with_v("bar_y_m", &deep.iter().map(|_| (h - wall.cover_m).max(0.0)).collect::<Vec<_>>()),
        ));
    }
    let frost = match wall.frost_class {
        FrostClass::Exempt => 0.0,
        FrostClass::Moderate => 1.0,
        FrostClass::Severe => 2.0,
    };
    let mut durability = wall_case(annex, wall, "frostClass", ".durability")
        .with_path("frost_class", &wall_path(wall, "frostClass"))
        .with_path("fm_pa", &wall_path(wall, "mortarStrengthPa"))
        .with_n("fm_pa", f_m_mpa(wall) * 1e6)
        .with_s("exposure", &format!("{:?}", wall.exposure).to_ascii_lowercase())
        .with_s("material", material_key(wall.unit_material))
        .with_n("frost_class", frost);
    if wall.reinforced {
        durability = durability.with_n("cover_m", wall.cover_m).with_path("cover_m", &wall_path(wall, "coverM"));
    }
    out.push(("M-60", durability));
    if wall.is_basement && wall.soil_height_m > 0.0 {
        out.push((
            "M-34",
            wall_case(annex, wall, "soilHeightM", ".soil")
                .with_path("he_m", &wall_path(wall, "soilHeightM"))
                .with_n("h_m", h)
                .with_n("t_m", t)
                .with_n("he_m", wall.soil_height_m)
                .with_n("rho_e_n_m3", wall.soil_density_kg_m3 * GRAVITY)
                .with_n("n_min_n", g.n_min)
                .with_n("n_max_n", g.n_max)
                .with_n("fd_pa", f_d),
        ));
    }
    if wall.isotherm_depth_m > 0.0 {
        out.push((
            "M-91",
            wall_case(annex, wall, "isothermDepthM", ".isotherm")
                .with_path("a_theta_m", &wall_path(wall, "isothermDepthM"))
                .with_path("t_m", &wall_path(wall, "thicknessM"))
                .with_n("t_m", t)
                .with_n("l_m", l)
                .with_n("a_theta_m", wall.isotherm_depth_m)
                .with_n("k_theta", K_THETA)
                .with_n("fd_pa", f_d)
                .with_n("n_ed_fi_n", g.n_ed_fi),
        ));
    }
    out
}

/// 🧭 The bracing layout case (M-21) of the shear walls with plan positions, or `None` when the document has none.
pub fn bracing_case(annex: AnnexChoice, walls: &[MasonryWall]) -> Option<Case> {
    let shear: Vec<&MasonryWall> = walls.iter().filter(|w| matches!(w.wall_type, WallType::Shear) && w.in_plane_shear_n > 0.0).collect();
    let first = shear.first()?;
    let (xs, ls): (Vec<f64>, Vec<f64>) = shear.iter().map(|w| (w.plan_x_m, w.length_m)).unzip();
    let extent = xs.iter().zip(&ls).map(|(x, l)| x + l).fold(f64::MIN, f64::max) - xs.iter().copied().fold(f64::MAX, f64::min);
    let ids: Vec<&str> = shear.iter().map(|w| w.id.as_str()).collect();
    let flexible = crate::diaphragms(annex, walls).into_iter().find(|d| d.class == crate::DiaphragmClass::Flexible && d.walls.iter().any(|b| b.wall_id == first.id)).and_then(|d| d.tributary_of(&ids));
    let case = Case::new("bracing", annex)
            .bound(&first.id, &wall_path(first, "planXM"), loc(&first.label_en, &first.label_de))
            .with_path("x_m", &wall_path(first, "planXM"))
            .with_paths(shear.iter().enumerate().take(WALL_LENGTH_KEYS.len()).map(|(i, w)| (WALL_LENGTH_KEYS[i], wall_path(w, "lengthM"))))
            .with_v("l_m", &ls)
            .with_v("t_m", &shear.iter().map(|w| w.thickness_m).collect::<Vec<_>>())
            .with_v("x_m", &xs)
            .with_v("vrd_n", &shear.iter().map(|w| f_vk0_mpa(annex, w.unit_material, w.mortar_type, w.mortar_class) * 1e6 / 1.5 * w.thickness_m * w.length_m).collect::<Vec<_>>())
            .with_n("h_m", shear.iter().map(|w| w.height_m).fold(0.0, f64::max))
            .with_n("e_pa", 1000.0 * f_k_pa(annex, first))
            .with_n("v_ed_n", shear.iter().map(|w| w.in_plane_shear_n).sum::<f64>())
            .with_n("b_m", extent.max(0.0));
    Some(match flexible {
        Some(fractions) => case.with_s("diaphragm", "flexible").with_v("trib", &fractions),
        None => case,
    })
}
