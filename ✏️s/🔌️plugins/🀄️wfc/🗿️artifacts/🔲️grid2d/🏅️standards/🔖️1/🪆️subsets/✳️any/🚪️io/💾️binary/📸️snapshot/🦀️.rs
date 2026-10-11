//! 📦️ `s.wfc.grid2d` — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::schema::snapshot::Grid2dSnapshot;
use store::{ErasedSnapshotRetirement, PackError};

/// 📦️ Encodes a `Grid2dSnapshot` to its binary pack form.
pub fn encode(document: &Grid2dSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Grid2dSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Grid2dSnapshot, PackError> {
    <Grid2dSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region ♻️Retirement
/// 📖️ Decodes into a LIVE projection and hands back the displaced document as a bounded retirement
/// admitted under the caller's own five-currency grant — the only decode entry point a mounted store
/// may call, because it never drops the old snapshot inline. A refused admission restores the exact
/// displaced document into `target` and drops only the freshly decoded, never-published one.
pub fn decode_into(target: &mut Grid2dSnapshot, bytes: &[u8], grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, semio_framework_value::retained_clone::RetainedCloneProgress), PackError> {
    let next = decode(bytes)?;
    let displaced = std::mem::replace(target, next);
    match semio_framework_value::retirement::admit_owned_retirement(displaced, grant) {
        Ok(admitted) => Ok(admitted),
        Err((error, displaced)) => {
            drop(std::mem::replace(target, displaced));
            Err(PackError::from(error))
        }
    }
}
//#endregion ♻️Retirement

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `encode`/`decode` speak, named as the schema names the export.
pub type Grid2dSnapshotBinary = Vec<u8>;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_stdio_semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::standards::v1::subsets::any::io::text::snapshot::*;
const BASE64_ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
/// 🔤️ Encodes one palette index per pixel, row-major, as the base64 stream `Bitmap.pixels` carries.
/// Dependency-free by repo rule — no runtime library is pulled in for a 24-character transform.
pub fn encode_palette_indices(indices: &[u8]) -> String {
    let mut out = String::with_capacity(indices.len().div_ceil(3) * 4);
    for chunk in indices.chunks(3) {
        let taken = chunk.len();
        let mut block = 0u32;
        for (offset, byte) in chunk.iter().enumerate() {
            block |= u32::from(*byte) << (16 - 8 * offset);
        }
        for slot in 0..=taken {
            out.push(char::from(BASE64_ALPHABET[((block >> (18 - 6 * slot)) & 0x3f) as usize]));
        }
        for _ in taken..3 {
            out.push('=');
        }
    }
    out
}
/// 🔤️ The exact inverse of [`encode_palette_indices`]. A malformed stream yields the prefix it
/// could read, never a panic — a tile is media, not a protocol.
pub fn decode_palette_indices(pixels: &str) -> Vec<u8> {
    fn value(byte: u8) -> Option<u8> {
        match byte {
            b'A'..=b'Z' => Some(byte - b'A'),
            b'a'..=b'z' => Some(byte - b'a' + 26),
            b'0'..=b'9' => Some(byte - b'0' + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    }
    let clean: Vec<u8> = pixels.bytes().filter(|byte| *byte != b'=' && !byte.is_ascii_whitespace()).collect();
    let mut out = Vec::with_capacity(clean.len() * 3 / 4);
    for chunk in clean.chunks(4) {
        let Some(values) = chunk.iter().map(|byte| value(*byte)).collect::<Option<Vec<u8>>>() else {
            return out;
        };
        let taken = values.len();
        let combined = values.iter().fold(0u32, |accumulator, value| (accumulator << 6) | u32::from(*value)) << ((4 - taken) * 6);
        out.push((combined >> 16) as u8);
        if taken > 2 {
            out.push((combined >> 8) as u8);
        }
        if taken > 3 {
            out.push(combined as u8);
        }
    }
    out
}
}
pub use native_snapshot_codec::*;

mod native_codec {
use super::*;
use crate::schema::snapshot::{Grid2dSnapshot, WfcAdjacencyRule2d, WfcCell2d, WfcDirection2d, WfcPinnedCell2d, WfcTile2d, WfcTileMedia2d, WFC_GRID2D_DOCUMENT_SCHEMA};
use crate::standards::v1::subsets::any::io::text::snapshot::{Grid2dSnapshotDsl,grid2d_document_to_dsl,grid2d_document_from_dsl};

impl store::ArtifactPack for Grid2dSnapshotDsl {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl store::ArtifactPack for Grid2dSnapshot {
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> { Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec()) }
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        <Grid2dSnapshotDsl as store::ArtifactPack>::encode_pack_with(&grid2d_document_to_dsl(self), options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let parsed = <Grid2dSnapshotDsl as store::ArtifactPack>::decode_pack_with(bytes, options)?;
        grid2d_document_from_dsl(parsed).map_err(store::text_error_to_pack_error)
    }

    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        <Grid2dSnapshotDsl as store::ArtifactPack>::record_spec()
    }
}
}

/// 🫴️ Declares this owner’s actual controlled native record factories.
impl store::ArtifactNativeSnapshot for crate::Grid2dSnapshot {
 fn decode_native_snapshot(payload:store::NativeSnapshotInput<'_>,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{store::decode_native_snapshot_record(payload,<Self as store::ArtifactDsl>::envelope_id(),crate::standards::v1::subsets::any::io::text::snapshot::Grid2dSnapshotDsl::__dsl_spec_producer(),|record,native|{let maximum=native.maximum_bytes();crate::standards::v1::subsets::any::io::text::snapshot::controlled_native::construct(record,native,maximum)},control)}
 fn encode_native_snapshot(&self,encoding:store::NativeSnapshotEncoding,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<store::io_schema::IoPayload,semio_framework_value::ValueError>{store::encode_native_snapshot_record(encoding,<Self as store::ArtifactDsl>::envelope_id(),crate::standards::v1::subsets::any::io::text::snapshot::Grid2dSnapshotDsl::__dsl_spec_producer(),|native|crate::standards::v1::subsets::any::io::text::snapshot::controlled_native::project(self,native),control)}
}
