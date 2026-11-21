"""🌗️ Exercise current spherical certificates and authored face charts without rebuilding package archives."""
import os
from pathlib import Path
import subprocess
from tempfile import TemporaryDirectory

ticket = Path(__file__).parent
base = Path("🧰️framework/🔨️modules/🧊️3d/📐️brep/").resolve()
analysis = base / "💡️queries/🔎️analysis"
curve = base / "📸️representation/➰️curve"
contract = base / "📸️representation/🔖️contract"
fixture = Path("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🔣️spherical-measurement.json")
rust = r"""
extern crate self as semio_framework_3d_brep_curve;
pub use archived_curve::{bspline, curve_ops, Curve2, Curve3};
#[path = "__POLYNOMIAL__"]
pub mod bounded_polynomial;
#[path = "__SPLINE__"]
pub mod bounded_spline;
#[path = "__LINEAR_DISTANCE__"]
mod linear_distance;
#[path = "__CLOSEST__"]
mod planar_closest;
#[path = "__NEAREST__"]
mod nearest_analytic;
#[path = "__PLANAR_VOLUME__"]
mod planar_volume;
#[path = "__CAPS__"]
mod spherical_caps;
#[path = "__SHELL__"]
mod spherical_shell;
use planar_closest::{PlanarClosest, PlanarClosestOutput, planar_dual_coordinates};
use planar_volume::{PlanarMeasure, PlanarMeasureSource};
use semio_framework_3d_brep_representation::{arena::FaceId, topology::{Body, Edge, Face, Vertex, make_loop, history::OpRecorder}};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_vector::{Pnt2, Pnt3, Vec2, matrix::Frame3};
use serde_json::Value;
use spherical_caps::SphericalCaps;
use spherical_shell::SphericalShell;
use std::f64::consts::TAU;

fn point(value: &Value) -> Pnt3 {
    Pnt3::from_array(std::array::from_fn(|axis| value[axis].as_f64().unwrap()))
}
fn measure(body: &Body, face: FaceId) -> f64 {
    let mut cursor = PlanarMeasure::new(PlanarMeasureSource::Face(face), true);
    (0..16).find_map(|_| {
        let demand = cursor.next_demand(body).unwrap();
        assert!(demand.copy_bytes <= 128 && demand.depth > 0);
        cursor.step(body).unwrap()
    }).unwrap()
}

fn nearest(body: &Body, face: FaceId, row: &Value, chart: usize, shift: Pnt3, tolerance: f64) {
    let Some(probes) = row["nearest"].as_array() else { return; };
    let offset = shift - Pnt3::new(0.0, 0.0, 0.0);
    for probe in probes.iter().filter(|probe| probe["face"].as_u64() == Some(chart as u64)) {
        let target = point(&probe["target"]) - offset;
        let turn = |mut cursor: PlanarClosest| (0..8).find_map(|_| {
            let demand = cursor.next_demand(body).unwrap();
            assert!(demand.depth > 0 && demand.copy_bytes <= 128);
            cursor.advance(body).unwrap()
        }).unwrap();
        let PlanarClosestOutput::Point(actual) = turn(PlanarClosest::new(face, target)) else { panic!(); };
        if probe["point"].is_array() {
            assert!((actual + offset).distance(point(&probe["point"])) < tolerance, "{}: {actual:?}", row["id"]);
        }
        assert!((actual.distance(target) - probe["distance"].as_f64().unwrap()).abs() < tolerance);
        let PlanarClosestOutput::Distance(actual) = turn(PlanarClosest::from_vertex(face, target)) else { panic!(); };
        assert!((actual - probe["distance"].as_f64().unwrap()).abs() < tolerance, "{}: {actual}", row["id"]);
    }
}

fn cap_face_body(caps: SphericalCaps) -> (Body, [FaceId; 2]) {
    let mut body = Body::new();
    let vertices: [_; 3] = std::array::from_fn(|index| {
        let label = body.new_label();
        body.vertices.insert(Vertex { position: caps.vertex(index), label, tol: Default::default() })
    });
    let edges: [_; 5] = std::array::from_fn(|index| {
        let (curve, range, [a, b]) = caps.edge(index);
        let curve = body.curves3.insert(curve);
        let label = body.new_label();
        body.edges.insert(Edge { curve, range, v0: vertices[a], v1: vertices[b], label, tol: Default::default() })
    });
    let faces = std::array::from_fn(|index| {
        let surface = body.surfaces.insert(Surface::Sphere { frame: caps.frame(index), radius: caps.radius(index) });
        let label = body.new_label();
        let face = body.faces.insert(Face { surface, outer: None, inners: vec![], flipped: caps.is_inward(index), tol: Default::default(), label });
        let (a, b, low_pole, _) = caps.band(index);
        let seam = 1 + 2 * index;
        let pole = seam + 1;
        let [lower, upper] = if low_pole { [pole, 0] } else { [0, pole] };
        let coordinates = [(0.0, a, 1.0, 0.0, (0.0, TAU)), (TAU, 0.0, 0.0, 1.0, (a, b)),
            (0.0, b, 1.0, 0.0, (0.0, TAU)), (0.0, 0.0, 0.0, 1.0, (a, b))];
        let ring = make_loop(&mut body, face, &[(edges[lower], true), (edges[seam], true), (edges[upper], false), (edges[seam], false)]);
        body.faces.get_mut(face).unwrap().outer = Some(ring);
        let mut current = body.loops.get(ring).unwrap().first;
        for (u, v, du, dv, range) in coordinates {
            let pcurve = body.curves2.insert(Curve2::Line { origin: Pnt2::new(u, v), dir: Vec2::new(du, dv) });
            let row = body.coedges.get_mut(current).unwrap();
            row.pcurve = Some(pcurve);
            row.prange = range;
            current = row.next;
        }
        face
    });
    (body, faces)
}

fn main() {
    use semio_framework_3d_brep_construction::primitives::make_sphere;
    let data: Value = serde_json::from_str(include_str!("__FIXTURE__")).unwrap();
    let tolerance = data["relativeTolerance"].as_f64().unwrap();
    for row in data["cases"].as_array().unwrap() {
        let [r, s]: [f64; 2] = serde_json::from_value(row["radii"].clone()).unwrap();
        let [a, b] = [point(&row["centers"][0]), point(&row["centers"][1])];
        let kind = row["kind"].as_str().unwrap();
        let (area, volume, center, face_areas) = if kind == "shell" || (b-a).norm() < r-s {
            let frames = [a,b].map(|origin| Frame3 { origin, ..Frame3::WORLD });
            let shell = SphericalShell::from_spheres(frames, [r, s], 1e-7).unwrap();
            let areas = std::array::from_fn(|i| {
                let radius = [r,s][i];
                let mut body = Body::new();
                let _ = make_sphere(&mut body, radius, &mut OpRecorder::new()).unwrap();
                let face = body.faces.ids().next().unwrap();
                nearest(&body, face, row, i, [a,b][i], tolerance);
                measure(&body, face)
            });
            (shell.area(), shell.volume(), shell.center_of_mass(), areas)
        } else {
            let (positive, inward) = match kind {
                "intersect" => ([true,false], false),
                "fuse" => ([false,true], false),
                "cut" => ([false,false], true),
                _ => unreachable!(),
            };
            let caps = SphericalCaps::new(a, b, [r, s], positive, inward, 1e-7).unwrap();
            let (mut body, faces) = cap_face_body(caps);
            let measured = faces.map(|face| measure(&body, face));
            for (chart, face) in faces.into_iter().enumerate() {
                nearest(&body, face, row, chart, Pnt3::new(0.0, 0.0, 0.0), tolerance);
            }
            let loop_id = body.faces.get(faces[0]).unwrap().outer.unwrap();
            let coedge = body.loops.get(loop_id).unwrap().first;
            body.coedges.get_mut(coedge).unwrap().prange.1 *= 0.5;
            let mut invalid_chart = PlanarMeasure::new(PlanarMeasureSource::Face(faces[0]), true);
            assert!(invalid_chart.step(&body).unwrap().is_none());
            assert!(invalid_chart.step(&body).is_err());
            let mut invalid_projection = PlanarClosest::new(faces[0], a);
            assert!(invalid_projection.step(&body).unwrap().is_none());
            assert!(invalid_projection.step(&body).unwrap().is_none());
            assert!(invalid_projection.step(&body).is_err());
            (caps.area(), caps.volume(), caps.center_of_mass(), measured)
        };
        let check = |actual: f64, expected: &Value| {
            let expected = expected.as_f64().unwrap();
            assert!((actual - expected).abs() < tolerance * expected.abs().max(1.0), "{}: {actual} != {expected}", row["id"]);
        };
        check(area, &row["area"]);
        check(volume, &row["volume"]);
        for (i, measured) in face_areas.into_iter().enumerate() { check(measured, &row["faces"][i]); }
        assert!(center.distance(point(&row["center"])) < tolerance, "{}: {center:?}", row["id"]);
    }
}
"""
paths = {
    "__POLYNOMIAL__": curve / "📈️bounded-polynomial/🦀️.rs",
    "__SPLINE__": curve / "🧵️bounded-spline/🦀️.rs",
    "__LINEAR_DISTANCE__": analysis / "📏️linear-distance/🦀️.rs",
    "__CLOSEST__": analysis / "🎯️planar-closest/🦀️.rs",
    "__NEAREST__": analysis / "🎯️nearest-analytic/🦀️.rs",
    "__PLANAR_VOLUME__": analysis / "📦️planar-volume/🦀️.rs",
    "__CAPS__": contract / "🌗️spherical-caps/🦀️.rs",
    "__SHELL__": contract / "🔵️spherical-shell/🦀️.rs",
    "__FIXTURE__": fixture,
}
for substitute, path in paths.items():
    rust = rust.replace(substitute, str(path.resolve()).replace("\\", "\\\\"))
cache = tuple(Path(".cache/b/r/m/u/l/p46a1ba/prototype/build").glob("*/*/out"))
dependencies = [
    ("archived_curve", "semio_framework_3d_brep_curve"),
    *((name, name) for name in ["serde_json", "semio_framework_value", "semio_framework_3d_brep_vector",
        "semio_framework_3d_brep_representation", "semio_framework_3d_brep_surface", "semio_framework_3d_brep_construction"]),
]
generated = ticket / "🗑️generated"
generated.mkdir(exist_ok=True)
with TemporaryDirectory(dir=generated) as temp:
    source = Path(temp) / "spherical.rs"
    source.write_text(rust, encoding="utf-8")
    binary = Path(temp) / ("spherical.exe" if os.name == "nt" else "spherical")
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
        raise SystemExit("spherical measurement witness compilation failed")
    subprocess.run([str(binary)], check=True)
