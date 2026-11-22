/** 🔍️ Study aid: prints the print-suitability violations of one representation of the candidate per named pose, to find which pose and shapes a `range-end-*` finding comes from. `$BUN author-gap-probe.ts <scale> <requested>`
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts — `measureSuitability` */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { measureSuitability } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

const S = join(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const asset = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")).asset;
const profiles = JSON.parse(readFileSync(join(S, "🗂️catalog/🖨️print-profiles/🔣️.json"), "utf8"));
const profile = profiles.profiles.find((p: { id: string }) => p.id === profiles.defaultProfile);
const [scale, requested] = [process.argv[2] ?? "200", Number(process.argv[3] ?? "350")];
for (const pose of asset.poses) {
  const one = { ...asset, poses: [pose], presentation: { ...asset.presentation, defaultPose: pose.id } };
  const s = measureSuitability(one, one.representations[scale], requested, profile);
  console.log(`[DEBUG] ${pose.id}: ${JSON.stringify(s.violations)}`);
}
