//! ⚖️ VDI 3805 app — binary command protocol surface + laws (constitutional: protocol).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::mutations::Vdi3805Mutation;
use protocol::OpBinary;

/// 📦️ Encodes a document mutation to its binary op form.
pub fn encode_op(mutation: &Vdi3805Mutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    mutation.encode_op()
}

/// 📖️ Decodes a document mutation from its binary op form.
pub fn decode_op(bytes: &[u8]) -> Result<Vdi3805Mutation, protocol::ProtocolError> {
    Vdi3805Mutation::decode_op(bytes)
}

mod native_codec {
use super::*;
use crate::artifact_schema::mutations::Vdi3805Mutation;
use crate::artifact_schema::mutations::{
    add_geometry_connection::AddGeometryConnection, change_correction_as_of::ChangeCorrectionAsOf, change_edition_profile::ChangeEditionProfile, change_strict_mode::ChangeStrictMode, add_curve::AddCurve, add_geometry::AddGeometry,
    add_product::AddProduct, remove_curve::RemoveCurve, remove_geometry::RemoveGeometry, remove_product::RemoveProduct, remove_edition_profile::RemoveEditionProfile, remove_geometry_connection::RemoveGeometryConnection,
    rename_product::RenameProduct, change_curve_points::ChangeCurvePoints, change_geometry_parameters::ChangeGeometryParameters, change_product_configuration::ChangeProductConfiguration, resize_geometry::ResizeGeometry,
    change_manufacturer_file::ChangeManufacturerFile,
    change_limits::ChangeLimits,
};

fn read_str_bin(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    let bytes = reader.read_bytes(len).map_err(|e| e.to_string())?;
    String::from_utf8(bytes.to_vec()).map_err(|e| e.to_string())
}

fn write_json_bin<T: semio_framework_value::ToValue>(out: &mut Vec<u8>, value: &T) {
    write_str_bin(out, &semio_framework_pack_json::to_json_string(value));
}

fn read_json_bin<T: semio_framework_value::FromValue>(reader: &mut store::ByteReader<'_>) -> Result<T, String> {
    semio_framework_pack_json::from_json_str(&read_str_bin(reader)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| e.to_string())
}

fn write_bool_bin(out: &mut Vec<u8>, v: bool) {
    out.push(if v { 1 } else { 0 });
}

fn read_bool_bin(reader: &mut store::ByteReader<'_>) -> Result<bool, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(format!("bad bool tag {other}")),
    }
}

fn write_opt_usize_bin(out: &mut Vec<u8>, v: &Option<usize>) {
    match v {
        Some(index) => {
            out.push(1);
            store::pack_rt::write_varint_u64(out, *index as u64);
        }
        None => out.push(0),
    }
}

fn read_opt_usize_bin(reader: &mut store::ByteReader<'_>) -> Result<Option<usize>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(reader.read_varint_u64().map_err(|e| e.to_string())? as usize)),
        other => Err(format!("bad option tag {other}")),
    }
}

const TAG_UPDATE_MANUFACTURER_FILE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-manufacturer-file");

const TAG_CHANGE_LIMITS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-limits");

const TAG_CHANGE_CORRECTION_AS_OF: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-correction-as-of");

const TAG_CHANGE_STRICT_MODE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-strict-mode");

const TAG_CHANGE_EDITION_PROFILE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-edition-profile");

const TAG_REMOVE_EDITION_PROFILE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-edition-profile");

const TAG_CREATE_PRODUCT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "add-product");

const TAG_DELETE_PRODUCT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-product");

const TAG_RENAME_PRODUCT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "rename-product");

const TAG_REPLACE_PRODUCT_CONFIGURATION: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-product-configuration");

const TAG_CREATE_GEOMETRY: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "add-geometry");

const TAG_DELETE_GEOMETRY: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-geometry");

const TAG_RESIZE_GEOMETRY: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "resize-geometry");

const TAG_ADD_GEOMETRY_CONNECTION: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "add-geometry-connection");

const TAG_REMOVE_GEOMETRY_CONNECTION: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-geometry-connection");

const TAG_REPLACE_GEOMETRY_PARAMETERS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-geometry-parameters");

const TAG_CREATE_CURVE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "add-curve");

const TAG_DELETE_CURVE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-curve");

const TAG_REPLACE_CURVE_POINTS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-curve-points");

