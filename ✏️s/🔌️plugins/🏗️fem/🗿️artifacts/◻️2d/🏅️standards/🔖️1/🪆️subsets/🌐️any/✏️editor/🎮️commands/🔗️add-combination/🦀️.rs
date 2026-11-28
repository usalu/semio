//! 🏋️ 🏋️ Fem2d play app commands command — `add-combination`.

use semio_plugin::NoConfig;

use semio_plugin::NoConfigMutation;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::{FemCombination, FemCombinationTerm};
use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::combination_terms::{parse_combination_terms, CombinationTermsError};
use semio_plugin::ArtifactView;
use semio_plugin::ConfigView;
use semio_plugin::Emit;
use semio_plugin::Fault;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

type Fem2dSnapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "add-combination")]
pub struct AddCombination {
    pub name: String,
    /// 📦️ Case ids with factors as text, admitted by [`parse_terms`]; the syntax is shared with FEM 3D through `parse_combination_terms`.
    pub terms: String,
}

/// 🔗️ A refusal of the `fem2d.combination.terms-*` family; each code has an EN/DE notice in the editor's `fault_notices` table, filled from `params`.
fn refuse(code: &'static str, message: String, params: &[(&str, &str)]) -> Fault {
    params.iter().fold(Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new(code), message), |fault, (name, value)| fault.with_param(*name, *value))
}

/// 🔗️ Admits `terms` as typed by a user — `dead:1.35, live:1.5`, `dead=1.35, live=1.5`, `1.35 dead + 1.5 live` — or as JSON (`[[caseId, factor]]` pairs, `{caseId, factor}` records or one `{caseId: factor}` object). A case is named by its id or, ignoring case, by its name. Text that cannot be read, an empty list, a repeated or unknown case and a non-finite factor are refused with a localized `fem2d.combination.terms-*` notice.
pub fn parse_terms(text: &str, doc: &Fem2dSnapshot) -> Result<Vec<FemCombinationTerm>, Fault> {
    let pairs = parse_combination_terms(text).map_err(|error| match error {
        CombinationTermsError::Unreadable => refuse("fem2d.combination.terms-unreadable", format!("terms must read like dead:1.35, live:1.5 or [[\"dead\", 1.35]], got '{text}'"), &[("terms", text)]),
        CombinationTermsError::Empty => refuse("fem2d.combination.terms-empty", "a combination needs at least one term".into(), &[]),
    })?;
    let mut terms: Vec<FemCombinationTerm> = Vec::new();
    for (named, factor) in pairs {
        if !factor.is_finite() { return Err(refuse("fem2d.combination.terms-factor-invalid", format!("the factor of '{named}' is not a finite number"), &[("case", &named)])); }
        let case = doc.load_cases.iter().find(|case| case.id == named).or_else(|| doc.load_cases.iter().find(|case| case.name.eq_ignore_ascii_case(&named))).ok_or_else(|| refuse("fem2d.combination.terms-case-unknown", format!("'{named}' is not a load case"), &[("case", &named)]))?;
        if terms.iter().any(|term| term.case_id == case.id) { return Err(refuse("fem2d.combination.terms-case-repeated", format!("'{}' appears twice", case.id), &[("case", &case.id)])); }
        terms.push(FemCombinationTerm { case_id: case.id.clone(), factor });
    }
    Ok(terms)
}

pub fn handle(payload: &AddCombination, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let terms = parse_terms(&payload.terms, doc.snapshot)?;
    let id = doc.operation()?.entity_id("c", "combinations")?;
    Ok(Emit::mutations(vec![Fem2dMutation::CreateCombination(semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::create_combination::CreateCombination { combination: FemCombination { id, name: payload.name.clone(), terms }, index: None })]))
}
