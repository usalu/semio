use super::*;
use super::super::tests::kit;
use super::super::family_of;
use crate::{BeamType, ColumnType, FamilyCategory, ParameterKind};

fn values(snapshot: &ModelSnapshot) -> BTreeMap<String, FamilyValue> {
    snapshot.families.keys().map(|id| (id.clone(), family_of(snapshot, id, &BTreeMap::new()))).collect()
}

fn view(table: &BTreeMap<String, FamilyValue>) -> BTreeMap<&str, &FamilyValue> {
    table.iter().map(|(id, value)| (id.as_str(), value)).collect()
}

#[test]
fn every_issue_is_one_finding_naming_the_family_and_the_part() {
    let mut snapshot = kit::model();
    kit::family(&mut snapshot, "fam-table", FamilyCategory::Furniture);
    kit::parameter(&mut snapshot, "fam-table", "top", ParameterKind::Length, "ghost + 1 m");
    kit::parameter(&mut snapshot, "fam-table", "broken", ParameterKind::Length, "2 *");
    kit::family(&mut snapshot, "fam-flat", FamilyCategory::Profile);
    let table = values(&snapshot);
    let found = issue_findings(&view(&table));
    let codes: Vec<(FindingCode, Vec<String>)> = found.iter().map(|finding| (finding.code, finding.elements.clone())).collect();
    assert!(codes.contains(&(FindingCode::Issue(FamilyIssueCode::Unknown), vec!["fam-table".to_string(), "top".to_string()])));
    assert!(codes.contains(&(FindingCode::Issue(FamilyIssueCode::Syntax), vec!["fam-table".to_string(), "broken".to_string()])));
    assert!(codes.contains(&(FindingCode::Issue(FamilyIssueCode::Outline), vec!["fam-flat".to_string()])), "a family-level issue names only the family");
    assert_eq!(found.iter().find(|finding| finding.code == FindingCode::Issue(FamilyIssueCode::Unknown)).map(|finding| finding.missing.clone()), Some(vec!["ghost".to_string()]));
    assert_eq!(found.len(), 3);
}

#[test]
fn a_profile_that_names_a_missing_or_non_profile_family_is_a_dangling_reference() {
    let mut snapshot = kit::model();
    kit::hea(&mut snapshot);
    kit::family(&mut snapshot, "fam-table", FamilyCategory::Furniture);
    let family = |id: &str| Profile::Family { family: id.to_string() };
    snapshot.column_types.insert("ct-ok".into(), ColumnType { name: "ok".into(), profile: family("fam-hea"), material: "m-steel".into() });
    snapshot.column_types.insert("ct-ghost".into(), ColumnType { name: "ghost".into(), profile: family("fam-ghost"), material: "m-steel".into() });
    snapshot.beam_types.insert("bt-table".into(), BeamType { name: "table".into(), profile: family("fam-table"), material: "m-steel".into() });
    snapshot.beam_types.insert("bt-plain".into(), BeamType { name: "plain".into(), profile: Profile::Rectangle { width: 0.2, depth: 0.3 }, material: "m-steel".into() });
    let table = values(&snapshot);
    let found = reference_findings(&snapshot, &view(&table));
    let rows: Vec<(Vec<String>, Vec<String>)> = found.iter().map(|finding| (finding.elements.clone(), finding.missing.clone())).collect();
    assert_eq!(rows, vec![(vec!["bt-table".to_string()], vec!["fam-table".to_string()]), (vec!["ct-ghost".to_string()], vec!["fam-ghost".to_string()])]);
    assert!(found.iter().all(|finding| finding.code == FindingCode::ProfileReference));
}

#[test]
fn the_holders_are_the_types_and_railings_that_name_a_family() {
    let mut snapshot = kit::model();
    assert!(profile_holders(&snapshot).is_empty());
    let named = Profile::Family { family: "fam-hea".into() };
    snapshot.column_types.insert("ct".into(), ColumnType { name: "c".into(), profile: named.clone(), material: "m-steel".into() });
    snapshot.beam_types.insert("bt".into(), BeamType { name: "b".into(), profile: Profile::Circle { diameter: 0.1 }, material: "m-steel".into() });
    let holders = profile_holders(&snapshot);
    assert_eq!(holders.keys().collect::<Vec<_>>(), ["ct"]);
    assert_eq!(holders["ct"], BTreeSet::from(["fam-hea".to_string()]));
}

#[test]
fn the_reference_dependency_changes_exactly_with_the_references() {
    let mut snapshot = kit::model();
    let named = |id: &str| Profile::Family { family: id.to_string() };
    snapshot.column_types.insert("ct".into(), ColumnType { name: "c".into(), profile: named("fam-a"), material: "m-steel".into() });
    let before = reference_dependency(&snapshot);
    snapshot.column_types.get_mut("ct").expect("ct").name = "renamed".into();
    assert_eq!(reference_dependency(&snapshot), before, "a name is not a reference");
    snapshot.column_types.get_mut("ct").expect("ct").profile = named("fam-b");
    assert_ne!(reference_dependency(&snapshot), before);
}
