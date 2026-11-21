from pathlib import Path
import json
import os
import re
import subprocess
import sys

def write_source(file, text):
    file.write_text(text, encoding="utf-8")

ticket=Path(__file__).resolve().parent
mode=sys.argv[1] if len(sys.argv)>1 else "bevel"
previous=ticket/"🗑️generated"/((mode if mode!="authored-difference" else "authored-intersection")+"-native")
refresh="--rebuild-projection" in sys.argv[2:]
out=ticket/"🗑️generated"/(mode+"-current-native") if refresh else previous
out.mkdir(parents=True,exist_ok=True)
cache=Path(".cache/b/r/m/u/l")
snapshot=next((Path(arg.partition("=")[2]) for arg in sys.argv[2:] if arg.startswith("--compiler-inputs=")),None)
units=snapshot.glob("*") if snapshot is not None else (p for lane in cache.glob("*/prototype/build") for p in lane.glob("*/*/out/*"))
artifacts={p.stem.removeprefix("lib"):p for p in units if p.suffix in (".rlib",".dylib",".so",".dll")}
engine=next((p for name,p in artifacts.items() if name.startswith("semio_framework_mesh_engine-")),None)
protocol=None
def metadata(library):
    return json.loads(library.with_suffix(".rmeta.semio-deps").read_text(encoding="utf-8"))
if (engine is None or "--protocol" in sys.argv[2:]) and mode in ("convex-intersection","convex-difference","convex-union"):
    selected=next((name for name,_ in metadata(engine)["dependencies"] if name.startswith("protocol-")),None) if engine is not None else None
    requested=next((arg.partition("=")[2] for arg in sys.argv[2:] if arg.startswith("--protocol-key=")),selected)
    protocol=artifacts[requested] if requested in artifacts else next(p for name,p in artifacts.items() if name.startswith("protocol-"))
engine_protocol_key=next((arg.partition("=")[2] for arg in sys.argv[2:] if arg.startswith("--engine-protocol-key=")),None)
engine_protocol=artifacts[engine_protocol_key] if engine_protocol_key else None
lineage=dict(metadata(protocol or engine_protocol or engine)["dependencies"])
value_id=next(name for name in lineage if name.startswith("semio_framework_value-"))
dependencies={"semio_framework_value":artifacts[value_id]}
if protocol is not None:
    dependencies["protocol"]=protocol
else:
    pack_id=next((name for name in lineage if name.startswith("semio_framework_pack_json-")),None)
    pack=artifacts.get(pack_id) or next(p for name,p in artifacts.items() if name.startswith("semio_framework_pack_json-") and dict(metadata(p)["dependencies"]).get(value_id)==lineage[value_id])
    dependencies["semio_framework_mesh_engine"]=engine
    dependencies["semio_framework_pack_json"]=pack
    source_hold=next((p for name,p in artifacts.items() if name.startswith("semio_framework_value_source_hold-") and dict(metadata(p)["dependencies"]).get(value_id)==lineage[value_id]),None)
    if source_hold is not None:
        dependencies["semio_framework_value_source_hold"]=source_hold
for crate in ("semio_framework_value_derive","serde_json"):
    selected=next((name for name in lineage if name.startswith(crate+"-")),None)
    dependencies[crate]=artifacts[selected] if selected else next(p for name,p in artifacts.items() if name.startswith(crate+"-"))
reused_mesh=next((Path(arg.partition("=")[2]) for arg in sys.argv[2:] if arg.startswith("--use-modeling=")),None)
projection=next((Path(arg.partition("=")[2]) for arg in sys.argv[2:] if arg.startswith("--use-projection=")),None)
engine_stage=next((Path(arg.partition("=")[2]) for arg in sys.argv[2:] if arg.startswith("--use-engine=")),reused_mesh)
search=sorted({p.parent for p in artifacts.values()})+[out]
if refresh:
    search.append(previous)
