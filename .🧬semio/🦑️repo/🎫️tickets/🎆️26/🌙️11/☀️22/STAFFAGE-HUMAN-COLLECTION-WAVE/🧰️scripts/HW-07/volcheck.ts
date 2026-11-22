/** 🧊️ The measured volume check (`checkVolumes`: silhouette, bounds, feature, cross-scale, thermal coverage areas, structure coverage, connected, ...) of HW-07 assets or given slugs/files, per scale. bun volcheck.ts [slug|file] [--scale 50] [--rules a,b] */
import { join } from "node:path";
import { checkVolumes } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { HUMANS } from "./lib.ts";
const args = process.argv.slice(2);
const opt = (k: string) => { const i = args.indexOf(k); return i < 0 ? undefined : args[i + 1]; };
const targets = args.filter((x, i) => !x.startsWith("--") && !args[i - 1]?.startsWith("--"));
for (const t of targets.length ? targets : ["tennis-player", "basketball-player", "yoga-practitioner"]) {
  const path = t.endsWith(".json") ? t : join(HUMANS, t, "🔣️.json");
  const asset = (await Bun.file(path).json()).asset;
  const t0 = performance.now();
  const findings = checkVolumes(asset, { ...(opt("--scale") ? { scale: Number(opt("--scale")) } : {}), ...(opt("--rules") ? { rules: opt("--rules")!.split(",") } : {}) } as never);
  console.log(`${asset.id}\t${findings.length} findings\t${((performance.now() - t0) / 1000).toFixed(0)} s`);
  for (const f of findings as any[]) console.log(`  ${f.rule} 1:${f.scale ?? "-"} ${f.list ?? ""}: ${(f.message?.en ?? JSON.stringify(f)).slice(0, 400)}`);
}
