//! 🪜️ Built-up member checks: the chord force of laced and battened compression members with the shear flexibility of the web system (6.4.1 to 6.4.3) and the connection spacing of closely spaced members (6.4.4). See [EN 1993-1-1](https://eurocodes.jrc.ec.europa.eu/).

use super::*;
use crate::clauses::{misc, stability as st};
use semio_s_artifact_norm_en1993_model::BuiltUpMember;
use super::thin_walled_section_checks::{thin_constants, ThinConstants};
use semio_s_plugin_norm_mechanics::{self as mechanics, section::profiles, ThinWall};

const CHS_POINTS: usize = 96;

struct Context<'a> {
    row: &'a BuiltUpMember,
    member: &'a SteelMember,
    material: &'a SteelMaterial,
}

fn subject(r: &BuiltUpMember, field: &str, en: &str, de: &str) -> SubjectRef {
    SubjectRef::new(&r.id, format!("builtUpMembers[id={}].{field}", r.id), loc(en, de))
}

fn chord_check(annex: AnnexChoice, params: AnnexParams, c: &Context<'_>, report: &mut CheckReport) {
    let (r, m, mat) = (c.row, c.member, c.material);
    let (e, fy) = (mat.e_modulus / 1e6, mat.fy / 1e6);
    let (l, h0, a, a_ch, i_ch, i_b) = (m.length * 1e3, r.chord_distance * 1e3, r.panel_spacing * 1e3, r.chord_area * 1e6, r.chord_inertia * 1e12, r.batten_inertia * 1e12);
    let n = f64::from(r.planes.max(1));
    if l <= 0.0 || h0 <= 0.0 || a <= 0.0 || a_ch <= 0.0 || i_ch <= 0.0 || r.design_axial <= 0.0 {
        return;
    }
    let i1 = 0.5 * h0 * h0 * a_ch + 2.0 * i_ch;
    let i0 = (i1 / (2.0 * a_ch)).sqrt();
    let lambda = l / i0;
    let (mu, sv, system_en, system_de) = if r.kind == semio_s_artifact_norm_en1993_model::BuiltUpKind::Laced {
        if r.diagonal_area <= 0.0 || r.diagonal_length <= 0.0 {
            return;
        }
        (0.0, misc::sv_laced(n, e, r.diagonal_area * 1e6, a, h0, r.diagonal_length * 1e3), "laced, S_v=n E A_d a h_0²/(2 d³)", "Gitterstab, S_v=n E A_d a h_0²/(2 d³)")
    } else {
        (misc::mu_battened(lambda), misc::sv_battened(e, i_ch, a, n, i_b, h0), "battened, S_v by 6.4.3.1 (capped at 2π²E I_ch/a²)", "Rahmenstab, S_v nach 6.4.3.1 (begrenzt auf 2π²E I_ch/a²)")
    };
    let built = misc::built_up_member(r.design_axial, r.design_moment * 1e3, l, h0, a_ch, i_ch, mu, sv, e);
    if !built.n_ch_ed.is_finite() || built.n_cr <= r.design_axial || sv <= r.design_axial {
        let reason = loc(&format!("N_Ed={:.1} kN is not below N_cr={:.1} kN and S_v={:.1} kN: the built-up member is unstable.", r.design_axial / 1e3, built.n_cr / 1e3, sv / 1e3), &format!("N_Ed={:.1} kN liegt nicht unter N_cr={:.1} kN und S_v={:.1} kN: der mehrteilige Stab ist instabil.", r.design_axial / 1e3, built.n_cr / 1e3, sv / 1e3));
        report.push(CheckResult::assess(format!("en1993.1-1.6.4.chord.{}", r.id), "DIN EN 1993-1-1", ClauseId::new("EN 1993-1-1", "6", "6.4.1"), subject(r, "designAxial", "Axial force", "Normalkraft"), loc("Built-up member: chord force", "Mehrteiliger Stab: Gurtkraft")).annex(annex).utilization(force_n(r.design_axial), force_n(built.n_cr.min(sv).max(0.0))).explanation(reason).remedy(Remedy::at_most(subject(r, "designAxial", "Axial force", "Normalkraft"), force_n(r.design_axial), force_n(0.5 * built.n_cr.min(sv).max(0.0)), loc("Reduce N_Ed or stiffen the web system and the chords.", "N_Ed verringern oder Stegsystem und Gurte versteifen."))).build());
        return;
    }
    let i_chord = (i_ch / a_ch).sqrt();
    let eps = (235.0 / fy).sqrt();
    let lambda_ch = (a / i_chord) / (93.9 * eps);
    let chi = st::chi_general(lambda_ch, st::Curve::B.alpha(), 0.2, 1.0);
    let n_b_rd = chi * a_ch * fy / params.gamma_m1;
    let explanation = loc(
        &format!("{system_en}; I_eff={:.3e} mm⁴, μ={mu:.3}, N_cr={:.0} kN, S_v={:.0} kN, M_Ed (second order)={:.2} kNm, N_ch,Ed=0.5 N_Ed+M_Ed h_0 A_ch/(2 I_eff)={:.1} kN, V_Ed=π M_Ed/L={:.2} kN; chord buckling over a={a:.0} mm: λ̄={lambda_ch:.3}, χ={chi:.3}, N_b,Rd={:.1} kN.", built.i_eff, built.n_cr / 1e3, sv / 1e3, built.m_ed / 1e6, built.n_ch_ed / 1e3, built.v_ed / 1e3, n_b_rd / 1e3),
        &format!("{system_de}; I_eff={:.3e} mm⁴, μ={mu:.3}, N_cr={:.0} kN, S_v={:.0} kN, M_Ed (Theorie II. Ordnung)={:.2} kNm, N_ch,Ed=0,5 N_Ed+M_Ed h_0 A_ch/(2 I_eff)={:.1} kN, V_Ed=π M_Ed/L={:.2} kN; Gurtknicken über a={a:.0} mm: λ̄={lambda_ch:.3}, χ={chi:.3}, N_b,Rd={:.1} kN.", built.i_eff, built.n_cr / 1e3, sv / 1e3, built.m_ed / 1e6, built.n_ch_ed / 1e3, built.v_ed / 1e3, n_b_rd / 1e3),
    );
    let mut b = CheckResult::assess(format!("en1993.1-1.6.4.chord.{}", r.id), "DIN EN 1993-1-1", ClauseId::new("EN 1993-1-1", "6", "6.4.1"), subject(r, "chordArea", "Chord area", "Gurtfläche"), loc("Built-up member: chord force", "Mehrteiliger Stab: Gurtkraft"))
        .annex(annex)
        .ndp(AnnexParams::GAMMA_M1)
        .ndp("ndp.en1993.built-up.bow-divisor")
        .utilization(force_n(built.n_ch_ed), force_n(n_b_rd))
        .explanation(explanation);
    if built.n_ch_ed > n_b_rd {
        let area = r.chord_area * built.n_ch_ed / n_b_rd;
        let spacing = (r.panel_spacing * n_b_rd / built.n_ch_ed).max(0.05);
        b = b
            .remedy(Remedy::at_least(subject(r, "chordArea", "Chord area", "Gurtfläche"), area_m2(r.chord_area), area_m2(area), loc(&format!("Enlarge the chords to at least {:.0} mm² (first estimate).", area * 1e6), &format!("Gurte auf mindestens {:.0} mm² vergrößern (erste Schätzung).", area * 1e6))))
            .remedy(Remedy::at_most(subject(r, "panelSpacing", "Connection spacing", "Verbindungsabstand"), length_m(r.panel_spacing), length_m(spacing), loc(&format!("Or reduce the connection spacing a to about {:.2} m.", spacing), &format!("Oder den Verbindungsabstand a auf etwa {:.2} m verringern.", spacing))));
    }
    report.push(b.build());
}

