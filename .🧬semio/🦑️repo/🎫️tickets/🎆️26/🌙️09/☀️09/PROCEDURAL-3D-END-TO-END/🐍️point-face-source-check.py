"""Compile the actual polygon publication owners against a minimal in-memory BRep contract."""
import contextlib
import io
import json
from pathlib import Path
import runpy

root = Path("🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🧠️native-registry")
output = io.StringIO()
with contextlib.redirect_stdout(output):
    runpy.run_path(str(Path(__file__).parent / "🐍️planar-preview-source-check.py"))
preamble = output.getvalue().split("type FaceId=", 1)[0].replace("#[derive(Clone,Copy)] struct Vec3", "#[derive(Clone,Copy,PartialEq)] struct Vec3")
preamble = preamble.replace("pub struct PersistentLabel", "#[derive(Clone,Copy,PartialEq)] pub struct PersistentLabel")
preamble = preamble.replace("struct RetirementDemand {copy_bytes:", "struct RetirementDemand {maximum_items:usize,maximum_copy_bytes:usize,release_bytes:usize,copy_bytes:")
preamble = preamble.replace("struct RetainedCloneProgress {copied_items:", "struct RetainedCloneProgress {released_bytes:usize,copied_items:")
preamble = preamble.replace("self.copied_bytes<=g.copy_bytes", "self.copied_items<=g.maximum_items&&self.released_bytes<=g.release_bytes&&self.copied_bytes<=g.copy_bytes")
preamble = preamble.replace("g.depth>=d.depth", "g.depth>=d.depth&&g.release_bytes>=d.release_bytes")
preamble = preamble.replace("pub mod history {", """pub mod history {
    pub struct LabelSource(u64);
    impl LabelSource {
       pub fn next(&self) -> u64 {self.0}
       pub fn from_next(n:u64)->Self{Self(n)}
    }
""")
preamble = preamble.replace("struct Pnt2(f64,f64);impl Pnt2{fn new(x:f64,y:f64)->Self{Self(x,y)}}", "#[derive(Clone,Copy)] struct Pnt2{x:f64,y:f64} impl Pnt2{fn new(x:f64,y:f64)->Self{Self{x,y}}}")
for name in ("a", "b", "c"):
    preamble = preamble.replace(f"{name}.0", f"{name}.x").replace(f"{name}.1", f"{name}.y")
