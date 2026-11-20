/** 🪞 Says per scale and list whether the build script of an asset, its source file and the catalog file hold the same volumes.
 *
 * Usage: `bun r4-g07-same.ts <slug>` */
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
const slug = process.argv[2];
const built = await (await import(`./r3-${slug}-build.ts`)).volumes();
const source = JSON.parse(readFileSync(`volumes/humans/${slug}.json`, "utf8")).volumes;
const catalogRoot = "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
let catalog: any = null;
for (const category of readdirSync(catalogRoot)) {
  try { catalog = JSON.parse(readFileSync(join(catalogRoot, category, slug, "🔣️.json"), "utf8")); console.log(join(catalogRoot, category, slug)); break; } catch {}
}
const cat = (catalog.asset ?? catalog).volumes;
for (const scale of Object.keys(built)) for (const list of ["body", "thermal", "structure", "decisions"]) {
  const b = JSON.stringify(built[scale][list]), s = JSON.stringify(source[scale]?.[list]), c = JSON.stringify(cat[scale]?.[list]);
  console.log(scale, list, "built=source", b === s, "source=catalog", s === c, built[scale][list]?.length, source[scale]?.[list]?.length, cat[scale]?.[list]?.length);
}
