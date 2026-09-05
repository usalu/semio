//! ⚡️ ISO 16757 artifact — hand-rolled `OpText`/`OpBinary` for `Iso16757Mutation`.
//! `#[derive(dsl_derive::Mutations)]` only generates `Mutation`/`SemanticMutation` (see
//! `../🦀️.rs`'s `🔖️Mutations` region) — the wire-text/wire-binary codecs stay handcrafted
//! here, one keyword per semantic verb, grammar `keyword key1=value1 key2=value2 ...`. Structured
//! payload fields (entity records, catalogue values, selection constraints, the part-number rule)
//! round-trip through a quoted JSON string — every one of them already derives
//! `Serialize`/`Deserialize`, so a second handcrafted grammar per structured type would just
//! duplicate that losslessly.

pub use crate::artifacts::iso16757::schema::mutations::Iso16757Mutation;

use crate::artifacts::iso16757::schema::mutations::{
    add_selection_constraint::mutation::AddSelectionConstraint, change_exchange_process::mutation::ChangeExchangeProcess, change_part_number_input::mutation::ChangePartNumberInput, change_selection_class::mutation::ChangeSelectionClass,
    change_selection_series::mutation::ChangeSelectionSeries, create_product::mutation::CreateProduct, create_product_group::mutation::CreateProductGroup, create_property_definition::mutation::CreatePropertyDefinition,
    create_subject::mutation::CreateSubject, delete_product::mutation::DeleteProduct, delete_product_group::mutation::DeleteProductGroup, delete_property_definition::mutation::DeletePropertyDefinition, delete_subject::mutation::DeleteSubject,
    remove_part_number_input::mutation::RemovePartNumberInput, remove_selection_constraint::mutation::RemoveSelectionConstraint, rename_catalogue::mutation::RenameCatalogue, rename_manufacturer::mutation::RenameManufacturer,
    rename_product::mutation::RenameProduct, rename_product_group::mutation::RenameProductGroup, replace_part_number_rule::mutation::ReplacePartNumberRule, update_script_limits::mutation::UpdateScriptLimits,
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
fn enc_opt_str(s: &Option<String>) -> String {
    match s {
        Some(v) => enc_str(v),
        None => "-".to_string(),
    }
}
fn dec_opt_str(s: &str) -> Result<Option<String>, String> {
    if s == "-" {
        Ok(None)
    } else {
        Ok(Some(dec_str(s)?))
    }
}
fn enc_usize(v: usize) -> String {
    v.to_string()
}
fn dec_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
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
        Ok(Some(dec_usize(s)?))
    }
}
/// 🧬️ Every structured payload field (entity records, catalogue values, part-number rule,
/// selection constraints) already derives `ToValue`/`FromValue` — a quoted JSON string reuses
/// that losslessly instead of a second handcrafted grammar per type.
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
fn print_iso16757_mutation(mutation: &Iso16757Mutation) -> String {
    match mutation {
        Iso16757Mutation::ChangeExchangeProcess(p) => format!("change-exchange-process new-exchange-process={}", enc_json(&p.new_exchange_process)),
        Iso16757Mutation::UpdateScriptLimits(p) => format!("update-script-limits new-max-steps={} new-max-recursion={} new-timeout-ms={}", p.new_max_steps, p.new_max_recursion, p.new_timeout_ms),
        Iso16757Mutation::ReplacePartNumberRule(p) => format!("replace-part-number-rule new-rule={}", enc_json(&p.new_rule)),
        Iso16757Mutation::ChangePartNumberInput(p) => format!("change-part-number-input key={} new-value={}", enc_str(&p.key), enc_json(&p.new_value)),
        Iso16757Mutation::RemovePartNumberInput(p) => format!("remove-part-number-input key={}", enc_str(&p.key)),
        Iso16757Mutation::ChangeSelectionClass(p) => format!("change-selection-class new-class-id={}", enc_str(&p.new_class_id)),
        Iso16757Mutation::ChangeSelectionSeries(p) => format!("change-selection-series new-series-id={}", enc_opt_str(&p.new_series_id)),
        Iso16757Mutation::AddSelectionConstraint(p) => format!("add-selection-constraint constraint={}", enc_json(&p.constraint)),
        Iso16757Mutation::RemoveSelectionConstraint(p) => format!("remove-selection-constraint index={}", enc_usize(p.index)),
        Iso16757Mutation::RenameCatalogue(p) => format!("rename-catalogue new-name={}", enc_str(&p.new_name)),
        Iso16757Mutation::RenameManufacturer(p) => format!("rename-manufacturer new-name={}", enc_str(&p.new_name)),
        Iso16757Mutation::CreateProductGroup(p) => format!("create-product-group product-group={} index={}", enc_json(&p.product_group), enc_opt_usize(&p.index)),
        Iso16757Mutation::DeleteProductGroup(p) => format!("delete-product-group id={}", enc_str(&p.id)),
        Iso16757Mutation::RenameProductGroup(p) => format!("rename-product-group id={} new-name={}", enc_str(&p.id), enc_str(&p.new_name)),
        Iso16757Mutation::CreateProduct(p) => format!("create-product product={} index={}", enc_json(&p.product), enc_opt_usize(&p.index)),
        Iso16757Mutation::DeleteProduct(p) => format!("delete-product id={}", enc_str(&p.id)),
        Iso16757Mutation::RenameProduct(p) => format!("rename-product id={} new-name={}", enc_str(&p.id), enc_str(&p.new_name)),
        Iso16757Mutation::CreatePropertyDefinition(p) => format!("create-property-definition property-definition={} index={}", enc_json(&p.property_definition), enc_opt_usize(&p.index)),
        Iso16757Mutation::DeletePropertyDefinition(p) => format!("delete-property-definition id={}", enc_str(&p.id)),
        Iso16757Mutation::CreateSubject(p) => format!("create-subject subject={} index={}", enc_json(&p.subject), enc_opt_usize(&p.index)),
        Iso16757Mutation::DeleteSubject(p) => format!("delete-subject id={}", enc_str(&p.id)),
    }
}

