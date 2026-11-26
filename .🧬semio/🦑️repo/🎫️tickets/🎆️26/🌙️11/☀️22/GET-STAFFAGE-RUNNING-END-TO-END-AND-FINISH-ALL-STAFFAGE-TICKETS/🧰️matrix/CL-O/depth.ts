/** 📐️ The overall dimension of the gas hob as `🧪️tests/📐️plan-dimension` reads it, measured on the catalog asset or a trial copy outside the test runner: `bun depth.ts <id|file.json>` prints for the elevation and the plan at 1:50 the labelled quantity, its value, the label and the span between the extension lines in metres, then `DEPTH <id>: OK|FAIL` (plan value within 0.0005 m and span within 0.005 m of `dimensions.depthM`, elevation alike against `heightM`, the rounding of the case).
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/📐️plan-dimension/🟦️.ts — the pinned case */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const { defaultDepictionOptions, depict } = await import(join(S, "🔨️modules/🎨️depiction/🟦️.ts"));
const { evaluate } = await import(join(S, "🔨️modules/🧮️evaluation/🟦️.ts"));
const { writeSvg } = await import(join(S, "🔨️modules/📤️export/🖼️svg/🟦️.ts"));

for (const arg of process.argv.slice(2)) {
  const asset = readDoc(arg).doc.asset, selector = { kind: "scale", scale: { denominator: 50 } } as const;
  let ok = true;
  for (const [viewpoint, quantity, pinned] of [["elevation", "height", asset.dimensions.heightM], ["plan", "depth", asset.dimensions.depthM]] as const) {
    const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, selector), o = defaultDepictionOptions("dark-line", "en", 50, viewpoint);
    const svg: string = writeSvg(depict(state, { ...o, representation: selector, annotations: { ...o.annotations, legend: false } }));
    const tag = (id: string) => new RegExp(`<[^>]*id="${id}"[^>]*>`, "u").exec(svg)?.[0] ?? "";
    const attribute = (element: string, name: string) => new RegExp(` ${name}="([^"]*)"`, "u").exec(element)?.[1] ?? "";
    const y = (id: string) => Number(/^M[\d.-]+ ([\d.-]+)L/u.exec(attribute(tag(id), "d"))?.[1]);
    const line = tag("dimension-line"), value = Number(attribute(line, "data-value")), span = (y("dimension-extension-bottom") - y("dimension-extension-top")) / 20;
    const label = [...svg.matchAll(/<text[^>]*>([^<]*)<\/text>/gu)].map((m) => m[1]!).find((t) => / m$/u.test(t));
    const good = attribute(line, "data-quantity") === quantity && Math.abs(value - pinned) < 0.0005 && label === `${pinned.toFixed(2)} m` && Math.abs(span - pinned) < (viewpoint === "plan" ? 0.005 : 0.0005);
    ok &&= good;
    console.log(`${asset.id} 1:50 ${viewpoint}: ${attribute(line, "data-quantity")} ${value} m, label "${label}", span ${span.toFixed(4)} m, declared ${pinned} m${good ? "" : " !"}`);
  }
  console.log(`DEPTH ${asset.id}: ${ok ? "OK" : "FAIL"}`);
}
