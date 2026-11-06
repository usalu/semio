/** 🔬️ Observes the actual bounded Windows host-memory command without changing capacity policy. */
import { resolve } from "node:path";
import { mkdirSync, writeFileSync } from "node:fs";
import { fileURLToPath, pathToFileURL } from "node:url";
if (process.platform !== "win32") { console.log("[DEBUG] Windows-only observer skipped on " + process.platform); process.exit(0); }
const root = fileURLToPath(new URL("../../../../../../../", import.meta.url));
const owner = resolve(root, "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity");
const { runHostCommandV1, parseProcessTableV1 } = await import(pathToFileURL(resolve(owner, "📡️sampling/🟦️.ts")).href);
const schema = await Bun.file(resolve(owner, "🧬️schema/🔣️.json")).json();
const source = schema.definitions.Observers.const.platforms.win32.footprint;
const started = Date.now();
let result;
try {
  const text = await runHostCommandV1(source.command, source.arguments);
  result = { ok: true, elapsedMs: Date.now() - started, rows: parseProcessTableV1(text).length };
} catch (error) {
  result = { ok: false, elapsedMs: Date.now() - started, code: error.code, signal: error.signal, killed: error.killed, name: error.name, message: String(error.message).slice(0, 1200), stderr: String(error.stderr ?? "").slice(0, 1200) };
}
mkdirSync(new URL("./🗑️generated/", import.meta.url), { recursive: true });
writeFileSync(new URL("./🗑️generated/windows-memory-probe.json", import.meta.url), JSON.stringify(result, null, 2) + "\n");
console.log("[DEBUG] " + JSON.stringify(result));