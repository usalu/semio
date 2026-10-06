import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { pathToFileURL } from "node:url";
const root = process.env.SEMIO_FIXTURE_REPO_ROOT!, ticket = dirname(dirname(import.meta.dir));
const out = join(ticket, "🗑️generated/coordinator/audited-reader-runtime");
mkdirSync(out, { recursive: true });
const sub = "/🏅️standards/🔖️1/🪆️subsets/✳️any";
const exports: [string, string][] = [
 ["✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer" + sub + "/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🎚️config/🧪️tests/🔬️window-state-ownership/🟦️.ts", "testWriterWindowStateOracle"],
 ["✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation" + sub + "/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🎚️config/🧪️tests/🔬️window-config-ownership/🟦️.ts", "testEquationGraphWindowConfigOracle"],
 ["✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting" + sub + "/✏️editor/🪟️window/🎚️config/🧪️tests/🔬️window-config-ownership/🟦️.ts", "testRewritingWindowConfigOracle"],
 ["🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/👁️group-visibility/🟦️.ts", "testGroupVisibilityFixtures"],
 ["✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad" + sub + "/✏️editor/👥️presence/🧪️tests/🔬️cad-presence-retirement/🟦️.ts", "cadPresenceRetirementSelfTests"],
 ["🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🟦️.ts", "verifyPrintCommandBoundaries"],
];
const results: { name: string; passed: boolean; result?: unknown; error?: string }[] = [];
for (const [path, name] of exports) {
  try { const module = await import(pathToFileURL(join(root, path)).href); const result = await module[name](); results.push({ name, passed: true, result }); }
  catch (error) { results.push({ name, passed: false, error: String(error) }); }
  console.log("[DEBUG] " + JSON.stringify(results.at(-1)));
}
const energy = "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model" + sub;
const cases = ["600", "600FF", "610", "620", "630", "640", "650", "900", "900FF", "910", "920", "930", "940", "950"].map(name => join(root, energy + "/📚️examples/🏛️bestest-" + name + "/🧪️tests/🧩️example/🟦️.ts"));
cases.push(join(root, "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster" + sub + "/🚪️io/🧪️tests/🧩️suite/🟦️.ts"));
const child = Bun.spawn([process.execPath, "test", ...cases], { cwd: root, env: process.env, stdout: "pipe", stderr: "pipe" });
const [stdout, stderr, status] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
writeFileSync(join(out, "energy-raster.log"), stdout + stderr);
results.push({ name: "Energy and Raster canonical cases", passed: status === 0, result: status });
console.log("[DEBUG] " + JSON.stringify(results.at(-1)));
writeFileSync(join(out, "results.json"), JSON.stringify(results, null, 2) + "\n");
process.exitCode = results.every(row => row.passed) ? 0 : 1;