print(preamble.replace("ValueRefusalKind{InvalidValue,", "ValueRefusalKind{InvalidValue,UnsupportedOwner,"))
print(r"""
extern crate self as semio_framework_3d_brep_curve;
extern crate self as semio_framework_3d_brep_construction;
extern crate self as semio_framework_3d_brep_spline_trim;
extern crate self as semio_framework_3d_brep_solid_transaction;
extern crate self as semio_framework_3d_brep_tolerance;
extern crate self as semio_framework_3d_brep_surface;
use circular_opening_offset::CircularOpeningOffset;
pub use quadratic_pcurve::QuadraticPcurve;
mod matrix{pub(super) use super::Frame3;}
use std::collections::BTreeMap;
use topology::history::LabelSource;
use polygon_face_publication::PolygonFacePublication;
use polygon_wire_publication::{HelixWireRecipe,PolygonWirePublication,PolygonWireRecipe};
type EVec3=[f64;3];
type NativeVec3=Vec3;
fn pnt(a:EVec3)->Pnt3{Pnt3::new(a[0],a[1],a[2])}
fn vec3(a:EVec3)->Vec3{pnt(a)}
fn evec(p:Pnt3)->EVec3{[p.x,p.y,p.z]}
impl Pnt3 {fn to_array(self)->EVec3{evec(self)} fn to_vec(self)->Vec3{self}}
#[derive(Clone,Copy)]struct Affine3{linear:[Vec3;3],delta:Vec3}
impl Affine3{
 fn translation(delta:Vec3)->Self{Self{linear:[Vec3::new(1.,0.,0.),Vec3::new(0.,1.,0.),Vec3::Z],delta}}
 fn determinant(self)->f64{self.linear[0].cross(self.linear[1]).dot(self.linear[2])}
 fn max_singular_value(self)->f64{self.linear.iter().map(|v|v.norm()).fold(0., f64::max)}
 fn apply_vector(self,v:Vec3)->Vec3{Vec3::new(self.linear[0].dot(v),self.linear[1].dot(v),self.linear[2].dot(v))}
 fn apply_point(self,p:Pnt3)->Pnt3{self.apply_vector(p)+self.delta}
}
#[derive(Clone,Copy)]struct AnalyticTransform(Affine3);
impl AnalyticTransform{fn map(self)->Result<Affine3,ValueError>{Ok(self.0)}}
impl Vec3{const Z:Self=Self::new(0.,0.,1.);}
impl std::ops::Add for Vec3 {
 type Output=Self;fn add(self,w:Self)->Self{Self::new(self.x+w.x,self.y+w.y,self.z+w.z)}
}
#[derive(Clone,Copy)]struct Frame3{origin:Pnt3,x:Vec3,y:Vec3,z:Vec3}
impl Frame3{
 fn from_normal(origin:Pnt3,normal:Vec3)->Option<Self>{
   let z=normal.normalized()?;
   let [ax,ay,az]=[z.x.abs(),z.y.abs(),z.z.abs()];
   let seed=if ax<=ay && ax<=az{Vec3::new(1.,0.,0.)}else if ay<=az{Vec3::new(0.,1.,0.)}else{Vec3::Z};
   let x=z.cross(seed).normalized()?;
   Some(Self{origin,x,y:z.cross(x),z})
 }
 fn to_local(self,p:Pnt3)->Pnt3{self.to_local_vector(p-self.origin)}
 fn to_local_vector(self,v:Vec3)->Vec3{Pnt3::new(v.dot(self.x),v.dot(self.y),v.dot(self.z))}
}
impl std::ops::Mul<f64> for Vec3 {
 type Output=Self;fn mul(self,a:f64)->Self{Self::new(self.x*a,self.y*a,self.z*a)}
}
type EdgeId=usize;type CoedgeId=usize;type LoopId=usize;type FaceId=usize;type VertexId=usize;type Curve3Id=usize;
mod arena{pub(super) use super::{CoedgeId,LoopId,Curve3Id};}
#[derive(Clone,Copy)]struct Tol(f64);
impl Tol{const DEFAULT:Self=Self(1e-7);fn new(value:f64)->Self{Self(value)}fn value(self)->f64{self.0}fn scaled(self,factor:f64)->Self{Self(self.0*factor)}}
struct Vertex{position:Pnt3,label:PersistentLabel,tol:Tol}
struct Edge{curve:usize,range:(f64,f64),v0:usize,v1:usize,tol:Tol,label:PersistentLabel}
struct Coedge{edge:usize,forward:bool,pcurve:Option<usize>,prange:(f64,f64),loop_id:usize,next:usize,prev:usize}
struct Face{surface:usize,outer:Option<usize>,inners:Vec<usize>,flipped:bool,tol:Tol,label:PersistentLabel}
struct Loop{first:usize,face:usize}
fn dummy_coedge()->usize{usize::MAX}
#[derive(Default)]struct Wire{members:Vec<(EdgeId,bool)>,vertices:Vec<VertexId>,closed:bool}
enum Curve3{Circle{frame:Frame3,radius:f64},Ellipse{frame:Frame3,major_radius:f64,minor_radius:f64},Line{origin:Pnt3,dir:Vec3},Nurbs{knots:bspline::KnotVector,controls:Vec<Pnt3>,weights:Vec<f64>}}
impl Curve3{fn eval(&self,t:f64)->Pnt3{
 match self{
 Self::Circle{frame,radius}=>frame.origin+frame.x*(t.cos()*radius)+frame.y*(t.sin()*radius),
 Self::Ellipse{frame,major_radius,minor_radius}=>frame.origin+frame.x*(t.cos()*major_radius)+frame.y*(t.sin()*minor_radius),
 Self::Line{origin,dir}=>*origin+*dir*t,Self::Nurbs{knots,controls,weights}=>{
  let s=(t-knots.knots[2])/(knots.knots[3]-knots.knots[2]);let a=[(1.-s)*(1.-s),2.*s*(1.-s),s*s];
  let w=(0..3).map(|i|a[i]*weights[i]).sum::<f64>();
  (0..3).fold(Pnt3::new(0.,0.,0.),|p,i|p+controls[i]*(a[i]*weights[i]/w))
 },_=>unreachable!()}
}}
mod bspline{#[derive(Clone)]pub struct KnotVector{pub knots:Vec<f64>,pub degree:usize}}
impl Default for Pnt2{fn default()->Self{Self::new(0.,0.)}}
impl Pnt2{fn dot(self,b:Self)->f64{self.x*b.x+self.y*b.y}}
impl std::ops::Sub for Pnt2{type Output=Self;fn sub(self,b:Self)->Self{Self::new(self.x-b.x,self.y-b.y)}}
type Vec2=Pnt2;
enum Curve2{Circle{center:Pnt2,radius:f64},Ellipse{center:Pnt2,x_axis:Vec2,major_radius:f64,minor_radius:f64},Line{origin:Pnt2,dir:Vec2},Nurbs{knots:bspline::KnotVector,controls:Vec<Pnt2>,weights:Vec<f64>}}
impl Curve2 { fn eval(&self,t:f64)->Pnt2 { match self {
 Self::Circle{center,radius}=>Pnt2::new(center.x+radius*t.cos(),center.y+radius*t.sin()),
 Self::Ellipse{center,x_axis,major_radius,minor_radius}=>Pnt2::new(
    center.x+x_axis.x*major_radius*t.cos()-x_axis.y*minor_radius*t.sin(),
    center.y+x_axis.y*major_radius*t.cos()+x_axis.x*minor_radius*t.sin()),
 Self::Line{origin,dir}=>Pnt2::new(origin.x+t*dir.x,origin.y+t*dir.y),
 Self::Nurbs{knots,controls,weights}=>{
   let s=(t-knots.knots[2])/(knots.knots[3]-knots.knots[2]);
   let values=[(1.-s)*(1.-s),2.*s*(1.-s),s*s];
   let weight=(0..3).map(|i|values[i]*weights[i]).sum::<f64>();
   let eval=|coordinate:fn(Pnt2)->f64|(0..3).map(|i|coordinate(controls[i])*values[i]*weights[i]/weight).sum::<f64>();
   Pnt2::new(eval(|q|q.x),eval(|q|q.y))
 },

 _=>unreachable!(),
} } }

enum Surface{Plane{frame:Frame3},Other}
#[derive(Clone,Copy)]enum EntityRef{Vertex(usize),Curve3(usize),Edge(usize),Surface(usize),Face(usize),Loop(usize),Curve2(usize),Coedge(usize)}
enum Entity{Wire(Wire,PersistentLabel),Face(usize)}
struct Arena<T>(Vec<Option<T>>);
impl<T> Default for Arena<T>{fn default()->Self{Self(vec![])}}
impl<T> Arena<T>{
 fn get(&self,id:usize)->Option<&T>{self.0.get(id)?.as_ref()}
 fn contains(&self,id:usize)->bool{self.get(id).is_some()}
 fn get_mut(&mut self,id:usize)->Option<&mut T>{self.0.get_mut(id)?.as_mut()}
 fn remove(&mut self,id:usize)->Option<T>{self.0.get_mut(id)?.take()}
 fn len(&self)->usize{self.0.iter().filter(|v|v.is_some()).count()}
 fn next_remove_depth_demand(&self,_:usize)->Result<usize,ValueError>{Ok(1)}
 fn next_insert_depth_demand(&self)->Result<usize,ValueError>{Ok(1)}
 fn next_insert_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
 fn reserve_insert_step(&mut self,_:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,RetainedCloneProgress)>{unreachable!()}
 fn insert_reserved(&mut self,v:T,_:RetainedCloneGrant)->Result<(usize,RetainedCloneProgress),(ValueError,T)>{
  let id=self.0.len();self.0.push(Some(v));Ok((id,item()))
 }
}
struct Body{
 labels:LabelSource,vertices:Arena<Vertex>,edges:Arena<Edge>,curves3:Arena<Curve3>,curves2:Arena<Curve2>,
 faces:Arena<Face>,loops:Arena<Loop>,coedges:Arena<Coedge>,surfaces:Arena<Surface>,
}
impl Default for Body{
 fn default()->Self{
  Self{labels:LabelSource::from_next(1),vertices:Default::default(),edges:Default::default(),curves3:Default::default(),
   curves2:Default::default(),faces:Default::default(),loops:Default::default(),coedges:Default::default(),surfaces:Default::default()}
 }
}
impl Body {
 fn new_label(&mut self)->PersistentLabel{let n=self.labels.next();self.labels=LabelSource::from_next(n+1);PersistentLabel(n)}
 fn counts(&self)->[usize;8]{[self.vertices.len(),self.edges.len(),self.curves3.len(),self.curves2.len(),self.faces.len(),self.loops.len(),self.coedges.len(),self.surfaces.len()]}
}
#[derive(Default)]struct Live(BTreeMap<String,Entity>);
impl Live{
 fn contains_key(&self,key:&str)->bool{self.0.contains_key(key)}
 fn next_insert_capacity_byte_demand(&self,_:&str,_:usize)->Result<usize,ValueError>{Ok(0)}
 fn next_insert_depth_demand(&self,_:&str)->Result<usize,ValueError>{Ok(1)}
 fn reserve_insert_step(&mut self,_:&str,_:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,RetainedCloneProgress)>{unreachable!()}
 fn insert_reserved(&mut self,key:String,v:Entity,_:RetainedCloneGrant)->Result<(Option<Entity>,RetainedCloneProgress),(ValueError,String,Entity)>{
  Ok((self.0.insert(key,v),item()))
 }
}
#[derive(Default)]struct BrepRegistry{live:Live,body:Body,unpublished_topology:usize,shape_mutation_revision:u64}
impl BrepRegistry {
 fn original_query_entity(&self,s:&str)->Result<&Entity,ValueError>{self.live.0.get(s).ok_or(ValueError("missing"))}
 fn tessellation_source_depth_demand(&self,_:&str)->Result<usize,ValueError>{Ok(1)}
 fn advance_live_revision(&mut self){self.shape_mutation_revision+=1;}
}
mod analytic_publication{
 pub(super) fn primitive_handle(prefix:&[u8],label:u64)->[u8;64]{
  let mut bytes=[b'0';64];bytes[0]=prefix[0];let label=format!("{label:016x}");bytes[48..].copy_from_slice(label.as_bytes());bytes
 }
}
fn item()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
struct RetainedAllocationEffect<'a>(&'a RetirementDemand);
impl RetainedAllocationEffect<'_>{
 fn grant(&self)->RetiredGrant{*self.0}
 fn performed(&mut self,n:usize)->Result<bool,ValueError>{Ok(self.0.maximum_items>=n)}
}
type RetiredGrant=RetainedCloneGrant;
enum RetainedCloneStep{Progress(RetainedCloneProgress),Complete(RetainedCloneProgress)}
impl RetainedCloneStep{
 fn receipt(self)->RetainedCloneProgress{match self{Self::Progress(r)|Self::Complete(r)=>r}}
}
mod solid_publication {
 use super::*;
 type Grant=RetainedCloneGrant;type Progress=RetainedCloneProgress;
 macro_rules! each_entity {($entity:ident,$act:ident)=>{match $entity{
    EntityRef::Vertex(id)=>$act!(vertices,id),EntityRef::Curve3(id)=>$act!(curves3,id),EntityRef::Edge(id)=>$act!(edges,id),
    EntityRef::Surface(id)=>$act!(surfaces,id),EntityRef::Face(id)=>$act!(faces,id),EntityRef::Loop(id)=>$act!(loops,id),
    EntityRef::Curve2(id)=>$act!(curves2,id),EntityRef::Coedge(id)=>$act!(coedges,id),
 }};}
""")
solid = (root / "🏗️solid-transaction/🦀️.rs").read_text()
store = solid[solid.index("pub fn store_demand"):solid.index("impl SolidPublication")]
print(store.replace("<T, Id: ArenaId>", "<T>").replace("Store<T, Id>", "Arena<T>").replace("Option<Id>", "Option<usize>"))
print(solid[solid.index("pub fn topology_slot_depth"):solid.index("impl SolidPublication",solid.index("pub fn topology_slot_depth"))])
print("}")
print("mod primitives{use super::*;")
construction = root.parent.parent / "🏗️operations/🏗️construction/🏛️primitives/🦀️.rs"
if not construction.is_file():
    construction = next(p for p in root.parent.parent.glob("**/🦀️.rs") if "pub fn planar_pcurve(" in p.read_text())
