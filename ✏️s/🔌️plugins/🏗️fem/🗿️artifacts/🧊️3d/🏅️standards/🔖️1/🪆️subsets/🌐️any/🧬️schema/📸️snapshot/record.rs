//! 📑️ The actual persistent owner keeps the canonical authored record shape and requires original receiving authority.
use super::{Fem3dSnapshot, Fem3dSnapshotValue};
use semio_framework_dsl_record::{DslField, DslVariants, FieldValue, RecordSpec, RecordSpecProducer, RecordValue, native_encoding::EncodedRecord};
use semio_framework_value::{NativeEncodeControl, ValueError, ValueRefusalKind};
fn unavailable() -> ValueError {
    ValueError::literal(ValueRefusalKind::UnsupportedOwner, "persistent FEM snapshot requires its original NativeSnapshotDecodeOwner receiving interface")
}
fn row(texts: &[&str], numbers: &[f64]) -> RecordValue {
    let mut r = RecordValue::default();
    for (i, text) in texts.iter().enumerate() {
        r.fields.insert(i as u16, FieldValue::Text((*text).into()));
    }
    for (i, n) in numbers.iter().enumerate() {
        r.fields.insert((texts.len() + i) as u16, FieldValue::Float(*n));
    }
    r
}
fn encoded_row(texts: &[&str], numbers: &[f64], control: &mut NativeEncodeControl<'_>) -> Result<RecordValue, ValueError> {
    let mut r = EncodedRecord::new(texts.len() + numbers.len(), control)?;
    for (i, text) in texts.iter().enumerate() {
        r.insert(i as u16, FieldValue::Text(control.copy_text(text)?))?;
        control.step()?;
    }
    for (i, n) in numbers.iter().enumerate() {
        r.insert((texts.len() + i) as u16, FieldValue::Float(*n))?;
        control.step()?;
    }
    Ok(r.take())
}
impl Fem3dSnapshot {
    /// 🧬️ The explicit authored value owns the shared canonical record declaration.
    pub fn __dsl_spec() -> RecordSpec {
        Fem3dSnapshotValue::__dsl_spec()
    }
    /// 🏭️ Metadata factories stay lazy until the original encoder or decoder admits them.
    pub fn __dsl_spec_producer() -> RecordSpecProducer {
        Fem3dSnapshotValue::__dsl_spec_producer()
    }
    /// 📤️ The pure reference interface projects the actual owner without substituting its artifact identity.
    pub fn __dsl_to_record(&self) -> RecordValue {
        let mut r = RecordValue::default();
        r.fields.insert(0, FieldValue::List(self.nodes.iter().map(|n| FieldValue::Record(row(&[n.id.as_str()], &[n.x, n.y, n.z]))).collect()));
        r.fields.insert(1, FieldValue::Block(Box::new(FieldValue::Statements(self.elements.iter().map(DslVariants::to_named_record).collect()))));
        r.fields.insert(2, FieldValue::List(self.materials.iter().map(|m| FieldValue::Record(row(&[m.id.as_str(), m.name.as_str()], &[m.e, m.g, m.nu, m.rho]))).collect()));
        r.fields.insert(3, DslField::to_value(&self.sections));
        r.fields.insert(4, DslField::to_value(&self.solids));
        r.fields.insert(5, DslField::to_value(&self.supports));
        r.fields.insert(6, DslField::to_value(&self.load_cases));
        r.fields.insert(7, DslField::to_value(&self.combinations));
        r.fields.insert(8, FieldValue::Block(Box::new(DslField::to_value(&self.analysis))));
        r
    }
    /// 🛫️ Projects native persistent rows directly under the unchanged encoder instead of cloning an authoring snapshot.
    pub fn __dsl_to_record_controlled(&self, control: &mut NativeEncodeControl<'_>) -> Result<RecordValue, ValueError> {
        control.scoped_depth(64, |control| {
            control.scoped_stage(|control| {
                control.begin_stage(9)?;
                let mut r = EncodedRecord::new(9, control)?;
                let mut nodes = semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.allocate_vec(self.nodes.len())?, |v: Vec<FieldValue>| {
                    for f in v {
                        semio_framework_dsl_record::native_encoding::retire_field(f);
                    }
                });
                for n in self.nodes.iter() {
                    control.checkpoint()?;
                    nodes.as_mut().push(FieldValue::Record(encoded_row(&[n.id.as_str()], &[n.x, n.y, n.z], control)?));
                }
                r.insert(0, FieldValue::List(nodes.take()))?;
                let elements = semio_framework_dsl_record::native_encoding::project_statements(&self.elements, control)?;
                control.charge(std::mem::size_of::<FieldValue>())?;
                r.insert(1, FieldValue::Block(Box::new(elements)))?;
                let mut materials = semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.allocate_vec(self.materials.len())?, |v: Vec<FieldValue>| {
                    for f in v {
                        semio_framework_dsl_record::native_encoding::retire_field(f);
                    }
                });
                for m in self.materials.iter() {
                    control.checkpoint()?;
                    materials.as_mut().push(FieldValue::Record(encoded_row(&[m.id.as_str(), m.name.as_str()], &[m.e, m.g, m.nu, m.rho], control)?));
                }
                r.insert(2, FieldValue::List(materials.take()))?;
                r.insert(3, DslField::to_value_controlled(&self.sections, control)?)?;
                r.insert(4, DslField::to_value_controlled(&self.solids, control)?)?;
                r.insert(5, DslField::to_value_controlled(&self.supports, control)?)?;
                r.insert(6, DslField::to_value_controlled(&self.load_cases, control)?)?;
                r.insert(7, DslField::to_value_controlled(&self.combinations, control)?)?;
                let analysis = DslField::to_value_controlled(&self.analysis, control)?;
                control.charge(std::mem::size_of::<FieldValue>())?;
                r.insert(8, FieldValue::Block(Box::new(analysis)))?;
                Ok(r.take())
            })
        })
    }
    /// 🚫️ A stateless record binder cannot issue persistent sealed ownership.
    pub fn __dsl_from_record(_: &RecordValue) -> Result<Self, semio_framework_diagnostic::TextError> {
        Err(semio_framework_diagnostic::TextError::from_value_error(unavailable(), semio_framework_diagnostic::TextSpan::default()))
    }
    /// 🚫️ Cumulative allocation alone cannot replace the required original five-axis receiving wallet.
    pub fn __dsl_from_record_controlled(_: &RecordValue, control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        control.checkpoint()?;
        Err(unavailable())
    }
}
