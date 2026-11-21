"""Verify retained STEP certification, exact accounting, and solid publication against cached native units."""
import sys
from pathlib import Path

ticket = Path(__file__).resolve().parent
tools = (ticket / "🐍️solid-pattern-native-check.py").read_text()
exec(tools[tools.index("import json"):tools.index("rust =")])
root = Path.cwd()
step = root / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📐️geometry/📥️retained-read/🦀️.rs"
tests = root / "✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit"
rows = json.loads((tests / "🔣️original-step-import.json").read_text())["cases"]
output = ticket / "🗑️generated" / "step-native-check"
rust = f'#[path = {quoted(step)}] mod actual;\n'
rust += "const CASES: &[(&str, [f64;3], [f64;3])] = &[\n"
for row in rows:
    rust += f'(include_str!({quoted((tests / row["file"]).resolve())}), {row["origin"]}, {row["size"]}),\n'
rust += "];\n"
cylinders = json.loads((tests / "🔣️original-step-cylinders.json").read_text())["cases"]
rust += "const CYLINDERS: &[(&str, [f64;3], [f64;3], [[f64;3];2], f64, f64, f64, f64)] = &[\n"
for row in cylinders:
    axes = lambda key: [float(value) for value in row[key]]
    bounds = [[float(value) for value in side] for side in row["bounds"]]
    rust += f'(include_str!({quoted((tests / row["file"]).resolve())}), {axes("base")}, {axes("axis")}, {bounds}, {float(row["radius"])}, {float(row["height"])}, {row["area"]}, {row["volume"]}),\n'
