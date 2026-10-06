/// 🫳️ An authored field exposes its finite lazy borrowed schema without constructing metadata owners.
pub trait BorrowedDslField {
    const SHAPE:crate::BorrowedShape;
}
/// 📑️ An authored record returns only static metadata references and lazy shape producers.
pub trait BorrowedDslRecord {
    const RECORD:crate::BorrowedRecordSpec;
}
/// 🌿️ An authored tagged sum exposes immutable literal identities and original variant metadata.
pub trait BorrowedDslVariants {
    const VARIANTS:&'static[(&'static str,fn()->crate::BorrowedRecordSpec)];
    fn projected_borrowed_variant_identity(&self)->(&'static str,usize,crate::BorrowedRecordSpec);
}

macro_rules! borrowed_scalar_fields {
    ($shape:ident;$($owner:ty),+)=>{$(impl BorrowedDslField for $owner{const SHAPE:crate::BorrowedShape=crate::BorrowedShape::$shape;})+};
}
borrowed_scalar_fields!(Bool;bool);
borrowed_scalar_fields!(Int;i8,i16,i32,i64,isize);
borrowed_scalar_fields!(UInt;u8,u16,u32,u64,usize);
borrowed_scalar_fields!(Float;f32,f64);
borrowed_scalar_fields!(Text;String);
borrowed_scalar_fields!(Wire;crate::Wire);
borrowed_scalar_fields!(Value;semio_framework_value::DslValue);
borrowed_scalar_fields!(Value;semio_framework_value::ValueType);
/// 🔗️ Reads a finite shape while retaining lazy recursive type edges.
pub fn borrowed_field_shape<T:BorrowedDslField>()->crate::BorrowedShape{T::SHAPE}
/// 📚️ Reads one statically authored record without a schema allocation or eager recursion.
pub fn borrowed_record<T:BorrowedDslRecord>()->crate::BorrowedRecordSpec{T::RECORD}
/// 📏️ Resolves authored unit metadata during constant construction.
pub const fn borrowed_unit(symbol:&'static str)->&'static semio_framework_dsl::UnitSpec{match semio_framework_dsl::unit_by_symbol(symbol){Some(unit)=>unit,None=>panic!("borrowed schema declares an unknown unit")}}
impl<T:BorrowedDslField> BorrowedDslField for Box<T>{const SHAPE:crate::BorrowedShape=T::SHAPE;}
impl<T:BorrowedDslRecord> BorrowedDslRecord for Box<T>{const RECORD:crate::BorrowedRecordSpec=T::RECORD;}
impl<T:BorrowedDslField> BorrowedDslField for Vec<T>{const SHAPE:crate::BorrowedShape=crate::BorrowedShape::List(borrowed_field_shape::<T>);}
impl<T:BorrowedDslField> BorrowedDslField for std::collections::BTreeMap<String,T>{const SHAPE:crate::BorrowedShape=crate::BorrowedShape::Map(borrowed_field_shape::<T>);}
impl<T:BorrowedDslField,const N:usize> BorrowedDslField for [T;N]{const SHAPE:crate::BorrowedShape=crate::BorrowedShape::Tuple(borrowed_field_shape::<T>,Some(N));}
