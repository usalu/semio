/** 🖨️ Where a band breaks the print profile: `bun prints-where.ts <id|file.json> <scale>` measures the suitability of the band one pose and one view at a time (the other two views emptied) and prints every violation with its pose, view and the z, kind and bone of the shape; a second line gives the shapes painted above it on the sheet that touch it. Run it from `T`.
 * @see ../MC/prints.ts — the same check over all poses and views at once */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";
const { DEFAULT_PRINT_PROFILE, measureSuitability } = await import(join(S, "🔨️modules/🖨️representation/🟦️.ts"));
const asset = readDoc(process.argv[2]!).doc.asset;
const key = process.argv[3]!;
const rep = asset.representations[key];
for (const pose of asset.poses.filter((candidate: any) => !candidate.key))
  for (const view of ["elevation", "front", "plan"] as const) {
    const geometry = { elevation: [], front: [], plan: [], [view]: rep.geometry[view] };
    const one = { ...asset, poses: [pose], presentation: { ...asset.presentation, defaultPose: pose.id }, representations: { ...asset.representations, [key]: { ...rep, geometry } } };
    try {
      for (const v of measureSuitability(one, one.representations[key], Number(key), DEFAULT_PRINT_PROFILE).violations) {
        const shape = rep.geometry[view].find((s: any) => s.id === v.shapes[0]);
        console.log(`1:${key} ${pose.id} ${view} ${v.check} ${v.shapes.join(",")} ${v.measuredMm.toFixed(5)} < ${v.requiredMm} (bone ${shape?.bone} z ${shape?.z} ${shape?.solid?.kind ?? shape?.kind})`);
      }
    } catch (error) {
      console.log(`1:${key} ${pose.id} ${view} throws ${(error as Error).message.slice(0, 200)}`);
    }
  }
