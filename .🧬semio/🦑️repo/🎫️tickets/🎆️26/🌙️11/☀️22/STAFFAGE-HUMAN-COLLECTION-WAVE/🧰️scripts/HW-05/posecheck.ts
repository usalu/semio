/** 🔎️ Evaluates every pose of an asset file in every authored scale and viewpoint and reports the shapes whose drawing cannot be built: bun posecheck.ts <asset.json> */
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const asset = (await Bun.file(process.argv[2]!).json()).asset;
let bad = 0;
for (const pose of asset.poses) for (const scale of [500, 200, 100, 50]) for (const viewpoint of ["elevation", "front", "plan"]) {
  try { evaluate(asset, { kind: "pose", pose: pose.id }, { kind: "scale", scale: { denominator: scale }, viewpoint } as never); }
  catch (e) {
    bad++;
    const shapes = asset.representations[`${scale}`].geometry[viewpoint];
    const culprits: string[] = [];
    for (const s of shapes) {
      const trial = structuredClone(asset);
      trial.representations[`${scale}`].geometry[viewpoint] = [s];
      try { evaluate(trial, { kind: "pose", pose: pose.id }, { kind: "scale", scale: { denominator: scale }, viewpoint } as never); } catch { culprits.push(s.id); }
    }
    console.log(`[DEBUG] ${pose.id} 1:${scale} ${viewpoint}: ${(e as Error).message.slice(0, 80)} -> ${culprits.join(",")}`);
  }
}
for (const clip of asset.clips) for (let i = 0; i < 24; i++) for (const scale of [500, 200, 100, 50]) for (const viewpoint of ["elevation", "front", "plan"]) {
  const t = (clip.durationS * i) / 24;
  try { evaluate(asset, { kind: "clip", clip: clip.id, timeS: t }, { kind: "scale", scale: { denominator: scale }, viewpoint } as never); }
  catch (e) { bad++; console.log(`[DEBUG] ${clip.id}@${t.toFixed(3)} 1:${scale} ${viewpoint}: ${(e as Error).message.slice(0, 60)}`); }
}
console.log(bad ? `[DEBUG] ${bad} failing` : "all poses evaluate");
