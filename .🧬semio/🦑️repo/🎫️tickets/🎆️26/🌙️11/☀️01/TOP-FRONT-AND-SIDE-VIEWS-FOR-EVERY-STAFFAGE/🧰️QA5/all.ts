import { appendFileSync, mkdirSync, writeFileSync } from "node:fs";
import { checkAssetDetailed } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts";
import { runPreview } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";
import { authoredScales, consistency, sweep } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { loadCatalogFromFiles, DEFAULT_CATALOG_ROOT } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";

const ids = (process.env.QA5_ASSETS ?? "").split(",").filter(Boolean);
const out = process.env.QA5_OUT!;
const log = process.env.QA5_LOG!;
const doRender = process.env.QA5_RENDER !== "0";
const only = (process.env.QA5_ONLY ?? "").split(",").filter(Boolean);
const fractions = (process.env.QA5_FRACTIONS ?? "0.1,0.5,0.9").split(",").map(Number);
const scaleFilter = (process.env.QA5_SCALES ?? "").split(",").filter(Boolean).map(Number);
writeFileSync(log, "");
const say = (line: string): void => { appendFileSync(log, line + "\n"); };
const catalog = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
for (const id of ids) {
  const asset = catalog.assets.find((a) => a.id === id)!;
  const report = checkAssetDetailed(asset, { samples: 3 });
  say(`## ${id} check: ${report.problems.length ? "\n   " + report.problems.slice(0, 20).join("\n   ") : "ok"}`);
  const scales = authoredScales(asset);
  for (const scale of scales) {
    const findings = [...sweep(asset, scale), ...consistency(asset, scale)];
    for (const f of findings) say(`   [${f.check}] 1:${scale} ${f.message}`);
  }
  if (!doRender) continue;
  const dir = `${out}/${id}`;
  mkdirSync(dir, { recursive: true });
  for (const scale of scales) {
    if (scaleFilter.length && !scaleFilter.includes(scale)) continue;
    const base = ["--asset", id, "--scale", String(scale), "--views", "--no-png", "--out", dir];
    const jobs: string[][] = [];
    for (const pose of asset.poses.filter((p) => !p.key)) jobs.push(["--pose", pose.id]);
    for (const clip of asset.clips) for (const f of fractions) jobs.push(["--clip", clip.id, "--time", String(Math.round(clip.durationS * f * 100) / 100)]);
    for (const job of jobs) {
      if (only.length && !only.some((o) => job.includes(o))) continue;
      try { await runPreview([...base, ...job]); } catch (e) { say(`   render 1:${scale} ${job.join(" ")}: ${(e as Error).message}`); }
    }
    try { await runPreview([...base, "--view", "thermal"]); } catch (e) { say(`   thermal 1:${scale}: ${(e as Error).message}`); }
  }
}
say("done");
