use crate::events::KitEvent;

#[test]
fn side_set_design_piece_emits() {
    let (kit, _, dg, pb, _, cg) = super::common::kit_with_connection();
    let mut rx = kit.read().unwrap().subscribe();
    let piece_ref = {
        let kr = kit.read().unwrap();
        let d = kr.design(dg.as_str()).unwrap();
        let dr = d.read().unwrap();
        dr.piece(pb.as_str()).unwrap().clone()
    };
    let weak = std::sync::Arc::downgrade(&piece_ref);
    let connecting = {
        let kr = kit.read().unwrap();
        let d = kr.design(dg.as_str()).unwrap();
        let dr = d.read().unwrap();
        let c = dr.connection(cg.as_str()).unwrap();
        c.read().unwrap().connecting.clone()
    };
    connecting
        .write()
        .unwrap()
        .set_design_piece_weak(Some(weak));
    let evs = super::common::drain(&mut rx);
    assert!(evs
        .iter()
        .any(|e| matches!(e, KitEvent::FieldChanged { field: "designPiece", .. })));
}
