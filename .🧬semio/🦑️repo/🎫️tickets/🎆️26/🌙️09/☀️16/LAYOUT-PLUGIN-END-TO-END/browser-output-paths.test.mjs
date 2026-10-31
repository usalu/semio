import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { posix, win32 } from "node:path";
import { planBrowserOutputPaths } from "./browser-output-paths.mjs";
const require = createRequire(import.meta.url);
const Ajv = require("ajv");
const schema = JSON.parse(readFileSync(new URL("./browser-output-paths.schema.json", import.meta.url)));
const validate = new Ajv({ strict: true }).compile(schema);
const fixtures = JSON.parse(readFileSync(new URL("./browser-output-paths-cases.json", import.meta.url)));
const children = ["report.json", "01-artifact-panel-baseline.png", "downloads/" + "a".repeat(36)];
for (const paths of [posix, win32]) for (const row of fixtures.cases) {
  const root = row.root ?? ("/" + (row.emojiRepeat ? "🧬".repeat(row.emojiRepeat) : "a".repeat(row.rootRepeat)));
  const raw = row.raw ?? "a".repeat(row.repeat);
  const runName = raw.replace(/[^a-zA-Z0-9_.-]/g, "-");
  const out = paths.join(root, "🗑️generated", runName);
  const units = [out, ...children.map(child => paths.join(out, child))].map(value => value.length);
  assert.equal(validate({ runName, units }), row.valid, row.name + " independent schema");
  if (row.valid) assert.equal(planBrowserOutputPaths(root, raw, children, paths).outDir, out);
  else assert.throws(() => planBrowserOutputPaths(root, raw, children, paths), /output/);
}
console.log("[DEBUG] Browser output path neutral cases passed on POSIX and Windows ports");
