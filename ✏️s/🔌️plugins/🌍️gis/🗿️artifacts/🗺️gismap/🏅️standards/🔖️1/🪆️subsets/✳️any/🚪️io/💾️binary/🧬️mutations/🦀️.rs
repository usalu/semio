//! ⚖️ GIS map artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`; no `📡️protocol` path segment may survive under plugins).
//!
//! 🧷️ `GisMapMutation` derives `dsl::DslEnum` directly (no foreign `CollectionMutation` in its
//! shape — every variant wraps a local `dsl::DslRecord` payload declared in its own triad leaf), so
//! this component is a pure pass-through over the derived codec, matching `🏔️gisterrain`'s sibling
//! facet's identical shape.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::mutations::GisMapMutation;
use protocol::OpBinary;

//#region 🔖️Codec
/// 📦️ Encodes a `GisMapMutation` to its binary command form.
pub fn encode_op(operation: &GisMapMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `GisMapMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<GisMapMutation, protocol::ProtocolError> {
    GisMapMutation::decode_op(bytes)
}
//#endregion 🔖️Codec

//#region 🔖️OwnedSprCatalog

//#endregion 🔖️OwnedSprCatalog

//#region 🔖️RetainedStoreInitialization

//#endregion 🔖️RetainedStoreInitialization

//#region 🧪️Tests
//#endregion 🧪️Tests

mod native_codec {
#[cfg(test)] use super::*;
use crate::schema::mutations::{GisMapMutation};
#[cfg(test)] use crate::schema::mutations::{inverse_gis_map_mutation};

impl protocol::OpBinary for GisMapMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(protocol::os_pack::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(self)))
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let malformed = |detail: String| protocol::ProtocolError::Malformed { what: "gis map operation", offset: 0, detail };
        let value = protocol::os_pack::pack_rt::decode_wire_value(bytes).map_err(|error| malformed(error.to_string()))?;
        semio_framework_value::FromValue::from_value(value).map_err(|error: semio_framework_value::ValueError| malformed(error.to_string()))
    }
}

dsl::original_pack_value_operation_receiving!(GisMapMutation);
}