fn closely_spaced_check(annex: AnnexChoice, r: &BuiltUpMember, report: &mut CheckReport) {
    if r.kind != semio_s_artifact_norm_en1993_model::BuiltUpKind::Closely || r.min_radius <= 0.0 || r.chain_spacing <= 0.0 {
        return;
    }
    let limit = misc::closely_spaced_max_spacing(r.back_to_back == 1) * r.min_radius;
    let explanation = loc(
        &format!("Connection spacing {:.2} m against {:.0} i_min={:.2} m (Table 6.9, {}).", r.chain_spacing, misc::closely_spaced_max_spacing(r.back_to_back == 1), limit, if r.back_to_back == 1 { "angles back to back" } else { "other members" }),
        &format!("Verbindungsabstand {:.2} m gegen {:.0} i_min={:.2} m (Tabelle 6.9, {}).", r.chain_spacing, misc::closely_spaced_max_spacing(r.back_to_back == 1), limit, if r.back_to_back == 1 { "Winkel Rücken an Rücken" } else { "übrige Stäbe" }),
    );
    let mut b = CheckResult::assess(format!("en1993.1-1.6.4.4.spacing.{}", r.id), "DIN EN 1993-1-1", ClauseId::new("EN 1993-1-1", "6", "6.4.4"), subject(r, "chainSpacing", "Connection spacing", "Verbindungsabstand"), loc("Closely spaced built-up member: connection spacing", "Eng gestellter Stab: Verbindungsabstand"))
        .annex(annex)
        .utilization(length_m(r.chain_spacing), length_m(limit))
        .explanation(explanation);
    if r.chain_spacing > limit {
        b = b.remedy(Remedy::at_most(subject(r, "chainSpacing", "Connection spacing", "Verbindungsabstand"), length_m(r.chain_spacing), length_m(limit), loc(&format!("Connect the members at most every {limit:.2} m."), &format!("Stäbe höchstens alle {limit:.2} m verbinden."))));
    }
    report.push(b.build());
}

