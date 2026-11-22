from pathlib import Path
from tempfile import TemporaryDirectory
import json, os, re, subprocess, sys
ticket=Path(__file__).resolve().parent
lanes=Path(".cache/b/r/m/u/l")
wasm=len(sys.argv)>2 and sys.argv[2]=="wasm32-wasip2"
build_path=Path("wasm32-wasip2/prototype/build" if wasm else "prototype/build")
match=next(lanes.glob(f"*/{build_path}/semio-framework-3d-brep-native-registry/*/out/*.rlib"),None)
if match is None and len(sys.argv)==1:
    sys.exit("No managed lane has compiled the BRep dependencies")
c=lanes/sys.argv[1]/build_path if len(sys.argv)>1 else match.parents[3]
engine=next(Path(".").glob("*framework/*modules/*3d/*brep/*engine"))
registry=next(engine.glob("*native-registry"))
def metadata(file):
    record=file.with_suffix(".rmeta.semio-deps")
    return json.loads(record.read_text()) if record.is_file() else {}

anchor=max(c.glob("semio-framework-3d-brep-native-registry/*/out/*.rlib"),key=lambda f:f.stat().st_mtime,default=None)
if anchor is None:
    sys.exit("No compiled BRep registry artifact in "+str(c))
lineage=dict(metadata(anchor)["dependencies"])
sources=[next(engine.glob("*solid-topology")),next(registry.glob("*solid-transaction")),registry]

def dependency_names(manifest):
    body=manifest.read_text().split("[dependencies]",1)[1].split("\n[",1)[0]
    return [key.replace("-","_") for key in re.findall(r"(?m)^([\w-]+)\.workspace",body)]

manifests=[next(source.glob("*packages/*rust/Cargo.toml")).resolve() for source in sources]
required={key for manifest in manifests for key in dependency_names(manifest)}
rebuilt={re.search(r'(?m)^name = "([^"]+)"',manifest.read_text())[1].replace("-","_") for manifest in manifests}
required.update(["serde_json","semio_framework_3d_brep_queries"])
pool={}
for lane in sorted(lanes.iterdir(),key=lambda path: c != path/build_path):
    for build in {lane/build_path,lane/"prototype/build"}:
        for file in build.glob("*/*/out/lib*"):
            key=file.stem.removeprefix("lib")
            if key not in lineage or key in pool:
                continue
            if (file.suffix==".rlib" and build==lane/build_path and metadata(file).get("svh")==lineage[key]) or file.suffix in (".dylib",".dll",".so"):
                pool[key]=file
artifacts={}
for key in required-rebuilt:
    match=next((file for label,file in pool.items() if label.rsplit("-",1)[0]==key),None)
    if match is None:
        sys.exit("Managed lane is missing exact compiler lineage for "+key)
    artifacts[key]=match
directories=sorted({file.parent for file in pool.values()})
compiler=["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20","rustc","--edition=2021","-A","warnings","-C","opt-level=1"]

if wasm:
    compiler+=["--target","wasm32-wasip2"]

def compile_source(source,dest,aliases,options,manifest_dir):
    args=compiler+options+["-o",str(dest),str(source)]
    args.extend(f"-Ldependency={d}" for d in [dest.parent,*directories])
    for key in aliases:
        f=artifacts[key]
        args.extend(f"--extern={key}={artifact}" for artifact in (f,f.with_suffix(".rmeta")) if artifact.exists())
    result=subprocess.run(args,env={**os.environ,"CARGO_MANIFEST_DIR":str(manifest_dir)})
    if result.returncode:
        sys.exit(result.returncode)

