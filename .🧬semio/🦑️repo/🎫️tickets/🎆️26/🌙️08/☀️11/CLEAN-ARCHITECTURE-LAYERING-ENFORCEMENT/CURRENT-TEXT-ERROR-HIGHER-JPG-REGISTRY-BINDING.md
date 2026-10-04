# Actual JPG Registry Retains Owned Parser Refusals

The actual twelve-leaf JPG mutation registry returns TextError and the frame forwards it unchanged. Set-snapshot preserves JsonError through into_value_error and direct ValueError into an authored text span. Patch-snapshot retains its actual inner TextError directly. Ten remaining exact leaf functions have only private grammar/scalar helpers; these author InvalidValue at their syntax boundary. The mandatory kind schema and language-neutral TextError corpus are canonical. This is a caller API binding under the established missing-kind RED; whole JPG native proofs are still required and not claimed. Public parse String transports are retired at these actual owners; no implicit String conversion or compatibility constructor is introduced.

Complete immediate source text, inverse, authored full text, byte counts, hashes and exact per-site decisions are retained in generated/value-refusal/text-error-higher-jpg-registry-authored-1.json. Native admission is pending.

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/📝️text/🦀️.rs

```rust
//! 📝️ Framing and direct codec registry for JpgMutation.
use crate::schema::mutations::JpgMutation;

//#region Registry
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
pub struct Entry {
    pub opcode: &'static str,
    pub print: fn(&JpgMutation) -> Option<String>,
    pub parse: fn(&str) -> Result<JpgMutation, String>,
}
pub const REGISTRY: &[Entry] = &[
    crate::schema::mutations::patch_snapshot::text::CODEC,
    crate::schema::mutations::set_snapshot::text::CODEC,
    crate::schema::mutations::change_jfif_header::text::CODEC,
    crate::schema::mutations::replace_quant_table::text::CODEC,
    crate::schema::mutations::remove_quant_table::text::CODEC,
    crate::schema::mutations::replace_huffman_table::text::CODEC,
    crate::schema::mutations::remove_huffman_table::text::CODEC,
    crate::schema::mutations::change_restart_interval::text::CODEC,
    crate::schema::mutations::insert_other_segment::text::CODEC,
    crate::schema::mutations::remove_other_segment::text::CODEC,
    crate::schema::mutations::replace_pixels::text::CODEC,
    crate::schema::mutations::change_re_encode_quality::text::CODEC,
];
//#endregion Registry

//#region Framing
impl protocol::OpText for JpgMutation {
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

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🪓️remove-huffman/📝️text/🦀️.rs

```rust
//! 📝️ Direct remove-huffman-table text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "remove-huffman-table";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &JpgMutation) -> Option<String> {
    let JpgMutation::RemoveHuffmanTable(RemoveHuffmanTableMutation { key }) = value else { return None };
    Some(format!("remove-huffman-table key={}", enc_huffman_key(key)))
}
pub fn parse(line: &str) -> Result<JpgMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    Ok(JpgMutation::RemoveHuffmanTable(RemoveHuffmanTableMutation { key: dec_huffman_key(arg("key")?)? }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🔁️change-restart/📝️text/🦀️.rs

```rust
//! 📝️ Direct change-restart-interval text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "change-restart-interval";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &JpgMutation) -> Option<String> {
    let JpgMutation::ChangeRestartInterval(ChangeRestartIntervalMutation { restart_interval }) = value else { return None };
    Some(format!("change-restart-interval restart-interval={}", encode_option(restart_interval, |v| v.to_string())))
}
pub fn parse(line: &str) -> Result<JpgMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    Ok(JpgMutation::ChangeRestartInterval(ChangeRestartIntervalMutation { restart_interval: decode_option(arg("restart-interval")?, parse_u16)? }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🔲️replace-pixels/📝️text/🦀️.rs

```rust
//! 📝️ Direct replace-pixels text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "replace-pixels";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &JpgMutation) -> Option<String> {
    let JpgMutation::ReplacePixels(ReplacePixelsMutation { pixels }) = value else { return None };
    Some(format!("replace-pixels pixels={}", hex_encode(pixels)))
}
pub fn parse(line: &str) -> Result<JpgMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    Ok(JpgMutation::ReplacePixels(ReplacePixelsMutation { pixels: hex_decode(arg("pixels")?)? }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🎚️change-re/📝️text/🦀️.rs

```rust
//! 📝️ Direct change-re-encode-quality text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "change-re-encode-quality";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &JpgMutation) -> Option<String> {
    let JpgMutation::ChangeReEncodeQuality(ChangeReEncodeQualityMutation { quality }) = value else { return None };
    Some(format!("change-re-encode-quality quality={}", encode_option(quality, |v| v.to_string())))
}
pub fn parse(line: &str) -> Result<JpgMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    Ok(JpgMutation::ChangeReEncodeQuality(ChangeReEncodeQualityMutation { quality: decode_option(arg("quality")?, parse_u8)? }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/📥️insert-other/📝️text/🦀️.rs

```rust
//! 📝️ Direct insert-other-segment text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "insert-other-segment";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &JpgMutation) -> Option<String> {
    let JpgMutation::InsertOtherSegment(InsertOtherSegmentMutation { index, segment }) = value else { return None };
    Some(format!("insert-other-segment index={index} segment={}", enc_segment(segment)))
}
pub fn parse(line: &str) -> Result<JpgMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    Ok(JpgMutation::InsertOtherSegment(InsertOtherSegmentMutation { index: parse_usize(arg("index")?)?, segment: dec_segment(arg("segment")?)? }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🗑️remove-other/📝️text/🦀️.rs

```rust
//! 📝️ Direct remove-other-segment text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "remove-other-segment";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &JpgMutation) -> Option<String> {
    let JpgMutation::RemoveOtherSegment(RemoveOtherSegmentMutation { index }) = value else { return None };
    Some(format!("remove-other-segment index={index}"))
}
pub fn parse(line: &str) -> Result<JpgMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    Ok(JpgMutation::RemoveOtherSegment(RemoveOtherSegmentMutation { index: parse_usize(arg("index")?)? }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🌳️replace-huffman/📝️text/🦀️.rs

```rust
//! 📝️ Direct replace-huffman-table text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "replace-huffman-table";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &JpgMutation) -> Option<String> {
    let JpgMutation::ReplaceHuffmanTable(ReplaceHuffmanTableMutation { table }) = value else { return None };
    Some(format!("replace-huffman-table table={}", enc_huffman_table(table)))
}
pub fn parse(line: &str) -> Result<JpgMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    Ok(JpgMutation::ReplaceHuffmanTable(ReplaceHuffmanTableMutation { table: dec_huffman_table(arg("table")?)? }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/📸️set-snapshot/📝️text/🦀️.rs

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
pub fn print(value: &JpgMutation) -> Option<String> {
    let JpgMutation::SetSnapshot(SetSnapshot { snapshot }) = value else { return None };
    Some(format!("{TEXT_OPCODE} snapshot={}", hex_encode(semio_framework_pack_json::to_json_string(snapshot).as_bytes())))
}
pub fn parse(line: &str) -> Result<JpgMutation, String> {
    let (opcode, arguments) = line.split_once(' ').unwrap_or((line, ""));
    if opcode != TEXT_OPCODE { return Err(format!("expected {TEXT_OPCODE}")); }
    let encoded = arguments.strip_prefix("snapshot=").ok_or_else(|| "missing snapshot".to_string())?;
    let bytes = hex_decode(encoded)?;
    let parsed = semio_framework_pack_json::parse_bytes(&bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let snapshot = <JpgSnapshot as dsl::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| error.to_string())?;
    Ok(JpgMutation::SetSnapshot(SetSnapshot { snapshot }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🩹️patch-snapshot/📝️text/🦀️.rs

```rust
//! 📝️ Direct compact JPEG snapshot-patch text codec.

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
pub fn print(value: &JpgMutation) -> Option<String> {
    let JpgMutation::PatchSnapshot(PatchSnapshot { patch }) = value else { return None };
    Some(format!("{TEXT_OPCODE} patch={}", hex_encode(patch.print_op().as_bytes())))
}
pub fn parse(line: &str) -> Result<JpgMutation, String> {
    let (opcode, arguments) = line.split_once(' ').unwrap_or((line, ""));
    if opcode != TEXT_OPCODE { return Err(format!("expected {TEXT_OPCODE}")); }
    let encoded = arguments.strip_prefix("patch=").ok_or_else(|| "missing patch".to_string())?;
    let bytes = hex_decode(encoded)?;
    let source = std::str::from_utf8(&bytes).map_err(|error| error.to_string())?;
    let patch = editing::SnapshotPatch::parse_op(source).map_err(|error| error.to_string())?;
    Ok(JpgMutation::PatchSnapshot(PatchSnapshot { patch }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🧹️remove-quant/📝️text/🦀️.rs

```rust
//! 📝️ Direct remove-quant-table text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "remove-quant-table";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &JpgMutation) -> Option<String> {
    let JpgMutation::RemoveQuantTable(RemoveQuantTableMutation { id }) = value else { return None };
    Some(format!("remove-quant-table id={id}"))
}
pub fn parse(line: &str) -> Result<JpgMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    Ok(JpgMutation::RemoveQuantTable(RemoveQuantTableMutation { id: parse_u8(arg("id")?)? }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🪪️change-jfif/📝️text/🦀️.rs

```rust
//! 📝️ Direct change-jfif-header text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "change-jfif-header";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &JpgMutation) -> Option<String> {
    let JpgMutation::ChangeJfifHeader(ChangeJfifHeaderMutation { version, density_units, x_density, y_density, thumbnail }) = value else { return None };
    Some(format!("change-jfif-header version={} density-units={} x-density={x_density} y-density={y_density} thumbnail={}", enc_version(version), enc_density_units(density_units), encode_option(thumbnail, enc_thumbnail),))
}
pub fn parse(line: &str) -> Result<JpgMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    Ok(JpgMutation::ChangeJfifHeader(ChangeJfifHeaderMutation {
        version: dec_version(arg("version")?)?,
        density_units: dec_density_units(arg("density-units")?)?,
        x_density: parse_u16(arg("x-density")?)?,
        y_density: parse_u16(arg("y-density")?)?,
        thumbnail: decode_option(arg("thumbnail")?, dec_thumbnail)?,
    }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/📊️replace-quant/📝️text/🦀️.rs

```rust
//! 📝️ Direct replace-quant-table text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "replace-quant-table";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &JpgMutation) -> Option<String> {
    let JpgMutation::ReplaceQuantTable(ReplaceQuantTableMutation { table }) = value else { return None };
    Some(format!("replace-quant-table table={}", enc_quant_table(table)))
}
pub fn parse(line: &str) -> Result<JpgMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    Ok(JpgMutation::ReplaceQuantTable(ReplaceQuantTableMutation { table: dec_quant_table(arg("table")?)? }))
}

```

