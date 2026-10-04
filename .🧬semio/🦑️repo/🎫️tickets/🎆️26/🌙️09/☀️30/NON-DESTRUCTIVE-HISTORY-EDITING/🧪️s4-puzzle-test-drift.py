#!/usr/bin/env python3
"""🧯️ S4-PUZZLE: repairs the peer drift in the puzzle 2d/5d TEST files (`🧪️tests/**`) the lib sweeps did not reach:
`protocol::{Locale, Terminology}` (no longer re-exported) → `semio_framework_ui_locale::…`; `ViewModel` lost `Default`, so the
context-menu laws pass the host's own default axes (`ViewModel::new(En, Native)`); `PngSnapshot` keeps only the encoded bytes,
so raster geometry is read from its IHDR (`png_layout`); `store::json` → `semio_framework_pack_json`; the sqlite owners refuse with
`ValueError`. Idempotent; `--check` reports."""
import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts")
TREES = ("◻️2d", "🖐️5d", "🧊️3d")
VIEW = "semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)"
EXACT = {
    "◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️unit/🦀️.rs": [
        ('let png = semio_s_artifact_stdio_png::io::decode_png(&png_out::serialize_bytes(&board()).expect("png")).expect("decodes as png");\n    assert_eq!((png.width, png.height), (240 + 64, 120 + 64));',
         'let png = semio_s_artifact_stdio_png::io::png_layout(&semio_s_artifact_stdio_png::io::decode_png(&png_out::serialize_bytes(&board()).expect("png")).expect("decodes as png")).expect("png header");\n    assert_eq!((png.width, png.height), (240 + 64, 120 + 64));'),
    ],
    "🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️unit/🦀️.rs": [
        ('let png = semio_s_artifact_stdio_png::io::decode_png(&png_out::serialize_bytes(&assembly()).expect("png")).expect("decodes as png");',
         'let png = semio_s_artifact_stdio_png::io::png_layout(&semio_s_artifact_stdio_png::io::decode_png(&png_out::serialize_bytes(&assembly()).expect("png")).expect("decodes as png")).expect("png header");'),
    ],
    "◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs": [
        ("store::json::from_json_str(&laws[\"snapshot\"].to_string()).unwrap()",
         "semio_framework_pack_json::from_json_str(&laws[\"snapshot\"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()"),
        ("Puzzle2dSnapshot::from_sqlite_database(&d,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()))).is_err()",
         "Puzzle2dSnapshot::from_sqlite_database(&d,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).map_err(|e|e.to_string())).is_err()"),
    ],
    "🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs": [
        ("fn restore(d:&SqliteDatabase)->Result<Puzzle5dSnapshot,String>{", "fn restore(d:&SqliteDatabase)->Result<Puzzle5dSnapshot,semio_framework_value::ValueError>{"),
    ],
}


def convert(relative: str, text: str) -> str:
    text = text.replace("protocol::Locale", "semio_framework_ui_locale::Locale").replace("protocol::Terminology", "semio_framework_ui_locale::Terminology")
    text = text.replace("app.context_menu(&request, &Default::default())", f"app.context_menu(&request, &{VIEW})")
    for old, new in EXACT.get(relative, []):
        if new in text:
            continue
        if text.count(old) != 1:
            sys.exit(f"anchor count {text.count(old)} in {relative}: {old[:80]!r}")
        text = text.replace(old, new)
    return text


def main() -> None:
    check = "--check" in sys.argv
    changed = 0
    for tree in TREES:
        for path in sorted((ROOT / tree).rglob("🦀️.rs")):
            if "🧪️tests" not in path.parts:
                continue
            relative = str(path.relative_to(ROOT))
            text = path.read_text(encoding="utf-8")
            written = convert(relative, text)
            if written != text:
                changed += 1
                print(f"{'pending' if check else 'converted'} {relative}")
                if not check:
                    path.write_text(written, encoding="utf-8")
    print(f"{changed} file(s) {'pending' if check else 'converted'}")
    sys.exit(1 if check and changed else 0)


if __name__ == "__main__":
    main()
