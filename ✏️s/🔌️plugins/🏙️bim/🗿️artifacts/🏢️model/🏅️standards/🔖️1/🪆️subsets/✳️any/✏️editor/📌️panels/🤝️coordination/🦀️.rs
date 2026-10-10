//! 🤝️ The coordination panels of the BIM editor, three virtualised trees that share their rows: the clash panel (every clash set with the clashes it finds, grouped by the element that takes part in most of them; a
//! clash row opens the actions that select the pair, zoom to it, isolate it and raise an issue from it), the rules panel (every rule with the members that break it, the measured value against the limit; a member
//! row selects the element) and the issues panel (the BCF topics by status with their comments, the actions that select their elements and capture or restore their viewpoint, and the BCF export). Running the checks is
//! the `analyseModel` job, which has progress and cancel; everything shown is inferred, the issues are the authored records. Opening or closing a section is the filter, the host keeps the open state per view.

use crate::editor::bim::entities::kind_holding;
use crate::editor::bim::kit::{bim_action, tree_item_with_icon, ui_capacity_error, ui_label, ui_value_list, ui_value_map, ui_value_text};
use crate::editor::bim::panels::outliner::{item, leaf};
use crate::editor::bim::terminology::BimLabels;
use crate::standards::v1::subsets::any::schema::inferences::clash_sets::{Clash, ClashGroup, ClashKind, ClashSetResult};
use crate::standards::v1::subsets::any::schema::inferences::rule_results::RuleResult;
use crate::{comments_of, Issue, IssueStatus, ModelInference, ModelSnapshot, Rule, RuleSeverity};
use semio_framework_plugin::{BuiltNode, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, TreeWindows, UiAssemblyResult, UiValue, FRAMEWORK_PANEL_TAB_INSPECTION_ID};
use semio_framework_ui_locale::LocalizedLabel;

//#region 🔖️Constants
pub const CLASHES_KEY: &str = "bim.edit.clashes";
pub const RULES_KEY: &str = "bim.edit.rules";
pub const ISSUES_KEY: &str = "bim.edit.issues";
const CLASHES_ROOT: &str = "bim-clashes";
const RULES_ROOT: &str = "bim-rules";
const ISSUES_ROOT: &str = "bim-issues";
/// ✂️ The longest text a row shows, in characters (a fixed UI text holds 512 bytes).
const CLIP: usize = 200;
/// 📂️ A list with at most this many rows opens on first paint; a longer one waits for the host to open it and then streams its slice.
const OPEN_LIMIT: usize = 12;
/// 🏁️ The statuses in the order of the issue sections.
pub const STATUSES: [IssueStatus; 4] = [IssueStatus::Open, IssueStatus::InProgress, IssueStatus::Resolved, IssueStatus::Closed];
//#endregion 🔖️Constants

//#region 🔖️Definitions
fn definition(label: LocalizedLabel, key: &str) -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()), label, group: PanelGroup::Details, body_key: Some(key.into()), children: Vec::new() }
}

/// 🧱️ The clash panel tab, stitched into the app manifest by `create_bim_app`.
pub fn clashes_definition() -> PanelTabDefinition {
    definition(BimLabels::localized(|labels| labels.panel_clashes), CLASHES_KEY)
}

/// 🧱️ The rules panel tab.
pub fn rules_definition() -> PanelTabDefinition {
    definition(BimLabels::localized(|labels| labels.panel_rules), RULES_KEY)
}

/// 🧱️ The issues panel tab.
pub fn issues_definition() -> PanelTabDefinition {
    definition(BimLabels::localized(|labels| labels.panel_issues), ISSUES_KEY)
}
//#endregion 🔖️Definitions

//#region 🔖️Rows
fn clip(text: &str) -> String {
    if text.chars().count() <= CLIP {
        return text.to_string();
    }
    let mut clipped: String = text.chars().take(CLIP - 1).collect();
    clipped.push('…');
    clipped
}