source = construction.read_text()
start = source.index("pub fn planar_pcurve(")
end = source.index("\n}", start) + 2
print(source[start:end])
print("}")
for name, path in [("circular_opening_offset", "🕳️circular-opening-offset"), ("conic_face_draft","🪷️conic-face-draft")]:
    source = (root / "🏗️solid-transaction" / path / "🦀️.rs").read_text().split("\n",2)[2]
    source = "\n".join(line for line in source.splitlines() if not line.startswith(("use semio_framework_3d_brep_representation", "use semio_framework_3d_brep_vector", "use semio_framework_3d_brep_curve", "use semio_framework_3d_brep_surface", "use semio_framework_3d_brep_tolerance")))
    source = source.replace("use super::{remove_topology_slot", "use super::solid_publication::{remove_topology_slot")
    if name=="conic_face_draft":
        arc = next(root.rglob("🪢️quadratic-arc/🦀️.rs")).read_text().split("\n",2)[2]
        arc = arc.replace("use semio_framework_3d_brep_curve::bspline::KnotVector;", "use crate::bspline::KnotVector;")
        arc = arc.replace("use semio_framework_value::ValueRefusalKind;", "").replace("ValueError::literal(ValueRefusalKind::AllocationFailed,", "ValueError(")
        source = source.replace("pub use semio_framework_3d_brep_spline_construction::quadratic_arc::SectionArc;",
            "mod quadratic_arc {use super::*;"+arc+"}\npub use quadratic_arc::SectionArc;")
        source = source.replace("use semio_framework_3d_brep_spline_construction::quadratic_arc::{self, ArcBuffers};",
            "use quadratic_arc::ArcBuffers;")

    print(f"mod {name} {{\nuse super::*;\n{source}\n}}")

bore = (root.parent.parent / "🛠️operations/🔀️boolean/🧱️certified-boundaries/🕳️bore-boundary/🦀️.rs").read_text()
bore = bore[bore.index("pub struct BoreBoundary"):]
rect = (root / "🏗️solid-transaction/📐️rectangular-extrusion/🦀️.rs").read_text()
rect = rect[rect.index("/// 📋️"):]
print(r"""
impl Frame3{
 const WORLD:Self=Self{origin:Pnt3::new(0.,0.,0.),x:Vec3::new(1.,0.,0.),y:Vec3::new(0.,1.,0.),z:Vec3::Z};
}
struct PrimitiveEdge{vertices:[usize;2],range:(f64,f64),curve:Curve3}
struct PrimitiveTrim{edge:usize,forward:bool,curve:Curve2,range:(f64,f64)}
#[derive(Clone,Copy)] enum SolidPrimitive{Box{origin:Pnt3,width:f64,depth:f64,height:f64},Cylinder{frame:Frame3,radius:f64,height:f64}}
impl SolidPrimitive{
 fn validate(&self)->Result<(),()> {Ok(())}
 fn volume(&self)->f64 {match *self{Self::Box{width,depth,height,..}=>width*depth*height,Self::Cylinder{radius,height,..}=>std::f64::consts::PI*radius*radius*height}}
 fn vertex(&self,_:usize)->Pnt3 {unreachable!()}
 fn edge(&self,_:usize)->PrimitiveEdge {unreachable!()}
 fn surface(&self,_:usize)->Surface {unreachable!()}
 fn trim(&self,_:usize,_:usize)->PrimitiveTrim {unreachable!()}
}
fn unsupported()->ValueError{ValueError("unsupported")}
""")
contract=(root.parent.parent/"📸️representation/🔖️contract/🦀️.rs").read_text()
print("#[derive(Clone,Copy)]"+contract[contract.index("pub struct BorePreviewSource"):contract.index("/// 📍️ Point classification")])
print("mod bored_stock{use super::*;use primitives::planar_pcurve;"+bore+"}")
print("use bored_stock::BoreBoundary;")
print("mod bored_rectangle{use super::*;use circular_opening_offset::opening_member;"+rect+"}")

