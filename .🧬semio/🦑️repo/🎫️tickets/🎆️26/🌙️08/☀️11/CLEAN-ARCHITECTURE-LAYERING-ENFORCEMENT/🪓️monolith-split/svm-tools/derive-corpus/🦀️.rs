//! 🔬️ Differential corpus (svm): runs the pristine pre-split expansion (`before`) and the split expansion (`crate::component`) over every derive input found in the corpus
//! files and requires byte-identical token streams. Temporary verification asset of the SCHEMA-VALUE-MATH-SPLITS slice; mount it only for the compile queue run.

use proc_macro2::TokenStream;
use syn::{DeriveInput, Item};

#[path = "before/🦀️.rs"]
mod before;

fn repo_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../..").canonicalize().unwrap()
}

fn derives(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attribute| attribute.path().is_ident("derive") && {
        let tokens = attribute.meta.require_list().map(|list| list.tokens.to_string()).unwrap_or_default();
        tokens.contains("ToValue") || tokens.contains("FromValue") || tokens.contains("CanonicalJsonTree")
    })
}

fn collect(items: &[Item], out: &mut Vec<DeriveInput>) {
    for item in items {
        match item {
            Item::Struct(item) if derives(&item.attrs) => out.push(DeriveInput::from(item.clone())),
            Item::Enum(item) if derives(&item.attrs) => out.push(DeriveInput::from(item.clone())),
            Item::Mod(item) => {
                if let Some((_, nested)) = &item.content {
                    collect(nested, out);
                }
            }
            _ => {}
        }
    }
}

fn render(result: syn::Result<TokenStream>) -> String {
    match result {
        Ok(tokens) => tokens.to_string(),
        Err(error) => format!("ERR {}", error.to_compile_error()),
    }
}

#[test]
fn svm_split_expansion_token_streams_equal_the_pristine_expansion() {
    let root = repo_root();
    let list = std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/svm-tools/derive-corpus/corpus-files.txt")).unwrap();
    let mut before_dump = String::new();
    let mut after_dump = String::new();
    let (mut inputs, mut compared) = (0usize, 0usize);
    for path in list.lines().filter(|line| !line.trim().is_empty()) {
        let Ok(source) = std::fs::read_to_string(root.join(path)) else { continue };
        let Ok(file) = syn::parse_file(&source) else { continue };
        let mut found = Vec::new();
        collect(&file.items, &mut found);
        for input in found {
            inputs += 1;
            let pairs = [
                ("to_value", render(before::expand_to_value(&input)), render(crate::component::expand_to_value(&input))),
                ("from_value", render(before::expand_from_value(&input)), render(crate::component::expand_from_value(&input))),
                ("canonical_tree", render(before::expand_canonical_tree(&input)), render(crate::component::expand_canonical_tree(&input))),
                ("value_crate", render(before::declared_value_crate(&input).map(|path| quote::quote!(#path))), render(crate::component::declared_value_crate(&input).map(|path| quote::quote!(#path)))),
            ];
            for (kind, before_tokens, after_tokens) in pairs {
                before_dump.push_str(&format!("{path}::{}::{kind}\n{before_tokens}\n", input.ident));
                after_dump.push_str(&format!("{path}::{}::{kind}\n{after_tokens}\n", input.ident));
                assert_eq!(before_tokens, after_tokens, "{path}::{}::{kind}", input.ident);
                compared += 1;
            }
        }
    }
    if let Some(prefix) = std::env::var_os("SVM_CORPUS_OUT") {
        let prefix = std::path::PathBuf::from(prefix);
        std::fs::write(prefix.with_extension("before.txt"), &before_dump).unwrap();
        std::fs::write(prefix.with_extension("after.txt"), &after_dump).unwrap();
    }
    assert!(inputs > 100, "corpus too small: {inputs}");
    println!("[DEBUG] svm corpus inputs={inputs} compared token streams={compared}");
}
