//! 📐️ Parametric section checks: the section properties computed from the dimensions against the section row they describe (fillets and root radii explain a few percent). See [EN 1993-1-1](https://eurocodes.jrc.ec.europa.eu/).

use super::*;
use crate::clauses::parametric_sections::{angle_principal, chs, max_deviation, rhs, welded_i, Properties};
use semio_s_artifact_norm_en1993_model::ParametricSection;
use semio_s_plugin_norm_mechanics::{self as mechanics, section::profiles};

const CHS_POINTS: usize = 96;

/// 🌀️ Saint-Venant torsion constant, warping constant and shear centre (relative to the centroid, y and z) of a section from the finite-element solver of the mechanics kernel (SI).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TorsionConstants {
    pub it: f64,
    pub iw: f64,
    pub y_sc: f64,
    pub z_sc: f64,
    pub triangles: usize,
}

fn outline(p: &ParametricSection) -> Option<profiles::Outline> {
    let (h, b, tw, tf) = (p.height, p.width, p.web_thickness, p.flange_thickness);
    match p.shape {
        semio_s_artifact_norm_en1993_model::ParametricShape::Rhs => (2.0 * tw < h.min(b)).then(|| profiles::rectangular_hollow_section(h, b, tw, 0.0, 1)),
        semio_s_artifact_norm_en1993_model::ParametricShape::Chs => (2.0 * tw < h).then(|| profiles::circular_hollow_section(h, tw, CHS_POINTS)),
        semio_s_artifact_norm_en1993_model::ParametricShape::Angle => (tw > 0.0 && tw < h.min(b)).then(|| profiles::angle_section(h, b, tw, 0.0, 0.0, 1)),
        semio_s_artifact_norm_en1993_model::ParametricShape::WeldedI => (tf > 0.0 && 2.0 * tf < h && tw < b).then(|| profiles::i_section(h, b, tf, tw, 0.0, 1)),
    }
}

/// 🌀️ I_t, I_w and the shear centre of the sharp-corner outline of a parametric section by the kernel `analyse_section`, replacing the thin-walled closed forms; `None` for dimensions that give no outline.
pub fn torsion_constants(p: &ParametricSection) -> Option<TorsionConstants> {
    let (outer, holes) = outline(p)?;
    let steel = mechanics::Material { id: "steel".into(), e: 210.0e9, g: 81.0e9, rho: 7850.0 };
    let c = mechanics::analyse_section(&outer, &holes, &steel).ok()?;
    Some(TorsionConstants { it: c.j, iw: c.iw, y_sc: c.shear_centre_y - c.centroid_y, z_sc: c.shear_centre_z - c.centroid_z, triangles: c.triangles })
}

fn subject(p: &ParametricSection, field: &str, en: &str, de: &str) -> SubjectRef {
    SubjectRef::new(&p.id, format!("parametricSections[id={}].{field}", p.id), loc(en, de))
}

fn props_of(s: &SteelSection) -> Properties {
    Properties { area: s.area * 1e6, iy: s.iy * 1e12, iz: s.iz * 1e12, w_el_y: s.w_el_y * 1e9, w_pl_y: s.w_pl_y * 1e9 }
}

fn computed(p: &ParametricSection) -> Properties {
    let (h, b, tw, tf) = (p.height * 1e3, p.width * 1e3, p.web_thickness * 1e3, p.flange_thickness * 1e3);
    match p.shape {
        semio_s_artifact_norm_en1993_model::ParametricShape::Rhs => rhs(h, b, tw),
        semio_s_artifact_norm_en1993_model::ParametricShape::Chs => chs(h, tw),
        semio_s_artifact_norm_en1993_model::ParametricShape::Angle => angle_principal(h.max(b), h.min(b), tw),
        semio_s_artifact_norm_en1993_model::ParametricShape::WeldedI => welded_i(h, b, tw, tf),
    }
}

/// 📐️ Appends the parametric section checks of every admitted row.
pub fn check_parametric_sections(document: &En1993Snapshot, annex: AnnexChoice, scope: Scope<'_>, report: &mut CheckReport) {
    for p in document.parametric_sections.iter().filter(|p| scope.admits("parametricSections", &p.id)) {
        let Some(section) = document.sections.iter().find(|s| s.id == p.section_id) else { continue };
        if p.height <= 0.0 || p.web_thickness <= 0.0 || p.tolerance <= 0.0 {
            continue;
        }
        let (c, r) = (computed(p), props_of(section));
        let deviation = max_deviation(&c, &r);
        let explanation = loc(
            &format!("From the dimensions: A={:.0} mm² (row {:.0}), I_y={:.3e} mm⁴ (row {:.3e}), I_z={:.3e} mm⁴ (row {:.3e}), W_el,y={:.3e} mm³ (row {:.3e}), W_pl,y={:.3e} mm³ (row {:.3e}); largest deviation {:.2} % against the tolerance {:.2} %.", c.area, r.area, c.iy, r.iy, c.iz, r.iz, c.w_el_y, r.w_el_y, c.w_pl_y, r.w_pl_y, deviation * 100.0, p.tolerance * 100.0),
            &format!("Aus den Abmessungen: A={:.0} mm² (Zeile {:.0}), I_y={:.3e} mm⁴ (Zeile {:.3e}), I_z={:.3e} mm⁴ (Zeile {:.3e}), W_el,y={:.3e} mm³ (Zeile {:.3e}), W_pl,y={:.3e} mm³ (Zeile {:.3e}); größte Abweichung {:.2} % gegen die Toleranz {:.2} %.", c.area, r.area, c.iy, r.iy, c.iz, r.iz, c.w_el_y, r.w_el_y, c.w_pl_y, r.w_pl_y, deviation * 100.0, p.tolerance * 100.0),
        );
        let mut builder = CheckResult::assess(format!("en1993.sections.param.{}", p.id), "DIN EN 1993-1-1", ClauseId::new("EN 1993-1-1", "6", "6.2.5"), subject(p, "height", "Dimensions", "Abmessungen"), loc("Section properties from the dimensions agree with the section row", "Querschnittswerte aus den Abmessungen stimmen mit dem Querschnitt überein"))
            .annex(annex)
            .utilization(dimensionless(deviation), dimensionless(p.tolerance))
            .explanation(explanation);
        if deviation > p.tolerance {
            let options: Vec<String> = rolled_heb_catalogue().into_iter().filter(|s| max_deviation(&c, &props_of(s)) <= p.tolerance).map(|s| s.designation).collect();
            builder = builder.remedy(Remedy::one_of(subject(p, "sectionId", "Described section", "Beschriebener Querschnitt"), options.clone(), loc(&format!("Describe one of the catalogue sections {} or correct the dimensions.", options.join(", ")), &format!("Einen der Katalogquerschnitte {} beschreiben oder die Abmessungen korrigieren.", options.join(", ")))));
        }
        report.push(builder.build());
        torsion_row(p, section, annex, report);
    }
}

