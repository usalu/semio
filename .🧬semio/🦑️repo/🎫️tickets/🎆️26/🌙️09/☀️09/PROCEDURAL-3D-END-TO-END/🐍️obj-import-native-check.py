"""Exercise funded OBJ and STL encoding, decoding and native topology custody against cached units."""
import sys
from pathlib import Path

ticket = Path(__file__).resolve().parent
tools = (ticket / "🐍️solid-pattern-native-check.py").read_text()
exec(tools[tools.index("import json"):tools.index("rust =")])
source_root = Path.cwd() / "🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/📦️mesh-io"
fixture = Path.cwd() / "✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🔣️original-obj-import.json"
output = ticket / "🗑️generated" / "obj-native-check"
rust = f'#[path = {quoted(source_root / "🦀️.rs")}] mod actual;\n'
rust += f'#[path = {quoted(source_root.parent / "📥️mesh-publication/🦀️.rs")}] mod publication;\n'
session_export = Path.cwd() / "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📤️mesh-export/🦀️.rs"
rust += f'#[path = {quoted(session_export)}] mod export_session;\n'
rust += f'const FIXTURE:&str=include_str!({quoted(fixture)});\n'
rust += f'const STL_FIXTURE:&str=include_str!({quoted(fixture.with_name("🔣️original-stl-import.json"))});\n'
rust += f'const EXPORT_FIXTURE:&str=include_str!({quoted(fixture.with_name("🔣️original-mesh-export.json"))});\n'
rust += r'''
use actual::{MeshDecode, ObjDecode, StlDecode, MeshImportCursor, MeshTextExport, MeshTextFormat, PrimitivePreviewJob};
use semio_framework_3d_brep_representation::topology::{Body, EntityRef};
use semio_framework_3d_brep_representation::arena::{ShellId, SolidId};
use semio_framework_3d_brep_construction::primitives::SolidPrimitive;
use semio_framework_3d_brep_tessellation::{TessellationJob, PlanarPreviewRoot};
use semio_framework_mesh_engine::MeshData;
use semio_framework_job::OriginalRetirementPump;
use semio_framework_value::AuthoringWallet;
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneGrant as Grant,
    RetainedCloneProgress as Progress, ValueError, RetirementDemand};
use semio_framework_value::{RetainedWorkPoll, RetainedCloneProgress, ValueRefusalKind, retirement::{RetireOwned, controlled::ControlledRetirement}};

#[global_allocator]
static ALLOCATOR: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

extern crate self as semio_framework_3d_brep_native_registry;
extern crate self as semio_framework_3d_brep_engine;
extern crate self as semio_framework;
fn primitive_handle(_: &[u8], _: u64) -> [u8;64] { [b'1';64] }
enum SolidProvenance { PlanarBoundary }
enum Entity { Solid(SolidId,Option<SolidProvenance>), Shell(ShellId) }
#[derive(Default)]
struct Registry {
    body: Body,
    live: semio_framework_mesh_engine::HistoryFoldIndex<String, Entity>,
    unpublished_topology: usize,
    shape_mutation_revision: u64,
    live_revision: u64,
}
impl Registry { fn advance_live_revision(&mut self) { self.live_revision += 1; } }
#[derive(Default)]
struct Brep { native: Registry, planar: Option<SolidId> }

impl Brep {
    fn original_validation_is_certified(&self, key: &str) -> Result<bool, ValueError> { Ok(key.len()==64) }
    fn preview_validation_revision(&self) -> u64 { self.native.shape_mutation_revision }
    fn certified_solid_depth(&self, _: &str) -> Result<usize,ValueError> { Ok(1) }
    fn admit_primitive_preview(&self, _: &str, tol: f64) -> Result<Option<PrimitivePreviewJob>,ValueError> {
        use semio_framework_3d_brep_representation::arena::ArenaId;
        if self.planar.is_some() {return Ok(None)}
        PrimitivePreviewJob::new(SolidId::from_raw(0,0),SolidPrimitive::axis_box(2.,3.,4.),0,tol).map(Some)
    }
    fn admit_tessellation_job(&self, _: &str, tol: f64) -> Result<(TessellationJob,bool),ValueError> {
        let id = self.planar.unwrap();
        Ok((TessellationJob::for_planar_shape(PlanarPreviewRoot::Solid(id), self.native.shape_mutation_revision, tol),false))
    }
    fn primitive_preview_demand(&self, _: &str, job: &PrimitivePreviewJob) -> Result<RetirementDemand,ValueError> {Ok(job.demand())}
    fn step_primitive_preview(&self, _: &str, job: &mut PrimitivePreviewJob, effect: &mut RetainedAllocationEffect<'_>) -> Result<(Option<MeshData>,Progress),ValueError> {job.step(effect)}
    fn analytic_preview_demand(&self, _: &str, job: &TessellationJob) -> Result<Option<RetirementDemand>,ValueError> {
        job.planar_preview_demand(&self.native.body,None).map(Some)
    }
    fn step_analytic_tessellation(&self, _: &str, job: &mut TessellationJob, grant: Grant) -> Result<(Option<MeshData>,Progress),ValueError> {
        job.step_planar_funded(&self.native.body,PlanarPreviewRoot::Solid(self.planar.unwrap()),None,self.native.shape_mutation_revision,grant)
    }
}

#[derive(Default)]
struct Session { kernel: Brep }
impl Session {
    fn with_original_geometry_query<T>(&self, run: impl FnOnce(&Brep)->Result<T,ValueError>) -> Result<T,ValueError> {run(&self.kernel)}
}

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

fn decoder(source: &str, stl: bool) -> Result<MeshDecode, ValueError> {
    if stl { StlDecode::new(source, 1e-6).map(MeshDecode::Stl) } else { ObjDecode::new(source, 1e-6).map(MeshDecode::Obj) }
}

fn decode(source: &str, stl: bool) -> Result<MeshImportCursor, ValueError> {
    let mut codec = decoder(source, stl)?;
    let result = (|| {
        for _ in 0..10000 {
            if codec.ready() { return codec.into_cursor(); }
            assert!(codec.demand().copy_bytes <= 128);
            turn(codec.demand(), false, |effect| codec.step(source, effect))?;
        }
        panic!("mesh decode stalls")
    })();
    while !codec.terminal_is_empty() { turn(codec.close_demand(), true, |effect| codec.close(effect).map(|step| step.progress()))?; }
    result
}

fn close(cursor: &mut MeshImportCursor, body: &mut Body) {
    for _ in 0..20000 {
        if cursor.cleanup_complete() { return; }
        let copy = cursor.next_close_copy_byte_demand(body).unwrap();
        let quote = RetirementDemand { depth: cursor.next_close_depth_demand(body).unwrap(), copy_bytes: copy,
            capacity_bytes: cursor.next_close_capacity_byte_demand(body,copy).unwrap(), release_bytes: cursor.next_close_release_byte_demand(body).unwrap() };
        turn(quote, true, |effect| cursor.close_step(body,effect).map(|step|step.progress())).unwrap();
    }
    panic!("mesh close stalls");
}

fn main() {
    verify_exports();
    let fixture: serde_json::Value = serde_json::from_str(FIXTURE).unwrap();
    let stl_fixture: serde_json::Value = serde_json::from_str(STL_FIXTURE).unwrap();
    for (stl, record, input) in [(false, &fixture, "obj"), (true, &stl_fixture, "base64")] {
        let authored = record["cases"][0][input].as_str().unwrap();
        let different_owner = authored.to_owned();
        let mut codec = decoder(authored, stl).unwrap();
        assert!(turn(codec.demand(), false, |effect| codec.step(&different_owner, effect)).is_err());
        for row in record["cases"].as_array().unwrap() {
        for commit in [false,true] {
            let mut body = Body::new();
            let mut cursor = decode(row[input].as_str().unwrap(), stl).unwrap();
            for _ in 0..10000 {
                if cursor.original_import_root().is_some() { break; }
                turn(cursor.original_import_demand(&body).unwrap(), false, |effect| cursor.step_original_import(&mut body,effect)).unwrap();
            }
            let root = cursor.original_import_root().expect("funded topology completed");
            assert_eq!(match root {EntityRef::Solid(_)=>"solid", EntityRef::Shell(_)=>"shell", _=>unreachable!()},row["kind"]);
            for (actual,key) in [(body.vertices.len(),"vertices"),(body.edges.len(),"edges"),(body.faces.len(),"faces")] {
                assert_eq!(actual,row[key].as_u64().unwrap() as usize);
            }
            let (area, volume) = body.faces.iter().fold((0.0,0.0), |(area,volume),(_,face)| {
                let first = body.loops.get(face.outer.unwrap()).unwrap().first;
                let second = body.coedges.get(first).unwrap().next;
                let third = body.coedges.get(second).unwrap().next;
                let [a,b,c] = [first,second,third].map(|id|body.vertices.get(body.coedge_endpoints(id).unwrap().0).unwrap().position);
                (area + 0.5*(b-a).cross(c-a).norm(),volume + a.to_vec().dot(b.to_vec().cross(c.to_vec()))/6.0)
            });
            assert!((area-row["area"].as_f64().unwrap()).abs()<1e-10);
            assert!((volume-row["volume"].as_f64().unwrap()).abs()<1e-10);
            if commit { cursor.commit_original_import(); } else { cursor.cancel(); }
            close(&mut cursor,&mut body);
            assert_eq!(body.faces.len(),if commit {row["faces"].as_u64().unwrap() as usize} else {0});
            if !commit {assert_eq!([body.vertices.len(),body.edges.len(),body.solids.len(),body.shells.len()], [0;4]);}
        }
        }
        verify_publication(authored, stl);
    }
    assert!(decode(fixture["invalid"][0]["obj"].as_str().unwrap(), false).is_err());
    for row in stl_fixture["invalid"].as_array().unwrap() {
        assert!(decode(row["base64"].as_str().unwrap(), true).is_err());
    }
    let mut body = Body::new();
    let mut broken = decode(fixture["invalid"][1]["obj"].as_str().unwrap(), false).unwrap();
    assert!((0..10000).any(|_|turn(broken.original_import_demand(&body).unwrap(), false, |effect|broken.step_original_import(&mut body,effect)).is_err()));
    close(&mut broken,&mut body);
    assert!(body.vertices.len()==0&&body.faces.len()==0);
    println!("funded OBJ and STL decode, topology, publication custody and rollback passed");
}

fn retire<T: RetireOwned>(value: T) {
    let mut owner = ControlledRetirement::new(value).ok().unwrap();
    for _ in 0..10000 {
        if owner.terminal_is_empty() { return; }
        let copy = owner.next_copy_byte_demand().unwrap();
        let quote = RetirementDemand { copy_bytes: copy, capacity_bytes: owner.next_capacity_byte_demand(copy).unwrap(),
            depth: owner.next_depth_demand().unwrap(), release_bytes: owner.next_release_byte_demand().unwrap() };
        turn(quote,true,|effect|match owner.step(effect.grant().maximum_copy_bytes,effect) {
            RetainedWorkPoll::Complete(result) => result.result.map(|step|step.progress()).map_err(|error|error.into_value_error()),
            RetainedWorkPoll::Pending{work} => {assert_eq!(work,0);Ok(Default::default())}
        }).unwrap();
    }
    panic!("mesh text retirement stalls")
}

fn encode(mesh: semio_framework_mesh_engine::MeshData, format: MeshTextFormat) -> String {
    let mut writer = MeshTextExport::new(&mut Some(mesh),format).unwrap();
    for _ in 0..10000 {
        if writer.ready() { break; }
        let demand = writer.demand();
        assert!(demand.copy_bytes<=128);
        turn(demand,false,|effect|writer.step(effect)).unwrap();
    }
    let output = writer.take_output().unwrap();
    retire(writer);
    output
}

fn verify_exports() {
    let fixture: serde_json::Value = serde_json::from_str(EXPORT_FIXTURE).unwrap();
    for (format, channel) in [(MeshTextFormat::Obj,"obj"),(MeshTextFormat::Stl,"stl")] {
        let original = &fixture["mesh"];
        let scalars = |key| original[key].as_array().unwrap().iter().map(|v|v.as_f64().unwrap() as f32).collect();
        let mesh = semio_framework_mesh_engine::MeshData {positions:scalars("positions"),normals:scalars("normals"),
            indices:original["indices"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u32).collect(),..Default::default()};
        assert_eq!(encode(mesh,format),fixture[channel].as_str().unwrap());

        for planar in [false,true] {
            let mut session = Session::default();
            if planar {
                use semio_framework_3d_brep_representation::topology::history::OpRecorder;
                let solid = semio_framework_3d_brep_construction::primitives::make_box(
                    &mut session.kernel.native.body, 2.,3.,4., &mut OpRecorder::new()).unwrap();
                session.kernel.planar = Some(solid);
            }
            let handle = "1".repeat(64);
            let other_key = handle.clone();
            let other_engine = Session::default();
            let mut job = session.plan_mesh_export(&handle,0.1,format).unwrap();
            assert!(session.mesh_export_demand(&other_key,&job).is_err());
            assert!(other_engine.mesh_export_demand(&handle,&job).is_err());
            session.kernel.native.shape_mutation_revision += 1;
            assert!(session.mesh_export_demand(&handle,&job).is_err());
            session.kernel.native.shape_mutation_revision -= 1;
            for _ in 0..10000 {
                if job.ready() { break; }
                turn(session.mesh_export_demand(&handle,&job).unwrap(),false,|effect|session.step_mesh_export(&handle,&mut job,effect)).unwrap();
            }
            let text = job.take_output().unwrap();
            retire(job);
            let mut body = Body::new();
            let mut decoded = decode(&text,channel=="stl").unwrap();
            for _ in 0..10000 {
                if decoded.original_import_root().is_some() { break; }
                turn(decoded.original_import_demand(&body).unwrap(),false,|effect|decoded.step_original_import(&mut body,effect)).unwrap();
            }
            assert!(matches!(decoded.original_import_root(),Some(EntityRef::Solid(_))));
            assert_eq!(body.faces.len(),fixture["closedBox"]["triangles"].as_u64().unwrap() as usize);
            decoded.cancel();
            close(&mut decoded,&mut body);
            assert_eq!(body.faces.len(),0);

            let body_before = session.kernel.native.body.solids.len();
            let mut cancelled = session.plan_mesh_export(&handle,0.1,format).unwrap();
            for _ in 0..10000 {
                let encoding = cancelled.phase()=="mesh-export-encoding";
                let receipt = turn(session.mesh_export_demand(&handle,&cancelled).unwrap(),false,
                    |effect|session.step_mesh_export(&handle,&mut cancelled,effect)).unwrap();
                if encoding && receipt.retained_capacity_bytes>0 {break;}
            }
            assert_eq!(cancelled.phase(),"mesh-export-encoding");
            assert!(!cancelled.ready());
            retire(cancelled);
            assert_eq!(session.kernel.native.body.solids.len(),body_before);
        }
    }
}

fn verify_publication(authored: &str, stl: bool) {
    let mut codec = decoder(authored, stl).unwrap();
    while !codec.ready() { turn(codec.demand(),false,|effect|codec.step(authored,effect)).unwrap(); }
    let mut kernel = Brep::default();
    let other = Brep::default();
    let mut mesh = kernel.plan_mesh_publication(&mut codec).unwrap();
    assert!(other.mesh_preparation_demand(&mesh).is_err());
    assert!(other.mesh_publication_close_demand(&mesh,0).is_err());
    kernel.native.shape_mutation_revision += 1;
    assert!(kernel.mesh_preparation_demand(&mesh).is_err());
    assert!(kernel.mesh_publication_close_demand(&mesh,0).is_ok());
    kernel.native.shape_mutation_revision -= 1;
    assert_eq!(kernel.native.unpublished_topology,1);
    for _ in 0..2000 {
        if mesh.handle().is_some() {break;}
        turn(kernel.mesh_preparation_demand(&mesh).unwrap(),false,|effect|kernel.step_mesh_preparation(&mut mesh,effect)).unwrap();
    }
    assert_eq!(mesh.kind(),"solid");
    let key = mesh.handle().unwrap().to_owned();
    assert!(!kernel.native.live.contains_key(&key));
    loop {
        let demand = kernel.mesh_publication_demand(&mesh,0).unwrap();
        if demand.capacity_bytes==0 {break;}
        turn(demand,false,|effect| {
            assert!(effect.performed(1)?);
            kernel.reserve_mesh_publication(&mesh,effect.grant())
        }).unwrap();
    }
    turn(kernel.mesh_publication_demand(&mesh,0).unwrap(),false,|effect| {
        assert!(effect.performed(1)?);
        kernel.publish_mesh(&mut mesh,key,effect.grant()).map_err(|(error,_)|error)
    }).unwrap();
    assert_eq!(kernel.native.unpublished_topology,0);
    assert_eq!(kernel.native.live_revision,1);
    let mut closures = 0;
    while !mesh.terminal_is_empty() {
        let copy = kernel.mesh_publication_close_demand(&mesh,0).unwrap().copy_bytes;
        turn(kernel.mesh_publication_close_demand(&mesh,copy).unwrap(),true,
            |effect|kernel.close_mesh_publication(&mut mesh,effect).map(|step|step.progress())).unwrap();
        closures += 1;
        assert!(closures<2000);
    }
    assert_eq!(kernel.native.body.solids.len(),1);
}
'''
names = [
    "semio_framework_3d_brep_construction", "semio_framework_3d_brep_curve",
    "semio_framework_3d_brep_kernel_contract", "semio_framework_3d_brep_representation",
    "semio_framework_3d_brep_surface", "semio_framework_3d_brep_tessellation",
    "semio_framework_3d_brep_tolerance", "semio_framework_3d_brep_vector",
    "semio_framework_mesh_component_references", "semio_framework_mesh_engine", "semio_framework_job", "semio_framework_trace", "semio_framework_value", "serde_json",
]
output.mkdir(parents=True, exist_ok=True)
try:
    source = output / "obj-check.rs"
    source.write_text(rust, encoding="utf-8")
    target = output / "obj-check"
    channel = run("rustup", "--", "show", "active-toolchain", capture_output=True,text=True).stdout.split()[0]
    run("rustup", "--", "run",channel,"rustc","-A","dead_code","-A","unused_imports","--edition=2024",str(source),"-o",str(target),*external_arguments(build,names))
    run("rustup","--","run",channel,str(target))
finally:
    shutil.rmtree(output)
