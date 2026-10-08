use crate::standards::v1::subsets::any::io::text::mutations::encode_en1995_mutation_json;
use crate::standards::v1::subsets::any::io::text::mutations::decode_en1995_mutation_json;
use super::*;
use crate::standards::v1::subsets::any::io::{apply_en1995_mutation, inverse_en1995_mutation};
use protocol::SemanticMutation;

fn every_mutation() -> Vec<En1995Mutation> {
    let base = En1995Snapshot::compliant_building_beam();
    vec![
        En1995Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: crate::document::AnnexChoice::En }),
        En1995Mutation::InsertMember(insert_member::InsertMember { index: 99, member: crate::TimberMember { id: "beam-B9".into(), ..base.members[0].clone() } }),
        En1995Mutation::RemoveMember(remove_member::RemoveMember { index: 0 }),
        En1995Mutation::ChangeMemberLabelEn(change_member_label_en::ChangeMemberLabelEn { member_id: base.members[0].id.clone(), new_value: "Beam B1 (revised)".into() }),
        En1995Mutation::ChangeMemberLabelDe(change_member_label_de::ChangeMemberLabelDe { member_id: base.members[0].id.clone(), new_value: "Träger B1 (überarbeitet)".into() }),
        En1995Mutation::ChangeMemberRole(change_member_role::ChangeMemberRole { member_id: base.members[0].id.clone(), new_value: crate::MemberRole::Column }),
        En1995Mutation::ChangeMemberStrengthClass(change_member_strength_class::ChangeMemberStrengthClass { member_id: base.members[0].id.clone(), new_value: "GL32h".into() }),
        En1995Mutation::ChangeMemberServiceClass(change_member_service_class::ChangeMemberServiceClass { member_id: base.members[0].id.clone(), new_value: 2 }),
        En1995Mutation::ChangeMemberSupport(change_member_support::ChangeMemberSupport { member_id: base.members[0].id.clone(), new_value: crate::SupportType::Cantilever }),
        En1995Mutation::ChangeMemberB(change_member_b::ChangeMemberB { member_id: base.members[0].id.clone(), new_value: 0.24 }),
        En1995Mutation::ChangeMemberH(change_member_h::ChangeMemberH { member_id: base.members[0].id.clone(), new_value: 0.5 }),
        En1995Mutation::ChangeMemberSpan(change_member_span::ChangeMemberSpan { member_id: base.members[0].id.clone(), new_value: 7.0 }),
        En1995Mutation::ChangeMemberSupportLength(change_member_support_length::ChangeMemberSupportLength { member_id: base.members[0].id.clone(), new_value: 0.2 }),
        En1995Mutation::ChangeMemberBearingLength(change_member_bearing_length::ChangeMemberBearingLength { member_id: base.members[0].id.clone(), new_value: 0.2 }),
        En1995Mutation::ChangeMemberBucklingLengthY(change_member_buckling_length_y::ChangeMemberBucklingLengthY { member_id: base.members[0].id.clone(), new_value: 6.0 }),
        En1995Mutation::ChangeMemberBucklingLengthZ(change_member_buckling_length_z::ChangeMemberBucklingLengthZ { member_id: base.members[0].id.clone(), new_value: 2.0 }),
        En1995Mutation::ChangeMemberRestraintSpacing(change_member_restraint_spacing::ChangeMemberRestraintSpacing { member_id: base.members[0].id.clone(), new_value: 1.0 }),
        En1995Mutation::ChangeMemberNotchDepth(change_member_notch_depth::ChangeMemberNotchDepth { member_id: base.members[0].id.clone(), new_value: 0.02 }),
        En1995Mutation::ChangeMemberNotchDistance(change_member_notch_distance::ChangeMemberNotchDistance { member_id: base.members[0].id.clone(), new_value: 0.1 }),
        En1995Mutation::ChangeMemberMCrit(change_member_m_crit::ChangeMemberMCrit { member_id: base.members[0].id.clone(), new_value: 150000.0 }),
        En1995Mutation::ChangeMemberMassKgPerM(change_member_mass_kg_per_m::ChangeMemberMassKgPerM { member_id: base.members[0].id.clone(), new_value: 140.0 }),
        En1995Mutation::ChangeMemberMassKgPerM2(change_member_mass_kg_per_m2::ChangeMemberMassKgPerM2 { member_id: base.members[0].id.clone(), new_value: 60.0 }),
        En1995Mutation::ChangeMemberDampingXi(change_member_damping_xi::ChangeMemberDampingXi { member_id: base.members[0].id.clone(), new_value: 0.02 }),
        En1995Mutation::ChangeMemberFireDuration(change_member_fire_duration::ChangeMemberFireDuration { member_id: base.members[0].id.clone(), new_value: 1800.0 }),
        En1995Mutation::ChangeMemberBridgeNObs(change_member_bridge_n_obs::ChangeMemberBridgeNObs { member_id: base.members[0].id.clone(), new_value: 1.0e5 }),
        En1995Mutation::ChangeMemberBridgeTLYears(change_member_bridge_tl_years::ChangeMemberBridgeTLYears { member_id: base.members[0].id.clone(), new_value: 50.0 }),
        En1995Mutation::ChangeMemberBridgeBeta(change_member_bridge_beta::ChangeMemberBridgeBeta { member_id: base.members[0].id.clone(), new_value: 5.0 }),
        En1995Mutation::ChangeMemberBridgeA(change_member_bridge_a::ChangeMemberBridgeA { member_id: base.members[0].id.clone(), new_value: 15.0 }),
        En1995Mutation::ChangeMemberBridgeB(change_member_bridge_b::ChangeMemberBridgeB { member_id: base.members[0].id.clone(), new_value: 4.0 }),
        En1995Mutation::ChangeMemberBridgeCrowd(change_member_bridge_crowd::ChangeMemberBridgeCrowd { member_id: base.members[0].id.clone(), new_value: 1.0 }),
        En1995Mutation::InsertMemberAction(insert_member_action::InsertMemberAction { member_id: base.members[0].id.clone(), index: 99, action: crate::CharacteristicAction { id: "w".into(), ..base.members[0].actions[0].clone() } }),
        En1995Mutation::RemoveMemberAction(remove_member_action::RemoveMemberAction { member_id: base.members[0].id.clone(), index: 0 }),
        En1995Mutation::ChangeMemberActionKind(change_member_action_kind::ChangeMemberActionKind { member_id: base.members[0].id.clone(), action_id: base.members[0].actions[0].id.clone(), new_value: "imposed".into() }),
        En1995Mutation::ChangeMemberActionCategory(change_member_action_category::ChangeMemberActionCategory { member_id: base.members[0].id.clone(), action_id: base.members[0].actions[0].id.clone(), new_value: "B".into() }),
        En1995Mutation::ChangeMemberLoadDuration(change_member_load_duration::ChangeMemberLoadDuration { member_id: base.members[0].id.clone(), action_id: base.members[0].actions[0].id.clone(), new_value: "short".into() }),
        En1995Mutation::ChangeMemberActionQLine(change_member_action_q_line::ChangeMemberActionQLine { member_id: base.members[0].id.clone(), action_id: base.members[0].actions[0].id.clone(), new_value: 3500.0 }),
        En1995Mutation::ChangeMemberActionFPoint(change_member_action_f_point::ChangeMemberActionFPoint { member_id: base.members[0].id.clone(), action_id: base.members[0].actions[0].id.clone(), new_value: 2000.0 }),
        En1995Mutation::ChangeMemberActionMK(change_member_action_mk::ChangeMemberActionMK { member_id: base.members[0].id.clone(), action_id: base.members[0].actions[0].id.clone(), new_value: 12000.0 }),
        En1995Mutation::ChangeMemberActionVK(change_member_action_vk::ChangeMemberActionVK { member_id: base.members[0].id.clone(), action_id: base.members[0].actions[0].id.clone(), new_value: 8000.0 }),
        En1995Mutation::ChangeMemberActionNK(change_member_action_nk::ChangeMemberActionNK { member_id: base.members[0].id.clone(), action_id: base.members[0].actions[0].id.clone(), new_value: 5000.0 }),
        En1995Mutation::ChangeMemberActionNTK(change_member_action_ntk::ChangeMemberActionNTK { member_id: base.members[0].id.clone(), action_id: base.members[0].actions[0].id.clone(), new_value: 3000.0 }),
        En1995Mutation::ChangeMemberActionFC90K(change_member_action_fc90_k::ChangeMemberActionFC90K { member_id: base.members[0].id.clone(), action_id: base.members[0].actions[0].id.clone(), new_value: 9000.0 }),
        En1995Mutation::InsertConnection(insert_connection::InsertConnection { index: 99, connection: crate::TimberConnection { id: "conn-C9".into(), ..base.connections[0].clone() } }),
        En1995Mutation::RemoveConnection(remove_connection::RemoveConnection { index: 0 }),
        En1995Mutation::ChangeConnectionLabelEn(change_connection_label_en::ChangeConnectionLabelEn { connection_id: base.connections[0].id.clone(), new_value: "Bolt group (revised)".into() }),
        En1995Mutation::ChangeConnectionLabelDe(change_connection_label_de::ChangeConnectionLabelDe { connection_id: base.connections[0].id.clone(), new_value: "Schraubverbund (überarbeitet)".into() }),
        En1995Mutation::ChangeConnectionFastenerType(change_connection_fastener_type::ChangeConnectionFastenerType { connection_id: base.connections[0].id.clone(), new_value: "screw".into() }),
        En1995Mutation::ChangeConnectionStrengthClass(change_connection_strength_class::ChangeConnectionStrengthClass { connection_id: base.connections[0].id.clone(), new_value: "C24".into() }),
        En1995Mutation::ChangeConnectionServiceClass(change_connection_service_class::ChangeConnectionServiceClass { connection_id: base.connections[0].id.clone(), new_value: 2 }),
        En1995Mutation::ChangeConnectionDiameter(change_connection_diameter::ChangeConnectionDiameter { connection_id: base.connections[0].id.clone(), new_value: 0.016 }),
        En1995Mutation::ChangeConnectionNumber(change_connection_number::ChangeConnectionNumber { connection_id: base.connections[0].id.clone(), new_value: 12 }),
        En1995Mutation::ChangeConnectionRows(change_connection_rows::ChangeConnectionRows { connection_id: base.connections[0].id.clone(), new_value: 3 }),
        En1995Mutation::ChangeConnectionSpacing(change_connection_spacing::ChangeConnectionSpacing { connection_id: base.connections[0].id.clone(), new_value: 0.1 }),
        En1995Mutation::ChangeConnectionEdgeDistance(change_connection_edge_distance::ChangeConnectionEdgeDistance { connection_id: base.connections[0].id.clone(), new_value: 0.05 }),
        En1995Mutation::ChangeConnectionEndDistance(change_connection_end_distance::ChangeConnectionEndDistance { connection_id: base.connections[0].id.clone(), new_value: 0.1 }),
        En1995Mutation::ChangeConnectionT1(change_connection_t1::ChangeConnectionT1 { connection_id: base.connections[0].id.clone(), new_value: 0.22 }),
        En1995Mutation::ChangeConnectionT2(change_connection_t2::ChangeConnectionT2 { connection_id: base.connections[0].id.clone(), new_value: 0.22 }),
        En1995Mutation::ChangeConnectionSteelPlate(change_connection_steel_plate::ChangeConnectionSteelPlate { connection_id: base.connections[0].id.clone(), new_value: true }),
        En1995Mutation::ChangeConnectionPlateThickness(change_connection_plate_thickness::ChangeConnectionPlateThickness { connection_id: base.connections[0].id.clone(), new_value: 0.008 }),
        En1995Mutation::ChangeConnectionShearPlanes(change_connection_shear_planes::ChangeConnectionShearPlanes { connection_id: base.connections[0].id.clone(), new_value: 2 }),
        En1995Mutation::ChangeConnectionFUK(change_connection_fuk::ChangeConnectionFUK { connection_id: base.connections[0].id.clone(), new_value: 500_000_000.0 }),
        En1995Mutation::InsertConnectionAction(insert_connection_action::InsertConnectionAction { connection_id: base.connections[0].id.clone(), index: 99, action: crate::ConnectionAction { id: "w".into(), ..base.connections[0].actions[0].clone() } }),
        En1995Mutation::RemoveConnectionAction(remove_connection_action::RemoveConnectionAction { connection_id: base.connections[0].id.clone(), index: 0 }),
        En1995Mutation::ChangeConnectionActionKind(change_connection_action_kind::ChangeConnectionActionKind { connection_id: base.connections[0].id.clone(), action_id: base.connections[0].actions[0].id.clone(), new_value: "imposed".into() }),
        En1995Mutation::ChangeConnectionLoadDuration(change_connection_load_duration::ChangeConnectionLoadDuration { connection_id: base.connections[0].id.clone(), action_id: base.connections[0].actions[0].id.clone(), new_value: "short".into() }),
        En1995Mutation::ChangeConnectionActionFK(change_connection_action_fk::ChangeConnectionActionFK { connection_id: base.connections[0].id.clone(), action_id: base.connections[0].actions[0].id.clone(), new_value: 12000.0 }),
    ]
}

