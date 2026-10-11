//! ⚠️ `issues`: what can be wrong with a family, as typed values. Every issue has a stable code, the owner it belongs to (a parameter, a solid or the family) and, for a fault inside a
//! formula, the child-index path of the node (`semio_framework_expression::Expr::at`). The English `detail` is the fallback of the expression crate; the German and English messages
//! of the diagnostics are built from the code and the structured fields in [`message`].

use semio_framework_expression::{ErrorKind, ExprError, ParseError};

/// 🏷️ What kind of fault a formula or a solid has.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub enum FamilyIssueCode {
    Syntax,
    Kind,
    Cycle,
    Unknown,
    DivisionByZero,
    Negative,
    Dependency,
    Domain,
    Outline,
}

impl FamilyIssueCode {
    /// 📖️ Every code in declaration order.
    pub const ALL: [FamilyIssueCode; 9] = [Self::Syntax, Self::Kind, Self::Cycle, Self::Unknown, Self::DivisionByZero, Self::Negative, Self::Dependency, Self::Domain, Self::Outline];

    /// 🏷️ The stable kebab-case slug of the code (the tail of the diagnostic key).
    pub const fn slug(self) -> &'static str {
        match self {
            Self::Syntax => "syntax",
            Self::Kind => "kind",
            Self::Cycle => "cycle",
            Self::Unknown => "unknown",
            Self::DivisionByZero => "division-by-zero",
            Self::Negative => "negative",
            Self::Dependency => "dependency",
            Self::Domain => "domain",
            Self::Outline => "outline",
        }
    }
}

/// 🧩 What an issue belongs to.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub enum IssueOwner {
    Parameter,
    Solid,
    Family,
}

/// 🚨️ One fault of a family. `subject` is the parameter name or the solid id (empty for the family); `field` names the formula slot (`value`, `height`, `profile.width`, ...); `path` addresses the node
/// of the formula; `detail` is the English fallback text; `names` are the parameters or the material the fault is about.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct FamilyIssue {
    pub code: FamilyIssueCode,
    pub owner: IssueOwner,
    pub subject: String,
    pub field: String,
    pub path: Vec<u32>,
    pub detail: String,
    pub names: Vec<String>,
}

impl FamilyIssue {
    /// 🏗️ An issue without a node path.
    pub fn new(code: FamilyIssueCode, owner: IssueOwner, subject: &str, field: &str, detail: impl Into<String>, names: Vec<String>) -> Self {
        Self { code, owner, subject: subject.to_string(), field: field.to_string(), path: Vec::new(), detail: detail.into(), names }
    }

    /// 🚨️ The issue of a failed evaluation: the code follows the kind of the error, the path stays.
    pub fn of_error(owner: IssueOwner, subject: &str, field: &str, error: &ExprError, declared: &dyn Fn(&str) -> bool) -> Self {
        let (code, names) = match &error.kind {
            ErrorKind::UnknownParam { name } if declared(name) => (FamilyIssueCode::Dependency, vec![name.clone()]),
            ErrorKind::UnknownParam { name } => (FamilyIssueCode::Unknown, vec![name.clone()]),
            ErrorKind::UnknownOverride => (FamilyIssueCode::Unknown, vec![subject.to_string()]),
            ErrorKind::Cycle { members } => (FamilyIssueCode::Cycle, members.clone()),
            ErrorKind::FailedDependency { name } => (FamilyIssueCode::Dependency, vec![name.clone()]),
            ErrorKind::DivisionByZero => (FamilyIssueCode::DivisionByZero, Vec::new()),
            ErrorKind::Domain { .. } | ErrorKind::Overflow => (FamilyIssueCode::Domain, Vec::new()),
            _ => (FamilyIssueCode::Kind, Vec::new()),
        };
        Self { code, owner, subject: subject.to_string(), field: field.to_string(), path: error.path.iter().map(|step| *step as u32).collect(), detail: error.kind.message(), names }
    }

    /// 🔤️ The issue of a formula that does not parse.
    pub fn of_parse(owner: IssueOwner, subject: &str, field: &str, error: &ParseError) -> Self {
        Self { code: FamilyIssueCode::Syntax, owner, subject: subject.to_string(), field: field.to_string(), path: Vec::new(), detail: error.message(), names: Vec::new() }
    }
}

/// 💬️ The message of an issue in `locale` (`en` or `de`): what is wrong and where.
pub fn message(issue: &FamilyIssue, locale: &str) -> String {
    let german = locale == "de";
    let place = match (issue.owner, german) {
        (IssueOwner::Parameter, false) => format!("parameter \"{}\"", issue.subject),
        (IssueOwner::Parameter, true) => format!("Parameter \"{}\"", issue.subject),
        (IssueOwner::Solid, false) => format!("solid \"{}\", {}", issue.subject, issue.field),
        (IssueOwner::Solid, true) => format!("Körper \"{}\", {}", issue.subject, issue.field),
        (IssueOwner::Family, false) => "the family".to_string(),
        (IssueOwner::Family, true) => "die Familie".to_string(),
    };
    let named = issue.names.join(", ");
    let what = match (issue.code, german) {
        (FamilyIssueCode::Syntax, false) => format!("the formula does not parse ({})", issue.detail),
        (FamilyIssueCode::Syntax, true) => format!("die Formel lässt sich nicht lesen ({})", issue.detail),
        (FamilyIssueCode::Kind, false) => format!("the formula mixes kinds ({})", issue.detail),
        (FamilyIssueCode::Kind, true) => format!("die Formel mischt Größenarten ({})", issue.detail),
        (FamilyIssueCode::Cycle, false) => format!("circular reference between {named}"),
        (FamilyIssueCode::Cycle, true) => format!("Zirkelbezug zwischen {named}"),
        (FamilyIssueCode::Unknown, false) => format!("unknown name {named}"),
        (FamilyIssueCode::Unknown, true) => format!("unbekannter Name {named}"),
        (FamilyIssueCode::DivisionByZero, false) => "division by zero".to_string(),
        (FamilyIssueCode::DivisionByZero, true) => "Division durch null".to_string(),
        (FamilyIssueCode::Negative, false) => format!("a dimension must be positive ({})", issue.detail),
        (FamilyIssueCode::Negative, true) => format!("eine Abmessung muss positiv sein ({})", issue.detail),
        (FamilyIssueCode::Dependency, false) => format!("depends on {named} which has no value"),
        (FamilyIssueCode::Dependency, true) => format!("hängt von {named} ab, das keinen Wert hat"),
        (FamilyIssueCode::Domain, false) => format!("the formula has no finite value ({})", issue.detail),
        (FamilyIssueCode::Domain, true) => format!("die Formel hat keinen endlichen Wert ({})", issue.detail),
        (FamilyIssueCode::Outline, false) => format!("the outline is not usable ({})", issue.detail),
        (FamilyIssueCode::Outline, true) => format!("die Kontur ist nicht brauchbar ({})", issue.detail),
    };
    format!("{place}: {what}")
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
