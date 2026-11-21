"""🧪 Verify PDF image authoring against a language-neutral fixture and PyMuPDF."""

from base64 import b64decode
import argparse
import json
from pathlib import Path
import runpy
import subprocess
import sys

import fitz

TICKET = Path(__file__).resolve().parent
STAGE = runpy.run_path(str(TICKET / "image-authoring-stage.py"))
FIXTURE = json.loads((TICKET / "image-authoring.json").read_text(encoding="utf-8"))
OUTPUT = STAGE["OUTPUT"]

TYPES = r'''
#![allow(dead_code)]
type Fault = String;
type PdfMatrix = [f64;6];
fn fault(text: &str) -> Fault { text.to_owned() }
pub struct RasterImage { width: u32, height: u32, pixels: Vec<u8> }
impl RasterImage {
    fn sampled(&self) -> image_authoring::SampledRgba<'_> {
        image_authoring::SampledRgba { width: self.width, height: self.height, pixels: &self.pixels }
    }
}
#[derive(Debug, PartialEq, Eq)]
struct PdfImage { id: String, width: u32, height: u32, values: Vec<u8>, soft_mask: Option<String> }
impl PdfImage {
    fn rgb8(id: impl Into<String>, width: u32, height: u32, values: Vec<u8>) -> Self {
        Self { id: id.into(), width, height, values, soft_mask: None }
    }
    fn gray8(id: impl Into<String>, width: u32, height: u32, values: Vec<u8>) -> Self { Self::rgb8(id, width, height, values) }
}
struct Form { id: String }
struct PdfSnapshot { pages: Vec<Page>, images: Vec<PdfImage>, forms: Vec<Form> }
impl PdfSnapshot { fn image(&self, id: &str) -> Option<&PdfImage> { self.images.iter().find(|image| image.id == id) } }
struct Page { content: Vec<PdfOp> }
#[derive(Debug, PartialEq)]
enum PdfOp {
    Save, Restore, NonImage, InlineImage { image: PdfImage }, PaintXObject { name: String },
    Transform { matrix: PdfMatrix },
}
struct SetImage { image: PdfImage, index: Option<usize> }
pub struct AppendPageContent { index: usize, content: Vec<PdfOp> }
pub struct ReplaceContent { index: usize, at: usize, op: PdfOp }
enum PdfMutation { SetImage(SetImage), AppendPageContent(AppendPageContent), ReplaceContent(ReplaceContent) }
'''

