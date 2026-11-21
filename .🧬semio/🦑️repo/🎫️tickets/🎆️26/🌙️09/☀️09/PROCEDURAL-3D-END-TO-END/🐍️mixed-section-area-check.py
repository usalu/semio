"""📐️ Exercise the production segment area equations in a compact native binary."""
import json
import os
from pathlib import Path
import subprocess
from tempfile import TemporaryDirectory

ticket = Path(__file__).parent
fixtures = Path("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/")
area = Path("🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/🔎️analysis/🎯️planar-closest/🪢️arc/📐️area/🦀️.rs").resolve()
rows = json.loads((fixtures / "🔣️mixed-section-distance.json").read_text())["sections"]
planar = json.loads((fixtures / "🔣️planar-face-projection.json").read_text())

laws = r"""
#[path = "__AREA__"]
mod arc_area;
use std::f64::consts::PI;

fn assert_near(value: f64, expected: f64) {
    assert!((value-expected).abs() <= 1e-10 * expected.abs().max(1.0), "{value} != {expected}");
}
fn chord_ring(vertices: &[[f64; 2]]) -> f64 {
    vertices.iter().zip(vertices.iter().cycle().skip(1)).map(|(a, b)| a[0] * b[1] - a[1] * b[0]).sum()
}
fn elliptic_section(intervals: &[(f64, f64)], expected: f64) {
    let radii = [2.0 * 2.0_f64.sqrt(), 2.0];
    let mut boundary = vec![];
    let mut twice = 0.0;
    for &(lo, hi) in intervals {
        twice += arc_area::elliptic(radii, hi-lo);
        for t in [lo, hi] { boundary.push([radii[0] * t.cos(), radii[1] * t.sin()]); }
    }
    assert_near((chord_ring(&boundary) + twice).abs() / 2.0, expected);
}
fn cone_section(slope: f64, cap: f64, middle: f64, weight: f64, expected: f64) {
    let half_width = (16.0 - cap*cap).sqrt();
    let determinant = (middle-cap) * slope.hypot(1.0) * (2.0 * half_width);
    assert_near((determinant * arc_area::rational(weight)).abs() / 2.0, expected);
}
fn compare_rational_weights() {
    for weight in [0.05_f64, 0.25, 0.6, 0.999999999999, 1.0, 1.000000000001, 2.0, 8.0] {
        let count = 65_536;
        let value = |t: f64| {
            let denominator = 1.0 + 2.0*(weight-1.0)*t*(1.0-t);
            2.0*weight*t*t / (denominator*denominator)
        };
        let mut total = value(0.0) + value(1.0);
        for i in 1..count {
            let coefficient = if i % 2 == 0 { 2.0 } else { 4.0 };
            total += coefficient * value(f64::from(i) / f64::from(count));
        }
        assert_near(arc_area::rational(weight), total / (3.0*f64::from(count)));
    }
    let tiny = 1e-12;
    let exact_lead = tiny * tiny * tiny;
    let actual = arc_area::elliptic([3.0, 2.0], tiny);
    assert!((actual / exact_lead - 1.0).abs() < 1e-15);
    assert_eq!(arc_area::elliptic([3.0, 2.0], -tiny), -actual);
}
fn main() {
    compare_rational_weights();
    __FIXTURES__
}
"""
statements = []
for row in rows:
    if row["kind"] == "cylinder":
        intervals = "[(-2.0*PI/3.0,2.0*PI/3.0)]" if row["height"] == 4 else "[(PI/3.0,2.0*PI/3.0),(4.0*PI/3.0,5.0*PI/3.0)]"
        statements.append(f'elliptic_section(&{intervals}, {row["area"]});')
    else:
        source = "2.0,1.5,-4.0,1.0" if row["normal"][0] == 2 else "3.0,1.0,-2.0,2.0"
        statements.append(f'cone_section({source}, {row["area"]});')
def loop(points):
    return "&[" + ",".join(f"[{float(x)},{float(y)}]" for x, y, _ in points) + "]"
rings = [planar["outer"], *planar["holes"]]
terms = " - ".join(f"chord_ring({loop(points)}).abs()" for points in rings)
statements.append(f'assert_near(0.5 * 2.0_f64.sqrt() * ({terms}), {planar["area"]});')

