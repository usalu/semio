use super::*;
use crate::wfc_engine::topology::GraphTopologyBuilder;

fn checkerboard() -> (CompiledModel, crate::wfc_engine::topology::GraphTopology) {
    let mut b = ModelBuilder::new();
    let black = b.add_pattern(1.0);
    let white = b.add_pattern(2.0);
    b.add_tag(black, "dark");
    let adj = b.add_relation("adjacent");
    b.allow_mirrored(adj, black, white);
    let model = b.compile().unwrap();
    let mut tb = GraphTopologyBuilder::new(3);
    tb.arc(crate::wfc_engine::ids::NodeId(0), crate::wfc_engine::ids::NodeId(1), adj);
    tb.arc(crate::wfc_engine::ids::NodeId(1), crate::wfc_engine::ids::NodeId(0), adj);
    tb.arc(crate::wfc_engine::ids::NodeId(1), crate::wfc_engine::ids::NodeId(2), adj);
    tb.arc(crate::wfc_engine::ids::NodeId(2), crate::wfc_engine::ids::NodeId(1), adj);
    (model, tb.build().unwrap())
}

#[test]
fn from_model_compile_round_trip_preserves_fingerprint() {
    let (model, _topo) = checkerboard();
    let doc = SourceModelDoc::from_model(&model);
    let recompiled = doc.compile().unwrap();
    assert_eq!(recompiled.fingerprint(), model.fingerprint());
    assert_eq!(recompiled.pattern_count(), model.pattern_count());
}

#[test]
fn source_model_doc_json_round_trips() {
    let (model, _topo) = checkerboard();
    let doc = SourceModelDoc::from_model(&model);
    let json = dsl::json::to_json_string(&doc);
    let back: SourceModelDoc = dsl::json::from_json_str(&json).unwrap();
    assert_eq!(back, doc);
    assert_eq!(back.compile().unwrap().fingerprint(), model.fingerprint());
}

#[test]
fn compile_rejects_unknown_schema_version() {
    let (model, _topo) = checkerboard();
    let mut doc = SourceModelDoc::from_model(&model);
    doc.version = 999;
    assert_eq!(doc.compile().unwrap_err(), ModelError::SchemaVersionMismatch { expected: SOURCE_MODEL_VERSION, actual: 999 });
}

#[test]
fn hand_authored_asymmetric_allow_fails_validate_on_compile() {
    // A hand-edited document declares `adj` self-inverse but only allows black->white, never
    // the reverse: `compile()` must still run `validate()` and reject it, not just build
    // silently-broken bitset tables.
    let doc = SourceModelDoc {
        version: SOURCE_MODEL_VERSION,
        patterns: vec![PatternDoc { weight: 1.0, tags: vec![] }, PatternDoc { weight: 1.0, tags: vec![] }],
        relations: vec![RelationDoc { name: "adj".to_string(), inverse: None }],
        allow: vec![PairDoc { relation: 0, src: 0, dst: 1 }],
        deny: vec![],
    };
    assert!(matches!(doc.compile().unwrap_err(), ModelError::AsymmetricInverse { .. }));
}

#[test]
fn checkpoint_doc_round_trips_and_resumes() {
    let (model, topo) = checkerboard();
    let fingerprint = model.fingerprint();
    let mut domains = vec![model.full_domain(); topo.node_count()];
    let mut pinned = PatternSet::new_empty(model.pattern_count());
    pinned.set(PatternId(0), true);
    domains[0] = pinned;
    let checkpoint = Checkpoint::new(domains, fingerprint, 5);

    let doc = CheckpointDoc::from_checkpoint(&checkpoint);
    let json = dsl::json::to_json_string(&doc);
    let back: CheckpointDoc = dsl::json::from_json_str(&json).unwrap();
    let restored = back.into_checkpoint(&model, topo.node_count()).unwrap();
    assert_eq!(restored.model_fingerprint, fingerprint);
    assert_eq!(restored.seed, 5);
    assert!(restored.domains[0].get(PatternId(0)));
}

#[test]
fn checkpoint_doc_rejects_version_mismatch() {
    let (model, topo) = checkerboard();
    let mut doc = CheckpointDoc { version: CHECKPOINT_VERSION, domains: vec![model.full_domain(); topo.node_count()], model_fingerprint: model.fingerprint(), seed: 0 };
    doc.version = 7;
    assert_eq!(doc.into_checkpoint(&model, topo.node_count()).unwrap_err(), SolveError::CheckpointVersionMismatch { expected: CHECKPOINT_VERSION, actual: 7 });
}

#[test]
fn checkpoint_doc_rejects_fingerprint_mismatch() {
    let (model, topo) = checkerboard();
    let doc = CheckpointDoc { version: CHECKPOINT_VERSION, domains: vec![model.full_domain(); topo.node_count()], model_fingerprint: 0xDEAD_BEEF, seed: 0 };
    assert_eq!(doc.into_checkpoint(&model, topo.node_count()).unwrap_err(), SolveError::CorruptCheckpoint { reason: "model fingerprint mismatch" });
}

#[test]
fn checkpoint_doc_rejects_wrong_domain_count() {
    let (model, topo) = checkerboard();
    let doc = CheckpointDoc { version: CHECKPOINT_VERSION, domains: vec![model.full_domain(); topo.node_count() - 1], model_fingerprint: model.fingerprint(), seed: 0 };
    assert_eq!(doc.into_checkpoint(&model, topo.node_count()).unwrap_err(), SolveError::CorruptCheckpoint { reason: "domain count does not match topology node count" });
}

#[test]
fn checkpoint_doc_rejects_wrong_bitset_length() {
    let (model, topo) = checkerboard();
    let mut domains = vec![model.full_domain(); topo.node_count()];
    domains[0] = PatternSet::new_full(model.pattern_count() + 1);
    let doc = CheckpointDoc { version: CHECKPOINT_VERSION, domains, model_fingerprint: model.fingerprint(), seed: 0 };
    assert_eq!(doc.into_checkpoint(&model, topo.node_count()).unwrap_err(), SolveError::CorruptCheckpoint { reason: "domain bitset length does not match model pattern count" });
}

#[test]
fn checkpoint_doc_rejects_tampered_bitset_from_raw_json() {
    // Simulates a hand-edited file: valid JSON shape, but a bitset with a stray bit set past
    // its declared `len` in the `words` array — must be caught by `is_well_formed`, not panic.
    let (model, topo) = checkerboard();
    let json = format!(r#"{{"version":1,"domains":[{{"words":[999999],"len":2}}{}],"model_fingerprint":{},"seed":0}}"#, ",{\"words\":[3],\"len\":2}".repeat(topo.node_count() - 1), model.fingerprint());
    let doc: CheckpointDoc = dsl::json::from_json_str(&json).unwrap();
    assert_eq!(doc.into_checkpoint(&model, topo.node_count()).unwrap_err(), SolveError::CorruptCheckpoint { reason: "domain bitset failed structural well-formedness check" });
}
