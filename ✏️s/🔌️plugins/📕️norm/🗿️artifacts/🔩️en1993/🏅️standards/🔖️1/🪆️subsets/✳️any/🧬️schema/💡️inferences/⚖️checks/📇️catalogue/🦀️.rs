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
