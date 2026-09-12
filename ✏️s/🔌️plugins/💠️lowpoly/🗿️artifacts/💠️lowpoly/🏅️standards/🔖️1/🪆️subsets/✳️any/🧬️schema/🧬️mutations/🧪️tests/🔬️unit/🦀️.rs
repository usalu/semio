use super::*;
use crate::{schema::default_snapshot, LowpolyObject};
use protocol::{Mutation, MutationDiff};

fn tiny_object(id: &str, name: &str) -> LowpolyObject {
    let mesh = default_snapshot().objects[0].mesh.clone();
    LowpolyObject { id: id.into(), name: name.into(), transform: Default::default(), smooth_shading: false, mesh, paint_layers: Vec::new() }
}

//#region ⚖️SemanticLaws
/// ⚖️ `assert_mutation_inverse_law`/`assert_mutation_diff_absorb_law` (`protocol::os_spr::protocol_laws`,
/// reachable via this crate's existing `semio-framework-os-kernel` dependency — no new Cargo
/// dependency needed) against an id-keyed create/delete pair and a scalar rename.
#[semio_framework_async_macros::async_test]
async fn create_object_obeys_the_inverse_and_absorb_laws() {
    let base = default_snapshot();
    let create = LowpolyMutation::CreateObject(super::super::create_object::CreateObject { index: base.objects.len(), object: tiny_object("obj-99", "Extra") });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &create).await;
    let d1 = create.diff(&base).into_parts().0;
    let after = d1.apply(&base).expect("valid mutation diff");
    let d2 = LowpolyMutation::RenameObject(super::super::rename_object::RenameObject { id: "obj-99".into(), new_name: "Renamed".into() }).diff(&after).into_parts().0;
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn delete_object_of_a_missing_id_has_an_empty_inverse() {
    let base = default_snapshot();
    let delete = LowpolyMutation::DeleteObject(super::super::delete_object::DeleteObject { id: "nope".into() });
    assert!(delete.inverse(&base).is_empty(), "deleting an absent id has nothing to undo");
}

#[semio_framework_async_macros::async_test]
async fn move_object_obeys_the_inverse_law() {
    let base = default_snapshot();
    let id = base.objects[0].id.clone();
    let mutation = LowpolyMutation::MoveObject(super::super::move_object::MoveObject { id, new_position: [4.0, 5.0, 6.0] });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
}
//#endregion ⚖️SemanticLaws

//#region 🔖️OutcomeLaws
/// ✅️ 26/08/16 MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS §C2 laws — one per
/// representative verb family: `assert_missing_target_is_error`/`assert_fatal_never_applies`
/// above, `assert_outcome_policy_matrix` below (create, delete, move, rename).
#[semio_framework_async_macros::async_test]
async fn delete_object_missing_target_is_an_error() {
    let base = default_snapshot();
    let mutation = LowpolyMutation::DeleteObject(super::super::delete_object::DeleteObject { id: "does-not-exist".into() });
    protocol::os_spr::protocol_laws::assert_missing_target_is_error(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn move_object_missing_target_is_an_error() {
    let base = default_snapshot();
    let mutation = LowpolyMutation::MoveObject(super::super::move_object::MoveObject { id: "does-not-exist".into(), new_position: [1.0, 2.0, 3.0] });
    protocol::os_spr::protocol_laws::assert_missing_target_is_error(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn rename_object_missing_target_is_an_error() {
    let base = default_snapshot();
    let mutation = LowpolyMutation::RenameObject(super::super::rename_object::RenameObject { id: "does-not-exist".into(), new_name: "X".into() });
    protocol::os_spr::protocol_laws::assert_missing_target_is_error(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn create_object_duplicate_id_is_fatal_and_never_applies() {
    let base = default_snapshot();
    let existing_id = base.objects[0].id.clone();
    let mutation = LowpolyMutation::CreateObject(super::super::create_object::CreateObject { index: 0, object: tiny_object(&existing_id, "Dup") });
    let outcome = mutation.diff(&base);
    assert_eq!(outcome.worst_level(), Some(protocol::os_dsl::Severity::Fatal));
    protocol::os_spr::protocol_laws::assert_fatal_never_applies(&outcome).await;
}

#[semio_framework_async_macros::async_test]
async fn create_object_outcome_obeys_the_policy_matrix() {
    let base = default_snapshot();
    let mutation = LowpolyMutation::CreateObject(super::super::create_object::CreateObject { index: base.objects.len(), object: tiny_object("obj-99", "Extra") });
    protocol::os_spr::protocol_laws::assert_outcome_policy_matrix(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn delete_object_outcome_obeys_the_policy_matrix() {
    let base = default_snapshot();
    let existing_id = base.objects[0].id.clone();
    let mutation = LowpolyMutation::DeleteObject(super::super::delete_object::DeleteObject { id: existing_id });
    protocol::os_spr::protocol_laws::assert_outcome_policy_matrix(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn move_object_outcome_obeys_the_policy_matrix() {
    let base = default_snapshot();
    let id = base.objects[0].id.clone();
    let mutation = LowpolyMutation::MoveObject(super::super::move_object::MoveObject { id, new_position: [4.0, 5.0, 6.0] });
    protocol::os_spr::protocol_laws::assert_outcome_policy_matrix(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn rename_object_outcome_obeys_the_policy_matrix() {
    let base = default_snapshot();
    let id = base.objects[0].id.clone();
    let mutation = LowpolyMutation::RenameObject(super::super::rename_object::RenameObject { id, new_name: "Renamed".into() });
    protocol::os_spr::protocol_laws::assert_outcome_policy_matrix(&base, &mutation).await;
}
//#endregion 🔖️OutcomeLaws

//#region 🧪️KindsCatalog
/// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling
/// `#[derive(dsl::Mutations)]` assigns, and every entry must also appear in the committed oracle
/// manifest's catalog — the framework never parses Rust, so this is the only thing that keeps the
/// declared vocabulary and the measured one from drifting apart.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    let descriptors = <LowpolyMutation as protocol::SemanticMutation<LowpolySnapshot>>::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared LowpolyMutation variant");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
    }
    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
//#endregion 🧪️KindsCatalog
