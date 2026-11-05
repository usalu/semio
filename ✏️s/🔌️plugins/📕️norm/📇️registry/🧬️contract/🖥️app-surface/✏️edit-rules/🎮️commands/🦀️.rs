//! 🎮️ Norm edit commands, remedies and atomic typed mutation staging.

use super::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub(crate) mod tests;

//#region 🔖️Commands
fn unresolved(error: String) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("norm.edit-unresolved"), error)
}

/// ✏️ Generic `setField` — `{path, value}` over the document value tree, resolved by the family to concrete kinds.
///
/// 📐️ **SI convention:** snapshot scalar quantity fields MUST store SI (m, N, Pa, …). `applyRemedy` writes
/// `Remedy.required.value` (already SI) straight into `remedy.target.path`. Family agents must not store mm/kN/MPa
/// in snapshot fields — display units are UI-only via [`format_quantity`] / [`FieldSpec::unit`].
pub fn handle_set_field<D, M, F>(document: &D, path: &str, value: semio_framework_value::DslValue, resolve: F) -> Result<Emit<M, NoConfigMutation>, Fault>
where
    F: FnOnce(&D, &NormEdit) -> Result<Vec<M>, String>,
{
    commit_snapshot_fields(resolve(document, &NormEdit::SetField { path: path.to_string(), value }).map_err(unresolved)?)
}

/// ➕ Generic `insertItem` — `{path, index, value?}`.
pub fn handle_insert_item<D, M, F>(document: &D, path: &str, index: usize, value: Option<semio_framework_value::DslValue>, resolve: F) -> Result<Emit<M, NoConfigMutation>, Fault>
where
    F: FnOnce(&D, &NormEdit) -> Result<Vec<M>, String>,
{
    commit_snapshot_fields(resolve(document, &NormEdit::InsertItem { path: path.to_string(), index, value }).map_err(unresolved)?)
}

/// ➖ Generic `removeItem` — `{path, index}`.
pub fn handle_remove_item<D, M, F>(document: &D, path: &str, index: usize, resolve: F) -> Result<Emit<M, NoConfigMutation>, Fault>
where
    F: FnOnce(&D, &NormEdit) -> Result<Vec<M>, String>,
{
    commit_snapshot_fields(resolve(document, &NormEdit::RemoveItem { path: path.to_string(), index }).map_err(unresolved)?)
}

/// 🩹 What a remedy does to the document: write a value at a leaf path, remove the row its target path selects, or insert a typed row at the end of the list its target path names.
#[derive(Clone, Debug, PartialEq)]
pub enum RemedyEdit {
    Set { path: String, value: semio_framework_value::DslValue },
    Remove { path: String },
    Insert { path: String, index: usize, value: semio_framework_value::DslValue },
}

/// 🩹 Resolves a remedy on `report` into the path it writes and the value it writes there (numeric SI or OneOf option string), reading
/// the current leaf type from `tree` without touching it; a row-removing remedy is no value write and fails here (see [`remedy_resolution`]).
pub fn remedy_edit(report: &CheckReport, check_id: &str, remedy_index: usize, option_index: usize, tree: &semio_framework_value::DslValue) -> Result<(String, semio_framework_value::DslValue), Fault> {
    match remedy_resolution(report, check_id, remedy_index, option_index, tree)? {
        RemedyEdit::Set { path, value } => Ok((path, value)),
        RemedyEdit::Remove { path } => Err(Fault::new(FaultOrigin::App, FaultCode::new("norm.remedy-removes-row"), format!("remedy {remedy_index} on '{check_id}' removes the row '{path}' and writes no value"))),
        RemedyEdit::Insert { path, .. } => Err(Fault::new(FaultOrigin::App, FaultCode::new("norm.remedy-inserts-row"), format!("remedy {remedy_index} on '{check_id}' inserts a row into '{path}' and writes no value"))),
    }
}

