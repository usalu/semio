//! 🔗️ The text syntax of combination terms, shared by every FEM 2D surface that names load cases with factors.

use semio_framework_value::DslValue;

/// 🔗️ Why a combination's term text was refused: not readable as terms at all, or readable but without any term.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CombinationTermsError {
    Unreadable,
    Empty,
}

fn json_term_pairs(text: &str) -> Option<Vec<(String, f64)>> {
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

fn natural_term_pair(segment: &str) -> Option<(String, f64)> {
    let number = |text: &str| text.trim().parse::<f64>().ok();
    let (left, right) = match segment.split_once([':', '=', '*', '×']) {
        Some((left, right)) => (left.trim(), right.trim()),
        None => {
            let (first, rest) = segment.split_once(char::is_whitespace)?;
            fn single(name: &str) -> Option<&str> { (!name.contains(char::is_whitespace)).then_some(name) }
            if number(first).is_some() { (first, single(rest.trim())?) } else { let (rest, last) = segment.rsplit_once(char::is_whitespace)?; (single(rest.trim())?, last) }
        }
    };
    match (number(left), number(right)) {
        (Some(factor), None) if !right.is_empty() => Some((right.to_owned(), factor)),
        (None, Some(factor)) if !left.is_empty() => Some((left.to_owned(), factor)),
        _ => None,
    }
}

/// 🔤️ Reads `(case, factor)` pairs from user text — `dead:1.35, live:1.5`, `dead=1.35`, `1.35 dead + 1.5 live` — or JSON (`[[case, factor]]`, `[{caseId, factor}]` or `{case: factor}`); segments split by `,` `;` `+` or a line break. Case names are not resolved here.
pub fn parse_combination_terms(text: &str) -> Result<Vec<(String, f64)>, CombinationTermsError> {
    let trimmed = text.trim();
    let pairs = if trimmed.starts_with(['[', '{']) { json_term_pairs(trimmed) } else { trimmed.split([',', ';', '+', '\n']).map(str::trim).filter(|segment| !segment.is_empty()).map(natural_term_pair).collect() }.ok_or(CombinationTermsError::Unreadable)?;
    if pairs.is_empty() { Err(CombinationTermsError::Empty) } else { Ok(pairs) }
}