const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
impl protocol::OpBinary for Vdi3805Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            Vdi3805Mutation::ChangeManufacturerFile(_) => TAG_UPDATE_MANUFACTURER_FILE,
            Vdi3805Mutation::ChangeLimits(_) => TAG_CHANGE_LIMITS,
            Vdi3805Mutation::ChangeCorrectionAsOf(_) => TAG_CHANGE_CORRECTION_AS_OF,
            Vdi3805Mutation::ChangeStrictMode(_) => TAG_CHANGE_STRICT_MODE,
            Vdi3805Mutation::ChangeEditionProfile(_) => TAG_CHANGE_EDITION_PROFILE,
            Vdi3805Mutation::RemoveEditionProfile(_) => TAG_REMOVE_EDITION_PROFILE,
            Vdi3805Mutation::AddProduct(_) => TAG_CREATE_PRODUCT,
            Vdi3805Mutation::RemoveProduct(_) => TAG_DELETE_PRODUCT,
            Vdi3805Mutation::RenameProduct(_) => TAG_RENAME_PRODUCT,
            Vdi3805Mutation::ChangeProductConfiguration(_) => TAG_REPLACE_PRODUCT_CONFIGURATION,
            Vdi3805Mutation::AddGeometry(_) => TAG_CREATE_GEOMETRY,
            Vdi3805Mutation::RemoveGeometry(_) => TAG_DELETE_GEOMETRY,
            Vdi3805Mutation::ResizeGeometry(_) => TAG_RESIZE_GEOMETRY,
            Vdi3805Mutation::AddGeometryConnection(_) => TAG_ADD_GEOMETRY_CONNECTION,
            Vdi3805Mutation::RemoveGeometryConnection(_) => TAG_REMOVE_GEOMETRY_CONNECTION,
            Vdi3805Mutation::ChangeGeometryParameters(_) => TAG_REPLACE_GEOMETRY_PARAMETERS,
            Vdi3805Mutation::AddCurve(_) => TAG_CREATE_CURVE,
            Vdi3805Mutation::RemoveCurve(_) => TAG_DELETE_CURVE,
            Vdi3805Mutation::ChangeCurvePoints(_) => TAG_REPLACE_CURVE_POINTS,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            Vdi3805Mutation::ChangeManufacturerFile(p) => write_json_bin(&mut out, &p.new_manufacturer_file),
            Vdi3805Mutation::ChangeLimits(p) => write_json_bin(&mut out, &p.new_limits),
            Vdi3805Mutation::ChangeCorrectionAsOf(p) => write_json_bin(&mut out, &p.new_correction_as_of),
            Vdi3805Mutation::ChangeStrictMode(p) => write_bool_bin(&mut out, p.new_strict_mode),
            Vdi3805Mutation::ChangeEditionProfile(p) => {
                write_str_bin(&mut out, &p.sheet);
                write_json_bin(&mut out, &p.new_choice);
            }
            Vdi3805Mutation::RemoveEditionProfile(p) => write_str_bin(&mut out, &p.sheet),
            Vdi3805Mutation::AddProduct(p) => {
                write_json_bin(&mut out, &p.product);
                write_opt_usize_bin(&mut out, &p.index);
            }
            Vdi3805Mutation::RemoveProduct(p) => write_str_bin(&mut out, &p.id),
            Vdi3805Mutation::RenameProduct(p) => {
                write_str_bin(&mut out, &p.id);
                write_json_bin(&mut out, &p.new_title);
            }
            Vdi3805Mutation::ChangeProductConfiguration(p) => {
                write_str_bin(&mut out, &p.id);
                write_json_bin(&mut out, &p.new_configuration);
            }
            Vdi3805Mutation::AddGeometry(p) => write_json_bin(&mut out, &p.geometry),
            Vdi3805Mutation::RemoveGeometry(p) => write_str_bin(&mut out, &p.id),
            Vdi3805Mutation::ResizeGeometry(p) => {
                write_str_bin(&mut out, &p.id);
                write_json_bin(&mut out, &p.new_bbox);
            }
            Vdi3805Mutation::AddGeometryConnection(p) => {
                write_str_bin(&mut out, &p.id);
                write_json_bin(&mut out, &p.connection);
            }
            Vdi3805Mutation::RemoveGeometryConnection(p) => {
                write_str_bin(&mut out, &p.id);
                write_str_bin(&mut out, &p.connection_id);
            }
            Vdi3805Mutation::ChangeGeometryParameters(p) => {
                write_str_bin(&mut out, &p.id);
                write_json_bin(&mut out, &p.new_parameters);
            }
            Vdi3805Mutation::AddCurve(p) => write_json_bin(&mut out, &p.curve),
            Vdi3805Mutation::RemoveCurve(p) => write_str_bin(&mut out, &p.id),
            Vdi3805Mutation::ChangeCurvePoints(p) => {
                write_str_bin(&mut out, &p.id);
                write_json_bin(&mut out, &p.new_points);
            }
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            TAG_UPDATE_MANUFACTURER_FILE => Ok(Vdi3805Mutation::ChangeManufacturerFile(ChangeManufacturerFile { new_manufacturer_file: read_json_bin(&mut reader).map_err(|e| malformed("new_manufacturer_file", reader.position(), e))? })),
            TAG_CHANGE_LIMITS => Ok(Vdi3805Mutation::ChangeLimits(ChangeLimits { new_limits: read_json_bin(&mut reader).map_err(|e| malformed("new_limits", reader.position(), e))? })),
            TAG_CHANGE_CORRECTION_AS_OF => Ok(Vdi3805Mutation::ChangeCorrectionAsOf(ChangeCorrectionAsOf { new_correction_as_of: read_json_bin(&mut reader).map_err(|e| malformed("new_correction_as_of", reader.position(), e))? })),
            TAG_CHANGE_STRICT_MODE => Ok(Vdi3805Mutation::ChangeStrictMode(ChangeStrictMode { new_strict_mode: read_bool_bin(&mut reader).map_err(|e| malformed("new_strict_mode", reader.position(), e))? })),
            TAG_CHANGE_EDITION_PROFILE => {
                let sheet = read_str_bin(&mut reader).map_err(|e| malformed("sheet", reader.position(), e))?;
                let new_choice = read_json_bin(&mut reader).map_err(|e| malformed("new_choice", reader.position(), e))?;
                Ok(Vdi3805Mutation::ChangeEditionProfile(ChangeEditionProfile { sheet, new_choice }))
            }
            TAG_REMOVE_EDITION_PROFILE => Ok(Vdi3805Mutation::RemoveEditionProfile(RemoveEditionProfile { sheet: read_str_bin(&mut reader).map_err(|e| malformed("sheet", reader.position(), e))? })),
            TAG_CREATE_PRODUCT => {
                let product = read_json_bin(&mut reader).map_err(|e| malformed("product", reader.position(), e))?;
                let index = read_opt_usize_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                Ok(Vdi3805Mutation::AddProduct(AddProduct { product, index }))
            }
            TAG_DELETE_PRODUCT => Ok(Vdi3805Mutation::RemoveProduct(RemoveProduct { id: read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))? })),
            TAG_RENAME_PRODUCT => {
                let id = read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))?;
                let new_title = read_json_bin(&mut reader).map_err(|e| malformed("new_title", reader.position(), e))?;
                Ok(Vdi3805Mutation::RenameProduct(RenameProduct { id, new_title }))
            }
            TAG_REPLACE_PRODUCT_CONFIGURATION => {
                let id = read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))?;
                let new_configuration = read_json_bin(&mut reader).map_err(|e| malformed("new_configuration", reader.position(), e))?;
                Ok(Vdi3805Mutation::ChangeProductConfiguration(ChangeProductConfiguration { id, new_configuration }))
            }
            TAG_CREATE_GEOMETRY => Ok(Vdi3805Mutation::AddGeometry(AddGeometry { geometry: read_json_bin(&mut reader).map_err(|e| malformed("geometry", reader.position(), e))? })),
            TAG_DELETE_GEOMETRY => Ok(Vdi3805Mutation::RemoveGeometry(RemoveGeometry { id: read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))? })),
            TAG_RESIZE_GEOMETRY => {
                let id = read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))?;
                let new_bbox = read_json_bin(&mut reader).map_err(|e| malformed("new_bbox", reader.position(), e))?;
                Ok(Vdi3805Mutation::ResizeGeometry(ResizeGeometry { id, new_bbox }))
            }
            TAG_ADD_GEOMETRY_CONNECTION => {
                let id = read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))?;
                let connection = read_json_bin(&mut reader).map_err(|e| malformed("connection", reader.position(), e))?;
                Ok(Vdi3805Mutation::AddGeometryConnection(AddGeometryConnection { id, connection }))
            }
            TAG_REMOVE_GEOMETRY_CONNECTION => {
                let id = read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))?;
                let connection_id = read_str_bin(&mut reader).map_err(|e| malformed("connection_id", reader.position(), e))?;
                Ok(Vdi3805Mutation::RemoveGeometryConnection(RemoveGeometryConnection { id, connection_id }))
            }
            TAG_REPLACE_GEOMETRY_PARAMETERS => {
                let id = read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))?;
                let new_parameters = read_json_bin(&mut reader).map_err(|e| malformed("new_parameters", reader.position(), e))?;
                Ok(Vdi3805Mutation::ChangeGeometryParameters(ChangeGeometryParameters { id, new_parameters }))
            }
            TAG_CREATE_CURVE => Ok(Vdi3805Mutation::AddCurve(AddCurve { curve: read_json_bin(&mut reader).map_err(|e| malformed("curve", reader.position(), e))? })),
            TAG_DELETE_CURVE => Ok(Vdi3805Mutation::RemoveCurve(RemoveCurve { id: read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))? })),
            TAG_REPLACE_CURVE_POINTS => {
                let id = read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))?;
                let new_points = read_json_bin(&mut reader).map_err(|e| malformed("new_points", reader.position(), e))?;
                Ok(Vdi3805Mutation::ChangeCurvePoints(ChangeCurvePoints { id, new_points }))
            }
            other => Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        }
    }
}

pub(crate) fn write_str_bin(out: &mut Vec<u8>, s: &str) {
    store::pack_rt::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}
}
