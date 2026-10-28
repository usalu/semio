//! 🫴️ Analytical records bind original fields into retained authoring slots before persistent row receiving.
use super::{Fem3dSnapshot, Fem3dSnapshotValue};
use crate::standards::v1::subsets::any::schema::structure::{FemDof, FemElement, FemLoad, FemLoadCase, FemSection, FemSupport};
use semio_framework_dsl_record::{
    FieldValue, RecordValue,
    receiving::{DslReceivingField, NativeSnapshotDecodeOwner, work},
};
use semio_framework_value::{RetireOwned, ValueError, ValueRefusalKind};
use semio_s_fem_material::volumetric::FemMaterialValue;
use semio_s_fem_node::volumetric::FemNodeValue;
use std::mem::size_of;
fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "FEM exchange record differs from its declared analytical shape")
}
#[cfg(test)]
#[path = "loads/receiver-tests.rs"]
mod load_tests;
fn field(record: &RecordValue, id: u16) -> Result<&FieldValue, ValueError> {
    record.get(id).ok_or_else(invalid)
}
fn record(value: &FieldValue) -> Result<&RecordValue, ValueError> {
    match value {
        FieldValue::Record(r) => Ok(r),
        _ => Err(invalid()),
    }
}
fn declared(record: &RecordValue, count: usize, owner: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<(), ValueError> {
    if record.fields.len() != count {
        return Err(invalid());
    }
    for (i, (id, _)) in record.fields.iter().enumerate() {
        work(owner, size_of::<u16>(), 1)?;
        if *id != i as u16 {
            return Err(invalid());
        }
    }
    Ok(())
}
fn text(record: &RecordValue, id: u16, owner: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<String, ValueError> {
    String::receive_value(field(record, id)?, owner)
}
fn number(record: &RecordValue, id: u16, owner: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<f64, ValueError> {
    let n = f64::receive_value(field(record, id)?, owner)?;
    if !n.is_finite() {
        return Err(invalid());
    }
    Ok(n)
}
fn row<T: RetireOwned>(
    value: &RecordValue,
    owner: &mut NativeSnapshotDecodeOwner<'_, '_>,
    initial: impl FnOnce() -> T,
    fill: impl FnOnce(&mut T, &RecordValue, &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<(), ValueError>,
) -> Result<T, ValueError> {
    owner.receive_nested::<T, T>(|slot, original| {
        work(original, size_of::<T>(), 1)?;
        *slot = Some(initial());
        fill(slot.as_mut().unwrap(), value, original)?;
        work(original, size_of::<T>(), 1)?;
        Ok(slot.take().unwrap())
    })
}
#[derive(RetireOwned)]
struct Rows<T: RetireOwned> {
    values: Vec<T>,
    child: Option<T>,
}
fn rows<T: RetireOwned>(value: &FieldValue, owner: &mut NativeSnapshotDecodeOwner<'_, '_>, mut receive: impl FnMut(&RecordValue, &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<T, ValueError>) -> Result<Vec<T>, ValueError> {
    let FieldValue::List(values) = value else {
        return Err(invalid());
    };
    owner.receive_nested::<Rows<T>, Vec<T>>(|slot, original| {
        work(original, size_of::<Rows<T>>(), 1)?;
        *slot = Some(Rows { values: Vec::new(), child: None });
        slot.as_mut().unwrap().values = original.receive::<Vec<T>, Vec<T>>(|slot, native, body| {
            *slot = Some(Vec::new());
            body.allocate_vec_into(native, values.len(), slot.as_mut().unwrap(), 1)?;
            Ok(slot.take().unwrap())
        })?;
        for value in values {
            let frame = slot.as_mut().unwrap();
            frame.child = Some(receive(record(value)?, original)?);
            work(original, size_of::<T>(), 1)?;
            frame.values.push(frame.child.take().unwrap());
        }
        work(original, size_of::<Vec<T>>(), 1)?;
        Ok(std::mem::take(&mut slot.as_mut().unwrap().values))
    })
}
fn variants<T: RetireOwned>(value: &FieldValue, owner: &mut NativeSnapshotDecodeOwner<'_, '_>, mut receive: impl FnMut(&str, &RecordValue, &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<T, ValueError>) -> Result<Vec<T>, ValueError> {
    let FieldValue::Block(inner) = value else { return Err(invalid()); };
    let FieldValue::Statements(values) = inner.as_ref() else { return Err(invalid()); };
    owner.receive_nested::<Rows<T>, Vec<T>>(|slot, original| {
        work(original, size_of::<Rows<T>>(), 1)?;
        *slot = Some(Rows { values: Vec::new(), child: None });
        slot.as_mut().unwrap().values = original.receive::<Vec<T>, Vec<T>>(|slot, native, body| {
            *slot = Some(Vec::new());
            body.allocate_vec_into(native, values.len(), slot.as_mut().unwrap(), 1)?;
            Ok(slot.take().unwrap())
        })?;
        for (kind, value) in values {
            slot.as_mut().unwrap().child = Some(receive(kind, value, original)?);
            work(original, size_of::<T>(), 1)?;
            let frame = slot.as_mut().unwrap();
            frame.values.push(frame.child.take().unwrap());
        }
        work(original, size_of::<Vec<T>>(), 1)?;
        Ok(std::mem::take(&mut slot.as_mut().unwrap().values))
    })
}
fn node(r: &RecordValue, o: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<FemNodeValue, ValueError> {
    declared(r, 4, o)?;
    row(
        r,
        o,
        || FemNodeValue { id: String::new(), x: 0., y: 0., z: 0. },
        |v, r, o| {
            v.id = text(r, 0, o)?;
            v.x = number(r, 1, o)?;
            v.y = number(r, 2, o)?;
            v.z = number(r, 3, o)?;
            Ok(())
        },
    )
}
fn material(r: &RecordValue, o: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<FemMaterialValue, ValueError> {
    declared(r, 6, o)?;
    row(
        r,
        o,
        || FemMaterialValue { id: String::new(), name: String::new(), e: 0., g: 0., nu: 0., rho: 0. },
        |v, r, o| {
            v.id = text(r, 0, o)?;
            v.name = text(r, 1, o)?;
            v.e = number(r, 2, o)?;
            v.g = number(r, 3, o)?;
            v.nu = number(r, 4, o)?;
            v.rho = number(r, 5, o)?;
            Ok(())
        },
    )
}
fn section(r: &RecordValue, o: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<FemSection, ValueError> {
    declared(r, 6, o)?;
    row(
        r,
        o,
        || FemSection { id: String::new(), name: String::new(), area: 0., iy: 0., iz: 0., j: 0. },
        |v, r, o| {
            v.id = text(r, 0, o)?;
            v.name = text(r, 1, o)?;
            v.area = number(r, 2, o)?;
            v.iy = number(r, 3, o)?;
            v.iz = number(r, 4, o)?;
            v.j = number(r, 5, o)?;
            if [v.area, v.iy, v.iz, v.j].iter().any(|v| *v <= 0.) {
                return Err(invalid());
            }
            Ok(())
        },
    )
}
fn support(r: &RecordValue, o: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<FemSupport, ValueError> {
    declared(r, 3, o)?;
    row(
        r,
        o,
        || FemSupport { id: String::new(), node_id: String::new(), fixed: Vec::new() },
        |v, r, o| {
            v.id = text(r, 0, o)?;
            v.node_id = text(r, 1, o)?;
            let FieldValue::List(values) = field(r, 2)? else {
                return Err(invalid());
            };
            if values.len() > 6 {
                return Err(invalid());
            }
            v.fixed = o.receive::<Vec<FemDof>, Vec<FemDof>>(|slot, native, body| {
                *slot = Some(Vec::new());
                body.allocate_vec_into(native, values.len(), slot.as_mut().unwrap(), 1)?;
                for value in values {
                    native.checkpoint()?;
                    let FieldValue::Enum(index) = value else {
                        return Err(invalid());
                    };
                    let dof = *([FemDof::Tx, FemDof::Ty, FemDof::Tz, FemDof::Rx, FemDof::Ry, FemDof::Rz].get(*index as usize).ok_or_else(invalid)?);
                    if slot.as_ref().unwrap().contains(&dof) {
                        return Err(invalid());
                    }
                    body.admit_frontier(semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: size_of::<FemDof>(), maximum_depth: 1, ..Default::default() })?;
                    slot.as_mut().unwrap().push(dof);
                    body.record_progress(semio_framework_value::RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<FemDof>(), ..Default::default() })?;
                }
                Ok(slot.take().unwrap())
            })?;
            Ok(())
        },
    )
}
#[derive(RetireOwned)]
struct Member {
    id: String,
    start: String,
    end: String,
    material: String,
    section: String,
    roll: f64,
}
fn member(r: &RecordValue, o: &mut NativeSnapshotDecodeOwner<'_, '_>, frame: bool) -> Result<FemElement, ValueError> {
    declared(r, if frame { 6 } else { 5 }, o)?;
    o.receive_nested::<Member, FemElement>(|slot, original| {
        work(original, size_of::<Member>(), 1)?;
        *slot = Some(Member { id: String::new(), start: String::new(), end: String::new(), material: String::new(), section: String::new(), roll: 0. });
        let v = slot.as_mut().unwrap();
        v.id = text(r, 0, original)?;
        v.start = text(r, 1, original)?;
        v.end = text(r, 2, original)?;
        v.material = text(r, 3, original)?;
        v.section = text(r, 4, original)?;
        if frame {
            v.roll = number(r, 5, original)?;
            if v.roll != 0. {
                return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "native FEM exchange admits zero-roll frames only"));
            }
        }
        work(original, size_of::<FemElement>(), 1)?;
        let v = slot.take().unwrap();
        Ok(if frame {
            FemElement::Frame { id: v.id, start: v.start, end: v.end, material_id: v.material, section_id: v.section, roll: v.roll }
        } else {
            FemElement::Bar { id: v.id, start: v.start, end: v.end, material_id: v.material, section_id: v.section }
        })
    })
}
#[derive(RetireOwned)]
struct LoadFields {
    id: String,
    target: String,
    dof: FemDof,
    values: [f64; 3],
}
fn load(kind: &str, r: &RecordValue, o: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<FemLoad, ValueError> {
    let nodal = match kind {
        "nodal" => true,
        "member-udl" => false,
        "area" => return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "native FEM exchange does not admit solid area loads")),
        _ => return Err(invalid()),
    };
    declared(r, if nodal { 4 } else { 5 }, o)?;
    o.receive_nested::<LoadFields, FemLoad>(|slot, original| {
        work(original, size_of::<LoadFields>(), 1)?;
        *slot = Some(LoadFields { id: String::new(), target: String::new(), dof: FemDof::Tx, values: [0.; 3] });
        let v = slot.as_mut().unwrap();
        v.id = text(r, 0, original)?;
        v.target = text(r, 1, original)?;
        if nodal {
            work(original, size_of::<FemDof>(), 1)?;
            let FieldValue::Enum(index) = field(r, 2)? else { return Err(invalid()); };
            v.dof = *[FemDof::Tx, FemDof::Ty, FemDof::Tz, FemDof::Rx, FemDof::Ry, FemDof::Rz].get(*index as usize).ok_or_else(invalid)?;
            v.values[0] = number(r, 3, original)?;
        } else {
            for i in 0..3 { v.values[i] = number(r, (i + 2) as u16, original)?; }
        }
        work(original, size_of::<FemLoad>(), 1)?;
        let v = slot.take().unwrap();
        Ok(if nodal {
            FemLoad::Nodal { id: v.id, node_id: v.target, dof: v.dof, value: v.values[0] }
        } else {
            FemLoad::MemberUdl { id: v.id, element_id: v.target, wx: v.values[0], wy: v.values[1], wz: v.values[2] }
        })
    })
}
fn load_case(r: &RecordValue, o: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<FemLoadCase, ValueError> {
    declared(r, 4, o)?;
    row(r, o, || FemLoadCase { id: String::new(), name: String::new(), loads: Vec::new(), self_weight: false }, |v, r, o| {
        v.id = text(r, 0, o)?;
        v.name = text(r, 1, o)?;
        v.loads = variants(field(r, 2)?, o, load)?;
        v.self_weight = bool::receive_value(field(r, 3)?, o)?;
        Ok(())
    })
}
impl Fem3dSnapshot {
    /// 📑️ Receives analytical members, supports and nodal/member loads under the original publication frame.
    pub fn receive_exchange_record(r: &RecordValue, owner: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<Self, ValueError> {
        declared(r, 9, owner)?;
        for id in [4, 7] {
            if !matches!(field(r,id)?,FieldValue::List(values)if values.is_empty()) {
                return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "native FEM exchange does not admit solids or combinations"));
            }
        }
        owner.receive_nested::<Fem3dSnapshotValue, Self>(|slot, original| {
            work(original, size_of::<Fem3dSnapshotValue>(), 1)?;
            *slot = Some(Fem3dSnapshotValue::default());
            slot.as_mut().unwrap().nodes = rows(field(r, 0)?, original, node)?;
            slot.as_mut().unwrap().materials = rows(field(r, 2)?, original, material)?;
            slot.as_mut().unwrap().sections = rows(field(r, 3)?, original, section)?;
            slot.as_mut().unwrap().supports = rows(field(r, 5)?, original, support)?;
            slot.as_mut().unwrap().load_cases = rows(field(r, 6)?, original, load_case)?;
            slot.as_mut().unwrap().elements = variants(field(r, 1)?, original, |kind, r, original| {
                member(r, original, match kind { "bar" => false, "frame" => true, _ => return Err(invalid()) })
            })?;
            let FieldValue::Block(inner) = field(r, 8)? else {
                return Err(invalid());
            };
            let a = record(inner)?;
            declared(a, 3, original)?;
            slot.as_mut().unwrap().analysis.modal_count = usize::receive_value(field(a, 0)?, original)?;
            slot.as_mut().unwrap().analysis.buckling_count = usize::receive_value(field(a, 1)?, original)?;
            slot.as_mut().unwrap().analysis.deformation_scale = number(a, 2, original)?;
            Self::receive_authoring(slot, original)
        })
    }
}