orthogonal=root.parent.parent/"🛠️operations/🔀️boolean/🧱️certified-boundaries/🧱️orthogonal-boundary"
orth=(orthogonal/"🦀️.rs").read_text()
material=(orthogonal/"🧊️material-cells/🦀️.rs").read_text().split("\n",2)[2]
material="\n".join(line for line in material.splitlines() if not line.startswith("semio_framework_value::"))
orth=orth[orth.index("pub use material_cells"):].replace("pub use material_cells::OrthogonalCells;", "mod material_cells{"+material+"} use material_cells::OrthogonalCells;")
proof=(root/"🏗️solid-transaction/🪟️perforated-extrusion/🦀️.rs").read_text()
proof=proof[proof.index("fn unsupported()"):]
print(r"""
impl Vec3{const X:Self=Self::new(1.,0.,0.);const Y:Self=Self::new(0.,1.,0.);}
#[derive(Clone,Copy)] enum BooleanOp{Cut,Unite,Intersect}
fn empty_result()->ValueError{ValueError("empty")}
enum SolidBoundary{Bore(BoreBoundary),Orthogonal(OrthogonalBoundary)}
""")
print("mod orthogonal_fixture{use super::*;use primitives::planar_pcurve;"+orth+"}")
frames=(root.parent.parent/"📸️representation/➡️vector/🔢️matrix/🦀️.rs").read_text()
to_world=frames[frames.index("    pub fn to_world(&self, local: Pnt3)"):frames.index("    pub fn to_world_vector(")]
preserve=frames[frames.index("    pub fn preserves_orthogonal_geometry("):frames.index("    /// 🧭️ Maps every axis")]
print("impl Frame3{"+to_world+preserve+"}")

print("use orthogonal_fixture::OrthogonalBoundary;")
print("mod perforated_fixture{use super::*;use bored_rectangle::bored_rectangular_extrusion;"+proof+"}")
spline=(root.parent.parent/"💡️queries/🧩tessellation/🪡️spline-trim/🔳️quadratic-pcurve/🦀️.rs").read_text().split("\n",2)[2]
print("mod quadratic_pcurve{"+spline+"}")
topology = root.parent / "🏗️solid-topology"
opening_code = (topology / "🕳️polygon-opening/🦀️.rs").read_text().split("#[derive(Clone, Copy)]", 1)[1]
opening_code = opening_code.replace("Pnt3::from_array(", "pnt(")
opening_code = "\n".join(line for line in opening_code.splitlines() if "*effect.actual()" not in line and "if effect.actual().is_some()" not in line)
opening_code = opening_code.replace(" || effect.actual().is_some()", "")
section_contract = (root.parent.parent / "📸️representation/🔖️contract/✂️box-section/🦀️.rs").read_text()
section_polygon = section_contract[section_contract.index("const MAX_PROFILE"):section_contract.index("/// 🎟️ Intersects")]
print("mod polygon_opening { use super::*; use solid_publication::store_demand; use retained_clone::retained_clone_grant_admits; "
      "use std::mem::{size_of,take}; type Progress=RetainedCloneProgress;"
      + section_polygon + "#[derive(Clone, Copy)]" + opening_code + "}")
wire_recipe = (topology / "🛤️wire-recipe/🦀️.rs").read_text().split("/// 🌀️ Samples", 1)[1]
print("mod wire_recipe { use super::*; /// 🌀️ Samples" + wire_recipe + "}")
solid_mapping = (topology / "🔁️mapped-boundary/🦀️.rs").read_text().split("const MAXIMUM_EDGES", 1)[1]
print("mod solid_mapped_boundary { use super::*; use std::mem::size_of; const MAXIMUM_EDGES" + solid_mapping + "}")
suffix = "}"
for name,path in [("polygon_wire_publication","🛤️polygon-wire-publication"),("polygon_face_publication","📐️polygon-face-publication")]:
    source = (root / path / "🦀️.rs").read_text().split("\n",2)[2]
    source = source.replace("use semio_framework_3d_brep_representation::topology::{Edge, Vertex};", "")
    source = source.replace("use semio_framework_3d_brep_representation::topology::{Coedge, Face, Loop, dummy_coedge};", "")
    if name == "polygon_wire_publication":
        source = source.replace("pub use semio_framework_3d_brep_solid_topology::", "pub use crate::wire_recipe::")
    if name == "polygon_face_publication":
        source = source.replace("semio_framework_3d_brep_solid_topology::mapped_boundary::", "crate::solid_mapped_boundary::")
        source = source.replace("use semio_framework_3d_brep_solid_topology::", "use polygon_opening::")
        source = source.replace("use semio_framework_3d_brep_vector::Pnt2;", "").replace("use semio_framework_3d_brep_curve::Curve2;", "")
        submodule = (root / path / "🔁️mapped-boundary/🦀️.rs").read_text().split("\n",2)[2]
        submodule = submodule.replace("use semio_framework_3d_brep_representation::topology::Edge;", "")
        submodule = submodule.replace("semio_framework_3d_brep_solid_topology::mapped_boundary::", "crate::solid_mapped_boundary::")
        source = source.replace('#[path = "🔁️mapped-boundary/🦀️.rs"]\nmod mapped_boundary;', "mod mapped_boundary {\n" + submodule + "\n}")
        start=source.index("    /// 📦️ Recognizes a rectangular source")
        end=source.index("    /// 🗄️ Quotes the live index reservation",start)
        source=source[:start]+source[end:]
    print(f"mod {name} {{\n{source}\n{suffix}")

fixture = next(Path("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d").glob("**/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🏳️brep-surface/🔣️.json"))
spec = json.loads(fixture.read_text())
def points(name):
    case = next(row for row in spec["cases"] if row["name"] == name)
    return "[" + ",".join("["+",".join(str(float(v)) for v in point)+"]" for point in case["inputs"]["points"]) + "]"

