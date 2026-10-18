//! 🤝️ Sparse patches of the coordination records: clash sets, rules, issues and issue comments.

use crate::standards::v1::subsets::any::schema::snapshot::{entities::*, values::*};
use crate::standards::v1::subsets::any::schema::diff::algebra::*;

/// 🩹 Sparse patch of [`ClashSet`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct ClashSetPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub a: Option<ElementSelector>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub b: Option<ElementSelector>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub tolerance: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub clearance: Option<f64>,
}

impl Patch<ClashSet> for ClashSetPatch {
    fn write(&self, base: &ClashSet) -> ClashSet {
        ClashSet { name: take(&self.name, &base.name), a: take(&self.a, &base.a), b: take(&self.b, &base.b), tolerance: take(&self.tolerance, &base.tolerance), clearance: take(&self.clearance, &base.clearance) }
    }
    fn restoring(&self, base: &ClashSet) -> Self {
        Self { name: restore(&self.name, &base.name), a: restore(&self.a, &base.a), b: restore(&self.b, &base.b), tolerance: restore(&self.tolerance, &base.tolerance), clearance: restore(&self.clearance, &base.clearance) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.a, later.a);
        merge_slot(&mut self.b, later.b);
        merge_slot(&mut self.tolerance, later.tolerance);
        merge_slot(&mut self.clearance, later.clearance);
    }
    fn minimal(&self, base: &ClashSet) -> Self {
        Self { name: changed(&self.name, &base.name), a: changed(&self.a, &base.a), b: changed(&self.b, &base.b), tolerance: changed(&self.tolerance, &base.tolerance), clearance: changed(&self.clearance, &base.clearance) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.name.is_some() {
            out.push("name".to_string());
        }
        if self.a.is_some() {
            out.push("a".to_string());
        }
        if self.b.is_some() {
            out.push("b".to_string());
        }
        if self.tolerance.is_some() {
            out.push("tolerance".to_string());
        }
        if self.clearance.is_some() {
            out.push("clearance".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`Rule`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct RulePatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub kind: Option<RuleKind>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub severity: Option<RuleSeverity>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub scope: Option<RuleScope>,
}

impl Patch<Rule> for RulePatch {
    fn write(&self, base: &Rule) -> Rule {
        Rule { name: take(&self.name, &base.name), kind: take(&self.kind, &base.kind), limit: take(&self.limit, &base.limit), severity: take(&self.severity, &base.severity), scope: take(&self.scope, &base.scope) }
    }
    fn restoring(&self, base: &Rule) -> Self {
        Self { name: restore(&self.name, &base.name), kind: restore(&self.kind, &base.kind), limit: restore(&self.limit, &base.limit), severity: restore(&self.severity, &base.severity), scope: restore(&self.scope, &base.scope) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.kind, later.kind);
        merge_slot(&mut self.limit, later.limit);
        merge_slot(&mut self.severity, later.severity);
        merge_slot(&mut self.scope, later.scope);
    }
    fn minimal(&self, base: &Rule) -> Self {
        Self { name: changed(&self.name, &base.name), kind: changed(&self.kind, &base.kind), limit: changed(&self.limit, &base.limit), severity: changed(&self.severity, &base.severity), scope: changed(&self.scope, &base.scope) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.name.is_some() {
            out.push("name".to_string());
        }
        if self.kind.is_some() {
            out.push("kind".to_string());
        }
        if self.limit.is_some() {
            out.push("limit".to_string());
        }
        if self.severity.is_some() {
            out.push("severity".to_string());
        }
        if self.scope.is_some() {
            out.push("scope".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`Issue`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct IssuePatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub status: Option<IssueStatus>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub priority: Option<IssuePriority>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub elements: Option<Vec<String>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub clash: Option<Assigned<Option<ClashRef>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub viewpoint: Option<Assigned<Option<IssueViewpoint>>>,
}

impl Patch<Issue> for IssuePatch {
    fn write(&self, base: &Issue) -> Issue {
        Issue { title: take(&self.title, &base.title), description: take(&self.description, &base.description), status: take(&self.status, &base.status), priority: take(&self.priority, &base.priority), assignee: take(&self.assignee, &base.assignee), author: take(&self.author, &base.author), created: take(&self.created, &base.created), labels: take(&self.labels, &base.labels), elements: take(&self.elements, &base.elements), clash: take_assigned(&self.clash, &base.clash), viewpoint: take_assigned(&self.viewpoint, &base.viewpoint) }
    }
    fn restoring(&self, base: &Issue) -> Self {
        Self { title: restore(&self.title, &base.title), description: restore(&self.description, &base.description), status: restore(&self.status, &base.status), priority: restore(&self.priority, &base.priority), assignee: restore(&self.assignee, &base.assignee), author: restore(&self.author, &base.author), created: restore(&self.created, &base.created), labels: restore(&self.labels, &base.labels), elements: restore(&self.elements, &base.elements), clash: restore_assigned(&self.clash, &base.clash), viewpoint: restore_assigned(&self.viewpoint, &base.viewpoint) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.title, later.title);
        merge_slot(&mut self.description, later.description);
        merge_slot(&mut self.status, later.status);
        merge_slot(&mut self.priority, later.priority);
        merge_slot(&mut self.assignee, later.assignee);
        merge_slot(&mut self.author, later.author);
        merge_slot(&mut self.created, later.created);
        merge_slot(&mut self.labels, later.labels);
        merge_slot(&mut self.elements, later.elements);
        merge_slot(&mut self.clash, later.clash);
        merge_slot(&mut self.viewpoint, later.viewpoint);
    }
    fn minimal(&self, base: &Issue) -> Self {
        Self { title: changed(&self.title, &base.title), description: changed(&self.description, &base.description), status: changed(&self.status, &base.status), priority: changed(&self.priority, &base.priority), assignee: changed(&self.assignee, &base.assignee), author: changed(&self.author, &base.author), created: changed(&self.created, &base.created), labels: changed(&self.labels, &base.labels), elements: changed(&self.elements, &base.elements), clash: changed_assigned(&self.clash, &base.clash), viewpoint: changed_assigned(&self.viewpoint, &base.viewpoint) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.title.is_some() {
            out.push("title".to_string());
        }
        if self.description.is_some() {
            out.push("description".to_string());
        }
        if self.status.is_some() {
            out.push("status".to_string());
        }
        if self.priority.is_some() {
            out.push("priority".to_string());
        }
        if self.assignee.is_some() {
            out.push("assignee".to_string());
        }
        if self.author.is_some() {
            out.push("author".to_string());
        }
        if self.created.is_some() {
            out.push("created".to_string());
        }
        if self.labels.is_some() {
            out.push("labels".to_string());
        }
        if self.elements.is_some() {
            out.push("elements".to_string());
        }
        if self.clash.is_some() {
            out.push("clash".to_string());
        }
        if self.viewpoint.is_some() {
            out.push("viewpoint".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`IssueComment`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct IssueCommentPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub issue: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl Patch<IssueComment> for IssueCommentPatch {
    fn write(&self, base: &IssueComment) -> IssueComment {
        IssueComment { issue: take(&self.issue, &base.issue), author: take(&self.author, &base.author), date: take(&self.date, &base.date), text: take(&self.text, &base.text) }
    }
    fn restoring(&self, base: &IssueComment) -> Self {
        Self { issue: restore(&self.issue, &base.issue), author: restore(&self.author, &base.author), date: restore(&self.date, &base.date), text: restore(&self.text, &base.text) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.issue, later.issue);
        merge_slot(&mut self.author, later.author);
        merge_slot(&mut self.date, later.date);
        merge_slot(&mut self.text, later.text);
    }
    fn minimal(&self, base: &IssueComment) -> Self {
        Self { issue: changed(&self.issue, &base.issue), author: changed(&self.author, &base.author), date: changed(&self.date, &base.date), text: changed(&self.text, &base.text) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.issue.is_some() {
            out.push("issue".to_string());
        }
        if self.author.is_some() {
            out.push("author".to_string());
        }
        if self.date.is_some() {
            out.push("date".to_string());
        }
        if self.text.is_some() {
            out.push("text".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}