with TemporaryDirectory(dir=ticket/"🗑️generated") as d:
    d=Path(d)
    for source,manifest in zip(sources,manifests):
        name=re.search(r'(?m)^name = "([^"]+)"',manifest.read_text())[1].replace("-","_")
        out=d/f"lib{name}.rlib"
        print("[DEBUG] Rebuilding "+name,flush=True)
        compile_source(next(source.glob("*.rs")),out,dependency_names(manifest),["--crate-type=rlib","--crate-name",name],manifest.parent)
        artifacts[name]=out
    funded=(ticket/"🔬️linked-section-region.rs").read_text().split("struct Work;",1)[1].split("fn offset_box",1)[0]
    funded=("struct Work;"+funded).replace("maximum_copy_bytes: 128","maximum_copy_bytes: 256")
    fixture=next(Path(".").glob("*s/*plugins/*flow/*extensions/*brep/*tests/*unit/*original-straight-sweeps.json")).resolve()
    source=d/"guided.rs"
    source.write_text("""use semio_framework_value::{
    RetainedAllocationEffect, RetainedCloneGrant as Grant, ValueError,
    retained_clone::{OriginalSourceTurnIdentity, OriginalWork, OriginalWorkAuthority, OriginalWorkPartialCheck,
        OriginalWorkPartialCheckComplete, OriginalWorkQuote, OriginalWorkQuoteComplete, OriginalWorkRecipient},
};
"""+funded+"use semio_framework_3d_brep_native_registry::{BrepRegistry, Entity, PlanarExtrusion, PolygonWireRecipe};\nuse semio_framework_3d_brep_representation::topology::EntityRef;\nuse semio_framework_3d_brep_queries::mass_properties::solid_volume;\n\nfn wire(registry: &mut BrepRegistry, recipe: PolygonWireRecipe, points: &[[f64; 3]]) -> String {\n    let mut draft = registry.plan_polygon_wire(recipe).unwrap();\n    for _ in 0..512 {\n        if draft.topology_ready() { break; }\n        let point = draft.next_point_index().map(|i| points[i]);\n        funded(grant(), |effect| registry.step_polygon_wire_topology(&mut draft, point, effect)).unwrap();\n    }\n    assert!(draft.topology_ready());\n    while registry.polygon_wire_demand(&draft, 0).unwrap().capacity_bytes != 0 {\n        registry.reserve_polygon_wire(&draft, grant()).unwrap();\n    }\n    let key = draft.handle().to_owned();\n    registry.publish_polygon_wire(&mut draft, key.clone(), grant()).unwrap();\n    key\n}\n\nfn main() {\n    let law: serde_json::Value = serde_json::from_str(include_str!(FIXTURE)).unwrap();\n    let current = law[\"cases\"].as_array().unwrap().iter().find(|row| row[\"guide\"].is_array()).unwrap();\n    let mut registry = BrepRegistry::new();\n    let size: [f64; 2] = serde_json::from_value(current[\"size\"].clone()).unwrap();\n    let section = wire(&mut registry, PolygonWireRecipe::Rectangle { width: size[0], height: size[1] }, &[]);\n    let mut face = registry.plan_polygon_face(&section, false).unwrap().unwrap();\n    for _ in 0..512 {\n        if face.topology_ready() { break; }\n        funded(grant(), |effect| registry.step_polygon_face_topology(&mut face, Some(&section), None, effect)).unwrap();\n    }\n    assert!(face.topology_ready());\n    while registry.polygon_face_demand(&face, 0).unwrap().capacity_bytes != 0 {\n        registry.reserve_polygon_face(&face, grant()).unwrap();\n    }\n    let profile = face.handle().to_owned();\n    registry.publish_polygon_face(&mut face, profile.clone(), grant()).unwrap();\n    let mut rail = |key| {\n        let points: Vec<[f64; 3]> = serde_json::from_value(current[key].clone()).unwrap();\n        wire(&mut registry, PolygonWireRecipe::Polyline { vertices: 2, closed: false }, &points)\n    };\n    let (path, guide, varying) = (rail(\"path\"), rail(\"guide\"), rail(\"varyingGuide\"));\n    let before = registry.live_handles().len();\n    for refused in [varying.as_str(), path.as_str()] {\n        let Err(error) = registry.plan_straight_sweep(&profile, &path, Some(refused)) else { panic!(\"varying or zero guide\"); };\n        assert_eq!(error.kind, semio_framework_value::ValueRefusalKind::UnsupportedOwner);\n    }\n    assert_eq!(registry.live_handles().len(), before);\n    assert!(registry.straight_sweep_depth(&profile, &path, Some(&guide)).is_ok());\n    let PlanarExtrusion::Prism(mut draft) = registry.plan_straight_sweep(&profile, &path, Some(&guide)).unwrap() else { panic!(\"guided prism\") };\n    let mut solid = (0..512).find_map(|_| funded(grant(), |effect| registry.step_prism_draft(&profile, &mut draft, effect)).unwrap().0).unwrap();\n    for _ in 0..2048 {\n        if solid.topology_ready() { break; }\n        funded(grant(), |effect| registry.step_solid_primitive_topology(&mut solid, effect)).unwrap();\n    }\n    assert!(solid.topology_ready());\n    while registry.solid_primitive_demand(&solid, 0).unwrap().capacity_bytes != 0 {\n        registry.reserve_solid_primitive(&solid, grant()).unwrap();\n    }\n    let key = solid.handle().to_owned();\n    registry.publish_solid_primitive(&mut solid, key.clone(), grant()).unwrap();\n    let Entity::Solid(id, _) = *registry.original_query_entity(&key).unwrap() else { panic!(\"solid\") };\n    let body = registry.representation();\n    let reachable = body.reachable_from(&[EntityRef::Solid(id)]);\n    assert_eq!(reachable.faces.len() as u64, current[\"faces\"].as_u64().unwrap());\n    for edge in reachable.edges {\n        let uses = body.edge_coedges(edge);\n        assert_eq!(uses.len(), 2);\n        let orientation = |use_id| {\n            let member = body.coedges.get(use_id).unwrap();\n            let face = body.loops.get(member.loop_id).unwrap().face;\n            member.forward != body.faces.get(face).unwrap().flipped\n        };\n        assert_ne!(orientation(uses[0]), orientation(uses[1]));\n    }\n    let [low, high]: [[f64; 3]; 2] = serde_json::from_value(current[\"bounds\"].clone()).unwrap();\n    let (mut min, mut max) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);\n    for point in reachable.vertices {\n        let p = body.vertices.get(point).unwrap().position;\n        for (axis, value) in [p.x, p.y, p.z].into_iter().enumerate() {\n            min[axis] = min[axis].min(value);\n            max[axis] = max[axis].max(value);\n        }\n    }\n    assert_eq!([min, max], [low, high]);\n    assert!((solid_volume(body, id, 1e-3).unwrap() - current[\"volume\"].as_f64().unwrap()).abs() < 1e-8);\n    println!(\"[DEBUG] Retained registry and funded guided Pipe prism passed\");\n}\n".replace("FIXTURE",json.dumps(str(fixture),ensure_ascii=False)))
    output=d/("guided.wasm" if wasm else "guided.exe" if os.name=="nt" else "guided")
    compile_source(source,output,["semio_framework_"+a for a in ["value","3d_brep_native_registry","3d_brep_representation","3d_brep_queries"]]+["serde_json"],[],d)
    if wasm:
        runner=["repo","tools","exec","--light","node","--","node_modules/@bytecodealliance/jco/dist/jco.js","run",str(output)]
    else:
        runner=compiler[:8]+[str(output.resolve())]
    subprocess.run(runner,check=True,env={**os.environ,"RUST_MIN_STACK":"33554432"})