print(r"""
fn advance(kernel:&mut BrepRegistry,job:&mut PolygonFacePublication,points:&[EVec3])->Result<(),ValueError>{
 let mut d=kernel.polygon_face_topology_demand(job,None)?;
 assert!(d.copy_bytes<=128);
 let point=job.next_point_index().map(|index|points[index]);
 let zero=kernel.step_polygon_face_topology(job,None,point,&mut RetainedAllocationEffect(&d))?;
 assert_eq!(zero.copied_items,0);
 d.maximum_items=1;
 let receipt=kernel.step_polygon_face_topology(job,None,point,&mut RetainedAllocationEffect(&d))?;
 assert_eq!(receipt.copied_items,1);assert!(receipt.fits(d));
 Ok(())
}
fn close(kernel:&mut BrepRegistry,job:&mut PolygonFacePublication){
 for _ in 0..200{
  if job.terminal_is_empty(){return;}
  let mut d=kernel.polygon_face_close_demand(job).unwrap();
  let zero=kernel.close_polygon_face(job,&mut RetainedAllocationEffect(&d)).unwrap().receipt();
  assert_eq!(zero.copied_items,0);
  d.maximum_items=1;
  let paid=kernel.close_polygon_face(job,&mut RetainedAllocationEffect(&d)).unwrap().receipt();
  assert_eq!(paid.copied_items,1);assert!(paid.fits(d));
 }
 panic!("polygon rollback stalled")
}
""")
print(f"const CONCAVE:[[f64;3];5]={points(spec['pointFacePreview']['name'])};")
print(f"const NONPLANAR:[[f64;3];4]={points('face from points that leave their plane')};")
print(r"""
#[test]fn successful_publication_transfers_only_face_topology(){
 let mut kernel=BrepRegistry::default();
 let mut job=kernel.plan_polygon_face_points(CONCAVE.len()).unwrap();
 for _ in 0..500{
  if job.topology_ready(){break;}
  assert!(kernel.live.0.is_empty());
  advance(&mut kernel,&mut job,&CONCAVE).unwrap();
 }
 assert!(job.topology_ready());assert_eq!(kernel.unpublished_topology,2);
 let handle=job.handle().to_owned();
 let grant=RetirementDemand{maximum_items:1,..Default::default()};
 assert!(kernel.publish_polygon_face(&mut job,handle.clone(),grant).is_ok());
 assert_eq!(kernel.unpublished_topology,0);
 assert_eq!(kernel.live.0.len(),1);
 let Entity::Face(id)=kernel.live.0[&handle] else{panic!("expected face")};
 let body=&kernel.body;
 let face=body.faces.get(id).unwrap();
 let lead=body.loops.get(face.outer.unwrap()).unwrap().first;
 let mut member=lead;
 for _ in &CONCAVE {
  let record=body.coedges.get(member).unwrap();
  assert!(record.pcurve.is_some());
  member=record.next;
 }
 assert_eq!(lead,member);
 close(&mut kernel,&mut job);
 assert_eq!(kernel.body.counts(),[5,5,5,5,1,1,5,1]);
}
#[test]fn polygon_openings_repeat_with_reciprocal_exact_trims_and_cancel_cleanly(){
 let outer=[[0.,0.,3.],[6.,0.,3.],[6.,6.,3.],[0.,6.,3.]];
 for cancel in [false,true]{
  let mut kernel=BrepRegistry::default();
  let mut plan=kernel.plan_polygon_face_points(outer.len()).unwrap();
  let square=|low:f64,high:f64|polygon_opening::SectionPolygon::ordered_quadrilateral(
    [[low,low,3.],[low,high,3.],[high,high,3.],[high,low,3.]].map(pnt));
  plan.opening=Some(polygon_face_publication::FaceOpening::Polygon(
    polygon_opening::PolygonOpening::new([None,Some(square(1.,2.)),None,Some(square(4.,5.))])));
  for _ in 0..500{
   if plan.topology_ready() || (cancel && kernel.body.loops.len()==2){break;}
   assert!(kernel.live.0.is_empty());
   advance(&mut kernel,&mut plan,&outer).unwrap();
  }
  if cancel{
   assert_eq!(kernel.body.loops.len(),2);
   close(&mut kernel,&mut plan);
   assert_eq!(kernel.body.counts(),[0;8]);
   assert_eq!(kernel.unpublished_topology,0);
   continue;
  }
  assert!(plan.topology_ready());
  let key=plan.handle().to_owned();
  let grant=RetirementDemand{maximum_items:1,..Default::default()};
  assert!(kernel.publish_polygon_face(&mut plan,key.clone(),grant).is_ok());
  close(&mut kernel,&mut plan);
  assert_eq!(kernel.body.counts(),[12,12,12,12,1,3,12,1]);
  let Entity::Face(id)=kernel.live.0[&key] else{panic!("section face")};
  let face=kernel.body.faces.get(id).unwrap();
  assert_eq!(face.inners.len(),2);
  let Surface::Plane{frame}=kernel.body.surfaces.get(face.surface).unwrap() else{panic!("section plane")};
  for &ring in &face.inners{
   let first=kernel.body.loops.get(ring).unwrap().first;
   let mut current=first;
   let mut twice_area=0.;
   for index in 0..4{
    let coedge=kernel.body.coedges.get(current).unwrap();
    let next=kernel.body.coedges.get(coedge.next).unwrap();
    assert_eq!(next.prev,current);
    assert!(coedge.forward);
    let edge=kernel.body.edges.get(coedge.edge).unwrap();
    let a=kernel.body.vertices.get(edge.v0).unwrap().position;
    let b=kernel.body.vertices.get(edge.v1).unwrap().position;
    let trim=kernel.body.curves2.get(coedge.pcurve.unwrap()).unwrap();
    for (position,parameter) in [(a,coedge.prange.0),(b,coedge.prange.1)]{
     let expected=frame.to_local(position);
     let actual=trim.eval(parameter);
     assert!((actual.x-expected.x).abs()<1e-12 && (actual.y-expected.y).abs()<1e-12);
    }
    twice_area+=a.x*b.y-a.y*b.x;
    current=coedge.next;
    assert_eq!(current==first,index==3);
   }
   assert_eq!(twice_area,-2.);
  }
 }
}
#[test]fn cancellation_and_invalid_boundary_remove_every_record(){
 for (points,cancel) in [(&CONCAVE[..],true),(&NONPLANAR[..],false)]{
  let mut kernel=BrepRegistry::default();
  let mut job=kernel.plan_polygon_face_points(points.len()).unwrap();
  let mut stopped=false;
  for _ in 0..500{
   assert!(kernel.live.0.is_empty());
   if cancel && kernel.body.coedges.len()>0 {
     assert!(kernel.body.edges.len()>0 && kernel.body.faces.len()>0);
     stopped=true;break;
   }
   if advance(&mut kernel,&mut job,points).is_err(){assert!(!cancel);stopped=true;break;}
  }
  assert!(stopped);
  close(&mut kernel,&mut job);
  assert_eq!(kernel.body.counts(),[0;8]);
  assert_eq!(kernel.unpublished_topology,0);
  assert!(kernel.live.0.is_empty());
 }
}
""")

row = spec["exactSplineFace"]
def array(xs):
    return '['+','.join(array(x) if isinstance(x,list) else str(float(x)) for x in xs)+']'

for label,key,shape in [('CONTROLS','controls','[[f64;3];3]'),('ENDS','ends','[[f64;3];2]'),
                        ('KNOTS','knots','[f64;6]'),('TRIM','trim','[f64;2]'),('WEIGHTS','weights','[f64;3]')]:
    print(f'const {label}:{shape}={array(row[key])};')

