#[path = "placement-wire.rs"]
mod placement_wire;
#[path = "operation-hook.rs"]
mod operation_hook;

use operation_hook::PdfDctPlacementOperation;
use placement_wire::{DCT_INSTALL_PLACEMENT_TAG, PdfDctPlacementRows, reopen_dct_placement, seal_dct_placement};
use semio_framework_dsl_record::native_encoding::FieldProjectionView;
use semio_os_spr::ColdMutation;
use semio_os_store_one_item_publication::lifted_prepared_operation_wire::ArtifactPreparedOperationSource;
use semio_os_vcs::apply_mutation;
use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::base::schema::{
    mutations::{AppendPageContent, PdfMutation, ReplaceContent, SetImage},
    snapshot::{PdfColorSpace, PdfImage, PdfImageBody, PdfOp, PdfPage, PdfSnapshot},
};

const REVISION: [u8; 32] = [4; 32];
const MATRIX: [f64; 6] = [1024.0, 0.0, 0.0, 1024.0, 0.0, 0.0];

fn image_mutation(name: &str, reference: &ArtifactRef) -> PdfMutation {
    let mut image = PdfImage::samples(name, 1024, 1024, PdfColorSpace::DeviceRgb, 8, Vec::new());
    image.body = PdfImageBody::Artifact { reference: reference.clone() };
    PdfMutation::SetImage(SetImage { image, index: None })
}

fn undo_and_redo(
    before: &PdfSnapshot,
    forward: &[PdfMutation; 2],
    check: impl Fn(&PdfSnapshot),
) -> PdfSnapshot {
    let mut current = before.clone();
    let mut inverse = Vec::new();
    for mutation in forward {
        inverse.push(ColdMutation::inverse(mutation, &current).unwrap());
        current = apply_mutation(&current, mutation).unwrap().0;
    }
    check(&current);
    let applied = current.clone();
    for row in inverse.iter().rev().flatten() {
        current = apply_mutation(&current, row).unwrap().0;
    }
    assert_eq!(&current, before);
    for mutation in forward {
        current = apply_mutation(&current, mutation).unwrap().0;
    }
    assert_eq!(current, applied);
    current
}

async fn spr_roundtrip(before: &PdfSnapshot, forward: &[PdfMutation; 2]) {
    use semio_os_spr::{OpBinary, history::{DecodeOptions, EncodeOptions, HistoryEdit, HistoryEdits, HistoryLog, OpPayload, decode_history, encode_history}};
    let mut applied = before.clone();
    let mut backward = Vec::new();
    for op in forward {
        backward.push(ColdMutation::inverse(op, &applied).unwrap());
        applied = apply_mutation(&applied, op).unwrap().0;
    }
    let payload = |op: &PdfMutation| OpPayload { text: None, binary: Some(op.encode_op().unwrap()) };
    let edit = HistoryEdit {
        id: "sealed-dct-placement".into(), actor: None, started_at: "2026-11-21T00:00:00Z".into(),
        finished_at: None, verb: None, line: None, ops: forward.iter().map(payload).collect(),
        inverse: backward.iter().rev().flatten().map(payload).collect(), meta: None, lane: None,
    };
    let log = HistoryLog {
        doc_id: "pdf-dct-placement-fixture".into(), schema: before.schema.clone(),
        edits: HistoryEdits::try_from_iter([edit]).ok().unwrap(), ..Default::default()
    };
    let options = EncodeOptions { write_backwards_section: true, ..Default::default() };
    let persisted = encode_history(&log, &options).await.unwrap();
    assert!(persisted.len() < 8192);
    let reopened = decode_history(&persisted, &DecodeOptions::default()).await.unwrap();
    assert_eq!(reopened, log);
    let edit = reopened.edits.iter().next().unwrap();
    let replay = |mut snapshot: PdfSnapshot, rows: &[OpPayload]| {
        for row in rows {
            let op = PdfMutation::decode_op(row.binary.as_deref().unwrap()).unwrap();
            snapshot = apply_mutation(&snapshot, &op).unwrap().0;
        }
        snapshot
    };
    assert_eq!(replay(before.clone(), &edit.ops), applied);
    assert_eq!(replay(applied.clone(), &edit.inverse), *before);
    assert_eq!(replay(replay(applied.clone(), &edit.inverse), &edit.ops), applied);
}

