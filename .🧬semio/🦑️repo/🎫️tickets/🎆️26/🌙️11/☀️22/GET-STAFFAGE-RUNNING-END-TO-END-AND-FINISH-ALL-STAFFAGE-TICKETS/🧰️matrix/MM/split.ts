/** 🪓️ Splits the processor seconds of one resolve into the evaluation of the state and the rest (suitability probe, analysis, three depictions): `bun split.ts <asset id> <denominator> [<asset id> <denominator> …]`.
 *
 * Per asset and scale it resolves the default pose once to warm the caches, then measures `evaluate` alone and `resolveViews` in each of the seven modes, and prints how many evaluations the matrix repeats for one subject (seven modes, two languages at a pose) although the state of a subject is the same in every mode and language. Lines carry the `[DEBUG] ` prefix.
 *
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔭️multiview/🟦️.ts — `resolveViews` evaluates once per call
 */
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { resolveViews } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔭️multiview/🟦️.ts";
import { MODE_LIST, drawOptions, loadAsset, scaleSelector } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage/🧰️.ts";

const cpu = (): number => {
  const used = process.cpuUsage();
  return (used.user + used.system) / 1e6;
};
const args = process.argv.slice(2);
for (let i = 0; i + 1 < args.length; i += 2) {
  const asset = loadAsset(args[i]!)!;
  const denominator = Number(args[i + 1]);
  const query = { kind: "pose", pose: asset.presentation.defaultPose } as const;
  resolveViews(asset, query, { kind: "scale", denominator }, "light-line", drawOptions("light-line", denominator, false, "en"));
  let before = cpu();
  for (let round = 0; round < 3; round++) evaluate(asset, query, scaleSelector(denominator));
  const evaluation = (cpu() - before) / 3;
  const modes: Record<string, number> = {};
  for (const mode of MODE_LIST) {
    before = cpu();
    resolveViews(asset, query, { kind: "scale", denominator }, mode, drawOptions(mode, denominator, false, "en"));
    modes[mode] = Number((cpu() - before).toFixed(3));
  }
  const resolve = Object.values(modes).reduce((sum, seconds) => sum + seconds, 0) / MODE_LIST.length;
  console.error(`[DEBUG] ${asset.id} 1:${denominator} evaluate ${evaluation.toFixed(3)} s, resolveViews mean ${resolve.toFixed(3)} s (${JSON.stringify(modes)}), evaluation share ${((100 * evaluation) / resolve).toFixed(0)} %; a pose costs ${(14 * resolve).toFixed(1)} s with 14 evaluations and would cost ${(14 * resolve - 13 * evaluation).toFixed(1)} s with one`);
}
