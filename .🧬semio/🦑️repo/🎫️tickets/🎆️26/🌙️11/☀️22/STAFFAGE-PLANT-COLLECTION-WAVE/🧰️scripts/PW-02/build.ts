/** 🏭️ Builds the PW-02 trees: writes `🗂️catalog/🌳️vegetation/<id>/🔣️.json`, the manifest row `📥️manifest-rows/<id>.json` and a build log under `🗑️generated/PW-02/`.
 * Usage: `repo tools exec bun -- build.ts horse-chestnut european-beech common-ash [--candidate]` (`--candidate` writes only `🗑️generated/PW-02/<id>/🔣️.json`)
 * @see ./kit.ts */
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { buildAsset, type Species } from "./kit.ts";

const HERE = dirname(fileURLToPath(import.meta.url));
const TICKET = join(HERE, "../..");
const CATALOG = join(HERE, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation");
const ids = process.argv.slice(2).filter((a) => !a.startsWith("--"));
const candidate = process.argv.includes("--candidate");
for (const id of ids) {
  const sp: Species = (await import(`./${id}.ts`)).species;
  const started = Date.now();
  const { doc, row, report } = buildAsset(sp);
  const dir = candidate ? join(TICKET, "🗑️generated/PW-02", id) : join(CATALOG, id);
  mkdirSync(dir, { recursive: true });
  writeFileSync(join(dir, "🔣️.json"), JSON.stringify(doc, null, 1) + "\n");
  mkdirSync(join(TICKET, "📥️manifest-rows"), { recursive: true });
  if (!candidate) writeFileSync(join(TICKET, "📥️manifest-rows", `${id}.json`), JSON.stringify(row, null, 1) + "\n");
  mkdirSync(join(TICKET, "🗑️generated/PW-02"), { recursive: true });
  writeFileSync(join(TICKET, "🗑️generated/PW-02", `build-${id}.txt`), report.join("\n") + "\n");
  console.log(`[DEBUG] ${id}: written in ${Date.now() - started} ms`);
  for (const line of report) console.log(`[DEBUG]   ${line}`);
}
