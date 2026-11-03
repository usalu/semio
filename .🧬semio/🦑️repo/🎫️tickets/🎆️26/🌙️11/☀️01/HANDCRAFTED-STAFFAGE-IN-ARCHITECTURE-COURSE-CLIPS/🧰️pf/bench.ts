/** ⏱️ Exports the given casts with the exporter at `<exporter>` (a path) and prints CPU, wall and peak memory: `bun bench.ts <exporter.ts> <manifest> <out> <catalog> <ids,...>`. */
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

const [exporter, manifest, out, catalog, only] = process.argv.slice(2);
const { exportCast } = (await import(pathToFileURL(resolve(exporter!)).href)) as typeof import("../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎭️cast/🟦️.ts");
const m = JSON.parse(await readFile(manifest!, "utf8"));
const ids = only ? only.split(",") : Object.keys(m.casts);
const started = performance.now();
const cpu0 = process.cpuUsage();
await exportCast(m, ids, out!, { language: "en", catalog: catalog! });
const cpu = process.cpuUsage(cpu0);
console.log(`[wall] ${((performance.now() - started) / 1000).toFixed(1)} s [cpu] ${((cpu.user + cpu.system) / 1e6).toFixed(1)} s [peak] ${Math.round(process.resourceUsage().maxRSS / 1024)} MB ${only ?? "all"}`);
