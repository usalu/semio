//! 🧪️ Ordered-collection law fixtures: removing or inserting a MIDDLE row restores that row at its original index, and the
//! inverse diffs sum to the negative of the forward diff, for every list the document owns.

use super::*;
use crate::standards::v1::subsets::any::io::{apply_en1995_mutation, inverse_en1995_mutation};

fn three_row_document() -> En1995Snapshot {
    let mut base = En1995Snapshot::compliant_building_beam();
    let member = base.members[0].clone();
    for id in ["beam-b", "beam-c"] {
        let mut copy = member.clone();
        copy.id = id.into();
        base.members.push(copy);
    }
    for member in &mut base.members {
        let template = member.actions.first().cloned().expect("the fixture member carries an action");
        for id in ["a2", "a3"] {
            let mut action = template.clone();
            action.id = id.into();
            member.actions.push(action);
        }
    }
    if let Some(connection) = base.connections.first().cloned() {
        for id in ["conn-b", "conn-c"] {
            let mut copy = connection.clone();
            copy.id = id.into();
            base.connections.push(copy);
        }
        for connection in &mut base.connections {
            if let Some(template) = connection.actions.first().cloned() {
                for id in ["ca2", "ca3"] {
                    let mut action = template.clone();
                    action.id = id.into();
                    connection.actions.push(action);
                }
            }
        }
    }
    base
}

#[semio_framework_async_macros::async_test]
async fn removing_a_middle_row_obeys_the_inverse_sum_law() {
    let base = three_row_document();
    let member_id = base.members[1].id.clone();
    let mut removals = vec![
        En1995Mutation::RemoveMember(remove_member::RemoveMember { index: 1 }),
        En1995Mutation::RemoveMemberAction(remove_member_action::RemoveMemberAction { member_id, index: 1 }),
    ];
    if base.connections.len() >= 3 {
        removals.push(En1995Mutation::RemoveConnection(remove_connection::RemoveConnection { index: 1 }));
        if base.connections[1].actions.len() >= 3 {
            removals.push(En1995Mutation::RemoveConnectionAction(remove_connection_action::RemoveConnectionAction { connection_id: base.connections[1].id.clone(), index: 1 }));
        }
    }
    for mutation in &removals {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(mutation, &base).await;
    }
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_row_obeys_the_inverse_sum_law() {
    let base = three_row_document();
    let mut member = base.members[0].clone();
    member.id = "beam-new".into();
    let mut action = base.members[1].actions[0].clone();
    action.id = "a-new".into();
    let insertions = [
        En1995Mutation::InsertMember(insert_member::InsertMember { index: 1, member }),
        En1995Mutation::InsertMemberAction(insert_member_action::InsertMemberAction { member_id: base.members[1].id.clone(), index: 1, action }),
    ];
    for mutation in &insertions {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(mutation, &base).await;
    }
}

#[test]
fn a_removed_middle_row_returns_to_its_original_index() {
    let base = three_row_document();
    let removal = En1995Mutation::RemoveMember(remove_member::RemoveMember { index: 1 });
    let (after, _) = apply_en1995_mutation(&base, &removal).expect("removal applies");
    let restored = inverse_en1995_mutation(&removal, &base).expect("inverse").iter().fold(after, |document, step| apply_en1995_mutation(&document, step).expect("inverse applies").0);
    assert_eq!(restored, base);
    assert_eq!(restored.members[1].id, "beam-b");
}
