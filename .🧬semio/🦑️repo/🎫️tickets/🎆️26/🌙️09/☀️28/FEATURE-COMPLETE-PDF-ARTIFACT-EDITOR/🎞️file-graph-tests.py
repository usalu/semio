"""🔗️ Runs the authored sampled-image PDF through the exact retained file graph parser."""

from pathlib import Path
import os
import re
import sys


TICKET = Path(__file__).resolve().parent
OUTPUT = TICKET / "🗑️generated"


def exercise(pdf):
    runner = TICKET / "📜️xref-section-tests.py"
    source = "\n".join(line for line in runner.read_text().splitlines() if "mod codec;" not in line and "pub use codec::*;" not in line) + "\n"
    source = source.replace('shutil.copyfile(t/"📜️flate-run-tests.py",runner)', 'copy_native_runner(t, runner)')
    for reference in ("@CONTENT_CONTRACT@", "@PAGE_CONTRACT@"):
        source = "\n".join(line for line in source.splitlines() if not line.startswith(f'#[path = "{reference}"]')) + "\n"
    source = source.replace("testkit = (l.parents[1]", 'extra = stage_paths(base, out, extra + image_graph_regression)\ntestkit = (l.parents[1]')
    sys.argv = [str(runner), str(pdf / "🔨️modules/🔤️lexer"), str(TICKET), str(pdf / "🔨️modules/🗜️filters"),
                str(OUTPUT), str(pdf / "🔨️modules/🔗️xref")]
    engine = (pdf / "🔨️modules/🗜️filters/📦️packages/🦀️rust/🦀️.rs").as_posix()
    include = f'#[path = "{engine}"] mod filters_native;\npub use filters_native::*;\n'
    test = include + TEST.replace("@PDF@", (OUTPUT / "🎞️controlled-images.pdf").as_posix())
    test = test.replace("@PACKED@", OUTPUT.as_posix())
    test = test.replace("@TEXT@", (OUTPUT / "large-mounted-pdf.pdf").as_posix())
    assert "mod codec;" not in source
    exec(compile(source, str(runner), "exec"), {"__file__": str(runner), "image_graph_regression": test, "copy_native_runner": copy_native_runner, "stage_paths": stage_paths})


def copy_native_runner(_, runner):
    runner.write_text("""from pathlib import Path
import os
import subprocess
import sys

source = Path(sys.argv[2])
binary = source.with_name("flate-tests.exe" if os.name == "nt" else "flate-tests")
subprocess.run(["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc",
                "--edition=2021", "-Awarnings", str(source), "-o", str(binary)], check=True)
subprocess.run([str(binary)], check=True)
""")


def stage_paths(base, scratch, extra):
    filters = base / "🔨️modules/🗜️filters"
    xref = base / "🔨️modules/🔗️xref"
    mapping = [
        (filters / "🌀️bounded-flate/🧮️predictor/🦀️.rs", "predictor"),
        (filters / "🌀️bounded-flate/🦀️.rs", "flate"),
        (filters / "🪣️stream-data/🦀️.rs", "stream-data"),
        (filters / "📦️packages/🦀️rust/🦀️.rs", "filters-owner"),
        (filters / "🎚️decoded-stream/🦀️.rs", "decoded-stream"),
        (filters / "🦀️.rs", "filters"),
        (xref / "📦️normal-projection/🦀️.rs", "normal"),
        (xref / "💧️stream/📦️packages/🦀️rust/🦀️.rs", "xref-stream"),
        (xref / "📥️retained-graph/🦀️.rs", "graph"),
    ]
    raw_deflate = Path("🧰️framework/🔨️modules/🗜️deflate/🦀️.rs").resolve()
    inflater_core = raw_deflate.read_text().split('#[path = "📤️deflate-encoder/🦀️.rs"]', 1)[0]
    inflater_core = re.sub(r'#\[path = "([^"]+)"\]', lambda match: f'#[path = "{(raw_deflate.parent / match[1]).as_posix()}"]', inflater_core)
    (scratch / "raw-inflater.rs").write_text(inflater_core + "use semio_framework_value::RetirementDemand;\n")
    extra += VALUE_ABI
    font_copy = scratch / "font-codec.rs"
    font_base = base / "🔨️modules/🔤️fonts"
    font_copy.write_text(re.sub(r'#\[path = "([^"]+)"\]', lambda m: f'#[path = "{(font_base / m[1]).as_posix()}"]', font_copy.read_text()))
    replaced = {}
    for original, name in mapping:
        overlay = OUTPUT / f"🎞️{name}-history.rs"
        source = overlay if overlay.exists() and os.environ.get("PDF_CONTROLLED_LIVE") != "1" else original
        selected = scratch / f"{name}.rs"
        def abspath(match):
            target = (original.parent / match[1]).resolve()
            return f'#[path = "{replaced.get(target, target).as_posix()}"]'
        selected.write_text(re.sub(r'#\[path = "([^"]+)"\]', abspath, source.read_text()))
        replaced[original.resolve()] = selected
        extra = extra.replace(f'#[path = "{original.as_posix()}"]', f'#[path = "{selected.as_posix()}"]')
    return extra


