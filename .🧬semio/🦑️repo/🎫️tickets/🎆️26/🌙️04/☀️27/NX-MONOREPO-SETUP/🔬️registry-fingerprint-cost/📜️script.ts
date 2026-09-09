import { readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { createHash } from "node:crypto";

const start = performance.now();
const { loadCatalogTaxonomy, registryCatalogInputView, registryCatalogInputPaths } = await import("../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts");
const imported = performance.now(), taxonomy = loadCatalogTaxonomy(), loaded = performance.now();
const view = registryCatalogInputView(process.cwd(), taxonomy), viewed = performance.now();
const paths = registryCatalogInputPaths(process.cwd(), taxonomy, view), discovered = performance.now(), hash = createHash("sha256");
let bytes = 0;
for (const path of paths) {
  const kind = view.kind(path); if (kind === "symlink") throw new Error(`Symlink input ${path}`);
  const content = kind === "file" ? readFileSync(join(process.cwd(), path)) : Buffer.alloc(0);
  hash.update(JSON.stringify([path, kind, content.byteLength]) + "\n").update(content); bytes += content.byteLength;
}
const hashed = performance.now();
const result = { importMs: imported - start, taxonomyMs: loaded - imported, viewMs: viewed - loaded, discoveryMs: discovered - viewed, hashMs: hashed - discovered, totalMs: hashed - start, inputs: paths.length, bytes, digest: hash.digest("hex") };
writeFileSync(resolve(import.meta.dir, "../🗑️generated/registry-fingerprint-cost.json"), JSON.stringify(result, null, 2)); console.log("[DEBUG] Registry fingerprint costs", JSON.stringify(result));
