//! 🧪️ `pin-solution` — the commit laws of `s.wfc.bitmap.solve`: a solve's pixels become pins that
//! reproduce it, a second commit of the same solve writes nothing, and a contradiction or a foreign
//! buffer is refused.

use super::*;
use crate::mutations::apply_bitmap_mutation;
use crate::schema::snapshot::encode_base64;

fn commit(snapshot: &BitmapSnapshot, payload: &PinSolution) -> BitmapSnapshot {
    let mut next = snapshot.clone();
    for mutation in solution_operations(snapshot, payload).expect("the solve commits") {
        apply_bitmap_mutation(&mut next, &mutation).expect("every pin applies");
    }
    next
}

#[test]
fn a_committed_solve_pins_every_output_cell_to_its_colour() {
    let mut snapshot = crate::examples::rooms_16::snapshot();
    snapshot.output = crate::schema::snapshot::BitmapOutputSpec { width: 8, height: 6, periodic: snapshot.output.periodic };
    snapshot.pinned = vec![crate::schema::snapshot::BitmapPinnedPixel { x: 1, y: 0, color: 0 }, crate::schema::snapshot::BitmapPinnedPixel { x: 2, y: 0, color: 2 }];
    let palette = snapshot.input.palette.len();
    let indices: Vec<u8> = (0..48).map(|cell| u8::try_from((cell * 7 + 3) % palette).expect("palette index")).collect();
    let payload = PinSolution { pixels: encode_base64(&indices), contradiction: false };
    let operations = solution_operations(&snapshot, &payload).expect("the solve commits");
    assert_eq!(operations.len(), 47, "the one cell already pinned to its solved colour writes no pin");
    let next = commit(&snapshot, &payload);
    assert_eq!(next.pinned.len(), 48, "every one of the 8×6 cells is pinned");
    for pin in &next.pinned {
        assert_eq!(u32::from(indices[(pin.y * 8 + pin.x) as usize]), pin.color, "pin ({}, {}) carries the solved colour", pin.x, pin.y);
    }
    assert!(solution_operations(&next, &payload).expect("idempotent").is_empty(), "committing the same solve twice writes no second edit");
}

#[test]
fn a_contradiction_or_a_foreign_buffer_is_refused() {
    let snapshot = crate::examples::rooms_16::snapshot();
    let cells = (snapshot.output.width * snapshot.output.height) as usize;
    let refused = |payload: PinSolution| format!("{:?}", solution_operations(&snapshot, &payload).expect_err("refused"));
    assert!(refused(PinSolution { pixels: encode_base64(&vec![0; cells]), contradiction: true }).contains("contradiction"));
    assert!(refused(PinSolution { pixels: "not base64!".into(), contradiction: false }).contains("not-base64"));
    assert!(refused(PinSolution { pixels: encode_base64(&vec![0; cells - 1]), contradiction: false }).contains("extent"));
    assert!(refused(PinSolution { pixels: encode_base64(&vec![250; cells]), contradiction: false }).contains("palette"));
}
