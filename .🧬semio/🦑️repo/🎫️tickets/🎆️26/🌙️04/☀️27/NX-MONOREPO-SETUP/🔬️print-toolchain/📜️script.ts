import assert from "node:assert/strict";
import { readFileSync, mkdtempSync, existsSync, rmSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { createRequire } from "node:module";
const root = process.cwd(), require = createRequire(import.meta.url);
const modulePath = "🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🔧️toolchain";
const manifest = JSON.parse(readFileSync(join(root, modulePath, "🔣️.json"), "utf8"));
assert.equal(new (require("ajv/dist/2020").default)().validate(JSON.parse(readFileSync(join(root, modulePath, "🧬️schema.json"), "utf8")), manifest), true);
const api = await import(join(root, modulePath, "📜️script.ts"));
const release = JSON.parse(readFileSync(join(import.meta.dir, "🧫️release.json"), "utf8"));
for (const row of manifest.platforms) {
  assert.deepEqual(api.tectonicDistribution(row.platform, row.architecture), row);
  const asset = release.assets.find((asset: any) => asset.name === row.archive);
  assert.equal(asset.digest, "sha256:" + row.sha256);
  assert.equal(asset.browser_download_url, manifest.release + "/" + row.archive);
}
assert.throws(() => api.tectonicDistribution("unknown", "x64"), /Unsupported/);
const selected = api.tectonicDistribution(process.platform, process.arch);
assert.ok(api.tectonicBinaryPath(root).endsWith(selected.target + "/tectonic" + (process.platform === "win32" ? ".exe" : "")));
console.log("[DEBUG] Tectonic platform selection and archive identity match the published release oracle PASS");

const fixture = mkdtempSync(join(import.meta.dir, "../🗑️generated/print-toolchain-")), fetch = globalThis.fetch;
try {
  globalThis.fetch = async () => new Response("unverified bytes");
  await assert.rejects(api.prepareTectonic(fixture), /checksum/);
  assert.equal(existsSync(api.tectonicBinaryPath(fixture)), false);
  const parent = join(api.tectonicBinaryPath(fixture), "../..");
  assert.ok(readdirSync(parent).every((name) => !name.startsWith(".prepare-")));
  const controller = new AbortController(); controller.abort(new Error("fixture cancellation"));
  await assert.rejects(api.prepareTectonic(fixture, controller.signal), /fixture cancellation/);
  assert.ok(readdirSync(parent).every((name) => !name.startsWith(".prepare-")));
} finally { globalThis.fetch = fetch; rmSync(fixture, { recursive: true }); }
console.log("[DEBUG] Unverified and cancelled Tectonic acquisitions publish no executable and leave no preparation trees PASS");
