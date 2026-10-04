# Actual BMP Registry Retains Owned Parser Refusals

The actual BMP mutation registry returns owned TextError. Five ordinary DSL leaves retain their actual parser TextError directly, including its authored span; set-snapshot retains JSON and Value causes. BMP's registry has six leaves and no patch-snapshot leaf. Mandatory kind owner proof is established; higher native proof remains pending.

Complete immediate source text, inverse, authored full text, byte counts, hashes and exact per-site decisions are retained in generated/value-refusal/text-error-higher-bmp-registry-authored-1.json. Native admission is pending.

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs

```rust
//! 📝️ Framing and direct codec registry for BmpMutation.
use crate::schema::mutations::BmpMutation;

//#region Registry
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
pub struct Entry {
    pub opcode: &'static str,
    pub print: fn(&BmpMutation) -> Option<String>,
    pub parse: fn(&str) -> Result<BmpMutation, String>,
}
pub const REGISTRY: &[Entry] = &[
    crate::schema::mutations::set_snapshot::text::CODEC,
    crate::schema::mutations::change_header_fields::text::CODEC,
    crate::schema::mutations::insert_palette_entry::text::CODEC,
    crate::schema::mutations::remove_palette_entry::text::CODEC,
    crate::schema::mutations::replace_palette_entry::text::CODEC,
    crate::schema::mutations::replace_pixel_data::text::CODEC,
];
//#endregion Registry

//#region Framing
impl protocol::OpText for BmpMutation {
    fn print_op(&self) -> String {
        REGISTRY.iter().find_map(|entry| (entry.print)(self)).expect("every aggregate variant has a direct text owner")
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let opcode = line.split_once(' ').map_or(line, |(opcode, _)| opcode);
        let entry = REGISTRY.iter().find(|entry| entry.opcode == opcode).ok_or_else(|| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown mutation opcode {opcode}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        (entry.parse)(line).map_err(|error| semio_framework_diagnostic::TextError::new(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
//#endregion Framing

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📤️remove-palette-entry/📝️text/🦀️.rs

```rust
//! 📝️ Direct remove-palette-entry text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "remove-palette-entry";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn spec() -> dsl::RecordSpec {
    let mut spec = dsl::__rt::newtype_variant_spec::<RemovePaletteEntryMutation>();
    spec.keyword = Some(TEXT_OPCODE.into());
    spec
}
pub fn print(value: &BmpMutation) -> Option<String> {
    let BmpMutation::RemovePaletteEntry(payload) = value else { return None };
    Some(dsl::print(&dsl::__rt::newtype_variant_to_record(payload), &spec(), dsl::JoinMode::Inline))
}
pub fn parse(line: &str) -> Result<BmpMutation, String> {
    let value = dsl::parse(line, &spec(), &dsl::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: dsl::SourceMode::Inline }).map_err(|error| error.to_string())?;
    dsl::__rt::newtype_variant_from_record(&value).map(BmpMutation::RemovePaletteEntry).map_err(|error| error.to_string())
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️replace-palette-entry/📝️text/🦀️.rs

```rust
//! 📝️ Direct replace-palette-entry text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "replace-palette-entry";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn spec() -> dsl::RecordSpec {
    let mut spec = dsl::__rt::newtype_variant_spec::<ReplacePaletteEntryMutation>();
    spec.keyword = Some(TEXT_OPCODE.into());
    spec
}
pub fn print(value: &BmpMutation) -> Option<String> {
    let BmpMutation::ReplacePaletteEntry(payload) = value else { return None };
    Some(dsl::print(&dsl::__rt::newtype_variant_to_record(payload), &spec(), dsl::JoinMode::Inline))
}
pub fn parse(line: &str) -> Result<BmpMutation, String> {
    let value = dsl::parse(line, &spec(), &dsl::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: dsl::SourceMode::Inline }).map_err(|error| error.to_string())?;
    dsl::__rt::newtype_variant_from_record(&value).map(BmpMutation::ReplacePaletteEntry).map_err(|error| error.to_string())
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-header-fields/📝️text/🦀️.rs

```rust
//! 📝️ Direct change-header-fields text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "change-header-fields";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn spec() -> dsl::RecordSpec {
    let mut spec = dsl::__rt::newtype_variant_spec::<ChangeHeaderFieldsMutation>();
    spec.keyword = Some(TEXT_OPCODE.into());
    spec
}
pub fn print(value: &BmpMutation) -> Option<String> {
    let BmpMutation::ChangeHeaderFields(payload) = value else { return None };
    Some(dsl::print(&dsl::__rt::newtype_variant_to_record(payload), &spec(), dsl::JoinMode::Inline))
}
pub fn parse(line: &str) -> Result<BmpMutation, String> {
    let value = dsl::parse(line, &spec(), &dsl::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: dsl::SourceMode::Inline }).map_err(|error| error.to_string())?;
    dsl::__rt::newtype_variant_from_record(&value).map(BmpMutation::ChangeHeaderFields).map_err(|error| error.to_string())
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/📝️text/🦀️.rs

