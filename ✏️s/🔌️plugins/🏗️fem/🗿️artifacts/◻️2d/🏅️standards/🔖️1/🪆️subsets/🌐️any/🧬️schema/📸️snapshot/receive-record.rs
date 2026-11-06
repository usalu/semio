//! 🫴️ Analytical records bind original fields into retained authoring slots before persistent row receiving.
use super::{Fem2dSnapshot, Fem2dSnapshotValue};
use crate::standards::v1::subsets::any::schema::structure::{FemCombination, FemCombinationTerm, FemDof, FemElement, FemLoad, FemLoadCase, FemRegion, FemSection, FemSupport};
use semio_framework_dsl_record::{
    FieldValue, RecordValue,
    receiving::{DslReceivingField,NativeSnapshotDecodeOwner,NativeSnapshotFailure,NativeSnapshotEffectProgress,NativeSnapshotDecodeWorkQuote,ReceivingWork,receiving_turn},
};
use semio_framework_value::{retirement::RetireOwned,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use semio_s_fem_material::planar::FemMaterialValue;
use semio_s_fem_node::planar::FemNodeValue;
use std::mem::size_of;
fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "FEM exchange record differs from its declared analytical shape")
}
fn record<'a>(value:&'a FieldValue,owner:&mut NativeSnapshotDecodeOwner<'_,'_>,progress:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,meter:&mut ReceivingWork)->Result<&'a RecordValue,NativeSnapshotFailure>{
 let mut result=None;let quote=NativeSnapshotDecodeWorkQuote::reserve(owner.native())?;let turn=quote.run(&mut progress.performed,unwind,&mut |performed|{*performed=true;let FieldValue::Record(record)=value else{return Err(invalid())};result=Some(record);Ok(())});meter.take(turn)?;progress.reset_completed()?;result.ok_or_else(||invalid().into())
}
use semio_s_fem_receiving::row;
use semio_s_fem_receiving::{declared,field,number,text};
fn rows<T:RetireOwned>(value:&FieldValue,output:&mut Option<Vec<T>>,progress:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>,mut receive:impl FnMut(&RecordValue,&mut Option<T>,&mut NativeSnapshotEffectProgress,&mut Option<NativeSnapshotFailure>,&mut NativeSnapshotDecodeOwner<'_,'_>,&mut ReceivingWork)->Result<(),NativeSnapshotFailure>,meter:&mut ReceivingWork)->Result<(),NativeSnapshotFailure>{
 lists(value,output,progress,unwind,owner,|value,output,progress,unwind,original,meter|receive(record(value,original,progress,unwind,meter)?,output,progress,unwind,original,meter),meter)
}
fn lists<T:RetireOwned>(value:&FieldValue,output:&mut Option<Vec<T>>,progress:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>,receive:impl FnMut(&FieldValue,&mut Option<T>,&mut NativeSnapshotEffectProgress,&mut Option<NativeSnapshotFailure>,&mut NativeSnapshotDecodeOwner<'_,'_>,&mut ReceivingWork)->Result<(),NativeSnapshotFailure>,meter:&mut ReceivingWork)->Result<(),NativeSnapshotFailure>{
 let mut values=None;let quote=NativeSnapshotDecodeWorkQuote::reserve(owner.native())?;let turn=quote.run(&mut progress.performed,unwind,&mut |performed|{*performed=true;let FieldValue::List(list)=value else{return Err(invalid())};values=Some(list);Ok(())});meter.take(turn)?;progress.reset_completed()?;semio_s_fem_receiving::collect(values.ok_or_else(invalid)?,output,owner,receive,meter)
}
fn variants<T:RetireOwned>(value:&FieldValue,output:&mut Option<Vec<T>>,progress:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>,mut receive:impl FnMut(&str,&RecordValue,&mut Option<T>,&mut NativeSnapshotEffectProgress,&mut Option<NativeSnapshotFailure>,&mut NativeSnapshotDecodeOwner<'_,'_>,&mut ReceivingWork)->Result<(),NativeSnapshotFailure>,meter:&mut ReceivingWork)->Result<(),NativeSnapshotFailure>{
 let mut values=None;let quote=NativeSnapshotDecodeWorkQuote::reserve(owner.native())?;let turn=quote.run(&mut progress.performed,unwind,&mut |performed|{*performed=true;let FieldValue::Block(inner)=value else{return Err(invalid())};let FieldValue::Statements(items)=inner.as_ref() else{return Err(invalid())};values=Some(items);Ok(())});meter.take(turn)?;progress.reset_completed()?;
 semio_s_fem_receiving::collect(values.ok_or_else(invalid)?,output,owner,|(kind,value),output,progress,unwind,original,meter|receive(kind,value,output,progress,unwind,original,meter),meter)
}
fn node(r: &RecordValue, output:&mut Option<FemNodeValue>, parent_progress:&mut NativeSnapshotEffectProgress, parent_unwind:&mut Option<NativeSnapshotFailure>, o: &mut NativeSnapshotDecodeOwner<'_, '_>,meter:&mut ReceivingWork) -> Result<(), NativeSnapshotFailure> {
    declared(r,3,o,parent_progress,parent_unwind,meter)?;
    row( r, output, o,
        || (FemNodeValue { id: String::new(), x: 0., y: 0. },None::<String>),
        |v,s,progress,unwind,r,o,meter| {
            text(r,0,&mut v.id,s,progress,unwind,o,meter)?;
            number(r,1,&mut v.x,progress,unwind,o,meter)?;
            number(r,2,&mut v.y,progress,unwind,o,meter)?;
            Ok(())
        },
    meter)
}
fn material(r: &RecordValue, output:&mut Option<FemMaterialValue>, parent_progress:&mut NativeSnapshotEffectProgress, parent_unwind:&mut Option<NativeSnapshotFailure>, o: &mut NativeSnapshotDecodeOwner<'_, '_>,meter:&mut ReceivingWork) -> Result<(), NativeSnapshotFailure> {
    declared(r,5,o,parent_progress,parent_unwind,meter)?;
    row( r, output, o,
        || (FemMaterialValue { id: String::new(), name: String::new(), e: 0., nu: 0., rho: 0. },None::<String>),
        |v,s,progress,unwind,r,o,meter| {
            text(r,0,&mut v.id,s,progress,unwind,o,meter)?;
            text(r,1,&mut v.name,s,progress,unwind,o,meter)?;
            number(r,2,&mut v.e,progress,unwind,o,meter)?;
            number(r,3,&mut v.nu,progress,unwind,o,meter)?;
            number(r,4,&mut v.rho,progress,unwind,o,meter)?;
            Ok(())
        },
    meter)
}
fn section(r: &RecordValue, output:&mut Option<FemSection>, parent_progress:&mut NativeSnapshotEffectProgress, parent_unwind:&mut Option<NativeSnapshotFailure>, o: &mut NativeSnapshotDecodeOwner<'_, '_>,meter:&mut ReceivingWork) -> Result<(), NativeSnapshotFailure> {
    declared(r,4,o,parent_progress,parent_unwind,meter)?;
    row( r, output, o,
        || (FemSection { id: String::new(), name: String::new(), area: 0., iy: 0. },None::<String>),
        |v,s,progress,unwind,r,o,meter| {
            text(r,0,&mut v.id,s,progress,unwind,o,meter)?;
            text(r,1,&mut v.name,s,progress,unwind,o,meter)?;
            number(r,2,&mut v.area,progress,unwind,o,meter)?;
            number(r,3,&mut v.iy,progress,unwind,o,meter)?;
            let quote=NativeSnapshotDecodeWorkQuote::reserve(o.native())?;let turn=quote.run(&mut progress.performed,unwind,&mut |performed|{let valid=[v.area, v.iy].iter().all(|value|*value>0.);*performed=true;if valid{Ok(())}else{Err(invalid())}});meter.take(turn)?;progress.reset_completed()?;
            Ok(())
        },
    meter)
}
fn support(r: &RecordValue, output:&mut Option<FemSupport>, parent_progress:&mut NativeSnapshotEffectProgress, parent_unwind:&mut Option<NativeSnapshotFailure>, o: &mut NativeSnapshotDecodeOwner<'_, '_>,meter:&mut ReceivingWork) -> Result<(), NativeSnapshotFailure> {
    declared(r,3,o,parent_progress,parent_unwind,meter)?;
    row( r, output, o,
        || (FemSupport { id: String::new(), node_id: String::new(), fixed: Vec::new() },(None::<String>,None::<Vec<FemDof>>)),
        |v,s,progress,unwind,r,o,meter| {
            text(r,0,&mut v.id,&mut s.0,progress,unwind,o,meter)?;
            text(r,1,&mut v.node_id,&mut s.0,progress,unwind,o,meter)?;
            semio_s_fem_receiving::enum_list(field(r,2)?,&mut s.1,&[FemDof::Tx,FemDof::Ty,FemDof::Tz,FemDof::Rx,FemDof::Ry,FemDof::Rz],progress,unwind,o,meter)?;
            semio_s_fem_receiving::move_vec(o,&mut v.fixed,&mut s.1,progress,unwind,meter)?;
            Ok(())
        },
    meter)
}
#[derive(semio_framework_value::RetireOwned)]
struct Member {
    scratch: Option<String>,
    progress:NativeSnapshotEffectProgress,
    unwind:Option<NativeSnapshotFailure>,
    id: String,
    start: String,
    end: String,
    material: String,
    section: String,
}
fn member(r: &RecordValue, output:&mut Option<FemElement>, parent_progress:&mut NativeSnapshotEffectProgress, parent_unwind:&mut Option<NativeSnapshotFailure>, o: &mut NativeSnapshotDecodeOwner<'_, '_>, beam: bool,meter:&mut ReceivingWork) -> Result<(), NativeSnapshotFailure> {
    declared(r,5,o,parent_progress,parent_unwind,meter)?;
    meter.take(o.receive_nested::<Member, FemElement,store::NativeSnapshotFailure>(output,&mut |slot, output, birth, birth_unwind, original| {receiving_turn(|meter|{
        let bytes=size_of::<Member>();let demand=semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()};
        let turn=original.effect(demand,birth,birth_unwind,&mut |performed,physical|{*slot=Some(Member { scratch:None,progress:Default::default(),unwind:None,id: String::new(), start: String::new(), end: String::new(), material: String::new(), section: String::new() });*performed=true;*physical=semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};Ok(())});meter.take(turn)?;birth.reset_completed()?;
        let v = slot.as_mut().unwrap();
        text(r,0,&mut v.id,&mut v.scratch,&mut v.progress,&mut v.unwind,original,meter)?;
        text(r,1,&mut v.start,&mut v.scratch,&mut v.progress,&mut v.unwind,original,meter)?;
        text(r,2,&mut v.end,&mut v.scratch,&mut v.progress,&mut v.unwind,original,meter)?;
        text(r,3,&mut v.material,&mut v.scratch,&mut v.progress,&mut v.unwind,original,meter)?;
        text(r,4,&mut v.section,&mut v.scratch,&mut v.progress,&mut v.unwind,original,meter)?;
        if slot.is_none()||output.is_some(){return Err(invalid().into())}let bytes=size_of::<u8>();let demand=semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()};
        let turn=original.effect(demand,birth,birth_unwind,&mut |performed,physical|{let v=slot.take().unwrap();*output=Some(if beam {
            FemElement::Beam { id: v.id, start: v.start, end: v.end, material_id: v.material, section_id: v.section }
        } else {
            FemElement::Bar { id: v.id, start: v.start, end: v.end, material_id: v.material, section_id: v.section }
        });*performed=true;*physical=semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};Ok(())});meter.take(turn)?;birth.reset_completed()?;Ok(())
    })}))
}
#[derive(semio_framework_value::RetireOwned)]
struct LoadFields {
    scratch: Option<String>,
    progress:NativeSnapshotEffectProgress,
    unwind:Option<NativeSnapshotFailure>,
    id: String,
    target: String,
    dof: FemDof,
    values: [f64; 2],
}
#[derive(Clone, Copy)]
enum LoadKind {
    Nodal,
    MemberUdl,
    Area,
}
fn load(kind: &str, r: &RecordValue, output:&mut Option<FemLoad>, parent_progress:&mut NativeSnapshotEffectProgress, parent_unwind:&mut Option<NativeSnapshotFailure>, o: &mut NativeSnapshotDecodeOwner<'_, '_>,meter:&mut ReceivingWork) -> Result<(), NativeSnapshotFailure> {
    let kind = match kind {
        "nodal" => LoadKind::Nodal,
        "member-udl" => LoadKind::MemberUdl,
        "area" => LoadKind::Area,
        _ => return Err(invalid().into()),
    };
    declared(r,if matches!(kind, LoadKind::Area) { 3 } else { 4 },o,parent_progress,parent_unwind,meter)?;
    meter.take(o.receive_nested::<LoadFields, FemLoad,store::NativeSnapshotFailure>(output,&mut |slot, output, birth, birth_unwind, original| {receiving_turn(|meter|{
        let bytes=size_of::<LoadFields>();let demand=semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()};
        let turn=original.effect(demand,birth,birth_unwind,&mut |performed,physical|{*slot=Some(LoadFields { scratch:None,progress:Default::default(),unwind:None,id: String::new(), target: String::new(), dof: FemDof::Tx, values: [0.; 2] });*performed=true;*physical=semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};Ok(())});meter.take(turn)?;birth.reset_completed()?;
        let v = slot.as_mut().unwrap();
        text(r,0,&mut v.id,&mut v.scratch,&mut v.progress,&mut v.unwind,original,meter)?;
        text(r,1,&mut v.target,&mut v.scratch,&mut v.progress,&mut v.unwind,original,meter)?;
        match kind {
            LoadKind::Nodal => {
                let mut dof=None;let quote=NativeSnapshotDecodeWorkQuote::reserve(original.native())?;let turn=quote.run(&mut v.progress.performed,&mut v.unwind,&mut |performed|{*performed=true;let FieldValue::Enum(index)=field(r,2)? else{return Err(invalid())};dof=Some(*[FemDof::Tx,FemDof::Ty,FemDof::Tz,FemDof::Rx,FemDof::Ry,FemDof::Rz].get(*index as usize).ok_or_else(invalid)?);Ok(())});meter.take(turn)?;v.progress.reset_completed()?;semio_s_fem_receiving::move_scalar(original,&mut v.dof,&mut dof,&mut v.progress,&mut v.unwind,meter)?;
                number(r,3,&mut v.values[0],&mut v.progress,&mut v.unwind,original,meter)?;
            }
            LoadKind::MemberUdl => {
                number(r,2,&mut v.values[0],&mut v.progress,&mut v.unwind,original,meter)?;
                number(r,3,&mut v.values[1],&mut v.progress,&mut v.unwind,original,meter)?;
            }
            LoadKind::Area => number(r,2,&mut v.values[0],&mut v.progress,&mut v.unwind,original,meter)?,
        }
        if slot.is_none()||output.is_some(){return Err(invalid().into())}let bytes=size_of::<u8>()+match kind{LoadKind::Nodal=>size_of::<FemDof>()+size_of::<f64>(),LoadKind::MemberUdl=>2*size_of::<f64>(),LoadKind::Area=>size_of::<f64>()};let demand=semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()};
        let turn=original.effect(demand,birth,birth_unwind,&mut |performed,physical|{let v=slot.take().unwrap();*output=Some(match kind {
            LoadKind::Nodal => FemLoad::Nodal { id: v.id, node_id: v.target, dof: v.dof, value: v.values[0] },
            LoadKind::MemberUdl => FemLoad::MemberUdl { id: v.id, element_id: v.target, wx: v.values[0], wy: v.values[1] },
            LoadKind::Area => FemLoad::Area { id: v.id, region_id: v.target, pressure: v.values[0] },
        });*performed=true;*physical=semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};Ok(())});meter.take(turn)?;birth.reset_completed()?;Ok(())
    })}))
}
fn load_case(r: &RecordValue, output:&mut Option<FemLoadCase>, parent_progress:&mut NativeSnapshotEffectProgress, parent_unwind:&mut Option<NativeSnapshotFailure>, o: &mut NativeSnapshotDecodeOwner<'_, '_>,meter:&mut ReceivingWork) -> Result<(), NativeSnapshotFailure> {
    declared(r,4,o,parent_progress,parent_unwind,meter)?;
    row(r, output, o, || (FemLoadCase { id: String::new(), name: String::new(), loads: Vec::new(), self_weight: false },(None::<String>,None::<Vec<FemLoad>>)), |v,s,progress,unwind,r,o,meter| {
        text(r,0,&mut v.id,&mut s.0,progress,unwind,o,meter)?;
        text(r,1,&mut v.name,&mut s.0,progress,unwind,o,meter)?;
        variants(field(r, 2)?,&mut s.1,progress,unwind,o, load,meter)?;semio_s_fem_receiving::move_vec(o,&mut v.loads,&mut s.1,progress,unwind,meter)?;
        let mut flag=None;meter.take(bool::receive_value(field(r, 3)?,&mut flag,o))?;semio_s_fem_receiving::move_scalar(o,&mut v.self_weight,&mut flag,progress,unwind,meter)?;
        Ok(())
    },meter)
}
fn point(value:&FieldValue,output:&mut Option<[f64;2]>,progress:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>,meter:&mut ReceivingWork)->Result<(),NativeSnapshotFailure>{
 let mut items=None;let quote=NativeSnapshotDecodeWorkQuote::reserve(owner.native())?;let turn=quote.run(&mut progress.performed,unwind,&mut |performed|{*performed=true;let FieldValue::Tuple(tuple)=value else{return Err(invalid())};if tuple.len()!=2{return Err(invalid())}items=Some(tuple);Ok(())});meter.take(turn)?;progress.reset_completed()?;
 let mut point=[0.;2];for(slot,item)in point.iter_mut().zip(items.ok_or_else(invalid)?){let mut received=None;meter.take(f64::receive_value(item,&mut received,owner))?;semio_s_fem_receiving::finite(owner,*received.as_ref().ok_or_else(invalid)?,progress,unwind,meter)?;semio_s_fem_receiving::move_scalar(owner,slot,&mut received,progress,unwind,meter)?;}
 semio_s_fem_receiving::publish_scalar(owner,output,point,progress,unwind,meter)
}

