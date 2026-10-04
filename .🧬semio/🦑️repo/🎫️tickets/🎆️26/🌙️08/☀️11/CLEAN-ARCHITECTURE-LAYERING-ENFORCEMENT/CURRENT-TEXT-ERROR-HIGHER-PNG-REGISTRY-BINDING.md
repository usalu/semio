# Actual PNG Registry Retains Owned Parser Refusals

The actual PNG mutation registry returns owned TextError. Sixteen pure grammar/scalar leaves author InvalidValue; set-snapshot and patch-snapshot retain JSON/Value and inner TextError causes directly. Mandatory kind owner proof is established; higher native proof remains pending.

Complete immediate source text, inverse, authored full text, byte counts, hashes and exact per-site decisions are retained in generated/value-refusal/text-error-higher-png-registry-authored-1.json. Native admission is pending.

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs

```rust
//! 📝️ Framing and direct codec registry for PngMutation.
use crate::schema::mutations::PngMutation;

//#region Registry
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
pub struct Entry {
    pub opcode: &'static str,
    pub print: fn(&PngMutation) -> Option<String>,
    pub parse: fn(&str) -> Result<PngMutation, String>,
}
pub const REGISTRY: &[Entry] = &[
    crate::schema::mutations::set_snapshot::text::CODEC,
    crate::schema::mutations::patch_snapshot::text::CODEC,
    crate::schema::mutations::change_header::text::CODEC,
    crate::schema::mutations::replace_palette::text::CODEC,
    crate::schema::mutations::change_transparency::text::CODEC,
    crate::schema::mutations::change_gamma::text::CODEC,
    crate::schema::mutations::change_chromaticities::text::CODEC,
    crate::schema::mutations::change_srgb_intent::text::CODEC,
    crate::schema::mutations::change_physical_dims::text::CODEC,
    crate::schema::mutations::change_timestamp::text::CODEC,
    crate::schema::mutations::change_background::text::CODEC,
    crate::schema::mutations::insert_text_chunk::text::CODEC,
    crate::schema::mutations::remove_text_chunk::text::CODEC,
    crate::schema::mutations::replace_text_chunk::text::CODEC,
    crate::schema::mutations::replace_pixels::text::CODEC,
    crate::schema::mutations::patch_pixels::text::CODEC,
    crate::schema::mutations::insert_unknown_chunk::text::CODEC,
    crate::schema::mutations::remove_unknown_chunk::text::CODEC,
];
//#endregion Registry

//#region Framing
impl protocol::OpText for PngMutation {
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

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-header/📝️text/🦀️.rs

```rust
//! 📝️ Direct change-header text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "change-header";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &PngMutation) -> Option<String> {
    let PngMutation::ChangeHeader(ChangeHeaderMutation { width, height, bit_depth, color_type, interlace }) = value else { return None };
    Some(format!("change-header width={width} height={height} bit-depth={bit_depth} color-type={} interlace={}", enc_color_type(*color_type), if *interlace { 1 } else { 0 },))
}
pub fn parse(line: &str) -> Result<PngMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    {
        Ok(PngMutation::ChangeHeader(ChangeHeaderMutation {
            width: parse_u32(arg("width")?)?,
            height: parse_u32(arg("height")?)?,
            bit_depth: parse_u8(arg("bit-depth")?)?,
            color_type: dec_color_type(arg("color-type")?)?,
            interlace: arg("interlace")? == "1",
        }))
    }
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️replace-pixels/📝️text/🦀️.rs

