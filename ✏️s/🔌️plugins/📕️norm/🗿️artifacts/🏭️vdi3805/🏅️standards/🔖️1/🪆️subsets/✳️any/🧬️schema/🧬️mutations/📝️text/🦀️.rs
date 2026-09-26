//! ⚡️ VDI 3805 artifact — hand-rolled `OpText`/`OpBinary` for `Vdi3805Mutation`.
//! `#[derive(dsl_derive::Mutations)]` only generates `Mutation`/`SemanticMutation` (see
//! `../🦀️.rs`'s `🔖️Mutations` region) — the wire-text/wire-binary codecs stay handcrafted
//! here, one keyword per semantic verb, grammar `keyword key1=value1 key2=value2 ...`. Structured
//! payload fields (manufacturer file header, security limits, catalogue products, geometry,
//! curves, ...) round-trip through a quoted JSON string — every one of them already derives
//! `Serialize`/`Deserialize`, so a second handcrafted grammar per structured type would just
//! duplicate that losslessly.

pub use crate::artifact_schema::mutations::Vdi3805Mutation;

use crate::artifact_schema::mutations::{
    add_geometry_connection::AddGeometryConnection, change_correction_as_of::ChangeCorrectionAsOf, change_edition_profile::ChangeEditionProfile, change_strict_mode::ChangeStrictMode, add_curve::AddCurve, add_geometry::AddGeometry,
    add_product::AddProduct, remove_curve::RemoveCurve, remove_geometry::RemoveGeometry, remove_product::RemoveProduct, remove_edition_profile::RemoveEditionProfile, remove_geometry_connection::RemoveGeometryConnection,
    rename_product::RenameProduct, change_curve_points::ChangeCurvePoints, change_geometry_parameters::ChangeGeometryParameters, change_product_configuration::ChangeProductConfiguration, resize_geometry::ResizeGeometry,
    change_manufacturer_file::ChangeManufacturerFile,
};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️ScalarCodec
/// 🔤️ Quoted-string encode/decode — the only value kind that can contain a raw space, so every
/// other scalar's text form stays space-free and tokenizable by [`tokenize_args`].
fn enc_str(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}
fn dec_str(s: &str) -> Result<String, String> {
    let inner = s.strip_prefix('"').and_then(|s| s.strip_suffix('"')).ok_or_else(|| format!("expected quoted string, got {s:?}"))?;
    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('\\') => out.push('\\'),
            Some('"') => out.push('"'),
            Some(other) => return Err(format!("bad escape \\{other}")),
            None => return Err("dangling escape".into()),
        }
    }
    Ok(out)
}
fn enc_bool(v: bool) -> String {
    v.to_string()
}
fn dec_bool(s: &str) -> Result<bool, String> {
    s.parse().map_err(|e: std::str::ParseBoolError| e.to_string())
}
fn enc_opt_usize(v: &Option<usize>) -> String {
    match v {
        Some(index) => index.to_string(),
        None => "-".to_string(),
    }
}
fn dec_opt_usize(s: &str) -> Result<Option<usize>, String> {
    if s == "-" {
        Ok(None)
    } else {
        Ok(Some(s.parse().map_err(|e: std::num::ParseIntError| e.to_string())?))
    }
}
/// 🧬️ Every structured payload field (manufacturer file header, security limits, catalogue
/// products, geometry, curves, ...) already derives `ToValue`/`FromValue` — a quoted JSON
/// string reuses that losslessly instead of a second handcrafted grammar per type.
fn enc_json<T: dsl::ToValue>(value: &T) -> String {
    enc_str(&pack::json::to_json_string(value))
}
fn dec_json<T: dsl::FromValue>(s: &str) -> Result<T, String> {
    pack::json::from_json_str(&dec_str(s)?).map_err(|e| e.to_string())
}
//#endregion 🔖️ScalarCodec

