import Parser from "web-tree-sitter";
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { execFileSync } from "node:child_process";

const owner = "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack";
const subset = `${owner}/🏅️standards/🔖️1/🪆️subsets/✳️any`;
const executor = `${subset}/🧬️schema/🧮️executor/🦀️.rs`;
const text = `${subset}/🚪️io/📝️text/📸️snapshot/🦀️.rs`;
const language = `${subset}/🧬️schema/🗣️language-service/🦀️.rs`;
await Parser.init();
const parser = new Parser();
parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", process.cwd())), "out/tree-sitter-rust.wasm")));
const source = readFileSync(text, "utf8"), tree = parser.parse(source)!;
const semantic = tree.rootNode.namedChildren.find(node => node.type === "mod_item" && node.childForFieldName("name")?.text === "snapshot_wire_codec")!;
const functions = semantic.childForFieldName("body")!.namedChildren.filter(node => node.type === "function_item" && !["run", "run_json"].includes(node.childForFieldName("name")!.text));
const moved = functions.map(node => node.text).join("\n\n");
writeFileSync(executor, readFileSync(executor, "utf8").replace("use crate::language_service::parse;\n", "").replace("pub(crate) use execution::QUERY_OUTPUT_MAXIMUM_BYTES;\n", "") + "\n\n" + moved + "\n");
const physical = `/// ▶️ Parses and executes a text query, publishing its semantic graph effects.
pub fn run(graph: &mut crate::Graph, source: &str) -> Result<crate::ast::QueryResult, String> {
    let query = crate::language_service::parse(source)?;
    let (result, effects) = crate::executor::execute(graph, &query)?;
    crate::apply_graph_effects(graph, &effects).map_err(|error| error.to_string())?;
    Ok(result)
}

/// 📤️ Encodes a typed query result under caller-owned physical progress and byte admission.
pub fn encode_query_result_json(result: &crate::ast::QueryResult, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<String, semio_framework_value::ValueError> {
    semio_framework_pack_json::to_json_string_controlled(result, control)
}

/// ▶️ Runs a text query and emits its response under physical JSON byte admission.
pub fn run_json_controlled(graph: &mut crate::Graph, source: &str, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<String, String> {
    let result = run(graph, source)?;
    encode_query_result_json(&result, control).map_err(|error| error.to_string())
}

/// ▶️ Executes a text query with the default response byte grant.
pub fn run_json(graph: &mut crate::Graph, source: &str) -> Result<String, String> {
    let mut progress = |_| true;
    let mut control = semio_framework_value::NativeEncodeControl::new(1_048_576, &mut progress);
    run_json_controlled(graph, source, &mut control)
}`;
let body = source.slice(0, semantic.startIndex) + physical + source.slice(semantic.endIndex);
body = body.replace("pub use snapshot_wire_codec::*;", "");
const demoTree = parser.parse(body)!, demo = demoTree.rootNode.namedChildren.find(node => node.type === "mod_item" && node.childForFieldName("name")?.text === "snapshot_wire2_codec")!;
const example = demo.childForFieldName("body")!.namedChildren.find(node => node.type === "function_item" && node.childForFieldName("name")?.text === "example_graph_snapshot")!;
writeFileSync(language, readFileSync(language, "utf8") + "\n\n/// 🧩️ Typed example graph for language service and playground consumers.\n" + example.text + "\n");
body = body.slice(0, demo.startIndex) + `/// 📤️ Encodes the typed language-service example at its physical text boundary.
pub fn example_graph_snapshot_json() -> String {
    semio_framework_pack_json::to_json_string(&crate::language_service::example_graph_snapshot())
}` + body.slice(demo.endIndex);
body = body.replace("pub use snapshot_wire2_codec::*;", "");
writeFileSync(text, body);
const files = execFileSync("rg", ["--files", "-g", "*.rs", "✏️s/🔌️plugins/🔱️trinity"], {encoding:"utf8"}).trim().split("\n");
for (const file of files) {
  const old = readFileSync(file, "utf8");
  const updated = old.replaceAll("executor::io::text::snapshot::execute", "executor::execute").replaceAll("executor::io::text::snapshot::run", "standards::v1::subsets::any::io::text::snapshot::run");
  if (updated !== old) writeFileSync(file, updated);
}
console.log(`[DEBUG] Jack semantic declarations restored=${functions.length + 1}`);
