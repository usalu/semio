"""📥️ Stage the bounded import command over the shared Esri row reader."""

from difflib import unified_diff
from pathlib import Path
import re


TICKET = Path(__file__).resolve().parent
APP = Path("✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor")
COMMANDS = APP / "🎮️commands"
PICKER = COMMANDS / "📂️local-file/🦀️.rs"
RASTER = COMMANDS / "📥️import-elevation/🦀️.rs"
GEOJSON = COMMANDS / "🌍️geojson/🦀️.rs"
EDITOR = APP / "🦀️.rs"
WORK = RASTER.parent / "⏩️work/🦀️.rs"
OUT = TICKET / "terrain-rectangular-import-work.patch"


def replace(source, old, new):
    assert source.count(old) == 1, old
    return source.replace(old, new)


def diff(path, before, after):
    return "".join(unified_diff(
        before.splitlines(True), after.splitlines(True),
        "a/" + path.as_posix() if before else "/dev/null", "b/" + path.as_posix(),
    ))


def staged_raster():
    patch = (TICKET / "terrain-rectangular-ascii.patch").read_text()
    selected = next(block for block in re.split(r"(?m)(?=^--- a/)", patch)
                    if block.startswith("--- a/" + RASTER.as_posix() + "\n"))
    source = RASTER.read_text()
    for hunk in re.split(r"(?m)^@@ .*\n", selected)[1:]:
        old = "".join(line[1:] for line in hunk.splitlines(True) if line.startswith((" ", "-")))
        new = "".join(line[1:] for line in hunk.splitlines(True) if line.startswith((" ", "+")))
        source = replace(source, old, new)
    return source


def picker(source):
    source = replace(source, "2 * MAX_FILE_BYTES as usize + 1024", "2 * import_elevation::MAX_FILE_BYTES as usize + 4096")
    source = replace(source, "import_action: &str) -> Effect", "import_action: &str, maximum_bytes: u32) -> Effect")
    source = replace(source, "max_file_bytes: Some(MAX_FILE_BYTES)", "max_file_bytes: Some(maximum_bytes)")
    source = replace(source, "valid_length(input: &str)", "valid_length(input: &str, maximum_bytes: u32)")
    source = replace(source, "input.len() <= MAX_FILE_BYTES as usize", "input.len() <= maximum_bytes as usize")
    source = replace(source, "receive(args: &mut Option<DslValue>)", "receive(args: &mut Option<DslValue>, maximum_bytes: u32)")
    source = replace(source, "valid_length(payload)", "valid_length(payload, maximum_bytes)")
    old = "ToolExecutionContract::bounded_first_step(MAX_WIRE_BYTES, 32, 32, 16_384, 7_500)"
    assert source.count(old) == 2
    return source.replace(old, "ToolExecutionContract::resumable(MAX_WIRE_BYTES, 32, 4096, 128 * 1024, 7_500, 64, 1)")


def raster(source):
    source = replace(source, "pub use super::local_file::MAX_FILE_BYTES;", "pub const MAX_FILE_BYTES: u32 = 64 * 1024;")
    source = replace(source, '".asc", FILE_ACTION)', '".asc", FILE_ACTION, MAX_FILE_BYTES)')
    source = replace(source, "local_file::receive(args)", "local_file::receive(args, MAX_FILE_BYTES)")
    source = replace(source, "24 KiB or smaller", "64 KiB or smaller").replace("höchstens 24 KiB", "höchstens 64 KiB")
    source = replace(source, '#[path = "📤️export/🦀️.rs"]', '#[path = "⏩️work/🦀️.rs"]\npub(crate) mod work;\n\n#[path = "📤️export/🦀️.rs"]')
    return source


def geojson(source):
    source = replace(source, 'application/geo+json", FILE_ACTION)', 'application/geo+json", FILE_ACTION, local_file::MAX_FILE_BYTES)')
    return replace(source, "local_file::receive(args)", "local_file::receive(args, local_file::MAX_FILE_BYTES)")


def editor(source):
    for suffix, maximum in [("ElevationGrid", "import_elevation::MAX_FILE_BYTES"), ("GeoJson", "local_file::MAX_FILE_BYTES")]:
        old = f"Import{suffix}File(file) => return local_file::valid_length(&file.payload)"
        source = replace(source, old, f"Import{suffix}File(file) => return local_file::valid_length(&file.payload, {maximum})")
    marker = "        if tool_id == contours::ACTION {"
    return replace(source, marker, """        if tool_id == import_elevation::FILE_ACTION {
            return semio_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, Gis3dCommand::command_id, maximum_raw_bytes, GIS3D_RETAINED_WORK_ITEMS, false, |_| import_elevation::work::Work::default());
        }
""" + marker)


changes = []
for path, transform in [(PICKER, picker), (RASTER, raster), (GEOJSON, geojson), (EDITOR, editor)]:
    before = staged_raster() if path == RASTER else path.read_text()
    changes.append(diff(path, before, transform(before)))
changes.append(diff(WORK, "", (TICKET / "terrain-rectangular-import-work.rs").read_text()))
OUT.write_text("".join(changes))
