/** 🚀️ Builds the PW-01 maples and writes each candidate document to `🗑️generated/PW-01/<id>/🔣️.json` (and, with `--ship`, to the catalog folder and the manifest row to `📥️manifest-rows/<id>.json`).
 * Usage: `repo tools exec bun -- run.ts [norway-maple,field-maple,sycamore-maple] [--ship]`
 * @see ./asset.ts */
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { build } from "./asset.ts";

const HERE = dirname(fileURLToPath(import.meta.url));
const TICKET = join(HERE, "../..");
const CATALOG = join(TICKET, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation");
const all = ["norway-maple", "field-maple", "sycamore-maple"];
const pick = (process.argv[2] && !process.argv[2].startsWith("--") ? process.argv[2].split(",") : all).filter((x) => all.includes(x));
const ship = process.argv.includes("--ship");
for (const id of pick) {
  const { design } = await import(`./${id}.ts`);
  const t0 = Date.now();
  const { document, manifest, report } = build(design);
  const out = join(TICKET, "🗑️generated/PW-01", id);
  mkdirSync(out, { recursive: true });
  writeFileSync(join(out, "🔣️.json"), JSON.stringify(document) + "\n");
  if (ship) {
    mkdirSync(join(CATALOG, id), { recursive: true });
    writeFileSync(join(CATALOG, id, "🔣️.json"), JSON.stringify(document) + "\n");
    writeFileSync(join(TICKET, "📥️manifest-rows", `${id}.json`), JSON.stringify(manifest, null, 2) + "\n");
  }
  console.log(`${id} (${((Date.now() - t0) / 1000).toFixed(1)} s)${ship ? " shipped" : ""}\n  ${report.join("\n  ")}`);
}
