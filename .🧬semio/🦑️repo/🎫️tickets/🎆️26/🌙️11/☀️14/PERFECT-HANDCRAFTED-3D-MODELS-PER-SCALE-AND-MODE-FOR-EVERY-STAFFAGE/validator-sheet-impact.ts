/** 📄️ Impact of the declared thin sheet on the catalog: every solid that is sheet-shaped but declares neither `sheet: true` nor a thermal layer gauge, so that it passed `volumes-feature` before the declaration was required and fails now.
 *
 * Usage: `bun validator-sheet-impact.ts`: reads every `🗂️catalog/<category>/<slug>/🔣️.json`, one asset at a time, and writes `validator-sheet-impact.md` next to this script.
 * @see ./validator-sheet-impact.md
 * @see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts — `featureRule` */
import { readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { featureRule } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";

const HINT = "declare `sheet: true`";
const catalog = join(import.meta.dirname, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
const folders = (path: string): string[] => readdirSync(path).filter((name) => statSync(join(path, name)).isDirectory()).sort();
const lines: string[] = [];
const thrown: string[] = [];
let [assets, affected, solids] = [0, 0, 0];
for (const category of folders(catalog))
  for (const slug of folders(join(catalog, category))) {
    const name = `${category}/${slug}`;
    try {
      const { asset } = JSON.parse(readFileSync(join(catalog, category, slug, "🔣️.json"), "utf8"));
      assets++;
      const groups = new Map<string, string[]>();
      for (const finding of featureRule(asset).findings) {
        if (!finding.message.en.includes(HINT)) continue;
        const key = `1:${finding.scale} ${finding.list}`;
        groups.set(key, [...(groups.get(key) ?? []), `${finding.data?.solid}`]);
        solids++;
      }
      if (groups.size === 0) continue;
      affected++;
      for (const [key, ids] of groups) lines.push(`- ${name}: ${key} ${ids.join(", ")}`);
    } catch (error) {
      thrown.push(`- ${name}: ${`${(error as Error).message}`.slice(0, 200)}`);
    }
  }
writeFileSync(join(import.meta.dirname, "validator-sheet-impact.md"), [
  "# Impact of the Declared Thin Sheet on the Catalog",
  "",
  "`volumes-feature` lets a thin sheet pass below the smallest feature only when it is declared: `sheet: true` on the solid, or a thermal layer gauge (`thicknessM`) for its bone and material. The solids below are sheet-shaped (middle extent at least the limit and at least 20 times the thickness) but undeclared: they passed before and fail now. A true sheet (vane, leaf blade, fabric, membrane, foil, sheet metal, thin board or panel) gets `sheet: true`; anything else is rebuilt for its scale.",
  "",
  `Totals: ${assets} assets read, ${affected} assets affected, ${solids} solids (counted per scale and list), ${thrown.length} assets that could not be read or judged.`,
  "",
  "## Solids per Asset, Scale and List",
  "",
  ...lines,
  ...(thrown.length === 0 ? [] : ["", "## Assets That Threw", "", ...thrown]),
  "",
].join("\n"));
console.log(`assets ${assets}, affected ${affected}, solids ${solids}, thrown ${thrown.length}`);
