"""🧪️ Drive the current funded cuboid spline transaction against the neutral fixture."""
import base64
import hashlib
import io
import os
from pathlib import Path
import subprocess
import sys
import tarfile
import urllib.request
from tempfile import TemporaryDirectory

ticket=Path(__file__).parent
root=Path("🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🧠️native-registry/🏗️solid-transaction")
fixture=Path("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🔣️nurbs-cuboid.json")
source=r'''
#[path = "__TRANSACTION__"]
mod transaction;
#[path = "__WRITER__"]
mod exchange;
#[path = "__READER__"]
mod decode;
pub use transaction::*;
use semio_framework_3d_brep_construction::primitives::{SolidPrimitive,make_box,make_cylinder};
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_representation::{arena::SolidId,topology::{Body,history::OpRecorder}};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_vector::Pnt3;
use semio_framework_value::{AuthoringWallet,RetainedCloneGrant as Grant,RetirementDemand,RetainedWorkPoll,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use serde_json::Value;
#[cfg(heap_witness)]
fn measured<T>(operation:impl FnOnce()->T)->(T,usize,usize) {
    let (output,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(operation);
    (output,heap.requested_bytes,heap.released_bytes)
}

#[cfg(not(heap_witness))]
fn measured<T>(operation:impl FnOnce()->T)->(T,usize,usize) { (operation(),0,0) }

fn physical(actual:(usize,usize),expected:(usize,usize)) {
    if cfg!(heap_witness) { assert_eq!(actual,expected); }
}

const STOCK: Grant = Grant {maximum_items:65536,maximum_copy_bytes:128<<20,maximum_capacity_bytes:256<<20,maximum_release_bytes:512<<20,maximum_depth:4096};

fn quote(demand: RetirementDemand) -> Grant {
    Grant {maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}
}

fn point(v: &Value) -> Pnt3 {
    Pnt3::from_array(std::array::from_fn(|i|v[i].as_f64().unwrap()))
}

fn step_document(body:&Body, solid:SolidId)->Result<String,ValueError> {
    let mut wallet=AuthoringWallet::new(STOCK,1_000_000_000_000);
    let handle="a".repeat(64);
    let source=exchange::StepWriteSource{body,solid,revision:1,handle:&handle,depth:1};
    let mut writer=exchange::StepWrite::new(source);
    let mut turns=0;
    while !writer.ready() {
        let exact=quote(writer.demand(source).unwrap());
        if exact.maximum_copy_bytes>0 {
            let denied=Grant{maximum_copy_bytes:exact.maximum_copy_bytes-1,..exact};
            let turn=wallet.run(denied,1,|effect|{
                let (result,born,free)=measured(||writer.step(source,effect));
                physical((born,free),(0,0));
                result
            }).unwrap().unwrap();
            assert_eq!(turn.output.unwrap(),Default::default());
            assert_eq!(quote(writer.demand(source).unwrap()),exact);
        }
        let turn=wallet.run(exact,1,|effect|{
            let (result,born,free)=measured(||writer.step(source,effect));
            let receipt=match &result { Ok(receipt)=>*receipt,Err(error)=>error.retained_progress() };
            physical((born,free),(receipt.retained_capacity_bytes,receipt.released_bytes));
            result
        }).unwrap().unwrap();
        match turn.output {
            Ok(receipt)=>{ assert!(receipt.fits(exact)); wallet.post(receipt).unwrap(); }
            Err(error)=>{ assert_eq!(writer.output_capacity(),0); return Err(error); }
        }
        turns+=1;
        assert!(turns<2000);
    }
    writer.take_output(source)
}

fn decode_document(text:&str)->Result<decode::StepSolid,ValueError> {
    let mut wallet=AuthoringWallet::new(STOCK,1_000_000_000_000);
    let mut importer=decode::StepSolidDecode::new(text);
    let mut turns=0;
    while !importer.is_ready() {
        let exact=quote(importer.demand());
        let result=wallet.run(exact,1,|effect|{
            let (result,born,free)=measured(||importer.step(text,effect));
            let receipt=match &result { Ok(receipt)=>*receipt,Err(error)=>error.retained_progress() };
            assert!(receipt.fits(exact));
            physical((born,free),(receipt.retained_capacity_bytes,receipt.released_bytes));
            result
        }).unwrap().unwrap().output;
        match result {
            Ok(receipt)=>wallet.post(receipt).unwrap(),
            Err(error)=>{
                let exact=quote(importer.close_demand());
                let receipt=wallet.run(exact,1,|effect|{
                    let (result,born,free)=measured(||importer.close(effect));
                    let receipt=result.unwrap();
                    physical((born,free),(0,receipt.released_bytes));
                    receipt
                }).unwrap().unwrap().output;
                wallet.post(receipt).unwrap();
                assert!(importer.terminal_is_empty());
                return Err(error);
            }
        }
        turns+=1;
        assert!(turns<10000);
    }
    assert!(importer.terminal_is_empty());
    Ok(importer.ready().unwrap())
}

fn replay(proof:decode::StepSolid)->String {
    let mut body=Body::new();
    let mut wallet=AuthoringWallet::new(STOCK,1_000_000_000_000);
    let mut draft=SolidPublication::begin_nurbs_cuboid(1,proof.nurbs_cuboid().unwrap(),&mut body.labels, |_| [b'a';64]).unwrap();
    while !draft.topology_ready() {
        let exact=quote(draft.topology_demand(&body).unwrap());
        let receipt=wallet.run(exact,1,|effect|draft.advance(&mut body,effect)).unwrap().unwrap().output.unwrap();
        assert!(receipt.fits(exact));
        wallet.post(receipt).unwrap();
    }
    let (id,_)=draft.product(1).unwrap();
    step_document(&body,id).unwrap()
}

fn check_entities(text:&str,counts:impl IntoIterator<Item=(String,usize)>) {
    for (entity,count) in counts {
        let marker=format!(" = {entity}(");
        assert_eq!(text.lines().filter(|line|line.contains(&marker)).count(),count,"{entity}");
    }
}

fn check_analytic_regression() {
    let mut body=Body::new();
    let id=make_box(&mut body,4.0,2.0,3.0,&mut OpRecorder::new()).unwrap();
    let planar=step_document(&body,id).unwrap();
    check_entities(&planar,[("LINE",12),("PLANE",6),("B_SPLINE_SURFACE_WITH_KNOTS",0)].map(|(tag,count)|(tag.to_string(),count)));

    let mut body=Body::new();
    let id=make_cylinder(&mut body,2.0,5.0,&mut OpRecorder::new()).unwrap();
    let cylinder=step_document(&body,id).unwrap();
    check_entities(&cylinder,[("CYLINDRICAL_SURFACE",1),("CIRCLE",4),("SEAM_CURVE",1),("PCURVE",6)].map(|(tag,count)|(tag.to_string(),count)));
}

fn main() {
    check_analytic_regression();
    let data:Value=serde_json::from_str(include_str!("__FIXTURE__")).unwrap();
    let tol=data["tolerance"].as_f64().unwrap();
    for (index,row) in data["cases"].as_array().unwrap().iter().enumerate() {
        let origin=point(&row["origin"]);
        let [width,depth,height]=point(&row["size"]).to_array();
        let primitive=SolidPrimitive::Box{origin,width,depth,height};
        for stop in [Some(12),Some(105),None] {
            let mut body=Body::new();
            let mut wallet=AuthoringWallet::new(STOCK,1_000_000_000_000);
            let mut draft=SolidPublication::begin_nurbs_cuboid(1,primitive,&mut body.labels, |_| [b'a';64]).unwrap();
            let mut turns=0;
            while !draft.topology_ready() && stop != Some(turns) {
                let exact=quote(draft.topology_demand(&body).unwrap());
                if exact.maximum_capacity_bytes>0 {
                    let denied=Grant{maximum_capacity_bytes:0,..exact};
                    let (turn,_)=wallet.run_settled(denied,1,|effect| { let (result,born,free)=measured(||draft.advance(&mut body,effect)); let receipt=result.as_ref().unwrap(); physical((born,free),(receipt.retained_capacity_bytes,receipt.released_bytes)); RetainedWorkPoll::Complete(RetainedWorkOutcome { result,work:effect.performed_work() }) }).unwrap().unwrap();
                    assert_eq!(turn.result.unwrap(),Default::default());
                    assert_eq!(quote(draft.topology_demand(&body).unwrap()),exact);
                }
                let (turn,_)=wallet.run_settled(exact,1,|effect| { let (result,born,free)=measured(||draft.advance(&mut body,effect)); let receipt=result.as_ref().unwrap(); physical((born,free),(receipt.retained_capacity_bytes,receipt.released_bytes)); RetainedWorkPoll::Complete(RetainedWorkOutcome { result,work:effect.performed_work() }) }).unwrap().unwrap();
                let receipt=turn.result.unwrap();
                assert!(receipt.fits(exact) && receipt!=Default::default());
                wallet.post(receipt).unwrap();
                turns+=1;
                assert!(turns<600);
            }
            if stop.is_none() {
                let (solid,proof)=draft.product(1).unwrap();
                assert!(matches!(proof,Some(SolidProvenance::Primitive(proof)) if proof.nurbs_cuboid));
                assert_eq!([body.vertices.len(),body.edges.len(),body.faces.len(),body.curves3.len(),body.surfaces.len()],[8,12,6,12,6]);
                assert!((primitive.volume()-row["volume"].as_f64().unwrap()).abs()<tol);
                assert!((primitive.area()-row["area"].as_f64().unwrap()).abs()<tol);
                for (index,(_,edge)) in body.edges.iter().enumerate() {
                    let actual=body.curves3.get(edge.curve).unwrap();
                    assert!(matches!(actual,Curve3::Nurbs{knots,controls,weights} if knots.degree==1 && controls.len()==2 && weights==&[1.0,1.0]));
                    for t in [0.0,0.125,0.375,0.75,1.0] {
                        assert!(actual.eval(t).distance(primitive.edge(index).curve.eval(t))<tol);
                    }
                    if index==0 {assert!(actual.eval(row["edge"]["parameter"].as_f64().unwrap()).distance(point(&row["edge"]["point"]))<tol);}
                }
                for (idx,(_,face)) in body.faces.iter().enumerate() {
                    let actual=body.surfaces.get(face.surface).unwrap();
                    assert!(matches!(actual,Surface::Nurbs{u_knots,v_knots,controls,weights} if u_knots.degree==1 && v_knots.degree==1 && controls.len()==2 && weights.iter().all(|r|r==&[1.0,1.0])));
                    let expected=primitive.surface(idx);
                    for member in 0..4 {
                        let trim=primitive.trim(idx,member);
                        for t in [0.0,0.375,1.0] {
                            let uv=trim.curve.eval(t);
                            assert!(actual.eval(uv.x,uv.y).distance(expected.eval(uv.x,uv.y))<tol);
                        }
                    }
                    let ((u0,u1),(v0,v1))=actual.domain();
                    assert!(u0<u1 && v0<v1);
                    let (u,v)=(u0+(u1-u0)*0.375,v0+(v1-v0)*0.75);
                    assert!(actual.eval(u,v).distance(expected.eval(u,v))<tol);
                    if idx==0 {
                        let expected=point(&row["patch"]["point"]);
                        let Surface::Plane { frame }=primitive.surface(idx) else { panic!() };
                        let tangent=expected-frame.origin;
                        assert!(actual.eval(tangent.dot(frame.x),tangent.dot(frame.y)).distance(expected)<tol);
                    }
                }
                let text=step_document(&body,solid).unwrap();
                check_entities(&text,data["stepExport"]["counts"].as_object().unwrap().iter()
                    .map(|(name,count)|(name.clone(),count.as_u64().unwrap() as usize)));
                let expected=decode::StepSolid::NurbsCuboid((origin.to_array(),[width,depth,height]));
                let proof=decode_document(&text).unwrap();
                assert_eq!(proof,expected);
                let recovered=replay(proof);
                assert_eq!(recovered,text);
                if let Some(folder)=std::env::args().nth(1) {
                    let folder=std::path::Path::new(&folder);
                    let external=std::fs::read_to_string(folder.join(format!("nurbs-external-{index}.step"))).unwrap();
                    let external_proof=decode_document(&external).unwrap();
                    assert_eq!(external_proof,expected);
                    assert_eq!(replay(external_proof),recovered);
                    std::fs::write(folder.join(format!("nurbs-{index}.step")),text).unwrap();
                    std::fs::write(folder.join(format!("nurbs-roundtrip-{index}.step")),recovered).unwrap();
                }
                let edge=body.edges.ids().next().unwrap();
                let curve=body.edges.get(edge).unwrap().curve;
                let Curve3::Nurbs { weights, .. }=body.curves3.get_mut(curve).unwrap() else { panic!() };
                weights[0]=0.5;
                assert_eq!(step_document(&body,solid).unwrap_err().kind,ValueRefusalKind::UnsupportedOwner);
                let Curve3::Nurbs { weights, .. }=body.curves3.get_mut(curve).unwrap() else { panic!() };
                weights[0]=1.0;
                body.edges.get_mut(edge).unwrap().range.0=0.125;
                assert_eq!(step_document(&body,solid).unwrap_err().kind,ValueRefusalKind::UnsupportedOwner);
                body.edges.get_mut(edge).unwrap().range=(2.0,5.0);
                let Curve3::Nurbs { knots, .. }=body.curves3.get_mut(curve).unwrap() else { panic!() };
                knots.knots.copy_from_slice(&[2.0,2.0,5.0,5.0]);
                assert!(decode_document(&step_document(&body,solid).unwrap()).is_err());

                body.edges.get_mut(edge).unwrap().range=(1.0,0.0);
                let Curve3::Nurbs { knots, controls, .. }=body.curves3.get_mut(curve).unwrap() else { panic!() };
                knots.knots.copy_from_slice(&[0.0,0.0,1.0,1.0]);
                controls.reverse();
                assert!(decode_document(&step_document(&body,solid).unwrap()).is_err());
                body.edges.get_mut(edge).unwrap().range=(0.0,1.0);
                let Curve3::Nurbs { controls, .. }=body.curves3.get_mut(curve).unwrap() else { panic!() };
                controls.reverse();

                let surface=body.faces.iter().nth(1).unwrap().1.surface;
                let Surface::Nurbs { u_knots, .. }=body.surfaces.get_mut(surface).unwrap() else { panic!() };
                for knot in &mut u_knots.knots { *knot+=1.0; }
                assert!(decode_document(&step_document(&body,solid).unwrap()).is_err());
                let edited=body.surfaces.get_mut(surface).unwrap();
                let original=std::mem::replace(edited,primitive.surface(1));
                assert!(decode_document(&step_document(&body,solid).unwrap()).is_err());
                *body.surfaces.get_mut(surface).unwrap()=original;
            }
            while !draft.terminal_is_empty() {
                let exact=quote(draft.close_demand(&body).unwrap());
                let (turn,_)=wallet.run_settled(exact,1,|effect| { let (result,born,free)=measured(||draft.close(&mut body,effect)); let receipt=result.as_ref().unwrap().progress(); physical((born,free),(receipt.retained_capacity_bytes,receipt.released_bytes)); RetainedWorkPoll::Complete(RetainedWorkOutcome { result,work:effect.performed_work() }) }).unwrap().unwrap();
                let receipt=turn.result.unwrap().progress();
                assert!(receipt.fits(exact));
                wallet.post(receipt).unwrap();
            }
            assert_eq!([body.vertices.len(),body.edges.len(),body.faces.len(),body.curves3.len(),body.surfaces.len()],[0;5]);
        }
    }
}
'''
step=Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📐️geometry")
source=source.replace("__FIXTURE__",fixture.resolve().as_posix()).replace("__WRITER__",(step/"📝️retained-write/🦀️.rs").resolve().as_posix()).replace("__READER__",(step/"📥️retained-read/🦀️.rs").resolve().as_posix())
live=(root/"🦀️.rs").read_text()
transaction=live[live.index("#[derive(Clone, Copy)]\nstruct TopologyOffsets"):]
start=transaction.index("        let provenance = match &self.boundary {")
end=transaction.index("        Ok((entity!",start)
analytic=next(line for line in transaction[start:end].splitlines() if "SolidBoundary::Analytic" in line)
transaction=transaction[:start]+"        let provenance = match &self.boundary {\n"+analytic+"\n        };\n"+transaction[end:]
imports="\n".join(l for l in live.splitlines()[:14] if l.startswith("use ") and "certified_boundaries" not in l and "representation::contract" not in l)
imports=imports.replace("circular_prism::CircularPrism,conical_frustum::ConicalFrustum,point_hull::HullBoundary, prism_boundary::PrismBoundary, ","")
preamble=r'''
const MAX_ORTHOGONAL_ENTITIES:usize=2048;
#[path = "__SPLINE__"]
mod nurbs_cuboid;
use nurbs_cuboid::{Payload as SplinePayload, Recipe as SplineRecipe};
#[derive(Clone, Copy)]
pub enum SolidProvenance { Primitive(PrimitiveProvenance) }
pub enum SolidBoundary { Analytic(SolidPrimitive) }
impl SolidBoundary {
    fn primitive(&self)->SolidPrimitive { let Self::Analytic(p)=self; *p }
    fn vertex_count(&self)->usize { self.primitive().vertex_count() }
    fn edge_count(&self)->usize { self.primitive().edge_count() }
    fn face_count(&self)->usize { self.ring_sizes().len() }
    fn ring_sizes(&self)->&[usize] { self.primitive().ring_sizes() }
    fn shell_count(&self)->usize { 1 }
    fn ring_owner(&self, ring:usize)->usize { ring }
    fn inner_ring(&self, _:usize)->Option<usize> { None }
    fn face_flipped(&self, _:usize)->bool { false }
    fn shell_face_count(&self, _:usize)->usize { self.face_count() }
    fn vertex(&self, i:usize)->Pnt3 { self.primitive().vertex(i) }
    fn edge(&self, i:usize)->PrimitiveEdge { self.primitive().edge(i) }
    fn surface(&self, i:usize)->Surface { self.primitive().surface(i) }
    fn trim(&self, i:usize, member:usize)->PrimitiveTrim { self.primitive().trim(i,member) }
}
'''.replace("__SPLINE__",(root/"〰️nurbs-cuboid/🦀️.rs").resolve().as_posix())
proof=live[live.index("#[derive(Clone, Copy)]\npub struct PrimitiveProvenance"):live.index("semio_framework_value::artifact_retire_leaf!(PrimitiveProvenance)")]
transaction=imports+"\n"+preamble+proof+transaction
dependencies=["semio_framework_3d_brep_"+name for name in ["construction","curve","representation","surface","tolerance","vector"]]+["semio_framework_value","serde_json"]
profiles=Path(".cache/b/r/m/u/l").glob("*/prototype/build")
available=[]
for profile in profiles:
    directories=tuple(profile.glob("*/*/out"))
    libraries={name:[lib for directory in directories for lib in directory.glob("lib"+name+"-*.rlib")]
               for name in dependencies+["semio_framework_trace"]}
    if all(libraries[name] for name in dependencies):
        available.append((libraries["semio_framework_value"][0].stat().st_mtime,directories,libraries))
