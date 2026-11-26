#!/usr/bin/env python3
"""🎨️ Compile the latest native Boolean preview cursor over the existing linked math libraries."""
import itertools
import json
import re
import sys
from pathlib import Path
import subprocess

ticket = Path(__file__).resolve().parent
g = ticket / "🗑️generated/bridge-current"
target = g / "boolean-native"
precision = "--precision" in sys.argv
root = Path.cwd()
b = root / "🧰️framework/🔨️modules/🧊️3d/📐️brep"
cache = root / ".cache/b/r/m/tmp/flow-native-n25/debug/build"
preview = b / "⚙️engine/📦️mesh-io/🖼️primitive-preview/🦀️.rs"

source = target / "preview_law.rs"
code = (target / "trim_law.rs").read_text().replace("fn main()", "fn trim_law()")
code += f'\n#[path={json.dumps(str(preview),ensure_ascii=False)}]\nmod primitive_preview;\n'
code += """
use primitive_preview::PrimitivePreviewJob;
use semio_framework_mesh_engine::MeshData;
use semio_framework_3d_brep_vector::Vec3;
use semio_framework_job::{OperationId,Generation,OriginalJobFixtureCaller};
use semio_framework_value::RetainedCloneGrant as Grant;
use semio_framework_3d_brep_representation::arena::{ArenaId,SolidId};

fn step(mut job: PrimitivePreviewJob) -> MeshData {
    let stock=Grant{maximum_items:65536,maximum_copy_bytes:128<<20,maximum_capacity_bytes:256<<20,maximum_release_bytes:512<<20,maximum_depth:4096};
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(81231),Generation(1),stock,||Some(1000),1500,131072);
    for round in 0..20000 {
        let demand=job.demand();
        let grant=Grant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth.max(1)};
        let mut output=None;
        caller.turn(grant,|cx|cx.with_original_retirement(|cx|cx.with_original_work_loan(1,1500,|cx|{
            if let Some((polled,units))=cx.with_original_effect(grant,|_,effect|job.step(effect)).unwrap() {
                let (mesh,receipt)=polled.unwrap();
                assert_eq!(units,1);
                assert!(receipt.fits(grant));
                cx.consume_retained(receipt)?;
                output=mesh;
            }
            Ok(())
        }))).unwrap();
        if let Some(output)=output {
            println!("[DEBUG] preview slots consumed: {}",round+1);
            caller.retire(stock);
            return output;
        }
    }
    panic!("preview did not terminate");
}

fn check(mesh:MeshData,expected_volume:f64,bounds:[[f64;3];2],volume_tolerance:f64) {
    mesh.validate_component_references().unwrap();
    let points=mesh.positions.chunks_exact(3).map(|p|Vec3::new(p[0] as f64,p[1] as f64,p[2] as f64)).collect::<Vec<_>>();
    let mut edges=std::collections::BTreeMap::<_,(usize,isize)>::new();
    let mut volume=0.;
    for t in mesh.indices.chunks_exact(3) {
        let [a,b,c]=[t[0],t[1],t[2]].map(|i|points[i as usize]);
        volume+=a.dot(b.cross(c))/6.;
        let [a,b,c]=[a,b,c].map(|p|p.to_array().map(|n|(n*100000.).round() as i32));
        for (a,b) in [(a,b),(b,c),(c,a)] {
            assert_ne!(a,b);
            let (key,sense)=if a<b{((a,b),1)}else{((b,a),-1)};
            let (count,orientation)=edges.entry(key).or_default();
            *count+=1;
            *orientation+=sense;
        }
    }
    let low=std::array::from_fn::<_,3,_>(|i|points.iter().map(|p|p.to_array()[i]).reduce(f64::min).unwrap());
    let high=std::array::from_fn::<_,3,_>(|i|points.iter().map(|p|p.to_array()[i]).reduce(f64::max).unwrap());
    assert!([low,high].into_iter().flatten().zip(bounds.into_iter().flatten()).all(|(x,y)|(x-y).abs()<0.025));
    let bad=edges.iter().filter(|(_,v)|**v!=(2,0)).take(6).collect::<Vec<_>>();
    assert!(bad.is_empty(),"{bad:?}");
    assert!((volume-expected_volume).abs()<=volume_tolerance,"mesh volume {volume} vs {expected_volume}");
    println!("[DEBUG] {} closed outward triangles, volume {volume}, bounds {:?}..{:?}",mesh.indices.len()/3,low,high);
}

fn main() {
    trim_law();
    let origin=Frame3::WORLD.origin;
    let id=SolidId::from_raw(0,0);
    let box_=SphericalCornerBox::new(origin,1.2,origin,[1.5;3],1e-7).unwrap();
    let job=PrimitivePreviewJob::new_spherical_corner_box(id,box_,100,0.025).unwrap();
    check(step(job),9.708450789637022,[[-1.2;3],[1.5;3]],0.035 * 9.708450789637022);
    let torus=SphericalTorusCut::new(Frame3::WORLD,2.2,2.,0.5,1e-7).unwrap();
    let job=PrimitivePreviewJob::new_spherical_torus_cut(id,torus,100,0.025).unwrap();
    check(step(job),37.84114261331553,[[-2.1475,-2.1475,-2.2],[2.1475,2.1475,2.2]],0.035 * 37.84114261331553);
}
"""

