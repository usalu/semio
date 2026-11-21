"""🔎 Audits the coherent staged PDF archive install against two captured builds."""

from argparse import ArgumentParser
from hashlib import sha256
import json
from pathlib import Path
import re
import runpy
from textwrap import indent
import unicodedata

TICKET = Path(__file__).resolve().parent
GENERATED = TICKET / "🗑️generated"
ARCHIVE = GENERATED / "archive-leaves"
OUT = GENERATED / "archive-install"
ARCHIVE_STAGE = runpy.run_path(str(TICKET / "archive-leaves-stage.py"))
VT_STAGE = runpy.run_path(str(TICKET / "vt-archive-load-stage.py"))
NATIVE_LOG = GENERATED / "pdf-editor-native-gate-media-handoff-retry.log"
BROWSER_LOG = GENERATED / "pdf-browser-direct-components-factory-repair.log"


def source_admissions(path):
    entries = {}
    with path.open(errors="replace") as log:
        for line in log:
            if '"resource":"cargo:source-admission:' not in line:
                continue
            stage = json.loads(line.split("[script] ", 1)[1])
            reading = json.loads(stage["activity"]["resource"].split("cargo:source-admission:", 1)[1])
            if reading["phase"] == "complete":
                entries.setdefault(reading["packageName"], set()).add(reading["unit"])
    return entries


def show(path, owners):
    readers = source_admissions(path)
    return {name: sorted(readers[name]) for name in sorted(owners) if name in readers}


def digest(path):
    return sha256(path.read_bytes()).hexdigest() if path.is_file() else None


