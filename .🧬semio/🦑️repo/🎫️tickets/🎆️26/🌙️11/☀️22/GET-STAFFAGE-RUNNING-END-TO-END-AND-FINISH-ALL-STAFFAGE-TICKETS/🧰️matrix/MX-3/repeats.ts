/** 🔁️ The `repeat` count of the solids of a model: repeats.ts <asset id> <scale> <view> <id regexp> */
import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { modelOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🟦️.ts";
const [which, scale, view, pattern] = process.argv.slice(2);
const root = process.env.S + "/🗂️catalog";
const file = readdirSync(root).map((c) => join(root, c, which!, "🔣️.json")).find((f) => { try { readFileSync(f); return true; } catch { return false; } })!;
const model = await modelOf(JSON.parse(readFileSync(file, "utf8")).asset, { scale: Number(scale), view: view as never, poses: [], clips: [] });
for (const s of model.solids.filter((s: any) => new RegExp(pattern!).test(s.id)) as any[]) console.log(s.id, JSON.stringify(s.repeat ?? null), Object.keys(s).join(","));
