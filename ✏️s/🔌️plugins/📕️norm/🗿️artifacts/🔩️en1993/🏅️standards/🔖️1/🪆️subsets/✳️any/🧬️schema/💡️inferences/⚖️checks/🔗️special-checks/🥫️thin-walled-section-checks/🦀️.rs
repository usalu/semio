//! 🥫️ Open thin-walled section constants of the cold-formed members (EN 1993-1-3 and the chords of built-up members) from the centreline walls by the mechanics kernel `analyse_thin_walled`: I_t, I_w and the shear centre replace entered values and closed-form approximations. See [EN 1993-1-3](https://eurocodes.jrc.ec.europa.eu/).

use super::*;
use semio_s_artifact_norm_en1993_model::{ColdFormedMember, PlatedPanel, StiffenerShape};
use semio_s_plugin_norm_mechanics::{self as mechanics, ThinWall};

/// 🪶️ Constants of an open thin-walled section from the kernel (SI, `y`/`z` axes of the walls, shear centre relative to the centroid).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThinConstants {
    pub area: f64,
    pub iy: f64,
    pub iz: f64,
    pub it: f64,
    pub iw: f64,
    pub y_sc: f64,
    pub z_sc: f64,
    pub centroid_y: f64,
    pub centroid_z: f64,
}

/// 🪶️ Runs `analyse_thin_walled` on the walls; `None` for walls the kernel refuses (closed cells, disconnected walls, non-positive thickness).
pub fn thin_constants(walls: &[ThinWall]) -> Option<ThinConstants> {
    let steel = mechanics::Material { id: "steel".into(), e: 210.0e9, g: 81.0e9, rho: 7850.0 };
    let c = mechanics::analyse_thin_walled(walls, &steel).ok()?;
    Some(ThinConstants { area: c.area, iy: c.iy, iz: c.iz, it: c.j, iw: c.iw, y_sc: c.shear_centre_y - c.centroid_y, z_sc: c.shear_centre_z - c.centroid_z, centroid_y: c.centroid_y, centroid_z: c.centroid_z })
}

fn wall(y1: f64, z1: f64, y2: f64, z2: f64, thickness: f64) -> ThinWall {
    ThinWall { y1, z1, y2, z2, thickness }
}

/// 🥫️ Centreline walls of a lipped channel (EN 1993-1-3 notional widths): web of slant height `hw` at the web inclination `phi_deg` (0 = vertical), flanges `b1` and `b2` and edge folds `cp` perpendicular to the flanges towards the web side.
pub fn lipped_channel(hw: f64, phi_deg: f64, b1: f64, b2: f64, cp: f64, t: f64) -> Vec<ThinWall> {
    let phi = if phi_deg > 0.0 && phi_deg < 180.0 { phi_deg.to_radians() } else { std::f64::consts::FRAC_PI_2 };
    let (dy, dz) = (hw * phi.cos(), hw * phi.sin());
    let mut walls = vec![wall(0.0, 0.0, dy, dz, t), wall(0.0, 0.0, b1, 0.0, t), wall(dy, dz, dy + b2, dz, t)];
    if cp > 0.0 {
        walls.push(wall(b1, 0.0, b1, cp, t));
        walls.push(wall(dy + b2, dz, dy + b2, dz - cp, t));
    }
    walls
}

/// 🥫️ Walls of the centreline of the cold-formed member row (web, both flanges and edge folds); `None` for rows without section dimensions.
pub fn cold_formed_walls(c: &ColdFormedMember) -> Option<Vec<ThinWall>> {
    (c.thickness > 0.0 && c.web_height > 0.0 && c.flange_width > 0.0).then(|| {
        let b2 = if c.opposite_flange_width > 0.0 { c.opposite_flange_width } else { c.flange_width };
        lipped_channel(c.web_height, c.web_slope, c.flange_width, b2, c.lip_length, c.thickness)
    })
}

/// 🧱️ Solid outline of the transverse stiffener in plate coordinates (`z` from the plate surface, the attachment point is the origin, counter-clockwise): a flat bar, a T section (flange centred on the web top) or an angle (second leg at the free end of the first), the geometry the 3D view extrudes. `None` for rows without a valid outline.
pub fn stiffener_outline(p: &PlatedPanel) -> Option<Vec<[f64; 2]>> {
    let (h, t, bf, tf) = (p.stiffener_depth, p.stiffener_thickness, p.stiffener_flange_width, p.stiffener_flange_thickness);
    if h <= 0.0 || t <= 0.0 {
        return None;
    }
    let flanged = bf > 0.0 && tf > 0.0 && tf < 2.0 * h;
    match p.stiffener_shape {
        StiffenerShape::Flat => Some(vec![[-t / 2.0, 0.0], [t / 2.0, 0.0], [t / 2.0, h], [-t / 2.0, h]]),
        StiffenerShape::Tee if flanged => Some(vec![[-t / 2.0, 0.0], [t / 2.0, 0.0], [t / 2.0, h - tf / 2.0], [bf / 2.0, h - tf / 2.0], [bf / 2.0, h + tf / 2.0], [-bf / 2.0, h + tf / 2.0], [-bf / 2.0, h - tf / 2.0], [-t / 2.0, h - tf / 2.0]]),
        StiffenerShape::Angle if flanged => Some(vec![[-t / 2.0, 0.0], [t / 2.0, 0.0], [t / 2.0, h - tf / 2.0], [bf, h - tf / 2.0], [bf, h + tf / 2.0], [-t / 2.0, h + tf / 2.0]]),
        _ => None,
    }
}

