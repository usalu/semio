"""🧪️ Compile the live Terrain raster fill reducer against its neutral fixture."""

import json
import os
import re
import subprocess
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
ARTIFACT = ROOT / "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any"
FILL = ARTIFACT / "✏️editor/🎮️commands/📐️edit-elevation"
FRAMEWORK = ROOT / "🧰️framework/🔨️modules/🗺️surface/🏔️terrain"


def rows(values):
    return "vec![" + ", ".join(f"vec!{row}" for row in values) + "]"


def edit(command):
    x, y, radius, amount = (command[key] for key in ("column", "row", "radius", "value"))
    return f"ElevationEdit {{ mode: ElevationEditMode::FillMissing, column: {x}, row: {y}, radius: {radius}, value: {round(amount * 1000)} }}"


def harness():
    fixture = json.loads((FILL / "🧪️tests/🌫️fill-missing/🔣️.json").read_text())
    snapshot = (ARTIFACT / "🧬️schema/📸️snapshot/🦀️.rs").read_text()
    declarations = snapshot.split("pub const ELEVATION_MIN_SIDE:", 1)[1].split("#[doc(hidden)]", 1)[0]
    declarations = re.sub(r"(?m)^\s*#\[(?:derive|canonical_json|value)\([^\n]*\)\]\n", "\n", declarations)
    declarations = declarations.replace("pub struct ElevationSource {", "#[derive(Clone, Debug)]\npub struct ElevationSource {")
    implementation = "impl ElevationSource {" + snapshot.split("impl ElevationSource {", 1)[1].split("\n//#endregion", 1)[0]
    mask = (FRAMEWORK / "🕳️missing-samples/🦀️.rs").read_text()
    reducer = (ARTIFACT / "🧬️schema/💡️inferences/📐️elevation-edit/🦀️.rs").read_text()
    original = fixture["source"]
    cases = []
    for case in fixture["cases"]:
        expected = case["expected"]
        expected_mask = f'Some("{expected["missingSamples"]}")' if "missingSamples" in expected else "None"
        cases.append(f"""
        source = {edit(case["command"])}.apply(&source).unwrap();
        assert_eq!(source.rows_millimetres, {rows(expected["rowsMillimetres"])});
        assert_eq!(source.missing_samples.as_deref(), {expected_mask});
        assert_eq!(source.bounds, initial.bounds);
        assert!(source.is_valid());
        """)
    return f"""
    #![allow(dead_code)]
    extern crate self as semio_framework_value;
    extern crate self as semio_framework_surface;

    #[derive(Debug)]
    pub struct ValueError;
    pub enum ValueRefusalKind {{ InvalidValue }}
    impl ValueError {{
        pub fn literal(_: ValueRefusalKind, _: &'static str) -> Self {{ Self }}
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
            pub const ELEVATION_MIN_SIDE:{declarations}
            {implementation}
        }}
    }}

    mod elevation_edit {{
{reducer}
    }}

    use elevation_edit::{{ElevationEdit, ElevationEditMode}};
    use schema::snapshot::ElevationSource;

    #[test]
    fn fill_missing_fixture() {{
        let initial = ElevationSource {{
            bounds: {fixture["bounds"]},
            rows_millimetres: {rows(original["rowsMillimetres"])},
            missing_samples: Some("{original["missingSamples"]}".into()),
        }};
        let mut source = initial.clone();
        {''.join(cases)}
        assert_eq!(initial.missing_samples.as_deref(), Some("{original["missingSamples"]}"));
        assert!({edit(fixture["outsideSource"])}.apply(&initial).is_err());
        assert!({edit(fixture["cases"][0]["command"])}.apply_stroke(&initial, &[[0, 0]]).is_err());
    }}
    """


def main():
    work = TICKET / "🗑️generated" / "fill"
    work.mkdir(exist_ok=True)
    source = work / "probe.rs"
    binary = work / ("test.exe" if os.name == "nt" else "test")
    source.write_text(harness(), encoding="utf-8")
    toolchain = re.search(r'(?m)^channel\s*=\s*"([^"]+)"', (ROOT / "rust-toolchain.toml").read_text())[1]
    subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", toolchain, "rustc",
                    "--edition=2024", "--test", str(source), "-o", str(binary)], cwd=ROOT, check=True)
    subprocess.run([str(binary)], cwd=ROOT, check=True)


if __name__ == "__main__":
    main()
