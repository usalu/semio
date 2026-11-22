/** 🧪️ Probe: measured volume quantities whose name matches a filter (e.g. area, p95). bun probe-measures.ts <slug|file> <filter> [--scale 50] */
import { join } from "node:path";
import { measureVolumes } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { HUMANS } from "./lib.ts";
const [t, filter, , sc] = process.argv.slice(2);
const asset = (await Bun.file(t!.endsWith(".json") ? t! : join(HUMANS, t!, "🔣️.json")).json()).asset;
for (const m of measureVolumes(asset, sc ? ({ scale: Number(sc) } as never) : {}) as any[]) if (`${m.rule} ${m.name}`.includes(filter!)) console.log(m.rule, m.scale, m.list, m.name, typeof m.value === "number" ? m.value.toFixed(4) : m.value, m.limit, m.bound, m.ok === false ? "FAIL" : "");
