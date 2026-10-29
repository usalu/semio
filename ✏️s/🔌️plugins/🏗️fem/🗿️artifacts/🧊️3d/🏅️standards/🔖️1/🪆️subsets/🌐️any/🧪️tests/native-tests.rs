use super::*;
use semio_framework_value_native_owned::native_decoding::NativeDecodeConstructionRecipient;
fn original() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 16384, maximum_capacity_bytes: 65536, maximum_release_bytes: 65536, maximum_depth: 64 }
}
struct Recipient;
impl NativeDecodeConstructionRecipient for Recipient {
    fn grant(&mut self, d: RetirementDemand) -> Result<RetainedCloneGrant, ValueError> {
        let g = original();
        assert!(retained_clone_grant_admits(g, d));
        Ok(g)
    }
    fn accept(&mut self, g: RetainedCloneGrant, p: RetainedCloneProgress) -> Result<(), ValueError> {
        assert!(p.fits(g));
        Ok(())
    }
}
fn authoring() -> Fem3dSnapshotValue {
    let mut s = Fem3dSnapshotValue::default();
    s.nodes = vec![FemNodeValue { id: "a".into(), x: 1., y: 2., z: 3. }, FemNodeValue { id: "b".into(), x: 4., y: 5., z: 6. }];
    s.materials = vec![FemMaterialValue { id: "steel".into(), name: "Steel".into(), e: 2e11, g: 8e10, nu: 0.25, rho: 7850. }];
    s
}
fn close(rx: &mut Fem3dReceiving) {
    rx.begin_close();
    for _ in 0..100000 {
        if rx.terminal_is_empty() {
            return;
        }
        let d = rx.close_demands(original().maximum_copy_bytes).unwrap();
        assert!(retained_clone_grant_admits(original(), d));
        let p = rx.close_step(original()).unwrap().progress();
        assert!(p.fits(original()));
    }
    panic!("receiving failed to close");
}
#[test]
fn native_owner_receives_actual_persistent_rows_and_closes_every_cancelled_prefix() {
    for cut in 0..10000 {
        let (mut rx, p) = Fem3dReceiving::capture(authoring(), original()).map_err(|(e, _)| e).unwrap();
        assert!(p.fits(original()));
        let running = std::cell::Cell::new(true);
        let mut callback = |_| running.get();
        let mut control = NativeDecodeControl::new(1_000_000, &mut callback);
        let mut born = 0;
        control
            .with_construction_recipient(&mut Recipient, |control| -> Result<(), ValueError> {
                for _ in 0..cut {
                    if rx.ready() {
                        break;
                    }
                    let d = rx.demands(original().maximum_copy_bytes)?;
                    assert!(retained_clone_grant_admits(original(), d));
                    let p = rx.advance(control, original().maximum_copy_bytes, original())?.progress();
                    assert!(p.fits(original()));
                    born += p.retained_capacity_bytes;
                    assert_eq!(control.owned_bytes(), born);
                }
                Ok(())
            })
            .unwrap();
        let ready = rx.ready();
        if !ready {
            running.set(false);
            assert_eq!(rx.advance(&mut control, original().maximum_copy_bytes, original()).unwrap_err().kind, ValueRefusalKind::Canceled);
        }
        if ready {
            let (value, p) = rx.take(original()).unwrap().unwrap();
            assert!(p.fits(original()));
            assert_eq!(value.nodes.len(), 2);
            assert_eq!(value.nodes.get(0).unwrap().id.as_str(), "a");
            assert_eq!(value.nodes.get(1).unwrap().z, 6.);
            assert_eq!(value.materials.get(0).unwrap().name.as_str(), "Steel");
            drain(value);
        }
        close(&mut rx);
        assert!(rx.terminal_is_empty());
        drop(rx);
        if ready {
            assert!(cut > 20);
            return;
        }
    }
    panic!("receiving never became ready");
}
#[test]
fn denied_capture_and_physical_frontier_preserve_original_owner() {
    let value = authoring();
    let pointer = value.nodes.as_ptr();
    let denied = RetainedCloneGrant { maximum_capacity_bytes: 0, maximum_copy_bytes: 0, ..original() };
    let (error, value) = Fem3dReceiving::capture(value, denied).err().unwrap();
    assert_eq!(error.kind, ValueRefusalKind::OwnershipLimit);
    assert_eq!(value.nodes.as_ptr(), pointer);
    let (mut rx, _) = Fem3dReceiving::capture(value, original()).map_err(|(e, _)| e).unwrap();
    let mut callback = |_| true;
    let mut control = NativeDecodeControl::new(1_000_000, &mut callback);
    let before = rx.demands(4096).unwrap();
    let no_copy = RetainedCloneGrant { maximum_copy_bytes: 0, ..original() };
    assert_eq!(rx.advance(&mut control, 4096, no_copy).unwrap().progress(), RetainedCloneProgress::default());
    assert_eq!(control.owned_bytes(), 0);
    assert_eq!(rx.advance(&mut control, 4096, denied).unwrap().progress(), RetainedCloneProgress::default());
    assert_eq!(rx.demands(4096).unwrap(), before);
    close(&mut rx);
}

