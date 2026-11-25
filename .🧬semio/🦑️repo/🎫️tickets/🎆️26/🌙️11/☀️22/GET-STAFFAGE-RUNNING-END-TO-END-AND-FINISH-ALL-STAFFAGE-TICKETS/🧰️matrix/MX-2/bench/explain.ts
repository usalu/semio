/** 🪧️ What the explanation box says of an asset at a scale it cannot draw, and how wide its lines are in Anta against the box: `bun explain.ts <id> <denominator> [en|de]`. Run from `S/📦️packages/🟦️typescript`.
 * @see ../../../🧪️tests/🔤️caption-fit — the case that holds the box to its width */
import { join } from "node:path";
import { S, readDoc } from "../../MR/lib.ts";
const { evaluate } = await import(join(S, "🔨️modules/🧮️evaluation/🟦️.ts"));
const { depict, defaultDepictionOptions } = await import(join(S, "🔨️modules/🎨️depiction/🟦️.ts"));
const { explanationBox } = await import(join(S, "🔨️modules/🎨️depiction/🪧️explanation/🟦️.ts"));
const { textWidthMm } = await import(join(S, "🔨️modules/📐️units/🟦️.ts"));
const { LABEL_TEXT_MM, ANNOTATION_TEXT_MM } = await import(join(S, "🔨️modules/📐️units/🟦️.ts"));
const [id, denominator, language = "en"] = process.argv.slice(2);
const asset = readDoc(id!).doc.asset;
const selector = { kind: "scale", scale: { denominator: Number(denominator) } } as const;
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, selector);
try {
  depict(state, { ...defaultDepictionOptions("dark-line", language, Number(denominator), "front"), representation: selector });
  console.log("drawn");
} catch (error: any) {
  if (error.resolution === undefined) throw error;
  const box = explanationBox(asset, error.resolution, language, 80);
  for (const line of box.lines) {
    const w = textWidthMm(line.text, line.bold ? ANNOTATION_TEXT_MM : LABEL_TEXT_MM, line.bold);
    console.log(`${(w + 2).toFixed(1)} of ${box.widthMm} mm ${w + 2 > box.widthMm - 2 ? "OVER" : "ok"}  ${line.text}`);
  }
}
