use semio_framework_test_fixtures::{fixture_path, fixture_text};
use semio_s_artifact_norm_en1995_io::text::mutations::decode_en1995_mutation_json;
use semio_s_artifact_norm_en1995_io::text::mutations::encode_en1995_mutation_json;
use semio_s_artifact_norm_en1995_mutations::mutation_bridge::{apply_en1995_mutation, inverse_en1995_mutation};
use semio_s_artifact_norm_en1995_mutations::*;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use protocol::SemanticMutation;

/// 🧭️ Every set / insert / remove kind of the editor vocabulary is reachable through a declared edit rule, and the rules name only declared kinds.
#[test]
fn edit_rules_name_only_declared_kinds() {
    let rules = EDIT_RULES;
    for kind in rules.set_field.iter().map(|rule| rule.kind).chain(rules.insert_item.iter().map(|rule| rule.kind)).chain(rules.remove_item.iter().map(|rule| rule.kind)) {
        assert!(KINDS.contains(&kind), "{kind} is not an En1995 kind");
    }
}

/// 🧭️ A value-tree edit resolves to exactly the concrete kind its path names.
#[test]
fn value_tree_edits_resolve_to_concrete_kinds() {
    use semio_s_plugin_norm_app_surface::NormEdit;
    let base = En1995Snapshot::compliant_building_beam();
    let member = base.members[0].id.clone();
    let set = NormEdit::SetField { path: format!("members[id={member}].spanM"), value: semio_framework_value::DslValue::float(6.5) };
    assert_eq!(EDIT_RULES.resolve::<En1995Snapshot, En1995Mutation>(&base, &set), Ok(vec![En1995Mutation::ChangeMemberSpan(change_member_span::ChangeMemberSpan { member_id: member.clone(), new_value: 6.5 })]));
    let remove = NormEdit::RemoveItem { path: "members".into(), index: 0 };
    assert_eq!(EDIT_RULES.resolve::<En1995Snapshot, En1995Mutation>(&base, &remove), Ok(vec![En1995Mutation::RemoveMember(remove_member::RemoveMember { index: 0 })]));
    let unknown = NormEdit::SetField { path: "members[0].id".into(), value: semio_framework_value::DslValue::String("x".into()) };
    assert!(EDIT_RULES.resolve::<En1995Snapshot, En1995Mutation>(&base, &unknown).is_err());
}

fn fixtures_root() -> std::path::PathBuf {
    fixture_path!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations")
}

/// 🗂️ `(kind, fixture scenario folder)` for every kind, named by the committed `en1995-1-any` catalog.
fn catalog_vector_dirs() -> Vec<(&'static str, std::path::PathBuf)> {
    let manifest: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../🔮️oracles/🔣️.json")).expect("oracle manifest");
    let vectors = manifest["mutationCatalogs"].as_array().and_then(|c| c.iter().find(|c| c["id"] == "en1995-1-any")).and_then(|c| c["vectors"].as_array()).expect("en1995-1-any vectors");
    KINDS
        .iter()
        .map(|kind| {
            let vector = vectors.iter().find(|v| v["mutationId"] == *kind).unwrap_or_else(|| panic!("{kind} missing from catalog"));
            let scenario = vector["scenarios"][0]["directoryName"].as_str().expect("scenario directory");
            (*kind, fixtures_root().join(vector["mutationDirectoryName"].as_str().expect("mutation directory")).join(scenario))
        })
        .collect()
}

fn text_at(dir: &std::path::Path, facet: &str) -> String {
    let path = dir.join(format!("🔣️.{facet}.json"));
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

macro_rules! decoded {
    ($ty:ty, $dir:expr, $facet:expr, $kind:expr) => {
        semio_framework_pack_json::from_json_str::<$ty>(&text_at($dir, $facet), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|e| panic!("{} {}: {e}", $kind, $facet))
    };
}

/// 🧫️ The committed vector of every kind: its `before` snapshot and its decoded mutation, in [`KINDS`] order.
fn vector_cases() -> Vec<(&'static str, En1995Snapshot, En1995Mutation)> {
    catalog_vector_dirs().into_iter().map(|(kind, dir)| (kind, decoded!(En1995Snapshot, &dir, "before", kind), decoded!(En1995Mutation, &dir, "mutation", kind))).collect()
}

/// 🏷️ Every declared variant labels itself, names its own kind, and the closed vocabulary is exactly [`KINDS`].
#[test]
fn every_declared_kind_has_a_label_and_matches_kinds_len() {
    let cases = vector_cases();
    assert_eq!(cases.len(), KINDS.len());
    for (kind, _, mutation) in cases {
        let _ = <En1995Mutation as SemanticMutation<En1995Snapshot>>::label(&mutation);
        assert_eq!(<En1995Mutation as SemanticMutation<En1995Snapshot>>::semantics(&mutation).kind, kind);
    }
}

/// ↩️ The external bridges round-trip every kind through JSON, apply it silently, and invert it exactly on its committed `before` snapshot.
#[test]
fn bridges_round_trip_apply_and_invert_every_kind() {
    for (kind, base, mutation) in vector_cases() {
        assert_eq!(decode_en1995_mutation_json(&encode_en1995_mutation_json(&mutation)).expect("decodes"), mutation, "{kind}");
        let (after, messages) = apply_en1995_mutation(&base, &mutation).expect("applies");
        assert!(messages.is_empty(), "{kind}: {messages:?}");
        assert_ne!(after, base, "{kind} must be observable");
        let mut restored = after;
        for step in inverse_en1995_mutation(&mutation, &base).expect("valid retained mutation inverse") {
            restored = apply_en1995_mutation(&restored, &step).expect("inverse step applies").0;
        }
        assert_eq!(restored, base, "{kind}: inverse must restore the base");
    }
}
