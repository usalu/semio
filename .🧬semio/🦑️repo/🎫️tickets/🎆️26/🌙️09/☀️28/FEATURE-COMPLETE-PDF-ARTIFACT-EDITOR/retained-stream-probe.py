"""🔗️ Rehearse the retained PDF stream fix against production Resolver and content source."""

import base64
import difflib
import importlib.util
import json
import os
from pathlib import Path
import re
import sys
from tempfile import TemporaryDirectory
import zlib

TICKET = Path(__file__).resolve().parent
OUTPUT = TICKET / "🗑️generated"
OLD_ARM = "PdfObject::Stream { dict, data, .. } => normalize_stream(dict, data),"
NEW_ARM = "PdfObject::Stream { dict, data, filters } => normalize_stream(dict, data, filters),"
OLD_HELPER = """fn normalize_stream(dict: Vec<PdfDictEntry>, raw: Vec<u8>) -> PResult<PdfObject> {
    let (data, filters, normalized) = retained_stream_data(&dict, std::borrow::Cow::Owned(raw))?;"""
NEW_HELPER = """fn normalize_stream(dict: Vec<PdfDictEntry>, data: Vec<u8>, filters: Vec<PdfStreamFilter>) -> PResult<PdfObject> {
    let (data, filters, normalized) = if filters.is_empty() {
        retained_stream_data(&dict, std::borrow::Cow::Owned(data))?
    } else {
        (data, filters, true)
    };"""

IMPORT_XREF = "modules::xref::{read_file_xref,"
IMPORT_TAKE = """                let file = cursor.take(grant).expect("completed PDF byte graph under unbounded reference grants");
                return Ok(Some(file.0));"""
IMPORT_NORMALIZE = """                let (mut file, _) = cursor.take(grant).expect("completed PDF byte graph under unbounded reference grants");
                for object in &mut file.graph.objects {
                    if matches!(&object.value, PdfObject::Stream { dict, filters, .. }
                        if filters.is_empty()
                            && dict_get(dict, "Subtype").and_then(PdfObject::as_name) == Some("Image")
                            && dict.iter().any(|entry| matches!(entry.key.as_str(), "Filter" | "F"))) {
                        object.value = normalize_pdf_object(std::mem::replace(&mut object.value, PdfObject::Null))?;
                    }
                }
                return Ok(Some(file));"""


def read_fixture(base):
    fixture = base / "✏️editor/🖼️page/🧪️tests/🔬️unit/🧪️coded-image.json"
    return [base64.b64decode(case["pdfBase64"]) for case in json.loads(fixture.read_text())["cases"]]


def stage_overlay(path, old_arm, new_arm, old_helper, new_helper, filename):
    source = path.read_text()
    assert source.count(old_arm) == source.count(old_helper) == 1
    staged = source.replace(old_arm, new_arm).replace(old_helper, new_helper)
    diff = difflib.unified_diff(source.splitlines(True), staged.splitlines(True), "a/" + str(path), "b/" + str(path))
    patch = TICKET / filename
    content = "".join(diff)
    if not patch.exists() or patch.read_text() != content:
        patch.write_text(content)
    return source, staged


def changes(base):
    path = base / "🔨️modules/🔗️xref/🦀️.rs"
    original, staged = stage_overlay(path, OLD_ARM, NEW_ARM, OLD_HELPER, NEW_HELPER, "retained-stream-pipeline.patch")
    importer = base / "🚪️io/📥️import/🦀️.rs"
    import_original, import_staged = stage_overlay(
        importer, IMPORT_XREF, IMPORT_XREF.replace("{read_file_xref", "{normalize_pdf_object, read_file_xref"),
        IMPORT_TAKE, IMPORT_NORMALIZE, "cold-dct-import.patch",
    )
    return [(path, original, staged), (importer, import_original, import_staged)]


def apply(base):
    staged_files = changes(base)
    assert all(path.read_text() == original for path, original, _ in staged_files)
    for path, original, staged in staged_files:
        assert path.read_text() == original, f"Concurrent edit to {path}"
        path.write_text(staged)


def exercise(base, mode):
    spec = importlib.util.spec_from_file_location("codec_probe", TICKET / "📜️cff-file-codec-probe.py")
    probe = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(probe)
    if mode == "live":
        path, importer = base / "🔨️modules/🔗️xref/🦀️.rs", base / "🚪️io/📥️import/🦀️.rs"
        original = source = path.read_text()
        import_original = reader = importer.read_text()
    else:
        (path, original, staged), (importer, import_original, import_staged) = changes(base)
        source = staged if mode == "after" else original
        reader = import_staged if mode == "after" else import_original
    cold_read = reader[reader.index("fn read_clear_pdf_graph("):reader.index("fn read_pdf_reference(")].rstrip()
    original_stage = probe.probes.stage_paths
    os.environ["PDF_CONTROLLED_LIVE"] = "1"

    def stage_paths(base, scratch, extra):
        extra = original_stage(base, scratch, extra)
        if mode == "live":
            return extra
        resolved = path.resolve()
        overlay = scratch / "staged-xref.rs"
        relocated = re.sub(r'#\[path = "([^"]+)"\]', lambda match: f'#[path = "{(resolved.parent / match[1]).as_posix()}"]', source)
        overlay.write_text(relocated)
        return extra.replace(f'#[path = "{resolved.as_posix()}"]', f'#[path = "{overlay.as_posix()}"]')

    probe.probes.stage_paths = stage_paths
    with TemporaryDirectory(prefix="jpeg-xref-", dir=OUTPUT) as folder:
        inputs = Path(folder)
        jfif, adobe, inline = read_fixture(base)
        for name, data in (("jfif", jfif), ("adobe", adobe), ("inline", inline)):
            (inputs / (name + ".pdf")).write_bytes(data)
        (inputs / "gray.jpg").write_bytes(bytes.fromhex(re.search(rb"ID\n([0-9a-f]+)>", inline)[1].decode()))
        (inputs / "encoded").write_bytes(zlib.compress(b"cold flate stream"))
        (inputs / "predictor").write_bytes(zlib.compress(bytes([2, 2, 4, 6, 8, 2, 1, 3, 5, 7])))
        (inputs / "not-image.pdf").write_bytes(jfif.replace(b"/Subtype /Image", b"/Subtype /Other", 1))
        (inputs / "unknown.pdf").write_bytes(jfif.replace(b"/DCTDecode ", b"/UnknownXX ", 1))
        body = (TICKET / "retained-stream-probe.rs").read_text().replace("@INPUT@", inputs.as_posix())
        body = body.replace("@FIXED@", str(mode != "before").lower())
        body = body.replace("@COLD_READER@", cold_read)
        probe.exercise(base, "unused.pdf", body=body)
    assert path.read_text() == original and importer.read_text() == import_original, "live source changed during rehearsal"


if __name__ == "__main__":
    sys.dont_write_bytecode = True
    mode = sys.argv[1]
    assert mode in {"before", "after", "apply", "live"}
    base = Path(sys.argv[2])
    OUTPUT.mkdir(exist_ok=True)
    if mode == "apply":
        apply(base)
    else:
        exercise(base, mode)
