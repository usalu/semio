import { checkAssetDetailed } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts";
import { DEFAULT_PRINT_PROFILE } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { runPreview } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";
import { authoredScales, catalog, consistency, sweep } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";

const OUT = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🗑️generated/C1/pv";
const JOBS: Record<string, string[]> = {
  "glass-balustrade": ["200:pose:handrail-line-load", "100:pose:handrail-line-load", "50:pose:handrail-line-load", "50:clip:handrail-push@1.5"],
  "glulam-beam": ["500:pose:self-weight", "200:pose:point-load-midspan", "100:pose:point-load-midspan", "50:pose:distributed-live-load", "50:clip:moving-point-load@3"],
  "cantilever-balcony": ["500:pose:distributed-live-load", "200:pose:distributed-live-load", "100:pose:distributed-live-load", "50:pose:distributed-live-load", "50:pose:point-load-tip"],
  "curtain-wall-bay": ["500:pose:dead-load", "200:pose:dead-load", "100:pose:dead-load", "50:pose:dead-load", "50:clip:cradle-pass@4"],
  "masonry-wall-with-lintel": ["500:pose:dead-load", "200:pose:dead-load", "100:pose:dead-load", "50:pose:dead-load", "50:pose:beam-over-opening"],
  "panel-radiator": ["200:pose:rated", "100:pose:rated", "50:pose:rated", "50:pose:off", "50:clip:heat-up@3"],
  "sectional-garage-door": ["500:pose:travel-4", "200:pose:open", "100:pose:half-open", "50:pose:closed", "50:pose:half-open", "50:pose:open", "50:pose:travel-3", "50:pose:travel-6", "50:clip:open-close@8", "50:clip:open-close@20"],
  "external-venetian-blind": ["200:pose:lowered-closed", "100:pose:lowered-cut-off-45", "50:pose:lowered-horizontal", "50:pose:retracted", "50:pose:lowered-cut-off-45", "50:pose:lowered-closed", "50:pose:tilt-1", "50:pose:lift-2", "50:clip:tilt-sweep@6"],
};

const wanted = process.argv.slice(2);
const all = catalog();
const known = new Set(all.map((asset) => asset.id));
for (const id of Object.keys(JOBS)) {
  if (wanted.length > 0 && !wanted.includes(id)) continue;
  const asset = all.find((candidate) => candidate.id === id)!;
  console.log(`===== ${id}`);
  try {
    const report = checkAssetDetailed(asset, { samples: 9, profile: DEFAULT_PRINT_PROFILE, assets: known, physics: true });
    console.log(`CHECK ${report.problems.length ? "FAIL" : "ok"}`);
    for (const line of report.lines) console.log(`  ${line}`);
    for (const problem of report.problems.slice(0, 40)) console.log(`  ! ${problem}`);
  } catch (error) {
    console.log(`CHECK ERROR ${(error as Error).message}`);
  }
  for (const scale of authoredScales(asset)) {
    try {
      const findings = [...sweep(asset, scale), ...consistency(asset, scale)];
      console.log(`CONSISTENCY 1:${scale} ${findings.length === 0 ? "ok" : "FAIL"}`);
      for (const finding of findings.slice(0, 20)) console.log(`  [${finding.check}] ${finding.message}`);
    } catch (error) {
      console.log(`CONSISTENCY 1:${scale} ERROR ${(error as Error).message}`);
    }
  }
  for (const job of JOBS[id]!) {
    const [scale, kind, rest] = job.split(":");
    const args = ["--asset", id, "--scale", scale!, "--views", "--view", "light-line", "--dpi", "130", "--out", `${OUT}/${id}`];
    if (kind === "pose") args.push("--pose", rest!);
    else {
      const [clip, time] = rest!.split("@");
      args.push("--clip", clip!, "--time", time!);
    }
    try {
      for (const path of await runPreview(args)) if (path.endsWith(".png")) console.log(`PREVIEW ${path}`);
    } catch (error) {
      console.log(`PREVIEW ERROR ${job}: ${(error as Error).message}`);
    }
    process.exitCode = 0;
  }
}
console.log("ALL DONE");
