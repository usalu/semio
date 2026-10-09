import Parser from "web-tree-sitter";
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { execFileSync } from "node:child_process";
const ticket = dirname(dirname(import.meta.path));
const owner = "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack";
const subset = `${owner}/🏅️standards/🔖️1/🪆️subsets/✳️any`;
const changed = [
  `${subset}/🧬️schema/🧮️executor/🦀️.rs`, `${subset}/🧬️schema/🧮️executor/🪜️execution/🦀️.rs`,
  `${subset}/🧬️schema/🗣️language-service/🦀️.rs`, `${subset}/🚪️io/📝️text/📸️snapshot/🦀️.rs`,
  `${subset}/🧬️schema/🧮️executor/🧪️tests/🔬️unit/🦀️.rs`, `${subset}/🧬️schema/🧮️executor/🧪️tests/🪜️resumable-query/🟦️.ts`,
  `${subset}/🧬️schema/🧮️executor/🧫️fixtures/🪜️resumable-query/🔣️.json`, `${owner}/🐚️shell/📦️packages/🦀️rust/📦️bin.rs`,
  "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs",
  "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🦀️.rs",
];
await Parser.init();
const parser = new Parser();
parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", process.cwd())), "out/tree-sitter-rust.wasm")));
const files = execFileSync("rg", ["--files", "-g", "*.rs", "✏️s/🔌️plugins/🔱️trinity"], {encoding:"utf8"}).trim().split("\n");
for (const file of files) {
  const tree = parser.parse(readFileSync(file,"utf8"))!;
  if (tree.rootNode.hasError()) throw Error(`Rust syntax invalid: ${file}`);
  tree.delete();
}
console.log(`[DEBUG] Jack final Rust syntax files=${files.length} oracle=tree-sitter`);
writeFileSync(join(ticket,"jack-query-boundary-separation.md"), `# Jack Query Domain And Text Separation

## Changes

Ten typed execution, matching, evaluation and return-construction functions moved from text snapshot IO to canonical executor schema. The typed demo fixture returned to language-service schema. Text parsing and run orchestration remain in IO, calling semantic execute directly. Old executor IO facade consumer routes were replaced directly in Jack tests/shell and Rewriting semantic/text consumers.

Typed resumable result admission now uses an explicit caller retained ownership grant, default 1048576 bytes. QueryExecution and QueryExecutionPreparation expose with_result_grant. QueryResult inline storage, result vector capacities, UTF-8 buffers, existing nested property/node/edge accounting, and retained graph/manifest metadata replace physical JSON escaping estimates and multiplier heuristics. Vector growth is admitted before reserve; actual extra allocator capacity is charged. Existing work, depth and entity limits remain enforced. Empty results also require their inline storage grant.

JSON response admission stays physical: encode_query_result_json and run_json_controlled use NativeEncodeControl and the framework bounded JSON writer. This exposes progress/cancellation and rejects before over-limit JSON allocation. The default run_json response ceiling remains 1048576 bytes.

## Neutral Regression Contract

The existing language-neutral query fixture now includes DELETE with incident-edge retirement and a MERGE no-op, validated by the existing independent SQLite oracle and Rust semantic/resumable comparison. A new resultOwnership block gives two 256-byte strings (ASCII and control characters) equal 512-byte semantic grants while imposing a separate 1536-byte text ceiling. SQLite UTF-8 blob length and JSON1 json_quote, plus JavaScript JSON, independently validate the fixture. Mounted Rust tests compare typed result admission, independent serde_json encoding, bounded physical encoding and cancellation. Existing oversized-result checks now assert retained text exceeds the semantic grant before considering encoded output.

## Executed Verification

Tree-sitter parsed all ${files.length} Trinity Rust sources after final edits without syntax errors. Source scans found no old executor::io routes or physical JSON accounting helpers in production executor sources.

Actual registered Nx oracle command: bun nx run @semio-tech/trinity-jack-rs:verify-jack-query-ownership --skip-nx-cache -- oracle (tool session 28090). It waited for shared graph construction and dispatched the native owner wrapper; completion is pending. Tests were authored before extraction but shared graph/owner waits prevented observing an initial RED before source implementation; no initial RED is claimed.

Actual registered Nx native command: bun nx run @semio-tech/trinity-jack-rs:verify-jack-query-ownership --skip-nx-cache (tool session 4887). Completion is pending. No compiled or runtime pass is asserted until those commands return.

## Changed Files

${changed.map(file => "- " + file).join("\n")}
`);