/// ➕ The typed row of an [`RemedyBound::Insert`] remedy: its first option parsed as a JSON object, every string value `"$required"` replaced by the remedy's computed `required` value, the row id checked against the list so a second application never duplicates it.
fn remedy_row(remedy: &Remedy, rows: &[semio_framework_value::DslValue]) -> Result<semio_framework_value::DslValue, String> {
    fn fill(value: &mut semio_framework_value::DslValue, required: f64) {
        match value {
            semio_framework_value::DslValue::String(text) if text == "$required" => *value = semio_framework_value::DslValue::float(required),
            semio_framework_value::DslValue::Array(items) => items.iter_mut().for_each(|item| fill(item, required)),
            semio_framework_value::DslValue::Object(entries) => entries.iter_mut().for_each(|(_, item)| fill(item, required)),
            _ => {}
        }
    }
    let json = remedy.options.first().ok_or("an insert remedy carries its row as the first option")?;
    let mut row = semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    if !matches!(row, semio_framework_value::DslValue::Object(_)) {
        return Err("an insert remedy's row must be a JSON object".into());
    }
    fill(&mut row, remedy.required.value);
    match object_id_field(&row) {
        Some(id) if rows.iter().any(|existing| object_id_field(existing) == Some(id)) => Err(format!("a row '{id}' already exists")),
        _ => Ok(row),
    }
}

/// 🩹 Resolves a remedy into its [`RemedyEdit`]: [`RemedyBound::Remove`] selects the row of the target path (it must exist), [`RemedyBound::Insert`] appends its typed row to the list of the target path, every other bound writes a value like [`remedy_edit`].
pub fn remedy_resolution(report: &CheckReport, check_id: &str, remedy_index: usize, option_index: usize, tree: &semio_framework_value::DslValue) -> Result<RemedyEdit, Fault> {
    let check = report.checks.iter().find(|check| check.id == check_id).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-missing-check"), format!("check '{check_id}' not in report")))?;
    let remedy = check.remedies.get(remedy_index).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-missing-remedy"), format!("remedy {remedy_index} missing on '{check_id}'")))?;
    if !remedy.applicable {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-not-applicable"), format!("remedy {remedy_index} on '{check_id}' is not applicable")));
    }
    let path = remedy.target.path.clone();
    if path.is_empty() {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-empty-path"), format!("remedy {remedy_index} on '{check_id}' has empty target path")));
    }
    let current = get_value_at_path(tree, &path).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("norm.value-path"), error))?;
    let value = match remedy.bound {
        RemedyBound::Remove => {
            if !matches!(parse_path(&path).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("norm.value-path"), error))?.last(), Some(PathSegment::Index(_) | PathSegment::Id(_))) {
                return Err(Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-remove-path"), format!("remove remedy {remedy_index} on '{check_id}' must target a list row, not '{path}'")));
            }
            return Ok(RemedyEdit::Remove { path });
        }
        RemedyBound::Insert => {
            let rows = current.as_array().ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-insert-path"), format!("insert remedy {remedy_index} on '{check_id}' must target a list, not '{path}'")))?;
            let value = remedy_row(remedy, rows).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-insert-row"), format!("insert remedy {remedy_index} on '{check_id}': {error}")))?;
            return Ok(RemedyEdit::Insert { path, index: rows.len(), value });
        }
        RemedyBound::OneOf => {
            let option = remedy
                .options
                .get(option_index)
                .ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-missing-option"), format!("option {option_index} missing on OneOf remedy {remedy_index} for '{check_id}'")))?;
            match current {
                semio_framework_value::DslValue::Bool(_) => semio_framework_value::DslValue::Bool(option.eq_ignore_ascii_case("true") || option == "1"),
                semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(_)) => {
                    let parsed = option.parse::<u64>().map_err(|_| Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-option-type"), format!("OneOf option '{option}' is not a u64 for '{path}'")))?;
                    semio_framework_value::DslValue::uint(parsed)
                }
                semio_framework_value::DslValue::Number(semio_framework_value::Number::Int(_)) => {
                    let parsed = option.parse::<i64>().map_err(|_| Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-option-type"), format!("OneOf option '{option}' is not an i64 for '{path}'")))?;
                    semio_framework_value::DslValue::int(parsed)
                }
                semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(_)) => {
                    let parsed = option.parse::<f64>().map_err(|_| Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-option-type"), format!("OneOf option '{option}' is not a float for '{path}'")))?;
                    semio_framework_value::DslValue::float(parsed)
                }
                _ => semio_framework_value::DslValue::String(option.clone()),
            }
        }
        RemedyBound::AtLeast | RemedyBound::AtMost | RemedyBound::Exactly => match current {
            semio_framework_value::DslValue::Bool(_) => semio_framework_value::DslValue::Bool(remedy.required.value >= 0.5),
            semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(_)) => {
                if !remedy.required.value.is_finite() || remedy.required.value < 0.0 {
                    return Err(Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-uint"), format!("required {} is not a u64 for '{path}'", remedy.required.value)));
                }
                semio_framework_value::DslValue::uint(remedy.required.value.round() as u64)
            }
            semio_framework_value::DslValue::Number(semio_framework_value::Number::Int(_)) => {
                if !remedy.required.value.is_finite() {
                    return Err(Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-int"), format!("required {} is not an i64 for '{path}'", remedy.required.value)));
                }
                semio_framework_value::DslValue::int(remedy.required.value.round() as i64)
            }
            semio_framework_value::DslValue::String(_) => semio_framework_value::DslValue::String(format!("{}", remedy.required.value)),
            _ => semio_framework_value::DslValue::float(remedy.required.value),
        },
    };
    Ok(RemedyEdit::Set { path, value })
}

