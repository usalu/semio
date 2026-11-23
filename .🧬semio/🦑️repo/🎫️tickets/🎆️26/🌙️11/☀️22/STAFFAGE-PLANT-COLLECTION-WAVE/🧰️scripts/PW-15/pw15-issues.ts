/** 🧾 PW-15 authoring aid: every `validateAsset` issue of one asset file grouped by code, scale and solid family (the id without its numbers), with one example message each, so a generator change is judged at a glance.
 *
 * `bun pw15-issues.ts <asset document json>`
 *
 * @see ../FIN-B/probe.ts */
import { readFileSync } from "node:fs";
import { validateAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { DEFAULT_CATALOG_ROOT, loadEnvironments } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";

const asset = JSON.parse(readFileSync(process.argv[2]!, "utf8")).asset;
const environments = new Set(((await loadEnvironments(DEFAULT_CATALOG_ROOT))?.presets ?? []).map((preset: { id: string }) => preset.id));
const issues = validateAsset(asset, { assets: new Map([[asset.id, asset]]), environments } as never);
const groups = new Map<string, { n: number; example: string; ids: string[] }>();
for (const issue of issues) {
  const scale = /^volumes\.(\d+)\.(\w+)/.exec(issue.path);
  const id = /"([^"]+)"/.exec(issue.message.en)?.[1] ?? "";
  const key = `${issue.code} ${scale ? `1:${scale[1]} ${scale[2]}` : issue.path.split(".").slice(0, 2).join(".")} ${id.replace(/\d+/g, "#")}`;
  const g = groups.get(key) ?? { n: 0, example: issue.message.en.slice(0, 260), ids: [] };
  g.n++;
  if (g.ids.length < 6) g.ids.push(id);
  groups.set(key, g);
}
console.log(`[DEBUG] ${asset.id}: ${issues.length} issue(s)`);
for (const [key, g] of groups) console.log(`[DEBUG] ${g.n} x ${key} | ${g.ids.join(", ")} | ${g.example}`);
