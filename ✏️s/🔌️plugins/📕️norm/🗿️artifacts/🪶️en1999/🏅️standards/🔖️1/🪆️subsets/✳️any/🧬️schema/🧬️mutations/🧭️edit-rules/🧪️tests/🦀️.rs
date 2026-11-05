//! 🪶️ Complete editor payloads exercise actual EN 1999 typed mutations and preserve their coupled fields.

use super::*;
use semio_framework_value::{DslValue, ToValue};

fn json(document: &En1999Snapshot) -> serde_json::Value { serde_json::Value::from(&document.to_value()) }

#[test]
fn buckling_z_uses_the_declared_axis_and_preserves_y_and_t() {
    let document = En1999Snapshot::full_showcase();
    let original = json(&document);
    let member = document.members.first().expect("showcase member");
    let value = member.buckling_length_z + 0.5;
    let operations = resolve_edit(&document, &NormEdit::SetField { path: format!("members[id={}].bucklingLengthZ", member.id), value: DslValue::float(value) }).expect("complete buckling payload");
    let [En1999Mutation::ChangeMemberBucklingLength(payload)] = operations.as_slice() else { panic!("canonical buckling mutation") };
    assert_eq!(payload.member_id, member.id);
    assert_eq!(payload.axis, "z");
    assert_eq!(payload.new_length, value);
    let (changed, messages) = crate::protocol::apply_mutation(&document, &operations[0]).expect("typed buckling application");
    assert!(messages.is_empty(), "{messages:?}");
    let mut expected = original.clone();
    *expected.pointer_mut("/members/0/bucklingLengthZ").expect("independent z pointer") = serde_json::json!(value);
    assert_eq!(json(&changed), expected);
    assert_eq!(changed.members[0].buckling_length_y, member.buckling_length_y);
    assert_eq!(changed.members[0].buckling_length_t, member.buckling_length_t);
    assert_eq!(json(&document), original);
}

#[test]
fn bolt_rows_preserve_the_current_addressed_bolts_per_row() {
    let document = En1999Snapshot::full_showcase();
    let original = json(&document);
    let index = document.connections.iter().position(|connection| connection.bolts.rows > 0 && connection.bolts.bolts_per_row > 0).expect("showcase bolted connection");
    let connection = &document.connections[index];
    let rows = connection.bolts.rows.checked_add(1).expect("fixture row headroom");
    let operations = resolve_edit(&document, &NormEdit::SetField { path: format!("connections[id={}].bolts.rows", connection.id), value: DslValue::uint(u64::from(rows)) }).expect("complete bolt-count payload");
    let [En1999Mutation::ChangeBoltCount(payload)] = operations.as_slice() else { panic!("canonical bolt-count mutation") };
    assert_eq!(payload.connection_id, connection.id);
    assert_eq!(payload.new_rows, rows);
    assert_eq!(payload.new_bolts_per_row, connection.bolts.bolts_per_row);
    let (changed, messages) = crate::protocol::apply_mutation(&document, &operations[0]).expect("typed bolt-count application");
    assert!(messages.is_empty(), "{messages:?}");
    let mut expected = original.clone();
    *expected.pointer_mut(&format!("/connections/{index}/bolts/rows")).expect("independent row pointer") = serde_json::json!(rows);
    assert_eq!(json(&changed), expected);
    assert_eq!(changed.connections[index].bolts.bolts_per_row, connection.bolts.bolts_per_row);
    assert_eq!(json(&document), original);
}

#[test]
fn a_paired_bolt_edit_reads_its_sibling_from_the_preceding_admitted_projection() {
    let document = En1999Snapshot::full_showcase();
    let original = json(&document);
    let index = document.connections.iter().position(|connection| connection.bolts.rows > 0 && connection.bolts.bolts_per_row > 0).expect("showcase bolted connection");
    let connection = &document.connections[index];
    let rows = connection.bolts.rows.checked_add(1).expect("fixture row headroom");
    let per_row = connection.bolts.bolts_per_row.checked_add(1).expect("fixture column headroom");
    let edits = [
        NormEdit::SetField { path: format!("connections[id={}].bolts.boltsPerRow", connection.id), value: DslValue::uint(u64::from(per_row)) },
        NormEdit::SetField { path: format!("connections[id={}].bolts.rows", connection.id), value: DslValue::uint(u64::from(rows)) },
    ];
    let operations = semio_s_plugin_norm_app_surface::resolve_norm_edits(&document, &edits, resolve_edit).expect("ordered typed pair");
    let [_, En1999Mutation::ChangeBoltCount(payload)] = operations.as_slice() else { panic!("second operation is the canonical coupled count setter") };
    assert_eq!(payload.new_bolts_per_row, per_row);
    let mut changed = document.clone();
    for operation in &operations {
        let (next, messages) = crate::protocol::apply_mutation(&changed, operation).expect("typed paired application");
        assert!(messages.is_empty(), "{messages:?}");
        changed = next;
    }
    let mut expected = original.clone();
    *expected.pointer_mut(&format!("/connections/{index}/bolts/rows")).expect("independent row pointer") = serde_json::json!(rows);
    *expected.pointer_mut(&format!("/connections/{index}/bolts/boltsPerRow")).expect("independent column pointer") = serde_json::json!(per_row);
    assert_eq!(json(&changed), expected);
    assert_eq!(json(&document), original);
}