/// 🏷️ Every declared variant labels itself and the closed vocabulary is exactly [`KINDS`].
#[test]
fn every_declared_kind_has_a_label_and_matches_kinds_len() {
    for mutation in every_mutation() {
        let _ = <En1995Mutation as SemanticMutation<En1995Snapshot>>::label(&mutation);
    }
    assert_eq!(every_mutation().len(), KINDS.len());
}

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
    use crate::app_surface::NormEdit;
    let base = En1995Snapshot::compliant_building_beam();
    let member = base.members[0].id.clone();
    let set = NormEdit::SetField { path: format!("members[id={member}].spanM"), value: semio_framework_value::DslValue::float(6.5) };
    assert_eq!(EDIT_RULES.resolve::<En1995Snapshot, En1995Mutation>(&base, &set), Ok(vec![En1995Mutation::ChangeMemberSpan(change_member_span::ChangeMemberSpan { member_id: member.clone(), new_value: 6.5 })]));
    let remove = NormEdit::RemoveItem { path: "members".into(), index: 0 };
    assert_eq!(EDIT_RULES.resolve::<En1995Snapshot, En1995Mutation>(&base, &remove), Ok(vec![En1995Mutation::RemoveMember(remove_member::RemoveMember { index: 0 })]));
    let unknown = NormEdit::SetField { path: "members[0].id".into(), value: semio_framework_value::DslValue::String("x".into()) };
    assert!(EDIT_RULES.resolve::<En1995Snapshot, En1995Mutation>(&base, &unknown).is_err());
}

