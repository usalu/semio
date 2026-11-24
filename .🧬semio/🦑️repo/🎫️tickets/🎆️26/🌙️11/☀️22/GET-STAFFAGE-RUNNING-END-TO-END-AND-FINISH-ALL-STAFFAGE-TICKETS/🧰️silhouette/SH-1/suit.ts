/** 🖨️ The print-suitability violations of people per authored scale under the default profile, as the check CLI measures them (shape ids, measured and required paper millimetres); optional ops applied first.
 *  bun suit.ts <slug|file.json> [ops.json] */
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { applyOps } from "./ops.ts";
import { fileOf } from "./people.ts";

const [name, opsArg] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
if (opsArg) applyOps(asset, JSON.parse(await Bun.file(opsArg).text()));
for (const d of [50, 100, 200, 500]) {
  const rep = asset.representations[String(d)];
  if (rep?.status !== "authored") continue;
  const s = measureSuitability(asset, rep, d, DEFAULT_PRINT_PROFILE);
  console.log(`${asset.id} 1:${d} ${s.suitable ? "suitable" : "UNSUITABLE"}${s.violations.map((v: any) => ` | ${v.check} ${v.shapes.join("+")} ${v.measuredMm.toFixed(3)} < ${v.requiredMm}`).join("")}`);
}
