//! 📚️ The example-seat predicate and the example-refusal vocabulary, driven by the language-agnostic
//! `🧫️fixtures/📚️example-refusal.json` the TypeScript twin (`🟦️.ts` beside this file) reads too.

use super::*;
use std::collections::BTreeMap;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ExampleRefusalFixture {
    messages: BTreeMap<String, FixtureMessage>,
    cases: Vec<FixtureCase>,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct FixtureMessage {
    en: String,
    de: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct FixtureCase {
    name: String,
    example_id: String,
    declared: Vec<String>,
    decode: String,
    expected: FixtureExpected,
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum FixtureExpected {
    Seat { seat: String },
    Refusal { refusal: String },
}

fn fixture() -> ExampleRefusalFixture {
    serde_json::from_str(include_str!("../../🧫️fixtures/📚️example-refusal.json")).expect("the example-refusal fixture must parse")
}

/// 🎯️ Every example switch of the fixture seats or refuses exactly as it says — the empty id is the genesis document,
/// an undeclared id is refused before anything decodes, and a declared example refuses when it does not decode or
/// decodes to the genesis document.
#[test]
fn every_example_switch_seats_or_refuses_as_the_fixture_says() {
    for case in fixture().cases {
        let examples: Vec<(&str, &str)> = case.declared.iter().map(|id| (id.as_str(), "authored body")).collect();
        let decoded: Result<String, &str> = match case.decode.as_str() {
            "document" => Ok("document".to_string()),
            "genesis" => Ok(String::new()),
            "undecodable" => Err("line 1: unexpected token"),
            other => panic!("{}: unknown decode outcome {other}", case.name),
        };
        let outcome = example_seat(&case.example_id, &examples).and_then(|seat| match seat {
            None => Ok("genesis"),
            Some(_) => example_decoded(&case.example_id, decoded).map(|_| "example"),
        });
        match (&case.expected, outcome) {
            (FixtureExpected::Seat { seat }, Ok(actual)) => assert_eq!(actual, seat, "{}", case.name),
            (FixtureExpected::Refusal { refusal }, Err(actual)) => {
                assert_eq!(actual.kind.code(), refusal, "{}", case.name);
                assert_eq!(actual.example_id, case.example_id, "{}", case.name);
            }
            (_, other) => panic!("{}: resolved to {other:?}", case.name),
        }
    }
}

/// 🗣️ Every refusal kind carries the fixture's frozen code and its English and German notice, and crosses the
/// boundary as a non-retryable warning fault whose one cause is the decoder's detail.
#[test]
fn every_refusal_kind_carries_its_code_and_both_notices() {
    let fixture = fixture();
    assert_eq!(fixture.messages.len(), ExampleRefusalKind::ALL.len());
    for kind in ExampleRefusalKind::ALL {
        let message = &fixture.messages[kind.code()];
        assert_eq!(kind.label().resolve(Terminology::Native, Locale::En), message.en);
        assert_eq!(kind.label().resolve(Terminology::Native, Locale::De), message.de);
        assert_eq!(ExampleRefusalKind::from_code(kind.code()), Some(kind));
        let fault: crate::Fault = ExampleRefusal { kind, example_id: "demo".to_string(), detail: "line 1".to_string() }.into();
        assert_eq!(fault.code.0, kind.code());
        assert_eq!(fault.severity, crate::Severity::Warning);
        assert!(!fault.retryable);
        assert_eq!(fault.causes.len(), 1);
        assert_eq!(fault.causes[0].message, "line 1");
    }
    assert_eq!(ExampleRefusalKind::from_code("mutation.rejected"), None);
    let unknown: crate::Fault = ExampleRefusal::unknown("gone").into();
    assert!(unknown.causes.is_empty());
    assert!(unknown.message.ends_with("(gone)"));
}
