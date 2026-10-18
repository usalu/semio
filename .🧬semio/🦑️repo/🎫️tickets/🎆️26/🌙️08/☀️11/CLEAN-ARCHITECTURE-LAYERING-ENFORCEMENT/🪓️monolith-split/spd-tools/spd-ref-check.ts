/** 🔍️ Resolves the `$ref` of every culprit row of a `spd-why` dump against the owner index: `bun spd-ref-check.ts <why.json>`. */
import { readFileSync } from "node:fs";
const root = process.cwd();
const base = `${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection`;
const { SchemaCatalog, buildOwnerIndex } = await import(`${base}/🗂️documents/🟦️.ts`);
const index = buildOwnerIndex(root, new SchemaCatalog(root));
const rows: [string, string, string, string][] = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const seen = new Set<string>();
for (const [name, leaf, keys, detail] of rows) {
  if (!keys.includes("$ref")) continue;
  const ref = detail.startsWith("{") ? (JSON.parse(detail) as { $ref: string }).$ref : detail;
  const key = `${ref}`;
  if (seen.has(key)) continue;
  seen.add(key);
  const document = ref.split("#")[0]!;
  const owner = index.get(document);
  console.log(name, "|", ref, "|", owner === undefined ? "NO OWNER" : `owner root=${owner.root} exports=${owner.exports.size}`);
}
