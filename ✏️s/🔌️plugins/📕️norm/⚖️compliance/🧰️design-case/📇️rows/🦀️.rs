//! 📇️ Case rows: the persisted design case every artifact snapshot embeds as `cases`, its positional delta and the pure diff and inverse logic of the four shared mutation kinds `add-case`, `change-case-number`, `change-case-text` and `remove-case`. An artifact owns only the four thin mutation leaves (payload, label, descriptor) that call these functions. See [`super`] and `../🧬️schema/🔣️.json`.
use super::*;

//#region 🔖️Rows
/// 🔢️ One numeric input of a case, in the stored unit of its vocabulary row.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "compliance-testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CaseNumber {
    pub id: String,
    pub value: f64,
}
/// 🔤 One text or choice input of a case.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "compliance-testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CaseText {
    pub id: String,
    pub value: String,
}
/// 📇️ One design case of a snapshot: identity, bilingual label, the catalogue entry it verifies and its input rows.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "compliance-testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CaseRow {
    pub id: String,
    pub label_en: String,
    pub label_de: String,
    pub entry_id: String,
    #[dsl(table)]
    pub numbers: Vec<CaseNumber>,
    #[dsl(table)]
    pub texts: Vec<CaseText>,
}
impl From<&CaseRow> for DesignCase {
    fn from(row: &CaseRow) -> Self {
        Self {
            id: row.id.clone(),
            label_en: row.label_en.clone(),
            label_de: row.label_de.clone(),
            entry_id: row.entry_id.clone(),
            values: CaseValues { numbers: row.numbers.iter().map(|n| (n.id.clone(), n.value)).collect(), texts: row.texts.iter().map(|t| (t.id.clone(), t.value.clone())).collect() },
        }
    }
}
impl From<&DesignCase> for CaseRow {
    fn from(case: &DesignCase) -> Self {
        Self {
            id: case.id.clone(),
            label_en: case.label_en.clone(),
            label_de: case.label_de.clone(),
            entry_id: case.entry_id.clone(),
            numbers: case.values.numbers.iter().map(|(id, value)| CaseNumber { id: id.clone(), value: *value }).collect(),
            texts: case.values.texts.iter().map(|(id, value)| CaseText { id: id.clone(), value: value.clone() }).collect(),
        }
    }
}
//#endregion 🔖️Rows

//#region 🔖️Delta
protocol::row_patch! {
    #[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "compliance-testing"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one [`CaseNumber`].
    pub CaseNumberPatch of CaseNumber { set { value: f64 } }
}
protocol::list_delta! {
    #[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "compliance-testing"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one [`CaseNumber`] list.
    pub CaseNumberDelta { removal: CaseNumberRemoval, insertion: CaseNumberInsertion, relocation: CaseNumberRelocation, modification: CaseNumberModification, row: CaseNumber, patch: CaseNumberPatch, key: id }
}
protocol::row_patch! {
    #[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "compliance-testing"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one [`CaseText`].
    pub CaseTextPatch of CaseText { set { value: String } }
}
protocol::list_delta! {
    #[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "compliance-testing"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one [`CaseText`] list.
    pub CaseTextDelta { removal: CaseTextRemoval, insertion: CaseTextInsertion, relocation: CaseTextRelocation, modification: CaseTextModification, row: CaseText, patch: CaseTextPatch, key: id }
}
protocol::row_patch! {
    #[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "compliance-testing"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one [`CaseRow`].
    pub CaseRowPatch of CaseRow { set { label_en: String, label_de: String, entry_id: String } nest { numbers: CaseNumberDelta, texts: CaseTextDelta } }
}
protocol::list_delta! {
    #[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "compliance-testing"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `cases` list; the lane every artifact diff embeds.
    pub CaseRowDelta { removal: CaseRowRemoval, insertion: CaseRowInsertion, relocation: CaseRowRelocation, modification: CaseRowModification, row: CaseRow, patch: CaseRowPatch, key: id }
}
//#endregion 🔖️Delta

