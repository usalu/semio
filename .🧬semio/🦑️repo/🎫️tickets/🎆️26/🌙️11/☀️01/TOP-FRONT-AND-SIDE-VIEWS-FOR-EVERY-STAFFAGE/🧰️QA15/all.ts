import { checkAssetDetailed } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts";
import { DEFAULT_PRINT_PROFILE } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { runPreview } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";
import { authoredScales, catalog, consistency, sweep } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";

const OUT = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🗑️generated/QA15/pv";
const IDS = (process.env.IDS ? process.env.IDS.split(",") : ["glass-balustrade", "glulam-beam", "cantilever-balcony", "curtain-wall-bay", "masonry-wall-with-lintel", "panel-radiator", "external-venetian-blind", "sectional-garage-door"]);
const KEYS50 = new Set(["panel-radiator", "external-venetian-blind", "sectional-garage-door"]);
function jobsFor(asset: any): string[] {
  const scales = authoredScales(asset);
  const jobs: string[] = [];
  for (const scale of scales) {
    for (const pose of asset.poses) if (!pose.key) jobs.push(`${scale}:pose:${pose.id}`);
    for (const clip of asset.clips) {
      const times = new Set<number>([0.25, 0.5, 0.75].map((f) => +(f * clip.durationS).toFixed(2)));
      if (scale === 50 && KEYS50.has(asset.id)) for (const key of clip.keys) times.add(key.timeS ?? key.t);
      for (const t of times) jobs.push(`${scale}:clip:${clip.id}@${t}`);
    }
  }
  return jobs;
}
const JOBS: Record<string, string[]> = {};

const wanted = process.argv.slice(2);
const all = catalog();
for (const id of IDS) JOBS[id] = jobsFor(all.find((a) => a.id === id)!);
const known = new Set(all.map((asset) => asset.id));
for (const id of IDS) {
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
  JOBS[id]!.push(`100:thermal:${asset.presentation.defaultPose}`);
  const SC = process.env.SCALES ? process.env.SCALES.split(",") : null;
  for (const job of JOBS[id]!) {
    if (SC && !SC.includes(job.split(":")[0]!)) continue;
    const [scale, kind, rest] = job.split(":");
    const args = ["--asset", id, "--scale", scale!, "--views", "--view", "light-line", "--no-png", "--out", `${OUT}/${id}`];
    if (kind === "thermal") { args.splice(args.indexOf("light-line"), 1, "thermal"); args.push("--pose", rest!); }
    else if (kind === "pose") args.push("--pose", rest!);
    else {
      const [clip, time] = rest!.split("@");
      args.push("--clip", clip!, "--time", time!);
    }
    try {
      for (const path of await runPreview(args)) if (path.endsWith(".svg")) console.log(`PREVIEW ${path}`);
    } catch (error) {
      console.log(`PREVIEW ERROR ${job}: ${(error as Error).message}`);
    }
    process.exitCode = 0;
  }
}
console.log("ALL DONE");
