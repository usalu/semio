""" 🧪 Verify the staged elevation edit against the shared NoData fixture."""

import json
import re
import subprocess
import tempfile
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
CANONICAL = TICKET / "terrain-nodata-canonical-exchange.patch"
EDITING = TICKET / "terrain-nodata-editing.patch"
HUNK = re.compile(r"@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@")


def staged(patch, suffix):
    text = patch.read_text(encoding="utf-8")
    sections = re.split(r"(?m)(?=^--- (?:a/|/dev/null))", text)
    file, = [section.splitlines(keepends=True) for section in sections if section
             and section.splitlines()[1].endswith(suffix)]
    source = [] if file[0].startswith("--- /dev/null") else (
        ROOT / file[0].removeprefix("--- a/").strip()
    ).read_text(encoding="utf-8").splitlines(keepends=True)
    starts = [i for i, row in enumerate(file) if HUNK.match(row)]
    out = []
    offset = 0
    for first, end in zip(starts, starts[1:] + [len(file)]):
        start, remove, _, add = HUNK.match(file[first]).groups()
        at = int(start) - (remove != "0")
        old = [line[1:] for line in file[first + 1:end] if line[0] in " -"]
        new = [line[1:] for line in file[first + 1:end] if line[0] in " +"]
        assert len(old) == int(remove or "1") and len(new) == int(add or "1"), file[first]
        assert at >= offset and source[at:at + len(old)] == old, (suffix, file[first])
        out.extend(source[offset:at])
        out.extend(new)
        offset = at + len(old)
    return "".join(out + source[offset:])


def rows(samples):
    return "vec![" + ", ".join(f"vec!{row}" for row in samples) + "]"


def test_case(case):
    name = case["operation"].replace("-", "_")
    edit_mode = name.title().replace("_", "")
    column, row, radius, value = (case[key] for key in ("column", "row", "radius", "value"))
    apply = f"let edit = ElevationEdit {{ mode: ElevationEditMode::{edit_mode}, column: {column}, row: {row}, radius: {radius}, value: {value} }};"
    if "error" in case:
        check = f'assert_eq!(edit.apply(&source).unwrap_err().0, {json.dumps(case["error"])});'
    else:
        check = f"""
        let next = edit.apply(&source).unwrap();
        assert_eq!(next.rows_millimetres, {rows(case["rowsMillimetres"])});
        assert_eq!(next.bounds, source.bounds);
        assert_eq!(next.missing_samples, source.missing_samples);
        assert!(next.is_valid());
        """
        if name in {"raise_lower", "smooth"}:
            check += f"""
            let anchor = [[{column}, {row}]];
            assert_eq!(edit.apply_stroke(&source, &anchor).unwrap(), next);
            assert_eq!(edit.apply_stroke_with(&source, &anchor, || false).unwrap_err().0, "gis-terrain-elevation-stroke-cancelled");
            """
    return f"""
    #[test]
    fn {name}() {{
        let source = terrain();
        {apply}
        {check}
        assert_eq!(source, terrain());
    }}
    """


def harness():
    fixture = json.loads((TICKET / "terrain-nodata-editing.json").read_text(encoding="utf-8"))
    edit = staged(EDITING, "📐️elevation-edit/🦀️.rs")
    mask = staged(CANONICAL, "🕳️missing-samples/🦀️.rs")
    snapshot = staged(CANONICAL, "🧬️schema/📸️snapshot/🦀️.rs")
    model = snapshot[snapshot.index("pub const ELEVATION_MIN_SIDE:"):snapshot.index("#[doc(hidden)]")]
    model = re.sub(r"(?m)^\s*#\[(?:derive|canonical_json|value)\([^\n]*\)\]\n", "\n", model)
    model = model.replace("pub struct ElevationSource {", "#[derive(Clone, Debug)]\npub struct ElevationSource {")
    impl = snapshot[snapshot.index("impl ElevationSource {"):].split("\n//#endregion", 1)[0]
    bounds = [float(value) for value in fixture["bounds"]]
    tests = "".join(test_case(case) for case in fixture["tests"])
    return f"""
    #![allow(dead_code)]
    extern crate self as semio_framework_value;
    extern crate self as semio_framework_surface;

    #[derive(Debug)]
    pub struct ValueError(&'static str);
    pub enum ValueRefusalKind {{ InvalidValue }}

    impl ValueError {{
        pub fn literal(_: ValueRefusalKind, message: &'static str) -> Self {{ Self(message) }}
    }}

    pub mod terrain {{
        pub mod missing_samples {{
{mask}
        }}
        pub struct TerrainElevationGrid {{
            pub bounds: [f64; 4],
            pub rows_millimetres: Vec<Vec<i32>>,
            pub missing_samples: Option<String>,
        }}
    }}

    mod schema {{
        pub mod snapshot {{
            use crate::terrain::missing_samples::{{self, MAX_MASK_HEX_BYTES}};
{model}
{impl}
        }}
    }}

    mod elevation_edit {{
{edit}
    }}

    use elevation_edit::{{ElevationEdit, ElevationEditMode}};
    use schema::snapshot::ElevationSource;

    fn terrain() -> ElevationSource {{
        ElevationSource {{
            bounds: {bounds},
            rows_millimetres: {rows(fixture["rowsMillimetres"])},
            missing_samples: Some({json.dumps(fixture["missingSamples"])}.into()),
        }}
    }}

    {tests}
    """


def main():
    workspace = TICKET / "🗑️generated"
    workspace.mkdir(exist_ok=True)
    toolchain = re.search(r'(?m)^channel\s*=\s*"([^"]+)"', (ROOT / "rust-toolchain.toml").read_text(encoding="utf-8"))[1]
    with tempfile.TemporaryDirectory(prefix="nodata-edit-", dir=workspace) as work:
        source = Path(work) / "probe.rs"
        runner = source.with_name("probe.exe")
        source.write_text(harness(), encoding="utf-8")
        subprocess.run(["repo", "tools", "exec", "rustup", "--", "run", toolchain, "rustc",
                        "--edition=2024", "--test", str(source), "-o", str(runner)], cwd=ROOT, check=True)
        subprocess.run([str(runner)], check=True)


if __name__ == "__main__":
    main()