/// 🏷️ The name of an element or space for a row: its authored name, else its id.
pub fn name_of(snapshot: &ModelSnapshot, id: &str) -> String {
    kind_holding(snapshot, id).and_then(|row| (row.name)(snapshot, id)).filter(|name| !name.is_empty()).unwrap_or_else(|| id.to_string())
}

fn ids_value(ids: &[String]) -> UiAssemblyResult<UiValue> {
    ui_value_list(ids.iter().map(ui_value_text).collect::<UiAssemblyResult<Vec<_>>>()?)
}

/// 🎯️ A row that runs `action` with the text arguments `texts` and the id list `ids` when activated.
fn action_row(id: String, label: &str, icon: &str, action: &str, texts: &[(&'static str, &str)], ids: Option<&[String]>) -> UiAssemblyResult<BuiltNode> {
    let mut entries: Vec<(&'static str, UiValue)> = texts.iter().map(|(key, value)| ui_value_text(value).map(|value| (*key, value))).collect::<UiAssemblyResult<Vec<_>>>()?;
    if let Some(ids) = ids {
        entries.push(("ids", ids_value(ids)?));
    }
    tree_item_with_icon(id, label, icon, bim_action(action, Some(ui_value_map(entries)?)))
}

fn run_row(root: &str, text: &str) -> UiAssemblyResult<BuiltNode> {
    tree_item_with_icon(format!("{root}.run"), text, "play", bim_action("analyseModel", None))
}

fn note_row(id: String, text: &str, icon: &str) -> UiAssemblyResult<BuiltNode> {
    leaf(item(&id, &clip(text), icon, None, None)?)
}

fn mm(metres: f64) -> String {
    format!("{} mm", (metres * 1000.0).round())
}
//#endregion 🔖️Rows

//#region 🔖️Clashes
/// 🗂️ The clashes of a result by group, as `(group, clashes)` in the order of the groups.
pub fn grouped<'a>(result: &'a ClashSetResult) -> Vec<(&'a ClashGroup, Vec<&'a Clash>)> {
    result.groups.iter().map(|group| (group, group.members.iter().filter_map(|at| result.clashes.get(*at as usize)).collect())).collect()
}

fn kind_label(labels: &BimLabels, kind: ClashKind) -> &str {
    match kind {
        ClashKind::Hard => labels.clash_hard.as_str(),
        ClashKind::Clearance => labels.clash_soft.as_str(),
    }
}

fn clash_row(windows: &TreeWindows<'_>, snapshot: &ModelSnapshot, labels: &BimLabels, set: &str, id: &str, clash: &Clash) -> UiAssemblyResult<BuiltNode> {
    let pair = [clash.first.clone(), clash.second.clone()];
    let text = clip(&format!("{} ↔ {}", name_of(snapshot, &clash.first), name_of(snapshot, &clash.second)));
    let measure = match clash.kind {
        ClashKind::Hard => format!("{} {}", kind_label(labels, clash.kind), mm(-clash.distance)),
        ClashKind::Clearance => format!("{} {}", kind_label(labels, clash.kind), mm(clash.distance)),
    };
    let builder = item(id, &text, if clash.kind == ClashKind::Hard { "alert-circle" } else { "triangle-alert" }, None, Some(&measure))?;
    let with_set = [("set", set), ("first", clash.first.as_str()), ("second", clash.second.as_str())];
    semio_framework_plugin::tree_window_indexed_item(windows, builder, id, false, 4, |position| match position {
        0 => action_row(format!("{id}.select"), labels.clash_select.as_str(), "mouse-pointer", "selectFindings", &[], Some(&pair)),
        1 => action_row(format!("{id}.zoom"), labels.clash_zoom.as_str(), "zoom-in", "viewClash", &[("first", clash.first.as_str()), ("second", clash.second.as_str()), ("mode", "zoom")], None),
        2 => action_row(format!("{id}.isolate"), labels.clash_isolate.as_str(), "eye", "viewClash", &[("first", clash.first.as_str()), ("second", clash.second.as_str()), ("mode", "isolate")], None),
        _ => action_row(format!("{id}.raise"), labels.clash_raise.as_str(), "flag", "raiseIssue", &with_set, None),
    })
}

fn group_row(windows: &TreeWindows<'_>, snapshot: &ModelSnapshot, labels: &BimLabels, set: &str, group: &(&ClashGroup, Vec<&Clash>)) -> UiAssemblyResult<BuiltNode> {
    let id = format!("{CLASHES_ROOT}.{set}.{}", group.0.anchor);
    let builder = item(&id, &clip(&name_of(snapshot, &group.0.anchor)), "layers", None, Some(&group.1.len().to_string()))?;
    semio_framework_plugin::tree_window_indexed_item(windows, builder, &id, group.1.len() <= OPEN_LIMIT, group.1.len(), |position| {
        let clash = group.1[position];
        clash_row(windows, snapshot, labels, set, &format!("{id}.{}-{}", clash.first, clash.second), clash)
    })
}

/// 💥️ Renders the clash panel: a run row, then a section per clash set with its groups; an empty-state row when the model has no clash set.
pub fn render_clashes(snapshot: &ModelSnapshot, inference: &ModelInference, labels: &BimLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let mut builder = PanelTreeBuilder::new(CLASHES_ROOT)?;
    builder = builder.section(format!("{CLASHES_ROOT}.job"), Some(ui_label(labels.panel_clashes.as_str())?), true, semio_framework_plugin::ui_node_list([run_row(CLASHES_ROOT, labels.clash_run.as_str())?])?)?;
    if snapshot.clash_sets.is_empty() {
        return builder.section(format!("{CLASHES_ROOT}.none"), None, true, semio_framework_plugin::ui_node_list([note_row(format!("{CLASHES_ROOT}.empty"), labels.clash_empty.as_str(), "info")?])?)?.build();
    }
    for (set_id, set) in &snapshot.clash_sets {
        let Some(result) = inference.clash_sets.get(set_id) else { continue };
        let rows = grouped(result);
        let heading = format!("{} ({} {} · {} {} · {} {})", clip(&set.name), result.hard(), labels.clash_hard.as_str(), result.soft(), labels.clash_soft.as_str(), result.tested, labels.clash_tested.as_str());
        if rows.is_empty() {
            builder = builder.section(format!("{CLASHES_ROOT}.set.{set_id}"), Some(ui_label(&heading)?), true, semio_framework_plugin::ui_node_list([note_row(format!("{CLASHES_ROOT}.{set_id}.clean"), labels.clash_none.as_str(), "check")?])?)?;
        } else {
            builder = builder.window_section(windows, &format!("{CLASHES_ROOT}.set.{set_id}"), Some(ui_label(&heading)?), true, &rows, |group| group_row(windows, snapshot, labels, set_id, group))?;
        }
    }
    builder.build()
}
//#endregion 🔖️Clashes

//#region 🔖️Rules
fn severity_icon(severity: RuleSeverity) -> &'static str {
    match severity {
        RuleSeverity::Error => "alert-circle",
        RuleSeverity::Warning => "triangle-alert",
        RuleSeverity::Note => "info",
    }
}

fn severity_word(labels: &BimLabels, severity: RuleSeverity) -> &str {
    match severity {
        RuleSeverity::Error => labels.diag_error.as_str(),
        RuleSeverity::Warning => labels.diag_warning.as_str(),
        RuleSeverity::Note => labels.diag_note.as_str(),
    }
}

fn violation_row(snapshot: &ModelSnapshot, labels: &BimLabels, rule_id: &str, rule: &Rule, index: usize, finding: &crate::standards::v1::subsets::any::schema::inferences::rule_results::RuleFinding) -> UiAssemblyResult<BuiltNode> {
    let unit = if rule.kind == crate::RuleKind::MaxCompartmentArea { "m²" } else if rule.kind == crate::RuleKind::MaxRampSlope { "" } else { "m" };
    let description = clip(&format!("{} · {} {} {} {}", severity_word(labels, rule.severity), format_number(finding.measured), unit, if rule.kind.is_maximum() { labels.rule_above.as_str() } else { labels.rule_below.as_str() }, format_number(finding.limit)));
    let mut node = tree_item_with_icon(format!("{RULES_ROOT}.{rule_id}.{index}"), &clip(&name_of(snapshot, &finding.element)), severity_icon(rule.severity), bim_action("selectFindings", Some(ui_value_map([("ids", ids_value(std::slice::from_ref(&finding.element))?)])?)))?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
        props.description = Some(semio_framework_plugin::UiText::try_from_str(&description).ok_or_else(ui_capacity_error)?);
    }
    Ok(node)
}

