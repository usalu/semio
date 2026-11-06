use super::*;
use semio_framework_value::RetainedCloneStep;
use semio_framework_value_native_owned::native_decoding::{NativeDecodeConstructionRecipient, NativeDecodeWorkRecipient};
/// 🎟️ These finite lower-level loans do not stand in for job authority.
struct Loan { remaining:u64 }
impl NativeDecodeWorkRecipient for Loan {fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=self.remaining;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=self.remaining))}}
struct EncodeLoan { remaining:u64, posted:u64 }
impl semio_framework_value::native_encoding::NativeEncodeWorkRecipient for EncodeLoan {fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=self.remaining))}fn committed(&mut self,actual:u64)->Result<(),ValueError>{self.posted=self.posted.checked_add(actual).filter(|total|*total<=self.remaining).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"finite encoder test loan exhausted"))?;Ok(())}}
fn checked<T>(turn:semio_framework_value::RetainedWorkOutcome<T,store::NativeSnapshotFailure>,fuel:&mut u64)->Result<T,store::NativeSnapshotFailure>{*fuel=fuel.checked_sub(turn.work).expect("finite test work loan");turn.result}
fn failure_kind(error:store::NativeSnapshotFailure)->ValueRefusalKind{let kind=error.kind();drain(error);kind}
fn advance(rx:&mut Fem3dReceiving,control:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant,fuel:&mut u64)->semio_framework_value::RetainedWorkOutcome<RetainedCloneStep,store::NativeSnapshotFailure>{
 let mut loan=Loan{remaining:*fuel};let mut recipient=Recipient{submitted:Default::default(),remaining:grant};
 let result=control.with_work_recipient(&mut loan,|control|control.with_construction_recipient(&mut recipient,|control|Ok::<_,ValueError>(rx.advance(control,body,grant))));
 let turn=match result{Ok(turn)=>turn,Err(error)=>semio_framework_value::RetainedWorkOutcome{work:0,result:Err(error.into())}};
 *fuel=fuel.checked_sub(turn.work).expect("finite test work loan");turn
}
fn original() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 16384, maximum_capacity_bytes: 65536, maximum_release_bytes: 65536, maximum_depth: 64 }
}
struct Recipient{submitted:RetainedCloneProgress,remaining:RetainedCloneGrant}
impl NativeDecodeConstructionRecipient for Recipient {
 fn submitted(&self)->RetainedCloneProgress{self.submitted}fn grant(&mut self,d:RetirementDemand)->Result<RetainedCloneGrant,ValueError>{
  if !retained_clone_grant_admits(self.remaining,d){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"finite test construction loan exhausted"))}
  Ok(self.remaining)
 }
 fn accept(&mut self,g:RetainedCloneGrant,p:RetainedCloneProgress)->Result<(),ValueError>{self.submitted=self.submitted.checked_add(p)?;
  assert!(p.fits(g)&&p.fits(self.remaining));
  self.remaining.maximum_items-=p.copied_items;self.remaining.maximum_copy_bytes-=p.copied_bytes;
  self.remaining.maximum_capacity_bytes-=p.retained_capacity_bytes;self.remaining.maximum_release_bytes-=p.released_bytes;Ok(())
 }
}
fn authoring() -> Fem3dSnapshotValue {
    let mut s = Fem3dSnapshotValue::default();
    s.nodes = vec![FemNodeValue { id: "a".into(), x: 1., y: 2., z: 3. }, FemNodeValue { id: "b".into(), x: 4., y: 5., z: 6. }];
    s.materials = vec![FemMaterialValue { id: "steel".into(), name: "Steel".into(), e: 2e11, g: 8e10, nu: 0.25, rho: 7850. }];
    s
}
fn admitted_receiver(value:Fem3dSnapshotValue,g:RetainedCloneGrant)->(Fem3dReceiving,RetainedCloneProgress){
    let born=Fem3dReceiving::empty(g);let mut fuel=2u64;fuel=fuel.checked_sub(born.work).unwrap();assert_eq!(born.work,1);let(mut receiver,birth)=born.result.unwrap();assert_eq!(birth.copied_items,1);assert_eq!(birth.copied_bytes,std::mem::size_of::<Fem3dReceiving>());let mut source=Some(value);let turn=receiver.admit_source(&mut source,g);fuel=fuel.checked_sub(turn.work).unwrap();let progress=turn.result.unwrap().progress();assert_eq!(turn.work,1);assert_eq!(progress.copied_items,1);assert_eq!(progress.copied_bytes,0);assert!(source.is_none());assert_eq!(fuel,0);(receiver,birth)
}
fn close(rx: &mut Fem3dReceiving) {
    rx.begin_close();
    let mut retirement_fuel=10_000_000u64;
    for _ in 0..100000 {
        if rx.terminal_is_empty() {
            return;
        }
        let d = rx.close_demands(original().maximum_copy_bytes).unwrap();
        assert!(retained_clone_grant_admits(original(), d));
        let turn=rx.close_step(exact(d));retirement_fuel=retirement_fuel.checked_sub(turn.work).expect("finite retirement work loan");let p=turn.result.unwrap().progress();
        assert!(p.fits(original()));
        assert!(p.fits(exact(d)));
    }
    panic!("receiving failed to close");
}
#[test]
fn native_owner_receives_actual_persistent_rows_and_closes_every_cancelled_prefix() {
    for cut in 0..10000 {
        let (mut rx, p) = admitted_receiver(authoring(),original());
        assert!(p.fits(original()));
        let running = std::cell::Cell::new(true);
        let mut callback = |_| running.get();
        let mut control = NativeDecodeControl::new(1_000_000, &mut callback);
        let mut work_fuel=10_000_000;
        let mut born = 0;
        for _ in 0..cut {
            if rx.ready(){break}
            let d=rx.demands(original().maximum_copy_bytes).unwrap();
            assert!(retained_clone_grant_admits(original(),d));
            let turn=advance(&mut rx,&mut control,original().maximum_copy_bytes,original(),&mut work_fuel);
            let p=turn.result.unwrap().progress();
            assert!(p.fits(original()));born+=p.retained_capacity_bytes;assert_eq!(control.owned_bytes(),born);
        }
        let ready = rx.ready();
        if !ready {
            running.set(false);
            assert_eq!(failure_kind(advance(&mut rx,&mut control,original().maximum_copy_bytes,original(),&mut work_fuel).result.unwrap_err()), ValueRefusalKind::Canceled);
        }
        if ready {
            let turn=rx.take(original());assert!(turn.work<=1);let (value,p)=turn.result.unwrap().unwrap();
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
    let denied_birth=Fem3dReceiving::empty(denied);assert_eq!(denied_birth.work,0);assert_eq!(denied_birth.result.err().unwrap().kind,ValueRefusalKind::OwnershipLimit);
    assert_eq!(value.nodes.as_ptr(), pointer);
    let(mut rx,_)=Fem3dReceiving::empty(original()).result.unwrap();let mut value=Some(value);
    let no_items=RetainedCloneGrant{maximum_items:0,..original()};let turn=rx.admit_source(&mut value,no_items);assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),RetainedCloneProgress::default());assert_eq!(value.as_ref().unwrap().nodes.as_ptr(),pointer);
    let turn=rx.admit_source(&mut value,original());assert_eq!(turn.work,1);assert_eq!(turn.result.unwrap().progress().copied_items,1);assert!(value.is_none());
    let mut callback = |_| true;
    let mut control = NativeDecodeControl::new(1_000_000, &mut callback);
    let mut work_fuel=10_000_000;
    let mut turns = 0;
    while rx.demands(4096).unwrap().capacity_bytes == 0 {
        assert!(turns < 100_000 && !rx.terminal_is_empty(), "no capacity frontier reached");
        advance(&mut rx,&mut control,4096,original(),&mut work_fuel).result.unwrap();
        turns += 1;
    }
    let before = rx.demands(4096).unwrap();
    let no_capacity = RetainedCloneGrant { maximum_capacity_bytes: 0, ..original() };
    assert_eq!(advance(&mut rx,&mut control,4096,no_capacity,&mut work_fuel).result.unwrap().progress(), RetainedCloneProgress::default());
    assert_eq!(control.owned_bytes(), 0);
    assert_eq!(advance(&mut rx,&mut control,4096,denied,&mut work_fuel).result.unwrap().progress(), RetainedCloneProgress::default());
    assert_eq!(rx.demands(4096).unwrap(), before);
    close(&mut rx);
}
#[test]
fn copy_denial_at_a_copying_frontier_makes_no_progress_and_keeps_the_demand() {
    let (mut rx, _) = admitted_receiver(authoring(),original());
    let mut callback = |_| true;
    let mut control = NativeDecodeControl::new(1_000_000, &mut callback);
    let mut work_fuel=10_000_000;
    let mut turns = 0;
    while rx.demands(4096).unwrap().copy_bytes == 0 {
        assert!(turns < 100_000 && !rx.terminal_is_empty(), "no copying frontier reached");
        advance(&mut rx,&mut control,4096,original(),&mut work_fuel).result.unwrap();
        turns += 1;
    }
    let before = rx.demands(4096).unwrap();
    let no_copy = RetainedCloneGrant { maximum_copy_bytes: 0, ..original() };
    assert_eq!(advance(&mut rx,&mut control,4096,no_copy,&mut work_fuel).result.unwrap().progress(), RetainedCloneProgress::default());
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
    let output = native
        .with_work_recipient(&mut Loan { remaining:10_000_000 }, |native| {
            let mut work_fuel=10_000_000;
            let mut owner = store::NativeSnapshotDecodeOwner::new(native, wallet());
            let mut published=None;checked(<Fem3dSnapshot as store::ArtifactPackReceiving>::receive_pack(&source,&mut published,&mut owner),&mut work_fuel).unwrap();let output=published.take().unwrap();
            assert_eq!(output.nodes.get(1).unwrap().z, 6.);
            assert_eq!(output.materials.get(0).unwrap().e, 2e11);
            assert!(owner.progress().retained_capacity_bytes > 0);
            Ok::<_, ValueError>(output)
        })
        .unwrap();
    assert!(!native.has_retirement_owner());
    let expected = pack::snapshot::ArtifactPack::encode_pack(&output);
    let mut publish = |_| true;
    let mut encode_recipient = semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
    let mut encode = semio_framework_value::NativeEncodeControl::new(128 * 1024 * 1024, &mut publish);
    encode.install_retirement_recipient(&mut encode_recipient).unwrap();
    let payload=encode.with_work_recipient(&mut EncodeLoan { remaining:10_000_000,posted:0 }, |encode| {
        let mut work_fuel=10_000_000;
        let mut owner=store::NativeSnapshotEncodeOwner::new(encode,wallet());
        let mut published=None;checked(output.encode_exchange_pack(&mut published,&mut owner),&mut work_fuel).unwrap();let payload=published.take().unwrap();
        assert!(owner.progress().retained_capacity_bytes>0);
        Ok::<_,ValueError>(payload)
    }).unwrap();
    let store::io_schema::IoPayload::Binary(bytes) = payload else { unreachable!() };
    assert_eq!(bytes, expected);
    assert!(!encode.has_retirement_owner());
    let mut close = semio_framework_value::retirement::controlled::ControlledRetirement::new((output, bytes)).ok().unwrap();
    let mut retirement_fuel=10_000_000u64;
    while !close.terminal_is_empty() {
        let demand=close.next_demand(4096).unwrap();assert!(retained_clone_grant_admits(wallet(),demand));let offered=exact(demand);let step=close.step(offered).unwrap();assert!(step.progress().fits(offered));
        retirement_fuel=retirement_fuel.checked_sub(close.last_turn_work()).expect("finite retirement work loan");
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
    native
        .with_work_recipient(&mut Loan { remaining:10_000_000 }, |native| {
            let mut work_fuel=10_000_000;
            let mut owner = store::NativeSnapshotDecodeOwner::new(native, wallet());
            let mut published=None;let error=checked(<Fem3dSnapshot as store::ArtifactPackReceiving>::receive_pack(&source,&mut published,&mut owner),&mut work_fuel).unwrap_err();assert!(published.is_none());
            assert_eq!(failure_kind(error), ValueRefusalKind::Canceled);
            Ok::<_, ValueError>(())
        })
        .unwrap();
    assert!(native.has_retirement_owner());
    native.with_work_recipient(&mut Loan { remaining:10_000_000 }, |native| native
        .with_retirement(wallet(), |native| {
            let mut retirement_fuel=10_000_000u64;
            for _ in 0..100000 {
                if !native.has_retirement_owner() {
                    return Ok(());
                }
                let demand=native.retirement_demands(4096)?;assert!(retained_clone_grant_admits(wallet(),demand));let offered=exact(demand);let turn=native.close_retirement_recipient(offered);retirement_fuel=retirement_fuel.checked_sub(turn.work).expect("finite retirement work loan");let step=turn.result?;assert!(step.progress().fits(offered));
            }
            Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "retirement never closed"))
        }))
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
    let output = native
        .with_work_recipient(&mut Loan { remaining:10_000_000 }, |native| {
            let mut work_fuel=10_000_000;
            let mut owner = store::NativeSnapshotDecodeOwner::new(native, grant);
            let mut published=None;checked(Fem3dSnapshot::receive_authoring(&mut value,&mut published,&mut owner),&mut work_fuel).unwrap();let output=published.take().unwrap();
            assert_eq!(output.nodes.len(), count);
            assert_eq!(output.nodes.get(count - 1).unwrap().id.as_str(), "n09999");
            assert!(owner.progress().copied_items < grant.maximum_items);
            Ok::<_, ValueError>(output)
        })
        .unwrap();
    assert!(!native.has_retirement_owner());
    let mut close = semio_framework_value::retirement::controlled::ControlledRetirement::new(output).ok().unwrap();
    let mut retirement_fuel=10_000_000u64;
    while !close.terminal_is_empty() {
        let demand=close.next_demand(4096).unwrap();assert!(retained_clone_grant_admits(grant,demand));let offered=exact(demand);let step=close.step(offered).unwrap();assert!(step.progress().fits(offered));
        retirement_fuel=retirement_fuel.checked_sub(close.last_turn_work()).expect("finite retirement work loan");
    }
}