/// 🩹 Writes a resolved remedy into a value tree — the remedy laws' perturbation helper; the mutation path resolves the same edit
/// to a concrete kind instead.
pub fn apply_remedy_edit(report: &CheckReport, check_id: &str, remedy_index: usize, option_index: usize, tree: &mut semio_framework_value::DslValue) -> Result<(), Fault> {
    let outcome = match remedy_resolution(report, check_id, remedy_index, option_index, tree)? {
        RemedyEdit::Set { path, value } => set_value_at_path(tree, &path, value),
        RemedyEdit::Remove { path } => remove_value_at_path(tree, &path, 0),
        RemedyEdit::Insert { path, index, value } => insert_value_at_path(tree, &path, index, Some(value)),
    };
    outcome.map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("norm.value-path"), error))
}

/// 🩹 Generic `applyRemedy` — re-evaluates, finds `checkId`, writes SI `required` (or OneOf option) into `remedy.target.path`.
pub fn handle_apply_remedy<F, Map>(document: &F::Document, check_id: &str, remedy_index: usize, resolve: Map) -> Result<Emit<F::Mutation, NoConfigMutation>, Fault>
where
    F: NormFamily,
    F::Document: Clone + semio_framework_value::ToValue,
    Map: FnOnce(&F::Document, &NormEdit) -> Result<Vec<F::Mutation>, String>,
{
    handle_apply_remedy_with_option::<F, Map>(document, check_id, remedy_index, 0, resolve)
}

/// 🩹 `applyRemedy` with an explicit OneOf `option_index` (ignored for numeric bounds).
pub fn handle_apply_remedy_with_option<F, Map>(document: &F::Document, check_id: &str, remedy_index: usize, option_index: usize, resolve: Map) -> Result<Emit<F::Mutation, NoConfigMutation>, Fault>
where
    F: NormFamily,
    F::Document: Clone + semio_framework_value::ToValue,
    Map: FnOnce(&F::Document, &NormEdit) -> Result<Vec<F::Mutation>, String>,
{
    let report = F::live_report(document);
    let tree = semio_framework_value::ToValue::to_value(document);
    let edit = match remedy_resolution(&report, check_id, remedy_index, option_index, &tree)? {
        RemedyEdit::Set { path, value } => NormEdit::SetField { path, value },
        RemedyEdit::Remove { path } => NormEdit::RemoveItem { path, index: 0 },
        RemedyEdit::Insert { path, index, value } => NormEdit::InsertItem { path, index, value: Some(value) },
    };
    commit_snapshot_fields(resolve(document, &edit).map_err(unresolved)?)
}
//#endregion 🔖️Commands