if reused_mesh is not None:
    search.append(reused_mesh)
if projection is not None:
    search.append(projection)
if engine_stage is not None and engine_stage!=reused_mesh:
    search.append(engine_stage)

def compile_source(source,crate,requires,test=False,edition="2024"):
    print("[DEBUG] compiling "+crate,flush=True)
    binary=out/((crate+(".exe" if os.name=="nt" else "")) if test else "lib"+crate+".rlib")
    reuse=reused_mesh
    if crate in ("semio_framework_3d_mesh_clipping","semio_bevel_modeling"):
        if projection is not None:
            reuse=projection
        if "--rebuild-clipping" in sys.argv[2:]:
            reuse=None
    if refresh and crate not in ("semio_framework_3d_mesh_reconstruction","semio_bevel_modeling"):
        reuse=previous
    if not test and reuse is not None:
        copied=reuse/binary.name
        if not copied.is_file():
            raise SystemExit("Missing compiled owner "+str(copied))
        dependencies[crate]=copied
        return copied
    if not test and ("--reuse" in sys.argv[2:] or ("--rebuild-modeling" in sys.argv[2:] and crate!="semio_bevel_modeling")):
        if not binary.is_file():
            raise SystemExit("Missing previously compiled "+crate)
        dependencies[crate]=binary
        return binary
    command=["repo","tools","exec","rustup","--","run","nightly-2026-07-20","rustc",str(source),"--edition",edition,"--crate-name",crate,"-C","debuginfo=0","-o",str(binary)]
    command+=["--test"] if test else ["--crate-type","rlib"]
    if mode=="schema-edit":
        command+=["--error-format=short"]
    for path in search:
        command+=["-L","dependency="+str(path)]
    for name in requires:
        name=name.replace("-","_")
        library=dependencies[name]
        command+=["--extern",name+"="+str(library)]
        if library.with_suffix(".rmeta").exists():
            command+=["--extern",name+"="+str(library.with_suffix(".rmeta"))]
    status=subprocess.run(command).returncode
    if status:
        raise SystemExit(status)
    dependencies[crate]=binary
    return binary

if protocol is not None:
    forward=out/"mesh-engine-reexports.rs"
    write_source(forward,"pub use protocol::causal::{HistoryFoldIndex,HistoryFoldSet};\n")
    compile_source(forward,"semio_framework_mesh_engine",["protocol"])

base=Path("🧰️framework/🔨️modules")
if protocol is None and "semio_framework_value_source_hold" not in dependencies:
    compile_source(base/"🌱️value/🔗️source-hold/🦀️.rs","semio_framework_value_source_hold",["semio_framework_value"],edition="2021")
mesh=base/"🧊️3d/🥽️mesh"
folders=[base/"🔢️number",base/"🧊️3d/📐️brep/📸️representation/〰️polynomial",base/"🧊️3d/📐️brep/📸️representation/➡️vector",mesh/"🧬️contract",mesh/"📄️document",mesh/"🛠️modeling/✂️clipping",mesh/"🛠️modeling/🔁️reconstruction"]
if "--rebuild-engine" in sys.argv[2:]:
    engine_source=base/"🏗️mesh-engine"
    manifest=(engine_source/"📦️packages/🦀️rust/Cargo.toml").read_text(encoding="utf-8")
    declared=manifest.partition("[dependencies]")[2].split("[dev-dependencies]",1)[0]
    requires=[line.split()[0].replace("-","_") for line in declared.splitlines() if " = " in line]
    for crate in requires:
        if crate=="semio_framework_mesh_component_references":
            continue
        if crate=="protocol" and engine_protocol is not None:
            dependencies[crate]=engine_protocol
            continue
        nested=next((name for name in lineage if name.startswith(crate+"-")),None)
        if nested is not None:
            dependencies[crate]=artifacts[nested]
        else:
            compatible=next((p for name,p in artifacts.items() if name.startswith(crate+"-") and dict(metadata(p)["dependencies"]).get(value_id)==lineage[value_id]),None)
            if compatible is not None:
                dependencies[crate]=compatible
            elif crate=="pack" and engine_protocol is not None:
                forwarding=out/"pack-reexports.rs"
                write_source(forwarding,"pub use semio_framework_value as value;\npub use semio_framework_pack_json as json;\n")
                compile_source(forwarding,"pack",["semio_framework_value","semio_framework_pack_json"],edition="2021")
            else:
                raise SystemExit("Missing compatible cached owner "+crate)
    compile_source(engine_source/"🎯️component-references/🦀️.rs","semio_framework_mesh_component_references",
        ["semio_framework_value","semio_framework_pack_json"],edition="2021")
    compile_source(engine_source/"🦀️.rs","semio_framework_mesh_engine",requires,edition="2021")
