//! 🤝️ The coordination rows of the entity table: how an authored clash set, rule, issue and issue comment read off the snapshot, how an edited value becomes a `set-*` mutation, what a new one is and the
//! pickers (rule check, severity, status, priority) the properties panel offers. A selector or a scope is edited as text of `key=value,value` parts separated by semicolons (`classes=beam,column;
//! storeys=st-0`); an empty text selects everything. The clash counts and the rule findings are inferred rows of the `clash-sets` and `rule-results` inferences.

use super::{number, parse_number, parse_text, partial, variant, Created, EntityKind, FieldRow, InferredRow};
use crate::editor::bim::terminology::BimLabels;
use crate::{ClashSet, ElementClass, ElementSelector, Issue, IssueComment, IssuePriority, IssueStatus, ModelMutation, ModelSnapshot, Phase, Rule, RuleKind, RuleScope, RuleSeverity, DEFAULT_TOLERANCE, RULE_KINDS};
use semio_framework_plugin::plugin_app_close_prelude::InputKind;

//#region 🔖️Text
const PHASES: [Phase; 4] = [Phase::Existing, Phase::New, Phase::Demolished, Phase::Temporary];

fn words(text: &str) -> Vec<String> {
    text.split(',').map(str::trim).filter(|word| !word.is_empty()).map(str::to_string).collect()
}

fn parts(text: &str) -> Option<Vec<(String, String)>> {
    text.split(';').map(str::trim).filter(|part| !part.is_empty()).map(|part| part.split_once('=').map(|(key, value)| (key.trim().to_ascii_lowercase(), value.trim().to_string()))).collect()
}

fn phase_names(phases: &[Phase]) -> String {
    phases.iter().map(|phase| format!("{phase:?}")).collect::<Vec<_>>().join(",")
}

fn push(out: &mut Vec<String>, key: &str, items: Vec<String>) {
    if !items.is_empty() {
        out.push(format!("{key}={}", items.join(",")));
    }
}

/// 🎯️ The text of a selector: its non-empty parts, empty for a selector that picks everything.
pub fn selector_text(selector: &ElementSelector) -> String {
    let mut out = Vec::new();
    push(&mut out, "classes", selector.classes.iter().map(|class| class.name().to_string()).collect());
    push(&mut out, "storeys", selector.storeys.clone());
    push(&mut out, "phases", vec![phase_names(&selector.phases)].into_iter().filter(|text| !text.is_empty()).collect());
    push(&mut out, "ids", selector.ids.clone());
    out.join("; ")
}

/// 🎯️ The selector a text names; none when a part is unknown or a class or phase is misspelled.
pub fn parse_selector(text: &str) -> Option<ElementSelector> {
    let mut selector = ElementSelector::all();
    for (key, value) in parts(text)? {
        match key.as_str() {
            "classes" => selector.classes = words(&value).iter().map(|word| ElementClass::parse(word)).collect::<Option<_>>()?,
            "storeys" => selector.storeys = words(&value),
            "phases" => selector.phases = words(&value).iter().map(|word| variant(word, &PHASES)).collect::<Option<_>>()?,
            "ids" => selector.ids = words(&value),
            _ => return None,
        }
    }
    Some(selector)
}

/// 🎯️ The text of a rule scope: its non-empty parts.
pub fn scope_text(scope: &RuleScope) -> String {
    let mut out = Vec::new();
    push(&mut out, "storeys", scope.storeys.clone());
    push(&mut out, "phases", vec![phase_names(&scope.phases)].into_iter().filter(|text| !text.is_empty()).collect());
    push(&mut out, "ids", scope.ids.clone());
    if !scope.filter.is_empty() {
        out.push(format!("filter={}", scope.filter));
    }
    out.join("; ")
}

/// 🎯️ The rule scope a text names; none when a part is unknown or a phase is misspelled.
pub fn parse_scope(text: &str) -> Option<RuleScope> {
    let mut scope = RuleScope::all();
    for (key, value) in parts(text)? {
        match key.as_str() {
            "storeys" => scope.storeys = words(&value),
            "phases" => scope.phases = words(&value).iter().map(|word| variant(word, &PHASES)).collect::<Option<_>>()?,
            "ids" => scope.ids = words(&value),
            "filter" => scope.filter = value,
            _ => return None,
        }
    }
    Some(scope)
}

