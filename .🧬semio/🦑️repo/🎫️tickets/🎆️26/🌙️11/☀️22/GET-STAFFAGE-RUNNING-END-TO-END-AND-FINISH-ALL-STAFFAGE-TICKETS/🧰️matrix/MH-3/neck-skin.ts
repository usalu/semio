/** 🧣️ Lengthens the thermal neck skin `th-neck` of `person-hanging-laundry` at 1:500 to the smallest feature of that scale: `bun neck-skin.ts <id|file.json> [--write] [--out <file>]`. The solid was 40 mm long (the bare neck above the collar of the finer bands) at a scale whose smallest feature is 75 mm (0.15 mm on paper); `volumes-feature` only missed it because the plan neck of 1:500 was a plate 18 mm thin (the rule takes the thinnest drawing of the bone as its limit). At 1:500 no collar is drawn and the whole neck is printed as skin, so the solid now starts 75.5 mm below its end on the body loft `neck` (centre and half axes interpolated on that loft). A second run changes nothing. It runs before `fit.ts` in the chain of the asset, which makes the plan neck a prism.
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts — `featureRule` */
import { readDoc, writeDoc } from "../MR/lib.ts";

const LENGTH_M = 0.0755;
const args = process.argv.slice(2);
const out = args.includes("--out") ? args[args.indexOf("--out") + 1] : undefined;
const read = readDoc(args[0]!);
const volume = read.doc.asset.volumes["500"];
const skin = volume.thermal.find((solid: any) => solid.id === "th-neck");
const body = volume.body.find((solid: any) => solid.id === "neck");
const round = (value: number): number => Math.round(value * 1e5) / 1e5;
let said = "unchanged: no th-neck shorter than the feature";
if (skin !== undefined && body !== undefined && skin.sections.at(-1).at[0] - skin.sections[0].at[0] < LENGTH_M - 1e-6) {
  const [from, to] = [body.sections[0], body.sections.at(-1)];
  const u = skin.sections.at(-1).at[0] - LENGTH_M;
  const t = (u - from.at[0]) / (to.at[0] - from.at[0]);
  if (!(t >= 0 && t <= 1)) throw new Error(`th-neck cannot start at u ${u.toFixed(4)}: outside the body loft "neck"`);
  const mix = (a: number, b: number): number => round(a + (b - a) * t);
  Object.assign(skin.sections[0], { at: [round(u), mix(from.at[1], to.at[1]), mix(from.at[2], to.at[2])], rx: mix(from.rx, to.rx), ry: mix(from.ry, to.ry) });
  said = `th-neck of 1:500 starts at u ${round(u)} (${(LENGTH_M * 1000).toFixed(1)} mm long)`;
}
console.log(said);
if (args.includes("--write") || out !== undefined) console.log(writeDoc(read, out) ? `written: ${out ?? read.file}` : "file unchanged");