fn format_number(value: f64) -> String {
    let rounded = (value * 1000.0).round() / 1000.0;
    format!("{rounded}")
}

/// ⚖️ Renders the rules panel: a run row, then a section per rule with its violations; an empty-state row when the model has no rule.
pub fn render_rules(snapshot: &ModelSnapshot, inference: &ModelInference, labels: &BimLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let mut builder = PanelTreeBuilder::new(RULES_ROOT)?;
    builder = builder.section(format!("{RULES_ROOT}.job"), Some(ui_label(labels.panel_rules.as_str())?), true, semio_framework_plugin::ui_node_list([run_row(RULES_ROOT, labels.rules_run.as_str())?])?)?;
    if snapshot.rules.is_empty() {
        return builder.section(format!("{RULES_ROOT}.none"), None, true, semio_framework_plugin::ui_node_list([note_row(format!("{RULES_ROOT}.empty"), labels.rules_empty.as_str(), "info")?])?)?.build();
    }
    for (rule_id, rule) in &snapshot.rules {
        let Some(result): Option<&RuleResult> = inference.rule_results.get(rule_id) else { continue };
        let heading = format!("{} ({} / {})", clip(&rule.name), result.violations.len(), result.checked);
        if result.violations.is_empty() {
            builder = builder.section(format!("{RULES_ROOT}.rule.{rule_id}"), Some(ui_label(&heading)?), false, semio_framework_plugin::ui_node_list([note_row(format!("{RULES_ROOT}.{rule_id}.ok"), labels.rules_ok.as_str(), "check")?])?)?;
        } else {
            builder = builder.window_section(windows, &format!("{RULES_ROOT}.rule.{rule_id}"), Some(ui_label(&heading)?), result.violations.len() <= OPEN_LIMIT, &(0..result.violations.len()).collect::<Vec<_>>(), |index| violation_row(snapshot, labels, rule_id, rule, *index, &result.violations[*index]))?;
        }
    }
    builder.build()
}
//#endregion 🔖️Rules

