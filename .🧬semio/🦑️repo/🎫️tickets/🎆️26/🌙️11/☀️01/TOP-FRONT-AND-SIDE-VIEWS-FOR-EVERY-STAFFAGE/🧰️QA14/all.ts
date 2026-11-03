import { writeFileSync, readFileSync, appendFileSync } from "node:fs";
import { authoredScales, catalog, consistency, sweep } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { checkAssetDetailed } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts";
import { runPreview } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";

const [log, jobsFile, ...ids] = process.argv.slice(2);
const say = (line: string): void => appendFileSync(log!, line + "\n", "utf8");
writeFileSync(log!, "", "utf8");
const all = catalog();
const mine = all.filter((a) => ids.includes(a.id));
const known = new Set(all.map((a) => a.id));
for (const asset of (jobsFile === "-" || process.env.QA14_SKIP === undefined ? mine : [])) {
  say(`=== consistency ${asset.id}`);
  for (const scale of authoredScales(asset)) for (const f of [...sweep(asset, scale), ...consistency(asset, scale)]) say(`[${scale}] [${f.check}] ${f.message}`);
  say(`=== check ${asset.id}`);
  try {
    const report = checkAssetDetailed(asset, { samples: 5, assets: known });
    for (const line of report.lines) say("  " + line);
    for (const p of report.problems) say("  PROBLEM " + p);
  } catch (e) {
    say("  CHECK FAIL " + (e as Error).message.slice(0, 400));
  }
}
if (jobsFile !== undefined && jobsFile !== "-") {
  const jobs: string[][] = JSON.parse(readFileSync(jobsFile, "utf8"));
  say("=== previews " + jobs.length);
  for (const job of jobs) {
    try {
      const t0 = Date.now();
      for (const p of await runPreview(job)) say("[DEBUG] " + (Date.now() - t0) + "ms " + p);
    } catch (e) {
      say("[DEBUG] FAIL " + job.join(" ") + " :: " + (e as Error).message.slice(0, 300));
    }
  }
}
say("=== done");
