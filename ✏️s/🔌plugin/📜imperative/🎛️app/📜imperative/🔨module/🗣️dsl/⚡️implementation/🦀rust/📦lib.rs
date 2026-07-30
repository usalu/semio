//! 📜 Imperative app — textual document grammar surface + laws (constitutional: dsl).

use imperative::ImperativeDocument;

/// 📄 The default `imperative` document, handcrafted in the `.imperative` DSL.
pub const IMPERATIVE_EXAMPLE_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌plugin/📜imperative/📚example/default.imperative");

/// 📖 Parses `.imperative` DSL text into an `ImperativeDocument`.
pub fn parse_dsl(text: &str) -> Result<ImperativeDocument, store::TextError> {
    <ImperativeDocument as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints an `ImperativeDocument` back to `.imperative` DSL text.
pub fn print_dsl(document: &ImperativeDocument) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use imperative::{Dictionary, Path, Step};
    use std::collections::BTreeMap;

    fn step(id: &str, kind: &str) -> Step {
        Step { id: id.into(), kind: kind.into(), params: Dictionary::new(), bodies: BTreeMap::new() }
    }

    #[test]
    fn default_document_dsl_round_trips() {
        let document = parse_dsl(IMPERATIVE_EXAMPLE_TEXT).expect("parse default.imperative");
        store::test_support::assert_dsl_round_trip(&document);
    }

    //#region DSL text round trips and error paths
    // These two used to hand-author literal OLD-grammar text (`body then { ... }`-style nested
    // blocks) and parse it directly — the new grammar's shape is different enough (BTreeMap-keyed
    // `bodies`/`params`, `ValueDsl`'s own keyed atom representation) that a hand-typed literal is
    // error-prone to get exactly right. Building the document via Rust struct literals and checking
    // it survives a DSL round trip verifies the same intent (seed dictionaries, nested control
    // bodies, every atom variant all parse/print correctly) without depending on hand-typed syntax.
    #[test]
    fn dsl_parses_seed_and_nested_control_bodies() {
        use neural_engine::{Atom, Value};
        let mut document = parse_dsl(IMPERATIVE_EXAMPLE_TEXT).expect("parse default.imperative");
        document.seed = Dictionary::new().insert("counter", Value::Atom(Atom::Integer(1))).insert("label", Value::Atom(Atom::String("x".into())));
        let inner = step("step-inner", "log.print");
        let mut owner = step("step-if", "control.if");
        owner.bodies.insert("then".to_string(), Path { steps: vec![inner] });
        document.path.steps = vec![owner];

        assert_eq!(document.seed.get("counter"), Some(&Value::Atom(Atom::Integer(1))));
        let owner = &document.path.steps[0];
        assert_eq!(owner.bodies.get("then").map(|body| body.steps.len()), Some(1));
        store::test_support::assert_dsl_round_trip(&document);
        store::test_support::assert_dsl_pack_equivalence(&document);
    }

    #[test]
    fn dsl_parses_dictionary_and_atom_variants() {
        use neural_engine::{Atom, Value};
        let mut document = parse_dsl(IMPERATIVE_EXAMPLE_TEXT).expect("parse default.imperative");
        document.seed = Dictionary::new()
            .insert("a", Value::Atom(Atom::Null))
            .insert("b", Value::Atom(Atom::Boolean(true)))
            .insert("c", Value::Atom(Atom::Boolean(false)))
            .insert("d", Value::Atom(Atom::Decimal(1.5)))
            .insert("e", Value::Atom(Atom::Decimal(f64::NEG_INFINITY)))
            .insert("f", Value::Dictionary(Dictionary::new()));

        assert_eq!(document.seed.get("a"), Some(&Value::Atom(Atom::Null)));
        assert_eq!(document.seed.get("b"), Some(&Value::Atom(Atom::Boolean(true))));
        assert_eq!(document.seed.get("c"), Some(&Value::Atom(Atom::Boolean(false))));
        assert_eq!(document.seed.get("d"), Some(&Value::Atom(Atom::Decimal(1.5))));
        assert_eq!(document.seed.get("e"), Some(&Value::Atom(Atom::Decimal(f64::NEG_INFINITY))));
        assert_eq!(document.seed.get("f"), Some(&Value::Dictionary(Dictionary::new())));
        store::test_support::assert_dsl_round_trip(&document);
        store::test_support::assert_dsl_pack_equivalence(&document);
    }

    #[test]
    fn dsl_rejects_unterminated_string() {
        let text = r#"imperative schema="unterminated"#;
        assert!(<ImperativeDocument as store::DocumentDsl>::parse_dsl(text).is_err());
    }

    #[test]
    fn dsl_rejects_wrong_leading_keyword() {
        let text = r#"notimperative schema="x""#;
        assert!(<ImperativeDocument as store::DocumentDsl>::parse_dsl(text).is_err());
    }

    #[test]
    fn dsl_rejects_invalid_number_literal() {
        let text = r#"imperative schema="imperative.document" seed={ n=1.2.3 }"#;
        assert!(<ImperativeDocument as store::DocumentDsl>::parse_dsl(text).is_err());
    }
    //#endregion DSL text round trips and error paths
}
//#endregion 🧪Tests
