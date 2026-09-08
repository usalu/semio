import assert from "node:assert/strict";
import { readFileSync, mkdirSync, mkdtempSync, rmSync, existsSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { createRequire } from "node:module";
const root = process.cwd(), ticket = dirname(dirname(import.meta.dir)), modulePath = "🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📚️bundle";
const require = createRequire(import.meta.url), catalog = JSON.parse(readFileSync(join(root, modulePath, "🔒️dependencies.json"), "utf8"));
assert.equal(new (require("ajv/dist/2020").default)().validate(JSON.parse(readFileSync(join(root, modulePath, "🧬️schema.json"), "utf8")), catalog), true);
const api = await import(join(root, modulePath, "📜️script.ts"));
const output = mkdtempSync(join(ticket, "🗑️generated/print-bundle-contract-"));
const original = globalThis.fetch;
try {
  const controller = new AbortController(); controller.abort();
  await assert.rejects(() => api.preparePrintBundle(output, controller.signal), /abort/i);
  let requests = 0;
  globalThis.fetch = (async () => { requests++; return new Response("bad content", { status: 206, headers: { "content-range": "bytes 0-10/2881562112" } }); }) as typeof fetch;
  await assert.rejects(() => api.preparePrintBundle(output), /range|checksum|length/i);
  assert.ok(requests > 0);
  assert.equal(existsSync(api.printBundleDirectory(output)), false);
  const parent = dirname(api.printBundleDirectory(output));
  if (existsSync(parent)) assert.equal(readdirSync(parent).some(name => name.startsWith(".prepare-")), false);
} finally { globalThis.fetch = original; rmSync(output, { recursive: true }); }
console.log("[DEBUG] Print bundle schema, pre-cancellation and corrupt-range atomic failure PASS");