elif "--current-engine" not in sys.argv[2:] and engine_stage is not None and (engine_stage/"libsemio_framework_mesh_engine.rlib").is_file():
    dependencies["semio_framework_mesh_engine"]=engine_stage/"libsemio_framework_mesh_engine.rlib"
if mode=="authored-unwrap":
    folders=[mesh/"🧬️contract",mesh/"📄️document"]
if mode in ("convex-intersection","convex-difference","convex-union"):
    folders=[mesh/"🧬️contract",mesh/"🛠️modeling/✂️clipping"]
def diagnostic_clipping(source):
    parts={
        "✂️clipping":"clipping",
        "⚗️convex-intersection":"convex",
        "➕️convex-union":"union",
        "➗️partial-union":"partial",
    }
    original=source.read_text(encoding="utf-8")
    def mount(match):
        target=(source.parent/match[1]).resolve()
        if target.parent.name in parts:
            target=diagnostic_clipping(target)
        return '#[path='+json.dumps(str(target),ensure_ascii=False)+']'
    code=re.sub(r'#\[path\s*=\s*"([^"]+)"\]',mount,original)
    if source.parent.name=="➗️partial-union":
        marker='if uses[direction]{return Err(invalid("partial union produced nonmanifold edge incidence"));}'
        report='''if uses[direction]{
            eprintln!("[DEBUG] repeated directed seam {a}->{b} on {} at {:?}",self.face,self.output.faces[self.face]);
            for (index,polygon) in self.output.geometry.faces.iter().enumerate(){
                let ring=polygon.iter().map(|&id|(id,self.output.geometry.positions[id as usize])).collect::<Vec<_>>();
                eprintln!("[DEBUG] {index} {:?}: {ring:?}",self.output.faces[index]);
            }
            return Err(invalid("partial union produced nonmanifold edge incidence"));
        }'''
        if marker in code:
            code=code.replace(marker,report)
        open_marker='if self.paired!=self.edges.len(){return Err(invalid("partial union produced open incidence"));}'
        open_report='''if self.paired!=self.edges.len(){
            for slot in 0..self.edges.slot_count(){
                if let Some((&(a,b),uses))=self.edges.slot_entry(slot){
                    if *uses==[true,true]{continue;}
                    let at=|id|self.output.geometry.positions[id as usize];
                    eprintln!("[DEBUG] open union edge {a} {:?} -> {b} {:?} {uses:?}",at(a),at(b));
                }
            }
            return Err(invalid("partial union produced open incidence"));
        }'''
        assert open_marker in code
        code=code.replace(open_marker,open_report)
    result=out/("diagnostic-"+parts[source.parent.name]+".rs")
    write_source(result,code)
    return result

