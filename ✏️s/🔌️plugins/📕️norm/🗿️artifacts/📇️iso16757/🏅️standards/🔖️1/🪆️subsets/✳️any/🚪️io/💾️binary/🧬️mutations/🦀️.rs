//! ⚖️ ISO 16757 app — binary command protocol surface + laws (constitutional: protocol).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::mutations::Iso16757Mutation;
use protocol::OpBinary;

/// 📦️ Encodes a document mutation to its binary op form.
pub fn encode_op(mutation: &Iso16757Mutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    mutation.encode_op()
}

/// 📖️ Decodes a document mutation from its binary op form.
pub fn decode_op(bytes: &[u8]) -> Result<Iso16757Mutation, protocol::ProtocolError> {
    Iso16757Mutation::decode_op(bytes)
}

mod native_codec {
use super::*;
use crate::artifact_schema::mutations::Iso16757Mutation;
use crate::artifact_schema::mutations::{
    add_selection_constraint::mutation::AddSelectionConstraint, change_exchange_process::mutation::ChangeExchangeProcess, change_part_number_input::mutation::ChangePartNumberInput, change_selection_class::mutation::ChangeSelectionClass,
    change_selection_series::mutation::ChangeSelectionSeries, introduce_geometry_object::mutation::IntroduceGeometryObject, introduce_product::mutation::IntroduceProduct, introduce_product_class::mutation::IntroduceProductClass,
    introduce_product_group::mutation::IntroduceProductGroup, introduce_product_index::mutation::IntroduceProductIndex, introduce_product_series::mutation::IntroduceProductSeries, introduce_property_definition::mutation::IntroducePropertyDefinition,
    introduce_subject::mutation::IntroduceSubject, retire_geometry_object::mutation::RetireGeometryObject, retire_product::mutation::RetireProduct, retire_product_class::mutation::RetireProductClass,
    retire_product_group::mutation::RetireProductGroup, retire_product_index::mutation::RetireProductIndex, retire_product_series::mutation::RetireProductSeries, retire_property_definition::mutation::RetirePropertyDefinition,
    retire_subject::mutation::RetireSubject, remove_part_number_input::mutation::RemovePartNumberInput, remove_selection_constraint::mutation::RemoveSelectionConstraint, rename_catalogue::mutation::RenameCatalogue,
    rename_manufacturer::mutation::RenameManufacturer, rename_product::mutation::RenameProduct, rename_product_group::mutation::RenameProductGroup, replace_part_number_rule::mutation::ReplacePartNumberRule,
    change_script_limits::mutation::ChangeScriptLimits,
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

fn write_opt_str_bin(out: &mut Vec<u8>, s: &Option<String>) {
    match s {
        Some(v) => {
            out.push(1);
            write_str_bin(out, v);
        }
        None => out.push(0),
    }
}

fn read_opt_str_bin(reader: &mut store::ByteReader<'_>) -> Result<Option<String>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(read_str_bin(reader)?)),
        other => Err(format!("bad option tag {other}")),
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

const TAG_CHANGE_EXCHANGE_PROCESS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-exchange-process");

const TAG_UPDATE_SCRIPT_LIMITS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-script-limits");

const TAG_REPLACE_PART_NUMBER_RULE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "replace-part-number-rule");

const TAG_CHANGE_PART_NUMBER_INPUT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-part-number-input");

const TAG_REMOVE_PART_NUMBER_INPUT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-part-number-input");

const TAG_CHANGE_SELECTION_CLASS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-selection-class");

const TAG_CHANGE_SELECTION_SERIES: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-selection-series");

const TAG_ADD_SELECTION_CONSTRAINT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "add-selection-constraint");

const TAG_REMOVE_SELECTION_CONSTRAINT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-selection-constraint");

const TAG_RENAME_CATALOGUE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "rename-catalogue");

const TAG_RENAME_MANUFACTURER: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "rename-manufacturer");

const TAG_CREATE_PRODUCT_GROUP: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "introduce-product-group");

const TAG_DELETE_PRODUCT_GROUP: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "retire-product-group");

const TAG_RENAME_PRODUCT_GROUP: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "rename-product-group");

const TAG_CREATE_PRODUCT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "introduce-product");