//#region 🔖️Logic
/// 🚫 Why one of the four case mutations is refused; the artifact leaf maps it to `MutationOutcome::fatal(code, message, targets)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CaseRefusal {
    DuplicateCase(String),
    UnknownCase(String),
    UnknownEntry(String),
}
impl CaseRefusal {
    pub fn code(&self) -> &'static str {
        match self {
            Self::DuplicateCase(_) => "mutation.duplicate-id",
            _ => "mutation.invariant",
        }
    }
    pub fn message(&self) -> String {
        match self {
            Self::DuplicateCase(id) => format!("Case id {id} already exists."),
            Self::UnknownCase(id) => format!("Unknown case id {id}."),
            Self::UnknownEntry(id) => format!("Unknown catalogue entry {id}."),
        }
    }
    pub fn targets(&self) -> Vec<String> {
        match self {
            Self::DuplicateCase(id) | Self::UnknownCase(id) | Self::UnknownEntry(id) => vec![id.clone()],
        }
    }
}
fn row_of<'a>(rows: &'a [CaseRow], id: &str) -> Result<&'a CaseRow, CaseRefusal> { rows.iter().find(|row| row.id == id).ok_or_else(|| CaseRefusal::UnknownCase(id.into())) }
/// ➕️ `add-case`: inserts `case` at `index` (clamped); the id must be new and the entry known to the catalogue `C`.
pub fn add_case<C: CaseCatalogue>(rows: &Vec<CaseRow>, index: usize, case: &CaseRow) -> Result<CaseRowDelta, CaseRefusal> {
    if rows.iter().any(|row| row.id == case.id) {
        return Err(CaseRefusal::DuplicateCase(case.id.clone()));
    }
    if entry::<C>(&case.entry_id).is_none() {
        return Err(CaseRefusal::UnknownEntry(case.entry_id.clone()));
    }
    Ok(CaseRowDelta::insertion(index.min(rows.len()), case.clone()))
}
/// ➖️ `remove-case`: removes the case with the id `id`.
pub fn remove_case(rows: &Vec<CaseRow>, id: &str) -> Result<CaseRowDelta, CaseRefusal> {
    let index = rows.iter().position(|row| row.id == id).ok_or_else(|| CaseRefusal::UnknownCase(id.into()))?;
    Ok(CaseRowDelta::removal(rows, index))
}
/// 🔢️ `change-case-number`: sets the number `key` of the case, adds the row when absent, removes it for `None`; setting an absent row to `None` is an empty delta.
pub fn change_case_number(rows: &Vec<CaseRow>, case_id: &str, key: &str, value: Option<f64>) -> Result<CaseRowDelta, CaseRefusal> {
    let case = row_of(rows, case_id)?;
    let existing = case.numbers.iter().position(|number| number.id == key);
    let numbers = match (existing, value) {
        (Some(_), Some(value)) => CaseNumberDelta::modification(key, CaseNumberPatch { value: Some(value) }),
        (Some(index), None) => CaseNumberDelta::removal(&case.numbers, index),
        (None, Some(value)) => CaseNumberDelta::insertion(case.numbers.len(), CaseNumber { id: key.into(), value }),
        (None, None) => return Ok(CaseRowDelta::default()),
    };
    Ok(CaseRowDelta::modification(case_id, CaseRowPatch { numbers, ..Default::default() }))
}
/// 🔤 `change-case-text`: the text counterpart of [`change_case_number`].
pub fn change_case_text(rows: &Vec<CaseRow>, case_id: &str, key: &str, value: Option<&str>) -> Result<CaseRowDelta, CaseRefusal> {
    let case = row_of(rows, case_id)?;
    let existing = case.texts.iter().position(|text| text.id == key);
    let texts = match (existing, value) {
        (Some(_), Some(value)) => CaseTextDelta::modification(key, CaseTextPatch { value: Some(value.into()) }),
        (Some(index), None) => CaseTextDelta::removal(&case.texts, index),
        (None, Some(value)) => CaseTextDelta::insertion(case.texts.len(), CaseText { id: key.into(), value: value.into() }),
        (None, None) => return Ok(CaseRowDelta::default()),
    };
    Ok(CaseRowDelta::modification(case_id, CaseRowPatch { texts, ..Default::default() }))
}
/// ↩️ The previous value of the number `key`, the value the inverse of `change-case-number` restores (`None` when the row was absent).
pub fn previous_number(rows: &[CaseRow], case_id: &str, key: &str) -> Option<f64> { rows.iter().find(|row| row.id == case_id)?.numbers.iter().find(|number| number.id == key).map(|number| number.value) }
/// ↩️ The previous value of the text `key`.
pub fn previous_text(rows: &[CaseRow], case_id: &str, key: &str) -> Option<String> { rows.iter().find(|row| row.id == case_id)?.texts.iter().find(|text| text.id == key).map(|text| text.value.clone()) }
//#endregion 🔖️Logic
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
