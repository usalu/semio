import { readFile } from "node:fs/promises";
import { exportCast } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎭️cast/🟦️.ts";
const [manifest, out, catalog, only] = process.argv.slice(2);
const m = JSON.parse(await readFile(manifest!, "utf8"));
const ids = only ? only.split(",") : Object.keys(m.casts);
const started = performance.now(); const cpu0 = process.cpuUsage();
try { await exportCast(m, ids, out!, { language: "en", catalog: catalog! }); } catch (e) { console.log((e as Error).stack); }
const cpu = process.cpuUsage(cpu0); console.log("[wall]", ((performance.now() - started) / 1000).toFixed(1), "s [cpu]", ((cpu.user + cpu.system) / 1e6).toFixed(1), "s [peak]", Math.round(process.resourceUsage().maxRSS / 1024), "MB");
