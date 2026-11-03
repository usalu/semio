//! 🎚️ Scalar grips: one diamond grip per numeric leaf of a row that has no drawn dimension, side by side on a board beside the entity. A drag writes `base + k · s` with a gain of a quarter of the value (at least ten steps) per metre, rounded to the step of the leaf.
//!
//! 🧬️ The leaf tables below are generated from the snapshot schema (label, unit, step): `🧰️scripts/en1993-scalar-grips/gen.py` of the ticket NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS. The tests of the scene crate fail when a numeric leaf of a covered row has no grip.

use super::model;
use super::kit::{along, entity_ref, loc, v3};
use super::layout::Layout;
use super::scene::*;
use super::scene::projection_kit::handle_kit::{field_binding, grid, handle, identity_binding, round, HandleSpec as Spec};

/// 🧭️ How a leaf may be dragged: a count steps by one, a positive quantity stays above zero, a signed one may pass through it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Count,
    Positive,
    Signed,
}

/// 🎚️ One numeric leaf of a row: its camel-case name, unit, step, drag kind, English and German label and the current value.
#[derive(Clone, Debug)]
pub struct Scalar {
    pub field: &'static str,
    pub unit: &'static str,
    pub step: f64,
    pub kind: Kind,
    pub en: &'static str,
    pub de: &'static str,
    pub value: f64,
}

macro_rules! scalars {
    ($row:expr; $(($field:ident, $camel:literal, $unit:literal, $step:literal, $kind:ident, $en:literal, $de:literal)),* $(,)?) => {
        vec![$(Scalar { field: $camel, unit: $unit, step: $step, kind: Kind::$kind, en: $en, de: $de, value: f64::from($row.$field) }),*]
    };
}

/// 🧱️ Where the grips of one entity stand: the board origin and the two directions it is laid out in (columns along `right`, rows downwards along `up`).
#[derive(Clone, Debug)]
pub struct Board {
    pub origin: Vec3,
    pub right: Vec3,
    pub up: Vec3,
}

const COLUMNS: usize = 8;
const PITCH: f64 = 0.25;

fn span(unit: &str) -> f64 {
    match unit {
        "m" => 10.0,
        "m²" | "m⁴" => 1.0,
        "Pa" => 2.0e9,
        "N" | "N/m" | "N·m/m" => 1.0e8,
        "N·m" => 1.0e9,
        "°" => 180.0,
        _ => 100.0,
    }
}

fn limits(s: &Scalar) -> (f64, f64) {
    match s.kind {
        Kind::Count => (s.value.min(0.0), s.value.max(16.0)),
        Kind::Positive => (s.value.min(0.0), s.value.max(span(s.unit)).max(4.0 * s.value)),
        Kind::Signed => (s.value.min(-span(s.unit)).min(4.0 * s.value), s.value.max(span(s.unit)).max(4.0 * s.value)),
    }
}

/// 🎚️ The grips of `scalars` on `board`, skipping every leaf in `taken` (a dedicated grip already writes it).
pub fn grips(entity: &EntityRef, collection: &str, id: &str, board: &Board, scalars: Vec<Scalar>, taken: &[Handle]) -> Vec<Handle> {
    let path = |field: &str| format!("{collection}[id={id}].{field}");
    scalars
        .into_iter()
        .filter(|s| !taken.iter().any(|h| h.writes.iter().any(|w| w.path == path(s.field))))
        .enumerate()
        .map(|(index, s)| {
            let anchor = along(&along(&board.origin, &board.right, PITCH * (index % COLUMNS) as f64), &board.up, -PITCH * (index / COLUMNS) as f64);
            let tag = format!("in:{}", s.field);
            let name = loc(s.en, s.de);
            let (lo, hi) = limits(&s);
            if s.kind == Kind::Count {
                return handle(Spec { entity, tag: &tag, kind: HandleKind::CountStepper, anchor, grip: GripStyle::Diamond, writes: vec![identity_binding(path(s.field), "1", Some(Quantise::Int))], snap: SnapRule::NONE, limits: (lo, hi), name });
            }
            let k = (0.25 * s.value.abs()).max(10.0 * s.step);
            handle(Spec { entity, tag: &tag, kind: HandleKind::ScalarAlong { axis: board.up.clone() }, anchor, grip: GripStyle::Diamond, writes: vec![field_binding(path(s.field), Affine { base: s.value, k }, s.unit, round(s.step))], snap: grid(0.05), limits: ((lo - s.value) / k, (hi - s.value) / k), name })
        })
        .collect()
}

