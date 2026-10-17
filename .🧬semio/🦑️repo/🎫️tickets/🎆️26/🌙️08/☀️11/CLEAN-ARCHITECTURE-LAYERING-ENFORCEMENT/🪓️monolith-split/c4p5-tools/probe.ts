// 🔬️ c4p5 reader probe: runs the source-text readers that name the puzzle 3d split files and prints deterministic JSON, so a run before and a run after the split can be compared byte for byte.
// usage: bun probe.ts > <out.json>
import {execFileSync} from "node:child_process";
import {toolJobFactoryProofActivationScan, toolJobPluginProductionSources, toolJobProofCatalogFailures} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
import {toolJobAppOwnedRows, toolJobDispositions, toolJobProofs, toolJobStaticRows} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts";
import {interactivityPuzzleFillP4eFailures, interactivityPuzzleFillRunJobFailures, interactivityPuzzleFillSources, interactivityPuzzleFillTraceFailures} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🪣️tool-run-and-history/🟦️.ts";
import {completionRejectionOracle} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️artifact-admission-and-completion-oracles/🟦️.ts";

const root = "/Users/ueli/Documents/semio";
const out: Record<string, unknown> = {};
const is3d = (text: string) => /Puzzle3d|puzzle3d|🧊️3d/u.test(text) && /🧩️puzzle/u.test(text + " 🧩️puzzle") ;

const files = toolJobPluginProductionSources(root);
const proofs = toolJobProofs(files);
const rows = toolJobStaticRows(files);
const dispositions = toolJobDispositions(files);
const failures = toolJobProofCatalogFailures(files, rows, dispositions, proofs);
const puzzle3dKey = "🧩️puzzle/🗿️artifacts/🧊️3d";
out.toolJob = {
  sources: files.size,
  proofs: proofs.length,
  proofsPuzzle3d: proofs.filter(p => p.ownerFile.includes(puzzle3dKey)).map(p => [p.ownerFile.slice(-40), p.ownerTypeName, p.controllerId, p.factory, p.factoryType ?? null, p.toolId, p.sourceFile === p.ownerFile]),
  rowsPuzzle3d: rows.filter(r => r.file.includes(puzzle3dKey)).length,
  appOwnedPuzzle3d: toolJobAppOwnedRows(files).filter(r => r.ownerFile.includes(puzzle3dKey)).map(r => [r.toolId, r.publicationReady]),
  failuresTotal: failures.length,
  failuresPuzzle3d: failures.filter(f => /Puzzle3d|puzzle3d|🧊️3d/u.test(f)).sort(),
  failuresHash: Bun.hash(failures.slice().sort().join("\n")).toString(),
  scan: toolJobFactoryProofActivationScan(root),
};
const sources = interactivityPuzzleFillSources(root);
out.interactivity = {
  lengths: Object.fromEntries(Object.entries(sources).map(([k, v]) => [k, v.length > 0])),
  runJob: interactivityPuzzleFillRunJobFailures(sources),
  trace: interactivityPuzzleFillTraceFailures(sources),
  p4e: interactivityPuzzleFillP4eFailures(sources.precompute, sources.fill, sources.geometry),
};
try { out.completion = completionRejectionOracle(root); } catch (error) { out.completion = "FAIL " + String((error as Error).message).slice(0, 300); }
const run = (cwd: string, args: string[]): string => {
  try { return execFileSync("bun", args, {cwd, encoding: "utf8", stdio: ["ignore", "pipe", "pipe"], timeout: 900_000}).toString().slice(-600); }
  catch (error) { const e = error as {status?: number; stdout?: string; stderr?: string}; return `EXIT ${e.status}\n${String(e.stdout ?? "").slice(-300)}\n${String(e.stderr ?? "").slice(-500)}`; }
};
out.publicationAuthority3d = run(`${root}/✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript`, ["./📜️script.ts", "publication-authority-audit", "Puzzle3dPlayApp"]);
out.computeOwnership = run(`${root}/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏛️ownership/🧮️compute`, ["test", "./🧪️tests/🟦️.ts"]).replace(/\[[0-9.]+m?s\]/g, "[t]").replace(/\n.*ms\]/g, "");
console.log(JSON.stringify(out, null, 1));
