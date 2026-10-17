//! 🌳️ Report tree rendering of the norm app surface: the roster example, the text helpers, the check tree and the report render. See [`super`].

use super::*;

//#region 🔖️Examples
/// 🎨️ Resolves `setActiveExample` through the editor's OWN example roster — the list its navbar picker
/// offers — so every example the picker shows loads, and nothing outside it does. An empty id is the
/// empty document, an id outside the roster is `None` (the no-op a picker's stale selection needs),
/// and an example body that does not parse is a named fault, never a silent fallback.
pub fn roster_example_snapshot<D: pack::snapshot::ArtifactDsl + Default>(examples: Vec<ExampleSource>, example_id: &str) -> Result<Option<D>, Fault> {
    let id = example_id.trim();
    if id.is_empty() {
        return Ok(Some(D::default()));
    }
    let Some(example) = examples.into_iter().find(|example| example.id() == id) else {
        return Ok(None);
    };
    D::parse_dsl(&example.document())
        .map(Some)
        .map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("norm.set-active-example-invalid"), format!("example '{id}' does not parse: {error:?}")))
}
//#endregion 🔖️Examples

//#region 🔖️Render
pub(super) fn render_text(value: impl Into<String>) -> UiAssemblyResult<BuiltNode> {
    let label = ui::Label::try_from(value.into()).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "norm UI label admission failed"))?;
    ui::text(label).try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "norm UI text build failed"))
}

pub(super) fn norm_ui_label(value: impl Into<String>) -> UiAssemblyResult<ui::Label> {
    ui::Label::try_from(value.into()).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "norm UI label admission failed"))
}

pub(super) fn render_text_chunks(value: &str) -> UiAssemblyResult<BuiltNode> {
    if value.len() <= ui::UI_TEXT_MAX_BYTES {
        return render_text(value);
    }
    let mut rest = value;
    let mut children = Vec::new();
    while !rest.is_empty() {
        let mut end = rest.len().min(ui::UI_TEXT_MAX_BYTES);
        while !rest.is_char_boundary(end) {
            end -= 1;
        }
        let label = ui::Label::try_from(&rest[..end]).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "norm UI text chunk admission failed"))?;
        let builder = ui::text(label).try_id(format!("norm-text-chunk-{}", children.len())).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "norm UI text chunk id admission failed"))?;
        children.push(builder.try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "norm UI text chunk build failed"))?);
        rest = &rest[end..];
    }
    ui::column().try_children(children).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "norm text chunk admission failed"))?.try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "norm text chunk build failed"))
}

pub(super) fn ui_error(code: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new(code, "norm UI admission failed")
}

pub(super) fn ui_text(value: impl AsRef<str>) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(value.as_ref()).ok_or_else(|| ui_error("ui.text"))
}

pub(super) fn ui_value_text(value: impl AsRef<str>) -> UiAssemblyResult<UiValue> {
    ui_text(value).map(UiValue::Text)
}

pub(super) fn action_args_map(entries: Vec<(&str, UiValue)>) -> UiAssemblyResult<UiValue> {
    let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_error("ui.value.map"))?;
    for (key, value) in entries {
        args.push(key.into(), value).map_err(|_| ui_error("ui.value.map.entry"))?;
    }
    Ok(UiValue::Map(args.finish()))
}

pub(super) fn norm_action(controller_id: &'static str, action: &str, args: Option<UiValue>) -> UiAssemblyResult<(semio_framework_plugin::ActionId, Option<UiValue>)> {
    ActionFactory::new(controller_id).action(action, args)
}

pub(super) fn bind_change<B: HasBase>(builder: B, controller_id: &'static str, action: &str, args: UiValue) -> UiAssemblyResult<B> {
    bind_on(builder, Trigger::Change, controller_id, action, args)
}

pub(super) fn bind_on<B: HasBase>(builder: B, trigger: Trigger, controller_id: &'static str, action: &str, args: UiValue) -> UiAssemblyResult<B> {
    let (action, args) = norm_action(controller_id, action, Some(args))?;
    match args {
        Some(args) => builder.try_on_with(trigger, action, args).map_err(|_| ui_error("ui.control.binding")),
        None => builder.try_on(trigger, action).map_err(|_| ui_error("ui.control.binding")),
    }
}

pub(super) fn control_row(row_id: &str, label: &str, control: BuiltNode) -> UiAssemblyResult<BuiltNode> {
    let row = ui::tree_item(norm_ui_label(label)?).try_id(row_id).map_err(|_| ui_error("ui.node.id"))?;
    row.try_child(control).map_err(|_| ui_error("ui.node.child"))?.try_build().map_err(|_| ui_error("ui.node.build"))
}

fn check_row_label(check: &CheckResult, index: usize, locale: Locale) -> String {
    let protocol_locale = protocol_locale(locale);
    let subject = check.subject.label.resolve(&protocol_locale);
    let subject_path = if check.subject.path.is_empty() {
        subject.to_string()
    } else {
        format!("{subject} @ {}", check.subject.path)
    };
    format!(
        "{}. [{}] {} — {} — {} vs {} u={:.2} — {}",
        index + 1,
        status_label(check.status, locale),
        check.title.resolve(&protocol_locale),
        subject_path,
        format_quantity(&check.computed),
        format_quantity(&check.limit),
        check.utilization,
        check.clause
    )
}

fn report_flat_rows<'a>(report: &'a CheckReport) -> Vec<ReportListRow<'a>> {
    let mut rows = Vec::new();
    let mut check_index = 0usize;
    for (part_index, (part, checks)) in report.by_part().into_iter().enumerate() {
        let verdict = report.summary.parts.iter().find(|item| item.part == part);
        rows.push(ReportListRow::Part { part_index, part, verdict });
        for check in checks {
            rows.push(ReportListRow::Check { check_index, check });
            check_index += 1;
        }
    }
    rows
}

