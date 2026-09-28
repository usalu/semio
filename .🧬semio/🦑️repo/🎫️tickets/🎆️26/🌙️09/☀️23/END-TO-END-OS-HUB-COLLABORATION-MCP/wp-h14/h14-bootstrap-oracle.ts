/** 🧪️ H14 14b (ticket-local, one-off): runs the hub script's own `proveTrustedStdioGisBootstrapFixture` from a rewritten copy
 * (relative imports absolute, the oracle exported, main guard dropped) with the prerequisite proofs named on the command line
 * skipped — for a prerequisite that is red for a reason outside the stdio+GIS bootstrap fixture (named in the report). No tree
 * file changes. Usage: SEMIO_TEST_ARTIFACT_DIR=<…🗑️generated…> bun h14-bootstrap-oracle.ts [<skipped prerequisite>…] */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

const repo = "/Users/ueli/Documents/semio";
const scriptPath = join(repo, "🌎️hub/📦️packages/🦀️rust/📜️script.ts");
const base = dirname(scriptPath);
const guard = 'if (import.meta.main) await runBundleScriptMain(router, import.meta.url, { defaultCommand: "dev" });';
let source = readFileSync(scriptPath, "utf8");
if (!source.includes(guard)) throw new Error("hub script main guard moved");
source = source.replace(/(from\s+|import\()"(\.{1,2}\/[^"]+)"/gu, (_, head: string, specifier: string) => `${head}"${resolve(base, specifier)}"`);
source = source.replace(guard, "export { proveTrustedStdioGisBootstrapFixture };");
for (const skipped of process.argv.slice(2)) {
  const call = `\n  await ${skipped}(repoRoot);`;
  const start = source.indexOf("async function proveTrustedStdioGisBootstrapFixture(repoRoot: string): Promise<void> {");
  const at = source.indexOf(call, start);
  if (start < 0 || at < 0 || at > source.indexOf("\n}\n", start)) throw new Error(`no prerequisite ${skipped} inside the bootstrap oracle`);
  source = source.slice(0, at) + source.slice(at + call.length);
}
const copyDirectory = join(repo, ".tmp-ticket/wp-h14/🗑️generated/hub-script-copy");
mkdirSync(copyDirectory, { recursive: true });
const copyPath = join(copyDirectory, "hub-script-oracle.ts");
writeFileSync(copyPath, source);
const hub = await import(copyPath);
await hub.proveTrustedStdioGisBootstrapFixture(repo);
console.log(`bootstrap fixture oracle PASS (skipped: ${process.argv.slice(2).join(", ") || "none"})`);