for folder in folders:
    manifest=next(folder.glob("📦️packages/*/Cargo.toml"))
    content=manifest.read_text(encoding="utf-8")
    name=re.search(r'^name = "([^"]+)"',content,re.M)[1].replace("-","_")
    path=re.search(r'^path = "([^"]+)"',content,re.M)
    source=(manifest.parent/(path[1] if path else "🦀️.rs")).resolve()
    declared=content.partition("[dependencies]")[2].split("[",1)[0]
    requires=[re.split(r"\s|\.",line)[0] for line in declared.splitlines() if line and not line.startswith("#")]
    if name=="semio_framework_3d_mesh_clipping" and "--diagnose-union" in sys.argv[2:]:
        source=diagnostic_clipping(source)
    compile_source(source,name,requires)

facade={"convex-union":"semio_framework_3d_mesh_clipping","convex-difference":"semio_framework_3d_mesh_clipping","authored-unwrap":"semio_framework_3d_mesh_document","convex-intersection":"semio_framework_3d_mesh_clipping"}.get(mode,"semio_bevel_modeling")
if facade=="semio_bevel_modeling":
    compile_source(mesh/"🦀️.rs","semio_bevel_modeling",["semio_framework_3d_mesh_document","semio_framework_3d_mesh_reconstruction","semio_framework_3d_mesh_contract","semio_framework_3d_mesh_clipping","semio_framework_3d_brep_vector","semio_framework_value_source_hold","semio_framework_value","semio_framework_mesh_engine","semio_framework_value_derive"])
compound_source=(ticket/"mesh-compound-source-proof.rs").resolve()
compound_module=f'\n#[path = "{compound_source}"] mod compound;\n'
compound_fixture=mesh/"🧪️tests/🧫️fixtures/🛠️modeling/🧩️disconnected-difference.json"
if mode=="flow-union":
    flow=Path("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🧮️operator-core/🔗️combine-source/🦀️.rs").resolve()
    driver=out/"flow_union.rs"
    write_source(driver,(ticket/"flow-union-native.test.rs").read_text(encoding="utf-8")+"\n"+f'#[path = "{flow}"] mod production;'+compound_module)
    binary=compile_source(driver,"flow_union_proof",[facade,"serde_json","semio_framework_value","semio_framework_value_derive","semio_framework_mesh_engine"],test=True)
    fixture=mesh/"🧪️tests/🧫️fixtures/🛠️modeling/➕️convex-union.json"
    raise SystemExit(subprocess.run([str(binary),"--nocapture"],env={**os.environ,"SEMIO_MESH_UNION_FIXTURE":str(fixture),"SEMIO_MESH_COMPOUND_FIXTURE":str(compound_fixture)}).returncode)
if mode=="schema-edit":
    import os
    job=next((p for name,p in artifacts.items() if name.startswith("semio_framework_job-") and dict(metadata(p)["dependencies"]).get(value_id)==lineage[value_id]),None)
    if job is None:
        caller=next(p for name,p in artifacts.items() if name.startswith("semio_framework_job_testing_caller-") and dict(metadata(p)["dependencies"]).get(value_id)==lineage[value_id])
        job=next(artifacts[name] for name,_ in metadata(caller)["dependencies"] if name.startswith("semio_framework_job_context-"))
    dependencies["semio_framework_job"]=job
    subset=Path("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any")
    geometry=subset/"🧬️schema/💡️inferences/📐️geometry"
    support=(geometry/"🧱️mesh-support/🦀️.rs").read_text(encoding="utf-8")
    compute=(geometry/"⚙️compute/🦀️.rs").read_text(encoding="utf-8")
    edit=(geometry/"✏️mesh-edit/🦀️.rs").read_text(encoding="utf-8")
    def section(text,start,end):
        position=text.index(start)
        return text[position:text.index(end,position+len(start))]
    parts=[
        support[support.index("pub const BATCH:"):support.index("//#endregion 🔖️Faults")],
        "#[derive(Clone, Copy, Debug, PartialEq, Eq)]\n"+section(support,"pub enum Element","pub fn mode_element"),
        section(support,"pub fn owned_mesh","fn narrowed"),
        section(support,"pub enum Flow","/// 🔎️ Reads a polygon source"),
        section(compute,"pub fn fuel_budget","pub trait WidgetJob"),
        section(compute,"pub mod testing","#[cfg(test)]"),
        section(edit,"fn bridge(","fn dissolve_edges"),
    ]
    own=geometry/"🧱️mesh-support/🔗️combine/🦀️.rs"
    driver=out/"schema_edit.rs"
    write_source(driver,(ticket/"schema-mesh-native.test.rs").read_text(encoding="utf-8")+"\n"+f'#[path = "{own.resolve()}"] mod combine;'+compound_module+"\n".join(parts))
    binary=compile_source(driver,"schema_mesh_edit_proof",[facade,"serde_json","semio_framework_value","semio_framework_mesh_engine","semio_framework_job"],test=True)
    fixture=subset/"🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/✏️mesh-edit/🔣️.json"
    raise SystemExit(subprocess.run([str(binary),"--nocapture"],env={**os.environ,"SEMIO_MESH_EDIT_FIXTURE":str(fixture),"SEMIO_MESH_COMPOUND_FIXTURE":str(compound_fixture)}).returncode)

