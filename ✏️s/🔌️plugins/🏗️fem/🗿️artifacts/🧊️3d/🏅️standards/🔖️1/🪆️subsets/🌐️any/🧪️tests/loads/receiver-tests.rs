use super::*;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::{FemLoad, FemLoadCase};
use semio_framework_value::{NativeDecodeControl, RetainedCloneGrant};
use semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient;

#[derive(semio_framework_value_derive::FromValue, RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
struct IdentityCase {
    id: String,
    cases: Vec<CaseIds>,
    valid: bool,
}
#[derive(semio_framework_value_derive::FromValue, RetireOwned)]
#[value(deny_unknown_fields)]
struct CaseIds {
    id: String,
    loads: Vec<String>,
}
#[derive(semio_framework_value_derive::FromValue, RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
struct Receiver {
    variants: Vec<String>,
    area: String,
    unknown: String,
    self_weight: String,
    empty_cases: String,
    node_reference: String,
    member_reference: String,
}
#[derive(semio_framework_value_derive::FromValue, RetireOwned)]
#[value(deny_unknown_fields)]
struct Cases {
    schema: String,
    dofs: Vec<String>,
    bits: Vec<String>,
    identities: Vec<IdentityCase>,
    receiver: Receiver,
    ownership: Vec<String>,
}
fn cases() -> Cases {
    semio_framework_pack_json::from_json_str(include_str!("cases.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()
}
fn grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 10_000_000, maximum_copy_bytes: 256 * 1048576, maximum_capacity_bytes: 256 * 1048576, maximum_release_bytes: 256 * 1048576, maximum_depth: 128 }
}
fn source() -> Fem3dSnapshotValue {
    let mut value = Fem3dSnapshotValue::default();
    value.nodes = vec![FemNodeValue { id: "a".into(), x: 0., y: 0., z: 0. }, FemNodeValue { id: "b".into(), x: 1., y: 0., z: 0. }];
    value.materials = vec![FemMaterialValue { id: "m".into(), name: "Material".into(), e: 2e11, g: 8e10, nu: 0.25, rho: 7850. }];
    value.sections = vec![FemSection { id: "s".into(), name: "Section".into(), area: 0.01, iy: 1e-5, iz: 2e-5, j: 3e-5 }];
    value.elements = vec![FemElement::Bar { id: "e".into(), start: "a".into(), end: "b".into(), material_id: "m".into(), section_id: "s".into() }];
    value
}
fn nodal(id: &str, dof: FemDof, bits: u64) -> FemLoad {
    FemLoad::Nodal { id: id.into(), node_id: "b".into(), dof, value: f64::from_bits(bits) }
}
fn load_case(id: &str, loads: Vec<FemLoad>) -> FemLoadCase {
    FemLoadCase { id: id.into(), name: "Case é 🏋".into(), loads, self_weight: true }
}
fn pack(value: &Fem3dSnapshotValue) -> Vec<u8> {
    let body = store::pack_rt::encode_document(&Fem3dSnapshotValue::__dsl_spec(), &value.__dsl_to_record(), &Default::default()).unwrap();
    let envelope = store::semio_format::SemioEnvelope::from_envelope_id("fem.fem3d", store::semio_format::Component::Pack, 1).unwrap();
    store::semio_format::wrap_binary(&envelope, &body)
}
fn drain<T: RetireOwned>(value: T) {
    let mut close = semio_framework_value::retirement::controlled::ControlledRetirement::new(value).ok().unwrap();
    while !close.terminal_is_empty() {
        close.step(grant()).unwrap();
    }
}
fn receive(bytes: &[u8], g: RetainedCloneGrant, cut: usize) -> (Result<Fem3dSnapshot, ValueError>, usize) {
    let calls = std::cell::Cell::new(0);
    let mut callback = |_| {
        let n = calls.get();
        calls.set(n + 1);
        n < cut
    };
    let mut recipient = NativeDecodeRetirementRecipient::new();
    let mut native = NativeDecodeControl::new(256 * 1048576, &mut callback);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let mut owner = NativeSnapshotDecodeOwner::new(&mut native, g);
    let result = <Fem3dSnapshot as store::ArtifactPackReceiving>::receive_pack(bytes, &mut owner);
    assert!(owner.progress().copied_items <= g.maximum_items);
    drop(owner);
    native
        .with_retirement(grant(), |native| {
            while native.has_retirement_owner() {
                native.close_retirement_recipient(grant())?;
            }
            Ok(())
        })
        .unwrap();
    (result, calls.get())
}
#[test]
fn neutral_vectors_receive_actual_pack_and_preserve_all_selected_bits() {
    let cases = cases();
    assert_eq!(cases.schema, "semio.fem.native-load-receiving.v1");
    assert_eq!(cases.receiver.variants, ["nodal", "member-udl"]);
    assert_eq!(cases.receiver.self_weight, "retained");
    assert_eq!(cases.receiver.empty_cases, "retained");
    assert_eq!(cases.ownership.len(), 6);
    for (name, dof) in cases.dofs.iter().zip([FemDof::Tx, FemDof::Ty, FemDof::Tz, FemDof::Rx, FemDof::Ry, FemDof::Rz]) {
        for bits in &cases.bits {
            let bits: u64 = bits.parse().unwrap();
            let mut input = source();
            input.load_cases = vec![load_case("case", vec![nodal("load", dof, bits)]), load_case("empty", vec![])];
            let (output, _) = receive(&pack(&input), grant(), usize::MAX);
            let output = output.unwrap_or_else(|e| panic!("{name}/{bits}: {e:?}"));
            assert_eq!(output.load_cases.len(), 2);
            assert_eq!(output.load_cases[0].name, input.load_cases[0].name);
            assert!(output.load_cases[0].self_weight && output.load_cases[1].loads.is_empty());
            match &output.load_cases[0].loads[0] {
                FemLoad::Nodal { id, node_id, dof: actual, value } => {
                    assert_eq!(id, "load");
                    assert_eq!(node_id, "b");
                    assert_eq!(*actual, dof);
                    assert_eq!(value.to_bits(), bits);
                }
                _ => panic!("actual variant changed"),
            }
            let mut callback = |_| true;
            let mut recipient = semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
            let mut native = semio_framework_value::NativeEncodeControl::new(256 * 1048576, &mut callback);
            native.install_retirement_recipient(&mut recipient).unwrap();
            let mut owner = store::NativeSnapshotEncodeOwner::new(&mut native, grant());
            let store::io_schema::IoPayload::Binary(bytes) = output.encode_exchange_pack(&mut owner).unwrap() else { unreachable!() };
            assert_eq!(bytes, pack(&input));
            drop(owner);
            assert!(!native.has_retirement_owner());
            drain((output, bytes));
        }
    }
}
#[test]
fn original_namespace_index_admits_exact_case_scoped_identity_rows() {
    for case in cases().identities {
        let mut input = source();
        input.load_cases = case.cases.into_iter().map(|row| load_case(&row.id, row.loads.into_iter().map(|id| nodal(&id, FemDof::Tx, 0)).collect())).collect();
        let (output, _) = receive(&pack(&input), grant(), usize::MAX);
        assert_eq!(output.is_ok(), case.valid, "{}", case.id);
        if let Ok(output) = output {
            drain(output);
        }
    }
}
#[test]
fn canonical_member_udl_is_retained_but_missing_targets_and_area_are_refused() {
    let cases = cases();
    assert_eq!(cases.receiver.node_reference, "required");
    assert_eq!(cases.receiver.member_reference, "required");
    assert_eq!(cases.receiver.area, "unsupported");
    assert_eq!(cases.receiver.unknown, "invalid");
    let mut input = source();
    input.load_cases = vec![load_case("case", vec![FemLoad::MemberUdl { id: "u".into(), element_id: "e".into(), wx: -0., wy: 2., wz: -3. }])];
    let output = receive(&pack(&input), grant(), usize::MAX).0.unwrap();
    assert_eq!(output.load_cases[0], input.load_cases[0]);
    let FemLoad::MemberUdl { wx, .. } = &output.load_cases[0].loads[0] else { unreachable!() };
    assert_eq!(wx.to_bits(), (-0f64).to_bits());
    drain(output);
    for load in [
        FemLoad::Nodal { id: "n".into(), node_id: "absent".into(), dof: FemDof::Tx, value: 0. },
        FemLoad::MemberUdl { id: "u".into(), element_id: "absent".into(), wx: 0., wy: 0., wz: 0. },
        FemLoad::Area { id: "a".into(), solid_id: "absent".into(), pressure: 1. },
    ] {
        let area = matches!(&load, FemLoad::Area { .. });
        input.load_cases[0].loads = vec![load];
        assert_eq!(receive(&pack(&input), grant(), usize::MAX).0.unwrap_err().kind, if area { ValueRefusalKind::UnsupportedOwner } else { ValueRefusalKind::InvalidValue });
    }
}
#[test]
fn declared_variant_dof_and_scalar_shapes_are_closed() {
    let mut input = source();
    input.load_cases = vec![load_case("case", vec![nodal("load", FemDof::Tx, 0)])];
    for defect in 0..4 {
        let mut record = input.__dsl_to_record();
        let FieldValue::List(cases) = record.fields.get_mut(&6).unwrap() else { unreachable!() };
        let FieldValue::Record(case) = &mut cases[0] else { unreachable!() };
        if defect == 3 {
            case.fields.insert(3, FieldValue::Text("true".into()));
        } else {
            let FieldValue::Block(block) = case.fields.get_mut(&2).unwrap() else { unreachable!() };
            let FieldValue::Statements(loads) = block.as_mut() else { unreachable!() };
            match defect {
                0 => loads[0].0 = "aliased-nodal".into(),
                1 => {
                    loads[0].1.fields.insert(2, FieldValue::Enum(6));
                }
                _ => {
                    loads[0].1.fields.insert(1, FieldValue::UInt(1));
                }
            }
        }
        let mut callback = |_| true;
        let mut recipient = NativeDecodeRetirementRecipient::new();
        let mut native = NativeDecodeControl::new(256 * 1048576, &mut callback);
        native.install_retirement_recipient(&mut recipient).unwrap();
        let mut owner = NativeSnapshotDecodeOwner::new(&mut native, grant());
        assert_eq!(Fem3dSnapshot::receive_exchange_record(&record, &mut owner).unwrap_err().kind, ValueRefusalKind::InvalidValue);
        drop(owner);
        native
            .with_retirement(grant(), |native| {
                while native.has_retirement_owner() {
                    native.close_retirement_recipient(grant())?;
                }
                Ok(())
            })
            .unwrap();
    }
}
#[test]
fn every_actual_checkpoint_prefix_and_original_grant_denial_closes_without_publication() {
    let mut input = source();
    input.load_cases = vec![load_case("case", vec![nodal("load", FemDof::Rz, (-0f64).to_bits())])];
    let bytes = pack(&input);
    let (ready, calls) = receive(&bytes, grant(), usize::MAX);
    drain(ready.unwrap());
    for cut in 0..calls {
        let (output, _) = receive(&bytes, grant(), cut);
        assert_eq!(output.unwrap_err().kind, ValueRefusalKind::Canceled, "cut {cut}");
    }
    for g in [
        RetainedCloneGrant { maximum_items: 0, ..grant() },
        RetainedCloneGrant { maximum_copy_bytes: 0, ..grant() },
        RetainedCloneGrant { maximum_capacity_bytes: 0, ..grant() },
        RetainedCloneGrant { maximum_release_bytes: 0, ..grant() },
        RetainedCloneGrant { maximum_depth: 0, ..grant() },
    ] {
        assert!(receive(&bytes, g, usize::MAX).0.is_err());
    }
}
#[test]
fn scoped_index_rejects_substitute_source_and_oversized_words() {
    let mut input = source();
    input.load_cases = vec![load_case("case", vec![nodal("load", FemDof::Tx, 0)])];
    let substitute = input.clone();
    let mut recipient = NativeDecodeRetirementRecipient::new();
    let mut callback = |_| true;
    let mut native = NativeDecodeControl::new(256 * 1048576, &mut callback);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let mut owner = NativeSnapshotDecodeOwner::new(&mut native, grant());
    let index = FemIdentityIndex::receive(&input, &mut owner).unwrap();
    let before = owner.progress();
    assert_eq!(index.lookup(&substitute, FemIdentityKind::Node, "a", &mut owner).unwrap_err().kind, ValueRefusalKind::InvariantViolated);
    assert_eq!(owner.progress(), before);
    drop(owner);
    drain(index);
    input.load_cases[0].id = "é".repeat(32768);
    assert_eq!(receive(&pack(&input), grant(), usize::MAX).0.unwrap_err().kind, ValueRefusalKind::InvalidValue);
}
#[test]
fn identity_child_restores_original_finite_parent_stage_on_success_and_error() {
    for duplicate in [false, true] {
        let mut input = source();
        input.load_cases = vec![load_case("case", vec![nodal("load", FemDof::Tx, 0)])];
        if duplicate {
            input.load_cases.push(load_case("case", vec![]));
        }
        let last = std::cell::Cell::new((0usize, 0usize));
        let mut callback = |p: semio_framework_value_native_owned::native_decoding::NativeDecodeProgress| {
            last.set((p.completed, p.total));
            true
        };
        let mut recipient = NativeDecodeRetirementRecipient::new();
        let mut native = NativeDecodeControl::new(256 * 1048576, &mut callback);
        native.install_retirement_recipient(&mut recipient).unwrap();
        native.begin_stage(1).unwrap();
        let mut owner = NativeSnapshotDecodeOwner::new(&mut native, grant());
        let result = FemIdentityIndex::receive(&input, &mut owner);
        assert_eq!(result.is_err(), duplicate);
        assert!(owner.progress().copied_items > 0);
        if let Ok(index) = &result {
            assert_eq!(index.lookup(&input, FemIdentityKind::LoadCase, "case", &mut owner).unwrap(), 0);
            assert_eq!(index.lookup(&input, FemIdentityKind::Node, "a", &mut owner).unwrap(), 0);
        }
        drop(owner);
        if let Ok(index) = result {
            drain(index);
        }
        native
            .with_retirement(grant(), |native| {
                while native.has_retirement_owner() {
                    native.close_retirement_recipient(grant())?;
                }
                Ok(())
            })
            .unwrap();
        native.step().unwrap();
        native.checkpoint().unwrap();
        assert_eq!(last.get(), (1, 1));
    }
}
