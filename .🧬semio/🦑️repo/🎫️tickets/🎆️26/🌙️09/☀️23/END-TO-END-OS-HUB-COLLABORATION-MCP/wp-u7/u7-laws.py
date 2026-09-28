#!/usr/bin/env python3
"""📚️ U7 test-only laws (rule 22): the stdio seated-example sweep (`semio-s-plugin-stdio --test example_sweep`) and the
draw demo decode law (`semio-s-artifact-draw-drawing` demo example test). Compiles against the CURRENT tree; lands only
inside a native-lane hold (`u7-probe.sh`) and is reverted there when it does not compile.

usage: python3 u7-laws.py --dry-run | --write | --revert [--part stdio-sweep|draw-demo]...
"""
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
HERE = Path(__file__).resolve().parent
STDIO_CARGO = ROOT / "✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml"
STDIO_SWEEP = ROOT / "✏️s/🔌️plugins/🗄️stdio/🧪️tests/📚️example-sweep/🦀️.rs"
STDIO_ANCHOR = '[[test]]\nname = "shipped_fleet"\npath = "../../🧪️tests/🚢️shipped-fleet/🦀️.rs"\n'
STDIO_BLOCK = '\n[[test]]\nname = "example_sweep"\npath = "../../🧪️tests/📚️example-sweep/🦀️.rs"\nrequired-features = ["full-artifact-catalog"]\n'
DRAW_TEST = ROOT / "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"
DRAW_OLD = '''#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}
'''
DRAW_NEW = '''/// 📚️ LAW: the demo `setActiveExample` seats decodes into a non-empty drawing — a stale asset is named here, never
/// opened as an empty canvas.
#[semio_framework_async_macros::async_test]
async fn demo_decodes_into_a_non_empty_drawing() {
    let demo = <crate::DrawingSnapshot as store::ArtifactDsl>::parse_dsl(super::PRIMARY_TEXT).unwrap_or_else(|error| panic!("the draw demo does not decode: {error}"));
    assert!(demo != crate::DrawingSnapshot::default(), "the draw demo decodes to the genesis drawing");
}
'''


def stdio_sweep(mode):
    source = (HERE / "payload/laws/stdio-example-sweep.rs").read_text()
    cargo = STDIO_CARGO.read_text()
    present = STDIO_SWEEP.exists()
    wired = STDIO_BLOCK in cargo
    if mode == "dry-run":
        if not present and not wired and cargo.count(STDIO_ANCHOR) != 1:
            return "PROBLEM stdio Cargo anchor (shipped_fleet test block) not found exactly once"
        return f"stdio-sweep: file {'present' if present else 'absent'}, Cargo {'wired' if wired else 'unwired'}"
    if mode == "write":
        if not present:
            STDIO_SWEEP.parent.mkdir(parents=True, exist_ok=True)
            STDIO_SWEEP.write_text(source)
        if not wired:
            if cargo.count(STDIO_ANCHOR) != 1:
                raise SystemExit("stdio Cargo anchor missing")
            STDIO_CARGO.write_text(cargo.replace(STDIO_ANCHOR, STDIO_ANCHOR + STDIO_BLOCK, 1))
        return "stdio-sweep written"
    if wired:
        STDIO_CARGO.write_text(cargo.replace(STDIO_BLOCK, "", 1))
    if present:
        STDIO_SWEEP.unlink()
        try:
            STDIO_SWEEP.parent.rmdir()
        except OSError:
            pass
    return "stdio-sweep reverted"


def draw_demo(mode):
    text = DRAW_TEST.read_text()
    old, new = text.count(DRAW_OLD), text.count(DRAW_NEW)
    if mode == "dry-run":
        if old + new != 1:
            return f"PROBLEM draw demo test: old={old} new={new}"
        return f"draw-demo: {'applied' if new else 'pending'}"
    if mode == "write":
        if old == 1:
            DRAW_TEST.write_text(text.replace(DRAW_OLD, DRAW_NEW, 1))
        return "draw-demo written"
    if new == 1:
        DRAW_TEST.write_text(text.replace(DRAW_NEW, DRAW_OLD, 1))
    return "draw-demo reverted"


PARTS = {"stdio-sweep": stdio_sweep, "draw-demo": draw_demo}


def main():
    args = sys.argv[1:]
    mode = next((arg[2:] for arg in args if arg in ("--dry-run", "--write", "--revert")), None)
    if mode is None:
        raise SystemExit(__doc__)
    parts = [args[index + 1] for index, arg in enumerate(args) if arg == "--part"] or list(PARTS)
    problems = 0
    for part in parts:
        line = PARTS[part](mode)
        problems += line.startswith("PROBLEM")
        print(line)
    raise SystemExit(1 if problems else 0)


if __name__ == "__main__":
    main()
