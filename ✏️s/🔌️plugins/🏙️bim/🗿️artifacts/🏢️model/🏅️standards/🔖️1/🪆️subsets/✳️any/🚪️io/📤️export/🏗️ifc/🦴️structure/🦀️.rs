//! 🦴️ IFC4 structural analysis entities projected from the existing graph, with boundary conditions and load cases.
use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::io::export::ifc::context::Export;
use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::ifc::writer::{Ifc, Schema, V, text, unset, en, rf, refs, real, typed};
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::Point3;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::StructuralLocation;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::placement_of;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::TopConstraint;
use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::storey_levels::{StoreyLevel, vertical_of};
use std::collections::BTreeMap;
fn representation(ifc: &mut Ifc, points: &[Point3]) -> V { if points.is_empty(){return unset();}let ids:Vec<u64>=points.iter().map(|p|ifc.point3([p.x,p.y,p.z])).collect();let (item,kind)=if points.len()==1 {(ids[0],"Point")} else {(ifc.add("IFCPOLYLINE",vec![refs(&ids)]),"Curve3D")};let shape=ifc.shape(ifc.axis,"Reference",kind,&[item]);rf(ifc.add("IFCPRODUCTDEFINITIONSHAPE",vec![unset(),unset(),refs(&[shape])])) }
fn assigned(ifc:&mut Ifc,key:&str,group:u64,items:&[u64]) {if !items.is_empty(){ifc.rooted("IFCRELASSIGNSTOGROUP",key,"","",vec![refs(items),unset(),rf(group)]);} }

fn analysis_group(ifc: &mut Ifc, placement: u64, cases: &[u64]) -> u64 {
    ifc.rooted("IFCSTRUCTURALANALYSISMODEL", "structural-analysis", "Structural analysis", "s.bim.model.structural-solver@1",
        vec![unset(), en("LOADING_3D"), unset(), if cases.is_empty() { unset() } else { refs(cases) }, unset(), rf(placement)])
}

fn add_member(ifc: &mut Ifc, id: &str, kind: &str, points: &[Point3], surface: bool, placement: u64) -> u64 {
    let rep = representation(ifc, points);
    let (entity, tail) = if surface {
        ("IFCSTRUCTURALSURFACEMEMBER", vec![text(kind), rf(placement), rep, en("SHELL"), unset()])
    } else {
        let axis = ifc.dir3([0.0, 0.0, 1.0]);
        ("IFCSTRUCTURALCURVEMEMBER", vec![text(kind), rf(placement), rep, en("RIGID_JOINED_MEMBER"), rf(axis)])
    };
    ifc.rooted(entity, &format!("analytical/{id}"), id, id, tail)
}

/// 🪜️ Authored plumb columns contribute one IFC4 analytical axis per step.
#[derive(Default, semio_framework_value::RetireOwned)]
pub struct ColdColumns {
    next: usize,
    group: Option<(u64, u64)>,
    members: Vec<V>,
}

impl ColdColumns {
    /// 📐️ Emits one analytical axis and completes the group when its last column is written.
    pub fn step(&mut self, export: &mut Export<'_>, level: &dyn Fn(&str) -> Option<StoreyLevel>) -> bool {
        if export.schema() != Schema::Ifc4 {
            return true;
        }
        if let Some((id, column)) = export.model.columns.get_index(self.next) {
            self.next += 1;
            if let Some(own) = level(&column.storey) {
                let target = match &column.top {
                    TopConstraint::Storey { storey, .. } => level(storey),
                    _ => None,
                };
                let (base, top) = vertical_of(column.base_offset, &column.top, &own, target.as_ref());
                if top - base > 1e-9 {
                    let placement = placement_of(export.model, &column.storey, Some(&own));
                    let (sin, cos) = placement.rotation.sin_cos();
                    let x = placement.x + cos * column.position.x - sin * column.position.y;
                    let y = placement.y + sin * column.position.x + cos * column.position.y;
                    let path = [Point3 { x, y, z: base + placement.z }, Point3 { x, y, z: top + placement.z }];
                    let ifc = &mut export.ifc;
                    let (placement, _) = *self.group.get_or_insert_with(|| {
                        let placement = ifc.add("IFCLOCALPLACEMENT", vec![unset(), rf(ifc.origin)]);
                        (placement, analysis_group(ifc, placement, &[]))
                    });
                    self.members.push(rf(add_member(ifc, id, "column", &path, false, placement)));
                }
            }
        }
        if self.next < export.model.columns.len() {
            return false;
        }
        if let Some((_, group)) = self.group {
            export.ifc.rooted("IFCRELASSIGNSTOGROUP", "structural-members", "", "",
                vec![V::List(std::mem::take(&mut self.members)), unset(), rf(group)]);
        }
        true
    }
}

