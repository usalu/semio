//! 📦 Receives simple COS insert and remove payloads with owned, paid stream-role rows.

use super::{PdfMutation, receiving_invalid, metadata::{
    array, boolean, named, named_mut, optional, record, reserve, sequence, small_array, small_record, string, unsigned,
}};
use crate::standards::v1_7::subsets::base::schema::{
    diff::{PdfIndexedDiff, PdfIndexedItem},
    mutations::{InsertObject, RemoveObject},
    snapshot::{ObjRef, PdfObject},
    stream_roles::{PdfAdmittedStreamRole, PdfGraphIdentity, PdfStreamRoleValue},
};
use semio_framework_value::{
    DslValue, RetainedAllocationEffect, RetainedCloneProgress, RetirementDemand, ValueError,
    retained_clone::retained_clone_grant_admits,
};

const FIELDS: [&str; 4] = ["id", "value", "index", "admittedStreamRoles"];
const ROW_KEYS: [&str; 3] = ["removed", "modified", "added"];
const RECORD_SEARCH: u64 = 24;
const SMALL_ROLE_ROWS: usize = 4;

#[derive(Clone, Copy, Default)]
enum Phase { #[default] Find, Scope, Build, Id, Cos, Index, Role, Removed, Modified, Added, Done }

#[derive(Default)]
pub(super) struct ObjectReceiving {
    phase: Phase,
    scan: usize,
    positions: [Option<usize>; FIELDS.len()],
}

pub(super) enum ObjectStep { Pending, Ordinary, Complete }

fn required<'a>(value: &'a DslValue, key: &str) -> Result<&'a DslValue, ValueError> {
    named(value, key).ok_or_else(|| receiving_invalid("PDF archive object needs its declared field"))
}

