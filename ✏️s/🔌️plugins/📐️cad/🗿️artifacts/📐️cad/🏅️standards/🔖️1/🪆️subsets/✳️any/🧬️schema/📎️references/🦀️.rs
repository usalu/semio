//! 📎️ Literal reference ownership shared by the persistent CAD document and snapshot.
use crate::CadReferenceList;
use semio_framework_dsl_record::{DslField, FieldValue, NativeSchemaControl, Shape};
use semio_framework_value::{DslValue, FromValue, NativeDecodeControl, NativeEncodeControl, ToValue, ValueEdit, ValueError, ValueRefusalKind, ValueShape};
use semio_framework_value::retirement::{RetireOwned, RetirementCursor};
use std::cmp::Ordering;

#[path = "🚦️frontiers/🦀️.rs"]
pub(crate) mod frontiers;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CadReferenceIndex {
    entries: Vec<(String, CadReferenceList)>,
}

impl CadReferenceIndex {
    pub fn new() -> Self { Self { entries: Vec::new() } }
    pub fn len(&self) -> usize { self.entries.len() }
    pub fn is_empty(&self) -> bool { self.entries.is_empty() }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = (&String, &CadReferenceList)> { self.entries.iter().map(|(key, rows)| (key, rows)) }
    pub fn values(&self) -> impl ExactSizeIterator<Item = &CadReferenceList> { self.entries.iter().map(|(_, rows)| rows) }
    pub fn values_mut(&mut self) -> impl ExactSizeIterator<Item = &mut CadReferenceList> { self.entries.iter_mut().map(|(_, rows)| rows) }
    pub fn keys(&self) -> impl ExactSizeIterator<Item = &String> { self.entries.iter().map(|(key, _)| key) }
    fn position(&self, key: &str) -> Result<usize, usize> { self.entries.binary_search_by(|(owned, _)| owned.as_bytes().cmp(key.as_bytes())) }
    pub fn get(&self, key: &str) -> Option<&CadReferenceList> { self.position(key).ok().map(|index| &self.entries[index].1) }
    pub fn get_mut(&mut self, key: &str) -> Option<&mut CadReferenceList> { self.position(key).ok().map(|index| &mut self.entries[index].1) }
    pub fn contains_key(&self, key: &str) -> bool { self.position(key).is_ok() }
    pub fn insert(&mut self, key: String, rows: CadReferenceList) -> Option<CadReferenceList> {
        match self.position(&key) {
            Ok(index) => { close(key); Some(std::mem::replace(&mut self.entries[index].1, rows)) }
            Err(index) => { self.entries.insert(index, (key, rows)); None }
        }
    }
    pub fn remove(&mut self, key: &str) -> Option<CadReferenceList> { self.position(key).ok().map(|index| { let (key, rows) = self.entries.remove(index); close(key); rows }) }
    pub fn from_entries(entries: Vec<(String, CadReferenceList)>) -> Result<Self, ValueError> {
        match (Self { entries }).finish_ordinary() { Ok(value) => Ok(value), Err(value) => { close(value); Err(invalid("CAD reference keys must be unique")) } }
    }
    pub(crate) fn owned_slots(count: usize, control: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError> { Ok(Self { entries: control.allocate_vec(count)? }) }
    pub(crate) fn append(&mut self, key: String, rows: CadReferenceList) -> Result<(), ValueError> {
        if self.entries.len() == self.entries.capacity() { close((key, rows)); return Err(invalid("CAD reference index exceeded admitted slots")); }
        self.entries.push((key, rows)); Ok(())
    }
    pub(crate) fn finish(mut self, control: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        let result = frontiers::order(&mut self.entries, |left, right, control| frontiers::compare_literal(&left.0, &right.0, control), control);
        if let Err(error) = result { close(self); return Err(error); }
        for index in 1..self.entries.len() {
            match frontiers::compare_literal(&self.entries[index - 1].0, &self.entries[index].0, control) {
                Ok(Ordering::Equal) => { close(self); return Err(invalid("CAD reference keys must be unique")); }
                Err(error) => { close(self); return Err(error); }
                _ => {}
            }
        }
        Ok(self)
    }
    fn finish_ordinary(mut self) -> Result<Self, Self> {
        self.entries.sort_unstable_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
        if self.entries.windows(2).any(|pair| pair[0].0 == pair[1].0) { Err(self) } else { Ok(self) }
    }
}

impl std::ops::Index<&str> for CadReferenceIndex {
    type Output = CadReferenceList;
    fn index(&self, key: &str) -> &Self::Output { self.get(key).expect("declared CAD reference key") }
}
impl<'a> IntoIterator for &'a CadReferenceIndex {
    type Item = (&'a String, &'a CadReferenceList);
    type IntoIter = std::iter::Map<std::slice::Iter<'a, (String, CadReferenceList)>, fn(&'a (String, CadReferenceList)) -> Self::Item>;
    fn into_iter(self) -> Self::IntoIter { self.entries.iter().map(|(key, rows)| (key, rows)) }
}
impl RetireOwned for CadReferenceIndex {
    fn retirement(self) -> Box<dyn RetirementCursor> { self.entries.retirement() }
}

