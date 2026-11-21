#![allow(dead_code)]

#[path = "paged-png-placement-identity.rs"]
mod plane_identity;
#[path = "paged-png-placement-pack.rs"]
mod plane_pack;
#[path = "paged-png-placement-rows.rs"]
mod plane_rows;

use pack::PackRefusal;
use plane_identity::{PlaneIdentity, PlaneRole, PlaneShape, PngPlanes};
use plane_pack::PagedPngPlacementPayload;
use plane_rows::{PngPlacement, PngPlacementRows, seal_png_placement};
use semio_framework_pack_source::PackSink;
use semio_framework_value::{RetainedCloneGrant, paged::PagedBytes};
use semio_os_spr::ColdMutation;
use semio_os_vcs::apply_mutation;
use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::base::schema::{
    mutations::{AppendPageContent, PdfMutation},
    snapshot::{PdfOp, PdfPage, PdfSnapshot},
};
use std::{fs::{self, File}, future::Future, io::{Read, Write}, path::Path, task::{Context, Poll, Waker}};

const REVISION: [u8; 32] = [4; 32];
const GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4096,
    maximum_capacity_bytes: 8192, maximum_release_bytes: 0, maximum_depth: 64 };

#[derive(Debug)]
enum TestError { Pack(PackRefusal), Io(std::io::Error), Canceled }
impl From<PackRefusal> for TestError { fn from(reason: PackRefusal) -> Self { Self::Pack(reason) } }
impl From<std::io::Error> for TestError { fn from(reason: std::io::Error) -> Self { Self::Io(reason) } }

struct PrivateSink { file: File, extent: u64 }
impl PrivateSink {
    fn open(path: &Path) -> Result<Self, TestError> {
        Ok(Self { file: File::create(path)?, extent: 0 })
    }
}

impl PackSink for PrivateSink {
    type Error = TestError;
    async fn write_all(&mut self, input: &[u8]) -> Result<(), Self::Error> {
        for part in input.chunks(4096) { self.file.write_all(part)?; }
        self.extent += input.len() as u64;
        Ok(())
    }
    async fn position(&self) -> u64 { self.extent }
    async fn flush(&mut self) -> Result<(), Self::Error> { Ok(self.file.flush()?) }
}

fn paged(path: &Path) -> Result<PagedBytes<{isize::MAX as usize}>, TestError> {
    let mut source = File::open(path)?;
    let mut page = [0; 4096];
    let mut output = PagedBytes::default();
    loop {
        let count = source.read(&mut page)?;
        if count == 0 { return Ok(output); }
        for byte in &page[..count] {
            while !output.retained_bytes().has_reserved_slot() { output.reserve_byte_backing(GRANT).unwrap(); }
            output.push_reserved_byte(*byte).unwrap();
        }
    }
}

