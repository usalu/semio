#[path = "pdf-segmented-save-page-test.rs"]
mod output_stage;
#[path = "pdf-segmented-save-page.rs"]
mod page;
#[path = "pdf-registered-save-window.rs"]
mod window;

use output_stage::{TURN, caller, close_construction, finish, retire};
use semio_plugin_decl_app::media_contract::MediaConstruction;
use semio_plugin_decl_media::ArtifactOutputChunks;
use std::io::ErrorKind;

#[test]
fn pays_the_source_and_sink_even_when_the_sink_yields_after_a_partial_write() {
    let mut caller = caller();
    let mut decoder = caller.admitted_decoder();
    caller.retain_dispatch().unwrap();
    let mut construction = MediaConstruction::default();
    let mut page = None;
    while !caller.turn_with_decoder(TURN, &mut decoder, |cx|
        page::admit_pdf_save_page(&mut page, &mut construction, cx)
    ).unwrap() {}
    let page = page.as_mut().unwrap();
    caller.turn_with_decoder(TURN, &mut decoder, |cx| {
        let bytes = [42; ArtifactOutputChunks::CHUNK_BYTES - 1];
        let n = window::write_pdf_original_turn(page, cx, |sink, _| (sink.write(&bytes), 0))?
            .unwrap().unwrap();
        assert_eq!(n, bytes.len());
        Ok(())
    }).unwrap();
    let narrow = semio_framework_value::RetainedCloneGrant { maximum_copy_bytes: 97, ..TURN };
    caller.turn_with_decoder(narrow, &mut decoder, |cx| {
        let before = cx.retained_progress();
        let result = window::write_pdf_original_turn(page, cx, |sink, combined| {
            assert_eq!(combined, 97);
            let original = [137; 48];
            let mut frontier = [0; 48];
            frontier.copy_from_slice(&original);
            assert_eq!(sink.write(&frontier).unwrap(), 1);
            (sink.write(&frontier[1..]), frontier.len())
        })?.unwrap();
        assert_eq!(result.unwrap_err().kind(), ErrorKind::WouldBlock);
        assert_eq!(cx.retained_progress().copied_bytes - before.copied_bytes, 49);
        assert_eq!(cx.retained_progress().copied_items - before.copied_items, 1);
        assert_eq!(page.len(), ArtifactOutputChunks::CHUNK_BYTES);
        assert_eq!(page.last(), Some(&137));
        Ok(())
    }).unwrap();
    retire(&mut caller, &mut decoder, std::mem::take(page));
    close_construction(&mut caller, &mut decoder, &mut construction);
    finish(caller, &mut decoder);
}
