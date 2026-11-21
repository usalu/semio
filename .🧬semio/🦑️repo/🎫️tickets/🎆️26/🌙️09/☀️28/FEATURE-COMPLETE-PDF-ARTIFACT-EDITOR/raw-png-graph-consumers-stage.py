"""🎨 Stages the PNG-specific consumers at the actual preview, native resource and PDF lower owners."""

import hashlib
import json
from os.path import relpath
from pathlib import Path
import runpy
from textwrap import indent

TICKET = Path(__file__).resolve().parent
STAGE = runpy.run_path(str(TICKET / "raw-png-graph-stage.py"))
BASE = STAGE["BASE"]
ROOT = STAGE["ROOT"]
OWNER = STAGE["OWNER"]
OUT = TICKET / "🗑️generated/raw-png-consumers-overlay.json"


def stage():
    STAGE["stage"]()
    output = json.loads(STAGE["PROBE"].read_text(encoding="utf8"))
    preimages = output["preimages"]
    postimages = output["candidate"]

    def edit(path, anchor, replacement):
        original = (ROOT / path).read_bytes()
        preimages[str(path)] = hashlib.sha256(original).hexdigest()
        text = postimages.get(str(path), original.decode("utf8"))
        assert text.count(anchor) == 1, (path, anchor)
        postimages[str(path)] = text.replace(anchor, replacement)

    edit(OWNER, "    fn resolve(&self,reference:&ArtifactRef)->PResult<PdfObject> {",
         """    fn resolve(&self,reference:&ArtifactRef)->PResult<PdfObject> {
        if png_source::PngPlane::match_reference(reference).is_some() {
            let (role, source) = png_source::PngGraphSource::resolve(self.graph, reference)
                .map_err(|_| PdfEngineError::Malformed("retained PDF PNG source binding is incomplete".into()))?;
            return png_source::samples::lower_source_plane(role, source);
        }""")
    marked = "pub mod png_source {\n"
    resolver = (TICKET / "raw-png-graph-resolve.rs").read_text(encoding="utf8")
    body = postimages[str(OWNER)]
    end = body.rindex("\n}")
    assert body.count(marked) == 1
    postimages[str(OWNER)] = body[:end] + "\n    pub(super) mod samples {\n" + indent(resolver, "        ") + "    }" + body[end:]
    atoms = STAGE["INFERENCES"] / "Cargo.toml"
    vendor = Path("🧰️framework/🔨️modules/🔲️pixels/📦️packages/🦀️rust")
    relative = Path(relpath(vendor, atoms.parent)).as_posix()
    edit(atoms, "[dependencies]\n", f'[dependencies]\nsemio-framework-pixels = {{ path = "{relative}" }}\n')

    images = BASE / "🔨️modules/🖼️images/🦀️.rs"
    old = 'PdfImageBody::Artifact {reference}=>{let PdfObject::Stream {data,filters,..}=sink.resolve_artifact(reference)? else{return Err(PdfEngineError::Unsupported("image artifact is not a native stream".into()));};if !filters.iter().any(|filter|filter.is_image_codec()){return Err(PdfEngineError::Unsupported("referenced image artifact has no native image codec".into()));}(data,filters)},'
    new = """PdfImageBody::Artifact {reference}=>{
            use crate::standards::v1_7::subsets::base::io::foreign_artifacts::png_source::PngPlane;
            let value = sink.resolve_artifact(reference)?;
            let sourced_png = PngPlane::match_reference(reference);
            if let Some(role) = sourced_png {
                let number = |key| value.dict_get(key).and_then(PdfObject::as_i64);
                if image.bits_per_component != 8 || role.channels() != image_components(image) as usize
                    || number("Width") != Some(i64::from(image.width)) || number("Height") != Some(i64::from(image.height))
                {
                    return Err(PdfEngineError::Malformed("PDF PNG resource geometry changed before image lowering".into()));
                }
            }
            let PdfObject::Stream {data,filters,..} = value else {
                return Err(PdfEngineError::Unsupported("image artifact is not a native stream".into()));
            };
            if sourced_png.is_none() && !filters.iter().any(|filter|filter.is_image_codec()) {
                return Err(PdfEngineError::Unsupported("referenced image artifact has no native image codec".into()));
            }
            (data,filters)
        },"""
    edit(images, old, new)

    preview = BASE / "✏️editor/🖼️page/🔆️image-preview/🦀️.rs"
    edit(preview, "dct_samples(&resources, reference, size, channels);", """semio_s_artifact_stdio_pdf_editor_document::graph_png_preview(&resources, reference, size, channels)
        .or_else(|| dct_samples(&resources, reference, size, channels));""")

    export = BASE / "🚪️io/📤️export/🧬️encode-and-reconcile/🦀️.rs"
    edit(export, "let (objects, trailer) = if retained {", "let (mut objects, trailer) = if retained {")
    edit(export, "        Ok(Self { version, objects, trailer, write })", """        let private = |object: &PdfIndirectObject| crate::standards::v1_7::subsets::base::io::foreign_artifacts::png_source::referenced_source_object(&object.value, &snapshot.images);
        if let Cow::Owned(objects) = &mut objects { objects.retain(|object| !private(object)); }
        else if objects.iter().any(private) { return Err(PdfEngineError::Malformed("retained PDF PNG source requires lowered image resources".into())); }
        Ok(Self { version, objects, trailer, write })""")
    OUT.write_text(json.dumps(output, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
    print(f"[DEBUG] Guarded {len(postimages)} PDF PNG graph and preview/lowering postimages")


if __name__ == "__main__":
    stage()
