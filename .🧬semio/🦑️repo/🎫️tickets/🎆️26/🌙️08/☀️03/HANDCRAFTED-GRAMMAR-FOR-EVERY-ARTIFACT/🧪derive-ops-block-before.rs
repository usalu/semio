    let expanded = quote! {
        #variants_impl

        // 🎞️ `OpText` lives in `protocol_command`, re-exported as `protocol::OpText` — every
        // `#[derive(dsl::DslOps)]` crate depends on `protocol` directly for its `Operation` impl
        // anyway, so this resolves without new Cargo.toml deps. The error type stays
        // `::store::TextError` (a transparent re-export of `dsl_core::TextError`, the exact type
        // `protocol::OpText::parse_op` declares) rather than switching to `::dsl_core::TextError`
        // directly, since not every deriving crate has `dsl_core` as a *direct* dependency.
        impl ::protocol::OpText for #name {
            fn parse_op(line: &str) -> Result<Self, ::store::TextError> {
                let variants = <Self as ::dsl::DslVariants>::variants();
                for (keyword, spec_fn) in &variants {
                    let probe = format!("{} ", keyword);
                    if line == keyword.as_str() || line.starts_with(&probe) {
                        let record = ::dsl::__rt::parse_inline_record(line, &spec_fn())?;
                        return <Self as ::dsl::DslVariants>::from_named_record(keyword, &record);
                    }
                }
                Err(::dsl::__rt::field_error(format!("unknown operation line '{line}'")))
            }
            fn print_op(&self) -> String {
                let (keyword, record) = <Self as ::dsl::DslVariants>::to_named_record(self);
                let variants = <Self as ::dsl::DslVariants>::variants();
                let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
                ::dsl::__rt::print_inline_record(&record, &spec_fn())
            }
        }

        // 🎞️ Binary twin of the `OpText` impl above — same `DslVariants` lowering, byte layout
        // owned by `::dsl::op_rt` (`format u8 | variant ordinal varint | record body`), the op-level
        // mirror of the `DocumentDsl`/`DocumentPack` pairing. Resolves through `dsl` (not `store`)
        // because the runtime's bound is `dsl::DslVariants` itself — see `dsl::op_rt`'s doc.
        impl ::protocol::OpBinary for #name {
            fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
                ::dsl::op_rt::encode_op(self)
            }
            fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
                ::dsl::op_rt::decode_op(bytes)
            }
        }
    };
