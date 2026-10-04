# Actual Higher JSON Word Syntax Producers

The four actual Jack, Program, Puzzle5d and Puzzle3d conversion modules perform literal object/member/array and closed scalar word syntax admission. Their complete immediate source is admitted: no Native control, checkpoint, quota, JSON parser, IO or recursive foreign error occurs. Caller TextError constructors require explicit InvalidValue for these own syntax refusals. This does not classify an arbitrary native transport String. All original field rules and messages remain. Native consumer proofs are pending.

Complete immediate source text, inverse, authored full text, byte counts, hashes and exact per-site decisions are retained in generated/value-refusal/text-error-higher-json-words-authored-1.json. Native admission is pending.

## ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs

```rust
//! jack -> json
use crate::JackSnapshot;
use semio_s_artifact_stdio_json::schema::snapshot::write_json_pretty;
use semio_s_artifact_stdio_json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

/// 🌉 Bridges via json's own RFC8259 text codec (`JsonSnapshot::value` is `JsonValue`, json's
/// own key-order/lexeme-preserving model, not `pack::JsonValue` -- see json's snapshot module).
pub fn register() {}

pub fn serialize(snapshot: &JackSnapshot) -> Result<JsonSnapshot, semio_framework_diagnostic::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let raw=crate::standards::v1::subsets::any::io::json_native::convert(dsl::ToValue::to_value(snapshot),false).map_err(|e|semio_framework_diagnostic::TextError::new(e,semio_framework_diagnostic::TextSpan::at(1,1)))?;
    let value = semio_framework_pack_json::from_dsl_value(&raw);
    Ok(JsonSnapshot::from_value(value))
}

pub fn serialize_bytes(snapshot: &JackSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}

```

## ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs

```rust
//! jack <- json
use crate::JackSnapshot;
use semio_s_artifact_stdio_json::schema::snapshot::parse_json_text;
use semio_s_artifact_stdio_json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<JackSnapshot, semio_framework_diagnostic::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let raw=crate::standards::v1::subsets::any::io::json_native::convert(semio_framework_pack_json::to_dsl_value(&from.to_pack_value()),true).map_err(|e|semio_framework_diagnostic::TextError::new(e,semio_framework_diagnostic::TextSpan::at(1,1)))?;
    let out: JackSnapshot = dsl::FromValue::from_value(raw).map_err(|e: dsl::ValueError| semio_framework_diagnostic::TextError::new(e.kind, format!("jack<-json: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    Ok(out)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<JackSnapshot, semio_framework_diagnostic::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text)?;
    deserialize(&JsonSnapshot::from_value(value))
}

```

## ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs

```rust
//! puzzle3d -> json
//!
//! 🩹️ `stdio_gap`/foreign-lag fix — see the paired import leaf's doc comment (same wave,
//! `JsonSnapshot.value: serde_json::Value` -> stdio's own `JsonValue`). Routes through
//! `JsonSnapshot::from_value` (stdio's own real reverse `serde_json::Value -> JsonValue` bridge,
//! no hand-rolled converter here) and stdio's own real `write_json_pretty` for `serialize_bytes`
//! (the previous `serde_json::to_vec_pretty(&value)` would have serialized the internally-tagged
//! `JsonValue` shape verbatim, not real JSON text — a latent bug this fix also corrects).
//!
//! 🩹️ Ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`: no longer
//! routes through `serde_json::to_value` — `Puzzle3dSnapshot` only derives `Serialize` under
//! `#[cfg(test)]` now. `dsl::ToValue::to_value` (first-party) -> `semio_framework_pack_json::from_dsl_value`
//! (`DslValue` -> stdio's own `JsonValue`) instead, same shape the sibling `block3d` leaf already
//! uses.
use crate::Puzzle3dSnapshot;
use semio_s_artifact_stdio_json::schema::snapshot::write_json_pretty;
use semio_s_artifact_stdio_json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &Puzzle3dSnapshot) -> Result<JsonSnapshot, semio_framework_diagnostic::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let raw = crate::standards::v1::subsets::any::io::json_native::convert(dsl::ToValue::to_value(snapshot),false).map_err(|e| semio_framework_diagnostic::TextError::new(e,semio_framework_diagnostic::TextSpan::at(1,1)))?;
    Ok(JsonSnapshot::from_value(semio_framework_pack_json::from_dsl_value(&raw)))
}

