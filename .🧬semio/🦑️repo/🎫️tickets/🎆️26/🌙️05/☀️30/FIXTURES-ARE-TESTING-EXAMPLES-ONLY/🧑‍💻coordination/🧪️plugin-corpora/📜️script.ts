import assert from "node:assert/strict";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import Ajv from "ajv";
const root = process.env.SEMIO_FIXTURE_REPO_ROOT!;
const ticket = dirname(dirname(import.meta.dir)), out = join(ticket, "🗑️generated/coordinator/plugin-corpus-runtime");
mkdirSync(out, { recursive: true });
const routes = [
  ["💠️lowpoly", "test"],
  ["🧱️block", "publication-authority-audit"],
  ["🖍️draw", "publication-authority-audit"],
  ["➗️mathematical", "publication-authority-audit"],
  ["🧩️puzzle", "publication-authority-audit"],
  ["📐️cad", "retained-audit"],
];
const results: { plugin: string; command: string; status: number; timedOut: boolean }[] = [];
let next = 0;
await Promise.all(Array.from({ length: 2 }, async () => {
  for (;;) {
    const row = routes[next++];
    if (!row) return;
    const [plugin, command] = row;
    const file = join(root, "✏️s/🔌️plugins", plugin!, "📦️packages/🟦️typescript/📜️script.ts");
    const child = Bun.spawn([process.execPath, file, command!], { cwd: root, env: process.env, stdout: "pipe", stderr: "pipe" });
    let timedOut = false;
    const timer = setTimeout(() => { timedOut = true; child.kill(); }, 90000);
    const [stdout, stderr, status] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
    clearTimeout(timer);
    writeFileSync(join(out, plugin + ".log"), stdout + stderr);
    results.push({ plugin: plugin!, command: command!, status, timedOut });
    console.log("[DEBUG] " + JSON.stringify(results.at(-1)));
  }
}));
const raster = join(root, "✏️s/🔌️plugins/🖨️raster");
const module = JSON.parse(readFileSync(join(raster, "🧬️schema/🔣️.json"), "utf8"));
const fixture = JSON.parse(readFileSync(join(raster, "🧫️fixtures/🔏️publication-authority/🔣️.json"), "utf8"));
const ajv = new Ajv({ strict: true, allErrors: true });
ajv.addKeyword({ keyword: "x-semio-formats", metaSchema: { type: "array", items: { type: "string" } } });
ajv.addSchema(module);
const validate = ajv.compile({ $ref: module.$id + "#/$defs/RasterPublicationAuthority" });
assert(validate(fixture), JSON.stringify(validate.errors));
for (const owner of fixture.owners) assert(readFileSync(join(raster, owner.source), "utf8").includes(owner.owner));
console.log("[DEBUG] Raster retained fixture independently validated with Ajv and current owner source");
writeFileSync(join(out, "results.json"), JSON.stringify(results, null, 2) + "\n");
process.exitCode = results.every(row => row.status === 0) ? 0 : 1;

