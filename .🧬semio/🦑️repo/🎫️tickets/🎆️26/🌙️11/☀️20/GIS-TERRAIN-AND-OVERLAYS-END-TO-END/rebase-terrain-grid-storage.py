from difflib import unified_diff
from pathlib import Path
import re


TICKET = Path(__file__).resolve().parent
SOURCE = TICKET / "terrain-rectangular-storage.patch"
TARGET = TICKET / "terrain-rectangular-storage-current.patch"


def diff(path, before, after):
    return "".join(
        unified_diff(before.splitlines(True), after.splitlines(True), "a/" + path, "b/" + path)
    )


def proposed_hunks(block):
    return "".join(
        line[1:]
        for line in block.splitlines(True)[2:]
        if line.startswith((" ", "+"))
    )


def rebase_schema(path, block):
    before = Path(path).read_text()
    hunks = re.split(r"(?m)^@@ .*\n", block)[1:]
    first, second = (proposed_hunks(f"--- a\n+++ b\n{hunk}") for hunk in hunks)
    label = r'^\s+"x-semio-ui":.*$'
    bounds, rows = re.findall(label, before, flags=re.M)
    after = first.replace('    "bounds": {\n', '    "bounds": {\n' + bounds + "\n")
    after += before[before.index('      "type": "array",', before.index('    "bounds": {')):before.index('      "minItems": 4,')]
    after += second.replace('    "rowsMillimetres": {\n', '    "rowsMillimetres": {\n' + rows + "\n")
    return diff(path, before, after)


def rebase_snapshot(path, block):
    before = Path(path).read_text()
    start = before.index("/// 🏔️ Authored elevation")
    end = before.index("//#endregion 🔹Snapshot", start)
    new = proposed_hunks(block)
    new = new[new.index("pub const ELEVATION_MIN_SIDE"):new.index("//#endregion 🔹Snapshot")]
    native_start = before.index("use semio_framework_value_native_owned::native_decoding::{", start)
    native_end = before.index("impl FromValue for ElevationSource", native_start)
    native = before[native_start:native_end]
    native = native.replace("Self::validate_parts(value.bounds, &value.rows)", "Self::validate_parts(value.bounds, &value.rows_millimetres)")
    native = native.replace("Self { bounds: value.bounds, rows: value.rows }", "value.into_source()")
    new = new.replace("#[derive(semio_framework_value::FromValue)]", "#[doc(hidden)]\n#[derive(semio_framework_value::FromValue, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned)]")
    new = new.replace("struct ElevationSourceFields {", "pub struct ElevationSourceFields {")
    new = new.replace("impl FromValue for ElevationSource {", native + "impl FromValue for ElevationSource {")
    new = new.replace(
        "        self.is_valid().then_some(()).ok_or_else(elevation_shape_error)",
        "        Self::validate_parts(self.bounds, &self.rows_millimetres)\n"
        "    }\n\n"
        "    fn validate_parts(bounds: [f64; 4], rows: &[Vec<i32>]) -> Result<(), semio_framework_value::ValueError> {\n"
        "        Self::valid_parts(bounds, rows).then_some(()).ok_or_else(elevation_shape_error)",
    )
    new = new.replace(
        "        let columns = self.rows_millimetres.first()?.len();\n"
        "        let rows = self.rows_millimetres.len();\n"
        "        (elevation_side_in_range(columns) && elevation_side_in_range(rows)\n"
        "            && self.rows_millimetres.iter().all(|row| row.len() == columns)).then_some((columns, rows))",
        "        Self::dimensions_of(&self.rows_millimetres)\n"
        "    }\n\n"
        "    fn dimensions_of(rows: &[Vec<i32>]) -> Option<(usize, usize)> {\n"
        "        let columns = rows.first()?.len();\n"
        "        (elevation_side_in_range(columns) && elevation_side_in_range(rows.len())\n"
        "            && rows.iter().all(|row| row.len() == columns)).then_some((columns, rows.len()))",
    )
    new = new.replace(
        "        let [west, south, east, north] = self.bounds;",
        "        Self::valid_parts(self.bounds, &self.rows_millimetres)\n"
        "    }\n\n"
        "    fn valid_parts([west, south, east, north]: [f64; 4], rows: &[Vec<i32>]) -> bool {",
    ).replace("&& self.dimensions().is_some()", "&& Self::dimensions_of(rows).is_some()")
    return diff(path, before, before[:start] + new + before[end:])


def rebase_history(path, block):
    before = Path(path).read_text()
    deleted = [line[1:] for line in block.splitlines(True)[2:] if line.startswith("-")]
    added = [line[1:] for line in block.splitlines(True)[2:] if line.startswith("+")]
    after = before
    assert len(deleted) == len(added)
    for old, new in zip(deleted, added):
        assert after.count(old) == 1
        after = after.replace(old, new)
    return diff(path, before, after)


if __name__ == "__main__":
    handlers = {
        "🧬️schema/🗺️imported-map/🏔️elevation/🔣️.json": rebase_schema,
        "🧬️schema/📸️snapshot/🦀️.rs": rebase_snapshot,
        "🧬️history-edits/🟦️.test.ts": rebase_history,
    }
    rebased = []
    for block in filter(None, re.split(r"(?m)(?=^--- a/)", SOURCE.read_text())):
        path = block.splitlines()[0].removeprefix("--- a/")
        handler = next((fn for suffix, fn in handlers.items() if path.endswith(suffix)), None)
        rebased.append(handler(path, block) if handler else block)
    TARGET.write_text("".join(rebased))