fn region(r: &RecordValue, output:&mut Option<FemRegion>, parent_progress:&mut NativeSnapshotEffectProgress, parent_unwind:&mut Option<NativeSnapshotFailure>, o: &mut NativeSnapshotDecodeOwner<'_, '_>,meter:&mut ReceivingWork) -> Result<(), NativeSnapshotFailure> {
    declared(r,7,o,parent_progress,parent_unwind,meter)?;
    row( r, output, o,
        || (FemRegion { id: String::new(), name: String::new(), outline: Vec::new(), holes: Vec::new(), thickness: 0., material_id: String::new(), mesh_size: 0. },(None::<String>,None::<Vec<[f64;2]>>,None::<Vec<Vec<[f64;2]>>>)),
        |v,s,progress,unwind,r,o,meter| {
            text(r,0,&mut v.id,&mut s.0,progress,unwind,o,meter)?;
            text(r,1,&mut v.name,&mut s.0,progress,unwind,o,meter)?;
            lists(field(r,2)?,&mut s.1,progress,unwind,o,point,meter)?;semio_s_fem_receiving::move_vec(o,&mut v.outline,&mut s.1,progress,unwind,meter)?;
            lists(field(r,3)?,&mut s.2,progress,unwind,o,|hole,output,progress,unwind,o,meter|lists(hole,output,progress,unwind,o,point,meter),meter)?;semio_s_fem_receiving::move_vec(o,&mut v.holes,&mut s.2,progress,unwind,meter)?;
            number(r,4,&mut v.thickness,progress,unwind,o,meter)?;
            text(r,5,&mut v.material_id,&mut s.0,progress,unwind,o,meter)?;
            number(r,6,&mut v.mesh_size,progress,unwind,o,meter)?;
            Ok(())
        },
    meter)
}
fn term(r: &RecordValue, output:&mut Option<FemCombinationTerm>, parent_progress:&mut NativeSnapshotEffectProgress, parent_unwind:&mut Option<NativeSnapshotFailure>, o: &mut NativeSnapshotDecodeOwner<'_, '_>,meter:&mut ReceivingWork) -> Result<(), NativeSnapshotFailure> {
    declared(r,2,o,parent_progress,parent_unwind,meter)?;
    row( r, output, o, || (FemCombinationTerm { case_id: String::new(), factor: 0. },None::<String>), |v,s,progress,unwind,r,o,meter| {
        text(r,0,&mut v.case_id,s,progress,unwind,o,meter)?;
        number(r,1,&mut v.factor,progress,unwind,o,meter)?;
        Ok(())
    },meter)
}
fn combination(r: &RecordValue, output:&mut Option<FemCombination>, parent_progress:&mut NativeSnapshotEffectProgress, parent_unwind:&mut Option<NativeSnapshotFailure>, o: &mut NativeSnapshotDecodeOwner<'_, '_>,meter:&mut ReceivingWork) -> Result<(), NativeSnapshotFailure> {
    declared(r,3,o,parent_progress,parent_unwind,meter)?;
    row(r, output, o, || (FemCombination { id: String::new(), name: String::new(), terms: Vec::new() },(None::<String>,None::<Vec<FemCombinationTerm>>)), |v,s,progress,unwind,r,o,meter| {
        text(r,0,&mut v.id,&mut s.0,progress,unwind,o,meter)?;
        text(r,1,&mut v.name,&mut s.0,progress,unwind,o,meter)?;
        rows(field(r,2)?,&mut s.1,progress,unwind,o,term,meter)?;semio_s_fem_receiving::move_vec(o,&mut v.terms,&mut s.1,progress,unwind,meter)?;
        Ok(())
    },meter)
}
#[derive(Default,semio_framework_value::RetireOwned)]
struct SnapshotFrame {
    progress:NativeSnapshotEffectProgress,
    unwind:Option<NativeSnapshotFailure>,
    value:Option<Fem2dSnapshotValue>,
    nodes:Option<Vec<FemNodeValue>>,
    regions:Option<Vec<FemRegion>>,
    materials:Option<Vec<FemMaterialValue>>,
    sections:Option<Vec<FemSection>>,
    supports:Option<Vec<FemSupport>>,
    load_cases:Option<Vec<FemLoadCase>>,
    combinations:Option<Vec<FemCombination>>,
    elements:Option<Vec<FemElement>>,
}
impl Fem2dSnapshot {
    /// 📑️ Receives analytical fields in a retained publication frame.
    pub fn receive_exchange_record(r:&RecordValue,output:&mut Option<Self>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{receiving_turn(|meter|{
        meter.take(owner.receive_nested::<SnapshotFrame,Self,NativeSnapshotFailure>(output,&mut |slot,output,birth,birth_unwind,original|receiving_turn(|meter|{
            let bytes=size_of::<SnapshotFrame>();let demand=semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()};let turn=original.effect(demand,birth,birth_unwind,&mut |performed,physical|{*slot=Some(SnapshotFrame{value:Some(Fem2dSnapshotValue::default()),..Default::default()});*performed=true;*physical=semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};Ok(())});meter.take(turn)?;birth.reset_completed()?;let frame=slot.as_mut().unwrap();declared(r,9,original,&mut frame.progress,&mut frame.unwind,meter)?;
            rows(field(r,0)?,&mut frame.nodes,&mut frame.progress,&mut frame.unwind,original,node,meter)?;
            semio_s_fem_receiving::move_vec(original,&mut frame.value.as_mut().unwrap().nodes,&mut frame.nodes,&mut frame.progress,&mut frame.unwind,meter)?;
            rows(field(r,2)?,&mut frame.regions,&mut frame.progress,&mut frame.unwind,original,region,meter)?;
            semio_s_fem_receiving::move_vec(original,&mut frame.value.as_mut().unwrap().regions,&mut frame.regions,&mut frame.progress,&mut frame.unwind,meter)?;
            rows(field(r,3)?,&mut frame.materials,&mut frame.progress,&mut frame.unwind,original,material,meter)?;
            semio_s_fem_receiving::move_vec(original,&mut frame.value.as_mut().unwrap().materials,&mut frame.materials,&mut frame.progress,&mut frame.unwind,meter)?;
            rows(field(r,4)?,&mut frame.sections,&mut frame.progress,&mut frame.unwind,original,section,meter)?;
            semio_s_fem_receiving::move_vec(original,&mut frame.value.as_mut().unwrap().sections,&mut frame.sections,&mut frame.progress,&mut frame.unwind,meter)?;
            rows(field(r,5)?,&mut frame.supports,&mut frame.progress,&mut frame.unwind,original,support,meter)?;
            semio_s_fem_receiving::move_vec(original,&mut frame.value.as_mut().unwrap().supports,&mut frame.supports,&mut frame.progress,&mut frame.unwind,meter)?;
            rows(field(r,6)?,&mut frame.load_cases,&mut frame.progress,&mut frame.unwind,original,load_case,meter)?;
            semio_s_fem_receiving::move_vec(original,&mut frame.value.as_mut().unwrap().load_cases,&mut frame.load_cases,&mut frame.progress,&mut frame.unwind,meter)?;
            rows(field(r,7)?,&mut frame.combinations,&mut frame.progress,&mut frame.unwind,original,combination,meter)?;
            semio_s_fem_receiving::move_vec(original,&mut frame.value.as_mut().unwrap().combinations,&mut frame.combinations,&mut frame.progress,&mut frame.unwind,meter)?;
            variants(field(r,1)?,&mut frame.elements,&mut frame.progress,&mut frame.unwind,original,|kind,r,output,progress,unwind,original,meter|member(r,output,progress,unwind,original,match kind{"bar"=>false,"beam"=>true,_=>return Err(invalid().into())},meter),meter)?;
            semio_s_fem_receiving::move_vec(original,&mut frame.value.as_mut().unwrap().elements,&mut frame.elements,&mut frame.progress,&mut frame.unwind,meter)?;
            let FieldValue::Block(inner)=field(r,8)? else{return Err(invalid().into())};let a=record(inner,original,&mut frame.progress,&mut frame.unwind,meter)?;declared(a,3,original,&mut frame.progress,&mut frame.unwind,meter)?;
            let mut modal=None;meter.take(usize::receive_value(field(a,0)?,&mut modal,original))?;semio_s_fem_receiving::move_scalar(original,&mut frame.value.as_mut().unwrap().analysis.modal_count,&mut modal,&mut frame.progress,&mut frame.unwind,meter)?;
            let mut buckling=None;meter.take(usize::receive_value(field(a,1)?,&mut buckling,original))?;semio_s_fem_receiving::move_scalar(original,&mut frame.value.as_mut().unwrap().analysis.buckling_count,&mut buckling,&mut frame.progress,&mut frame.unwind,meter)?;
            number(a,2,&mut frame.value.as_mut().unwrap().analysis.deformation_scale,&mut frame.progress,&mut frame.unwind,original,meter)?;
            meter.take(Self::receive_authoring(&mut frame.value,output,original))
        })))
    })}
}
