//! 📇️ Design-case catalogue of EN 1998 kernel formulas that no snapshot entity reaches: each entry turns an existing pure function into an editable, evaluated case through the shared [`CaseCatalogue`] bridge. See [`super`] and `./📚️inputs/🔣️.json`.

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
    Entry { id: "C98-01", part: "EN 1998-1", clause: "4.3.3.2.2", en: "Lateral force base shear from the design spectrum", de: "Fundamentschubkraft aus dem Bemessungsspektrum", inputs: &["a_g", "s_soil", "t_b", "t_c", "t_d", "q_factor", "t_1", "mass", "lambda_c", "v_rd"], eval: evaluators::base_shear_design },
    Entry { id: "C98-02", part: "EN 1998-1", clause: "3.2.2.2", en: "Elastic response force with damping correction", de: "Elastische Antwortkraft mit Dämpfungskorrektur", inputs: &["a_g", "s_soil", "t_b", "t_c", "t_d", "t_1", "mass", "xi_percent", "v_rd"], eval: evaluators::elastic_force },
    Entry { id: "C98-03", part: "EN 1998-1", clause: "3.2.2.3", en: "Vertical seismic force from the vertical spectrum", de: "Vertikale Erdbebenkraft aus dem Vertikalspektrum", inputs: &["a_g", "avg_ratio", "t_v", "mass", "f_rd_v"], eval: evaluators::vertical_force },
    Entry { id: "C98-04", part: "EN 1998-1", clause: "4.3.3.3", en: "Modal response spectrum analysis of a three-storey shear building", de: "Antwortspektrenverfahren eines dreigeschossigen Schubbalkens", inputs: &["m_1", "m_2", "m_3", "k_1", "k_2", "k_3", "a_g", "s_soil", "t_b", "t_c", "t_d", "q_factor", "xi_percent", "v_rd"], eval: evaluators::modal_analysis },
    Entry { id: "C98-05", part: "EN 1998-1", clause: "3.1.2", en: "Ground type from the shear wave velocity of three layers", de: "Baugrundklasse aus der Scherwellengeschwindigkeit dreier Schichten", inputs: &["h_1", "h_2", "h_3", "vs_1", "vs_2", "vs_3", "vs_min"], eval: evaluators::ground_velocity },
    Entry { id: "C98-06", part: "EN 1998-5", clause: "4.1.4", en: "Liquefaction safety factor of a sand layer", de: "Sicherheit gegen Bodenverflüssigung einer Sandschicht", inputs: &["n1_60", "fines_percent", "a_max", "sigma_v", "sigma_v_eff", "z_depth", "magnitude", "f_l_min"], eval: evaluators::liquefaction },
    Entry { id: "C98-07", part: "EN 1998-5", clause: "5.4.2", en: "Pile bending moment from inertial loading", de: "Pfahlbiegemoment aus Trägheitsbeanspruchung", inputs: &["e_pile", "d_pile", "l_pile", "k_h", "head_shear", "head_fixity", "z_check", "m_rd"], eval: evaluators::pile_inertial },
    Entry { id: "C98-08", part: "EN 1998-1", clause: "4.4.2.5", en: "Diaphragm chord force at a section", de: "Gurtkraft einer Deckenscheibe in einem Schnitt", inputs: &["f_storey", "gamma_d", "span_dia", "depth_dia", "x_pos", "chord_area", "chord_fd"], eval: evaluators::diaphragm_chord },
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
                base(row.key, format!("{} – {}", e.en, row.en), format!("{} – {}", e.de, row.de))
                    .explanation(LocalizedCopy::new(row.steps.clone(), row.steps))
                    .remedies_attached_by("evaluate_case")
                    .utilization(Quantity::new(row.kind, row.demand), Quantity::new(row.kind, row.capacity))
                    .build()
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