enum ReportListRow<'a> {
    Part { part_index: usize, part: &'a str, verdict: Option<&'a semio_s_plugin_norm_compliance::PartVerdict> },
    Check { check_index: usize, check: &'a CheckResult },
}

fn part_verdict_label(part: &str, verdict: Option<&semio_s_plugin_norm_compliance::PartVerdict>, locale: Locale) -> String {
    let Some(verdict) = verdict else {
        return part.to_string();
    };
    let status = if verdict.complies {
        chrome("complies", "erfüllt", locale)
    } else {
        chrome("does not comply", "nicht erfüllt", locale)
    };
    format!(
        "{part} — {status} (✓{} ⚠{} ✗{} · u={:.2})",
        verdict.pass, verdict.warning, verdict.fail, verdict.worst_utilization
    )
}

fn build_check_tree_item(check: &CheckResult, check_index: usize, locale: Locale, controller_id: Option<&'static str>) -> UiAssemblyResult<BuiltNode> {
    let protocol_locale = protocol_locale(locale);
    let label = check_row_label(check, check_index, locale);
    let id = format!("norm-report-check-{check_index}");
    let expand = matches!(check.status, CheckStatus::Fail | CheckStatus::Warning);
    if !expand {
        return tree_item_desc(id, norm_ui_label(label)?, None);
    }
    let mut children = UiFixedList::default();
    children
        .try_push(tree_item_desc(
            format!("{id}.explanation"),
            norm_ui_label(chrome("Explanation", "Erläuterung", locale))?,
            Some(check.explanation.resolve(&protocol_locale).to_string()),
        )?)
        .map_err(|_| ui_error("ui.fixed-capacity"))?;
    for (remedy_index, remedy) in check.remedies.iter().enumerate() {
        let remedy_label = format!("{}: {}", chrome("Remedy", "Abhilfe", locale), remedy.action.resolve(&protocol_locale));
        if matches!(remedy.bound, RemedyBound::OneOf) {
            if remedy.options.is_empty() {
                children
                    .try_push(tree_item_desc(format!("{id}.remedy-{remedy_index}"), norm_ui_label(remedy_label)?, Some(chrome("No options", "Keine Optionen", locale)))?)
                    .map_err(|_| ui_error("ui.fixed-capacity"))?;
                continue;
            }
            for (option_index, option) in remedy.options.iter().enumerate() {
                let child = if let Some(controller_id) = controller_id {
                    let args = action_args_map(vec![("path", ui_value_text(&remedy.target.path)?), ("value", ui_value_text(option)?)])?;
                    tree_item_with_action(
                        format!("{id}.remedy-{remedy_index}.option-{option_index}"),
                        norm_ui_label(format!("{} — {option}", chrome("Choose", "Wählen", locale)))?,
                        Some(remedy_label.clone()),
                        norm_action(controller_id, "setField", Some(args))?,
                    )?
                } else {
                    tree_item_desc(format!("{id}.remedy-{remedy_index}.option-{option_index}"), norm_ui_label(format!("{remedy_label}: {option}"))?, None)?
                };
                children.try_push(child).map_err(|_| ui_error("ui.fixed-capacity"))?;
            }
            continue;
        }
        let child = if remedy.applicable {
            if let Some(controller_id) = controller_id {
                let args = action_args_map(vec![
                    ("checkId", ui_value_text(&check.id)?),
                    ("remedyIndex", UiValue::Number(remedy_index as f64)),
                ])?;
                tree_item_with_action(
                    format!("{id}.remedy-{remedy_index}"),
                    norm_ui_label(format!("{} — {}", chrome("Apply", "Anwenden", locale), remedy_label))?,
                    Some(format!("{} → {}", format_quantity(&remedy.current), format_quantity(&remedy.required))),
                    norm_action(controller_id, "applyRemedy", Some(args))?,
                )?
            } else {
                tree_item_desc(format!("{id}.remedy-{remedy_index}"), norm_ui_label(remedy_label)?, Some(format!("{} → {}", format_quantity(&remedy.current), format_quantity(&remedy.required))))?
            }
        } else {
            tree_item_desc(format!("{id}.remedy-{remedy_index}"), norm_ui_label(remedy_label)?, Some(chrome("Not auto-applicable", "Nicht automatisch anwendbar", locale)))?
        };
        children.try_push(child).map_err(|_| ui_error("ui.fixed-capacity"))?;
    }
    tree_group(id, norm_ui_label(label)?, true, children)
}

/// 📑️ Renders a whole `CheckReport` grouped by `part`, virtualised as one flat windowed section.
pub fn render_report(report: &CheckReport, windows: &TreeWindows<'_>, locale: Locale, controller_id: Option<&'static str>) -> UiAssemblyResult<BuiltNode> {
    let rows = report_flat_rows(report);
    PanelTreeBuilder::new(NORM_REPORT_TREE_ID)?
        .window_section_or_placeholder(
            windows,
            NORM_REPORT_SECTION_ID,
            Some(norm_ui_label(chrome("Checks", "Nachweise", locale))?),
            true,
            &rows,
            |row| match row {
                ReportListRow::Part { part_index, part, verdict } => {
                    tree_item_desc(format!("norm-report-part-{part_index}"), norm_ui_label(part_verdict_label(part, *verdict, locale))?, None)
                }
                ReportListRow::Check { check_index, check } => build_check_tree_item(check, *check_index, locale, controller_id),
            },
            norm_ui_label(chrome("No checks computed.", "Keine Nachweise berechnet.", locale))?,
        )?
        .build()
}
//#endregion 🔖️Render