pub fn serialize_bytes(snapshot: &Puzzle3dSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}

```

## ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs

```rust
//! puzzle3d <- json
//!
//! 🩹️ `stdio_gap`/foreign-lag fix (not part of this wave's svg/dwg-pattern scope — see
//! `w5b--puzzle-report.md`): `JsonSnapshot.value` was retyped from `serde_json::Value` to stdio's
//! own lexeme-preserving `JsonValue` (own type, `#[serde(tag = "kind")]` — an intentional boundary
//! per that schema's own doc comment, NOT structurally plain JSON) by a concurrent stdio wave,
//! breaking this pre-existing leaf's compile. Fixed as a minimal lagging-call-site update: routes
//! through `JsonSnapshot::to_serde_value` (stdio's own real `JsonValue -> serde_json::Value`
//! bridge) plus stdio's own real `parse_json_text` — no hand-rolled converter here.
//!
//! 🩹️ Ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`: no longer
//! routes through `serde_json::from_value` — `Puzzle3dSnapshot` only derives `Deserialize` under
//! `#[cfg(test)]` now. The number lexemes cross into `dsl::DslValue` through stdio's own
//! first-party `to_pack_value()` bridge, then `dsl::FromValue::from_value` hydrates the typed
//! snapshot — same shape the sibling `block3d` leaf already uses. NOT through
//! `to_serde_value()`: `serde_json`'s default (no `float_roundtrip`) number parser reconstructs an
//! f64 as `significand as f64 * 10^exponent`, which is off by one ULP for any 17-significant-digit
//! literal (`0.42839899821678995` came back as `0.4283989982167899`) and made every example whose
//! geometry needs full f64 precision fail its lossless-round-trip oracle.
use crate::Puzzle3dSnapshot;
use semio_s_artifact_stdio_json::schema::snapshot::parse_json_text;
use semio_s_artifact_stdio_json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<Puzzle3dSnapshot, semio_framework_diagnostic::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let raw: dsl::DslValue = semio_framework_pack_json::to_dsl_value(&from.to_pack_value());
    let raw = crate::standards::v1::subsets::any::io::json_native::convert(raw,true).map_err(|e| semio_framework_diagnostic::TextError::new(e, semio_framework_diagnostic::TextSpan::at(1,1)))?;
    let snap: Puzzle3dSnapshot = dsl::FromValue::from_value(raw).map_err(|e| semio_framework_diagnostic::TextError::new(e.kind, format!("puzzle3d<-json: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    Ok(snap)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Puzzle3dSnapshot, semio_framework_diagnostic::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text)?;
    deserialize(&JsonSnapshot::from_value(value))
}

```

## ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs

```rust
//! puzzle5d -> json
//!
//! 🩹️ `stdio_gap`/foreign-lag fix — see the paired import leaf's doc comment (same wave,
//! `JsonSnapshot.value: serde_json::Value` -> stdio's own `JsonValue`). Routes through
//! `JsonSnapshot::from_value` (stdio's own real reverse `serde_json::Value -> JsonValue` bridge,
//! no hand-rolled converter here) and stdio's own real `write_json_pretty` for `serialize_bytes`
//! (the previous `serde_json::to_vec_pretty(&value)` would have serialized the internally-tagged
//! `JsonValue` shape verbatim, not real JSON text — a latent bug this fix also corrects).
//!
//! 🩹️ Ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`: no longer
//! routes through `serde_json::to_value` — `Puzzle5dSnapshot` only derives `Serialize` under
//! `#[cfg(test)]` now. `dsl::ToValue::to_value` (first-party) -> `semio_framework_pack_json::from_dsl_value`
//! (`DslValue` -> stdio's own `JsonValue`) instead, same shape the sibling `block5d` leaf already
//! uses.
use crate::Puzzle5dSnapshot;
use semio_s_artifact_stdio_json::schema::snapshot::write_json_pretty;
use semio_s_artifact_stdio_json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &Puzzle5dSnapshot) -> Result<JsonSnapshot, semio_framework_diagnostic::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let raw=crate::standards::v1::subsets::any::io::puzzle5d_json::convert(dsl::ToValue::to_value(snapshot),false).map_err(|message|semio_framework_diagnostic::TextError::new(message,semio_framework_diagnostic::TextSpan::at(1,1)))?;
    Ok(JsonSnapshot::from_value(semio_framework_pack_json::from_dsl_value(&raw)))
}

