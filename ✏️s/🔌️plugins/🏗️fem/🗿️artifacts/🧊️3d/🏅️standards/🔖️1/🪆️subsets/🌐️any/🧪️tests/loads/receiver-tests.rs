use super::*;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::{FemLoad, FemLoadCase};
use semio_framework_value::{NativeDecodeControl, RetainedCloneGrant};
use semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient;



#[derive(semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
struct IdentityCase {
    id: String,
    cases: Vec<CaseIds>,
    valid: bool,
}
#[derive(semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(deny_unknown_fields)]
struct CaseIds {
    id: String,
    loads: Vec<String>,
}
#[derive(semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
struct Receiver {
    variants: Vec<String>,
    unknown: String,
    self_weight: String,
    empty_cases: String,
    node_reference: String,
    member_reference: String,
    solid_reference: String,
}
#[derive(semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
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
fn exact(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn drain<T: RetireOwned>(value: T) {
    semio_s_fem_receiving::testkit::retire(value)
}
fn checked<T>(turn:semio_framework_value::RetainedWorkOutcome<T,store::NativeSnapshotFailure>,fuel:&mut u128)->Result<T,store::NativeSnapshotFailure>{*fuel=fuel.checked_sub(turn.work).expect("finite test work loan");turn.result}
fn failure_kind(error:store::NativeSnapshotFailure)->ValueRefusalKind{let kind=error.kind();drain(error);kind}
fn receive(bytes: &[u8], g: RetainedCloneGrant, cut: usize) -> (Result<Fem3dSnapshot, store::NativeSnapshotFailure>, usize) {
    let calls = std::cell::Cell::new(0);
    let mut callback = |_| {
        let n = calls.get();
        calls.set(n + 1);
        n < cut
    };
    let result = original_owners::with_decoding_granted(g, &mut callback, |owner| {
        let mut published = None;
        let result = <Fem3dSnapshot as store::ArtifactPackReceiving>::receive_pack(bytes, &mut published, owner);
        assert!(owner.progress().copied_items <= g.maximum_items);
        assert!(result.work <= 10_000_000);
        match result.result {
            Ok(()) => Ok(published.take().unwrap()),
            Err(error) => {
                if let Some(value) = published.take() {
                    drain(value)
                }
                Err(error)
            }
        }
    });
    (result, calls.get())
}
#[test]
fn neutral_vectors_receive_actual_pack_and_preserve_all_selected_bits() {
    let cases = cases();
    assert_eq!(cases.schema, "semio.fem.native-load-receiving.v1");
    assert_eq!(cases.receiver.variants, ["nodal", "member-udl", "area"]);
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
            let bytes = original_owners::with_encoding(|owner| {
                let mut published = None;
                let mut fuel = 10_000_000;
                checked(output.encode_exchange_pack(&mut published, owner), &mut fuel).unwrap();
                let store::io_schema::IoPayload::Binary(bytes) = published.unwrap() else { unreachable!() };
                bytes
            });
            assert_eq!(bytes, pack(&input));
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
        match output { Ok(output)=>drain(output), Err(error)=>drain(error) }
    }
}
#[test]
fn canonical_member_udl_is_retained_but_missing_targets_are_refused() {
    let cases = cases();
    assert_eq!(cases.receiver.node_reference, "required");
    assert_eq!(cases.receiver.member_reference, "required");
    assert_eq!(cases.receiver.solid_reference, "required");
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
        input.load_cases[0].loads = vec![load];
        assert_eq!(failure_kind(receive(&pack(&input), grant(), usize::MAX).0.unwrap_err()), ValueRefusalKind::InvalidValue);
    }
}
#[test]
fn declared_variant_dof_and_scalar_shapes_are_closed() {
    with_lend(grant(), |loan| {
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
        original_owners::with_decoding_granted(grant(), &mut |_| true, |owner| {
            let mut work_fuel=10_000_000;
            let mut published=None;let error=checked(Fem3dSnapshot::receive_exchange_record(&record, &mut published, owner),&mut work_fuel).unwrap_err();let value=published.take();let absent=value.is_none();if let Some(value)=value{drain(value)}assert!(absent);assert_eq!(failure_kind(error), ValueRefusalKind::InvalidValue);
        });
    }
    })
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
        assert_eq!(failure_kind(output.unwrap_err()), ValueRefusalKind::Canceled, "cut {cut}");
    }
    for g in [
        RetainedCloneGrant { maximum_items: 0, ..grant() },
        RetainedCloneGrant { maximum_copy_bytes: 0, ..grant() },
        RetainedCloneGrant { maximum_capacity_bytes: 0, ..grant() },
        RetainedCloneGrant { maximum_release_bytes: 0, ..grant() },
        RetainedCloneGrant { maximum_depth: 0, ..grant() },
    ] {
        let error=receive(&bytes, g, usize::MAX).0.unwrap_err();drain(error);
    }
}
#[test]
fn scoped_index_rejects_substitute_source_and_oversized_words() {
    let mut input = source();
    input.load_cases = vec![load_case("case", vec![nodal("load", FemDof::Tx, 0)])];
    let substitute = input.clone();
    let index = original_owners::with_decoding_granted(grant(), &mut |_| true, |owner| {
        let mut fuel = 10_000_000;
        let mut published = None;
        checked(FemIdentityIndex::receive(&input, &mut published, owner), &mut fuel).unwrap();
        published.unwrap()
    });
    original_owners::with_decoding_granted(grant(), &mut |_| true, |owner| {
        let mut fuel = 10_000_000;
        let mut found = None;
        let error = checked(index.lookup(&substitute, FemIdentityKind::Node, "a", &mut found, owner), &mut fuel).unwrap_err();
        assert_eq!(failure_kind(error), ValueRefusalKind::InvariantViolated);
        assert!(found.is_none());
    });
    drain(index);
    input.load_cases[0].id = "é".repeat(32768);
    assert_eq!(failure_kind(receive(&pack(&input), grant(), usize::MAX).0.unwrap_err()), ValueRefusalKind::InvalidValue);
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
        original_owners::with_decoding_granted(grant(), &mut callback, |owner| {
            owner.native().begin_stage(1).unwrap();
            let mut fuel = 10_000_000;
            let mut published = None;
            let result = checked(FemIdentityIndex::receive(&input, &mut published, owner), &mut fuel);
            assert_eq!(result.is_err(), duplicate);
            assert!(owner.progress().copied_items > 0);
            match result {
                Ok(()) => drain(published.unwrap()),
                Err(error) => {
                    assert!(published.is_none());
                    drain(error);
                }
            }
            owner.native().step().unwrap();
            owner.native().checkpoint().unwrap();
            assert_eq!(last.get(), (1, 1));
        });
    }
}
#[test]
fn a_row_failing_validation_is_never_used() {
    for (case, damage) in [("material modulus", 0usize), ("node identity", 1usize)] {
        let mut input = source();
        match damage {
            0 => input.materials[0].e = -1.0,
            _ => input.nodes[0].id = String::new(),
        }
        let (result, _) = receive(&pack(&input), grant(), usize::MAX);
        let error = result.err().unwrap_or_else(|| panic!("{case}: an invalid row produced a usable snapshot"));
        assert_eq!(failure_kind(error), ValueRefusalKind::InvalidValue, "{case}");
    }
}
#[test]
fn area_load_is_retained_exactly_and_a_dangling_solid_reference_is_refused() {
    use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::{FemAxis, FemSolid};
    let solid = FemSolid { id: "slab".into(), name: "Slab".into(), outline: vec![[0., 0.], [1., 0.], [1., 1.], [0., 1.]], holes: vec![], base_z: 0., height: 0.2, layers: 2, mesh_size: 0.5, material_id: "m".into(), axis: FemAxis::Z };
    for target in ["slab", "absent"] {
        let mut input = source();
        input.solids = vec![solid.clone()];
        input.load_cases = vec![load_case("case", vec![FemLoad::Area { id: "a".into(), solid_id: target.into(), pressure: -1e3 }])];
        let record = input.__dsl_to_record();
        original_owners::with_decoding_granted(grant(), &mut |_| true, |owner| {
            let mut work_fuel = 10_000_000;
            let mut published = None;
            let outcome = checked(Fem3dSnapshot::receive_exchange_record(&record, &mut published, owner), &mut work_fuel);
            match outcome {
                Ok(()) => {
                    assert_eq!(target, "slab");
                    let output = published.take().unwrap();
                    assert_eq!(output.load_cases[0], input.load_cases[0]);
                    assert_eq!(output.solids[0], input.solids[0]);
                    drain(output);
                }
                Err(error) => {
                    assert_eq!(target, "absent");
                    assert!(published.is_none());
                    assert_eq!(failure_kind(error), ValueRefusalKind::InvalidValue);
                }
            }
        });
    }
}