fn parse_words(text: &str) -> Option<Vec<String>> {
    Some(words(text))
}

fn parse_kind(text: &str) -> Option<RuleKind> {
    RuleKind::parse(text).or_else(|| variant(text, &RULE_KINDS))
}

fn parse_severity(text: &str) -> Option<RuleSeverity> {
    variant(text, &[RuleSeverity::Error, RuleSeverity::Warning, RuleSeverity::Note])
}

fn parse_status(text: &str) -> Option<IssueStatus> {
    variant(text, &[IssueStatus::Open, IssueStatus::InProgress, IssueStatus::Resolved, IssueStatus::Closed])
}

fn parse_priority(text: &str) -> Option<IssuePriority> {
    variant(text, &[IssuePriority::Low, IssuePriority::Normal, IssuePriority::High, IssuePriority::Critical])
}

fn clash_text(issue: &Issue) -> String {
    issue.clash.as_ref().map_or_else(String::new, |clash| format!("{}: {} ↔ {}", clash.set, clash.first, clash.second))
}

fn viewpoint_text(issue: &Issue) -> String {
    issue.viewpoint.as_ref().map_or_else(String::new, |viewpoint| {
        let mut out = vec![format!("camera {} m", number((viewpoint.camera.distance * 100.0).round() / 100.0))];
        if viewpoint.section.is_some() {
            out.push("section box".to_string());
        }
        if !viewpoint.isolate.is_empty() {
            out.push(format!("{} isolated", viewpoint.isolate.len()));
        }
        out.join(", ")
    })
}
//#endregion 🔖️Text

//#region 🔖️Choices
/// 📏️ The checks of a rule by their stored name and their localized label.
pub fn kind_choices(_: &ModelSnapshot, labels: &BimLabels) -> Vec<(String, String)> {
    RULE_KINDS
        .into_iter()
        .map(|kind| {
            let label = match kind {
                RuleKind::MinClearHeight => labels.rule_min_clear_height,
                RuleKind::MaxRiser => labels.rule_max_riser,
                RuleKind::MinTread => labels.rule_min_tread,
                RuleKind::MinStairWidth => labels.rule_min_stair_width,
                RuleKind::MinDoorWidth => labels.rule_min_door_width,
                RuleKind::MaxRampSlope => labels.rule_max_ramp_slope,
                RuleKind::MinCorridorWidth => labels.rule_min_corridor_width,
                RuleKind::MaxCompartmentArea => labels.rule_max_compartment_area,
            };
            (format!("{kind:?}"), label.as_str().to_string())
        })
        .collect()
}

/// 🚦️ The severities of a rule by their stored name and their localized label.
pub fn severity_choices(_: &ModelSnapshot, labels: &BimLabels) -> Vec<(String, String)> {
    vec![("Error".to_string(), labels.diag_error.as_str().to_string()), ("Warning".to_string(), labels.diag_warning.as_str().to_string()), ("Note".to_string(), labels.diag_note.as_str().to_string())]
}

/// 🏁️ The statuses of an issue by their stored name and their localized label.
pub fn status_choices(_: &ModelSnapshot, labels: &BimLabels) -> Vec<(String, String)> {
    vec![
        ("Open".to_string(), labels.status_open.as_str().to_string()),
        ("InProgress".to_string(), labels.status_in_progress.as_str().to_string()),
        ("Resolved".to_string(), labels.status_resolved.as_str().to_string()),
        ("Closed".to_string(), labels.status_closed.as_str().to_string()),
    ]
}

/// ⏫️ The priorities of an issue by their stored name and their localized label.
pub fn priority_choices(_: &ModelSnapshot, labels: &BimLabels) -> Vec<(String, String)> {
    vec![
        ("Low".to_string(), labels.priority_low.as_str().to_string()),
        ("Normal".to_string(), labels.priority_normal.as_str().to_string()),
        ("High".to_string(), labels.priority_high.as_str().to_string()),
        ("Critical".to_string(), labels.priority_critical.as_str().to_string()),
    ]
}
//#endregion 🔖️Choices

