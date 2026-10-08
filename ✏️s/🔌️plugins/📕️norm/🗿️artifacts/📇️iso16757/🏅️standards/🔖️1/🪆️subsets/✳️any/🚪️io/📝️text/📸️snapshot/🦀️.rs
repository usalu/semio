//! 📜️ ISO 16757 app — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::Iso16757Snapshot;

/// 📄️ The `default` example document, handcrafted in the `.iso16757` DSL — a demo HVAC catalogue
/// worked example (control valve product group/class/series/product/variant, ISO 16757-4 dictionary
/// subject/property/controlled list, a box-primitive geometry with an inlet port, a selection
/// request, and a scripted part-number rule), mirroring `Document::reference_fixture()`.
pub const ISO16757_DEFAULT_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.iso16757` DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<Iso16757Snapshot, semio_framework_diagnostic::TextError> {
    <Iso16757Snapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.iso16757` DSL text.
pub fn print_dsl(document: &Iso16757Snapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Iso16757SnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{part_1, part_2, part_4, part_5, CatalogueValue};
use framework_schema::ArtifactSchema;
use std::collections::BTreeMap;

/// 📤️ The canonical JSON projection of a [`Iso16757Snapshot`] — the surface
/// `../../../../../🧪️tests/📇️mutate-iso16757-1` is compared through under `ordered-json-v1`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_iso16757_snapshot_json(snapshot: &Iso16757Snapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📥️ The `serde_json` inverse of [`encode_iso16757_snapshot_json`] — decodes the committed
/// `../🧬️mutations/<kind>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors into real [`Iso16757Snapshot`] values, so the case adapter reads the committed
/// fixture instead of re-declaring it as a Rust literal beside it. Reaching `serde_json` from that
/// adapter is impossible — the generated test host links only this crate — which is why the bridge
/// belongs here.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_iso16757_snapshot_json(text: &str) -> Result<Iso16757Snapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📖️ Parses the committed `.dsl.semio` artifact into a [`Iso16757Snapshot`]. Calls the `ArtifactDsl`
/// trait method directly rather than the `📝️text` facet's async wrapper, because a test host has no
/// async runtime to drive one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_iso16757_dsl(text: &str) -> Result<Iso16757Snapshot, String> {
    <Iso16757Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 🖨️ Prints a [`Iso16757Snapshot`] back to its canonical `.dsl.semio` body. Canonical is the operative
/// word: the committed example assets ARE this function's own output, which is why the identity
/// scenario asserts byte-exactness rather than the no-byte-pass-through inequality.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_iso16757_dsl(snapshot: &Iso16757Snapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{part_1, part_2, part_4, part_5, CatalogueValue};
use framework_schema::ArtifactSchema;
use std::collections::BTreeMap;




}
pub use snapshot_wire_codec::*;

crate::impl_norm_artifact_record!(@text crate::Iso16757Snapshot, extension="iso16757", envelope_id="norm.iso16757");

use crate::{CatalogueId,CatalogueValue,part_5::PartNumberRule};

impl semio_framework_dsl_record::DslField for CatalogueId {
    fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{<String as semio_framework_dsl_record::DslField>::shape_controlled(control)}
    fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,semio_framework_value::ValueError>{<String as semio_framework_dsl_record::DslField>::to_value_controlled(&self.0,control)}
    fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{<String as semio_framework_dsl_record::DslField>::from_value_controlled(value,control).map(Self)}

    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Text
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Text(self.0.clone())
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Text(s) => Ok(CatalogueId(s.clone())),
            other => Err(format!("expected Text, found {other:?}")),
        }
    }
}

impl semio_framework_dsl_record::DslField for CatalogueValue {
    fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{control.checkpoint()?;Ok(semio_framework_dsl_record::Shape::Value)}
    fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,semio_framework_value::ValueError>{semio_framework_value::ToValue::to_value_controlled(self,control).map(semio_framework_dsl_record::FieldValue::Value)}
    fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{native_decoding::catalogue_value(value,control)}
    fn retire_decoded(self){crate::standards::v1::subsets::any::schema::snapshot::retire_decoded_catalogue_value(self)}

    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Value
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Value(semio_framework_value::ToValue::to_value(self))
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Value(dsl_value) => {
                semio_framework_value::FromValue::from_value(dsl_value.clone()).map_err(|error|error.to_string())
            }
            other => Err(format!("expected Value, found {other:?}")),
        }
    }
}

impl semio_framework_dsl_record::DslField for PartNumberRule {
    fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{control.checkpoint()?;Ok(semio_framework_dsl_record::Shape::Value)}
    fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,semio_framework_value::ValueError>{semio_framework_value::ToValue::to_value_controlled(self,control).map(semio_framework_dsl_record::FieldValue::Value)}
        fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{native_decoding::part_number(value,control)}

        fn shape() -> semio_framework_dsl_record::Shape {
            semio_framework_dsl_record::Shape::Value
        }
        fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
            semio_framework_dsl_record::FieldValue::Value(semio_framework_value::ToValue::to_value(self))
        }
        fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
            match value {
                semio_framework_dsl_record::FieldValue::Value(dsl_value) => semio_framework_value::FromValue::from_value(dsl_value.clone()).map_err(|error|error.to_string()),
                other => Err(format!("expected Value, found {other:?}")),
            }
        }
    }

impl semio_framework_dsl_record::BorrowedDslField for CatalogueId { const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Text; }
impl semio_framework_dsl_record::BorrowedDslField for CatalogueValue { const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Value; }
impl semio_framework_dsl_record::BorrowedDslField for PartNumberRule { const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Value; }

#[path = "🛬️native/🦀️.rs"]
pub(crate) mod native_decoding;
