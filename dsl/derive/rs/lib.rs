//! 🧬 `dsl_derive` — compiles `#[dsl(...)]`-annotated struct/enum declarations into
//! `vcs::DocumentDsl`/`vcs::OpText` implementations (and the `dsl::DslField`/`dsl::DslVariants`
//! bindings nested usage composes through), so a technology declares its grammar instead of
//! hand-writing a parser/printer. Analyze → IR → emit, per the repo's `fsm_macros` convention.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type};

//#region 🔖Attrs
#[derive(Default, Clone)]
struct ContainerAttrs {
    extension: Option<String>,
    keyword: Option<String>,
    lines_layout: bool,
}

#[derive(Default, Clone)]
struct FieldAttrs {
    key: Option<String>,
    positional: bool,
    ident: bool,
    list: bool,
    tuple: bool,
    statements: bool,
    base64: bool,
    flatten: bool,
    raw_lines_count_field: Option<String>,
}

fn parse_container_attrs(input: &DeriveInput) -> ContainerAttrs {
    let mut out = ContainerAttrs::default();
    for attr in &input.attrs {
        if !attr.path().is_ident("dsl") {
            continue;
        }
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("extension") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.extension = Some(value.value());
            } else if meta.path.is_ident("keyword") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.keyword = Some(value.value());
            } else if meta.path.is_ident("layout") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.lines_layout = value.value() == "lines";
            }
            Ok(())
        });
    }
    out
}

fn parse_field_attrs(attrs: &[syn::Attribute]) -> FieldAttrs {
    let mut out = FieldAttrs::default();
    for attr in attrs {
        if !attr.path().is_ident("dsl") {
            continue;
        }
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("key") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.key = Some(value.value());
            } else if meta.path.is_ident("positional") {
                out.positional = true;
            } else if meta.path.is_ident("ident") {
                out.ident = true;
            } else if meta.path.is_ident("list") {
                out.list = true;
            } else if meta.path.is_ident("tuple") {
                out.tuple = true;
            } else if meta.path.is_ident("statements") {
                out.statements = true;
            } else if meta.path.is_ident("base64") {
                out.base64 = true;
            } else if meta.path.is_ident("flatten") {
                out.flatten = true;
            } else if meta.path.is_ident("raw_lines") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.raw_lines_count_field = Some(value.value());
            }
            Ok(())
        });
    }
    out
}
//#endregion 🔖Attrs

//#region 🔖TypeShape
enum FieldKind {
    Scalar,
    OptionScalar(Box<Type>),
    VecList(Box<Type>),
    VecTuple(Box<Type>),
    VecStatements(Box<Type>),
    Bytes64,
    IdentString,
}

fn inner_of(ty: &Type, wrapper: &str) -> Option<Type> {
    let Type::Path(path) = ty else { return None };
    let segment = path.path.segments.last()?;
    if segment.ident != wrapper {
        return None;
    }
    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else { return None };
    args.args.iter().find_map(|arg| match arg {
        syn::GenericArgument::Type(t) => Some(t.clone()),
        _ => None,
    })
}

fn is_vec_u8(ty: &Type) -> bool {
    inner_of(ty, "Vec").is_some_and(|inner| matches!(&inner, Type::Path(p) if p.path.is_ident("u8")))
}

fn classify_field(ty: &Type, attrs: &FieldAttrs) -> (FieldKind, Type) {
    if let Some(inner) = inner_of(ty, "Option") {
        return (FieldKind::OptionScalar(Box::new(inner.clone())), inner);
    }
    if attrs.base64 && is_vec_u8(ty) {
        return (FieldKind::Bytes64, ty.clone());
    }
    if let Some(inner) = inner_of(ty, "Vec") {
        if attrs.statements {
            return (FieldKind::VecStatements(Box::new(inner.clone())), inner);
        }
        if attrs.tuple {
            return (FieldKind::VecTuple(Box::new(inner.clone())), inner);
        }
        return (FieldKind::VecList(Box::new(inner.clone())), inner);
    }
    if attrs.ident && matches!(ty, Type::Path(p) if p.path.is_ident("String")) {
        return (FieldKind::IdentString, ty.clone());
    }
    (FieldKind::Scalar, ty.clone())
}
//#endregion 🔖TypeShape