TEST = r'''
fn fixture() -> (PdfSnapshot, RasterImage) {
    let snapshot = PdfSnapshot {
        pages: vec![
            Page { content: vec![
                PdfOp::Save, PdfOp::Transform { matrix: @MATRIX@ }, PdfOp::PaintXObject { name: "@EXISTING_IMAGE@".into() }, PdfOp::Restore,
                PdfOp::Save, PdfOp::Transform { matrix: @INLINE_MATRIX@ }, PdfOp::InlineImage { image: PdfImage::rgb8("", 1, 1, vec![0;3]) }, PdfOp::Restore,
            ]},
            Page { content: vec![PdfOp::PaintXObject { name: "@EXISTING_IMAGE@".into() }] },
        ],
        images: vec![PdfImage::rgb8("@EXISTING_IMAGE@", 1, 1, vec![0; 3])],
        forms: vec![Form { id: "@EXISTING_FORM@".into() }],
    };
    (snapshot, RasterImage { width: @WIDTH@, height: @HEIGHT@, pixels: vec!@RGBA@ })
}

fn verify_images(changes: &[PdfMutation]) {
    let [PdfMutation::SetImage(mask), PdfMutation::SetImage(image), _] = changes else { panic!("expected atomic mask, image and page placement") };
    assert!(mask.index.is_none() && image.index.is_none());
    assert_eq!(mask.image.id, "@MASK@");
    assert_eq!(mask.image.values, @ALPHA@);
    assert_eq!(image.image.id, "@IMAGE@");
    assert_eq!(image.image.values, @RGB@);
    assert_eq!(image.image.soft_mask.as_deref(), Some("@MASK@"));
    assert_eq!((image.image.width, image.image.height), (@WIDTH@, @HEIGHT@));
    assert_eq!((mask.image.width, mask.image.height), (@WIDTH@, @HEIGHT@));
}

fn main() {
    use image_authoring::{place_sampled_image, ImagePlacement, MAX_IMAGE_IMPORT_PIXELS};

    let (snapshot, mut raster) = fixture();
    let paint = || PdfOp::PaintXObject { name: "@IMAGE@".into() };
    let insert = || ImagePlacement::Insert { page: 0, matrix: @INSERT_MATRIX@ };
    let changes = place_sampled_image(&snapshot, insert(), raster.sampled()).unwrap();
    verify_images(&changes);
    let PdfMutation::AppendPageContent(placement) = &changes[2] else { panic!("expected a page append") };
    assert_eq!(placement.index, 0);
    assert_eq!(placement.content, [PdfOp::Save, PdfOp::Transform { matrix: @INSERT_MATRIX@ }, paint(), PdfOp::Restore]);
    for operator in [@REPLACEMENT@, 6] {
        let changes = place_sampled_image(&snapshot, ImagePlacement::Replace { page: 0, operator }, raster.sampled()).unwrap();
        verify_images(&changes);
        let PdfMutation::ReplaceContent(placement) = &changes[2] else { panic!("expected one replacement") };
        assert_eq!((placement.index, placement.at, &placement.op), (0, operator, &paint()));
        assert_eq!(snapshot.images[0].values, [0;3]);
        assert_eq!(snapshot.pages[1].content, [PdfOp::PaintXObject { name: "@EXISTING_IMAGE@".into() }]);
    }
    assert!(place_sampled_image(&snapshot, ImagePlacement::Replace { page: 0, operator: 1 }, raster.sampled()).is_err());
    assert!(place_sampled_image(&snapshot, ImagePlacement::Replace { page: 2, operator: 0 }, raster.sampled()).is_err());
    assert!(place_sampled_image(&snapshot, ImagePlacement::Insert { page: 0, matrix: [0.0;6] }, raster.sampled()).is_err());
    assert!(place_sampled_image(&snapshot, ImagePlacement::Insert { page: 0, matrix: [f64::NAN;6] }, raster.sampled()).is_err());
    for sample in raster.pixels.chunks_exact_mut(4) { sample[3] = 255; }
    let changes = place_sampled_image(&snapshot, insert(), raster.sampled()).unwrap();
    let [PdfMutation::SetImage(image), PdfMutation::AppendPageContent(_)] = changes.as_slice() else { panic!("opaque images do not require a mask") };
    assert!(image.image.soft_mask.is_none());
    raster.pixels.pop();
    assert!(place_sampled_image(&snapshot, insert(), raster.sampled()).is_err());
    assert_eq!(MAX_IMAGE_IMPORT_PIXELS, @MAX_PIXELS@);
    raster.width = 1;
    raster.height = (MAX_IMAGE_IMPORT_PIXELS + 1) as u32;
    raster.pixels = vec![0; raster.height as usize * 4];
    assert!(place_sampled_image(&snapshot, insert(), raster.sampled()).is_err());
    raster.width = u32::MAX;
    raster.height = u32::MAX;
    assert!(place_sampled_image(&snapshot, insert(), raster.sampled()).is_err());
    println!("[DEBUG] Staged source passed insert, inline/referenced selected replacement, ID collision, alpha policy and bounds");
}
'''