pub fn serialize_bytes(snapshot: &Puzzle5dSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}

```

## ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs

```rust
//! puzzle5d <- json
//!
//! 🩹️ `stdio_gap`/foreign-lag fix (not part of this wave's svg/dwg-pattern scope — see
//! `w5b--puzzle-report.md`): `JsonSnapshot.value` was retyped from `serde_json::Value` to stdio's
//! own lexeme-preserving `JsonValue` (own type, `#[serde(tag = "kind")]` — an intentional boundary
//! per that schema's own doc comment, NOT structurally plain JSON) by a concurrent stdio wave,
//! breaking this pre-existing leaf's compile. Fixed as a minimal lagging-call-site update: routes
//! through `JsonSnapshot::to_serde_value` (stdio's own real `JsonValue -> serde_json::Value`
//! bridge) plus stdio's own real `parse_json_text` — no hand-rolled converter here.
//!
//! 🩹️ Ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`: no longer
//! routes through `serde_json::from_value` — `Puzzle5dSnapshot` only derives `Deserialize` under
//! `#[cfg(test)]` now. The number lexemes cross into `dsl::DslValue` through stdio's own
//! first-party `to_pack_value()` bridge, then `dsl::FromValue::from_value` hydrates the typed
//! snapshot — same shape the sibling `block5d` leaf already uses. NOT through
//! `to_serde_value()`: `serde_json`'s default (no `float_roundtrip`) number parser reconstructs an
//! f64 as `significand as f64 * 10^exponent`, which is off by one ULP for any 17-significant-digit
//! literal (`0.42839899821678995` came back as `0.4283989982167899`) and made every example whose
//! geometry needs full f64 precision fail its lossless-round-trip oracle.
use crate::Puzzle5dSnapshot;
use semio_s_artifact_stdio_json::schema::snapshot::parse_json_text;
use semio_s_artifact_stdio_json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<Puzzle5dSnapshot, semio_framework_diagnostic::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let raw=crate::standards::v1::subsets::any::io::puzzle5d_json::convert(semio_framework_pack_json::to_dsl_value(&from.to_pack_value()),true).map_err(|message|semio_framework_diagnostic::TextError::new(message,semio_framework_diagnostic::TextSpan::at(1,1)))?;
    let snap: Puzzle5dSnapshot = dsl::FromValue::from_value(raw).map_err(|e| semio_framework_diagnostic::TextError::new(e.kind, format!("puzzle5d<-json: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    Ok(snap)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Puzzle5dSnapshot, semio_framework_diagnostic::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text)?;
    deserialize(&JsonSnapshot::from_value(value))
}

/// 🧪️ Exact native word identity crosses the actual declared JSON serializer and importer.
#[cfg(test)]
#[test]
fn sqlite_snapshot_puzzle5d_declared_json_exact_words() {
    for raw in [0_u64, 0x8000000000000000, 0x3ff0000000000000, 0x7ff0000000000000, 0xfff0000000000000, 0x7ff0000000000001, 0x7ff8000000000042, 0xfff0000000000042] {
        let mut snapshot = Puzzle5dSnapshot::default();
        let mut part = crate::Puzzle5dPart::default();
        part.id = "literal".into();
        part.part_2d.x = f64::from_bits(raw);
        snapshot.parts.push(part);
        let bytes = crate::standards::v1::subsets::any::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).expect("declared JSON export");
        let independent: serde_json::Value = serde_json::from_slice(&bytes).expect("independent JSON oracle");
        assert_eq!(independent["parts"][0]["2d"]["x"], serde_json::json!({"bits": format!("{raw:016x}")}));
        assert_eq!(deserialize_bytes(&bytes).expect("declared JSON import").parts[0].part_2d.x.to_bits(), raw);
    }
}

```

## ✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs

```rust
//! program -> json
//!
//! 🩹️ `stdio_gap`/foreign-lag fix (not part of this wave's csv/tsv scope): `JsonSnapshot.value`
//! was retyped from `serde_json::Value` to stdio's own lexeme-preserving `JsonValue`
//! (`#[serde(tag = "kind")]`, NOT structurally plain JSON by design) by a concurrent stdio wave,
//! breaking this pre-existing placeholder leaf's compile. Fixed as a minimal lagging-call-site
//! update, mirroring the same pattern animate/fem used for the identical gap: a real, honest
//! structural `serde_json::Value -> JsonValue` converter (stdio provides no such bridge) plus
//! stdio's own real `write_json_pretty` text codec for `serialize_bytes`.
use crate::ProgramSnapshot;
use semio_s_artifact_stdio_json::schema::snapshot::write_json_pretty;
use semio_s_artifact_stdio_json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &ProgramSnapshot) -> Result<JsonSnapshot, semio_framework_diagnostic::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let projected=crate::standards::v1::subsets::any::io::program_json::convert(dsl::ToValue::to_value(snapshot),false).map_err(|message|semio_framework_diagnostic::TextError::new(message,semio_framework_diagnostic::TextSpan::at(1,1)))?;
    let value=dsl::json::from_dsl_value(&projected);
    Ok(JsonSnapshot::from_value(value))
}

