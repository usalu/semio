/** 🔒️ Re-cuts the language-law allowlist rows of the files of one batch from their new texts and lowers the ceiling by what the rows lost: bun cp-allow.ts <spec json> with {allowlist, entries:[{rel, work}]}. Edits the allowlist work file in place. */
import { readFileSync, writeFileSync } from "node:fs";
const live = "/Users/ueli/Documents/semio";
const language = await import(`${live}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🗣️language-catalogs/🟦️.ts`);
const spec = JSON.parse(readFileSync(process.argv[2]!, "utf8")) as { allowlist: string; entries: { rel: string; work: string }[] };
const list = JSON.parse(readFileSync(spec.allowlist, "utf8")) as { ceiling: number; files: Record<string, number>; defaults: Record<string, number> };
const before = Object.values(list.files).reduce((sum, n) => sum + n, 0);
for (const { rel, work } of spec.entries) {
  const count = language.proseSitesV1(readFileSync(work, "utf8")) as number;
  if (count > 0) list.files[rel] = count; else delete list.files[rel];
}
const sorted: Record<string, number> = {};
for (const key of Object.keys(list.files).sort()) sorted[key] = list.files[key]!;
list.files = sorted;
const after = Object.values(list.files).reduce((sum, n) => sum + n, 0);
if (after < before) list.ceiling -= before - after;
writeFileSync(spec.allowlist, JSON.stringify(list, null, 1) + "\n");
console.error("[DEBUG] allowlist", before, "->", after, "ceiling", list.ceiling);
