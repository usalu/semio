
    /// @emoji 📋 Full store scenario catalog (`kit-store.comprehensive.compose.json`) under `assets/compose/`.
    fn kit_store_comprehensive_fixture_path() -> Option<PathBuf> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../assets/compose/kit-store.comprehensive.compose.json");
        if path.is_file() {
            Some(path)
        } else {
            None
        }
    }

    fn kit_store_substitute_fixture_vars(template: &str, store_id: &str, vars: &std::collections::HashMap<String, String>) -> String {
        let mut out = template.replace("${storeId}", store_id);
        for (k, v) in vars {
            out = out.replace(&format!("${{{}}}", k), v);
        }
        out
    }

    fn kit_store_json_pointer_get<'a>(v: &'a serde_json::Value, pointer: &str) -> Option<&'a serde_json::Value> {
        let p = if pointer.starts_with('/') {
            pointer.to_string()
        } else {
            format!("/{}", pointer.replace('.', "/"))
        };
        v.pointer(&p)
    }

    /// @emoji 🪢 Reads a fixture capture scalar from a JSON leaf (`string` or `{ "value": "…" }` IdResult shape).
    fn kit_store_capture_scalar(v: &serde_json::Value) -> Option<String> {
        v.as_str()
            .map(str::to_string)
            .or_else(|| v.get("value").and_then(|inner| inner.as_str()).map(str::to_string))
    }

    async fn kit_store_replay_golden_ops_on_graph(g: &std::sync::Arc<crate::store::Graph>, ops_json: &serde_json::Value, engine: &str) {
        let workspace_id = g.id.clone();
        let tx_id = crate::id::Id::from(ops_json["transactionId"].as_str().expect("transactionId"));
        let golden_ops = crate::kit_backbone::golden_operation_records_ref(ops_json).expect("operations|ops");
        for rec in golden_ops {
            let kind = rec["kind"].as_str().expect("operation kind");
            let input = rec.get("input").expect("input");
            match engine {
                "apply_create_fixed_piece" => {
                    if kind != "createdFixedPiece" {
                        panic!("unsupported golden operation kind: {kind}");
                    }
                    let design_id = crate::id::Id::from(input["designId"].as_str().expect("designId"));
                    let blueprint_id = crate::id::Id::from(input["blueprintId"].as_str().expect("blueprintId"));
                    let position = crate::kit_backbone::position_input_from_json(&input["position"]).expect("position from golden json");
                    let name = input.get("name").and_then(|v| v.as_str()).map(|s| s.to_string());
                    let description = input.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
                    g.apply_create_fixed_piece(workspace_id.clone(), tx_id.clone(), design_id, blueprint_id, position, name, description)
                        .await
                        .expect("apply createFixedPiece");
                }
                "kit_graph_engine" => {
                    let op = crate::kit_backbone::kit_operation_from_stored(kind, input).await.expect("kit_operation_from_stored");
                    let applied = crate::kit_graph_engine::apply_kit_operation(g, &workspace_id, &tx_id, op).await.expect("apply_kit_operation");
                    assert!(applied.created_piece.is_some(), "expected piece for {kind}");
                }
                other => panic!("unknown replay engine: {other}"),
            }
        }
    }

    async fn kit_store_assert_golden_invariants(g: &std::sync::Arc<crate::store::Graph>, exp: &serde_json::Value) {
        let inv = &exp["invariants"];
        let workspace_id = g.id.clone();
        let kit = g.materialized_kit_for_workspace(&workspace_id).await;
        let ds = kit.designs.read().await;
        assert_eq!(ds.len(), inv["designCount"].as_u64().expect("designCount") as usize, "designCount");
        let mut total = 0usize;
        let mut centers: Vec<[f64; 2]> = Vec::new();
        for d in ds.iter() {
            for p in d.pieces.read().await.iter() {
                total += 1;
                let guard = p.position.read().await;
                let n = guard.as_ref().expect("piece position");
                let pv = n.snapshot_input().await;
                centers.push([pv.center.u, pv.center.v]);
            }
        }
        centers.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap().then_with(|| a[1].partial_cmp(&b[1]).unwrap()));
        assert_eq!(total, inv["totalPieces"].as_u64().expect("totalPieces") as usize, "totalPieces");
        let expect_centers: Vec<[f64; 2]> = serde_json::from_value(inv["sortedPieceCenters"].clone()).expect("sortedPieceCenters shape");
        assert_eq!(centers.len(), expect_centers.len(), "centers len");
        for (got, want) in centers.iter().zip(expect_centers.iter()) {
            assert!((got[0] - want[0]).abs() < 1e-9, "center u");
            assert!((got[1] - want[1]).abs() < 1e-9, "center v");
        }
        let fp = stable_projection_fingerprint(&g.materialized_kit_for_workspace(&workspace_id).await).await;
        let exp_fp = exp["projectionFingerprint"].as_str().expect("projectionFingerprint");
        assert_eq!(fp, exp_fp, "projectionFingerprint");
    }

    async fn kit_store_run_comprehensive_graphql_step(step: &serde_json::Value, store_id: &str, vars: &mut std::collections::HashMap<String, String>, schema: &AppSchema) {
        let query = kit_store_substitute_fixture_vars(step["query"].as_str().expect("query"), store_id, vars);
        let mut request = Request::new(query);
        if let Some(v) = step.get("variables") {
            let raw = serde_json::to_string(v).expect("variables serialize");
            let substituted = kit_store_substitute_fixture_vars(&raw, store_id, vars);
            let parsed: serde_json::Value = serde_json::from_str(&substituted).expect("variables json");
            request = request.variables(Variables::from_json(parsed));
        }
        let res = schema.execute(request).await;
        assert!(res.errors.is_empty(), "step {} graphql errors: {:?}", step["id"], res.errors);
        let data = res.data.into_json().unwrap();
        if let Some(captures) = step.get("capture").and_then(|c| c.as_object()) {
            for (name, pointer) in captures {
                let node = kit_store_json_pointer_get(&data, pointer.as_str().expect("capture pointer"))
                    .unwrap_or_else(|| panic!("capture pointer {:?} in step {:?}", pointer, step["id"]));
                let got = kit_store_capture_scalar(node)
                    .unwrap_or_else(|| panic!("capture value {:?} at {:?} in step {:?}", node, pointer, step["id"]));
                vars.insert(name.clone(), got);
            }
        }
        if let Some(expect) = step.get("expect").and_then(|e| e.as_object()) {
            for (pointer, want) in expect {
                let got = kit_store_json_pointer_get(&data, pointer).expect("expect pointer");
                assert_eq!(got, want, "step {} expect {}", step["id"], pointer);
            }
        }
        if let Some(empty) = step.get("expectArrayEmpty").and_then(|a| a.as_array()) {
            for pointer in empty {
                let arr = kit_store_json_pointer_get(&data, pointer.as_str().expect("pointer"))
                    .and_then(|v| v.as_array())
                    .expect("array");
                assert!(arr.is_empty(), "step {} expected empty {}", step["id"], pointer);
            }
        }
        if let Some(min_lens) = step.get("expectArrayMinLen").and_then(|o| o.as_object()) {
            for (pointer, min) in min_lens {
                let len = kit_store_json_pointer_get(&data, pointer)
                    .and_then(|v| v.as_array())
                    .map(|a| a.len())
                    .unwrap_or(0);
                assert!(len >= min.as_u64().expect("min") as usize, "step {} min len {}", step["id"], pointer);
            }
        }
        if step.get("expectAuthoritativeDesignsHaveNoPieces").and_then(|v| v.as_bool()) == Some(true) {
            let edges = data["store"]["authoritative"]["theKit"]["kit"]["designs"]["edges"].as_array().expect("auth design edges");
            let all_empty = edges.iter().all(|e| e["node"]["pieces"]["edges"].as_array().map(|pe| pe.is_empty()).unwrap_or(true));
            assert!(all_empty, "authoritative must not mirror wip pieces");
        }
        if let Some(relay_contains) = step.get("expectRelayContains").and_then(|o| o.as_object()) {
            for (pointer, spec) in relay_contains {
                let field = spec["field"].as_str().expect("relay contains field");
                let want = spec["value"].as_str().expect("relay contains value");
                let edges = kit_store_json_pointer_get(&data, pointer)
                    .and_then(|v| v.as_array())
                    .unwrap_or_else(|| panic!("expectRelayContains edges {:?}", pointer));
                let hit = edges.iter().any(|e| e.pointer("/node").and_then(|n| n.get(field)).and_then(|v| v.as_str()) == Some(want));
                assert!(hit, "step {} expectRelayContains {:?} field={field} value={want}", step["id"], pointer);
            }
        }
    }

    fn kit_store_validate_comprehensive_fixture(fixture: &serde_json::Value) {
        assert_eq!(fixture["kind"].as_str(), Some("compose.kit_store.comprehensive"));
        assert!(fixture["schema"].as_str().is_some(), "schema");
        assert!(fixture["storeId"].as_str().is_some(), "storeId");
        let steps = fixture["steps"].as_array().expect("steps array");
        assert!(!steps.is_empty(), "steps must not be empty");
        for step in steps {
            assert!(step["id"].as_str().is_some(), "step id");
            assert!(step["kind"].as_str().is_some(), "step kind");
        }
        let coverage = fixture.get("coverage").and_then(|c| c.as_object()).expect("coverage object");
        assert!(coverage.get("reads").and_then(|v| v.as_array()).is_some(), "coverage.reads");
        assert!(coverage.get("writes").and_then(|v| v.as_array()).is_some(), "coverage.writes");
        assert!(kit_store_golden_fixture_paths().is_some(), "golden ops+expected files");
        assert!(kit_store_comprehensive_fixture_path().is_some(), "comprehensive fixture path");
    }

    async fn kit_store_replay_golden_ops_backbone(ops_json: &serde_json::Value, exp: &serde_json::Value, backbone: &str) {
        let exp_fp = exp["projectionFingerprint"].as_str().expect("projectionFingerprint");
        match backbone {
            "devJson" => {
                let dir = tempfile::tempdir().expect("temp dir");
                let path = dir.path().join("dev-kit.json");
                let g = crate::store::Graph::new().await;
                let legacy_workspace = ops_json["draftId"].as_str().expect("draftId");
                let graph_workspace = g.id.as_str().to_string();
                let mut stored = crate::kit_backbone::stored_operations_from_golden_operations_json(ops_json).expect("golden → stored operations");
                for op in &mut stored {
                    if op.workspace_id == legacy_workspace {
                        op.workspace_id = graph_workspace.clone();
                    }
                }
                let uri_full = format!("file://{}", path.display());
                let norm = crate::kit_backbone::normalize_connection_uri(&uri_full);
                let bundle = crate::kit_backbone::DevBackboneBundleDoc::from_stored_operations(&stored);
                std::fs::write(&path, serde_json::to_string_pretty(&bundle).expect("serialize kit-store bundle")).expect("write kit-store bundle");
                crate::kit_backbone::AttachedBackbone::mount_and_replay(&norm, "wip", &g).await.expect("dev json mount+replay");
                let fp = stable_projection_fingerprint(&g.materialized_kit_for_workspace(&g.id).await).await;
                assert_eq!(fp, exp_fp, "devJson backbone fingerprint");
            }
            "localDotCompose" => {
                let dir = tempfile::tempdir().expect("temp dir");
                let proj_root = dir.path().join("workspace");
                std::fs::create_dir_all(&proj_root).expect("mkdir workspace");
                let proj_canon = proj_root.canonicalize().expect("canonical workspace");
                let uri_local = format!("local://{}", proj_canon.display());
                let norm = crate::kit_backbone::normalize_connection_uri(&uri_local);
                let g_bootstrap = crate::store::Graph::new().await;
                let _bones = crate::kit_backbone::AttachedBackbone::mount_and_replay(&norm, "wip", &g_bootstrap).await.expect("bootstrap .compose layout");
                let g2 = crate::store::Graph::new().await;
                let legacy_workspace = ops_json["draftId"].as_str().expect("draftId");
                let graph_workspace = g2.id.as_str().to_string();
                let mut stored = crate::kit_backbone::stored_operations_from_golden_operations_json(ops_json).expect("golden → stored operations");
                for op in &mut stored {
                    if op.workspace_id == legacy_workspace {
                        op.workspace_id = graph_workspace.clone();
                    }
                }
                let db_path = proj_canon.join(".compose").join("wip.db");
                let conn = rusqlite::Connection::open(&db_path).expect("open wip.db");
                for operation in &stored {
                    let input_json = serde_json::to_string(&operation.input).expect("input json");
                    conn.execute(
                        "INSERT INTO _operation_log (draft_id, transaction_id, kind, input_json, kit_diff_json) VALUES (?1, ?2, ?3, ?4, ?5)",
                        rusqlite::params![operation.workspace_id, operation.transaction_id, operation.kind, input_json, operation.kit_diff.as_ref().map(|v| serde_json::to_string(v).expect("kit diff json"))],
                    )
                    .expect("insert operation entity");
                }
                drop(conn);
                crate::kit_backbone::AttachedBackbone::mount_and_replay(&norm, "wip", &g2).await.expect("replay wip.db");
                let fp = stable_projection_fingerprint(&g2.materialized_kit_for_workspace(&g2.id).await).await;
                assert_eq!(fp, exp_fp, "localDotCompose backbone fingerprint");
            }
            other => panic!("unknown backbone kind in comprehensive fixture: {other}"),
        }
    }

    async fn kit_store_run_comprehensive_fixture_steps(fixture: &serde_json::Value, rt: &std::sync::Arc<crate::worker::ParentStore>, schema: &AppSchema) {
        let store_id = fixture["storeId"].as_str().expect("storeId");
        let (ops_path, exp_path) = kit_store_golden_fixture_paths().expect("golden ops+expected beside comprehensive fixture");
        let ops_json: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(ops_path).expect("read golden ops")).expect("parse golden ops");
        let exp: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(exp_path).expect("read golden expected")).expect("parse golden expected");
        let mut vars = std::collections::HashMap::new();
        let mut replayed = false;
        for step in fixture["steps"].as_array().expect("steps") {
            match step["kind"].as_str().expect("kind") {
                "replayGoldenOps" => {
                    let engine = step.get("engine").and_then(|e| e.as_str()).unwrap_or("kit_graph_engine");
                    kit_store_replay_golden_ops_on_graph(&rt.wip_graph, &ops_json, engine).await;
                    kit_store_assert_golden_invariants(&rt.wip_graph, &exp).await;
                    replayed = true;
                }
                "assertGoldenInvariants" => {
                    assert!(replayed, "assertGoldenInvariants requires replayGoldenOps on the same store");
                    kit_store_assert_golden_invariants(&rt.wip_graph, &exp).await;
                }
                "graphql" => {
                    kit_store_run_comprehensive_graphql_step(step, store_id, &mut vars, schema).await;
                }
                "sleepMs" => {
                    let ms = step.get("ms").and_then(|m| m.as_u64()).unwrap_or(150);
                    std::thread::sleep(std::time::Duration::from_millis(ms));
                }
                other => panic!("unknown comprehensive step kind: {other}"),
            }
        }
        assert!(replayed, "comprehensive fixture must replay golden ops on wip");
        if let Some(native_steps) = fixture.get("nativeSteps").and_then(|v| v.as_array()) {
            for step in native_steps {
                match step["kind"].as_str().expect("native step kind") {
                    "replayGoldenOpsBackbone" => {
                        let backbone = step["backbone"].as_str().expect("backbone");
                        kit_store_replay_golden_ops_backbone(&ops_json, &exp, backbone).await;
                    }
                    other => panic!("unknown native comprehensive step kind: {other}"),
                }
            }
        }
    }

    #[test]
    fn kit_store_comprehensive_fixture_contract_is_valid() {
        let Some(path) = kit_store_comprehensive_fixture_path() else {
            eprintln!("[DEBUG] skip kit_store_comprehensive_fixture_contract_is_valid: missing kit-store.comprehensive.compose.json");
            return;
        };
        let fixture: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path).expect("read comprehensive fixture")).expect("parse comprehensive fixture");
        kit_store_validate_comprehensive_fixture(&fixture);
    }

    #[test]
    fn kit_store_comprehensive_fixture_all_replay_engines_match_golden() {
        block_on(async {
            let Some(path) = kit_store_comprehensive_fixture_path() else {
                eprintln!("[DEBUG] skip kit_store_comprehensive_fixture_all_replay_engines_match_golden");
                return;
            };
            let fixture: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path).expect("read comprehensive fixture")).expect("parse comprehensive fixture");
            kit_store_validate_comprehensive_fixture(&fixture);
            let (ops_path, exp_path) = kit_store_golden_fixture_paths().expect("golden paths");
            let ops_json: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(ops_path).expect("read ops")).expect("parse ops");
            let exp: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(exp_path).expect("read expected")).expect("parse expected");
            for engine in fixture["replayEngines"].as_array().expect("replayEngines") {
                let engine = engine.as_str().expect("engine name");
                let g = crate::store::Graph::new().await;
                kit_store_replay_golden_ops_on_graph(&g, &ops_json, engine).await;
                kit_store_assert_golden_invariants(&g, &exp).await;
            }
        });
    }

    #[test]
    fn kit_store_comprehensive_fixture_all_scenarios() {
        block_on(async {
            let Some(path) = kit_store_comprehensive_fixture_path() else {
                eprintln!("[DEBUG] skip kit_store_comprehensive_fixture_all_scenarios: missing kit-store.comprehensive.compose.json");
                return;
            };
            let fixture: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path).expect("read comprehensive fixture")).expect("parse comprehensive fixture");
            kit_store_validate_comprehensive_fixture(&fixture);
            let rt = crate::worker::ParentStore::spawn().await;
            let schema = crate::gql::build_schema_for(rt.clone());
            kit_store_run_comprehensive_fixture_steps(&fixture, &rt, &schema).await;
        });
    }
