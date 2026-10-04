use std::collections::BTreeSet;

fn identifiers(tokens: proc_macro2::TokenStream, output: &mut BTreeSet<String>) {
    for token in tokens {
        match token {
            proc_macro2::TokenTree::Ident(ident) => { output.insert(ident.to_string()); }
            proc_macro2::TokenTree::Group(group) => identifiers(group.stream(), output),
            _ => {}
        }
    }
}

#[test]
fn product_expanders_own_envelopes_and_diff_transport_only() {
    use quote::ToTokens;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪆️record-owner/🔣️.json")).unwrap();
    let source = syn::parse_file(include_str!("../../🦀️.rs")).unwrap();
    for contract in fixture["functions"].as_array().unwrap() {
        let functions: Vec<_> = source.items.iter().filter_map(|item| match item { syn::Item::Fn(function) if function.sig.ident == contract["name"].as_str().unwrap() => Some(function), _ => None }).collect();
        assert_eq!(functions.len(), 1);
        let mut names = BTreeSet::new();
        identifiers(functions[0].block.to_token_stream(), &mut names);
        for name in contract["required"].as_array().unwrap() { assert!(names.contains(name.as_str().unwrap()), "missing {name}"); }
        for name in contract["forbidden"].as_array().unwrap() { assert!(!names.contains(name.as_str().unwrap()), "duplicate generic identity {name}"); }
    }
}

#[test]
fn product_macro_owner_has_no_generic_record_generator() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪆️record-owner/🔣️.json")).unwrap();
    let source = syn::parse_file(include_str!("../../🦀️.rs")).unwrap();
    let functions: BTreeSet<_> = source.items.iter().filter_map(|item| match item { syn::Item::Fn(function) => Some(function.sig.ident.to_string()), _ => None }).collect();
    for name in fixture["retired"].as_array().unwrap() { assert!(!functions.contains(name.as_str().unwrap()), "retired {name}"); }
    assert!(functions.contains("expand_derive_composite_mutation"));
    assert!(functions.contains("expand_mutation_leaf"));
    assert!(functions.contains("expand_derive_mutations"));
}