def exact_source_probe():
    """🔬 Compile the exact staged functions against the small PDF adapter surface they use."""
    fixture = FIXTURE
    expected = fixture["expected"]
    fields = {
        "MATRIX": list(map(float, fixture["existing"]["placement"])),
        "INLINE_MATRIX": list(map(float, fixture["existing"]["inlinePlacement"])),
        "INSERT_MATRIX": list(map(float, fixture["existing"]["insertPlacement"])),
        "EXISTING_IMAGE": fixture["existing"]["images"][0],
        "EXISTING_FORM": fixture["existing"]["forms"][0],
        "WIDTH": fixture["width"],
        "HEIGHT": fixture["height"],
        "RGBA": fixture["rgba"],
        "MASK": expected["mask"],
        "IMAGE": expected["image"],
        "ALPHA": fixture["softMask"],
        "RGB": fixture["rgb"],
        "REPLACEMENT": expected["replacement"],
        "MAX_PIXELS": fixture["bounds"]["maxPixels"],
    }
    tests = TEST
    for key, value in fields.items():
        tests = tests.replace(f"@{key}@", str(value))
    OUTPUT.mkdir(parents=True, exist_ok=True)
    source = OUTPUT / "probe.rs"
    binary = OUTPUT / ("probe.exe" if sys.platform == "win32" else "probe")
    source.write_text(TYPES + "\nmod image_authoring {\n" + STAGE["SOURCE"] + "\n}\n" + tests, encoding="utf-8")
    subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "--edition=2024", str(source), "-o", str(binary)], check=True)
    subprocess.run([str(binary.resolve())], check=True)


def image_object(doc, dimensions, stream, space, soft_mask=None):
    """🖼️ Write an independent raw PDF image or its grayscale soft mask."""
    ref = doc.get_new_xref()
    width, height = dimensions
    doc.update_object(ref, f"<< /Type /XObject /Subtype /Image /Width {width} /Height {height} /ColorSpace /{space} /BitsPerComponent 8 >>")
    doc.update_stream(ref, bytes(stream))
    if soft_mask is not None:
        doc.xref_set_key(ref, "SMask", f"{soft_mask} 0 R")
    return ref