```rust
//! 📝️ Direct replace-pixels text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "replace-pixels";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &PngMutation) -> Option<String> {
    let PngMutation::ReplacePixels(ReplacePixelsMutation { pixels }) = value else { return None };
    Some(format!("replace-pixels pixels={}", hex_encode(pixels)))
}
pub fn parse(line: &str) -> Result<PngMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    Ok(PngMutation::ReplacePixels(ReplacePixelsMutation { pixels: hex_decode(arg("pixels")?)? }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕰️change-timestamp/📝️text/🦀️.rs

```rust
//! 📝️ Direct change-timestamp text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "change-timestamp";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &PngMutation) -> Option<String> {
    let PngMutation::ChangeTimestamp(ChangeTimestampMutation { time }) = value else { return None };
    Some(format!("change-timestamp time={}", encode_option(time, enc_timestamp)))
}
pub fn parse(line: &str) -> Result<PngMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    Ok(PngMutation::ChangeTimestamp(ChangeTimestampMutation { time: decode_option(arg("time")?, dec_timestamp)? }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥️insert-text-chunk/📝️text/🦀️.rs

```rust
//! 📝️ Direct insert-text-chunk text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "insert-text-chunk";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &PngMutation) -> Option<String> {
    let PngMutation::InsertTextChunk(InsertTextChunkMutation { index, chunk }) = value else { return None };
    Some(format!("insert-text-chunk index={index} chunk={}", enc_text_chunk(chunk)))
}
pub fn parse(line: &str) -> Result<PngMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    let usize_arg = |key: &str| -> Result<usize, String> { arg(key)?.parse().map_err(|error: std::num::ParseIntError| error.to_string()) };
    Ok(PngMutation::InsertTextChunk(InsertTextChunkMutation { index: usize_arg("index")?, chunk: dec_text_chunk(arg("chunk")?)? }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️change-transparency/📝️text/🦀️.rs

```rust
//! 📝️ Direct change-transparency text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "change-transparency";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &PngMutation) -> Option<String> {
    let PngMutation::ChangeTransparency(ChangeTransparencyMutation { trns }) = value else { return None };
    Some(format!("change-transparency trns={}", encode_option(trns, enc_transparency)))
}
pub fn parse(line: &str) -> Result<PngMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    Ok(PngMutation::ChangeTransparency(ChangeTransparencyMutation { trns: decode_option(arg("trns")?, dec_transparency)? }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️replace-text-chunk/📝️text/🦀️.rs

```rust
//! 📝️ Direct replace-text-chunk text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "replace-text-chunk";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &PngMutation) -> Option<String> {
    let PngMutation::ReplaceTextChunk(ReplaceTextChunkMutation { index, chunk }) = value else { return None };
    Some(format!("replace-text-chunk index={index} chunk={}", enc_text_chunk(chunk)))
}
pub fn parse(line: &str) -> Result<PngMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    let usize_arg = |key: &str| -> Result<usize, String> { arg(key)?.parse().map_err(|error: std::num::ParseIntError| error.to_string()) };
    Ok(PngMutation::ReplaceTextChunk(ReplaceTextChunkMutation { index: usize_arg("index")?, chunk: dec_text_chunk(arg("chunk")?)? }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌈️change-chromaticities/📝️text/🦀️.rs

```rust
//! 📝️ Direct change-chromaticities text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "change-chromaticities";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &PngMutation) -> Option<String> {
    let PngMutation::ChangeChromaticities(ChangeChromaticitiesMutation { chrm }) = value else { return None };
    Some(format!("change-chromaticities chrm={}", encode_option(chrm, enc_chromaticities)))
}
pub fn parse(line: &str) -> Result<PngMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    Ok(PngMutation::ChangeChromaticities(ChangeChromaticitiesMutation { chrm: decode_option(arg("chrm")?, dec_chromaticities)? }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/📝️text/🦀️.rs

```rust
//! 📝️ Direct set-snapshot text codec.

use super::*;
use crate::schema::mutations::text::Entry;

pub const TEXT_OPCODE: &str = "set-snapshot";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn hex_decode(value: &str) -> Result<Vec<u8>, String> {
    if !value.len().is_multiple_of(2) {
        return Err("snapshot payload has odd hexadecimal length".into());
    }
    (0..value.len()).step_by(2).map(|index| u8::from_str_radix(&value[index..index + 2], 16).map_err(|error| error.to_string())).collect()
}

pub fn print(value: &PngMutation) -> Option<String> {
    let PngMutation::SetSnapshot(SetSnapshot { snapshot }) = value else { return None };
    Some(format!("{TEXT_OPCODE} snapshot={}", hex_encode(semio_framework_pack_json::to_json_string(snapshot).as_bytes())))
}

pub fn parse(line: &str) -> Result<PngMutation, String> {
    let (opcode, arguments) = line.split_once(' ').unwrap_or((line, ""));
    if opcode != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let encoded = arguments.strip_prefix("snapshot=").ok_or_else(|| "missing snapshot".to_string())?;
    let bytes = hex_decode(encoded)?;
    let parsed = semio_framework_pack_json::parse_bytes(&bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let snapshot = <PngSnapshot as dsl::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| error.to_string())?;
    Ok(PngMutation::SetSnapshot(SetSnapshot { snapshot }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-background/📝️text/🦀️.rs

```rust
//! 📝️ Direct change-background text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "change-background";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &PngMutation) -> Option<String> {
    let PngMutation::ChangeBackground(ChangeBackgroundMutation { bkgd }) = value else { return None };
    Some(format!("change-background bkgd={}", encode_option(bkgd, enc_background)))
}
pub fn parse(line: &str) -> Result<PngMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    Ok(PngMutation::ChangeBackground(ChangeBackgroundMutation { bkgd: decode_option(arg("bkgd")?, dec_background)? }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️patch-snapshot/📝️text/🦀️.rs

```rust
//! 📝️ Direct compact PNG snapshot-patch text codec.

use super::*;
use crate::schema::mutations::text::Entry;
use protocol::OpText;

pub const TEXT_OPCODE: &str = "patch-snapshot";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn hex_decode(value: &str) -> Result<Vec<u8>, String> {
    let bytes = value.as_bytes();
    if !bytes.len().is_multiple_of(2) {
        return Err("snapshot patch has odd hexadecimal length".into());
    }
    let nibble = |byte| match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    };
    bytes.chunks_exact(2).map(|pair| Ok(nibble(pair[0]).ok_or_else(|| "snapshot patch contains non-hexadecimal bytes".to_string())? << 4 | nibble(pair[1]).ok_or_else(|| "snapshot patch contains non-hexadecimal bytes".to_string())?)).collect()
}

pub fn print(value: &PngMutation) -> Option<String> {
    let PngMutation::PatchSnapshot(PatchSnapshot { patch }) = value else { return None };
    Some(format!("{TEXT_OPCODE} patch={}", hex_encode(patch.print_op().as_bytes())))
}

pub fn parse(line: &str) -> Result<PngMutation, String> {
    let (opcode, arguments) = line.split_once(' ').unwrap_or((line, ""));
    if opcode != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let encoded = arguments.strip_prefix("patch=").ok_or_else(|| "missing patch".to_string())?;
    let bytes = hex_decode(encoded)?;
    let source = std::str::from_utf8(&bytes).map_err(|error| error.to_string())?;
    let patch = editing::SnapshotPatch::parse_op(source).map_err(|error| error.to_string())?;
    Ok(PngMutation::PatchSnapshot(PatchSnapshot { patch }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌗️change-gamma/📝️text/🦀️.rs

```rust
//! 📝️ Direct change-gamma text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "change-gamma";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &PngMutation) -> Option<String> {
    let PngMutation::ChangeGamma(ChangeGammaMutation { gama }) = value else { return None };
    Some(format!("change-gamma gama={}", encode_option(gama, |x: &u32| x.to_string())))
}
pub fn parse(line: &str) -> Result<PngMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    Ok(PngMutation::ChangeGamma(ChangeGammaMutation { gama: decode_option(arg("gama")?, parse_u32)? }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️patch-pixels/📝️text/🦀️.rs

```rust
//! 📝️ Direct patch-pixels text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "patch-pixels";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &PngMutation) -> Option<String> {
    let PngMutation::PatchPixels(PatchPixelsMutation { index, remove_count, pixels, move_to }) = value else { return None };
    Some(format!("patch-pixels index={index} remove-count={remove_count} pixels={} move-to={}", hex_encode(pixels), move_to.map_or_else(|| "none".into(), |value| value.to_string())))
}
pub fn parse(line: &str) -> Result<PngMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE { return Err(format!("expected {TEXT_OPCODE}")); }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    let move_to = match arg("move-to")? { "none" => None, value => Some(value.parse::<u64>().map_err(|error| error.to_string())?) };
    Ok(PngMutation::PatchPixels(PatchPixelsMutation {
        index: arg("index")?.parse::<u64>().map_err(|error| error.to_string())?,
        remove_count: arg("remove-count")?.parse::<u64>().map_err(|error| error.to_string())?,
        pixels: hex_decode(arg("pixels")?)?,
        move_to,
    }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️insert-unknown-chunk/📝️text/🦀️.rs

```rust
//! 📝️ Direct insert-unknown-chunk text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "insert-unknown-chunk";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &PngMutation) -> Option<String> {
    let PngMutation::InsertUnknownChunk(InsertUnknownChunkMutation { index, chunk }) = value else { return None };
    Some(format!("insert-unknown-chunk index={index} chunk={}", enc_chunk(chunk)))
}
pub fn parse(line: &str) -> Result<PngMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    let usize_arg = |key: &str| -> Result<usize, String> { arg(key)?.parse().map_err(|error: std::num::ParseIntError| error.to_string()) };
    Ok(PngMutation::InsertUnknownChunk(InsertUnknownChunkMutation { index: usize_arg("index")?, chunk: dec_chunk(arg("chunk")?)? }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📤️remove-unknown-chunk/📝️text/🦀️.rs

```rust
//! 📝️ Direct remove-unknown-chunk text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "remove-unknown-chunk";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &PngMutation) -> Option<String> {
    let PngMutation::RemoveUnknownChunk(RemoveUnknownChunkMutation { index }) = value else { return None };
    Some(format!("remove-unknown-chunk index={index}"))
}
pub fn parse(line: &str) -> Result<PngMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    let usize_arg = |key: &str| -> Result<usize, String> { arg(key)?.parse().map_err(|error: std::num::ParseIntError| error.to_string()) };
    Ok(PngMutation::RemoveUnknownChunk(RemoveUnknownChunkMutation { index: usize_arg("index")? }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-text-chunk/📝️text/🦀️.rs

```rust
//! 📝️ Direct remove-text-chunk text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "remove-text-chunk";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &PngMutation) -> Option<String> {
    let PngMutation::RemoveTextChunk(RemoveTextChunkMutation { index }) = value else { return None };
    Some(format!("remove-text-chunk index={index}"))
}
pub fn parse(line: &str) -> Result<PngMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    let usize_arg = |key: &str| -> Result<usize, String> { arg(key)?.parse().map_err(|error: std::num::ParseIntError| error.to_string()) };
    Ok(PngMutation::RemoveTextChunk(RemoveTextChunkMutation { index: usize_arg("index")? }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️replace-palette/📝️text/🦀️.rs

```rust
//! 📝️ Direct replace-palette text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "replace-palette";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &PngMutation) -> Option<String> {
    let PngMutation::ReplacePalette(ReplacePaletteMutation { plte }) = value else { return None };
    Some(format!("replace-palette plte={}", encode_option(plte, |v: &Vec<PngRgb>| enc_list(v, enc_rgb))))
}
pub fn parse(line: &str) -> Result<PngMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    Ok(PngMutation::ReplacePalette(ReplacePaletteMutation { plte: decode_option(arg("plte")?, |s| dec_list(s, dec_rgb))? }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️change-srgb-intent/📝️text/🦀️.rs

```rust
//! 📝️ Direct change-srgb-intent text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "change-srgb-intent";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &PngMutation) -> Option<String> {
    let PngMutation::ChangeSrgbIntent(ChangeSrgbIntentMutation { srgb }) = value else { return None };
    Some(format!("change-srgb-intent srgb={}", encode_option(srgb, |v: &PngSrgbIntent| enc_srgb_intent(*v))))
}
pub fn parse(line: &str) -> Result<PngMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    Ok(PngMutation::ChangeSrgbIntent(ChangeSrgbIntentMutation { srgb: decode_option(arg("srgb")?, dec_srgb_intent)? }))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-physical-dims/📝️text/🦀️.rs

```rust
//! 📝️ Direct change-physical-dims text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "change-physical-dims";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &PngMutation) -> Option<String> {
    let PngMutation::ChangePhysicalDims(ChangePhysicalDimsMutation { phys }) = value else { return None };
    Some(format!("change-physical-dims phys={}", encode_option(phys, enc_physical_dims)))
}
pub fn parse(line: &str) -> Result<PngMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    Ok(PngMutation::ChangePhysicalDims(ChangePhysicalDimsMutation { phys: decode_option(arg("phys")?, dec_physical_dims)? }))
}

```


## Exact Actual Scope

19 actual Rust sources were mounted after full originals/inverses and an independent Rust grammar check; all parsed clean and post hashes have zero gaps. Sixteen pure grammar/scalar leaves author InvalidValue; set-snapshot and patch-snapshot retain JSON/Value and inner TextError causes directly.
