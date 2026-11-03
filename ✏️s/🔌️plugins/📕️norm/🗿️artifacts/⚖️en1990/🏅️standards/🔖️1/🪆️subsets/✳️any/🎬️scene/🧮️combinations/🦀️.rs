//! 🧮️ Combinations: the generated EN 1990 combinations of the whole structure as matrix columns (family, equation, leading action, γ·ψ per action), the active column and the link from a check to its column.
//!
//! The factors do not depend on the member influences, so the columns are generated once from the unit-influence action list of the document; the generator is the one the compliance report uses.

use super::checks::{combinations, offers_610ab, Action, ActionClass, Combination, Equation, FactorSet};
use super::inferences::{actions_with, context_for, member_actions};
use super::model::En1990Snapshot;
use super::norm::CheckResult;

/// 🏷️ The limit-state family of a matrix column.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Family {
    Str,
    Equ,
    Acc,
    Sei,
    SlsChar,
    SlsFreq,
    SlsQp,
}

impl Family {
    pub const ALL: [Self; 7] = [Self::Str, Self::Equ, Self::Acc, Self::Sei, Self::SlsChar, Self::SlsFreq, Self::SlsQp];

    pub fn key(self) -> &'static str {
        match self {
            Self::Str => "str",
            Self::Equ => "equ",
            Self::Acc => "acc",
            Self::Sei => "sei",
            Self::SlsChar => "sls-char",
            Self::SlsFreq => "sls-freq",
            Self::SlsQp => "sls-qp",
        }
    }

    pub fn title(self) -> (&'static str, &'static str) {
        match self {
            Self::Str => ("STR/GEO", "STR/GEO"),
            Self::Equ => ("EQU", "EQU"),
            Self::Acc => ("Accidental", "Außergewöhnlich"),
            Self::Sei => ("Seismic", "Erdbeben"),
            Self::SlsChar => ("SLS characteristic", "GZG charakteristisch"),
            Self::SlsFreq => ("SLS frequent", "GZG häufig"),
            Self::SlsQp => ("SLS quasi-permanent", "GZG quasi-ständig"),
        }
    }

    fn set(self) -> FactorSet {
        if self == Self::Equ { FactorSet::EquA } else { FactorSet::StrB }
    }

    fn equations(self, offers_ab: bool) -> Vec<Equation> {
        match self {
            Self::Str if offers_ab => vec![Equation::E610a, Equation::E610b],
            Self::Str | Self::Equ => vec![Equation::E610],
            Self::Acc => vec![Equation::E611b],
            Self::Sei => vec![Equation::E612b],
            Self::SlsChar => vec![Equation::Characteristic],
            Self::SlsFreq => vec![Equation::Frequent],
            Self::SlsQp => vec![Equation::QuasiPermanent],
        }
    }

    /// 🔗️ The family a check id belongs to; fatigue is verified on the frequent combination.
    pub fn of_check(id: &str) -> Option<Self> {
        let rest = id.strip_prefix("en1990.")?;
        if rest.starts_with("str.") || rest.starts_with("geo.") {
            Some(Self::Str)
        } else if rest.starts_with("equ.") {
            Some(Self::Equ)
        } else if rest.starts_with("6.11b.") {
            Some(Self::Acc)
        } else if rest.starts_with("6.12b.") {
            Some(Self::Sei)
        } else if rest.starts_with("sls.") {
            [(Equation::Characteristic, Self::SlsChar), (Equation::Frequent, Self::SlsFreq), (Equation::QuasiPermanent, Self::SlsQp)].into_iter().find(|(eq, _)| id.ends_with(eq.label())).map(|(_, family)| family)
        } else if rest.starts_with("fat.") {
            Some(Self::SlsFreq)
        } else {
            None
        }
    }
}

/// 🧾️ One matrix column: a generated combination with its stable id `family:equation:leading`.
#[derive(Clone, Debug, PartialEq)]
pub struct Column {
    pub id: String,
    pub family: Family,
    pub combination: Combination,
}

impl Column {
    /// 🔢️ The total factor γ·ψ (with K_FI) an action carries in this column.
    pub fn factor(&self, action: &str) -> Option<f64> {
        let factors: Vec<f64> = self.combination.terms.iter().filter(|t| t.action == action).map(|t| t.factor).collect();
        (!factors.is_empty()).then(|| factors.iter().sum())
    }

    /// 🎚️ Whether a project factor override acts on this action in this column (the factor deviates from the National Annex).
    pub fn deviates(&self, action: &str) -> bool {
        self.combination.terms.iter().any(|t| t.action == action && t.deviates)
    }

    pub fn leading(&self) -> &str {
        if self.combination.leading.is_empty() { "-" } else { &self.combination.leading }
    }

    pub fn is_leading(&self, action: &str) -> bool {
        self.combination.leading.split('+').any(|lead| lead == action)
    }

    pub fn equation(&self) -> Equation {
        self.combination.equation
    }
}