rust += "];\n"
rust += r'''
use actual::{StepSolid,StepSolidDecode};
use semio_framework_3d_brep_solid_transaction::{SolidBoundary, SolidPublication};
use semio_framework_3d_brep_representation::topology::{Body, EntityRef};
use semio_framework_job::OriginalRetirementPump;
use semio_framework_value::{AuthoringWallet, RetainedAllocationEffect, RetainedCloneGrant as Grant,
    RetainedCloneProgress as Progress, ValueError, RetirementDemand};

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

fn decode(source: &str) -> Result<StepSolidDecode, ValueError> {
    let mut parser = StepSolidDecode::new(source);
    for _ in 0..10000 {
        if parser.is_ready() { return Ok(parser); }
        if let Err(error) = turn(parser.demand(), false, |effect| parser.step(source,effect)) {
            if !parser.terminal_is_empty() { turn(parser.close_demand(), true, |effect| parser.close(effect)).unwrap(); }
            return Err(error);
        }
    }
    panic!("bounded STEP source did not finish");
}

fn key(_: u64) -> [u8; 64] { [b'0'; 64] }

fn main() {
    for &(source, origin, size) in CASES {
        let parser = decode(source).unwrap();
        assert_eq!(parser.ready(), Some(StepSolid::Box((origin,size))));
        assert!(parser.terminal_is_empty());
        let mut body = Body::new();
        let mut publication = SolidPublication::begin(1, SolidBoundary::Analytic(parser.ready().unwrap().analytic_primitive().unwrap()), &mut body.labels, key).unwrap();
        while !publication.topology_ready() { turn(publication.topology_demand(&body).unwrap(), false, |effect| publication.advance(&mut body,effect)).unwrap(); }
        let (id, _) = publication.product(1).unwrap();
        let reachable = body.reachable_from(&[EntityRef::Solid(id)]);
        assert_eq!((reachable.vertices.len(), reachable.edges.len(), reachable.faces.len()), (8,12,6));
        for id in reachable.vertices.iter() {
            let p = body.vertices.get(*id).unwrap().position.to_array();
            assert!((0..3).all(|axis| p[axis] == origin[axis] || p[axis] == origin[axis] + size[axis]));
        }
        publication.mark_published();
        while !publication.terminal_is_empty() { turn(publication.close_demand(&body).unwrap(), true, |effect| publication.close(&mut body,effect).map(|step|step.progress())).unwrap(); }

        let mut cancelled = StepSolidDecode::new(source);
        let foreign = source.to_owned();
        assert!(turn(cancelled.demand(), false, |effect| cancelled.step(&foreign,effect)).is_err());
        assert!(cancelled.terminal_is_empty());
        while cancelled.terminal_is_empty() { turn(cancelled.demand(), false, |effect| cancelled.step(source,effect)).unwrap(); }
        let quote = cancelled.close_demand();
        assert!(quote.release_bytes > 0);
        assert_eq!(turn(quote, true, |effect| cancelled.close(effect)).unwrap().released_bytes, quote.release_bytes);
        assert!(cancelled.terminal_is_empty());
    }
    for &(source,base,axis,bounds,radius,height,area,volume) in CYLINDERS {
        let parser = decode(source).unwrap();
        let proof = parser.ready().unwrap();
        let StepSolid::Cylinder { base: actual_base, axis: actual_axis, radius: actual_radius, height: actual_height, .. } = proof else { panic!("native cylinder proof") };
        assert_eq!((actual_base,actual_axis,actual_radius,actual_height),(base,axis,radius,height));
        let primitive = proof.analytic_primitive().unwrap();
        assert_eq!(primitive.axis_aligned_bounds().map(|point| point.to_array()),bounds);
        assert!((primitive.area()-area).abs()<1e-9 && (primitive.volume()-volume).abs()<1e-9);
        let mut body = Body::new();
        let mut publication = SolidPublication::begin(1,SolidBoundary::Analytic(primitive),&mut body.labels,key).unwrap();
        while !publication.topology_ready() { turn(publication.topology_demand(&body).unwrap(),false,|effect|publication.advance(&mut body,effect)).unwrap(); }
        let (id,_) = publication.product(1).unwrap();
        let topo = body.reachable_from(&[EntityRef::Solid(id)]);
        assert_eq!((topo.vertices.len(),topo.edges.len(),topo.faces.len()),(2,3,3));
        publication.mark_published();
        while !publication.terminal_is_empty() { turn(publication.close_demand(&body).unwrap(),true,|effect|publication.close(&mut body,effect).map(|step|step.progress())).unwrap(); }
        let broken = source.replacen("SEAM_CURVE(", "SURFACE_CURVE(", 1);
        assert_ne!(broken,source);
        assert!(decode(&broken).is_err());
        let altered = source.replacen("CARTESIAN_POINT('',(6.28318530718,-0.))", "CARTESIAN_POINT('',(3.14159265359,-0.))", 1);
        if altered != source { assert!(decode(&altered).is_err()); }
    }
    let changed = CASES[0].0.replacen("ORIENTED_EDGE('',*,*,#21,.F.)", "ORIENTED_EDGE('',*,*,#21,.T.)", 1);
    assert!(changed != CASES[0].0);
    assert!(decode(&changed).is_err());
    let quoted = CASES[0].0.replacen("HEADER;", "HEADER;\n/* ; a header comment */", 1);
    assert_eq!(decode(&quoted).unwrap().ready(), Some(StepSolid::Box((CASES[0].1, CASES[0].2))));
    println!("retained STEP box and cylinder certification, native topology and cancellation passed");
}
'''
names = [
    "semio_framework_3d_brep_boolean", "semio_framework_3d_brep_construction", "semio_framework_3d_brep_curve",
    "semio_framework_3d_brep_representation", "semio_framework_3d_brep_solid_transaction", "semio_framework_3d_brep_surface",
    "semio_framework_3d_brep_tolerance", "semio_framework_3d_brep_vector",
    "semio_framework_value", "semio_framework_job", "semio_framework_trace",
]
output.mkdir(parents=True, exist_ok=True)
try:
    source = output / "step-check.rs"
    source.write_text(rust, encoding="utf-8")
    target = output / "step-check"
    channel = run("rustup", "--", "show", "active-toolchain", capture_output=True, text=True).stdout.split()[0]
    run("rustup", "--", "run", channel, "rustc", "-A", "dead_code", "--edition=2024", str(source), "-o", str(target), *external_arguments(build,names))
    run("rustup", "--", "run", channel, str(target))
finally:
    shutil.rmtree(output)
