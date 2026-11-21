#[path = "pdf-segmented-save-page.rs"]
pub(crate) mod page;

use std::{fs::File, io::{BufReader, Read}};
use semio_framework_job::{Generation, OperationId, OriginalJobDecodeOwner, OriginalJobFixtureCaller, close_original_effect_step};
use semio_framework_value::{retained_clone::{RetainedCloneClose, RetainedCloneGrant}, retirement::RetireOwned};
use semio_plugin_decl_app::media_contract::MediaConstruction;
use semio_plugin_decl_media::ArtifactOutputChunks;

pub(crate) const TURN: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 64, maximum_copy_bytes: 32768, maximum_capacity_bytes: 1 << 20,
    maximum_release_bytes: 1 << 20, maximum_depth: 4096,
};

pub(crate) fn caller() -> OriginalJobFixtureCaller {
    OriginalJobFixtureCaller::admit(OperationId(884), Generation(7), RetainedCloneGrant {
        maximum_items: 1 << 22, maximum_copy_bytes: 1 << 28,
        maximum_capacity_bytes: 1 << 27, maximum_release_bytes: 1 << 27, maximum_depth: 4096,
    }, semio_framework_job::default_now_us, u64::MAX, 1 << 25)
}

pub(crate) fn retire<T: RetireOwned>(caller: &mut OriginalJobFixtureCaller, decoder: &mut OriginalJobDecodeOwner, value: T) {
    let (mut value, mut close) = (Some(value), RetainedCloneClose::default());
    while value.is_some() || !close.is_empty() {
        caller.turn_with_decoder(TURN, decoder, |cx| {
            let demand = close.next_owner_demands::<T>(value.is_some(), &None, TURN.maximum_copy_bytes)?;
            close_original_effect_step(cx, demand, |effect| semio_plugin_job_receiving::original_steps::begin(&mut close, &mut value, effect))?;
            Ok(())
        }).unwrap();
    }
}

pub(crate) fn close_construction(caller: &mut OriginalJobFixtureCaller, decoder: &mut OriginalJobDecodeOwner, construction: &mut MediaConstruction) {
    for _ in 0..64 {
        if construction.terminal_is_empty() { return; }
        caller.turn_with_decoder(TURN, decoder, |cx| cx.with_original_retirement(|cx| {
            construction.close_step(cx)?;
            Ok(())
        })).unwrap();
    }
    panic!("PDF output construction did not reach terminal");
}

pub(crate) fn finish(mut caller: OriginalJobFixtureCaller, decoder: &mut OriginalJobDecodeOwner) {
    for _ in 0..semio_framework_job::ORIGINAL_DECODER_TEARDOWN_STEPS {
        let (quote, _) = semio_framework_job::original_decoder_teardown_quote(decoder).unwrap();
        caller.turn(quote, |cx| { decoder.close_on_source(cx)?; Ok(()) }).unwrap();
    }
    assert!(decoder.terminal_is_empty());
    caller.retire(TURN);
}