//#region 🫧️LiveEdits
/// 🧱️ Validates a write bundle: non-empty, within [`NORM_LIVE_MAX_WRITES`], no path twice, every number finite.
pub fn validate_writes(writes: &[FieldWrite]) -> Result<(), String> {
    if writes.is_empty() {
        return Err("setFields needs at least one write".into());
    }
    if writes.len() > NORM_LIVE_MAX_WRITES {
        return Err(format!("setFields carries {} writes, at most {NORM_LIVE_MAX_WRITES} are allowed", writes.len()));
    }
    for (at, write) in writes.iter().enumerate() {
        if writes[..at].iter().any(|earlier| earlier.path == write.path) {
            return Err(format!("path '{}' is written twice", write.path));
        }
        if matches!(&write.value, semio_framework_value::DslValue::Number(number) if !number.as_f64().is_finite()) {
            return Err(format!("write '{}' is not finite", write.path));
        }
    }
    Ok(())
}

/// 🔢️ Carries a numeric write over to the leaf's own number kind (counts stay integers); other kinds pass through.
pub fn coerce_like(current: &semio_framework_value::DslValue, value: &semio_framework_value::DslValue) -> semio_framework_value::DslValue {
    use semio_framework_value::{DslValue, Number};
    match (current, value) {
        (DslValue::Number(Number::UInt(_)), DslValue::Number(number)) if number.as_f64() >= 0.0 => DslValue::uint(number.as_f64().round() as u64),
        (DslValue::Number(Number::Int(_)), DslValue::Number(number)) => DslValue::int(number.as_f64().round() as i64),
        (DslValue::Number(Number::Float(_)), DslValue::Number(number)) => DslValue::float(number.as_f64()),
        _ => value.clone(),
    }
}

/// 🧪️ Resolves ordered intents against one privately owned typed projection, admitting every operation before exposing the bundle.
pub fn resolve_norm_edits<D, M, F>(document: &D, edits: &[NormEdit], mut resolve: F) -> Result<Vec<M>, String>
where
    D: Clone + semio_framework_value::ToValue,
    M: protocol::Mutation<D> + protocol::ColdMutation<D> + semio_framework_value::retirement::RetireOwned,
    F: FnMut(&D, &NormEdit) -> Result<Vec<M>, String>,
{
    if edits.len() > NORM_LIVE_MAX_WRITES { return Err(format!("The edit plan exceeds {NORM_LIVE_MAX_WRITES} intents")) }
    if edits.is_empty() { return Ok(Vec::new()) }
    let mut scratch = store::os_store::ledger::ColdReplayProjection::<D, M>::new(document.clone());
    let mut mutations: Vec<M> = Vec::new();
    let result = (|| {
        for edit in edits {
            let first = mutations.len();
            mutations.extend(resolve(&*scratch, edit)?);
            for index in first..mutations.len() {
                let operation_index = u32::try_from(index).map_err(|_| "The field bundle has too many mutations".to_owned())?;
                let (admitted, messages) = store::os_store::replay::fold_operation_into(scratch.projection_mut(), &mutations[index], operation_index);
                let refusal = (!admitted).then(|| messages.iter().map(|message| format!("{}: {}", message.code.0, message.message)).collect::<Vec<_>>().join("; "));
                drop(messages);
                if let Some(refusal) = refusal { return Err(format!("operation {operation_index} refused: {refusal}")) }
            }
        }
        Ok(())
    })();
    if let Err(error) = result {
        for mutation in mutations { protocol::Mutation::<D>::retire_cold(mutation); }
        return Err(error);
    }
    Ok(mutations)
}

