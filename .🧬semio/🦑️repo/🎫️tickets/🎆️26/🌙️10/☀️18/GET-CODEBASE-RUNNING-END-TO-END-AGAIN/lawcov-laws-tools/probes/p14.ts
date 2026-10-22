import { mkdirSync, mkdtempSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { policyTaxonomyBarrelShapeBreaches } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🗺️taxonomy-shape/🧱️lib-and-barrel-shape/🟦️.ts";
const root = mkdtempSync(join(realpathSync(tmpdir()), "p14-"));
const big = Array.from({ length: 160 }, (_, i) => `const v${i} = ${i};`).join("\n") + "\n";
const files: Record<string, string> = {
  "🧰️framework/🔨️modules/🔸️b/📦️packages/🟦️typescript/package.json": '{"name":"b","semio":{"role":"library"}}\n',
  "🧰️framework/🔨️modules/🔸️b/📦️packages/🟦️typescript/🟦️.ts": big,
};
for (const [p, t] of Object.entries(files)) { mkdirSync(dirname(join(root, p)), { recursive: true }); writeFileSync(join(root, p), t); }
try { console.log(JSON.stringify(policyTaxonomyBarrelShapeBreaches(root).map((b) => [b.id, b.scope, b.priority]))); } catch (e) { console.log("ERR", String(e).slice(0, 300)); }
rmSync(root, { recursive: true, force: true });