//#region 🔖️Fields
/// 🧾️ The authored parameters of a clash set.
pub static CLASH_SET_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.clash_sets.get(id).map(|row| row.name.clone()), parse_text => set_clash_set::SetClashSet),
    field!("a", field_side_a, Text, |s, id| s.clash_sets.get(id).map(|row| selector_text(&row.a)), parse_selector => set_clash_set::SetClashSet),
    field!("b", field_side_b, Text, |s, id| s.clash_sets.get(id).map(|row| selector_text(&row.b)), parse_selector => set_clash_set::SetClashSet),
    field!("tolerance", field_tolerance, Number, |s, id| s.clash_sets.get(id).map(|row| number(row.tolerance)), parse_number => set_clash_set::SetClashSet),
    field!("clearance", field_clearance, Number, |s, id| s.clash_sets.get(id).map(|row| number(row.clearance)), parse_number => set_clash_set::SetClashSet),
];

/// 🧾️ The authored parameters of a rule.
pub static RULE_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.rules.get(id).map(|row| row.name.clone()), parse_text => set_rule::SetRule),
    field!("kind", field_check, Text, |s, id| s.rules.get(id).map(|row| format!("{:?}", row.kind)), choices: kind_choices, parse_kind => set_rule::SetRule),
    field!("limit", field_limit, Number, |s, id| s.rules.get(id).map(|row| number(row.limit)), parse_number => set_rule::SetRule),
    field!("severity", field_severity, Text, |s, id| s.rules.get(id).map(|row| format!("{:?}", row.severity)), choices: severity_choices, parse_severity => set_rule::SetRule),
    field!("scope", field_scope, Text, |s, id| s.rules.get(id).map(|row| scope_text(&row.scope)), parse_scope => set_rule::SetRule),
];

/// 🧾️ The authored parameters of an issue.
pub static ISSUE_FIELDS: &[FieldRow] = &[
    field!("title", field_title, Text, |s, id| s.issues.get(id).map(|row| row.title.clone()), parse_text => set_issue::SetIssue),
    field!("description", field_description, Text, |s, id| s.issues.get(id).map(|row| row.description.clone()), parse_text => set_issue::SetIssue),
    field!("status", field_status, Text, |s, id| s.issues.get(id).map(|row| format!("{:?}", row.status)), choices: status_choices, parse_status => set_issue::SetIssue),
    field!("priority", field_priority, Text, |s, id| s.issues.get(id).map(|row| format!("{:?}", row.priority)), choices: priority_choices, parse_priority => set_issue::SetIssue),
    field!("assignee", field_assignee, Text, |s, id| s.issues.get(id).map(|row| row.assignee.clone()), parse_text => set_issue::SetIssue),
    field!("author", field_author, Text, |s, id| s.issues.get(id).map(|row| row.author.clone()), parse_text => set_issue::SetIssue),
    field!("created", field_created, Text, |s, id| s.issues.get(id).map(|row| row.created.clone()), parse_text => set_issue::SetIssue),
    field!("labels", field_issue_labels, Text, |s, id| s.issues.get(id).map(|row| row.labels.join(",")), parse_words => set_issue::SetIssue),
    field!("elements", field_issue_elements, Text, |s, id| s.issues.get(id).map(|row| row.elements.join(",")), parse_words => set_issue::SetIssue),
    field!("clash", field_issue_clash, Text, |s, id| s.issues.get(id).map(clash_text)),
    field!("viewpoint", field_issue_viewpoint, Text, |s, id| s.issues.get(id).map(viewpoint_text)),
];