/// 🧱️ I_T, I_w and the polar moment of inertia I_p about the attachment point of the stiffener outline (EN 1993-1-5 9.2.1(8)): the finite-element section solver `analyse_section` on the solid outline (converged against sectionproperties to 0.5 %), I_p = I_y + I_z + A (y_c² + z_c²). The thick bar (h/t = 10) is why the centreline model of `analyse_thin_walled` is not used here: it is 7 % above in I_t and double counts the web/flange overlap in I_p. `None` without a valid outline.
pub fn stiffener_constants(p: &PlatedPanel) -> Option<(ThinConstants, f64)> {
    let steel = mechanics::Material { id: "steel".into(), e: 210.0e9, g: 81.0e9, rho: 7850.0 };
    let c = mechanics::analyse_section(&stiffener_outline(p)?, &[], &steel).ok()?;
    let k = ThinConstants { area: c.area, iy: c.iy, iz: c.iz, it: c.j, iw: c.iw, y_sc: c.shear_centre_y - c.centroid_y, z_sc: c.shear_centre_z - c.centroid_z, centroid_y: c.centroid_y, centroid_z: c.centroid_z };
    Some((k, k.iy + k.iz + k.area * (k.centroid_y.powi(2) + k.centroid_z.powi(2))))
}

/// 🥫️ Appends the thin-walled section row of one cold-formed member: I_t, I_w and the shear centre of its lipped centreline section (informative).
pub fn check_cold_formed_section(c: &ColdFormedMember, annex: AnnexChoice, report: &mut CheckReport) {
    let Some(k) = cold_formed_walls(c).and_then(|w| thin_constants(&w)) else { return };
    let subject = SubjectRef::new(&c.id, format!("coldFormedMembers[id={}].webHeight", c.id), loc("Section dimensions", "Querschnittsabmessungen"));
    let explanation = loc(
        &format!("Open thin-walled centreline section of web {:.1} mm, flanges {:.1}/{:.1} mm, edge folds {:.1} mm, t={:.2} mm (kernel analyse_thin_walled, web inclination {:.0}°): A={:.1} mm², I_y={:.3e} mm⁴, I_z={:.3e} mm⁴, I_t={:.3e} mm⁴, I_w={:.3e} mm⁶, shear centre y_sc={:.1} mm, z_sc={:.1} mm from the centroid.", c.web_height * 1e3, c.flange_width * 1e3, if c.opposite_flange_width > 0.0 { c.opposite_flange_width } else { c.flange_width } * 1e3, c.lip_length * 1e3, c.thickness * 1e3, if c.web_slope > 0.0 { c.web_slope } else { 90.0 }, k.area * 1e6, k.iy * 1e12, k.iz * 1e12, k.it * 1e12, k.iw * 1e18, k.y_sc * 1e3, k.z_sc * 1e3),
        &format!("Offener dünnwandiger Mittellinienquerschnitt mit Steg {:.1} mm, Flanschen {:.1}/{:.1} mm, Randlippen {:.1} mm, t={:.2} mm (Kern analyse_thin_walled, Stegneigung {:.0}°): A={:.1} mm², I_y={:.3e} mm⁴, I_z={:.3e} mm⁴, I_t={:.3e} mm⁴, I_w={:.3e} mm⁶, Schubmittelpunkt y_sc={:.1} mm, z_sc={:.1} mm vom Schwerpunkt.", c.web_height * 1e3, c.flange_width * 1e3, if c.opposite_flange_width > 0.0 { c.opposite_flange_width } else { c.flange_width } * 1e3, c.lip_length * 1e3, c.thickness * 1e3, if c.web_slope > 0.0 { c.web_slope } else { 90.0 }, k.area * 1e6, k.iy * 1e12, k.iz * 1e12, k.it * 1e12, k.iw * 1e18, k.y_sc * 1e3, k.z_sc * 1e3),
    );
    report.push(
        CheckResult::assess(format!("en1993.1-3.section.fe.{}", c.id), "DIN EN 1993-1-3", ClauseId::new("EN 1993-1-3", "5", "5.1"), subject, loc("Torsion constants of the cold-formed section", "Torsionskennwerte des Kaltprofils"))
            .annex(annex)
            .informative(dimensionless(k.it * 1e12))
            .explanation(explanation)
            .build(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_lipped_channel_matches_the_sectionproperties_thin_walled_oracle() {
        let k = thin_constants(&lipped_channel(0.2, 0.0, 0.075, 0.075, 0.02, 0.002)).expect("section");
        let close = |got: f64, want: f64, what: &str| assert!((got - want).abs() / want.abs() < 0.02, "{what} {got} vs {want}");
        let exact = |got: f64, want: f64, what: &str| assert!((got - want).abs() / want.abs() < 1e-6, "{what} {got} vs {want}");
        exact(k.area, (0.2 + 2.0 * 0.075 + 2.0 * 0.02) * 0.002, "A = sum l t");
        exact(k.it, (0.2 + 2.0 * 0.075 + 2.0 * 0.02) * 0.002_f64.powi(3) / 3.0, "I_t = sum l t^3 / 3");
        close(k.iw, 5.112_886e-9, "I_w");
        let (h, b, cp, wall_t) = (0.2, 0.075, 0.02, 0.002);
        let yc = (b * b + 2.0 * cp * b) / (h + 2.0 * b + 2.0 * cp);
        let strips = |got: f64, want: f64, what: &str| assert!((got - want).abs() / want.abs() < 1e-3, "{what} {got} vs {want}");
        strips(k.iy, wall_t * (h.powi(3) / 12.0 + 2.0 * b * (h / 2.0).powi(2) + 2.0 * (cp.powi(3) / 12.0 + cp * (h / 2.0 - cp / 2.0).powi(2))), "I_y of the thin strips");
        strips(k.iz, wall_t * (h * yc * yc + 2.0 * (b.powi(3) / 12.0 + b * (b / 2.0 - yc).powi(2)) + 2.0 * cp * (b - yc).powi(2)), "I_z of the thin strips");
        close(k.y_sc, -0.056_445_46, "y_sc");
    }

    #[test]
    fn the_cold_formed_showcase_row_reports_the_kernel_torsion_constants() {
        let doc = En1993Snapshot::special_parts_showcase();
        let mut report = CheckReport::default();
        check_cold_formed_section(doc.cold_formed_members.iter().find(|c| c.id == "cf-1").unwrap(), AnnexChoice::En, &mut report);
        let check = report.checks.iter().find(|c| c.id == "en1993.1-3.section.fe.cf-1").expect("row");
        assert!(check.explanation.en.contains("analyse_thin_walled") && check.explanation.de.contains("Kern analyse_thin_walled"));
        let k = thin_constants(&cold_formed_walls(doc.cold_formed_members.iter().find(|c| c.id == "cf-1").unwrap()).unwrap()).unwrap();
        assert!(k.it > 0.0 && k.iw > 0.0);
    }

    #[test]
    fn the_stiffener_outline_matches_the_converged_sectionproperties_oracle() {
        let mut p = En1993Snapshot::special_parts_showcase().plated_panels[0].clone();
        let close = |got: f64, want: f64, tolerance: f64, what: &str| assert!((got - want).abs() / want.abs() < tolerance, "{what} {got} vs {want}");
        let cases = [
            (StiffenerShape::Flat, 0.0, 0.0, 3.123_252e-8, 3.341_667e-6, 6.642_911e-12),
            (StiffenerShape::Tee, 0.08, 0.012, 7.604_844e-8, 1.289_997e-5, 3.231_973e-11),
            (StiffenerShape::Angle, 0.06, 0.012, 6.559_759e-8, 1.145_031e-5, 3.674_157e-11),
        ];
        for (shape, bf, tf, it, polar, iw) in cases {
            (p.stiffener_shape, p.stiffener_depth, p.stiffener_thickness) = (shape, 0.1, 0.01);
            (p.stiffener_flange_width, p.stiffener_flange_thickness) = (bf, tf);
            let (k, ip) = stiffener_constants(&p).expect("outline");
            close(k.it, it, 0.005, &format!("{shape:?} I_t"));
            close(ip, polar, 0.005, &format!("{shape:?} I_p"));
            close(k.iw, iw, 0.01, &format!("{shape:?} I_w"));
        }
        p.stiffener_shape = StiffenerShape::None;
        assert!(stiffener_constants(&p).is_none());
    }

    #[test]
    fn walls_the_kernel_refuses_give_no_constants() {
        assert!(thin_constants(&[]).is_none());
        assert!(thin_constants(&lipped_channel(0.2, 0.0, 0.075, 0.075, 0.02, 0.0)).is_none());
    }
}