/// 🏗️ Writes structural groups only to IFC4; IFC2x3 reports the omitted authored boundary data.
pub fn emit(export:&mut Export<'_>) { let analysis=&export.inferred.structural_analysis; if export.schema()!=Schema::Ifc4 { if !export.model.supports.is_empty()||!export.model.loads.is_empty(){export.notes.push("Structural analysis hooks require IFC4.".into());} return; } if analysis.members.is_empty(){return;}
 let ifc=&mut export.ifc;let placement=ifc.add("IFCLOCALPLACEMENT",vec![unset(),rf(ifc.origin)]);let mut cases=BTreeMap::new();for(id,case)in &export.model.load_cases {let row=ifc.rooted("IFCSTRUCTURALLOADCASE",&format!("structural-case/{id}"),&case.name,id,vec![text(&case.category),en("LOAD_CASE"),en("USERDEFINED"),en("USERDEFINED"),real(case.factor),unset(),unset()]);cases.insert(id.clone(),row);}
 let loaded:Vec<u64>=cases.values().copied().collect();let group=analysis_group(ifc,placement,&loaded);let mut members=BTreeMap::new();let mut assigned_items=Vec::new();for(id,member)in &analysis.members {let surface=!member.boundary.is_empty();let row=add_member(ifc,id,&member.kind,if surface{&member.boundary}else{&member.path},surface,placement);members.insert(id.clone(),row);assigned_items.push(row);}
 for(id,support)in &analysis.supports {let r=&support.authored.restraints;let boolean=|v:bool|typed("IFCBOOLEAN",en(if v{"T"}else{"F"}));let condition=ifc.add("IFCBOUNDARYNODECONDITION",vec![text(&support.authored.name),boolean(r.x),boolean(r.y),boolean(r.z),boolean(r.rx),boolean(r.ry),boolean(r.rz)]);for(index,point)in support.points.iter().enumerate(){let rep=representation(ifc,&[*point]);let connection=ifc.rooted("IFCSTRUCTURALPOINTCONNECTION",&format!("support/{id}/{index}"),&support.authored.name,id,vec![unset(),rf(placement),rep,rf(condition),unset()]);assigned_items.push(connection);if let Some(member)=members.get(&support.authored.member){ifc.rooted("IFCRELCONNECTSSTRUCTURALMEMBER",&format!("support-link/{id}/{index}"),"","",vec![rf(*member),rf(connection),unset(),unset(),unset(),unset()]);}}if !matches!(support.authored.location,StructuralLocation::Point{..}){export.notes.push(format!("Distributed support {id} is represented by boundary point connections in IFC4; solver JSON retains the exact range."));}}
 let mut activities:BTreeMap<String,Vec<u64>>=BTreeMap::new();for(id,load)in &analysis.loads {let a=&load.authored;let point=matches!(a.location,StructuralLocation::Point{..});let line=matches!(a.location,StructuralLocation::Line{..});let entity=if point{"IFCSTRUCTURALLOADSINGLEFORCE"}else if line{"IFCSTRUCTURALLOADLINEARFORCE"}else{"IFCSTRUCTURALLOADPLANARFORCE"};let mut values=vec![text(&a.name),real(a.force.x),real(a.force.y),real(a.force.z)];if point||line{values.extend([real(a.moment.x),real(a.moment.y),real(a.moment.z)]);}else if a.moment.x!=0.0||a.moment.y!=0.0||a.moment.z!=0.0{export.notes.push(format!("Area load {id} moments are retained only in solver JSON."));}let applied=ifc.add(entity,values);let rep=representation(ifc,&load.points);let mut tail=vec![unset(),rf(placement),rep,rf(applied),en("GLOBAL_COORDS"),unset()];if !point{tail.extend([en("TRUE_LENGTH"),en("CONST")]);}let activity=ifc.rooted(if point{"IFCSTRUCTURALPOINTACTION"}else if line{"IFCSTRUCTURALCURVEACTION"}else{"IFCSTRUCTURALSURFACEACTION"},&format!("load/{id}"),&a.name,id,tail);activities.entry(a.load_case.clone()).or_default().push(activity);if let Some(member)=members.get(&a.member){ifc.rooted("IFCRELCONNECTSSTRUCTURALACTIVITY",&format!("load-link/{id}"),"","",vec![rf(*member),rf(activity)]);}}
 for(id,case)in cases {assigned(ifc,&format!("load-case-items/{id}"),case,activities.get(&id).map(Vec::as_slice).unwrap_or(&[]));}assigned(ifc,"structural-members",group,&assigned_items); export.notes.extend(analysis.findings.iter().cloned());
}
