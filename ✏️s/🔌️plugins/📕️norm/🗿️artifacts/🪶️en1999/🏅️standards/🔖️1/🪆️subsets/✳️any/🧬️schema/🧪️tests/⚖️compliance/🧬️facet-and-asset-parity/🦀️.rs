//! 🧬️ Facet and asset parity of the EN 1999 compliance tests: the example snapshot schema, the bundled assets, the field metadata and the facet leaves. See [`super`].

use super::*;

#[test]
fn example_snapshot_validates_against_json_schema() {
    let schema_path = snapshot_schema_path();
    for snap in [En1999Snapshot::compliant_roof_purlin(), En1999Snapshot::noncompliant_multi_fail()] {
        let instance = serde_json::to_string(&snap).expect("instance");
        let tmp = std::env::temp_dir().join(format!("en1999-instance-{}.json", snap.members[0].id));
        std::fs::write(&tmp, &instance).unwrap();
        let status = Command::new("python3")
            .arg("-c")
            .arg("import json,sys,jsonschema; s=json.load(open(sys.argv[1])); i=json.load(open(sys.argv[2])); jsonschema.validate(instance=i, schema=s); print('ok')")
            .arg(&schema_path)
            .arg(&tmp)
            .output()
            .expect("python jsonschema");
        assert!(
            status.status.success(),
            "jsonschema: {}\n{}",
            String::from_utf8_lossy(&status.stdout),
            String::from_utf8_lossy(&status.stderr)
        );
    }
}

#[test]

fn bundled_example_assets_match_regenerated_dsl_and_pack() {
    let any = family_any_dir();
    for (name, snap) in [
        ("🏠️aluminium-roof-purlin", En1999Snapshot::compliant_roof_purlin()),
        ("🏚️noncompliant-multi-fail", En1999Snapshot::noncompliant_multi_fail()),
    ] {
        let dsl_path = any.join("🖼️assets").join(name).join(name).join("🗣️.dsl.semio");
        let pack_path = any.join("🖼️assets").join(name).join("🎒️.pack.semio");
        let expected_dsl = encode_en1999_dsl(&snap);
        let expected_pack = semio_s_artifact_norm_en1999_io::binary::snapshot::encode_en1999_pack(&snap);
        let committed_dsl = std::fs::read_to_string(&dsl_path).unwrap_or_default();
        let committed_pack = std::fs::read(&pack_path).unwrap_or_default();
        assert_eq!(
            committed_dsl, expected_dsl,
            "DSL drift for {name}: regenerate 🖼️assets/{name}/{name}/🗣️.dsl.semio"
        );
        assert_eq!(
            committed_pack, expected_pack,
            "pack drift for {name}: regenerate 🖼️assets/{name}/🎒️.pack.semio"
        );
    }
}

#[test]
fn field_meta_covers_every_editable_leaf_en_de() {
    use semio_s_artifact_norm_en1999_editor::field_meta::en1999_field_meta;
    for snap in [En1999Snapshot::compliant_roof_purlin(), En1999Snapshot::noncompliant_multi_fail()] {
        let value = serde_json::to_value(&snap).expect("json");
        let mut missing = Vec::new();
        walk_leaves("", &value, &mut |path| {
            if path.is_empty() {
                return;
            }
            match en1999_field_meta(path) {
                None => missing.push(path.to_string()),
                Some(meta) => {
                    assert!(!meta.label_en.is_empty(), "{path} empty en");
                    assert!(!meta.label_de.is_empty(), "{path} empty de");
                    if let Some(choices) = meta.choices {
                        for c in choices {
                            assert!(!c.label_en.is_empty(), "{path} choice en");
                            assert!(!c.label_de.is_empty(), "{path} choice de");
                            assert!(
                                c.label_en != c.value || c.label_de != c.value,
                                "{path} choice labels must not be raw codes only ({})",
                                c.value
                            );
                        }
                    }
                }
            }
        });
        assert!(missing.is_empty(), "missing field-meta for: {missing:?}");
    }
}

#[test]
fn facet_field_names_match_snapshot_json_schema() {
    let schema_path = snapshot_schema_path();
    let schema: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&schema_path).unwrap()).unwrap();
    let ts = std::fs::read_to_string(family_any_dir().join("🧬️schema/📸️snapshot/🟦️.ts")).unwrap_or_default();
    let gql = std::fs::read_to_string(family_any_dir().join("🧬️schema/📸️snapshot/🔗️.graphql")).unwrap_or_default();
    let proto = std::fs::read_to_string(family_any_dir().join("🧬️schema/📸️snapshot/🛰️.proto")).unwrap_or_default();
    let mut_ts = std::fs::read_to_string(family_any_dir().join("🧬️schema/🧬️mutations/🟦️.ts")).unwrap_or_default();
    assert!(ts.contains("nK") && ts.contains("gKLine") && ts.contains("coldFormed") && ts.contains("shells") && ts.contains("detailCategory") && ts.contains("m1"), "TS facet stale");
    assert!(!ts.contains("vYEd") && !ts.contains("slopeM") && !ts.contains("mYEd"), "TS still has member design-effect names");
    assert!(ts.contains("MemberAction") && ts.contains("actions: MemberAction"));
    assert!(gql.contains("nK") && gql.contains("coldFormed") && gql.contains("m1") && !gql.contains("slopeM") && !gql.contains("vYEd"));
    assert!(proto.contains("n_k") && proto.contains("cold_formed") && proto.contains("m1") && !proto.contains("slope_m") && !proto.contains("v_y_ed"));
    let actions = schema["properties"]["members"]["items"]["properties"]["actions"]["items"]["properties"].as_object().unwrap();
    for key in ["id","kind","category","source","gKLine","qKLine","nK","vYK","vZK","mYK","mZK"] {
        assert!(actions.contains_key(key), "json schema missing action.{key}");
        assert!(ts.contains(key), "TS missing {key}");
    }
}

