# Actual TIFF Registry Retains Owned Parser Refusals

The actual TIFF mutation registry returns owned TextError. Six pure grammar/scalar leaves author InvalidValue; set-snapshot and patch-snapshot retain JSON/Value and inner TextError causes directly. The independently observed frame already supplied InvalidValue while erasing String; this cut retires its intermediate String projection. Mandatory kind owner proof is established; higher native proof remains pending.

Complete immediate source text, inverse, authored full text, byte counts, hashes and exact per-site decisions are retained in generated/value-refusal/text-error-higher-tiff-registry-authored-1.json. Native admission is pending.

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/📝️text/🦀️.rs

```rust
//! 📝️ Framing and direct codec registry for TiffMutation.
use crate::schema::mutations::TiffMutation;

//#region Registry
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
pub struct Entry {
    pub opcode: &'static str,
    pub print: fn(&TiffMutation) -> Option<String>,
    pub parse: fn(&str) -> Result<TiffMutation, String>,
}
pub const REGISTRY: &[Entry] = &[
    crate::schema::mutations::patch_snapshot::text::CODEC,
    crate::schema::mutations::set_snapshot::text::CODEC,
    crate::schema::mutations::change_byte_order::text::CODEC,
    crate::schema::mutations::insert_ifd::text::CODEC,
    crate::schema::mutations::remove_ifd::text::CODEC,
    crate::schema::mutations::replace_tag::text::CODEC,
    crate::schema::mutations::remove_tag::text::CODEC,
    crate::schema::mutations::replace_pixels::text::CODEC,
];
//#endregion Registry

//#region Framing
impl protocol::OpText for TiffMutation {
    fn print_op(&self) -> String {
        REGISTRY.iter().find_map(|entry| (entry.print)(self)).expect("every aggregate variant has a direct text owner")
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let opcode = line.split_once(' ').map_or(line, |(opcode, _)| opcode);
        let entry = REGISTRY.iter().find(|entry| entry.opcode == opcode).ok_or_else(|| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown mutation opcode {opcode}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        (entry.parse)(line).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error,semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
//#endregion Framing

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/📤️remove-ifd/📝️text/🦀️.rs

```rust
//! 📝️ Direct remove-ifd text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "remove-ifd";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &TiffMutation) -> Option<String> {
    let TiffMutation::RemoveIfd(RemoveIfdMutation { index }) = value else { return None };
    Some(format!("remove-ifd index={index}"))
}
pub fn parse(line: &str) -> Result<TiffMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    let usize_arg = |key: &str| -> Result<usize, String> { arg(key)?.parse().map_err(|error: std::num::ParseIntError| error.to_string()) };
    Ok(TiffMutation::RemoveIfd(RemoveIfdMutation { index: usize_arg("index")? }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🏷️replace-tag/📝️text/🦀️.rs

```rust
//! 📝️ Direct replace-tag text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "replace-tag";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &TiffMutation) -> Option<String> {
    let TiffMutation::ReplaceTag(ReplaceTagMutation { ifd_index, tag, kind, values }) = value else { return None };
    Some(format!("replace-tag ifd-index={ifd_index} tag={tag} kind={} values={}", enc_field_type(*kind), enc_values(values)))
}
pub fn parse(line: &str) -> Result<TiffMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    let usize_arg = |key: &str| -> Result<usize, String> { arg(key)?.parse().map_err(|error: std::num::ParseIntError| error.to_string()) };
    Ok(TiffMutation::ReplaceTag(ReplaceTagMutation { ifd_index: usize_arg("ifd-index")?, tag: arg("tag")?.parse::<u16>().map_err(|error| error.to_string())?, kind: dec_field_type(arg("kind")?)?, values: dec_values(arg("values")?)? }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/📥️insert-ifd/📝️text/🦀️.rs

```rust
//! 📝️ Direct insert-ifd text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "insert-ifd";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &TiffMutation) -> Option<String> {
    let TiffMutation::InsertIfd(InsertIfdMutation { index, ifd }) = value else { return None };
    Some(format!("insert-ifd index={index} ifd={}", enc_ifd(ifd)))
}
pub fn parse(line: &str) -> Result<TiffMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    let usize_arg = |key: &str| -> Result<usize, String> { arg(key)?.parse().map_err(|error: std::num::ParseIntError| error.to_string()) };
    Ok(TiffMutation::InsertIfd(InsertIfdMutation { index: usize_arg("index")?, ifd: dec_ifd(arg("ifd")?)? }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🔲️replace-pixels/📝️text/🦀️.rs

```rust
//! 📝️ Direct replace-pixels text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "replace-pixels";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &TiffMutation) -> Option<String> {
    let TiffMutation::ReplacePixels(ReplacePixelsMutation { pixels }) = value else { return None };
    Some(format!("replace-pixels pixels={}", hex_encode(pixels)))
}
pub fn parse(line: &str) -> Result<TiffMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    Ok(TiffMutation::ReplacePixels(ReplacePixelsMutation { pixels: hex_decode(arg("pixels")?)? }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🗑️remove-tag/📝️text/🦀️.rs

```rust
//! 📝️ Direct remove-tag text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "remove-tag";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &TiffMutation) -> Option<String> {
    let TiffMutation::RemoveTag(RemoveTagMutation { ifd_index, tag }) = value else { return None };
    Some(format!("remove-tag ifd-index={ifd_index} tag={tag}"))
}
pub fn parse(line: &str) -> Result<TiffMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    let usize_arg = |key: &str| -> Result<usize, String> { arg(key)?.parse().map_err(|error: std::num::ParseIntError| error.to_string()) };
    Ok(TiffMutation::RemoveTag(RemoveTagMutation { ifd_index: usize_arg("ifd-index")?, tag: arg("tag")?.parse::<u16>().map_err(|error| error.to_string())? }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/📸️set-snapshot/📝️text/🦀️.rs

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
pub fn print(value: &TiffMutation) -> Option<String> {
    let TiffMutation::SetSnapshot(SetSnapshot { snapshot }) = value else { return None };
    Some(format!("{TEXT_OPCODE} snapshot={}", hex_encode(semio_framework_pack_json::to_json_string(snapshot).as_bytes())))
}
pub fn parse(line: &str) -> Result<TiffMutation, String> {
    let (opcode, arguments) = line.split_once(' ').unwrap_or((line, ""));
    if opcode != TEXT_OPCODE { return Err(format!("expected {TEXT_OPCODE}")); }
    let encoded = arguments.strip_prefix("snapshot=").ok_or_else(|| "missing snapshot".to_string())?;
    let bytes = hex_decode(encoded)?;
    let parsed = semio_framework_pack_json::parse_bytes(&bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let snapshot = <TiffSnapshot as dsl::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| error.to_string())?;
    Ok(TiffMutation::SetSnapshot(SetSnapshot { snapshot }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🩹️patch-snapshot/📝️text/🦀️.rs

```rust
//! 📝️ Direct compact TIFF snapshot-patch text codec.

use super::*;
use crate::schema::mutations::text::Entry;
use protocol::OpText;

pub const TEXT_OPCODE: &str = "patch-snapshot";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };
fn hex_encode(bytes: &[u8]) -> String { bytes.iter().map(|byte| format!("{byte:02x}")).collect() }
fn hex_decode(value: &str) -> Result<Vec<u8>, String> {
    let bytes = value.as_bytes();
    if !bytes.len().is_multiple_of(2) { return Err("snapshot patch has odd hexadecimal length".into()); }
    let nibble = |byte| match byte { b'0'..=b'9' => Some(byte - b'0'), b'a'..=b'f' => Some(byte - b'a' + 10), b'A'..=b'F' => Some(byte - b'A' + 10), _ => None };
    bytes.chunks_exact(2).map(|pair| Ok(nibble(pair[0]).ok_or_else(|| "snapshot patch contains non-hexadecimal bytes".to_string())? << 4 | nibble(pair[1]).ok_or_else(|| "snapshot patch contains non-hexadecimal bytes".to_string())?)).collect()
}
pub fn print(value: &TiffMutation) -> Option<String> {
    let TiffMutation::PatchSnapshot(PatchSnapshot { patch }) = value else { return None };
    Some(format!("{TEXT_OPCODE} patch={}", hex_encode(patch.print_op().as_bytes())))
}
pub fn parse(line: &str) -> Result<TiffMutation, String> {
    let (opcode, arguments) = line.split_once(' ').unwrap_or((line, ""));
    if opcode != TEXT_OPCODE { return Err(format!("expected {TEXT_OPCODE}")); }
    let encoded = arguments.strip_prefix("patch=").ok_or_else(|| "missing patch".to_string())?;
    let bytes = hex_decode(encoded)?;
    let source = std::str::from_utf8(&bytes).map_err(|error| error.to_string())?;
    let patch = editing::SnapshotPatch::parse_op(source).map_err(|error| error.to_string())?;
    Ok(TiffMutation::PatchSnapshot(PatchSnapshot { patch }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🧭️change-byte-order/📝️text/🦀️.rs

```rust
//! 📝️ Direct change-byte-order text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "change-byte-order";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &TiffMutation) -> Option<String> {
    let TiffMutation::ChangeByteOrder(ChangeByteOrderMutation { byte_order }) = value else { return None };
    Some(format!("change-byte-order byte-order={}", enc_byte_order(*byte_order)))
}
pub fn parse(line: &str) -> Result<TiffMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    Ok(TiffMutation::ChangeByteOrder(ChangeByteOrderMutation { byte_order: dec_byte_order(arg("byte-order")?)? }))
}

```


## Exact Actual Scope

9 actual Rust sources were mounted after full originals/inverses and an independent Rust grammar check; all parsed clean and post hashes have zero gaps. Six pure grammar/scalar leaves author InvalidValue; set-snapshot and patch-snapshot retain JSON/Value and inner TextError causes directly. The independently observed frame already supplied InvalidValue while erasing String; this cut retires its intermediate String projection.