/// 🧾️ The authored parameters of an issue comment.
pub static COMMENT_FIELDS: &[FieldRow] = &[
    field!("issue", field_issue, Text, |s, id| s.issue_comments.get(id).map(|row| row.issue.clone())),
    field!("author", field_author, Text, |s, id| s.issue_comments.get(id).map(|row| row.author.clone()), parse_text => set_issue_comment::SetIssueComment),
    field!("date", field_written, Text, |s, id| s.issue_comments.get(id).map(|row| row.date.clone()), parse_text => set_issue_comment::SetIssueComment),
    field!("text", field_text, Text, |s, id| s.issue_comments.get(id).map(|row| row.text.clone()), parse_text => set_issue_comment::SetIssueComment),
];
//#endregion 🔖️Fields

//#region 🔖️Inferred
/// 💡️ What a clash set finds now, read off the `clash-sets` inference: the hard and the soft clashes and the pairs tested.
pub static CLASH_SET_INFERRED: &[InferredRow] = &[
    inferred!("hard", field_hard, |s, inference, id| inference.clash_sets.get(id).filter(|_| s.clash_sets.contains_key(id)).map(|result| result.hard().to_string())),
    inferred!("soft", field_soft, |s, inference, id| inference.clash_sets.get(id).filter(|_| s.clash_sets.contains_key(id)).map(|result| result.soft().to_string())),
    inferred!("tested", field_tested, |s, inference, id| inference.clash_sets.get(id).filter(|_| s.clash_sets.contains_key(id)).map(|result| result.tested.to_string())),
];

/// 💡️ What a rule finds now, read off the `rule-results` inference: the members measured and the violations.
pub static RULE_INFERRED: &[InferredRow] = &[
    inferred!("checked", field_checked, |s, inference, id| inference.rule_results.get(id).filter(|_| s.rules.contains_key(id)).map(|result| result.checked.to_string())),
    inferred!("violations", field_violations, |s, inference, id| inference.rule_results.get(id).filter(|_| s.rules.contains_key(id)).map(|result| result.violations.len().to_string())),
];
//#endregion 🔖️Inferred

//#region 🔖️Create
/// 📅️ The latest moment the model knows (an issue, a comment, a sheet or a revision): the editor has no clock, so a new issue or comment starts there and is dated by its author afterwards; the epoch for a model that knows none.
pub fn latest_moment(snapshot: &ModelSnapshot) -> String {
    snapshot
        .issues
        .values()
        .map(|row| row.created.as_str())
        .chain(snapshot.issue_comments.values().map(|row| row.date.as_str()))
        .chain(snapshot.sheets.values().map(|row| row.date.as_str()))
        .chain(snapshot.sheet_revisions.values().map(|row| row.date.as_str()))
        .filter(|moment| !moment.is_empty())
        .max_by_key(|moment| moment.get(..10).unwrap_or(moment).to_string())
        .map_or_else(|| "1970-01-01".to_string(), str::to_string)
}

fn author(snapshot: &ModelSnapshot) -> String {
    if snapshot.project.author.trim().is_empty() { "unknown".to_string() } else { snapshot.project.author.clone() }
}

/// 💥️ A new clash set: everything against everything with the default tolerance, named by the caller.
pub fn create_clash_set(_: &ModelSnapshot, id: &str, _parent: &str, name: &str) -> Created {
    let clash_set = ClashSet { tolerance: DEFAULT_TOLERANCE, ..ClashSet::standard(name, ElementSelector::all(), ElementSelector::all()) };
    Ok(ModelMutation::CreateClashSet(crate::mutations::create_clash_set::CreateClashSet { id: id.into(), clash_set }))
}

/// ⚖️ A new rule: the riser limit of DIN 18065 as an error everywhere, named by the caller.
pub fn create_rule(_: &ModelSnapshot, id: &str, _parent: &str, name: &str) -> Created {
    let rule = Rule { name: name.into(), kind: RuleKind::MaxRiser, limit: 0.19, severity: RuleSeverity::Error, scope: RuleScope::all() };
    Ok(ModelMutation::CreateRule(crate::mutations::create_rule::CreateRule { id: id.into(), rule }))
}