#[test]
fn facet_diff_matches_rust() {
    let diff_ts = std::fs::read_to_string(family_any_dir().join("🧬️schema/🔺️diff/🟦️.ts")).unwrap_or_default();
    let diff_gql = std::fs::read_to_string(family_any_dir().join("🧬️schema/🔺️diff/🔗️.graphql")).unwrap_or_default();
    let diff_proto = std::fs::read_to_string(family_any_dir().join("🧬️schema/🔺️diff/🛰️.proto")).unwrap_or_default();
    for field in ["artifact", "annex", "materials", "sections", "members", "connections", "fireScenarios", "fatigueDetails", "coldFormed", "shells"] {
        assert!(diff_ts.contains(field), "diff TS missing {field}");
        assert!(diff_gql.contains(field), "diff GraphQL missing {field}");
    }
    for field in ["artifact", "annex", "materials", "sections", "members", "connections", "fire_scenarios", "fatigue_details", "cold_formed", "shells"] {
        assert!(diff_proto.contains(field), "diff proto missing {field}");
    }
    for stale in ["nEdKn", "mEdKnm", "chi?:", "sheetMEdKnm", "sigmaEdShellMpa", "n_ed_kn", "sigma_ed_shell_mpa", "double chi", "chi:"] {
        assert!(!diff_ts.contains(stale), "diff TS still has stale scalar {stale}");
        assert!(!diff_gql.contains(stale), "diff GraphQL still has stale scalar {stale}");
        assert!(!diff_proto.contains(stale), "diff proto still has stale scalar {stale}");
    }
}

#[test]
fn facet_mutations_match_kinds() {
    use semio_s_artifact_norm_en1999_mutations::KINDS;
    let mut_ts = std::fs::read_to_string(family_any_dir().join("🧬️schema/🧬️mutations/🟦️.ts")).unwrap_or_default();
    let mut_gql = std::fs::read_to_string(family_any_dir().join("🧬️schema/🧬️mutations/🔗️.graphql")).unwrap_or_default();
    let mut_proto = std::fs::read_to_string(family_any_dir().join("🧬️schema/🧬️mutations/🛰️.proto")).unwrap_or_default();
    assert_eq!(KINDS.len(), 18, "Rust KINDS length drift");
    for kind in KINDS {
        assert!(mut_ts.contains(kind), "mutations TS missing KINDS entry {kind}");
        assert!(mut_gql.contains(kind), "mutations GraphQL missing KINDS entry {kind}");
        assert!(mut_proto.contains(kind), "mutations proto missing KINDS entry {kind}");
    }
    for stale in ["changeNEdKn", "changeChi", "changeSheetMEdKnm", "changeSigmaEdShellMpa", "nEdKn", "change_n_ed_kn"] {
        assert!(!mut_ts.contains(stale), "mutations TS still has stale scalar mutation {stale}");
        assert!(!mut_gql.contains(stale), "mutations GraphQL still has stale scalar mutation {stale}");
        assert!(!mut_proto.contains(stale), "mutations proto still has stale scalar mutation {stale}");
    }
}

#[test]
fn governing_uls_combination_named_in_member_explanations() {
    let report = evaluate_structure(&En1999Snapshot::compliant_roof_purlin());
    let uls: Vec<_> = report.checks.iter().filter(|c| c.id.contains("en1999.6.2.") || c.id.contains("en1999.6.3.")).collect();
    assert!(!uls.is_empty());
    for c in &uls {
        assert!(
            c.explanation.en.contains("uls-610a") || c.explanation.en.contains("uls-610b") || c.explanation.en.contains("uls-g"),
            "missing ULS combo in en explanation for {}: {}", c.id, c.explanation.en
        );
        assert!(
            c.explanation.de.contains("uls-610a") || c.explanation.de.contains("uls-610b") || c.explanation.de.contains("uls-g"),
            "missing ULS combo in de explanation for {}: {}", c.id, c.explanation.de
        );
        assert!(
            c.explanation.en.contains("lead ") && c.explanation.de.contains("führend "),
            "missing action_id lead tag for {}: en={} de={}", c.id, c.explanation.en, c.explanation.de
        );
    }
    assert!(uls.iter().any(|c| c.explanation.de.contains("GZT")));
    assert!(report.checks.iter().any(|c| c.id.contains("sls-freq") && c.explanation.en.contains("sls-freq")));
}

#[test]
fn explanations_en_de_not_identical_except_numbers() {
    for snap in [En1999Snapshot::compliant_roof_purlin(), En1999Snapshot::noncompliant_multi_fail()] {
        let report = evaluate_structure(&snap);
        for c in &report.checks {
            let en = strip_nums(&c.explanation.en);
            let de = strip_nums(&c.explanation.de);
            if en.is_empty() { continue; }
            assert_ne!(en, de, "identical en/de explanation for {}: {:?}", c.id, c.explanation);
            for r in &c.remedies {
                let ren = strip_nums(&r.action.en);
                let rde = strip_nums(&r.action.de);
                if ren.is_empty() { continue; }
                assert_ne!(ren, rde, "identical en/de remedy for {}: {:?}", c.id, r.action);
            }
        }
    }
}
