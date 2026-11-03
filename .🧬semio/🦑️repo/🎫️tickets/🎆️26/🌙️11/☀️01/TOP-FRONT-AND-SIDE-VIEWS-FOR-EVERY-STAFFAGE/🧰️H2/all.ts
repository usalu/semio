import { readdirSync, existsSync, readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { authoredScales, consistency, sweep } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { checkAssetDetailed } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { runPreview } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";

const S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const [outDir, mode, ...ids] = process.argv.slice(2);
mkdirSync(outDir, { recursive: true });
const find = (id: string): string => {
  const cat = readdirSync(S).find((n) => n.includes("catalog"))!;
  for (const c of readdirSync(`${S}/${cat}`)) { const d = `${S}/${cat}/${c}/${id}`; if (existsSync(d)) return `${d}/${readdirSync(d).find((n) => n.endsWith(".json"))}`; }
  throw new Error(id);
};
const log: string[] = [];
const say = (line: string) => { log.push(line); writeFileSync(`${outDir}/all.log`, log.join("\n"), "utf8"); };
for (const id of ids) {
  const file = find(id);
  const asset = JSON.parse(readFileSync(file, "utf8")).asset;
  say(`== ${id}`);
  try {
    for (const scale of authoredScales(asset)) for (const f of [...sweep(asset, scale), ...consistency(asset, scale)]) say(`[${f.check}] ${f.message}`);
  } catch (e) { say(`consistency error ${(e as Error).message}`); }
  if (mode.includes("dbg")) {
    const st = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: 50 } });
    for (const s of st.shapes.front) { let y1 = -1e9; for (const r of s.rings) for (const q of r) y1 = Math.max(y1, q.y); say(`[DEBUG] front ${s.shape.id} bone ${s.shape.bone} ymax ${y1.toFixed(3)}`); }
    for (const b of ["rib-front", "umbrella"]) say(`[DEBUG] bone ${b} ${JSON.stringify(st.bones[b]?.origin)} ${JSON.stringify(st.bones[b]?.tip)}`);
  }
  if (mode.includes("check")) {
    try {
      const rep = checkAssetDetailed(asset, { samples: 5 });
      for (const l of rep.lines) say(`  ${l}`);
      for (const p of rep.problems.slice(0, 40)) say(`  PROBLEM ${p}`);
    } catch (e) { say(`check error ${(e as Error).message}`); }
  }
  if (mode.includes("prev")) {
    const jobs: string[][] = [];
    const def = asset.presentation.defaultPose;
    for (const s of [500, 200, 100, 50]) if (asset.representations[String(s)]?.status === "authored") jobs.push(["--scale", String(s), "--pose", def]);
    const s100 = asset.representations["100"]?.status === "authored" ? "100" : "50";
    for (const p of asset.poses) if (!p.key) jobs.push(["--scale", s100, "--pose", p.id]);
    for (const c of asset.clips) for (const f of [0.25, 0.6]) jobs.push(["--scale", s100, "--clip", c.id, "--time", String(c.durationS * f)]);
    for (const j of jobs) {
      try { await runPreview(["--asset", id, "--views", "--dpi", "110", ...j, "--out", `${outDir}/${id}`]); }
      catch (e) { say(`preview ${j.join(" ")} failed: ${(e as Error).message}`); }
    }
    say(`  previews ${jobs.length}`);
  }
}
say("DONE");
