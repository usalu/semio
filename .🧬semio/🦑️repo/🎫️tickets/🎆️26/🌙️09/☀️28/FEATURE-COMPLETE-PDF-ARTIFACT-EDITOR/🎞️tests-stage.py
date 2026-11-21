"""🧪️ Stages PDF image regressions and moves two public lift cases to the existing testkit."""

from pathlib import Path
from textwrap import indent
import sys

LIFTER = Path("🔨️modules/⬇️lift/🧩️lifter-and-document/🦀️.rs")
TESTKIT = Path("🧬️schema/🧪️testkit/📦️packages/🦀️rust/🦀️.rs")
EDITOR = Path("✏️editor/🧪️tests/🔬️unit/🦀️.rs")
TICKET = Path(__file__).resolve().parent
OUTPUT = TICKET / "🗑️generated"


def stage(base, apply=False):
    lifter = (base / LIFTER).read_text()
    start = lifter.index("#[cfg(test)]\n#[test]\nfn document_lifting_preserves_inherited")
    tests = lifter[start:].replace("#[cfg(test)]\n", "")
    tests = tests.replace("    use crate::standards::v1_7::subsets::base::schema::graph_source::GraphSource;\n", "")
    tests = tests.replace("super::page_tree::tests::objects()", "inherited_fixture_objects()")
    tests = """fn inherited_fixture_objects() -> Vec<PdfIndirectObject> {
    let input = include_bytes!("../../../../🧪️tests/🧫️fixtures/🌳️page-tree/🧪️nested-inheritance.pdf");
    (1..=13).map(|number| {
        let header = format!("\\n{number} 0 obj\\n");
        let offset = input.windows(header.len()).position(|bytes| bytes == header.as_bytes()).unwrap() + 1;
        let (id, value) = parse_indirect_at(input, offset).unwrap();
        PdfIndirectObject { id, value }
    }).collect()
}

""" + tests
    lifter = lifter[:start].rstrip() + "\n"
    testkit = (base / TESTKIT).read_text()
    testkit = testkit.replace("lift::{lift_document, ControlledDocumentLiftCursor", "lift::{lift_document, lift_document_with, ControlledDocumentLiftCursor", 1)
    position = testkit.index("    #[test]\n    fn resource_free_document")
    testkit = testkit[:position] + indent(tests, "    ") + "\n" + testkit[position:]
    editor = (base / EDITOR).read_text()
    marker = '    let mut output = b"%PDF-1.7\\n".to_vec();'
    assert editor.count(marker) == 1
    editor = editor.replace(marker, '''    pdf_fixture_file(rows)
}

fn pdf_fixture_file(rows: Vec<Vec<u8>>) -> Vec<u8> {
    use std::io::Write;
''' + marker)
    reg = (TICKET / "🎞️editor-regression.rs").read_text()
    reg = reg.replace('spec["name"] == image.id', 'spec["name"].as_str() == Some(image.id.as_str())')
    editor += reg
    for name, path, patched in [
        ("lifter", LIFTER, lifter), ("testkit", TESTKIT, testkit), ("editor", EDITOR, editor)
    ]:
        selected = (base / path) if apply else OUTPUT / f"🎞️{name}-integration.rs"
        selected.write_text(patched)


if __name__ == "__main__":
    stage(Path(sys.argv[1]), len(sys.argv) > 2 and sys.argv[2] == "--apply")