/// 🎚️ The numeric leaves of `joints` rows.
pub fn joint_leaves(row: &model::SteelJoint) -> Vec<Scalar> {
    scalars![row;
        (bolt_diameter, "boltDiameter", "m", 0.001, Positive, "Bolt diameter d", "Schraubendurchmesser d"),
        (bolt_rows, "boltRows", "1", 1.0, Count, "Bolt rows", "Schraubenreihen"),
        (bolts_per_row, "boltsPerRow", "1", 1.0, Count, "Bolts per row", "Schrauben je Reihe"),
        (pitch, "pitch", "m", 0.005, Positive, "Pitch p₁", "Lochabstand p₁"),
        (gauge, "gauge", "m", 0.005, Positive, "Gauge p₂", "Lochabstand quer p₂"),
        (end_distance, "endDistance", "m", 0.005, Positive, "End distance e₁", "Randabstand in Kraftrichtung e₁"),
        (edge_distance, "edgeDistance", "m", 0.005, Positive, "Edge distance e₂", "Randabstand quer zur Kraftrichtung e₂"),
        (shear_planes, "shearPlanes", "1", 1.0, Count, "Shear planes", "Scherfugen"),
        (plate_thickness, "plateThickness", "m", 0.001, Positive, "Plate thickness t", "Blechdicke t"),
        (plate_fu, "plateFu", "Pa", 10000000.0, Positive, "Plate tensile strength f_u", "Zugfestigkeit des Blechs f_u"),
        (weld_throat, "weldThroat", "m", 0.0005, Positive, "Weld throat thickness a", "Nahtdicke a"),
        (weld_length, "weldLength", "m", 0.005, Positive, "Weld length l_w", "Nahtlänge l_w"),
        (weld_fu, "weldFu", "Pa", 10000000.0, Positive, "Weld metal tensile strength f_u", "Zugfestigkeit des Schweißguts f_u"),
        (friction_mu, "frictionMu", "1", 0.01, Positive, "Slip factor μ", "Reibungszahl μ"),
        (preload_force, "preloadForce", "N", 1000.0, Signed, "Preloading force F_p,C", "Vorspannkraft F_p,C"),
        (slip_factor_ks, "slipFactorKs", "1", 0.01, Positive, "Hole-type factor k_s", "Lochbeiwert k_s"),
        (friction_surfaces, "frictionSurfaces", "1", 1.0, Count, "Friction surfaces n", "Reibflächen n"),
        (plate_fy, "plateFy", "Pa", 1000000.0, Positive, "Plate yield strength f_y,p", "Blechstreckgrenze f_y,p"),
        (plate_width, "plateWidth", "m", 0.005, Positive, "Plate width b_p", "Blechbreite b_p"),
        (bolt_to_web, "boltToWeb", "m", 0.005, Positive, "Bolt to web fillet m", "Schraube zum Stegansatz m"),
        (extension_to_flange, "extensionToFlange", "m", 0.005, Positive, "Extended part: bolt to flange m_x", "Überstand: Schraube zum Gurt m_x"),
        (extension_edge, "extensionEdge", "m", 0.005, Positive, "Extended part: bolt to edge e_x", "Überstand: Schraube zum Rand e_x"),
        (alpha_factor, "alphaFactor", "1", 0.01, Positive, "Effective length factor α (Figure 6.11)", "Faktor α der wirksamen Länge (Bild 6.11)"),
        (bolt_stress_area, "boltStressArea", "m²", 0.0001, Signed, "Bolt stress area A_s", "Schraubenspannungsquerschnitt A_s"),
        (bolt_grip_length, "boltGripLength", "m", 0.005, Positive, "Bolt grip length L_b", "Klemmlänge L_b"),
        (column_flange_thickness, "columnFlangeThickness", "m", 0.005, Positive, "Column flange thickness t_fc", "Stützengurtdicke t_fc"),
        (column_web_thickness, "columnWebThickness", "m", 0.005, Positive, "Column web thickness t_wc", "Stützenstegdicke t_wc"),
        (column_web_depth, "columnWebDepth", "m", 0.005, Positive, "Column clear web depth d_wc", "Stützen-Steghöhe d_wc"),
        (column_web_shear_area, "columnWebShearArea", "m²", 0.0001, Signed, "Column shear area A_vc", "Stützenschubfläche A_vc"),
        (column_root_radius, "columnRootRadius", "m", 0.005, Positive, "Column root radius r_c", "Stützenausrundung r_c"),
        (column_fy, "columnFy", "Pa", 1000000.0, Positive, "Column yield strength f_y,c", "Stützenstreckgrenze f_y,c"),
        (column_web_stress, "columnWebStress", "Pa", 1000000.0, Signed, "Column web longitudinal stress σ_com,Ed", "Stützensteglängsspannung σ_com,Ed"),
        (beam_flange_thickness, "beamFlangeThickness", "m", 0.005, Positive, "Beam flange thickness t_fb", "Trägergurtdicke t_fb"),
        (compression_lever, "compressionLever", "m", 0.005, Positive, "Lever arm of the first bolt row h_1", "Hebelarm der ersten Schraubenreihe h_1"),
        (moment_demand, "momentDemand", "N·m", 1000.0, Signed, "Joint moment M_j,Ed", "Anschlussmoment M_j,Ed"),
        (beam_moment_resistance, "beamMomentResistance", "N·m", 1000.0, Signed, "Beam plastic moment M_b,pl,Rd", "Plastisches Trägermoment M_b,pl,Rd"),
        (column_moment_resistance, "columnMomentResistance", "N·m", 1000.0, Signed, "Column plastic moment M_c,pl,Rd", "Plastisches Stützenmoment M_c,pl,Rd"),
        (beam_inertia, "beamInertia", "m⁴", 1e-08, Positive, "Beam second moment I_b", "Träger-Flächenmoment I_b"),
        (beam_span, "beamSpan", "m", 0.005, Positive, "Beam span L_b", "Trägerspannweite L_b"),
        (braced_frame, "bracedFrame", "1", 1.0, Count, "Braced frame (0/1)", "Ausgesteifter Rahmen (0/1)"),
        (stiffness_ratio, "stiffnessRatio", "1", 0.01, Positive, "Stiffness ratio K_b/K_c", "Steifigkeitsverhältnis K_b/K_c"),
        (transformation_parameter, "transformationParameter", "1", 0.01, Positive, "Transformation parameter β", "Transformationsparameter β"),
        (base_plate_length, "basePlateLength", "m", 0.005, Positive, "Base plate length L_p", "Fußplattenlänge L_p"),
        (concrete_fcd, "concreteFcd", "Pa", 1000000.0, Positive, "Concrete design strength f_cd", "Betondruckfestigkeit f_cd"),
        (concrete_modulus, "concreteModulus", "Pa", 1000000.0, Positive, "Concrete modulus E_cm", "Beton-E-Modul E_cm"),
        (concrete_area_a0, "concreteAreaA0", "m²", 0.0001, Positive, "Loaded concrete area A_c0", "Belastete Betonfläche A_c0"),
        (concrete_area_a1, "concreteAreaA1", "m²", 0.0001, Positive, "Distributed concrete area A_c1", "Verteilungsfläche A_c1"),
        (joint_coefficient, "jointCoefficient", "1", 0.01, Positive, "Joint coefficient β_j", "Fugenbeiwert β_j"),
        (anchor_area, "anchorArea", "m²", 0.0001, Positive, "Anchor stress area A_b", "Ankerspannungsquerschnitt A_b"),
        (anchor_length, "anchorLength", "m", 0.005, Positive, "Anchor elongation length L_b", "Ankerdehnlänge L_b"),
        (base_normal_force, "baseNormalForce", "N", 1000.0, Signed, "Base normal force N_Ed (tension +)", "Fußnormalkraft N_Ed (Zug +)"),
        (base_moment, "baseMoment", "N·m", 1000.0, Signed, "Base moment M_Ed", "Fußmoment M_Ed"),
        (base_lever_left, "baseLeverLeft", "m", 0.005, Positive, "Lever arm left z_l", "Hebelarm links z_l"),
        (base_lever_right, "baseLeverRight", "m", 0.005, Positive, "Lever arm right z_r", "Hebelarm rechts z_r"),
        (beam_web_thickness, "beamWebThickness", "m", 0.0005, Positive, "Beam web thickness t_wb (0 = not given)", "Trägerstegdicke t_wb (0 = nicht angegeben)"),
        (beam_web_fy, "beamWebFy", "Pa", 5000000.0, Positive, "Beam web yield strength f_y,wb (0 = not given)", "Trägerstegstreckgrenze f_y,wb (0 = nicht angegeben)"),
    ]
}

