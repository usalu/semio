// 🔎 Scratch: reproduce CheckScript's staleness diff, but print WHICH entries differ instead of just
// bailing, so we can tell whether norm's Shape V2 retrofit caused the staleness or it's pre-existing
// repo-wide concurrent-session drift (per master ticket's documented pattern).
import { generatePluginRegistry, generatePlaygroundRegistry } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️implementations/🟦️typescript/📇️registry/📜️script.ts";
import { readFileSync } from "fs";

const repoRoot = "/Users/ueli/Documents/semio";
const entries = generatePluginRegistry(repoRoot);
const playgrounds = generatePlaygroundRegistry(repoRoot);

const expectedPluginsJson = `${JSON.stringify(entries, null, 2)}\n`;
const outDir = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️implementations/🟦️typescript/📇️registry/🤖️generated/";
const actualPluginsJson = readFileSync(outDir + "🔣️plugins.json", "utf8");

const expectedLines = expectedPluginsJson.split("\n");
const actualLines = actualPluginsJson.split("\n");
console.log(`expected entries: ${entries.length}, expected lines: ${expectedLines.length}, actual lines: ${actualLines.length}`);

const normEntry = entries.find((e: any) => e.id === "norm" || e.pluginId === "norm" || JSON.stringify(e).includes("norm"));
console.log("norm entry (computed now):", JSON.stringify(normEntry, null, 2));

// crude line diff
let firstDiff = -1;
for (let i = 0; i < Math.max(expectedLines.length, actualLines.length); i++) {
  if (expectedLines[i] !== actualLines[i]) { firstDiff = i; break; }
}
console.log("first differing line index:", firstDiff);
if (firstDiff >= 0) {
  console.log("expected:", expectedLines.slice(Math.max(0, firstDiff - 2), firstDiff + 3));
  console.log("actual:  ", actualLines.slice(Math.max(0, firstDiff - 2), firstDiff + 3));
}