fn parse_iso16757_mutation(line: &str) -> Result<Iso16757Mutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args = parse_args(rest)?;
    let arg = |k: &str| args.get(k).cloned().ok_or_else(|| format!("iso16757 mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "change-exchange-process" => Ok(Iso16757Mutation::ChangeExchangeProcess(ChangeExchangeProcess { new_exchange_process: dec_json(&arg("new-exchange-process")?)? })),
        "update-script-limits" => Ok(Iso16757Mutation::UpdateScriptLimits(UpdateScriptLimits {
            new_max_steps: arg("new-max-steps")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
            new_max_recursion: arg("new-max-recursion")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
            new_timeout_ms: arg("new-timeout-ms")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
        })),
        "replace-part-number-rule" => Ok(Iso16757Mutation::ReplacePartNumberRule(ReplacePartNumberRule { new_rule: dec_json(&arg("new-rule")?)? })),
        "change-part-number-input" => Ok(Iso16757Mutation::ChangePartNumberInput(ChangePartNumberInput { key: dec_str(&arg("key")?)?, new_value: dec_json(&arg("new-value")?)? })),
        "remove-part-number-input" => Ok(Iso16757Mutation::RemovePartNumberInput(RemovePartNumberInput { key: dec_str(&arg("key")?)? })),
        "change-selection-class" => Ok(Iso16757Mutation::ChangeSelectionClass(ChangeSelectionClass { new_class_id: dec_str(&arg("new-class-id")?)? })),
        "change-selection-series" => Ok(Iso16757Mutation::ChangeSelectionSeries(ChangeSelectionSeries { new_series_id: dec_opt_str(&arg("new-series-id")?)? })),
        "add-selection-constraint" => Ok(Iso16757Mutation::AddSelectionConstraint(AddSelectionConstraint { constraint: dec_json(&arg("constraint")?)? })),
        "remove-selection-constraint" => Ok(Iso16757Mutation::RemoveSelectionConstraint(RemoveSelectionConstraint { index: dec_usize(&arg("index")?)? })),
        "rename-catalogue" => Ok(Iso16757Mutation::RenameCatalogue(RenameCatalogue { new_name: dec_str(&arg("new-name")?)? })),
        "rename-manufacturer" => Ok(Iso16757Mutation::RenameManufacturer(RenameManufacturer { new_name: dec_str(&arg("new-name")?)? })),
        "create-product-group" => Ok(Iso16757Mutation::CreateProductGroup(CreateProductGroup { product_group: dec_json(&arg("product-group")?)?, index: dec_opt_usize(&arg("index")?)? })),
        "delete-product-group" => Ok(Iso16757Mutation::DeleteProductGroup(DeleteProductGroup { id: dec_str(&arg("id")?)? })),
        "rename-product-group" => Ok(Iso16757Mutation::RenameProductGroup(RenameProductGroup { id: dec_str(&arg("id")?)?, new_name: dec_str(&arg("new-name")?)? })),
        "create-product" => Ok(Iso16757Mutation::CreateProduct(CreateProduct { product: dec_json(&arg("product")?)?, index: dec_opt_usize(&arg("index")?)? })),
        "delete-product" => Ok(Iso16757Mutation::DeleteProduct(DeleteProduct { id: dec_str(&arg("id")?)? })),
        "rename-product" => Ok(Iso16757Mutation::RenameProduct(RenameProduct { id: dec_str(&arg("id")?)?, new_name: dec_str(&arg("new-name")?)? })),
        "create-property-definition" => Ok(Iso16757Mutation::CreatePropertyDefinition(CreatePropertyDefinition { property_definition: dec_json(&arg("property-definition")?)?, index: dec_opt_usize(&arg("index")?)? })),
        "delete-property-definition" => Ok(Iso16757Mutation::DeletePropertyDefinition(DeletePropertyDefinition { id: dec_str(&arg("id")?)? })),
        "create-subject" => Ok(Iso16757Mutation::CreateSubject(CreateSubject { subject: dec_json(&arg("subject")?)?, index: dec_opt_usize(&arg("index")?)? })),
        "delete-subject" => Ok(Iso16757Mutation::DeleteSubject(DeleteSubject { id: dec_str(&arg("id")?)? })),
        other => Err(format!("iso16757 mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for Iso16757Mutation {
    fn print_op(&self) -> String {
        print_iso16757_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_iso16757_mutation(line).map_err(|e| store::TextError::new(e, store::TextSpan::at(1, 1)))
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

impl protocol::OpBinary for Iso16757Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            Iso16757Mutation::ChangeExchangeProcess(_) => 0,
            Iso16757Mutation::UpdateScriptLimits(_) => 1,
            Iso16757Mutation::ReplacePartNumberRule(_) => 2,
            Iso16757Mutation::ChangePartNumberInput(_) => 3,
            Iso16757Mutation::RemovePartNumberInput(_) => 4,
            Iso16757Mutation::ChangeSelectionClass(_) => 5,
            Iso16757Mutation::ChangeSelectionSeries(_) => 6,
            Iso16757Mutation::AddSelectionConstraint(_) => 7,
            Iso16757Mutation::RemoveSelectionConstraint(_) => 8,
            Iso16757Mutation::RenameCatalogue(_) => 9,
            Iso16757Mutation::RenameManufacturer(_) => 10,
            Iso16757Mutation::CreateProductGroup(_) => 11,
            Iso16757Mutation::DeleteProductGroup(_) => 12,
            Iso16757Mutation::RenameProductGroup(_) => 13,
            Iso16757Mutation::CreateProduct(_) => 14,
            Iso16757Mutation::DeleteProduct(_) => 15,
            Iso16757Mutation::RenameProduct(_) => 16,
            Iso16757Mutation::CreatePropertyDefinition(_) => 17,
            Iso16757Mutation::DeletePropertyDefinition(_) => 18,
            Iso16757Mutation::CreateSubject(_) => 19,
            Iso16757Mutation::DeleteSubject(_) => 20,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            Iso16757Mutation::ChangeExchangeProcess(p) => write_json_bin(&mut out, &p.new_exchange_process),
            Iso16757Mutation::UpdateScriptLimits(p) => {
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
            Iso16757Mutation::AddSelectionConstraint(p) => write_json_bin(&mut out, &p.constraint),
            Iso16757Mutation::RemoveSelectionConstraint(p) => store::pack_rt::write_varint_u64(&mut out, p.index as u64),
            Iso16757Mutation::RenameCatalogue(p) => write_str_bin(&mut out, &p.new_name),
            Iso16757Mutation::RenameManufacturer(p) => write_str_bin(&mut out, &p.new_name),
            Iso16757Mutation::CreateProductGroup(p) => {
                write_json_bin(&mut out, &p.product_group);
                write_opt_usize_bin(&mut out, &p.index);
            }
            Iso16757Mutation::DeleteProductGroup(p) => write_str_bin(&mut out, &p.id),
            Iso16757Mutation::RenameProductGroup(p) => {
                write_str_bin(&mut out, &p.id);
                write_str_bin(&mut out, &p.new_name);
            }
            Iso16757Mutation::CreateProduct(p) => {
                write_json_bin(&mut out, &p.product);
                write_opt_usize_bin(&mut out, &p.index);
            }
            Iso16757Mutation::DeleteProduct(p) => write_str_bin(&mut out, &p.id),
            Iso16757Mutation::RenameProduct(p) => {
                write_str_bin(&mut out, &p.id);
                write_str_bin(&mut out, &p.new_name);
            }
            Iso16757Mutation::CreatePropertyDefinition(p) => {
                write_json_bin(&mut out, &p.property_definition);
                write_opt_usize_bin(&mut out, &p.index);
            }
            Iso16757Mutation::DeletePropertyDefinition(p) => write_str_bin(&mut out, &p.id),
            Iso16757Mutation::CreateSubject(p) => {
                write_json_bin(&mut out, &p.subject);
                write_opt_usize_bin(&mut out, &p.index);
            }
            Iso16757Mutation::DeleteSubject(p) => write_str_bin(&mut out, &p.id),
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            0 => Ok(Iso16757Mutation::ChangeExchangeProcess(ChangeExchangeProcess { new_exchange_process: read_json_bin(&mut reader).map_err(|e| malformed("new_exchange_process", reader.position(), e))? })),
            1 => {
                let new_max_steps = reader.read_varint_u64().map_err(|e| malformed("new_max_steps", reader.position(), e.to_string()))? as u32;
                let new_max_recursion = reader.read_varint_u64().map_err(|e| malformed("new_max_recursion", reader.position(), e.to_string()))? as u32;
                let new_timeout_ms = reader.read_varint_u64().map_err(|e| malformed("new_timeout_ms", reader.position(), e.to_string()))?;
                Ok(Iso16757Mutation::UpdateScriptLimits(UpdateScriptLimits { new_max_steps, new_max_recursion, new_timeout_ms }))
            }
            2 => Ok(Iso16757Mutation::ReplacePartNumberRule(ReplacePartNumberRule { new_rule: read_json_bin(&mut reader).map_err(|e| malformed("new_rule", reader.position(), e))? })),
            3 => {
                let key = read_str_bin(&mut reader).map_err(|e| malformed("key", reader.position(), e))?;
                let new_value = read_json_bin(&mut reader).map_err(|e| malformed("new_value", reader.position(), e))?;
                Ok(Iso16757Mutation::ChangePartNumberInput(ChangePartNumberInput { key, new_value }))
            }
            4 => Ok(Iso16757Mutation::RemovePartNumberInput(RemovePartNumberInput { key: read_str_bin(&mut reader).map_err(|e| malformed("key", reader.position(), e))? })),
            5 => Ok(Iso16757Mutation::ChangeSelectionClass(ChangeSelectionClass { new_class_id: read_str_bin(&mut reader).map_err(|e| malformed("new_class_id", reader.position(), e))? })),
            6 => Ok(Iso16757Mutation::ChangeSelectionSeries(ChangeSelectionSeries { new_series_id: read_opt_str_bin(&mut reader).map_err(|e| malformed("new_series_id", reader.position(), e))? })),
            7 => Ok(Iso16757Mutation::AddSelectionConstraint(AddSelectionConstraint { constraint: read_json_bin(&mut reader).map_err(|e| malformed("constraint", reader.position(), e))? })),
            8 => {
                let index = reader.read_varint_u64().map_err(|e| malformed("index", reader.position(), e.to_string()))? as usize;
                Ok(Iso16757Mutation::RemoveSelectionConstraint(RemoveSelectionConstraint { index }))
            }
            9 => Ok(Iso16757Mutation::RenameCatalogue(RenameCatalogue { new_name: read_str_bin(&mut reader).map_err(|e| malformed("new_name", reader.position(), e))? })),
            10 => Ok(Iso16757Mutation::RenameManufacturer(RenameManufacturer { new_name: read_str_bin(&mut reader).map_err(|e| malformed("new_name", reader.position(), e))? })),
            11 => {
                let product_group = read_json_bin(&mut reader).map_err(|e| malformed("product_group", reader.position(), e))?;
                let index = read_opt_usize_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                Ok(Iso16757Mutation::CreateProductGroup(CreateProductGroup { product_group, index }))
            }
            12 => Ok(Iso16757Mutation::DeleteProductGroup(DeleteProductGroup { id: read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))? })),
            13 => {
                let id = read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))?;
                let new_name = read_str_bin(&mut reader).map_err(|e| malformed("new_name", reader.position(), e))?;
                Ok(Iso16757Mutation::RenameProductGroup(RenameProductGroup { id, new_name }))
            }
            14 => {
                let product = read_json_bin(&mut reader).map_err(|e| malformed("product", reader.position(), e))?;
                let index = read_opt_usize_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                Ok(Iso16757Mutation::CreateProduct(CreateProduct { product, index }))
            }
            15 => Ok(Iso16757Mutation::DeleteProduct(DeleteProduct { id: read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))? })),
            16 => {
                let id = read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))?;
                let new_name = read_str_bin(&mut reader).map_err(|e| malformed("new_name", reader.position(), e))?;
                Ok(Iso16757Mutation::RenameProduct(RenameProduct { id, new_name }))
            }
            17 => {
                let property_definition = read_json_bin(&mut reader).map_err(|e| malformed("property_definition", reader.position(), e))?;
                let index = read_opt_usize_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                Ok(Iso16757Mutation::CreatePropertyDefinition(CreatePropertyDefinition { property_definition, index }))
            }
            18 => Ok(Iso16757Mutation::DeletePropertyDefinition(DeletePropertyDefinition { id: read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))? })),
            19 => {
                let subject = read_json_bin(&mut reader).map_err(|e| malformed("subject", reader.position(), e))?;
                let index = read_opt_usize_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                Ok(Iso16757Mutation::CreateSubject(CreateSubject { subject, index }))
            }
            20 => Ok(Iso16757Mutation::DeleteSubject(DeleteSubject { id: read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))? })),
            other => Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        }
    }
}
//#endregion 🔖️OpBinaryCodec