/// 🌀️ Section constants of one chord of a built-up member from the finite-element solver of the mechanics kernel (SI, outline axes: `y` along the chord distance h_0, `z` perpendicular to it) and the assembled torsion constants of the chord pair.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChordConstants {
    pub area: f64,
    pub iy: f64,
    pub iz: f64,
    pub it: f64,
    pub iw: f64,
    pub y_sc: f64,
    pub z_sc: f64,
    pub triangles: usize,
    pub it_assembled: f64,
    pub iw_assembled: f64,
    pub thin: Option<ThinConstants>,
}

fn chord_walls(r: &BuiltUpMember) -> Option<Vec<ThinWall>> {
    let (d, b, tf, tw) = (r.chord_depth, r.chord_width, r.chord_flange_thickness, r.chord_web_thickness);
    let wall = |y1: f64, z1: f64, y2: f64, z2: f64, thickness: f64| ThinWall { y1, z1, y2, z2, thickness };
    let (z0, z1) = (tf / 2.0, d - tf / 2.0);
    match r.chord_shape {
        semio_s_artifact_norm_en1993_model::ChordShape::Channel => Some(vec![wall(tw / 2.0, z0, tw / 2.0, z1, tw), wall(tw / 2.0, z0, b, z0, tf), wall(tw / 2.0, z1, b, z1, tf)]),
        semio_s_artifact_norm_en1993_model::ChordShape::I => Some(vec![wall(b / 2.0, z0, b / 2.0, z1, tw), wall(0.0, z0, b / 2.0, z0, tf), wall(b / 2.0, z0, b, z0, tf), wall(0.0, z1, b / 2.0, z1, tf), wall(b / 2.0, z1, b, z1, tf)]),
        _ => None,
    }
}

fn chord_outline(r: &BuiltUpMember) -> Option<profiles::Outline> {
    let (d, b, tf, tw) = (r.chord_depth, r.chord_width, r.chord_flange_thickness, r.chord_web_thickness);
    match r.chord_shape {
        semio_s_artifact_norm_en1993_model::ChordShape::Channel => (tf > 0.0 && tw > 0.0 && 2.0 * tf < d && tw < b).then(|| profiles::channel_section(d, b, tf, tw, 0.0, 1)),
        semio_s_artifact_norm_en1993_model::ChordShape::I => (tf > 0.0 && tw > 0.0 && 2.0 * tf < d && tw < b).then(|| profiles::i_section(d, b, tf, tw, 0.0, 1)),
        semio_s_artifact_norm_en1993_model::ChordShape::Rhs => (tf > 0.0 && 2.0 * tf < d.min(b)).then(|| profiles::rectangular_hollow_section(d, b, tf, 0.0, 1)),
        semio_s_artifact_norm_en1993_model::ChordShape::Chs => (tf > 0.0 && 2.0 * tf < d).then(|| profiles::circular_hollow_section(d, tf, CHS_POINTS)),
        _ => None,
    }
}