print(r"""
#[test]fn rational_face_transfers_exact_pcurve_and_rolls_back_dynamic_payload(){
 for (reverse,abort,bend) in [(false,0,false),(true,0,false),(false,1,false),(false,2,false),(false,0,true)] {
  let mut kernel=BrepRegistry::default();
  let points=[pnt(ENDS[0]),pnt(ENDS[1]),Pnt3::new(0.,0.,0.)];
  let vertices=points.map(|position| {
   let label=kernel.body.new_label();
   kernel.body.vertices.insert_reserved(Vertex{position,label,tol:Tol::DEFAULT},Default::default()).ok().unwrap().0
  });
  let mut authored=CONTROLS.map(pnt);
  if bend{authored[1].z=0.05;}
  let range=(TRIM[0],TRIM[1]);
  let edges=std::array::from_fn::<_,3,_>(|i|{
   let j=(i+1)%3;
   let curve=if i==0{Curve3::Nurbs{knots:bspline::KnotVector{knots:KNOTS.to_vec(),degree:2},controls:authored.to_vec(),weights:WEIGHTS.to_vec()}}
    else{Curve3::Line{origin:points[i],dir:points[j]-points[i]}};
   let curve=kernel.body.curves3.insert_reserved(curve,Default::default()).ok().unwrap().0;
   let label=kernel.body.new_label();
   kernel.body.edges.insert_reserved(Edge{curve,range:if i==0{range}else{(0.,1.)},v0:vertices[i],v1:vertices[j],tol:Tol::DEFAULT,label},Default::default()).ok().unwrap().0
  });
  let order=if reverse{[0,2,1]}else{[0,1,2]};
  let wire=Wire{members:order.map(|i|(edges[i],!reverse)).to_vec(),vertices:vertices.to_vec(),closed:true};
  let label=kernel.body.new_label();
  let source="wire";
  kernel.live.0.insert(source.to_owned(),Entity::Wire(wire,label));
  let mut job=kernel.plan_polygon_face(source,false).unwrap().unwrap();
  let mut stored=0;
  let mut rejected=false;
  for _ in 0..500 {
   if job.topology_ready(){break;}
   let mut d=kernel.polygon_face_topology_demand(&job,Some(source)).unwrap();
   assert!(d.copy_bytes<=128);
   let idle=kernel.step_polygon_face_topology(&mut job,Some(source),None,&mut RetainedAllocationEffect(&d)).unwrap();
   assert_eq!(idle.copied_items,0);
   d.maximum_items=1;
   let receipt=match kernel.step_polygon_face_topology(&mut job,Some(source),None,&mut RetainedAllocationEffect(&d)){
    Ok(r)=>r, Err(_)=>{rejected=true;break;}
   };
   assert!(receipt.fits(d));assert_eq!(receipt.copied_items,1);
   if receipt.retained_capacity_bytes==QuadraticPcurve::STORAGE{stored+=1;}
   if abort==1 && stored==1{break;}
   if abort==2 && kernel.body.curves2.0.iter().any(|c|matches!(c,Some(Curve2::Nurbs{..}))){break;}
  }
  assert_eq!(rejected,bend);
  assert_eq!(stored,usize::from(!bend));
  if abort==0 && !bend{
   assert!(job.topology_ready());
   let key=job.handle().to_owned();
   let grant=RetirementDemand{maximum_items:1,..Default::default()};
   assert!(kernel.publish_polygon_face(&mut job,key,grant).is_ok());
   let spline_use=kernel.body.coedges.0.iter().flatten().find(|u|u.edge==edges[0]).unwrap();
   assert_eq!(spline_use.forward,!reverse);assert_eq!(spline_use.prange, range);
   let Some(Curve2::Nurbs{knots,controls,weights})=kernel.body.curves2.get(spline_use.pcurve.unwrap())else{panic!("missing exact spline");};
   assert_eq!(knots.degree,2);assert_eq!(knots.knots,KNOTS);assert_eq!(weights, &WEIGHTS);
   let Some(Surface::Plane{frame})=kernel.body.surfaces.get(0)else{panic!("missing plane");};
   for (actual,source) in controls.iter().zip(authored){let p=frame.to_local(source);assert_eq!([actual.x,actual.y],[p.x,p.y]);}
  }
  let mut released_pcurve=false;
  for _ in 0..50{
   if job.terminal_is_empty(){break;}
   let mut d=kernel.polygon_face_close_demand(&job).unwrap();
   d.maximum_items=1;
   let receipt=kernel.close_polygon_face(&mut job,&mut RetainedAllocationEffect(&d)).unwrap().receipt();
   assert_eq!(receipt.released_bytes,d.release_bytes);
   if d.release_bytes==QuadraticPcurve::STORAGE{released_pcurve=true;}
  }
  assert!(job.terminal_is_empty());assert_eq!(released_pcurve,abort!=0);
  if abort!=0 || bend{assert_eq!(kernel.body.counts(),[3,3,3,0,0,0,0,0]);assert_eq!(kernel.live.0.len(),1);}
 }
}
""")

