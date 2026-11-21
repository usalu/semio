use super::*;
use semio_framework_value::{AuthoringWallet, RetainedCloneGrant};
use owner::RetainedJpgPreview;

const TURN: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 64, maximum_copy_bytes: 32768, maximum_capacity_bytes: 16777216,
    maximum_release_bytes: 1048576, maximum_depth: 4096,
};
const STOCK: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 1 << 22, maximum_copy_bytes: 1 << 27, maximum_capacity_bytes: 1 << 24,
    maximum_release_bytes: 1 << 24, ..TURN
};

fn fixture() -> Source { Source::read(Path::new(&std::env::var_os("SEMIO_PDF_JPEG_PREVIEW_FIXTURE").unwrap())).unwrap() }

fn size() -> preview::JpgPreviewSize {
    preview::JpgPreviewSize { source: [1024, 1024], display: [256, 256], channels: 3 }
}

fn advance(owner: &mut RetainedJpgPreview, source: &Source, wallet: &mut AuthoringWallet, steps: usize) -> usize {
    let mut capacity = 0;
    for _ in 0..steps {
        if owner.rendered().is_some() { break; }
        let turn = wallet.run(TURN.restricted_by(wallet.remaining()), 32768, |effect| owner.step(source, effect)).unwrap().unwrap();
        let receipt = turn.output.unwrap();
        assert_eq!(turn.actual.unwrap_or(0), receipt.retained_capacity_bytes);
        assert!(turn.performed > 0);
        assert!(receipt.copied_bytes <= 4096);
        wallet.post(receipt).unwrap();
        capacity += receipt.retained_capacity_bytes;
    }
    capacity
}

fn close(owner: &mut RetainedJpgPreview, wallet: &mut AuthoringWallet, retained: usize) {
    let (mut held, mut released) = (retained, 0);
    while !owner.closed() {
        let (turn, _) = wallet.run_settled(TURN.restricted_by(wallet.remaining()), 65536, |effect| owner.close(TURN.maximum_copy_bytes, effect).unwrap()).unwrap().unwrap();
        let receipt = turn.result.unwrap().progress();
        wallet.post(receipt).unwrap();
        held += receipt.retained_capacity_bytes;
        released += receipt.released_bytes;
        assert!(owner.rendered().is_none());
    }
    assert_eq!(held, released);
}

#[test]
fn paid_preview_keeps_all_pixels_private_until_eoi_and_retires() {
    let source = fixture();
    let mut wallet = AuthoringWallet::new(STOCK, 1 << 27);
    let mut owner = RetainedJpgPreview::new(source.len(), size()).unwrap();
    let denied = RetainedCloneGrant { maximum_capacity_bytes: 0, ..TURN };
    let turn = wallet.run(denied, 32768, |effect| owner.step(&source, effect)).unwrap().unwrap();
    assert_eq!(turn.performed, 0);
    let capacity = advance(&mut owner, &source, &mut wallet, 100_000);
    let (metadata, output) = owner.rendered().unwrap();
    assert_eq!(metadata.color_transform, Some(1));
    assert_eq!(output.len(), 256 * 256 * 3);
    let mut direct = preview::BaselineJpgPreview::new(source.len(), size()).unwrap();
    let mut logical = vec![0; output.len()];
    loop {
        let next = direct.step(&source, &mut |x, y, channel, value| logical[(y * 256 + x) * 3 + channel] = value).unwrap();
        if next.is_some() { break; }
    }
    assert!(output.iter().eq(logical));
    assert!(capacity > output.len());
    close(&mut owner, &mut wallet, capacity);
    println!("[DEBUG] Finite native work held and released {capacity} bytes of sample backing");
}

#[test]
fn cancellation_retires_a_partially_rendered_preview() {
    let source = fixture();
    let mut wallet = AuthoringWallet::new(STOCK, 1 << 27);
    let mut owner = RetainedJpgPreview::new(source.len(), size()).unwrap();
    let capacity = advance(&mut owner, &source, &mut wallet, 1000);
    assert!(capacity > 0);
    assert!(owner.rendered().is_none());
    close(&mut owner, &mut wallet, capacity);
    let turn = wallet.run(TURN.restricted_by(wallet.remaining()), 32768, |effect| owner.step(&source, effect)).unwrap().unwrap();
    assert_eq!(turn.performed, 0);
    assert_eq!(turn.output.unwrap().copied_bytes, 0);
}
