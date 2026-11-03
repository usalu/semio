//! 🩹️ Remedy chips of the checks window: one actionable row per remedy or per option of a choice remedy. See [`super`].

use super::*;

/// 👻️ The variants whose editors declare the live preview commands.
pub const LIVE_VARIANTS: [&str; 10] = ["en1990", "en1991", "en1992", "en1993", "en1994", "en1995", "en1996", "en1997", "en1998", "en1999"];

/// 👻️ True when `controller_id` (`s.norm.<variant>@1/*#editor`) belongs to a live editor that dispatches `previewRemedy` and `clearPreview`.
pub fn controller_is_live(controller_id: &str) -> bool {
    LIVE_VARIANTS.iter().any(|variant| controller_id.contains(&format!(".{variant}@")))
}

/// 👻️ The row that shows a remedy (or one option of a choice remedy) as a ghost in the viewport without touching the document.
pub fn preview_row(id: String, check_id: &str, remedy_index: usize, option_index: usize, label: &str, controller_id: &'static str, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let args = action_args_map(vec![("checkId", ui_value_text(check_id)?), ("remedyIndex", UiValue::Number(remedy_index as f64)), ("optionIndex", UiValue::Number(option_index as f64))])?;
    tree_item_with_action(id, norm_ui_label(format!("👻 {} — {label}", chrome("Preview", "Vorschau", locale)))?, None, norm_action(controller_id, "previewRemedy", Some(args))?)
}

/// 👻️ The row that discards the ghost preview.
pub fn clear_preview_row(id: String, controller_id: &'static str, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    tree_item_with_action(id, norm_ui_label(format!("👻 {}", chrome("Clear preview", "Vorschau verwerfen", locale)))?, None, norm_action(controller_id, "clearPreview", None)?)
}

/// 🩹️ Relation symbol of a remedy's required value.
pub fn remedy_bound_symbol(bound: RemedyBound) -> &'static str {
    match bound {
        RemedyBound::AtLeast => "≥",
        RemedyBound::AtMost => "≤",
        RemedyBound::Exactly => "=",
        RemedyBound::OneOf => "∈",
        RemedyBound::Remove => "✕",
        RemedyBound::Insert => "＋",
    }
}

/// 🩹️ The change a chip promises, `12.0 mm → ≥ 22.0 mm`.
pub fn remedy_change_text(remedy: &Remedy, locale: Locale) -> String {
    if matches!(remedy.bound, RemedyBound::Remove | RemedyBound::Insert) {
        return remedy.action.resolve(&protocol_locale(locale)).to_owned();
    }
    format!("{} → {} {}", quantity_text(&remedy.current, locale), remedy_bound_symbol(remedy.bound), quantity_text(&remedy.required, locale))
}

/// 🩹️ Chip rows of one check: choice remedies yield one `setField` row per option, applicable remedies an `applyRemedy` row, the rest a read-only row.
pub fn remedy_chip_rows(id: &str, check: &CheckResult, locale: Locale, controller_id: Option<&'static str>) -> UiAssemblyResult<Vec<BuiltNode>> {
    let protocol = protocol_locale(locale);
    let mut rows = Vec::new();
    for (remedy_index, remedy) in check.remedies.iter().enumerate() {
        let action = remedy.action.resolve(&protocol);
        let chip_id = format!("{id}.remedy-{remedy_index}");
        if matches!(remedy.bound, RemedyBound::OneOf) {
            if remedy.options.is_empty() {
                rows.push(tree_item_desc(&chip_id, norm_ui_label(format!("🩹 {action}"))?, Some(chrome("No options", "Keine Optionen", locale)))?);
            }
            for (option_index, option) in remedy.options.iter().enumerate() {
                let label = norm_ui_label(format!("🩹 {} — {option}", chrome("Choose", "Wählen", locale)))?;
                let description = Some(format!("{action} · {}", remedy.target.path));
                rows.push(match controller_id {
                    Some(controller_id) => {
                        let args = action_args_map(vec![("path", ui_value_text(&remedy.target.path)?), ("value", ui_value_text(option)?)])?;
                        tree_item_with_action(format!("{chip_id}.option-{option_index}"), label, description, norm_action(controller_id, "setField", Some(args))?)?
                    }
                    None => tree_item_desc(format!("{chip_id}.option-{option_index}"), label, description)?,
                });
                if let Some(controller_id) = controller_id.filter(|controller_id| controller_is_live(controller_id)) {
                    rows.push(preview_row(format!("{chip_id}.option-{option_index}.preview"), &check.id, remedy_index, option_index, option, controller_id, locale)?);
                }
            }
            continue;
        }
        let description = Some(if remedy.applicable { remedy_change_text(remedy, locale) } else { format!("{} · {}", remedy_change_text(remedy, locale), chrome("Not auto-applicable", "Nicht automatisch anwendbar", locale)) });
        let applicable_controller = controller_id.filter(|_| remedy.applicable);
        rows.push(match applicable_controller {
            Some(controller_id) => {
                let args = action_args_map(vec![("checkId", ui_value_text(&check.id)?), ("remedyIndex", UiValue::Number(remedy_index as f64))])?;
                tree_item_with_action(&chip_id, norm_ui_label(format!("🩹 {} — {action}", chrome("Apply", "Anwenden", locale)))?, description, norm_action(controller_id, "applyRemedy", Some(args))?)?
            }
            None => tree_item_desc(chip_id.clone(), norm_ui_label(format!("🩹 {action}"))?, description)?,
        });
        if let Some(controller_id) = applicable_controller.filter(|controller_id| controller_is_live(controller_id) && !matches!(remedy.bound, RemedyBound::Remove | RemedyBound::Insert)) {
            rows.push(preview_row(format!("{chip_id}.preview"), &check.id, remedy_index, 0, &action, controller_id, locale)?);
        }
    }
    if let Some(controller_id) = controller_id.filter(|controller_id| controller_is_live(controller_id)).filter(|_| !check.remedies.is_empty()) {
        rows.push(clear_preview_row(format!("{id}.preview-clear"), controller_id, locale)?);
    }
    Ok(rows)
}
