//! 🧪️ Audits exact plane and inflater capacity through the repository’s paid value owners.

use super::*;
use crate::paid::PngPaidPlanes;
use png::PngSampleCursor;
use semio_framework_value::{AuthoringWallet, RetainedCloneProgress, retirement::controlled::ControlledRetirement};

const TURN: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 64, maximum_copy_bytes: 32768, maximum_capacity_bytes: 16777216,
    maximum_release_bytes: 1048576, maximum_depth: 4096,
};
const STOCK: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 1 << 22, maximum_copy_bytes: 1 << 27,
    maximum_capacity_bytes: 1 << 24, maximum_release_bytes: 1 << 24, ..TURN
};

pub fn decode_paid(samples: PngSampleCursor, source: &MeasuredInput, destination: PathBuf, cancel: Option<usize>) {
    let mut app = PngPaidPlanes::<OUTPUT_CAP>::new(samples).unwrap();
    let mut wallet = AuthoringWallet::new(STOCK, 1 << 26);
    let mut retained = 0;
    let mut held = 0;
    let mut max_turn = 0;
    let mut turns = 0;
    let mut rendered = false;
    while turns < 2_000_000 && cancel != Some(turns) {
        if let Some((color, alpha)) = app.rendered() {
            write_plane(destination.with_extension("color"), color);
            write_plane(destination.with_extension("alpha"), alpha);
            rendered = true;
            break;
        }
        let grant = TURN.restricted_by(wallet.remaining());
        let turn = wallet.run(grant, 16, |effect| app.advance(source, effect)).unwrap().unwrap();
        let receipt = turn.output.unwrap();
        assert_eq!(turn.actual.unwrap_or(0), receipt.retained_capacity_bytes);
        assert!(turn.performed > 0);
        wallet.post(receipt).unwrap();
        retained += receipt.retained_capacity_bytes;
        max_turn = max_turn.max(receipt.retained_capacity_bytes);
        turns += 1;
    }
    assert_eq!(rendered, cancel.is_none());
    app.stop();
    assert!(app.rendered().is_none());
    let (mut samples, planes) = app.into_closing();
    let mut released = 0;
    for step in 0..64 {
        let allowed = TURN.restricted_by(wallet.remaining());
        let turn = wallet.run(allowed, 4096, |effect| {
            let before = samples.history_capacity();
            assert!(effect.performed(1).unwrap());
            let done = samples.close_history_step(1, allowed.maximum_release_bytes);
            let report = RetainedCloneProgress { released_bytes: before - samples.history_capacity(), copied_items: 1, ..Default::default() };
            (done, report)
        }).unwrap().unwrap();
        let (done, report) = turn.output;
        released += report.released_bytes;
        wallet.post(report).unwrap();
        if done { break; }
        assert!(step < 63);
    }
    let mut planes = ControlledRetirement::new(planes).map_err(|(error, _)| error).unwrap();
    while !planes.terminal_is_empty() {
        let grant = TURN.restricted_by(wallet.remaining());
        let (turn, _) = wallet.run_settled(grant, 65536, |effect| planes.step(grant.maximum_copy_bytes, effect)).unwrap().unwrap();
        let receipt = turn.result.unwrap().progress();
        wallet.post(receipt).unwrap();
        held += receipt.retained_capacity_bytes;
        released += receipt.released_bytes;
    }
    assert_eq!(retained + held, released);
    assert!(max_turn <= 32768);
    println!("[DEBUG] Paid PNG rendered {rendered} after {turns} steps, peak {max_turn}, equal retained and released {released} bytes");
}