/// ⚡️ The actions of the whole structure with unit influence: the value of an action is its characteristic or design magnitude.
pub fn structure_actions(document: &En1990Snapshot) -> Vec<Action> {
    actions_with(document, |_| 1.0)
}

/// 🧮️ Every generated combination of the structure, family by family, in generation order; empty when the document vocabulary is invalid.
pub fn columns(document: &En1990Snapshot) -> Vec<Column> {
    let actions = structure_actions(document);
    let mut out: Vec<Column> = Vec::new();
    for family in Family::ALL {
        let Some(ctx) = context_for(document, family.set()) else { continue };
        for equation in family.equations(offers_610ab(document.annex, ctx.domain)) {
            for combination in combinations(&ctx, &actions, equation).unwrap_or_default() {
                let lead = if combination.leading.is_empty() { "-".to_string() } else { combination.leading.clone() };
                let group = if combination.group.is_empty() { String::new() } else { format!("@{}", combination.group) };
                let mut id = format!("{}:{}:{lead}{group}", family.key(), equation.label());
                let mut n = 1;
                while out.iter().any(|c| c.id == id) {
                    n += 1;
                    id = format!("{}:{}:{lead}{group}#{n}", family.key(), equation.label());
                }
                out.push(Column { id, family, combination });
            }
        }
    }
    out
}

/// 🎯️ The requested column, else the governing STR/GEO column (largest total effect), else the first column.
pub fn active<'a>(columns: &'a [Column], requested: Option<&str>) -> Option<&'a Column> {
    requested
        .and_then(|id| columns.iter().find(|c| c.id == id))
        .or_else(|| columns.iter().filter(|c| c.family == Family::Str).max_by(|a, b| a.combination.total().total_cmp(&b.combination.total())))
        .or_else(|| columns.first())
}

/// 🔗️ The column a check was evaluated on: family from the check id, equation and leading action from the generator description in the explanation.
pub fn column_for_check<'a>(columns: &'a [Column], check: &CheckResult) -> Option<&'a Column> {
    let family = Family::of_check(&check.id)?;
    let text = &check.explanation.en;
    let parsed = text.find("leading=").and_then(|at| {
        let lead: String = text[at + "leading=".len()..].chars().take_while(|c| !c.is_whitespace() && *c != ')').collect();
        let label = text[..at].rsplit(|c: char| c == ' ' || c == '(').find(|part| !part.is_empty()).map(str::to_string)?;
        Some((label, lead))
    });
    let in_family = || columns.iter().filter(|c| c.family == family);
    parsed.and_then(|(label, lead)| in_family().find(|c| c.equation().label() == label && c.leading() == lead)).or_else(|| in_family().next())
}

/// 🔗️ The id of the column a check was evaluated on, for the editor: selecting a check selects its combination.
pub fn id_for_check(document: &En1990Snapshot, check: &CheckResult) -> Option<String> {
    column_for_check(&columns(document), check).map(|c| c.id.clone())
}

/// 🔤️ The ids of all columns, the selectable combinations of the editor.
pub fn ids(document: &En1990Snapshot) -> Vec<String> {
    columns(document).into_iter().map(|c| c.id).collect()
}

/// 🪜️ The permanent factor γ_G of every permanent action of a member in every column (EN 1990 A1.3.1: each permanent action takes γ_G,sup where its effect on the member adds in the direction of the combination and γ_G,inf where it relieves): one row per permanent action with an effect on the member, in table order, one value per column. A column the member's own generation does not contain falls back to the factor of the unit-influence column.
pub fn permanent_factors(document: &En1990Snapshot, member_id: &str, columns: &[Column]) -> Vec<(String, Vec<f64>)> {
    let actions = member_actions(document, member_id);
    let permanents: Vec<&Action> = actions.iter().filter(|a| a.class == ActionClass::Permanent && a.value.abs() > 1e-12).collect();
    if permanents.is_empty() {
        return Vec::new();
    }
    let mut generated: Vec<(String, Vec<Combination>)> = Vec::new();
    let per_column: Vec<Vec<f64>> = columns
        .iter()
        .map(|column| {
            let key = format!("{}:{}", column.family.key(), column.equation().label());
            let at = generated.iter().position(|(k, _)| *k == key).unwrap_or_else(|| {
                let list = context_for(document, column.family.set()).and_then(|ctx| combinations(&ctx, &actions, column.equation()).ok()).unwrap_or_default();
                generated.push((key, list));
                generated.len() - 1
            });
            let found = generated[at].1.iter().find(|c| c.leading == column.combination.leading && c.group == column.combination.group);
            permanents.iter().map(|a| found.map_or_else(|| column.factor(&a.id).unwrap_or(0.0), |c| c.terms.iter().filter(|t| t.action == a.id).map(|t| t.factor).sum::<f64>())).collect()
        })
        .collect();
    permanents.iter().enumerate().map(|(i, a)| (a.id.clone(), per_column.iter().map(|values| values[i]).collect())).collect()
}
