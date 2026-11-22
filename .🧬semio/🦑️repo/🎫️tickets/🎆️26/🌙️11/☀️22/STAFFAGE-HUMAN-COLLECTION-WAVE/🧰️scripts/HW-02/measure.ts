/** 📐️ Prints the `measureVolumes` measurements of one asset (optionally filtered by rule and name substring).
 *  bun measure.ts <slug|file.json> [rule] [nameFilter] */
import { join, resolve } from "node:path";
const S = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const { measureVolumes } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts"));
const [arg, rule, filter] = process.argv.slice(2);
const file = arg!.endsWith(".json") ? arg! : join(S, "🗂️catalog/👤️humans", arg!, "🔣️.json");
const asset = (await Bun.file(file).json()).asset;
for (const m of measureVolumes(asset, rule ? { rules: [rule] } : {})) if (!filter || m.name.includes(filter)) console.log(`${m.ok ? " " : "✗"} ${m.rule} 1:${m.scale} ${m.list ?? ""} ${m.name} ${typeof m.value === "number" ? m.value.toFixed(4) : m.value} (${m.bound} ${m.limit})`);
