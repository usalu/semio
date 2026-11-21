//! 🔣️ Candidate owned binding for the authored QuestionDictionary entry literal.

use super::QuestionDictionaryEntry;
use semio_framework_value::{DslValue, Number, ValueError, ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::{
    NativeReceive,
    owned::{
        NativeMappedReceiver, NativeReceiveMap,
        tree::{NativeScalarReceiver, NativeScalarValue, NativeValueSource},
    },
};
use std::mem::{ManuallyDrop, size_of};

fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "dictionary numeric limb or octet is out of range")
}

fn integer<I: NativeValueSource + ?Sized>(source: &I) -> Result<u64, ValueError> {
    match source.number()? {
        Number::UInt(value) => Ok(value),
        Number::Int(value) => u64::try_from(value).map_err(|_| invalid()),
        Number::Float(_) => Err(invalid()),
    }
}

macro_rules! wire_integer {
    ($name:ident, $inner:ty) => {
        #[repr(transparent)]
        #[derive(semio_framework_value::RetireOwned)]
        struct $name($inner);
        impl<I: NativeValueSource + ?Sized> NativeScalarValue<I> for $name {
            fn read(source: &I) -> Result<Self, ValueError> {
                <$inner>::try_from(integer(source)?).map(Self).map_err(|_| invalid())
            }

            fn read_copy_bytes(_: &I) -> Result<usize, ValueError> {
                Ok(size_of::<Option<Self>>())
            }
        }
        impl<I: NativeValueSource + ?Sized> NativeReceive<I> for $name {
            type Receiver = NativeScalarReceiver<I, Self>;

            fn receiver() -> Self::Receiver {
                Self::Receiver::new()
            }
        }
    };
}

wire_integer!(Limb, u32);
wire_integer!(Octet, u8);

fn word(high: Limb, low: Limb) -> u64 {
    (u64::from(high.0) << 32) | u64::from(low.0)
}

#[repr(transparent)]
#[derive(semio_framework_value::RetireOwned)]
struct Literal(DslValue);

#[repr(transparent)]
#[derive(semio_framework_value::RetireOwned)]
struct Member((String, DslValue));

#[derive(semio_framework_value::NativeReceive, semio_framework_value::RetireOwned)]
#[value(deny_unknown_fields)]
struct MemberWire {
    name: String,
    value: Literal,
}

impl NativeReceiveMap<MemberWire> for Member {
    fn from_native(value: MemberWire) -> Self {
        Self((value.name, value.value.0))
    }
}

impl<I: NativeValueSource + ?Sized> NativeReceive<I> for Member {
    type Receiver = NativeMappedReceiver<I, MemberWire, Self>;

    fn receiver() -> Self::Receiver {
        Self::Receiver::new()
    }
}

#[derive(semio_framework_value::NativeReceive, semio_framework_value::RetireOwned)]
#[value(tag = "kind", rename_all = "camelCase", deny_unknown_fields, receive_recursive)]
enum LiteralWire {
    Null,
    Boolean { value: bool },
    Unsigned { high: Limb, low: Limb },
    Signed { high: Limb, low: Limb },
    Float { high: Limb, low: Limb },
    Text { value: String },
    Bytes { value: Vec<Octet> },
    Array { items: Vec<Literal> },
    Object { members: Vec<Member> },
}

/// 📦️ Transfers the buffer of initialized transparent wrappers to their sole field type.
/// Each caller must use the wrapper's exact field type and no wrapper-specific destructor.
unsafe fn transfer_buffer<Wrapper, Value>(values: Vec<Wrapper>) -> Vec<Value> {
    let mut values = ManuallyDrop::new(values);
    unsafe { Vec::from_raw_parts(values.as_mut_ptr().cast(), values.len(), values.capacity()) }
}

impl NativeReceiveMap<LiteralWire> for Literal {
    fn from_native(value: LiteralWire) -> Self {
        Self(match value {
            LiteralWire::Null => DslValue::Null,
            LiteralWire::Boolean { value } => DslValue::Bool(value),
            LiteralWire::Unsigned { high, low } => DslValue::uint(word(high, low)),
            LiteralWire::Signed { high, low } => DslValue::int(word(high, low) as i64),
            LiteralWire::Float { high, low } => DslValue::float(f64::from_bits(word(high, low))),
            LiteralWire::Text { value } => DslValue::String(value),
            LiteralWire::Bytes { value } => DslValue::Bytes(unsafe { transfer_buffer(value) }),
            LiteralWire::Array { items } => DslValue::Array(unsafe { transfer_buffer(items) }),
            LiteralWire::Object { members } => DslValue::Object(unsafe { transfer_buffer(members) }),
        })
    }
}

impl<I: NativeValueSource + ?Sized> NativeReceive<I> for Literal {
    type Receiver = NativeMappedReceiver<I, LiteralWire, Self>;

    fn receiver() -> Self::Receiver {
        Self::Receiver::new()
    }
}

/// 📎️ The entry's wire fields preserve both its identity and the native typed literal.
#[derive(semio_framework_value::NativeReceive, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct EntryWire {
    question_id: String,
    value: Literal,
}

impl NativeReceiveMap<EntryWire> for QuestionDictionaryEntry {
    fn from_native(entry: EntryWire) -> Self {
        Self { question_id: entry.question_id, value: entry.value.0 }
    }
}

impl<I: NativeValueSource + ?Sized> NativeReceive<I> for QuestionDictionaryEntry {
    type Receiver = NativeMappedReceiver<I, EntryWire, Self>;

    fn receiver() -> Self::Receiver {
        Self::Receiver::new()
    }
}