#[test]
fn actual_chain_receiving_identity_and_reference_work_scales_below_threefold() {
    fn measured(count: usize) -> (usize,u64) {
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
        let (output, work, selected_work) = native
            .with_work_recipient(&mut Loan { remaining:10_000_000 }, |native| {
                let mut work_fuel=10_000_000;
            let mut owner = store::NativeSnapshotDecodeOwner::new(native, wallet());
                let mut published=None;let turn=Fem3dSnapshot::receive_authoring(&mut value,&mut published,&mut owner);let selected_work=turn.work;checked(turn,&mut work_fuel).unwrap();let output=published.take().unwrap();
                assert_eq!(output.elements.len(), count - 1);
                let work = owner.progress().copied_items;
                Ok::<_, ValueError>((output, work, selected_work))
            })
            .unwrap();
        let mut close = semio_framework_value::retirement::controlled::ControlledRetirement::new(output).ok().unwrap();
        let mut retirement_fuel=10_000_000u64;
        while !close.terminal_is_empty() {
            let demand=close.next_demand(4096).unwrap();assert!(retained_clone_grant_admits(wallet(),demand));let offered=exact(demand);let step=close.step(offered).unwrap();assert!(step.progress().fits(offered));
            retirement_fuel=retirement_fuel.checked_sub(close.last_turn_work()).expect("finite retirement work loan");
        }
        (work,selected_work)
    }
    let large=measured(1024);let small=measured(512);
    assert!(large.0 < 3 * small.0);
    assert!(large.1 < 3 * small.1);
}
fn exact(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn drain<T: RetireOwned>(value: T) {
    let mut owner = semio_framework_value::retirement::controlled::ControlledRetirement::new(value).ok().unwrap();
    let mut retirement_fuel=10_000_000u64;
    while !owner.terminal_is_empty() {
        let demand=owner.next_demand(4096).unwrap();
        assert!(retained_clone_grant_admits(original(),demand));
        let step=owner.step(exact(demand)).unwrap();
        assert!(step.progress().fits(exact(demand)));
        retirement_fuel=retirement_fuel.checked_sub(owner.last_turn_work()).expect("finite retirement work loan");
    }
}

#[test]
fn receiving_charges_below_sixty_four_kibibytes_of_retained_capacity_per_node() {
    let count = 1000;
    let mut value = Some(Fem3dSnapshotValue::default());
    value.as_mut().unwrap().nodes = (0..count).map(|i| FemNodeValue { id: format!("n{i:05}"), x: i as f64, y: 0., z: 0. }).collect();
    let limit = count * 64 * 1024;
    let grant = RetainedCloneGrant { maximum_items: 10_000_000, maximum_copy_bytes: 512 * 1048576, maximum_capacity_bytes: limit, maximum_release_bytes: 512 * 1048576, maximum_depth: 128 };
    let mut observer = |_| true;
    let mut recipient = semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient::new();
    let mut native = NativeDecodeControl::new(limit, &mut observer);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let output = native
        .with_work_recipient(&mut Loan { remaining:10_000_000 }, |native| {
            let mut work_fuel=10_000_000;
            let mut owner = store::NativeSnapshotDecodeOwner::new(native, grant);
            let mut published=None;checked(Fem3dSnapshot::receive_authoring(&mut value,&mut published,&mut owner),&mut work_fuel).unwrap();let output=published.take().unwrap();
            assert!(owner.progress().retained_capacity_bytes <= limit);
            Ok::<_, ValueError>(output)
        })
        .unwrap();
    let mut close = semio_framework_value::retirement::controlled::ControlledRetirement::new(output).ok().unwrap();
    let mut retirement_fuel=10_000_000u64;
    while !close.terminal_is_empty() {
        let demand=close.next_demand(4096).unwrap();assert!(retained_clone_grant_admits(grant,demand));let offered=exact(demand);let step=close.step(offered).unwrap();assert!(step.progress().fits(offered));
        retirement_fuel=retirement_fuel.checked_sub(close.last_turn_work()).expect("finite retirement work loan");
    }
}

#[test]
fn debug_per_step_capacity() {
    const GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 10_000_000, maximum_copy_bytes: 1 << 26, maximum_capacity_bytes: 1 << 28, maximum_release_bytes: 1 << 28, maximum_depth: 128 };
    for n in [1usize, 3] {
        let mut v = Fem3dSnapshotValue::default();
        v.nodes = (0..n).map(|i| FemNodeValue { id: format!("n{i}"), x: 0., y: 0., z: 0. }).collect();
        let (mut rx, _) = admitted_receiver(v,GRANT);
        let mut callback = |_| true;
        let mut control = NativeDecodeControl::new(1 << 28, &mut callback);
        let mut work_fuel=10_000_000;
        let (mut steps, mut total) = (0usize, 0usize);
        while !rx.ready(){
            let p=advance(&mut rx,&mut control,GRANT.maximum_copy_bytes,GRANT,&mut work_fuel).result.unwrap().progress();
            steps+=1;total+=p.retained_capacity_bytes;
            if p.retained_capacity_bytes>=256{eprintln!("[DEBUG] n={n} step {steps} phase-capacity {}",p.retained_capacity_bytes);}
        }
        eprintln!("[DEBUG] n={n} steps {steps} total capacity {total}");
        close(&mut rx);
    }
}