#[test]
fn saves_a_genuine_pdf_through_paid_output_pages_without_flattening() {
    let fixture = std::env::var_os("SEMIO_PDF_SAVE_PAGE_FIXTURE").unwrap();
    let file = File::open(&fixture).unwrap();
    let total = file.metadata().unwrap().len() as usize;
    assert!(total > 2 << 20);
    let mut input = BufReader::new(file);
    let mut caller = caller();
    let mut decoder = caller.admitted_decoder();
    caller.retain_dispatch().unwrap();
    let mut output = None;
    while !caller.turn_with_decoder(TURN, &mut decoder, |cx| ArtifactOutputChunks::original_birth_admit(total, &mut output, cx)).unwrap() {}
    let output = output.unwrap();
    let mut construction = MediaConstruction::default();
    let mut page = None;
    let mut finished = 0;
    let mut first_pointer = None;
    let mut chunk = [0; ArtifactOutputChunks::CHUNK_BYTES];
    while finished < total {
        let amount = chunk.len().min(total - finished);
        input.read_exact(&mut chunk[..amount]).unwrap();
        while !caller.turn_with_decoder(TURN, &mut decoder, |cx| page::admit_pdf_save_page(&mut page, &mut construction, cx)).unwrap() {}
        let buffer = page.as_mut().unwrap();
        first_pointer.get_or_insert(buffer.as_ptr());
        if finished == 0 {
            let denied = RetainedCloneGrant { maximum_copy_bytes: 0, ..TURN };
            caller.turn_with_decoder(denied, &mut decoder, |cx| {
                assert!(page::write_pdf_save_turn(buffer, cx, |_, _| panic!("unfunded PDF writer ran"))?.is_none());
                Ok(())
            }).unwrap();
        }
        let mut used = 0;
        while used < amount {
            let grant = RetainedCloneGrant { maximum_copy_bytes: 1501, ..TURN };
            used += caller.turn_with_decoder(grant, &mut decoder, |cx| {
                let before = cx.retained_progress().copied_bytes;
                let n = page::write_pdf_save_turn(buffer, cx, |sink, _| sink.write(&chunk[used..amount]))?.unwrap().unwrap();
                assert_eq!(cx.retained_progress().copied_bytes - before, n);
                Ok(n)
            }).unwrap();
        }
        assert!(output.take_chunk().is_err());
        finished += amount;
        if finished == amount {
            let denied = RetainedCloneGrant { maximum_copy_bytes: 1, ..TURN };
            caller.turn_with_decoder(denied, &mut decoder, |cx| {
                assert!(!page::donate_pdf_save_page(&mut page, false, &output, None, cx)?);
                Ok(())
            }).unwrap();
            assert_eq!(first_pointer, page.as_ref().map(Vec::as_ptr));
        }
        while !caller.turn_with_decoder(TURN, &mut decoder, |cx| page::donate_pdf_save_page(&mut page, finished == total, &output, None, cx)).unwrap() {}
        assert!(page.is_none());
    }
    assert_eq!(output.bytes(), total);
    while !caller.turn_with_decoder(TURN, &mut decoder, |cx| output.seal_original(cx)).unwrap() {}
    let mut expected = BufReader::new(File::open(&fixture).unwrap());
    let mut terminal = false;
    let mut actual = 0;
    while !terminal {
        let mut received = None;
        while !caller.turn_with_decoder(TURN, &mut decoder, |cx| output.take_chunk_original(&mut received, &mut terminal, cx)).unwrap() {}
        if let Some(bytes) = received {
            expected.read_exact(&mut chunk[..bytes.len()]).unwrap();
            assert_eq!(bytes, &chunk[..bytes.len()]);
            if actual == 0 { assert_eq!(first_pointer, Some(bytes.as_ptr())); }
            actual += bytes.len();
            retire(&mut caller, &mut decoder, bytes);
        }
    }
    assert_eq!(actual, total);
    assert_eq!(output.bytes_remaining(), 0);
    retire(&mut caller, &mut decoder, output);
    close_construction(&mut caller, &mut decoder, &mut construction);
    finish(caller, &mut decoder);
    println!("[DEBUG] Donated, sealed, drained and retired {actual} output bytes");
}

#[test]
fn discards_a_partial_unpublished_pdf_page() {
    let mut caller = caller();
    let mut decoder = caller.admitted_decoder();
    caller.retain_dispatch().unwrap();
    let (mut output, mut page) = (None, None);
    let mut construction = MediaConstruction::default();
    assert!(caller.turn_with_decoder(TURN, &mut decoder, |cx| ArtifactOutputChunks::original_birth_admit(4097, &mut output, cx)).unwrap());
    let output = output.unwrap();
    while !caller.turn_with_decoder(TURN, &mut decoder, |cx| page::admit_pdf_save_page(&mut page, &mut construction, cx)).unwrap() {}
    caller.turn_with_decoder(TURN, &mut decoder, |cx| {
        assert_eq!(page::write_pdf_save_turn(page.as_mut().unwrap(), cx, |sink, _| sink.write(b"%PDF-1.7"))?.unwrap().unwrap(), 8);
        assert!(!page::donate_pdf_save_page(&mut page, false, &output, None, cx)?);
        Ok(())
    }).unwrap();
    assert!(output.take_chunk().is_err() && output.bytes() == 0);
    retire(&mut caller, &mut decoder, page);
    retire(&mut caller, &mut decoder, output);
    close_construction(&mut caller, &mut decoder, &mut construction);
    finish(caller, &mut decoder);
}