/// 🌀️ I_t, I_w and the shear centre of the chord outline by the kernel `analyse_section`, and the pair of chords at the distance h_0: I_t,assembled = n I_t (the closed-section action of the lacing is neglected) and the Vlasov upper bound I_w,assembled = n I_w + n I_y (h_0/2)² for rigid connections (formula unverified against the print: lacing and battens are shear-flexible, so the true value is lower). `None` for a row without a valid outline.
pub fn chord_constants(r: &BuiltUpMember) -> Option<ChordConstants> {
    let (outer, holes) = chord_outline(r)?;
    let steel = mechanics::Material { id: "steel".into(), e: 210.0e9, g: 81.0e9, rho: 7850.0 };
    let c = mechanics::analyse_section(&outer, &holes, &steel).ok()?;
    let chords = 2.0;
    Some(ChordConstants {
        area: c.area,
        iy: c.iy,
        iz: c.iz,
        it: c.j,
        iw: c.iw,
        y_sc: c.shear_centre_y - c.centroid_y,
        z_sc: c.shear_centre_z - c.centroid_z,
        triangles: c.triangles,
        it_assembled: chords * c.j,
        iw_assembled: chords * c.iw + chords * c.iy * (r.chord_distance / 2.0).powi(2),
        thin: chord_walls(r).and_then(|w| thin_constants(&w)),
    })
}

fn outline_row(annex: AnnexChoice, r: &BuiltUpMember, report: &mut CheckReport) {
    let Some(c) = chord_constants(r) else { return };
    let area_ratio = if r.chord_area > 0.0 { c.area / r.chord_area } else { 0.0 };
    let inertia_ratio = if r.chord_inertia > 0.0 { c.iz / r.chord_inertia } else { 0.0 };
    let (thin_en, thin_de) = c.thin.map_or((String::new(), String::new()), |k| (format!("Centreline thin-walled cross-check (kernel analyse_thin_walled): I_t={:.3e} m⁴, I_w={:.3e} m⁶, y_sc={:.2} mm. ", k.it, k.iw, k.y_sc * 1e3), format!("Mittellinien-Gegenprobe dünnwandig (Kern analyse_thin_walled): I_t={:.3e} m⁴, I_w={:.3e} m⁶, y_sc={:.2} mm. ", k.it, k.iw, k.y_sc * 1e3)));
    let explanation = loc(
        &format!("Finite-element section analysis of the sharp-corner chord outline ({} triangles): A={:.3e} m² (row {:.3e}, ratio {area_ratio:.3}), I_y={:.3e} m⁴, I_z={:.3e} m⁴ (row I_ch {:.3e}, ratio {inertia_ratio:.3}), I_t={:.3e} m⁴, I_w={:.3e} m⁶, shear centre y_sc={:.2} mm, z_sc={:.2} mm from the centroid. {thin_en}Chord pair at h_0={:.3} m: I_t={:.3e} m⁴ (lacing closed-section action neglected), I_w ≤ {:.3e} m⁶ (rigid-connection Vlasov bound, formula unverified).", c.triangles, c.area, r.chord_area, c.iy, c.iz, r.chord_inertia, c.it, c.iw, c.y_sc * 1e3, c.z_sc * 1e3, r.chord_distance, c.it_assembled, c.iw_assembled),
        &format!("Finite-Elemente-Querschnittsberechnung der scharfkantigen Gurtkontur ({} Dreiecke): A={:.3e} m² (Zeile {:.3e}, Verhältnis {area_ratio:.3}), I_y={:.3e} m⁴, I_z={:.3e} m⁴ (Zeile I_ch {:.3e}, Verhältnis {inertia_ratio:.3}), I_t={:.3e} m⁴, I_w={:.3e} m⁶, Schubmittelpunkt y_sc={:.2} mm, z_sc={:.2} mm vom Schwerpunkt. {thin_de}Gurtpaar im Abstand h_0={:.3} m: I_t={:.3e} m⁴ (Schließwirkung der Vergitterung vernachlässigt), I_w ≤ {:.3e} m⁶ (Vlasov-Obergrenze bei starrer Verbindung, Formel ungeprüft).", c.triangles, c.area, r.chord_area, c.iy, c.iz, r.chord_inertia, c.it, c.iw, c.y_sc * 1e3, c.z_sc * 1e3, r.chord_distance, c.it_assembled, c.iw_assembled),
    );
    report.push(
        CheckResult::assess(format!("en1993.1-1.6.4.outline.{}", r.id), "DIN EN 1993-1-1", ClauseId::new("EN 1993-1-1", "6", "6.4.1"), subject(r, "chordShape", "Chord outline", "Gurtkontur"), loc("Built-up member: torsion constants of the chord outline", "Mehrteiliger Stab: Torsionskennwerte der Gurtkontur"))
            .annex(annex)
            .informative(dimensionless(area_ratio))
            .explanation(explanation)
            .build(),
    );
}