/// 🎚️ The numeric leaves of `jointPlates` rows.
pub fn joint_plate_leaves(row: &model::JointPlate) -> Vec<Scalar> {
    scalars![row;
        (plate_height, "plateHeight", "m", 0.005, Positive, "Plate height h_p (along the bolt line)", "Blechhöhe h_p (entlang der Schraubenlinie)"),
        (plate_width, "plateWidth", "m", 0.005, Positive, "Plate width b_p", "Blechbreite b_p"),
        (plate_thickness, "plateThickness", "m", 0.005, Positive, "Plate thickness t_p", "Blechdicke t_p"),
        (plate_count, "plateCount", "1", 1.0, Count, "Plates acting in parallel (1 or 2)", "Parallel wirkende Bleche (1 oder 2)"),
        (plate_fy, "plateFy", "Pa", 1000000.0, Positive, "Plate yield strength f_y,p", "Blechstreckgrenze f_y,p"),
        (plate_fu, "plateFu", "Pa", 1000000.0, Positive, "Plate tensile strength f_u,p", "Zugfestigkeit des Blechs f_u,p"),
        (bolt_diameter, "boltDiameter", "m", 0.005, Positive, "Bolt diameter d", "Schraubendurchmesser d"),
        (hole_diameter, "holeDiameter", "m", 0.005, Positive, "Hole diameter d_0", "Lochdurchmesser d_0"),
        (bolt_rows, "boltRows", "1", 1.0, Count, "Bolt rows n_1 along the line of action", "Schraubenreihen n_1 in Kraftrichtung"),
        (bolts_per_row, "boltsPerRow", "1", 1.0, Count, "Bolts per row n_2", "Schrauben je Reihe n_2"),
        (pitch, "pitch", "m", 0.005, Positive, "Pitch p_1", "Lochabstand p_1"),
        (gauge, "gauge", "m", 0.005, Positive, "Gauge p_2", "Lochabstand quer p_2"),
        (end_distance, "endDistance", "m", 0.005, Positive, "End distance e_1", "Randabstand e_1"),
        (edge_distance, "edgeDistance", "m", 0.005, Positive, "Edge distance e_2", "Randabstand e_2"),
        (shear_planes, "shearPlanes", "1", 1.0, Count, "Shear planes per bolt", "Scherfugen je Schraube"),
        (eccentricity, "eccentricity", "m", 0.005, Signed, "Eccentricity z of the bolt line to the support", "Exzentrizität z der Schraubenlinie zum Auflager"),
        (weld_throat, "weldThroat", "m", 0.005, Positive, "Weld throat thickness a", "Nahtdicke a"),
        (weld_fu, "weldFu", "Pa", 1000000.0, Positive, "Weld reference tensile strength f_u", "Bezugs-Zugfestigkeit f_u der Naht"),
        (weld_beta_w, "weldBetaW", "1", 0.01, Positive, "Correlation factor β_w (Table 4.1)", "Korrelationsbeiwert β_w (Tabelle 4.1)"),
    ]
}

