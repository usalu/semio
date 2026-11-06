/** 🔬️ Records the actual canonical runtime Source census without changing its admission policy. */
import { readFileSync, mkdirSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const ticket = dirname(fileURLToPath(import.meta.url));
const root = resolve(ticket, "../../../../../../..");
const reference = join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END/build-source-boundary-cases.json");
const fixture = JSON.parse(readFileSync(reference, "utf8"));
const selector = await import(join(root, fixture.runtimeSelection.owner));
const selected = selector.repositoryCommandKernelV1(fixture.runtimeSelection.args, fixture.runtimeSelection.environment);
if (selected.kind !== "runtime") throw Error("Canonical runtime entry required");
const entry = relative(root, selected.entry).replaceAll("\\", "/");
const authorizer = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts"));
const controller = new AbortController();
const deadline = setTimeout(() => controller.abort(Error("Source census deadline")), 120000);
const generated = join(ticket, "🗑️generated");
mkdirSync(generated, { recursive: true });
try {
  const captured = await authorizer.captureNxAuthorizerSourcesV1(root, entry, controller.signal, { installed: true });
  const sources = [...authorizer.nxAuthorizerCapturedBytesV1(captured)].map(([path, bytes]) => ({ path, bytes: bytes.byteLength }));
  const result = { entry, admitted: true, files: sources.length, bytes: sources.reduce((sum, row) => sum + row.bytes, 0), sources };
  writeFileSync(join(generated, "closure-census.json"), JSON.stringify(result, null, 2) + "\n");
  console.log("[DEBUG] Source admitted", result.files, result.bytes);
} catch (error) {
  const result = { entry, admitted: false, error: String(error), diagnostic: error.diagnostic ?? null, sources: error.sourceCensus ?? [] };
  writeFileSync(join(generated, "closure-census.json"), JSON.stringify(result, null, 2) + "\n");
  console.log("[DEBUG] Source refused", result.error);
  console.log(JSON.stringify(result.diagnostic));
  process.exitCode = 1;
} finally {
  clearTimeout(deadline);
}
