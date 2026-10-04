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
    let mut out: ProgramSnapshot = semio_framework_value::FromValue::from_value(crate::standards::v1::subsets::any::io::program_json::convert(semio_framework_pack_json::to_dsl_value(&from.to_pack_value()),true).map_err(|message|semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message,semio_framework_diagnostic::TextSpan::at(1,1)))?).map_err(|e: semio_framework_value::ValueError| semio_framework_diagnostic::TextError::new(e.kind, format!("program<-json: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
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
    let mut expected:ProgramSnapshot=semio_framework_pack_json::from_json_str(&std::fs::read_to_string(fixture).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
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
            let raw=raw.as_str().unwrap();let word=raw.parse::<u64>().unwrap();let mut expected:ProgramSnapshot=semio_framework_pack_json::from_json_str(&source, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
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
    let expected:ProgramSnapshot=semio_framework_pack_json::from_json_str(&root.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let wire=crate::io::export::serializers::artifacts::json::v_rfc8259::any::serialize(&expected).unwrap();let actual=deserialize(&wire).unwrap();assert_eq!(actual,expected);
    let text=semio_s_artifact_stdio_json::schema::snapshot::write_json_pretty(&wire.value);let independent:serde_json::Value=serde_json::from_str(&text).unwrap();for row in corpus["registerFixtures"].as_array().unwrap(){assert_eq!(independent[row[0].as_str().unwrap()].as_array().unwrap().len(),1);}
}
