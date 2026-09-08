import assert from "node:assert/strict";
import { copyFileSync, readFileSync, mkdirSync, mkdtempSync, writeFileSync, rmSync, renameSync, readdirSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { createRequire } from "node:module";
const workspace = process.cwd(), ticket = dirname(import.meta.dir);
const product = "🧰️framework/🛍️products/📓️print", modulePath = product + "/🔨️modules/🔤print-font-catalog";
const require = createRequire(import.meta.url), api = await import(join(workspace, modulePath, "🟦️.ts"));
const catalog = JSON.parse(readFileSync(join(modulePath, "🔣️.json"), "utf8"));
assert.equal(new (require("ajv/dist/2020").default)().validate(JSON.parse(readFileSync(join(modulePath, "🧬️schema.json"), "utf8")), catalog), true);
assert.equal(typeof api.stagePrintFonts, "function", "Fonts must stage authored source assets into an owned deliverable");
assert.deepEqual(api.printFontDescriptors(), catalog);
const root = mkdtempSync(join(ticket, "🗑️generated/print-fonts-"));
for (const row of catalog) {
  const path = join(product, "🖼️assets/🔤️font", row.directory, row.filename);
  mkdirSync(dirname(join(root, path)), { recursive: true }); copyFileSync(path, join(root, path));
}
await api.stagePrintFonts(root);
const output = api.printFontSearchPaths(root)[0];
assert.equal(output, join(root, product, "📦️packages/🟦️typescript/dist/fonts"));
const { GlobalFonts } = require("@napi-rs/canvas");
for (const row of catalog) {
  const path = join(output, row.texFilename);
  assert.deepEqual(readFileSync(path), readFileSync(join(root, product, "🖼️assets/🔤️font", row.directory, row.filename)));
  const key = GlobalFonts.registerFromPath(path, "nx-font-fixture-" + row.family); assert.ok(key, row.family); GlobalFonts.remove(key);
}
writeFileSync(join(output, "stale.ttf"), "stale");
await api.stagePrintFonts(root);
assert.deepEqual(JSON.parse(readFileSync(join(output, ".nx-artifact.json"), "utf8")).files, catalog.map((row: any) => row.texFilename).sort());
const prior = readFileSync(join(output, catalog[0].texFilename));
writeFileSync(join(root, product, "🖼️assets/🔤️font", catalog[0].directory, catalog[0].filename), "invalid");
await assert.rejects(async () => api.stagePrintFonts(root), /TTF/);
assert.deepEqual(readFileSync(join(output, catalog[0].texFilename)), prior);
rmSync(root, { recursive: true });
console.log("[DEBUG] Print font source staging, independent native font loading, stale replacement and failed-publication preservation PASS");

if (process.argv.includes("cache")) {
  const output = api.printFontSearchPaths(workspace)[0];
  const marker = JSON.parse(readFileSync(join(output, ".nx-artifact.json"), "utf8"));
  assert.equal(marker.owner, "@semio-tech/print:fonts");
  const files = new Map(readdirSync(output).map((name) => [name, readFileSync(join(output, name))]));
  const backup = join(ticket, "🗑️generated/print-fonts-restore-backup");
  assert.equal(existsSync(backup), false);
  renameSync(output, backup);
  try {
    const child = Bun.spawn(["bun", "nx", "run", "@semio-tech/print:fonts", "--output-style=stream"], { cwd: workspace, env: { ...process.env, SEMIO_TICKET_DIR: ticket }, stdout: "pipe", stderr: "pipe" });
    const [out, err, status] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
    writeFileSync(join(ticket, "🗑️generated/print-fonts-restoration-nx.log"), out + err);
    assert.equal(status, 0, out + err);
    assert.match(out, /cache instead of running the command for 1 out of 1 tasks/);
    for (const [name, bytes] of files) assert.deepEqual(readFileSync(join(output, name)), bytes);
    for (const row of catalog) { const key = GlobalFonts.registerFromPath(join(output, row.texFilename), "restored-" + row.family); assert.ok(key); GlobalFonts.remove(key); }
    rmSync(backup, { recursive: true });
    console.log("[DEBUG] Actual Print Nx font deliverables restored byte-identically from cache and loaded by native font consumer PASS");
  } catch (error) {
    rmSync(output, { recursive: true, force: true }); renameSync(backup, output); throw error;
  }
}
