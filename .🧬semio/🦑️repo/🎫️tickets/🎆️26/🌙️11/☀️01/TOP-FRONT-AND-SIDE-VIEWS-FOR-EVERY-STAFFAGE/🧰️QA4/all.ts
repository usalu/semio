import { appendFileSync, mkdirSync, rmSync, writeFileSync } from "node:fs";
import { execFileSync } from "node:child_process";
import { join } from "node:path";
import { checkAssetDetailed } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts";
import { runPreview } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";
import { authoredScales, consistency, sweep } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { loadCatalogFromFiles, DEFAULT_CATALOG_ROOT } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";

const HUMANS = join(DEFAULT_CATALOG_ROOT, "👤️humans");
const ids = (process.env.QA4_ASSETS ?? "child-on-kick-scooter,courier-with-hand-truck,cyclist,elderly-with-cane,jogger,kneeling-gardener,leaning-person").split(",").filter(Boolean);
const out = process.env.QA4_OUT!;
const log = process.env.QA4_LOG!;
const doRender = process.env.QA4_RENDER !== "0";
const only = (process.env.QA4_ONLY ?? "").split(",").filter(Boolean);
const fractions = (process.env.QA4_FRACTIONS ?? "0.1,0.5,0.9").split(",").map(Number);
const scaleFilter = (process.env.QA4_SCALES ?? "").split(",").filter(Boolean).map(Number);
writeFileSync(log, "");
const say = (line: string): void => { appendFileSync(log, line + "\n"); };
const catalog = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
const pending: [string, string][] = [];
const collect = { rasterize(svg: string, png: string): void { pending.push([svg, png]); } };
for (const id of ids) {
  const asset = catalog.assets.find((a) => a.id === id)!;
  const report = checkAssetDetailed(asset, { samples: 3 });
  say(`## ${id} check: ${report.problems.length ? "\n   " + report.problems.slice(0, 20).join("\n   ") : "ok"}`);
  const scales = authoredScales(asset);
  for (const scale of scales) for (const f of [...sweep(asset, scale), ...consistency(asset, scale)]) say(`   [${f.check}] 1:${scale} ${f.message}`);
  if (!doRender) continue;
  const file = join(HUMANS, id, "🔣️.json");
  for (const scale of scales) {
    if (scaleFilter.length && !scaleFilter.includes(scale)) continue;
    const dir = `${out}/${id}/${scale}`;
    rmSync(dir, { recursive: true, force: true });
    mkdirSync(dir, { recursive: true });
    const jobs: string[][] = [];
    for (const pose of asset.poses.filter((p) => !p.key)) jobs.push(["--pose", pose.id]);
    for (const clip of asset.clips) for (const f of fractions) jobs.push(["--clip", clip.id, "--time", String(Math.round(clip.durationS * f * 100) / 100)]);
    for (const job of jobs) {
      if (only.length && !only.some((o) => job.includes(o))) continue;
      try { await runPreview(["--file", file, "--scale", String(scale), "--views", "--dpi", "150", "--out", dir, ...job], collect); } catch (e) { say(`   render 1:${scale} ${job.join(" ")}: ${(e as Error).message}`); }
    }
  }
}
const list = join(out, "rasterize.json");
writeFileSync(list, JSON.stringify(pending));
execFileSync("python", [join(import.meta.dir, "../🧰️QA1/raster.py"), list, "150"], { stdio: "inherit" });
rmSync(list);
say(`done ${pending.length} sheets`);
