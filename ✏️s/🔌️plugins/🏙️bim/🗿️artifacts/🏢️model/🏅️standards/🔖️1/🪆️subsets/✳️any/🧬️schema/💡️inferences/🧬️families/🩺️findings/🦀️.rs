//! 🩺️ `findings`: what the diagnostics say about families. Every issue of a family value is one finding (the family, the parameter or solid it belongs to, the names it is about), and a `Profile::Family` that names a
//! family which does not exist or is not of category `Profile` is a dangling profile reference of the column type, beam type, curtain wall type, wall sweep or railing that holds it. The module is
//! free of the diagnostic vocabulary: `compute` maps a [`FindingCode`] to its `DiagnosticCode`.

use super::{FamilyIssueCode, FamilyValue};
use crate::{FamilyCategory, ModelSnapshot, Profile};
use semio_framework_value::DslValue;
use std::collections::{BTreeMap, BTreeSet};

/// 🏷️ What a finding is about.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FindingCode {
    Issue(FamilyIssueCode),
    ProfileReference,
}

/// 🩺️ One finding: `elements` are the family and its parameter or solid (or the holder of a profile), `missing` the names it is about.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding {
    pub code: FindingCode,
    pub elements: Vec<String>,
    pub missing: Vec<String>,
}

/// 🚨️ The findings of the issues of the families, in family, then issue order.
pub fn issue_findings(families: &BTreeMap<&str, &FamilyValue>) -> Vec<Finding> {
    families
        .iter()
        .flat_map(|(id, family)| {
            family.issues.iter().map(move |issue| {
                let mut elements = vec![(*id).to_string()];
                if !issue.subject.is_empty() {
                    elements.push(issue.subject.clone());
                }
                Finding { code: FindingCode::Issue(issue.code), elements, missing: issue.names.clone() }
            })
        })
        .collect()
}

fn named(profile: &Profile) -> Option<&str> {
    super::family_of_profile(profile)
}

/// 🔗️ The families the profiles of the model name, by the id of the holder: a column type, a beam type, a curtain wall type, a wall sweep or a railing.
pub fn profile_holders(snapshot: &ModelSnapshot) -> BTreeMap<String, BTreeSet<String>> {
    let mut holders: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut hold = |id: &String, profiles: &[&Profile]| {
        let families: BTreeSet<String> = profiles.iter().filter_map(|profile| named(profile)).map(str::to_string).collect();
        if !families.is_empty() {
            holders.entry(id.clone()).or_default().extend(families);
        }
    };
    snapshot.column_types.iter().for_each(|(id, row)| hold(id, &[&row.profile]));
    snapshot.beam_types.iter().for_each(|(id, row)| hold(id, &[&row.profile]));
    snapshot.curtain_wall_types.iter().for_each(|(id, row)| hold(id, &[&row.interior_mullion, &row.border_mullion]));
    snapshot.wall_sweeps.iter().for_each(|(id, row)| hold(id, &[&row.profile]));
    snapshot.railings.iter().for_each(|(id, row)| hold(id, &[&row.profile, &row.post_profile]));
    snapshot.railings.iter().filter_map(|(id, row)| row.baluster.as_ref().map(|baluster| (id, &baluster.profile))).for_each(|(id, profile)| hold(id, &[profile]));
    holders
}

/// 🔗️ The dangling profile references: a holder whose family does not exist or is no profile family.
pub fn reference_findings(snapshot: &ModelSnapshot, families: &BTreeMap<&str, &FamilyValue>) -> Vec<Finding> {
    let usable = |id: &str| families.get(id).is_some_and(|family| family.category == Some(FamilyCategory::Profile));
    profile_holders(snapshot)
        .into_iter()
        .flat_map(|(holder, named)| named.into_iter().filter(|family| !usable(family)).map(move |family| Finding { code: FindingCode::ProfileReference, elements: vec![holder.clone()], missing: vec![family] }))
        .collect()
}

/// 🔑️ What `reference_findings` reads of the snapshot: the profile references of the holders.
pub fn reference_dependency(snapshot: &ModelSnapshot) -> DslValue {
    DslValue::object(profile_holders(snapshot).into_iter().map(|(holder, named)| (holder, super::dep_value(&named.into_iter().collect::<Vec<_>>()))))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