/// 🚩️ A new open issue titled by the caller, raised by the author of the project at the latest moment the model knows.
pub fn create_issue(snapshot: &ModelSnapshot, id: &str, _parent: &str, name: &str) -> Created {
    let issue = Issue {
        title: name.into(),
        description: String::new(),
        status: IssueStatus::Open,
        priority: IssuePriority::Normal,
        assignee: String::new(),
        author: author(snapshot),
        created: latest_moment(snapshot),
        labels: Vec::new(),
        elements: Vec::new(),
        clash: None,
        viewpoint: None,
    };
    Ok(ModelMutation::CreateIssue(crate::mutations::create_issue::CreateIssue { id: id.into(), issue }))
}

/// 💬️ A new comment on the issue `parent`, worded by the caller.
pub fn create_comment(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    if !snapshot.issues.contains_key(parent) {
        return Err("bim.create.issue-missing");
    }
    let issue_comment = IssueComment { issue: parent.into(), author: author(snapshot), date: latest_moment(snapshot), text: name.into() };
    Ok(ModelMutation::CreateIssueComment(crate::mutations::create_issue_comment::CreateIssueComment { id: id.into(), issue_comment }))
}
//#endregion 🔖️Create

//#region 🔖️Kinds
/// 💥️ The entity row of clash sets.
pub const CLASH_SET: EntityKind = EntityKind {
    kind: "clash-set",
    icon: "zap",
    library: false,
    label: |labels| labels.kind_clash_set,
    group: |labels| labels.group_clash_sets,
    ids: |snapshot| snapshot.clash_sets.keys().cloned().collect(),
    name: |snapshot, id| snapshot.clash_sets.get(id).map(|row| row.name.clone()),
    parent: |_, _| None,
    delete: delete!(delete_clash_set::DeleteClashSet),
    rename: renaming!(set_clash_set::SetClashSet),
    create: Some(create_clash_set),
    fields: CLASH_SET_FIELDS,
    inferred: CLASH_SET_INFERRED,
};

/// ⚖️ The entity row of rules.
pub const RULE: EntityKind = EntityKind {
    kind: "rule",
    icon: "scale",
    library: false,
    label: |labels| labels.kind_rule,
    group: |labels| labels.group_rules,
    ids: |snapshot| snapshot.rules.keys().cloned().collect(),
    name: |snapshot, id| snapshot.rules.get(id).map(|row| row.name.clone()),
    parent: |_, _| None,
    delete: delete!(delete_rule::DeleteRule),
    rename: renaming!(set_rule::SetRule),
    create: Some(create_rule),
    fields: RULE_FIELDS,
    inferred: RULE_INFERRED,
};

/// 🚩️ The entity row of issues: named by their title.
pub const ISSUE: EntityKind = EntityKind {
    kind: "issue",
    icon: "flag",
    library: false,
    label: |labels| labels.kind_issue,
    group: |labels| labels.group_issues,
    ids: |snapshot| snapshot.issues.keys().cloned().collect(),
    name: |snapshot, id| snapshot.issues.get(id).map(|row| row.title.clone()),
    parent: |_, _| None,
    delete: delete!(delete_issue::DeleteIssue),
    rename: Some(|_, id, name| set!(set_issue::SetIssue, id, "title", &name.to_string())),
    create: Some(create_issue),
    fields: ISSUE_FIELDS,
    inferred: &[],
};

/// 💬️ The entity row of issue comments: named by their first words, nested under their issue.
pub const ISSUE_COMMENT: EntityKind = EntityKind {
    kind: "issue-comment",
    icon: "message-square",
    library: false,
    label: |labels| labels.kind_issue_comment,
    group: |labels| labels.group_issue_comments,
    ids: |snapshot| snapshot.issue_comments.keys().cloned().collect(),
    name: |snapshot, id| snapshot.issue_comments.get(id).map(|row| row.text.chars().take(40).collect()),
    parent: |snapshot, id| snapshot.issue_comments.get(id).map(|row| row.issue.clone()),
    delete: delete!(delete_issue_comment::DeleteIssueComment),
    rename: None,
    create: Some(create_comment),
    fields: COMMENT_FIELDS,
    inferred: &[],
};
//#endregion 🔖️Kinds

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
