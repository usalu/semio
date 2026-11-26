/** 🔎️ The solid ids of the model of an asset per scale and view as the suite `🧊️model-bounds` reads them (id, node, tags): ids.ts <asset id> <scales> <views> [<id regexp>] */
import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { modelOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🟦️.ts";
const [which, scales, views, pattern = "."] = process.argv.slice(2);
const root = process.env.S + "/🗂️catalog";
const file = readdirSync(root).map((c) => join(root, c, which!, "🔣️.json")).find((f) => { try { readFileSync(f); return true; } catch { return false; } })!;
const asset = JSON.parse(readFileSync(file, "utf8")).asset;
for (const scale of scales!.split(",").map(Number)) for (const view of views!.split(",")) {
  const model = await modelOf(asset, { scale, view: view as never, poses: [], clips: [] });
  const hits = model.solids.filter((s: any) => new RegExp(pattern).test(s.id) && !(s.tags ?? []).includes("glyph"));
  console.log(`== ${asset.id} 1:${scale} ${view}: ${hits.length}/${model.solids.length}: ${hits.map((s: any) => s.id + "@" + s.node).join(" ")}`);
}
