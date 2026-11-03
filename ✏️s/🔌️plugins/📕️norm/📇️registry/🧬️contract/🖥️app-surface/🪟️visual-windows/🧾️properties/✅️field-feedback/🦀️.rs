//! ✅️ Inline per-field feedback of the norm inspector: which checks use a field, how utilised it is and which remedy targets it. See [`super`].

use crate::*;

/// ✅️ A remedy that writes the feedback's field.
#[derive(Clone, Debug, PartialEq)]
pub struct RemedyChip {
    pub check_id: String,
    pub remedy_index: usize,
    pub action: String,
    pub required: String,
    pub applicable: bool,
}

/// ✅️ The governing check of one field.
#[derive(Clone, Debug, PartialEq)]
pub struct FieldFeedback {
    pub check_id: String,
    pub status: CheckStatus,
    pub utilization: f64,
    pub title: String,
    pub remedy: Option<RemedyChip>,
}

/// 🧱️ Roll-up of the checks that concern one entity.
#[derive(Clone, Debug, PartialEq)]
pub struct EntityFeedback {
    pub status: CheckStatus,
    pub utilization: f64,
    pub checks: usize,
}

/// 🔣️ Status glyph — never colour only.
pub fn status_glyph(status: CheckStatus) -> &'static str {
    match status {
        CheckStatus::Pass => "✓",
        CheckStatus::Warning => "⚠",
        CheckStatus::Fail => "✗",
        CheckStatus::NotApplicable => "∅",
        CheckStatus::UnverifiedBasis => "?",
        CheckStatus::Info => "ℹ",
    }
}

fn severity(status: CheckStatus) -> u8 {
    match status {
        CheckStatus::Fail => 3,
        CheckStatus::Warning | CheckStatus::UnverifiedBasis => 2,
        CheckStatus::Pass => 1,
        CheckStatus::NotApplicable | CheckStatus::Info => 0,
    }
}

fn remedy_for<'a>(check: &'a CheckResult, path: &str) -> Option<(usize, &'a semio_s_plugin_norm_compliance::Remedy)> {
    check.remedies.iter().enumerate().find(|(_, remedy)| remedy.target.path == path)
}

/// 🎯️ Whether `prefix` addresses `path` itself or an ancestor of it.
pub fn path_covers(prefix: &str, path: &str) -> bool {
    !prefix.is_empty() && (path == prefix || path.strip_prefix(prefix).is_some_and(|rest| rest.starts_with('.') || rest.starts_with('[')))
}

/// ✅️ The worst check whose subject is exactly `path` or whose remedy targets it; remedies are only offered for failing or warning checks.
pub fn field_feedback(report: &CheckReport, path: &str, locale: Locale) -> Option<FieldFeedback> {
    let protocol = protocol_locale(locale);
    report
        .checks
        .iter()
        .filter(|check| check.subject.path == path || remedy_for(check, path).is_some())
        .max_by(|a, b| (severity(a.status), a.utilization).partial_cmp(&(severity(b.status), b.utilization)).unwrap_or(std::cmp::Ordering::Equal))
        .map(|check| FieldFeedback {
            check_id: check.id.clone(),
            status: check.status,
            utilization: check.utilization,
            title: check.title.resolve(&protocol).to_string(),
            remedy: remedy_for(check, path).filter(|_| matches!(check.status, CheckStatus::Fail | CheckStatus::Warning)).map(|(index, remedy)| RemedyChip {
                check_id: check.id.clone(),
                remedy_index: index,
                action: remedy.action.resolve(&protocol).to_string(),
                required: format_quantity(&remedy.required),
                applicable: remedy.applicable,
            }),
        })
}

/// 🧱️ The worst status and utilisation over every check whose subject lies at or below `entity_path` or carries `entity_id`.
pub fn entity_feedback(report: &CheckReport, entity_id: &str, entity_path: &str) -> Option<EntityFeedback> {
    let mut related = report.checks.iter().filter(|check| (!entity_id.is_empty() && check.subject.entity_id == entity_id) || path_covers(entity_path, &check.subject.path)).peekable();
    related.peek()?;
    let mut feedback = EntityFeedback { status: CheckStatus::NotApplicable, utilization: 0.0, checks: 0 };
    for check in related {
        feedback.checks += 1;
        feedback.utilization = feedback.utilization.max(check.utilization);
        if severity(check.status) > severity(feedback.status) {
            feedback.status = check.status;
        }
    }
    Some(feedback)
}

/// 📶️ Percent text of a utilisation, glyph first: `✗ 112 %`.
pub fn utilization_text(status: CheckStatus, utilization: f64) -> String {
    format!("{} {:.0} %", status_glyph(status), utilization * 100.0)
}

fn tone_of(status: CheckStatus) -> ui::Tone {
    match status {
        CheckStatus::Pass => ui::Tone::Success,
        CheckStatus::Warning | CheckStatus::UnverifiedBasis => ui::Tone::Warning,
        CheckStatus::Fail => ui::Tone::Danger,
        CheckStatus::NotApplicable | CheckStatus::Info => ui::Tone::Neutral,
    }
}

/// 📶️ Utilisation bar on a 0–120 % scale with the glyph in its value text.
pub fn utilization_bar(id: String, status: CheckStatus, utilization: f64, title: &str, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let text = format!("{} — {} — {}", utilization_text(status, utilization), status_label(status, locale), title);
    let bar = ui::progress((utilization * 100.0).clamp(0.0, 120.0), norm_ui_label(text)?).total(120.0).try_id(id).map_err(|_| ui_error("ui.node.id"))?.tone(tone_of(status));
    let bar = bar.try_label(chrome("Utilisation", "Ausnutzung", locale)).map_err(|_| ui_error("ui.node.label"))?;
    bar.try_build().map_err(|_| ui_error("ui.node.build"))
}

/// 🩹️ One-click remedy chip with the required value.
pub fn remedy_chip(id: String, chip: &RemedyChip, controller_id: &'static str, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let label = format!("{}: {} — {}", chrome("Apply", "Anwenden", locale), chip.required, chip.action);
    if !chip.applicable {
        return tree_item_desc(id, norm_ui_label(label)?, Some(chrome("Not auto-applicable", "Nicht automatisch anwendbar", locale)));
    }
    let args = action_args_map(vec![("checkId", ui_value_text(&chip.check_id)?), ("remedyIndex", UiValue::Number(chip.remedy_index as f64))])?;
    tree_item_with_action(id, norm_ui_label(label)?, None, norm_action(controller_id, "applyRemedy", Some(args))?)
}
