//! 📇️ Design-case catalogue of the EN 1993 clause functions that no snapshot entity reaches: each entry turns an existing pure function into an editable, evaluated case through the shared [`CaseCatalogue`] bridge. See [`super`] and `./📚️inputs/🔣️.json`.

use semio_s_plugin_norm_compliance::design_case::{self, CaseCatalogue, CaseEntry, CaseRow, DesignCase, InputSpec};
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckResult, ClauseId, LocalizedCopy, Quantity, SubjectRef};

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
    Entry { id: "C93-01", part: "EN 1993-1-7", clause: "6", en: "Plate under transverse pressure", de: "Platte unter Querdruck", inputs: &["k_plate", "p_plate", "b_plate", "t_plate", "fy", "gm0"], eval: evaluators::plate_lateral },
    Entry { id: "C93-02", part: "EN 1993-2", clause: "9.5.2", en: "Bridge fatigue with damage equivalence factors", de: "Brückenermüdung mit Schadensäquivalenzfaktoren", inputs: &["lambda_1", "lambda_2", "lambda_3", "lambda_4", "lambda_max", "phi_2", "delta_sigma_p", "delta_sigma_c", "gamma_ff", "gamma_mf"], eval: evaluators::bridge_fatigue },
    Entry { id: "C93-03", part: "EN 1993-1-9", clause: "8", en: "Fatigue under combined normal and shear stress ranges", de: "Ermüdung unter kombinierten Normal- und Schubspannungsschwingbreiten", inputs: &["delta_sigma", "delta_sigma_c", "delta_tau", "delta_tau_c", "gamma_ff", "gamma_mf"], eval: evaluators::fatigue_combined },
    Entry { id: "C93-04", part: "EN 1993-5", clause: "5.2.2", en: "Sheet pile bending resistance", de: "Biegetragfähigkeit der Spundwand", inputs: &["m_ed", "beta_b", "w_el", "fy", "gm0"], eval: evaluators::sheet_pile_bending },
    Entry { id: "C93-05", part: "EN 1993-1-11", clause: "6.2", en: "Cable tension resistance", de: "Zugtragfähigkeit des Seils", inputs: &["f_ed", "f_uk", "f_k", "gamma_r"], eval: evaluators::cable_tension },
    Entry { id: "C93-06", part: "EN 1993-3-1", clause: "5", en: "Wind-amplified tower shaft compression", de: "Windverstärkte Druckbeanspruchung des Mastschafts", inputs: &["n_ed", "amplification", "chi", "area", "fy", "gm1"], eval: evaluators::tower_leg },
    Entry { id: "C93-07", part: "EN 1993-1-8", clause: "6.2.8", en: "Base plate T-stub in compression", de: "Fußplatten-T-Stummel auf Druck", inputs: &["f_ed", "beta_j", "k_j", "f_cd", "t_plate", "fy", "gm0", "t_flange", "b_flange"], eval: evaluators::base_plate },
    Entry { id: "C93-08", part: "EN 1993-6", clause: "5.7.1", en: "Crane runway web under wheel load", de: "Kranbahnsteg unter Radlast", inputs: &["i_rf", "t_web", "fz_ed", "fy", "gm0"], eval: evaluators::crane_web },
    Entry { id: "C93-09", part: "EN 1993-1-1", clause: "6.2.1", en: "Combined stresses by the von Mises criterion", de: "Spannungsüberlagerung nach von Mises", inputs: &["sigma_x", "sigma_z", "tau", "fy", "gm0"], eval: evaluators::von_mises },
    Entry { id: "C93-10", part: "EN 1993-1-1", clause: "6.2.9.2", en: "Class 3 cross-section under axial force and bending", de: "Querschnitt der Klasse 3 unter Normalkraft und Biegung", inputs: &["n_ed", "area", "my_ed", "w_el_y", "mz_ed", "w_el_z", "fy", "gm0"], eval: evaluators::class3_stress },
    Entry { id: "C93-11", part: "EN 1993-1-1", clause: "6.2.9.1", en: "I section under axial force and biaxial bending", de: "I-Querschnitt unter Normalkraft und zweiachsiger Biegung", inputs: &["n_ed", "area", "a_web", "my_ed", "mz_ed", "w_pl_y", "w_pl_z", "fy", "gm0"], eval: evaluators::biaxial_bending },
    Entry { id: "C93-12", part: "EN 1993-1-5", clause: "7.1", en: "Interaction of bending and shear in plate girders", de: "Interaktion von Biegung und Querkraft bei Blechträgern", inputs: &["m_ed", "m_pl_rd", "m_f_rd", "v_ed", "v_bw_rd"], eval: evaluators::bending_shear_interaction },
    Entry { id: "C93-13", part: "EN 1993-1-2", clause: "4.2.4", en: "Critical temperature against ISO 834 steel temperature", de: "Kritische Temperatur gegen Stahltemperatur nach ISO 834", inputs: &["e_fi_d", "r_fi_d_0", "k_sh", "am_over_v", "t_fire", "eps_m", "eps_f"], eval: evaluators::critical_temperature },
    Entry { id: "C93-14", part: "EN 1993-4-3", clause: "6", en: "Pipe wall thickness under internal pressure", de: "Rohrwanddicke unter Innendruck", inputs: &["p_int", "d_pipe", "t_pipe", "fy", "gm0"], eval: evaluators::pipe_hoop },
    Entry { id: "C93-15", part: "EN 1993-4-3", clause: "7", en: "Ovalisation of a buried pipe", de: "Ovalisierung eines erdverlegten Rohres", inputs: &["k_b", "w_line", "d_pipe", "e_pipe", "i_wall", "e_soil", "ovalisation_limit"], eval: evaluators::pipe_ovalisation },
    Entry { id: "C93-16", part: "EN 1993-1-9", clause: "Annex B / 8(1)–(2)", en: "Weld toe fatigue using structural hot-spot stress", de: "Ermüdung am Nahtübergang mit Strukturspannung", inputs: &["hot_spot_detail", "hot_spot_crack", "hot_spot_basis", "hot_spot_angle", "hot_spot_range", "hot_spot_frequent", "fy", "gamma_ff", "gamma_mf"], eval: evaluators::hot_spot_fatigue },
    Entry { id: "C93-17", part: "EN 1993-1-9", clause: "6.4 / Tables 4.1–4.2", en: "Fatigue of a welded hollow-section lattice joint", de: "Ermüdung eines geschweißten Hohlprofil-Fachwerkknotens", inputs: &["lattice_section", "lattice_joint", "lattice_member", "lattice_basis", "lattice_beta", "lattice_gamma", "lattice_tau", "lattice_angle", "lattice_range", "delta_sigma_c", "gamma_ff", "gamma_mf"], eval: evaluators::lattice_joint_fatigue },
    Entry { id: "C93-18", part: "EN 1993-1-9", clause: "8.1–8.2 / Table 8.10", en: "Top flange-to-web fatigue of crane runway beams", de: "Ermüdung des Obergurt-Steg-Anschlusses von Kranbahnträgern", inputs: &["runway_detail", "runway_location", "runway_basis", "runway_range", "runway_frequent", "fy", "gamma_ff", "gamma_mf"], eval: evaluators::runway_junction_fatigue },
    Entry { id: "C93-19", part: "EN 1993-1-9", clause: "8.1–8.2 / Table 8.9(1)", en: "Fatigue of an open stringer at a cross girder", de: "Ermüdung einer offenen Längsrippe am Querträger", inputs: &["deck_open_end_basis", "deck_open_thickness", "deck_open_range", "deck_open_frequent_direct", "fy", "gamma_ff", "gamma_mf"], eval: evaluators::open_stringer_fatigue },
    Entry { id: "C93-20", part: "EN 1993-1-9", clause: "8.1–8.2 / Table 8.9(2)", en: "Fatigue at a continuous open stringer through a cross girder", de: "Ermüdung am Querträger bei durchlaufender offener Längsrippe", inputs: &["deck_open_cross_basis", "deck_cross_direct", "deck_cross_shear", "deck_cross_equivalence", "deck_open_frequent_direct", "deck_cross_frequent_shear", "deck_between_utilization", "fy", "gamma_ff", "gamma_mf"], eval: evaluators::continuous_open_stringer_fatigue },
    Entry { id: "C93-21", part: "EN 1993-1-9", clause: "8.1–8.2 / Table 8.8(1)–(4)", en: "Closed stringer connections and backed splices", de: "Anschlüsse geschlossener Längsrippen und Stöße mit Badsicherung", inputs: &["closed_connection_detail", "closed_geometry", "closed_web_thickness", "closed_range", "closed_frequent", "fy", "gamma_ff", "gamma_mf"], eval: evaluators::closed_stringer_fatigue },
    Entry { id: "C93-22", part: "EN 1993-1-9", clause: "8.1–8.2 / Table 8.8(6)", en: "Cross-girder web with a closed-stringer cutout", de: "Querträgersteg mit Ausschnitt für eine geschlossene Längsrippe", inputs: &["closed_cutout_method", "closed_geometry", "closed_range", "closed_frequent", "fy", "gamma_ff", "gamma_mf"], eval: evaluators::closed_stringer_cutout_fatigue },
    Entry { id: "C93-23", part: "EN 1993-1-9", clause: "8.1–8.2 / Table 8.8(5)", en: "Unbacked butt splice in a closed stringer", de: "Stumpfstoß einer geschlossenen Längsrippe ohne Badsicherung", inputs: &["closed_splice_group", "closed_geometry", "closed_splice_thickness", "closed_range", "closed_frequent", "fy", "gamma_ff", "gamma_mf"], eval: evaluators::closed_stringer_splice_fatigue },
    Entry { id: "C93-24", part: "EN 1993-1-9", clause: "8.1–8.2 / Table 8.8(7)–(8)", en: "Deck plate weld to a trapezoidal or V-shaped rib", de: "Deckblechnaht zu einer trapez- oder V-förmigen Längsrippe", inputs: &["closed_weld_type", "closed_geometry", "closed_weld_rib_thickness", "closed_weld_throat", "closed_weld_gap", "closed_weld_root_face", "closed_weld_moment", "closed_weld_frequent_moment", "fy", "gamma_ff", "gamma_mf"], eval: evaluators::closed_stringer_deck_weld_fatigue },
    Entry { id: "C93-25", part: "EN 1993-1-9", clause: "6.4, 8 / Table 8.7", en: "Welded lattice joint chord or brace fatigue", de: "Ermüdung des Gurts oder einer Strebe am geschweißten Fachwerkknoten", inputs: &["node_section", "node_joint", "node_member", "node_stress_basis", "node_geometry", "node_t0", "node_ti", "node_chord_width", "node_chord_height", "node_brace_width", "node_angle", "node_eip", "node_eop", "node_gap", "node_overlap", "node_beta", "node_gamma", "node_equiv_range", "node_frequent_range", "fy", "gamma_ff", "gamma_mf"], eval: evaluators::lattice_node_fatigue },
    Entry { id: "C93-26", part: "EN 1993-1-9", clause: "8 / Table 8.6", en: "Fatigue of hollow-section connections", de: "Ermüdung von Hohlprofilanschlüssen", inputs: &["hollow_detail", "hollow_execution", "hollow_wall", "hollow_diameter", "hollow_slot_angle", "hollow_weld_dimension", "hollow_reinforcement", "hollow_attachment_width", "hollow_range", "hollow_frequent", "hollow_weld_shear", "hollow_weld_frequent_shear", "fy", "gamma_ff", "gamma_mf"], eval: evaluators::hollow_section_fatigue },
    Entry { id: "C93-27", part: "EN 1993-1-9", clause: "8 / Table 8.5", en: "Fatigue of load-carrying welds", de: "Ermüdung tragender Schweißnähte", inputs: &["weld85_detail", "weld85_basis", "weld85_length", "weld85_plate", "weld85_cover", "weld85_intermediate", "weld85_misalignment", "weld85_end_distance", "weld85_reinforcing_length", "weld85_slope", "weld85_penetration", "weld85_range", "weld85_frequent", "weld85_root", "weld85_root_frequent", "weld85_shear", "weld85_shear_frequent", "fy", "gamma_ff", "gamma_mf"], eval: evaluators::load_carrying_weld_fatigue },
    Entry { id: "C93-28", part: "EN 1993-1-9", clause: "8 / Table 8.4", en: "Fatigue of welded attachments and stiffeners", de: "Ermüdung angeschweißter Anschlüsse und Steifen", inputs: &["weld84_detail", "weld84_basis", "weld84_length", "weld84_thickness", "weld84_height", "weld84_angle", "weld84_radius", "weld84_reinforced_length", "weld84_range", "weld84_frequent", "fy", "gamma_ff", "gamma_mf"], eval: evaluators::attachment_fatigue },
    Entry { id: "C93-29", part: "EN 1993-1-9", clause: "8 / Table 8.3", en: "Fatigue of transverse butt welds", de: "Ermüdung quer laufender Stumpfnähte", inputs: &["weld83_detail", "weld83_basis", "weld83_thickness", "weld83_thicker", "weld83_offset", "weld83_slope", "weld83_weld_width", "weld83_reinforcement", "weld83_end_distance", "weld83_fit", "weld83_length", "weld83_radius", "weld83_cross_reference", "weld83_flange", "weld83_range", "weld83_frequent", "weld83_perpendicular_range", "weld83_perpendicular_frequent", "fy", "gamma_ff", "gamma_mf"], eval: evaluators::transverse_butt_weld_fatigue },
    Entry { id: "C93-30", part: "EN 1993-1-9", clause: "8 / Table 8.2", en: "Fatigue of longitudinal welds in built-up sections", de: "Ermüdung von Längsnähten in zusammengesetzten Querschnitten", inputs: &["weld82_detail", "weld82_basis", "weld82_stop", "weld82_repair", "weld82_finish", "weld82_gap", "weld82_web_height", "weld82_cope_height", "weld82_cope_basis", "weld82_attachment_length", "weld82_attachment_thickness", "weld82_attachment_height", "weld82_wall", "weld82_ends", "weld82_ends_utilization", "weld82_range", "weld82_frequent", "fy", "gamma_ff", "gamma_mf"], eval: evaluators::built_up_weld_fatigue },
    Entry { id: "C93-31", part: "EN 1993-1-9", clause: "8 / Table 8.1", en: "Fatigue of plain members and mechanically fastened joints", de: "Ermüdung ungeschweißter Bauteile und mechanischer Verbindungen", inputs: &["detail81_detail", "detail81_basis", "detail81_steel", "detail81_diameter", "detail81_end_parallel", "detail81_edge_transverse", "detail81_pitch_parallel", "detail81_pitch_transverse", "detail81_reversal", "detail81_threads", "detail81_bolt_type", "detail81_bolt_grade", "detail81_first_moment", "detail81_inertia", "detail81_thickness", "detail81_shear_force", "detail81_frequent_force", "detail81_range", "detail81_frequent", "fy", "gamma_ff", "gamma_mf"], eval: evaluators::plain_and_fastened_fatigue },
    Entry { id: "C93-32", part: "EN 1993-1-9:2005", clause: "3(2), 3(3), 3(7)", en: "Fatigue assessment method and maintenance basis", de: "Ermüdungsnachweiskonzept und Instandhaltungsgrundlagen", inputs: &["fatigue_method", "fatigue_rapid_failure", "fatigue_lifetime_maintenance", "fatigue_tolerance_basis", "fatigue_maintenance_record"], eval: evaluators::fatigue_method::evaluate },
];