/// 🪜️ Appends the built-up member checks of every admitted row.
pub fn check_built_up_members(document: &En1993Snapshot, annex: AnnexChoice, params: AnnexParams, scope: Scope<'_>, report: &mut CheckReport) {
    for row in document.built_up_members.iter().filter(|r| scope.admits("builtUpMembers", &r.id)) {
        let Some(member) = document.members.iter().find(|m| m.id == row.member_id) else { continue };
        let Some(material) = document.materials.iter().find(|m| m.id == member.material_id) else { continue };
        let context = Context { row, member, material };
        if row.kind != semio_s_artifact_norm_en1993_model::BuiltUpKind::Closely {
            chord_check(annex, params, &context, report);
        }
        closely_spaced_check(annex, row, report);
        outline_row(annex, row, report);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chord(shape: semio_s_artifact_norm_en1993_model::ChordShape, d: f64, b: f64, tf: f64, tw: f64) -> BuiltUpMember {
        BuiltUpMember { chord_shape: shape, chord_depth: d, chord_width: b, chord_flange_thickness: tf, chord_web_thickness: tw, chord_distance: 0.3, ..En1993Snapshot::special_parts_showcase().built_up_members[0].clone() }
    }

    #[test]
    fn the_channel_chord_matches_the_sectionproperties_oracle() {
        let c = chord_constants(&chord(semio_s_artifact_norm_en1993_model::ChordShape::Channel, 0.3, 0.1, 0.0165, 0.01)).expect("outline");
        let close = |got: f64, want: f64, what: &str| assert!((got - want).abs() / want.abs() < 0.005, "{what} {got} vs {want}");
        close(c.area, 5.97e-3, "A");
        close(c.iy, 8.224_378e-5, "I_y");
        close(c.iz, 5.760_906e-6, "I_z");
        close(c.it, 3.636_420e-7, "I_t");
        close(c.iw, 8.146_216e-8, "I_w");
        close(c.y_sc, -0.060_862, "y_sc");
        close(c.it_assembled, 2.0 * 3.636_420e-7, "I_t assembled");
        assert!(c.thin.is_some(), "the centreline cross-check is reported next to the finite-element constants");
    }

    #[test]
    fn a_row_without_a_valid_outline_has_no_constants_and_no_row() {
        assert!(chord_constants(&chord(semio_s_artifact_norm_en1993_model::ChordShape::Unset, 0.3, 0.1, 0.0165, 0.01)).is_none());
        assert!(chord_constants(&chord(semio_s_artifact_norm_en1993_model::ChordShape::Channel, 0.03, 0.1, 0.0165, 0.01)).is_none());
        let mut report = CheckReport::default();
        outline_row(AnnexChoice::En, &chord(semio_s_artifact_norm_en1993_model::ChordShape::Rhs, 0.0, 0.0, 0.0, 0.0), &mut report);
        assert!(report.checks.is_empty());
    }
}