/// ↩️ The external bridges round-trip every kind through JSON, apply it silently, and invert it exactly.
#[test]
fn bridges_round_trip_apply_and_invert_every_kind() {
    let base = En1995Snapshot::compliant_building_beam();
    for mutation in every_mutation() {
        assert_eq!(decode_en1995_mutation_json(&encode_en1995_mutation_json(&mutation)).expect("decodes"), mutation);
        let (after, messages) = apply_en1995_mutation(&base, &mutation).expect("applies");
        assert!(messages.is_empty(), "{mutation:?}: {messages:?}");
        assert_ne!(after, base, "{mutation:?} must be observable");
        let mut restored = after;
        for step in inverse_en1995_mutation(&mutation, &base).expect("valid retained mutation inverse fixture") {
            restored = apply_en1995_mutation(&restored, &step).expect("inverse step applies").0;
        }
        assert_eq!(restored, base, "{mutation:?}: inverse must restore the base");
    }
}

fn fixtures_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations")
}

/// 🗂️ `(kind, fixture scenario folder)` for every kind, named by the committed `en1995-1-any` catalog.
fn catalog_vector_dirs() -> Vec<(&'static str, std::path::PathBuf)> {
    let manifest: serde_json::Value = serde_json::from_str(include_str!("../../../../🔮️oracles/🔣️.json")).expect("oracle manifest");
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
    let path = dir.join(facet).join("🔣️.json");
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

macro_rules! decoded {
    ($ty:ty, $dir:expr, $facet:expr, $kind:expr) => {
        semio_framework_pack_json::from_json_str::<$ty>(&text_at($dir, $facet), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|e| panic!("{} {}: {e}", $kind, $facet))
    };
}

/// ♻️ `EN1995_REGEN_MUTATION_VECTORS=1` rewrites one before/mutation/diff/after/outcome vector per kind in the production JSON codec.
#[test]
fn regen_mutation_vectors() {
    if std::env::var("EN1995_REGEN_MUTATION_VECTORS").ok().as_deref() != Some("1") {
        return;
    }
    let pretty = |compact: String| serde_json::to_string_pretty(&serde_json::from_str::<serde_json::Value>(&compact).expect("codec json")).unwrap() + "\n";
    let base = En1995Snapshot::compliant_building_beam();
    let _ = std::fs::remove_dir_all(fixtures_root());
    for (mutation, (_, dir)) in every_mutation().into_iter().zip(catalog_vector_dirs()) {
        let outcome = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation, &base);
        let after = protocol::apply_diff(outcome.diff(), &base).expect("applies");
        let facets = [
            ("📸️snapshot/⬅️before", semio_framework_pack_json::to_json_string(&base)),
            ("📸️snapshot/➡️after", semio_framework_pack_json::to_json_string(&after)),
            ("🦠️mutation", semio_framework_pack_json::to_json_string(&mutation)),
            ("🔺️diff", semio_framework_pack_json::to_json_string(outcome.diff())),
            ("🎯️outcome", format!("{{\"status\":\"{}\"}}", if after == base { "no-op" } else { "applied" })),
        ];
        for (facet, compact) in facets {
            let folder = dir.join(facet);
            std::fs::create_dir_all(&folder).unwrap();
            std::fs::write(folder.join("🔣️.json"), pretty(compact)).unwrap();
        }
    }
}