def render_oracle():
    """🌇 Decode the local fixture and render isolated new image placements in PyMuPDF."""
    data = FIXTURE
    dims = (data["width"], data["height"])
    source = fitz.Pixmap(b64decode(data["png"]))
    assert (source.width, source.height, source.n) == (*dims, 4)
    assert list(source.samples[3::4]) == data["softMask"]
    for index in range(len(data["softMask"])):
        offset = index * 4
        alpha = data["softMask"][index]
        expected = [(channel * alpha + 127) // 255 for channel in data["rgba"][offset:offset + 3]]
        assert list(source.samples[offset:offset + 3]) == expected
    with fitz.open() as doc:
        doc.new_page(width=150, height=100)
        doc.new_page(width=150, height=100)
        prior = image_object(doc, (1, 1), [0, 0, 0], "DeviceRGB")
        mask = image_object(doc, dims, data["softMask"], "DeviceGray")
        rgba = image_object(doc, dims, data["rgb"], "DeviceRGB", mask)
        first = " ".join(map(str, data["existing"]["placement"]))
        second = " ".join(map(str, data["existing"]["inlinePlacement"]))
        inline = b"BI /W 1 /H 1 /CS /RGB /BPC 8 ID \x00\x00\x00 EI"
        third = " ".join(map(str, data["existing"]["insertPlacement"]))
        for mode in ("insert", "replace-reference", "replace-inline", "undo", "redo"):
            prefix = f"q {first} cm ".encode()
            suffix = f"q {second} cm ".encode()
            reference = b"/Before Do" if mode in ("insert", "replace-inline", "undo") else b"/After Do"
            content = prefix + reference + b" Q"
            if mode in ("insert", "replace-reference", "undo", "redo"):
                content += b"\n" + suffix + inline + b" Q"
            else:
                content += b"\n" + suffix + b"/After Do Q"
            if mode == "insert":
                content += f"\nq {third} cm /After Do Q".encode()
            for page_number, page_content in [(0, content), (1, prefix + b"/Before Do Q")]:
                page = doc[page_number]
                stream = doc.get_new_xref()
                doc.update_object(stream, "<<>>")
                doc.update_stream(stream, page_content)
                page.set_contents(stream)
                doc.xref_set_key(page.xref, "Resources", f"<< /XObject << /Before {prior} 0 R /After {rgba} 0 R >> >>")
            with fitz.open(stream=doc.tobytes(), filetype="pdf") as reopened:
                bitmap = reopened[0].get_pixmap(colorspace=fitz.csRGB)
                original = reopened[1].get_pixmap(colorspace=fitz.csRGB)
                for x, y, expected in data["expected"]["pixelProbes"]:
                    assert original.pixel(x, y) == (0, 0, 0)
                    if mode == "insert":
                        assert bitmap.pixel(x, y) == (0, 0, 0)
                        assert all(abs(a - b) <= 1 for a, b in zip(bitmap.pixel(x, y - 40), expected))
                    elif mode == "undo":
                        assert bitmap.pixel(x, y) == (0, 0, 0)
                    elif mode == "replace-inline":
                        assert bitmap.pixel(x, y) == (0, 0, 0)
                        assert all(abs(a - b) <= 1 for a, b in zip(bitmap.pixel(x + 70, y), expected))
                    else:
                        assert all(abs(a - b) <= 1 for a, b in zip(bitmap.pixel(x, y), expected))
                        assert bitmap.pixel(x + 70, y) == (0, 0, 0)
    print("[DEBUG] PyMuPDF decoded the local PNG and independently verified alpha, placement isolation and reopened output")


def write_browser_source(directory):
    """🧫 Prepare a two-page PDF with one shared reference and one inline image."""
    directory.mkdir(parents=True, exist_ok=True)
    geometry = FIXTURE["existing"]
    first = " ".join(map(str, geometry["placement"]))
    second = " ".join(map(str, geometry["inlinePlacement"]))
    with fitz.open() as doc:
        doc.new_page(width=150, height=100)
        doc.new_page(width=150, height=100)
        prior = image_object(doc, (1, 1), [0, 0, 0], "DeviceRGB")
        first_paint = f"q {first} cm /Before Do Q".encode()
        inline_paint = f"\nq {second} cm ".encode() + b"BI /W 1 /H 1 /CS /RGB /BPC 8 ID \x00\x00\x00 EI Q"
        for page, paint in [(doc[0], first_paint + inline_paint), (doc[1], first_paint)]:
            stream = doc.get_new_xref()
            doc.update_object(stream, "<<>>")
            doc.update_stream(stream, paint)
            page.set_contents(stream)
            doc.xref_set_key(page.xref, "Resources", f"<< /XObject << /Before {prior} 0 R >> >>")
        target = directory / "image-source.pdf"
        if target.exists():
            target.unlink()
        doc.save(target)


def verify_browser_output(directory):
    """📐 Compare the mounted editor's actual exported documents to the independent pixel fixture."""
    expected = {
        "image-inserted.pdf": (False, False),
        "image-replaced.pdf": (True, False),
        "image-undone.pdf": (False, False),
        "image-redone.pdf": (True, False),
        "image-inline.pdf": (True, True),
        "image-cancelled.pdf": (True, True),
    }
    geometry = FIXTURE["existing"]
    inline_dx = geometry["inlinePlacement"][4] - geometry["placement"][4]
    insert_dy = geometry["insertPlacement"][5] - geometry["placement"][5]
    black = [0, 0, 0]
    white = [255, 255, 255]
    for name, (replaced, replaced_inline) in expected.items():
        with fitz.open(directory / name) as doc:
            assert doc.page_count == 2
            edited = doc[0].get_pixmap(colorspace=fitz.csRGB)
            shared = doc[1].get_pixmap(colorspace=fitz.csRGB)
            for x, y, sample in FIXTURE["expected"]["pixelProbes"]:
                checks = [
                    (edited, x, y, sample if replaced else black),
                    (edited, x + inline_dx, y, sample if replaced_inline else black),
                    (edited, x, y - insert_dy, sample),
                    (shared, x, y, black),
                    (shared, x, y - insert_dy, white),
                ]
                for pixmap, at_x, at_y, color in checks:
                    actual = pixmap.pixel(at_x, at_y)
                    assert all(abs(a - b) <= 1 for a, b in zip(actual, color)), (name, at_x, at_y, actual, color)
    print("[DEBUG] Verified mounted PDF downloads, both image placement replacements, undo/redo and unchanged cancelled output")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--oracle-only", action="store_true")
    mode.add_argument("--prepare-browser", type=Path)
    mode.add_argument("--verify-browser", type=Path)
    args = parser.parse_args()
    if args.prepare_browser:
        write_browser_source(args.prepare_browser)
    elif args.verify_browser:
        verify_browser_output(args.verify_browser)
    else:
        if not args.oracle_only:
            exact_source_probe()
        render_oracle()