if precision:
    fixture = root / "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧲️sphere-box-fuse/🧪️tests/🧫️fixtures/🧩️example/🔣️.json"
    contract = json.loads(fixture.read_text())
    code = code.replace("id,box_,100,0.025", f"id,box_,100,{contract['tessellationTolerance']}")
    code = code.replace("0.035 * 9.708450789637022", str(contract["expect"]["volumeTolerance"]))
else:
    transaction = b / "⚙️engine/🧠️native-registry/🏗️solid-transaction/🦀️.rs"
    code += f'\n#[path={json.dumps(str(transaction),ensure_ascii=False)}]\nmod solid_transaction;\n'
    code = code.replace("    trim_law();", "    trim_law();\n    publication();")
    code += """
    extern crate self as semio_framework_3d_brep_solid_topology;
    pub use crate::corner_trims as spherical_box_boundary;
    pub use crate::torus_trims as spherical_torus_boundary;

    fn publication() {
        use semio_framework_3d_brep_boolean::BooleanOp;
        use semio_framework_3d_brep_construction::primitives::SolidPrimitive;
        use semio_framework_3d_brep_representation::topology::{Body,history::LabelSource};
        use semio_framework_3d_brep_validation::validate_body;
        use semio_framework_value::RetirementDemand;
        use solid_transaction::{SolidPublication,boolean_recipe::BooleanShape};
        let origin=Frame3::WORLD.origin;
        let stock=Grant{maximum_items:65536,maximum_copy_bytes:128<<20,maximum_capacity_bytes:256<<20,maximum_release_bytes:512<<20,maximum_depth:4096};
        let to_grant=|need: RetirementDemand| Grant{maximum_items:1,maximum_copy_bytes:need.copy_bytes,maximum_capacity_bytes:need.capacity_bytes,maximum_release_bytes:need.release_bytes,maximum_depth:need.depth.max(1)};
        let sphere=|radius|SolidPrimitive::Sphere{frame:Frame3::WORLD,radius};
        for (index, (a,b,operation,volume)) in [
            (sphere(1.2), SolidPrimitive::Box{origin,width:1.5,depth:1.5,height:1.5}, BooleanOp::Unite,9.708450789637022),
            (sphere(2.2), SolidPrimitive::Torus{frame:Frame3::WORLD,major:2.,minor:0.5},BooleanOp::Cut,37.84114261331553),
        ].into_iter().enumerate() {
            let recipe=||BooleanShape::Analytic(a).combine(BooleanShape::Analytic(b),operation).unwrap();
            for cancelled in [true,false] {
                let mut caller=OriginalJobFixtureCaller::admit(OperationId(81232+index as u64),Generation(1),stock,||Some(1000),1500,131072);
                let mut body=Body::new();
                let mut labels=LabelSource::from_next(100);
                let mut job=SolidPublication::begin(101,recipe(),&mut labels,|_|[b'a';64]).unwrap();
                let mut rounds=0;
                while !job.topology_ready() {
                    let grant=to_grant(job.topology_demand(&body).unwrap());
                    caller.turn(grant,|cx|cx.with_original_retirement(|cx|cx.with_original_work_loan(1,1500,|cx|{
                        let (polled, work)=cx.with_original_effect(grant,|_,effect|job.advance(&mut body,effect)).unwrap().unwrap();
                        let receipt=polled.unwrap();
                        assert_eq!(work,1);
                        assert!(receipt.fits(grant));
                        cx.consume_retained(receipt)?;
                        Ok(())
                    }))).unwrap();
                    rounds+=1;
                    assert!(rounds <= 2000);
                    if cancelled&&body.edges.len()>=2 {break;}
                    assert!(job.product(101).is_err()||job.topology_ready());
                }
                if !cancelled {
                    let (id,proof)=job.product(101).unwrap();
                    assert!(body.solids.contains(id));
                    assert!((proof.unwrap().volume().unwrap()-volume).abs()<1e-9);
                    let blocking=validate_body(&body).into_iter().filter(|issue|!issue.code.starts_with("warning-")).collect::<Vec<_>>();
                    assert!(blocking.is_empty(),"{blocking:?}");
                    job.mark_published();
                    println!("[DEBUG] validated funded native publication {index}, {rounds} build turns");
                }
                let mut closing=0;
                while !job.terminal_is_empty() {
                    let grant=to_grant(job.close_demand(&body).unwrap());
                    caller.turn(grant,|cx|cx.with_original_retirement(|cx|cx.with_original_work_loan(1,1500,|cx|{
                        let (polled, work)=cx.with_original_effect(grant,|_,effect|job.close(&mut body,effect)).unwrap().unwrap();
                        let receipt=polled.unwrap().progress();
                        assert_eq!(work,1);
                        assert!(receipt.fits(grant));
                        cx.consume_retained(receipt)?;
                        Ok(())
                    }))).unwrap();
                    closing+=1;
                    assert!(closing<=2000);
                }
                if cancelled {
                    assert!(body.edges.is_empty()&&body.vertices.is_empty()&&body.faces.is_empty()&&body.solids.is_empty());
                    println!("[DEBUG] original native boolean publication {index} cancelled and retired after {closing} close turns");
                }
                caller.retire(stock);
            }
        }
    }
    """
    for module,folder in [
        ("clip_boundary","✂️clip-boundary"),
        ("rounded_box_boundary","🟠️rounded-box-boundary"),
        ("sector_boundary","🍩️sector-boundary"),
        ("spherical_cap_boundary","🌗️spherical-cap-boundary"),
    ]:
        path=b/"⚙️engine/🏗️solid-topology"/folder/"🦀️.rs"
        code+=f'\n#[path={json.dumps(str(path),ensure_ascii=False)}]\npub mod {module};\n'
