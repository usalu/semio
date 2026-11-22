//! 📇️ Design-case catalogue of EN 1998 kernel formulas that no snapshot entity reaches: each entry turns an existing pure function into an editable, evaluated case through the shared [`CaseCatalogue`] bridge. See [`super`] and `./📚️inputs/🔣️.json`.

use semio_s_plugin_norm_compliance::design_case::{self, CaseCatalogue, CaseEntry, CaseRow, DesignCase, InputSpec};
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckResult, CheckStatus, ClauseId, LocalizedCopy, Quantity, SubjectRef};

#[path = "📚️inputs/🦀️.rs"]
pub mod case_inputs;

#[path = "🧮️evaluators/🦀️.rs"]
pub mod evaluators;

use evaluators::{Inputs, Outcome};

/// 🗂️ One catalogue entry with its evaluator.
pub struct Entry {
    pub id: &'static str,
    pub part: &'static str,
    pub clause: &'static str,
    pub en: &'static str,
    pub de: &'static str,
    pub inputs: &'static [&'static str],
    pub eval: fn(&Inputs<'_>) -> Outcome,
}

/// 📋️ The catalogue entries in presentation order.
pub static ENTRIES: &[Entry] = &[
    Entry { id: "C98-01", part: "EN 1998-1", clause: "4.3.3.2.2", en: "Lateral force base shear from the design spectrum", de: "Fundamentschubkraft aus dem Bemessungsspektrum", inputs: &["a_g", "s_soil", "t_b", "t_c", "t_d", "q_factor", "t_1", "mass", "lambda_c", "v_rd"], eval: evaluators::base_shear_design },
    Entry { id: "C98-02", part: "EN 1998-1", clause: "3.2.2.2", en: "Elastic response force with damping correction", de: "Elastische Antwortkraft mit Dämpfungskorrektur", inputs: &["a_g", "s_soil", "t_b", "t_c", "t_d", "t_1", "mass", "xi_percent", "v_rd"], eval: evaluators::elastic_force },
    Entry { id: "C98-03", part: "EN 1998-1", clause: "3.2.2.3", en: "Vertical seismic force from the vertical spectrum", de: "Vertikale Erdbebenkraft aus dem Vertikalspektrum", inputs: &["a_g", "avg_ratio", "t_v", "mass", "f_rd_v"], eval: evaluators::vertical_force },
    Entry { id: "C98-04", part: "EN 1998-1", clause: "4.3.3.3", en: "Modal response spectrum analysis of a three-storey shear building", de: "Antwortspektrenverfahren eines dreigeschossigen Schubbalkens", inputs: &["m_1", "m_2", "m_3", "k_1", "k_2", "k_3", "a_g", "s_soil", "t_b", "t_c", "t_d", "q_factor", "xi_percent", "v_rd"], eval: evaluators::modal_analysis },
    Entry { id: "C98-05", part: "EN 1998-1", clause: "3.1.2", en: "Ground type from the shear wave velocity of three layers", de: "Baugrundklasse aus der Scherwellengeschwindigkeit dreier Schichten", inputs: &["h_1", "h_2", "h_3", "vs_1", "vs_2", "vs_3", "vs_min"], eval: evaluators::ground_velocity },
    Entry { id: "C98-06", part: "EN 1998-5", clause: "4.1.4", en: "Liquefaction safety factor of a sand layer", de: "Sicherheit gegen Bodenverflüssigung einer Sandschicht", inputs: &["n1_60", "fines_percent", "a_max", "sigma_v", "sigma_v_eff", "z_depth", "magnitude", "f_l_min"], eval: evaluators::liquefaction },
    Entry { id: "C98-07", part: "EN 1998-5", clause: "5.4.2", en: "Pile bending moment from inertial loading", de: "Pfahlbiegemoment aus Trägheitsbeanspruchung", inputs: &["e_pile", "d_pile", "l_pile", "k_h", "head_shear", "head_fixity", "z_check", "m_rd"], eval: evaluators::pile_inertial },
    Entry { id: "C98-08", part: "EN 1998-1", clause: "4.4.2.5", en: "Diaphragm chord force at a section", de: "Gurtkraft einer Deckenscheibe in einem Schnitt", inputs: &["f_storey", "gamma_d", "span_dia", "depth_dia", "x_pos", "chord_area", "chord_fd"], eval: evaluators::diaphragm_chord },
    Entry { id: "C98-09", part: "EN 1998-6", clause: "4.2.4 / Annex B", en: "Modal damping of a tower, mast or chimney", de: "Modale Dämpfung eines Turms, Masts oder Schornsteins", inputs: &["tw_damping_source", "tw_action", "xi_percent", "tw_welded_energy", "tw_bolted_energy", "tw_reinforced_energy", "tw_prestressed_energy", "tw_ceramic_energy", "tw_masonry_energy", "tw_liner_steel_energy", "tw_frp_energy", "tw_sway_energy", "tw_rocking_energy", "tw_vertical_energy", "tw_sway_xi", "tw_rocking_xi", "tw_vertical_xi", "a_g", "s_soil", "t_b", "t_c", "t_d", "t_1", "mass", "v_rd"], eval: evaluators::tower_modal_damping },
    Entry { id: "C98-10", part: "EN 1998-6", clause: "4.2.3 / 8.2", en: "Cable stiffness and preload for a steel guyed mast", de: "Abspannseilsteifigkeit und Vorspannung eines Stahlmasts", inputs: &["guy_mast_height", "guy_mass_model", "guy_sag_model", "guy_modulus_basis", "guy_length", "guy_area", "guy_inclination", "guy_wrap_angle", "guy_modulus", "guy_unit_weight", "guy_equilibrium_stress", "guy_preload_stress", "guy_seismic_stress", "q_factor"], eval: evaluators::guyed_mast_cable },
    Entry { id: "C98-11", part: "EN 1998-4:2006", clause: "6.5.2 / Annex B", en: "Seismic wave passage in a buried welded steel pipe", de: "Seismischer Wellendurchgang in einem geschweißten erdverlegten Stahlrohr", inputs: &["pipe_wave_basis", "pipe_peak_velocity", "pipe_peak_a_long", "pipe_speed_long", "pipe_peak_a_trans", "pipe_speed_trans", "pipe_radius", "pipe_thickness", "pipe_modulus", "pipe_available_shear"], eval: evaluators::pipe_wave },
    Entry { id: "C98-12", part: "EN 1998-2:2005+A2:2011", clause: "3.3(1), (4)–(6)", en: "Continuous bridge deck and Set A ground displacement", de: "Durchlaufender Brückenüberbau und Bodenverschiebung nach Satz A", inputs: &["spatial_deck_length", "spatial_length_limit", "spatial_ground_types", "spatial_ground_displacement", "spatial_reference_distance", "spatial_decorrelation"], eval: evaluators::bridge_spatial_set_a },
    Entry { id: "C98-13", part: "EN 1998-2:2005+A2:2011", clause: "3.3(6)", en: "Set B opposite support displacement", de: "Gegenläufige Auflagerverschiebung nach Satz B", inputs: &["spatial_left_spacing", "spatial_right_spacing", "spatial_governing_strain", "spatial_opposite_factor"], eval: evaluators::bridge_spatial_set_b },
    Entry { id: "C98-14", part: "EN 1998-2:2005+A2:2011", clause: "3.3(7)", en: "Combine inertia and spatial effects of bridge response", de: "Überlagerung der Trägheitswirkung und räumlichen Einwirkungen an Brücken", inputs: &["spatial_effect_kind", "spatial_inertia", "spatial_effect_a", "spatial_effect_b"], eval: evaluators::bridge_spatial_effect },
    Entry { id: "C98-15", part: "EN 1998-3:2005+AC1", clause: "A.4.2.2", en: "Concrete jacket adjustment of monolithic resistance", de: "Anpassung des monolithischen Widerstands bei Betonummantelung", inputs: &["retrofit_basis", "retrofit_shear", "retrofit_moment", "retrofit_yield_rotation", "retrofit_ultimate_rotation"], eval: evaluators::retrofit::concrete },
    Entry { id: "C98-16", part: "EN 1998-3:2005+AC1", clause: "A.4.3.2", en: "Steel jacket shear contribution", de: "Schubanteil des Stahlmantels", inputs: &["retrofit_basis", "jacket_depth", "jacket_thickness", "jacket_width_ratio", "jacket_strut_angle", "jacket_strap_angle", "jacket_design_strength"], eval: evaluators::retrofit::steel },
    Entry { id: "C98-17", part: "EN 1998-3:2005+AC1", clause: "A.4.3.3", en: "Clamping of a lap splice by a steel jacket", de: "Klemmung eines Übergreifungsstoßes durch einen Stahlmantel", inputs: &["jacket_splice_location", "jacket_splice_length", "jacket_length", "jacket_bolts_near", "jacket_bolts_far"], eval: evaluators::retrofit::splice },
    Entry { id: "C98-18", part: "EN 1998-3:2005+AC1", clause: "A.4.4.2(1)–(8)", en: "FRP shear strengthening of a concrete member", de: "Querkraftverstärkung eines Betonbauteils mit Faserverbund", inputs: &["frp_basis", "frp_configuration", "frp_layout", "frp_member", "frp_thickness", "frp_modulus", "frp_diameter", "frp_depth", "frp_radius", "frp_web_width", "frp_ultimate", "frp_web_height", "frp_strip_width", "frp_strip_spacing", "frp_strut_angle", "frp_fibre_angle", "frp_concrete_tensile", "frp_debonding_factor", "frp_existing", "frp_web_cap", "frp_shear_span", "frp_cyclic_cap", "frp_demand"], eval: evaluators::retrofit::frp_shear },
    Entry { id: "C98-19", part: "EN 1998-4:2006", clause: "6.5.3(1)P / 6.5.2(2)P", en: "Welded steel pipe under differential ground movement", de: "Geschweißtes Stahlrohr bei unterschiedlichen Bodenbewegungen", inputs: &["pipe_deformation_basis", "pipe_deformation_radius", "pipe_deformation_thickness", "pipe_deformation_tension", "pipe_deformation_compression", "pipe_deformation_global"], eval: evaluators::pipe_deformation },
    Entry { id: "C98-20", part: "EN 1998-4:2006", clause: "6.5.2(3)P–(4)P", en: "Concrete pipe strain and leak-tightness", de: "Dehnungen und Dichtheit eines Betonrohrs", inputs: &["pipe_concrete_basis", "pipe_concrete_compression", "pipe_concrete_compression_limit", "pipe_steel_tension", "pipe_steel_tension_limit", "pipe_steel_compression", "pipe_steel_compression_limit", "pipe_dls_steel_tension", "pipe_dls_steel_limit"], eval: evaluators::pipe_concrete },
    Entry { id: "C98-21", part: "EN 1998-4:2006", clause: "6.5.2(1)P and (5)P", en: "Buried pipe joint under combined movement", de: "Verbindung eines erdverlegten Rohrs unter kombinierter Bewegung", inputs: &["pipe_joint_basis", "pipe_joint_displacement", "pipe_joint_rotation", "pipe_joint_rotation_limit"], eval: evaluators::pipe_joint },
    Entry { id: "C98-22", part: "EN 1998-4:2006", clause: "6.6(6) and (8)", en: "Pipe protection at a fault crossing", de: "Rohrschutz an einer Verwerfungsquerung", inputs: &["pipe_fault_basis", "pipe_fault_soil", "pipe_fault_wall_a", "pipe_fault_wall_b", "pipe_fault_backfill_a", "pipe_fault_backfill_b", "pipe_fault_trench_a", "pipe_fault_trench_b"], eval: evaluators::pipe_fault_crossing },
    Entry { id: "C98-23", part: "EN 1998-4:2006", clause: "3.3(5)–(9) and (11)–(12)", en: "Horizontal seismic pressure on a silo wall", de: "Seismischer Horizontaldruck auf eine Silowand", inputs: &["silo_pressure_basis", "silo_pressure_section", "silo_pressure_wall", "silo_pressure_fill", "silo_pressure_width", "silo_pressure_height", "silo_pressure_slope", "silo_pressure_acceleration", "silo_pressure_weight", "silo_pressure_azimuth", "silo_pressure_static"], eval: evaluators::silo_wall_pressure },
];

