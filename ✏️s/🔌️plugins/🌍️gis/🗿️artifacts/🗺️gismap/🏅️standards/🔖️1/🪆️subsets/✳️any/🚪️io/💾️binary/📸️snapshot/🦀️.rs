//! 📦️ GIS map artifact — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::GisMapSnapshot;
use store::PackError;

/// 📦️ Encodes a `GisMapSnapshot` to its binary pack form.
pub fn encode(document: &GisMapSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `GisMapSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<GisMapSnapshot, PackError> {
    <GisMapSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests



#[path = "📦️pack/🦀️.rs"]
pub(crate) mod owned_pack;

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{gis_map_drawing_child_handle, gis_map_value_child_handle, GisMapDrawingChild, GisMapImageChild, GisMapValueChild, MapFeature};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};
use crate::standards::v1::subsets::any::io::text::snapshot::*;
pub(crate) fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
pub(crate) fn write_ref(out: &mut Vec<u8>, r: &store::os_io::ArtifactRef) {
    write_str_lp(out, &r.to_uri());
}
pub(crate) fn write_child<S>(out: &mut Vec<u8>, c: &store::ArtifactChild<S>) {
    write_str_lp(out, &c.child_id);
    write_ref(out, &c.target);
}
pub(crate) fn write_child_opt<S>(out: &mut Vec<u8>, c: &Option<store::ArtifactChild<S>>) {
    match c {
        Some(c) => {
            out.push(1);
            write_child(out, c);
        }
        None => out.push(0),
    }
}
}
pub use native_snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{gis_map_drawing_child_handle, gis_map_value_child_handle, GisMapDrawingChild, GisMapImageChild, GisMapValueChild, MapFeature};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};

pub(crate) fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}

pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}

pub(crate) fn read_ref(reader: &mut store::ByteReader<'_>) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&read_str_lp(reader)?)
}

pub(crate) fn read_child<S>(reader: &mut store::ByteReader<'_>) -> Result<store::ArtifactChild<S>, String> {
    let child_id = read_str_lp(reader)?;
    let target = read_ref(reader)?;
    Ok(store::ArtifactChild::new(child_id, target))
}

pub(crate) fn read_child_opt<S>(reader: &mut store::ByteReader<'_>) -> Result<Option<store::ArtifactChild<S>>, String> {
    let presence = reader.read_u8().map_err(|e| e.to_string())?;
    if presence == 0 {
        Ok(None)
    } else {
        Ok(Some(read_child(reader)?))
    }
}
}
pub use snapshot_codec::*;