/// 🎚️ Adapts numeric UI writes to each current typed leaf before exact ordered admission.
pub(crate) fn resolve_field_edits<D, M, F>(document: &D, edits: &[NormEdit], mut resolve: F) -> Result<Vec<M>, String>
where
    D: Clone + semio_framework_value::ToValue,
    M: protocol::Mutation<D> + protocol::ColdMutation<D> + semio_framework_value::retirement::RetireOwned,
    F: FnMut(&D, &NormEdit) -> Result<Vec<M>, String>,
{
    resolve_norm_edits(document, edits, |current, edit| match edit {
        NormEdit::SetField { path, value } => {
            let tree = semio_framework_value::ToValue::to_value(current);
            let value = get_value_at_path(&tree, path).map_or_else(|_| value.clone(), |stored| coerce_like(stored, value));
            resolve(current, &NormEdit::SetField { path: path.clone(), value })
        }
        _ => resolve(current, edit),
    })
}

/// 📦️ Resolves a validated write bundle in authored order; every later typed parent includes earlier admitted child writes.
pub fn resolve_set_fields<D, M, F>(document: &D, writes: &[FieldWrite], resolve: F) -> Result<Vec<M>, String>
where
    D: Clone + semio_framework_value::ToValue,
    M: protocol::Mutation<D> + protocol::ColdMutation<D> + semio_framework_value::retirement::RetireOwned,
    F: FnMut(&D, &NormEdit) -> Result<Vec<M>, String>,
{
    validate_writes(writes)?;
    let edits: Vec<_> = writes.iter().filter(|write| !is_view_write(write)).map(|write| NormEdit::SetField { path: write.path.clone(), value: write.value.clone() }).collect();
    resolve_field_edits(document, &edits, resolve)
}

/// 📦️ Generic `setFields` — `{writes: [{path, value}]}` becomes ONE emit of all leaf mutations, i.e. one undo step for a drag release.
pub fn handle_set_fields<D, M, F>(document: &D, writes: &[FieldWrite], resolve: F) -> Result<Emit<M, NoConfigMutation>, Fault>
where
    D: Clone + semio_framework_value::ToValue,
    M: protocol::Mutation<D> + protocol::ColdMutation<D> + semio_framework_value::retirement::RetireOwned,
    F: FnMut(&D, &NormEdit) -> Result<Vec<M>, String>,
{
    commit_snapshot_fields(resolve_set_fields(document, writes, resolve).map_err(unresolved)?)
}

/// 👻️ `previewRemedy` — the ghost a remedy would write, resolved exactly like `applyRemedy` but kept in the transient lane.
pub fn preview_remedy(report: &CheckReport, tree: &semio_framework_value::DslValue, check_id: &str, remedy_index: usize, option_index: usize) -> Result<GhostPreview, Fault> {
    let (path, value) = remedy_edit(report, check_id, remedy_index, option_index, tree)?;
    Ok(GhostPreview { writes: vec![FieldWrite { path, value }], source: GhostSource::Remedy { check_id: check_id.to_string(), index: remedy_index as u32 } })
}
//#endregion 🫧️LiveEdits

//#region 🔖️RemedyLaw
fn remedy_severity(status: &CheckStatus) -> u8 {
    match status {
        CheckStatus::Fail => 2,
        CheckStatus::Warning => 1,
        _ => 0,
    }
}

