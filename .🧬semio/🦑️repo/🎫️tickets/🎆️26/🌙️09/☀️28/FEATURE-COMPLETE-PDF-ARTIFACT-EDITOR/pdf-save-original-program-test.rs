//! 🧪 Drives the exact normal Save emitter on an Original caller.

use crate::standards::v1_7::subsets::base::io::{export::encode_pdf, import::decode_pdf};
use crate::{close::ProgramClose, page::admit_pdf_save_page,
    producer::{PdfNativeSaveProgram, PlanDecision}, program::{ClassicPdfSaveProgram, admit_retained_candidate},
    window::write_pdf_original_turn};
use crate::output_stage::{TURN, caller, close_construction, finish, retire};
use semio_plugin::{ArtifactOutputChunks, MediaConstruction};
use semio_plugin::__job::InteractiveJobCloseStep;

#[test]
fn original_program_qualifies_writes_and_closes_normal_retained_pdf() {
    let path = std::env::var_os("SEMIO_PDF_EXACT_GRAPH_FIXTURE").unwrap();
    let input = std::fs::read(path).unwrap();
    let snapshot = decode_pdf(&input).unwrap();
    let expected = encode_pdf(&snapshot).unwrap();
    let revision = [42; 32];
    let mut source = caller();
    let mut decoder = source.admitted_decoder();
    source.retain_dispatch().unwrap();
    let mut candidate = None;
    let mut birth = MediaConstruction::default();
    assert!((0..64).any(|_| source.turn_with_decoder(TURN, &mut decoder, |cx|
        admit_retained_candidate(&snapshot, revision, &mut candidate, &mut birth, cx)).unwrap()));
    let mut program = ClassicPdfSaveProgram::new(candidate.unwrap(), ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES);
    let ready = (0..16384).find_map(|_| {
        match source.turn_with_decoder(TURN, &mut decoder, |cx| program.prepare_original(&snapshot, revision, cx)).unwrap() {
            PlanDecision::Pending => None,
            PlanDecision::Ready => Some(true),
            PlanDecision::ReferenceRequired => Some(false),
        }
    });
    assert_eq!(ready, Some(true));
    let mut actual = Vec::new();
    let mut page = None;
    let mut construction = MediaConstruction::default();
    let mut page_ready = false;
    let mut done = false;
    for _ in 0..16384 {
        if !page_ready {
            page_ready = source.turn_with_decoder(TURN, &mut decoder, |cx|
                admit_pdf_save_page(&mut page, &mut construction, cx)).unwrap();
            if !page_ready { continue; }
        }
        let buffer = page.as_mut().unwrap();
        if let Some(ready) = source.turn_with_decoder(TURN, &mut decoder, |cx| {
            let result = write_pdf_original_turn(buffer, cx, |sink, copy| {
                let turn = program.emit_original(&snapshot, sink, copy);
                let read = turn.read;
                (turn, read)
            })?;
            let Some(result) = result else { return Ok(None); };
            cx.consume_fuel(result.work as u64);
            Ok(Some(result.result?))
        }).unwrap() {
            done = ready;
        }
        if done || buffer.len() == ArtifactOutputChunks::CHUNK_BYTES {
            let original = page.take().unwrap();
            actual.extend_from_slice(&original);
            retire(&mut source, &mut decoder, original);
            page_ready = false;
        }
        if done { break; }
    }
    assert!(done);
    let mut close = ProgramClose::new(program.retire());
    let mut last = None;
    for _ in 0..16384 {
        last = Some(source.turn_with_decoder(TURN, &mut decoder, |cx|
            cx.with_original_retirement(|cx| close.close_step(cx))).unwrap());
        if matches!(last, Some(InteractiveJobCloseStep::Complete { .. })) && close.terminal_is_empty() { break; }
    }
    assert!(close.terminal_is_empty(), "Original graph close did not settle: {:?}", last);
    close_construction(&mut source, &mut decoder, &mut construction);
    close_construction(&mut source, &mut decoder, &mut birth);
    finish(source, &mut decoder);
    if let Some(path) = std::env::var_os("SEMIO_PDF_ORIGINAL_SAVE_OUTPUT") {
        std::fs::write(path, &actual).unwrap();
    }
    let first = actual.iter().zip(&expected).position(|(a, b)| a != b);
    assert!(first.is_none() && actual.len() == expected.len(),
        "Original vs reference: first difference {first:?}, original {}, reference {}", actual.len(), expected.len());
    println!("[DEBUG] exact Original Save program wrote and retired {} bytes", actual.len());
}
