//! 🧪 Exercises the exact staged compressed writer and reader under funded framework effects.

use crate::native::png_source::{PngGraphSource, PngPlane};
use crate::writer::PngGraphWriter;
use crate::{ObjRef, PdfIndirectObject, PdfMutation};
use semio_framework_value::{AuthoringWallet, RetainedCloneGrant, paged::ByteReadSource};

const FIXTURE: &[u8] = include_bytes!("🗑️generated/large-picked/large-picked.png");
const DIGEST: &[u8; 32] = include_bytes!("🗑️generated/raw-png-writer/identity");
const BODY: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 16, maximum_copy_bytes: 4096,
    maximum_capacity_bytes: 64 * 1024, maximum_release_bytes: 64 * 1024, maximum_depth: 64 };
const STOCK: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: 1 << 23,
    maximum_capacity_bytes: 1 << 23, maximum_release_bytes: 1 << 23, ..BODY };

#[test]
fn funded_writer_builds_exact_native_rows_and_refuses_foreign_source_revision() {
    let source = FIXTURE.to_vec();
    let foreign = source.clone();
    let revision = [0x5a; 32];
    let mut wrong = revision;
    wrong[31] ^= 1;
    let mut wallet = AuthoringWallet::new(STOCK, 1 << 24);
    let mut work = PngGraphWriter::begin(&source, revision).unwrap();
    let mut graph = Vec::new();
    let mut row = None;
    let id = ObjRef { num: 1, gen: 0 };
    assert!(wallet.run(BODY, 4096, |effect| work.advance(&foreign, &revision, id, &mut row, effect)).unwrap().unwrap().output.is_err());
    assert!(wallet.run(BODY, 4096, |effect| work.advance(&source, &wrong, id, &mut row, effect)).unwrap().unwrap().output.is_err());
    let no_copy = RetainedCloneGrant { maximum_copy_bytes: 0, ..BODY };
    assert_eq!(wallet.run(no_copy, 4096, |effect| work.advance(&source, &revision, id, &mut row, effect)).unwrap().unwrap().output.unwrap(), Default::default());
    for _ in 0..1024 {
        let id = ObjRef { num: graph.len() as u32 + 1, gen: 0 };
        wallet.run(BODY, 4096, |effect| work.advance(&source, &revision, id, &mut row, effect)).unwrap().unwrap().output.unwrap();
        if let Some(PdfMutation::InsertObject(row)) = row.take() { graph.push(PdfIndirectObject { id: row.id, value: row.value }); }
        if work.rows_done() { break; }
    }
    assert!(work.rows_done());
    assert_eq!(graph.len(), 15);
    assert_eq!(work.resource().unwrap().digest, *DIGEST);
    let reference = PngPlane::Rgb.reference(*DIGEST);
    let (_, reopened) = PngGraphSource::resolve(&graph, &reference).unwrap();
    let mut reconstructed = vec![0; source.len()];
    assert_eq!(reopened.read_bytes(0, &mut reconstructed), source.len());
    assert_eq!(reconstructed, source);

    let mut reuse = PngGraphWriter::begin(&source, revision).unwrap();
    while reuse.resource().is_none() {
        wallet.run(BODY, 4096, |effect| reuse.advance(&source, &revision, id, &mut row, effect)).unwrap().unwrap().output.unwrap();
        assert!(row.is_none());
    }
    for object in graph.iter().rev() {
        assert!(reuse.matches_existing(&object.value).unwrap());
        assert!((0..128).any(|_| wallet.run(BODY, 4096, |effect| {
            reuse.compare_existing(&source, &revision, &object.value, effect)
        }).unwrap().unwrap().output.unwrap().0));
    }
    assert!(wallet.run(BODY, 4096, |effect| {
        reuse.compare_existing(&source, &revision, &graph[0].value, effect)
    }).unwrap().unwrap().output.is_err());
    reuse.finish_existing().unwrap();
    assert!(reuse.rows_done());
    assert_eq!(reuse.resource().unwrap().digest, *DIGEST);
    assert_eq!(wallet.run(BODY, 4096, |effect| {
        reuse.advance(&source, &revision, id, &mut row, effect)
    }).unwrap().unwrap().output.unwrap(), Default::default());
    assert!(row.is_none());
    for _ in 0..128 {
        if reuse.terminal_is_empty() { break; }
        let (turn, _) = wallet.run_settled(BODY, 8192, |effect| reuse.close(4096, effect)).unwrap().unwrap();
        turn.result.unwrap();
    }
    assert!(reuse.terminal_is_empty());

    for _ in 0..128 {
        if work.terminal_is_empty() { break; }
        let (turn, _) = wallet.run_settled(BODY, 8192, |effect| work.close(4096, effect)).unwrap().unwrap();
        turn.result.unwrap();
    }
    assert!(work.terminal_is_empty());
}