/// ⚖️ The remedy law over one document: every remedy of every check of its report must (1) carry EN and DE text, (2) be an advisory (`applicable: false`, no options) or name an existing field, numeric for `AtLeast` / `AtMost` / `Exactly` and with at least one option for `OneOf`, (3) apply without a fault and yield a document that decodes (so an enum token, a count and a text field stay valid), (4) write a finite value inside the field's hard range (`limits(path)`, the field meta range; the schema types are enforced by the decode) and (5) lower the check's utilisation or its severity when the report is evaluated again (a `Remove` and the fix of an invalid-input check, id with `.input.`, may drop the check). Returns one finding per violation, empty when the law holds.
pub fn remedy_law_findings<S>(label: &str, document: &S, evaluate: impl Fn(&S) -> CheckReport, limits: impl Fn(&str) -> Option<(f64, f64)>) -> Vec<String>
where
    S: semio_framework_value::ToValue + semio_framework_value::FromValue,
{
    let mut findings = Vec::new();
    let report = evaluate(document);
    let tree = semio_framework_value::ToValue::to_value(document);
    for check in &report.checks {
        for (index, remedy) in check.remedies.iter().enumerate() {
            let at = format!("{label}: {} remedy {index} on '{}'", check.id, remedy.target.path);
            if remedy.action.en.trim().is_empty() || remedy.action.de.trim().is_empty() {
                findings.push(format!("{at}: the action text lacks EN or DE"));
            }
            let advisory = remedy.bound == RemedyBound::OneOf && remedy.options.is_empty();
            if !remedy.applicable {
                if !advisory {
                    findings.push(format!("{at}: a remedy that is not applicable must be an advisory (OneOf without options)"));
                }
                continue;
            }
            if advisory {
                findings.push(format!("{at}: an applicable OneOf remedy has no options (apply fails with 'option 0 missing')"));
                continue;
            }
            match get_value_at_path(&tree, &remedy.target.path) {
                Err(error) => {
                    findings.push(format!("{at}: the target does not exist: {error}"));
                    continue;
                }
                Ok(current) => {
                    let numeric = matches!(current, semio_framework_value::DslValue::Number(_) | semio_framework_value::DslValue::Bool(_));
                    if matches!(remedy.bound, RemedyBound::AtLeast | RemedyBound::AtMost | RemedyBound::Exactly) && !(numeric && remedy.required.value.is_finite()) {
                        findings.push(format!("{at}: a numeric remedy ({}) targets a non-numeric field or carries a non-finite value", remedy.required.value));
                        continue;
                    }
                }
            }
            let options = if remedy.bound == RemedyBound::OneOf { remedy.options.len() } else { 1 };
            for option in 0..options {
                let mut changed = tree.clone();
                if let Err(fault) = apply_remedy_edit(&report, &check.id, index, option, &mut changed) {
                    findings.push(format!("{at}: applying option {option} fails: {fault:?}"));
                    continue;
                }
                if let (Some((min, max)), Ok(semio_framework_value::DslValue::Number(written))) = (limits(&remedy.target.path), get_value_at_path(&changed, &remedy.target.path)) {
                    let value = written.as_f64();
                    if !(value >= min && value <= max) {
                        findings.push(format!("{at}: the written value {value} lies outside the field range {min} to {max}"));
                    }
                }
                let fixed = match <S as semio_framework_value::FromValue>::from_value(changed) {
                    Ok(fixed) => fixed,
                    Err(error) => {
                        findings.push(format!("{at}: the document is invalid after applying option {option}: {error:?}"));
                        continue;
                    }
                };
                match evaluate(&fixed).checks.iter().find(|updated| updated.id == check.id) {
                    None if remedy.bound == RemedyBound::Remove || check.id.contains(".input.") => {}
                    None => findings.push(format!("{at}: the check vanished after applying option {option}")),
                    Some(updated) => {
                        let lowered = remedy_severity(&updated.status) < remedy_severity(&check.status) || updated.utilization < check.utilization;
                        if !lowered {
                            findings.push(format!("{at}: option {option} does not help: {:?} u {:.4} -> {:?} u {:.4}", check.status, check.utilization, updated.status, updated.utilization));
                        }
                    }
                }
            }
        }
    }
    findings
}
//#endregion 🔖️RemedyLaw