async fn law(input: &Path, output: &Path) -> Result<(), TestError> {
    let shape = PlaneShape::new(512, 512, 3, true).unwrap();
    let color = paged(&input.join("paid-rgba.color"))?;
    let alpha = paged(&input.join("paid-rgba.alpha"))?;
    let planes = PngPlanes::new(shape, &color, &alpha).unwrap();
    let mut data = [0; 4096];
    let mut identifier = PlaneIdentity::new(shape);
    while identifier.position().is_some() { assert_ne!(identifier.read_one(planes, &mut data, 4096), Some(0)); }
    let facts = identifier.finish().unwrap();
    let color_ref = facts.reference(PlaneRole::Color).unwrap();
    let mask_ref = facts.reference(PlaneRole::Alpha).unwrap();
    let mut before = PdfSnapshot::default();
    before.pages.push(PdfPage::new(512., 512.));
    let paint = PdfMutation::AppendPageContent(AppendPageContent { index: 0, content: vec![
        PdfOp::Save, PdfOp::Transform { matrix: [512., 0., 0., 512., 0., 0.] },
        PdfOp::PaintXObject { name: "Color".into() }, PdfOp::Restore,
    ] });
    let rows = PngPlacementRows::prepare(REVISION, &before, facts, "Color", Some("Opacity"), paint)?;
    let partial = output.join("canceled.pack");
    let mut fragments = 0;
    let canceled = seal_png_placement(PrivateSink::open(&partial)?, facts, planes, &rows, async |count| {
        assert!(count <= 4096);
        fragments += 1;
        if fragments == 8 { return Err(TestError::Canceled); }
        Ok(())
    }).await;
    assert!(matches!(canceled, Err(TestError::Canceled)));
    let partial_bytes = paged(&partial)?;
    assert!(PngPlacement::open(PagedPngPlacementPayload(&partial_bytes), &color_ref).await.is_err());
    fs::remove_file(partial)?;
    let archive = output.join("planes.pack");
    let done = seal_png_placement(PrivateSink::open(&archive)?, facts, planes, &rows, async |count| {
        assert!(count <= 4096); Ok(())
    }).await?;
    assert!(done.extent < 1_048_576 + 32 * 1024);
    drop(done);
    drop(color);
    drop(alpha);
    drop(rows);
    let original = paged(&archive)?;
    assert!(PngPlacement::open(PagedPngPlacementPayload(&original), &mask_ref).await.is_err());
    let install = PngPlacement::open(PagedPngPlacementPayload(&original), &color_ref).await?;
    let mut proof = install.planes();
    assert!(install.authorize_insertion(REVISION, &proof).is_err());
    let mut color_out = File::create(output.join("reopened.color"))?;
    let mut alpha_out = File::create(output.join("reopened.alpha"))?;
    while !proof.verified() {
        if let Some((role, count)) = proof.step(&mut data).await? {
            let sink = match role { PlaneRole::Color => &mut color_out, PlaneRole::Alpha => &mut alpha_out };
            sink.write_all(&data[..count])?;
        }
    }
    assert!(install.authorize_insertion([5; 32], &proof).is_err());
    let mut applied = before.clone();
    let mut undo = Vec::new();
    for op in install.authorize_insertion(REVISION, &proof)? {
        undo.push(ColdMutation::inverse(op, &applied).unwrap());
        applied = apply_mutation(&applied, op).unwrap().0;
    }
    assert!(applied.objects.is_empty());
    assert_eq!(applied.images.len(), 2);
    assert_eq!(applied.pages[0].content.len(), 4);
    let color = applied.image("Color").unwrap();
    let alpha = applied.image("Opacity").unwrap();
    assert!(install.authorize_save(color, None, &proof).is_err());
    assert!(install.authorize_save(alpha, Some(color), &proof).is_err());
    let save = install.authorize_save(color, Some(alpha), &proof)?;
    assert_eq!(save.images().0.id, "Color");
    assert_eq!(save.images().1.unwrap().id, "Opacity");
    let mut save_source = save.planes();
    let mut saved_bytes = 0;
    while !save_source.verified() {
        if let Some((_, count)) = save_source.step(&mut data).await? { saved_bytes += count; }
    }
    assert_eq!(saved_bytes, 1_048_576);
    let mut saved = applied.clone();
    for op in undo.iter().rev().flatten() { saved = apply_mutation(&saved, op).unwrap().0; }
    assert_eq!(saved, before);
    for op in install.authorize_insertion(REVISION, &proof)? { saved = apply_mutation(&saved, op).unwrap().0; }
    assert_eq!(saved, applied);
    println!("[DEBUG] Sealed PNG Pack reopened one original lease; exact PDF mutation order, inverse, redo and Save pair checks passed");
    Ok(())
}

fn main() -> Result<(), TestError> {
    let paths = std::env::args_os().skip(1).collect::<Vec<_>>();
    let run = law(Path::new(&paths[0]), Path::new(&paths[1]));
    let mut run = std::pin::pin!(run);
    match run.as_mut().poll(&mut Context::from_waker(Waker::noop())) {
        Poll::Ready(result) => result,
        Poll::Pending => panic!("file-backed staged probe unexpectedly yielded"),
    }
}