if not available:
    raise SystemExit("coherent cached BRep dependency set unavailable")
_,cache,libraries=max(available,key=lambda row:row[0])
generated=ticket/"🗑️generated"
generated.mkdir(exist_ok=True)
with TemporaryDirectory(dir=generated) as scratch:
    file=Path(scratch)/"nurbs.rs"
    transaction_path=Path(scratch)/"transaction.rs"
    transaction_path.write_text(transaction,encoding="utf-8")
    file.write_text(source.replace("__TRANSACTION__",transaction_path.resolve().as_posix()),encoding="utf-8")
    binary=Path(scratch)/("nurbs.exe" if os.name=="nt" else "nurbs")
    args=["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20","rustc","--edition=2021","-A","dead_code","-A","irrefutable_let_patterns","-C","opt-level=1","-C","strip=symbols"]
    for directory in cache:
        args += ["-L","dependency="+str(directory)]
    if libraries["semio_framework_trace"]:
        dependencies.append("semio_framework_trace")
        args += ["--cfg","heap_witness"]
    for name in dependencies:
        library=max(libraries[name],key=lambda lib:lib.stat().st_size)
        args += ["--extern",name+"="+str(library)]
        if library.with_suffix(".rmeta").is_file():
            args += ["--extern",name+"="+str(library.with_suffix(".rmeta"))]
    if subprocess.run(args+[str(file),"-o",str(binary)]).returncode:
        raise SystemExit("could not compile cuboid transaction witness")
    subprocess.run(["repo","tools","exec","--light","bun","--",str(ticket/"🧮️nurbs-cuboid-oracle.mjs"),str(scratch)],check=True)
    if subprocess.run([str(binary),str(scratch)]).returncode:
        raise SystemExit("cuboid transaction witness failed")
    if "--occt" in sys.argv:
        with urllib.request.urlopen("https://registry.npmjs.org/occt-import-js/-/occt-import-js-0.0.23.tgz",timeout=20) as response:
            package=response.read(12*1024*1024)
        digest=base64.b64encode(hashlib.sha512(package).digest()).decode("ascii")
        assert digest=="RFfYQXYFX5C1mB1Aywm0ShcUKzXOr/VzTnlzhBSDJOR6YCAPt1HYCzeXWg1vwwjn/cUxwqRNhhtf1dlewoZYCQ=="
        with tarfile.open(fileobj=io.BytesIO(package),mode="r:gz") as files:
            for name in ["occt-import-js.js","occt-import-js.wasm"]:
                (Path(scratch)/name.replace(".js",".cjs")).write_bytes(files.extractfile("package/dist/"+name).read())
        subprocess.run(["repo","tools","exec","--light","node","--",str(ticket/"🟨️step-export-occt.mjs"),str(scratch),str(fixture)],check=True)
        subprocess.run(["repo","tools","exec","--light","node","--",str(ticket/"🟨️step-export-occt.mjs"),str(scratch),str(fixture),"roundtrip"],check=True)
        subprocess.run(["repo","tools","exec","--light","node","--",str(ticket/"🟨️step-export-occt.mjs"),str(scratch),str(fixture),"external"],check=True)
