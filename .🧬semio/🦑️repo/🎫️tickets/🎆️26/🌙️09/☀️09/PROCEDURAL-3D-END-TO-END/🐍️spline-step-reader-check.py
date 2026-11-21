"""🔎️ Run the current STEP scanner and spline proof without cached framework archives."""
from pathlib import Path
from tempfile import TemporaryDirectory
import json
import os
import subprocess

ticket=Path(__file__).parent
step=Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📐️geometry")
reader=(step/"📥️retained-read/🦀️.rs").read_text()
reader=reader.replace('#[path = "〰️spline-proof.rs"]','#[path = r"'+str((step/"📥️retained-read/〰️spline-proof.rs").resolve())+'"]')
reader=reader.replace('#[path = "🔺️prism-proof.rs"]\nmod prism_proof;',"""mod prism_proof {
    use super::*;
    pub(super) fn certify_prism(_: &[Face])->Result<StepSolid,ValueError>{ unreachable!() }
    pub(super) fn prism_recipe(_: [[f64;3];3],_:Vec3)->Option<crate::prism_boundary::PrismBoundary>{ unreachable!() }
}""")
reader=reader.replace('#[path = "🧊️orthogonal-proof.rs"]\nmod orthogonal_proof;',"""mod orthogonal_proof {
    use super::*;
    pub(super) struct OrthogonalProof;
    impl OrthogonalProof {
        pub(super) fn new()->Self {Self}
        pub(super) fn coordinates(&mut self,_:&Face)->Result<(),ValueError>{unreachable!()}
        pub(super) fn prepare(&mut self)->Result<(),ValueError>{unreachable!()}
        pub(super) fn face(&mut self,_:&Face)->Result<(),ValueError>{unreachable!()}
        pub(super) fn ray(&mut self)->Result<bool,ValueError>{unreachable!()}
        pub(super) fn finish(&mut self)->Result<StepSolid,ValueError>{unreachable!()}
    }
}""")
reader=reader.replace('#[path = "🛢️cylinder-proof.rs"]\nmod cylinder_proof;',"""mod cylinder_proof {
    use super::*;
    #[derive(Clone,Copy)]
    pub(super) enum CylinderFace {Empty}
    impl StepSolidDecode {
        pub(super) fn cylinder_face(&self,_:&str,_:u64)->Result<CylinderFace,ValueError>{ unreachable!() }
    }
    pub(super) fn certify_cylinder(_: &[CylinderFace;3])->Result<StepSolid,ValueError>{ unreachable!() }
}""")
reader=reader.replace('#[path = "🟠️rounded-proof.rs"]\nmod rounded_proof;',"""mod rounded_proof {
    use super::*;
    pub(super) struct RoundedProof;
    impl RoundedProof {
        pub(super) fn new()->Self {Self}
        pub(super) fn observe(&mut self,_:&StepSolidDecode,_:&str,_:u64)->Result<(),ValueError>{unreachable!()}
        pub(super) fn recipe(&self)->Result<Option<StepSolid>,ValueError>{unreachable!()}
        pub(super) fn verify(&mut self,_:&StepSolidDecode,_:&str,_:u64,_:crate::contract::RoundedBox)->Result<(),ValueError>{unreachable!()}
        pub(super) fn complete(&self)->Result<(),ValueError>{unreachable!()}
    }
    pub(super) fn recipe(_:[f64;3],_:[f64;3],_:f64)->Option<crate::contract::RoundedBox>{unreachable!()}
}""")
source=r"""
extern crate self as semio_framework_3d_brep_construction;
extern crate self as semio_framework_3d_brep_representation;
pub mod contract{pub struct RoundedBox;}
extern crate self as semio_framework_3d_brep_surface;
extern crate self as semio_framework_3d_brep_vector;
extern crate self as semio_framework_value;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point([f64;3]);
pub type Vec3=Point;
pub type Pnt3=Point;
impl Point {
    pub const ZERO:Self=Self([0.0;3]);
    pub fn from_array(c:[f64;3])->Self{ Self(c) }
    pub fn to_array(self)->[f64;3]{self.0}
    pub fn cross(self,other:Self)->Self{Self(std::array::from_fn(|i|self.0[(i+1)%3]*other.0[(i+2)%3]-self.0[(i+2)%3]*other.0[(i+1)%3]))}
}
impl std::ops::Neg for Point {
    type Output=Self;
    fn neg(self)->Self{Self(self.0.map(|v|-v))}
}
pub mod matrix {
    use super::Point;
    #[derive(Clone,Copy)]
    pub struct Frame3 {pub origin:Point,pub x:Point,pub y:Point,pub z:Point}
    impl Frame3 {pub fn from_x_z(_:Point,_:Point,_:Point)->Option<Self>{unreachable!()}}
}
pub enum Surface {Plane{frame:matrix::Frame3}}
impl Surface {
    pub fn eval(&self,u:f64,v:f64)->Point{
        let Self::Plane{frame}=self;
        Point(std::array::from_fn(|i|frame.origin.0[i]+frame.x.0[i]*u+frame.y.0[i]*v))
    }
}
pub mod primitives {
    use super::{Point,Surface,matrix::Frame3};
    pub mod prism_boundary {pub struct PrismBoundary;}
    #[derive(Clone,Copy)]
    pub enum SolidPrimitive {Box{origin:Point,width:f64,depth:f64,height:f64},Cylinder{frame:Frame3,radius:f64,height:f64}}
    pub struct Edge {pub vertices:[usize;2]}
    impl SolidPrimitive {
        pub fn vertex(self,i:usize)->Point{
            let Self::Box{origin,width,depth,height}=self else{unreachable!()};
            let [min,mid,max]=origin.0;
            Point([min+if matches!(i,1|2|5|6){width}else{0.},mid+if matches!(i,2|3|6|7){depth}else{0.},max+if i>=4{height}else{0.}])
        }
        pub fn edge(self,i:usize)->Edge{
            let vertices=[[0,1],[1,2],[2,3],[3,0],[4,5],[5,6],[6,7],[7,4],[0,4],[1,5],[2,6],[3,7]][i];
            Edge{vertices}
        }
        pub fn surface(self,i:usize)->Surface{
            let (corner,axis,sign)=[(0,2,-1.),(4,2,1.),(0,1,-1.),(3,1,1.),(0,0,-1.),(1,0,1.)][i];
            let unit=|axis,value|Point(std::array::from_fn(|i|if i==axis{value}else{0.}));
            let z=unit(axis,sign);
            let x=z.cross(unit(usize::from(axis==0),1.0));
            let y=z.cross(x);
            Surface::Plane{frame:Frame3{origin:self.vertex(corner),x,y,z}}
        }
    }
}
pub use primitives::prism_boundary;
#[derive(Default,Clone,Copy)]
pub struct RetainedCloneProgress {pub copied_items:usize,pub copied_bytes:usize,pub retained_capacity_bytes:usize,pub released_bytes:usize}
#[derive(Default,Clone,Copy)]
pub struct RetirementDemand{pub copy_bytes:usize,pub capacity_bytes:usize,pub release_bytes:usize,pub depth:usize}
#[derive(Debug,PartialEq)]
pub enum ValueRefusalKind{OwnershipLimit,InvalidValue,UnsupportedOwner}
#[derive(Debug)]
pub struct ValueError{pub kind:ValueRefusalKind,pub reason:&'static str}
impl ValueError {
    pub fn literal(kind:ValueRefusalKind,reason:&'static str)->Self{Self{kind,reason}}
    pub fn with_retained_progress(self,_:RetainedCloneProgress)->Self{self}
}
pub struct RetainedAllocationEffect<'a>(&'a RetirementDemand);
impl RetainedAllocationEffect<'_>{
    pub fn grant(&self)->&RetirementDemand{self.0}
    pub fn performed(&self,_:usize)->Result<bool,ValueError>{Ok(true)}
}
pub mod retained_clone {
    use super::RetirementDemand;
    pub fn retained_clone_grant_admits(grant:&RetirementDemand,demand:RetirementDemand)->bool{
        grant.depth>=demand.depth && grant.copy_bytes>=demand.copy_bytes && grant.release_bytes>=demand.release_bytes && grant.capacity_bytes>=demand.capacity_bytes
    }
}
#[path="reader.rs"]
mod decoder;

fn decode(text:&str)->Result<decoder::StepSolid,ValueError>{
    let mut job=decoder::StepSolidDecode::new(text);
    for _ in 0..10000 {
        if let Some(proof)=job.ready(){assert!(job.terminal_is_empty());return Ok(proof);}
        let quote=job.demand();
        job.step(text,&mut RetainedAllocationEffect(&quote))?;
    }
    panic!("STEP decoder failed to finish")
}

fn main(){
    let folder=std::env::args().nth(1).unwrap();
    let inputs=__CASES__;
    for (i,expected) in inputs.into_iter().enumerate(){
        let file=std::path::Path::new(&folder).join(format!("nurbs-external-{i}.step"));
        let text=std::fs::read_to_string(file).unwrap();
        assert_eq!(decode(&text).unwrap(),decoder::StepSolid::NurbsCuboid(expected));
        let analytic=std::path::Path::new(&folder).join(format!("analytic-external-{i}.step"));
        assert_eq!(decode(&std::fs::read_to_string(analytic).unwrap()).unwrap(),decoder::StepSolid::Box(expected));
        let changed=text.replacen("(2,2),(0.,1.)","(2,2),(0.,2.)",1);
        assert!(matches!(decode(&changed),Err(ValueError{kind:ValueRefusalKind::UnsupportedOwner,..})));
        let surface=text.lines().find(|line|line.contains("=B_SPLINE_SURFACE_WITH_KNOTS")).unwrap();
        let (body, suffix)=surface.rsplit_once("),.UNSPECIFIED.);").unwrap();
        let (body, span)=body.rsplit_once(",(").unwrap();
        let (a,b)=span.split_once(',').unwrap();
        let shifted=format!("{body},({},{}) , .UNSPECIFIED.);{suffix}",a.parse::<f64>().unwrap()+1., b.parse::<f64>().unwrap()+1.);
        let changed=text.replacen(surface,&shifted,1);
        assert_ne!(text,changed);
        assert!(matches!(decode(&changed),Err(ValueError{kind:ValueRefusalKind::UnsupportedOwner,..})));

        let curve=text.lines().find(|line|line.contains("=B_SPLINE_CURVE_WITH_KNOTS")).unwrap();
        let (head, rest)=curve.split_once("('',1,(").unwrap();
        let (controls, tail)=rest.split_once("),").unwrap();
        let (a,b)=controls.split_once(',').unwrap();
        let switched=format!("{head}('',1,({b},{a}),{tail}");
        let reference=head.split_once('=').unwrap().0;
        let connects=format!(",{reference},.T.);");
        let reverse=format!(",{reference},.F.);");
        let changed=text.replacen(curve,&switched,1).replacen(&connects,&reverse,1);
        assert!(matches!(decode(&changed),Err(ValueError{kind:ValueRefusalKind::UnsupportedOwner,..})));

        let (surface_ref,_)=surface.split_once('=').unwrap();
        let plane=format!("{surface_ref}=PLANE('',#16);");
        let changed=text.replacen(surface,&plane,1);
        assert!(matches!(decode(&changed),Err(ValueError{kind:ValueRefusalKind::UnsupportedOwner,..})));
    }
}
"""
fixture=Path("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🔣️nurbs-cuboid.json")
cases=json.loads(fixture.read_text())["cases"]
point=lambda xyz: "["+",".join(str(float(value)) for value in xyz)+"]"
source=source.replace("__CASES__","["+",".join("("+point(row["origin"])+","+point(row["size"])+")" for row in cases)+"]")
generated=ticket/"🗑️generated"
with TemporaryDirectory(dir=generated) as output:
    folder=Path(output)
    (folder/"reader.rs").write_text(reader,encoding="utf-8")
    frame=folder/"isolated-decoder.rs"
    frame.write_text(source,encoding="utf-8")
    binary=folder/("isolated-decoder.exe" if os.name=="nt" else "isolated-decoder")
    subprocess.run(["repo","tools","exec","--light","bun","--",str(ticket/"🧮️nurbs-cuboid-oracle.mjs"),str(folder)],check=True)
    subprocess.run(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20","rustc","--edition=2021","-A","dead_code",str(frame),"-o",str(binary)],check=True)
    subprocess.run([str(binary),str(folder)],check=True)
