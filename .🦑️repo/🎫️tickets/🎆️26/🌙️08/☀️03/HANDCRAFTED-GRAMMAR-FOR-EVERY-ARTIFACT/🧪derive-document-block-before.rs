    let expanded = quote! {
        impl #name {
            pub fn __dsl_spec() -> ::dsl::RecordSpec {
                ::dsl::RecordSpec::new_owned(#keyword_expr, #layout_expr, vec![ #(#spec_exprs),* ])
            }
            pub fn __dsl_to_record(&self) -> ::dsl::RecordValue {
                let mut record = ::dsl::RecordValue::default();
                #(#to_value_stmts)*
                record
            }
            pub fn __dsl_from_record(record: &::dsl::RecordValue) -> Result<Self, ::store::TextError> {
                #(#from_value_stmts)*
                Ok(Self { #(#field_idents),* })
            }
        }

        impl ::store::DocumentDsl for #name {
            const EXTENSION: &'static str = #extension_suffix_lit;
            fn envelope_id() -> &'static str {
                #envelope_id_lit
            }
            fn parse_dsl(text: &str) -> Result<Self, ::store::TextError> {
                let body = match ::store::semio_format::split_text_preamble(text) {
                    Ok((_, rest)) => rest,
                    Err(_) => text,
                };
                let record = ::dsl::__rt::parse_document_record(body, &Self::__dsl_spec())?;
                Self::__dsl_from_record(&record)
            }
            fn print_dsl(&self) -> String {
                let body = ::dsl::__rt::print_document_record(&self.__dsl_to_record(), &Self::__dsl_spec());
                let envelope = ::store::semio_format::SemioEnvelope::from_envelope_id(
                    <Self as ::store::DocumentDsl>::envelope_id(),
                    ::store::semio_format::Component::Dsl,
                    1,
                ).expect("valid envelope_id");
                ::store::semio_format::wrap_text(&envelope, &body)
            }
        }

        // A document type can also be nested as an ordinary field (e.g. a "whole document
        // snapshot" operation variant), so it needs `DslField` too, not just `store::DocumentDsl`.
        impl ::dsl::DslField for #name {
            fn shape() -> ::dsl::Shape {
                ::dsl::Shape::Record(Self::__dsl_spec)
            }
            fn to_value(&self) -> ::dsl::FieldValue {
                ::dsl::FieldValue::Record(self.__dsl_to_record())
            }
            fn from_value(value: &::dsl::FieldValue) -> Result<Self, String> {
                match value {
                    ::dsl::FieldValue::Record(record) => Self::__dsl_from_record(record).map_err(|e| e.message),
                    other => Err(format!("expected Record, found {other:?}")),
                }
            }
        }

        // 📦️ Binary counterpart of the `store::DocumentDsl` impl above — same `__dsl_spec`/
        // `__dsl_to_record`/`__dsl_from_record` trio, routed through `pack` instead of the DSL
        // grammar engine. `store::text_error_to_pack_error` (a free function, not `PackError: From
        // <TextError>` — that impl is an orphan-rule violation since neither type is local to
        // `store`) bridges `__dsl_from_record`'s `TextError` into `PackError`.
        impl ::store::DocumentPack for #name {
            fn encode_pack_with(&self, options: &::store::PackEncodeOptions) -> Result<Vec<u8>, ::store::PackError> {
                let inner = ::store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
                let envelope = ::store::semio_format::SemioEnvelope::from_envelope_id(
                    <Self as ::store::DocumentDsl>::envelope_id(),
                    ::store::semio_format::Component::Pack,
                    1,
                ).map_err(|e| ::store::PackError::Schema(e.to_string()))?;
                Ok(::store::semio_format::wrap_binary(&envelope, &inner))
            }
            fn decode_pack_with(bytes: &[u8], options: &::store::PackDecodeOptions) -> Result<Self, ::store::PackError> {
                let (envelope, inner) = ::store::semio_format::unwrap_binary(bytes)
                    .map_err(|e| ::store::PackError::Schema(e.to_string()))?;
                if envelope.envelope_id() != <Self as ::store::DocumentDsl>::envelope_id() {
                    return Err(::store::PackError::Schema(format!(
                        "pack envelope mismatch: expected {}, got {}",
                        <Self as ::store::DocumentDsl>::envelope_id(),
                        envelope.envelope_id()
                    )));
                }
                let (record, _report) = ::store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
                Self::__dsl_from_record(&record).map_err(::store::text_error_to_pack_error)
            }
            fn record_spec() -> Option<::dsl::RecordSpec> {
                Some(Self::__dsl_spec())
            }
        }
    };