if mode=="convex-difference" and "--registered-clipping" in sys.argv[2:]:
    library=mesh/"🛠️modeling/✂️clipping/🦀️.rs"
    if "--diagnose-union" in sys.argv[2:]:
        library=diagnostic_clipping(library)
    witness=compile_source(library,"registered_clipping_proof",["semio_framework_3d_mesh_contract","semio_framework_mesh_engine","semio_framework_value","serde_json"],test=True)
    raise SystemExit(subprocess.run([str(witness),"--nocapture"]).returncode)
test_file="🎨️surface/🧪️tests/🔬️unit/🦀️.rs" if mode=="authored-unwrap" else "🧪️tests/🔬️modeling/🦀️.rs"
if mode in ("convex-intersection","convex-difference","convex-union"):
    test_file="🛠️modeling/✂️clipping/🧪️tests/🦀️.rs"
tests=(mesh/test_file).read_text(encoding="utf-8")
name={"authored-partial-union":"authored_convex_intersection_projects_both_uv_color_and_material_domains","authored-cavity":"authored_convex_intersection_projects_both_uv_color_and_material_domains","authored-pocket":"authored_convex_intersection_projects_both_uv_color_and_material_domains","authored-tunnel":"authored_convex_intersection_projects_both_uv_color_and_material_domains","authored-octant":"authored_convex_intersection_projects_both_uv_color_and_material_domains","authored-slab":"authored_convex_intersection_projects_both_uv_color_and_material_domains","authored-corner":"authored_convex_intersection_projects_both_uv_color_and_material_domains","authored-union":"authored_convex_intersection_projects_both_uv_color_and_material_domains","convex-union":"convex_union_welds_matching_faces_and_preserves_source_ownership","authored-difference":"authored_convex_intersection_projects_both_uv_color_and_material_domains","convex-difference":"convex_difference_keeps_reversed_cut_provenance","authored-intersection":"authored_convex_intersection_projects_both_uv_color_and_material_domains","convex-intersection":"convex_intersection_keeps_halfspaces_and_two_source_provenance","boundary-extrude":"boundary_extrusion_preserves_a_connected_authored_strip","polygon-capture":"polygon_capture_keeps_canonical_rows_and_owned_metadata","local-bevel":"isolated_convex_bevel_preserves_a_concave_closed_prism","bridge":"bridge_joins_two_anchored_rims_and_preserves_authored_surface","selected-weld":"selected_vertex_weld_preserves_authored_surface","dissolve-seam":"boundary_vertex_dissolve_protects_both_authored_seam_segments","authored-unwrap":"unwrapping_replaces_only_the_active_named_uv_channel","bevel":"bevel_preserves_authored_surface_through_all_profile_planes"}[mode]
def extract_test(name):
    start=tests.index("#[test]\nfn "+name)
    end=tests.find("#[test]",start+1)
    return tests[start:len(tests) if end<0 else end]
