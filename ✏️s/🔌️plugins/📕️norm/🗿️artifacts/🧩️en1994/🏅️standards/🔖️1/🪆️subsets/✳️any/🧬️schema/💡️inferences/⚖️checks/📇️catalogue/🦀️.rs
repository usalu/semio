//! 📇️ Design-case catalogue of EN 1994 catalogue formulas that no snapshot entity reaches: each entry turns an existing pure function into an editable, evaluated case through the shared [`CaseCatalogue`] bridge. See [`super`] and `./📚️inputs/🔣️.json`.

use semio_s_plugin_norm_compliance::design_case::{self, CaseCatalogue, CaseEntry, CaseRow, DesignCase, InputSpec};
use semio_s_plugin_norm_compliance::incremental_evaluation::Scope;
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckReport, CheckResult, ClauseId, LocalizedCopy, Quantity, SubjectRef};

#[path = "📚️inputs/🦀️.rs"]
pub mod case_inputs;

#[path = "🧮️evaluators/🦀️.rs"]
pub mod evaluators;

#[path = "📐️geometry/🦀️.rs"]
pub mod geometry;

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
    Entry { id: "C94-01", part: "EN 1994-1-1", clause: "6.6.4.2", en: "Headed stud resistance with profiled sheeting", de: "Dübeltragfähigkeit mit Profilblech", inputs: &["d_stud", "h_sc", "fu_stud", "fck", "gamma_v", "h_p", "b_0", "t_sheet", "n_r", "ribs", "p_ed"], eval: evaluators::stud_sheeting },
    Entry { id: "C94-02", part: "EN 1994-1-5", clause: "6", en: "Patch loading of a web with bending interaction", de: "Lasteinleitung in den Steg mit Biegeinteraktion", inputs: &["h_w", "t_w", "b_f", "t_f", "fyw", "fyf", "s_s", "a_panel", "gamma_m1", "f_ed", "m_ed", "m_rd"], eval: evaluators::patch_loading },
    Entry { id: "C94-03", part: "EN 1994-1-1", clause: "6.7.4", en: "Load introduction into a composite column", de: "Lasteinleitung in eine Verbundstütze", inputs: &["kind", "n_intro", "perimeter", "d_min", "l_col", "p_rd"], eval: evaluators::load_introduction },
    Entry { id: "C94-04", part: "EN 1994-1-1", clause: "7.3.1", en: "Deflection of a simply supported composite beam", de: "Durchbiegung eines Verbundträgers als Einfeldträger", inputs: &["l_span", "g_line", "q_line", "i_long", "i_short", "span_ratio"], eval: evaluators::deflection_simple_span },
    Entry { id: "C94-05", part: "EN 1994-1-1", clause: "7.3.1", en: "Deflection of a two-span composite beam", de: "Durchbiegung eines zweifeldrigen Verbundträgers", inputs: &["l_span", "g_line", "q_line", "i_long", "i_short", "span_ratio"], eval: evaluators::deflection_two_span },
    Entry { id: "C94-06", part: "EN 1994-1-2", clause: "3.2", en: "Steel strength at elevated temperature", de: "Stahlfestigkeit bei erhöhter Temperatur", inputs: &["theta_a", "fy", "sigma_ed"], eval: evaluators::steel_at_temperature },
    Entry { id: "C94-07", part: "EN 1994-1-1", clause: "6.2.1.2", en: "Sagging plastic moment resistance with full shear connection", de: "Plastische Momententragfähigkeit im Feld bei vollständiger Verdübelung", inputs: &["sec_h", "sec_b", "sec_tf", "sec_tw", "sec_a", "sec_iy", "fyd", "ys", "b_eff", "h_c", "fck", "gamma_c", "m_ed"], eval: evaluators::sagging_plastic },
    Entry { id: "C94-08", part: "EN 1994-1-1", clause: "6.2.1.2", en: "Hogging plastic moment resistance", de: "Plastische Momententragfähigkeit über der Stütze", inputs: &["sec_h", "sec_b", "sec_tf", "sec_tw", "sec_a", "sec_iy", "fyd", "ys", "as_rebar", "fsd", "y_r", "m_ed"], eval: evaluators::hogging_plastic },
    Entry { id: "C94-09", part: "EN 1994-1-1", clause: "7.3.1", en: "Natural frequency of a composite floor beam", de: "Eigenfrequenz eines Verbunddeckenträgers", inputs: &["l_span", "g_line", "q_line", "psi_vib", "i_short", "f_min"], eval: evaluators::floor_vibration },
    Entry { id: "C94-10", part: "EN 1994-1-1", clause: "6.4.2", en: "Lateral-torsional buckling of a continuous composite beam over the support", de: "Biegedrillknicken eines Durchlaufverbundträgers über der Stütze", inputs: &["sec_h", "sec_b", "sec_tf", "sec_tw", "sec_a", "sec_iy", "fy", "gamma_m1", "m_ed", "l_ltb", "i2_slab", "a_beam", "k_alpha", "steel_kind"], eval: evaluators::ltb_continuous },
    Entry { id: "C94-11", part: "EN 1994-1-1", clause: "7.3.1", en: "Deflection with creep and shrinkage of a composite beam", de: "Durchbiegung eines Verbundträgers mit Kriechen und Schwinden", inputs: &["sec_h", "sec_b", "sec_tf", "sec_tw", "sec_a", "sec_iy", "ys", "b_eff", "h_c", "fck", "phi_t", "psi_l", "eps_cs", "l_span", "g_line", "q_line", "span_ratio"], eval: evaluators::deflection_creep_shrinkage },
    Entry { id: "C94-12", part: "EN 1994-1-2", clause: "4.3", en: "Protected steel temperature against the critical temperature", de: "Temperatur des bekleideten Stahls gegen die kritische Temperatur", inputs: &["am_v", "d_p", "lambda_p", "c_p", "rho_p", "t_fire", "mu0"], eval: evaluators::protected_steel_fire },
];

