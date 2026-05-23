"""Restore lib.rs from HEAD export and add kit_store_comprehensive_e2e + tests."""
from pathlib import Path

root = Path(r"c:\git\semio")
lib = root / "semio" / "client" / "lib" / "rs" / "lib.rs"
head = lib.read_text(encoding="utf-8")
if "pub mod kit_store_comprehensive_e2e" in head:
    print("kit_store_comprehensive_e2e already present")
    raise SystemExit(0)

ticket = next(root.glob(".repo/**/STORE-COMPREHENSIVE-FIXTURE/comprehensive_tests_snippet.rs"))
raw = ticket.read_text(encoding="utf-8")
snippet = raw.split("    #[test]")[0].strip()

golden_paths = '''
    /// @emoji 📎 US-001 replay fixtures (`kit-store.golden.*`) under `semio/assets/semio/`.
    pub fn kit_store_golden_fixture_paths() -> Option<(PathBuf, PathBuf)> {
        let base = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../assets/semio");
        let ops = base.join("kit-store.golden.ops.semio.json");
        let exp = base.join("kit-store.golden.expected.semio.json");
        if ops.is_file() && exp.is_file() {
            Some((ops, exp))
        } else {
            None
        }
    }

'''

body = golden_paths + snippet
body = body.replace("\n    fn kit_store_", "\n    pub fn kit_store_")
body = body.replace(
    "let fp = stable_projection_fingerprint(&g.materialized_kit_for_workspace(&workspace_id).await).await;",
    "let kit_arc = g.materialized_kit_for_workspace(&workspace_id).await;\n        let fp = crate::kit_graph_engine::projection_fingerprint_for_kit(kit_arc.as_ref()).await;",
)
body = body.replace(
    "let fp = stable_projection_fingerprint(&g.materialized_kit_for_workspace(&g.id).await).await;",
    "let kit_arc = g.materialized_kit_for_workspace(&g.id).await;\n                let fp = crate::kit_graph_engine::projection_fingerprint_for_kit(kit_arc.as_ref()).await;",
)
body = body.replace(
    "let fp = stable_projection_fingerprint(&g2.materialized_kit_for_workspace(&g2.id).await).await;",
    "let kit_arc = g2.materialized_kit_for_workspace(&g2.id).await;\n                let fp = crate::kit_graph_engine::projection_fingerprint_for_kit(kit_arc.as_ref()).await;",
)

mod = f"""//#region 📋 kit_store_comprehensive_e2e

/// @emoji 📋 In-process runner for `kit-store.comprehensive.semio.json` (native hosts + semio-store E2E).
#[cfg(not(target_arch = "wasm32"))]
pub mod kit_store_comprehensive_e2e {{
    use std::path::Path;
    use std::path::PathBuf;
    use std::sync::Arc;

    use async_graphql::{{Request, Variables}};

    use crate::gql::{{build_schema_for, AppSchema}};

{body}

    /// @emoji 🧪 Load and validate fixture, run GraphQL steps + native backbone steps (in-process ParentStore).
    pub async fn run_in_process(fixture: &serde_json::Value) {{
        kit_store_validate_comprehensive_fixture(fixture);
        let rt = crate::worker::ParentStore::spawn().await;
        let schema = build_schema_for(rt.clone());
        kit_store_run_comprehensive_fixture_steps(fixture, &rt, &schema).await;
    }}

    /// @emoji 🧪 Every `replayEngines` entry matches golden invariants on a fresh graph.
    pub async fn run_all_replay_engines(fixture: &serde_json::Value) {{
        kit_store_validate_comprehensive_fixture(fixture);
        let (ops_path, exp_path) = kit_store_golden_fixture_paths().expect("golden paths");
        let ops_json: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(ops_path).expect("read ops")).expect("parse ops");
        let exp: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(exp_path).expect("read expected")).expect("parse expected");
        for engine in fixture["replayEngines"].as_array().expect("replayEngines") {{
            let engine = engine.as_str().expect("engine name");
            let g = crate::vcs::Graph::new().await;
            kit_store_replay_golden_ops_on_graph(&g, &ops_json, engine).await;
            kit_store_assert_golden_invariants(&g, &exp).await;
        }}
    }}
}}

//#endregion 📋 kit_store_comprehensive_e2e

"""

tests = '''
    use crate::kit_store_comprehensive_e2e::kit_store_comprehensive_fixture_path;

    #[test]
    fn kit_store_comprehensive_fixture_contract_is_valid() {
        let Some(path) = kit_store_comprehensive_fixture_path() else {
            eprintln!("[DEBUG] skip kit_store_comprehensive_fixture_contract_is_valid: missing kit-store.comprehensive.semio.json");
            return;
        };
        let fixture: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path).expect("read comprehensive fixture")).expect("parse comprehensive fixture");
        crate::kit_store_comprehensive_e2e::kit_store_validate_comprehensive_fixture(&fixture);
    }

    #[test]
    fn kit_store_comprehensive_fixture_all_replay_engines_match_golden() {
        block_on(async {
            let Some(path) = kit_store_comprehensive_fixture_path() else {
                eprintln!("[DEBUG] skip kit_store_comprehensive_fixture_all_replay_engines_match_golden");
                return;
            };
            let fixture: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path).expect("read comprehensive fixture")).expect("parse comprehensive fixture");
            crate::kit_store_comprehensive_e2e::run_all_replay_engines(&fixture).await;
        });
    }

    #[test]
    fn kit_store_comprehensive_fixture_all_scenarios() {
        block_on(async {
            let Some(path) = kit_store_comprehensive_fixture_path() else {
                eprintln!("[DEBUG] skip kit_store_comprehensive_fixture_all_scenarios: missing kit-store.comprehensive.semio.json");
                return;
            };
            let fixture: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path).expect("read comprehensive fixture")).expect("parse comprehensive fixture");
            crate::kit_store_comprehensive_e2e::run_in_process(&fixture).await;
        });
    }

'''

anchor = head.index("pub mod wasm_bridge")
re = anchor
while True:
    re = head.index("//#endregion", re + 1)
    line_end = head.index("\n", re)
    if "wasm_bridge" in head[re:line_end]:
        break
head = head[: line_end + 1] + "\n" + mod + head[line_end + 1 :]

anchor = "    /// @emoji"
idx = head.find("fn schema_matches_target_graphql_file()")
idx = head.rfind(anchor, 0, idx)
head = head[:idx] + tests + head[idx:]

lib.write_text(head, encoding="utf-8")
print("wrote", lib, len(head.splitlines()), "lines")