/// 🎚️ The numeric leaves of `hollowJoints` rows.
pub fn hollow_joint_leaves(row: &model::HollowJoint) -> Vec<Scalar> {
    scalars![row;
        (chord_width, "chordWidth", "m", 0.005, Positive, "Chord width b_0", "Gurtbreite b_0"),
        (chord_height, "chordHeight", "m", 0.005, Positive, "Chord height h_0", "Gurthöhe h_0"),
        (chord_thickness, "chordThickness", "m", 0.005, Positive, "Chord wall thickness t_0", "Gurtwanddicke t_0"),
        (chord_fy, "chordFy", "Pa", 1000000.0, Positive, "Chord yield strength f_y0", "Gurtstreckgrenze f_y0"),
        (chord_stress, "chordStress", "Pa", 1000000.0, Signed, "Chord stress σ_0,Ed (compression negative)", "Gurtspannung σ_0,Ed (Druck negativ)"),
        (brace1_width, "brace1Width", "m", 0.005, Positive, "Brace 1 width b_1", "Streben-1-Breite b_1"),
        (brace1_height, "brace1Height", "m", 0.005, Positive, "Brace 1 height h_1", "Streben-1-Höhe h_1"),
        (brace1_thickness, "brace1Thickness", "m", 0.005, Positive, "Brace 1 wall thickness t_1", "Streben-1-Wanddicke t_1"),
        (brace1_angle, "brace1Angle", "°", 1.0, Signed, "Brace 1 angle θ_1", "Streben-1-Winkel θ_1"),
        (brace1_force, "brace1Force", "N", 1000.0, Signed, "Brace 1 force N_1,Ed", "Streben-1-Kraft N_1,Ed"),
        (brace2_width, "brace2Width", "m", 0.005, Positive, "Brace 2 width b_2", "Streben-2-Breite b_2"),
        (brace2_height, "brace2Height", "m", 0.005, Positive, "Brace 2 height h_2", "Streben-2-Höhe h_2"),
        (brace2_thickness, "brace2Thickness", "m", 0.005, Positive, "Brace 2 wall thickness t_2", "Streben-2-Wanddicke t_2"),
        (brace2_angle, "brace2Angle", "°", 1.0, Signed, "Brace 2 angle θ_2", "Streben-2-Winkel θ_2"),
        (brace2_force, "brace2Force", "N", 1000.0, Signed, "Brace 2 force N_2,Ed", "Streben-2-Kraft N_2,Ed"),
        (gap, "gap", "m", 0.005, Positive, "Gap g between the braces", "Spalt g zwischen den Streben"),
        (brace_fy, "braceFy", "Pa", 1000000.0, Positive, "Brace yield strength f_yi", "Strebenstreckgrenze f_yi"),
    ]
}

