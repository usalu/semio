//! 🛡️ Parsed Rust syntax enforces renderer-neutral artifact IO authority.
use proc_macro2::{TokenStream, TokenTree};
use serde::Deserialize;
use std::path::Path;
use syn::visit::{self, Visit};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Fixture {
    forbidden_segments: Vec<String>,
    forbidden_prefixes: Vec<String>,
    cases: Vec<Case>,
}

#[derive(Deserialize)]
struct Case {
    id: String,
    source: String,
    allowed: bool,
}

struct Authority<'a> {
    fixture: &'a Fixture,
    forbidden: Vec<String>,
}

impl Authority<'_> {
    fn inspect(&mut self, identifier: &str) {
        if self.fixture.forbidden_segments.iter().any(|value| value == identifier)
            || self.fixture.forbidden_prefixes.iter().any(|value| identifier.starts_with(value)) {
            self.forbidden.push(identifier.into());
        }
    }

    fn tokens(&mut self, tokens: TokenStream) {
        for token in tokens {
            match token {
                TokenTree::Ident(identifier) => self.inspect(&identifier.to_string()),
                TokenTree::Group(group) => self.tokens(group.stream()),
                _ => {},
            }
        }
    }

    fn code_path(&mut self, path: &str) {
        for segment in path.split(['/', '\\']) {
            if ["✏️editor", "👁️viewer", "📺️renderer"].contains(&segment) {
                self.forbidden.push(path.into());
            }
        }
    }
}

impl<'ast> Visit<'ast> for Authority<'_> {
    fn visit_expr_method_call(&mut self, call: &'ast syn::ExprMethodCall) {
        self.inspect(&call.method.to_string());
        visit::visit_expr_method_call(self, call);
    }

    fn visit_path(&mut self, path: &'ast syn::Path) {
        for segment in &path.segments { self.inspect(&segment.ident.to_string()); }
        visit::visit_path(self, path);
    }

    fn visit_use_tree(&mut self, tree: &'ast syn::UseTree) {
        match tree {
            syn::UseTree::Path(path) => self.inspect(&path.ident.to_string()),
            syn::UseTree::Name(name) => self.inspect(&name.ident.to_string()),
            syn::UseTree::Rename(rename) => self.inspect(&rename.ident.to_string()),
            _ => {},
        }
        visit::visit_use_tree(self, tree);
    }

    fn visit_macro(&mut self, declaration: &'ast syn::Macro) {
        self.tokens(declaration.tokens.clone());
        if declaration.path.is_ident("include") {
            if let Ok(path) = syn::parse2::<syn::LitStr>(declaration.tokens.clone()) { self.code_path(&path.value()); }
        }
        visit::visit_macro(self, declaration);
    }

    fn visit_attribute(&mut self, attribute: &'ast syn::Attribute) {
        if attribute.path().is_ident("path") {
            if let syn::Meta::NameValue(value) = &attribute.meta {
                if let syn::Expr::Lit(expression) = &value.value {
                    if let syn::Lit::Str(path) = &expression.lit { self.code_path(&path.value()); }
                }
            }
        }
        visit::visit_attribute(self, attribute);
    }
}

fn violations(source: &str, fixture: &Fixture) -> Vec<String> {
    let syntax = syn::parse_file(source).expect("valid Rust source");
    let mut authority = Authority { fixture, forbidden: Vec::new() };
    authority.visit_file(&syntax);
    authority.forbidden
}

fn inspect_directory(directory: &Path, fixture: &Fixture) {
    for entry in std::fs::read_dir(directory).expect("IO source directory") {
        let path = entry.expect("IO source entry").path();
        if path.is_dir() {
            if !["🧪️tests", "🧫️fixtures"].contains(&path.file_name().unwrap().to_str().unwrap()) { inspect_directory(&path, fixture); }
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            let source = std::fs::read_to_string(&path).expect("IO Rust source");
            assert!(violations(&source, fixture).is_empty(), "{}: {:?}", path.display(), violations(&source, fixture));
        }
    }
}

#[test]
fn generation3d_io_authority_is_renderer_neutral_in_parsed_rust() {
    let fixture: Fixture = serde_json::from_str(include_str!("../../../🧫️fixtures/🛡️authority/🔣️.json")).expect("neutral authority fixture");
    for case in &fixture.cases {
        assert_eq!(violations(&case.source, &fixture).is_empty(), case.allowed, "{}", case.id);
    }
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io");
    inspect_directory(&directory, &fixture);
}