fn case_entry(e: &'static Entry) -> CaseEntry {
    CaseEntry { id: e.id, part: e.part, clause: e.clause, en: e.en, de: e.de, inputs: e.inputs, geometry: None }
}

fn to_results(e: &Entry, annex: AnnexChoice, case: &DesignCase, outcome: Outcome) -> Vec<CheckResult> {
    let subject = SubjectRef::new(&case.id, format!("cases[id={}]", case.id), LocalizedCopy::new(&case.label_en, &case.label_de));
    let clause = ClauseId::new(e.part, e.clause.split('.').next().unwrap_or(e.clause), e.clause);
    let base = |key: &str, en: String, de: String| CheckResult::assess(format!("en1998.{}.{key}.{}", e.id.to_ascii_lowercase(), case.id), e.part, clause.clone(), subject.clone(), LocalizedCopy::new(en, de)).annex(annex);
    match outcome {
        Err(reason) => vec![base("input", e.en.into(), e.de.into()).not_applicable(semio_s_plugin_norm_compliance::reason_copy(e.id, &reason)).build()],
        Ok(rows) => rows
            .into_iter()
            .map(|row| {
                let builder = base(row.key, format!("{} – {}", e.en, row.en), format!("{} – {}", e.de, row.de))
                    .explanation(LocalizedCopy::new(row.steps.clone(), row.steps))
                    .remedies_attached_by("evaluate_case");
                let computed = Quantity::new(row.kind, row.demand);
                if row.status == Some(CheckStatus::Info) {
                    return builder.informative(computed).build();
                }
                if row.status == Some(CheckStatus::UnverifiedBasis) {
                    return builder.status(CheckStatus::UnverifiedBasis).build();
                }
                let builder = builder.utilization(computed, Quantity::new(row.kind, row.capacity));
                match row.status {
                    Some(status) => builder.status(status).build(),
                    None => builder.build(),
                }
            })
            .collect(),
    }
}