const TAG_DELETE_PRODUCT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "retire-product");

const TAG_RENAME_PRODUCT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "rename-product");

const TAG_CREATE_PROPERTY_DEFINITION: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "introduce-property-definition");

const TAG_DELETE_PROPERTY_DEFINITION: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "retire-property-definition");

const TAG_CREATE_SUBJECT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "introduce-subject");

const TAG_DELETE_SUBJECT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "retire-subject");

const TAG_CREATE_PRODUCT_CLASS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "introduce-product-class");

const TAG_DELETE_PRODUCT_CLASS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "retire-product-class");

const TAG_CREATE_PRODUCT_SERIES: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "introduce-product-series");

const TAG_DELETE_PRODUCT_SERIES: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "retire-product-series");

const TAG_CREATE_PRODUCT_INDEX: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "introduce-product-index");

const TAG_DELETE_PRODUCT_INDEX: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "retire-product-index");

const TAG_CREATE_GEOMETRY_OBJECT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "introduce-geometry-object");

const TAG_DELETE_GEOMETRY_OBJECT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "retire-geometry-object");

const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
impl protocol::OpBinary for Iso16757Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            Iso16757Mutation::ChangeExchangeProcess(_) => TAG_CHANGE_EXCHANGE_PROCESS,
            Iso16757Mutation::ChangeScriptLimits(_) => TAG_UPDATE_SCRIPT_LIMITS,
            Iso16757Mutation::ReplacePartNumberRule(_) => TAG_REPLACE_PART_NUMBER_RULE,
            Iso16757Mutation::ChangePartNumberInput(_) => TAG_CHANGE_PART_NUMBER_INPUT,
            Iso16757Mutation::RemovePartNumberInput(_) => TAG_REMOVE_PART_NUMBER_INPUT,
            Iso16757Mutation::ChangeSelectionClass(_) => TAG_CHANGE_SELECTION_CLASS,
            Iso16757Mutation::ChangeSelectionSeries(_) => TAG_CHANGE_SELECTION_SERIES,
            Iso16757Mutation::AddSelectionConstraint(_) => TAG_ADD_SELECTION_CONSTRAINT,
            Iso16757Mutation::RemoveSelectionConstraint(_) => TAG_REMOVE_SELECTION_CONSTRAINT,
            Iso16757Mutation::RenameCatalogue(_) => TAG_RENAME_CATALOGUE,
            Iso16757Mutation::RenameManufacturer(_) => TAG_RENAME_MANUFACTURER,
            Iso16757Mutation::IntroduceProductGroup(_) => TAG_CREATE_PRODUCT_GROUP,
            Iso16757Mutation::RetireProductGroup(_) => TAG_DELETE_PRODUCT_GROUP,
            Iso16757Mutation::RenameProductGroup(_) => TAG_RENAME_PRODUCT_GROUP,
            Iso16757Mutation::IntroduceProduct(_) => TAG_CREATE_PRODUCT,
            Iso16757Mutation::RetireProduct(_) => TAG_DELETE_PRODUCT,
            Iso16757Mutation::RenameProduct(_) => TAG_RENAME_PRODUCT,
            Iso16757Mutation::IntroducePropertyDefinition(_) => TAG_CREATE_PROPERTY_DEFINITION,
            Iso16757Mutation::RetirePropertyDefinition(_) => TAG_DELETE_PROPERTY_DEFINITION,
            Iso16757Mutation::IntroduceSubject(_) => TAG_CREATE_SUBJECT,
            Iso16757Mutation::RetireSubject(_) => TAG_DELETE_SUBJECT,
            Iso16757Mutation::IntroduceProductClass(_) => TAG_CREATE_PRODUCT_CLASS,
            Iso16757Mutation::RetireProductClass(_) => TAG_DELETE_PRODUCT_CLASS,
            Iso16757Mutation::IntroduceProductSeries(_) => TAG_CREATE_PRODUCT_SERIES,
            Iso16757Mutation::RetireProductSeries(_) => TAG_DELETE_PRODUCT_SERIES,
            Iso16757Mutation::IntroduceProductIndex(_) => TAG_CREATE_PRODUCT_INDEX,
            Iso16757Mutation::RetireProductIndex(_) => TAG_DELETE_PRODUCT_INDEX,
            Iso16757Mutation::IntroduceGeometryObject(_) => TAG_CREATE_GEOMETRY_OBJECT,
            Iso16757Mutation::RetireGeometryObject(_) => TAG_DELETE_GEOMETRY_OBJECT,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            Iso16757Mutation::ChangeExchangeProcess(p) => write_json_bin(&mut out, &p.new_exchange_process),
            Iso16757Mutation::ChangeScriptLimits(p) => {
                store::pack_rt::write_varint_u64(&mut out, p.new_max_steps as u64);
                store::pack_rt::write_varint_u64(&mut out, p.new_max_recursion as u64);
                store::pack_rt::write_varint_u64(&mut out, p.new_timeout_ms);
            }
            Iso16757Mutation::ReplacePartNumberRule(p) => write_json_bin(&mut out, &p.new_rule),
            Iso16757Mutation::ChangePartNumberInput(p) => {
                write_str_bin(&mut out, &p.key);
                write_json_bin(&mut out, &p.new_value);
            }
            Iso16757Mutation::RemovePartNumberInput(p) => write_str_bin(&mut out, &p.key),
            Iso16757Mutation::ChangeSelectionClass(p) => write_str_bin(&mut out, &p.new_class_id),
            Iso16757Mutation::ChangeSelectionSeries(p) => write_opt_str_bin(&mut out, &p.new_series_id),
            Iso16757Mutation::AddSelectionConstraint(p) => {
                write_json_bin(&mut out, &p.constraint);
                write_opt_usize_bin(&mut out, &p.index);
            }
            Iso16757Mutation::RemoveSelectionConstraint(p) => store::pack_rt::write_varint_u64(&mut out, p.index as u64),
            Iso16757Mutation::RenameCatalogue(p) => write_str_bin(&mut out, &p.new_name),
            Iso16757Mutation::RenameManufacturer(p) => write_str_bin(&mut out, &p.new_name),
            Iso16757Mutation::IntroduceProductGroup(p) => {
                write_json_bin(&mut out, &p.product_group);
                write_opt_usize_bin(&mut out, &p.index);
            }
            Iso16757Mutation::RetireProductGroup(p) => write_str_bin(&mut out, &p.id),
            Iso16757Mutation::RenameProductGroup(p) => {
                write_str_bin(&mut out, &p.id);
                write_str_bin(&mut out, &p.new_name);
            }
            Iso16757Mutation::IntroduceProduct(p) => {
                write_json_bin(&mut out, &p.product);
                write_opt_usize_bin(&mut out, &p.index);
            }
            Iso16757Mutation::RetireProduct(p) => write_str_bin(&mut out, &p.id),
            Iso16757Mutation::RenameProduct(p) => {
                write_str_bin(&mut out, &p.id);
                write_str_bin(&mut out, &p.new_name);
            }
            Iso16757Mutation::IntroducePropertyDefinition(p) => {
                write_json_bin(&mut out, &p.property_definition);
                write_opt_usize_bin(&mut out, &p.index);
            }
            Iso16757Mutation::RetirePropertyDefinition(p) => write_str_bin(&mut out, &p.id),
            Iso16757Mutation::IntroduceSubject(p) => {
                write_json_bin(&mut out, &p.subject);
                write_opt_usize_bin(&mut out, &p.index);
            }
            Iso16757Mutation::RetireSubject(p) => write_str_bin(&mut out, &p.id),
            Iso16757Mutation::IntroduceProductClass(p) => {
                write_json_bin(&mut out, &p.product_class);
                write_opt_usize_bin(&mut out, &p.index);
            }
            Iso16757Mutation::RetireProductClass(p) => write_str_bin(&mut out, &p.id),
            Iso16757Mutation::IntroduceProductSeries(p) => {
                write_json_bin(&mut out, &p.product_series);
                write_opt_usize_bin(&mut out, &p.index);
            }
            Iso16757Mutation::RetireProductSeries(p) => write_str_bin(&mut out, &p.id),
            Iso16757Mutation::IntroduceProductIndex(p) => {
                write_json_bin(&mut out, &p.product_index);
                write_opt_usize_bin(&mut out, &p.index);
            }
            Iso16757Mutation::RetireProductIndex(p) => write_str_bin(&mut out, &p.id),
            Iso16757Mutation::IntroduceGeometryObject(p) => write_json_bin(&mut out, &p.geometry_object),
            Iso16757Mutation::RetireGeometryObject(p) => write_str_bin(&mut out, &p.id),
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            TAG_CHANGE_EXCHANGE_PROCESS => Ok(Iso16757Mutation::ChangeExchangeProcess(ChangeExchangeProcess { new_exchange_process: read_json_bin(&mut reader).map_err(|e| malformed("new_exchange_process", reader.position(), e))? })),
            TAG_UPDATE_SCRIPT_LIMITS => {
                let new_max_steps = reader.read_varint_u64().map_err(|e| malformed("new_max_steps", reader.position(), e.to_string()))? as u32;
                let new_max_recursion = reader.read_varint_u64().map_err(|e| malformed("new_max_recursion", reader.position(), e.to_string()))? as u32;
                let new_timeout_ms = reader.read_varint_u64().map_err(|e| malformed("new_timeout_ms", reader.position(), e.to_string()))?;
                Ok(Iso16757Mutation::ChangeScriptLimits(ChangeScriptLimits { new_max_steps, new_max_recursion, new_timeout_ms }))
            }
            TAG_REPLACE_PART_NUMBER_RULE => Ok(Iso16757Mutation::ReplacePartNumberRule(ReplacePartNumberRule { new_rule: read_json_bin(&mut reader).map_err(|e| malformed("new_rule", reader.position(), e))? })),
            TAG_CHANGE_PART_NUMBER_INPUT => {
                let key = read_str_bin(&mut reader).map_err(|e| malformed("key", reader.position(), e))?;
                let new_value = read_json_bin(&mut reader).map_err(|e| malformed("new_value", reader.position(), e))?;
                Ok(Iso16757Mutation::ChangePartNumberInput(ChangePartNumberInput { key, new_value }))
            }
            TAG_REMOVE_PART_NUMBER_INPUT => Ok(Iso16757Mutation::RemovePartNumberInput(RemovePartNumberInput { key: read_str_bin(&mut reader).map_err(|e| malformed("key", reader.position(), e))? })),
            TAG_CHANGE_SELECTION_CLASS => Ok(Iso16757Mutation::ChangeSelectionClass(ChangeSelectionClass { new_class_id: read_str_bin(&mut reader).map_err(|e| malformed("new_class_id", reader.position(), e))? })),
            TAG_CHANGE_SELECTION_SERIES => Ok(Iso16757Mutation::ChangeSelectionSeries(ChangeSelectionSeries { new_series_id: read_opt_str_bin(&mut reader).map_err(|e| malformed("new_series_id", reader.position(), e))? })),
            TAG_ADD_SELECTION_CONSTRAINT => {
                let constraint = read_json_bin(&mut reader).map_err(|e| malformed("constraint", reader.position(), e))?;
                let index = read_opt_usize_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                Ok(Iso16757Mutation::AddSelectionConstraint(AddSelectionConstraint { constraint, index }))
            }
            TAG_REMOVE_SELECTION_CONSTRAINT => {
                let index = reader.read_varint_u64().map_err(|e| malformed("index", reader.position(), e.to_string()))? as usize;
                Ok(Iso16757Mutation::RemoveSelectionConstraint(RemoveSelectionConstraint { index }))
            }
            TAG_RENAME_CATALOGUE => Ok(Iso16757Mutation::RenameCatalogue(RenameCatalogue { new_name: read_str_bin(&mut reader).map_err(|e| malformed("new_name", reader.position(), e))? })),
            TAG_RENAME_MANUFACTURER => Ok(Iso16757Mutation::RenameManufacturer(RenameManufacturer { new_name: read_str_bin(&mut reader).map_err(|e| malformed("new_name", reader.position(), e))? })),
            TAG_CREATE_PRODUCT_GROUP => {
                let product_group = read_json_bin(&mut reader).map_err(|e| malformed("product_group", reader.position(), e))?;
                let index = read_opt_usize_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                Ok(Iso16757Mutation::IntroduceProductGroup(IntroduceProductGroup { product_group, index }))
            }
            TAG_DELETE_PRODUCT_GROUP => Ok(Iso16757Mutation::RetireProductGroup(RetireProductGroup { id: read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))? })),
            TAG_RENAME_PRODUCT_GROUP => {
                let id = read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))?;
                let new_name = read_str_bin(&mut reader).map_err(|e| malformed("new_name", reader.position(), e))?;
                Ok(Iso16757Mutation::RenameProductGroup(RenameProductGroup { id, new_name }))
            }
            TAG_CREATE_PRODUCT => {
                let product = read_json_bin(&mut reader).map_err(|e| malformed("product", reader.position(), e))?;
                let index = read_opt_usize_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                Ok(Iso16757Mutation::IntroduceProduct(IntroduceProduct { product, index }))
            }
            TAG_DELETE_PRODUCT => Ok(Iso16757Mutation::RetireProduct(RetireProduct { id: read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))? })),
            TAG_RENAME_PRODUCT => {
                let id = read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))?;
                let new_name = read_str_bin(&mut reader).map_err(|e| malformed("new_name", reader.position(), e))?;
                Ok(Iso16757Mutation::RenameProduct(RenameProduct { id, new_name }))
            }
            TAG_CREATE_PROPERTY_DEFINITION => {
                let property_definition = read_json_bin(&mut reader).map_err(|e| malformed("property_definition", reader.position(), e))?;
                let index = read_opt_usize_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                Ok(Iso16757Mutation::IntroducePropertyDefinition(IntroducePropertyDefinition { property_definition, index }))
            }
            TAG_DELETE_PROPERTY_DEFINITION => Ok(Iso16757Mutation::RetirePropertyDefinition(RetirePropertyDefinition { id: read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))? })),
            TAG_CREATE_SUBJECT => {
                let subject = read_json_bin(&mut reader).map_err(|e| malformed("subject", reader.position(), e))?;
                let index = read_opt_usize_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                Ok(Iso16757Mutation::IntroduceSubject(IntroduceSubject { subject, index }))
            }
            TAG_DELETE_SUBJECT => Ok(Iso16757Mutation::RetireSubject(RetireSubject { id: read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))? })),
            TAG_CREATE_PRODUCT_CLASS => {
                let product_class = read_json_bin(&mut reader).map_err(|e| malformed("product_class", reader.position(), e))?;
                let index = read_opt_usize_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                Ok(Iso16757Mutation::IntroduceProductClass(IntroduceProductClass { product_class, index }))
            }
            TAG_DELETE_PRODUCT_CLASS => Ok(Iso16757Mutation::RetireProductClass(RetireProductClass { id: read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))? })),
            TAG_CREATE_PRODUCT_SERIES => {
                let product_series = read_json_bin(&mut reader).map_err(|e| malformed("product_series", reader.position(), e))?;
                let index = read_opt_usize_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                Ok(Iso16757Mutation::IntroduceProductSeries(IntroduceProductSeries { product_series, index }))
            }
            TAG_DELETE_PRODUCT_SERIES => Ok(Iso16757Mutation::RetireProductSeries(RetireProductSeries { id: read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))? })),
            TAG_CREATE_PRODUCT_INDEX => {
                let product_index = read_json_bin(&mut reader).map_err(|e| malformed("product_index", reader.position(), e))?;
                let index = read_opt_usize_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                Ok(Iso16757Mutation::IntroduceProductIndex(IntroduceProductIndex { product_index, index }))
            }
            TAG_DELETE_PRODUCT_INDEX => Ok(Iso16757Mutation::RetireProductIndex(RetireProductIndex { id: read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))? })),
            TAG_CREATE_GEOMETRY_OBJECT => Ok(Iso16757Mutation::IntroduceGeometryObject(IntroduceGeometryObject {
                geometry_object: read_json_bin(&mut reader).map_err(|e| malformed("geometry_object", reader.position(), e))?,
            })),
            TAG_DELETE_GEOMETRY_OBJECT => Ok(Iso16757Mutation::RetireGeometryObject(RetireGeometryObject { id: read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))? })),
            other => Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        }
    }
}

pub(crate) fn write_str_bin(out: &mut Vec<u8>, s: &str) {
    store::pack_rt::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}
}
