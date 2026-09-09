use super::*;

fn checkerboard_model() -> CompiledModel {
    let mut b = ModelBuilder::new();
    let black = b.add_pattern(1.0);
    let white = b.add_pattern(1.0);
    let adj = b.add_relation("adjacent");
    b.allow_mirrored(adj, black, white);
    b.allow_mirrored(adj, white, black);
    b.compile().unwrap()
}

#[test]
fn compile_rejects_empty_pattern_universe() {
    let b = ModelBuilder::new();
    assert_eq!(b.compile().unwrap_err(), ModelError::EmptyPatternUniverse);
}

#[test]
fn compile_rejects_invalid_weight() {
    let mut b = ModelBuilder::new();
    b.add_pattern(-1.0);
    assert!(matches!(b.compile().unwrap_err(), ModelError::InvalidWeight { .. }));
}

#[test]
fn allowed_and_supporters_are_transposes() {
    let m = checkerboard_model();
    let adj = RelationId(0);
    for src in 0..m.pattern_count() {
        let src_id = PatternId::from_index(src);
        for dst in m.allowed(adj, src_id).iter_ones() {
            assert!(m.supporters(adj, dst).get(src_id));
        }
    }
}

#[test]
fn validate_passes_on_mirrored_model() {
    let m = checkerboard_model();
    assert!(m.validate().is_ok());
}

#[test]
fn assembly_cursor_compiler_matches_canonical_builder() {
    let mut canonical = ModelBuilder::new();
    let a = canonical.add_pattern(1.0);
    let b = canonical.add_pattern(2.0);
    let relation = canonical.add_relation("adjacent");
    canonical.allow_mirrored(relation, a, b);
    let canonical = canonical.compile().expect("canonical model");
    let mut pairs = std::collections::BTreeSet::new();
    pairs.insert((a.get(), b.get()));
    pairs.insert((b.get(), a.get()));
    let mut build = AssemblyModelBuild::new(vec![1.0, 2.0], pairs).expect("assembly compiler");
    let incremental = loop {
        if let Some(model) = build.step().expect("assembly unit") {
            break model;
        }
    };
    assert_eq!(build.progress(), (build.total_units, build.total_units));
    assert_eq!(incremental.fingerprint(), canonical.fingerprint());
    assert_eq!(incremental.allowed(relation, a), canonical.allowed(relation, a));
    assert_eq!(incremental.allowed(relation, b), canonical.allowed(relation, b));
    assert_eq!(incremental.supporters(relation, a), canonical.supporters(relation, a));
    assert_eq!(incremental.supporters(relation, b), canonical.supporters(relation, b));
}

#[test]
fn validate_fails_on_asymmetric_declaration() {
    let mut b = ModelBuilder::new();
    let a = b.add_pattern(1.0);
    let c = b.add_pattern(1.0);
    let r = b.add_relation("one_way");
    b.allow(r, a, c); // only one direction declared; r is self-inverse by default
    let m = b.compile().unwrap();
    assert!(matches!(m.validate().unwrap_err(), ModelError::AsymmetricInverse { .. }));
}

#[test]
fn deny_wins_over_allow_regardless_of_order() {
    let mut b = ModelBuilder::new();
    let a = b.add_pattern(1.0);
    let c = b.add_pattern(1.0);
    let r = b.add_relation("r");
    b.deny(r, a, c);
    b.allow(r, a, c);
    let m = b.compile().unwrap();
    assert!(!m.allowed(r, a).get(c));
}

#[test]
fn fingerprint_is_deterministic_and_sensitive() {
    let m1 = checkerboard_model();
    let m2 = checkerboard_model();
    assert_eq!(m1.fingerprint(), m2.fingerprint());

    let mut b = ModelBuilder::new();
    let a = b.add_pattern(1.0);
    let c = b.add_pattern(2.0); // different weight
    let r = b.add_relation("adjacent");
    b.allow_mirrored(r, a, c);
    b.allow_mirrored(r, c, a);
    let m3 = b.compile().unwrap();
    assert_ne!(m1.fingerprint(), m3.fingerprint());
}

#[test]
fn lint_flags_unconstrained_and_unsupported() {
    let mut b = ModelBuilder::new();
    let a = b.add_pattern(1.0);
    let c = b.add_pattern(1.0);
    let free = b.add_relation("free");
    b.allow_mirrored(free, a, c);
    b.allow_mirrored(free, a, a);
    b.allow_mirrored(free, c, c);
    let starved = b.add_relation("starved");
    b.allow(starved, a, a); // c has no supporters at all under `starved`
    let m = b.compile().unwrap();
    let findings = m.lint();
    assert!(findings.contains(&LintFinding::UnconstrainedRelation { relation: free }));
    assert!(findings.iter().any(|f| matches!(f, LintFinding::UnsupportedPattern { pattern, relation } if *pattern == c && *relation == starved)));
}

#[test]
fn stats_report_sane_values() {
    let m = checkerboard_model();
    let stats = m.stats();
    assert_eq!(stats.pattern_count, 2);
    assert_eq!(stats.relation_count, 1);
    assert_eq!(stats.allowed_pair_count, 2);
    assert_eq!(stats.weight_min, 1.0);
    assert_eq!(stats.weight_max, 1.0);
}

#[test]
fn tags_are_interned_and_deduplicated() {
    let mut b = ModelBuilder::new();
    let p = b.add_pattern(1.0);
    let id1 = b.add_tag(p, "solid");
    let id2 = b.add_tag(p, "solid");
    assert_eq!(id1, id2);
    b.add_relation("r");
    let m = b.compile().unwrap();
    assert_eq!(m.pattern_info(p).tags, vec![id1]);
    assert_eq!(m.tag_name(id1), Some("solid"));
    assert_eq!(m.tag_id("solid"), Some(id1));
}
