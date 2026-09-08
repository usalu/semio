import assert from "node:assert/strict";
import { readFileSync, writeFileSync, readdirSync, renameSync, rmSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { createRequire } from "node:module";
const root = process.cwd(), ticket = dirname(dirname(import.meta.dir));
const product = "🧰️framework/🛍️products/📓️print";
const api = await import(join(root, product, "🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🟦️.ts"));
const directory = api.printDocumentOutputDirectory("viz-api", root);
const before = new Map(readdirSync(directory).map(name => [name, readFileSync(join(directory, name))]));
assert.equal(JSON.parse(before.get(".nx-artifact.json")!.toString()).owner, "@semio-tech/print:build-viz-api");
async function run(name: string): Promise<string> {
  const child = Bun.spawn(["bun", "nx", "run", "@semio-tech/print:build-viz-api", "--output-style=stream"], { cwd: root, env: { ...process.env, SEMIO_TICKET_DIR: ticket }, stdout: "pipe", stderr: "pipe" });
  const [out, err, status] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
  writeFileSync(join(ticket, `🗑️generated/print-document-${name}.log`), out + err);
  assert.equal(status, 0, out + err);
  return out;
}
await run("cache-cold");
for (const [name, bytes] of before) assert.deepEqual(readFileSync(join(directory, name)), bytes, `Native repeat bytes: ${name}`);
console.log("[DEBUG] Repeated build outputs are byte-identical PASS");
const backup = join(ticket, "🗑️generated/print-document-restore-backup");
assert.equal(existsSync(backup), false);
renameSync(directory, backup);
try {
  const restored = await run("cache-restored");
  assert.match(restored, /@semio-tech\/print:build-viz-api  \[local cache\]/);
  assert.match(restored, /cache instead of running the command for 3 out of 5 tasks/);
  for (const [name, bytes] of before) assert.deepEqual(readFileSync(join(directory, name)), bytes, `Restored bytes: ${name}`);
  const require = createRequire(join(root, "node_modules/pdfjs-dist/legacy/build/pdf.mjs")), canvas = require("@napi-rs/canvas");
  (globalThis as any).DOMMatrix ??= canvas.DOMMatrix;
  const { getDocument } = await import("pdfjs-dist/legacy/build/pdf.mjs");
  const contract = JSON.parse(readFileSync(join(root, product, "🎮️commands/🧪️print-pipeline-verification/🧪️tests/🧫️merge-contract.json"), "utf8"));
  for (const name of before.keys()) {
    if (!name.endsWith(".pdf")) continue;
    const document = await getDocument({ data: new Uint8Array(readFileSync(join(directory, name))) }).promise;
    try {
      const texts: string[] = [];
      for (let index = 1; index <= document.numPages; index++) {
        const page = await document.getPage(index);
        texts.push((await page.getTextContent()).items.map(item => "str" in item ? item.str : "").join(" "));
      }
      for (const text of contract.vizText) assert.ok(texts.join(" ").includes(text), `${name}: ${text}`);
    } finally { await document.destroy(); }
  }
  rmSync(backup, { recursive: true });
  console.log("[DEBUG] Actual Nx PDF restoration, byte identity and independent PDF.js text consumption PASS");
} catch (error) {
  rmSync(directory, { recursive: true, force: true }); renameSync(backup, directory); throw error;
}
