/** ⚡️ Prints the total power of every catalog asset as JSON `{ id: { category, totalW, elements } }`. */
import { Glob } from "bun";
import { join, resolve } from "node:path";
import { powerOf } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
const out: Record<string, unknown> = {};
for (const path of [...new Glob("*/*/🔣️.json").scanSync(catalog)].sort()) {
  const asset = (await Bun.file(join(catalog, path)).json()).asset;
  const power = powerOf(asset);
  out[asset.id] = { category: asset.category, totalW: power.totalW, elements: power.elements.map((e) => [e.id, Math.round(e.powerW * 10) / 10]) };
}
console.log(JSON.stringify(out));