opening = spec["retainedBoreCapOffset"]
print(f'const STOCK:[f64;3]={array(opening["stock"])};')
print(f'const BORE_RADIUS:f64={float(opening["bore"]["radius"])};')
print(f'const BORE_ORIGIN:[f64;3]={array(opening["bore"]["origin"])};')
print(r"""
#[test] fn circular_opening_survives_offset_or_rolls_back(){
 for (normal,offset,cancel,overlap) in [(1.,0.5,false,false),(-1.,0.5,false,false),(1.,-0.25,false,false),(1.,0.5,true,false),(1.,0.5,false,true)] {
  let z=if normal>0.{STOCK[2]}else{0.};
  let mut square=[[0.,0.,z],[STOCK[0],0.,z],[STOCK[0],STOCK[1],z],[0.,STOCK[1],z]];
  if normal<0.{square.reverse();}
  let mut kernel=BrepRegistry::default();
  let mut input=kernel.plan_polygon_face_points(square.len()).unwrap();
  for _ in 0..250{if input.topology_ready(){break;}advance(&mut kernel,&mut input,&square).unwrap();}
  let grant=RetirementDemand{maximum_items:1,..Default::default()};
  let source=input.handle().to_owned();
  assert!(kernel.publish_polygon_face(&mut input,source.clone(),grant).is_ok());
  close(&mut kernel,&mut input);
  let Entity::Face(original)=kernel.live.0[&source] else{unreachable!()};
  let body=&mut kernel.body;
  let Some(Surface::Plane{frame:source_plane})=body.surfaces.get(body.faces.get(original).unwrap().surface)else{unreachable!()};
  let mut center=pnt(BORE_ORIGIN);
  center.z=z;
  if overlap{center.x=BORE_RADIUS/2.;}
  let frame=Frame3::from_normal(center,Vec3::Z).unwrap();
  let circle=Curve3::Circle{frame,radius:BORE_RADIUS};
  let position=circle.eval(0.);
  let trim=primitives::planar_pcurve(&circle,*source_plane);
  let curve=body.curves3.insert_reserved(circle,grant).ok().unwrap().0;
  let uv=body.curves2.insert_reserved(trim,grant).ok().unwrap().0;
  let label=body.new_label();
  let vertex=body.vertices.insert_reserved(Vertex{position,tol:Tol::DEFAULT,label},grant).ok().unwrap().0;
  let label=body.new_label();
  let range=(0.,std::f64::consts::TAU);
  let edge=body.edges.insert_reserved(Edge{curve,range,v0:vertex,v1:vertex,tol:Tol::DEFAULT,label},grant).ok().unwrap().0;
  let ring=body.loops.insert_reserved(Loop{face:original,first:dummy_coedge()},grant).ok().unwrap().0;
  let id=body.coedges.insert_reserved(Coedge{edge,prange:range,forward:normal<0.,pcurve:Some(uv),loop_id:ring,next:dummy_coedge(),prev:dummy_coedge()},grant).ok().unwrap().0;
  let row=body.coedges.get_mut(id).unwrap(); row.next=id;row.prev=id;
  body.loops.get_mut(ring).unwrap().first=id;
  body.faces.get_mut(original).unwrap().inners.push(ring);
  let baseline=body.counts();
  let mut job=kernel.plan_polygon_face_offset(&source,offset).unwrap().unwrap();
  let thickness=if offset<0.{-0.625}else{0.5};
  let bore=perforated_fixture::perforated_extrusion(&kernel.body, original,[0.,0.,normal*thickness]);
  if overlap{assert!(bore.is_err());}
  else{
   let SolidBoundary::Bore(recipe)=bore.unwrap()else{unreachable!()};
   let proof=recipe.preview_source(1);
   assert_eq!(proof.center,[center.x,center.y]);assert_eq!(proof.radius,BORE_RADIUS);
   assert_eq!(proof.origin,[0.,0.,z+(normal*thickness).min(0.)]);
   assert_eq!(proof.dimensions,[STOCK[0],STOCK[1],thickness.abs()]);
   assert!((proof.volume-(STOCK[0]*STOCK[1]-std::f64::consts::PI*BORE_RADIUS*BORE_RADIUS)*thickness.abs()).abs()<1e-12);
   assert_eq!(bored_rectangle::rectangular_face_depth(&kernel.body,original).unwrap(),1);
   assert!(bored_rectangle::bored_rectangular_extrusion(&kernel.body,original,[0.1,0.,thickness]).is_err());
  }
  let mut opening_bytes=0;
  let mut rejected=false;
  for _ in 0..1000 {
   if job.topology_ready(){break;}
   assert_eq!(kernel.live.0.len(),1);
   let mut d=kernel.polygon_face_topology_demand(&job,Some(&source)).unwrap();
   assert!(d.copy_bytes<=128);
   let idle=kernel.step_polygon_face_topology(&mut job,Some(&source),None,&mut RetainedAllocationEffect(&d)).unwrap();
   assert_eq!(idle.copied_items,0);
   d.maximum_items=1;
   let paid=match kernel.step_polygon_face_topology(&mut job,Some(&source),None,&mut RetainedAllocationEffect(&d)){
    Ok(paid)=>paid,Err(_)=>{rejected=true;break;}
   };
   assert!(paid.fits(d));assert_eq!(paid.copied_items,1);
   if paid.retained_capacity_bytes==std::mem::size_of::<LoopId>() && kernel.body.curves3.0.iter().flatten().filter(|c|matches!(c,Curve3::Circle{..})).count()==2{
    opening_bytes=paid.retained_capacity_bytes;
    if cancel{break;}
   }
  }
  assert_eq!(rejected,overlap);
  if cancel || overlap {
   let mut released_opening=0;
   for _ in 0..1000{
    if job.terminal_is_empty(){break;}
    let mut d=kernel.polygon_face_close_demand(&job).unwrap();
    let idle=kernel.close_polygon_face(&mut job,&mut RetainedAllocationEffect(&d)).unwrap().receipt();
    assert_eq!(idle.copied_items,0);
    d.maximum_items=1;
    let paid=kernel.close_polygon_face(&mut job,&mut RetainedAllocationEffect(&d)).unwrap().receipt();
    assert!(paid.fits(d));
    if d.release_bytes==std::mem::size_of::<LoopId>(){released_opening+=paid.released_bytes;}
   }
   assert!(job.terminal_is_empty());assert_eq!(kernel.body.counts(),baseline);assert_eq!(kernel.unpublished_topology,0);
   assert_eq!(released_opening,opening_bytes);
   assert_eq!(opening_bytes,if cancel{std::mem::size_of::<LoopId>()}else{0});
   assert_eq!(kernel.live.0.len(),1);
  } else {
   assert!(job.topology_ready());
   let key=job.handle().to_owned();
   assert!(kernel.publish_polygon_face(&mut job,key.clone(),grant).is_ok());
   close(&mut kernel,&mut job);
   let Entity::Face(target)=kernel.live.0[&key]else{unreachable!()};
   let body=&kernel.body;
   let face=body.faces.get(target).unwrap();
   let Some(Surface::Plane{frame:plane})=body.surfaces.get(face.surface)else{unreachable!()};
   assert_eq!(face.inners.len(),1);assert!((plane.origin.z-(z+normal*offset)).abs()<1e-12);
   let ring=body.loops.get(face.inners[0]).unwrap();
   let member=body.coedges.get(ring.first).unwrap();
   assert_eq!((member.prev,member.next),(ring.first,ring.first));
   let edge=body.edges.get(member.edge).unwrap();
   let Some(Curve3::Circle{frame,radius})=body.curves3.get(edge.curve)else{unreachable!()};
   assert_eq!(frame.origin.to_array(),[center.x,center.y,z+normal*offset]);
   assert_eq!(*radius,BORE_RADIUS);assert_eq!(member.forward,normal<0.);
   let Some(Curve2::Ellipse{center,x_axis,major_radius,minor_radius})=body.curves2.get(member.pcurve.unwrap())else{unreachable!()};
   assert_eq!((*major_radius,*minor_radius),(BORE_RADIUS,BORE_RADIUS*normal));
   for t in [0_f64,0.5,1.,2.,4.,6.]{
    let y_axis=Vec2::new(-x_axis.y,x_axis.x);
    let u=center.x+x_axis.x*BORE_RADIUS*t.cos()+y_axis.x*minor_radius*t.sin();
    let v=center.y+x_axis.y*BORE_RADIUS*t.cos()+y_axis.y*minor_radius*t.sin();
    let world=plane.origin+plane.x*u+plane.y*v;
    let p=body.curves3.get(edge.curve).unwrap().eval(t);
    assert!((world-p).norm()<1e-12);
   }
   assert_eq!(kernel.body.faces.get(original).unwrap().inners,[body.coedges.get(id).unwrap().loop_id]);
  }
 }
}
""")

print(r"""
#[test] fn conic_face_draft_retains_and_retires_exact_topology(){
 use conic_face_draft::{Conic,ConicBoundary,ConicFacePublication};
 let mut body=Body::default();
 let frame=Frame3::from_normal(pnt(BORE_ORIGIN),Vec3::Z).unwrap();
 let mut job=ConicFacePublication::new(1,frame,Conic::Circle(BORE_RADIUS),ConicBoundary::FULL,false,1,[b'f';64]);
 assert!(job.check_owner(1).is_ok()&&job.check_owner(2).is_err());
 for _ in 0..20{
  if job.topology_ready(){break;}
  let mut d=job.topology_demand(&body).unwrap();
  assert_eq!(job.step(&mut body,&mut RetainedAllocationEffect(&d)).unwrap().copied_items,0);
  d.maximum_items=1;
  assert!(job.step(&mut body,&mut RetainedAllocationEffect(&d)).unwrap().fits(d));
 }
 assert!(job.topology_ready());assert_eq!(body.counts(),[1;8]);
 for _ in 0..20{
  if job.terminal_is_empty(){break;}
  let mut d=job.close_demand(&body).unwrap();
  d.maximum_items=1;
  assert!(job.close(&mut body,&mut RetainedAllocationEffect(&d)).unwrap().receipt().fits(d));
 }
 assert!(job.terminal_is_empty());assert_eq!(body.counts(),[0;8]);
}
""")

