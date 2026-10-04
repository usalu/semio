/// ✉️ Owns artifact envelope metadata independently from canonical Record fields.
pub fn expand_dsl_document(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    if !matches!(input.data, Data::Struct(_)) {
        return syn::Error::new_spanned(&input, "DslArtifact only supports structs").to_compile_error().into();
    }
    let mut id: Option<String> = None;
    let mut extension: Option<String> = None;
    for attribute in &input.attrs {
        if !attribute.path().is_ident("artifact") { continue; }
        if let Err(error) = attribute.parse_nested_meta(|meta| {
            let target = if meta.path.is_ident("id") { &mut id } else if meta.path.is_ident("extension") { &mut extension } else { return Err(meta.error("unsupported artifact attribute")); };
            if target.is_some() { return Err(meta.error("duplicate artifact attribute")); }
            let value: syn::LitStr = meta.value()?.parse()?;
            *target = Some(value.value());
            Ok(())
        }) { return error.to_compile_error().into(); }
    }
    let envelope_id = match id.or_else(|| extension.clone()) {
        Some(id) => id,
        None => return syn::Error::new_spanned(&input, "DslArtifact requires #[artifact(id = \"plugin.artifact\")] or #[artifact(extension = \"...\")]").to_compile_error().into(),
    };
    let suffix = extension.as_deref().unwrap_or_else(|| envelope_id.rsplit('.').next().unwrap_or(&envelope_id));
    let name = &input.ident;
    quote! {
        impl #name {
            pub const __DSL_ENVELOPE_ID: &'static str = #envelope_id;
            pub const __DSL_EXTENSION: &'static str = #suffix;
        }
    }.into()
}

/// 🧩 Owns OS diff transport semantics over canonical Record projection and construction.
pub fn expand_dsl_diff(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    if !matches!(input.data, Data::Struct(_)) {
        return syn::Error::new_spanned(&input, "DslDiff only supports structs").to_compile_error().into();
    }
    let name = &input.ident;
    quote! {
        impl ::semio_framework_os_kernel::DiffCodec for #name {
            fn print_diff(&self) -> String {
                ::semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), ::semio_framework_dsl_record::JoinMode::Inline)
            }
            fn parse_diff(line: &str) -> Result<Self, ::semio_framework_diagnostic::TextError> {
                let record = ::semio_framework_dsl_record::parse(line, &Self::__dsl_spec(), &::semio_framework_dsl_record::ParseOptions { limits: ::semio_framework_diagnostic::Limits::default(), mode: ::semio_framework_dsl_record::SourceMode::Inline })?;
                Self::__dsl_from_record(&record)
            }
            fn encode_diff(&self) -> Result<Vec<u8>, ::semio_framework_os_kernel::ProtocolError> {
                ::semio_framework_os_kernel::os_store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), &::semio_framework_os_kernel::os_store::PackEncodeOptions::default()).map_err(::semio_framework_os_kernel::ProtocolError::from)
            }
            fn decode_diff(bytes: &[u8]) -> Result<Self, ::semio_framework_os_kernel::ProtocolError> {
                let (record, _report) = ::semio_framework_os_kernel::os_store::pack_rt::decode_document(bytes, &Self::__dsl_spec(), &::semio_framework_os_kernel::os_store::PackDecodeOptions::default()).map_err(::semio_framework_os_kernel::ProtocolError::from)?;
                Self::__dsl_from_record(&record).map_err(|error| ::semio_framework_os_kernel::ProtocolError::Malformed { what: "diff record", offset: 0, detail: error.to_string() })
            }
        }
    }.into()
}
