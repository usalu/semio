"""en1998 v9 step 9: member wall storey references are validated against buildings[].storeys[]. python -I"""
import sys; sys.path.insert(0, __file__.rsplit("/", 1)[0])
from lib import *
P = S / "💡️inferences/⚖️part-evaluators/🦀️.rs"
sub(P, '''        push_duplicate_ids(report, annex, "members", &building.members.iter().map(|m| m.id.clone()).collect::<Vec<_>>(), &|id| format!("buildings[id={bid}].members[id={id}].id"));
''', '''        push_duplicate_ids(report, annex, "members", &building.members.iter().map(|m| m.id.clone()).collect::<Vec<_>>(), &|id| format!("buildings[id={bid}].members[id={id}].id"));
        for member in building.members.iter().filter(|m| !m.wall_storey_id.is_empty() && !building.storeys.iter().any(|s| s.id == m.wall_storey_id)) {
            push_dangling_reference(report, annex, &format!("buildings[id={bid}].members"), &member.id, "wallStoreyId", "storey", building.storeys.iter().map(|s| s.id.clone()).chain([String::new()]).collect());
        }
''')
CT = S / "🧪️tests/⚖️compliance/🦀️.rs"
sub(CT, '''#[semio_framework_async_macros::async_test]
async fn v_s30_ground_class_and_joint_gap_checks_report_mismatches() {''', '''#[semio_framework_async_macros::async_test]
async fn a_wall_storey_id_must_name_a_storey_row_of_the_building_and_empty_means_every_storey() {
    let fine = details_report(|d| d.buildings[0].members[0].wall_storey_id = d.buildings[0].storeys[1].id.clone());
    assert!(!fine.checks.iter().any(|c| c.id.contains("integrity.reference") && c.id.ends_with("wallStoreyId")));
    let every = details_report(|d| d.buildings[0].members[0].wall_storey_id = String::new());
    assert!(!every.checks.iter().any(|c| c.id.contains("integrity.reference") && c.id.ends_with("wallStoreyId")));
    let dangling = details_report(|d| d.buildings[0].members[0].wall_storey_id = "s9".into());
    let check = dangling.checks.iter().find(|c| c.id.contains("integrity.reference") && c.id.ends_with("wallStoreyId")).expect("dangling reference");
    assert_eq!(check.status, CheckStatus::Fail);
    assert!(check.subject.path.contains("members[id=col-c1].wallStoreyId"), "{}", check.subject.path);
    assert!(!check.remedies.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn v_s30_ground_class_and_joint_gap_checks_report_mismatches() {''')
flush()