fn torsion_row(p: &ParametricSection, section: &SteelSection, annex: AnnexChoice, report: &mut CheckReport) {
    let Some(fe) = torsion_constants(p) else { return };
    let ratio = if section.it > 0.0 { fe.it / section.it } else { 0.0 };
    let wratio = if section.iw > 0.0 { fe.iw / section.iw } else { 0.0 };
    let explanation = loc(
        &format!("Finite-element section analysis of the sharp-corner outline ({} triangles): I_t={:.4e} m⁴ (section row {:.4e}, ratio {ratio:.3}), I_w={:.4e} m⁶ (row {:.4e}, ratio {wratio:.3}), shear centre y_sc={:.2} mm, z_sc={:.2} mm from the centroid. Root fillets of rolled rows raise I_t above the sharp-corner value; the row values are not replaced.", fe.triangles, fe.it, section.it, fe.iw, section.iw, fe.y_sc * 1e3, fe.z_sc * 1e3),
        &format!("Finite-Elemente-Querschnittsberechnung der scharfkantigen Kontur ({} Dreiecke): I_t={:.4e} m⁴ (Querschnittszeile {:.4e}, Verhältnis {ratio:.3}), I_w={:.4e} m⁶ (Zeile {:.4e}, Verhältnis {wratio:.3}), Schubmittelpunkt y_sc={:.2} mm, z_sc={:.2} mm vom Schwerpunkt. Ausrundungen gewalzter Zeilen erhöhen I_t über den scharfkantigen Wert; die Zeilenwerte werden nicht ersetzt.", fe.triangles, fe.it, section.it, fe.iw, section.iw, fe.y_sc * 1e3, fe.z_sc * 1e3),
    );
    report.push(
        CheckResult::assess(format!("en1993.sections.param.fe.{}", p.id), "DIN EN 1993-1-1", ClauseId::new("EN 1993-1-1", "6", "6.2.7"), subject(p, "height", "Dimensions", "Abmessungen"), loc("Torsion constants from the finite-element section analysis", "Torsionskennwerte aus der Finite-Elemente-Querschnittsberechnung"))
            .annex(annex)
            .informative(dimensionless(ratio))
            .explanation(explanation)
            .build(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parametric(shape: semio_s_artifact_norm_en1993_model::ParametricShape, h: f64, b: f64, tw: f64, tf: f64) -> ParametricSection {
        ParametricSection { id: "p".into(), section_id: "s".into(), label: "p".into(), shape, height: h, width: b, web_thickness: tw, flange_thickness: tf, tolerance: 0.05 }
    }

    #[test]
    fn the_welded_i_matches_the_sectionproperties_oracle() {
        let c = torsion_constants(&parametric(semio_s_artifact_norm_en1993_model::ParametricShape::WeldedI, 0.24, 0.24, 0.01, 0.017)).expect("outline");
        assert!((c.it - 8.346_058_351e-7).abs() / 8.346_058_351e-7 < 0.01, "I_t {}", c.it);
        assert!((c.iw - 4.867_436_581e-7).abs() / 4.867_436_581e-7 < 0.01, "I_w {}", c.iw);
        assert!(c.y_sc.abs() < 1e-5 && c.z_sc.abs() < 1e-5);
    }

    #[test]
    fn the_rectangular_hollow_section_matches_the_sectionproperties_oracle() {
        let c = torsion_constants(&parametric(semio_s_artifact_norm_en1993_model::ParametricShape::Rhs, 0.2, 0.1, 0.008, 0.0)).expect("outline");
        assert!((c.it - 1.807_580_097e-5).abs() / 1.807_580_097e-5 < 0.01, "I_t {}", c.it);
        assert!((c.iw - 4.131_862_542e-9).abs() / 4.131_862_542e-9 < 0.05, "I_w {}", c.iw);
    }

    #[test]
    fn impossible_dimensions_give_no_outline() {
        assert!(torsion_constants(&parametric(semio_s_artifact_norm_en1993_model::ParametricShape::Rhs, 0.02, 0.02, 0.02, 0.0)).is_none());
        assert!(torsion_constants(&parametric(semio_s_artifact_norm_en1993_model::ParametricShape::WeldedI, 0.2, 0.1, 0.01, 0.1)).is_none());
    }
}