```rust
//! 📝️ Direct set-snapshot text codec.

use super::*;
use crate::schema::mutations::text::Entry;

pub const TEXT_OPCODE: &str = "set-snapshot";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

fn hex_encode(bytes: &[u8]) -> String { bytes.iter().map(|byte| format!("{byte:02x}")).collect() }
fn hex_decode(value: &str) -> Result<Vec<u8>, String> {
    if !value.len().is_multiple_of(2) { return Err("snapshot payload has odd hexadecimal length".into()); }
    (0..value.len()).step_by(2).map(|index| u8::from_str_radix(&value[index..index + 2], 16).map_err(|error| error.to_string())).collect()
}
pub fn print(value: &BmpMutation) -> Option<String> {
    let BmpMutation::SetSnapshot(SetSnapshot { snapshot }) = value else { return None };
    Some(format!("{TEXT_OPCODE} snapshot={}", hex_encode(semio_framework_pack_json::to_json_string(snapshot).as_bytes())))
}
pub fn parse(line: &str) -> Result<BmpMutation, String> {
    let (opcode, arguments) = line.split_once(' ').unwrap_or((line, ""));
    if opcode != TEXT_OPCODE { return Err(format!("expected {TEXT_OPCODE}")); }
    let encoded = arguments.strip_prefix("snapshot=").ok_or_else(|| "missing snapshot".to_string())?;
    let bytes = hex_decode(encoded)?;
    let parsed = semio_framework_pack_json::parse_bytes(&bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let snapshot = <BmpSnapshot as dsl::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| error.to_string())?;
    Ok(BmpMutation::SetSnapshot(SetSnapshot { snapshot }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️replace-pixel-data/📝️text/🦀️.rs

```rust
//! 📝️ Direct replace-pixel-data text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "replace-pixel-data";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn spec() -> dsl::RecordSpec {
    let mut spec = dsl::__rt::newtype_variant_spec::<ReplacePixelDataMutation>();
    spec.keyword = Some(TEXT_OPCODE.into());
    spec
}
pub fn print(value: &BmpMutation) -> Option<String> {
    let BmpMutation::ReplacePixelData(payload) = value else { return None };
    Some(dsl::print(&dsl::__rt::newtype_variant_to_record(payload), &spec(), dsl::JoinMode::Inline))
}
pub fn parse(line: &str) -> Result<BmpMutation, String> {
    let value = dsl::parse(line, &spec(), &dsl::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: dsl::SourceMode::Inline }).map_err(|error| error.to_string())?;
    dsl::__rt::newtype_variant_from_record(&value).map(BmpMutation::ReplacePixelData).map_err(|error| error.to_string())
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥️insert-palette-entry/📝️text/🦀️.rs

```rust
//! 📝️ Direct insert-palette-entry text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "insert-palette-entry";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn spec() -> dsl::RecordSpec {
    let mut spec = dsl::__rt::newtype_variant_spec::<InsertPaletteEntryMutation>();
    spec.keyword = Some(TEXT_OPCODE.into());
    spec
}
pub fn print(value: &BmpMutation) -> Option<String> {
    let BmpMutation::InsertPaletteEntry(payload) = value else { return None };
    Some(dsl::print(&dsl::__rt::newtype_variant_to_record(payload), &spec(), dsl::JoinMode::Inline))
}
pub fn parse(line: &str) -> Result<BmpMutation, String> {
    let value = dsl::parse(line, &spec(), &dsl::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: dsl::SourceMode::Inline }).map_err(|error| error.to_string())?;
    dsl::__rt::newtype_variant_from_record(&value).map(BmpMutation::InsertPaletteEntry).map_err(|error| error.to_string())
}

```


## Exact Actual Scope

7 actual Rust sources were mounted after full originals/inverses and an independent Rust grammar check; all parsed clean and post hashes have zero gaps. Five ordinary DSL leaves retain their actual parser TextError directly, including its authored span; set-snapshot retains JSON and Value causes. BMP's registry has six leaves and no patch-snapshot leaf.