generated = ticket / "🗑️generated"
generated.mkdir(exist_ok=True)
with TemporaryDirectory(dir=generated) as temp:
    source = Path(temp) / "area.rs"
    source.write_text(laws.replace("__AREA__", str(area).replace("\\", "\\\\")).replace("__FIXTURES__", "\n    ".join(statements)), encoding="utf-8")
    binary = Path(temp) / ("area.exe" if os.name == "nt" else "area")
    result = subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "--edition=2021", "-C", "opt-level=1", "-C", "strip=symbols", str(source), "-o", str(binary)])
    if result.returncode:
        raise SystemExit("segment area witness compilation failed")
    subprocess.run([str(binary)], check=True)


brep = Path("🧰️framework/🔨️modules/🧊️3d/📐️brep/").resolve()
analysis = brep / "💡️queries/🔎️analysis"
curve = brep / "📸️representation/➰️curve"
fixture_builder = (ticket / "🐍️line-conic-source-check.py").read_text(encoding="utf-8")
mixed = fixture_builder.split("fn verify_mixed_sections() {", 1)[1].split('        for query in row["queries"]', 1)[0]
holes = fixture_builder.split("fn verify_face_distances() {", 1)[1].split("    assert!(planar_projection::closest_face_depth", 1)[0]
integration = r"""
extern crate self as semio_framework_3d_brep_curve;
pub use archived_curve::{bspline, curve_ops, Curve2, Curve3};
#[path = "__POLYNOMIAL__"]
pub mod bounded_polynomial;
#[path = "__SPLINE__"]
pub mod bounded_spline;
#[path = "__LINEAR_DISTANCE__"]
mod linear_distance;
#[path = "__PLANAR_CLOSEST__"]
mod planar_closest;
#[path = "__NEAREST__"]
mod nearest_analytic;
#[path = "__MEASURE__"]
mod planar_volume;
use planar_closest::planar_dual_coordinates;
use semio_framework_3d_brep_representation::{arena::FaceId, topology::{Body, Edge, Face, Vertex, make_loop}};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_vector::{Pnt2, Pnt3, Vec2, Vec3, matrix::Frame3};
use serde_json::Value;
use bspline::KnotVector;

fn xyz(raw: &Value) -> [f64; 3] { std::array::from_fn(|axis| raw[axis].as_f64().unwrap()) }
fn measure(body: &Body, face: FaceId, expected: f64) {
    let mut cursor = planar_volume::PlanarMeasure::new(planar_volume::PlanarMeasureSource::Face(face), true);
    let actual = (0..64).find_map(|_| {
        let demand = cursor.next_demand(body).unwrap();
        assert!(demand.copy_bytes <= 128 && demand.depth > 0);
        cursor.step(body).unwrap()
    }).unwrap();
    assert!((actual - expected).abs() < 1e-9, "{actual} != {expected}");
}

fn clipped() {
    __MIXED_SETUP__
        measure(&body, face, row["area"].as_f64().unwrap());
    }
}

fn polygon_with_holes() {
    __HOLES_SETUP__
    measure(&body, face, data["area"].as_f64().unwrap());
    for row in data["cases"].as_array().unwrap() {
        let target = Pnt3::from_array(xyz(&row["target"]));
        let mut vertex = planar_closest::PlanarClosest::from_vertex(face, target);
        let planar_closest::PlanarClosestOutput::Distance(measured) = (0..64)
            .find_map(|_| vertex.advance(&body).unwrap()).unwrap() else { panic!("expected a scalar"); };
        assert!((measured - row["distance"].as_f64().unwrap()).abs() < 1e-10);
        let mut point = planar_closest::PlanarClosest::new(face, target);
        let measured = (0..64).find_map(|_| point.step(&body).unwrap()).unwrap();
        assert!(measured.distance(Pnt3::from_array(xyz(&row["point"]))) < 1e-10);
    }
    for row in data["linearDistance"].as_array().unwrap() {
        let curve = Curve3::Line { origin: Pnt3::from_array(xyz(&row["origin"])), dir: Vec3::from_array(xyz(&row["direction"])) };
        let trim = row.get("range").map(|range| (range[0].as_f64().unwrap(), range[1].as_f64().unwrap()));
        let mut cursor = planar_closest::PlanarClosest::from_line(face, (&curve, trim)).unwrap();
        let planar_closest::PlanarClosestOutput::Distance(measured) = (0..64).find_map(|_| {
            let demand = cursor.next_demand(&body).unwrap();
            assert!(demand.copy_bytes <= 128 && demand.depth > 0);
            cursor.advance(&body).unwrap()
        }).unwrap() else { panic!("expected a scalar"); };
        let expected = row["distance"].as_f64().unwrap();
        assert!((expected - measured).abs() < 1e-10, "{}: {measured} != {expected}", row["id"]);
    }
    let ring = body.faces.get(face).unwrap().outer.unwrap();
    let first = body.loops.get(ring).unwrap().first;
    let edge = body.coedges.get(first).unwrap().edge;
    body.edges.get_mut(edge).unwrap().range.0 += 0.25;
    let curve = Curve3::Line { origin: Pnt3::from_array([14.0, 20.0, 34.0]), dir: Vec3::X };
    let mut cursor = planar_closest::PlanarClosest::from_line(face, (&curve, None)).unwrap();
    assert!(cursor.advance(&body).unwrap().is_none());
    assert!(cursor.advance(&body).unwrap().is_none());
    assert!(cursor.advance(&body).is_err());

}

fn solid_regression() {
    use semio_framework_3d_brep_construction::primitives::make_box;
    use semio_framework_3d_brep_representation::topology::history::OpRecorder;
    let mut body = Body::new();
    let solid = make_box(&mut body, 2.0, 3.0, 4.0, &mut OpRecorder::new()).unwrap();
    for (area, expected) in [(true, 52.0), (false, 24.0)] {
        let mut cursor = planar_volume::PlanarMeasure::new(planar_volume::PlanarMeasureSource::Solid(solid), area);
        let actual = (0..100).find_map(|_| {
            cursor.next_demand(&body).unwrap();
            cursor.step(&body).unwrap()
        }).unwrap();
        assert!((actual - expected).abs() < 1e-10, "{actual} != {expected}");
    }
}

fn main() {
    clipped();
    polygon_with_holes();
    solid_regression();
}
"""
paths = {
    "__POLYNOMIAL__": curve / "📈️bounded-polynomial/🦀️.rs",
    "__SPLINE__": curve / "🧵️bounded-spline/🦀️.rs",
    "__LINEAR_DISTANCE__": analysis / "📏️linear-distance/🦀️.rs",
    "__PLANAR_CLOSEST__": analysis / "🎯️planar-closest/🦀️.rs",
    "__NEAREST__": analysis / "🎯️nearest-analytic/🦀️.rs",
    "__MEASURE__": analysis / "📦️planar-volume/🦀️.rs",
    "__MIXED_SECTION__": fixtures / "🔣️mixed-section-distance.json",
    "__PLANAR_FACE__": fixtures / "🔣️planar-face-projection.json",
}
integration = integration.replace("__MIXED_SETUP__", mixed).replace("__HOLES_SETUP__", holes)
for key, target in paths.items():
    integration = integration.replace(key, str(target.resolve()).replace("\\", "\\\\"))

cache = tuple(Path(".cache/b/r/m/u/l/p46a1ba/prototype/build").glob("*/*/out"))
dependencies = [
    ("archived_curve", "semio_framework_3d_brep_curve"),
    ("serde_json", "serde_json"),
    *((name, name) for name in ["semio_framework_value", "semio_framework_3d_brep_vector",
        "semio_framework_3d_brep_representation", "semio_framework_3d_brep_surface", "semio_framework_3d_brep_construction"]),
]
with TemporaryDirectory(dir=generated) as temp:
    source = Path(temp) / "cursor.rs"
    source.write_text(integration, encoding="utf-8")
    binary = Path(temp) / ("cursor.exe" if os.name == "nt" else "cursor")
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
    if subprocess.run(args + [str(source), "-o", str(binary)]).returncode:
        raise SystemExit("planar area cursor witness compilation failed")
    subprocess.run([str(binary)], check=True)