pub(crate) fn invalid(message: &'static str) -> ValueError { ValueError::new(ValueRefusalKind::InvalidValue, message) }
pub(crate) fn close<T: RetireOwned>(value: T) {
    let mut retirement = semio_framework_value::retirement::owned_retirement(value);
    while !retirement.terminal_is_empty() { retirement.close_step(256, 65536).expect("CAD native cold retirement grant"); }
}

impl semio_framework_dsl_record::BorrowedDslField for CadReferenceIndex {
    const SHAPE: semio_framework_dsl_record::BorrowedShape = semio_framework_dsl_record::BorrowedShape::Map(semio_framework_dsl_record::borrowed_field_shape::<CadReferenceList>);
}

impl DslField for CadReferenceIndex {
    fn projection_view(&self, path: &[usize]) -> Result<semio_framework_dsl_record::native_encoding::FieldProjectionView<'_>, ValueError> {
        if path.is_empty() { return Ok(semio_framework_dsl_record::native_encoding::FieldProjectionView::Map(self.len())); }
        self.entries.get(path[0]).ok_or_else(semio_framework_dsl_record::native_encoding::projection_path_error)?.1.projection_view(&path[1..])
    }
    fn projection_key(&self, path: &[usize], index: usize) -> Result<&str, ValueError> {
        if path.is_empty() { return self.entries.get(index).map(|(key, _)| key.as_str()).ok_or_else(semio_framework_dsl_record::native_encoding::projection_path_error); }
        self.entries.get(path[0]).ok_or_else(semio_framework_dsl_record::native_encoding::projection_path_error)?.1.projection_key(&path[1..], index)
    }
    fn shape() -> Shape { Shape::Map(Box::new(<CadReferenceList as DslField>::shape())) }
    fn shape_controlled<C: NativeSchemaControl>(control: &mut C) -> Result<Shape, ValueError> {
        control.scoped_depth(64, |control| Ok(Shape::Map(semio_framework_dsl_record::producer::boxed(<CadReferenceList as DslField>::shape_controlled(control)?, control)?)))
    }
    fn to_value(&self) -> FieldValue { FieldValue::Map(self.entries.iter().map(|(key, rows)| (key.clone(), <CadReferenceList as DslField>::to_value(rows))).collect()) }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        let FieldValue::Map(entries) = value else { return Err("CAD requires a literal native reference map".into()); };
        let mut output = semio_framework_dsl_record::__rt::DecodedFieldOwner::new(Self::new(), close::<Self>);
        for (key, value) in entries {
            let key = semio_framework_dsl_record::__rt::DecodedFieldOwner::new(key.clone(), close::<String>);
            let rows = <CadReferenceList as DslField>::from_value(value)?;
            output.as_mut().entries.push((key.take(), rows));
        }
        match output.take().finish_ordinary() {
            Ok(value) => Ok(value),
            Err(value) => { close(value); Err("CAD reference keys must be unique".into()) }
        }
    }
    fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        let FieldValue::Map(entries) = value else { return Err(invalid("CAD requires a literal native reference map")); };
        control.scoped_stage(|control| {
            control.begin_stage(entries.len())?;
            let mut output = semio_framework_dsl_record::__rt::DecodedFieldOwner::new(Self::owned_slots(entries.len(), control)?, close::<Self>);
            for (key, value) in entries {
                let key = semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.copy_text(key)?, close::<String>);
                let rows = control.scoped_stage(|control| { control.begin_stage(1)?; <CadReferenceList as DslField>::from_value_controlled(value, control) })?;
                output.as_mut().append(key.take(), rows)?;
                control.step()?;
            }
            output.take().finish(control)
        })
    }
    fn to_value_controlled(&self, control: &mut NativeEncodeControl<'_>) -> Result<FieldValue, ValueError> {
        control.scoped_stage(|control| {
            control.begin_stage(self.len())?;
            let mut output = semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.allocate_vec(self.len())?, close::<Vec<(String, FieldValue)>>);
            for (key, rows) in &self.entries {
                let key = semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.copy_text(key)?, close::<String>);
                let rows = control.scoped_stage(|control| <CadReferenceList as DslField>::to_value_controlled(rows, control))?;
                output.as_mut().push((key.take(), rows)); control.step()?;
            }
            Ok(FieldValue::Map(output.take()))
        })
    }
    fn retire_decoded(self) { close(self); }
}

