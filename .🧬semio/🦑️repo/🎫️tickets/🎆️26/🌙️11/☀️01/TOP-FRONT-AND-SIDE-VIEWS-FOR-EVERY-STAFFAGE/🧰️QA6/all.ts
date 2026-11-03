import { readFileSync, mkdirSync, writeFileSync } from "node:fs";
import { runPreview } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";
import { checkAsset } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts";

const root = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery";
const out = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🗑️generated/QA6";
const ids = (process.env.QA6_IDS ?? "oil-pumpjack,plate-heat-exchanger,scissor-lift,scroll-compressor,sorption-wheel,tower-crane,wheel-loader,solar-thermal-collector").split(",");
const lines: string[] = [];
const log = (t: string) => { lines.push(t); console.log(t); };
for (const id of ids) {
  try {
    const file = `${root}/${id}/🔣️.json`;
    const asset = JSON.parse(readFileSync(file, "utf8")).asset;
    const authored = ["50", "100", "200", "500"].filter((s) => !process.env.QA6_SCALES || process.env.QA6_SCALES.split(",").includes(s)).filter((s) => asset.representations[s]?.status === "authored");
    log(`=== ${id} authored ${authored.join(",")}`);
    try { for (const l of checkAsset(asset, 6)) log("check: " + l.slice(0, 400)); } catch (e) { log("check threw " + (e as Error).message); }
    for (const s of authored) {
      const dir = `${out}/${id}/${s}`;
      mkdirSync(dir, { recursive: true });
      const base = ["--file", file, "--scale", s, "--views", "--view", "dark-line", "--no-png", "--out", dir];
      const jobs: string[][] = [];
      for (const p of asset.poses.filter((x: any) => !process.env.QA6_POSE || x.id === process.env.QA6_POSE)) jobs.push(["--pose", p.id]);
      for (const c of process.env.QA6_POSE ? [] : asset.clips) for (const k of [0.1, 0.5, 0.9]) jobs.push(["--clip", c.id, "--time", String(Math.round(k * c.durationS * 1000) / 1000)]);
      
      if (process.env.QA6_JOBS) { jobs.length = 0; for (const t of process.env.QA6_JOBS.split(";")) { const [k, a, b] = t.split(":"); jobs.push(k === "pose" ? ["--pose", a!] : ["--clip", a!, "--time", b!]); } }
      for (const j of jobs) { try { await runPreview([...base, ...j]); } catch (e) { log(`PREVIEW FAIL 1:${s} ${j.join(" ")}: ${(e as Error).message.slice(0, 200)}`); } }
      try { await runPreview(["--file", file, "--scale", s, "--views", "--view", "thermal", "--no-png", "--pose", asset.presentation.defaultPose, "--out", dir]); } catch (e) { log(`THERMAL FAIL 1:${s}: ${(e as Error).message.slice(0, 200)}`); }
    }
  } catch (e) { log(`ASSET FAILED ${id}: ${(e as Error).stack?.split("\n").slice(0, 4).join(" | ")}`); }
}
writeFileSync(`${out}/all-result.txt`, lines.join("\n"));