//#region 🔖RecordCodegen
struct FieldPlan {
    ident: syn::Ident,
    id: u16,
    key: String,
    positional: Option<u16>,
    optional: bool,
    kind: FieldKind,
    elem_ty: Type,
}

fn plan_fields(fields: &Fields) -> Vec<FieldPlan> {
    let mut positional_counter: u16 = 0;
    let mut out = Vec::new();
    for (index, field) in fields.iter().enumerate() {
        let attrs = parse_field_attrs(&field.attrs);
        let ident = field.ident.clone().expect("dsl_derive only supports named fields");
        let (kind, elem_ty) = classify_field(&field.ty, &attrs);
        let key = attrs.key.clone().unwrap_or_else(|| ident.to_string());
        let optional = matches!(kind, FieldKind::OptionScalar(_));
        let positional = if attrs.positional {
            let p = positional_counter;
            positional_counter += 1;
            Some(p)
        } else {
            None
        };
        out.push(FieldPlan { ident, id: index as u16, key, positional, optional, kind, elem_ty });
    }
    out
}

/// @emoji 🏗️ Builds the three code fragments shared by `DslRecord`/`DslDocument`/`DslOps` variant
/// bodies: the `RecordSpec` field-spec expressions, the struct→`RecordValue` conversion, and the
/// `RecordValue`→struct conversion.
fn record_codegen(fields: &Fields) -> (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>, Vec<syn::Ident>) {
    let plans = plan_fields(fields);
    let mut spec_exprs = Vec::new();
    let mut to_value_stmts = Vec::new();
    let mut from_value_stmts = Vec::new();
    let mut field_idents = Vec::new();

    for plan in &plans {
        let FieldPlan { ident, id, key, positional, optional, kind, elem_ty } = plan;
        field_idents.push(ident.clone());
        let pos_expr = match positional {
            Some(p) => quote! { .positional(#p as u8) },
            None => quote! {},
        };
        let opt_expr = if *optional { quote! { .optional() } } else { quote! {} };

        let (shape_expr, to_value_expr, from_value_expr): (proc_macro2::TokenStream, proc_macro2::TokenStream, proc_macro2::TokenStream) = match kind {
            FieldKind::Scalar => (
                quote! { <#elem_ty as ::dsl::DslField>::shape() },
                quote! { ::dsl::DslField::to_value(&self.#ident) },
                quote! { <#elem_ty as ::dsl::DslField>::from_value(value).map_err(::dsl::__rt::field_error)? },
            ),
            FieldKind::IdentString => (
                quote! { ::dsl::Shape::Ident },
                quote! { ::dsl::FieldValue::Ident(self.#ident.clone()) },
                quote! {
                    match value {
                        ::dsl::FieldValue::Ident(s) | ::dsl::FieldValue::Text(s) => s.clone(),
                        other => return Err(::dsl::__rt::field_error(format!("expected Ident, found {other:?}"))),
                    }
                },
            ),
            FieldKind::Bytes64 => (
                quote! { ::dsl::Shape::Bytes64 },
                quote! { ::dsl::FieldValue::Bytes64(self.#ident.clone()) },
                quote! {
                    match value {
                        ::dsl::FieldValue::Bytes64(bytes) => bytes.clone(),
                        other => return Err(::dsl::__rt::field_error(format!("expected Bytes64, found {other:?}"))),
                    }
                },
            ),
            FieldKind::OptionScalar(inner) => (
                quote! { <#inner as ::dsl::DslField>::shape() },
                quote! {
                    match &self.#ident {
                        Some(v) => ::dsl::DslField::to_value(v),
                        None => ::dsl::FieldValue::Absent,
                    }
                },
                quote! {
                    match value {
                        ::dsl::FieldValue::Absent => None,
                        other => Some(<#inner as ::dsl::DslField>::from_value(other).map_err(::dsl::__rt::field_error)?),
                    }
                },
            ),
            FieldKind::VecList(inner) => (
                quote! { ::dsl::Shape::List(Box::new(<#inner as ::dsl::DslField>::shape())) },
                quote! { ::dsl::FieldValue::List(self.#ident.iter().map(|v| ::dsl::DslField::to_value(v)).collect()) },
                quote! {
                    match value {
                        ::dsl::FieldValue::List(items) => items.iter().map(|v| <#inner as ::dsl::DslField>::from_value(v)).collect::<Result<Vec<_>, String>>().map_err(::dsl::__rt::field_error)?,
                        other => return Err(::dsl::__rt::field_error(format!("expected List, found {other:?}"))),
                    }
                },
            ),
            FieldKind::VecTuple(inner) => (
                quote! { ::dsl::Shape::Tuple(Box::new(<#inner as ::dsl::DslField>::shape()), None) },
                quote! { ::dsl::FieldValue::Tuple(self.#ident.iter().map(|v| ::dsl::DslField::to_value(v)).collect()) },
                quote! {
                    match value {
                        ::dsl::FieldValue::Tuple(items) => items.iter().map(|v| <#inner as ::dsl::DslField>::from_value(v)).collect::<Result<Vec<_>, String>>().map_err(::dsl::__rt::field_error)?,
                        other => return Err(::dsl::__rt::field_error(format!("expected Tuple, found {other:?}"))),
                    }
                },
            ),
            FieldKind::VecStatements(inner) => (
                quote! { ::dsl::Shape::Statements(<#inner as ::dsl::DslVariants>::variants()) },
                quote! { ::dsl::FieldValue::Statements(self.#ident.iter().map(|v| ::dsl::DslVariants::to_named_record(v)).collect()) },
                quote! {
                    match value {
                        ::dsl::FieldValue::Statements(items) => items
                            .iter()
                            .map(|(keyword, record)| <#inner as ::dsl::DslVariants>::from_named_record(keyword, record))
                            .collect::<Result<Vec<_>, ::dsl::TextError>>()?,
                        other => return Err(::dsl::__rt::field_error(format!("expected Statements, found {other:?}"))),
                    }
                },
            ),
        };

        spec_exprs.push(quote! {
            ::dsl::FieldSpec::new(#id, #key, #shape_expr) #pos_expr #opt_expr
        });
        to_value_stmts.push(quote! {
            record.fields.insert(#id, #to_value_expr);
        });
        from_value_stmts.push(quote! {
            let #ident = {
                let value = record.get(#id).ok_or_else(|| ::dsl::__rt::field_error(format!("missing field '{}'", #key)))?;
                #from_value_expr
            };
        });
    }

    (spec_exprs, to_value_stmts, from_value_stmts, field_idents)
}
//#endregion 🔖RecordCodegen

//#region 🔖DslRecord
#[proc_macro_derive(DslRecord, attributes(dsl))]
pub fn derive_dsl_record(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    let container = parse_container_attrs(&input);
    let Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(&input, "DslRecord only supports structs").to_compile_error().into();
    };
    let (spec_exprs, to_value_stmts, from_value_stmts, field_idents) = record_codegen(&data.fields);
    let keyword_expr = match &container.keyword {
        Some(k) => quote! { Some(#k.to_string()) },
        None => quote! { None },
    };
    let layout_expr = if container.lines_layout { quote! { ::dsl::RecordLayout::Lines } } else { quote! { ::dsl::RecordLayout::Inline } };

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
            pub fn __dsl_from_record(record: &::dsl::RecordValue) -> Result<Self, ::dsl::TextError> {
                #(#from_value_stmts)*
                Ok(Self { #(#field_idents),* })
            }
        }

        impl ::dsl::DslField for #name {
            fn shape() -> ::dsl::Shape {
                ::dsl::Shape::Record(Box::new(Self::__dsl_spec()))
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
    };
    expanded.into()
}
//#endregion 🔖DslRecord

//#region 🔖DslDocument
#[proc_macro_derive(DslDocument, attributes(dsl))]
pub fn derive_dsl_document(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    let container = parse_container_attrs(&input);
    let Some(extension) = &container.extension else {
        return syn::Error::new_spanned(&input, "DslDocument requires #[dsl(extension = \"...\")]").to_compile_error().into();
    };
    let Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(&input, "DslDocument only supports structs").to_compile_error().into();
    };
    let (spec_exprs, to_value_stmts, from_value_stmts, field_idents) = record_codegen(&data.fields);
    let keyword_expr = match &container.keyword {
        Some(k) => quote! { Some(#k.to_string()) },
        None => quote! { None },
    };
    let layout_expr = if container.lines_layout { quote! { ::dsl::RecordLayout::Lines } } else { quote! { ::dsl::RecordLayout::Inline } };

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
            pub fn __dsl_from_record(record: &::dsl::RecordValue) -> Result<Self, ::vcs::TextError> {
                #(#from_value_stmts)*
                Ok(Self { #(#field_idents),* })
            }
        }

        impl ::vcs::DocumentDsl for #name {
            const EXTENSION: &'static str = #extension;
            fn parse_dsl(text: &str) -> Result<Self, ::vcs::TextError> {
                let record = ::dsl::__rt::parse_document_record(text, &Self::__dsl_spec())?;
                Self::__dsl_from_record(&record)
            }
            fn print_dsl(&self) -> String {
                ::dsl::__rt::print_document_record(&self.__dsl_to_record(), &Self::__dsl_spec())
            }
        }
    };
    expanded.into()
}
//#endregion 🔖DslDocument

//#region 🔖DslScalar
#[proc_macro_derive(DslScalar, attributes(dsl))]
pub fn derive_dsl_scalar(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    let Data::Enum(data) = &input.data else {
        return syn::Error::new_spanned(&input, "DslScalar only supports unit-variant enums").to_compile_error().into();
    };
    let mut variant_tags = Vec::new();
    let mut match_to_ordinal = Vec::new();
    let mut match_from_ordinal = Vec::new();
    for (ordinal, variant) in data.variants.iter().enumerate() {
        if !matches!(variant.fields, Fields::Unit) {
            return syn::Error::new_spanned(variant, "DslScalar only supports unit variants").to_compile_error().into();
        }
        let attrs = parse_field_attrs(&variant.attrs);
        let variant_ident = variant.ident.clone();
        let tag = attrs.key.unwrap_or_else(|| variant_ident.to_string());
        let ordinal = ordinal as u32;
        variant_tags.push(quote! { (#tag.to_string(), #ordinal) });
        match_to_ordinal.push(quote! { #name::#variant_ident => #ordinal });
        match_from_ordinal.push(quote! { #ordinal => Ok(#name::#variant_ident) });
    }

    let expanded = quote! {
        impl ::dsl::DslField for #name {
            fn shape() -> ::dsl::Shape {
                ::dsl::Shape::Enum(vec![ #(#variant_tags),* ])
            }
            fn to_value(&self) -> ::dsl::FieldValue {
                ::dsl::FieldValue::Enum(match self { #(#match_to_ordinal),* })
            }
            fn from_value(value: &::dsl::FieldValue) -> Result<Self, String> {
                match value {
                    ::dsl::FieldValue::Enum(ordinal) => match *ordinal {
                        #(#match_from_ordinal,)*
                        other => Err(format!("unknown enum ordinal {other}")),
                    },
                    other => Err(format!("expected Enum, found {other:?}")),
                }
            }
        }
    };
    expanded.into()
}
//#endregion 🔖DslScalar

//#region 🔖DslOps
#[proc_macro_derive(DslOps, attributes(dsl))]
pub fn derive_dsl_ops(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    let Data::Enum(data) = &input.data else {
        return syn::Error::new_spanned(&input, "DslOps only supports enums").to_compile_error().into();
    };

    let mut variants_exprs = Vec::new();
    let mut to_named_arms = Vec::new();
    let mut from_named_arms = Vec::new();

    for variant in &data.variants {
        let attrs = parse_field_attrs(&variant.attrs);
        let variant_ident = variant.ident.clone();
        let keyword = attrs.key.clone().unwrap_or_else(|| to_kebab_or_camel(&variant_ident.to_string()));
        let fields = &variant.fields;
        let (spec_exprs, _to_value_stmts, from_value_stmts, field_idents) = record_codegen(fields);

        variants_exprs.push(quote! {
            (#keyword.to_string(), ::dsl::RecordSpec::new_owned(Some(#keyword.to_string()), ::dsl::RecordLayout::Inline, vec![ #(#spec_exprs),* ]))
        });

        // Build a per-variant to-record conversion using the field bindings from a `match` on
        // `self`, since (unlike `DslRecord`) the fields live inside an enum variant, not `self.field`.
        // A true unit variant (`Variant`, no braces at all) needs a bare match pattern — `Variant {}`
        // is only valid Rust for a variant that was itself declared with (empty) braces.
        let field_binds: Vec<proc_macro2::TokenStream> = field_idents.iter().map(|f| quote! { #f }).collect();
        let to_value_stmts_for_variant: Vec<proc_macro2::TokenStream> = record_codegen_to_value_from_bindings(fields);
        let is_unit = matches!(fields, Fields::Unit);
        let match_pattern = if is_unit { quote! { #name::#variant_ident } } else { quote! { #name::#variant_ident { #(#field_binds),* } } };
        let construct_expr = if is_unit { quote! { #name::#variant_ident } } else { quote! { #name::#variant_ident { #(#field_idents),* } } };
        to_named_arms.push(quote! {
            #match_pattern => {
                let mut record = ::dsl::RecordValue::default();
                #(#to_value_stmts_for_variant)*
                (#keyword.to_string(), record)
            }
        });
        from_named_arms.push(quote! {
            #keyword => {
                #(#from_value_stmts)*
                Ok(#construct_expr)
            }
        });
    }

    let expanded = quote! {
        impl ::dsl::DslVariants for #name {
            fn variants() -> Vec<(String, ::dsl::RecordSpec)> {
                vec![ #(#variants_exprs),* ]
            }
            fn to_named_record(&self) -> (String, ::dsl::RecordValue) {
                match self { #(#to_named_arms),* }
            }
            fn from_named_record(keyword: &str, record: &::dsl::RecordValue) -> Result<Self, ::dsl::TextError> {
                match keyword {
                    #(#from_named_arms,)*
                    other => Err(::dsl::__rt::field_error(format!("unknown operation keyword '{other}'"))),
                }
            }
        }

        impl ::vcs::OpText for #name {
            fn parse_op(line: &str) -> Result<Self, ::vcs::TextError> {
                let variants = <Self as ::dsl::DslVariants>::variants();
                for (keyword, spec) in &variants {
                    let probe = format!("{} ", keyword);
                    if line == keyword.as_str() || line.starts_with(&probe) {
                        let record = ::dsl::__rt::parse_inline_record(line, spec)?;
                        return <Self as ::dsl::DslVariants>::from_named_record(keyword, &record);
                    }
                }
                Err(::dsl::__rt::field_error(format!("unknown operation line '{line}'")))
            }
            fn print_op(&self) -> String {
                let (keyword, record) = <Self as ::dsl::DslVariants>::to_named_record(self);
                let variants = <Self as ::dsl::DslVariants>::variants();
                let spec = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| s.clone()).expect("variant spec must exist for its own keyword");
                ::dsl::__rt::print_inline_record(&record, &spec)
            }
        }
    };
    expanded.into()
}

/// @emoji 🔡 Falls back to the variant's own Rust identifier as the keyword when no
/// `#[dsl(key = "...")]` override is given (kept literal, no case conversion, so the printed
/// keyword matches exactly what a reader would expect from the enum's own naming).
fn to_kebab_or_camel(name: &str) -> String {
    name.to_string()
}

/// @emoji 🏗️ Like the `to_value` half of `record_codegen`, but reading from bare local bindings
/// (`ident`) instead of `self.ident` — what a `match self { Variant { fields... } => ... }` arm
/// needs, since enum variant fields aren't reached through `self.field` syntax.
fn record_codegen_to_value_from_bindings(fields: &Fields) -> Vec<proc_macro2::TokenStream> {
    let plans = plan_fields(fields);
    plans
        .iter()
        .map(|plan| {
            let FieldPlan { ident, id, kind, .. } = plan;
            let to_value_expr: proc_macro2::TokenStream = match kind {
                FieldKind::Scalar => quote! { ::dsl::DslField::to_value(#ident) },
                FieldKind::IdentString => quote! { ::dsl::FieldValue::Ident(#ident.clone()) },
                FieldKind::Bytes64 => quote! { ::dsl::FieldValue::Bytes64(#ident.clone()) },
                FieldKind::OptionScalar(_) => quote! {
                    match #ident {
                        Some(v) => ::dsl::DslField::to_value(v),
                        None => ::dsl::FieldValue::Absent,
                    }
                },
                FieldKind::VecList(_) => quote! { ::dsl::FieldValue::List(#ident.iter().map(|v| ::dsl::DslField::to_value(v)).collect()) },
                FieldKind::VecTuple(_) => quote! { ::dsl::FieldValue::Tuple(#ident.iter().map(|v| ::dsl::DslField::to_value(v)).collect()) },
                FieldKind::VecStatements(_) => quote! { ::dsl::FieldValue::Statements(#ident.iter().map(|v| ::dsl::DslVariants::to_named_record(v)).collect()) },
            };
            quote! { record.fields.insert(#id, #to_value_expr); }
        })
        .collect()
}
//#endregion 🔖DslOps
