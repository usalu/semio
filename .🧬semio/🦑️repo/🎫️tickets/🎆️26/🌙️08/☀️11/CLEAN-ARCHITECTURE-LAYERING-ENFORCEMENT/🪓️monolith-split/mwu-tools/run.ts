import { runUnmirror } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🪞️unmirror/🏃️run/🟦️.ts";

const [mode, tag, ...artifacts] = process.argv.slice(2);
const result = runUnmirror({ repoRoot: "/Users/ueli/Documents/semio", artifacts, excluded: ["✏️s/🔌️plugins/🏙️bim/"], apply: mode === "apply", force: process.env.FORCE === "1", stageDir: mode === "stage" ? `/Users/ueli/.cache/mwu/out-${tag}` : undefined, log: (line) => console.log(`  ${line}`) });
const r = result.report;
console.log(JSON.stringify({ targets: result.targets.length, files: r.edits.size, manifests: result.manifests.size, removedLeaves: r.removedLeaves, repointedLeaves: r.repointedLeaves, repointedChains: r.repointedChains, addedImports: r.addedImports, removedModules: r.removedModules, unresolved: r.unresolvedMirrors.length, multi: r.multiMounted.length, cycles: result.cycles.length, regressions: result.regressions.length, invisible: r.invisible.length, conflicts: result.conflicts.length, written: result.written }));
for (const line of r.unresolvedMirrors.slice(0, 40)) console.log(`UNRESOLVED ${line}`);
for (const line of r.multiMounted.slice(0, 40)) console.log(`MULTI ${line}`);
for (const line of result.regressions.slice(0, 60)) console.log(`REGRESSION ${line}`);
for (const line of r.invisible.slice(0, 40)) console.log(`INVISIBLE ${line}`);
for (const line of result.cycles.slice(0, 10)) console.log(`CYCLE ${line}`);
for (const line of result.conflicts) console.log(`CONFLICT ${line}`);
