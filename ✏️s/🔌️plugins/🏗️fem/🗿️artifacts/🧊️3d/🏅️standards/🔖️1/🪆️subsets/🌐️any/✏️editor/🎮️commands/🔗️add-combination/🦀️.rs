//! 🏋️ 🏋️ FEM 3D app commands command — `add-combination`.

use semio_plugin::NoConfig;

use semio_plugin::NoConfigMutation;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::create_combination;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_plugin::ArtifactView;
use semio_plugin::ConfigView;
use semio_plugin::Emit;
use semio_plugin::Fault;
use semio_framework_value::DslValue;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "add-combination")]
pub struct AddCombination {
    pub name: String,
    /// 📦️ Case ids with factors as text, admitted by [`parse_terms`]: `FemCombination::terms` is a `BTreeMap<String, f64>`, which the wire carries as text.
    pub terms: String,
}

/// 🔗️ A refusal of the `fem3d.combination.terms-*` family; each code has an EN/DE notice in the editor's `fault_notices` table, filled from `params`.
fn refuse(code: &'static str, message: String, params: &[(&str, &str)]) -> Fault {
    params.iter().fold(Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new(code), message), |fault, (name, value)| fault.with_param(*name, *value))
}

fn json_pairs(text: &str) -> Option<Vec<(String, f64)>> {
    let value = semio_framework_pack_json::from_json_str::<DslValue>(text, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()?;
    match &value {
        DslValue::Object(entries) => entries.iter().map(|(case, factor)| factor.as_f64().map(|factor| (case.clone(), factor))).collect(),
        DslValue::Array(items) => items.iter().map(|item| match item {
            DslValue::Array(pair) if pair.len() == 2 => pair[0].as_str().zip(pair[1].as_f64()).map(|(case, factor)| (case.to_owned(), factor)),
            DslValue::Object(_) => item.get("caseId").and_then(DslValue::as_str).zip(item.get("factor").and_then(DslValue::as_f64)).map(|(case, factor)| (case.to_owned(), factor)),
            _ => None,
        }).collect(),
        _ => None,
    }
}

/// 🔤️ One `case:factor`, `case=factor`, `factor*case`, `factor case` or `case factor` segment.
fn natural_pair(segment: &str) -> Option<(String, f64)> {
    let number = |text: &str| text.trim().parse::<f64>().ok();
    let (left, right) = match segment.split_once([':', '=', '*', '×']) {
        Some((left, right)) => (left.trim(), right.trim()),
        None => {
            let (first, rest) = segment.split_once(char::is_whitespace)?;
            if number(first).is_some() { (first, rest.trim()) } else { let (rest, last) = segment.rsplit_once(char::is_whitespace)?; (rest.trim(), last) }
        }
    };
    match (number(left), number(right)) {
        (Some(factor), None) if !right.is_empty() => Some((right.to_owned(), factor)),
        (None, Some(factor)) if !left.is_empty() => Some((left.to_owned(), factor)),
        _ => None,
    }
}

/// 🔤️ Terms typed as text: segments split by `,` `;` `+` or a line break, each a [`natural_pair`].
fn natural_pairs(text: &str) -> Option<Vec<(String, f64)>> {
    text.split([',', ';', '+', '\n']).map(str::trim).filter(|segment| !segment.is_empty()).map(natural_pair).collect()
}

/// 🔗️ Admits `terms` as typed by a user — `dead:1.35, live:1.5`, `dead=1.35, live=1.5`, `1.35 dead + 1.5 live` — or as JSON (`[[caseId, factor]]` pairs, `{caseId, factor}` records or one `{caseId: factor}` object). A case is named by its id or, ignoring case, by its name. Text that cannot be read, an empty list, a repeated or unknown case and a non-finite factor are refused with a localized `fem3d.combination.terms-*` notice.
pub fn parse_terms(text: &str, doc: &Fem3dSnapshot) -> Result<std::collections::BTreeMap<String, f64>, Fault> {
    let trimmed = text.trim();
    let pairs = if trimmed.starts_with(['[', '{']) { json_pairs(trimmed) } else { natural_pairs(trimmed) }.ok_or_else(|| refuse("fem3d.combination.terms-unreadable", format!("terms must read like dead:1.35, live:1.5 or [[\"dead\", 1.35]], got '{text}'"), &[("terms", text)]))?;
    if pairs.is_empty() { return Err(refuse("fem3d.combination.terms-empty", "a combination needs at least one term".into(), &[])); }
    let mut terms = std::collections::BTreeMap::new();
    for (named, factor) in pairs {
        if !factor.is_finite() { return Err(refuse("fem3d.combination.terms-factor-invalid", format!("the factor of '{named}' is not a finite number"), &[("case", &named)])); }
        let case = doc.load_cases.iter().find(|case| case.id == named).or_else(|| doc.load_cases.iter().find(|case| case.name.eq_ignore_ascii_case(&named))).ok_or_else(|| refuse("fem3d.combination.terms-case-unknown", format!("'{named}' is not a load case"), &[("case", &named)]))?;
        if terms.insert(case.id.clone(), factor).is_some() { return Err(refuse("fem3d.combination.terms-case-repeated", format!("'{}' appears twice", case.id), &[("case", &case.id)])); }
    }
    Ok(terms)
}

pub fn handle(payload: &AddCombination, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let terms = parse_terms(&payload.terms, doc.snapshot)?;
    let id = doc.operation()?.entity_id("c", "combinations")?;
    Ok(Emit::mutations(vec![Fem3dMutation::CreateCombination(semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::create_combination::CreateCombination { combination: semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemCombination { id, name: payload.name.clone(), terms }, index: None })]))
}
