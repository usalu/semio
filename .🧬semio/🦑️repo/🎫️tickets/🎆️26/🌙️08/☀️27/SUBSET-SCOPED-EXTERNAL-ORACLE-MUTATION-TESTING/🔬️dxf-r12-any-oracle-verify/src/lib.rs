//! 🔬️ Scratch verification of `s.stdio.dxf@r12/any`'s committed oracle module against the NEW
//! `drafting-plate` third-party-generated fixture. Links the real oracle source by `#[path]` and
//! nothing else, so the shared stdio oracle crate's unrelated `cargo test` breakage (pdf@1.4 and
//! step ap214 fixture restructuring, already flagged by the gif/las/pdf17 wave) cannot block it.
//!
//! Ticket 26/08/27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING.

#[path = "../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
pub mod dxf_r12_any;

#[cfg(test)]
mod tests {
    use super::dxf_r12_any::{oracle_apply_mutation, oracle_apply_mutation_inverse, project_dxf_r12};
    use semio_repo_test_host::{parse_json, Json};

    const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧫️fixtures/drafting-plate/drafting-plate.dxf");

    /// 📇️ One JSON row per declared kind, chosen to land on THIS fixture's own starting state
    /// (layers `0`/`DIMS`/`TEXT`, styles `STANDARD`/`NOTES`/`TITLES`, linetypes
    /// `BYLAYER`/`BYBLOCK`/`CONTINUOUS`/`DASHED`/`HIDDEN`, blocks `SHELTER_POST`/`BENCH`, 7 entities).
    const ROWS: &[(&str, &str)] = &[
        ("no-mutation", "{}"),
        ("set-snapshot", r#"{"insertionBase": [5, 5, 0], "layers": [{"name": "0", "color": 7, "linetype": "CONTINUOUS"}], "entities": [{"entityKind": "circle", "layer": "0", "center": [0, 0, 0], "radius": 42}]}"#),
        ("set-header-var", r#"{"name": "$INSBASE", "value": [40, 60, 0]}"#),
        ("remove-header-var", r#"{"name": "$INSBASE"}"#),
        ("insert-layer", r#"{"index": 1, "name": "MARKERS", "color": 6, "linetype": "HIDDEN"}"#),
        ("remove-layer", r#"{"name": "DIMS"}"#),
        ("set-layer", r#"{"name": "DIMS", "color": 4, "linetype": "HIDDEN"}"#),
        ("insert-style", r#"{"index": 1, "name": "LABELS", "font": "arial.ttf"}"#),
        ("remove-style", r#"{"name": "NOTES"}"#),
        ("set-style", r#"{"name": "NOTES", "font": "simplex.shx"}"#),
        ("insert-linetype", r#"{"index": 1, "name": "CENTER", "description": "Center line __ . __ ."}"#),
        ("remove-linetype", r#"{"name": "DASHED"}"#),
        ("set-linetype", r#"{"name": "DASHED", "description": "Dash pattern reworked"}"#),
        ("insert-entity", r#"{"index": 2, "entityKind": "circle", "layer": "0", "center": [1000, 100, 0], "radius": 30}"#),
        ("remove-entity", r#"{"index": 3}"#),
        ("set-entity", r#"{"index": 5, "entityKind": "text", "layer": "TEXT", "position": [200, 260, 0], "height": 80, "value": "PLATE REVISION B"}"#),
        ("insert-block", r#"{"index": 1, "name": "BENCH_MARK", "basePoint": [0, 0, 0], "entities": [{"entityKind": "line", "layer": "0", "start": [0, 0, 0], "end": [100, 0, 0]}]}"#),
        ("remove-block", r#"{"index": 1}"#),
        ("set-block", r#"{"index": 0, "name": "SHELTER_POST", "basePoint": [3, 4, 0], "entities": [{"entityKind": "circle", "layer": "0", "center": [0, 0, 0], "radius": 20}]}"#),
    ];

    fn base() -> Vec<u8> {
        std::fs::read(FIXTURE).unwrap_or_else(|error| panic!("read {FIXTURE}: {error}"))
    }

    fn spec(kind: &str, params: &str) -> Json {
        parse_json(&format!(r#"{{"kind": "{kind}", "params": {params}}}"#)).unwrap_or_else(|error| panic!("bad spec JSON for {kind}: {error}"))
    }

    //#region 🔖️ComparisonProfile
    /// ⚖️ `semantic-dxf-r12-v1` as its own registration declares it: tolerance `1e-4` on every
    /// number, and `handle`/`ownerHandle`/`fileSize`/`byteLength` ignored wherever they appear.
    /// Implemented HERE rather than assumed, so the gate can be exercised in both directions.
    const TOLERANCE: f64 = 0.0001;
    const IGNORE_KEYS: &[&str] = &["handle", "ownerHandle", "fileSize", "byteLength"];

    #[derive(Debug)]
    struct Difference {
        path: String,
        detail: String,
        delta: f64,
    }

    fn compare(left: &Json, right: &Json) -> Vec<Difference> {
        let mut out = Vec::new();
        walk("$", left, right, &mut out);
        out
    }

    fn walk(path: &str, left: &Json, right: &Json, out: &mut Vec<Difference>) {
        match (left, right) {
            (Json::Number(a), Json::Number(b)) => {
                let delta = (a - b).abs();
                if delta > TOLERANCE {
                    out.push(Difference { path: path.to_string(), detail: format!("{a} vs {b}"), delta });
                }
            }
            (Json::String(a), Json::String(b)) => {
                if a != b {
                    out.push(Difference { path: path.to_string(), detail: format!("{a:?} vs {b:?}"), delta: f64::INFINITY });
                }
            }
            (Json::Bool(a), Json::Bool(b)) => {
                if a != b {
                    out.push(Difference { path: path.to_string(), detail: format!("{a} vs {b}"), delta: f64::INFINITY });
                }
            }
            (Json::Null, Json::Null) => {}
            (Json::Array(a), Json::Array(b)) => {
                if a.len() != b.len() {
                    out.push(Difference { path: path.to_string(), detail: format!("length {} vs {}", a.len(), b.len()), delta: f64::INFINITY });
                }
                for (index, (l, r)) in a.iter().zip(b.iter()).enumerate() {
                    walk(&format!("{path}[{index}]"), l, r, out);
                }
            }
            (Json::Object(a), Json::Object(b)) => {
                for (key, value) in a {
                    if IGNORE_KEYS.contains(&key.as_str()) {
                        continue;
                    }
                    match b.iter().find(|(name, _)| name == key) {
                        Some((_, other)) => walk(&format!("{path}.{key}"), value, other, out),
                        None => out.push(Difference { path: format!("{path}.{key}"), detail: "present vs absent".to_string(), delta: f64::INFINITY }),
                    }
                }
                for (key, _) in b {
                    if IGNORE_KEYS.contains(&key.as_str()) {
                        continue;
                    }
                    if !a.iter().any(|(name, _)| name == key) {
                        out.push(Difference { path: format!("{path}.{key}"), detail: "absent vs present".to_string(), delta: f64::INFINITY });
                    }
                }
            }
            _ => out.push(Difference { path: path.to_string(), detail: "type mismatch".to_string(), delta: f64::INFINITY }),
        }
    }
    //#endregion 🔖️ComparisonProfile

    /// 🧾️ Prints the fixture's own projection, so what the gate is measured over is stated, not
    /// inferred from the generator's intent.
    #[test]
    fn projects_the_declared_structure() {
        let projection = project_dxf_r12(&base()).expect("project base fixture");
        println!("[projection] {}", projection.to_string());
        assert_eq!(projection.str("acadVersion"), "R12");
        assert_eq!(projection.array("layers").len(), 3, "3 LAYER rows");
        assert_eq!(projection.array("styles").len(), 3, "3 STYLE rows");
        assert_eq!(projection.array("linetypes").len(), 5, "5 LTYPE rows");
        assert_eq!(projection.array("blocks").len(), 2, "2 BLOCKs");
        assert_eq!(projection.array("entities").len(), 7, "7 top-level ENTITIES");
        let kinds: Vec<String> = projection.array("entities").iter().map(|e| e.str("entityKind")).collect();
        assert_eq!(kinds, vec!["line", "line", "circle", "arc", "solid", "text", "insert"], "all six typed entity kinds present");
    }

    /// 🦠️ Every declared kind, forward and inverse, against THIS fixture. Reports per kind whether
    /// the projection actually MOVED — the witnessability evidence the manifest's `oracleRequirements`
    /// stand on.
    #[test]
    fn all_19_kinds_mutate_and_invert_on_this_fixture() {
        assert_eq!(ROWS.len(), 19, "must exercise all 19 declared kinds");
        let input = base();
        let base_projection = project_dxf_r12(&input).expect("project base fixture");

        for (kind, params) in ROWS {
            let spec = spec(kind, params);
            let mutated = oracle_apply_mutation(&input, &spec).unwrap_or_else(|error| panic!("mutate {kind} failed: {error}"));
            assert!(!mutated.is_empty(), "mutate {kind} produced empty bytes");
            let mutated_projection = project_dxf_r12(&mutated).unwrap_or_else(|error| panic!("project mutate {kind} output failed: {error}"));
            let forward = compare(&base_projection, &mutated_projection);
            if *kind == "no-mutation" {
                assert!(forward.is_empty(), "no-mutation moved the projection: {forward:?}");
            } else {
                assert!(!forward.is_empty(), "{kind} produced no semantic change — NOT witnessable on this fixture");
            }

            let inverted = oracle_apply_mutation_inverse(&input, &spec).unwrap_or_else(|error| panic!("inverse {kind} failed: {error}"));
            let inverted_projection = project_dxf_r12(&inverted).unwrap_or_else(|error| panic!("project inverse {kind} output failed: {error}"));
            let restored = compare(&base_projection, &inverted_projection);
            assert!(restored.is_empty(), "inverse {kind} did not restore the base projection: {restored:?}");

            let witness: Vec<&str> = forward.iter().map(|difference| difference.path.as_str()).take(4).collect();
            println!("[witness] {kind:<18} moved={:<3} first={:?}", forward.len(), witness);
        }
    }

    /// ✅️ The gate ACCEPTS a known-good pair: the oracle's own re-encode of the fixture (a real
    /// `dxf` load/save round trip, byte-different from the input) against the input itself.
    #[test]
    fn gate_accepts_a_known_good_pair() {
        let input = base();
        let reencoded = oracle_apply_mutation(&input, &spec("no-mutation", "{}")).expect("re-encode");
        assert_ne!(reencoded, input, "byte pass-through would make this test vacuous");
        let differences = compare(&project_dxf_r12(&input).expect("project input"), &project_dxf_r12(&reencoded).expect("project re-encode"));
        println!("[gate accept] bytes {} vs {} (differ), projection differences = {}", input.len(), reencoded.len(), differences.len());
        assert!(differences.is_empty(), "gate rejected a known-good pair: {differences:?}");
    }

    /// ❌️ The gate REJECTS a known-bad pair: the SAME mutation kind and payload applied to the WRONG
    /// target. `set-entity` on index 5 (the TEXT) is correct; index 3 (the ARC) is the wrong one.
    /// Both documents are produced by the reference library itself — neither is hand-built to differ.
    #[test]
    fn gate_rejects_the_same_mutation_on_the_wrong_target() {
        let input = base();
        let payload = r#"{"entityKind": "text", "layer": "TEXT", "position": [200, 260, 0], "height": 80, "value": "PLATE REVISION B"}"#;
        let right = oracle_apply_mutation(&input, &spec("set-entity", &format!(r#"{{"index": 5, {}"#, &payload[1..]))).expect("mutate right target");
        let wrong = oracle_apply_mutation(&input, &spec("set-entity", &format!(r#"{{"index": 3, {}"#, &payload[1..]))).expect("mutate wrong target");
        let differences = compare(&project_dxf_r12(&right).expect("project right"), &project_dxf_r12(&wrong).expect("project wrong"));
        for difference in &differences {
            println!("[gate reject] {} — {} (delta {})", difference.path, difference.detail, difference.delta);
        }
        assert!(!differences.is_empty(), "gate ACCEPTED a mutation applied to the wrong entity — it measures nothing");
    }

    /// 📏️ The 1e-4 tolerance is a real threshold, not decoration: a `radius` perturbed BELOW it is
    /// accepted and the same field perturbed ABOVE it is rejected, both through real `dxf` writes.
    #[test]
    fn gate_tolerance_discriminates_at_1e_minus_4() {
        let input = base();
        let reference = project_dxf_r12(&oracle_apply_mutation(&input, &spec("set-entity", r#"{"index": 2, "entityKind": "circle", "layer": "0", "center": [600, 400, 0], "radius": 150.0}"#)).expect("reference")).expect("project reference");
        let under = project_dxf_r12(&oracle_apply_mutation(&input, &spec("set-entity", r#"{"index": 2, "entityKind": "circle", "layer": "0", "center": [600, 400, 0], "radius": 150.00001}"#)).expect("under")).expect("project under");
        let over = project_dxf_r12(&oracle_apply_mutation(&input, &spec("set-entity", r#"{"index": 2, "entityKind": "circle", "layer": "0", "center": [600, 400, 0], "radius": 150.01}"#)).expect("over")).expect("project over");
        let under_differences = compare(&reference, &under);
        let over_differences = compare(&reference, &over);
        println!("[tolerance] 1e-5 perturbation → {} difference(s); 1e-2 perturbation → {} difference(s) {:?}", under_differences.len(), over_differences.len(), over_differences.iter().map(|d| (d.path.as_str(), d.delta)).collect::<Vec<_>>());
        assert!(under_differences.is_empty(), "a 1e-5 difference must sit under the 1e-4 tolerance");
        assert_eq!(over_differences.len(), 1, "a 1e-2 difference must be reported exactly once");
    }
}