pub fn serialize_bytes(snapshot: &ProgramSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}

```

## ✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs

```rust
//! program <- json
//!
//! 🩹️ `stdio_gap`/foreign-lag fix — see the paired export leaf's doc comment (same wave,
//! `JsonSnapshot.value: serde_json::Value` -> stdio's own `JsonValue`). Mirrors it with the
//! reverse structural converter and stdio's own real `parse_json_text` for `deserialize_bytes`.
use crate::ProgramSnapshot;
use crate::ARCHITECT_PROGRAM_SCHEMA;
use semio_s_artifact_stdio_json::schema::snapshot::parse_json_text;
use semio_s_artifact_stdio_json::JsonSnapshot;

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<ProgramSnapshot, semio_framework_diagnostic::TextError> {
    let _ = ARCHITECT_PROGRAM_SCHEMA;
    let mut out: ProgramSnapshot = dsl::FromValue::from_value(crate::standards::v1::subsets::any::io::program_json::convert(dsl::json::to_dsl_value(&from.to_pack_value()),true).map_err(|message|semio_framework_diagnostic::TextError::new(message,semio_framework_diagnostic::TextSpan::at(1,1)))?).map_err(|e: dsl::ValueError| semio_framework_diagnostic::TextError::new(e.kind, format!("program<-json: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    if out.schema.is_empty() {
        out.schema = ARCHITECT_PROGRAM_SCHEMA.into();
    }
    Ok(out)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<ProgramSnapshot, semio_framework_diagnostic::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text)?;
    deserialize(&JsonSnapshot::from_value(value))
}

#[cfg(test)]
#[test]
fn sqlite_snapshot_program_declared_json_preserves_every_owned_ieee_word() {
    let base=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any");
    let laws:serde_json::Value=serde_json::from_slice(&std::fs::read(base.join("🧬️schema/📸️snapshot/🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap()).unwrap();
    let fixture=base.join(laws["unsigned32Fixtures"]["BenchmarkRecord"][0].as_str().unwrap());
    let mut expected:ProgramSnapshot=store::json::from_json_str(&std::fs::read_to_string(fixture).unwrap()).unwrap();
    for raw in laws["binary64Words"].as_array().unwrap() {
        let raw=raw.as_str().unwrap();let bits=u64::from_str_radix(raw,16).unwrap();expected.benchmarks_payload[0].value=f64::from_bits(bits);
        let wire=crate::io::export::serializers::artifacts::json::v_rfc8259::any::serialize(&expected).unwrap();
        let text=semio_s_artifact_stdio_json::schema::snapshot::write_json_pretty(&wire.value);
        let projected:serde_json::Value=serde_json::from_str(&text).unwrap();
        assert_eq!(projected["benchmarksPayload"][0]["value"],serde_json::json!({"bits":raw}));
        let restored=deserialize(&wire).unwrap();assert_eq!(restored.benchmarks_payload[0].value.to_bits(),bits);
    }
}

#[cfg(test)]
#[test]
fn sqlite_snapshot_program_declared_json_preserves_full_unsigned64_transport(){
    let base=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any");
    let corpus:serde_json::Value=serde_json::from_slice(&std::fs::read(base.join("🧬️schema/📸️snapshot/🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap()).unwrap();
    for row in corpus["unsigned64Fields"].as_array().unwrap(){
        let entity=row[0].as_str().unwrap();let field=row[1].as_str().unwrap();let fixture=&corpus["unsigned64Fixtures"][entity];let slot=fixture[1].as_str().unwrap();let source=std::fs::read_to_string(base.join(fixture[0].as_str().unwrap())).unwrap();
        for raw in corpus["unsigned64Corpus"]["valid"].as_array().unwrap(){
            let raw=raw.as_str().unwrap();let word=raw.parse::<u64>().unwrap();let mut expected:ProgramSnapshot=store::json::from_json_str(&source).unwrap();
            match entity{"AnalysisRecord"=>expected.analyses[0].duration_ms=Some(word),"SearchFilter"=>expected.search_filters[0].use_count=word,"TemplateRecord"=>expected.templates[0].usage_count=word,"KnowledgeRecord"=>expected.knowledge_payload[0].usage_count=word,_=>panic!("unknown authored unsigned field")}
            let wire=crate::io::export::serializers::artifacts::json::v_rfc8259::any::serialize(&expected).unwrap();let text=semio_s_artifact_stdio_json::schema::snapshot::write_json_pretty(&wire.value);let projected:serde_json::Value=serde_json::from_str(&text).unwrap();assert_eq!(projected[slot][0][field],serde_json::Value::String(raw.into()));
            let restored=deserialize(&wire).unwrap();let actual=match entity{"AnalysisRecord"=>restored.analyses[0].duration_ms.unwrap(),"SearchFilter"=>restored.search_filters[0].use_count,"TemplateRecord"=>restored.templates[0].usage_count,"KnowledgeRecord"=>restored.knowledge_payload[0].usage_count,_=>unreachable!()};assert_eq!(actual,word);
            if word>9007199254740991{let mut malformed=projected;malformed[slot][0][field]=serde_json::json!(word);assert!(deserialize_bytes(malformed.to_string().as_bytes()).is_err(),"unsafe JSON numeric unsigned64 must not enter the declared boundary");}
        }
    }
}

#[cfg(test)]
#[test]
fn sqlite_snapshot_program_declared_json_covers_all_authored_registers_and_optional_quantities(){
    let base=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any");let corpus:serde_json::Value=serde_json::from_slice(&std::fs::read(base.join("🧬️schema/📸️snapshot/🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap()).unwrap();
    let mut root:serde_json::Value=serde_json::from_slice(&std::fs::read(base.join("🧫️fixtures/🧬️mutations/👥️stakeholder/🌱️create/🌱️creates-a/📸️snapshot/➡️after/🔣️.json")).unwrap()).unwrap();
    for row in corpus["registerFixtures"].as_array().unwrap(){let key=row[0].as_str().unwrap();let path=base.join("🧫️fixtures/🧬️mutations").join(row[1].as_str().unwrap()).join(row[2].as_str().unwrap()).join("📸️snapshot/➡️after/🔣️.json");let source:serde_json::Value=serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();root[key]=source[key].clone();}
    let expected:ProgramSnapshot=store::json::from_json_str(&root.to_string()).unwrap();let wire=crate::io::export::serializers::artifacts::json::v_rfc8259::any::serialize(&expected).unwrap();let actual=deserialize(&wire).unwrap();assert_eq!(actual,expected);
    let text=semio_s_artifact_stdio_json::schema::snapshot::write_json_pretty(&wire.value);let independent:serde_json::Value=serde_json::from_str(&text).unwrap();for row in corpus["registerFixtures"].as_array().unwrap(){assert_eq!(independent[row[0].as_str().unwrap()].as_array().unwrap().len(),1);}
}

```