#[test]
fn variant_tags_match_the_shared_neutral_schema(){
 use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::{FemDof,FemElement,FemLoad};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../🧪️tests/🏷️variant-tag/🔣️fixture.json")).unwrap();let cases=fixture["cases"].as_array().unwrap();
 let elements=[FemElement::Bar{id:String::new(),start:String::new(),end:String::new(),material_id:String::new(),section_id:String::new()},FemElement::Frame{id:String::new(),start:String::new(),end:String::new(),material_id:String::new(),section_id:String::new(),roll:0.}];
 for(name,value)in ["bar","frame"].into_iter().zip(elements.iter()){let expected=cases.iter().find(|case|case["dimension"]==3&&case["family"]=="element"&&case["variant"]==name).unwrap()["tag"].as_u64().unwrap();let tag=unsafe{std::ptr::from_ref(value).cast::<u8>().read()};assert_eq!(u64::from(tag),expected);}
 for value in elements{drain(value)}
 let loads=[FemLoad::Nodal{id:String::new(),node_id:String::new(),dof:FemDof::Tx,value:0.},FemLoad::MemberUdl{id:String::new(),element_id:String::new(),wx:0.,wy:0.,wz:0.},FemLoad::Area{id:String::new(),solid_id:String::new(),pressure:0.}];
 for(name,value)in ["nodal","memberUdl","area"].into_iter().zip(loads.iter()){let expected=cases.iter().find(|case|case["dimension"]==3&&case["family"]=="load"&&case["variant"]==name).unwrap()["tag"].as_u64().unwrap();let tag=unsafe{std::ptr::from_ref(value).cast::<u8>().read()};assert_eq!(u64::from(tag),expected);}
 for value in loads{drain(value)}
 let dofs=[FemDof::Tx,FemDof::Ty,FemDof::Tz,FemDof::Rx,FemDof::Ry,FemDof::Rz];for(name,value)in ["Tx","Ty","Tz","Rx","Ry","Rz"].into_iter().zip(dofs.iter()){let expected=cases.iter().find(|case|case["dimension"]==3&&case["family"]=="dof"&&case["variant"]==name).unwrap()["tag"].as_u64().unwrap();let tag=unsafe{std::ptr::from_ref(value).cast::<u8>().read()};assert_eq!(u64::from(tag),expected);}
}
