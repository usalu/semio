//! 🧭️ Position-exact inverses: removing or inserting a MIDDLE row restores the document exactly, and the inverse names the original index.

use super::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;
use protocol::Mutation as _;

fn pad<T: Clone>(rows: &mut Vec<T>, rename: impl Fn(&mut T, String)) {
    while rows.len() < 3 {
        let mut row = rows[0].clone();
        rename(&mut row, format!("pad-{}", rows.len()));
        rows.push(row);
    }
}

fn padded() -> En1990Snapshot {
    let mut base = En1990Snapshot::default();
    if base.accidentals.is_empty() {
        base.accidentals.push(crate::AccidentalAction { id: "A0".into(), ad: 1.0 });
    }
    if base.seismics.is_empty() {
        base.seismics.push(crate::SeismicAction { id: "S0".into(), a_ek: 1.0, importance_class: crate::ImportanceClass::I });
    }
    pad(&mut base.permanents, |row, id| row.id = id);
    pad(&mut base.variables, |row, id| row.id = id);
    pad(&mut base.accidentals, |row, id| row.id = id);
    pad(&mut base.seismics, |row, id| row.id = id);
    pad(&mut base.members, |row, id| row.id = id);
    pad(&mut base.effects, |_, _| ());
    base
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_permanents_row_is_undone_at_its_index() {
    let base = padded();
    let remove = En1990Mutation::RemovePermanent(remove_permanent::RemovePermanent { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1990Mutation::InsertPermanent(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_permanents_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = En1990Mutation::InsertPermanent(insert_permanent::InsertPermanent { index: Some(1), item: { let mut row = base.permanents[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_permanents_row() {
    let base = padded();
    let insert = En1990Mutation::InsertPermanent(insert_permanent::InsertPermanent { index: None, item: { let mut row = base.permanents[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1990Mutation::RemovePermanent(remove)] if remove.index == base.permanents.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_variables_row_is_undone_at_its_index() {
    let base = padded();
    let remove = En1990Mutation::RemoveVariable(remove_variable::RemoveVariable { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1990Mutation::InsertVariable(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_variables_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = En1990Mutation::InsertVariable(insert_variable::InsertVariable { index: Some(1), item: { let mut row = base.variables[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_variables_row() {
    let base = padded();
    let insert = En1990Mutation::InsertVariable(insert_variable::InsertVariable { index: None, item: { let mut row = base.variables[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1990Mutation::RemoveVariable(remove)] if remove.index == base.variables.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_accidentals_row_is_undone_at_its_index() {
    let base = padded();
    let remove = En1990Mutation::RemoveAccidental(remove_accidental::RemoveAccidental { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1990Mutation::InsertAccidental(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_accidentals_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = En1990Mutation::InsertAccidental(insert_accidental::InsertAccidental { index: Some(1), item: { let mut row = base.accidentals[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_accidentals_row() {
    let base = padded();
    let insert = En1990Mutation::InsertAccidental(insert_accidental::InsertAccidental { index: None, item: { let mut row = base.accidentals[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1990Mutation::RemoveAccidental(remove)] if remove.index == base.accidentals.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_seismics_row_is_undone_at_its_index() {
    let base = padded();
    let remove = En1990Mutation::RemoveSeismic(remove_seismic::RemoveSeismic { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1990Mutation::InsertSeismic(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_seismics_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = En1990Mutation::InsertSeismic(insert_seismic::InsertSeismic { index: Some(1), item: { let mut row = base.seismics[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_seismics_row() {
    let base = padded();
    let insert = En1990Mutation::InsertSeismic(insert_seismic::InsertSeismic { index: None, item: { let mut row = base.seismics[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1990Mutation::RemoveSeismic(remove)] if remove.index == base.seismics.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_members_row_is_undone_at_its_index() {
    let base = padded();
    let remove = En1990Mutation::RemoveMember(remove_member::RemoveMember { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1990Mutation::InsertMember(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_members_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = En1990Mutation::InsertMember(insert_member::InsertMember { index: Some(1), item: { let mut row = base.members[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_members_row() {
    let base = padded();
    let insert = En1990Mutation::InsertMember(insert_member::InsertMember { index: None, item: { let mut row = base.members[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1990Mutation::RemoveMember(remove)] if remove.index == base.members.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_effects_row_is_undone_at_its_index() {
    let base = padded();
    let remove = En1990Mutation::RemoveEffect(remove_effect::RemoveEffect { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1990Mutation::InsertEffect(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_effects_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = En1990Mutation::InsertEffect(insert_effect::InsertEffect { index: Some(1), item: base.effects[0].clone() });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_effects_row() {
    let base = padded();
    let insert = En1990Mutation::InsertEffect(insert_effect::InsertEffect { index: None, item: base.effects[0].clone() });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1990Mutation::RemoveEffect(remove)] if remove.index == base.effects.len()), "the inverse must remove the appended row: {inverse:?}");
}
