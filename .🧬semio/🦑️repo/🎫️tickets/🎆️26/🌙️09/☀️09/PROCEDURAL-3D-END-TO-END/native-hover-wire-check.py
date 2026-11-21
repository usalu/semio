from pathlib import Path
import re
import subprocess
import sys
import os

root = Path.cwd()
workspace = next(root.glob("🧰️framework/*/💻️os/*/♾️infinite/🌍️world"))
fixture = next((root / "🧰️framework").rglob("🎯️grouped-hover.json"))
bridge = (next(workspace.rglob("🎞️scene-bridge")) / "🦀️.rs").read_text()
cursors = (workspace / "🧲️cursors" / "🦀️.rs").read_text()
laws = (next(workspace.rglob("🖱️pointer-gestures")) / "🦀️.rs").read_text()
scene_record = bridge[bridge.index("#[derive(Clone, Debug, Deserialize)]", bridge.index("/// 🧬️ The exact analytic")):bridge.index("/// 🎥️ ")]
wire = cursors[cursors.index("pub struct WorldInteractionText"):cursors.index("pub fn world_hover_action_string_bytes")]
wire = wire.replace("semio_ui_action::ACTION_STRING_BYTE_CAPACITY", "ACTION_STRING_BYTE_CAPACITY")
scene_record = re.sub(r"(?m)^#\[derive\(semio_framework_value_derive::RetireOwned\)\]\n", "", scene_record)
capacity_test = laws[laws.index("#[test]\nfn authored_hover_groups_obey"):laws.index("#[test]\nfn authored_hover_falls_back")]
source = """
extern crate self as semio_os_infinite_world_cursors;
use serde::Deserialize;
use std::collections::HashSet;
const WORLD_INTERACTION_ID_BYTE_CAPACITY: usize = 256;
const WORLD_INTERACTION_OBJECT_CAPACITY: usize = 1024;
const ACTION_STRING_BYTE_CAPACITY: usize = 4096;
macro_rules! fixture_text { ($path:literal) => { include_str!(env!("NATIVE_HOVER_FIXTURE")) } }
""" + wire + scene_record + capacity_test + """
#[test]
fn shared_fixture_admits_canonical_native_group_wire() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(env!("NATIVE_HOVER_FIXTURE"))).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let mut instance = case["instance"].clone();
        instance["meshId"] = "fixture mesh".into();
        let record: World3dSceneInstanceEntry = serde_json::from_value(instance.clone()).unwrap();
        let expected = &case["expected"];
        match record.hover_targets {
            Some(text) => {
                assert_eq!(text, serde_json::to_string(expected).unwrap(), "{}", case["name"]);
            }
            None => {
                let primary_id = record.interaction_id.as_deref().unwrap_or(&record.id);
                let granularity = record.interaction_granularity_id.as_deref().unwrap_or_else(|| case["defaultGranularity"].as_str().unwrap());
                assert_eq!(expected, &serde_json::json!([{"granularity": granularity, "id": primary_id}]));
            }
        }
    }
}
"""
output = Path(__file__).resolve().parent / "🗑️generated/native"
output.mkdir(parents=True, exist_ok=True)
input_rs = output / "admission.rs"
input_rs.write_text(source)
binary = output / "admission"
cache = root / ".cache/b/r/m/u/l/p9ded5f/prototype/build"
env = {**os.environ, "NATIVE_HOVER_FIXTURE": str(fixture)}
args = ["--edition=2021", "--test", str(input_rs), "-o", str(binary)]
for directory in cache.glob("*/*/out"):
    args.extend(["-L", f"dependency={directory}"])
for name, hash_id in [("serde", "607cdc1cc2557d4f"), ("serde_json", "f88d768e58264d46")]:
    output_path = cache / name / hash_id / "out"
    for suffix in ["rlib", "rmeta"]:
        args.extend(["--extern", f"{name}={output_path / f'lib{name}-{hash_id}.{suffix}'}"])
compiler = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc"]
result = subprocess.run(compiler + args, env=env)
if result.returncode == 0:
    result = subprocess.run(["repo", "tools", "exec", "--light", "python", "--", "-c", "import os, sys; os.execv(sys.argv[1], sys.argv[1:])", str(binary)])
sys.exit(result.returncode)
