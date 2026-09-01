// Row-A verification for the two `inspectRustNonRepoJoinBaseSpans` fixes (🔍️discovery/🟦️component.ts):
// (1) the let-shadowing evaluation-order bug — `bindings.delete(name)` ran BEFORE evaluating the RHS
//     in both `visit`'s flat let-handling and `bodyReturnsNonRepo`'s helper-hop let-handling, so
//     `let out_dir = PathBuf::from(out_dir)` (rebinding a name using its own prior value) could never
//     prove the RHS non-repo, because the identifier lookup for the shadowed `out_dir` had already
//     been deleted from `bindings` by the time `rootEnd` ran; (2) a new suffix-matched rule for
//     `<any path>::test_support::tempdir()`, a project-owned wrapper (🏪️store/🦀️component.rs's
//     `pub fn tempdir()`) that itself is exactly `std::env::temp_dir().join(...)`.
//
// This is a live-file, current-tree check (not a git-HEAD diff): 🔍️discovery/🟦️component.ts is a
// shared file under concurrent edit by other sessions this ticket, so comparing against git HEAD
// drifted mid-run. Each row below was independently confirmed fail-before via the ticket's own
// unresolved census (rust-path-join-unproven at these exact lines) before this fix landed.
import { readFileSync } from "node:fs";
import { inspectRustNonRepoJoinBaseSpans } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts";

const root = "/Users/ueli/Documents/semio";

function suppressedLines(path: string): Set<number> {
  const content = readFileSync(`${root}/${path}`, "utf8");
  const rows = inspectRustNonRepoJoinBaseSpans(content);
  const lines = content.split("\n");
  let acc = 0;
  const lineStarts = lines.map((line) => { const start = acc; acc += line.length + 1; return start; });
  const out = new Set<number>();
  for (const offset of rows) {
    let line = 0;
    for (let i = 0; i < lineStarts.length; i++) if (lineStarts[i] <= offset) line = i + 1; else break;
    out.add(line);
  }
  return out;
}

const targets: { readonly path: string; readonly newlyClearedLines: readonly number[]; readonly stillUnresolvedLines: readonly number[] }[] = [
  { path: ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/m0-describe-fixture/main.rs", newlyClearedLines: [53, 54], stillUnresolvedLines: [] },
  { path: ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/m3-describe-fixture/main.rs", newlyClearedLines: [49, 50], stillUnresolvedLines: [] },
  { path: "🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️bin.rs", newlyClearedLines: [140], stillUnresolvedLines: [] },
  { path: "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📇️registry/🦀️component.rs", newlyClearedLines: [162], stillUnresolvedLines: [52] },
  { path: "🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs", newlyClearedLines: [], stillUnresolvedLines: [2787] },
  { path: "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs", newlyClearedLines: [], stillUnresolvedLines: [344, 351, 1900] },
  { path: ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w18-mutation-fixture-completeness/🏗️vector-converter/src/main.rs", newlyClearedLines: [], stillUnresolvedLines: [146, 155] },
];

let failures = 0;
for (const target of targets) {
  const suppressed = suppressedLines(target.path);
  console.log("\n===", target.path);
  for (const line of target.newlyClearedLines) {
    const ok = suppressed.has(line);
    console.log(`  [${ok ? "PASS" : "FAIL"}] line ${line}: newly suppressed (proven non-repo)=${ok}`);
    if (!ok) failures++;
  }
  for (const line of target.stillUnresolvedLines) {
    const ok = !suppressed.has(line);
    console.log(`  [${ok ? "PASS" : "FAIL"}] line ${line}: correctly still unresolved=${ok}`);
    if (!ok) failures++;
  }
}
console.log("\n\nTOTAL FAILURES:", failures);
process.exit(failures > 0 ? 1 : 0);
