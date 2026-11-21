use semio_framework_3d_brep_vector::Pnt3;
use std::collections::BTreeSet;

type SolidId = usize;
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct VertexId(usize);
impl VertexId { fn raw_index(self) -> usize { self.0 } }
#[derive(Clone, Copy)]
enum EntityRef { Solid(SolidId) }
type Scope = BTreeSet<VertexId>;
struct Reach { vertices: Scope }

struct Vertex { position: Pnt3, label: usize }
struct Vertices(Vec<Vertex>);
impl Vertices {
    fn iter(&self) -> impl Iterator<Item=(VertexId,&Vertex)> { self.0.iter().enumerate().map(|(id, vertex)|(VertexId(id),vertex)) }
    fn get(&self, id: VertexId) -> Option<&Vertex> { self.0.get(id.0) }
    fn get_mut(&mut self, id: VertexId) -> Option<&mut Vertex> { self.0.get_mut(id.0) }
}

struct Solids(Vec<Scope>);
impl Solids {
    fn iter(&self) -> impl Iterator<Item=(usize,&Scope)> { self.0.iter().enumerate() }
    fn get(&self,id:usize) -> Option<&Scope> { self.0.get(id) }
}
struct Body { vertices: Vertices, solids: Solids, edges: Vec<(usize,usize)> }
impl Body {
    fn add_box(&mut self, [width,depth,height]: [f64;3]) -> SolidId {
        let mut scope=Scope::new();
        for x in [0.,width] {
            for y in [0.,depth] {
                for z in [0.,height] {
                    let id=self.vertices.0.len();
                    self.vertices.0.push(Vertex{position:Pnt3::new(x,y,z),label:id});
                    scope.insert(VertexId(id));
                }
            }
        }
        let id=self.solids.0.len();
        self.solids.0.push(scope);
        id
    }
    fn reachable_from(&self, roots:&[EntityRef]) -> Reach {
        let vertices=roots.iter().flat_map(|EntityRef::Solid(id)| self.solids.0[*id].iter().copied()).collect();
        Reach { vertices }
    }
    fn edge_coedges(&self,_: &usize) -> Vec<usize> { Vec::new() }
    fn coedge_endpoints(&self,_:usize) -> Option<(VertexId,VertexId)> { None }
}

#[derive(Debug)]
enum KernelError { Operation(String) }
struct OpRecorder(Vec<usize>);
impl OpRecorder { fn record_modified(&mut self, label:usize) { self.0.push(label); } }
fn solid_exists(body:&Body,solid:SolidId) -> Result<(),KernelError> {
    body.solids.get(solid).map(|_|()).ok_or_else(||KernelError::Operation("missing solid".into()))
}

struct Issue { entity: String }
fn validate_body(body:&Body) -> Vec<Issue> {
    let mut issues=Vec::new();
    for (index,scope) in body.solids.iter() {
        let points=scope.iter().map(|&id|body.vertices.get(id).unwrap().position).collect::<Vec<_>>();
        if points.iter().enumerate().any(|(i, point)|points[i+1..].contains(point)) {
            issues.push(Issue{entity:format!("solid-{index}")});
        }
    }
    issues
}
fn issue_reaches(reach:&Reach,label:&str) -> bool {
    let id=label.strip_prefix("solid-").unwrap().parse::<usize>().unwrap();
    reach.vertices.iter().any(|vertex| vertex.0 / 8 == id)
}

fn main() {
    let fixture: serde_json::Value=serde_json::from_str(include_str!(FIXTURE)).unwrap();
    let mut body=Body{vertices:Vertices(Vec::new()),solids:Solids(Vec::new()),edges:Vec::new()};
    let selected=body.add_box(serde_json::from_value(fixture["selectedSize"].clone()).unwrap());
    let other=body.add_box(serde_json::from_value(fixture["otherSize"].clone()).unwrap());
    let mut recorder=OpRecorder(Vec::new());
    let previous=body.vertices.iter().map(|(id,v)|(id,v.position)).collect::<Vec<_>>();
    let output=heal_solid(&mut body,selected,fixture["healingTolerance"].as_f64().unwrap(),&mut recorder);
    let other_scope=body.reachable_from(&[EntityRef::Solid(other)]);
    assert!(previous.iter().filter(|(id,_)|other_scope.vertices.contains(id))
        .all(|&(id,p)|body.vertices.get(id).unwrap().position==p), "unselected vertices moved");
    assert_eq!(output.unwrap().total_repairs(),0);
    assert!(heal_solid(&mut body,selected,fixture["collapseTolerance"].as_f64().unwrap(),&mut recorder).is_err());
    assert!(previous.iter().all(|&(id,p)|body.vertices.get(id).unwrap().position==p), "refused repair altered geometry");
    assert!(recorder.0.is_empty(), "refused repair altered the history");
}
