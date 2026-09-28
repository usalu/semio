"""🗂️ S20 window-3 prepared patch: process3d resolves its export formats from the stdio codec crates it links.

`export_process3d_model` looked every format up in the process-global format catalog (`semio_framework::format_descriptor`),
which only holds rows that plugins of the SAME guest registered at assembly. The process guest registers none (only stdio
artifacts call `.formats(…)`), so every `exportModel` was refused `unknown process export format kind step/obj/stl/glb`
(io-matrix 27 19:5x, `demonstrator/process3d`); its unit test registers the stdio rows by hand, which hid it. The exporter
now reads the row from the codec crate that owns the format (`semio_s_artifact_stdio_{step,obj,stl,gltf}::formats()`, all
already dependencies of the crate).

Usage: python3 s20-patch-process-formats.py [--dry-run]   (idempotent)
"""
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
IO = ROOT / "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"
DRY = "--dry-run" in sys.argv

HUNKS = [
    (
        "glb lookup",
        '        let descriptor = semio_framework::format_descriptor("glb").map_err(|error| error.to_string())?.ok_or_else(|| "unknown process export format kind `glb`".to_string())?;\n',
        '        let descriptor = process_export_format("glb")?;\n',
    ),
    (
        "solid lookup",
        '    let descriptor = semio_framework::format_descriptor(format_kind).map_err(|error| error.to_string())?.ok_or_else(|| format!("unknown process export format kind `{format_kind}`"))?;\n',
        "    let descriptor = process_export_format(format_kind)?;\n",
    ),
    (
        "resolver",
        "/// 📤️ Encodes the replayed stock through `format`'s codec.",
        '''/// 🗂️ The format row of one process export, read from the stdio codec crate that owns the format — never from the
/// process-global format catalog, which only holds rows the plugins of the SAME guest registered (this guest registers
/// none, so every export used to be refused as an unknown format kind).
fn process_export_format(format_kind: &str) -> Result<semio_framework_plugin::io::FormatDescriptor, String> {
    for rows in [semio_s_artifact_stdio_step::formats(), semio_s_artifact_stdio_obj::formats(), semio_s_artifact_stdio_stl::formats(), semio_s_artifact_stdio_gltf::formats()] {
        let rows = rows.map_err(|error| format!("process export format rows: {error:?}"))?;
        if let Some(row) = rows.into_iter().find(|row| row.kind_id == format_kind || row.short_id == format_kind || row.aliases.iter().any(|alias| alias == format_kind)) {
            return Ok(row);
        }
    }
    Err(format!("unknown process export format kind `{format_kind}`"))
}

/// 📤️ Encodes the replayed stock through `format`'s codec.''',
    ),
]


def main() -> None:
    text = IO.read_text()
    notes = []
    for name, old, new in HUNKS:
        if text.count(old) == 1 and new not in text:
            text = text.replace(old, new)
            notes.append(f"apply     {name}")
        elif new in text:
            notes.append(f"applied   {name}")
        else:
            notes.append(f"CONFLICT  {name}: anchor found {text.count(old)}×")
    print("\n".join(notes))
    if any(line.startswith("CONFLICT") for line in notes):
        raise SystemExit("conflict: nothing written")
    if not DRY:
        IO.write_text(text)
    print("dry-run" if DRY else "applied")


if __name__ == "__main__":
    main()