rectangle=spec["retainedRectangularOpeningExtrusion"]
print(f'const RECTANGULAR_OUTER:[[f64;2];2]={array(rectangle["outer"])};')
print(f'const RECTANGULAR_OPENING:[[f64;2];2]={array(rectangle["opening"])};')
print(f'const ORTHOGONAL_CASES:[[f64;4];3]={array([[r["axis"],r["level"],r["delta"],r["volume"]] for r in rectangle["cases"]])};')
print(r"""
#[test] fn orthogonal_perforated_recipe_accepts_all_world_axes_and_builds_closed_volume(){
 for [axis,level,delta,volume] in ORTHOGONAL_CASES {
  let axis=axis as usize;
  let u=(axis+1)%3;
  let v=(axis+2)%3;
  let world=|a,b|{let mut p=[0.;3];p[axis]=level;p[u]=a;p[v]=b;p};
  let mut axis_vector=[0.;3];axis_vector[axis]=delta;
  let mut normal=[0.;3];normal[axis]=1.;
  let rectangle=|[[a,b],[c,d]]:[[f64;2];2]|[world(a,b),world(c,b),world(c,d),world(a,d)];
  let mut kernel=BrepRegistry::default();
  let outer=rectangle(RECTANGULAR_OUTER);
  let mut job=kernel.plan_polygon_face_points(4).unwrap();
  for _ in 0..256 {if job.topology_ready(){break;}advance(&mut kernel,&mut job,&outer).unwrap();}
  assert!(job.topology_ready());
  let id=kernel.body.faces.0.iter().position(Option::is_some).unwrap();
  let plane=kernel.body.faces.get(id).unwrap().surface;
  let Some(Surface::Plane{frame})=kernel.body.surfaces.get(plane)else{unreachable!()};
  assert_eq!(frame.z.to_array(),normal);
  let grant=RetirementDemand::default();
  let ring=kernel.body.loops.insert_reserved(Loop{face:id,first:dummy_coedge()},grant).ok().unwrap().0;
  let mut opening=rectangle(RECTANGULAR_OPENING);
  opening.reverse();
  let corners=opening.map(|p|{
   let label=kernel.body.new_label();
   kernel.body.vertices.insert_reserved(Vertex{position:pnt(p),label,tol:Tol::DEFAULT},grant).ok().unwrap().0
  });
  let members=std::array::from_fn::<_,4,_>(|i|{
   let [j,k]=[i,(i+1)%4];
   let origin=pnt(opening[j]);let dir=pnt(opening[k])-origin;
   let curve=kernel.body.curves3.insert_reserved(Curve3::Line{origin,dir},grant).ok().unwrap().0;
   let range=(0.,1.);
   let label=kernel.body.new_label();
   let edge=kernel.body.edges.insert_reserved(Edge{curve,range,v0:corners[j],v1:corners[k],tol:Tol::DEFAULT,label},grant).ok().unwrap().0;
   kernel.body.coedges.insert_reserved(Coedge{edge,forward:true,pcurve:None,prange:range,loop_id:ring,next:dummy_coedge(),prev:dummy_coedge()},grant).ok().unwrap().0
  });
  for i in 0..4{let use_=kernel.body.coedges.get_mut(members[i]).unwrap();use_.next=members[(i+1)%4];use_.prev=members[(i+3)%4];}
  kernel.body.loops.get_mut(ring).unwrap().first=members[0];
  kernel.body.faces.get_mut(id).unwrap().inners.push(ring);
  let SolidBoundary::Orthogonal(recipe)=perforated_fixture::perforated_extrusion(&kernel.body,id,axis_vector).unwrap()else{unreachable!()};
  assert_eq!(recipe.shell_sizes().len(),1);
  let mut use_count=vec![0_i32;recipe.edge_count()];
  let mut sum=0.;
  for (face,member_count) in recipe.ring_sizes().iter().copied().enumerate(){
   assert_eq!(member_count,4);
   let vertices=std::array::from_fn::<_,4,_>(|i| {
    let trim=recipe.trim(face,i);
    use_count[trim.edge]+=if trim.forward{1}else{-1};
    let edge=recipe.edge(trim.edge);
    recipe.vertex(edge.vertices[usize::from(!trim.forward)])
   });
   for i in [1,2]{
    sum+=vertices[0].dot(vertices[i].cross(vertices[i+1]))/6.;
   }
  }
  assert!(use_count.iter().all(|&n|n==0));
  assert!((sum-volume).abs()<1e-10);
  let [a,b]=[recipe.vertex(0).to_array()[axis],recipe.vertex(1).to_array()[axis]];
  assert!([a,b].into_iter().all(|z|z==level||z==level+delta));
  axis_vector[u]=1.;
  assert!(perforated_fixture::perforated_extrusion(&kernel.body,id,axis_vector).is_err());
  axis_vector[u]=0.;
  kernel.body.coedges.get_mut(members[0]).unwrap().forward=false;
  assert!(perforated_fixture::perforated_extrusion(&kernel.body,id,axis_vector).is_err());
  close(&mut kernel,&mut job);
 }
}
""")

intersection_fixture=next(Path("✏️s/🔌️plugins/🌀️procedural").glob("**/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/✂️brep-intersect/🔣️.json"))
circle_contacts=json.loads(intersection_fixture.read_text())["retainedConicPlane"][0]["points"]
print(f'const CONTACT_SEGMENT:[EVec3;2]={array(circle_contacts)};')
print(r"""
#[test]fn conic_contact_segment_uses_one_paid_topology_turn_and_retires_unpublished_records(){
 for cancel in [false,true]{
  let mut kernel=BrepRegistry::default();
  let mut job=kernel.plan_polygon_wire(PolygonWireRecipe::Contacts{points:[CONTACT_SEGMENT[0],CONTACT_SEGMENT[1],[0.;3],[0.;3]],count:2}).unwrap();
  for _ in 0..20{
   if job.topology_ready(){break;}
   assert!(kernel.live.0.is_empty());
   assert_eq!(job.next_point_index(),None);
   let mut d=kernel.polygon_wire_demand(&job,128).unwrap();
   assert_eq!(kernel.step_polygon_wire_topology(&mut job,None,&mut RetainedAllocationEffect(&d)).unwrap().copied_items,0);
   d.maximum_items=1;
   let effect=kernel.step_polygon_wire_topology(&mut job,None,&mut RetainedAllocationEffect(&d)).unwrap();
   assert_eq!(effect.copied_items,1);assert!(effect.fits(d));
  }
  assert!(job.topology_ready());assert_eq!(kernel.body.counts(),[2,1,1,0,0,0,0,0]);
  if cancel{
   for _ in 0..20{
    if job.terminal_is_empty(){break;}
    let mut d=kernel.polygon_wire_close_demand(&job).unwrap();
    assert_eq!(kernel.close_polygon_wire(&mut job,&mut RetainedAllocationEffect(&d)).unwrap().receipt().copied_items,0);
    d.maximum_items=1;
    assert!(kernel.close_polygon_wire(&mut job,&mut RetainedAllocationEffect(&d)).unwrap().receipt().fits(d));
   }
   assert!(job.terminal_is_empty());assert_eq!(kernel.body.counts(),[0;8]);assert!(kernel.live.0.is_empty());
  }else{
   let key=job.handle().to_owned();
   assert!(kernel.publish_polygon_wire(&mut job,key.clone(),RetirementDemand{maximum_items:1,..Default::default()}).is_ok());
   let Entity::Wire(ref wire,_)=kernel.live.0[&key]else{unreachable!()};
   assert!(!wire.closed);
   assert_eq!(wire.vertices.iter().map(|&id|kernel.body.vertices.get(id).unwrap().position.to_array()).collect::<Vec<_>>(),CONTACT_SEGMENT);
  }
  assert_eq!(kernel.unpublished_topology,0);
 }
}
""")
