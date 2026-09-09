use super::*;

trait ProgramChildOwnerOracle {
    fn expected() -> serde_json::Value;
}

struct SerdeJsonProgramChildOwnerOracle;

impl ProgramChildOwnerOracle for SerdeJsonProgramChildOwnerOracle {
    fn expected() -> serde_json::Value {
        serde_json::from_str(include_str!("../../🧫️fixtures/🧫️child-owner-isolation/🔣️.json")).expect("language-neutral Architect child-owner fixture")
    }
}

#[semio_framework_async_macros::async_test]
async fn empty_plugin_has_schema() {
    let program = empty_plugin();
    assert_eq!(program.schema, ARCHITECT_PROGRAM_SCHEMA);
    assert_eq!(program.meta.schema, ARCHITECT_PROGRAM_SCHEMA);
}

#[semio_framework_async_macros::async_test]
async fn sample_plugin_round_trips_json() {
    let program = sample_plugin();
    let json = dsl::json::to_json_string(&program);
    let decoded: ProgramSnapshot = dsl::json::from_json_str(&json).expect("deserialize");
    let oracle: serde_json::Value = serde_json::from_str(&json).expect("third-party JSON oracle");
    assert_eq!(oracle["elements"].as_array().expect("elements").len(), decoded.elements.len());
    assert_eq!(oracle["adjacencies"].as_array().expect("adjacencies").len(), decoded.adjacencies.len());
    assert_eq!(decoded.elements.len(), 2);
    assert_eq!(decoded.adjacencies.len(), 1);
}

#[test]
fn program_document_contract_json_text_pack_and_projection() {
    let source = include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧫️fixtures/🔬️document-contract/🔣️.json");
    let oracle: serde_json::Value = serde_json::from_str(source).expect("language-neutral Program document fixture");
    assert!(oracle.get("documents").is_none(), "obsolete documents alias must be rejected from the final fixture");
    assert!(oracle.get("artifacts").is_some(), "native artifacts register must be present");

    let snapshot: ProgramSnapshot = dsl::json::from_json_str(source).expect("native Program JSON decoder");
    let canonical: serde_json::Value = serde_json::from_str(&dsl::json::to_json_string(&snapshot)).expect("native Program canonical JSON");
    assert_eq!(canonical, oracle, "native JSON codec must preserve the fixture contract");

    let mut foreign_parent = oracle.clone();
    foreign_parent.as_object_mut().expect("Program fixture object").insert("foreign".into(), serde_json::Value::Bool(true));
    let foreign_parent_json = serde_json::to_string(&foreign_parent).expect("foreign parent JSON");
    assert!(dsl::json::from_json_str::<ProgramSnapshot>(&foreign_parent_json).is_err(), "native Program snapshot decoder must reject a foreign parent field");
    assert!(dsl::json::from_json_str::<crate::schema::ProgramArtifact>(&foreign_parent_json).is_err(), "native Program artifact decoder must reject a foreign parent field");

    let mut foreign_row: serde_json::Value = serde_json::from_str(&dsl::json::to_json_string(&sample_plugin())).expect("sample Program JSON");
    foreign_row["stakeholders"].as_array_mut().expect("stakeholder register")[0]
        .as_object_mut()
        .expect("stakeholder row")
        .insert("foreign".into(), serde_json::Value::Bool(true));
    let foreign_row_json = serde_json::to_string(&foreign_row).expect("foreign nested row JSON");
    assert!(dsl::json::from_json_str::<ProgramSnapshot>(&foreign_row_json).is_err(), "native Program snapshot decoder must reject a foreign nested row field");
    for child in [&snapshot.knowledge, &snapshot.benchmarks] {
        assert_eq!(child.child_id, child.target.artifact_id, "child target must address its minted child identity");
        assert_eq!(child.target.dialect.artifact_kind, "s.stdio.semio");
        assert_eq!(child.target.dialect.standard, "v1");
        assert_eq!(child.target.dialect.subset, "table");
    }

    let artifact = crate::schema::ProgramArtifact::from_snapshot(snapshot.clone());
    assert_eq!(artifact.to_snapshot(), snapshot, "artifact/snapshot projection must preserve every document field");
    semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&snapshot);
    semio_framework_os_kernel::os_store::test_support::assert_pack_round_trip(&snapshot);
    println!("[DEBUG] program-document-contract json=text=pack=projection child-identities=2 unknown-fields=parent,nested-row");
}

