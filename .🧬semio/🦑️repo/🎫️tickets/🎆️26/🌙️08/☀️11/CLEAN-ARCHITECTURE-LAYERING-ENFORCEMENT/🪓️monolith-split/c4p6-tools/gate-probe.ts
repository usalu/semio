import { readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { toolJobProofs, toolJobStaticRows, toolJobDispositions, toolJobAppOwnedRows, toolJobScanThenMonolithRows, toolJobProductionProtocolSource } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts";
import { toolJobProofCatalogFailures, toolJobFactoryProofActivationScan } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
import { toolJobPuzzleReservedRoutesExact, toolJobPuzzle2dReservedRoutesExact } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🛣️routes/🟦️.ts";
import { policyReadRustPolicySource } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";
import { policyReadFileSafe, policyReadModuleTreeSource, policyModuleTreeFiles, POLICY_SPLIT_SOURCE_ROOTS } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts";

const root = fileURLToPath(new URL("/Users/ueli/Documents/semio/", import.meta.url)).replace(/\/$/, "");
const SKIP = new Set(["node_modules", ".git", "target", "dist", "build", "🤖️generated", ".nx", "vendor", ".🧬semio"]);
function walk(rel: string, out: string[]): void {
  for (const entry of readdirSync(join(root, rel), { withFileTypes: true })) {
    if (SKIP.has(entry.name)) continue;
    const child = `${rel}/${entry.name}`;
    if (entry.isDirectory()) walk(child, out);
    else if (entry.isFile() && entry.name.endsWith(".rs")) out.push(child);
  }
}
const all: string[] = [];
walk("✏️s/🔌️plugins/🧩️puzzle", all);
const mounted = new Set(POLICY_SPLIT_SOURCE_ROOTS.filter((path) => path.includes("✏️s/🔌️plugins/🧩️puzzle/")).flatMap((path) => policyModuleTreeFiles(root, path).slice(1)));
const sources = new Map(all.filter((file) => !file.includes("/🧪️tests/") && !mounted.has(file)).map((file) => [file, toolJobProductionProtocolSource(POLICY_SPLIT_SOURCE_ROOTS.includes(file) ? policyReadModuleTreeSource(root, file) : policyReadFileSafe(root, file))]));
const P5 = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs";
const P2 = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs";
const isPuzzle = (file: string): boolean => file === P5 || file === P2;
const rows = toolJobStaticRows(sources);
const dispositions = toolJobDispositions(sources);
const proofs = toolJobProofs(sources);
const appOwned = toolJobAppOwnedRows(sources);
const failures = toolJobProofCatalogFailures(sources, rows, dispositions, proofs);
const plugin = policyReadFileSafe(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs");
const puzzleJobs = policyReadFileSafe(root, "✏️s/🔌️plugins/🧩️puzzle/🎮️commands/📤️jobs/🦀️.rs");
const out = {
  files: sources.size,
  puzzleSourcesPresent: [P5, P2].map((file) => sources.has(file)),
  rows5d: rows.filter((row) => row.file === P5).map((row) => `${row.id}:${row.source}`).sort(),
  rows2d: rows.filter((row) => row.file === P2).map((row) => `${row.id}:${row.source}`).sort(),
  rowsElsewhereForPuzzleChildren: rows.filter((row) => row.file.includes("🧩️puzzle/🗿️artifacts/") && (row.file.includes("🖐️5d/") || row.file.includes("◻️2d/")) && row.file.includes("✏️editor/") && !isPuzzle(row.file) && row.file.endsWith("🦀️.rs") && /✏️editor\/[^/]+\/🦀️\.rs$/.test(row.file)).map((row) => row.file),
  dispositions5d: [...dispositions].filter(([key]) => key.startsWith(P5 + "\0")).map(([key, value]) => `${key.slice(P5.length + 1)}=${value}`).sort(),
  dispositions2d: [...dispositions].filter(([key]) => key.startsWith(P2 + "\0")).map(([key, value]) => `${key.slice(P2.length + 1)}=${value}`).sort(),
  proofs: proofs.filter((proof) => isPuzzle(proof.sourceFile) || isPuzzle(proof.ownerFile)).map((proof) => `${proof.sourceFile === proof.ownerFile ? "same" : "DIFF"}|${proof.ownerFile.includes("5d") ? "5d" : "2d"}|${proof.ownerTypeName}|${proof.factory}|${proof.factoryType ?? ""}|${proof.toolId}|${proof.execution}|${proof.values.join(",")}`).sort(),
  appOwned: appOwned.filter((row) => isPuzzle(row.ownerFile)).map((row) => `${row.ownerFile === P5 ? "5d" : "2d"}|${row.factoryFile === row.ownerFile ? "same" : "DIFF"}|${row.factory}|${row.toolId}|${row.publicationLanes.join("+")}|${row.publicationReady}`).sort(),
  scanThenMonolith: toolJobScanThenMonolithRows(sources, proofs).filter((row) => isPuzzle(row.file)).map((row) => `${row.file === P5 ? "5d" : "2d"}|${row.id}|${row.helper}`),
  failuresAll: failures.length,
  failuresPuzzle: failures.filter((failure) => /uzzle|puzzle/.test(failure)).map((failure) => failure.replaceAll(root, "")),
  activation: toolJobFactoryProofActivationScan(root),
  reserved5d: toolJobPuzzleReservedRoutesExact(policyReadRustPolicySource(root, P5), plugin, puzzleJobs),
  reserved2d: toolJobPuzzle2dReservedRoutesExact(policyReadModuleTreeSource(root, P2), plugin, puzzleJobs),
};
console.log(JSON.stringify(out, null, 1));
