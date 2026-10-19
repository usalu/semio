import { runUnmirror } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🪞️unmirror/🏃️run/🟦️.ts";
const stage = process.argv[2]!;
const art = "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model";
const result = runUnmirror({ repoRoot: "/Users/ueli/Documents/semio", artifacts: [art], apply: false, stageDir: stage });
console.error(`[DEBUG] staged ${result.report.edits.size} regressions ${result.regressions.length} cycles ${result.cycles.length}`);
for (const r of result.regressions) console.error(`[DEBUG] reg ${r}`);
for (const r of result.report.unresolvedMirrors) console.error(`[DEBUG] unresolved ${r}`);
for (const r of result.report.invisible) console.error(`[DEBUG] invisible ${r}`);
for (const r of result.report.multiMounted.slice(0, 30)) console.error(`[DEBUG] multi ${r}`);