fn wallet() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1_000_000, maximum_copy_bytes: 128 * 1024 * 1024, maximum_capacity_bytes: 128 * 1024 * 1024, maximum_release_bytes: 128 * 1024 * 1024, maximum_depth: 128 }
}
fn pack(value: &Fem3dSnapshotValue) -> Vec<u8> {
    let body = store::pack_rt::encode_document(&Fem3dSnapshotValue::__dsl_spec(), &value.__dsl_to_record(), &Default::default()).unwrap();
    let envelope = store::semio_format::SemioEnvelope::from_envelope_id("fem.fem3d", store::semio_format::Component::Pack, 1).unwrap();
    store::semio_format::wrap_binary(&envelope, &body)
}
#[test]
fn actual_pack_receiving_retains_original_wallet_and_persistent_output() {
    let source = pack(&authoring());
    let mut recipient = semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient::new();
    let mut callback = |_| true;
    let mut native = NativeDecodeControl::new(128 * 1024 * 1024, &mut callback);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let mut owner = store::NativeSnapshotDecodeOwner::new(&mut native, wallet());
    let output = <Fem3dSnapshot as store::ArtifactPackReceiving>::receive_pack(&source, &mut owner).unwrap();
    assert_eq!(output.nodes.get(1).unwrap().z, 6.);
    assert_eq!(output.materials.get(0).unwrap().e, 2e11);
    assert!(owner.progress().retained_capacity_bytes > 0);
    drop(owner);
    assert!(!native.has_retirement_owner());
    let expected = pack::snapshot::ArtifactPack::encode_pack(&output);
    let mut publish = |_| true;
    let mut encode_recipient = semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
    let mut encode = semio_framework_value::NativeEncodeControl::new(128 * 1024 * 1024, &mut publish);
    encode.install_retirement_recipient(&mut encode_recipient).unwrap();
    let mut owner = store::NativeSnapshotEncodeOwner::new(&mut encode, wallet());
    let payload = output.encode_exchange_pack(&mut owner).unwrap();
    let store::io_schema::IoPayload::Binary(bytes) = payload else { unreachable!() };
    assert_eq!(bytes, expected);
    assert!(owner.progress().retained_capacity_bytes > 0);
    drop(owner);
    assert!(!encode.has_retirement_owner());
    let mut close = semio_framework_value::retirement::controlled::ControlledRetirement::new((output, bytes)).ok().unwrap();
    while !close.terminal_is_empty() {
        close.step(wallet()).unwrap();
    }
}
#[test]
fn actual_pack_cancelled_inside_large_material_name_keeps_original_retirement_owner() {
    let mut input = authoring();
    input.materials[0].name = "Steel".repeat(30000);
    let source = pack(&input);
    let mut recipient = semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient::new();
    let mut callback = |p: semio_framework_value_native_owned::native_decoding::NativeDecodeProgress| p.owned_bytes < 70000;
    let mut native = NativeDecodeControl::new(128 * 1024 * 1024, &mut callback);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let mut owner = store::NativeSnapshotDecodeOwner::new(&mut native, wallet());
    let error = <Fem3dSnapshot as store::ArtifactPackReceiving>::receive_pack(&source, &mut owner).unwrap_err();
    assert_eq!(error.kind, ValueRefusalKind::Canceled);
    drop(owner);
    assert!(native.has_retirement_owner());
    native
        .with_retirement(wallet(), |native| {
            for _ in 0..100000 {
                if !native.has_retirement_owner() {
                    return Ok(());
                }
                native.close_retirement_recipient(wallet())?;
            }
            Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "retirement never closed"))
        })
        .unwrap();
    assert!(!native.has_retirement_owner());
}

