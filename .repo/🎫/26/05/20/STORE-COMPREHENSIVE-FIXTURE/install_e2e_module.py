"""Insert kit_store_comprehensive_e2e module and slim tests mod."""
from pathlib import Path

lib = Path(r"c:\git\compose\compose\client\lib\rs\lib.rs")
text = lib.read_text(encoding="utf-8")
raw = Path(__file__).with_name("comprehensive_tests_snippet.rs").read_text(encoding="utf-8")
snippet = raw.split("    #[test]")[0].strip()

golden_paths = '''
    /// @emoji 📎 US-001 replay fixtures (`kit-store.golden.*`) under `compose/assets/compose/`.
    pub fn kit_store_golden_fixture_paths() -> Option<(PathBuf, PathBuf)> {
        let base = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../assets/compose");
        let ops = base.join("kit-store.golden.ops.compose.json");
        let exp = base.join("kit-store.golden.expected.compose.json");
        if ops.is_file() && exp.is_file() {
            Some((ops, exp))
        } else {
            None
        }
    }

'''

body = golden_paths + snippet
body = body.replace("fn kit_store_", "pub fn kit_store_")
body = body.replace("pub async fn kit_store_", "pub async fn kit_store_")
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

/// @emoji 📋 In-process runner for `kit-store.comprehensive.compose.json` (native hosts + compose-store E2E).
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
        let rt = Arc::new(crate::worker::ParentStore::spawn().await);
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

marker = "//#region 🧪 tests"
if "pub mod kit_store_comprehensive_e2e" in text:
    print("module already present")
else:
    text = text.replace(marker, mod + marker, 1)

# remove duplicate block in tests between golden_fixture_paths closing and schema_matches
start = text.index("    /// @emoji 📋 Full store scenario catalog")
end = text.index("    /// @emoji 📜 gql::sdl() is code-first")
replacement = """    use crate::kit_store_comprehensive_e2e::kit_store_comprehensive_fixture_path;

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
            crate::kit_store_comprehensive_e2e::run_all_replay_engines(&fixture).await;
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
            crate::kit_store_comprehensive_e2e::run_in_process(&fixture).await;
        });
    }

    """
text = text[:start] + replacement + text[end:]
lib.write_text(text, encoding="utf-8")
print("installed e2e module")
