/** 🖨️ `../MH-4/prints-where.ts` at a denominator of the band that is not its key (the ends of the valid range): `bun prints-at.ts <id|file.json> <band> <denominator>` measures the suitability of the band one pose and one view at a time (the other two views emptied) and prints every violation with its pose, view and the z, kind and bone of the shape. Run it from `T`.
 * @see ../MH-4/prints-where.ts — the tool this one repeats */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";
const { DEFAULT_PRINT_PROFILE, measureSuitability } = await import(join(S, "🔨️modules/🖨️representation/🟦️.ts"));
const asset = readDoc(process.argv[2]!).doc.asset;
const key = process.argv[3]!;
const denominator = Number(process.argv[4] ?? key);
const rep = asset.representations[key];
let count = 0;
for (const pose of asset.poses.filter((candidate: any) => !candidate.key))
  for (const view of ["elevation", "front", "plan"] as const) {
    const geometry = { elevation: [], front: [], plan: [], [view]: rep.geometry[view] };
    const one = { ...asset, poses: [pose], presentation: { ...asset.presentation, defaultPose: pose.id }, representations: { ...asset.representations, [key]: { ...rep, geometry } } };
    try {
      for (const v of measureSuitability(one, one.representations[key], denominator, DEFAULT_PRINT_PROFILE).violations) {
        const shape = rep.geometry[view].find((s: any) => s.id === v.shapes[0]);
        count++;
        console.log(`band ${key} at 1:${denominator} ${pose.id} ${view} ${v.check} ${v.shapes.join(",")} ${v.measuredMm.toFixed(5)} < ${v.requiredMm} (bone ${shape?.bone} z ${shape?.z} ${shape?.solid?.kind ?? shape?.kind} pen ${shape?.penMm})`);
      }
    } catch (error) {
      console.log(`band ${key} at 1:${denominator} ${pose.id} ${view} throws ${(error as Error).message.slice(0, 200)}`);
    }
  }
console.log(`${asset.id} band ${key} at 1:${denominator}: ${count} violations`);