def verify(workspace, rows, browser_store):
    matches = {}
    for relative, row in rows.items():
        live = workspace / relative
        staged = OUT / row["stage"]
        assert digest(staged) == row["after"]
        assert digest(live) == row["before"], relative
        path = unicodedata.normalize("NFD", str(live.resolve()))
        assert len(path.encode("utf-16-le")) // 2 <= 256, relative
        source = staged.read_text()
        lines = len(source.splitlines())
        assert staged.suffix != ".rs" or lines <= 4000, relative
        captured = row["before"] is not None and digest(browser_store / row["before"]) == row["before"]
        matches[relative] = {**row, "lines": lines,
            "absolutePathUnitsNfd": len(path.encode("utf-16-le")) // 2, "browserPreimageBlob": captured}
    return matches


PACK = "semio-framework-pack-json"
BINARY = "semio-s-artifact-stdio-pdf-mut-family-8"
MUTATION = "semio-s-artifact-stdio-pdf-mut-base-3"
TESTKIT = "semio-s-artifact-stdio-pdf-testkit"
EDITOR = "semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests"
VIEWER = "semio-s-artifact-stdio-pdf-viewer"
BASE_EDITOR = "semio-s-artifact-stdio-pdf-schema-5"
VT_FILES = [("editor.rs", EDITOR, False), ("viewer.rs", VIEWER, False), ("tests.rs", VIEWER, True)]
TEST_START = """//! 📩️ Runs the canonical PDF mutation archive laws through their registered codec.

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
use dsl::OpBinary;
pub use semio_s_artifact_stdio_pdf_testkit::standards;
use standards::v1_7::subsets::base::{
    io::mutation_bridge::apply_pdf_mutation,
    schema::{mutations::PdfMutation, snapshot::{ObjRef, PdfIndirectObject, PdfObject, PdfSnapshot}},
};

#[path = "../🦀️.rs"]
mod aggregate_operation_contracts;

"""


def archive_rows(workspace, check):
    previous = json.loads((ARCHIVE / "receipt.json").read_bytes())
    archived = {row["stage"]: (path, row["before"]) for path, row in previous.items()}
    tests = Path(archived["6.rs"][0]).parent / "📩️archive"
    manifest = tests.parents[3] / "🧪️testkit/📦️packages/🦀️rust/Cargo.toml"
    staged = {}

    def stage(path, name, data, owner, test_only, before):
        assert digest(workspace / path) == before, path
        data = data.encode()
        output = OUT / name
        if check:
            assert output.read_bytes() == data, name
        else:
            output.write_bytes(data)
        staged[str(path)] = {"stage": name, "before": before, "after": sha256(data).hexdigest(),
            "owner": owner, "testOnly": test_only}

    def source(number):
        return (ARCHIVE / f"{number}.rs").read_text()

    for number in range(3):
        path, before = archived[f"{number}.rs"]
        stage(path, f"{number}.rs", source(number), PACK, False, before)

    pack_root = Path(archived["0.rs"][0]).parent
    receiving = Path("📥️decode/🧳️receiving")
    pack_receiver = pack_root / receiving / "🦀️.rs"
    pack_manifest = pack_root / "📦️packages/🦀️rust/Cargo.toml"
    pack_test = receiving / "🧪️tests/🔬️owner/🦀️.rs"
    pack_preimage = workspace / pack_receiver
    producer = pack_preimage.read_text()
    doc, body = producer.split("\n", 1)
    seams_at = body.index("\n#[cfg(test)]")
    seams = body[seams_at:]
    assert seams.count("#[cfg(test)]") == 3
    producer = ARCHIVE_STAGE["change"](body[:seams_at],
        "pub struct OriginalJsonReceiving", f"/// {doc.removeprefix('//! ')}\npub struct OriginalJsonReceiving")
    stage(pack_receiver, "pack-owner.rs", producer, PACK, False, digest(pack_preimage))
    pack_prelude = """//! 🧪️ Runs private Original receiving custody laws on their authored owner.

use semio_framework_pack_json::{
    ArtifactCanonicalJsonTree, ArtifactCanonicalJsonTreeCursor, JsonError, JsonGrammarCursor, JsonMemberPolicy,
};

#[path = "."]
mod original_receiving {
    include!("../../🦀️.rs");
"""
    test_seams = seams.replace('#[path="', '#[path="../../').replace("#[cfg(test)]\n", "")
    stage(pack_root / pack_test, "pack-owner-laws.rs", pack_prelude + indent(test_seams.strip(), "    ") + "\n}\n",
        PACK, True, None)
    live = workspace / pack_manifest
    entry = f'''\n[[test]]
name = "original-json-owner"
path = "../../{pack_test.as_posix()}"
'''
    manifest_text = live.read_text().rstrip()
    assert 'name = "original-json-owner"' not in manifest_text
    stage(pack_manifest, "pack-json.toml", manifest_text + "\n" + entry, PACK, False, digest(live))

    receiver = source(5)
    for number, folder, module in [(8, "🧾️metadata", "metadata"), (9, "📦️object", "object"), (10, "🌿️leaves", "leaves")]:
        receiver = ARCHIVE_STAGE["change"](receiver, f'#[path = "{folder}/🦀️.rs"]\nmod {module};',
            f"mod {module} {{\n{indent(source(number).rstrip(), '    ')}\n}}")
    binary = ARCHIVE_STAGE["change"](source(3), '#[path = "📩️receiving/🦀️.rs"]\nmod receiving;',
        f"mod receiving {{\n{indent(receiver.rstrip(), '    ')}\n}}")
    for number, data in [(3, binary), (4, source(4))]:
        path, before = archived[f"{number}.rs"]
        stage(path, f"{number}.rs", data, BINARY, False, before)

    laws = source(6)
    laws = laws[laws.index("fn image_archive_receiving_stock"):].replace(
        'include_bytes!("../../../../🧪️tests', 'include_bytes!("../../../../../🧪️tests')
    laws = ARCHIVE_STAGE["change"](laws, "schema::testkit::snapshots;", "io::import::decode_pdf;")
    laws = ARCHIVE_STAGE["change"](laws, "let (before, after) = snapshots(before_bytes, after_bytes);",
        "let before = decode_pdf(before_bytes).unwrap();\n        let after = decode_pdf(after_bytes).unwrap();")
    stage(tests / "🦀️.rs", "laws.rs", TEST_START + laws, TESTKIT, True, None)
    stage(tests / "set-image-donation.json", "laws.json", (ARCHIVE / "7.json").read_text(), TESTKIT, True, None)
    live = workspace / manifest
    entry = """\n[[test]]
name = "original_mutation_archive"
path = "../../../🧬️mutations/🧪️tests/🔬️unit/📩️archive/🦀️.rs"
"""
    text = live.read_text()
    assert 'name = "original_mutation_archive"' not in text
    stage(manifest, "testkit.toml", text.rstrip() + "\n" + entry, TESTKIT, False, digest(live))
    mirror = manifest.parent / "🪞️mirror/🦀️.rs"
    live = workspace / mirror
    section = '''                        #[cfg(test)]
                        #[path = "../../../../🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
'''
    text = ARCHIVE_STAGE["change"](live.read_text(), section, "")
    stage(mirror, "testkit-mirror.rs", text, TESTKIT, True, digest(live))
    return staged


def audit(workspace, check):
    browser_text = BROWSER_LOG.read_text(errors="replace")
    generation = re.search(r"warm generation (\S+):", browser_text)[1]
    browser_store = Path(generation) / "cs"
    assert browser_store.is_dir()
    if not check:
        OUT.mkdir(parents=True, exist_ok=True)
    staged = archive_rows(workspace, check)
    vt_dir = OUT / "vt"
    vt = VT_STAGE["stage"]
    vt.__globals__["OUT"] = vt_dir
    vt(workspace, check)
    vt_rows = json.loads((vt_dir / "receipt.json").read_bytes())
    for (relative, row), (name, owner, test_only) in zip(vt_rows.items(), VT_FILES):
        assert row["stage"] == name
        staged[relative] = {**row, "stage": f"vt/{name}", "owner": owner, "testOnly": test_only}
    base = VT_STAGE["VT"].parent / "🧱️base"
    change = ARCHIVE_STAGE["change"]
    replacements = {
        base / "✏️editor/🦀️.rs": ("base-editor.rs", BASE_EDITOR, [
            ("\n    const SINGLE_TURN_NATURAL_DECODE", VT_STAGE["EDITOR_OWNER"]["QUOTE"].rstrip() + "\n    const SINGLE_TURN_NATURAL_DECODE"),
            ("        bounded_native: true,", VT_STAGE["EDITOR_OWNER"]["CATALOG"] + "        bounded_native: true,"),
        ]),
        base / "👁️viewer/🦀️.rs": ("base-viewer.rs", EDITOR, [
            ("    fn reference_birth()", VT_STAGE["VIEWER_OWNER"] + "    fn reference_birth()"),
        ]),
    }
    for path, (name, owner, parts) in replacements.items():
        source = (workspace / path).read_bytes()
        after = source.decode()
        for original, replacement in parts:
            after = change(after, original, replacement)
        candidate = OUT / name
        data = after.encode()
        if check:
            assert candidate.read_bytes() == data
        else:
            candidate.write_bytes(data)
        staged[str(path)] = {"stage": name, "before": sha256(source).hexdigest(), "after": sha256(data).hexdigest(),
            "owner": owner, "testOnly": False}
    owners = {entry["owner"] for entry in staged.values()} | {MUTATION}
    return {
        "paths": verify(workspace, staged, browser_store),
        "nativeSourceAdmitted": show(NATIVE_LOG, owners),
        "browserSourceAdmitted": show(BROWSER_LOG, owners),
    }


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument("workspace", type=Path)
    parser.add_argument("--verify", action="store_true")
    args = parser.parse_args()
    workspace = args.workspace.resolve()
    ARCHIVE_STAGE["stage"](workspace, True)
    result = json.dumps(audit(workspace, args.verify), ensure_ascii=False, indent=2) + "\n"
    if args.verify:
        assert json.loads((OUT / "receipt.json").read_bytes())["paths"] == json.loads(result)["paths"]
    else:
        (OUT / "receipt.json").write_text(result)
    print(f"[DEBUG] {'verified' if args.verify else 'staged'} coherent {len(json.loads(result)['paths'])}-path archive install")