fn identity(value: Option<&DslValue>) -> Result<ObjRef, ValueError> {
    let value = value.ok_or_else(|| receiving_invalid("PDF archive object needs an id"))?;
    let num = unsigned(Some(required(value, "num")?))?;
    let generation = unsigned(Some(required(value, "gen")?))?;
    Ok(ObjRef { num, r#gen: generation })
}

fn tag(value: &DslValue) -> Option<&str> {
    named(value, "kind").and_then(DslValue::as_str)
}

fn simple_cos(value: &DslValue) -> bool {
    small_record(value) && matches!(tag(value), None | Some("null" | "bool" | "int" | "text" | "name" | "ref"))
}

fn cos(value: Option<&mut DslValue>) -> Result<PdfObject, ValueError> {
    let value = value.ok_or_else(|| receiving_invalid("insertObject needs a COS value"))?;
    match tag(value) {
        Some("null") => Ok(PdfObject::Null),
        Some("bool") => boolean(named(value, "value")).map(PdfObject::Bool),
        Some("int") => required(value, "value")?.as_i64().map(PdfObject::Int)
            .ok_or_else(|| receiving_invalid("PDF COS integer needs an exact signed integer")),
        Some("text") => string(named_mut(value, "value")).map(PdfObject::Text),
        Some("name") => string(named_mut(value, "value")).map(PdfObject::Name),
        Some("ref") => identity(Some(value)).map(PdfObject::Ref),
        _ => Err(receiving_invalid("invalid PDF COS discriminator")),
    }
}

fn no_children(value: Option<&DslValue>) -> bool {
    array(value).map_or(true, <[DslValue]>::is_empty)
}

fn simple_role(item: &DslValue) -> bool {
    small_record(item) && named(item, "value").is_none_or(|role| {
        small_record(role) && no_children(named(role, "dependencies"))
            && named(role, "identity").is_none_or(|id| {
                small_record(id) && no_children(named(id, "path"))
                    && named(id, "owner").is_none_or(small_record)
            })
            && named(role, "value").is_none_or(|value| {
                small_record(value) && matches!(tag(value), None | Some("metadataText" | "calculatorProgram"))
            })
    })
}

fn small_role_diff(value: &DslValue) -> bool {
    small_record(value) && small_array(named(value, "removed"))
        && ROW_KEYS[1..].iter().all(|field| {
            array(named(value, field)).map_or(true, |rows| rows.len() <= SMALL_ROLE_ROWS && rows.iter().all(simple_role))
        })
}

#[derive(Clone, Copy)]
enum RoleText { Metadata, Calculator }

fn role_shape(item: &DslValue) -> Result<(usize, ObjRef, RoleText), ValueError> {
    let index = unsigned(Some(required(item, "index")?))?;
    let role = required(item, "value")?;
    let graph = required(role, "identity")?;
    let owner = identity(Some(required(graph, "owner")?))?;
    if !array(Some(required(graph, "path")?))?.is_empty()
        || !array(Some(required(role, "dependencies")?))?.is_empty()
    {
        return Err(ValueError::invariant("archive stream role scope changed"));
    }
    let value = required(role, "value")?;
    let (kind, name) = match tag(value) {
        Some("metadataText") => (RoleText::Metadata, "text"),
        Some("calculatorProgram") => (RoleText::Calculator, "code"),
        _ => return Err(receiving_invalid("invalid PDF archive stream role")),
    };
    if required(value, name)?.as_str().is_none() {
        return Err(receiving_invalid("PDF archive stream role needs text"));
    }
    Ok((index, owner, kind))
}

fn role_rows(
    raw: Option<&mut DslValue>, effect: &mut RetainedAllocationEffect<'_>,
) -> Result<Vec<PdfIndexedItem<PdfAdmittedStreamRole>>, ValueError> {
    let Some(raw) = raw else { return Ok(Vec::new()); };
    type Row = (usize, ObjRef, RoleText);
    let mut shape: [Option<Row>; SMALL_ROLE_ROWS] = [None; SMALL_ROLE_ROWS];
    for (index, item) in array(Some(raw))?.iter().enumerate() {
        shape[index] = Some(role_shape(item)?);
    }
    let DslValue::Array(items) = raw else { unreachable!() };
    let mut output = reserve(items.len(), effect)?;
    for (item, data) in items.iter_mut().zip(shape) {
        let (index, owner, kind) = data.expect("validated role row");
        let role = named_mut(item, "value").expect("validated role entry");
        let value = named_mut(role, "value").expect("validated role body");
        let value = match kind {
            RoleText::Metadata => PdfStreamRoleValue::MetadataText {
                text: string(named_mut(value, "text")).expect("validated metadata text"),
            },
            RoleText::Calculator => PdfStreamRoleValue::CalculatorProgram {
                code: string(named_mut(value, "code")).expect("validated calculator text"),
            },
        };
        output.push(PdfIndexedItem { index, value: PdfAdmittedStreamRole {
            identity: PdfGraphIdentity { owner, path: Vec::new() }, dependencies: Vec::new(), value,
        } });
    }
    Ok(output)
}

fn roles_target(target: &mut PdfMutation) -> &mut Option<PdfIndexedDiff<PdfAdmittedStreamRole>> {
    match target {
        PdfMutation::InsertObject(operation) => &mut operation.admitted_stream_roles,
        PdfMutation::RemoveObject(operation) => &mut operation.admitted_stream_roles,
        _ => unreachable!("selected PDF object mutation"),
    }
}

impl ObjectReceiving {
    fn field<'a>(&self, value: &'a DslValue, field: usize) -> Result<Option<&'a DslValue>, ValueError> {
        let fields = record(value)?;
        Ok(self.positions[field].map(|index| &fields[index].1))
    }

    fn row_fields(&self) -> Option<(usize, usize)> {
        match self.phase {
            Phase::Removed => Some((0, size_of::<usize>())),
            Phase::Modified => Some((1, size_of::<PdfIndexedItem<PdfAdmittedStreamRole>>())),
            Phase::Added => Some((2, size_of::<PdfIndexedItem<PdfAdmittedStreamRole>>())),
            _ => None,
        }
    }

    pub(super) fn demand(&self, value: &DslValue) -> Result<(RetirementDemand, u64), ValueError> {
        let mut demand = RetirementDemand { depth: 4, copy_bytes: size_of::<Self>() + 2 * size_of::<Option<PdfMutation>>(), ..Default::default() };
        let work = if let Some((field, element)) = self.row_fields() {
            let count = self.field(value, 3)?.and_then(|roles| array(named(roles, ROW_KEYS[field])).ok()).map_or(0, <[DslValue]>::len);
            demand.capacity_bytes = count * element;
            demand.copy_bytes += 2 * demand.capacity_bytes;
            if field == 0 { 2 * RECORD_SEARCH + 4 * count as u64 + 1 }
            else { 2 * RECORD_SEARCH + 18 * RECORD_SEARCH * count as u64 + 1 }
        } else { match self.phase {
            Phase::Find => FIELDS.len() as u64 + 1,
            Phase::Scope => 24 * RECORD_SEARCH * SMALL_ROLE_ROWS as u64 + 1,
            Phase::Id | Phase::Cos => 4 * RECORD_SEARCH + 1,
            _ => 1,
        }};
        Ok((demand, work))
    }

    pub(super) fn advance(
        &mut self, insert: bool, value: &mut DslValue, target: &mut Option<PdfMutation>, effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<(ObjectStep, RetainedCloneProgress), ValueError> {
        let (demand, work) = self.demand(value)?;
        if !retained_clone_grant_admits(effect.grant(), demand) || effect.remaining_work_units()? < work || !effect.performed(work)? {
            return Ok((ObjectStep::Pending, Default::default()));
        }
        let action = self.perform(insert, value, target, effect);
        let receipt = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes,
            retained_capacity_bytes: effect.actual().unwrap_or(0), ..Default::default() };
        action.map(|step| (step, receipt)).map_err(|error| error.with_retained_progress(receipt))
    }

    fn perform(
        &mut self, insert: bool, value: &mut DslValue, target: &mut Option<PdfMutation>, effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<ObjectStep, ValueError> {
        if let Some((field, _)) = self.row_fields() {
            let roles = roles_target(target.as_mut().expect("retained typed object")).as_mut().expect("retained admitted roles");
            let DslValue::Object(fields) = value else { unreachable!() };
            let raw = &mut fields[self.positions[3].expect("admitted role field")].1;
            let raw = named_mut(raw, ROW_KEYS[field]);
            self.phase = match field {
                0 => {
                    roles.removed = sequence(array(raw.as_deref())?, |value| unsigned(Some(value)), effect)?;
                    Phase::Modified
                }
                1 => { roles.modified = role_rows(raw, effect)?; Phase::Added }
                _ => { roles.added = role_rows(raw, effect)?; Phase::Done }
            };
            return Ok(ObjectStep::Pending);
        }
        self.phase = match self.phase {
            Phase::Find => {
                if let Some((key, _)) = record(value)?.get(self.scan) {
                    if let Some(field) = FIELDS.iter().position(|field| field == key) {
                        self.positions[field] = Some(self.scan);
                    }
                    self.scan += 1;
                    Phase::Find
                } else { Phase::Scope }
            }
            Phase::Scope => {
                if self.field(value, 0)?.is_some_and(|id| !small_record(id))
                    || insert && self.field(value, 1)?.is_some_and(|cos| !simple_cos(cos))
                    || self.field(value, 3)?.is_some_and(|roles| !small_role_diff(roles))
                { return Ok(ObjectStep::Ordinary); }
                Phase::Build
            }
            Phase::Build => {
                if target.is_some() { return Err(ValueError::invariant("PDF archive object target occupied")); }
                *target = Some(if insert { PdfMutation::InsertObject(InsertObject {
                    id: ObjRef::default(), value: PdfObject::Null, index: None, admitted_stream_roles: None,
                }) } else { PdfMutation::RemoveObject(RemoveObject { id: ObjRef::default(), admitted_stream_roles: None }) });
                Phase::Id
            }
            Phase::Id => {
                let id = identity(self.field(value, 0)?)?;
                match target.as_mut().expect("retained typed object") {
                    PdfMutation::InsertObject(operation) => { operation.id = id; Phase::Cos }
                    PdfMutation::RemoveObject(operation) => { operation.id = id; Phase::Role }
                    _ => unreachable!("selected PDF object mutation"),
                }
            }
            Phase::Cos => {
                let Some(PdfMutation::InsertObject(operation)) = target.as_mut() else { unreachable!() };
                let DslValue::Object(fields) = value else { unreachable!() };
                operation.value = cos(self.positions[1].map(|index| &mut fields[index].1))?;
                Phase::Index
            }
            Phase::Index => {
                let Some(PdfMutation::InsertObject(operation)) = target.as_mut() else { unreachable!() };
                operation.index = optional(self.field(value, 2)?, unsigned::<usize>)?;
                Phase::Role
            }
            Phase::Role => match self.field(value, 3)? {
                None | Some(DslValue::Null) => Phase::Done,
                Some(roles) => {
                    record(roles)?;
                    *roles_target(target.as_mut().expect("retained typed object")) = Some(PdfIndexedDiff::default());
                    Phase::Removed
                }
            },
            Phase::Done => return Ok(ObjectStep::Complete),
            _ => unreachable!(),
        };
        Ok(ObjectStep::Pending)
    }
}
