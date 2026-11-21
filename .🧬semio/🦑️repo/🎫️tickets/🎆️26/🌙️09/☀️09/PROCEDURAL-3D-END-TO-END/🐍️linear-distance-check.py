"""📏️ Compile the original linear-distance solver against cached kernel primitives."""
import os
from pathlib import Path
import subprocess
from tempfile import TemporaryDirectory

ticket = Path(__file__).parent
root = Path("🧰️framework/🔨️modules/🧊️3d/📐️brep/")
fixture = Path("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🔣️linear-distance.json")
source = r"""
extern crate self as semio_framework_3d_brep_curve;
pub use archived_curve::Curve3;
#[path = "__WIDE__"]
pub mod bounded_polynomial;
#[path = "__DISTANCE__"]
mod distance;
use semio_framework_3d_brep_vector::{Pnt3, Vec3};
use serde_json::Value;

fn xyz(raw: &Value) -> [f64; 3] { std::array::from_fn(|axis| raw[axis].as_f64().unwrap()) }
fn operand(raw: &Value) -> (Curve3, Option<(f64, f64)>) {
    let curve = Curve3::Line { origin: Pnt3::from_array(xyz(&raw["origin"])), dir: Vec3::from_array(xyz(&raw["direction"])) };
    let trim = raw.get("range").map(|range| (range[0].as_f64().unwrap(), range[1].as_f64().unwrap()));
    (curve, trim)
}
fn main() {
    let data: Value = serde_json::from_str(include_str!("__FIXTURE__")).unwrap();
    let tolerance = data["tolerance"].as_f64().unwrap();
    for row in data["cases"].as_array().unwrap() {
        let (a, arange) = operand(&row["a"]);
        let (b, brange) = operand(&row["b"]);
        let expected = row["distance"].as_f64().unwrap();
        for (left, right) in [((&a, arange), (&b, brange)), ((&b, brange), (&a, arange))] {
            let actual = distance::original_linear_distance(left, right).unwrap_or_else(|error| panic!("{}: {error:?}", row["id"]));
            assert!((actual - expected).abs() <= tolerance * expected.max(1.0), "{}: {actual} != {expected}", row["id"]);
        }
    }
    let line = Curve3::Line { origin: Pnt3::from_array([0.0; 3]), dir: Vec3::X };
    let zero = Curve3::Line { origin: Pnt3::from_array([0.0; 3]), dir: Vec3::ZERO };
    let tiny = f64::from_bits(1);
    let small_a = Curve3::Line { origin: Pnt3::from_array([0.0; 3]), dir: Vec3::new(tiny, 0.0, 0.0) };
    let small_b = Curve3::Line { origin: Pnt3::from_array([0.0, 0.0, 3.0]), dir: Vec3::new(0.0, tiny * 2.0, 0.0) };
    assert_eq!(distance::original_linear_distance((&small_a, None), (&small_b, None)).unwrap(), 3.0);
    let circle = Curve3::Circle { frame: semio_framework_3d_brep_vector::matrix::Frame3::WORLD, radius: 2.0 };
    assert!(distance::original_linear_distance((&line, None), (&zero, None)).is_err());
    assert!(distance::original_linear_distance((&line, Some((1.0, 0.0))), (&line, None)).is_err());
    assert!(distance::original_linear_distance((&line, None), (&circle, None)).unwrap_err().to_string().contains("linear supports"));
}
"""
paths = {
    "__WIDE__": root / "📸️representation/➰️curve/📈️bounded-polynomial/🦀️.rs",
    "__DISTANCE__": root / "💡️queries/🔎️analysis/📏️linear-distance/🦀️.rs",
    "__FIXTURE__": fixture,
}
for key, path in paths.items():
    source = source.replace(key, str(path.resolve()).replace("\\", "\\\\"))
cache = tuple(Path(".cache/b/r/m/u/l/p46a1ba/prototype/build").glob("*/*/out"))
dependencies = [
    ("archived_curve", "semio_framework_3d_brep_curve"),
    *((name, name) for name in ["serde_json", "semio_framework_value", "semio_framework_3d_brep_vector"]),
]
generated = ticket / "🗑️generated"
generated.mkdir(exist_ok=True)
with TemporaryDirectory(dir=generated) as temp:
    source_path = Path(temp) / "linear.rs"
    source_path.write_text(source, encoding="utf-8")
    binary = Path(temp) / ("linear.exe" if os.name == "nt" else "linear")
    args = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "--edition=2021",
        "-A", "dead_code", "-C", "opt-level=1", "-C", "strip=symbols"]
    for directory in cache:
        args += ["-L", "dependency=" + str(directory)]
    for alias, name in dependencies:
        libraries = [library for directory in cache for library in directory.glob("lib" + name + "-*.rlib")]
        if not libraries:
            raise SystemExit("cached dependency unavailable: " + name)
        library = max(libraries, key=lambda file: file.stat().st_size)
        args += ["--extern", alias + "=" + str(library)]
        if library.with_suffix(".rmeta").is_file():
            args += ["--extern", alias + "=" + str(library.with_suffix(".rmeta"))]
    if subprocess.run(args + [str(source_path), "-o", str(binary)]).returncode:
        raise SystemExit("linear distance witness compilation failed")
    subprocess.run([str(binary)], check=True)