fn case_entry(e: &'static Entry) -> CaseEntry {
    CaseEntry { id: e.id, part: e.part, clause: e.clause, en: e.en, de: e.de, inputs: e.inputs, geometry: None }
}

fn to_results(e: &Entry, annex: AnnexChoice, case: &DesignCase, outcome: Outcome) -> Vec<CheckResult> {
    let subject = SubjectRef::new(&case.id, format!("cases[id={}]", case.id), LocalizedCopy::new(&case.label_en, &case.label_de));
    let clause = ClauseId::new(e.part, e.clause.split('.').next().unwrap_or(e.clause), e.clause);
    let base = |key: &str, en: String, de: String| CheckResult::assess(format!("en1993.{}.{key}.{}", e.id.to_ascii_lowercase(), case.id), e.part, clause.clone(), subject.clone(), LocalizedCopy::new(en, de)).annex(annex);
    match outcome {
        Err(reason) => vec![base("input", e.en.into(), e.de.into()).not_applicable(semio_s_plugin_norm_compliance::reason_copy(e.id, &reason)).build()],
        Ok(rows) => rows
            .into_iter()
            .map(|row| {
                base(row.key, format!("{} – {}", e.en, row.en), format!("{} – {}", e.de, row.de))
                    .explanation(LocalizedCopy::new(row.steps.clone(), row.steps))
                    .remedies_attached_by("evaluate_case")
                    .utilization(Quantity::new(row.kind, row.demand), Quantity::new(row.kind, row.capacity))
                    .build()
            })
            .collect(),
    }
}

/// 📚️ The EN 1993 catalogue as a [`CaseCatalogue`].
pub struct En1993Cases;

impl CaseCatalogue for En1993Cases {
    fn entries() -> Vec<CaseEntry> {
        ENTRIES.iter().map(case_entry).collect()
    }

    fn vocabulary() -> &'static [InputSpec] {
        case_inputs::VOCABULARY
    }

    fn new_document_case(entry: &CaseEntry, id: &str) -> DesignCase {
        let mut case = design_case::seed::<Self>(entry, id);
        if entry.id == "C93-32" {
            case.values = design_case::CaseValues::default().with_text("fatigue_method", "unknown");
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
    design_case::evaluate_case::<En1993Cases>(case, annex)
}

/// 🌱 The full-catalogue demo cases: one seeded case for every catalogue entry, so that every id is reachable from a document. See [`evaluate_design_case`].
pub fn catalogue_demo_cases() -> Vec<CaseRow> {
    En1993Cases::entries().iter().map(|entry| CaseRow::from(&design_case::seed::<En1993Cases>(entry, &format!("case-{}", entry.id.to_ascii_lowercase())))).collect()
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