source.write_text(code)
ids = [
    ("semio_framework_3d_brep_boolean", "3b036bc19f251a0f"),
    ("semio_framework_3d_brep_spline_construction", "20589d53eef2b668"),
    ("semio_framework_3d_brep_tolerance", "cc4503d8d5588cc2"),
    ("semio_framework_3d_brep_validation", "51a0ae045652cbf6"),
    ("semio_framework_3d_brep_vector", "df81ea6ab80bc74a"),
    ("semio_framework_3d_brep_curve", "08385df39d8403da"),
    ("semio_framework_3d_brep_construction", "f8adaa2878205a2d"),
    ("semio_framework_3d_brep_surface", "cd1137686430bf90"),
    ("semio_framework_3d_brep_representation", "cceb650e623642c4"),
    ("semio_framework_mesh_engine", "dd35759ec202b7fc"),
    ("semio_framework_value", "8857b9f3c67eb6c3"),
    ("semio_framework_job", "f724f40ceb640030"),
]
if precision: ids = ids[4:]
record=json.loads((g/"geometry-test-command.json").read_text())["args"]
search=[value for value in record if value.startswith(("dependency=","native="))]
externs=[]
for name, unit in ids:
    out = cache / name.replace("_","-") / unit / "out"
    path = out / f"lib{name}-{unit}.rlib"
    alias = "surface_base" if name.endswith("_surface") else name
    search.append(f"dependency={out}")
    externs += ["--extern",f"{alias}={path}","--extern",f"{alias}={path.with_suffix('.rmeta')}"]
rustc=subprocess.check_output(["rustup","which","rustc"],text=True).strip()
units = set()
for arg in externs:
    if arg.endswith(".rmeta"):
        metadata = subprocess.check_output([rustc,"-Zls=all",arg.split("=",1)[1]],text=True)
        units.update(re.findall(r"^(?:name|[0-9]+) ([^ ]+-[0-9a-f]{16})(?: |$)",metadata,re.MULTILINE))
for entry in units:
    crate,unit=entry.rsplit("-",1)
    additional=cache/crate.replace("_","-")/unit/"out"
    if additional.is_dir():
        search.append(f"dependency={additional}")
dependencies={v.rsplit("-",1)[1] for v in units}
search=[v for v in search if v.startswith("native=") or Path(v.split("=",1)[1]).parent.name in dependencies]
print(f"[DEBUG] Source law compiling with {len(set(search))} exact dependency search paths",flush=True)
args=[rustc,"--edition=2021","-Awarnings","-Zthreads=1",str(source),"-o",str(target/"preview-law"),*externs]
args+=list(itertools.chain.from_iterable(("-L",path) for path in dict.fromkeys(search)))
result = subprocess.run([g/'retry-compiler',*args])
if result.returncode: raise SystemExit(result.returncode)
subprocess.run([target/"preview-law"],check=True)
