//! 🏳️ Check parameters: the inspector section that lists every nationally determined parameter the selected check rests on, with its value under the check's annex, its clause, the verified or unverified badge and, where the document holds a factor override for it, a jump to that override.
//!
//! 🔗️ Registry: [`ndp`](semio_s_plugin_norm_compliance::ndp), [`annex_parameter_rows`](semio_s_plugin_norm_compliance::annex_parameter_rows); selection: [`NormSelection`](crate::NormSelection).

use crate::*;
use semio_framework_value::DslValue;
use semio_s_plugin_norm_compliance::{ndp, AnnexChoice, NdpEntryAnnex, NdpStatus, NdpValue};

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

/// 🏳️ One parameter a check cites, resolved against the registry: `entry` is `None` for an id the registry does not know (reported, never hidden), `override_row` the id of the factor-override row of the document that replaces it.
#[derive(Clone, Debug, PartialEq)]
pub struct CheckParameter {
    pub id: String,
    pub symbol: String,
    pub clause: String,
    pub value: String,
    pub annex: AnnexChoice,
    pub verified: bool,
    pub override_row: Option<String>,
}

/// 🎚️ The factor kind and target an NDP id is overridden by: `…psi.<category>.<n>` by `psi<n>` on that category, a symbol starting with γ by `gamma`, `…xi.…` by `xi`, `…k_fi…` by `k_fi`.
fn override_key(id: &str, symbol: &str) -> Option<(String, Option<String>)> {
    let segments: Vec<&str> = id.split('.').collect();
    if let Some(at) = segments.iter().position(|segment| *segment == "psi") {
        if let [category, n] = segments[at + 1..] {
            return Some((format!("psi{n}"), Some(category.to_owned())));
        }
    }
    match () {
        _ if symbol.starts_with('γ') => Some(("gamma".to_owned(), None)),
        _ if segments.contains(&"xi") => Some(("xi".to_owned(), None)),
        _ if segments.iter().any(|segment| segment.eq_ignore_ascii_case("k_fi")) => Some(("k_fi".to_owned(), None)),
        _ => None,
    }
}

/// 🎚️ The id of the `factorOverrides` row of `root` that replaces the parameter: the last row of the factor whose target is the parameter's category, else the last one with an empty target (every action); `None` when the document has no such row.
pub fn override_row_for(id: &str, symbol: &str, root: &DslValue) -> Option<String> {
    let (factor, target) = override_key(id, symbol)?;
    let DslValue::Array(rows) = root.get("factorOverrides")? else { return None };
    let text = |row: &DslValue, key: &str| match row.get(key) { Some(DslValue::String(text)) => text.trim().to_owned(), _ => String::new() };
    let of_factor = || rows.iter().filter(|row| text(row, "factor") == factor);
    let exact = target.as_deref().and_then(|target| of_factor().filter(|row| text(row, "target") == target).last());
    exact.or_else(|| of_factor().filter(|row| text(row, "target").is_empty()).last()).map(|row| text(row, "id")).filter(|id| !id.is_empty())
}

fn value_text(value: NdpValue, unit: &str, locale: Locale) -> String {
    match value {
        NdpValue::Number(number) if unit.is_empty() => crate::format_number(number, 6, locale),
        NdpValue::Number(number) => format!("{} {unit}", crate::format_number(number, 6, locale)),
        NdpValue::Text(text) => text.to_owned(),
        NdpValue::Unset => "—".to_owned(),
    }
}

/// 🏳️ Every parameter the check cites in citation order, with the value of the check's own annex.
pub fn check_parameters(check: &CheckResult, root: &DslValue, locale: Locale) -> Vec<CheckParameter> {
    check
        .ndps
        .iter()
        .map(|id| match ndp(id) {
            Some(entry) => CheckParameter { id: id.clone(), symbol: entry.symbol.to_owned(), clause: entry.clause.to_owned(), value: value_text(entry.value(check.annex), entry.unit, locale), annex: check.annex, verified: entry.status == NdpStatus::Verified, override_row: override_row_for(id, entry.symbol, root) },
            None => CheckParameter { id: id.clone(), symbol: id.clone(), clause: String::new(), value: "—".to_owned(), annex: check.annex, verified: false, override_row: None },
        })
        .collect()
}

/// 🏳️ The label of a parameter row: symbol, clause and the verified or unverified badge in words, so the badge never rests on colour.
pub fn check_parameter_label(parameter: &CheckParameter, locale: Locale) -> String {
    let badge = if parameter.verified { chrome("verified", "geprüft", locale) } else { chrome("unverified", "ungeprüft", locale) };
    let clause = if parameter.clause.is_empty() { String::new() } else { format!(" · {}", semio_s_plugin_norm_compliance::designator(&parameter.clause, &locale)) };
    format!("{}{clause} [{badge}]", parameter.symbol)
}

/// 🏳️ The inspector section "Check parameters" of the selected check; `None` when it cites no parameter. A parameter row shows its id, value and annex; with a factor override in the document the row activates `setSelectedCheckIndex` for the governing check of that override row, which selects the override in the outliner, the properties and the viewport.
pub fn check_parameters_section(report: &CheckReport, index: usize, root: &DslValue, locale: Locale, controller_id: &'static str) -> UiAssemblyResult<Option<BuiltNode>> {
    let Some(check) = report.checks.get(index) else { return Ok(None) };
    let parameters = check_parameters(check, root, locale);
    if parameters.is_empty() {
        return Ok(None);
    }
    let mut rows = UiFixedList::default();
    for parameter in &parameters {
        let id = format!("norm-inputs.check-parameters.{}", parameter.id);
        let description = format!("{} · {} {}: {}", parameter.id, chrome("Annex", "Anhang", locale), parameter.annex.label(), parameter.value);
        let target = parameter.override_row.as_deref().and_then(|row| crate::selection_mirror::governing(report, row));
        let label = norm_ui_label(check_parameter_label(parameter, locale))?;
        let row = match target {
            Some(target) => {
                let args = action_args_map(vec![("index", UiValue::Number(target as f64))])?;
                tree_item_with_action(id, label, Some(format!("{description} → {}", parameter.override_row.as_deref().unwrap_or_default())), norm_action(controller_id, "setSelectedCheckIndex", Some(args))?)?
            }
            None => tree_item_desc(id, label, Some(description))?,
        };
        rows.try_push(row).map_err(|_| ui_error("ui.fixed-capacity"))?;
    }
    tree_group("norm-inputs.check-parameters".to_owned(), norm_ui_label(format!("{} ({})", chrome("Check parameters", "Nachweisparameter", locale), parameters.len()))?, true, rows).map(Some)
}