impl ToValue for CadReferenceIndex {
    fn to_value(&self) -> DslValue { DslValue::Object(self.entries.iter().map(|(key, rows)| (key.clone(), ToValue::to_value(rows))).collect()) }
    fn to_value_controlled(&self, control: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
        control.scoped_stage(|control| {
            control.begin_stage(self.len())?;
            let mut output = DslValue::object_encoding_controlled(self.len(), control)?;
            for (key, rows) in &self.entries {
                let key = semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.copy_text(key)?, close::<String>);
                let rows = control.scoped_stage(|control| ToValue::to_value_controlled(rows, control))?;
                output.get_mut().push((key.take(), rows)); control.step()?;
            }
            Ok(DslValue::Object(output.take()))
        })
    }
    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        let Some((key, rest)) = path.split_first() else { return Ok(ToValue::to_value(self)); };
        self.get(key).ok_or_else(|| invalid("CAD reference key is absent"))?.value_at_path(rest).map_err(|error| error.under(key))
    }
    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        let Some((key, rest)) = path.split_first() else { return Ok(ValueShape::Object { len: self.len() }); };
        self.get(key).ok_or_else(|| invalid("CAD reference key is absent"))?.value_shape_at_path(rest).map_err(|error| error.under(key))
    }
    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, ValueError> {
        let Some((key, rest)) = path.split_first() else { return self.entries.get(index).map(|(key, _)| key.clone()).ok_or_else(|| invalid("CAD reference key ordinal is absent")); };
        self.get(key).ok_or_else(|| invalid("CAD reference key is absent"))?.value_key_at_path(rest, index).map_err(|error| error.under(key))
    }
}
impl FromValue for CadReferenceIndex {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(entries) = value else { close(value); return Err(invalid("CAD requires a literal reference object")); };
        let mut output = semio_framework_dsl_record::__rt::DecodedFieldOwner::new(Self::new(), close::<Self>);
        let mut input = semio_framework_dsl_record::__rt::DecodedFieldOwner::new(entries, close::<Vec<(String, DslValue)>>);
        while let Some((key, value)) = input.as_mut().pop() {
            let key = semio_framework_dsl_record::__rt::DecodedFieldOwner::new(key, close::<String>);
            let rows = <CadReferenceList as FromValue>::from_value(value)?;
            output.as_mut().entries.push((key.take(), rows));
        }
        match output.take().finish_ordinary() { Ok(value) => Ok(value), Err(value) => { close(value); Err(invalid("CAD reference keys must be unique")) } }
    }
    fn from_value_controlled(value: &DslValue, control: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        let DslValue::Object(entries) = value else { return Err(invalid("CAD requires a literal reference object")); };
        control.scoped_stage(|control| {
            control.begin_stage(entries.len())?;
            let mut output = semio_framework_dsl_record::__rt::DecodedFieldOwner::new(Self::owned_slots(entries.len(), control)?, close::<Self>);
            for (key, value) in entries {
                let key = semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.copy_text(key)?, close::<String>);
                let rows = control.scoped_stage(|control| { control.begin_stage(1)?; <CadReferenceList as FromValue>::from_value_controlled(value, control) })?;
                output.as_mut().append(key.take(), rows)?; control.step()?;
            }
            output.take().finish(control)
        })
    }
    fn default_value_controlled(control: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError> { control.checkpoint()?; Ok(Self::new()) }
    fn retire_decoded(self) { close(self); }
    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        let Some((key, rest)) = path.split_first() else {
            return match edit { ValueEdit::Set(value) => { let replacement = FromValue::from_value(value)?; close(std::mem::replace(self, replacement)); Ok(()) }, _ => Err(invalid("CAD reference edit requires a key")) };
        };
        if !rest.is_empty() { return self.get_mut(key).ok_or_else(|| invalid("CAD reference key is absent"))?.edit_value_at_path(rest, edit); }
        match edit {
            ValueEdit::Set(value) => {
                if !self.contains_key(key) { close(value); return Err(invalid("CAD reference key is absent")); }
                let replacement = <CadReferenceList as FromValue>::from_value(value)?;
                let target = self.get_mut(key).ok_or_else(|| invalid("CAD reference key is absent"))?;
                close(std::mem::replace(target, replacement));
            }
            ValueEdit::Insert(value) | ValueEdit::InsertAt { value, .. } => {
                if self.contains_key(key) { close(value); return Err(invalid("CAD reference key already exists")); }
                let replacement = <CadReferenceList as FromValue>::from_value(value)?;
                self.insert((*key).into(), replacement);
            }
            ValueEdit::Remove => close(self.remove(key).ok_or_else(|| invalid("CAD reference key is absent"))?),
        }
        Ok(())
    }
}

semio_framework_value::artifact_retire_struct!(crate::CadReference { id, source_url, media_kind, origin, orientation, scale, width_world, hidden, locked, opacity });