/// 🧫️ Every committed vector replays through the production codec: the mutation decodes, applies to `before`, and yields exactly `diff` and `after`.
#[test]
fn committed_mutation_vectors_replay_for_every_kind() {
    let dirs = catalog_vector_dirs();
    let on_disk = std::fs::read_dir(fixtures_root()).expect("fixtures root").count();
    assert_eq!(on_disk, KINDS.len(), "one fixture folder per kind, no leftovers");
    for ((kind, dir), expected) in dirs.into_iter().zip(every_mutation()) {
        let before = decoded!(En1995Snapshot, &dir, "📸️snapshot/⬅️before", kind);
        let mutation = decoded!(En1995Mutation, &dir, "🦠️mutation", kind);
        assert_eq!(mutation, expected, "{kind} mutation");
        assert_eq!(<En1995Mutation as SemanticMutation<En1995Snapshot>>::semantics(&mutation).kind, kind);
        let outcome = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation, &before);
        assert!(outcome.worst_level().is_none(), "{kind}: {:?}", outcome.messages());
        assert_eq!(outcome.diff(), &decoded!(En1995Diff, &dir, "🔺️diff", kind), "{kind} diff");
        let after = protocol::apply_diff(outcome.diff(), &before).expect("applies");
        assert_eq!(after, decoded!(En1995Snapshot, &dir, "📸️snapshot/➡️after", kind), "{kind} after");
        assert!(text_at(&dir, "🎯️outcome").contains("\"applied\""), "{kind} outcome");
    }
}
