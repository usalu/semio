import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { pathToFileURL } from "node:url";

//#region 🧪️CheckpointContinuation
let workspace = import.meta.dir;
while (!existsSync(join(workspace, "nx.json"))) {
  const parent = dirname(workspace);
  if (parent === workspace) throw new Error("Workspace root is not discoverable");
  workspace = parent;
}
const metadataPath = join(import.meta.dir, "../🧪️stdio-library-nextest/semio-nextest-ux87Lb/binaries-metadata.json");
const metadata = JSON.parse(readFileSync(metadataPath, "utf8"));
const binary = metadata["rust-binaries"]["semio-s-plugin-stdio"]["binary-path"] as string;
const bytes = readFileSync(binary);
const evidence = {
  sourceBoundary: "Existing executable built by stdio-library-runtime-fresh; not current-source recompilation",
  metadataPath,
  binary,
  bytes: bytes.length,
  modified: statSync(binary).mtime.toISOString(),
  sha256: createHash("sha256").update(bytes).digest("hex"),
};
mkdirSync(import.meta.dir, { recursive: true });
writeFileSync(join(import.meta.dir, "🔣️checkpoint.json"), JSON.stringify(evidence, null, 2));
console.log(`[DEBUG] ${JSON.stringify(evidence)}`);
const { runTestBudgeted } = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts")).href);
try {
  await runTestBudgeted("cargo", [
    "nextest", "run", "--binaries-metadata", metadataPath,
    "--profile", "quick", "--no-fail-fast", "--test-threads", "4",
    "--status-level", "fail", "--final-status-level", "fail",
    "--", "--skip", "long::", "--skip", "exhaustive::",
  ], { cwd: workspace, env: process.env, budgetMs: 180_000 });
} finally {
  console.log(`[DEBUG] ${JSON.stringify({ executableUnchanged: createHash("sha256").update(readFileSync(binary)).digest("hex") === evidence.sha256 })}`);
}
//#endregion 🧪️CheckpointContinuation