/// 🎚️ The numeric leaves of `platedPanels` rows.
pub fn panel_leaves(row: &model::PlatedPanel) -> Vec<Scalar> {
    scalars![row;
        (a, "a", "m", 0.01, Positive, "Panel length a", "Feldlänge a"),
        (b, "b", "m", 0.01, Positive, "Panel width b", "Feldbreite b"),
        (thickness, "thickness", "m", 0.001, Positive, "Plate thickness t", "Blechdicke t"),
        (fy, "fy", "Pa", 5000000.0, Positive, "Yield strength f_y", "Streckgrenze f_y"),
        (k_sigma, "kSigma", "1", 0.01, Positive, "Buckling factor k_σ", "Beulwert k_σ"),
        (stiffener_inertia, "stiffenerInertia", "m⁴", 1e-08, Positive, "Stiffener second moment I_sl", "Steifen-Flächenmoment I_sl"),
        (stiffener_area, "stiffenerArea", "m²", 0.0001, Positive, "Stiffener area A_sl (gross)", "Steifenfläche A_sl (brutto)"),
        (stiffener_eccentricity, "stiffenerEccentricity", "m", 0.005, Signed, "Stiffener extreme fibre distance e", "Randfaserabstand der Steife e"),
        (stiffener_alpha, "stiffenerAlpha", "1", 0.01, Positive, "Column imperfection α (0.34 closed, 0.49 open)", "Imperfektion α (0,34 geschlossen, 0,49 offen)"),
        (compression_stress, "compressionStress", "Pa", 1000000.0, Signed, "Plate compression stress σ_Ed", "Plattendruckspannung σ_Ed"),
        (half_width, "halfWidth", "m", 0.005, Positive, "Flange half width b_0", "Gurthalbbreite b_0"),
        (equivalent_span, "equivalentSpan", "m", 0.005, Positive, "Equivalent span L_e", "Ersatzstützweite L_e"),
        (hogging, "hogging", "1", 1.0, Count, "Hogging zone (0 sagging, 1 hogging)", "Stützbereich (0 Feld, 1 Stütze)"),
        (patch_force, "patchForce", "N", 1000.0, Signed, "Transverse force F_Ed", "Querkraft F_Ed"),
        (patch_length, "patchLength", "m", 0.005, Positive, "Stiff bearing length s_s", "Lasteinleitungslänge s_s"),
        (patch_offset, "patchOffset", "m", 0.005, Signed, "Distance to the end c", "Abstand zum Ende c"),
        (flange_thickness, "flangeThickness", "m", 0.005, Positive, "Flange thickness t_f", "Gurtdicke t_f"),
        (flange_width, "flangeWidth", "m", 0.005, Positive, "Flange width b_f", "Gurtbreite b_f"),
        (flange_fy, "flangeFy", "Pa", 1000000.0, Positive, "Flange yield strength f_yf", "Gurtstreckgrenze f_yf"),
        (longitudinal_offset, "longitudinalOffset", "m", 0.005, Signed, "Longitudinal stiffener distance b_1", "Längssteifenabstand b_1"),
        (longitudinal_inertia, "longitudinalInertia", "m⁴", 1e-08, Positive, "Longitudinal stiffener I_sl,1", "Längssteife I_sl,1"),
        (patch_moment, "patchMoment", "N·m", 1000.0, Signed, "Moment with patch load M_Ed", "Moment bei Lasteinleitung M_Ed"),
        (patch_moment_resistance, "patchMomentResistance", "N·m", 1000.0, Signed, "Moment resistance M_c,Rd", "Momententragfähigkeit M_c,Rd"),
        (bending_moment, "bendingMoment", "N·m", 1000.0, Signed, "Bending moment M_Ed (M-V)", "Biegemoment M_Ed (M-V)"),
        (plastic_moment, "plasticMoment", "N·m", 1000.0, Signed, "Plastic moment M_pl,Rd", "Plastisches Moment M_pl,Rd"),
        (flange_moment, "flangeMoment", "N·m", 1000.0, Signed, "Flange moment M_f,Rd", "Gurtmoment M_f,Rd"),
        (shear_force, "shearForce", "N", 1000.0, Signed, "Shear force V_Ed", "Querkraft V_Ed"),
        (transverse_stiffener_inertia, "transverseStiffenerInertia", "m⁴", 1e-08, Positive, "Transverse stiffener I_st", "Quersteife I_st"),
        (flange_induced_k, "flangeInducedK", "1", 0.01, Positive, "Flange-induced buckling k (0.3/0.4/0.55)", "Gurtinduziertes Beulen k (0,3/0,4/0,55)"),
        (flange_area, "flangeArea", "m²", 0.0001, Positive, "Compression flange area A_fc", "Druckgurtfläche A_fc"),
        (stress_x, "stressX", "Pa", 1000000.0, Signed, "Reduced-stress σ_x", "Vergleichsspannung σ_x"),
        (stress_z, "stressZ", "Pa", 1000000.0, Signed, "Reduced-stress σ_z", "Vergleichsspannung σ_z"),
        (shear_stress, "shearStress", "Pa", 1000000.0, Signed, "Reduced-stress τ", "Vergleichsspannung τ"),
        (alpha_cr, "alphaCr", "1", 0.01, Positive, "Critical load amplifier α_cr (0 = none)", "Kritischer Laststeigerungsfaktor α_cr (0 = keiner)"),
        (corrugation_flat, "corrugationFlat", "m", 0.005, Positive, "Corrugated web: flat fold width a_0 (0 = plane web)", "Trapezstegblech: ebene Faltenbreite a_0 (0 = ebener Steg)"),
        (corrugation_projection, "corrugationProjection", "m", 0.005, Positive, "Corrugated web: horizontal projection a_1 of the inclined fold", "Trapezstegblech: waagerechte Projektion a_1 der geneigten Falte"),
        (corrugation_depth, "corrugationDepth", "m", 0.005, Positive, "Corrugated web: corrugation depth d", "Trapezstegblech: Wellentiefe d"),
        (web_height, "webHeight", "m", 0.005, Positive, "Corrugated web height h_w", "Trapezstegblech: Steghöhe h_w"),
        (stiffener_spacing, "stiffenerSpacing", "m", 0.005, Positive, "Longitudinal stiffener spacing s (equal subpanels, n = b/s - 1 stiffeners; 0 = the single stiffener of the stiffener fields)", "Längssteifenabstand s (gleiche Teilfelder, n = b/s - 1 Steifen; 0 = die einzelne Steife der Steifenfelder)"),
        (stiffener_depth, "stiffenerDepth", "m", 0.005, Positive, "Stiffener web or leg height h_s, measured from the plate surface", "Steghöhe bzw. Schenkellänge h_s der Steife, ab Blechoberfläche"),
        (stiffener_thickness, "stiffenerThickness", "m", 0.005, Positive, "Stiffener web or leg thickness t_s", "Stegdicke bzw. Schenkeldicke t_s der Steife"),
        (stiffener_flange_width, "stiffenerFlangeWidth", "m", 0.005, Positive, "Stiffener flange width or second leg length b_f", "Flanschbreite bzw. zweite Schenkellänge b_f der Steife"),
        (stiffener_flange_thickness, "stiffenerFlangeThickness", "m", 0.005, Positive, "Stiffener flange or second leg thickness t_f", "Flanschdicke bzw. zweite Schenkeldicke t_f der Steife"),
        (lateral_pressure, "lateralPressure", "Pa", 1000000.0, Signed, "Uniform lateral pressure p on the panel (EN 1993-1-7, perpendicular to the plate)", "Gleichmäßiger Querdruck p auf das Blech (EN 1993-1-7, senkrecht zur Blechebene)"),
        (lateral_pressure_coefficient, "lateralPressureCoefficient", "1", 0.01, Signed, "Plate bending coefficient k of sigma = k p b²/t² for the panel aspect ratio and edge conditions (Timoshenko or Roark table, entered; table values not verified)", "Plattenbeiwert k von sigma = k p b²/t² für Seitenverhältnis und Lagerung (Tabelle nach Timoshenko oder Roark, eingegeben; Tabellenwerte ungeprüft)"),
    ]
}

