"""🧩 Stages the private compressed resource model at its actual PDF graph owner."""

import hashlib
import json
from pathlib import Path
from os.path import relpath
from textwrap import indent

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
BASE = Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base")
OWNER = BASE / "🚪️io/📦️foreign-artifacts/🦀️.rs"
DOC = BASE / "✏️editor/📄️document"
AUTHOR = DOC / "🖼️image-authoring/🦀️.rs"
WRITER = AUTHOR.with_name("🧱️graph-writer.rs")
PLACEMENT = AUTHOR.with_name("🏞️graph-placement.rs")
MANIFEST = DOC / "📦️packages/🦀️rust/Cargo.toml"
INFERENCES = BASE / "🧬️schema/💡️inferences/📦️packages/🦀️rust"
PROBE = TICKET / "🗑️generated/raw-png-graph-overlay.json"


def stage():
    original = (ROOT / OWNER).read_bytes()
    source = original.decode("utf8")
    assert "pub mod png_source" not in source
    body = (TICKET / "raw-png-graph-source.rs").read_text(encoding="utf8")
    postimage = source.rstrip() + "\n\npub mod png_source {\n" + indent(body, "    ") + "\n}\n"
    output = {
        "preimages": {str(OWNER): hashlib.sha256(original).hexdigest()},
        "candidate": {str(OWNER): postimage},
    }
    for path in (AUTHOR, MANIFEST):
        original = (ROOT / path).read_bytes()
        output["preimages"][str(path)] = hashlib.sha256(original).hexdigest()
        output["candidate"][str(path)] = original.decode("utf8")
    author = output["candidate"][str(AUTHOR)]
    root = 'mod admission;\npub use admission::*;'
    assert author.count(root) == 1
    author = author.replace(root, root + '\n\n#[path = "🧱️graph-writer.rs"]\nmod graph_writer;\npub use graph_writer::PngGraphWriter;' +
                            '\n\n#[path = "🏞️graph-placement.rs"]\nmod graph_placement;\npub use graph_placement::{graph_png_preview, place_graph_png_named};')
    anchor = "    let taken: std::collections::HashSet<_>"
    assert author.count(anchor) == 1
    author = author.replace(anchor, """    let rgb = PdfImageBody::Samples { values: rgb.into_iter().map(u32::from).collect() };
    let alpha = alpha.iter().any(|sample| *sample != 255)
        .then(|| PdfImageBody::Samples { values: alpha.into_iter().map(u32::from).collect() });
    place_image_bodies(snapshot, placement, page, raster.width, raster.height, rgb, alpha)
}

fn place_image_bodies(
    snapshot: &PdfSnapshot, placement: ImagePlacement, page: usize, width: u32, height: u32,
    rgb: PdfImageBody, alpha: Option<PdfImageBody>,
) -> Result<Vec<PdfMutation>, Fault> {
""" + anchor)
    old = """    let mut image = PdfImage::rgb8(&id, raster.width, raster.height, rgb);
    let mut changes = Vec::with_capacity(3);
    if alpha.iter().any(|sample| *sample != 255) {
        let mask = names.next().expect("available soft mask XObject id");
        image.soft_mask = Some(mask.clone());
        changes.push(PdfMutation::SetImage(SetImage { image: PdfImage::gray8(mask, raster.width, raster.height, alpha), index: None }));
    }
"""
    new = """    let alpha = alpha.map(|body| (names.next().expect("available soft mask XObject id"), body));
    place_named_image_bodies(placement, page, width, height, id, rgb, alpha)
}

fn place_named_image_bodies(
    placement: ImagePlacement, page: usize, width: u32, height: u32,
    id: String, rgb: PdfImageBody, alpha: Option<(String, PdfImageBody)>,
) -> Result<Vec<PdfMutation>, Fault> {
    if width == 0 || height == 0 { return Err(fault("PDF image geometry is empty")); }
    if id.is_empty() || alpha.as_ref().is_some_and(|(mask, _)| mask.is_empty() || mask == &id) {
        return Err(fault("PDF image and mask require distinct reserved resource names"));
    }
    let mut image = PdfImage::rgb8(&id, width, height, Vec::new());
    image.body = rgb;
    let mut changes = Vec::with_capacity(3);
    if let Some((mask, alpha)) = alpha {
        let mut alpha_image = PdfImage::gray8(&mask, width, height, Vec::new());
        alpha_image.body = alpha;
        image.soft_mask = Some(mask);
        changes.push(PdfMutation::SetImage(SetImage { image: alpha_image, index: None }));
    }
"""
    assert author.count(old) == 1
    output["candidate"][str(AUTHOR)] = author.replace(old, new)
    manifest = output["candidate"][str(MANIFEST)]
    anchor = "semio-plugin = { workspace = true }\n"
    assert manifest.count(anchor) == 1
    relative = Path(relpath(INFERENCES, MANIFEST.parent)).as_posix()
    output["candidate"][str(MANIFEST)] = manifest.replace(anchor, anchor +
        "semio-framework-hash = { workspace = true }\nsemio-framework-value = { path = \"../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust\" }\n" +
        "semio-framework-artifact-reference = { workspace = true }\n" +
        f'semio-s-artifact-stdio-pdf-inferences = {{ path = "{relative}" }}\n')
    assert not (ROOT / WRITER).exists()
    output["preimages"][str(WRITER)] = None
    output["candidate"][str(WRITER)] = (TICKET / "raw-png-graph-writer.rs").read_text(encoding="utf8")
    assert not (ROOT / PLACEMENT).exists()
    output["preimages"][str(PLACEMENT)] = None
    output["candidate"][str(PLACEMENT)] = (TICKET / "raw-png-graph-placement.rs").read_text(encoding="utf8")
    PROBE.parent.mkdir(parents=True, exist_ok=True)
    PROBE.write_text(json.dumps(output, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
    print(f"[DEBUG] Guarded {len(output['candidate'])} PDF native graph source postimages")


if __name__ == "__main__":
    stage()
