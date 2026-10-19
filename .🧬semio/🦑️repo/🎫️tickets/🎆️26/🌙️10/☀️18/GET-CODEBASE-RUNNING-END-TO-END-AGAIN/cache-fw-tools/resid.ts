import { readFileSync } from "node:fs";
import { join } from "node:path";
const a = JSON.parse(readFileSync(join(import.meta.dir, "../🗑️generated/cache-fw-census-after.json"), "utf8"));
const T = a.targets.filter((t: any) => t.owner === "🧰️framework");
const which = process.argv[2];
for (const t of T) { const hit = which === "absent" ? t.cache === "absent" : t.classProposal === which; if (hit) console.log(t.classProposal.slice(0, 8), t.cache, t.execution ?? "-", t.project + ":" + t.name, "|", (t.command ?? "").slice(0, 80)); }