/// 🎚️ The numeric leaves of `coldFormedMembers` rows.
pub fn cold_formed_leaves(row: &model::ColdFormedMember) -> Vec<Scalar> {
    scalars![row;
        (b_bar, "bBar", "m", 0.001, Positive, "Notional flat width b_p", "Rechnerische ebene Breite b_p"),
        (thickness, "thickness", "m", 1e-05, Positive, "Nominal thickness t", "Nennblechdicke t"),
        (k_sigma, "kSigma", "1", 0.01, Positive, "Buckling factor k_σ", "Beulwert k_σ"),
        (psi, "psi", "1", 0.05, Signed, "Stress ratio ψ", "Randspannungsverhältnis ψ"),
        (fy, "fy", "Pa", 5000000.0, Positive, "Yield strength f_yb", "Basisstreckgrenze f_yb"),
        (gross_resistance, "grossResistance", "N", 1000.0, Positive, "Gross cross-section resistance", "Widerstand des Bruttoquerschnitts"),
        (flange_width, "flangeWidth", "m", 0.005, Positive, "Flange width b_p", "Gurtbreite b_p"),
        (lip_length, "lipLength", "m", 0.005, Positive, "Edge fold length c_p (0 = unstiffened)", "Randlippe c_p (0 = unausgesteift)"),
        (web_height, "webHeight", "m", 0.005, Positive, "Web height h_w", "Steghöhe h_w"),
        (web_slope, "webSlope", "°", 1.0, Signed, "Web slope φ", "Stegneigung φ"),
        (opposite_flange_width, "oppositeFlangeWidth", "m", 0.005, Positive, "Opposite flange width b_2", "Gegengurtbreite b_2"),
        (spring_factor, "springFactor", "1", 0.01, Positive, "Spring factor k_f (1 symmetric, 0 antisymmetric)", "Federfaktor k_f (1 symmetrisch, 0 antimetrisch)"),
        (corner_radius, "cornerRadius", "m", 0.005, Positive, "Inner corner radius r", "Innerer Eckradius r"),
        (flange_stress, "flangeStress", "Pa", 1000000.0, Signed, "Flange compression stress σ_com,Ed", "Gurtdruckspannung σ_com,Ed"),
        (shear_force, "shearForce", "N", 1000.0, Signed, "Web shear force V_Ed", "Stegquerkraft V_Ed"),
        (support_reaction, "supportReaction", "N", 1000.0, Signed, "Support reaction F_Ed", "Auflagerkraft F_Ed"),
        (support_length, "supportLength", "m", 0.005, Positive, "Support length s_s", "Auflagerlänge s_s"),
        (crippling_category, "cripplingCategory", "1", 1.0, Count, "Web crippling category (1 or 2)", "Kategorie Stegkrüppeln (1 oder 2)"),
        (stiffened_support, "stiffenedSupport", "1", 1.0, Count, "Web with stiffened supports (0/1)", "Steg mit ausgesteiften Auflagern (0/1)"),
        (bending_moment, "bendingMoment", "N·m", 1000.0, Signed, "Bending moment M_Ed", "Biegemoment M_Ed"),
        (moment_resistance, "momentResistance", "N·m", 1000.0, Signed, "Moment resistance M_c,Rd", "Momententragfähigkeit M_c,Rd"),
        (sheeting_stiffness, "sheetingStiffness", "N·m/m", 100.0, Positive, "Sheeting rotational stiffness C_D,C", "Drehbettung Trapezblech C_D,C"),
        (required_restraint_stiffness, "requiredRestraintStiffness", "N·m/m", 100.0, Positive, "Required restraint stiffness C_D,req", "Erforderliche Drehbettung C_D,req"),
        (purlin_span, "purlinSpan", "m", 0.005, Positive, "Purlin span L (0 = not a purlin)", "Pfettenspannweite L (0 = keine Pfette)"),
        (purlin_load, "purlinLoad", "N/m", 10.0, Signed, "Purlin vertical design load q_Ed", "Pfetten-Bemessungsstreckenlast q_Ed"),
        (purlin_lateral_ratio, "purlinLateralRatio", "1", 0.01, Positive, "Lateral to vertical load ratio k_h on the free flange", "Verhältnis k_h seitliche zu lotrechter Last am freien Gurt"),
    ]
}

