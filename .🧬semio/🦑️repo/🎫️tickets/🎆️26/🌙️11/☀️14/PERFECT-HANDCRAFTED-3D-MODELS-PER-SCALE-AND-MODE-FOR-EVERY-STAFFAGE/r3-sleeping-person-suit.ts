/** 🖨️ Prints the print-suitability violations of every scale of a candidate document of the round 3 sleeping person (what makes the product refuse a drawing as unsuitable).
 * Usage: `bun r3-sleeping-person-suit.ts [document]`
 * @see ./r3-2d.ts */
import { readFileSync } from "node:fs";
import { defaultDepictionOptions, depict } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const file = process.argv[2] ?? "🗑️generated/r3-sleeping-person/🔣️.json";
const asset = JSON.parse(readFileSync(file, "utf8")).asset;
for (const scale of [50, 100, 200, 500]) {
  try {
    const selector = { kind: "scale", scale: { denominator: scale } };
    const state = evaluate(asset, { kind: "pose", pose: "supine" } as never, selector as never);
    const o = defaultDepictionOptions("light-line" as never, "en", scale, "elevation" as never) as any;
    depict(state, { ...o, representation: selector } as never, {});
    console.log(scale, "suitable");
  } catch (e: any) {
    const v = e.resolution?.suitability?.violations ?? [];
    console.log(scale, e.message, JSON.stringify(v.slice(0, 12)));
  }
}