VALUE_ABI = r"""
extern crate self as semio_framework_deflate;
extern crate self as semio_framework_value;
#[path = "raw-inflater.rs"] mod deflate_core;
pub use deflate_core::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueRefusalKind { AllocationFailed, InvalidValue, InvariantViolated, OwnershipLimit, UnsupportedOwner, WorkLimit }

#[derive(Debug)]
pub struct ValueError { pub kind: ValueRefusalKind, message: &'static str }
impl ValueError {
    pub fn literal(kind: ValueRefusalKind, message: &'static str) -> Self { Self { kind, message } }
    pub fn invariant(message: &'static str) -> Self { Self::literal(ValueRefusalKind::InvariantViolated, message) }
    pub fn into_message(self) -> String { self.message.into() }
}
impl std::fmt::Display for ValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str(self.message) }
}

#[derive(Debug, Default)]
pub struct RetirementDemand { pub release_bytes: usize, pub depth: usize }
"""

TEST = r'''

fn main() {
    use crate::controlled_graph::{PdfByteGraphEvent as E, PdfFilePreparationCursor};
    let bytes = include_bytes!("@PDF@");
    const COPY: usize = 32_768;
    const WORK_BODY: usize = 128;
    const RELEASE: usize = 1 << 20;
    for cancel in [None, Some(32_768), Some(256)] {
        let mut reader = PdfFilePreparationCursor::new(1024, 1 << 24);
        let mut work = 0;
        let mut live = 0;
        let mut history_count = 0;
        for _ in 0..350_000 {
            let empty = reader.reserving_empty_backing();
            let grant = CosGrant { work: 1, allocation_bytes: if empty { RELEASE } else { COPY - WORK_BODY }, release_bytes: RELEASE };
            let turn = reader.advance(bytes, grant, false);
            assert!(turn.work <= 1 && turn.allocated_bytes <= grant.allocation_bytes && turn.released_bytes <= grant.release_bytes);
            assert!(turn.work * WORK_BODY + if empty { 0 } else { turn.allocated_bytes } <= COPY);
            work += turn.work;
            live += turn.allocated_bytes;
            live -= turn.released_bytes;
            if turn.allocated_bytes == 32_768 {
                assert!(empty && turn.work == 1);
                history_count += 1;
                let next = reader.reserving_empty_backing();
                assert!(!next, "the first backed byte uses the live copy grant");
            }
            if history_count == 1 && cancel == Some(turn.allocated_bytes) {
                    assert!(matches!(reader.advance(bytes, grant, true).event, Ok(E::Cancelled)));
                    let mut release_bytes = 0;
                    let mut history_release = 0;
                    for _ in 0..2048 {
                        let close = reader.close_step(CosGrant { work: 1, allocation_bytes: 0, release_bytes });
                        assert!(close.work <= 1 && close.released_bytes <= release_bytes);
                        live -= close.released_bytes;
                        if close.released_bytes == 32_768 { history_release += 1; }
                        release_bytes = match close.event {
                            Ok(E::More) => 0,
                            Ok(E::NeedsRelease { bytes }) => bytes,
                            Ok(E::Closed) => break,
                            other => panic!("unexpected close: {other:?}"),
                        };
                    }
                    assert_eq!((live, history_release), (0, 1));
                    println!("[DEBUG] cancellation at allocation {:?} released the exact 32 KiB history and all retained graph bytes", cancel);
                    break;
            }
            match turn.event {
                Ok(E::More | E::Object) => {}
                Ok(E::Complete) => {
                    let file = reader.take().expect("retained native PDF graph");
                    assert_eq!(file.graph.objects.len(), 14);
                    for id in [5, 8, 11] {
                        let item = file.graph.objects.iter().find(|item| item.id.num == id).unwrap();
                        let PdfObject::Stream { dict, data, filters } = &item.value else { unreachable!(); };
                        assert_eq!(crate::lexer::dict_name(dict, "Subtype"), Some("Image"));
                        assert_eq!(data, &std::fs::read(format!("@PACKED@/{id}.samples")).unwrap());
                        assert!(if id == 5 { matches!(filters.as_slice(), [crate::model::PdfStreamFilter::Flate { .. }]) } else { filters.is_empty() });
                    }
                    println!("[DEBUG] retained PDF graph {work} work units, {live} retained bytes");
                    break;
                }
                other => {
                    reader.begin_close();
                    while !matches!(reader.close_step(CosGrant { work: 1, allocation_bytes: 0, release_bytes: RELEASE }).event, Ok(E::Closed)) {}
                    panic!("browser-sized image graph turn {work}: {other:?}");
                }
            }
        }
        assert_eq!(history_count, 1);
        assert!(work < 349_999);
    }
    text_fixture();
}

fn text_fixture() {
    use crate::controlled_graph::{PdfByteGraphEvent as E, PdfFilePreparationCursor};
    let bytes = include_bytes!("@TEXT@");
    let streams = std::fs::read_dir("@PACKED@/type1-streams-current").unwrap().map(|entry| {
        let entry = entry.unwrap();
        let owner: u32 = entry.file_name().to_str().unwrap().parse().unwrap();
        (owner, std::fs::read(entry.path()).unwrap())
    }).collect::<std::collections::BTreeMap<_, _>>();
    assert_eq!((bytes.len(), streams.len()), (33_038, 48));
    let mut reader = PdfFilePreparationCursor::new(1024, 1 << 24);
    let mut work = 0;
    let mut live = 0;
    let mut history_count = 0;
    for _ in 0..2_000_000 {
        let empty = reader.reserving_empty_backing();
        let grant = CosGrant { work: 1, allocation_bytes: if empty { 1 << 20 } else { 32_768 - 128 }, release_bytes: 1 << 20 };
        let turn = reader.advance(bytes, grant, false);
        assert!(turn.work <= 1 && turn.allocated_bytes <= grant.allocation_bytes && turn.released_bytes <= grant.release_bytes);
        assert!(turn.work * 128 + if empty { 0 } else { turn.allocated_bytes } <= 32_768);
        work += turn.work;
        live += turn.allocated_bytes;
        live -= turn.released_bytes;
        if turn.allocated_bytes == 32_768 {
            assert!(empty && turn.work == 1 && !reader.reserving_empty_backing());
            history_count += 1;
        }
        match turn.event {
            Ok(E::More | E::Object) => {}
            Ok(E::Complete) => {
                let file = reader.take().expect("retained native Type1 PDF graph");
                assert_eq!(file.graph.objects.len(), 196);
                let mut found = 0;
                for item in &file.graph.objects {
                    let Some(expected) = streams.get(&item.id.num) else { continue; };
                    let PdfObject::Stream { data, filters, .. } = &item.value else { unreachable!(); };
                    assert_eq!(expected, data, "content stream {}", item.id.num);
                    assert!(matches!(filters.as_slice(), [crate::model::PdfStreamFilter::Flate { .. }]));
                    found += 1;
                }
                assert_eq!((found, history_count), (48, 48));
                println!("[DEBUG] 48 native Flate content streams matched MuPDF, {work} work units, {live} retained bytes");
                return;
            }
            other => {
                reader.begin_close();
                while !matches!(reader.close_step(CosGrant { work: 1, allocation_bytes: 0, release_bytes: 1 << 20 }).event, Ok(E::Closed)) {}
                panic!("browser-sized Type1 graph turn {work}: {other:?}");
            }
        }
    }
    panic!("Type1 file preparation did not finish");
}
'''

if __name__ == "__main__":
    exercise(Path(sys.argv[1]).resolve())