/// 🎚️ The scalar grips of the row `id` (a joint, joint plate, hollow joint, plated panel or cold-formed member) beside its dedicated grips `taken`; empty for any other entity.
pub fn handles(d: &model::En1993Snapshot, layout: &Layout, id: &str, taken: &[Handle]) -> Vec<Handle> {
    let beside = |pose: &super::layout::MemberPose, at: f64| Board { origin: along(&along(&pose.frame.origin, &pose.frame.x, at), &pose.frame.z, 0.9), right: pose.frame.y.clone(), up: pose.frame.z.clone() };
    if let Some(joint) = d.joints.iter().find(|j| j.id == id) {
        let Some(pose) = layout.member(&joint.member_id) else { return Vec::new() };
        let board = Board { origin: along(&beside(pose, pose.length).origin, &pose.frame.y, 0.6), ..beside(pose, pose.length) };
        return grips(&entity_ref(EntityKind::Group, id, "joints"), "joints", id, &board, joint_leaves(joint), taken);
    }
    if let Some(joint) = d.hollow_joints.iter().find(|j| j.id == id) {
        let Some(pose) = layout.member(&joint.member_id) else { return Vec::new() };
        let board = Board { origin: along(&beside(pose, pose.length / 2.0).origin, &pose.frame.y, 0.6), ..beside(pose, pose.length / 2.0) };
        return grips(&entity_ref(EntityKind::Group, id, "hollowJoints"), "hollowJoints", id, &board, hollow_joint_leaves(joint), taken);
    }
    if let Some(plate) = d.joint_plates.iter().find(|p| p.id == id) {
        let Some(pose) = super::plate_layout::plate_pose(d, layout, plate) else { return Vec::new() };
        let board = Board { origin: along(&along(&pose.bolt_centre, &pose.across, 0.8), &pose.line, 0.6), right: pose.across.clone(), up: pose.line.clone() };
        return grips(&entity_ref(EntityKind::Group, id, "jointPlates"), "jointPlates", id, &board, joint_plate_leaves(plate), taken);
    }
    let board = |o: Vec3| Board { origin: v3(o.x, o.y - 0.6, o.z + 0.9), right: Vec3::X, up: Vec3::Z };
    if let Some(panel) = d.plated_panels.iter().find(|p| p.id == id) {
        return grips(&entity_ref(EntityKind::Plate, id, "platedPanels"), "platedPanels", id, &board(layout.origin(id)), panel_leaves(panel), taken);
    }
    if let Some(member) = d.cold_formed_members.iter().find(|c| c.id == id) {
        return grips(&entity_ref(EntityKind::Member, id, "coldFormedMembers"), "coldFormedMembers", id, &board(layout.origin(id)), cold_formed_leaves(member), taken);
    }
    Vec::new()
}

