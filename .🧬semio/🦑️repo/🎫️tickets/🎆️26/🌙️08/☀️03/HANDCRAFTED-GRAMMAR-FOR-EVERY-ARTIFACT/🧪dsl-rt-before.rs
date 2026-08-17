//#region 🔖️Runtime
/// @emoji ⚙️ Thin wrappers the derive-generated `impl crate::os_store::DocumentDsl`/`impl crate::os_spr::OpText`
/// bodies call into — kept as free functions (not methods) so generated code never has to name
/// this crate's internal types, only `crate::os_dsl::__rt::*`.
pub mod __rt {
    use super::*;

    pub fn parse_document_record(text: &str, spec: &RecordSpec) -> Result<RecordValue, TextError> {
        parse(text, spec, &ParseOptions { limits: Limits::default(), mode: SourceMode::Document })
    }

    pub fn print_document_record(value: &RecordValue, spec: &RecordSpec) -> String {
        print(value, spec, JoinMode::Document)
    }

    pub fn parse_inline_record(text: &str, spec: &RecordSpec) -> Result<RecordValue, TextError> {
        parse(text, spec, &ParseOptions { limits: Limits::default(), mode: SourceMode::Inline })
    }

    pub fn print_inline_record(value: &RecordValue, spec: &RecordSpec) -> String {
        print(value, spec, JoinMode::Inline)
    }

    pub fn field_error(message: impl Into<String>) -> TextError {
        TextError::new(message, TextSpan::at(1, 1))
    }

    /// @emoji 📐️ Resolves a `#[dsl(unit = "...")]`/`#[dsl(angle = "...")]` symbol at spec-build
    /// time. An unknown symbol is a derive-time misuse (a typo'd unit string, caught the first time
    /// the generated `__dsl_spec` runs — every RecordSpec-law test exercises this), so it panics
    /// rather than threading a `Result` through the whole spec-building call chain, matching
    /// `newtype_variant_spec`'s convention above.
    pub fn unit_for_derive(symbol: &'static str) -> &'static UnitSpec {
        unit_by_symbol(symbol).unwrap_or_else(|| panic!("dsl: unknown unit symbol '{symbol}' in #[dsl(unit = ...)]/#[dsl(angle = ...)]"))
    }

    /// @emoji 📦️ Single-field tuple ("newtype") enum variant support — `Variant(Body)` delegates its
    /// whole `RecordSpec`/value to `Body`'s own `DslField` impl rather than wrapping it in one
    /// positional field, so `Body` prints/parses identically whether reached through the enum or on
    /// its own. `Body` must have `Shape::Record` (i.e. itself come from `#[derive(DslRecord)]` or
    /// `#[derive(DslDocument)]`) — anything else is a derive-time misuse, hence the panic rather than
    /// a `Result` (there is no sensible recoverable path for a grammar that's wrong at compile time).
    pub fn newtype_variant_spec<T: DslField>() -> RecordSpec {
        match T::shape() {
            Shape::Record(spec_fn) => spec_fn(),
            other => panic!("newtype variant's inner type must have Record shape, found {other:?}"),
        }
    }

    pub fn newtype_variant_to_record<T: DslField>(inner: &T) -> RecordValue {
        match inner.to_value() {
            FieldValue::Record(record) => record,
            other => panic!("newtype variant's inner type must produce a Record value, found {other:?}"),
        }
    }

    pub fn newtype_variant_from_record<T: DslField>(record: &RecordValue) -> Result<T, TextError> {
        T::from_value(&FieldValue::Record(record.clone())).map_err(field_error)
    }
}
//#endregion 🔖️Runtime

//#region 🔖️OpRt
/// @emoji 🎯️ Runtime behind `crate::os_spr::OpBinary`, resolved as `::crate::os_dsl::op_rt::...` by
/// `#[derive(DslOps)]`'s emitted impl — the binary twin of `__rt`'s inline text path and the
/// op-level mirror of the `DocumentDsl`/`DocumentPack` pairing. An operation enum lowers to
/// `(variant keyword, RecordSpec, RecordValue)` via [`DslVariants`]; this module fixes the byte
/// layout `format u8 (=1) | variant ordinal varint | record body` where the ordinal indexes
/// `DslVariants::variants()` declaration order (reordering variants is a format break) and the
/// body is `crate::os_pack::encode_record_body`'s container-less encoding. Lives here rather than in
/// `store` because the bound is this crate's own `DslVariants` — a `store`-hosted twin would be a
/// distinct trait instance inside this crate's own test build (dev-dependency cycle). LAW:
/// `decode_op(&encode_op(op)) == op == parse_op(&print_op(op))`, and encoding is deterministic.
pub mod op_rt {
    use super::DslVariants;
    use crate::os_spr::ProtocolError;

    /// @emoji 🎯️ Format byte every encoded operation starts with.
    pub const OP_BINARY_FORMAT: u8 = 1;

    /// @emoji 🎯️ Encodes one operation deterministically (byte-identical for equal ops).
    pub fn encode_op<T: DslVariants>(op: &T) -> Result<Vec<u8>, ProtocolError> {
        let (keyword, record) = op.to_named_record();
        let variants = T::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1)();
        let body = crate::os_pack::encode_record_body(&spec, &record, &crate::os_pack::EncodeOptions::default())?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        crate::os_pack::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }

    /// @emoji 🎯️ Inverse of [`encode_op`].
    pub fn decode_op<T: DslVariants>(bytes: &[u8]) -> Result<T, ProtocolError> {
        let mut reader = crate::os_pack::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = T::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = crate::os_pack::decode_record_body(body, &spec, &crate::os_pack::DecodeOptions::default())?;
        T::from_named_record(keyword, &record).map_err(|error| ProtocolError::Malformed { what: "op record", offset: reader.position() as u64, detail: error.to_string() })
    }
}
//#endregion 🔖️OpRt
