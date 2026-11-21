/** 🖨️ Says for a candidate asset document which pose and which view carry the smallest printed feature of a scale.
 *
 * Usage: `bun r4-g07-print.ts <asset document json> <scale>` */
import { readFileSync } from "node:fs";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
const doc = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const scale = process.argv[3]!;
const asset = doc.asset ?? doc;
for (const pose of asset.poses) for (const view of ["elevation", "front", "plan", "all"]) {
  const one = JSON.parse(JSON.stringify(asset));
  one.poses = [pose];
  one.presentation.defaultPose = pose.id;
  one.clips = [];
  delete one.presentation.defaultClip;
  const rep = one.representations[scale];
  if (view !== "all") for (const v of ["elevation", "front", "plan"]) if (v !== view) rep.geometry[v] = rep.geometry[v].filter((s: any) => s.role === "ground");
  try {
    const s = measureSuitability(one as never, rep as never, Number(scale), DEFAULT_PRINT_PROFILE) as any;
    console.log(pose.id, view, s.suitable, Number(s.minFeatureMm).toFixed(3), JSON.stringify(s.violations.map((v: any) => [v.check, v.shapes, Number(v.measuredMm).toFixed(3)])));
  } catch (e) { console.log(pose.id, view, "threw", (e as Error).message.slice(0, 120)); }
}
