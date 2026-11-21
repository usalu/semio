"""🧾️ Matches the mounted regression's Rust fixture writer against the independent zlib corpus."""

from pathlib import Path
from tempfile import TemporaryDirectory
import runpy
import sys


def exercise(base, ticket, cache):
    generated = ticket / "🗑️generated"
    with TemporaryDirectory(dir=generated, prefix="image-fixture-") as directory:
        scratch = Path(directory)
        input = ticket / "🎞️editor-regression.rs"
        writer = input.read_text().split("#[semio_framework_async_macros::async_test]", 1)[0]
        file = (base / "✏️editor/🧪️tests/🔬️unit/🦀️.rs").read_text()
        xref = file.split('    let mut output = b"%PDF-1.7\\n".to_vec();', 1)[1].split("\n}\n", 1)[0]
        framed = 'fn pdf_fixture_file(rows: Vec<Vec<u8>>) -> Vec<u8> {\nuse std::io::Write;\nlet mut output = b"%PDF-1.7\\n".to_vec();' + xref + "\n}\n"
        reference = (generated / "🎞️controlled-images.pdf").as_posix()
        corpus = (ticket / "🎞️fixture.json").as_posix()
        source = scratch / "image-fixture.rs"
        source.write_text(writer + framed + f'''
#[test]
fn exact_mounted_fixture_writer_matches_mupdf_corpus() {{
    let spec: serde_json::Value = serde_json::from_str(include_str!("{corpus}")).unwrap();
    let (input, _) = sampled_fixture_bytes(&spec);
    assert_eq!(input.len(), include_bytes!("{reference}").len());
    assert_eq!(input.as_slice(), include_bytes!("{reference}"));
}}
''')
        native = runpy.run_path(str(ticket / "📜️flate-run-tests.py"))
        native["build"](scratch, cache, source, ("serde_json",))


if __name__ == "__main__":
    exercise(Path(sys.argv[1]).resolve(), Path(__file__).resolve().parent, Path(sys.argv[2]).resolve())