async fn run(dir: &Path) -> Result<(), TestError> {
    let source = read_paged_file(File::open(dir.join("large.jpg"))?)?;
    let mut hash = PdfDctResourceIdentity::new(Some(1)).unwrap();
    let mut window = [0; 4096];
    while !hash.complete(source.byte_len()) {
        let used = source.read_bytes(hash.position(), &mut window);
        assert!(used != 0);
        hash.append(&window[..used]);
    }
    let view = PdfDctImageView::from_admitted(
        &source, (1024, 1024), PdfDctImageColor::Rgb { transform: Some(true) }, hash.digest(),
    ).unwrap();
    let reference = view.reference();
    let insert = [
        image_mutation("SemioImage1", &reference),
        PdfMutation::AppendPageContent(AppendPageContent { index: 0, content: vec![
            PdfOp::Save, PdfOp::Transform { matrix: MATRIX }, PdfOp::PaintXObject { name: "SemioImage1".into() }, PdfOp::Restore,
        ]}),
    ];
    let prepared = PdfDctPlacementRows::prepare(REVISION, insert, view)?;
    assert!(prepared.mutations([5; 32]).is_err());

    let path = dir.join("large.dct-placement.pack");
    let sink = seal_dct_placement(FileSink::create(&path)?, &prepared, view, async |count| {
        assert!(count <= 4096);
        Ok(())
    }).await?;
    assert_eq!(sink.peak, 4096);
    drop(sink);
    drop(prepared);
    drop(source);

    let sealed = read_paged_file(File::open(&path)?)?;
    let projection = PdfDctPlacementOperation::from_sealed(&sealed);
    let ArtifactPreparedOperationSource::Pack { tag, body, spec } = projection.wire() else { panic!("native Pack projection required"); };
    assert_eq!(tag, DCT_INSTALL_PLACEMENT_TAG);
    assert_eq!(spec.fields[0].id, 1);
    let FieldProjectionView::BytesSource(original) = body.projection_view(&[0]).unwrap() else { panic!("paged sealed original loan required"); };
    assert!(std::ptr::addr_eq(original, &sealed as &dyn ByteReadSource));

    let (archive, sealed_rows) = reopen_dct_placement(PagedDctInstallPayload(original), &reference).await?;
    let mut genesis = PdfSnapshot::default();
    genesis.pages.push(PdfPage::new(1024.0, 1024.0));
    spr_roundtrip(&genesis, sealed_rows.mutations(REVISION)?).await;
    let after_insert = undo_and_redo(&genesis, sealed_rows.mutations(REVISION)?, |snapshot| {
        assert_eq!(snapshot.pages[0].content.len(), 4);
        assert!(matches!(&snapshot.image("SemioImage1").unwrap().body, PdfImageBody::Artifact { reference: active } if active == &reference));
        assert!(snapshot.objects.is_empty());
    });
    let replacement = [
        image_mutation("SemioImage2", &reference),
        PdfMutation::ReplaceContent(ReplaceContent { index: 0, at: 2, op: PdfOp::PaintXObject { name: "SemioImage2".into() } }),
    ];
    let after_replace = undo_and_redo(&after_insert, &replacement, |snapshot| {
        assert_eq!(snapshot.images.len(), 2);
        assert!(matches!(&snapshot.pages[0].content[2], PdfOp::PaintXObject { name } if name == "SemioImage2"));
        assert!(matches!(&snapshot.image("SemioImage2").unwrap().body, PdfImageBody::Artifact { reference: active } if active == &reference));
        assert!(snapshot.objects.is_empty());
    });

    assert_eq!(after_replace.pages[0].content.len(), 4);
    assert_eq!(archive.facts().reference(), reference);
    let mut reader = archive.verify();
    let mut jpeg = File::create(dir.join("large.dct-placement.jpg"))?;
    let mut count = 0;
    while !reader.complete() {
        let copied = reader.step(&mut window).await?;
        jpeg.write_all(&window[..copied])?;
        count += copied;
    }
    assert_eq!(count, archive.facts().length);
    jpeg.flush()?;
    fs::write(dir.join("large.dct-placement.ref"), &reference.artifact_id)?;
    println!("[DEBUG] sealed original + registered insert rows reopened; native SPR + real PDF insert/replace forwards and inverses passed with one original lease");
    Ok(())
}

fn main() -> Result<(), TestError> {
    let dir = std::env::args_os().nth(1).unwrap();
    block(run(Path::new(&dir)))
}
