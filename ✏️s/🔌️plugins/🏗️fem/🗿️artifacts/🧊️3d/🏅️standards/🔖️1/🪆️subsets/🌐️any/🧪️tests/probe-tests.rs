use super::*;
fn p<T: RetireOwned>(name: &str, v: T) {
    eprintln!("[DEBUG] {name}: supported={} birth={:?}", T::controlled_retirement_supported(), v.retirement_birth_bytes());
    std::mem::forget(v);
}
#[test]
fn probe_birth_authority() {
    p("Vec<(u8,usize,usize)>", Vec::<(u8, usize, usize)>::new());
    p("Vec<String>", Vec::<String>::new());
    p("String", String::new());
    p("Fem3dSnapshotValue", Fem3dSnapshotValue::default());
    p("Fem3dSnapshot", Fem3dSnapshot::default());
    p("FemIdentityIndex", FemIdentityIndex::default());
    p("Vec<[f64;2]>", Vec::<[f64; 2]>::new());
    p("Vec<FemDof>", Vec::<FemDof>::new());
    p("Option<String>", Option::<String>::None);
    p("Vec<FieldValue>", Vec::<FieldValue>::new());
    p("RecordValue", RecordValue::default());
    p("Vec<FemNodeValue>", Vec::<FemNodeValue>::new());
    p("Vec<FemElement>", Vec::<FemElement>::new());
    p("Vec<FemLoadCase>", Vec::<FemLoadCase>::new());
    p("BTreeMap<String,f64>", std::collections::BTreeMap::<String, f64>::new());
    p("Vec<FemSolid>", Vec::<FemSolid>::new());
    p("FemSolid", FemSolid { id: String::new(), name: String::new(), outline: vec![], holes: vec![], base_z: 0., height: 0., layers: 1, mesh_size: 1., material_id: String::new(), axis: FemAxis::Z });
}
struct Recipient;
impl NativeDecodeConstructionRecipient for Recipient {
    fn grant(&mut self, d: RetirementDemand) -> Result<RetainedCloneGrant, ValueError> {
        Ok(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 16384, maximum_capacity_bytes: 65536, maximum_release_bytes: 65536, maximum_depth: 64 })
    }
    fn accept(&mut self, g: RetainedCloneGrant, p: RetainedCloneProgress) -> Result<(), ValueError> {
        eprintln!("[DEBUG] recipient accept: {p:?}");
        Ok(())
    }
}
#[test]
fn probe_accounting() {
    let mut s = Fem3dSnapshotValue::default();
    s.nodes = vec![FemNodeValue { id: "a".into(), x: 1., y: 2., z: 3. }, FemNodeValue { id: "b".into(), x: 4., y: 5., z: 6. }];
    let g = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 16384, maximum_capacity_bytes: 65536, maximum_release_bytes: 65536, maximum_depth: 64 };
    let (mut rx, p) = Fem3dReceiving::capture(s, g).map_err(|(e, _)| e).unwrap();
    eprintln!("[DEBUG] capture progress {p:?}");
    let mut callback = |_| true;
    let mut control = NativeDecodeControl::new(1_000_000, &mut callback);
    let mut born = 0usize;
    control.with_construction_recipient(&mut Recipient, |control| -> Result<(), ValueError> {
        for step in 0..400 {
            if rx.ready() { break; }
            let d = rx.demands(g.maximum_copy_bytes)?;
            let p = rx.advance(control, g.maximum_copy_bytes, g)?.progress();
            born += p.retained_capacity_bytes;
            if born != control.owned_bytes() { eprintln!("[DEBUG] DRIFT step {step} demand={d:?} progress={p:?} born={born} owned={}", control.owned_bytes()); }
        }
        Ok(())
    }).unwrap();
    eprintln!("[DEBUG] ready={}", rx.ready());
    let (value, _) = rx.take(g).unwrap().unwrap();
    let mut rx = rx;
    rx.begin_close();
    for step in 0..2000 {
        if rx.terminal_is_empty() { break; }
        let d = rx.close_demands(g.maximum_copy_bytes);
        let r = rx.close_step(g);
        if r.is_err() || step < 3 { eprintln!("[DEBUG] rx close {step} demand={d:?} step={:?}", r.as_ref().map(|s| s.progress()).map_err(|e| e.to_string())); }
        if r.is_err() { break; }
    }
    eprintln!("[DEBUG] rx terminal={}", rx.terminal_is_empty());
    let mut owner = semio_framework_value::retirement::controlled::ControlledRetirement::new(value).ok().unwrap();
    for step in 0..5000 {
        if owner.terminal_is_empty() { break; }
        let d = owner.next_demand(g.maximum_copy_bytes);
        let r = owner.step(g);
        if r.is_err() || step < 3 { eprintln!("[DEBUG] value retire {step} demand={d:?} step={r:?}"); }
        if r.is_err() { break; }
    }
    eprintln!("[DEBUG] value terminal={}", owner.terminal_is_empty());
    std::mem::forget(owner);
    std::mem::forget(rx);
}

#[test]
fn probe_from_reference() {
    let mut s = Fem3dSnapshotValue::default();
    s.nodes = vec![FemNodeValue { id: "a".into(), x: 1., y: 2., z: 3. }];
    let r = Fem3dSnapshot::from_reference(s);
    eprintln!("[DEBUG] from_reference small: {:?}", r.as_ref().map(|v| v.nodes.len()));
    if let Err(e) = &r { eprintln!("[DEBUG] from_reference error {e:?}"); }
    std::mem::forget(r);
}
