"""Run the production STEP writer and an optional temporary independently pinned Open CASCADE reader."""
import base64
import hashlib
import io
import sys
import tarfile
import urllib.request
from pathlib import Path

ticket = Path(__file__).resolve().parent
tools = (ticket / "🐍️solid-pattern-native-check.py").read_text()
exec(tools[tools.index("import json"):tools.index("rust =")])
root = Path.cwd()
step = root / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📐️geometry"
tests = root / "✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit"
fixture = tests / "🔣️original-step-export.json"
output = ticket / "🗑️generated" / "step-export-native-check"
rust = f'#[path = {quoted(step / "📝️retained-write/🦀️.rs")}] mod actual;\n'
rust += f'#[path = {quoted(step / "📥️retained-read/🦀️.rs")}] mod import;\n'
orthogonal = root / "🧰️framework/🔨️modules/🧊️3d/📐️brep/🛠️operations/🔀️boolean/🧱️certified-boundaries/🧱️orthogonal-boundary/🦀️.rs"
rust += f'#[path = {quoted(orthogonal)}] mod orthogonal;\n'
rust += f'const FIXTURE: &str = include_str!({quoted(fixture)});\n'
cylinder_fixture = tests / "🔣️original-step-cylinder-export.json"
rust += f'const CYLINDER_FIXTURE: &str = include_str!({quoted(cylinder_fixture)});\n' 
rust += r'''
use actual::{StepWrite,StepWriteSource};
use semio_framework_3d_brep_solid_transaction::{SolidBoundary, SolidPublication};
use semio_framework_3d_brep_representation::topology::Body;
use semio_framework_3d_brep_construction::{prism_boundary::{MAX_PRISM_CORNERS, PrismBoundary},primitives::SolidPrimitive};
use semio_framework_3d_brep_vector::{Pnt3,Vec3,matrix::Frame3};
use semio_framework_job::OriginalRetirementPump;
use semio_framework_value::{AuthoringWallet, RetainedAllocationEffect, RetainedCloneGrant as Grant,
    RetainedCloneProgress as Progress, ValueError, RetirementDemand};
use serde_json::Value;
pub use semio_framework_3d_brep_boolean::BooleanOp;
use semio_framework_3d_brep_boolean::certified_boundaries::{empty_result, unsupported};


#[global_allocator]
static ALLOCATOR: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

fn turn(need: RetirementDemand, closing: bool, op: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError>) -> Result<Progress, ValueError> {
    let grant = Grant { maximum_items: 1, maximum_capacity_bytes: need.capacity_bytes, maximum_copy_bytes: need.copy_bytes,
        maximum_release_bytes: need.release_bytes, maximum_depth: need.depth.max(1) };
    let observe = |effect: &mut RetainedAllocationEffect<'_>| {
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| op(effect));
        let receipt = match &result { Ok(value) => *value, Err(error) => error.retained_progress() };
        assert!(receipt.fits(grant) && effect.performed_work() <= 1);
        assert_eq!((heap.requested_bytes, heap.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes));
        result
    };
    if closing {
        OriginalRetirementPump::new().turn(1 << 16, 1 << 20, || Some(1), grant)
            .and_then(|mut turn| turn.with_effect(observe)).unwrap().unwrap().0
    } else {
        AuthoringWallet::new(grant,1_000_000_000).run(grant,1<<20,observe).unwrap().unwrap().output
    }
}
fn key(_: u64) -> [u8;64] { [b'2';64] }
fn xyz(row: &Value) -> [f64;3] { std::array::from_fn(|index| row[index].as_f64().unwrap()) }
fn check_readback(step: &str, row: &Value) {
    let mut parser = import::StepSolidDecode::new(step);
    while !parser.is_ready() { turn(parser.demand(), false, |effect| parser.step(step,effect)).unwrap(); }
    let proof = parser.ready().unwrap();
    let analytic_bounds = match proof.analytic_primitive() {
        Some(SolidPrimitive::Cylinder {frame,radius,height}) => {
            assert_eq!(frame.origin.to_array(),xyz(&row["base"]));
            assert_eq!(frame.z.to_array(),xyz(&row["axis"]));
            assert_eq!(frame.x.to_array(),xyz(&row["radial"]));
            assert_eq!((radius,height),(row["radius"].as_f64().unwrap(),row["height"].as_f64().unwrap()));
            Some(proof.analytic_primitive().unwrap().axis_aligned_bounds().map(|point|point.to_array()))
        }
        _ => None,
    };
    let solid = if analytic_bounds.is_some() {
        SolidBoundary::Analytic(proof.analytic_primitive().unwrap())
    } else if row["kind"] == "box" {
        assert_eq!(proof, import::StepSolid::Box((xyz(&row["origin"]), xyz(&row["size"]))));
        SolidBoundary::Analytic(proof.analytic_primitive().unwrap())
    } else if let import::StepSolid::Orthogonal { axes, cells } = proof {
        assert_eq!(cells, 0b1011);
        assert_eq!((&axes[0][..3],&axes[1][..3],&axes[2][..2]), (&[0.,2.,4.][..],&[0.,2.,4.][..],&[0.,2.][..]));
        let rebuilt = orthogonal::OrthogonalBoundary::from_closed_cells(axes,cells).unwrap();
        assert_eq!(rebuilt.ring_sizes().len(),row["faces"].as_u64().unwrap() as usize);
        assert_eq!(rebuilt.edge_count(),row["edges"].as_u64().unwrap() as usize);
        assert_eq!(rebuilt.vertex_count(),row["vertices"].as_u64().unwrap() as usize);
        let bounds = |key: &str| [xyz(&row[key][0]),xyz(&row[key][1])];
        let expected = semio_framework_3d_brep_boolean::certified_boundaries::OrthogonalBoundary::new(
            bounds("a"), bounds("b"), BooleanOp::Unite,
        ).unwrap();
        assert!((0..rebuilt.vertex_count()).all(|i| rebuilt.vertex(i) == expected.vertex(i)));
        assert!((0..rebuilt.edge_count()).all(|i| rebuilt.edge(i).vertices == expected.edge(i).vertices));
        assert!((0..rebuilt.ring_sizes().len()).all(|i| rebuilt.surface(i) == expected.surface(i)));
        assert!(orthogonal::OrthogonalBoundary::from_closed_cells(axes, cells | (1<<26)).is_err());
        SolidBoundary::Orthogonal(expected)
    } else {
        SolidBoundary::Prism(proof.prism_boundary().unwrap())
    };
    let mut body = Body::new();
    let mut publication = SolidPublication::begin(1, solid, &mut body.labels, key).unwrap();
    while !publication.topology_ready() { turn(publication.topology_demand(&body).unwrap(), false, |effect| publication.advance(&mut body,effect)).unwrap(); }
    let (id, _) = publication.product(1).unwrap();
    let topo = body.reachable_from(&[semio_framework_3d_brep_representation::topology::EntityRef::Solid(id)]);
    for (actual, field) in [(topo.vertices.len(),"vertices"),(topo.edges.len(),"edges"),(topo.faces.len(),"faces")] {
        assert_eq!(actual, row[field].as_u64().unwrap() as usize);
    }
    let mut bound = [[f64::INFINITY;3],[f64::NEG_INFINITY;3]];
    for id in topo.vertices.iter() {
        let at = body.vertices.get(*id).unwrap().position.to_array();
        for i in 0..3 { bound[0][i]=bound[0][i].min(at[i]); bound[1][i]=bound[1][i].max(at[i]); }
    }
    assert_eq!(analytic_bounds.unwrap_or(bound),[xyz(&row["bounds"][0]),xyz(&row["bounds"][1])]);
    publication.mark_published();
    while !publication.terminal_is_empty() { turn(publication.close_demand(&body).unwrap(), true, |effect| publication.close(&mut body,effect).map(|step|step.progress())).unwrap(); }
}
fn produce(row: &Value, target: &str) {
    let kind = row["kind"].as_str().unwrap();
    let recipe = if kind.starts_with("cylinder") {
        let frame = Frame3::from_x_z(
            Pnt3::from_array(xyz(&row["base"])),Vec3::from_array(xyz(&row["radial"])),Vec3::from_array(xyz(&row["axis"])),
        ).unwrap();
        SolidBoundary::Analytic(SolidPrimitive::Cylinder { frame, radius:row["radius"].as_f64().unwrap(), height:row["height"].as_f64().unwrap() })
    } else if kind == "orthogonal" {
        use semio_framework_3d_brep_boolean::{BooleanOp,certified_boundaries::OrthogonalBoundary};
        let bounds = |key: &str| [xyz(&row[key][0]),xyz(&row[key][1])];
        SolidBoundary::Orthogonal(OrthogonalBoundary::new(bounds("a"),bounds("b"),BooleanOp::Unite).unwrap())
    } else if kind == "prism" {
        let mut ring = [Pnt3::from_array([0.;3]);MAX_PRISM_CORNERS];
        for (slot,point) in ring.iter_mut().zip(row["ring"].as_array().unwrap()) { *slot = Pnt3::from_array(xyz(point)); }
        SolidBoundary::Prism(PrismBoundary::from_planar_ring(ring, 3, Vec3::from_array(xyz(&row["vector"])), Vec3::Z).unwrap())
    } else {
        let [width,depth,height] = xyz(&row["size"]);
        SolidBoundary::Analytic(SolidPrimitive::Box { origin: Pnt3::from_array(xyz(&row["origin"])), width,depth,height })
    };
    let mut body = Body::new();
    let mut publication = SolidPublication::begin(1, recipe, &mut body.labels, key).unwrap();
    while !publication.topology_ready() { turn(publication.topology_demand(&body).unwrap(), false, |effect| publication.advance(&mut body,effect)).unwrap(); }
    let (solid, _) = publication.product(1).unwrap();
    publication.mark_published();
    while !publication.terminal_is_empty() { turn(publication.close_demand(&body).unwrap(), true, |effect| publication.close(&mut body,effect).map(|step|step.progress())).unwrap(); }
    let handle = String::from_utf8(key(2).into()).unwrap();
    let source = StepWriteSource { body:&body,solid,revision:1,handle:&handle,depth:1 };
    let mut writer = StepWrite::new(source);
    let changed = handle.clone();
    assert!(writer.demand(StepWriteSource{handle:&changed,..source}).is_err());
    assert!(writer.demand(StepWriteSource{revision:2,..source}).is_err());
    let mut steps = 0;
    while !writer.ready() {
        let quote = writer.demand(source).unwrap();
        if quote.copy_bytes > 0 {
            let progress = turn(RetirementDemand {copy_bytes:quote.copy_bytes-1,..quote},false,|effect|writer.step(source,effect)).unwrap();
            assert_eq!(progress,Progress::default());
        }
        turn(quote,false,|effect|writer.step(source,effect)).unwrap();
        steps += 1;
        assert!(steps < 10000);
    }
    let output = writer.take_output(source).unwrap();
    assert_eq!(writer.output_capacity(),0);
    for (entity, key) in [("VERTEX_POINT","vertices"),("EDGE_CURVE","edges"),("ADVANCED_FACE","faces")] {
        assert_eq!(output.lines().filter(|line|line.contains(&format!(" = {entity}("))).count(),row[key].as_u64().unwrap() as usize);
    }
    if kind.starts_with("cylinder") {
        assert_eq!(output.lines().filter(|line|line.contains(" = SEAM_CURVE(")).count(),1);
        assert_eq!(output.lines().filter(|line|line.contains(" = SURFACE_CURVE(")).count(),2);
        assert_eq!(output.lines().filter(|line|line.contains(" = PCURVE(")).count(),6);
        if row["reimport"] == true { check_readback(&output,row); }
    } else {
        check_readback(&output,row);
    }
    if kind != "box" && !kind.starts_with("cylinder") {
        let offset = output.find(" = ADVANCED_FACE(").unwrap();
        let end = offset + output[offset..].find(';').unwrap();
        let start = offset + output[offset..end].rfind('.').unwrap()-2;
        let mut changed = output.clone();
        let reversed = if &output[start..start+3] == ".T." {".F."} else {".T."};
        changed.replace_range(start..start+3,reversed);
        let mut broken = import::StepSolidDecode::new(&changed);
        let rejected = (0..10000).find_map(|_| {
            assert!(!broken.is_ready());
            turn(broken.demand(),false,|effect|broken.step(&changed,effect)).err()
        });
        assert!(rejected.is_some());
        turn(broken.close_demand(),true,|effect|broken.close(effect)).unwrap();
        assert!(broken.terminal_is_empty());
    }
    std::fs::write(format!("{target}/{kind}.step"),output).unwrap();

    let mut cancelled = Some(Box::new(StepWrite::new(source)));
    let size = std::mem::size_of::<StepWrite>();
    while cancelled.as_ref().unwrap().output_capacity() == 0 {
        turn(cancelled.as_ref().unwrap().demand(source).unwrap(),false,|effect|cancelled.as_mut().unwrap().step(source,effect)).unwrap();
    }
    let bytes = cancelled.as_ref().unwrap().output_capacity()+size;
    turn(RetirementDemand {release_bytes:bytes,depth:1,..Default::default()}, true, |effect| {
        assert!(effect.performed(1)?);
        drop(cancelled.take());
        Ok(Progress{copied_items:1,released_bytes:bytes,..Default::default()})
    }).unwrap();
    assert!(cancelled.is_none());
}
fn main() {
    let fixture:Value = serde_json::from_str(FIXTURE).unwrap();
    let target = std::env::args().nth(1).unwrap();
    let cylinders:Value = serde_json::from_str(CYLINDER_FIXTURE).unwrap();
    for row in fixture["cases"].as_array().unwrap().iter().chain(cylinders["cases"].as_array().unwrap()) {produce(row,&target);}
    println!("retained STEP export native cursor, topology, exact STEP reimport and cancellation passed");
}
'''
names = [
    "semio_framework_3d_brep_boolean", "semio_framework_3d_brep_construction", "semio_framework_3d_brep_curve",
    "semio_framework_3d_brep_representation", "semio_framework_3d_brep_solid_transaction", "semio_framework_3d_brep_surface",
    "semio_framework_3d_brep_tolerance", "semio_framework_3d_brep_vector",
    "semio_framework_value", "semio_framework_job", "semio_framework_trace", "serde_json",
]
output.mkdir(parents=True, exist_ok=True)
try:
    source = output / "step-export-check.rs"
    source.write_text(rust, encoding="utf-8")
    target = output / "step-export-check"
    channel = run("rustup", "--", "show", "active-toolchain", capture_output=True, text=True).stdout.split()[0]
    run("rustup", "--", "run", channel, "rustc", "-A", "dead_code", "-A", "unused_imports", "--edition=2024", str(source), "-o", str(target), *external_arguments(build,names))
    run("rustup", "--", "run", channel, str(target), str(output))
    if "--occt" in sys.argv:
        with urllib.request.urlopen("https://registry.npmjs.org/occt-import-js/-/occt-import-js-0.0.23.tgz", timeout=20) as response:
            data = response.read(12*1024*1024)
        digest = base64.b64encode(hashlib.sha512(data).digest()).decode("ascii")
        assert digest == "RFfYQXYFX5C1mB1Aywm0ShcUKzXOr/VzTnlzhBSDJOR6YCAPt1HYCzeXWg1vwwjn/cUxwqRNhhtf1dlewoZYCQ=="
        with tarfile.open(fileobj=io.BytesIO(data), mode="r:gz") as files:
            for name in ["occt-import-js.js", "occt-import-js.wasm"]:
                (output / name.replace(".js", ".cjs")).write_bytes(files.extractfile("package/dist/"+name).read())
        run("node", "--", str(ticket / "🟨️step-export-occt.mjs"), str(output), str(fixture))
        run("node", "--", str(ticket / "🟨️step-cylinder-occt.mjs"), str(output), str(tests / "🔣️original-step-cylinders.json"))
        run("node", "--", str(ticket / "🟨️step-cylinder-occt.mjs"), str(output), str(cylinder_fixture))
finally:
    shutil.rmtree(output)
