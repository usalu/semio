/** 📐️ Names the solids at the extremes of each axis per scale in the default pose (or POSE=): bun extremes.ts <asset.json> */
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { worldSolids } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
const asset = (await Bun.file(process.argv[2]!).json()).asset;
const state = evaluate(asset, { kind: "pose", pose: process.env.POSE ?? asset.presentation.defaultPose });
for (const scale of [50, 100, 200, 500]) {
  const ws = worldSolids(asset, state.bones as never, scale);
  const out: string[] = [];
  for (const k of [0, 1, 2]) {
    let lo = [Infinity, ""], hi = [-Infinity, ""];
    for (const w of ws) for (let i = k; i < w.positions.length; i += 3) { const v = w.positions[i]!; if (v < (lo[0] as number)) lo = [v, w.id]; if (v > (hi[0] as number)) hi = [v, w.id]; }
    out.push(`${"xyz"[k]} ${(lo[0] as number).toFixed(3)}(${lo[1]})..${(hi[0] as number).toFixed(3)}(${hi[1]})`);
  }
  console.log(`1:${scale} ${out.join("  ")}`);
}
