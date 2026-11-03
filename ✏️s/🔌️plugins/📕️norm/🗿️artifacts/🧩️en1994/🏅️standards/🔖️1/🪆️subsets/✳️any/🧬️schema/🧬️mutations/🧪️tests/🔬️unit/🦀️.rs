use crate::*;
use protocol::{ColdMutation, Mutation, SemanticMutation};

fn round_trip(base: &En1994Snapshot, operation: &En1994Mutation) -> En1994Snapshot {
    let (forward, _messages) = protocol::apply_mutation(base, operation).expect("valid mutation");
    let mut restored = forward.clone();
    for back in operation.inverse(base).expect("valid retained mutation inverse fixture") {
        let (next, _messages) = protocol::apply_mutation(&restored, &back).expect("valid inverse mutation");
        restored = next;
    }
    assert_eq!(&restored, base, "inverse(base) must restore the pre-mutation document");
    forward
}

#[test]
fn every_kind_round_trips_through_inverse() {
    let base = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    for op in demo_mutation_cases() {
        let _ = round_trip(&base, &op);
    }
}


#[test]
fn removing_a_beam_removes_its_stud_rows_in_the_same_batch_and_the_inverse_restores_them() {
    let base = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let beam = base.beams[0].id.clone();
    assert!(base.studs.iter().any(|stud| stud.beam_id == beam));
    let forward = round_trip(&base, &En1994Mutation::RemoveBeam(remove_beam::RemoveBeam { id: beam.clone() }));
    assert!(forward.beams.iter().all(|row| row.id != beam));
    assert!(forward.studs.iter().all(|stud| stud.beam_id != beam));
    assert_eq!(forward.studs.len(), base.studs.iter().filter(|stud| stud.beam_id != beam).count());
}

#[test]
fn nested_action_kinds_insert_at_a_position_and_remove_by_id() {
    let base = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    for (parent, count, first) in [(base.beams[0].id.clone(), base.beams[0].actions.len(), base.beams[0].actions[0].id.clone()), (base.columns[0].id.clone(), base.columns[0].actions.len(), base.columns[0].actions[0].id.clone()), (base.slabs[0].id.clone(), base.slabs[0].actions.len(), base.slabs[0].actions[0].id.clone())] {
        let (insert, remove): (En1994Mutation, En1994Mutation) = if parent == base.beams[0].id {
            let mut action = base.beams[0].actions[0].clone();
            action.id = "A-new".into();
            (En1994Mutation::InsertBeamAction(insert_beam_action::InsertBeamAction { beam_id: parent.clone(), index: 1, action }), En1994Mutation::RemoveBeamAction(remove_beam_action::RemoveBeamAction { beam_id: parent.clone(), id: first.clone() }))
        } else if parent == base.columns[0].id {
            let mut action = base.columns[0].actions[0].clone();
            action.id = "A-new".into();
            (En1994Mutation::InsertColumnAction(insert_column_action::InsertColumnAction { column_id: parent.clone(), index: 1, action }), En1994Mutation::RemoveColumnAction(remove_column_action::RemoveColumnAction { column_id: parent.clone(), id: first.clone() }))
        } else {
            let mut action = base.slabs[0].actions[0].clone();
            action.id = "A-new".into();
            (En1994Mutation::InsertSlabAction(insert_slab_action::InsertSlabAction { slab_id: parent.clone(), index: 1, action }), En1994Mutation::RemoveSlabAction(remove_slab_action::RemoveSlabAction { slab_id: parent.clone(), id: first.clone() }))
        };
        let inserted = round_trip(&base, &insert);
        let removed = round_trip(&base, &remove);
        let ids = |snapshot: &En1994Snapshot| -> Vec<String> {
            if parent == base.beams[0].id {
                snapshot.beams[0].actions.iter().map(|action| action.id.clone()).collect()
            } else if parent == base.columns[0].id {
                snapshot.columns[0].actions.iter().map(|action| action.id.clone()).collect()
            } else {
                snapshot.slabs[0].actions.iter().map(|action| action.id.clone()).collect()
            }
        };
        assert_eq!(ids(&inserted).len(), count + 1);
        assert_eq!(ids(&inserted)[1], "A-new");
        assert_eq!(ids(&removed).len(), count - 1);
        assert!(!ids(&removed).contains(&first));
    }
}

#[test]
fn nested_action_kinds_refuse_unknown_parents_duplicates_and_unknown_ids() {
    let base = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let beam = base.beams[0].id.clone();
    let first = base.beams[0].actions[0].clone();
    let refused = |operation: En1994Mutation| !ColdMutation::diff(&operation, &base).messages().is_empty();
    assert!(refused(En1994Mutation::RemoveBeamAction(remove_beam_action::RemoveBeamAction { beam_id: "no-beam".into(), id: first.id.clone() })));
    assert!(refused(En1994Mutation::RemoveBeamAction(remove_beam_action::RemoveBeamAction { beam_id: beam.clone(), id: "no-action".into() })));
    assert!(refused(En1994Mutation::InsertBeamAction(insert_beam_action::InsertBeamAction { beam_id: beam.clone(), index: 0, action: first.clone() })));
    assert!(refused(En1994Mutation::InsertBeamAction(insert_beam_action::InsertBeamAction { beam_id: beam, index: base.beams[0].actions.len() + 1, action: semio_s_artifact_norm_en1994_model::CharacteristicAction { id: "A-late".into(), ..first } })));
}

#[test]
fn nested_action_edits_resolve_to_the_id_addressed_kinds() {
    use semio_s_plugin_norm_app_surface::NormEdit;
    let base = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let beam = base.beams[0].id.clone();
    let first = base.beams[0].actions[0].clone();
    let removal = resolve_edit(&base, &NormEdit::RemoveItem { path: format!("beams[id={beam}].actions"), index: 0 }).expect("removal resolves");
    assert_eq!(removal, vec![En1994Mutation::RemoveBeamAction(remove_beam_action::RemoveBeamAction { beam_id: beam.clone(), id: first.id.clone() })]);
    let mut added = first;
    added.id = "A-new".into();
    let insertion = resolve_edit(&base, &NormEdit::InsertItem { path: format!("beams[id={beam}].actions"), index: 1, value: Some(semio_framework_value::ToValue::to_value(&added)) }).expect("insertion resolves");
    assert_eq!(insertion, vec![En1994Mutation::InsertBeamAction(insert_beam_action::InsertBeamAction { beam_id: beam, index: 1, action: added })]);
}