//#region 🔖️Issues
/// 🏁️ The heading word of a status.
pub fn status_label(labels: &BimLabels, status: IssueStatus) -> &str {
    match status {
        IssueStatus::Open => labels.status_open.as_str(),
        IssueStatus::InProgress => labels.status_in_progress.as_str(),
        IssueStatus::Resolved => labels.status_resolved.as_str(),
        IssueStatus::Closed => labels.status_closed.as_str(),
    }
}

fn priority_label(labels: &BimLabels, priority: crate::IssuePriority) -> &str {
    match priority {
        crate::IssuePriority::Low => labels.priority_low.as_str(),
        crate::IssuePriority::Normal => labels.priority_normal.as_str(),
        crate::IssuePriority::High => labels.priority_high.as_str(),
        crate::IssuePriority::Critical => labels.priority_critical.as_str(),
    }
}

/// 🚩️ The ids of the issues of one status in id order.
pub fn issues_with(snapshot: &ModelSnapshot, status: IssueStatus) -> Vec<String> {
    snapshot.issues.iter().filter(|(_, issue)| issue.status == status).map(|(id, _)| id.clone()).collect()
}

fn issue_row(windows: &TreeWindows<'_>, snapshot: &ModelSnapshot, labels: &BimLabels, id: &str, issue: &Issue) -> UiAssemblyResult<BuiltNode> {
    let row_id = format!("{ISSUES_ROOT}.issue.{id}");
    let assignee = if issue.assignee.is_empty() { labels.issue_unassigned.as_str() } else { issue.assignee.as_str() };
    let description = clip(&format!("{} · {}", priority_label(labels, issue.priority), assignee));
    let comments = comments_of(snapshot, id);
    let builder = item(&row_id, &clip(&issue.title), "flag", None, Some(&description))?;
    let count = 2 + usize::from(!issue.elements.is_empty()) + usize::from(issue.viewpoint.is_some()) + comments.len();
    let elements = issue.elements.clone();
    semio_framework_plugin::tree_window_indexed_item(windows, builder, &row_id, count <= OPEN_LIMIT, count, |position| {
        let mut at = position;
        if !issue.elements.is_empty() {
            if at == 0 {
                return action_row(format!("{row_id}.select"), labels.issue_select.as_str(), "mouse-pointer", "selectFindings", &[], Some(&elements));
            }
            at -= 1;
        }
        if issue.viewpoint.is_some() {
            if at == 0 {
                return action_row(format!("{row_id}.restore"), labels.issue_restore.as_str(), "camera", "restoreViewpoint", &[("issue", id)], None);
            }
            at -= 1;
        }
        match at {
            0 => action_row(format!("{row_id}.capture"), labels.issue_capture.as_str(), "focus", "captureViewpoint", &[("issue", id)], None),
            1 => action_row(format!("{row_id}.comment"), labels.issue_comment_add.as_str(), "message-square-plus", "createEntity", &[("kind", "issue-comment"), ("parent", id), ("name", labels.issue_new_comment.as_str())], None),
            _ => {
                let (comment_id, comment) = comments[at - 2];
                note_row(format!("{row_id}.c.{comment_id}"), &clip(&format!("{}: {}", comment.author, comment.text)), "message-square")
            }
        }
    })
}

