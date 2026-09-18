use super::*;
use crate::{schema::default_snapshot, LowpolyObject, LowpolySnapshot};
use protocol::{Mutation, MutationDiff};

fn tiny_object(id: &str, name: &str) -> LowpolyObject {
    let mesh = default_snapshot().objects[0].mesh.clone();
    LowpolyObject { id: id.into(), name: name.into(), transform: Default::default(), smooth_shading: false, mesh, paint_layers: Vec::new(), mesh_content: String::new() }
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

//#region 🔄FixtureRefresh
fn fixture_json_encode<T: dsl::ToValue>(value: &T) -> String {
    serde_json::to_string_pretty(&Into::<serde_json::Value>::into(dsl::ToValue::to_value(value))).expect("fixture json encode") + "\n"
}

fn fixture_json_decode<T: dsl::FromValue>(text: &str) -> T {
    let parsed: serde_json::Value = serde_json::from_str(text).expect("fixture json parses");
    dsl::FromValue::from_value(dsl::DslValue::from(parsed)).expect("fixture json decodes")
}

/// 🔄️ Re-encodes every committed mutation quintet under `🧫️fixtures/🧬️mutations` when
/// `REFRESH_LOWPOLY_MUTATION_FIXTURES=1`.
#[test]
fn refresh_lowpoly_mutation_fixtures_when_requested() {
    if std::env::var("REFRESH_LOWPOLY_MUTATION_FIXTURES").ok().as_deref() != Some("1") {
        return;
    }
    use std::path::{Path, PathBuf};
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations");
    fn collect_cases(dir: &Path, cases: &mut Vec<PathBuf>) {
        for entry in std::fs::read_dir(dir).expect("read fixtures dir") {
            let path = entry.expect("fixture dir entry").path();
            if !path.is_dir() {
                continue;
            }
            if path.join("🦠️mutation/🔣️.json").is_file() {
                cases.push(path);
            } else {
                collect_cases(&path, cases);
            }
        }
    }
    let mut cases = Vec::new();
    collect_cases(&root, &mut cases);
    for case in cases {
        let before_path = case.join("📸️snapshot/⬅️before/🔣️.json");
        let after_path = case.join("📸️snapshot/➡️after/🔣️.json");
        let mutation_path = case.join("🦠️mutation/🔣️.json");
        let diff_path = case.join("🔺️diff/🔣️.json");
        let before: LowpolySnapshot = fixture_json_decode(&std::fs::read_to_string(&before_path).expect("read before"));
        let mutation: LowpolyMutation = fixture_json_decode(&std::fs::read_to_string(&mutation_path).expect("read mutation"));
        std::fs::write(&before_path, fixture_json_encode(&before)).expect("write before");
        std::fs::write(&mutation_path, fixture_json_encode(&mutation)).expect("write mutation");
        let (after, _) = protocol::apply_mutation(&before, &mutation).expect("apply mutation");
        std::fs::write(&after_path, fixture_json_encode(&after)).expect("write after");
        let raised = <LowpolyMutation as Mutation<LowpolySnapshot>>::diff(&mutation, &before);
        std::fs::write(&diff_path, fixture_json_encode(raised.diff())).expect("write diff");
    }
}
//#endregion 🔄FixtureRefresh
