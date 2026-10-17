use super::*;
#[test]
fn declared_factory_namespace_qualifies_actual_owner_with_independent_syn_oracle(){
 let corpus:serde_json::Value=serde_json::from_str(runtime_fixture_text("../../🏭️factory/🧪️tests/🧫️fixtures/🧭️namespace.json")).unwrap();
 for row in corpus["cases"].as_array().unwrap(){let namespace=syn::LitStr::new(row["namespace"].as_str().unwrap(),proc_macro2::Span::call_site());let input:syn::DeriveInput=syn::parse2(quote!(#[value(crate=#namespace)] struct Owner{tag:u64})).unwrap();let output=expand(&input).unwrap();let syntax:syn::File=syn::parse2(output).unwrap();let syn::Item::Impl(item)=&syntax.items[0] else{panic!("actual factory implementation")};let path:Vec<_>=item.trait_.as_ref().unwrap().1.segments.iter().map(|segment|segment.ident.to_string()).collect();assert_eq!(serde_json::to_value(path).unwrap(),row["traitPath"]);}
}

#[test]
fn declared_retirement_namespace_qualifies_actual_owner_with_independent_syn_oracle(){
 let corpus:serde_json::Value=serde_json::from_str(runtime_fixture_text("../../🏭️factory/🧪️tests/🧫️fixtures/🧭️namespace.json")).unwrap();
 for row in corpus["cases"].as_array().unwrap(){let namespace=syn::LitStr::new(row["namespace"].as_str().unwrap(),proc_macro2::Span::call_site());let input:syn::DeriveInput=syn::parse2(quote!(#[value(crate=#namespace)] struct Owner{tag:u64})).unwrap();let output=crate::retained_clone::expand_retire_owned(&input).unwrap();let syntax:syn::File=syn::parse2(output).unwrap();let syn::Item::Impl(item)=&syntax.items[0] else{panic!("actual retirement implementation")};let path:Vec<_>=item.trait_.as_ref().unwrap().1.segments.iter().map(|segment|segment.ident.to_string()).collect();assert_eq!(serde_json::to_value(path).unwrap(),row["retirementTraitPath"]);}
}

#[test]
fn declared_clone_namespace_qualifies_actual_owner_with_independent_syn_oracle(){
 let corpus:serde_json::Value=serde_json::from_str(runtime_fixture_text("../../🏭️factory/🧪️tests/🧫️fixtures/🧭️namespace.json")).unwrap();
 for row in corpus["cases"].as_array().unwrap(){let namespace=syn::LitStr::new(row["namespace"].as_str().unwrap(),proc_macro2::Span::call_site());let input:syn::DeriveInput=syn::parse2(quote!(#[value(crate=#namespace)] struct Owner{tag:u64})).unwrap();let output=crate::retained_clone::expand_retained_clone(&input).unwrap();let syntax:syn::File=syn::parse2(output).unwrap();let item=syntax.items.iter().find_map(|item|match item{syn::Item::Impl(item)if item.trait_.as_ref().is_some_and(|(_,path,_)|path.segments.last().is_some_and(|segment|segment.ident=="RetainedClone"))=>Some(item),_=>None}).unwrap();let path:Vec<_>=item.trait_.as_ref().unwrap().1.segments.iter().map(|segment|segment.ident.to_string()).collect();assert_eq!(serde_json::to_value(path).unwrap(),row["cloneTraitPath"]);}
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
