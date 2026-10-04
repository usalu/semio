# Higher PDF 1.4 Registry Typed Refusals

The actual five payload owners retain PackJSON and Value refusal authority directly in TextError; the registry/frame retains that object. Only authored frame and hex grammar failures are InvalidValue at span 1:1. This binding preserves original print/grammar semantics and full originals. Whole native consumer proof is pending; bounded original Diagnostic ten-law proof is separate.

Complete immediate source text, inverse, authored full text, byte counts, hashes and exact per-site decisions are retained in generated/value-refusal/text-error-higher-pdf14-registry-authored-1.json. Native admission is pending.

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📝️text/🦀️.rs

```rust
//! 📝️ PDF 1.4 mutation text framing and executable direct-leaf registry.

use super::PdfMutation;
use protocol::OpText;

//#region 🔖️Grammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 🔖️Grammar

//#region 🔖️Registry
type Printer = fn(&PdfMutation) -> Option<String>;
type Parser = fn(&str) -> Result<PdfMutation, String>;
pub const REGISTRY: &[(&str, Printer, Parser)] = &[
    (super::insert_page::text::OPCODE, super::insert_page::text::print, super::insert_page::text::parse),
    (super::remove_page::text::OPCODE, super::remove_page::text::print, super::remove_page::text::parse),
    (super::move_page::text::OPCODE, super::move_page::text::print, super::move_page::text::parse),
    (super::resize_page::text::OPCODE, super::resize_page::text::print, super::resize_page::text::parse),
    (super::replace_page_text::text::OPCODE, super::replace_page_text::text::print, super::replace_page_text::text::parse),
];
//#endregion 🔖️Registry

//#region 🔖️Framing
pub(super) fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub(super) fn unhex(text: &str) -> Result<Vec<u8>, String> {
    if !text.len().is_multiple_of(2) || !text.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err("Invalid hexadecimal payload".into());
    }
    text.as_bytes().as_chunks::<2>().0.iter().map(|pair| u8::from_str_radix(std::str::from_utf8(pair).map_err(|error| error.to_string())?, 16).map_err(|error| error.to_string())).collect()
}

impl OpText for PdfMutation {
    fn print_op(&self) -> String {
        REGISTRY.iter().find_map(|(opcode, print, _)| print(self).map(|payload| format!("{opcode} payload={payload}"))).expect("Every mutation has one direct text owner")
    }

    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let parse = || -> Result<Self, String> {
            let (opcode, payload) = line.split_once(" payload=").ok_or("Expected opcode and payload")?;
            let (_, _, parser) = REGISTRY.iter().find(|(identity, _, _)| *identity == opcode).ok_or("Unknown PDF 1.4 mutation opcode")?;
            parser(payload)
        };
        parse().map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️Framing

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📥️insert-page/📝️text/🦀️.rs

```rust
//! 📝️ insert-page text payload owner.

use super::super::{
    text::{hex, unhex},
    PdfMutation,
};
use super::InsertPage;

//#region 🔖️Codec
pub const OPCODE: &str = "insert-page";

pub fn print(mutation: &PdfMutation) -> Option<String> {
    let PdfMutation::InsertPage(payload) = mutation else {
        return None;
    };
    Some(hex(&semio_framework_pack_json::to_json_string(payload).into_bytes()))
}

pub fn parse(payload: &str) -> Result<PdfMutation, String> {
    let bytes = unhex(payload)?;
    let parsed = semio_framework_pack_json::parse_bytes(&bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    <InsertPage as dsl::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map(PdfMutation::InsertPage).map_err(|error| error.to_string())
}
//#endregion 🔖️Codec

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🗑️remove-page/📝️text/🦀️.rs

```rust
//! 📝️ remove-page text payload owner.

use super::super::{
    text::{hex, unhex},
    PdfMutation,
};
use super::RemovePage;

//#region 🔖️Codec
pub const OPCODE: &str = "remove-page";

pub fn print(mutation: &PdfMutation) -> Option<String> {
    let PdfMutation::RemovePage(payload) = mutation else {
        return None;
    };
    Some(hex(&semio_framework_pack_json::to_json_string(payload).into_bytes()))
}

pub fn parse(payload: &str) -> Result<PdfMutation, String> {
    let bytes = unhex(payload)?;
    let parsed = semio_framework_pack_json::parse_bytes(&bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    <RemovePage as dsl::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map(PdfMutation::RemovePage).map_err(|error| error.to_string())
}
//#endregion 🔖️Codec

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🔀️move-page/📝️text/🦀️.rs

```rust
//! 📝️ move-page text payload owner.

use super::super::{
    text::{hex, unhex},
    PdfMutation,
};
use super::MovePage;

//#region 🔖️Codec
pub const OPCODE: &str = "move-page";

pub fn print(mutation: &PdfMutation) -> Option<String> {
    let PdfMutation::MovePage(payload) = mutation else {
        return None;
    };
    Some(hex(&semio_framework_pack_json::to_json_string(payload).into_bytes()))
}

pub fn parse(payload: &str) -> Result<PdfMutation, String> {
    let bytes = unhex(payload)?;
    let parsed = semio_framework_pack_json::parse_bytes(&bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    <MovePage as dsl::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map(PdfMutation::MovePage).map_err(|error| error.to_string())
}
//#endregion 🔖️Codec

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📐️resize-page/📝️text/🦀️.rs

```rust
//! 📝️ resize-page text payload owner.

use super::super::{
    text::{hex, unhex},
    PdfMutation,
};
use super::ResizePage;

//#region 🔖️Codec
pub const OPCODE: &str = "resize-page";

pub fn print(mutation: &PdfMutation) -> Option<String> {
    let PdfMutation::ResizePage(payload) = mutation else {
        return None;
    };
    Some(hex(&semio_framework_pack_json::to_json_string(payload).into_bytes()))
}

pub fn parse(payload: &str) -> Result<PdfMutation, String> {
    let bytes = unhex(payload)?;
    let parsed = semio_framework_pack_json::parse_bytes(&bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    <ResizePage as dsl::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map(PdfMutation::ResizePage).map_err(|error| error.to_string())
}
//#endregion 🔖️Codec

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/♻️replace-page-text/📝️text/🦀️.rs

```rust
//! 📝️ replace-page-text text payload owner.

use super::super::{
    text::{hex, unhex},
    PdfMutation,
};
use super::ReplacePageText;

//#region 🔖️Codec
pub const OPCODE: &str = "replace-page-text";

pub fn print(mutation: &PdfMutation) -> Option<String> {
    let PdfMutation::ReplacePageText(payload) = mutation else {
        return None;
    };
    Some(hex(&semio_framework_pack_json::to_json_string(payload).into_bytes()))
}

pub fn parse(payload: &str) -> Result<PdfMutation, String> {
    let bytes = unhex(payload)?;
    let parsed = semio_framework_pack_json::parse_bytes(&bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    <ReplacePageText as dsl::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map(PdfMutation::ReplacePageText).map_err(|error| error.to_string())
}
//#endregion 🔖️Codec

```