/// 📚️ The EN 1998 catalogue as a [`CaseCatalogue`].
pub struct En1998Cases;

impl CaseCatalogue for En1998Cases {
    fn entries() -> Vec<CaseEntry> {
        ENTRIES.iter().map(case_entry).collect()
    }

    fn vocabulary() -> &'static [InputSpec] {
        case_inputs::VOCABULARY
    }

    fn new_document_case(entry: &CaseEntry, id: &str) -> DesignCase {
        let mut case = design_case::seed::<Self>(entry, id);
        let unconfirmed = match entry.id {
            "C98-11" => Some("pipe_wave_basis"),
            "C98-15" | "C98-16" => Some("retrofit_basis"),
            "C98-17" => Some("jacket_splice_location"),
            "C98-18" => Some("frp_basis"),
            "C98-19" => Some("pipe_deformation_basis"),
            "C98-20" => Some("pipe_concrete_basis"),
            "C98-21" => Some("pipe_joint_basis"),
            "C98-22" => Some("pipe_fault_basis"),
            "C98-23" => Some("silo_pressure_basis"),
            _ => None,
        };
        if let Some(key) = unconfirmed {
            case.values = case.values.with_text(key, "unconfirmed");
        }
        case
    }

    fn evaluate(entry: &CaseEntry, annex: AnnexChoice, case: &DesignCase) -> Vec<CheckResult> {
        match ENTRIES.iter().find(|e| e.id == entry.id) {
            Some(e) => to_results(e, annex, case, (e.eval)(&Inputs(case))),
            None => Vec::new(),
        }
    }
}

/// 🧮 One design case through the shared bridge: input issues with remedies, then the catalogue results addressed to the case row with a sizing remedy on failure.
pub fn evaluate_design_case(case: &DesignCase, annex: AnnexChoice) -> Vec<CheckResult> {
    design_case::evaluate_case::<En1998Cases>(case, annex)
}

/// 🌱 The full-catalogue demo cases: one seeded case for every catalogue entry, so that every id is reachable from a document. See [`evaluate_design_case`].
pub fn catalogue_demo_cases() -> Vec<CaseRow> {
    En1998Cases::entries().iter().map(|entry| CaseRow::from(&design_case::seed::<En1998Cases>(entry, &format!("case-{}", entry.id.to_ascii_lowercase())))).collect()
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
