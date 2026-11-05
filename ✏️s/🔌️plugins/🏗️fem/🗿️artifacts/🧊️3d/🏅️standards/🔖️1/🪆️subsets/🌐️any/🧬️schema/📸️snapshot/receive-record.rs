//! 🫴️ Analytical records bind original fields into retained authoring slots before persistent row receiving.
use super::{Fem3dSnapshot, Fem3dSnapshotValue};
use crate::standards::v1::subsets::any::schema::structure::{FemDof, FemElement, FemLoad, FemLoadCase, FemSection, FemSupport};
use semio_framework_dsl_record::{
    FieldValue, RecordValue,
    receiving::{DslReceivingField,NativeSnapshotDecodeOwner,NativeSnapshotFailure,ReceivingWork,receiving_turn,work},
};
use semio_framework_value::{retirement::RetireOwned,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use semio_s_fem_material::volumetric::FemMaterialValue;
use semio_s_fem_node::volumetric::FemNodeValue;
use std::mem::size_of;
fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "FEM exchange record differs from its declared analytical shape")
}
fn field(record: &RecordValue, id: u16) -> Result<&FieldValue, ValueError> {
    record.get(id).ok_or_else(invalid)
}
fn record(value: &FieldValue) -> Result<&RecordValue, ValueError> {
    match value {
        FieldValue::Record(r) => Ok(r),
        _ => Err(invalid()),
    }
}
pub(super) fn action(owner:&mut NativeSnapshotDecodeOwner<'_,'_>,bytes:usize,depth:usize,meter:&mut ReceivingWork)->Result<(),NativeSnapshotFailure>{
    let g=owner.remaining_grant();
    let kind=if g.maximum_items==0{Some(ValueRefusalKind::WorkLimit)}else if g.maximum_depth<depth{Some(ValueRefusalKind::DepthLimit)}else if g.maximum_copy_bytes<bytes{Some(ValueRefusalKind::OwnershipLimit)}else{None};
    if let Some(kind)=kind{return Err(ValueError::literal(kind,"FEM primitive requires its original physical authority").into())}
    meter.admit(owner.native())?;
    work(owner,bytes,depth)?;
    Ok(())
}
fn declared(record: &RecordValue, count: usize, owner: &mut NativeSnapshotDecodeOwner<'_, '_>,meter:&mut ReceivingWork) -> Result<(), NativeSnapshotFailure> {
    action(owner,size_of::<usize>(),1,meter)?;
    if record.fields.len() != count {
        return Err(invalid().into());
    }
    for (i, (id, _)) in record.fields.iter().enumerate() {
        action(owner, size_of::<u16>(), 1,meter)?;
        if *id != i as u16 {
            return Err(invalid().into());
        }
    }
    Ok(())
}
fn text(record: &RecordValue, id: u16, owner: &mut NativeSnapshotDecodeOwner<'_, '_>,meter:&mut ReceivingWork) -> Result<String, NativeSnapshotFailure> {
    meter.take(String::receive_value(field(record, id)?, owner))
}
fn number(record: &RecordValue, id: u16, owner: &mut NativeSnapshotDecodeOwner<'_, '_>,meter:&mut ReceivingWork) -> Result<f64, NativeSnapshotFailure> {
    let n = meter.take(f64::receive_value(field(record, id)?, owner))?;
    action(owner,size_of::<f64>(),1,meter)?;
    if !n.is_finite() {
        return Err(invalid().into());
    }
    Ok(n)
}
fn row<T: RetireOwned>(
    value: &RecordValue,
    owner: &mut NativeSnapshotDecodeOwner<'_, '_>,
    initial: impl FnOnce() -> T,
    fill: impl FnOnce(&mut T, &RecordValue, &mut NativeSnapshotDecodeOwner<'_, '_>,&mut ReceivingWork) -> Result<(), NativeSnapshotFailure>,meter:&mut ReceivingWork) -> Result<T, NativeSnapshotFailure> {
    meter.take(owner.receive_nested::<T, T,store::NativeSnapshotFailure>(|slot, original| {receiving_turn(|meter|{
        action(original, size_of::<T>(), 1,meter)?;
        *slot = Some(initial());
        fill(slot.as_mut().unwrap(), value, original,meter)?;
        action(original, size_of::<T>(), 1,meter)?;
        Ok(slot.take().unwrap())
    })}))
}
#[derive(semio_framework_value::RetireOwned)]
struct Rows<T: RetireOwned> {
    values: Vec<T>,
    child: Option<T>,
}
fn rows<T: RetireOwned>(value: &FieldValue, owner: &mut NativeSnapshotDecodeOwner<'_, '_>, mut receive: impl FnMut(&RecordValue, &mut NativeSnapshotDecodeOwner<'_, '_>,&mut ReceivingWork) -> Result<T, NativeSnapshotFailure>,meter:&mut ReceivingWork) -> Result<Vec<T>, NativeSnapshotFailure> {
    let FieldValue::List(values) = value else {
        return Err(invalid().into());
    };
    meter.take(owner.receive_nested::<Rows<T>, Vec<T>,store::NativeSnapshotFailure>(|slot, original| {receiving_turn(|meter|{
        action(original, size_of::<Rows<T>>(), 1,meter)?;
        *slot = Some(Rows { values: Vec::new(), child: None });
        slot.as_mut().unwrap().values = meter.take(original.receive::<Vec<T>, Vec<T>,NativeSnapshotFailure>(|slot, native, body| {receiving_turn(|meter|{
            *slot = Some(Vec::new());
            body.admit_frontier(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<Vec<T>>(),maximum_capacity_bytes:values.len().checked_mul(size_of::<T>()).ok_or_else(invalid)?,maximum_release_bytes:0,maximum_depth:1})?;meter.admit(native)?;body.allocate_vec_into(native, values.len(), slot.as_mut().unwrap(), 1)?;
            Ok(slot.take().unwrap())
        })}))?;
        for value in values {
            let frame = slot.as_mut().unwrap();
            frame.child = Some(receive(record(value)?, original,meter)?);
            action(original, size_of::<T>(), 1,meter)?;
            frame.values.push(frame.child.take().unwrap());
        }
        action(original, size_of::<Vec<T>>(), 1,meter)?;
        Ok(std::mem::take(&mut slot.as_mut().unwrap().values))
    })}))
}
fn variants<T: RetireOwned>(value: &FieldValue, owner: &mut NativeSnapshotDecodeOwner<'_, '_>, mut receive: impl FnMut(&str, &RecordValue, &mut NativeSnapshotDecodeOwner<'_, '_>,&mut ReceivingWork) -> Result<T, NativeSnapshotFailure>,meter:&mut ReceivingWork) -> Result<Vec<T>, NativeSnapshotFailure> {
    let FieldValue::Block(inner) = value else { return Err(invalid().into()); };
    let FieldValue::Statements(values) = inner.as_ref() else { return Err(invalid().into()); };
    meter.take(owner.receive_nested::<Rows<T>, Vec<T>,store::NativeSnapshotFailure>(|slot, original| {receiving_turn(|meter|{
        action(original, size_of::<Rows<T>>(), 1,meter)?;
        *slot = Some(Rows { values: Vec::new(), child: None });
        slot.as_mut().unwrap().values = meter.take(original.receive::<Vec<T>, Vec<T>,NativeSnapshotFailure>(|slot, native, body| {receiving_turn(|meter|{
            *slot = Some(Vec::new());
            body.admit_frontier(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<Vec<T>>(),maximum_capacity_bytes:values.len().checked_mul(size_of::<T>()).ok_or_else(invalid)?,maximum_release_bytes:0,maximum_depth:1})?;meter.admit(native)?;body.allocate_vec_into(native, values.len(), slot.as_mut().unwrap(), 1)?;
            Ok(slot.take().unwrap())
        })}))?;
        for (kind, value) in values {
            slot.as_mut().unwrap().child = Some(receive(kind, value, original,meter)?);
            action(original, size_of::<T>(), 1,meter)?;
            let frame = slot.as_mut().unwrap();
            frame.values.push(frame.child.take().unwrap());
        }
        action(original, size_of::<Vec<T>>(), 1,meter)?;
        Ok(std::mem::take(&mut slot.as_mut().unwrap().values))
    })}))
}
fn node(r: &RecordValue, o: &mut NativeSnapshotDecodeOwner<'_, '_>,meter:&mut ReceivingWork) -> Result<FemNodeValue, NativeSnapshotFailure> {
    declared(r, 4, o,meter)?;
    row(
        r,
        o,
        || FemNodeValue { id: String::new(), x: 0., y: 0., z: 0. },
        |v, r, o,meter| {
            v.id = text(r, 0, o,meter)?;
            v.x = number(r, 1, o,meter)?;
            v.y = number(r, 2, o,meter)?;
            v.z = number(r, 3, o,meter)?;
            Ok(())
        },
    meter)
}
fn material(r: &RecordValue, o: &mut NativeSnapshotDecodeOwner<'_, '_>,meter:&mut ReceivingWork) -> Result<FemMaterialValue, NativeSnapshotFailure> {
    declared(r, 6, o,meter)?;
    row(
        r,
        o,
        || FemMaterialValue { id: String::new(), name: String::new(), e: 0., g: 0., nu: 0., rho: 0. },
        |v, r, o,meter| {
            v.id = text(r, 0, o,meter)?;
            v.name = text(r, 1, o,meter)?;
            v.e = number(r, 2, o,meter)?;
            v.g = number(r, 3, o,meter)?;
            v.nu = number(r, 4, o,meter)?;
            v.rho = number(r, 5, o,meter)?;
            Ok(())
        },
    meter)
}
fn section(r: &RecordValue, o: &mut NativeSnapshotDecodeOwner<'_, '_>,meter:&mut ReceivingWork) -> Result<FemSection, NativeSnapshotFailure> {
    declared(r, 6, o,meter)?;
    row(
        r,
        o,
        || FemSection { id: String::new(), name: String::new(), area: 0., iy: 0., iz: 0., j: 0. },
        |v, r, o,meter| {
            v.id = text(r, 0, o,meter)?;
            v.name = text(r, 1, o,meter)?;
            v.area = number(r, 2, o,meter)?;
            v.iy = number(r, 3, o,meter)?;
            v.iz = number(r, 4, o,meter)?;
            v.j = number(r, 5, o,meter)?;
            if [v.area, v.iy, v.iz, v.j].iter().any(|v| *v <= 0.) {
                return Err(invalid().into());
            }
            Ok(())
        },
    meter)
}
fn support(r: &RecordValue, o: &mut NativeSnapshotDecodeOwner<'_, '_>,meter:&mut ReceivingWork) -> Result<FemSupport, NativeSnapshotFailure> {
    declared(r, 3, o,meter)?;
    row(
        r,
        o,
        || FemSupport { id: String::new(), node_id: String::new(), fixed: Vec::new() },
        |v, r, o,meter| {
            v.id = text(r, 0, o,meter)?;
            v.node_id = text(r, 1, o,meter)?;
            let FieldValue::List(values) = field(r, 2)? else {
                return Err(invalid().into());
            };
            if values.len() > 6 {
                return Err(invalid().into());
            }
            v.fixed = meter.take(o.receive::<Vec<FemDof>, Vec<FemDof>,NativeSnapshotFailure>(|slot, native, body| {receiving_turn(|meter|{
                *slot = Some(Vec::new());
                body.admit_frontier(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<Vec<FemDof>>(),maximum_capacity_bytes:values.len().checked_mul(size_of::<FemDof>()).ok_or_else(invalid)?,maximum_release_bytes:0,maximum_depth:1})?;meter.admit(native)?;body.allocate_vec_into(native, values.len(), slot.as_mut().unwrap(), 1)?;
                for value in values {
                    body.admit_frontier(semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: size_of::<FemDof>(), maximum_depth: 1, ..Default::default() })?;
                    meter.admit(native)?;
                    let FieldValue::Enum(index) = value else {
                        return Err(invalid().into());
                    };
                    let dof = *([FemDof::Tx, FemDof::Ty, FemDof::Tz, FemDof::Rx, FemDof::Ry, FemDof::Rz].get(*index as usize).ok_or_else(invalid)?);
                    if slot.as_ref().unwrap().contains(&dof) {
                        return Err(invalid().into());
                    }
                    slot.as_mut().unwrap().push(dof);
                    body.record_progress(semio_framework_value::RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<FemDof>(), ..Default::default() })?;
                }
                Ok(slot.take().unwrap())
            })}))?;
            Ok(())
        },
    meter)
}
#[derive(semio_framework_value::RetireOwned)]
struct Member {
    id: String,
    start: String,
    end: String,
    material: String,
    section: String,
    roll: f64,
}
fn member(r: &RecordValue, o: &mut NativeSnapshotDecodeOwner<'_, '_>, frame: bool,meter:&mut ReceivingWork) -> Result<FemElement, NativeSnapshotFailure> {
    declared(r, if frame { 6 } else { 5 }, o,meter)?;
    meter.take(o.receive_nested::<Member, FemElement,store::NativeSnapshotFailure>(|slot, original| {receiving_turn(|meter|{
        action(original, size_of::<Member>(), 1,meter)?;
        *slot = Some(Member { id: String::new(), start: String::new(), end: String::new(), material: String::new(), section: String::new(), roll: 0. });
        let v = slot.as_mut().unwrap();
        v.id = text(r, 0, original,meter)?;
        v.start = text(r, 1, original,meter)?;
        v.end = text(r, 2, original,meter)?;
        v.material = text(r, 3, original,meter)?;
        v.section = text(r, 4, original,meter)?;
        if frame {
            v.roll = number(r, 5, original,meter)?;
            if v.roll != 0. {
                return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "native FEM exchange admits zero-roll frames only").into());
            }
        }
        action(original, size_of::<FemElement>(), 1,meter)?;
        let v = slot.take().unwrap();
        Ok(if frame {
            FemElement::Frame { id: v.id, start: v.start, end: v.end, material_id: v.material, section_id: v.section, roll: v.roll }
        } else {
            FemElement::Bar { id: v.id, start: v.start, end: v.end, material_id: v.material, section_id: v.section }
        })
    })}))
}
#[derive(semio_framework_value::RetireOwned)]
struct LoadFields {
    id: String,
    target: String,
    dof: FemDof,
    values: [f64; 3],
}
fn load(kind: &str, r: &RecordValue, o: &mut NativeSnapshotDecodeOwner<'_, '_>,meter:&mut ReceivingWork) -> Result<FemLoad, NativeSnapshotFailure> {
    let nodal = match kind {
        "nodal" => true,
        "member-udl" => false,
        "area" => return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "native FEM exchange does not admit solid area loads").into()),
        _ => return Err(invalid().into()),
    };
    declared(r, if nodal { 4 } else { 5 }, o,meter)?;
    meter.take(o.receive_nested::<LoadFields, FemLoad,store::NativeSnapshotFailure>(|slot, original| {receiving_turn(|meter|{
        action(original, size_of::<LoadFields>(), 1,meter)?;
        *slot = Some(LoadFields { id: String::new(), target: String::new(), dof: FemDof::Tx, values: [0.; 3] });
        let v = slot.as_mut().unwrap();
        v.id = text(r, 0, original,meter)?;
        v.target = text(r, 1, original,meter)?;
        if nodal {
            action(original, size_of::<FemDof>(), 1,meter)?;
            let FieldValue::Enum(index) = field(r, 2)? else { return Err(invalid().into()); };
            v.dof = *[FemDof::Tx, FemDof::Ty, FemDof::Tz, FemDof::Rx, FemDof::Ry, FemDof::Rz].get(*index as usize).ok_or_else(invalid)?;
            v.values[0] = number(r, 3, original,meter)?;
        } else {
            for i in 0..3 { v.values[i] = number(r, (i + 2) as u16, original,meter)?; }
        }
        action(original, size_of::<FemLoad>(), 1,meter)?;
        let v = slot.take().unwrap();
        Ok(if nodal {
            FemLoad::Nodal { id: v.id, node_id: v.target, dof: v.dof, value: v.values[0] }
        } else {
            FemLoad::MemberUdl { id: v.id, element_id: v.target, wx: v.values[0], wy: v.values[1], wz: v.values[2] }
        })
    })}))
}
fn load_case(r: &RecordValue, o: &mut NativeSnapshotDecodeOwner<'_, '_>,meter:&mut ReceivingWork) -> Result<FemLoadCase, NativeSnapshotFailure> {
    declared(r, 4, o,meter)?;
    row(r, o, || FemLoadCase { id: String::new(), name: String::new(), loads: Vec::new(), self_weight: false }, |v, r, o,meter| {
        v.id = text(r, 0, o,meter)?;
        v.name = text(r, 1, o,meter)?;
        v.loads = variants(field(r, 2)?, o, load,meter)?;
        v.self_weight = meter.take(bool::receive_value(field(r, 3)?, o))?;
        Ok(())
    },meter)
}
impl Fem3dSnapshot {
    /// 📑️ Receives analytical members, supports and nodal/member loads under the original publication frame.
    pub fn receive_exchange_record(r: &RecordValue, owner: &mut NativeSnapshotDecodeOwner<'_, '_>) -> RetainedWorkOutcome<Self, NativeSnapshotFailure> {receiving_turn(|meter|{
        declared(r, 9, owner,meter)?;
        for id in [4, 7] {
            if !matches!(field(r,id)?,FieldValue::List(values)if values.is_empty()) {
                return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "native FEM exchange does not admit solids or combinations").into());
            }
        }
        meter.take(owner.receive_nested::<Fem3dSnapshotValue, Self,store::NativeSnapshotFailure>(|slot, original| {receiving_turn(|meter|{
            action(original, size_of::<Fem3dSnapshotValue>(), 1,meter)?;
            *slot = Some(Fem3dSnapshotValue::default());
            slot.as_mut().unwrap().nodes = rows(field(r, 0)?, original, node,meter)?;
            slot.as_mut().unwrap().materials = rows(field(r, 2)?, original, material,meter)?;
            slot.as_mut().unwrap().sections = rows(field(r, 3)?, original, section,meter)?;
            slot.as_mut().unwrap().supports = rows(field(r, 5)?, original, support,meter)?;
            slot.as_mut().unwrap().load_cases = rows(field(r, 6)?, original, load_case,meter)?;
            slot.as_mut().unwrap().elements = variants(field(r, 1)?, original, |kind, r, original,meter| {
                member(r, original, match kind { "bar" => false, "frame" => true, _ => return Err(invalid().into()) },meter)
            },meter)?;
            let FieldValue::Block(inner) = field(r, 8)? else {
                return Err(invalid().into());
            };
            let a = record(inner)?;
            declared(a, 3, original,meter)?;
            slot.as_mut().unwrap().analysis.modal_count = meter.take(usize::receive_value(field(a, 0)?, original))?;
            slot.as_mut().unwrap().analysis.buckling_count = meter.take(usize::receive_value(field(a, 1)?, original))?;
            slot.as_mut().unwrap().analysis.deformation_scale = number(a, 2, original,meter)?;
            meter.take(Self::receive_authoring(slot, original))
        })}))
    })}
}