/// 🚩️ Renders the issues panel: the export and new-issue rows, then a section per status with its issues; an empty-state row when the model has no issue.
pub fn render_issues(snapshot: &ModelSnapshot, labels: &BimLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let mut builder = PanelTreeBuilder::new(ISSUES_ROOT)?;
    let export = tree_item_with_icon(format!("{ISSUES_ROOT}.export"), labels.issue_export.as_str(), "download", bim_action("exportModel", Some(ui_value_map([("format", ui_value_text("bcf")?)])?)))?;
    let create = tree_item_with_icon(format!("{ISSUES_ROOT}.new"), labels.issue_new.as_str(), "flag", bim_action("createEntity", Some(ui_value_map([("kind", ui_value_text("issue")?), ("name", ui_value_text(labels.issue_new_title.as_str())?)])?)))?;
    builder = builder.section(format!("{ISSUES_ROOT}.job"), Some(ui_label(labels.panel_issues.as_str())?), true, semio_framework_plugin::ui_node_list([create, export])?)?;
    if snapshot.issues.is_empty() {
        return builder.section(format!("{ISSUES_ROOT}.none"), None, true, semio_framework_plugin::ui_node_list([note_row(format!("{ISSUES_ROOT}.empty"), labels.issue_empty.as_str(), "info")?])?)?.build();
    }
    for status in STATUSES {
        let ids = issues_with(snapshot, status);
        if ids.is_empty() {
            continue;
        }
        let heading = format!("{} ({})", status_label(labels, status), ids.len());
        builder = builder.window_section(windows, &format!("{ISSUES_ROOT}.{status:?}"), Some(ui_label(&heading)?), matches!(status, IssueStatus::Open | IssueStatus::InProgress), &ids, |id| issue_row(windows, snapshot, labels, id, &snapshot.issues[id]))?;
    }
    builder.build()
}
//#endregion 🔖️Issues

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
