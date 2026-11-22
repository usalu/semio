//! 🕳️ Minimal analytic arena for exercising the production through-bore verifier without the application graph.
extern crate self as semio_framework_3d_brep_construction;
extern crate self as semio_framework_3d_brep_curve;
extern crate self as semio_framework_3d_brep_representation;
extern crate self as semio_framework_3d_brep_tolerance;
extern crate self as semio_framework_3d_brep_vector;

use std::f64::consts::TAU;
use std::ops::{Add, Neg, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 { pub x:f64, pub y:f64, pub z:f64 }
pub type Pnt3 = Vec3;
impl Vec3 {
    pub const X:Self = Self::new(1.0,0.0,0.0);
    pub const Y:Self = Self::new(0.0,1.0,0.0);
    pub const Z:Self = Self::new(0.0,0.0,1.0);
    pub const fn new(x:f64,y:f64,z:f64)->Self { Self{x,y,z} }
    pub fn from_array([x,y,z]:[f64;3])->Self { Self::new(x,y,z) }
    pub fn to_array(self)->[f64;3] { [self.x,self.y,self.z] }
    pub fn dot(self,b:Self)->f64 { self.x*b.x+self.y*b.y+self.z*b.z }
    pub fn cross(self,b:Self)->Self { Self::new(self.y*b.z-self.z*b.y,self.z*b.x-self.x*b.z,self.x*b.y-self.y*b.x) }
    pub fn distance(self,b:Self)->f64 { let d=self-b; d.dot(d).sqrt() }
    fn scale(self,s:f64)->Self { Self::new(self.x*s,self.y*s,self.z*s) }
    fn unit(self)->Self { self.scale(self.dot(self).sqrt().recip()) }
}
impl Add for Vec3 { type Output=Self; fn add(self,b:Self)->Self { Self::new(self.x+b.x,self.y+b.y,self.z+b.z) } }
impl Sub for Vec3 { type Output=Self; fn sub(self,b:Self)->Self { self+(-b) } }
impl Neg for Vec3 { type Output=Self; fn neg(self)->Self { self.scale(-1.0) } }

#[derive(Clone, Copy)]
pub struct Pnt2 { pub x:f64, pub y:f64 }
pub mod matrix {
    use super::*;
    #[derive(Clone, Copy)]
    pub struct Frame3 { pub origin:Pnt3, pub x:Vec3, pub y:Vec3, pub z:Vec3 }
    impl Frame3 {
        pub const WORLD:Self = Self { origin:Pnt3::new(0.0,0.0,0.0),x:Vec3::X,y:Vec3::Y,z:Vec3::Z };
        pub fn map(self,u:f64,v:f64,w:f64)->Pnt3 { self.origin+self.x.scale(u)+self.y.scale(v)+self.z.scale(w) }
        pub fn uv(self,vector:Vec3)->Pnt2 { Pnt2{x:vector.dot(self.x),y:vector.dot(self.y)} }
    }
}
use matrix::Frame3;
use contract::BorePreviewSource;
pub struct Resolution { pub linear:f64 }
impl Resolution { pub const DEFAULT:Self=Self{linear:1e-7}; }
struct Tol;
impl Tol { const DEFAULT:Self=Self; fn value(self)->f64 { Resolution::DEFAULT.linear } }
type ValueError=();
fn unsupported()->ValueError {}

#[derive(Clone, Copy)]
pub enum Curve3 { Line { origin:Pnt3, dir:Vec3 }, Circle { frame:Frame3, radius:f64 } }
impl Curve3 {
    pub fn eval(self,t:f64)->Pnt3 {
        match self {
            Self::Line{origin,dir}=>origin+dir.scale(t),
            Self::Circle{frame,radius}=>frame.map(radius*t.cos(),radius*t.sin(),0.0),
        }
    }
}
#[derive(Clone, Copy)]
pub enum Curve2 {
    Line { origin:Pnt2, dir:Pnt2 },
    Circle { center:Pnt2, radius:f64 },
    Ellipse { center:Pnt2, x_axis:Pnt2, major:f64, minor:f64 },
}
impl Curve2 {
    pub fn eval(self,t:f64)->Pnt2 {
        match self {
            Self::Line{origin,dir}=>Pnt2{x:origin.x+dir.x*t,y:origin.y+dir.y*t},
            Self::Circle{center,radius}=>Pnt2{x:center.x+radius*t.cos(),y:center.y+radius*t.sin()},
            Self::Ellipse{center,x_axis,major,minor}=>Pnt2{
                x:center.x+x_axis.x*major*t.cos()-x_axis.y*minor*t.sin(),
                y:center.y+x_axis.y*major*t.cos()+x_axis.x*minor*t.sin(),
            },
        }
    }
}
#[derive(Clone, Copy)]
enum Surface { Plane { frame:Frame3 }, Cylinder { frame:Frame3, radius:f64 } }
impl Surface {
    fn eval(self,u:f64,v:f64)->Pnt3 {
        match self {
            Self::Plane{frame}=>frame.map(u,v,0.0),
            Self::Cylinder{frame,radius}=>frame.map(radius*u.cos(),radius*u.sin(),v),
        }
    }
}

pub mod arena { pub type FaceId=usize; pub type EdgeId=usize; pub type LoopId=usize; pub type VertexId=usize; }
use arena::FaceId;
struct Label(u64);
struct Face { surface:usize, flipped:bool, outer:Option<usize>, inners:Vec<usize>, label:Label }
struct Edge { curve:usize, range:(f64,f64), v0:usize, v1:usize }
struct Loop { first:usize, face:usize }
struct Coedge { edge:usize, forward:bool, pcurve:Option<usize>, prange:(f64,f64), loop_id:usize, next:usize, prev:usize }
struct Vertex { position:Pnt3 }
struct Shell { faces:Vec<FaceId> }
#[derive(Default)]
struct Body {
    faces:Vec<Face>, edges:Vec<Edge>, loops:Vec<Loop>, coedges:Vec<Coedge>, vertices:Vec<Vertex>,
    curves3:Vec<Curve3>, curves2:Vec<Curve2>, surfaces:Vec<Surface>,
}
impl Body {
    fn add_face(&mut self,surface:Surface,flipped:bool)->usize {
        let id=self.faces.len();
        self.faces.push(Face{surface:self.surfaces.len(),flipped,outer:None,inners:vec![],label:Label(0)});
        self.surfaces.push(surface);
        id
    }
    fn add_edge(&mut self,curve:Curve3,range:(f64,f64),[v0,v1]:[usize;2])->usize {
        let id=self.edges.len();
        self.edges.push(Edge{curve:self.curves3.len(),range,v0,v1});
        self.curves3.push(curve);
        id
    }
    fn add_ring(&mut self,face:usize,members:&[(usize,bool,Curve2)],inner:bool) {
        let ring=self.loops.len();
        let first=self.coedges.len();
        self.loops.push(Loop{first,face});
        if inner { self.faces[face].inners.push(ring); } else { self.faces[face].outer=Some(ring); }
        let n=members.len();
        for (i,&(edge,forward,pcurve)) in members.iter().enumerate() {
            self.coedges.push(Coedge{
                edge,forward,pcurve:Some(self.curves2.len()),prange:self.edges[edge].range,loop_id:ring,
                next:first+(i+1)%n,prev:first+(i+n-1)%n,
            });
            self.curves2.push(pcurve);
        }
    }
    fn planar_trim(&self,frame:Frame3,edge:usize)->Curve2 {
        match self.curves3[self.edges[edge].curve] {
            Curve3::Line{origin,dir}=>Curve2::Line{origin:frame.uv(origin-frame.origin),dir:frame.uv(dir)},
            Curve3::Circle{frame:circle,radius}=>Curve2::Ellipse{
                center:frame.uv(circle.origin-frame.origin),x_axis:frame.uv(circle.x),
                major:radius,minor:radius*circle.z.dot(frame.z),
            },
        }
    }
}

#[derive(Clone, Copy)]
enum SolidPrimitive {
    Box { origin:Pnt3, width:f64, depth:f64, height:f64 },
    Cylinder { frame:Frame3, radius:f64, height:f64 },
}
impl SolidPrimitive {
    fn validate(self)->Result<(),ValueError> { (self.volume().is_finite()&&self.volume()>0.0).then_some(()).ok_or(()) }
    fn volume(self)->f64 {
        match self { Self::Box{width,depth,height,..}=>width*depth*height,Self::Cylinder{radius,height,..}=>TAU*0.5*radius*radius*height }
    }
    fn vertex(self,i:usize)->Pnt3 {
        let Self::Box{origin,width,depth,height}=self else { unreachable!() };
        origin+Pnt3::new(if [1,2].contains(&(i%4)) {width} else {0.0},if i%4>=2 {depth} else {0.0},if i>=4 {height} else {0.0})
    }
}
struct BoreBoundary { box_shape:SolidPrimitive, cylinder:SolidPrimitive, bore_span:[f64;2], volume:f64 }
pub mod primitives {
    pub struct BoxGeometry;
    impl BoxGeometry { /* BOX_TOPOLOGY */ }
}
use primitives::BoxGeometry;

fn input(dimensions:[f64;3],profiles:&[([f64;2],f64)],span:Option<[f64;2]>) -> (Body,Shell,Vec<FaceId>) {
    let [width,depth,height]=dimensions;
    let stock=SolidPrimitive::Box{origin:Pnt3::new(0.0,0.0,0.0),width,depth,height};
    let mut body=Body::default();
    body.vertices=(0..8).map(|i| Vertex{position:stock.vertex(i)}).collect();
    for ends @ [a,b] in BoxGeometry::EDGES {
        let origin=stock.vertex(a);
        body.add_edge(Curve3::Line{origin,dir:stock.vertex(b)-origin},(0.0,1.0),ends);
    }
    let mut frames=Vec::new();
    for square in BoxGeometry::FACES {
        let direction=|(edge,forward):(usize,bool)| {
            let [a,b]=BoxGeometry::EDGES[edge];
            let dir=stock.vertex(b)-stock.vertex(a);
            if forward {dir} else {-dir}
        };
        let x=direction(square[0]).unit();
        let y=direction(square[1]).unit();
        let frame=Frame3{origin:stock.vertex(BoxGeometry::EDGES[square[0].0][0]),x,y,z:x.cross(y)};
        let face=body.add_face(Surface::Plane{frame},false);
        body.add_ring(face,&square.map(|(edge,forward)|(edge,forward,body.planar_trim(frame,edge))),false);
        frames.push(frame);
    }
    let mut walls=Vec::new();
    for &([x,y],radius) in profiles {
        let frame=|z|Frame3{origin:Pnt3::new(x,y,z),..Frame3::WORLD};
        let base=body.vertices.len();
        let [low,high]=span.unwrap_or([0.0,height]);
        let bore_height=high-low;
        let rings=[low,high].map(|z| {
            let id=body.vertices.len();
            body.vertices.push(Vertex{position:Pnt3::new(x+radius,y,z)});
            body.add_edge(Curve3::Circle{frame:frame(z),radius},(0.0,TAU),[id,id])
        });
        let seam=body.add_edge(Curve3::Line{origin:body.vertices[base].position,dir:Vec3::Z},(0.0,bore_height),[base,base+1]);
        let wall=body.add_face(Surface::Cylinder{frame:frame(low),radius},true);
        let line=|x,y,u,v|Curve2::Line{origin:Pnt2{x,y},dir:Pnt2{x:u,y:v}};
        body.add_ring(wall,&[(rings[0],true,line(0.0,0.0,1.0,0.0)),(seam,true,line(TAU,0.0,0.0,1.0)),
            (rings[1],false,line(0.0,bore_height,1.0,0.0)),(seam,false,line(0.0,0.0,0.0,1.0))],false);
        walls.push(wall);
        for (cap,edge) in rings.into_iter().enumerate() {
            let z = [low,high][cap];
            let outer_cap = z == [0.0,height][cap];
            let (face,frame) = if outer_cap {
                (cap,frames[cap])
            } else {
                let frame = if cap==1 { frame(z) } else { Frame3 { y:-Vec3::Y,z:-Vec3::Z,..frame(z) } };
                let floor = body.add_face(Surface::Plane{frame},true);
                walls.push(floor);
                (floor,frame)
            };
            body.add_ring(face,&[(edge,(cap==0)==outer_cap,body.planar_trim(frame,edge))],outer_cap);
        }
    }
    let shell=Shell{faces:(0..body.faces.len()).rev().collect()};
    (body,shell,walls)
}

fn restore(dimensions:[f64;3],tool_profiles:&[([f64;2],f64)]) {
    let (mut body,shell,mut walls)=input(dimensions,tool_profiles,None);
    body.faces[0].inners.reverse();
    let repair=|body:&Body,walls:&[usize]| proof::certify_bore_stock(body,&shell,walls);
    for _ in 0..2 {
        let Some(SolidPrimitive::Box{origin,width,depth,height})=repair(&body,&walls) else { panic!("stock certificate") };
        assert_eq!(origin.to_array(),[0.0;3]);
        assert_eq!([width,depth,height],dimensions);
        walls.reverse();
    }
    assert!(repair(&body,&[0]).is_none());
    if walls.len()==2 {
        assert!(repair(&body,&walls[..1]).is_none());
        assert!(repair(&body,&[walls[0];2]).is_none());
        assert!(repair(&body,&[walls[0],0]).is_none());
        body.faces[0].inners[1]=body.faces[0].inners[0];
        assert!(repair(&body,&walls).is_none());
    }
}

fn restore_published(dimensions:[f64;3], placement:[f64;3], radius:f64, tool_height:f64, top_open:bool, floor_z:Option<f64>) {
    let [width,depth,height]=dimensions;
    let stock=SolidPrimitive::Box{origin:Pnt3::new(0.0,0.0,0.0),width,depth,height};
    let frame=Frame3{origin:Pnt3::from_array(placement),..Frame3::WORLD};
    let tool=SolidPrimitive::Cylinder{frame,radius,height:tool_height};
    let certificate=BoreBoundary::new(stock,tool).unwrap().preview_source(71);
    let [x,y,_]=placement;
    let (mut body,mut shell,mut features)=input(dimensions,&[([x,y],radius)],Some(certificate.bore_span));
    assert!(certificate.is_open(top_open));
    if let Some(z)=floor_z {
        assert!(!certificate.is_open(!top_open));
        assert_eq!(certificate.bore_span[usize::from(!top_open)],z);
    }
    shell.faces.sort_unstable();
    for (index,&face) in shell.faces.iter().enumerate() {
        body.faces[face].label=Label(certificate.face_label(index));
    }
    let repair=|body:&Body, selected:&[FaceId]| published::published_bore_stock(body,&shell,certificate,selected);
    for _ in 0..2 {
        for stock in [repair(&body,&features),proof::certify_bore_stock(&body,&shell,&features)] {
            let Some(SolidPrimitive::Box{origin,width,depth,height})=stock else { panic!("published and imported bore stock") };
            assert_eq!(origin.to_array(),[0.0;3]);
            assert_eq!([width,depth,height],dimensions);
        }
        features.reverse();
    }
    assert!(repair(&body,&[0]).is_none());
    if features.len()==2 {
        assert!(repair(&body,&features[..1]).is_none());
        assert!(repair(&body,&features[1..]).is_none());
        assert!(repair(&body,&[features[0];2]).is_none());
        assert!(repair(&body,&[features[0],0]).is_none());
        let foreign=body.add_face(Surface::Plane{frame},false);
        assert!(repair(&body,&[features[0],foreign]).is_none());
        assert!(proof::certify_bore_stock(&body,&shell,&features[..1]).is_none());
        assert!(proof::certify_bore_stock(&body,&shell,&[features[0];2]).is_none());
        assert!(proof::certify_bore_stock(&body,&shell,&[features[0],foreign]).is_none());
        let floor = features[1];
        body.faces[floor].flipped=!body.faces[floor].flipped;
        assert!(proof::certify_bore_stock(&body,&shell,&features).is_none());
        body.faces[floor].label=Label(0);
        assert!(repair(&body,&features).is_none());
    }
}

fn main() { /* FIXTURE_CALLS */ }