body=extract_test(name)
if mode=="authored-difference":
    body+=extract_test("partitioned_difference_projects_authored_cutter_faces")
if mode=="local-bevel":
    body+=extract_test("bevel_preserves_authored_surface_through_all_profile_planes")
if mode=="boundary-extrude":
    body+=extract_test("bridge_joins_two_anchored_rims_and_preserves_authored_surface")
fixture=(mesh/"🧪️tests/🧫️fixtures").resolve()
if mode=="authored-unwrap":
    helper=tests[tests.index("fn drain("):tests.index("fn grid(")]
    helper+=tests[tests.index("fn corner_uvs("):tests.index("#[test]")]
    body=helper+body.replace("../../../🧪️tests/🧫️fixtures/",str(fixture)+"/")
elif mode in ("convex-intersection","convex-difference","convex-union"):
    body+=extract_test("clipping_provenance_is_complete_and_transferred_once")
    if mode=="convex-union":
        body+=extract_test("convex_difference_keeps_reversed_cut_provenance")
    if mode=="convex-difference":
        body+=extract_test("convex_difference_retains_prismatic_openings")
    body=body.replace("../../../🧪️tests/🧫️fixtures/",str(fixture)+"/")
    body="use semio_framework_3d_mesh_contract::{Vec3,MeshPolygonSoup as Soup,MeshSourceVertex as SourceVertex};\n"+body
else:
    body=body.replace("../🧫️fixtures/",str(fixture)+"/")
driver=out/"proof.rs"
write_source(driver,"use "+facade+"::*;\n"+body+("" if mode not in ('bevel','bridge','local-bevel','boundary-extrude','authored-intersection','authored-difference','authored-union','authored-corner','authored-slab','authored-octant','authored-tunnel','authored-pocket','authored-cavity','authored-partial-union') else """
fn assert_edit_surface(mesh:&HalfedgeMesh,expected_area:Option<f64>,expected_volume:Option<f64>) {
    if expected_volume.is_some() {assert!(mesh.halfedges.iter().all(|edge|edge.twin.is_some()));}
    let (positions, faces) = mesh.polygon_soup();
    let mut live = std::collections::BTreeSet::new();
    for ring in faces {
        assert_eq!(ring.iter().copied().collect::<std::collections::BTreeSet<_>>().len(), ring.len());
        live.extend(ring);
    }
    assert_eq!(live.len(), positions.len());
    let mut area=0.0;
    let mut volume=0.0;
    let transfer=mesh.tessellate().unwrap();
    for triangle in transfer.indices.chunks_exact(3) {
        let point=|id:u32| {let i=id as usize*3;Vec3::new(transfer.positions[i],transfer.positions[i+1],transfer.positions[i+2])};
        let (a,b,c)=(point(triangle[0]),point(triangle[1]),point(triangle[2]));
        let normal=b.sub(a).cross(c.sub(a));
        area+=f64::from(normal.dot(normal)).sqrt()/2.0;
        volume+=f64::from(a.dot(b.cross(c)))/6.0;
    }
    if let Some(expected)=expected_area {assert!((area-expected).abs()<1e-5, "area={area}, expected={expected}");}
    if let Some(expected)=expected_volume {assert!((volume-expected).abs()<1e-5, "volume={volume}, expected={expected}");}
}
"""))
requires=[facade,"serde_json","semio_framework_value"]
if mode in ("convex-intersection","convex-difference","convex-union"):
    requires.append("semio_framework_3d_mesh_contract")
if mode in ("authored-intersection","authored-difference","authored-union","authored-corner","authored-slab","authored-octant","authored-tunnel","authored-pocket","authored-cavity","authored-partial-union"):
    requires.append("semio_framework_mesh_engine")
binary=compile_source(driver,mode.replace("-","_")+"_proof",requires,test=True)
raise SystemExit(subprocess.run([str(binary),"--nocapture"]).returncode)