//#region 🔖️DemoCases
/// 🧪️ One representative value per variant — reused by the round-trip law test below.
#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<Iso16757Mutation> {
    use crate::artifacts::iso16757::{part_1, part_4, part_5, Cardinality, CatalogueValue, LocalizedText, Names};

    let names = |text: &str| Names { preferred: LocalizedText { locale: "en".into(), text: text.into() }, short_name: None, alternatives: Vec::new() };

    vec![
        Iso16757Mutation::ChangeExchangeProcess(ChangeExchangeProcess { new_exchange_process: part_5::ExchangeProcess::ProvideCatalogue }),
        Iso16757Mutation::UpdateScriptLimits(UpdateScriptLimits { new_max_steps: 1, new_max_recursion: 2, new_timeout_ms: 3 }),
        Iso16757Mutation::ReplacePartNumberRule(ReplacePartNumberRule { new_rule: part_5::PartNumberRule::Literal { value: "X-1".into() } }),
        Iso16757Mutation::ChangePartNumberInput(ChangePartNumberInput { key: "dn".into(), new_value: CatalogueValue::Decimal { value: 50.0 } }),
        Iso16757Mutation::RemovePartNumberInput(RemovePartNumberInput { key: "dn".into() }),
        Iso16757Mutation::ChangeSelectionClass(ChangeSelectionClass { new_class_id: "class.valve".into() }),
        Iso16757Mutation::ChangeSelectionSeries(ChangeSelectionSeries { new_series_id: Some("series.cv".into()) }),
        Iso16757Mutation::ChangeSelectionSeries(ChangeSelectionSeries { new_series_id: None }),
        Iso16757Mutation::AddSelectionConstraint(AddSelectionConstraint { constraint: part_1::SelectionConstraint { property_id: "prop.dn".into(), operator: part_1::ConstraintOperator::Equal, value: CatalogueValue::Decimal { value: 50.0 } } }),
        Iso16757Mutation::RemoveSelectionConstraint(RemoveSelectionConstraint { index: 0 }),
        Iso16757Mutation::RenameCatalogue(RenameCatalogue { new_name: "Renamed \"Catalogue\"".into() }),
        Iso16757Mutation::RenameManufacturer(RenameManufacturer { new_name: "Renamed Mfg".into() }),
        Iso16757Mutation::CreateProductGroup(CreateProductGroup { product_group: part_1::ProductGroup { id: "group.new".into(), names: names("New Group"), dictionary_subject_id: None }, index: Some(0) }),
        Iso16757Mutation::DeleteProductGroup(DeleteProductGroup { id: "group.valves".into() }),
        Iso16757Mutation::RenameProductGroup(RenameProductGroup { id: "group.valves".into(), new_name: "Renamed Group".into() }),
        Iso16757Mutation::CreateProduct(CreateProduct {
            product: part_1::Product { id: "product.new".into(), series_id: "series.cv".into(), names: names("New Product"), parameter_domains: Vec::new(), variants: Vec::new(), static_properties: Vec::new() },
            index: None,
        }),
        Iso16757Mutation::DeleteProduct(DeleteProduct { id: "product.cv".into() }),
        Iso16757Mutation::RenameProduct(RenameProduct { id: "product.cv".into(), new_name: "Renamed Product".into() }),
        Iso16757Mutation::CreatePropertyDefinition(CreatePropertyDefinition {
            property_definition: part_1::PropertyDefinition {
                id: "prop.new".into(),
                names: names("New Prop"),
                data_type: "text".into(),
                unit: None,
                cardinality: Cardinality::optional(),
                kind: part_1::PropertyKind::Static,
                dictionary_property_id: None,
            },
            index: None,
        }),
        Iso16757Mutation::DeletePropertyDefinition(DeletePropertyDefinition { id: "prop.dn".into() }),
        Iso16757Mutation::CreateSubject(CreateSubject {
            subject: part_4::Subject { id: "subject.new".into(), kind: part_4::SubjectKind::ProductClass, names: names("New Subject"), definition: LocalizedText { locale: "en".into(), text: "def".into() }, parent_id: None },
            index: None,
        }),
        Iso16757Mutation::DeleteSubject(DeleteSubject { id: "subject.valve".into() }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{OpBinary, OpText};

    #[semio_framework_async_macros::async_test]
    fn op_text_binary_roundtrip_law() {
        for mutation in demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = <Iso16757Mutation as OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed: {e}"));
            let decoded = <Iso16757Mutation as OpBinary>::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch");
        }
    }
}
//#endregion 🧪️Tests
