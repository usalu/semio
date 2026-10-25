import { readFileSync, writeFileSync } from "node:fs";
import { languageLawAllowlistV1, readLanguageLawSourcesV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🗣️language-catalogs/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const file = root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🗣️language-catalogs/🔒️allowlist/🔣️.json";
const old = JSON.parse(readFileSync(file, "utf8")) as { version: 1; ceiling: number; files: Record<string, number>; defaultsCeiling: number; defaults: Record<string, number> };
const cut = languageLawAllowlistV1(readLanguageLawSourcesV1(root));
const shrink = (before: Record<string, number>, now: Record<string, number>) => {
  const out: Record<string, number> = {};
  for (const [path, allowed] of Object.entries(before)) {
    const actual = now[path] ?? 0;
    if (actual > 0) out[path] = Math.min(allowed, actual);
  }
  return out;
};
const files = shrink(old.files, cut.files), defaults = shrink(old.defaults, cut.defaults);
const sum = (rows: Record<string, number>) => Object.values(rows).reduce((a, b) => a + b, 0);
const next = { version: 1, ceiling: Math.min(old.ceiling, sum(files)), files, defaultsCeiling: Math.min(old.defaultsCeiling, sum(defaults)), defaults };
writeFileSync(file, JSON.stringify(next, null, 1) + "\n");
console.log(`files ${Object.keys(old.files).length}->${Object.keys(files).length} ceiling ${old.ceiling}->${next.ceiling}`);