#[semio_framework_async_macros::async_test]
async fn composed_register_rows_belong_to_each_exact_child() {
    let benchmarks = benchmarks_child_from_records(&[]);
    let knowledge = knowledge_child_from_records(&[]);
    let benchmark_wire = dsl::json::to_json_string(&benchmarks);
    let knowledge_wire = dsl::json::to_json_string(&knowledge);
    let reconstructed_benchmarks: ProgramBenchmarksChild = dsl::json::from_json_str(&benchmark_wire).expect("Architect benchmark child wire roundtrip");
    let reconstructed_knowledge: ProgramKnowledgeChild = dsl::json::from_json_str(&knowledge_wire).expect("Architect knowledge child wire roundtrip");
    let observed = serde_json::json!({
        "benchmarksOwned": benchmarks.local_owner::<ProgramBenchmarksWorkingTable>().is_some(),
        "knowledgeOwned": knowledge.local_owner::<ProgramKnowledgeWorkingTable>().is_some(),
        "benchmarksWireIdentityMatches": benchmarks == reconstructed_benchmarks,
        "knowledgeWireIdentityMatches": knowledge == reconstructed_knowledge,
        "benchmarksWireOwned": reconstructed_benchmarks.local_owner::<ProgramBenchmarksWorkingTable>().is_some(),
        "knowledgeWireOwned": reconstructed_knowledge.local_owner::<ProgramKnowledgeWorkingTable>().is_some(),
    });

    assert_eq!(observed, SerdeJsonProgramChildOwnerOracle::expected());
}

// #region 🔖️DslArtifact
#[semio_framework_async_macros::async_test]
async fn empty_plugin_dsl_round_trips() {
    semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&empty_plugin());
    semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&empty_plugin());
}

#[semio_framework_async_macros::async_test]
async fn sample_plugin_dsl_round_trips() {
    semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&sample_plugin());
}

#[test]
// 🪲️ Blocked on a confirmed upstream `pack` crate bug, NOT an architect defect: table
// rows (`#[dsl(table)] Vec<Stakeholder>` etc.) decode via `pack::value`'s self-describing
fn sample_plugin_dsl_pack_equivalence() {
    semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&sample_plugin());
}

#[semio_framework_async_macros::async_test]
async fn sample_plugin_dsl_text_is_parseable_and_reflects_registers() {
    let printed = sample_plugin().print_dsl();
    assert!(printed.contains("Sample Clinic"), "printed dsl text must contain program title: {printed}");
    assert!(printed.contains("REC"), "printed dsl text must contain the reception element code: {printed}");
}

/// @emoji 🧪️ The bundled `.architect` fixture (a static transcription of `sample_plugin()`)
/// parses and round-trips — the compile-time validation ground truth for
/// `ARCHITECT_EXAMPLE_TEXT`. Compared field-by-field rather than via `PartialEq` against a
/// freshly called `sample_plugin()`, because `EntityId::new_serial` draws from a
/// process-wide counter shared with every other test in this binary, so the serial ids a
/// fresh call mints depend on test execution order and never match the fixture's baked-in ids.
#[semio_framework_async_macros::async_test]
async fn architect_example_text_parses_to_sample_plugin_and_round_trips() {
    let parsed = ProgramSnapshot::parse_dsl(document_dsl::ARCHITECT_EXAMPLE_TEXT).expect("parse bundled .architect example");
    let expected = sample_plugin();
    assert_eq!(parsed.meta.title, expected.meta.title);
    assert_eq!(parsed.meta.industry_sector, expected.meta.industry_sector);
    assert_eq!(parsed.project.code, expected.project.code);
    assert_eq!(parsed.project.client_name, expected.project.client_name);
    assert_eq!(parsed.stakeholders.len(), expected.stakeholders.len());
    assert_eq!(parsed.stakeholders[0].header.name, expected.stakeholders[0].header.name);
    assert_eq!(parsed.elements.len(), expected.elements.len());
    assert_eq!(parsed.elements[0].code, expected.elements[0].code);
    assert_eq!(parsed.elements[1].code, expected.elements[1].code);
    assert_eq!(parsed.adjacencies.len(), expected.adjacencies.len());
    assert_eq!(parsed.adjacencies[0].kind, expected.adjacencies[0].kind);
    semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&parsed);
    semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&parsed);
}
// #endregion 🔖️DslArtifact
