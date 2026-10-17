// 🔬️ c4p44 reader probe: module-tree text of the 7 split files on the live tree (after install) and the needles the wasm unit test and the interactivity audit look for.
import {readModuleTreeSource, listModuleTree} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const M = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/";
const files = ["🏠️workspace/📨️plugin-artifact-channel", "🏠️workspace/🛂️gateway-backend", "💡️inference", "🗂️catalog", "🧬️schema", "🧵️bridge/🪢️session"];
for (const f of files) {
  const root = M + f + "/🦀️.rs";
  const text = readModuleTreeSource(root);
  const tree = listModuleTree(root);
  console.log(JSON.stringify({file: f, files: tree.length, lines: text.split("\n").length}));
}
const inf = readModuleTreeSource(M + "💡️inference/🦀️.rs");
for (const n of ["fn inference_original_retained(", "let authority=inference_original_turn(arguments)?;", "fn inference_original_turn(", "turn: authority.turn"]) console.log("needle", n, inf.includes(n));
console.log("absent", !inf.includes("const INFERENCE_MEMORY_POLICY:"), !inf.includes("map_or_else(mint_inference_request_id, str::to_string),"));