#[test]
fn actual_owner_receives_a_ten_thousand_node_fixture_with_bounded_identity_work() {
    let count = 10000;
    let mut value = Some(Fem3dSnapshotValue::default());
    value.as_mut().unwrap().nodes = (0..count).map(|i| FemNodeValue { id: format!("n{i:05}"), x: i as f64, y: 0., z: 0. }).collect();
    let grant = RetainedCloneGrant { maximum_items: 10_000_000, maximum_copy_bytes: 512 * 1048576, maximum_capacity_bytes: 512 * 1048576, maximum_release_bytes: 512 * 1048576, maximum_depth: 128 };
    let mut observer = |_| true;
    let mut recipient = semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient::new();
    let mut native = NativeDecodeControl::new(512 * 1048576, &mut observer);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let mut owner = store::NativeSnapshotDecodeOwner::new(&mut native, grant);
    let output = Fem3dSnapshot::receive_authoring(&mut value, &mut owner).unwrap();
    assert_eq!(output.nodes.len(), count);
    assert_eq!(output.nodes.get(count - 1).unwrap().id.as_str(), "n09999");
    assert!(owner.progress().copied_items < grant.maximum_items);
    drop(owner);
    assert!(!native.has_retirement_owner());
    let mut close = semio_framework_value::retirement::controlled::ControlledRetirement::new(output).ok().unwrap();
    while !close.terminal_is_empty() {
        close.step(grant).unwrap();
    }
}

#[test]
fn actual_chain_receiving_identity_and_reference_work_scales_below_threefold() {
    fn measured(count: usize) -> usize {
        use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::{FemElement, FemSection};
        let mut value = authoring();
        value.nodes = (0..count).map(|i| FemNodeValue { id: format!("n{i:05}"), x: i as f64, y: 0., z: 0. }).collect();
        value.sections = vec![FemSection { id: "section".into(), name: "Section".into(), area: 0.01, iy: 1e-5, iz: 2e-5, j: 3e-5 }];
        value.elements = (0..count - 1).map(|i| FemElement::Bar { id: format!("e{i:05}"), start: format!("n{i:05}"), end: format!("n{:05}", i + 1), material_id: "steel".into(), section_id: "section".into() }).collect();
        let mut value = Some(value);
        let mut observer = |_| true;
        let mut recipient = semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient::new();
        let mut native = NativeDecodeControl::new(128 * 1048576, &mut observer);
        native.install_retirement_recipient(&mut recipient).unwrap();
        let mut owner = store::NativeSnapshotDecodeOwner::new(&mut native, wallet());
        let output = Fem3dSnapshot::receive_authoring(&mut value, &mut owner).unwrap();
        assert_eq!(output.elements.len(), count - 1);
        let work = owner.progress().copied_items;
        drop(owner);
        let mut close = semio_framework_value::retirement::controlled::ControlledRetirement::new(output).ok().unwrap();
        while !close.terminal_is_empty() {
            close.step(wallet()).unwrap();
        }
        work
    }
    assert!(measured(1024) < 3 * measured(512));
}
fn drain<T: RetireOwned>(value: T) {
    let mut owner = semio_framework_value::retirement::controlled::ControlledRetirement::new(value).ok().unwrap();
    while !owner.terminal_is_empty() {
        owner.step(original()).unwrap();
    }
}
