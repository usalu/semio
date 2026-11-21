extern crate self as semio_s_artifact_stdio_pdf_inferences;

#[path = "🔎️dct-lookup.rs"]
pub mod embedded_lookup;
pub(crate) use foreign_artifacts::qualified_image_dictionary;
#[path = "authoring-core.rs"]
pub mod authoring;
use authoring::dct_placement::PendingDctPlacement;
use authoring::ImagePlacement;
use embedded_lookup::{DctLookupError, NativePdfDctLookup};

pub mod stream_roles {
    use crate::{ObjRef, PdfImage};
    pub struct GraphIdentity { pub owner: ObjRef, pub path: Vec<()> }
    pub struct AdmittedRole { pub identity: GraphIdentity, pub value: PdfStreamRoleValue }
    pub enum PdfStreamRoleValue { Image { image: PdfImage }, Metadata }
}

pub type PdfMatrix = [f64; 6];
type Fault = String;
fn fault(detail: impl Into<String>) -> Fault { detail.into() }

#[derive(Clone)]
pub enum PdfOp {
    Save, Restore, Transform { matrix: PdfMatrix }, PaintXObject { name: String }, InlineImage {},
}

#[derive(Clone)]
pub enum PdfColorSpace { DeviceGray, DeviceRgb, Other }

#[derive(Clone)]
pub enum PdfImageBody { Samples { values: Vec<u32> }, Artifact { reference: ArtifactRef } }

#[derive(Clone)]
pub struct PdfImage {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub color_space: Option<PdfColorSpace>,
    pub bits_per_component: u32,
    pub image_mask: bool,
    pub body: PdfImageBody,
    pub soft_mask: Option<String>,
}
impl PdfImage {
    pub fn samples(id: impl Into<String>, width: u32, height: u32, color: PdfColorSpace, bits: u32, values: Vec<u32>) -> Self {
        Self { id: id.into(), width, height, color_space: Some(color), bits_per_component: bits, image_mask: false, body: PdfImageBody::Samples { values }, soft_mask: None }
    }
    pub fn gray8(id: impl Into<String>, width: u32, height: u32, values: Vec<u8>) -> Self {
        Self::samples(id, width, height, PdfColorSpace::DeviceGray, 8, values.into_iter().map(u32::from).collect())
    }
    pub fn rgb8(id: impl Into<String>, width: u32, height: u32, values: Vec<u8>) -> Self {
        Self::samples(id, width, height, PdfColorSpace::DeviceRgb, 8, values.into_iter().map(u32::from).collect())
    }
}

pub struct PdfSnapshot {
    pub images: Vec<PdfImage>,
    pub forms: Vec<Form>,
    pub pages: Vec<Page>,
    pub objects: Vec<PdfIndirectObject>,
    pub admitted_stream_roles: Vec<stream_roles::AdmittedRole>,
}
impl PdfSnapshot { pub fn image(&self, id: &str) -> Option<&PdfImage> { self.images.iter().find(|image| image.id == id) } }
pub struct Page { pub content: Vec<PdfOp> }
pub struct Form { pub id: String }
pub struct SetImage { pub image: PdfImage, pub index: Option<usize> }
pub struct AppendPageContent { pub index: usize, pub content: Vec<PdfOp> }
pub struct ReplaceContent { pub index: usize, pub at: usize, pub op: PdfOp }
pub enum PdfMutation { SetImage(SetImage), AppendPageContent(AppendPageContent), ReplaceContent(ReplaceContent) }

pub mod baseline_resource {
    use super::{PdfDctImageColor, PdfDctImageView};
    pub struct AdmittedPdfJpeg {
        pub original: Vec<u8>,
        pub digest: [u8; 32],
    }
    impl AdmittedPdfJpeg {
        pub fn stream_view(&self) -> Option<PdfDctImageView<'_>> {
            PdfDctImageView::from_admitted(&self.original, (1024, 1024), PdfDctImageColor::Rgb { transform: Some(true) }, self.digest)
        }
    }
}

const PINNED: [u8; 32] = [17; 32];
const STALE: [u8; 32] = [18; 32];

