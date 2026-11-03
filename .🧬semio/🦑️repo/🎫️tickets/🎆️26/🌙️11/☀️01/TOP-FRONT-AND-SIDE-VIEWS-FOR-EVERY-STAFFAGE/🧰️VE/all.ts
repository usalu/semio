import { readFileSync } from "node:fs";
import type { Asset, AssetDocument } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { checkAssetDetailed } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts";
import { authoredScales, consistency, sweep } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { runPreview, type Rasterizer } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";

const [jobsFile, ...files] = process.argv.slice(2) as [string, ...string[]];
const assets: Asset[] = files.map((file) => (JSON.parse(readFileSync(file, "utf8")) as AssetDocument).asset);
const known = new Set(assets.map((asset) => asset.id));

for (const asset of assets) {
  for (const scale of authoredScales(asset)) {
    try {
      const findings = [...sweep(asset, scale), ...consistency(asset, scale)];
      console.log(`[DEBUG] consistency ${asset.id} 1:${scale}: ${findings.length === 0 ? "ok" : findings.length + " findings"}`);
      for (const finding of findings.slice(0, 6)) console.log(`[DEBUG]   [${finding.check}] ${finding.message}`);
    } catch (error) {
      console.log(`[DEBUG] consistency ${asset.id} 1:${scale} threw ${String(error).split("\n")[0]}`);
    }
  }
}
for (const asset of assets) {
  try {
    const report = checkAssetDetailed(asset, { samples: 3, assets: known });
    console.log(`[DEBUG] check ${asset.id}: ${report.problems.length === 0 ? "ok" : "PROBLEMS"}`);
    for (const line of report.lines) console.log(`[DEBUG]   ${line}`);
    for (const problem of report.problems.slice(0, 30)) console.log(`[DEBUG]   problem ${problem}`);
  } catch (error) {
    console.log(`[DEBUG] check ${asset.id} threw ${String(error).split("\n")[0]}`);
  }
}
const none: Rasterizer = { rasterize() {} };
const jobs = JSON.parse(readFileSync(jobsFile, "utf8")) as { argv: string[] }[];
for (const job of jobs) {
  try {
    await runPreview([...job.argv, "--no-png"], none);
  } catch (error) {
    console.log("[DEBUG] render fail", job.argv.slice(-4).join(" "), String(error).split("\n")[0]);
  }
}
console.log("[DEBUG] all done");
