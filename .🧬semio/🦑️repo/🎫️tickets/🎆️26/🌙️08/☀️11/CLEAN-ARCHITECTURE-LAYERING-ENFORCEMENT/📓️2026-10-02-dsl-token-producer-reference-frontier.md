# DSL Token Producer Reference Frontier

Read-only review of the actual remaining second-census unresolved producer. No source scanner exception or production edit is made by this review.

Observed source: 🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs; current full-source SHA-256 fe9949e8647bdfeac62ec856602bf525ef2d9136f4efcf6214a1b56d52a5d713.
The actual expand_mutation_leaf body resolves source authority, taxonomy and descriptor files, a payload schema and referenced schema documents, then interpolates their paths into four include_str token positions inside a quote! result. The source scanner currently refuses these nonliteral references.

The source-level quoted token positions need producer-versus-consumer provenance and actual resulting compilation-input evidence before the whole gate can admit them. A string/macro-name exemption, deletion of reference tracking, or conversion to untracked filesystem reads would not establish the deletion rule. Current native runs on other owners do not prove this specific complete generated-consumer mapping.

Future work must retain the actual compiler dependency includes, prove the mounted consumer contexts and actual emitted path inputs, and cover hostile producers/opaque expansions and absence with independent native oracles. No bounded proof here is represented as whole-tree closure.

```rust
pub fn expand_mutation_leaf(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    if matches!(input.data, Data::Union(_)) { return syn::Error::new_spanned(&input, "MutationLeaf does not support unions").to_compile_error().into(); }
    let attrs = match parse_mutation_leaf_attrs(&input) { Ok(attrs) => attrs, Err(error) => return error.to_compile_error().into() };
    let source = match input.ident.span().unwrap().local_file() { Some(source) => source, None => return syn::Error::new_spanned(&input, "MutationLeaf requires a local source file").to_compile_error().into() };
    let compiler_cwd = match std::env::current_dir() { Ok(path) => path, Err(error) => return syn::Error::new_spanned(&input, error.to_string()).to_compile_error().into() };
    let authority = match mutation_source_authority(&source, &compiler_cwd) { Ok(authority) => authority, Err(error) => return syn::Error::new_spanned(&input, format!("MutationLeaf source authority failed: {error}")).to_compile_error().into() };
    let raw_descriptor = match fs::read(&authority.descriptor_path) { Ok(raw) => raw, Err(error) => return syn::Error::new_spanned(&input, error.to_string()).to_compile_error().into() };
    let descriptor = match parse_mutation_leaf_descriptor(&raw_descriptor, &authority) { Ok(descriptor) => descriptor, Err(error) => return syn::Error::new_spanned(&input, format!("MutationLeaf descriptor failed: {error}")).to_compile_error().into() };
    let workspace_token = match mutation_leaf_workspace_token(&authority) { Ok(token) => token, Err(error) => return syn::Error::new_spanned(&input, format!("MutationLeaf provenance failed: {error}")).to_compile_error().into() };
    let mutation_root = match mutation_authority_relative(&authority.workspace_root, &authority.mutation_root) { Ok(path) => path, Err(error) => return syn::Error::new_spanned(&input, error).to_compile_error().into() };
    let source_path = match mutation_authority_relative(&authority.workspace_root, &authority.source_path) { Ok(path) => path, Err(error) => return syn::Error::new_spanned(&input, error).to_compile_error().into() };
    let descriptor_path = match mutation_authority_relative(&authority.workspace_root, &authority.descriptor_path) { Ok(path) => path, Err(error) => return syn::Error::new_spanned(&input, error).to_compile_error().into() };
    let taxonomy_path = match mutation_authority_relative(&authority.workspace_root, &authority.taxonomy_path) { Ok(path) => path, Err(error) => return syn::Error::new_spanned(&input, error).to_compile_error().into() };
    let payload_schema_path = match mutation_leaf_payload_schema_path(&authority, &descriptor.payload_schema) { Ok(path) => path, Err(error) => return syn::Error::new_spanned(&input, format!("MutationLeaf payload schema failed: {error}")).to_compile_error().into() };
    let referenced_documents = match mutation_leaf_referenced_documents(&payload_schema_path).and_then(|paths| paths.iter().map(|path| mutation_leaf_include_path(path)).collect::<Result<Vec<_>, _>>()) { Ok(paths) => paths, Err(error) => return syn::Error::new_spanned(&input, format!("MutationLeaf referenced schema documents failed: {error}")).to_compile_error().into() };
    let dependency_paths = [authority.taxonomy_path.clone(), authority.descriptor_path.clone(), payload_schema_path];
    let dependency_paths: Result<Vec<_>, _> = dependency_paths.iter().map(|path| mutation_leaf_include_path(path)).collect();
    let dependency_paths = match dependency_paths { Ok(paths) => paths, Err(error) => return syn::Error::new_spanned(&input, error).to_compile_error().into() };
    let [taxonomy_dependency, descriptor_dependency, payload_schema_dependency]: [String; 3] = match dependency_paths.try_into() { Ok(paths) => paths, Err(_) => unreachable!() };
    let name = &input.ident;
    let contract = &attrs.contract;
    let editable = attrs.payload.as_ref().map(|variant| quote! {
        fn input_schema(&self) -> ::core::option::Option<&'static str> {
            match self { Self::#variant(_) => ::core::option::Option::Some(<Self as #contract::MutationLeaf>::PAYLOAD_SCHEMA), _ => ::core::option::Option::None }
        }
        fn input_value(&self) -> #contract::DslValue {
            match self { Self::#variant(payload) => #contract::ToValue::to_value(payload), _ => #contract::ToValue::to_value(self) }
        }
        fn with_input_value(&self, value: #contract::DslValue) -> ::core::result::Result<Self, #contract::ValueError> {
            match self {
                Self::#variant(_) => #contract::FromValue::from_value(value).map(Self::#variant),
                _ => ::core::result::Result::Err(#contract::ValueError::new(::std::format!("{} is editable only as {}", ::core::stringify!(#name), ::core::stringify!(#variant)))),
            }
        }
        fn from_input_value(value: #contract::DslValue) -> ::core::result::Result<Self, #contract::ValueError> {
            #contract::FromValue::from_value(value).map(Self::#variant)
        }
    });
    let owner = &authority.owner;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let descriptor = emit_mutation_leaf_descriptor(contract, &descriptor);
    let workspace_token = workspace_token.iter();
    quote! {
        const _: &str = ::core::include_str!(#taxonomy_dependency);
        const _: &str = ::core::include_str!(#descriptor_dependency);
        impl #impl_generics #contract::MutationLeaf for #name #ty_generics #where_clause {
            const DESCRIPTOR: #contract::MutationLeafDescriptor = #descriptor;
            const PROVENANCE: #contract::MutationSourceProvenance = #contract::MutationSourceProvenance { workspace_token: [#(#workspace_token),*], mutation_root: #mutation_root, owner: #owner, source_path: #source_path, descriptor_path: #descriptor_path, taxonomy_path: #taxonomy_path };
            const PAYLOAD_SCHEMA: &'static str = ::core::include_str!(#payload_schema_dependency);
            const PAYLOAD_SCHEMA_DOCUMENTS: &'static [&'static str] = &[#(::core::include_str!(#referenced_documents)),*];
            #editable
        }
    }.into()
}

```
