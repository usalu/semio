/** 🎯️ Picks a default pose from the walk keys of a clip: measures print suitability of every scale at its range ends for the key at each time (diagnostic for the generators).
 *  bun pose-pick.ts <slug> <clip> <basePose> */
import { join } from "node:path";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { HUMANS } from "./lib.ts";

const [slug, clipId, poseId] = process.argv.slice(2);
const asset = (await Bun.file(join(HUMANS, slug!, "🔣️.json")).json()).asset;
const clip = asset.clips.find((c: any) => c.id === clipId);
const base = asset.poses.find((p: any) => p.id === poseId);
for (const key of clip.keys) {
  const contacts = clip.contacts.filter((c: any) => c.fromS <= key.t && key.t <= c.toS).map((c: any) => c.anchor);
  const pose = { ...base, bones: key.bones, contacts: [...new Set([...contacts, ...base.contacts.filter((c: string) => !/heel|ball/u.test(c))])] };
  const one = { ...asset, poses: [pose], clips: [], presentation: { ...asset.presentation, defaultPose: pose.id } };
  const bad: string[] = [];
  for (const s of ["50", "100", "200", "500"]) {
    const rep = one.representations[s];
    for (const end of [rep.validRange.minDenominator, Number(s), rep.validRange.maxDenominator]) {
      const r = measureSuitability(one, rep, end, DEFAULT_PRINT_PROFILE);
      if (!r.suitable) bad.push(`1:${s}@${end} ` + r.violations.map((v: any) => `${v.check} ${v.shapes.join("+")} ${v.measuredMm.toFixed(3)}`).join("; "));
    }
  }
  console.log(`t=${key.t} ${bad.length === 0 ? "OK" : bad.join(" | ")}`);
}