//#region 🔖️Tokenizer
/// 🔡️ Splits `key=value` tokens on plain spaces, EXCEPT spaces inside a `"..."` quoted value —
/// needed because names/JSON payloads may contain spaces.
fn tokenize_args(rest: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = rest.chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => {
                current.push(c);
                in_quotes = !in_quotes;
            }
            '\\' if in_quotes => {
                current.push(c);
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(c),
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}
fn parse_args(rest: &str) -> Result<std::collections::BTreeMap<String, String>, String> {
    tokenize_args(rest).into_iter().map(|token| token.split_once('=').map(|(k, v)| (k.to_string(), v.to_string())).ok_or_else(|| format!("bad arg token {token:?}"))).collect()
}
//#endregion 🔖️Tokenizer

//#region 🔖️OpText
fn print_vdi3805_mutation(mutation: &Vdi3805Mutation) -> String {
    match mutation {
        Vdi3805Mutation::ChangeManufacturerFile(p) => format!("change-manufacturer-file new-manufacturer-file={}", enc_json(&p.new_manufacturer_file)),
        Vdi3805Mutation::ChangeCorrectionAsOf(p) => format!("change-correction-as-of new-correction-as-of={}", enc_json(&p.new_correction_as_of)),
        Vdi3805Mutation::ChangeStrictMode(p) => format!("change-strict-mode new-strict-mode={}", enc_bool(p.new_strict_mode)),
        Vdi3805Mutation::ChangeEditionProfile(p) => format!("change-edition-profile sheet={} new-choice={}", enc_str(&p.sheet), enc_json(&p.new_choice)),
        Vdi3805Mutation::RemoveEditionProfile(p) => format!("remove-edition-profile sheet={}", enc_str(&p.sheet)),
        Vdi3805Mutation::AddProduct(p) => format!("add-product product={} index={}", enc_json(&p.product), enc_opt_usize(&p.index)),
        Vdi3805Mutation::RemoveProduct(p) => format!("remove-product id={}", enc_str(&p.id)),
        Vdi3805Mutation::RenameProduct(p) => format!("rename-product id={} new-title={}", enc_str(&p.id), enc_json(&p.new_title)),
        Vdi3805Mutation::ChangeProductConfiguration(p) => format!("change-product-configuration id={} new-configuration={}", enc_str(&p.id), enc_json(&p.new_configuration)),
        Vdi3805Mutation::AddGeometry(p) => format!("add-geometry geometry={}", enc_json(&p.geometry)),
        Vdi3805Mutation::RemoveGeometry(p) => format!("remove-geometry id={}", enc_str(&p.id)),
        Vdi3805Mutation::ResizeGeometry(p) => format!("resize-geometry id={} new-bbox={}", enc_str(&p.id), enc_json(&p.new_bbox)),
        Vdi3805Mutation::AddGeometryConnection(p) => format!("add-geometry-connection id={} connection={}", enc_str(&p.id), enc_json(&p.connection)),
        Vdi3805Mutation::RemoveGeometryConnection(p) => format!("remove-geometry-connection id={} connection-id={}", enc_str(&p.id), enc_str(&p.connection_id)),
        Vdi3805Mutation::ChangeGeometryParameters(p) => format!("change-geometry-parameters id={} new-parameters={}", enc_str(&p.id), enc_json(&p.new_parameters)),
        Vdi3805Mutation::AddCurve(p) => format!("add-curve curve={}", enc_json(&p.curve)),
        Vdi3805Mutation::RemoveCurve(p) => format!("remove-curve id={}", enc_str(&p.id)),
        Vdi3805Mutation::ChangeCurvePoints(p) => format!("change-curve-points id={} new-points={}", enc_str(&p.id), enc_json(&p.new_points)),
    }
}

fn parse_vdi3805_mutation(line: &str) -> Result<Vdi3805Mutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args = parse_args(rest)?;
    let arg = |k: &str| args.get(k).cloned().ok_or_else(|| format!("vdi3805 mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "change-manufacturer-file" => Ok(Vdi3805Mutation::ChangeManufacturerFile(ChangeManufacturerFile { new_manufacturer_file: dec_json(&arg("new-manufacturer-file")?)? })),
        "change-correction-as-of" => Ok(Vdi3805Mutation::ChangeCorrectionAsOf(ChangeCorrectionAsOf { new_correction_as_of: dec_json(&arg("new-correction-as-of")?)? })),
        "change-strict-mode" => Ok(Vdi3805Mutation::ChangeStrictMode(ChangeStrictMode { new_strict_mode: dec_bool(&arg("new-strict-mode")?)? })),
        "change-edition-profile" => Ok(Vdi3805Mutation::ChangeEditionProfile(ChangeEditionProfile { sheet: dec_str(&arg("sheet")?)?, new_choice: dec_json(&arg("new-choice")?)? })),
        "remove-edition-profile" => Ok(Vdi3805Mutation::RemoveEditionProfile(RemoveEditionProfile { sheet: dec_str(&arg("sheet")?)? })),
        "add-product" => Ok(Vdi3805Mutation::AddProduct(AddProduct { product: dec_json(&arg("product")?)?, index: dec_opt_usize(&arg("index")?)? })),
        "remove-product" => Ok(Vdi3805Mutation::RemoveProduct(RemoveProduct { id: dec_str(&arg("id")?)? })),
        "rename-product" => Ok(Vdi3805Mutation::RenameProduct(RenameProduct { id: dec_str(&arg("id")?)?, new_title: dec_json(&arg("new-title")?)? })),
        "change-product-configuration" => Ok(Vdi3805Mutation::ChangeProductConfiguration(ChangeProductConfiguration { id: dec_str(&arg("id")?)?, new_configuration: dec_json(&arg("new-configuration")?)? })),
        "add-geometry" => Ok(Vdi3805Mutation::AddGeometry(AddGeometry { geometry: dec_json(&arg("geometry")?)? })),
        "remove-geometry" => Ok(Vdi3805Mutation::RemoveGeometry(RemoveGeometry { id: dec_str(&arg("id")?)? })),
        "resize-geometry" => Ok(Vdi3805Mutation::ResizeGeometry(ResizeGeometry { id: dec_str(&arg("id")?)?, new_bbox: dec_json(&arg("new-bbox")?)? })),
        "add-geometry-connection" => Ok(Vdi3805Mutation::AddGeometryConnection(AddGeometryConnection { id: dec_str(&arg("id")?)?, connection: dec_json(&arg("connection")?)? })),
        "remove-geometry-connection" => Ok(Vdi3805Mutation::RemoveGeometryConnection(RemoveGeometryConnection { id: dec_str(&arg("id")?)?, connection_id: dec_str(&arg("connection-id")?)? })),
        "change-geometry-parameters" => Ok(Vdi3805Mutation::ChangeGeometryParameters(ChangeGeometryParameters { id: dec_str(&arg("id")?)?, new_parameters: dec_json(&arg("new-parameters")?)? })),
        "add-curve" => Ok(Vdi3805Mutation::AddCurve(AddCurve { curve: dec_json(&arg("curve")?)? })),
        "remove-curve" => Ok(Vdi3805Mutation::RemoveCurve(RemoveCurve { id: dec_str(&arg("id")?)? })),
        "change-curve-points" => Ok(Vdi3805Mutation::ChangeCurvePoints(ChangeCurvePoints { id: dec_str(&arg("id")?)?, new_points: dec_json(&arg("new-points")?)? })),
        other => Err(format!("vdi3805 mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for Vdi3805Mutation {
    fn print_op(&self) -> String {
        print_vdi3805_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_vdi3805_mutation(line).map_err(|e| store::TextError::new(e, store::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️OpText

//#region 🔖️OpBinaryCodec
/// 🎞️ Every variant's binary form is `tag u8 | json-string-per-field`; the JSON-per-field
/// consolidation used by `OpText` above applies equally here — one `write_str_bin` per field
/// regardless of that field's own structural complexity.
fn write_str_bin(out: &mut Vec<u8>, s: &str) {
    store::pack_rt::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}
fn read_str_bin(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    let bytes = reader.read_bytes(len).map_err(|e| e.to_string())?;
    String::from_utf8(bytes.to_vec()).map_err(|e| e.to_string())
}
fn write_json_bin<T: dsl::ToValue>(out: &mut Vec<u8>, value: &T) {
    write_str_bin(out, &pack::json::to_json_string(value));
}
fn read_json_bin<T: dsl::FromValue>(reader: &mut store::ByteReader<'_>) -> Result<T, String> {
    pack::json::from_json_str(&read_str_bin(reader)?).map_err(|e| e.to_string())
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

//#region 🏷️WireTags
/// 🏷️ Op tags of `Vdi3805Mutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = include_str!("../💾️binary/📡️.protocol.semio");
const TAG_UPDATE_MANUFACTURER_FILE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-manufacturer-file");
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
//#endregion 🏷️WireTags

impl protocol::OpBinary for Vdi3805Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            Vdi3805Mutation::ChangeManufacturerFile(_) => TAG_UPDATE_MANUFACTURER_FILE,
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
//#endregion 🔖️OpBinaryCodec

//#region 🔖️DemoCases
/// 🧪️ One representative value per variant — reused by the round-trip law test below.
#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<Vdi3805Mutation> {
    use crate::{GenericAttributes, 
        BoundingBox, CatalogueProduct, CharacteristicCurve, Configuration, ConnectionPoint, CurvePoint, EditionId, EditionProfileChoice, ExtensionBag, ParametricGeometry, ProductIdentity, SecurityLimits, SheetAttributes, SheetId, ValveHeatingAttributes,
        VdiQuantityKind, VdiUnit,
    };

    let product = CatalogueProduct {
        id: "VLV-NEW".into(),
        identity: ProductIdentity { manufacturer_code: "DEMO".into(), product_group: "HV".into(), article_number: "VLV-NEW".into() },
        title: crate::bilingual("Neu", "New"),
        sheet: SheetId(3),
        records: Vec::new(),
        configuration: Configuration { id: "cfg.new".into(), attributes: SheetAttributes::Generic(GenericAttributes::default()), geometry_ref: None, function_refs: Vec::new() },
        accessories: Vec::new(),
        components: Vec::new(),
        extensions: ExtensionBag::default(),
    };
    let geometry = ParametricGeometry { id: "geom.new".into(), bbox: BoundingBox::from_size(1.0, 1.0, 1.0), connections: Vec::new(), parameters: std::collections::BTreeMap::new() };
    let curve = CharacteristicCurve { id: "curve.new".into(), x_unit: VdiUnit::delta("%", VdiQuantityKind::Dimensionless, 0.01), y_unit: VdiUnit::absolute("m3/h", VdiQuantityKind::Volume, 1.0), points: vec![CurvePoint { x: 0.0, y: 0.0 }] };

    vec![
        Vdi3805Mutation::ChangeManufacturerFile(ChangeManufacturerFile { new_manufacturer_file: crate::reference_fixture().catalog.file }),
        Vdi3805Mutation::ChangeCorrectionAsOf(ChangeCorrectionAsOf { new_correction_as_of: EditionId::new(2025, 3) }),
        Vdi3805Mutation::ChangeStrictMode(ChangeStrictMode { new_strict_mode: true }),
        Vdi3805Mutation::ChangeEditionProfile(ChangeEditionProfile { sheet: "8".into(), new_choice: EditionProfileChoice::Legacy }),
        Vdi3805Mutation::RemoveEditionProfile(RemoveEditionProfile { sheet: "8".into() }),
        Vdi3805Mutation::AddProduct(AddProduct { product: product.clone(), index: Some(0) }),
        Vdi3805Mutation::RemoveProduct(RemoveProduct { id: "VLV-50-001".into() }),
        Vdi3805Mutation::RenameProduct(RenameProduct { id: "VLV-50-001".into(), new_title: crate::bilingual("Umbenannt", "Renamed") }),
        Vdi3805Mutation::ChangeProductConfiguration(ChangeProductConfiguration { id: "VLV-50-001".into(), new_configuration: product.configuration.clone() }),
        Vdi3805Mutation::AddGeometry(AddGeometry { geometry: geometry.clone() }),
        Vdi3805Mutation::RemoveGeometry(RemoveGeometry { id: "geom.valve.50".into() }),
        Vdi3805Mutation::ResizeGeometry(ResizeGeometry { id: "geom.valve.50".into(), new_bbox: BoundingBox::from_size(2.0, 2.0, 2.0) }),
        Vdi3805Mutation::AddGeometryConnection(AddGeometryConnection {
            id: "geom.valve.50".into(),
            connection: ConnectionPoint { id: "mid".into(), medium: "water".into(), position: [0.0, 0.0, 0.0], direction: [0.0, 1.0, 0.0], diameter_mm: Some(25.0) },
        }),
        Vdi3805Mutation::RemoveGeometryConnection(RemoveGeometryConnection { id: "geom.valve.50".into(), connection_id: "in".into() }),
        Vdi3805Mutation::ChangeGeometryParameters(ChangeGeometryParameters { id: "geom.valve.50".into(), new_parameters: std::collections::BTreeMap::from([("scale".to_string(), 2.0)]) }),
        Vdi3805Mutation::AddCurve(AddCurve { curve: curve.clone() }),
        Vdi3805Mutation::RemoveCurve(RemoveCurve { id: "curve-kvs".into() }),
        Vdi3805Mutation::ChangeCurvePoints(ChangeCurvePoints { id: "curve-kvs".into(), new_points: vec![CurvePoint { x: 0.0, y: 0.0 }, CurvePoint { x: 100.0, y: 9.0 }] }),
        Vdi3805Mutation::AddProduct(AddProduct {
            product: CatalogueProduct {
                configuration: Configuration {
                    id: "cfg.dn".into(),
                    attributes: SheetAttributes::ValveHeating(ValveHeatingAttributes::from_kvs_m3_h(80, 8.0, "PN16", "flange", 0.3, 0.7)),
                    geometry_ref: None,
                    function_refs: Vec::new(),
                },
                ..product
            },
            index: None,
        }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