/// 🚫 A verification an entry names but does not model: entry id, result key, title and reason in English and German.
struct NotModelled {
    entry: &'static str,
    key: &'static str,
    title: (&'static str, &'static str),
    reason: (&'static str, &'static str),
}

/// 🚫 The floor acceleration and comfort criterion of EN 1990 A1.4.4 is a National Annex or project value that the repository text does not contain, so no limit is encoded and the response is reported as not modelled next to the frequency check.
static NOT_MODELLED: &[NotModelled] = &[NotModelled {
    entry: "C94-09",
    key: "acceleration",
    title: ("Floor acceleration and comfort", "Deckenbeschleunigung und Schwingungskomfort"),
    reason: (
        "EN 1990 A1.4.4 leaves the acceleration limit and the response model to the National Annex or the project; neither is in the repository text, so no limit is assumed and the response is not modelled (only f1 against fmin is checked).",
        "EN 1990 A1.4.4 überlässt Beschleunigungsgrenze und Antwortmodell dem Nationalen Anhang oder dem Projekt; beides steht nicht im Text des Repositorys, daher wird keine Grenze angenommen und die Antwort nicht modelliert (geprüft wird nur f1 gegen fmin).",
    ),
}];

fn case_entry(e: &'static Entry) -> CaseEntry {
    CaseEntry { id: e.id, part: e.part, clause: e.clause, en: e.en, de: e.de, inputs: e.inputs, geometry: geometry::geometry_of(e.id) }
}

fn to_results(e: &Entry, annex: AnnexChoice, case: &DesignCase, outcome: Outcome) -> Vec<CheckResult> {
    let subject = SubjectRef::new(&case.id, format!("cases[id={}]", case.id), LocalizedCopy::new(&case.label_en, &case.label_de));
    let clause = ClauseId::new(e.part, e.clause.split('.').next().unwrap_or(e.clause), e.clause);
    let base = |key: &str, en: String, de: String| CheckResult::assess(format!("en1994.{}.{key}.{}", e.id.to_ascii_lowercase(), case.id), e.part, clause.clone(), subject.clone(), LocalizedCopy::new(en, de)).annex(annex);
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
            .chain(NOT_MODELLED.iter().filter(|gap| gap.entry == e.id).map(|gap| base(gap.key, format!("{} – {}", e.en, gap.title.0), format!("{} – {}", e.de, gap.title.1)).not_applicable(LocalizedCopy::new(gap.reason.0, gap.reason.1)).build()))
            .collect(),
    }
}

/// 📚️ The EN 1994 catalogue as a [`CaseCatalogue`].
pub struct En1994Cases;

impl CaseCatalogue for En1994Cases {
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
    design_case::evaluate_case::<En1994Cases>(case, annex)
}

/// 📇️ The design cases of a document restricted to `scope` (one case row of `cases`, or all): every case through the shared bridge, in document order.
pub fn evaluate_design_cases(annex: AnnexChoice, cases: &[CaseRow], scope: Scope<'_>) -> CheckReport {
    let mut report = CheckReport::default();
    for (_, case) in cases.iter().enumerate().filter(|(index, _)| scope.admits("cases", *index)) {
        report.extend(evaluate_design_case(&DesignCase::from(case), annex));
    }
    report
}

/// 🌱 The full-catalogue demo cases: one seeded case for every catalogue entry, so that every id is reachable from a document. See [`evaluate_design_case`].
pub fn catalogue_demo_cases() -> Vec<CaseRow> {
    En1994Cases::entries().iter().map(|entry| CaseRow::from(&design_case::seed::<En1994Cases>(entry, &format!("case-{}", entry.id.to_ascii_lowercase())))).collect()
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