fn exercise(dir: &Path) -> io::Result<()> {
    use stream_roles::{AdmittedRole, GraphIdentity, PdfStreamRoleValue};
    let original = fs::read(dir.join("large.jpg"))?;
    let mut hash = PdfDctResourceIdentity::new(Some(1)).unwrap();
    hash.advance(&original, original.len());
    let reference = hash.reference();
    let mut image = PdfImage::samples("SemioImage1", 1024, 1024, PdfColorSpace::DeviceRgb, 8, Vec::new());
    image.body = PdfImageBody::Artifact { reference: reference.clone() };
    let mut snapshot = PdfSnapshot {
        images: vec![image.clone()],
        forms: vec![Form { id: "SemioImage2".into() }],
        pages: vec![Page { content: vec![PdfOp::PaintXObject { name: image.id.clone() }] }],
        objects: vec![object(97, PdfObject::Null), object(5, PdfObject::Stream {
            dict: vec![
                PdfDictEntry::new("Type", PdfObject::name("XObject")),
                PdfDictEntry::new("Subtype", PdfObject::name("Image")),
                PdfDictEntry::new("Width", PdfObject::Int(1024)),
                PdfDictEntry::new("Height", PdfObject::Int(1024)),
                PdfDictEntry::new("BitsPerComponent", PdfObject::Int(8)),
                PdfDictEntry::new("ColorSpace", PdfObject::name("DeviceRGB")),
            ],
            data: original,
            filters: vec![PdfStreamFilter::Dct { color_transform: Some(1) }],
        })],
        admitted_stream_roles: vec![AdmittedRole {
            identity: GraphIdentity { owner: id(5), path: Vec::new() },
            value: PdfStreamRoleValue::Image { image },
        }],
    };

    let mut lookup = NativePdfDctLookup::new(PINNED, &reference).unwrap();
    assert_eq!(lookup.step(STALE, &snapshot, 4096), Err(DctLookupError::StaleRevision));
    let mut scanned = 0;
    while lookup.view(PINNED, &snapshot).unwrap().is_none() {
        let spent = lookup.step(PINNED, &snapshot, 32768).unwrap();
        assert!(spent > 0 && spent <= 4096);
        scanned += spent;
    }
    assert!(matches!(lookup.view(STALE, &snapshot), Err(DctLookupError::StaleRevision)));
    let view = lookup.view(PINNED, &snapshot).unwrap().unwrap();
    let PdfObject::Stream { data, .. } = &snapshot.objects[1].value else { unreachable!() };
    assert_eq!(scanned, data.len() + 3);
    assert_eq!(view.reference(), reference);
    assert_eq!(view.encoded().byte_at(0), Some(255));
    assert!(std::ptr::addr_eq(view.encoded(), data as &dyn ByteReadSource));

    emit_pdf(dir, "native-graph", view)?;

    let PdfObject::Stream { data, .. } = &mut snapshot.objects[1].value else { unreachable!() };
    data[4096] ^= 1;
    let mut rejected = NativePdfDctLookup::new(PINNED, &reference).unwrap();
    loop {
        match rejected.step(PINNED, &snapshot, 4096) {
            Ok(spent) => assert!(spent > 0),
            Err(DctLookupError::DigestMismatch) => break,
            Err(error) => panic!("{error:?}"),
        }
    }
    let PdfObject::Stream { data, .. } = &mut snapshot.objects[1].value else { unreachable!() };
    data[4096] ^= 1;
    let original = std::mem::take(data);
    let start = original.as_ptr();
    let owned = baseline_resource::AdmittedPdfJpeg { original, digest: hash.digest() };

    let invalid = PendingDctPlacement::prepare(PINNED, &snapshot, ImagePlacement::Replace { page: 0, operator: 3 }, owned);
    let owned = match invalid { Err((_, owned)) => owned, Ok(_) => panic!("missing operator accepted") };
    let insert = ImagePlacement::Insert { page: 0, matrix: [100.0, 0.0, 0.0, 100.0, 12.0, 18.0] };
    let pending = PendingDctPlacement::prepare(PINNED, &snapshot, insert, owned).ok().expect("valid insert");
    let (view, edits) = pending.borrow(PINNED).unwrap();
    let [PdfMutation::SetImage(set), PdfMutation::AppendPageContent(append)] = edits else { panic!("DCT insert row shape"); };
    let [PdfOp::Save, PdfOp::Transform { .. }, PdfOp::PaintXObject { name }, PdfOp::Restore] = append.content.as_slice() else {
        panic!("DCT insert paint shape");
    };
    assert_eq!(name, "SemioImage3");
    let PdfImageBody::Artifact { reference } = &set.image.body else { panic!("exact DCT ref is required"); };
    assert_eq!(*reference, view.reference());
    assert!(pending.borrow(STALE).is_err());
    let pending = match pending.take_for_publication(STALE) {
        Err(pending) => pending, Ok(_) => panic!("stale resource transaction was published"),
    };
    let (owned, _) = pending.take_for_publication(PINNED).ok().expect("fresh transaction");
    assert_eq!(owned.original.as_ptr(), start);

    let replace = ImagePlacement::Replace { page: 0, operator: 0 };
    let pending = PendingDctPlacement::prepare(PINNED, &snapshot, replace, owned).ok().expect("valid replacement");
    let (_, edits) = pending.borrow(PINNED).unwrap();
    let [PdfMutation::SetImage(set), PdfMutation::ReplaceContent(replace)] = edits else { panic!("DCT replace row shape"); };
    let PdfOp::PaintXObject { name } = &replace.op else { panic!("replacement uses external image paint"); };
    assert_eq!(set.image.id, *name);
    assert_eq!(replace.at, 0);
    assert_eq!(snapshot.images.len(), 1);
    assert_eq!(pending.abort().original.as_ptr(), start);
    println!("[DEBUG] exact native DCT reloan, bounded identity verification, insert/replace, stale and reject controls");
    Ok(())
}

fn main() -> io::Result<()> {
    let dir = std::env::args_os().nth(1).unwrap();
    exercise(Path::new(&dir))
}
